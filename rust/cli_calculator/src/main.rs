// Trying to only use Chapters 1-4, enums, Options/Results
// Using split_whitespace() due to manual indexing problems

fn main() {
    println!("~~~~~Calculator!~~~~~");
    println!("Expecting format: \"number operator number\"");
    println!("Operators are one of: + - * / %");
    loop {
        println!("Enter your operation below, or Q to quit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.to_lowercase() == "q" { break; }

        let temp = process(input);
        let (a, b, op) = match temp {
            Some((a, b, op)) => (a, b, op),
            None => { println!("Bad input, please try again"); continue }
        };

        let res = match op {
            Operator::Plus => a + b,
            Operator::Minus => a - b,
            Operator::Times => a * b,
            Operator::Div => a / b,
            Operator::Mod => a % b,
        };

        // Could use Format trait
        let op = match op {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Times => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
        };
        println!("Your input: {} {} {}", a, op, b);
        println!("Your result: {}", res);
    }
}

enum Operator {
    Plus,
    Minus,
    Times,
    Div,
    Mod,
}

fn process(input: &str) -> Option<(f64, f64, Operator)> {
    let mut res = (0f64, 0f64, Operator::Plus);

    let mut parts = input.split_whitespace();
    res.0 = match parts.next()?.parse() {
        Ok(n) => n,
        _ => return None
    };
    let op_str = match parts.next() {
        Some(s) => s,
        _ => return None
    };
    res.1 = match parts.next()?.parse() {
        Ok(n) => n,
        _ => return None
    };

    if parts.next().is_some() { return None; }

    res.2 = match op_str {
        "+" => Operator::Plus,
        "-" => Operator::Minus,
        "*" => Operator::Times,
        "/" => Operator::Div,
        "%" => Operator::Mod,
        _ => return None,
    };
    Some(res)
}


/*
CHATGPT: could use Display trait instead of converting back to "+", "-", etc &strs.
impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Times => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
        };
        write!(f, "{}", s)
    }
}
*/