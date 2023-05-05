use std::default::Default;

trait Trait {
    fn echo(&self) {}
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

#[derive(Default)]
struct Hoge;

impl Trait for Hoge {
    fn echo(&self) {
        println!("Hoge");
    }
}

#[derive(Default)]
struct Foo;

impl Trait for Foo {
    fn echo(&self) {
        println!("Foo");
    }
}

fn foo_generics<T: Trait + Default>() -> T {
    T::default()
}

fn foo_impl_trait() -> impl Trait {
    Foo::default()
}

fn main() {
    let closure = returns_closure();

    println!("{}", closure(3));

    foo_generics::<Hoge>().echo();
    foo_generics::<Foo>().echo();
    foo_impl_trait().echo();
}
