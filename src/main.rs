#![feature(panic_implementation)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub fn _start() -> ! {
    let slice = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };

    slice[0] = b'h';
    slice[1] = 0xf;
    slice[2] = b'e';
    slice[3] = 0xf;
    slice[4] = b'l';
    slice[5] = 0xf;
    slice[6] = b'l';
    slice[7] = 0xf;
    slice[8] = b'o';
    slice[9] = 0xf;
    slice[10] = b' ';
    slice[11] = 0xf;
    slice[12] = b'w';
    slice[13] = 0xf;
    slice[14] = b'o';
    slice[15] = 0xf;
    slice[16] = b'r';
    slice[17] = 0xf;
    slice[18] = b'l';
    slice[19] = 0xf;
    slice[20] = b'd';
    slice[21] = 0xf;

    loop {}
}
