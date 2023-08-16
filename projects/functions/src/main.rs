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
