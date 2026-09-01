// SPDX-License-Identifier: GPL-2.0
// C dependency intent: <stdlib.h> supplies NULL, and libunwind headers supply
// UNW_OBJ, unw_addr_space_t, unw_word_t, unw_dyn_info_t, unw_proc_info_t,
// unw_accessors_t, unw_create_addr_space, and unw_init_remote.

use core::ffi::{c_int, c_void};
use core::mem::MaybeUninit;
use core::ptr;

pub type unw_addr_space_t = *mut c_void;
pub type unw_word_t = usize;

#[repr(C)]
pub struct unw_dyn_info_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unw_proc_info_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unw_accessors_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    // C: extern int UNW_OBJ(dwarf_search_unwind_table)(...);
    // C macro intent: #define dwarf_search_unwind_table UNW_OBJ(dwarf_search_unwind_table)
    fn dwarf_search_unwind_table(
        as_: unw_addr_space_t,
        ip: unw_word_t,
        di: *mut unw_dyn_info_t,
        pi: *mut unw_proc_info_t,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;

    fn unw_create_addr_space(accessors: *mut unw_accessors_t, byteorder: c_int) -> unw_addr_space_t;
    fn unw_init_remote(cursor: *mut c_void, as_: unw_addr_space_t, arg: *mut c_void) -> c_int;
}

static mut accessors: MaybeUninit<unw_accessors_t> = MaybeUninit::uninit();

pub unsafe fn main() -> c_int {
    let addr_space: unw_addr_space_t;

    addr_space = unw_create_addr_space(accessors.as_mut_ptr(), 0);
    if !addr_space.is_null() {
        return 0;
    }

    unw_init_remote(ptr::null_mut(), addr_space, ptr::null_mut());
    dwarf_search_unwind_table(
        addr_space,
        0,
        ptr::null_mut(),
        ptr::null_mut(),
        0,
        ptr::null_mut(),
    );

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
