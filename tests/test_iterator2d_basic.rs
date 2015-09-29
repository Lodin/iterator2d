extern crate iterator2d;
extern crate num;

use iterator2d::Iterator2d;
use num::cast;
use std::slice::{ Iter as SliceIter, IterMut as SliceIterMut };

pub struct Collection2d {
    data: [i32; 9]
}

impl Collection2d {
    pub fn new() -> Collection2d {
        Collection2d {
            data: [
                1, 2, 3,
                4, 5, 6,
                7, 8, 9
            ]
        } 
    }

    pub fn iter(&self) -> Iter<i32> {
        Iter {
            iter: self.data.iter(),
            rows: 3,
            cols: 3
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<i32> {
        IterMut {
            iter: self.data.iter_mut(),
            rows: 3,
            cols: 3
        }
    }
}

macro_rules! iterator2d {
    ( struct $name:ident, struct $iter:ident, $t: ty ) => {
        impl<'a, T> $name<'a, T> {

            #[inline]
            pub fn new(iter: $iter<'a, T>, rows: usize, cols: usize) -> $name<T> {
                $name {
                    iter: iter,
                    rows: rows,
                    cols: cols
                }
            }
        }

        impl<'a, T> Iterator2d for $name<'a, T> {
    
            #[inline]
            fn rows(&self) -> usize {
                self.rows
            }

            #[inline]
            fn cols(&self) -> usize {
                self.cols
            }
        }

        impl<'a, T> Iterator for $name<'a, T> {
            type Item = $t;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.iter.size_hint()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                self.iter.nth(n)
            }

            #[inline]
            fn count(self) -> usize {
                self.iter.count()
            }
        } 
    }
}

/// Immutable two-dimensional collection iterator
pub struct Iter<'a, T: 'a> {
    iter: SliceIter<'a, T>,
    rows: usize,
    cols: usize
}

iterator2d!(struct Iter, struct SliceIter, &'a T);

/// Mutable two-dimensional collection iterator
pub struct IterMut<'a, T: 'a> {
    iter: SliceIterMut<'a, T>,
    rows: usize,
    cols: usize
}

iterator2d!(struct IterMut, struct SliceIterMut, &'a mut T);


#[test]
fn iteration_enumerated() {
    let collection = Collection2d::new();
    let mut it = collection.iter().enumerate2d();
    assert_eq!(it.nth(1).unwrap(), (0, 1, &2));
    assert_eq!(it.nth(3).unwrap(), (1, 2, &6));
    assert_eq!(it.next().unwrap(), (2, 0, &7));
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
    assert_eq!(it2.nth(3).unwrap(), (1, 2, &mut 9));
    assert_eq!(it2.next().unwrap(), (2, 0, &mut 9));
}
