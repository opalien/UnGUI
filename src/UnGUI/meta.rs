//! metadata
//!
//! AbsoluteMeta and Relative Meta

use crate::UnGUI::vector::Vector2D;


//////////////////////////////////////////////////AbsoluteMeta//////////////////////////////////////////////////
/// AbsoluteMeta represent some data for direct render screen
#[derive(Copy, Clone)]
pub struct AbsoluteMeta {
    position: Vector2D,
    width: u32,
    height: u32
}

impl AbsoluteMeta {
    pub fn new(position: Vector2D, width: u32, height: u32) -> Self {
        Self {
            position,
            width,
            height
        }
    }

    // getters
    /// get a copy of the position
    pub fn get_position(self) -> Vector2D { self.position }

    pub fn get_position_x(self) -> i32 { self.position.x }

    pub fn get_position_y(self) -> i32 { self.position.y }

    /// get a copy of width
    pub fn get_width(self) -> u32 { self.width }

    /// get a copy of height
    pub fn get_height(self) -> u32 { self.height }


    //setters
    /// set a new position
    pub fn set_position(&mut self, position: Vector2D) -> &Self {
        self.position = position;
        self
    }

    /// set a new width
    pub fn set_width(&mut self, width: u32) -> &Self {
        self.width = width;
        self
    }

    /// set a new height
    pub fn set_height(&mut self, height: u32) -> &Self {
        self.height = height;
        self
    }

    // other
    /// get RelativeMeta of the child by the default way
    pub fn get_child_absolute_meta_default(self, relative: &RelativeMeta) -> Self {
        AbsoluteMeta {
            position: self.position + relative.position,
            width: relative.get_width(),
            height: relative.get_height()
        }
    }
}


//////////////////////////////////////////////////RelativeMeta//////////////////////////////////////////////////

/// RelativeMeta represent data in relation to the parent widget
#[derive(Copy, Clone)]
pub struct RelativeMeta {
    position: Vector2D,
    width: u32,
    height: u32
}

impl RelativeMeta {
    pub fn new(position: Vector2D, width: u32, height: u32) -> Self {
        Self {
            position,
            width,
            height
        }
    }

    // getters
    /// get a copy of the position
    pub fn get_position(self) -> Vector2D { self.position }

    /// get a copy of width
    pub fn get_width(self) -> u32 { self.width }

    /// get a copy of height
    pub fn get_height(self) -> u32 { self.height }


    //setters
    /// set a new position
    pub fn set_position(&mut self, position: Vector2D) -> &Self {
        self.position = position;
        self
    }

    /// set a new width
    pub fn set_width(&mut self, width: u32) -> &Self {
        self.width = width;
        self
    }

    /// set a new height
    pub fn set_height(&mut self, height: u32) -> &Self {
        self.height = height;
        self
    }

    // other
    /// add to the position the vector passed in parameter
    pub fn move_position(&mut self, value: Vector2D) -> &Self {
        self.position = self.position + value;
        self
    }


}

//////////////////////////////////////////////////Some functions//////////////////////////////////////////////////

