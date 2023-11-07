use std::io::Write;

fn main() {
    // The string we'll try to access
    let mut pass: String = String::new();

    // Prompt the user for the password
    print!("Enter a password: ");
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut pass)
        .expect("Failed to read password");

    // Print the virtual memory address of the String
    println!("String.as_ptr {:p}", pass.as_ptr());

    // Add a useless input to fill the stdin buffer with something new
    // print!("Write anything: ");
    // let _ = std::io::stdout().flush();
    // let _ = std::io::stdin().read_line(&mut String::new());

    // Wait for the user to quit the program
    print!("Press enter to quit.");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut String::new());
}
