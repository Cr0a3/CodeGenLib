use crate::{Function, Jit, AdressManager, Result};

#[test]
pub fn jit_sum() -> Result<()> {
    let mut adr = AdressManager::new((0, 0));
    let mut func = Function::new(
        "add",
        &mut adr
    );

    func.ret_int(5);

    unsafe {
        let typed = func.typed::<(), u32>().unwrap();
        let res = typed();

        assert_eq!(5, res);
    };

    Ok(())
}