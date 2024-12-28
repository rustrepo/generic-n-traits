trait Speak {
    fn speak(&self) {
        println!("Default speaking!");
    }
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow", );
    }
}  // Cat uses the default speak method

fn main() {
    let dog = Dog;
    dog.speak();  // Output: Woof!
    
    let cat = Cat;
    cat.speak();  // Output: Default speaking!

    let cat2 = Cat;
    cat2.speak();//Imagine if we have 70 cats and 100 dogs, managing them with functions would be difficult.
    
}
