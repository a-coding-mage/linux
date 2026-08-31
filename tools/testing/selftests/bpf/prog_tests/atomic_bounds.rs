// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "atomic_bounds.skel.h"

#[repr(C)]
pub struct atomic_bounds {
    _private: [u8; 0],
}

extern "C" {
    fn atomic_bounds__open_and_load() -> *mut atomic_bounds;
    fn atomic_bounds__destroy(skel: *mut atomic_bounds);
    fn CHECK(condition: bool, name: *const ::std::os::raw::c_char, format: *const ::std::os::raw::c_char, ...) -> bool;
}

pub unsafe fn test_atomic_bounds() {
    let mut skel: *mut atomic_bounds;
    let duration: u32 = 0;

    skel = atomic_bounds__open_and_load();
    if CHECK(
        skel.is_null(),
        b"skel_load\0".as_ptr() as *const ::std::os::raw::c_char,
        b"couldn't load program\n\0".as_ptr() as *const ::std::os::raw::c_char,
    ) {
        return;
    }

    atomic_bounds__destroy(skel);
}
