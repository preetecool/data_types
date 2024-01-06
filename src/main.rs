fn main() {
    // Scalar Types
    // A scalar type represent a single value. The four primary scalar types for rust are: integers, floating-point numbers, Booleans, and characters.

    // Integer Types
    // An integer is a number without fractional component. 
    
    //   Length	    Signed	Unsigned
        // 8-bit	i8	    u8
        // 16-bit	i16	    u16
        // 32-bit	i32	    u32
        // 64-bit	i64	    u64
        // 128-bit	i128	u128
        // arch	    isize	usize
    //Signed or Unsigned refer to wether the number can be negative(signed) or positive(unsigned).
    // An 8-bit signed variant can store  -(2^(n - 1)) to 2^(n - 1) - 1 where n is the number of bits that variant uses.
    // So an i8 can store numbers from -(2^7) to 2^7 - 1, which equals -128 to 127. 
    // Unsigned variants can store numbers from 0 to 2^n - 1, so a u8 can store numbers from 0 to 2^(8 - 1), which equals 0 to 255.
    // The default interger type in Rust is i32.


    //Floating-Point Types
    let var_1 = 2.0; // f64

    let var_2: f32 = 3.0; // f32


    // Numeric Operations
    
    // Addition
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1

    // Remainder
    let remainder = 43 % 5;


    // The Boolean Type

    let t = true;

    let f: bool = false; // with explicit type annotation

    // The Character Type
    

    let c = 'z';
    let z_char: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    // Compound Types
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    // Can access tuplpes by using a period. In the example below we are accessing all tuples in order
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // The Array Type
    // Arrays are useful when you want data allocated on the stack rather than the heap or when we want a fixed number of elements.
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    // The following array is of i32 type and has 5 elements.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // The following is an array of 5 elements all with the value 3.
    let a = [3; 5];
    // Out of bound access will cause a panic and runtime error when trying to access an element that is out of bounds.


    // The Vector Type
    // Vectors are similar to arrays but can grow and shrink in size.
    


}
