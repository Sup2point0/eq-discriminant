//! This crate provides utilities for comparing enum variants:
//! 
//! - [`eq_discriminant`]
//! - [`neq_discriminant`]
//! - [`eq_discr!`].
//! 
//! # Why?
//! 
//! `std::mem::discriminant(&left) == std::mem::discriminant(&right)` is too much text for me, and places `left` and `right` too far apart.
//! 
//! `discriminant(&left) == discriminant(&right)` is better, but `discriminant` is still a far too long function name.
//! 
//! `eq_discriminant(&left, &right)` places them nicely side-by-side.
//! 
//! `eq_discr!(&left, &right)` is even terser!
//! 
//! # Usage
//! 
//! ## [`eq_discriminant`]
//! 
//! ```
//! use eq_discriminant::eq_discriminant;
//! 
//! enum Ex {
//!    Left { data: usize },
//!    Right { data: char },
//! }
//! 
//! eq_discriminant(
//!    &Ex::Left { data: 0 },
//!    &Ex::Left { data: 1 },
//! );  // -> true
//! 
//! eq_discriminant(
//!    &Ex::Left { data: 0 },
//!    &Ex::Right { data: 'c' },
//! );  // -> false
//! ```
//! 
//! ## [`eq_discr!`]
//! 
//! Use this if `eq_discriminant` is still too much text for you, or if you want to compare more than 2 values at once.
//! 
//! ```
//! use eq_discriminant::eq_discr;
//! 
//! enum Ex {
//!    Left { data: usize },
//!    Right { data: char },
//! }
//! 
//! eq_discr!(
//!    &Ex::Left { data: 0 },
//!    &Ex::Left { data: 1 },
//!    &Ex::Left { data: 2 },
//! );  // -> true
//! 
//! eq_discr!(
//!    &Ex::Left { data: 0 },
//!    &Ex::Left { data: 1 },
//!    &Ex::Right { data: 'c' },
//! );  // -> false
//! ```


/// Return `true` if and only if `left` and `right` are the same enum variant of `T`.
/// 
/// This is syntax sugar for:
/// 
/// ```ignore
/// std::mem::discriminant(&left) == std::mem::discriminant(&right)
/// ```
/// 
/// So this is a simple wrapper around [`std::mem::discriminant`]. As such, it should only be called on `enum` types.
#[must_use]
pub fn eq_discriminant<T>(left: &T, right: &T) -> bool
{
		std::mem::discriminant(left)
	== std::mem::discriminant(right)
}

/// Return `true` if and only if `left` and `right` are different enum variants of `T`.
/// 
/// This is syntax sugar for:
/// 
/// ```ignore
/// std::mem::discriminant(&left) != std::mem::discriminant(&right)
/// ```
/// 
/// So this is a simple wrapper around [`std::mem::discriminant`]. As such, it should only be called on `enum` types.
#[must_use]
pub fn neq_discriminant<T>(left: &T, right: &T) -> bool
{
	std::mem::discriminant(left) != std::mem::discriminant(right)
}

/// `eq_discr!<T>(first: T, second: T, third: T, ...)`
/// 
/// Return `true` if and only if all the provided arguments are the same enum variant of `T`.
/// 
/// This is a syntax sugar wrapper around [`eq_discriminant`], so similarly should only be applied to `enum` types.
/// 
/// # Usage
/// 
/// ```rs
/// enum Ex { Left, Right }
/// 
/// eq_discr!(Ex::Left, Ex::Left,  Ex::Left);   // true
/// eq_discr!(Ex::Left, Ex::Left,  Ex::Right);  // false
/// eq_discr!(Ex::Left, Ex::Right, Ex::Right);  // false
/// ```
#[macro_export]
macro_rules! eq_discr
{
	($prot:expr, $deut:expr) =>
	{
		$crate::eq_discriminant($prot, $deut)
	};

	($prot:expr, $deut:expr, $($rest:expr)*) =>
	{
		$crate::eq_discriminant($prot, $deut) && eq_discr!($deut, $($rest)*)
	};
}
