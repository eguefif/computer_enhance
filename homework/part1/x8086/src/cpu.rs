use crate::registers::Registers;

pub struct Cpu {
    pub memory: [u8; 0xFFF],
    pub registers: Registers,
    pub pc: u16,
}

impl Cpu {
    pub fn new(data: [u8; 0xFFF]) -> Cpu {
        Cpu {
            memory: data,
            registers: Registers::new(),
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode: u16 = self.get_opcode();
            println!("opcode {}: {:x} ", self.pc - 1, opcode);
            if self.pc == 0xFFF || self.memory[self.pc as usize] == 0 {
                break;
            }
        }
    }

    fn get_opcode(&mut self) -> u16 {
        let position = self.pc as usize;
        self.pc += 2;
        (self.memory[position] as u16) << 8 | self.memory[position + 1] as u16
    }
}

impl Default for Cpu {
    fn default() -> Self {
        let data = [0; 0xFFF];
        Self::new(data)
    }
}
