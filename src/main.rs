trait Loggable {
    fn log(&self);
}

struct Order {
    id: u8,
    total: u8,
}

struct User {
    email_id: String,
    username: String,
}

impl Loggable for User {
    fn log(&self){
        println!("User Name: {},\nEmail ID: {}", self.username, self.email_id );
    }
}

impl Loggable for Order {
    fn log(&self) {
        println!("Order ID: {},\nTotal quantity: {}", self.id, self.total);
    }
}

fn main() {
    let order1 = Order {id: 7, total: 10};
    let user1 = User {email_id: String::from("emailid@gmail.com"), username: String::from("JamesBond")};

    order1.log();
    user1.log();
}