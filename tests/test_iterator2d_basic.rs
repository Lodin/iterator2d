extern crate iterator2d;
extern crate num;

use iterator2d::{ Iterator2d, Iter2d, Iter2dMut };
use num::cast;

pub struct Collection2d {
    data: [i32; 9]
}

impl Collection2d {
    pub fn new() -> Collection2d {
        Collection2d {
            data: [1, 2, 3, 4, 5, 6, 7, 8, 9]
        } 
    }

    pub fn iter(&self) -> Iter2d<i32> {
        Iter2d::new(self.data.iter(), 3, 3)
    }

    pub fn iter_mut(&mut self) -> Iter2dMut<i32> {
        Iter2dMut::new(self.data.iter_mut(), 3, 3)
    }
}

#[test]
fn iteration_enumerated() {
    let collection = Collection2d::new();
    let mut it = collection.iter().enumerate2d();
    assert_eq!(it.nth(1).unwrap(), (0, 1, &2));
}

#[test]
fn iteration_mutable_enumerated() {
    let mut collection = Collection2d::new();

    {
        let it = collection.iter_mut().enumerate2d();

        for (i, j, el) in it {
            let m: i32 = cast(i).unwrap();
            let n: i32 = cast(j).unwrap();
            *el += m + n;
        }
    }
    
    let mut it2 = collection.iter_mut().enumerate2d();
    assert_eq!(it2.nth(1).unwrap(), (0, 1, &mut 3));
}
