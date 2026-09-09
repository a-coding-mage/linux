/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/asm-m68k/timex.h
 *
 * m68k architecture timex specifications
 */

pub type cycles_t = u32;

#[inline]
pub fn get_cycles() -> cycles_t {
    0
}

extern "C" {
    pub static mut mach_random_get_entropy: Option<unsafe extern "C" fn() -> libc::c_ulong>;
    pub fn random_get_entropy_fallback() -> libc::c_ulong;
}

#[inline]
pub unsafe fn random_get_entropy() -> libc::c_ulong {
    if let Some(get_entropy) = mach_random_get_entropy {
        return get_entropy();
    }
    random_get_entropy_fallback()
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
