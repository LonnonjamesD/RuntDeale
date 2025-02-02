use std::option::Option;
use std::path::Path;
use std::result::Result;

use graphics::{
    // Shapes.
    draw_state::DrawState,
    image::Image,

    // Other.
    math::Matrix2d,
    math::Scalar,
    rectangle::Rectangle,
    Graphics,
};
use opengl_graphics::{Texture, TextureSettings};

use super::position::Positioned;

/**
  Represents the rendering information
  for a sprite.
*/
pub enum RenderInfo {
    None,
    Rectangle(Rectangle),
    Image(Image, Texture),
}

impl RenderInfo {
    fn new_image(p: &Path) -> Self {
        let img = Image::new();
        let mut tex = None;

        match Texture::from_path(p, &TextureSettings::new()) {
            Result::Err(msg) => {
                let p = match p.to_str() {
                    Option::None => "<UNKNOWN PATH>",
                    Option::Some(msg) => msg,
                };
                debug_assert!(
                    false,
                    "Could not load texture at path: {}. Message:\n{}",
                    p, &msg
                );
            }

            Result::Ok(img) => {
                tex = Some(img);
            }
        }

        RenderInfo::Image(img, tex.unwrap())
    }
}

/**
  Contains methods for describing a RuntDeale sprite.
*/
pub trait SpriteDescriptor: Positioned {
    fn draw(&mut self, tf: Matrix2d);
}

/**
  Represents an object that can be drawn to the screen.
  Utility wrapper for Piston to make moving
  objects around easier.
*/
pub struct Sprite<'a, T: Graphics> {
    gfx_backend: &'a mut T,
    pub draw_state: DrawState,
    pub portrait: RenderInfo,
    pub x: Scalar,
    pub y: Scalar,
    pub w: Scalar,
    pub h: Scalar,
}

impl<'a, T: Graphics> Sprite<'a, T> {
    pub fn new(gbe: &'a mut T) -> Self {
        Sprite::<'a, T> {
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,
            gfx_backend: gbe,
            draw_state: DrawState::default(),
            portrait: RenderInfo::None,
        }
    }
}

impl<T: Graphics<Texture = Texture>> SpriteDescriptor for Sprite<'_, T> {
    fn draw(&mut self, tf: Matrix2d) {
        match &self.portrait {
            RenderInfo::Rectangle(rect) => {
                rect.draw(
                    [self.x, self.y, self.w, self.h],
                    &self.draw_state,
                    tf,
                    self.gfx_backend,
                );
            }
            RenderInfo::Image(img, tex) => {
                img.draw(tex, &self.draw_state, tf, self.gfx_backend);
            }

            _ => {}
        }
    }
}

impl<T: Graphics<Texture = Texture>> Positioned for Sprite<'_, T> {
    fn shift_position(&mut self, dx: Scalar, dy: Scalar) {
        self.x += dx;
        self.y += dy;
    }

    fn get_position(&self) -> (Scalar, Scalar) {
        (self.x, self.y)
    }

    fn set_position(&mut self, x: Scalar, y: Scalar) {
        self.x = x;
        self.y = y;
    }
}
