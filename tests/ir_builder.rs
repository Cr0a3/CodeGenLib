#[cfg(test)]
mod tests {
    use std::error::Error;

    use CodeGenLib::{ir::{AsmInstructionEnum::*, IrFunctionBuilder}, target::{linux::LinuxAbi, Abi}, Builder, IR::Register};

    #[test]
    fn ir_gen() -> Result<(), Box<dyn Error>> {
        let mut builder = Builder::new();
        let mut func = IrFunctionBuilder::new("test", &mut builder, &Abi::linux());

        func.build_return_int(5)?;

        assert_eq!(
            func.generated,
            vec![
                MovVal(Register::RAX, 5),
                Ret
            ]);

        Ok(())
    }
}