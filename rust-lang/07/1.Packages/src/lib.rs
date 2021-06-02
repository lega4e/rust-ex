mod front_of_house;

mod back_of_house
{
	pub struct Breakfast
	{
		pub toast: String,      // Является публичным
		seasonal_fruit: String, // Не является публичным
	}

	impl Breakfast
	{
		pub fn summer(toast: &str) -> Breakfast
		{
			return Breakfast {
				toast: toast.to_string(),
				seasonal_fruit: "peaches".to_string()
			};
		}
	}


	pub enum Appetizer
	{
		Soup, // По умолчанию поля публичные
		Salad // Тоже публичное поле
	}

	fn fix_incorrenct_order()
	{
		cook_order();
		super::serve_order();
	}

	fn cook_order() {}
}

fn serve_order() {}


/* Use, чтобы внешний (не в этом файле) код мог использовать hosting */
pub use front_of_house::hosting;

pub fn eat_at_restaurant()
{
	// abs path
	crate::front_of_house::hosting::add_to_waitlist();

	// rel path
	front_of_house::hosting::seat_at_table();

	let mut meal = back_of_house::Breakfast::summer("Rye");
	meal.toast = "Wheat".to_string();
	println!("I'd like {} toast, please", meal.toast);


	/* Использование use */
	use front_of_house::hosting::*;

	add_to_waitlist();
	seat_at_table();


	/* Подключение с псевдонимом */
	use front_of_house::hosting::add_to_waitlist as atw;
	use front_of_house::hosting::seat_at_table as sat;

	atw();
	sat();


	return;
}





/* END */
