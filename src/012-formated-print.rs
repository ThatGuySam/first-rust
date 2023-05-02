
// https://doc.rust-lang.org/rust-by-example/hello/print.html
#[allow(dead_code)]
fn index() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Sam", "Ben");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over"
    );

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10: {}", 42);
    println!("Base 2: {:b}", 42);
    println!("Base 8: {:o}", 42);
    println!("Base 16: {:x}", 42);
    println!("Base 16: {:X}", 42);

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1);

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));


    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 2;

    println!("{number:>0width$}");


    let pi = 3.141592; 
    let decimals = 3;

    println!("Pi is roughly {:.decimals$}", pi);
    println!("Pi is roughly {:.2}", pi);
}

// https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
#[allow(dead_code)]
fn debug() {

    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    // struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    #[derive(Debug)]
    struct DebugPrintable(i32);

    // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
    // is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printable
    // also.
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);

    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // So fmt::Debug definitely makes this printable but sacrifices some elegance. Rust also provides "pretty printing" with {:#?}.
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);

}

fn main() {
    // index();

    debug();

}