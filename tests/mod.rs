#![allow(unused)]

use eq_discriminant::{
	eq_discriminant,
	neq_discriminant,
	eq_discr,
};


macro_rules! assert_not
{
	($assertion:expr) => { assert!( !($assertion) ) };
}


mod smol
{
   use super::*;

	enum S {
		NANO,
		PICO,
	}

	#[test] fn same()
	{
		assert!( eq_discriminant(&S::NANO, &S::NANO) );
		assert!( eq_discriminant(&S::PICO, &S::PICO) );

		assert_not!( neq_discriminant(&S::NANO, &S::NANO) );
		assert_not!( neq_discriminant(&S::PICO, &S::PICO) );
		
		assert!( eq_discr!(&S::NANO, &S::NANO) );
		assert!( eq_discr!(&S::PICO, &S::PICO) );
	
		assert!( eq_discr!(&S::NANO, &S::NANO, &S::NANO) );
		assert!( eq_discr!(&S::PICO, &S::PICO, &S::PICO) );
	}

	#[test] fn different()
	{
		assert_not!( eq_discriminant(&S::PICO, &S::NANO) );
		assert_not!( eq_discriminant(&S::NANO, &S::PICO) );

		assert!( neq_discriminant(&S::PICO, &S::NANO) );
		assert!( neq_discriminant(&S::NANO, &S::PICO) );

		assert_not!( eq_discr!(&S::PICO, &S::NANO) );
		assert_not!( eq_discr!(&S::NANO, &S::PICO) );
		
		assert_not!( eq_discr!(&S::PICO, &S::PICO, &S::NANO) );
		assert_not!( eq_discr!(&S::PICO, &S::NANO, &S::NANO) );
		assert_not!( eq_discr!(&S::PICO, &S::NANO, &S::PICO) );
		assert_not!( eq_discr!(&S::NANO, &S::NANO, &S::PICO) );
		assert_not!( eq_discr!(&S::NANO, &S::PICO, &S::PICO) );
		assert_not!( eq_discr!(&S::NANO, &S::PICO, &S::NANO) );
	}
}


mod references
{
	use super::*;

	enum T { LEFT, RIGHT }

	#[test] fn shared_shared()
	{
		assert!( eq_discriminant(&T::LEFT, &T::LEFT) );
		assert!( eq_discriminant(&T::RIGHT, &T::RIGHT) );
	}

	#[test] fn shared_unique()
	{
		assert!( eq_discriminant(&T::LEFT, &mut T::LEFT) );
		assert!( eq_discriminant(&T::RIGHT, &mut T::RIGHT) );

		fn testing(x: &T) {
			let mut xs = vec![];
			xs.pop_if(|s| eq_discriminant(s, x));
		}
	}

	#[test] fn unique_shared()
	{
		assert!( eq_discriminant(&mut T::LEFT, &T::LEFT) );
		assert!( eq_discriminant(&mut T::RIGHT, &T::RIGHT) );
	}

	#[test] fn unique_unique()
	{
		assert!( eq_discriminant(&mut T::LEFT, &mut T::LEFT) );
		assert!( eq_discriminant(&mut T::RIGHT, &mut T::RIGHT) );
	}
}
