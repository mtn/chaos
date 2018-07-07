#![feature(panic_implementation)]
#![feature(core_intrinsics)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[cfg(test)]
extern crate std;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate array_init;
extern crate spin;
extern crate uart_16550;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;
mod serial;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");
    panic!("Wow such panic!");

    loop {}
}
