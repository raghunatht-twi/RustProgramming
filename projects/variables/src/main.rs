// import the std io library
use std::io;

fn main() {

    // variable assignment and printing

    const THREE_HOURS_IN_SECONDS: u32 = 3*60*60;
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    println!("The contstant is: {THREE_HOURS_IN_SECONDS}");

    //demonstrating the concept of shadowing

    let y = 5;
    let y = y + 1;
    {
	let y = y*2;
        println!("The value of y in hte inner scope is: {y}");   

    }
    println!("The value of y is: {y}");


    // about type casting in Rust

    let spaces = "   ";
    let spaces = spaces.len();
    println!("value of spaces is: {spaces}");  

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("Your guess is: {guess }");

    // The classic "Hello World", where it all begins :-)

    println!("Hello, world!");

    // Basic operations on variables

    let sum = 5+10;
    let difference = 95.5 -4.3;
    let product = 4*30;
    let quotient = 56.7/32.2;
    let truncated: f32 = -5.0/3.0;
    let reminder = 43%5;

    println!("all the computations are sum:{sum},difference:{difference},multiplication:{product}, division:{quotient}, {reminder}, {truncated}");

    // Exploring the compund types - Tuples and Arrays
    
    let tup: (i32, f64, u8) = (-500, 6.4, 1);
    
    //accessing tuples by destructuring
    let (_xt, yt, _zt) = tup;
    println!("The value of yt is: {yt}");

    //accessing tuples by indexing
    let temp = tup.0;
    println!("The value of xt is:{temp}");

    //array examples
    let arr = [1,2,3,4,5,6];
    let arr1: [i32; 5] = [2;5];
    println!("The arr is{:?} and arr1 is{:?}",arr,arr1);

    //use std io to generate array index out of bound exception
    let arr_demo = [12,14,18,26,42];
    println!("Please enter array index");

    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index was not a number");

    let element = arr_demo[index];
    println!( "The value of the element at index {index} is: {element}");

}
