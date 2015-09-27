#![crate_name="iterator2d"]
#![crate_type="lib"]
#![crate_type="rlib"]

use std::slice::{ Iter, IterMut };

/// An interface for making possible the iteration in two-dimensional
/// collections. Two-dimensional iterator mainly is the same as
/// `std::iter::Iterator`, and have main additional method: `enumerate2d()`.
/// This method allows getting indexes of rows and columns in the collection
/// and use it.
pub trait Iterator2d {

    /// Creates an iterator that yields triples `(i, j, val)`, where `i` and
    /// `j` is the current indexes of two-dimensional iteration and `val` is
    /// the value returned by iterator
    ///
    /// `enumerate2d` keeps its counts as `usize`.
    #[inline]
    fn enumerate2d(self) -> Enumerate2d<Self> where Self: Sized {
        Enumerate2d { iter: self, row: 0, col: 0 }
    }
    
    /// Returns height of the two-dimensional collection
    fn rows(&self) -> usize;
    
    /// Returns width of the two-dimensional collection
    fn cols(&self) -> usize;
}

macro_rules! iterator2d {
    ( struct $name:ident, struct $iter:ident, $t: ty ) => {
        impl<'a, T> $name<'a, T> {
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
pub struct Iter2d<'a, T: 'a> {
    iter: Iter<'a, T>,
    rows: usize,
    cols: usize
}

iterator2d!(struct Iter2d, struct Iter, &'a T);

/// Mutable two-dimensional collection iterator
pub struct Iter2dMut<'a, T: 'a> {
    iter: IterMut<'a, T>,
    rows: usize,
    cols: usize
}

iterator2d!(struct Iter2dMut, struct IterMut, &'a mut T);

/// An iterator for two-dimensional collection that yields current row, column
/// and the element during iteration
pub struct Enumerate2d<I> {
    iter: I,
    row: usize,
    col: usize
}

impl<I> Iterator for Enumerate2d<I> where I: Iterator + Iterator2d {
    type Item = (usize, usize, <I as Iterator>::Item);

    #[inline]
    fn next(&mut self) -> Option<(usize, usize, <I as Iterator>::Item)> {
        self.iter.next().map(|a| {
            let ret = (self.row, self.col, a);
            self.col += 1;
            if self.col == self.iter.cols() {
                self.col = 0;
                self.row += 1;
            }
            ret
        })
    }
    
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize)
        -> Option<(usize, usize, <I as Iterator>::Item)> {
        self.iter.nth(n).map(|a| {
            let j = n % self.iter.cols();
            let i = (n - j) / self.iter.cols();
            self.col = j + 1;
            self.row = i;

            if self.col == self.iter.cols() {
                self.row += 1;    
            }    

            (i, j, a)
        }) 
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }
}
