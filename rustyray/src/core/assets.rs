use std::any::{Any, TypeId};
use std::cell::Cell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetId(u64);

pub trait Asset: 'static {}

#[allow(async_fn_in_trait)]
pub trait AssetLoader: Asset + Sized {
    type Key: 'static;
    type Error: std::error::Error + 'static;

    async fn load(key: Self::Key) -> Result<Self, Self::Error>;
}

pub trait SyncAsset: Asset + Sized {
    type Key: 'static;
    type Error: std::error::Error + 'static;

    fn create(key: Self::Key) -> Result<Self, Self::Error>;
}

#[derive(Debug)]
struct RefCount {
    id: AssetId,
    jobs: Sender<Job>,
}

#[derive(Debug)]
pub struct Handle<T: Asset> {
    rc: Arc<RefCount>,
    _marker: PhantomData<T>,
}

impl<T: Asset> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            rc: Arc::clone(&self.rc),
            _marker: PhantomData,
        }
    }
}

impl<T: Asset> Drop for Handle<T> {
    fn drop(&mut self) {
        if Arc::strong_count(&self.rc) == 1 {
            let _ = self
                .rc
                .jobs
                .send(Job::Release(TypeId::of::<T>(), self.rc.id));
        }
    }
}

impl<T: Asset> Handle<T> {
    pub fn id(&self) -> AssetId {
        self.rc.id
    }
}

struct Store<T: Asset> {
    assets: HashMap<AssetId, T>,
}

trait ErasedStore {
    fn as_any_ref(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove(&mut self, id: AssetId);
}

impl<T: Asset> ErasedStore for Store<T> {
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove(&mut self, id: AssetId) {
        self.assets.remove(&id);
    }
}

enum Job {
    Release(TypeId, AssetId),
}

type LoadPollingFn = dyn FnMut(&mut AssetManager, &mut Context<'_>) -> Poll<()>;

struct PendingLoad {
    poll: Box<LoadPollingFn>,
}

pub struct AssetManager {
    next_id: Cell<u64>,
    stores: HashMap<TypeId, Box<dyn ErasedStore>>,
    jobs_rx: Mutex<Receiver<Job>>,
    jobs_tx: Sender<Job>,
    pending: Vec<PendingLoad>,
}

impl std::fmt::Debug for AssetManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssetManager").finish_non_exhaustive()
    }
}

fn noop_waker() -> Waker {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        |_| RawWaker::new(std::ptr::null(), &VTABLE),
        |_| {},
        |_| {},
        |_| {},
    );
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AssetManager {
    pub fn new() -> Self {
        let (jobs_tx, jobs_rx) = mpsc::channel();

        Self {
            next_id: Cell::new(1),
            stores: HashMap::new(),
            jobs_rx: Mutex::new(jobs_rx),
            jobs_tx,
            pending: Vec::new(),
        }
    }

    fn alloc_id(&self) -> AssetId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        AssetId(id)
    }

    fn make_handle<T: Asset>(&self, id: AssetId) -> Handle<T> {
        Handle {
            rc: Arc::new(RefCount {
                id,
                jobs: self.jobs_tx.clone(),
            }),
            _marker: PhantomData,
        }
    }

    fn store<T: Asset>(&self) -> Option<&Store<T>> {
        self.stores
            .get(&TypeId::of::<T>())?
            .as_any_ref()
            .downcast_ref()
    }

    fn store_mut<T: Asset>(&mut self) -> &mut Store<T> {
        let type_id = TypeId::of::<T>();
        self.stores.entry(type_id).or_insert_with(|| {
            Box::new(Store::<T> {
                assets: HashMap::new(),
            })
        });
        self.stores
            .get_mut(&type_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }

    fn insert_at<T: Asset>(&mut self, id: AssetId, asset: T) {
        self.store_mut::<T>().assets.insert(id, asset);
    }

    fn remove(&mut self, type_id: TypeId, id: AssetId) {
        let Some(store) = self.stores.get_mut(&type_id) else {
            return;
        };
        store.remove(id);
    }

    pub fn insert<T: Asset>(&mut self, asset: T) -> Handle<T> {
        let id = self.alloc_id();
        self.insert_at(id, asset);
        self.make_handle(id)
    }

    pub fn create<T: SyncAsset>(&mut self, key: T::Key) -> Result<Handle<T>, T::Error> {
        Ok(self.insert(T::create(key)?))
    }

    pub fn load<T: Asset + AssetLoader>(&mut self, key: T::Key) -> Handle<T> {
        let id = self.alloc_id();
        let handle = self.make_handle::<T>(id);
        let alive = Arc::downgrade(&handle.rc);

        let mut future = Box::pin(T::load(key));

        self.pending.push(PendingLoad {
            poll: Box::new(move |manager: &mut AssetManager, cx: &mut Context<'_>| {
                match future.as_mut().poll(cx) {
                    Poll::Ready(Ok(asset)) => {
                        if alive.upgrade().is_some() {
                            manager.insert_at(id, asset);
                        }
                        Poll::Ready(())
                    }
                    Poll::Ready(Err(err)) => {
                        eprintln!("failed to load asset {id:?}: {err}");
                        Poll::Ready(())
                    }
                    Poll::Pending => Poll::Pending,
                }
            }),
        });

        handle
    }

    pub fn process_assets(&mut self) {
        let jobs: Vec<Job> = self.jobs_rx.lock().unwrap().try_iter().collect();
        for job in jobs {
            match job {
                Job::Release(type_id, id) => self.remove(type_id, id),
            }
        }

        if self.pending.is_empty() {
            return;
        }

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        let pending = std::mem::take(&mut self.pending);
        let mut remaining = Vec::with_capacity(pending.len());

        for mut load in pending {
            if load.poll.as_mut()(self, &mut cx).is_pending() {
                remaining.push(load);
            }
        }

        self.pending = remaining;
    }

    pub fn get<T: Asset>(&self, handle: &Handle<T>) -> Option<&T> {
        self.store::<T>()?.assets.get(&handle.id())
    }

    pub fn get_mut<T: Asset>(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        self.store_mut::<T>().assets.get_mut(&handle.id())
    }

    pub fn is_ready<T: Asset>(&self, handle: &Handle<T>) -> bool {
        self.get(handle).is_some()
    }
}
