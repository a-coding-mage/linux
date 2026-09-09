/* SPDX-License-Identifier: GPL-2.0 */

// #define __ARCH_FORCE_SHMLBA 1
pub const __ARCH_FORCE_SHMLBA: i32 = 1;

unsafe extern "C" {
    pub static mut vac_cache_size: i32;
}

// C macro: SHMLBA (vac_cache_size ? vac_cache_size : PAGE_SIZE)
// PAGE_SIZE is supplied by the surrounding kernel environment.
#[inline]
pub unsafe fn SHMLBA() -> usize {
    if vac_cache_size != 0 {
        vac_cache_size as usize
    } else {
        PAGE_SIZE
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
