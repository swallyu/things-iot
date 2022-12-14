#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

use std::sync::Arc;

use crate::signal::block_waiting_ctrlc;
use tokio::runtime::Runtime;

mod signal;
mod mqtt;
mod http;

fn main() ->Result<(),std::io::Error> {
    println!("Hello, world!");

    let runtime = init_runtime(None)?;
    let runtime = Arc::new(runtime);

    runtime.clone().block_on(async move{
        println!("running in runtime ");

        signal::block_waiting_ctrlc();
    });

    
    Ok(())
}

fn init_runtime(cores:Option<usize>) ->Result<Runtime, std::io::Error> {
    use tokio::runtime::Builder;    
    match cores {
        None => Runtime::new(),
        Some(cores)=> match cores {
            0 => {
                let msg=format!("invalid core numbers {} ",cores);
                Err(std::io::Error::new(std::io::ErrorKind::Other, msg))
            }
            _=>Builder::new_multi_thread().enable_all().worker_threads(cores).build()
        }
    }
}