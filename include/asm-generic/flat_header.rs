/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/uaccess.h>

// The CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS build-time condition is preserved
// with Rust cfg attributes below.
#[inline]
pub unsafe fn flat_get_addr_from_rp(
    rp: *mut u32,
    relval: u32,
    flags: u32,
    addr: *mut u32,
) -> i32 {
    let _ = relval;
    let _ = flags;

    #[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
    {
        if crate::copy_from_user(addr as *mut core::ffi::c_void,
                                 rp as *const core::ffi::c_void,
                                 4) != 0 {
            -crate::EFAULT
        } else {
            0
        }
    }

    #[cfg(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS)]
    {
        crate::get_user(addr, rp)
    }
}

#[inline]
pub unsafe fn flat_put_addr_at_rp(rp: *mut u32, addr: u32, rel: u32) -> i32 {
    let _ = rel;

    #[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
    {
        if crate::copy_to_user(rp as *mut core::ffi::c_void,
                               &addr as *const u32 as *const core::ffi::c_void,
                               4) != 0 {
            -crate::EFAULT
        } else {
            0
        }
    }

    #[cfg(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS)]
    {
        crate::put_user(addr, rp)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
