/* BEGIN */

use std::*;
use std::error::Error;





// functions
pub fn run(cfg: Cfg) -> Result<(), Box<dyn Error>>
{
	let content = fs::read_to_string(cfg.fname)?;

	for line in search(&cfg.query, &content, cfg.icase)
	{
		println!("{}", line);
	}

	Ok(())
}

fn search<'a>(query: &str, content: &'a str, icase: bool) -> Vec<&'a str>
{
	let mut res = vec![];

	let query = if icase { query.to_lowercase() } else { query.to_string() };
	for line in content.lines()
	{
		if !icase && line.contains(&query) ||
			icase && line.to_lowercase().contains(&query)
		{
			res.push(line)
		}
	}

	res
}





// structs
pub struct Cfg
{
	query: String,
	fname: String,
	icase: bool // insensitive case
}

impl Cfg
{
	pub fn new(args: &[String]) -> Result<Cfg, &str>
	{
		if args.len() < 3
		{
			return Err("not enough arguments");
		}

		Ok( Cfg {
			query: args[1].clone(),
			fname: args[2].clone(),
			icase: env::var("CASE_INSENSITIVE").is_ok()
		} )
	}
}





#[cfg(test)]
mod test
{
	use super::*;

	const CONTENT: &str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
";


	#[test]
	fn several_results()
	{
		assert_eq!(
			vec![ "Then there's a pair of us - don't tell!" ],
			search("do", CONTENT, false)
		);

		assert_eq!(
			vec![
				"I'm nobody! Who are you?",
				"Are you nobody, too?",
				"How dreary to be somebody!"
			],
			search("body", CONTENT, false)
		);
	}


	#[test]
	fn case_sensetive()
	{
		assert_eq!(
			vec![
				"Then there's a pair of us - don't tell!",
				"They'd banish us, you know.",
			],
			search("The", CONTENT, false)
		);
	}


	#[test]
	fn case_insensetive()
	{
		assert_eq!(
			vec![
				"Then there's a pair of us - don't tell!",
				"They'd banish us, you know.",
				"To tell your name the livelong day",
			],
			search("The", CONTENT, true)
		);
	}
}





/* END */
