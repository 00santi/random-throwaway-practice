use std::io::Write;

fn print_choices() {
    println!("Type 1 to convert from Celsius to Fahrenheit");
    println!("Type 2 to convert from Fahrenheit to Celsius");
    println!("Type anything else to exit");
}

enum Choice {
    CelsiusToFahrenheit,
    FahrenheitToCelsius
}

fn main() {
    println!("\n~~~~~Temperature Converter~~~~~");
    println!("What type of conversion do you want to do?");

    loop {
        print_choices();

        let choice = input("Input choice: ");
        let choice = match choice.trim() {
            "1" => Choice::CelsiusToFahrenheit,
            "2" => Choice::FahrenheitToCelsius,
            _ => break
        };

        let temp = input("Type the temperature you want to convert: ");
        let temp: f32 = match temp.trim().parse() {
            Ok(t) => t,
            Err(_) => {
                println!("Invalid number, try again");
                continue;
            }
        };

        match choice {
            Choice::CelsiusToFahrenheit => {
                println!("Your temperature: {}", temp);
                println!("Your temperature in Fahrenheit: {}", c_to_f(temp));
            }
            Choice::FahrenheitToCelsius => {
                println!("Your temperature: {}", temp);
                println!("Your temperature in Celsius: {}", f_to_c(temp));
            }
        };
        println!();
    }
}

fn input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn c_to_f(temp: f32) -> f32 {
    32.0 + (1.8 * temp)
}

fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0) / 1.8
}