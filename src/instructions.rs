#[derive(Debug, PartialEq)]
pub enum VaneVMOpcodes {
    HLT,
    IGL, 
    LOD, // load
    LNK, // +
    FUK, // -
    FRK, // *
    DIV  // /
}


#[derive(Debug, PartialEq)]
pub struct VaneVMInstructions {
    opcode: VaneVMOpcodes
}

impl VaneVMInstructions {
    pub fn vane_new(opcode: VaneVMOpcodes) -> VaneVMInstructions {
        VaneVMInstructions {
            opcode: VaneVMOpcodes::HLT
        }
    }
}

impl From<u8> for VaneVMOpcodes {
    fn from(v: u8) -> Self {
        match v {
            0 => return VaneVMOpcodes::HLT,
            _ => return VaneVMOpcodes::IGL
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_hlt() {
        let o = VaneVMOpcodes::HLT;
        assert_eq!(o, VaneVMOpcodes::HLT);
    }

    #[test]
    fn create_ins() {
        let i = VaneVMInstructions::vane_new(VaneVMOpcodes::HLT);
        assert_eq!(i.opcode, VaneVMOpcodes::HLT);
    }
}
