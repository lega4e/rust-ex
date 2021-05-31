/* BEGIN */

use std::io;





fn main()
{
	let val = inp_i32();


	/* Условные операторы */
	if val < 5
	{
		println!("Val less then 5");
	}
	else if val > 5
	{
		println!("Val greater then 5");
	}
	else
	{
		println!("Val equal 5");
	}


	/* Тернарный оператор */
	let val = if val < 5 { -val } else { val * 2 };
	println!("New value is {}\n", val);



	/* Цикл loop */
	let mut i = 0;
	loop
	{
		println!("Loop, i = {}", i);
		i += 1;

		if i == 5 { break; }
	}
	println!("");


	/* Цикл white */
	let mut i = 0;
	while i < 5
	{
		println!("While, i = {}", i);
		i += 1;
	}
	println!("");


	/* Цикл for */
	for i in (0..5).into_iter()
	{
		println!("For, i = {}", i);
	}
	println!("");

	for i in (0..5).rev()
	{
		println!("For (rev), i = {}", i);
	}
	println!("");

	let array = [ 3, -4, 5, -6 ];
	for el in array.iter()
	{
		println!("For (array), el = {}", el);
	}
	println!("");


	return;
}





// functions
fn inp_i32() -> i32
{
	return loop {
		let mut input = String::new();
		io::stdin().read_line(&mut input).
			expect("Can't read line");

		match input.trim().parse() {
			Ok(num) => break num,
			Err(_)  => println!("Can't parse number")
		};
	};
}





/* END */
