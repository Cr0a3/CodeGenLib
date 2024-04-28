use std::error::Error;

use iced_x86::Register;

use crate::{
    abi::Abi, error::CodeGenLibError, Builder
};

pub use super::{Type, AsmInstructionEnum::{self, *}};

#[derive(Debug, Clone)]
pub struct IrFunctionBuilder {
    pub generated: Vec<AsmInstructionEnum>,
    pub name: String,
    args: Vec<((String, u64, Option<Register>), u64)>,
    vars: Vec<(String, i64)>, // i64 -> stack offset
    funcs: Vec<(String, Vec<Type>)>,
    public: bool,

    abi: Abi,

    pub builder: Builder,
}

impl IrFunctionBuilder {
    pub fn new(name: &str, builder: &mut Builder, abi: &Abi) -> Self {
        Self {
            generated: vec![],

            name: name.into(),

            args: vec![],
            vars: vec![],
            funcs: vec![],

            public: false,

            builder: builder.to_owned(),

            abi: abi.to_owned(),
        }
    }

    /// The input tuple values: `(String, u64)` represent:
    ///  * `String` -> The argument name
    ///  * `u64` -> The argument size in bytes
    pub fn args(&mut self, args: Vec<(&str, Type)>) {
        let mut mod_args: Vec<((String, u64, Option<Register>), u64)> = vec![];

        let mut reg_pasted_args = 0;

        let mut prev_size = 0;

        for arg in args {
            let reg: Option<Register> = {
                if reg_pasted_args < self.abi.reg_args() && arg.1.in_reg() {
                    reg_pasted_args += 1;

                    if arg.1.size() == 8 { // u64/i64/str
                        Some(self.abi.arg64(reg_pasted_args - 1))
                    } else {
                        Some(self.abi.arg32(reg_pasted_args - 1))
                    }

                } else {
                    None
                }
            };

            mod_args.push(((arg.0.into(), arg.1.size(), reg), prev_size));

            prev_size += arg.1.size();
        }

        println!("{:?}", mod_args);

        self.args = mod_args;
    }

    /// !Needs to be called after setuped args !
    /// The input tuple values: `(String, u64)` represent:
    ///  * `String` -> The argument name
    ///  * `u64` -> The var size in bytes
    pub fn vars(&mut self, vars: Vec<(&str, Type)>) {
        let mut mod_vars: Vec<(String, i64)> = vec![];
        let mut stack_args: Vec<(String, u64)> = vec![];

        let mut stack_offset: i64 = 8;

        for arg in self.args.iter() {
            let name = &arg.0.0;
            let size = arg.0.1;

            if arg.0.2.is_some() {
                self.generated
                   .push(Store(arg.0.2.unwrap(), self.abi.stack(-stack_offset)));

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

            stack_offset += var.1.size() as i64;
        }

        println!("{:?}", &mod_vars);

        self.vars = mod_vars;
    }

    pub fn efuncs(&mut self, funcs: Vec<(&str, Vec<Type>)>) {

        let mut mod_funcs: Vec<(String, Vec<Type>)> = vec![];

        for func in funcs {
            mod_funcs.push((func.0.into(), func.1));
        }

        self.funcs = mod_funcs;
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
            .push(Load(Register::RAX, self.abi.stack(var1.1)));
        self.generated
            .push(AddMem(Register::RAX, self.abi.stack(var2.1) ));

        self.generated
            .push(Store(Register::RAX, self.abi.stack(ret.1)));

        Ok(())
    }

    pub fn build_return_var(&mut self, var_name: &str) -> Result<(), CodeGenLibError> {
        let var = self.get_var(var_name.into())?;

        self.generated
           .push(Load(Register::RAX, self.abi.stack(var.1)));

        self.generated.push( Ret );

        Ok(())
    }

    pub fn gen_x_arg_for_func(&mut self, name: &str, index: usize, arg: Type) -> Result<(), CodeGenLibError> {
        // prepare func
        let mut func: (String, Vec<Type>) = (String::new(), vec![]);

        for _func in self.funcs.iter() {
            if _func.0 == name {
                func = _func.to_owned();
            }
        }
        if func.0 == String::new() { // Still uninitalised
            return Err(CodeGenLibError::FuncNotExist(name.into()));
        }

        let mut used_regs = 0;

        for _arg in func.1 {
            if _arg.empty() == arg.empty() { break; }

            if _arg.in_reg() {
                used_regs += 1;
            }
        }

        println!("arg {} -> {:?}", index, arg);
        println!("used regs {}", used_regs);

        if used_regs <= self.abi.reg_args() && arg.in_reg() {
            match arg {
                Type::u32(val) =>   {self.generated.push(MovVal(self.abi.arg32(used_regs), val as i64)); },
                Type::i32(val) =>   {self.generated.push(MovVal(self.abi.arg32(used_regs), val as i64)); },
                Type::u64(val) =>   {self.generated.push(MovVal(self.abi.arg64(used_regs), val as i64)); },
                Type::i64(val) =>   {self.generated.push(MovVal(self.abi.arg64(used_regs), val as i64)); },
                Type::Str(content) => {
                    let label_name = format!("{}.{}.{}", self.name, name, index);

                    self.builder.define_label(&label_name, false, content);

                    self.generated.push(MovPtr(self.abi.arg64(index), label_name));
                },
                _ => {},
            };

        } else {
            match arg {
                Type::Str(content) => {
                    let label_name = format!("{}.{}.{}", self.name, name, index);

                    self.builder.define_label(&label_name, false, content);

                    self.generated.push(PushPtr(label_name));
                },

                Type::u64(val) => { self.generated.push(PushVal(val as i64)) },
                Type::i64(val) => { self.generated.push(PushVal(val as i64)) },
                Type::u32(val) => { self.generated.push(PushVal(val as i64)) },
                Type::i32(val) => { self.generated.push(PushVal(val as i64)) },

                arg => {
                    let label_name = format!("{}.{}.{}", self.name, name, index);

                    self.builder.define_label(&label_name, false, arg.bytes());
        
                    self.generated.push(PushLabel(
                        label_name
                    ));
                }
            }
        }

        Ok(())
    }

    pub fn build_call(&mut self, func: &str, args: Vec<Type>) -> Result<(), Box<dyn Error>> {
        let mut index = 0;

        for arg in args {
            self.gen_x_arg_for_func(func, index + 1, arg)?;

            index += 1;
        } 

        self.generated.push(Call(func.into()));

        Ok(())
    }

    pub fn set_public(&mut self) {
        self.public = true;
    }
}

pub struct IrBuilder {
    functs: Vec<IrFunctionBuilder>,
    pub build: Builder,

    abi: Abi,
}

impl IrBuilder {
    pub fn new(target: Abi) -> Self {
        println!("{:?}", target);
        Self { 
            functs: vec![], 
            build: Builder::new(),
            abi: target,
        }
    }

    pub fn add(&mut self, name: &str) -> &mut IrFunctionBuilder {
        self.functs.push(
            IrFunctionBuilder::new(name, &mut self.build, &self.abi)
        );

        self.functs.last_mut().unwrap()
    }

    pub fn write(&mut self, outpath: &str) -> Result<(), Box<dyn std::error::Error>> {
        for func in self.functs.iter() {
            let func = func.to_owned();

            self.build.sync(&func.builder);

            self.build.define(&func.name, func.public, func.generated.clone())?;
        }

        self.build.write(outpath, self.abi.binary_format())
    }
}
