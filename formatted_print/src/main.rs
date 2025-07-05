use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

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

    let number: f64 = 1.0;
    let width: usize = 5;
    let sentence: String = format!("Hi, I'm {}", "Bob");
    println!("Bob: {} {number:>width$}", sentence);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:#?} will print!", Structure(3));

    println!("Now {:#?} will print!", Deep(Structure(7)));
}
