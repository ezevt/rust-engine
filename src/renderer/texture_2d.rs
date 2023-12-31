use std::ffi::CString;

use gl::types::*;

pub struct Texture2D {
    pub id: u32,
    width: u32,
    height: u32,
    internal_format: GLenum,
    data_format: GLenum,
    path: String,
}

impl Texture2D {
    pub fn new(width: u32, height: u32) -> Self {
        let mut id: u32 = 0;
        let internal_format = gl::RGBA8;
        let data_format = gl::RGBA;

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            gl::TextureStorage2D(id, 1, internal_format, width as i32, height as i32);

            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        };

        Self {
            id,
            width,
            height,
            internal_format,
            data_format,
            path: String::new(),
        }
    }

    pub fn from_path(path: &str) -> Self {
        let mut texture = Self::new(0, 0);
        texture.path = path.to_string();
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut channels: i32 = 0;

        let path_cstring = CString::new(path).expect("CString conversion failed");
        unsafe {
            stb_image::stb_image::stbi_set_flip_vertically_on_load(1);
            let data = stb_image::stb_image::stbi_load(
                path_cstring.as_ptr(),
                &mut width,
                &mut height,
                &mut channels,
                0,
            );
            assert!(!data.is_null(), "Failed to load image!");

            texture.width = width as u32;
            texture.width = height as u32;

            let (internal_format, data_format) = match channels {
                4 => (gl::RGBA8, gl::RGBA),
                3 => (gl::RGB8, gl::RGB),
                _ => panic!("Format not supported!"),
            };

            texture.internal_format = internal_format;
            texture.data_format = data_format;

            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut texture.id);
            gl::TextureStorage2D(texture.id, 1, internal_format, width, height);

            gl::TextureParameteri(texture.id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TextureParameteri(texture.id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(texture.id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TextureSubImage2D(
                texture.id,
                0,
                0,
                0,
                width,
                height,
                data_format,
                gl::UNSIGNED_BYTE,
                data as *const std::ffi::c_void,
            );

            stb_image::stb_image::stbi_image_free(data as *mut std::ffi::c_void);
        }

        texture
    }

    pub fn set_data(&self, data: *const std::ffi::c_void, size: usize) {
        let bpp = if self.data_format == gl::RGBA { 4 } else { 3 };
        assert_eq!(
            size as u32,
            self.width * self.height * bpp,
            "Data must be entire texture!"
        );
        unsafe {
            gl::TextureSubImage2D(
                self.id,
                0,
                0,
                0,
                self.width as i32,
                self.height as i32,
                self.data_format,
                gl::UNSIGNED_BYTE,
                data,
            );
        }
    }

    pub fn bind(&self, slot: u32) {
        unsafe {
            gl::BindTextureUnit(slot, self.id);
        }
    }
}
