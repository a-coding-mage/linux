/* SPDX-License-Identifier: GPL-2.0 */
/* x86-specific clocksource additions */

// Dependency supplied by <asm/vdso/clocksource.h>.

extern "C" {
    pub static mut vclocks_used: core::ffi::c_uint;
}

#[inline]
pub unsafe fn vclock_was_used(vclock: core::ffi::c_int) -> bool {
    unsafe {
        core::ptr::read_volatile(&vclocks_used) & (1u32 << vclock as u32) != 0
    }
}

#[inline]
pub unsafe fn vclocks_set_used(which: core::ffi::c_uint) {
    unsafe {
        let value = core::ptr::read_volatile(&vclocks_used)
            | (1u32 << which);
        core::ptr::write_volatile(&mut vclocks_used, value);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
