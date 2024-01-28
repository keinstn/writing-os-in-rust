#![no_std]
#![no_main]

use my_os::println;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    my_os::init();

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // }

    // x86_64::instructions::interrupts::int3();

    use x86_64::registers::control::Cr3;

    let (level4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level4_page_table.start_address()
    );

    println!("It did not crash!");
    my_os::hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}
