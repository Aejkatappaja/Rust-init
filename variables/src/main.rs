const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant declaration, we can do it outside fn and refer to it inside.

fn main() {
    let mut x = 5; // mut => mutable variable.

    // let x = 5 => sealed value.

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant declaration.

    println!("const THREE_HOURS_IN SECONDS : {}", THREE_HOURS_IN_SECONDS);

    println!("the value of x is {}", x);

    x = 6; // We can change the value of x because we declared x as a mutable variable on top of the fn.

    println!("the value of x is {}", x);

    shadowing(); // execute the shadowing function below.
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x + 2;
        println!(
            "(shadowing_func) The value of x in the inner scope is: {}",
            x
        )
    }

    println!("(shadowing_func) The value of x is : {}", x);

    let spaces = "    ";
    let spaces = spaces.len(); // When compiling => reading both spaces and return the 2nd as an operation to the first which is 4 (spaces.len()).

    // let mut spaces = "   ";
    // spaces = spaces.len();    // error on compiler because we're not allowed tu mutate a variable's type (&str != usize)!

    println!("(shadowing_func) spaces: {}", spaces);
}
