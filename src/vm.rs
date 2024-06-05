use crate::instructions::*;

pub struct VaneVM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>
}

impl VaneVM {
    pub fn create_new() -> VaneVM {
        VaneVM {
            registers: [0; 32],
            pc: 0,
            program: vec![]
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
                _ => {
                    println!("[!] Invalid Opcode matched.");
                    return;
                }
            }
        }
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
}
