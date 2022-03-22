fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant THREE_HOURS_IN_SECONDS {}", THREE_HOURS_IN_SECONDS);

    // shadowing
    let z = 5;

    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {}", z);
    }

    println!("The value of z is: {}", z);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces {}", spaces);
}
