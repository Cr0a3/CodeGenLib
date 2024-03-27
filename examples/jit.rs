use CodeGenLib::{Function, Jit, mem::AdressManager};

pub fn main() { // needs feature jit
    let mut func = Function::new(
        "add",
        &mut AdressManager::new((0, 0))
    );

    func.ret_int(5);

    unsafe {
        let typed = func.typed::<(), u32>();
        let res = typed();

        println!("called function add. 5 = {}", res);
    };
}