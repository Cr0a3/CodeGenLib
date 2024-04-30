#[cfg(test)]
mod tests {
    use std::error::Error;

    use CodeGenLib::{target::{linux::LinuxAbi, windows::WindowsAbi, Abi}, IR::Register};

    #[test]
    fn windows() -> Result<(), Box<dyn Error>> {
        // test return regs
        assert_eq!(Abi::windows().arg64(0), Register::RCX);
        assert_eq!(Abi::windows().arg64(1), Register::RDX);
        assert_eq!(Abi::windows().arg64(2), Register::R8);
        assert_eq!(Abi::windows().arg64(3), Register::R9);

        // test return regs len
        assert_eq!(Abi::windows().reg_args, 4);

        Ok(())
    }

    #[test]
    fn linux() -> Result<(), Box<dyn Error>> {
        // test return regs
        assert_eq!(Abi::linux().arg64(0), Register::RDI);
        assert_eq!(Abi::linux().arg64(1), Register::RSI);
        assert_eq!(Abi::linux().arg64(2), Register::RDX);
        assert_eq!(Abi::linux().arg64(3), Register::RCX);
        assert_eq!(Abi::linux().arg64(4), Register::R8);
        assert_eq!(Abi::linux().arg64(5), Register::R9);

        // test return regs len
        assert_eq!(Abi::linux().reg_args, 6);

        Ok(())
    }
}