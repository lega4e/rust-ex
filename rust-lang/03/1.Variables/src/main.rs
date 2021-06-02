/* BEGIN */





/* Константы */
const PI:   f64 = 3.14159265358979;
const MAXC: i32 = 32;





fn main()
{
    /* Изменяемые переменные */
    let mut x = 5;
    println!("Value of variable x is {}", x);
    x += 5;
    println!("Value of variable x is {}", x);


    /* Вывод констант */
    println!("PI:   {}", PI);
    println!("MAXC: {}", MAXC);
    
    
    /* Затенение переменных */
    let y = 2;
    let y = y+1;
    let y = y*2;
    println!("Value of variable y is {}", y);
    
    
    
    /* Затенение переменных с изменение типа */
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Value of variable spaces is {}", spaces);


    return;
}





/* END */
