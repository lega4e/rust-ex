/* BEGIN */





fn main()
{
	let art = Article {
		headline: "HEADLINE".to_string(),
		location: "LOCATION".to_string(),
		author:   "AUTHOR".to_string(),
		content:  "A CONTENT".to_string(),
	};

	let tweet = Tweet {
		username: "USERNAME".to_string(),
		content: "HI!".to_string(),
		reply:   false,
		retweet: false,
	};

	print_summary(&art);
	print_summary(&tweet);

	return;
}





// FUNCTIONS
fn print_summary(obj: &impl Summary)
{
	println!("{}", obj.summarize());
}





// STRUCTS
trait Summary
{
	fn summarize_author(&self) -> String;

	fn summarize(&self) -> String
	{
		format!("(Read more... @{})", self.summarize_author())
	}
}



/* Article */
struct Article
{
	headline: String,
	location: String,
	author:   String,
	content:  String,
}

impl Summary for Article
{
	fn summarize_author(&self) -> String
	{
		self.author.clone()
	}

	fn summarize(&self) -> String
	{
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}



/* Tweet */
struct Tweet
{
	username: String,
	content:  String,
	reply:    bool,
	retweet:  bool,
}

impl Summary for Tweet
{
	fn summarize_author(&self) -> String
	{
		self.username.clone()
	}

	fn summarize(&self) -> String
	{
		format!("{}: {}", self.username, self.content)
	}
}





/* END */
