#![no_std]
#![no_main]

use my_os::println;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    my_os::init();

    x86_64::instructions::interrupts::int3();

    println!("It did not crash!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}
