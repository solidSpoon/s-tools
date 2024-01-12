/// This function escapes special characters in a filename.
///
/// # Arguments
///
/// * `filename` - A string slice that holds the name of the file.
///
/// # Returns
///
/// * A `String` which is the escaped version of the input filename.
///
/// # Special Characters
///
/// The special characters that this function escapes are: `$`, `` ` ``, `"`, `\`, and ` ` (space).
///
/// # Example
///
/// ```
/// let filename = "my$file.txt";
/// let escaped_filename = escape_special_chars(filename);
/// assert_eq!(escaped_filename, "my\\$file.txt");
/// ```
pub fn escape_special_chars(filename: &str) -> String {
    // Define the special characters that need to be escaped
    let special_chars = ['$', '`', '"', '\\', ' '];

    // Initialize an empty string to hold the escaped filename
    let mut escaped_filename = String::new();

    // Iterate over each character in the input filename
    for c in filename.chars() {
        // If the character is a special character, add an escape character (`\`) before it
        if special_chars.contains(&c) {
            escaped_filename.push('\\');
        }
        // Add the character to the escaped filename
        escaped_filename.push(c);
    }

    // Return the escaped filename
    escaped_filename
}