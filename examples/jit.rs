use CodeGenLib::{Function, Jit, AdressManager, Result};

// Dosn't work

pub fn main() -> Result<()> {
    let mut adr = AdressManager::new((0, 0));
    let mut func = Function::new(
        "five",
        &mut adr
    );

    func.ret_int(5);

    unsafe {
        let typed = func.typed::<(), u32>().unwrap();
        let res = typed();

        println!("5 = {}", res);
    };

    Ok(())
}