/* SPDX-License-Identifier: GPL-2.0 */

/*
 * PowerPC architecture timex specifications
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub type cycles_t = ::core::ffi::c_ulong;

unsafe extern "C" {
    pub fn mftb() -> cycles_t;
}

#[inline]
pub unsafe fn get_cycles() -> cycles_t {
    unsafe { mftb() }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
