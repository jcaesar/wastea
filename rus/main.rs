use std::time::{SystemTime, UNIX_EPOCH};
use wasmer::{imports, Array, Function, Instance, Module, Store, WasmPtr};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = include_bytes!("../tea/target/generated/teavm/javamod.wasm");
    let store = Store::new(&Universal::new(Cranelift::default()).engine());
    let module = Module::new(&store, wasm_bytes)?;

    let import_object = imports! {
        "teavm" => {
            "logInt" => Function::new_native(&store, |i: i32| println!("TeaVM logInt: {}", i)),
            "logString" => Function::new_native(&store, |_ptr: WasmPtr<u8, Array>| println!("TeaVM logString: [TODO]")),
            "logOutOfMemory" => Function::new_native(&store, || println!("TeaVM: OOM") /* TODO: Abort? */),
            "currentTimeMillis" => Function::new_native(&store, ||
                SystemTime::now().duration_since(UNIX_EPOCH).expect("Land before our time").as_secs_f64() * 1e3
            ),
            "putwchar" => Function::new_native(&store, |c: u32| print!("{}", char::from_u32(c).expect("invalid char"))) // TODO: Buffer for performance
        },
        "teavmHeapTrace" => {
            "init" => Function::new_native(&store, |_i: i32| ()),
        },
        // For the sake of this example
        "teavm_unchained" => {
            "inc_by" => Function::new_native(&store, || 42),
        }
    };

    let instance = Instance::new(&module, &import_object)?;

    // Legend has it we have to call main once, first.
    instance
        .exports
        .get::<Function>("start")?
        .native::<i32, ()>()?
        .call(0)?;

    let function = instance
        .exports
        .get::<Function>("inc")?
        .native::<(), i32>()?;

    for _ in 0..3 {
        println!("Current counter (Host): {}", function.call()?);
    }

    Ok(())
}
