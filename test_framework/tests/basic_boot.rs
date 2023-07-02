#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  test_framework::test_panic_handler(info)
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
  test_main();

  loop {}
}

#[test_case]
fn test_println() {
  use test_framework::println;
  println!("test_println output");
}
