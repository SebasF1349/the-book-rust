#![allow(dead_code)]
#![allow(unused_variables)]

mod generics;
mod lifetimes;
mod traits;

use generics::generics;
use lifetimes::lifetimes;

fn main() {
    generics();
    lifetimes();
}
