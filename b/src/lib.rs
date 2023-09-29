use a::{set, get};

pub fn set_inner(value: usize) {
    set(value)
}

pub fn get_inner() -> usize {
    return get();
}

#[no_mangle]
pub extern "C" fn set_inner_c(value: usize) {
    set(value)
}

#[no_mangle]
pub extern "C" fn get_inner_c() -> usize {
    return get();
}