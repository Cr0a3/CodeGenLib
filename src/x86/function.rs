use super::asm::{ASMCall, REGISTER};


#[derive(Clone)]   
pub struct Function {
    pub name: String,
    pub gen: Vec<u8>,
    asm: ASMCall,
}

impl Function {
    pub fn new(name: &str) -> Self {
        Self {
            gen: vec![],
            name: name.into(),
            asm: ASMCall::new(),
        }
    }

    fn add_gen(&mut self) {
        for b in self.asm.generated.clone() {
            self.gen.push(b);
        }
    }

    pub fn ret(&mut self) {
        self.asm.ret();
        self.add_gen();
    }

    pub fn mov(&mut self, reg: REGISTER, value: u32) {
        if reg == REGISTER::EAX || reg == REGISTER::EBX || reg == REGISTER::ECX || reg == REGISTER::EDX {   // 32bit Register
            self.asm.mov_32(reg, value);
        } else if reg == REGISTER::AX || reg == REGISTER::BX || reg == REGISTER::DX {
            self.asm.mov_16(reg, value as u16);
        } else {
            self.asm.mov_8(reg, value as u8);
        }

        self.add_gen();
    }

    pub fn get_gen(&self) -> Vec<u8> {
        self.gen.clone()
    }
}