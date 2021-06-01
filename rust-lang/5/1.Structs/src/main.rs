/* BEGIN */





fn main()
{
	let user = User {
		name:   "Exis".to_string(),
		email:  "nove@gmail.com".to_string(),
		sign:   330,
		online: false
	};

	let mut user2 = build_user("Elvis", "el@x.com");
	user2.sign    = 884;
	user2.online  = true;

	let user3 = User {
		name: "NO USER".to_string(),
		email: user2.email.clone(),
		..user2
	};

	let user4 = build_users(
		"Home".to_string(),
		"y@y.ru".to_string()
	);


	print_user(&user);
	println!("");

	print_user(&user2);
	println!("");

	print_user(&user3);
	println!("");

	print_user(&user4);
	println!("");



	/* Кортежные структуры */
	let c = Color(183, 183, 0, 255);
	let x = Point2(3, 4);
	let y = Point2(45, 9);

	println!("c: {:#?}", c);
	println!("x: {:?}", x);
	println!("y: {:?}\n", y);



	return;
}





// structs
struct User
{
	name:   String,
	email:  String,
	sign:   u64,
	online: bool,
}

#[derive(Debug)]
struct Point2(i32, i32);

#[derive(Debug)]
struct Color(u8, u8, u8, u8);





// functions
fn build_user(name : &str, email : &str) -> User
{
	return User {
		email:  email.to_string(),
		name:   name.to_string(),
		sign:   666,
		online: false
	};
}

fn build_users(name : String, email : String) -> User
{
	return User {
		email, name,
		sign:   100,
		online: false
	};
}

fn print_user(u : &User)
{
	println!("name:   {}", u.name);
	println!("email:  {}", u.email);
	println!("sign:   {}", u.sign);
	println!("online: {}", u.online);
	return;
}





/* END */
