// cargo run --bin 03_borrowing

fn main() {
    let mut s = String::from("hello");
    let a = &mut s;
    let b = &mut s;
    tekst(a, b);
}

fn tekst_endring(a: &mut String, b: &mut String) {
    a.push_str("!");
    b.push_str("?");
}

// En immutabel referanse (&) kan deles med mange samtidig,
// men en mutabel referanse (&mut) kan kun eksistere én om gangen