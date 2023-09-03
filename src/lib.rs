#[no_mangle]
pub extern "C" fn libduckdb_custom_ext_init(_db: *mut std::ffi::c_void) {
    println!("hello from extension!");
}
