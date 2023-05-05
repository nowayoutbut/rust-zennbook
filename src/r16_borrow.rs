fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    a.to_string()
}

fn main() {
    let borrowed_number: &u8 = &10;
    let cloned_borrowed_number = borrowed_number.clone();
    let owned_borrowed_number = borrowed_number.to_owned();

    let borrowed_array: &[&str; 3] = &["a", "b", "c"];
    let cloned_borrowed_array = borrowed_array.clone();

    println!(
        "borrowed_number: {}, cloned: {}, owned: {}",
        type_of(borrowed_number),
        type_of(cloned_borrowed_number),
        type_of(owned_borrowed_number),
    );
    println!(
        "borrowed_array: {}, cloned: {}",
        type_of(borrowed_array),
        type_of(cloned_borrowed_array),
    );
}
