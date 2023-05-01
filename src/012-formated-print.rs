fn index() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
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

fn main() {
    // https://doc.rust-lang.org/rust-by-example/hello/print.html
    index();

}