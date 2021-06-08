/* BEGIN */

use std::*;





fn main()
{
	let v: Vec<i32> = vec![4, 9, 2, 6, 5];


	/* Проход итератором по циклу */
	for val in v.iter()
	{
		println!("{}", val);
	}
	println!("");


	/* Непосредственное использование итератора */
	let mut iter = v.iter();
	println!("{:?}", iter.next());
	println!("{:?}", iter.next());
	println!("{:?}", iter.next());
	println!("{:?}", iter.next());
	println!("{:?}", iter.next());
	println!("{:?}", iter.next());
	println!("");


	/* Суммирование */
	println!("sum: {}", v.iter().sum::<i32>());
	println!("");


	/* Отображение */
	let plusone: Vec<_> = v.iter().map(|x| *x + 1).collect();
	println!("newv: {:?}", plusone);
	println!("");


	/* Фильтрация */
	let gefour: Vec<_> = v.iter().filter(|x| **x > 4).collect(); 
	println!("gefour: {:?}", gefour);
	println!("");


	/* Использование собственного итератора */
	for i in NumIter::new(0, 5) {
		println!("{}", i);
	}
	println!("");


	/* Продвинутое использование собственных итераторов */
	let val: i32 = NumIter::new(0, 10).
		zip(NumIter::new(0, 10).skip(1)).
		map(|(a, b)| a * b).
		filter(|x| x % 3 == 0).
		sum();
	println!("val: {:?}", val);


	return;
}





// types
struct NumIter
{
	begin: i32,
	end:   i32,
}

impl NumIter
{
	fn new(begin: i32, end: i32) -> NumIter
	{
		NumIter { begin, end }
	}
}

impl Iterator for NumIter
{
	type Item = i32;

	fn next(&mut self) -> Option<i32>
	{
		if self.begin != self.end {
			self.begin += 1;
			return Some(self.begin-1);
		}
		None
	}
}





/* END */
