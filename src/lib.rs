#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod vga_buffer;
pub mod interrupts;

#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page

    // let hello = b"Hello World!";
    // let color_byte = 0x1f; // white foreground, blue background

    // let mut hello_colored = [color_byte; 24];
    // for (i, char_byte) in hello.into_iter().enumerate() {
    //     hello_colored[i*2] = *char_byte;
    // }

    // // write `Hello World!` to the center of the VGA text buffer
    // let buffer_ptr = (0xb8000 + 1988) as *mut _;
    // unsafe { *buffer_ptr = hello_colored };
    // vga_buffer::print_something();
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello This is a Test{}", "!");
    init(); // new

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3(); // new
    loop{}
}

// #[lang = "eh_personality"] extern fn eh_personality() {}
// #[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

pub fn init() {
    interrupts::init_idt();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}