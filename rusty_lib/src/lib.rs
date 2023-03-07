use std::{ffi::CStr, os::raw::c_char, path::Path, sync::mpsc::channel};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

#[no_mangle]
pub extern "C" fn watch(p: *const c_char, callback: extern "C" fn(*const c_char)) {
    let path = Path::new(unsafe { CStr::from_ptr(p).to_str().unwrap() });

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    watcher
        .watch(path.as_ref(), RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                let msg = format!("changed: {:?}", event);
                let c_msg = std::ffi::CString::new(msg).unwrap();
                let ptr = c_msg.as_ptr();
                callback(ptr);
            }
            Err(e) => {
                let msg = format!("watch error: {:?}", e);
                let c_msg = std::ffi::CString::new(msg).unwrap();
                let ptr = c_msg.as_ptr();
                callback(ptr);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
