# Changelog


<br>


## v1.1.0

- Narrow methods to require borrowed `&T` and reject `T`:

  ```rs
  // before
	eq_discr!(left, right)

	// now
	eq_discr!(&left, &right)
  ```
  
  - This fixes type issues with `(&mut T, &T)`.
  - Passing `(T, T)` is no longer allowed.
  - Passing `(&T, &T)` is unchanged.


<br>


## v1.0.1

- Fix `eq_discr!()` so that it doesn’t require `eq_discriminant::eq_discriminant()` to be in scope.


<br>


## v1.0.0

Initial release on [crates.io](https://crates.io/crates/eq-discriminant)!

Included items:

- `eq_discriminant()`
- `neq_discriminant()`
- `eq_discr!()`
