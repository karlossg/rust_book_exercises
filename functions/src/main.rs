// fn main() { 
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// fn main() {
//     new_function(5, 12);
// }

// fn new_function(a: i32, b: i32) {
//     println!("The value of a is {:?} and the value of b is {:?}", a, b);
// }

// fn main() {
//     let _x = 5; 

//     let y = {
//         let x = 3;
//         x + 1 
//     };

//     println!("the value of y is {:?}", y);
// }

// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is {:?}", x);
// }

fn main() {
    let x = plus_one(5);

    println!("The value of x is {:?}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}