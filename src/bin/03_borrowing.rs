// cargo run --bin 03_borrowing

fn main() {
    let s = String::from("hello");

    let len = calculate_length(&s);

    println!("Tekst: {s}, lengde: {len}");
}

fn calculate_length(text: &String) -> usize {
    text.len()
}
