/* BEGIN */





fn main()
{
	let str1 = "abcd".to_string();
	{
		let str2 = "xyz".to_string();
		let s = longest(&str1, &str2);
		println!("Longest: {}", s);
	}

	let s = S { reference: &str1 };
	println!("s: {}", s.reference);

	return;
}





// structs
struct S<'a>
{
	reference: &'a str,
}





// functions
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
{
	if a.len() > b.len() { a } else { b }
}

fn print_static_string(s: &'static str)
{
	println!("static string: {}", s);
}





/* END */
