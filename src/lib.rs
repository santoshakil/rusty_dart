use std::{
    ffi::{c_char, CString},
    sync::OnceLock,
};

pub static ONCELOCK: OnceLock<String> = OnceLock::new();

#[no_mangle]
pub extern "C" fn init() {
    _ = ONCELOCK.get_or_init(|| {
        println!("OnceLock is initialized!");
        "init".to_string()
    });
}

#[no_mangle]
pub extern "C" fn get() -> c_char {
    let data = ONCELOCK.get().unwrap();
    let c_str = CString::new(data.to_string()).unwrap();
    unsafe { *c_str.into_raw() }
}

#[no_mangle]
pub extern "C" fn set(data: *const c_char) {
    let c_str = unsafe { CString::from_raw(data as *mut c_char) };
    let data = c_str.to_str().unwrap();
    if let Err(err) = ONCELOCK.set(data.to_string()) {
        eprintln!("set error: {}", err);
    }
}
