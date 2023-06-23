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

    // interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data, which is normally disallowed by the bottoring rules
    // to mutate data the pattern uses `unsafe` code inside a data structure
    // we can only use this pattern when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can't guarantee that
    // the `unsafe code involved is then weapped in a safe API, and the outer type is still immutable

    // the `RefCell<T>` type follows the interior mutability pattern
    // unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data it holds
    // with `RefCell<T>` the borrowing rules' invariants are enforced at runtime instead of at compile time
    // this type is useful when you are sure the code follows the borrowing rules but the compiler is unable to understand and guarantee that
    // as `Rc<T>`, this type is only for use in single-threaded scenarios

    // reasons to choose Box, Rc or RefCell:
    /* - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
    - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable. (mutability pattern) */

    // a common way to use `RefCell<T>` is in combination with `Rc<T>` to get a value that can have multiple owners and that can be mutated

    #[derive(Debug)]
    enum ListMut {
        Cons(Rc<RefCell<i32>>, Rc<ListMut>),
        Nil,
    }
    use std::cell::RefCell;
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(ListMut::Cons(Rc::clone(&value), Rc::new(ListMut::Nil)));
    let b = ListMut::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = ListMut::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;

    // you can create a memory leak by using `Rc<T>`and `RefCell<T>` creating references where items refer to each other in a cycle as then the `strong_count` of the Rc will never be 0
    // to avoid memory leaks, you call `Rc::downgrade` and get a smart pointer of type `Weak<T>`
    // weak references don't express an ownership relationship and their count doesn't affect when an `Rc<T>` instance is cleaned up and its count doesn't need to be 0 for the `Rc<T>` instance to be cleaned up

    use std::rc::Weak;
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    //assert_eq!(leaf.parent.borrow().upgrade(), None);
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    //assert_eq!(leaf.parent.borrow().upgrade(), Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } }));
}
