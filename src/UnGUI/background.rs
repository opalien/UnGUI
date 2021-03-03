

use crate::UnGUI::widget::*;
use crate::UnGUI::meta::{RelativeMeta, AbsoluteMeta};
use crate::UnGUI::color::*;
use sdl2::render::WindowCanvas;

pub struct UnBackground {
    color: RGB,
    meta: RelativeMeta
}

impl UnBackground {
    pub fn new(color: RGB, meta: RelativeMeta) -> Self {
        Self {
            color,
            meta
        }
    }

    pub fn get_color(self) -> RGB {
        self.color
    }

    pub fn get_meta(self) -> RelativeMeta {
        self.meta
    }

    pub fn set_color(&mut self, color: RGB) -> &Self {
        self.color = color;
        self
    }

    pub fn set_meta(&mut self, meta: RelativeMeta) -> &Self {
        self.meta = meta;
        self
    }
}

impl UnWidget for UnBackground {
    fn display(&self, sdl_canvas: &mut WindowCanvas, meta: &AbsoluteMeta, children: &[(&dyn UnWidget, usize)]) {
        sdl_canvas.set_draw_color(sdl2::pixels::Color::RGB(self.color.r, self.color.g, self.color.b));
        sdl_canvas.fill_rect(sdl2::rect::Rect::new(meta.get_position_x(), meta.get_position_y(), meta.get_width(), meta.get_height()));
    }

    fn get_relative_meta(&self) -> &RelativeMeta {
        &self.meta
    }
}