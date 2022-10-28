#![no_std]

use core::panic::PanicInfo;


fn main() {
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}
