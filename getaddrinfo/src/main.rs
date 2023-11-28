mod getaddrinfo;
mod _types;

use std::io::{self, Write};

fn main() {
    let domain = &mut String::new();
    print!("Enter a domain to lookup: ");
    let _ = io::stdout().flush();
    match io::stdin().read_line(domain) {
        Ok(_) => {},
        Err(_) => {
            println!("Failed to read input");
            return;
        }
    }
    let domain = domain.trim_end();
    // println!("domain: {}", domain);

    let res = getaddrinfo::getaddrinfo(domain);
    match res {
        Ok(addr) => {
            println!("IPv4 address for {} is: {}", domain.trim_end(), addr.to_string())
        },
        Err(_) => {
            println!("Failed to lookup address info");
        }
    }
}