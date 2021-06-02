/* BEGIN */





fn main()
{
    hello();
    let res = sum(3, 6);
    println!("Result is {}", res);

    let value = {
        let x = 13;
        fn double(val: i32) -> i32 { return val * 2; }
        double(x)
    };
    println!("Value is {}", value);

    println!("Five is {}", five());

    return;
}





// functions
fn hello()
{
    println!("Hello!");
    return;
}

fn sum(x: i32, y: i32) -> i32
{
    return x + y;
}

fn five() -> i32
{
    5
}





/* END */
