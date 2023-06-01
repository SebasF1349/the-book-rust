#![allow(dead_code)]
#![allow(unused_variables)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    //this next line is needed to bring the code under test in the outer module into the scope of the inner module
    use super::*;

    #[test]
    fn this_works() {
        let result = add(2, 2);
        // this shows the values if it fails
        assert_eq!(result, 4); // tests if both values are the same
    }

    #[test]
    fn it_adds_two() {
        assert_ne!(add(4, 2), 5); //tests if both values are different
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic(expected = "must be between 1 and 100")] //expected is a substring of the panic message we expect; it's not needed, but helps when there are various possible panics to test
    fn greater_than_100() {
        Guess::new(200);
    }

    //tests not only work for panics, can also be used for `Results`
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // cl flags
    // to don't run tests on parallel: cargo test -- --test-threads=1
    // to show output of fn even if it passes: cargo test -- --show-output
    // to only run one test: cargo test [test_fn_name]
    // filter tests by name: cargo test [substring_of_test/s_name]
    // to run only ignored tests: cargo test -- --ignored
    // add #[ignore] after #[test] to ignore a test when running cargo test
    // to run all tests, even the ignored ones: cargo test -- --include-ignored

    // unit tests: small and focused, testing one module in isolation at a time and can test private interfaces
    // integration tests: external to the lib and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test

    // unit tests are placed in the src directory in each file with the code that they're testing; the convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `#[cfg(test)]` as to ignore them when building (cfg means configuration)

    // integration tests are placed in the tests directory (at the same level as src); `#[cfg(test)]` is not needed in them
    // to create helper fn for the integration tests and avoid them being mentioned in the test, they should be added as a submodule, as a mod.rs file inside a folder
    // integration tests are not possible when the project is a binary crate that only contains a src/main.rs file, that's one of the reasons why Rust projects that provide a binary have a straighforward src/main.rs file that calls logic that lives in the src/lib.rs file, so that lib.rs can be have an integration test.
}
