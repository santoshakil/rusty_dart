use std::{
    ffi::{c_char, c_int, CStr, CString},
    fs,
    path::Path,
    sync::mpsc::channel,
};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

fn main() {
    let source = "/Users/agmacone/Projects/Dart/rusty_dart/dummy/source";
    let destination = "/Users/agmacone/Projects/Dart/rusty_dart/dummy/dest";

    let source_cstring = CString::new(source).unwrap();
    let destination_cstring = CString::new(destination).unwrap();

    copy_dir(source_cstring.as_ptr(), destination_cstring.as_ptr());
}

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
pub extern "C" fn copy_dir(src: *const c_char, dst: *const c_char) -> c_int {
    let source = Path::new(unsafe { CStr::from_ptr(src).to_str().unwrap() });
    let destination = Path::new(unsafe { CStr::from_ptr(dst).to_str().unwrap() });
    if source.is_dir() {
        fs::create_dir_all(destination).unwrap();
        for entry in fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let dest_path = destination.join(entry.file_name());
            if entry_path.is_dir() {
                fs::create_dir_all(&entry_path).unwrap();
            } else {
                fs::copy(&entry_path, &dest_path).unwrap();
            }
        }
    } else {
        fs::copy(source, destination).unwrap();
    }
    0
}
