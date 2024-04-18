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

        let mut index = 0;

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

            index += 1;
            prev_size += arg.1;
        }

        println!("{:?}", mod_args);

        self.args = mod_args;
    }

    /// The input tuple values: `(String, u64)` represent:
    ///  * `String` -> The argument name
    ///  * `u64` -> The argument size in bytes
    pub fn vars(&mut self, vars: Vec<(&str, u64)>) {
        let mut mod_vars: Vec<(String, u64)> = vec![];

        for var in vars {
            mod_vars.push((var.0.into(), var.1));
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

    pub fn build_arg_add(
        &mut self,
        arg1: &str,
        arg2: &str,
        result_var: &str,
    ) -> Result<(), CodeGenLibError> {
        let arg1 = self.get_arg(arg1.into())?;
        let arg2 = self.get_arg(arg2.into())?;
        let ret = self.get_var(result_var.into())?;

        // Move first arg into rax
        if arg1.0 .2.is_none() {
            self.generated
                .push(Load(Register::RAX, stack(-(arg1.1 as i64))));
        } else {
            self.generated
                .push(MovReg(Register::RAX, arg1.0 .2.unwrap()));
        }

        // Move second arg into rbx
        if arg2.0 .2.is_none() {
            self.generated
                .push(Load(Register::RBX, stack(-(arg2.1 as i64))));
        } else {
            self.generated
                .push(MovReg(Register::RBX, arg2.0 .2.unwrap()));
        }

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
