use crate::*;


macro_rules! assert_not
{
	($assertion:expr) => { assert!( !($assertion) ) };
}


mod smol
{
   use super::*;

	enum Smol {
		NANO,
		PICO,
	}

	#[test] fn positive()
	{
		assert!( eq_discriminant(Smol::NANO, Smol::NANO) );
		assert!( eq_discriminant(Smol::PICO, Smol::PICO) );
		
		assert!( eq_discr!(Smol::NANO, Smol::NANO) );
		assert!( eq_discr!(Smol::PICO, Smol::PICO) );
	
		assert!( eq_discr!(Smol::NANO, Smol::NANO, Smol::NANO) );
		assert!( eq_discr!(Smol::PICO, Smol::PICO, Smol::PICO) );
	}

	#[test] fn negative()
	{
		assert_not!( eq_discriminant(Smol::PICO, Smol::NANO) );
		assert_not!( eq_discriminant(Smol::NANO, Smol::PICO) );

		assert_not!( eq_discr!(Smol::PICO, Smol::NANO) );
		assert_not!( eq_discr!(Smol::NANO, Smol::PICO) );
		
		assert_not!( eq_discr!(Smol::PICO, Smol::PICO, Smol::NANO) );
		assert_not!( eq_discr!(Smol::PICO, Smol::NANO, Smol::NANO) );
		assert_not!( eq_discr!(Smol::PICO, Smol::NANO, Smol::PICO) );
		assert_not!( eq_discr!(Smol::NANO, Smol::NANO, Smol::PICO) );
		assert_not!( eq_discr!(Smol::NANO, Smol::PICO, Smol::PICO) );
		assert_not!( eq_discr!(Smol::NANO, Smol::PICO, Smol::NANO) );
	}
}
