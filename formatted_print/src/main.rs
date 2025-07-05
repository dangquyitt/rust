fn main() {
    println!("{}", 32);

    println!("{0}: Hello {1}, {0}: I am {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        subject = "The Rust",
        verb = "is",
        object = "a crab"
    );

    println!("Base 10:                  {}", 1234);
    println!("Base 2    (binary)        {:b}", 1234);
    println!("Base 8    (octal)         {:o}", 1234);
    println!("Base 16   (hexadecimal)   {:x}", 1234);

    // Padding-left
    println!("{number:5>5}", number = 1);
    println!("{:5>5}", 1);
    println!("{0:5>5}", 1);

    // Padding-right
    println!("{number:5<5}", number = 1);
    println!("{:5<5}", 1);
    println!("{0:5<5}", 1);

    // Padding with named arguments
    println!("{number:0<with$}", number = 1, with = 10);

    #[allow(dead_code)] // Disable deadcode
    struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 5;
    let sentence: String = format!("Hi, I'm {}", "Bob");
    println!("Bob: {} {number:>width$}", sentence);
}
