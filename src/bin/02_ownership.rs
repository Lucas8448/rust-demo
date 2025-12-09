// cargo run --bin 02_ownership

fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{s}");
    // ^ verdi flyttet, kan ikke lenger brukes her
}

fn takes_ownership(text: String) {
    println!("Fikk: {text}");
}
