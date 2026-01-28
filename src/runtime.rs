use wasmtime::{Engine, Linker, Module, Store, Config, Instance, Caller};
use wasmtime_wasi::preview2::{self, WasiCtx, Table, WasiCtxBuilder, WasiView};
use wasmtime_wasi::preview2::preview1::{self, WasiPreview1View};
use anyhow::Result;
use tokio::sync::mpsc;
use crate::network::{self, Message};

struct NovaState {
    table: Table,
    ctx: WasiCtx,
    adapter: preview1::WasiPreview1Adapter,
    pid: u32,
    inbox: mpsc::Receiver<Message>,
}

impl WasiView for NovaState {
    fn table(&self) -> &Table { &self.table }
    fn table_mut(&mut self) -> &mut Table { &mut self.table }
    fn ctx(&self) -> &WasiCtx { &self.ctx }
    fn ctx_mut(&mut self) -> &mut WasiCtx { &mut self.ctx }
}

impl WasiPreview1View for NovaState {
    fn adapter(&self) -> &preview1::WasiPreview1Adapter { &self.adapter }
    fn adapter_mut(&mut self) -> &mut preview1::WasiPreview1Adapter { &mut self.adapter }
}

pub async fn run_wasm(path: &str, pid: u32, fuel: u64, memory_limit_mb: Option<u64>, map_dir: Option<String>) -> Result<()> {
    // 1. Configure Wasmtime
    let mut config = Config::new();
    config.async_support(true);
    config.consume_fuel(true); // Enable fuel metering
    config.cranelift_opt_level(wasmtime::OptLevel::Speed);

    if let Some(mb) = memory_limit_mb {
        let bytes = mb * 1024 * 1024;
        config.static_memory_maximum_size(bytes); 
    }

    let engine = Engine::new(&config)?;
    let mut linker = Linker::<NovaState>::new(&engine);
    
    // 2. Link WASI Preview 1 adapter
    preview1::add_to_linker_async(&mut linker)?;

    // Link Custom Networking Functions
    linker.func_wrap("env", "nova_send", |mut caller: Caller<'_, NovaState>, target_pid: u32, ptr: u32, len: u32| {
        let mem = match caller.get_export("memory") {
            Some(wasmtime::Extern::Memory(m)) => m,
            _ => return, // Handle error gracefully in real code
        };
        
        let mut buf = vec![0u8; len as usize];
        if let Err(_) = mem.read(&caller, ptr as usize, &mut buf) {
             return; 
        }

        // Send asynchronously (spawn a task or block? func_wrap is sync by default unless async func_wrap used)
        // For prototype, we'll spawn a detached task to send, to avoid blocking the wasm thread heavily.
        tokio::spawn(async move {
            let _ = network::GLOBAL_SWITCH.send(target_pid, buf).await;
        });
    })?;

    // nova_recv(ptr, len) -> bytes_written
    linker.func_wrap2_async("env", "nova_recv", |mut caller: Caller<'_, NovaState>, ptr: u32, len: u32| {
        Box::new(async move {
            let inbox = &mut caller.data_mut().inbox;
            
            // Try to receive. This is async!
            if let Some(msg) = inbox.recv().await {
                 let mem = match caller.get_export("memory") {
                    Some(wasmtime::Extern::Memory(m)) => m,
                    _ => return 0,
                };
                
                let bytes_to_write = std::cmp::min(len as usize, msg.len());
                if let Err(_) = mem.write(&mut caller, ptr as usize, &msg[0..bytes_to_write]) {
                    return 0;
                }
                bytes_to_write as u32
            } else {
                0 // Channel closed
            }
        })
    })?;

    // nova_get_pid() -> u32
    linker.func_wrap("env", "nova_get_pid", |caller: Caller<'_, NovaState>| -> u32 {
        caller.data().pid
    })?;


    // 3. Create context
    let table = Table::new();
    let mut builder = WasiCtxBuilder::new();
    builder.inherit_stdio();

    if let Some(map_dir) = map_dir {
        let parts: Vec<&str> = map_dir.split(':').collect();
        if parts.len() == 2 {
            let host_path = parts[0];
            let guest_path = parts[1];
            println!("📂 Mounting '{}' to '{}'", host_path, guest_path);
            
            let dir = std::fs::File::open(host_path)?; // Open host directory
            let wasi_dir = wasmtime_wasi::Dir::from_std_file(dir);
            builder.preopened_dir(
                wasi_dir,
                wasmtime_wasi::preview2::DirPerms::all(), 
                wasmtime_wasi::preview2::FilePerms::all(),
                guest_path,
            );
        } else {
             eprintln!("⚠️ Invalid map-dir format. Expected host:guest");
        }
    }

    let ctx = builder.build();
    let adapter = preview1::WasiPreview1Adapter::new();

    // Register with the global switch
    let inbox = network::GLOBAL_SWITCH.register(pid);

    let state = NovaState { table, ctx, adapter, pid, inbox };
    let mut store = Store::new(&engine, state);
    store.set_fuel(fuel)?; // Set the fuel limit

    // 4. Load module
    let module = Module::from_file(&engine, path)?;

    // 5. Instantiate
    let instance: Instance = linker.instantiate_async(&mut store, &module).await?;

    // 6. Run
    let start_func = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    start_func.call_async(&mut store, ()).await?;

    Ok(())
}
