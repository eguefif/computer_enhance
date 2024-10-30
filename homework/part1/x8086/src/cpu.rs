use crate::registers::Registers;

pub struct Cpu {
    pub memory: [u8; 0xFFF],
    pub registers: Registers,
}

impl Cpu {
    pub fn new(data: [u8; 0xFFF]) -> Cpu {
        Cpu {
            memory: data,
            registers: Registers::new(),
        }
    }

    pub fn run(&mut self) {
        let mut count = 0;
        loop {
            println!("byte {}: {:x} ", count, self.memory[count]);
            count += 1;
            if self.memory[count] == 0 {
                break;
            }
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        let data = [0; 0xFFF];
        Self::new(data)
    }
}
