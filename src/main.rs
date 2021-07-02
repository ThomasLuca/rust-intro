// Declare a function
fn main() {
    // Printing to terminal
    println!("Hello, world!");
    // Declaring variable
    let x = 10;
    println!("x is {}", x);
    // x = 20; ERROR: vars are immutable by default
    let mut y = 20; // mut declares as a mutable var
    y = 30;
    println!("y is {}", y);

    /**
     * !Integers
     */

    // Integers by default are i32 (singed 32bit value)
    let z: u8 = 4; // Declare unsigned 8bit value
    println!("z as u8: {}", z);

    /**
     * !Floats
     */
    // There are 2 floating-point types: f32 and f64
    let a = 10.1545744486846521;
    println!("A is a 64bit float: {}", a);
    let b: f32 = 10.1545744486846521;
    println!("B is a 32bit float {}", b);

    /**
     * !Arithmetic operations
     */
    // Rust can't divide two different datatype, one datatype needs to be casted
    let c = 10;
    let d = 3.0;
    let e = c as f64 / d ;
    println!("Result of division: {}", e);
    // 3 as f64: 3.0
    // 3.9 as i32: 3
    // 300 as u8: 44
    // -300 as u32: 4294966996

    /**
     * !Formatting print statements
     */
    println!("Result of division: {0:.3} or {0:08.3}", e); // Specify amount of decimals

    /**
     * !Bitwise operations
     */

    let value = 0b1111_0101u8;
    println!("value is {}", value);
    println!("value as binary is {:08b}", value);
    // ! = not
    // & = and
    // | = or
    // ^ = xor
    // << / >> = shift left / right

    /**
     * !Booleans
     */
    let f = true;
    let g = false;
    println!("&: {}, |: {},  ^: {}, !: {}", f & g, f | g, f ^ g, !f);

    /**
     * !Chars
     */
    let letter = 'a';
    let number = '5';
    let finger = '\u{261D}';
    println!("{}\n{}\n{}", letter, number, finger);

    /**
     * !Challenge
     */
    let h = 13;
    let i = 2.3;
    let c: f32 = 120.0;

    let average = (h as f64 + i + c as f64) / 3.0; 
    assert_eq!(average, 45.1);
    println!("Test passed");

    /**
     * !Arrays
     */
    // Elements have same datatype
    // Array has a fixed length
    let mut letters = ['a', 'b', 'c'];
    println!("first letter {}", letters[0]);
    letters[0] = 'x';
    println!("New first letter {}", letters[0]);

    // Creates empty array
    let numbers: [i32; 5];
    numbers = [1; 5];
    let index: usize = numbers.len();
    println!("{}", numbers[index-1]);

}
