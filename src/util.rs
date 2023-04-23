use std::io::{self, Write};

// TOOD move to utils
pub fn prompt_for_consent(message: &str) -> bool {
    let mut input = String::new();
    print!("{} [Y/n]: ", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();
    input == "y" || input.is_empty()
}
