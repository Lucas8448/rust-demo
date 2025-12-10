// cargo run --bin 04_null_safety

fn main() {
    // I Rust finnes ikke null
    // I stedet bruker vi Option<T>
    
    let tall: Option<i32> = Some(42);
    let ingenting: Option<i32> = None;
    
    // Kompilatoren tvinger deg til å håndtere begge tilfeller
    if tall.is_some() {
        println!("Har verdi: {}", tall.unwrap());
    }
    
    // Trygg tilgang med default-verdi
    println!("Med default: {}", ingenting.unwrap_or(0));
    
    // Trygg tilgang til array/vector
    let tall_liste = vec![1, 2, 3];
    
    // tall_liste[10] ville panikke (som IndexOutOfBounds i Java)
    // I stedet:
    let verdi = tall_liste.get(10);  // Returnerer Option, ikke exception
    println!("Index 10: {:?}", verdi);  // None
    println!("Index 0: {:?}", tall_liste.get(0));  // Some(1)
    
    // Rust lar deg gjøre usikre ting, men du må være eksplisitt
    unsafe {
        let tall_liste = vec![1, 2, 3];
        // get_unchecked sjekker ikke bounds - kan krasje
        let farlig = tall_liste.get_unchecked(10);
        println!("Farlig verdi: {}", farlig);
    }
}
