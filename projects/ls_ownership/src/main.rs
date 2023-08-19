//Some examples to understand ownership concepts

fn main() {
    let x = 5;
    let y = x;

    println!("The value of x is {x} and y is {y}");

    //variable s is not in scope here, as it's not yet defined
    {
        let s = "hello"; //variable s is initiated here and it gets in scope
        let t = "world";

        println!("We defined s and t as: {s}, {t}!");
    } // s is out of scope here
      //println!("value of s is {s}");
    println!("Hello, world!");

    let mut s = String::from("HELLO");

    s.push_str(", WORLD !!"); //push_str() appends a literal to a String

    println!("{s}"); //this will print the full String

    let s1 = String::from("Deep mind");
    println!("For the first time: {s1}");

    let s2 = s1.clone();
    println!("For the second time S1: {s1} and S2: {s2}");
}