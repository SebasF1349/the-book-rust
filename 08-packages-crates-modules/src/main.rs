#![allow(dead_code)]
#![allow(unused_variables)]

mod module;
mod module_in_folder;
mod module_with;

use module_with::submodule;
// "import" with an alias, although it not recommended to import the fn as then it will be harder to understand where is the fn when used
use module_with::submodule::call_mod as called_mod;

fn main() {
    // crate: the smallest amount of code that the Rust compiler considers at a time
    // crates can contain modules that may be defined in other files that get compiled with the crate
    // there are two types of crates:
        // binary crate: a program you can compile to an executable that you can run
            // it has a main function that defines what happens when the executable runs
        // library crate: define functionality intended to be shared with multiple proyects
            // doesn't have a main function and don't compile to an executable
            // usually just called a "crate"
    // crate root: a source file that the Rust compiler starts from and makes up the root module of the crate (if using Cargo, src/main.rs is the root in a binary crate and src/lib.rs in a library crate)
    // package: a bundle of one or more crates that provides a set of functionality
        // it contains a Cargo.toml file that describes how to build those crates
        // Cargo is a package that contains the binary crate for the command-line tool and also contains a library crate that the binary crate depends on
        // a package can contain as many binary crates as needed (at least one), but at most only one library crate

    // organization of a modules: check the file tree
    submodule::pr();

    //check module_with/submodule for the rest of the examples
}
