#![warn(missing_docs)]

//! Ergonomic indexing of standard collections using `at` method.

use std::collections::{LinkedList, VecDeque};
use std::ops::{Index, IndexMut};

/// At trait
/// The `At<V,T>` trait allows an ordered collection containing type `V` to be indexed by type `T`.
/// If `T` is negative, then the collection is indexed in reverse.
///
/// # Examples
/// ```
/// use ati::At;
///
/// let v = vec![1,2,3,4];
///
/// assert_eq!(1, *v.at(0));
/// assert_eq!(4, *v.at(-1));
/// ```
pub trait At<V, T> {
    /// Returns an item by reference.
    /// Support negative index.
    ///
    /// # Examples
    ///
    /// ```
    /// use ati::At;
    ///
    /// let v = vec![1,2,3];
    /// assert_eq!(2, *v.at(-2));
    /// ```
    fn at(&self, c: T) -> &V;

    /// Returns an item by mutable reference.
    /// Supports negative index.
    ///
    /// # Examples
    /// ```
    /// use ati::At;
    ///
    /// let mut v = vec![1,2,3];
    /// *v.at_mut(-1) = 5;
    /// assert!(matches!(&v[..], &[1, 2, 5]));
    /// ```
    fn at_mut(&mut self, c: T) -> &mut V;
}

trait Length {
    fn length(&self) -> usize;
}

impl<V> Length for Vec<V> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<V, const L: usize> Length for [V; L] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<V> Length for VecDeque<V> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<V> Length for LinkedList<V> {
    fn length(&self) -> usize {
        self.len()
    }
}

macro_rules! neg_index {
    ($len: expr, $i: expr, $it: ty) => {{
        let new_index = ($len as $it + $i);
        if new_index < 0 {
            panic!("index out of bounds: the index is ({new_index})");
        }
        new_index as usize
    }};
}

macro_rules! at_unsigned {
    ($e: ty) => {
        impl<V, T: Index<usize, Output = V> + IndexMut<usize, Output = V>> At<V, $e> for T {
            fn at(&self, c: $e) -> &V {
                &self[c as usize]
            }

            fn at_mut(&mut self, c: $e) -> &mut V {
                &mut self[c as usize]
            }
        }
    }
}

macro_rules! at_signed {
    ($e: ty) => {
        impl<V, T: Index<usize, Output = V> + IndexMut<usize, Output = V> + Length> At<V, $e> for T {
            fn at(&self, c: $e) -> &V {
                if c < 0 {
                    &self[neg_index!(self.length(), c, $e)]
                } else {
                    &self[c as usize]
                }
            }

            fn at_mut(&mut self, c: $e) -> &mut V {
                let l = self.length();
                if c < 0 {
                    &mut self[neg_index!(l, c, $e)]
                } else {
                    &mut self[c as usize]
                }
            }
        }
    }
}

at_unsigned!(u8);
at_unsigned!(u16);
at_unsigned!(u32);
at_unsigned!(u64);
at_unsigned!(u128);

at_signed!(i8);
at_signed!(i16);
at_signed!(i32);
at_signed!(i64);
at_signed!(i128);
at_signed!(isize);

#[test]
fn test_negative() {
    let v: Vec<i32> = (1..=10).rev().collect();
    for i in 1..=10 {
        assert_eq!(i, *v.at(-i));
    }
}

#[test]
fn test_positive() {
    let v: Vec<i32> = (0..10).collect();
    for i in 0..10 {
        assert_eq!(i, *v.at(i));
    }
}

#[test]
#[should_panic(expected = "index out of bounds: the index is (-1)")]
fn test_negative_panic() {
    let v: Vec<i32> = (0..2).collect();
    v.at(-3);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 2 but the index is 3")]
fn test_positive_panic() {
    let v: Vec<i32> = (0..2).collect();
    v.at(3);
}
