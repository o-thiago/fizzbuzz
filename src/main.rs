use std::fmt::Display;

use strum_macros::IntoStaticStr;

#[derive(IntoStaticStr)]
enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Other(i32),
}

impl FizzBuzz {
    const FIZZ: i32 = 3;
    const BUZZ: i32 = 5;
}

impl From<&FizzBuzz> for String {
    fn from(val: &FizzBuzz) -> Self {
        match val {
            FizzBuzz::Other(n) => n.to_string(),
            v => Into::<&'static str>::into(v).to_string(),
        }
    }
}

impl Display for FizzBuzz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self))
    }
}

impl From<i32> for FizzBuzz {
    fn from(val: i32) -> Self {
        match val {
            v if v % (Self::FIZZ * Self::BUZZ) == 0 => Self::FizzBuzz,
            v if v % Self::FIZZ == 0 => Self::Fizz,
            v if v % Self::BUZZ == 0 => Self::Buzz,
            v => Self::Other(v),
        }
    }
}

fn main() {
    for value in (1..101).map(Into::<FizzBuzz>::into) {
        println!("{}", value);
    }
}
