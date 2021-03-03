
use crate::UnGUI::widget::*;
use crate::UnGUI::meta::AbsoluteMeta;
use crate::UnGUI::skeleton::get_children;

pub struct UnCanvas {
    meta : AbsoluteMeta
}

impl UnCanvas {
    pub fn new(meta: AbsoluteMeta) -> Self {
        Self {
            meta
        }
    }

    pub fn get_meta(self) -> AbsoluteMeta {
        self.meta
    }

    pub fn set_meta(&mut self, meta : AbsoluteMeta) -> &Self {
        self.meta = meta;
        self
    }

    pub fn display(&self, sdl_canvas: &mut sdl2::render::WindowCanvas, children: &[(&UnWidget, usize)]) {
        let pos = get_children(children);

        if pos.len() == 0 {
            return
        }

        let pos1 = pos.get(0).unwrap();

        if pos1[0] == pos1[1] {
            children.get(pos1[0]).unwrap().0.display(sdl_canvas, &self.meta, &[]);
        } else {
            children.get(pos1[0]).unwrap().0.display(sdl_canvas, &self.meta, &children[pos1[0]+1..pos1[1]]);
        }

        return
    }
}