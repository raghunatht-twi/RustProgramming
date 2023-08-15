fn main() {

    const THREE_HOURS_IN_SECONDS: u32 = 3*60*60;
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    println!("The contstant is: {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 1;
    {
	let y = y*2;
        println!("The value of y in hte inner scope is: {y}");   

    }
    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("value of spaces is: {spaces}");  

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("Your guess is: {guess }");
    println!("Hello, world!");

    let sum = 5+10;
    let difference = 95.5 -4.3;
    let product = 4*30;
    let quotient = 56.7/32.2;
    let truncated: f32 = -5.0/3.0;
    let reminder = 43%5;

    println!("all the computations are sum:{sum},difference:{difference},multiplication:{product}, division:{quotient}, {reminder}, {truncated}");


}
