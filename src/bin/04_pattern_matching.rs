// cargo run --bin 04_pattern_matching

fn describe_number(n: u32) { // u32, kun positive heltall
    match n {
        0 => println!("Det er null"),
        1..=9 => println!("Et lite tall: {n}"),
        10..=99 => println!("Tosifret tall: {n}"),
        _ => println!("Stort tall: {n}"),
    }
}

fn main() {
    for n in [0, 5, 42, 1337] {
        describe_number(n);
    }
}
