#![no_std]
#![no_main]

use core::panic::PanicInfo;
use test_framework::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  serial_println!("[ok]");
  exit_qemu(QemuExitCode::Success);
  loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
  should_fail();
  serial_println!("[test did not panic]");
  exit_qemu(QemuExitCode::Failed);

  loop {}
}

fn should_fail() {
  serial_print!("test that should always fail...\t");
  assert_eq!(0, 1);
}
