// fn main() {
//     loop {
//         println!("Endless loop!!");
//     }
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{:?}!", number);

//         number = number - 1;
//     }

//     println!("tralalala");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     let mut index = 0;

//     while index < 5 {
//         println!("the value is {:?}", a[index]);

//         index += 1; // index = index + 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("The vale is {:?}", element);
//     }  
// }

fn main() {
    for number in (1..4).rev() {
        println!("{:?}!", number);
    }
    println!("LIFTOFF !!");
}

