fn main() {
    // default release profiles
    /* [profile.dev]
    opt-level = 0
    [profile.release]
    opt-level = 3 */

    /// documentation comments use three slashes, and they will generate HTML documentation
    /// they support markdown and should be placed just before the item they are documenting
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
    ///
    /// `cargo doc` will generate the HTML documentation
    /// `cargo doc --open` will generate and open the documentation
    ///
    /// Commonly used sections
    /// # Examples
    /// # Panics
    /// scenarios in which the function the function could panic
    /// # Errors
    /// if the fn returns a `Result`, describe the kind of errores and when
    /// # Safety
    /// if the fn is `unsafe`

    pub fn test() {}

    // code blocks in the documentation will run with `cargo test` as if they are test,
    // helping the docs to be up-to-date

    // This type of comments (//!) are used to add documentation to the item that contains the comments rather than to the items following, these are useful to describe crates and modules

    // to expose nested code (fns, enums, etc) to a more user friendly mod path, we can re-export the types to the top level with `pub use`

    // to publish a crate you need to create an account on https://crates.io/

    // Cargp.toml needs to have, at least, this info
    /*[package]
    name = "guessing_game" <needs to be unique>
    version = "0.1.0"
    edition = "2021"
    description = "A fun game where you guess what number the computer has chosen."
    license = "MIT OR Apache-2.0"
    <more metadata can be found in https://doc.rust-lang.org/cargo/>
    [dependencies] */

    // run `cargo publish` to publish the crate

    // to deprecate a Crate version run `cargo yank --vers <version_number>` (this does not remove any code but makes the crate impossible to install)

    // a workspace is a set of packages that share the same Cargo.toml and output directory, they are build in the same binary and tests are run for every file

    // file structure of a workspace
    /*
    ├── Cargo.lock
    ├── Cargo.toml
    ├── add_one
    │   ├── Cargo.toml
    │   └── src
    │       └── lib.rs
    ├── adder
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    └── target */

    // ../Cargo.toml :
    /*[workspace]
    members = [
        "adder",
        "add_one",
    ] */

    // adder/Cargo.toml
    /*[dependencies]
    add_one = { path = "../add_one" } */
}
