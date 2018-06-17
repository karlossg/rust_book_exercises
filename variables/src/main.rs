// fn main() {
//     let x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

//constants may be set only to a constant expression, 
//not the result of a function call or any other 
//value that could only be computed at runtime.

// const MAX_POINTS: u32 = 100_000;

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     let x = x * 2;

//     println!("The value of x is: {}", x);
// }

fn main() {
    let spaces = "    ";
    let spaces = spaces.len();

    println!("Spaces length: {}", spaces);
}