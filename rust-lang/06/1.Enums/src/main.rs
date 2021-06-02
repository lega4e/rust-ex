/* BEGIN */





fn main()
{
	/* Использование перечислений */
	let four = IpKind::V4;
	let six  = IpKind::V6;

	println!("four: {:?}", four);
	println!("six:  {:?}\n", six);

	let localhost = Ip {
		kind: IpKind::V4,
		addr: "127.0.0.1".to_string()
	};

	println!("localhost: {:#?}\n", localhost);


	/* Перечисления со значениями */
	let home = Ip2::V4(192, 28, 84, 39);
	let oth  = Ip2::V6("::1".to_string());

	println!("home: {:#?}", home);
	println!("oth:  {:#?}\n", oth);
	
	
	/* Более сложные перечисления */
	let quit = Action::Quit;
	let mv   = Action::Move { x: 3, y: 6 };

	quit.print();
	mv.print();
	println!("");
	
	
	/* Перечисление Option */
	let x = Some(5);
	let y = Some("String");
	let z = Some(6);
	// let t: Option<i32> = None;

	match x {
		None => println!("x is None"),
		Some(x) => match z {
			None => println!("z is None"),
			Some(z) => {
				println!("x + z: {}", x + z);
			}
		}
	};
	println!("x: {:?}, y: {:?}, z: {:?}", x, y, z);
	
	
	/* Перечисления для обычных типов, заполнитель */
	let x: u8 = 8;
	match x {
		1 => println!("one"),
		2 => println!("two"),
		3 => println!("three"),
		4 => println!("four"),
		5 => println!("five"),
		_ => println!("{}", x)
	};


	return;
}





// structs & enums
#[derive(Debug)]
enum IpKind
{
	V4, V6
}

#[derive(Debug)]
struct Ip
{
	kind: IpKind,
	addr: String
}

#[derive(Debug)]
enum Ip2
{
	V4(u8, u8, u8, u8),
	V6(String)
}

#[derive(Debug)]
enum Action
{
	Quit,
	Move { x: i32, y: i32 },
	// Write(String),
	// ChangeColor(u8, u8, u8),
}

impl Action
{
	fn print(&self)
	{
		println!("{:#?}", self);
	}
}

/*
 * #[derive(Debug)]
 * enum Coin
 * {
 *     One,
 *     Two,
 *     Five,
 *     Ten,
 *     Other(i32),
 *     None
 * }
 */





/* END */
