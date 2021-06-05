/* BEGIN */

use std::*;
use thread;
use time::Duration;





fn main()
{
	let simval  = 23;
	let randnum = 7;
	
	generate_workout(simval, randnum);

	let x = 3;
	let equal_x = |y| x == y;

	println!("{}", equal_x(3));
	println!("{}", equal_x(4));

	return;
}





// functions
struct Catcher<T>
where T: Fn(i32) -> i32
{
	calc: T,
	value: Option<i32>,
}

impl<T: Fn(i32) -> i32> Catcher<T>
{
	fn new(calc: T) -> Catcher<T>
	{
		Catcher {
			calc,
			value: None
		}
	}

	fn value(&mut self, arg: i32) -> i32
	{
		match self.value {
			Some(val) => val,
			None => {
				let value = (self.calc)(arg);
				self.value = Some(value);
				value
			}
		}
	}
}

fn generate_workout(intensity: i32, randnum: i32)
{
	let mut calc = Catcher::new(|arg| {
		println!("Calculation...");
		thread::sleep(Duration::from_secs(2));
		println!("Calculation finished!");
		arg * 2
	});

	if intensity < 25
	{
		println!("Today, do {} pushups!", calc.value(intensity));
		println!("Next, do {} situps!", calc.value(intensity));
	}
	else
	{
		if randnum == 3
		{
			println!("Take a break today! Remember to stay hydrated!");
		}
		else
		{
			println!("Today, run for {} minutes", calc.value(intensity));
		}
	}
}





/* END */
