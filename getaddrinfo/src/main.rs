mod getaddrinfo;
mod _types;

use std::io::{self, Write};

fn main() {
    let mut domain = String::new();
    print!("Enter a domain to lookup: ");
    let _ = io::stdout().flush();
    let input = io::stdin().read_line(&mut domain);
    match input {
        Ok(_) => {},
        Err(_) => {
            println!("Failed to read input");
            return ();
        }
    }
    let res = getaddrinfo::getaddrinfo("www.example.org");
    match res {
        Ok(addr) => {
            println!("IPv4 address for {} is: {}", domain.trim_end(), addr.to_string())
        },
        Err(_) => {
            println!("Failed to lookup address info");
        }
    }
}