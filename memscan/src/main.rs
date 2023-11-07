#![warn(clippy::pedantic, clippy::nursery)]

use clap::Parser;
use colored::Colorize;
use libc::{c_void, iovec, pid_t, process_vm_readv};
use std::fs;
use std::process::Command;

/// Dump the heap of a given process
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the target process
    #[arg(value_name = "PROCESS_NAME", index = 1)]
    process_name: String,

    /// Show the dump in ascii
    #[arg(long, short, default_value_t = false)]
    ascii: bool,

    /// Hide lines containing only zeroes
    #[arg(long, short = 'z', default_value_t = false)]
    hide_zero_lines: bool,
}

/// Return the pid of the target process using `pidof`
fn get_pid(process_name: &str) -> pid_t {
    // Call the `pidof` command
    let v = Command::new("sh")
        .arg("-c")
        .arg(format!("pidof {}", process_name))
        .output()
        .expect("failed to get pid")
        .stdout;

    // If the process doesn't exist, `pidof` returns nothing
    assert!(v.len() > 0, "PID not found");

    // Convert the output of `pidof` to a String
    let s = match String::from_utf8(v[0..(v.len() - 1)].to_vec()) {
        Ok(s) => s,
        Err(_) => panic!("Can't convert from utf8"),
    };

    // Convert to i32
    let i: i32 = match s.parse() {
        Ok(i) => i,
        Err(_) => panic!("Could not parse string"),
    };

    // Returns the pid as type pid_t
    pid_t::from(i)
}

fn get_heap_addresses(pid: pid_t) -> (usize, usize) {
    // Read the maps file
    let data = fs::read_to_string(format!("/proc/{}/maps", pid))
        .expect(&format!("Can't read /proc/{}/maps", pid));

    // String manipulation to get the range as a 2-valued tuple
    for line in data.split('\n') {
        if line.contains("[heap]") {
            let heap = line.split(' ').next().unwrap();
            let mut heap_v = heap.split('-');

            // Conversion &str -> i64 -> usize
            let start_addr = i64::from_str_radix(heap_v.next().unwrap(), 16).unwrap() as usize;
            let end_addr = i64::from_str_radix(heap_v.next().unwrap(), 16).unwrap() as usize;

            return (start_addr, end_addr);
        }
    }

    panic!("No heap found in /proc/{}/maps", pid)
}

fn main() {
    let args = Args::parse();
    let pid: pid_t = get_pid(&args.process_name);
    let heap_addr = get_heap_addresses(pid);

    let start = heap_addr.0;
    let end = heap_addr.1;

    const STEP: usize = 16;

    let data: [u8; STEP] = [0; STEP]; // We'll store the bytes we copy here

    for cur_addr in (start..=end).step_by(STEP) {
        // Local buffer
        let local = iovec {
            iov_base: data.as_ptr() as *mut c_void,
            iov_len: STEP, // Number of bytes in our buffer
        };

        // The remote buffer we want to read from
        let remote = iovec {
            iov_base: cur_addr as *mut c_void, // Pointer to the the virtual memory of
            iov_len: STEP,                     //   the other processs
        };

        // We retrieve the pointers to our two iovec structs
        let p_local = &local as *const iovec;
        let p_remote = &remote as *const iovec;

        let res = unsafe { process_vm_readv(pid, p_local, 1, p_remote, 1, 0) };

        if res == -1 {
            println!("ERROR: Could not read data");
        } else {
            // Set hide_line to false if one of the bytes is non-zero
            let mut hide_line = true;
            for _b in &data {
                if *_b != 0 {
                    hide_line = false;
                    break;
                }
            }

            // Ignore these bytes
            if hide_line && args.hide_zero_lines {
                continue;
            }

            // Print the current address
            print!("{:08x} | ", cur_addr);

            // Print the bytes we copied
            for b in &data {
                // Format to hex
                let s = format!("{:02x} ", b);

                // Print with colors
                if *b == 0x00 {
                    // No colors on 0x00
                    print!("{}", s.normal().clear());
                } else if *b > 0x00 && *b <= 0x7F {
                    // Valid and printable ascii
                    print!("{}", s.black().on_green());
                } else {
                    // All non-zero and non(printable)-ascii
                    print!("{}", s.yellow());
                }
            }

            // Print the bytes as ascii
            if args.ascii {
                print!(" | ");

                // For each byte
                for b in &data {
                    // If byte is non-zero
                    if *b != 0x00 {
                        let s_print: String = match *b {
                            // If the byte is printable ascii
                            0x20..=0x7E => String::from_utf8(vec![*b]).unwrap(), // SPACE to ~
                            // Else we'll print a .
                            _ => ".".to_string(),
                        };

                        print!("{}", s_print);
                    }
                }
            }
            println!();
        }
    }
}
