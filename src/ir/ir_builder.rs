use iced_x86::Register;

use crate::{
    asm::{
        arg32, stack,
        AsmInstructionEnum::{self, *},
    },
    error::CodeGenLibError,
    Builder,
};

#[derive(Clone)]
pub struct IrFunctionBuilder {
    pub generated: Vec<AsmInstructionEnum>,
    pub name: String,
    args: Vec<((String, u64, Option<Register>), u64)>,
    vars: Vec<(String, u64)>, // u64 -> stack offset
    public: bool,
}

impl IrFunctionBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            generated: vec![],
            name: name.into(),
            args: vec![],
            vars: vec![],
            public: false,
        }
    }

    /// The input tuple values: `(String, u64)` represent:
    ///  * `String` -> The argument name
    ///  * `u64` -> The argument size in bytes
    pub fn args(&mut self, args: Vec<(&str, u64)>) {
        let mut mod_args: Vec<((String, u64, Option<Register>), u64)> = vec![];

        let mut reg_pasted_args = 0;

        let mut prev_size = 0;

        for arg in args {
            let reg: Option<Register> = {
                if reg_pasted_args < 4 && (arg.1 <= 8) {
                    reg_pasted_args += 1;
                    Some(arg32(reg_pasted_args))
                } else {
                    None
                }
            };

            mod_args.push(((arg.0.into(), arg.1, reg), prev_size));

            prev_size += arg.1;
        }

        println!("{:?}", mod_args);

        self.args = mod_args;
    }

    /// !Needs to be called after setuped args !
    /// The input tuple values: `(String, u64)` represent:
    ///  * `String` -> The argument name
    ///  * `u64` -> The var size in bytes
    pub fn vars(&mut self, vars: Vec<(&str, u64)>) {
        let mut mod_vars: Vec<(String, u64)> = vec![];

        let mut stack_offset = 0;

        for arg in self.args.iter() {
            let name = &arg.0.0;
            let size = arg.0.1;

            if arg.0.2.is_some() {
                self.generated.push( Store(arg.0.2.unwrap(), stack(-(stack_offset as i64  + 4)) ))
            }

            mod_vars.push( (name.to_string(), stack_offset));

            stack_offset += size;
        }

        for var in vars {
            mod_vars.push((var.0.into(), stack_offset));

            stack_offset += var.1;
        }

        

        self.vars = mod_vars;
    }

    /// Throws error if given argument doesn't exists
    fn check_arg(&self, name: String) -> Result<(), CodeGenLibError> {
        for arg in self.args.iter() {
            let arg = &arg.0;
            let arg_name = &arg.0;

            if &name == arg_name {
                return Ok(());
            }
        }

        Err(CodeGenLibError::VarNotExist(name))
    }

    fn check_var(&self, name: String) -> Result<(), CodeGenLibError> {
        for var in self.vars.iter() {
            let var_name = &var.0;

            if &name == var_name {
                return Ok(());
            }
        }

        Err(CodeGenLibError::VarNotExist(name))
    }

    fn get_arg(
        &self,
        name: String,
    ) -> Result<((String, u64, Option<Register>), u64), CodeGenLibError> {
        for arg in self.args.iter() {
            let arg_1 = &arg.0;
            let arg_name = &arg_1.0;

            if arg_name.to_string() == name {
                return Ok((arg_1.clone(), arg.1));
            }
        }

        Err(CodeGenLibError::VarNotExist(name))
    }

    fn get_var(&self, name: String) -> Result<(String, u64), CodeGenLibError> {
        for var in self.vars.iter() {
            if var.0 == name {
                return Ok(var.to_owned());
            }
        }

        Err(CodeGenLibError::VarNotExist(name))
    }

    pub fn build_add(
        &mut self,
        var1: &str,
        var2: &str,
        result_var: &str,
    ) -> Result<(), CodeGenLibError> {
        let var1 = self.get_var(var1.into())?;
        let var2 = self.get_var(var2.into())?;
        let ret = self.get_var(result_var.into())?;

        self.generated.push(Load(Register::RAX, stack(-(var1.1 as i64 + 4)) ));
        self.generated.push(Load(Register::RBX, stack(-(var2.1 as i64 + 4)) ));
        self.generated.push(AddReg(Register::RAX, Register::RBX));
        self.generated
            .push(Store(Register::RAX, stack(-(ret.1 as i64))));

        Ok(())
    }

    pub fn build_return_var(&mut self, var_name: &str) -> Result<(), CodeGenLibError> {
        let var = self.get_var(var_name.into())?;

        self.generated
            .push(Load(Register::RAX, stack(-(var.1 as i64))));

        // self.generated.push( Ret ); // automaticly added via safe optimizations

        Ok(())
    }

    pub fn set_public(&mut self) {
        self.public = true;
    }
}

pub struct IrBuilder {
    functs: Vec<IrFunctionBuilder>,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self { functs: vec![] }
    }

    pub fn add(&mut self, name: &str) -> &mut IrFunctionBuilder {
        let func = IrFunctionBuilder::new(name);

        self.functs.push(func);

        self.functs.last_mut().unwrap()
    }

    pub fn builder(&self) -> Result<Builder, Box<dyn std::error::Error>> {
        let mut builder = Builder::new();

        for func in self.functs.iter() {
            let func = func.to_owned();

            builder.define(&func.name, func.public, func.generated)?;
        }

        Ok(builder)
    }
}
