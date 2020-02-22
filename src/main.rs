#![no_std]
#![no_main]


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        continue;
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}