/* BEGIN */

use std::*;





fn main()
{
	


	return;
}





// functions
fn read_all_numbers<T: io::BufRead>(xin: &mut T) -> Vec<i32>
{
	let mut nums: Vec<i32> = vec![];

	loop
	{
		let mut bytes: Vec<u8> = vec![];
		match xin.read_until(b' ', &mut bytes) {
			Err(_) => break,
			Ok(val) => {
				if val == 0 {
					break;
				}

				nums.push(
					String::from_utf8(bytes).unwrap().
					trim().parse::<i32>().unwrap() * 2
				);
			}
		}
	}

	nums
}





#[cfg(test)]
mod test
{
	use super::*;

	#[test]
	fn only_spaces()
	{
		let input = "0 1 2 3 4";
		assert_eq!(
			vec![0, 2, 4, 6, 8],
			read_all_numbers(&mut io::BufReader::new(input.as_bytes()))
		);
	}

	#[test]
	fn several_numbers()
	{
		let input = "\
0 84
28
40
59 58 0 3 5 12
3

38
   38     -3	1
   		
-5  -44
";

		assert_eq!(
			vec![0, 168, 56, 80, 118, 116, 0, 6, 10, 24, 6, 76, 76, -6, 2, -10, -88],
			read_all_numbers(&mut io::BufReader::new(input.as_bytes()))
		);
	}
}





/* END */
