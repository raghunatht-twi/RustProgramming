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

    println!("Hello, world!");
}
