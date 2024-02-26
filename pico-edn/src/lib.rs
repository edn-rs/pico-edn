#![no_main]
#![allow(unsafe_code)]
#![no_std]

extern crate alloc;

use alloc::ffi::CString;
use alloc::string::ToString;
use core::ffi::CStr;
use core::panic::PanicInfo;
use core::str::FromStr;

use edn_rs::Edn;

use edn_derive::{Deserialize, Serialize};
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

extern "C" {
    fn printf(format: *const i8, ...) -> i32;
}

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<4096> = emballoc::Allocator::new();

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[no_mangle]
pub extern "C" fn both_ways() {
    let point = Point { x: 1, y: 2 };

    let serialized = edn_rs::to_string(&point);
    let deserialized: Point = edn_rs::from_str(&serialized).unwrap();

    assert_eq!(point, deserialized);
}

#[no_mangle]
pub extern "C" fn some_edn(edn: *const i8) {
    let c_str: &CStr = unsafe { CStr::from_ptr(edn) };
    let str_slice: &str = c_str.to_str().unwrap();

    let edn = Edn::from_str(str_slice).unwrap();

    let edn_str = Edn::to_string(&edn);
    let c_str = CString::new(edn_str.as_str()).unwrap();
    unsafe {
        printf("hello edn %s\n\0".as_ptr() as *const i8, c_str.as_ptr());
    }
}
