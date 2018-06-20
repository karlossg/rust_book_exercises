use std::io;

fn main() {
    println!(
      "Please chose convert option:
       \n1) fahrenheit to celsius
       \n2) celsius to fahrenheit
       \nType 1 or 2 and press enter");

    let mut convert_option = String::new();

    io::stdin().read_line(&mut convert_option)
        .expect("failed to read line");

    let convert_option: u8 = convert_option.trim().parse()
        .expect("Please type 1 or 2!");
    
    if convert_option == 1 { 
        let user_fahrenheit_string = format_user_input();
        
        let celsius = calculate_celsius(user_fahrenheit_string);

        println!("The value of celsius is {:?}", celsius);
    } else {
        let user_celsius_string = format_user_input();

        let fahrenheit = calculate_fahrenheit(user_celsius_string);

        println!("The value of fahrenheit is {:?}", fahrenheit);

    }
}

fn format_user_input() -> i32 {
    println!("Give me value to convert");

    let mut user_string = String::new();
   
    io::stdin().read_line(&mut user_string)
        .expect("failed to read line");

    let user_string: i32 = user_string.trim().parse()
        .expect("Please type a number!");

    user_string
}

fn calculate_celsius( fahrenheit: i32 ) -> i32 {
    let celsius = ( fahrenheit - 32 ) * 5 / 9;
    celsius
}

fn calculate_fahrenheit( celsius: i32 ) -> i32 {
    let fahrenheit = celsius * 9 / 5 + 32;
    fahrenheit
}
