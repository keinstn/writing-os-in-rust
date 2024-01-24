#![no_std]
#![no_main]

use my_os::println;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    my_os::init();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }

    x86_64::instructions::interrupts::int3();

    println!("It did not crash!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}
