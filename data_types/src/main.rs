use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // We need to specify the type u32 otherwise the code won't compile.

    println!("guess: {}", guess);

    // Scalar Types: A scalar type represents a single value.

    // Rust has four primary scalar types:

    // - integers
    // - floating-point numbers
    // - Booleans
    // - characters

    // ---------- INTEGER TYPES ---------- //

    // i => signed integer => can be negative.
    // u => unsigned integer => cannot be negative, unsigned like no -

    // Length	Signed	  Unsigned
    // 8-bit	 i8	        u8
    // 16-bit	 i16     	u16
    // 32-bit	 i32	    u32
    // 64-bit	 i64	    u64
    // 128-bit	 i128	    u128
    // arch	     isize	    usize

    // ---------- INTEGER LITTERALS ---------- //

    //    Number literals	            Example
    //        Decimal	                98_222
    //        Hex	                    0xff
    //        Octal	                    0o77
    //        Binary	                0b1111_0000
    //        Byte (u8 only)	        b'A'

    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // specifying f32

    println!("x: {}, y: {}", x, y);

    operations();

    booleans();

    characters();

    compound_types();
}

fn operations() {
    let addition = 5 + 10;

    let substraction = 95.5 - 4.3;

    let multiplication = 4 * 30;

    let division = 56.7 / 32.2;
    let truncated_division = -5 / 3;

    let remainder = 43 % 5;

    println!(
        "(operations fn) 

    addition: {}, 

    substraction: {}, 

    multiplication: {},

    division: {},

    truncated_division: {},

    remainder: {}",
        addition, substraction, multiplication, division, truncated_division, remainder
    );
}

fn booleans() {
    let t = true; // type recognized automatically

    let f: bool = false; // type declaration hard coded

    println!("t: {}, f: {}", t, f);
}

fn characters() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // We can use single quote for characters, double quotes for strings.
    // char type is four bytes in size and represents a Unicode Scalar Value.

    println!(
        "c: {}, 
         z: {}, 
         heart_eyed_cat: {}",
        c, z, heart_eyed_cat
    )
}

fn compound_types() {
    tuple_type();
    array_type();
    array_element_access();
}

fn tuple_type() {
    // tuple is a general way of grouping together a number of values with a variety of types into one coumpound type.
    // They have fixed length, once declared they cannot grow or shrink in size.

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup.0; // ---------------------------------------------- //

    let six_point_four = tup.1; // ----- ACCESS TO A TUPLE ELEMENT USING A PERIOD (.) FOLLOWED BY THE INDEX ----- //

    let one = tup.2; // ---------------------------------------------- //

    println!("{} {} {}", five_hundred, six_point_four, one);

    let (x, y, z) = tup; // destructuring

    println!("The value of x is {}, y is: {}, z is {}", x, y, z);

    let tup_dactylo: (&str, char) = ("hello", 'q');

    let (string, char) = tup_dactylo;

    println!("The value of string is {}, char is: {}", string, char);
}

fn array_type() {
    // arrays must have the same type and a fixed length.

    let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5] = [1, 2, 3, 4, 5]; => we can hard code the type [i32;_] then the length [_; len];
    let a_len = a.len();
    let first = a[0];
    let second = a[1];

    let b = [3; 5]; // => defining value of each element in array [x; _] and defining length [_; len] => will results in => [3, 3, 3, 3, 3].

    println!("{:?} {} {} {} {:?}", a, a_len, first, second, b)
}

fn array_element_access() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // we need a better error handler because if we provide an index > 5 the program will crash instead of allowinf the memory acces and continuing.

    let element = a[index];

    println!(
        "The value of the element at index: {} is: {}",
        index, element
    )
}
