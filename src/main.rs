fn main() {
    let _guess: isize = "-42".parse().expect("Not a number!");

    let _x = 2.0; // f64

    let _y: f32 = -3.0;

    //addition
    let sum = 5 + 10;
    println!("Sum: {sum}");

    //subtraction
    let differnce = 95.5 - 4.3;
    println!("Difference {differnce}");

    //division
    let quotient = 56.7 / 32.2;
    println!("Quotient {quotient}");
    let truncated = -5 /3;
    println!("Quotient {truncated}");
    let non_truncated: f32 = -5 as f32 /3 as f32;
    println!("Quotient {non_truncated}");

    //remainder
    let remainder = 43 % 5;
    println!("Remainder {remainder}");

    //Boolean data type
    let t = true;
    let f: bool = false; //with explicit type annotation
    println!("bool values {}", t | !f); // or &

    //Character Type
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");

    //Compound Types
    // 1. The Tuple type
    let _unit = ();
    let tup: (i32, f64, u8) = (-500, 260.5, 1);
    println!("{0}, {2}, {1}",tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("{x}, {z}, {y}");

    //2. array type
    let a = [1,2,3,4,5];
    println!("{}", a[2]);
    // let _b = [1, "January"]; //causes mismatched types compile time error
    // let a: [i32; 5] = [1,2,3,4,5,6];//causes mismatched types compile time error as the size exceeds
    let _a: [i32; 5] = [1,2,3,4,5];
    let _b: [u32; 10] =  [260; 10]; //the entire array will have value as 260
}
