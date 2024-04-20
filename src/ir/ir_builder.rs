use std::error::Error;

use iced_x86::Register;

use crate::{
    asm::{
        arg32, arg64, stack, AsmInstructionEnum::{self, *}
    },
    error::CodeGenLibError,
    Builder,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArgTyp<'a> {
    Int32(u32),
    Int64(u64),
    Str(&'a str),
}

pub struct IrFunctionBuilder {
    pub generated: Vec<AsmInstructionEnum<'static>>,
    pub name: String,
    args: Vec<((String, u64, Option<Register>), u64)>,
    vars: Vec<(String, i64)>, // i64 -> stack offset
    funcs: Vec<(String, Vec<ArgTyp<'static>>)>,
    public: bool,

    builder: &'static mut Builder
}

impl IrFunctionBuilder {
    pub fn new(name: &str, builder: &'static mut Builder) -> Self {
        Self {
            generated: vec![],
            name: name.into(),
            args: vec![],
            vars: vec![],
            funcs: vec![],
            public: false,

            builder: builder,
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
        let mut mod_vars: Vec<(String, i64)> = vec![];
        let mut stack_args: Vec<(String, u64)> = vec![];

        let mut stack_offset: i64 = 4;

        for arg in self.args.iter() {
            let name = &arg.0.0;
            let size = arg.0.1;

            if arg.0.2.is_some() {
                self.generated
                   .push(Store(arg.0.2.unwrap(), stack(-stack_offset)));

                mod_vars.push((name.into(), -stack_offset));
            } else {
               stack_args.push((name.into(), size));
               mod_vars.push((name.to_string(), stack_offset));
            }

            stack_offset += size as i64;
        }

        for var in stack_args {
            mod_vars.push((var.0, var.1 as i64));
        }

        for var in vars {
            mod_vars.push((var.0.into(), -stack_offset));

            stack_offset += var.1 as i64;
        }

        println!("{:?}", &mod_vars);

        self.vars = mod_vars;
    }

    pub fn efuncs(&mut self, funcs: Vec<(String, Vec<ArgTyp<'static>>)>) {
        self.funcs = funcs;
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

    fn get_var(&self, name: String) -> Result<(String, i64), CodeGenLibError> {
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


        self.generated
            .push(Load(Register::RAX, stack(var1.1)));
        self.generated
            .push(AddMem(Register::RAX, stack(var2.1) ));

        self.generated
            .push(Store(Register::RAX, stack(ret.1)));

        Ok(())
    }

    pub fn build_return_var(&mut self, var_name: &str) -> Result<(), CodeGenLibError> {
        let var = self.get_var(var_name.into())?;

        self.generated
           .push(Load(Register::RAX, stack(var.1)));

        self.generated.push( Ret );

        Ok(())
    }

    pub fn gen_x_arg_for_func(&mut self, name: &'static str, index: usize, arg: ArgTyp) -> Result<(), CodeGenLibError> {
        // prepare func
        let mut func = (String::new(), vec![]);

        for _func in self.funcs.iter() {
            if _func.0 == name {
                func = _func.to_owned();
            }
        }
        if func.0 == String::new() { // Still uninitalised
            return Err(CodeGenLibError::FuncNotExist(name.into()));
        }

        let mut used_regs = 0;

        for arg in func.1 {
            if {
                match arg {
                    ArgTyp::Int32(_) => true,
                    ArgTyp::Int64(_) => true,
                    _ => false,

                }
            } {
                used_regs += 1;
            }
        }

        if used_regs < 4 {
            match arg {
                ArgTyp::Int32(val) =>   {self.generated.push(MovVal(arg32(used_regs), val as i64)); },
                ArgTyp::Int64(val) =>   {self.generated.push(MovVal(arg64(used_regs), val as i64)); },
                _ => {},
            };

        } else {
            let label_name = format!(".L{}.{}.{}", self.name, name, index);

            self.generated.push(PushLabel(
                &label_name
            ));
        }

        Ok(())
    }

    pub fn build_call(&mut self, func: &'static str, args: Vec<ArgTyp>) -> Result<(), Box<dyn Error>> {
        let mut index = 0;

        for arg in args {
            self.gen_x_arg_for_func(func, index, arg)?;

            index += 1;
        } 

        self.generated.push(Call(func));

        Ok(())
    }

    pub fn set_public(&mut self) {
        self.public = true;
    }
}

pub struct IrBuilder {
    functs: Vec<IrFunctionBuilder>,
    builder: Builder,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self { 
            functs: vec![], 
            builder: Builder::new(),
        }
    }

    pub fn add(&'static mut self, name: &str) -> &mut IrFunctionBuilder {
        let func = IrFunctionBuilder::new(name, &mut self.builder);

        self.functs.push(func);

        self.functs.last_mut().unwrap()
    }

    pub fn builder(&mut self) -> Result<&mut Builder, Box<dyn std::error::Error>> {
        for func in self.functs.iter() {
            let func = func.to_owned();

            self.builder.define(&func.name, func.public, func.generated.clone())?;
        }

        Ok(&mut self.builder)
    }
}
