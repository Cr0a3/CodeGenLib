use super::asm::{ASMCall, REGISTER};


#[derive(Clone)]   
pub struct Function {
    pub name: String,
    pub gen: Vec<u8>,
    asm: ASMCall,
    pos: usize,

    pub esymbols: Vec<ExternSymbol>,
}

impl Function {
    pub fn new(name: &str) -> Self {
        let mut asm = ASMCall::new();
        let mut gen = vec![];
        asm.endbr64();
        for b in asm.generated.clone() { gen.push(b) }
        asm.push(REGISTER::RBP);
        for b in asm.generated.clone() { gen.push(b) }
        asm.mov_reg(REGISTER::RBP, REGISTER::RSP);
        for b in asm.generated.clone() { gen.push(b) }

        Self {
            gen: gen.clone(),
            name: name.into(),
            asm: asm,
            pos: gen.len() - 1,
            esymbols: vec![],
        }
    }

    fn add_gen(&mut self) {
        for b in self.asm.generated.clone() {
            self.gen.push(b);
            self.pos += 1;
        }
    }

    pub fn asm_ret(&mut self) -> &mut Self {
        self.asm.nop();
        self.add_gen();
        self.asm.pop(REGISTER::RBP);
        self.add_gen();
        self.asm.ret();
        self.add_gen();

        self
    }

    pub fn asm_mov(&mut self, reg: REGISTER, value: u64) -> &mut Self {
        if  reg == REGISTER::RAX || reg == REGISTER::RBX || reg == REGISTER::RCX || reg == REGISTER::RDX || 
            reg == REGISTER::RBP || reg == REGISTER::RDI || reg == REGISTER::RIP || reg == REGISTER::RSI || 
            reg == REGISTER::RSP {
            self.asm.mov_64(reg, value);
        } else if reg == REGISTER::EAX || reg == REGISTER::EBX || reg == REGISTER::ECX || reg == REGISTER::EDX {   // 32bit Register
            self.asm.mov_32(reg, value as u32);
        } else if reg == REGISTER::AX || reg == REGISTER::BX || reg == REGISTER::DX {
            self.asm.mov_16(reg, value as u16);
        } else {
            self.asm.mov_8(reg, value as u8);
        }

        self.add_gen();

        self
    }

    pub fn ret_int(&mut self, value: u64) -> &mut Self {
        if value > 0xffffffff {
            self.asm_mov(REGISTER::RAX, value);
        } else {
            self.asm_mov(REGISTER::EAX, value);
        }
        self.asm_ret();

        self
    }

    pub fn asm_call(&mut self, adr: u64) -> &mut Self {
        self.asm.mov_64(REGISTER::RAX, adr);
        self.asm.call_reg(REGISTER::RAX);
        self.add_gen();

        self
    }

    pub fn call(&mut self, dest: &str) -> &mut Self {
        self.esymbols.push(
            ExternSymbol {
                start: self.name.clone(),
                dest: dest.into(),
                at: self.pos + 1,
            }
        );

        self.asm_call(0);

        self
    }

    pub fn get_gen(&self) -> Vec<u8> {
        self.gen.clone()
    }
}

#[derive(Clone)]
pub struct ExternFunction {
    pub name: String,
}

#[derive(Clone)]
pub struct ExternSymbol {
    pub start: String,
    pub dest: String,
    pub at: usize,
}