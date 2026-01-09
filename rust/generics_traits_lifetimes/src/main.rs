struct Container<'a, T> {
    member: &'a T,
}

impl<'a, T> Container<'a, T> {
    fn get_ref_implicit(&self) -> &T {
        self.member
    }
    fn get_ref_explicit(&'a self) -> &'a T {
        self.member
    }
}

trait Describable {
    fn describe(&self) -> String;
}

use std::fmt::Display;
impl<'a, T: Display> Describable for Container<'a, T> {
    fn describe(&self) -> String {
        format!("Description for this object: {}", self.member)
    }
}

impl Describable for i32 {
    fn describe(&self) -> String {
        format!("Description for this i32: {}", self)
    }
}

impl Describable for String {
    fn describe(&self) -> String {
        format!("Description for this String: {}", self)
    }
}

fn print_description<T: Describable>(item: &T) -> () {
    println!("{}", item.describe());
}

fn main() {
    let member1 = "member-of-container1";
    let container1 = Container { member: &member1 };

    let member2 = 7;
    let container2 = Container { member: &member2 };

    let member3 = "third-member-for-third-container".to_string();
    let container3 = Container { member: &member3 };

    print_description(&container1);
    print_description(&container2);
    print_description(&member2);
    print_description(&String::from("test string"));
    print_description(&container3);
    print_description(&member3);


    println!("Hello, world!");
}

enum MaybeValue<T> {
    Just(T),
    Nothing,
}

impl<T: Display> Describable for MaybeValue<T> {
    fn describe(&self) -> String {
        use MaybeValue::{Just, Nothing};
        match self {
            Just(t) => format!("something here: {}", t),
            Nothing => String::from("nothing here"),
        }
    }
}

fn longest_description<'a, T: Describable>
    (a: &'a T, b: &'a T) -> &'a T {
    if a.describe().len() > b.describe().len() {
        a
    } else {
        b
    }
}
