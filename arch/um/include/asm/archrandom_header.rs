/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to <linux/types.h>.

/* This is from <os.h>, but better not to include that in a global header here. */
unsafe extern "C" {
    pub fn os_getrandom(buf: *mut core::ffi::c_void, len: usize, flags: u32) -> isize;
}

#[inline]
#[must_use]
pub unsafe fn arch_get_random_longs(v: *mut usize, max_longs: usize) -> usize {
    let ret: isize;

    ret = unsafe {
        os_getrandom(
            v.cast::<core::ffi::c_void>(),
            max_longs.wrapping_mul(core::mem::size_of::<usize>()),
            0,
        )
    };
    if ret < 0 {
        return 0;
    }
    (ret as usize) / core::mem::size_of::<usize>()
}

#[inline]
#[must_use]
pub unsafe fn arch_get_random_seed_longs(_v: *mut usize, _max_longs: usize) -> usize {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
