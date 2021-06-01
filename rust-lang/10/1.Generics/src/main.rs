/* BEGIN */

use std::cmp::PartialOrd;
// use std::fmt::Display;
// use std::ops::*;





fn main()
{
	/* Numbers */
	println!("Numbers:");
	let mut numbers = vec![5, 3, 15, 7, 10];

	let (i, max) = largest(&numbers);
	println!("Max ({}): {}", i, *max);

	numbers[i] = -4;
	let (i, max) = largest(&numbers);
	println!("Max ({}): {}\n", i, *max);
	
	
	/* Chars */
	println!("Chars:");
	let mut chars = vec!['a', 'e', 'h', 'b', 'w', 'c'];

	let (i, max) = largest(&chars);
	println!("Max ({}): {}", i, *max);

	chars[i] = '\0';
	let (i, max) = largest(&chars);
	println!("Max ({}): {}", i, *max);
	
	
	/* Points */
	// let a = Point { x: 4, y: 3  };
	// let b = Point { x: 9, y: -4 };
	// let c = Point { x: 0, y: 10 };


	return;
}





// functions
fn largest<T: PartialOrd + Copy>(seq: &[T]) -> (usize, &T)
{
	let mut max      = &seq[0];
	let mut n: usize = 0;

	for (i, val) in seq.iter().enumerate()
	{
		if *val > *max
		{
			max = val;
			n   = i;
		}
	}

	return (n, max); 
}





// structs
// struct Point<T: Display>
// {
	// x: T,
	// y: T,
// }

// impl<T:
	// Display +
	// Mul<Output=T> +
	// Add<Output=T> +
	// Sub<Output=T>
// > Point<T>
// {
	// fn len(&self) -> f32
	// {
		// (self.x * self.x + self.y * self.y).sqrt()
	// }

	// fn dis(&self, othr: &Point<T>) -> f32
	// {
		// Point {
			// x: othr.x - self.x,
			// y: othr.y - self.y,
		// }.len()
	// }
// }





/* END */
