/* BEGIN */

use std::*;
use grep::*;





fn main()
{
	let args: Vec<String> = env::args().collect();
	let cfg = Cfg::new(&args).unwrap_or_else(|err| {
		eprintln!("Error: {}", err);
		process::exit(1);
	});

	if let Err(err) = run(cfg)
	{
		eprintln!("Error: {}", err);
		process::exit(1);
	}

	return;
}










/* END */
