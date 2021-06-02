/* BEGIN */

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;





fn main() -> Result<(), Box<dyn Error>>
{
	File::open("input").unwrap();

	let x = double(4).unwrap();
	println!("x: {}", x);

	let val = read_file()?;
	println!("val: {}", val);

	Ok(())
}





// functions
fn double(x: i32) -> Result<i32, String>
{
	return 
		if x < 0 { Result::Err("Value less then zero".to_string()) }
		else     { Result::Ok(x*2)                                 }
}

fn read_file() -> Result<String, io::Error>
{
	let mut f = File::open("input")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}

fn read_file2() -> Result<String, io::Error>
{
	let mut s = String::new();
	File::open("input")?.read_to_string(&mut s)?;
	Ok(s)
}





/* END */
