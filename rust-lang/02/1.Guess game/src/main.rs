/* BEGIN */
use std::cmp::Ordering;
use std::io;
use rand::Rng;





fn main()
{
    let number = rand::thread_rng().gen_range(1..101);

    // println!("Number is {}", number);
    println!("Отгадай число от 0 до 100!");


    loop
    {
        println!("Введи число:");
        let mut guess = String::new();

        io::stdin().
            read_line(&mut guess).
            expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("Что-то не то, давай ещё разок");
                continue;
            },
        };

        match guess.cmp(&number) {
            Ordering::Less    => println!("Маловато\n"),
            Ordering::Greater => println!("Многовато\n"),
            Ordering::Equal   => {
                println!("Ура!");
                break;
            }
        }
    }
}





/* END */
