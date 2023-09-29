use a::{set, get};

pub fn set_inner(value: usize) {
    set(value)
}

pub fn get_inner() -> usize {
    return get();
}