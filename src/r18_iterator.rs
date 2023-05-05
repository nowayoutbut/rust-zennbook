fn main() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let zipped = a1.iter().zip(a2.iter());

    let mapped = zipped.map(|z| z.0 + z.1);

    let sum = mapped
        .inspect(|n| println!("{}", n))
        .fold(0, |prev, value| prev + value);

    println!("{}", sum);
}
