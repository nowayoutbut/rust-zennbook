// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn new(width: u32, height: u32) -> Self {
//         Self { width, height }
//     }

//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// #[derive(Debug)]
// enum IpAddress {
//     V6(String, String, String, String, String, String),
//     V4(u8, u8, u8, u8),
// }

// impl IpAddress {
//     fn call(&self) -> () {
//         println!("{:?}", self);
//     }
// }

fn closure(y: i32) -> impl Fn(i32) -> i32 {
    let z = move |x| x + y;

    return z;
}

fn main() {
    let c = closure(20);

    println!("loop is {}", c(32));
}
