#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_blog::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_blog::{hlt_loop, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    os_blog::init();

    fn stack_overflow() {
        stack_overflow();
    }

    // stack_overflow();

    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It didn't crash");
    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_blog::test_panic_handler(info)
}
