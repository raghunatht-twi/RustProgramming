// Explore branching strategies with control statements

fn main() {
    
    let condition = true;

    let number = if condition {5} else {6};
    println!("The value of the number is: {number}");
    
    if number < 5 {
        println!("The condition was true");
    } else {
        println!("The condition was false");
    }

    //try conditions
    if number != 0 {
        println!("The number was something other than zero");
    }

    //exploring multiple conditions
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 or 2");
    }

}
