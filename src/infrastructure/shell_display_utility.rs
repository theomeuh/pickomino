/// print special shell character to clear a shell
pub fn clear_shell() {
    print!("\x1B[2J");
}

/// use to separate group of message
pub fn print_seperator_shell() {
    println!("--------------------------------");
}
