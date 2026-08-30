mod slug;
mod circular_buffer;

use slug::MySlug;
use circular_buffer::CircularBuffer;

fn main() {
    println!("Exercise 1 - MySlug Trait\n");

    // Examples of using is_slug()
    let valid_slug = "hello-world-123";
    let invalid_slug = "Hello World 123";

    println!("\"{}\" is a valid slug? {}", valid_slug, valid_slug.is_slug());
    println!("\"{}\" is a valid slug? {}", invalid_slug, invalid_slug.is_slug());

    // Examples of using to_slug()
    let text = "Hello World! This is a Test";
    let slug = text.to_slug();

    println!("\nOriginal text: \"{}\"", text);
    println!("Converted to slug: \"{}\"", slug);
    println!("Verification: \"{}\" is a valid slug? {}", slug, slug.is_slug());

    // Example with String
    let string_text = String::from("Another Example with Spaces and UPPERCASE");
    let string_slug = string_text.to_slug();

    println!("\nOriginal String: \"{}\"", string_text);
    println!("Converted to slug: \"{}\"", string_slug);

    println!("\nEnd of Exercise 1");

    // ---------------------------------------------

    println!("\n\nExercise 3 - Circular Buffer\n");

    // Creation of a circular buffer of integers
    let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5);

    println!("Buffer created with capacity: {}", buffer.capacity());
    println!("Is the buffer empty? {}", buffer.is_empty());

    // Writing elements to the buffer
    println!("\nWriting elements to the buffer:");
    for i in 1..=5 {
        match buffer.write(i * 10) {
            Ok(_) => println!("Element {} successfully written", i * 10),
            Err(e) => println!("Error during write: {}", e),
        }
    }

    println!("\nCurrent buffer size: {}", buffer.size());
    println!("Is the buffer full? {}", buffer.is_full());

    // Attempt to write to a full buffer
    println!("\nAttempt to write to a full buffer:");
    match buffer.write(60) {
        Ok(_) => println!("Element 60 successfully written"),
        Err(e) => println!("Error during write: {}", e),
    }

    // Reading elements from the buffer
    println!("\nReading elements from the buffer:");
    for _ in 0..3 {
        match buffer.read() {
            Ok(val) => println!("Element read: {}", val),
            Err(e) => println!("Error during read: {}", e),
        }
    }

    println!("\nCurrent buffer size after reading: {}", buffer.size());

    // Using overwrite
    println!("\nOverwriting elements:");
    buffer.overwrite(100);
    buffer.overwrite(110);

    println!("Reading after overwrite:");
    while !buffer.is_empty() {
        match buffer.read() {
            Ok(val) => println!("Element read: {}", val),
            Err(e) => println!("Error during read: {}", e),
        }
    }

    // Access via index
    println!("\nAccess via index:");
    for i in 1..=3 {
        buffer.write(i).unwrap();
    }

    println!("buffer[0] = {}", buffer[0]);
    println!("buffer[1] = {}", buffer[1]);
    println!("buffer[2] = {}", buffer[2]);

    // Demonstration of dereferencing
    println!("\nDemonstration of dereferencing:");

    // Check if the buffer is contiguous before dereferencing it
    if buffer.is_contiguous() {
        let slice: &[i32] = &buffer;
        println!("Dereferencing as slice: {:?}", slice);
    } else {
        println!("The buffer is not contiguous, making it contiguous before dereferencing");
        buffer.make_contiguous();
        let slice: &[i32] = &buffer;
        println!("Dereferencing as slice after make_contiguous(): {:?}", slice);
    }

    println!("\nEnd of Exercise 3");
}

