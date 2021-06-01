/* BEGIN */

use std::*;





fn main()
{
	let testc = i32r();

	for _ in (0..testc).into_iter()
	{
		let n = i32r();
		let mut a: i32;
		let mut line = String::new();
		io::stdin().read_line(&mut line).expect("");

		let mut min = 100;
		let mut count = 0;

		for word in line.split(' ')
		{
			a = word.trim().parse().expect("");

			if a == min
			{
				count += 1;
			}
			else if a < min
			{
				min = a;
				count = 1;
			}
		}

		println!("{}", n - count);
	}


	return;
}





// functions
fn i32r() -> i32
{
	let mut line = String::new();
	io::stdin().read_line(&mut line).
		expect("Can't read line");
	return line.trim().parse().
		expect("Can't parse number");
}





/* END */
