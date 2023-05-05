use std::cell::Cell;

fn main() {
    let a = Cell::new(10);

    dbg!(a.get());

    a.set(20);

    dbg!(a.get());
}
