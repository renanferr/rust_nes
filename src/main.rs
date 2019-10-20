mod nes;
use nes::cpu::CPU6502;

fn main() {
    println!("Hello, world!");
    let b = nes::bus::Bus::new();
    let mut c = CPU6502::new(b);
    let addr = 0x41;
    let data = 1;
    c.write(addr, data);

    println!("read {:?}", assert_eq!(c.read(addr), data));
}