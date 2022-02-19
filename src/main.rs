mod core;
mod utils;

use crate::core::{setup};

fn main()  {
    // setup terminal
    setup().map_err(|err| println!("{:?}", err)).ok();
}

