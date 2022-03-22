use std::io;

fn main() {
    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("guess {}", guess);

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (_, y, _) = tup;

    // println!("The value of y is: {}", y);
    // println!("at index = 0 is {}", tup.0);

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

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
