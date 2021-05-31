/* BEGIN */

/*
 * 1. Возможно множество неизменяемых ссылок
 * 2. Возможна только одна изменяемая ссылка
 * 3. Если существует изменяемая ссылка, то неизменяемых ссылок быть
 *    не может
 * 4. Область жизни ссылок — от их объявления до их последнего
 *    использования
 */





fn main()
{
	/* Передача по ссылке */
	let s = String::from("Hello, World!");
	let len = calculate_length(&s);

	println!("String: {}, len: {}\n", s, len);
	
	
	/* Изменение по ссылке */
	let mut s = String::from("Hello");
	println!("String: {}", s);

	add_world(&mut s);
	println!("String after: {}\n", s);


	/* Ссылка на примитив */
	let mut x = 2;
	println!("x: {}", x);

	double(&mut x);
	println!("x after: {}", x);
	

	return;
}





// functions
fn calculate_length(s : &String) -> usize
{
	return s.len();
}

fn add_world(s : &mut String)
{
	s.push_str(", World");
	return;
}

fn double(x : &mut i32)
{
	*x *= 2;
	return;
}





/* END */
