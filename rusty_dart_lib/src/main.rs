use std::{
    ffi::{c_char, c_int, CStr, CString},
    fs,
    path::Path,
};

fn main() {
    let source = "/Users/agmacone/Projects/Dart/rusty_dart/dummy/source";
    let destination = "/Users/agmacone/Projects/Dart/rusty_dart/dummy/dest";

    let source_cstring = CString::new(source).unwrap();
    let destination_cstring = CString::new(destination).unwrap();

    copy_dir(source_cstring.as_ptr(), destination_cstring.as_ptr());
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
