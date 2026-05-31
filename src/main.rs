// mochou-p/dondon/src/main.rs

mod audio;
mod scheduler;
mod theme;
mod ui;

use std::sync::atomic::{AtomicBool, Ordering};


static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() {
    let (stream,    s2a, u2a, sr) = audio    ::spawn(                    );
    let (scheduler,      u2s    ) = scheduler::spawn(        s2a,      sr);
                                    ui       ::  run(stream, u2a, u2s, sr);
 
    RUNNING.store(false, Ordering::Relaxed);
    scheduler.join().unwrap();
}

