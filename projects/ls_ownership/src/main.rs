//Some examples to understand ownership concepts

fn main() {
    let x = 5;
    let y = x;

    println!("The value of x is {x} and y is {y}");

    makes_copy(x); //makes copy of the integer

    println!("The value of x^2 is {}", x * x);

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

    let (s3, len) = calculate_length(s1);

    takes_ownership(s3.clone()); //s3's value moves to the function

    let s2 = s3.clone();
    println!(
        "For the second time S3,its length: {s3},{len} and S2, its length: {s2}, {}",
        calculate_length_only(&s2)
    );

    println!("Get value from fn gives_ownership: {}", gives_ownership());

    println!(
        "Take value and get back from fn: {}",
        take_and_give_back(s2)
    );

    //What happens when we try to modify something we do not own. Mutable reference works
    let mut st = String::from("what's your name");

    println!("Printing the st: {st}");
    change(&mut st);
    println!("Printing the st after the change: {st}");
}

//Function takes ownership implementation
fn takes_ownership(my_string: String) {
    //my_string comes into scope
    println!("My string value is: {my_string}");
}

//Function to make copy
fn makes_copy(my_integer: i32) {
    //my_integer comes into scope here
    println!("My integer value is: {my_integer}");
}

//Function to give ownership
fn gives_ownership() -> String {
    //gives ownership and retruns value to function
    let my_string = String::from("Yours truly");
    return my_string;
}

//Function to take a string and gives back a string
fn take_and_give_back(take_string: String) -> String {
    let mut modified_string = String::from(take_string);
    modified_string.push_str(" for deep work");
    return modified_string;
}

//Function to calculate length and return tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); //len() returns the length of a String
    return (s, length);
}

//Function to calculate length of string and return it
fn calculate_length_only(s: &String) -> usize {
    //s is a reference to a String and it does not take ownership

    return s.len();

    //Here s goes out of scope. But becuase it does not take ownership, the String is not dropped
}

fn change(a_string: &mut String) {
    //Function to change a string
    a_string.push_str(", hello");
}
