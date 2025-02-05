// Function to count the following values inside a string:
// - Even numbers
// - Odd numbers
// - Normal chars
// - Special chars
fn main() {
    // String to analize!
    let string = "asdf213987.;;--al::\"\"34asd;";

    println!("Counting chars for => {}", string);

    // Initialize each value vars
    let mut even_numbers = 0;
    let mut odd_numbers = 0;
    let mut normal_chars = 0;
    let mut special_chars = 0;

    // Obtain iterator chars for while loop!
    let mut string_chars = string.chars();

    // I investigated on rust loops. This is a while let pattern!
    // In this case, we iterate over our string chars and the .next() method returns an Option<char>.
    // We can use the Some pattern to get the value of the char and None to break the loop.
    while let Some(c) = string_chars.next() {
        // Verify current char is a digit
        if c.is_digit(10) {
            let number = c.to_digit(10).unwrap(); // We convert the char to a digit
            if number % 2 == 0 {
                even_numbers += 1;
            } else {
                odd_numbers += 1;
            }
        } else if c.is_alphabetic() { // Verify current char is a letter
            normal_chars += 1;
        } else { // Verify current char is a special char
            special_chars += 1;
        }
    }

    println!("Even numbers: {}", even_numbers);
    println!("Odd numbers: {}", odd_numbers);
    println!("Normal chars: {}", normal_chars);
    println!("Special chars: {}", special_chars);
}
