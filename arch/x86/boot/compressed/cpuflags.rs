// SPDX-License-Identifier: GPL-2.0
//! Native compressed-boot CPU flag lookup.

#[path = "../cpuflags.rs"]
mod shared;

use core::ffi::c_int;
use core::ptr::addr_of;

#[no_mangle]
pub(crate) unsafe extern "C" fn has_cpuflag(flag: c_int) -> bool {
    // SAFETY: the caller supplies a valid boot feature bit index. bitops.h
    // indexes 32-bit words, even on x86-64; a C bool is returned, not an int or
    // an uninitialized full register populated only by a one-byte SETcc.
    unsafe {
        shared::get_cpuflags();
        let words = addr_of!(shared::CPU.flags).cast::<u32>();
        (words.offset((flag >> 5) as isize).read() & (1u32 << (flag & 31))) != 0
    }
}
