//Code from https://github.com/bytecodealliance/wasmtime/blob/main/examples/hello.rs
use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    println!("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_file(&engine, "hello.wat")?;

    println!("Initializing...");
    let mut store = Store::new(
        &engine,
        ()
    );

    println!("Creating callback...");
    let hello_func = Func::wrap(&mut store, |_caller: Caller<'_, ()>| {
        println!("Calling back...");
    });

    println!("Instantiating module...");
    let imports = [hello_func.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;

    println!("Extracting export...");
    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;

    println!("Calling export...");
    run.call(&mut store, ())?;

    println!("Done.");
    Ok(())
}
