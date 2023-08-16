//made some changes
fn main() {

    println!("Hello, world!");

    //call another function
    another_function(257489);
    
    //call print labeled measurment function
    print_labeled_measurement(46,'h');

    //statements and expressions
    let x = {
        let y = 2;
        y + 4
    }; 
    another_function(x);

    //call function with return value
    let x_fn_return = five_hundred();
    println!("The value of x_fn_return is: {x_fn_return}");

    //call plus one function
    let x_plus_one = plus_one(762);
    println!("The value of x_plus_one is:{x_plus_one}");

}

//define another_function below

fn another_function( x: i32){

    println!("I am in another function (as part of functions)");
    println!("The value of x: is {x}");

}

//define labeled measurements function
fn print_labeled_measurement(value: i32, unit_label: char){
    
    println!("The measurement is: {value}{unit_label}");

}

//define function with return value
fn five_hundred() -> i32 {
    500
}

//define funciton plus_one
fn plus_one (x: i32) -> i32{
   
    //do not use semicolon here as it needs to  evaluate to an expression
    x+1
}
