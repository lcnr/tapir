#![forbid(unsafe_code)]
//! A crate exposing the `Tap` trait, which makes method chaining easier.
//!
//! Check out [`Tap`] for a more detailed documentation.
//!
//! # Examples
//!
//! ```
//! use tapir::Tap;
//!
//! fn smallest_factor(x: u32) -> u32 {
//!     for i in 2..x {
//!         if x % i == 0 {
//!             return i;
//!         }
//!     }
//!
//!     x
//! }
//!
//! let smallest_factors: Vec<u32> = (2..25).map(smallest_factor).collect();
//!
//! let unique_primes = smallest_factors.tap(|v| v.sort()).tap(Vec::dedup);
//! assert_eq!(unique_primes, [2, 3, 5, 7, 11, 13, 17, 19, 23]);
//! ```
//!
//! [`Tap`]: ./trait.Tap.html

/// An interface to enable the `tap` operation which is implemented for all `Sized` types.
///
/// The tap operation takes full ownership of a variable, calls the given function with a mutable
/// reference to the given variable and then returns full ownership of it.
/// This allows for easy mutation without having to declare the variable as mutable.
///
/// Tapping can be used to reduce the amount of local mutable variables,
/// which can make the code easier to read and may prevent accidental mutation.
///
/// # Examples
///
/// ```rust
/// fn get_unsorted_values() -> Vec<u32> {
///     vec![42, 7, 1337, 69]
/// }
///
/// fn use_sorted_values(values: &[u32]) {
///     assert_eq!(&[7, 42, 69, 1337], values);
/// }
///
/// // without tap one often needs mutable variables.
/// let mut old = get_unsorted_values();
/// old.sort();
/// use_sorted_values(&old);
///
/// // using tap, this can be simplified.
/// use tapir::Tap;
///
/// let tapped = get_unsorted_values().tap(|v| v.sort());
/// use_sorted_values(&tapped);
/// ```

pub trait Tap {
    /// Executes a closure on an object, returning it afterwards.
    ///
    /// # Examples
    ///
    /// ```
    /// use tapir::Tap;
    ///
    /// let mut max = 0;
    /// let data: [u32; 5] = [2, 8, 3, 4, 0];
    /// assert_eq!(
    ///     data.tap(|x| x.sort()).tap(|x| max += x.last().unwrap()),
    ///     [0, 2, 3, 4, 8]
    /// );
    /// assert_eq!(max, 8);
    /// ```
    fn tap<F: FnOnce(&mut Self)>(self, f: F) -> Self;
}

impl<T> Tap for T {
    #[inline]
    fn tap<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }
}
