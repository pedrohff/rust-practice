fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        subject = "the lazy dog",
        verb = "jumps over",
        object = "the quick brown fox"
    );
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
    println!("My name is {0}, {1} {0}", "Bond", "James");
}
