/* BEGIN */





fn main()
{
	let mut x: Option<i32> = None;
	let mut y = Some(34);

	/* Match x */
	println!("match x");
	for _ in (0..2).into_iter()
	{
		match x {
			None => println!("x is None"),
			Some(x) => println!("x is {}", x)
		}
		x = Some(3);
	}
	println!("");


	/* If-let y */
	println!("if-let y");
	for _ in (0..2).into_iter()
	{
		if let Some(y) = y
		{
			println!("y is {}", y);
		}

		y = None;
	}
}





/* END */
