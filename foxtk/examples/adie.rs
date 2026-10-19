#![forbid(unsafe_code)]

#[path = "components/adie.rs"]
mod adie;

use foxtk::prelude::Component;

fn main() {
    adie::Adie::run("Adie", "foxtk", "Adie - Untitled", 800, 600);
}
