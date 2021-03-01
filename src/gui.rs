

pub mod UnGUI {
    use std::ops;
    use sdl2;

    //--------------------Vector2D--------------------//
    #[derive(Copy, Clone)]
    pub struct Vector2D {
        pub x: i32,
        pub y: i32
    }

    impl Vector2D {
        pub fn new(x_: i32, y_: i32)->Vector2D {
            Vector2D {
                x: x_,
                y: y_
            }
        }
    }

    impl ops::Add<Vector2D> for Vector2D {
        type Output = Vector2D;
        fn add(self, vec: Vector2D)->Vector2D {
            Vector2D {
                x: self.x + vec.x,
                y: self.y + vec.y
            }
        }
    }

    //--------------------Color--------------------//

    #[derive(Copy, Clone)]
    pub struct RGB {
        pub r: u8,
        pub g: u8,
        pub b: u8
    }

    impl RGB {
        pub fn new(r_: u8, g_: u8, b_: u8)-> RGB {
            RGB {
                r:r_,
                g:g_,
                b:b_
            }
        }
    }


    //--------------------Meta--------------------//

    #[derive(Copy, Clone)]
    pub struct AbsoluteMeta {
        position: Vector2D,
        index: usize,
        width: u32,
        height: u32
    }

    impl AbsoluteMeta {
        pub fn new(position_: Vector2D)->AbsoluteMeta {
            AbsoluteMeta {
                position : position_,
                index: 0,
                width: 0,
                height: 0
            }
        }

        pub fn set_index(&mut self, index_: usize)-> &AbsoluteMeta {
            self.index = index_;
            self
        }

        pub fn get_index(self)-> usize { self.index }

        pub fn get_position(self)-> Vector2D {
            self.position
        }

        pub fn get_position_x(self)->i32 {
            self.position.x
        }

        pub fn get_position_y(self)->i32 {
            self.position.y
        }

        pub fn get_width(self)->u32 {
            self.width
        }

        pub fn get_height(self)->u32 {
            self.height
        }

        pub fn set_position(&mut self, position_: Vector2D)-> &AbsoluteMeta {
            self.position = position_;
            self
        }

        pub fn move_position(&mut self, position_: Vector2D)-> &AbsoluteMeta {
            self.position = self.position + position_;
            self
        }

        pub fn set_dimension(&mut self, width_: u32, height_: u32)-> &AbsoluteMeta {
            self.width = width_;
            self.height = height_;
            self
        }

        pub fn set_width(&mut self, width_: u32)-> &AbsoluteMeta {
            self.width = width_;
            self
        }

        pub fn set_height(&mut self, height_: u32)-> &AbsoluteMeta {
            self.height = height_;
            self
        }
    }

    #[derive(Copy, Clone)]
    pub struct RelativeMeta {
        position: Vector2D
    }

    impl RelativeMeta {
        pub fn new(position_: Vector2D)->RelativeMeta {
            RelativeMeta {
                position : position_
            }
        }

        pub fn change_position(&mut self, position_: Vector2D)-> &RelativeMeta {
            self.position = position_;
            self
        }

        pub fn move_position (&mut self, position_: Vector2D)-> &RelativeMeta {
            self.position = self.position + position_;
            self
        }
    }


    //--------------------UnWidget--------------------//
    pub trait UnWidget {
        fn display(&self, sdl_canvas: &mut sdl2::render::WindowCanvas, meta_ : &AbsoluteMeta, children: &[(&UnWidget, usize)]);
        fn get_relative_meta(&self)->&RelativeMeta;
    }


    //--------------------UnCanvas--------------------//

    pub struct UnCanvas {
        meta : AbsoluteMeta
    }

    impl UnCanvas {
        pub fn new(meta_: AbsoluteMeta)->UnCanvas {
            UnCanvas {
                meta : meta_
            }
        }

        pub fn get_meta(self)->AbsoluteMeta {
            self.meta
        }

        pub fn set_meta(&mut self, meta_: AbsoluteMeta)-> &UnCanvas {
            self.meta = meta_;
            self.meta.set_index(0);
            self
        }

        pub fn display(&self, sdl_canvas: &mut sdl2::render::WindowCanvas, children: &[(&UnWidget, usize)]){
            let mut i: usize = 0;

            while i < children.len() {

                if children.get(i).expect("albert").1 == 1 {
                    if i+1 < children.len() {
                        let j = i;
                        i+=1;
                        'lo: while i < children.len() {
                            if children.get(i).expect("bertrand").1 == 1 {
                                children.get(j).expect("celestin").0.display(sdl_canvas, &self.meta,  &children[j+1..i-1]);
                                break 'lo;
                            }
                            i += 1;
                        }

                        children.get(j).expect("celestin").0.display(sdl_canvas, &self.meta,  &children[j+1..i-1]);

                    } else {
                        let b : &[(&UnWidget, usize)] = &[];
                        children.get(0).expect("celestin").0.display(sdl_canvas, &self.meta,  b);
                    }

                    //children.get(1).expect("celestin").0.display(sdl_canvas, &self.meta,  &children[0]);
                }

                i+= 1;
            }
        }
    }


    //--------------------UnLayout_absolute--------------------//
    pub struct UnLayoutAbsolute {
        meta: RelativeMeta
    }

    impl UnLayoutAbsolute {
        pub fn new(meta_: RelativeMeta)->UnLayoutAbsolute {
            UnLayoutAbsolute {
                meta: meta_
            }
        }

        pub fn get_meta(self)-> RelativeMeta {
            self.meta
        }

        pub fn set_meta(&mut self, meta_: RelativeMeta) {
            self.meta = meta_
        }
    }

    impl UnWidget for UnLayoutAbsolute {
        fn display(&self, sdl_canvas: &mut sdl2::render::WindowCanvas, meta_ : &AbsoluteMeta, children: &[(&UnWidget, usize)]) {
            let mut i: usize = 0;
            let index = meta_.get_index() + 1;

            if children.len() == 0 {
                return;
            }

            while i < children.len() {
                if children.get(i).expect("albert").1 == index+1 {
                    if i+1 < children.len() {
                        let j=i;
                        'lo: while i < children.len() {
                            if children.get(i).expect("bertrand").1 == index+1 {
                                //children.get(i).expect("celestin").0.display(sdl_canvas, &self.meta, &children[j+1..i-1]);
                            }
                        }
                    }

                }
            }


        }

        fn get_relative_meta(&self) -> &RelativeMeta { &self.meta }
    }


    //--------------------UnBackground--------------------//
    pub struct UnBackground {
        color: RGB,
        meta: RelativeMeta
    }

    impl UnBackground {
        pub fn new(color_: RGB, meta_: RelativeMeta)-> UnBackground {
            UnBackground {
                color: color_,
                meta: meta_
            }
        }

        pub fn set_color(&mut self, color_: RGB)-> &UnBackground {
            self.color = color_;
            self
        }
    }

    impl UnWidget for UnBackground {
        fn display(&self, sdl_canvas: &mut sdl2::render::WindowCanvas, meta_ : &AbsoluteMeta, children: &[(&UnWidget, usize)]) {
            sdl_canvas.set_draw_color(sdl2::pixels::Color::RGB(self.color.r, self.color.g, self.color.b));
            sdl_canvas.fill_rect(sdl2::rect::Rect::new(meta_.get_position_x(), meta_.get_position_y(), meta_.get_width(), meta_.get_height()));
        }

        fn get_relative_meta(&self)->&RelativeMeta {
            &self.meta
        }
    }


    //--------------------Skeleton--------------------//
    /*
    pub mod Skeleton {
        use crate::gui::UnGUI::{UnWidget, UnBackground, RGB};

        pub trait Skeleton {
            fn new()-> Vec<(&'static dyn UnWidget, u32)>;
            fn child(&mut self, vec: &Vec<(&dyn UnWidget, u32)>)-> &Vec<(&dyn UnWidget, u32)>;
        }

        impl Skeleton for Vec<(&dyn UnWidget, u32)> {
            fn new() -> Vec<(&'static dyn UnWidget, u32)> {
                Vec::new()
            }

            fn child(&mut self, vec: &Vec<(&dyn UnWidget, u32)>) -> &Vec<(&dyn UnWidget, u32)> {
                let index_temp = self.last().unwrap().1;
                let mut vec = &vec;
                for mut w in *vec {
                    &w.1 += index_temp;
                }

                //self.append(vec);

                self
            }
        }
    }*/
}

