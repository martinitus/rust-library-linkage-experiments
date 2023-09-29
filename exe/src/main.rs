use a;
use b;
use c;

use std::ffi::c_void;


fn main() {
    println!("exe sees: {}", a::get());
    println!("b sees: {}", b::get_inner());
    println!("c sees: {}", c::get_inner());

    println!("================");

    println!("setting to 1 directly via a");
    a::set(1);
    println!("exe sees: {}", a::get());
    println!("b sees: {}", b::get_inner());
    println!("c sees: {}", c::get_inner());

    println!("================");

    println!("setting to 2 via b");
    b::set_inner(2);
    println!("exe sees: {}", a::get());
    println!("b sees: {}", b::get_inner());
    println!("c sees: {}", c::get_inner());

    println!("=======DYNAMIC LOADING=========");

    unsafe {
        let (b, c) = if cfg!(target_os = "linux") {
            (libloading::Library::new("./target/debug/libb.so").unwrap(),
             libloading::Library::new("./target/debug/libc.so").unwrap())
        } else {
            (libloading::Library::new("./target/debug/b.dll").unwrap(),
             libloading::Library::new("./target/debug/c.dll").unwrap())
        };

        let setter = b.get::<extern "C" fn(usize) -> *mut c_void>("set_inner_c".as_bytes()).unwrap();
        let getter = b.get::<extern "C" fn() -> usize>("get_inner_c".as_bytes()).unwrap();
        let c_getter = c.get::<extern "C" fn() -> usize>("get_inner_c2".as_bytes()).unwrap();
        println!("b-dyn-loaded sees: {}", getter());

        println!("setting to 3via dyn-loaded-b");

        setter(3);

        println!("exe sees: {}", a::get());
        println!("b sees: {}", b::get_inner());
        println!("b-dyn-loaded sees: {}", getter());
        println!("c sees: {}", c::get_inner());
        println!("c-dyn-loaded sees: {}", c_getter());
    }
}
