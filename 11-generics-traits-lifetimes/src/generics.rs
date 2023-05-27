#![allow(dead_code)]
#![allow(unused_variables)]

pub fn generics() {
    // generics can be used in fn, but must be sure that the operations are valid for every type. In the next case, this doesn't compile because not all types can be compared with `>`
    /* fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    } */

    struct Point<T> {
        x: T,
        y: T,
    }
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    struct PointTwoGenerics<T, U> {
        x: T,
        y: U,
    }
    let integer_and_floar = PointTwoGenerics { x: 5, y: 4.0 };

    enum EnumGenerics<T, U> {
        GenericOne(T),
        GenericTwo(U),
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    impl<X1, Y1> PointTwoGenerics<X1, Y1> {
        fn mixup<X2, Y2>(self, other: PointTwoGenerics<X2, Y2>) -> PointTwoGenerics<X1, Y2> {
            PointTwoGenerics {
                x: self.x,
                y: other.y,
            }
        }
    }
}
