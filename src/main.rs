#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"AMAZE! AMAZE! AMAZE!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    vga_buffer::WRITER.lock().write_str("Hola agayn").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numeros: {} {}", 42, 1.338).unwrap();

    loop {}
}
