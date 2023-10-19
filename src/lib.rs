use std::{
    collections::HashMap,
    ffi::{c_char, CStr, CString},
    sync::RwLock,
};

use once_cell::sync::Lazy;

pub static MAP: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| RwLock::new(HashMap::new()));

#[no_mangle]
pub extern "C" fn get() -> *const c_char {
    let data = MAP
        .read()
        .unwrap()
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("\n");
    println!("Rust: {}", data.clone());
    let data = CString::new(data).unwrap();
    data.into_raw()
}

#[no_mangle]
pub extern "C" fn set(key: *const c_char, value: *const c_char) {
    let key = unsafe { CStr::from_ptr(key).to_string_lossy().into_owned() };
    let value = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };
    MAP.write().unwrap().insert(key, value);
}
