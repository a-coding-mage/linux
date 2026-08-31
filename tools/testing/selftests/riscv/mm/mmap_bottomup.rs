// SPDX-License-Identifier: GPL-2.0-only
// C dependencies: <sys/mman.h>, <mmap_test.h>, "kselftest_harness.h"

unsafe extern "C" {
    fn memory_layout() -> ::std::os::raw::c_int;
    static BOTTOM_UP: ::std::os::raw::c_int;
}

#[test]
fn infinite_rlimit() {
    unsafe {
        assert_eq!(BOTTOM_UP, memory_layout());
    }
}

fn main() {}
