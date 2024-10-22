#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"sic transit gloria mundi";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        continue;
    }
}

/// Example from https://os.phil-opp.com/freestanding-rust-binary/ (Accessed Feb 2020)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}