// cargo run --bin 01_basics

fn main() {
    // IMMUTABILITY SOM STANDARD
    let x = 5;              // Immutable - default - optimalisert
    let mut y = 10;         // Explicit opt-in for mutation
    y += 1;
    println!("x={x}, y={y}");

    // TYPE INFERENCE (bidirectional)
    let a = 42;             // i32 inferred
    let b = 3.14;           // f64 inferred
    let mut v = Vec::new();
    v.push(1u64);           // Vec<u64> inferred backwards from usage
    
    let n = "42".parse::<i32>().unwrap();  // Såkalt Turbofish ::<T> operator for å spesifisere type når type ikke kan infereres
    println!("Parsed: {n}");

    // SHADOWING (rebinding, ikke mutasjon)
    let s = "hello";        // &str
    let s = s.len();        // usize - ny binding, type kan endres!
    println!("Length: {s}");
}


