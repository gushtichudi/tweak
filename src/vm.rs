use crate::instructions::*;

#[derive(Debug)]
pub struct VaneVM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32
}

impl VaneVM {
    pub fn create_new() -> VaneVM {
        VaneVM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0
        }
    }
  
     
    pub fn run_vm(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                break;
            }

            match self.decode() {
                VaneVMOpcodes::HLT => {
                    println!("[0] HLT Encountered!");
                    return;
                },
                VaneVMOpcodes::LOD => {
                    let r = self.eightbit() as usize;
                    let n = self.sixteenbit() as u16;
                    
                    self.registers[r] = n as i32;
                    continue;
                },

                _ => {
                    println!("[!] Invalid Opcode matched.");
                    return;
                }
            }
        }
    }
    

    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.exec();
        }
    }

    pub fn once(&mut self) {
        self.exec();
    }

    fn exec(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        match self.decode() {
            VaneVMOpcodes::HLT => {
                println!("[0] HLT Encountered!");
                return;
            },

            VaneVMOpcodes::LOD => {
                let r = self.eightbit() as usize;
                let n = self.sixteenbit() as u16;
                    
                self.registers[r] = n as i32;
            },

            VaneVMOpcodes::TWK => {
                let t = self.registers[self.eightbit() as usize];
                self.pc = t as usize;
            },

            VaneVMOpcodes::LNK => {
                let r = self.eightbit() as usize;
                let n = self.sixteenbit() as u16;

                self.registers[self.eightbit() as usize] = r + n; 
            },


            VaneVMOpcodes::FUK => {
                let r = self.eightbit() as usize;
                let n = self.sixteenbit() as u16;

                self.registers[self.eightbit() as usize] = r - n; 
            },


            VaneVMOpcodes::FRK => {
                let r = self.eightbit() as usize;
                let n = self.sixteenbit() as u16;

                self.registers[self.eightbit() as usize] = r * n; 
            },


            VaneVMOpcodes::DIV => {
                let r = self.eightbit() as usize;
                let n = self.sixteenbit() as u16;

                self.registers[self.eightbit() as usize] = r / n;
                self.remainder = (r % n) as u32;
            },

            _ => {
                println!("[!] Invalid Opcode matched.");
                return;
            }
        }
    }

    fn eightbit(&mut self) -> u8 {
        let res = self.program[self.pc];
        self.pc += 1;

        res
    }

    fn sixteenbit(&mut self) -> u16 {
        let res = (
            (self.program[self.pc] as u16) << 8
            ) | self.program[self.pc + 1] as u16;
        self.pc += 2;

        res
    }

    fn decode(&mut self) -> VaneVMOpcodes {
        let o = VaneVMOpcodes::from(self.program[self.pc]);
        self.pc += 1;

        return o;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crvm() {
        let testvm = VaneVM::create_new();
        assert_eq!(testvm.registers[0], 0);
    }

    #[test]
    fn hlt() {
        let mut tvm = VaneVM::create_new();
        let bytes = vec![0, 0, 0, 0];

        tvm.program = bytes;
        tvm.run_vm();

        assert_eq!(tvm.pc, 1);
    }

    
    #[test]
    fn igl() {
        let mut tvm = VaneVM::create_new();
        let bytes = vec![0, 0, 0, 0];

        tvm.program = bytes;
        tvm.run_vm();

        assert_eq!(tvm.pc, 1);
    }

    #[test]
    fn lod() {
        let mut tvm = VaneVM::create_new();
        tvm.program = vec![0, 0, 1, 24, 4];

        tvm.run_vm();
        
        assert_eq!(tvm.registers[0], 500);
    }
}
