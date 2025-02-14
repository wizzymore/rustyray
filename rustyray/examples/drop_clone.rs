#[derive(Clone)]
struct Unit;

struct Unit2;

trait Testing: AsRef<Unit2> {}
impl AsRef<Unit2> for Unit {
    fn as_ref(&self) -> &Unit2 {
        unsafe { std::mem::transmute(self) }
    }
}
impl Testing for Unit {}

fn main() {
    let a = Unit {};
}
