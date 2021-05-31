/* BEGIN */





fn main()
{
	let s = String::from("Hello, World!");

	print_string(s);
	// println!("String: {}", s);
	println!("");


	/* Клонирование объектов из кучи */
	let s = String::from("Other string");
	print_string(s.clone());
	println!("String: {}\n", s);


	/* Значения могут выходить за границы бока */
	let mut s = cstr();
	println!("String (1): {}", s);
	s = "Hello".to_string();
	println!("String (2): {}\n", s);


	/* Возвращение сразу нескольких значений */
	let (a, b) = two_values();
	println!("a: {}, b: {}", a, b);


	return;
}





// functions
/* Функция удаляет переданную ей строку */
fn print_string(string : String)
{
	println!("String: {}", string);
	return; // Переданная строка уничтожается здесь
}

fn cstr() -> String
{
	return String::from("A string");
}

fn two_values() -> (i32, i32)
{
	return (3, 5);
}





/* END */
