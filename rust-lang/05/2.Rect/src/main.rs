/* BEGIN */





fn main()
{
	let r = Rect {
		width: 5,
		height: 6
	};

	let s = Rect {
		width: 3,
		height: 6
	};
	
	let u = Rect::square(6);

	println!("r: {:#?}", r);
	println!("area: {}", r.area());
	println!("r.can_hold(&s): {}", r.can_hold(&s));
	println!("r.can_hold(&u): {}", r.can_hold(&u));
	println!("u.can_hold(&r): {}", r.can_hold(&r));

	return;
}





// structs
#[derive(Debug)]
struct Rect
{
	width:  i32,
	height: i32,
}

/* Может быть несколько блоков impl */
impl Rect
{
	fn area(&self) -> i32
	{
		return self.width * self.height;
	}

	fn can_hold(&self, oth: &Rect) -> bool
	{
		return self.width >= oth.width && self.height >= oth.height;
	}

	fn square(a: i32) -> Rect
	{
		return Rect { width: a, height: a };
	}
}





/* END */
