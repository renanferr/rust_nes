use super::cpu::CPU6502;

// const RAM_CAP: u16 = ;

pub struct Bus {
    ram: Vec<u8>,
}

impl Bus {
    pub fn new() -> Bus {
        let b = Bus{
            ram: [0x00; 64 * 1024].to_vec(),
        };
        return b;
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr as usize] = data;
        }
    }

    pub fn read(&self, addr: u16, b_read_only: bool) -> u8 {
        if addr >= 0x0000 && addr <= 0xFFFF {
            return self.ram[addr as usize];
        }
        return 0x00;
    }
}