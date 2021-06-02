/* BEGIN */

use std::*;
use std::error::Error;





// functions
pub fn run(cfg: Cfg) -> Result<(), Box<dyn Error>>
{
	let content = fs::read_to_string(cfg.fname)?;
	println!("content:\n{}", content);

	Ok(())
}





// structs
pub struct Cfg
{
	query: String,
	fname: String,
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
			fname: args[2].clone()
		} )
	}
}





/* END */
