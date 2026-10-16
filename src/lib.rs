/// Return `true` if and only if `left` and `right` are the same enum variant of `T`.
/// 
/// This function should only be called on `enum` types.
#[must_use]
pub fn eq_discriminant<T>(left: T, right: T) -> bool
{
	std::mem::discriminant(&left) == std::mem::discriminant(&right)
}

/// Return `true` if and only if all the provided arguments are the same enum variant of `T`.
/// 
/// This macro should only be applied to `enum` types.
#[macro_export]
macro_rules! eq_discr
{
	($prot:expr, $deut:expr) =>
	{
		eq_discriminant($prot, $deut)
	};

	($prot:expr, $deut:expr, $($rest:expr)*) =>
	{
		eq_discriminant($prot, $deut) && eq_discr!($deut, $($rest)*)
	};
}
