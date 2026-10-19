#![forbid(unsafe_code)]

#[path = "components/calculator.rs"]
mod calculator;

use foxtk::prelude::Component;

fn main() {
    calculator::Calc::run("FOX Calculator", "foxtk", "FOX Calculator", 320, 420);
}
