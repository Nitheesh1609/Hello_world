#![feature(lang_items)]
#![no_std]


use core::panic::PanicInfo;

#[macro_use]
mod macros;


#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    unsafe { panic(to_ptr!(c_string!("Rust panic was triggered"))) }
}

extern "C"{


    fn panic(msg: *const i8, ...) -> !;
    fn printk(msg: *const i8, ...);
}

#[no_mangle]
pub extern "C" fn init_module()->i32{
            unsafe {
                printk!("\n Hello World")
            };
return 0;
}
#[no_mangle]
pub extern "C" fn cleanup_module() {
 unsafe{
printk!("\nEnd Module Bye!")   
};
}
