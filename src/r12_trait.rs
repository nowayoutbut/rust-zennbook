use core::fmt;
use std::fmt::{Debug, Display};

pub trait Geometry {
    fn area(&self) -> f64;
    fn name(&self) -> &str {
        "Geometry"
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Geometry for Rectangle {
    fn area(&self) -> f64 {
        (self.width * self.height) as f64
    }
    fn name(&self) -> &str {
        "Rectangle"
    }
}

impl From<u32> for Rectangle {
    fn from(x: u32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Debug)]
struct Triangle {
    bottom: u32,
    height: u32,
}

impl Geometry for Triangle {
    fn area(&self) -> f64 {
        ((self.bottom * self.height) / 2) as f64
    }
    fn name(&self) -> &str {
        "Triangle"
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.bottom, self.height)
    }
}

fn draw<T>(geometry: &T) -> ()
where
    T: Geometry + Debug + Display,
{
    dbg!(geometry);
    println!("{}", geometry);
}

fn main() {
    let a = Rectangle {
        width: 10,
        height: 20,
    };

    let b = Triangle {
        bottom: 30,
        height: 20,
    };

    draw(&b);

    let c: Rectangle = (5).into();

    println!("{} area={}", a.name(), a.area());
    println!("{} area={}", b.name(), b.area());
    println!("{} area={}", c.name(), c.area());
}
