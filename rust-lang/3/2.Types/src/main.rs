/* BEGIN */





/*
 * Целые числа
 *
 * Размер Знаковый Беззнаковый
 * 8 бит    i8       u8
 * 16 бит   i16      u16
 * 32 бита  i32      u32
 * 64 бита  i64      u64
 * 128 бит  i128     u128
 * arch     isize    usize
 *
 * 
 * Литералы целых чисел
 *
 * Числовые литералы        Пример
 * Десятичный          98_222
 * Шестнадцатеричный   0xff
 * Восьмеричный        0o77
 * Бинарный            0b1111_0000
 * Байтовый            (только u8) b'A'
 *
 *
 * Числа с плавающей точкой: f32, f64
 */





fn main()
{
    /* Целые числа */
    let x: i128 = 1_000_000_000_000_000_000;
    let y: i128 = 32;

    let xaddy: i128 = x + y;
    let xsuby: i128 = x - y;
    let xmuly: i128 = x * y;
    let xdivy: i128 = x / y;
    let xmody: i128 = x % y;

    println!("x and y have type i128");
    println!("x: {}, y: {}", x, y);
    println!("xaddy: {}", xaddy);
    println!("xsuby: {}", xsuby);
    println!("xmuly: {}", xmuly);
    println!("xdivy: {}", xdivy);
    println!("xmody: {}\n", xmody);
    
    
    
    /* Числа с плазающей точкой */
    let x: f64 = 3.5;
    let y: f64 = 0.5;

    let xaddy: f64 = x + y;
    let xsuby: f64 = x - y;
    let xmuly: f64 = x * y;
    let xdivy: f64 = x / y;
    let xmody: f64 = x % y;

    println!("x and y have type f64");
    println!("x: {}, y: {}", x, y);
    println!("xaddy: {}", xaddy);
    println!("xsuby: {}", xsuby);
    println!("xmuly: {}", xmuly);
    println!("xdivy: {}", xdivy);
    println!("xmody: {}\n", xmody);



    /* Прочие простые типы */
    let flag: bool = true;
    let flag2      = false;
    let ch:   char = 'c';
    let z          = 'ℤ';
    let cat        = '😻';

    println!("flag:  {}", flag);
    println!("flag2: {}", flag2);
    println!("ch:    {}", ch);
    println!("z:     {}", z);
    println!("cat:   {}\n", cat);
    


    
    
    /* Кортежи */
    let tup                 = (53, 3.4, 'c', false);
    let color: (u8, u8, u8) = (255, 0, 0);
    let mut mtup            = (1, 2, 3);

    mtup.0 += 1;
    mtup.1 += 1;
    mtup.2 += 1;

    println!("tup:   ({}, {}, {})", tup.0,   tup.1,   tup.2);
    println!("color: ({}, {}, {})", color.0, color.1, color.2);
    println!("mtup:  ({}, {}, {})", mtup.0,  mtup.1,  mtup.2);
    
    
    /* Массивы (в отличие от кортежей, элементы однотипны) */
    let array         = [1, 2, 3, 4];
    let aru8: [u8; 3] = [1, 2, 0]; // указывается тип и размер

    println!("array: [{}, {}, {}, {}]", array[0], array[1], array[2], array[3]);
    println!("aru8:  [{}, {}, {}]",     aru8[0],  aru8[1],  aru8[2]);


    return;
}





/* END */
