use gdk::{
    DmabufTexture,
    DmabufTextureBuilder,
    Texture,
};

pub trait Builder {
    fn builder() -> TextureBuilder;
}

impl Builder for DmabufTexture {
    fn builder() -> TextureBuilder {
        TextureBuilder::new()
    }
}

pub struct TextureBuilder {
    inner: DmabufTextureBuilder,
}

impl Default for TextureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TextureBuilder {
    pub fn new() -> Self {
        Self {
            inner: DmabufTextureBuilder::new(),
        }
    }

    pub fn width(self, width: u32) -> Self {
        self.inner.set_width(width);
        self
    }

    pub fn height(self, height: u32) -> Self {
        self.inner.set_height(height);
        self
    }

    pub fn fd(self, plane: u32, fd: i32) -> Self {
        self.inner.set_fd(plane, fd);
        self
    }

    pub fn display(self, display: &gdk::Display) -> Self {
        self.inner.set_display(display);
        self
    }

    pub fn fourcc(self, fourcc: u32) -> Self {
        self.inner.set_fourcc(fourcc);
        self
    }

    pub fn modifier(self, modifier: u64) -> Self {
        self.inner.set_modifier(modifier);
        self
    }

    pub fn n_planes(self, n_planes: u32) -> Self {
        self.inner.set_n_planes(n_planes);
        self
    }

    pub fn offset(self, plane: u32, offset: u32) -> Self {
        self.inner.set_offset(plane, offset);
        self
    }

    pub fn premultiplied(self, premultiplied: bool) -> Self {
        self.inner.set_premultiplied(premultiplied);
        self
    }

    pub fn stride(self, plane: u32, stride: u32) -> Self {
        self.inner.set_stride(plane, stride);
        self
    }

    pub unsafe fn build(&self) -> Result<Texture, gdk::glib::Error> {
        unsafe { self.inner.clone().build() }
    }
}
