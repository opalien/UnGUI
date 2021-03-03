//! Methods and trait caracterizing widget
use crate::UnGUI::meta::*;

pub trait UnWidget {
    /// display is use to display directly the widget in the screen
    fn display(&self, sdl_canvas: &mut sdl2::render::WindowCanvas, meta_: &AbsoluteMeta, children: &[(&dyn UnWidget, usize)]);

    /// is an accessor to get the relative data of the child
    fn get_relative_meta(&self)->&RelativeMeta;
}

