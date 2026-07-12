use std::ffi::CString;
use std::path::Path;

use rustyray_sys::{
    ffi::{self, is_window_ready, load_image_from_memory, load_texture_from_image},
    texture::{
        Image as RayImage, RenderTexture as RayRenderTexture, RenderTextureLoadError,
        Texture as RayTexture, TextureLoadError,
    },
};

use super::assets::{Asset, AssetLoader, SyncAsset};
use super::math::Vector2i;

#[derive(Debug, PartialEq)]
pub struct Texture {
    inner: RayTexture,
}

#[derive(Debug)]
pub struct RenderTexture {
    inner: RayRenderTexture,
}

impl Texture {
    pub(crate) fn from_image(image: RayImage) -> Self {
        Self {
            inner: unsafe { load_texture_from_image(image) },
        }
    }

    pub fn size(&self) -> Vector2i {
        Vector2i {
            x: self.inner.width,
            y: self.inner.height,
        }
    }

    pub fn width(&self) -> i32 {
        self.inner.width
    }

    pub fn height(&self) -> i32 {
        self.inner.height
    }

    pub(crate) fn as_ray(&self) -> RayTexture {
        self.inner.clone()
    }
}

impl RenderTexture {
    pub(crate) fn from_size(width: i32, height: i32) -> Result<Self, RenderTextureLoadError> {
        Ok(Self {
            inner: RayRenderTexture::new(width, height)?,
        })
    }

    pub fn size(&self) -> Vector2i {
        Vector2i {
            x: self.inner.texture.width,
            y: self.inner.texture.height,
        }
    }

    pub fn width(&self) -> i32 {
        self.inner.texture.width
    }

    pub fn height(&self) -> i32 {
        self.inner.texture.height
    }

    pub(crate) fn as_ray(&self) -> RayRenderTexture {
        self.inner.clone()
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            ffi::unload_texture(self.inner.clone());
        }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe {
            ffi::unload_render_texture(self.inner.clone());
        }
    }
}

impl Asset for Texture {}

impl AssetLoader for Texture {
    type Key = String;
    type Error = TextureLoadError;

    async fn load(path: Self::Key) -> Result<Self, Self::Error> {
        let bytes = async_fs::read(&path)
            .await
            .map_err(|_| TextureLoadError::FileNotFound(path.clone()))?;

        if !unsafe { is_window_ready() } {
            return Err(TextureLoadError::WindowNotReady());
        }

        let extension = Path::new(&path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("png")
            .to_ascii_lowercase();

        let ext = CString::new(format!(".{extension}"))
            .map_err(|_| TextureLoadError::FileNotFound(String::from("invalid file extension")))?;
        let image =
            unsafe { load_image_from_memory(ext.as_ptr(), bytes.as_ptr(), bytes.len() as i32) };

        if image.data.is_null() || image.width <= 0 || image.height <= 0 {
            return Err(TextureLoadError::FileNotFound(String::from(
                "failed to decode image",
            )));
        }

        Ok(Self::from_image(image))
    }
}

impl Asset for RenderTexture {}

impl SyncAsset for RenderTexture {
    type Key = (i32, i32);
    type Error = RenderTextureLoadError;

    fn create((width, height): Self::Key) -> Result<Self, Self::Error> {
        Self::from_size(width, height)
    }
}