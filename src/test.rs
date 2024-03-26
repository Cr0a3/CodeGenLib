//! All test of CodeGenLib

use crate::x86::{function::Function, mem::AdressManager};

#[test]
fn function_ret_codegen() {
    let mut adr = AdressManager::new((0, 0));
    let mut func = Function::new("test", &mut adr);
    func.asm_ret();

    let expected_gen: Vec<u8> = vec![
        0xf3, 0x0f, 0x1e, 0xfa,  // endbr64        
        0x55,                   //  push rbp
        0x48, 0x89, 0xe5,       //  mov rbp, rsp
        0x90,                   //  nop
        0x5d,                   //  pop rbp
        0xc3,                   //  ret
    ];

    assert_eq!(expected_gen, func.gen);
}