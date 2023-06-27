#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    // Common OOP characteristics:

    // Objects
    // Rust doesn't have objects, but structs and enums have data and `impl` blocks provide methods, so both have the same functionality as objects

    // Encapsulation (hiding implementation details)
    // as every module, type, fn or methos is private by default, and is only public using the `pub` keyword, we can use Encapsulation in Rust
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }
    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }

    // Inheritance
    // Rust doesn't have inheritance, but there are workarounds
    // if you use inheritance to avoid repeating a method, you can use trait method implementations
    // the other reason to use inheritance is to do polymorphism (to enable a child type to be used in the same places as the parent type); in Rust you can use generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide

    // trait objects are like objects in other languages in the sense that they combine data and behaviour, but trait objects differ form traditional objects in other languages in that we can't add data to a trait object; they are not as useful as objects, their specific purpose is to allow abstraction across common behavior
    pub trait Draw {
        fn draw(&self);
    }
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>, //`Box<dyn Draw>` is the trait object: it's any type inside a `Box` that implements the `Draw` trait
    }
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            //code to actually draw a button
        }
    }
    let screen = Screen {
        components: vec![Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        })],
    };
    screen.run(); // this works for any component with the `Draw` trait (as `Button`)

    // this works differently from defining a struct that uses a generic type parameter with trait bounds, as a generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime
    /* pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    } */

    // one downside to using trait objects is how they interact with type inference
    /* let components: Vec<Box<dyn Draw>> = vec![ //type is needed as it can't be infered (it's `SelectBox` or `Button`?)
        Box::new(SelectBox { /* .. */ }),
        Box::new(Button { /* .. */ }),
    ]; */
    // in general, it is good to be aware that using trait objects can cause a worse developer experience for API clients in the case of type inference.

    // when using trait objects the compiler doesn't know all the types that might be used, so it doesn't know which method implemented on which type to call; instead, at runtime, Rust uses the pointers inside the trait object to know which method to call
    // this lookup incurs a runtime cost and also prevents the compiler some optimizations

    // The state pattern is an object-oriented design pattern. The crux of the pattern is that we define a set of states a value can have internally. The states are represented by a set of state objects, and the value’s behavior changes based on its state. We’re going to work through an example of a blog post struct that has a field to hold its state, which will be a state object from the set "draft", "review", or "published".
    //The final functionality will look like this:
    // 1.A blog post starts as an empty draft.
    // 2.When the draft is done, a review of the post is requested.
    // 3.When the post is approved, it gets published.
    // 4.Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
    //src/lib.rs
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }
    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }
        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }
    // we could have used an Enum instead of this State and the different structs, but that would mean to use match everytime to check the value of the enum and that could get more repetitive than this trait object solution
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }
    struct Draft {}
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
    struct PendingReview {}
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }
    struct Published {}
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
    //src/main.rs
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // By implementing the state pattern exactly as it’s defined for object-oriented languages, we’re not taking as full advantage of Rust’s strengths as we could. Let’s look at some changes we can make to the blog crate that can make invalid states and transitions into compile time errors.
    pub struct PostRusty {
        content: String,
    }
    pub struct DraftPost {
        content: String,
    }
    #[allow(clippy::new_ret_no_self)]
    impl PostRusty {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }
        pub fn content(&self) -> &str {
            &self.content
        }
    }
    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }
    pub struct PendingReviewPost {
        content: String,
    }
    impl PendingReviewPost {
        pub fn approve(self) -> PostRusty {
            PostRusty {
                content: self.content,
            }
        }
    }
    //main.rs
    fn main() {
        let mut post = PostRusty::new();
        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        let post = post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
    //this way we are moving the encoding of the state to the types of the struct, and so a draft can't call to content (compile error!)
}
