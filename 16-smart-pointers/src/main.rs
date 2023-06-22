#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    // a pointer is a general convept for a variable that contains an address in memory
    // the most common kind of pointer in Rust is a reference, indicated by the `&` symbol and borrow the value they point to but don't have any special capabilities
    // smart pointers are data structures that act like a pointer but also have additional metadata and capabilities
    // in Rust smart pointers own the data they point to
    // `String` and `Vec<T>` are smart pointers (!) as they own some memory and allow you to manipulate it and they also have metadata and extra capabilities or guarantees
    // smart pointers implement the `Deref` (allows an instance of the smart pointer struct to behave like a reference) and `Drop` (allows to customize the code that's run when an instance of the smart pointer goes out of scope) traits

    // the most common smart pointers in the std lib are `Box<T>`, `Rc<T>` and `Ref<T>` and `RefMut<T>`, accesed threouygh `RefCell<T>`

    // `Box<T>`
    // Boxes allow you to store data on the heap rather than the stack. In the stack only remains the pointer to the heap data.

    let b = Box::new(5);
    println!("b = {}", b);

    // boxes don't have performance overhead
    // they are used mostle in these situations
    // - when you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size (see next section)
    // - when you have a large amount of data and you want to transfer ownership but unsure the data won't be copied when you do so
    // - when you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type

    // A cons list is a data structure that comes from the Lisp programming language and is made up of nested pairs, and is the Lisp version of a linked list. Each item in a const list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called `Nil`without a next item
    // for example: (1, (2, (3, Nil))

    enum List {
        Cons(i32, Box<List>), // without the Box<T> the type has infinite size, so can't be compiled as the compiled don't know how much space to allocate for List, using Box<T> means it will only save the pointer, which has a fixed size
        Nil,
    }

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, *y); // we can dereference the Box because it has the `Deref` trait.

    // to implement the `Deref` trait you need to implement a `deref` fn that returns a reference to dereference
    use std::ops::Deref;
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // to implement the `Drop` trait you need to implement a `drop` fn that will be excecuted when the instance goes out of scope
    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    // `Rc<T>` is an abbreviation for reference counting, it keeps track of the number of references to a value to determine whether or not the value is still in use, if there are zero references to a value, the value can be cleaned up without any references becoming invalid
    // we use the `Rc<T>` type when we want to allocate some data on the heap for multiple parts of our program to read and we can't determine at compile time which part will finish using the data last
    // note that `Rc<T>` is only for use in single-threaded scenarios

    // lets reimplement the cons list but with 2 lists that owns a third one
    // with boxes that would return a compile error as we can't move a list to two different ones
    // to avoid this, we should use Rc instead of boxes
    use std::rc::Rc;
    enum ListRc {
        Cons(i32, Rc<ListRc>),
        Nil,
    }
    let a = Rc::new(ListRc::Cons(
        5,
        Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil))),
    ));
    let b = ListRc::Cons(3, Rc::clone(&a));
    let c = ListRc::Cons(4, Rc::clone(&a));
    // so now Rc<ListRc> saves the count of 3 references, we can use `Rc::strong_count(&a)` to get the number of references
}
