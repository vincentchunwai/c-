use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let mut num: u32 = 0;
    let mut is_outOfBound: bool = false;
    match a.get(index) {
        Some(value) => {
            println!("Value: {}", value);
            let mut num: u32 = *value;
            println!("The value of num is {num}");
        }
        None => {
            is_outOfBound = true; 
            println!("Index out of bound")
        },
    }
    if is_outOfBound {
        println!("The value of the element at index {index} is : undefined");
    } else {
        println!("The value of the element at index {index} is: {num}");
    }
}
