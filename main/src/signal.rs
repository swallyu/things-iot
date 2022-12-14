
pub fn block_waiting_ctrlc(){
    use std::sync::mpsc::channel;

    let (tx,rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("cound not send signal on channel"))
        .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C");
    rx.recv().expect("Count not receive from channel. ");
    
    println!("Got Ctrl-C Exiting...");
}

pub fn install_crash_handler() {
    // unsafe fn set_signal_handler(signal: libc::c_int, handler: unsafe extern "C" fn(libc::c_int)) {
        
        // use libc::{sigaction, sigfillset, sighandler_t};
    //     let mut sigset = std::mem::zeroed();
    //     if sigfillset(&mut sigset) != -1 {
    //         let mut action: sigaction = std::mem::zeroed();
    //         action.sa_mask = sigset;
    //         action.sa_sigaction = handler as sighandler_t;
    //         sigaction(signal, &action, std::ptr::null_mut());
    //     }
    // }

    // unsafe extern "C" fn signal_handler(sig: i32) {
    //     use std::process::abort;

    //     use backtrace::Backtrace;
    //     let name = std::thread::current()
    //         .name()
    //         .map(|n| format!(" for thread \"{}\"", n))
    //         .unwrap_or_else(|| "".to_owned());
    //     eprintln!(
    //         "Signal {}, Stack trace{}\n{:?}",
    //         sig,
    //         name,
    //         Backtrace::new()
    //     );
    //     abort();
    // }

    // unsafe {
    //     // handle segfaults
    //     set_signal_handler(libc::SIGSEGV, signal_handler);
    //     // handle stack overflow and unsupported CPUs
    //     set_signal_handler(libc::SIGILL, signal_handler);
    //     // handle invalid memory access
    //     set_signal_handler(libc::SIGBUS, signal_handler);
    // }
}