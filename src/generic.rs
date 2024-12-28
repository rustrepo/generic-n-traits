
use std::fmt::Debug;//without debug, we won't be able to print value Struct or Generic type


fn main() {

    fn print_value<T: Debug>(value: T) {

        println!("{:?}", value);

    }

print_value(42);    // Works with integers
print_value("Hello");  // Works with strings


struct Point <T: Debug> {
    x: T,
    y: T,
}

let p1 = Point {x:1,    y:2};
let p2 = Point {x:1.5,  y:2.5};

println!("{:?}", p1.x);
println!("{:?}", p2.y);    

}
