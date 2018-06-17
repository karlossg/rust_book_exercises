// TUPLE
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

//     let (x, _y, z) = tup;

//     println!("My first rust Tuple: {:?}", tup);

//     println!("Destructed tuple: x = {:?}, z = {:?} (y with _ = not used)", x, z);

//     let five_hundred = tup.0;

//     let six_point_four = tup.1;

//     let one = tup.2;

//     println!("Tuple dot notation: {}, {}, {}", five_hundred, six_point_four, one);
// }


// ARRAY - immutable length
fn main() {
    let arr = [1, 2, 3, 4, 5];

    let [a, b, _c, _d, _e] = arr;

    let index_from_arr = arr[3]; //4

    println!("First rust arr: {:?}", arr);
    
    println!("Arr destructuring? a = {:?}, b = {:?}", a, b);

    println!("Index 3: {:?}", index_from_arr);
}
