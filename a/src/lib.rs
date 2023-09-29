static mut STATE: usize = 0;

pub fn set(value: usize) {
    unsafe { STATE = value; }
}

pub fn get() -> usize {
    return unsafe { STATE };
}
