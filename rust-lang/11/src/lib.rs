/* BEGIN */





pub struct Rect
{
	w: u32,
	h: u32
}

impl Rect
{
	pub fn can_hold(&self, other: &Rect) -> bool
	{
		self.w > other.w && self.h > other.h
	}
}



pub fn add_two(x: i32) -> i32
{
	x + 2
}





#[cfg(test)]
mod tests
{
	use super::*;

    #[test]
    fn it_works()
	{
        assert_eq!(2 + 2, 4);
    }

	#[test]
	fn another()
	{
		// panic!("ERROR!!! A-AAA-AAAAA");
	}

	#[test]
	fn larger_can_hold_smaller()
	{
		let larger  = Rect { w: 8, h: 9 };
		let smaller = Rect { w: 7, h: 1 };

		assert!(larger.can_hold(&smaller));
		return;
	}

	#[test]
	fn smaller_cannot_hold_larger()
	{
		let larger  = Rect { w: 8, h: 9 };
		let smaller = Rect { w: 7, h: 1 };

		assert!(!smaller.can_hold(&larger));
		return;
	}

	#[test]
	fn test_add_two()
	{
		let (x, y, z) = (2, 5, -4);

		assert_eq!(add_two(x), 4);
		assert_eq!(add_two(y), 7);
		assert_eq!(add_two(z), -2);
		return;
	}

	#[test]
	#[should_panic]
	fn should_panic()
	{
		panic!("AAAAAAAAAAAAAAAAA");
	}

	#[test]
	fn test_by_result() -> Result<(), &'static str>
	{
		if 2 + 2 == 4 { Ok(()) } else { 
			Err("Something wrong")
		}
	}
}





/* END */
