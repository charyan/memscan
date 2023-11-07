/*
 * memscan
 * Copyright (C) 2023 Yannis Charalambidis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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
