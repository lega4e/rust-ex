/* BEGIN */

const NUMBER_COUNT: usize = 100;





fn main()
{
	let mut fib: [i128; NUMBER_COUNT] = [ 0; NUMBER_COUNT ];
	fib[1] = 1;

	for i in (2..NUMBER_COUNT).into_iter()
	{
		fib[i] = fib[i-1] + fib[i-2];
	}

	for i in (0..NUMBER_COUNT).into_iter()
	{
		println!("Fib {} = {}", i+1, fib[i]);
	}

	return;
}





/* END */
