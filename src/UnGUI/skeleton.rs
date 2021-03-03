//! methods and structure to encapsulate the display pipeline

use crate::UnGUI::widget::*;

/// skeleton is passed to the display pipeline
pub trait Skeleton {
    fn child(&mut self, vec: &'static mut Vec<(&dyn UnWidget, u32)>) -> &Self;
    fn widget(&mut self, widget: &'static dyn UnWidget) -> &Self;
    fn pack(&self) -> &[(&dyn UnWidget, u32)];
}

impl Skeleton for Vec<(&'static dyn UnWidget, u32)> {

    fn child(&mut self, child: &'static mut Vec<(&dyn UnWidget, u32)>) -> &Self {
        let index = self.last().unwrap().1;


        let mut i : usize = 0;
        while i < child.len() {
            child[i].1 += index;
            i+=1;
        }

        self.append(child);

        self
    }

    fn widget(&mut self, widget: &'static dyn UnWidget) -> &Self {
        self.push((widget, 1));
        self
    }

    fn pack(&self) -> &[(&dyn UnWidget, u32)] {
        self.as_slice()
    }
}

pub fn widget(widget: &'static dyn UnWidget) -> Vec<(&'static dyn UnWidget, u32)> {
    vec![(widget, 1)]
}

pub fn new_skeleton() -> Vec<(&'static dyn UnWidget, u32)> {
    vec![]
}



pub fn get_children(children_: &[(&UnWidget, usize)])->Vec<[usize;2]> {
    if children_.len() == 0 {
        return vec![]
    }

    let mut result : Vec<[usize;2]> = vec![];

    let mut index = children_.get(0).unwrap().1;

    let mut i: usize = 0;
    while i < children_.len() {
        if children_.get(i).unwrap().1 == index {
            if i+1 < children_.len() {
                let j = i;
                'l: while i < children_.len() {
                    if children_.get(i).unwrap().1 == index {
                        break 'l;
                    }
                    i+=1;
                }
                result.push([j, i-1]);

            } else {
                result.push([i, i]);
            }
        }
        i+=1;
    }

    return result
}