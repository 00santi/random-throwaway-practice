use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    println!("\nEnter Q at any point to Quit");
    println!("To add an employee: ADD [employee] TO [department]");
    println!("To list all employees: LIST ALL");
    println!("To list employees from a department: LIST [department]");
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let input = input("Enter input: ");
        let mut iterator = input.split_whitespace();

        let command = match iterator.next() {
            None => {
                println!("invalid input, try again");
                continue;
            }
            Some(command) => command.to_lowercase()
        };

        match command.as_str() {
            "q" => {
                println!("Exiting");
                break;
            }
            "add" => {
                let Some(employee) = iterator.next() else {
                    println!("expected employee name after \"add\", try again");
                    continue;
                };
                let Some(to) = iterator.next() else {
                    println!("invalid syntax, try again");
                    continue;
                };
                if to.to_lowercase() != "to" {
                    println!("invalid syntax, try again");
                    continue;
                }
                let Some(department) = iterator.next() else {
                    println!("expected department name for employee, try again");
                    continue;
                };
                map.entry(department.to_string()).or_insert(vec![]).push(employee.to_string());
                println!("Added {} to department {}", employee, department);
            },
            "list" => {
                let Some(target) = iterator.next() else {
                    println!("expected target to list, try again");
                    continue;
                };
                match target.to_lowercase().as_str() {
                    "all" => {
                        let mut dpts = vec![];
                        for dpt in map.keys() {
                            dpts.push(dpt);
                        }
                        dpts.sort();
                        for dpt in dpts.iter() {
                            println!("[{}]: ", dpt);
                            if let Some(empls) = map.get(*dpt) {
                                for empl in empls {
                                    println!("{}", empl);
                                }
                            }
                        }
                    },
                    _ => {
                        let Some(empls) = map.get(target) else {
                            println!("invalid target, try again");
                            continue;
                        };
                        println!("[{}]: ", target);
                        for empl in empls {
                            println!("{}", empl);
                        }
                    },
                };
            },
            _ => {
                println!("invalid command, try again");
                continue;
            }
        }
    }
}

fn input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}