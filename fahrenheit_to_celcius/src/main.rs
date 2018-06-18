use std::io;

fn main() {
    println!("Give me fahrenheit value to convert");

    let mut user_fahrenheit_string = String::new();

    io::stdin().read_line(&mut user_fahrenheit_string)
        .expect("failed to read line");

    let user_fahrenheit_string: i32 = user_fahrenheit_string.trim().parse()
        .expect("Please type a number!");
    
    calculate_celsius(user_fahrenheit_string);
}

fn calculate_celsius( fahrenheit: i32 ) -> i32 {
    let celsius = ( fahrenheit - 32 ) * 5 / 9;
    println!("The value of celsius is {:?}", celsius);
    celsius
}
