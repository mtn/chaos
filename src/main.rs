#![feature(panic_implementation)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate spin;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[panic_implementation]
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");
    panic!("Wow such panic!");

    loop {}
}
