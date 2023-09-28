#![no_std]

use fstd::Itself;
use fstd::fmt::Display;

use fstd::fmt::println;
use fstd::string::String;
use fstd::string::ToString;

fn main() {
    let user = User {
        name: "User11".to_string(),
        email: "user11@gmail.com".to_string(),
        age: 33,
    };
    let user = println(user.immutable()); // TODO
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: i32,
}

impl Display for User {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "User {{ {}, {}, {} }}", self.name, self.email, self.age)
    }
}