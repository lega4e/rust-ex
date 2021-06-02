/* BEGIN */





fn main()
{
	/* Создание срезов */
	let s = String::from("Hello, World");

	let hello = &s[..5]; // указываются байты, а не символы
	let world = &s[7..];

	println!("s: {}", s);
	println!("hello: {}, world: {}", hello, world);


	/* Получение первого строка в строке */
	println!("s: {}", s);
	println!("first word: {}", first_word(&s));
	
	
	/* Получение среза массива */
	let ar = [1, 4, 6, 2, 0, 4, 5, 6];
	print!("Array: ");
	print_array(&ar);

	let bz = before_zero(&ar[..]);
	print!("Before zero: ");
	print_array(&bz);


	return;
}





// functions
fn first_word(s : &str) -> &str
{
	for (i, ch) in s.as_bytes().iter().enumerate()
	{
		if *ch == b' '
		{
			return &s[..i];
		}
	}

	return &s[..];
}

fn before_zero(ar : &[i32]) -> &[i32]
{
	for (i, val) in ar.iter().enumerate()
	{
		if *val == 0
		{
			return &ar[..i];
		}
	}

	return &ar[..];
}

fn print_array(ar : &[i32])
{
	if ar.is_empty() { return }

	print!("{}", ar[0]);
	for val in ar[1..].iter()
	{
		print!(", {}", val);
	}

	println!("");
	return;
}





/* END */
