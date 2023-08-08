//! A contiguous growable array type with heap-allocated contents
//! with fast deletion process.
//!
//! This is a wrapper for [`Vec<Option<T>>`]
//!
//! ## Use an OptVec when:
//! * You want fast random access and deletion,
//! but don't want to use expensive structures like HashMap.
//! * You want to guarantee that the same index
//! keeps the same value even if another element is removed.
//!
//! ## Getting Started
//! Cargo.toml
//! ```text
//! [dependencies]
//! opt-vec = "*"
//! ```
//!
//! and then
//!
//! ```
//! use opt_vec::OptVec;
//!
//! let mut v = OptVec::new();
//! v.push(1);
//!
//! ```
//!
//!
//! ## Support `no_std`
//!
//! Cargo.toml
//!
//! ```text
//! [dependencies.opt-vec]
//! version = "*"
//! default-features = false
//! features = ["alloc"]
//! ````
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
mod lib {
    pub use std::convert::identity;
    pub use std::ops::{Index, IndexMut};
    pub use std::slice::{Iter, IterMut};
}

#[cfg(not(feature = "std"))]
mod lib {
    use core::convert::identity;
    use core::ops::{Index, IndexMut};
    use core::slice::{Iter, IterMut};
}

use lib::*;

/// A contiguous growable array type with heap-allocated contents
/// with fast deletion process.
///
/// This is a wrapper for [`Vec<Option<T>>`]
///
/// ## Examples
/// ```
/// use opt_vec::OptVec;
///
/// let mut opt_vec: OptVec<i32> = OptVec::new();
///
/// opt_vec.push(1);
/// opt_vec.push(2);
/// opt_vec.push(3);
/// assert_eq!(opt_vec[2], 3);
///
/// opt_vec.remove(1);
/// assert_eq!(opt_vec[2], 3);
///
/// opt_vec.push(4);
/// assert_eq!(opt_vec.to_vec(), vec![1, 4, 3]);
///
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct OptVec<T> {
    inner: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T> OptVec<T> {
    /// Number of elements actually stored
    /// If you want to know the length of the inner vector, use [`OptVec::inner_len()`]
    pub fn len(&self) -> usize {
        self.inner.iter().filter(|a| a.is_some()).count()
    }

    /// Returns the number of elements in the inner vector, also referred to as its `length`
    pub fn inner_len(&self) -> usize {
        self.inner.len()
    }

    /// Returns the total number of elements the vector can hold without reallocating.
    /// Calculated by the following formula:
    ///
    /// `inner vector capacity + free space length`
    ///
    pub fn capacity(&self) -> usize {
        self.inner.capacity() + self.free.len()
    }

    /// Converts the [`OptVec<T>`] into [`Vec<T>`]
    pub fn to_vec(self) -> Vec<T> {
        self.inner.into_iter().filter_map(|v| v).collect()
    }

    /// Constructs a new, empty `OptVec<T>`
    /// The vector will not be allocated until elements are pushed onto it.
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            free: Vec::new(),
        }
    }

    /// Constructs a new, empty `OptVec<T>` with at least the specified capacity.
    /// For a detailed explanation, see [here](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity)
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            free: Vec::with_capacity(capacity),
        }
    }

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop().and_then(identity)
    }

    /// Appends an element to the first free space.
    /// ## Panic
    /// Panics if the new capacity exceeds isize::MAX bytes.
    pub fn push(&mut self, value: T) -> usize {
        if let Some(i) = self.free.pop() {
            self.inner[i] = Some(value);
            i
        } else {
            self.inner.push(Some(value));
            self.inner.len() - 1
        }
    }

    /// Removes and returns the element at the position index within the vector.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if self.inner[index].is_some() {
            self.free.push(index);
            self.inner[index].take()
        } else {
            None
        }
    }
}

impl<T> Index<usize> for OptVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner[index].as_ref().unwrap()
    }
}

impl<T> IndexMut<usize> for OptVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.inner[index].as_mut().unwrap()
    }
}

impl<'a, T> IntoIterator for &'a OptVec<T> {
    type Item = &'a Option<T>;

    type IntoIter = Iter<'a, Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut OptVec<T> {
    type Item = &'a mut Option<T>;

    type IntoIter = IterMut<'a, Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::OptVec;

    #[test]
    fn ok() {
        let mut v: OptVec<i32> = OptVec::new();
        assert_eq!(v.push(1), 0);
        assert_eq!(v.push(2), 1);
        assert_eq!(v.push(3), 2);
        assert_eq!(v.push(4), 3);
        assert_eq!(v.push(5), 4);

        assert_eq!(v.pop(), Some(5));
        assert_eq!(v.inner, vec![Some(1), Some(2), Some(3), Some(4)]);
        assert_eq!(v.free, vec![] as Vec<usize>);

        assert_eq!(v.remove(1), Some(2));
        assert_eq!(v.inner, vec![Some(1), None, Some(3), Some(4)]);
        assert_eq!(v.free, vec![1]);

        assert_eq!(v.remove(1), None);
        assert_eq!(v.inner, vec![Some(1), None, Some(3), Some(4)]);
        assert_eq!(v.free, vec![1]);

        assert_eq!(v.push(5), 1);
        assert_eq!(v.inner, vec![Some(1), Some(5), Some(3), Some(4)]);
        assert_eq!(v.free, vec![] as Vec<usize>);
    }

    #[should_panic]
    #[test]
    fn err() {
        let mut v = OptVec::new();
        v.push(1);
        v.remove(1);
    }
}
