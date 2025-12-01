use ipnet::IpNet;
use ipnet_trie::IpnetTrie;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // 1. Read your text blocklist from stdin
    eprintln!("Reading blocklist from stdin...");
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    // 2. Build the tries
    let mut blocklist = IpnetTrie::<u8>::new();

    for line in contents.lines() {
        if let Ok(ip_network) = line.trim().parse::<IpNet>() {
            blocklist.insert(ip_network, 0);
        }
    }

    eprintln!("Built trie with {} IPv4 and {} IPv6 networks.", blocklist.len().0 , blocklist.len().1);

    // 3. Serialize to a single binary file
    let bytes: Vec<u8> = blocklist.export_to_bytes();

    // Write to stdout 
    std::io::stdout().write_all(&bytes)?;

    Ok(())
}
