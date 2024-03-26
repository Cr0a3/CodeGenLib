use super::asm::{ASMCall, REGISTER};
use super::mem::AdressManager;
use super::var::{Variabel, VarDataType};


#[derive(Clone)]   
pub struct Function<'a> {
    pub name: String,
    pub gen: Vec<u8>,
    asm: ASMCall,
    pos: usize,

    pub esymbols: Vec<ExternSymbol>,
    pub vars: Vec<Variabel<'a>>,

    adrmng: &'a AdressManager,
}

impl<'a> Function<'a> {
    pub fn new(name: &'a str, adrmng: &'a mut AdressManager) -> Function<'a> {
        let mut asm = ASMCall::new();
        let mut gen = vec![];
        asm.endbr64();
        for b in asm.generated.clone() { gen.push(b) }
        asm.push(REGISTER::RBP);
        for b in asm.generated.clone() { gen.push(b) }
        asm.mov_reg(REGISTER::RBP, REGISTER::RSP);
        for b in asm.generated.clone() { gen.push(b) }

        Function {
            gen: gen.clone(),
            name: name.into(),
            asm: asm,
            pos: gen.len() - 1,
            esymbols: vec![],
            vars: vec![],
            adrmng: adrmng,
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
            println!("64");
        } else if reg == REGISTER::EAX || reg == REGISTER::EBX || reg == REGISTER::ECX || reg == REGISTER::EDX {   // 32bit Register
            self.asm.mov_32(reg, value as u32);
            println!("64");
        } else if reg == REGISTER::AX || reg == REGISTER::BX || reg == REGISTER::DX {
            self.asm.mov_16(reg, value as u16);
            println!("64");
        } else {
            self.asm.mov_8(reg, value as u8);
            println!("64");
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

    pub fn asm_call(&mut self, adr: u32) -> &mut Self {
        self.asm.call(adr);
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

    pub fn create_var(&mut self, name: &str, typ: VarDataType) -> &mut Variabel {
        let adr = self.adrmng.to_owned();

        let var = Variabel::new(typ, &name.to_string(), &mut adr);
        self.vars.push(var);

        let list = self.vars.clone();
        self.vars.get_mut(list.len() -1)
            .expect("error while getting last function (CodeGenLib/x86/function.rs/121")
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