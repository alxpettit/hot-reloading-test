#[cfg(feature = "reload")]
use hot_lib::*;
#[cfg(not(feature = "reload"))]
use lib::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    // Reads public no_mangle functions from lib.rs and  generates hot-reloadable
    // wrapper functions with the same signature inside this module.
    // Note that this path relative to the project root (or absolute)
    hot_functions_from_file!("lib/src/lib.rs");

    // Because we generate functions with the exact same signatures,
    // we need to import types used
    pub use lib::State;
}

fn main() {
    let mut state = State { counter: 0 };
    // Running in a loop so you can modify the code and see the effects
    loop {
        step(&mut state);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
