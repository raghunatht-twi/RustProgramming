// Exploring with loops

fn main() {
    let mut counter = 0;
    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter *2;
        }
    };
    println!("The result is {result}");

    //test with nested loops and loop labels
    let mut count =0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1;
	}
        count +=1;
    }
    println!("End count = {count}");

    //explore While loop
    let mut second_number = 3;
    while second_number !=0 {
        println!("Second NUMBER is {second_number}!");
        second_number -=1;
    }
    println!("LIFTOFF !!!");

    //Looping through a collection
    let arr = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value at index is: {}", arr[index]);
        index +=1;
    }
    
    //Looping through a collection, using a for loop
    for element in arr {
        println!("The value is:{}",element);
    }

    //Using a for loop for count down 
    for num_cntr in (1..4).rev() {
        println!("The number is {num_cntr}!");
    }
    println!("LIFTOFF !");

}
