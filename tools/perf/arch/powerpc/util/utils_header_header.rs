/* SPDX-License-Identifier: GPL-2.0 */

/* Original C dependency: <linux/stringify.h>, used to stringify SPR numbers
 * inside the inline assembly macro.
 */

#[macro_export]
macro_rules! mfspr {
    ($rn:expr) => {{
        let rval: ::core::ffi::c_ulong;
        unsafe {
            ::core::arch::asm!(
                concat!("mfspr {0},", stringify!($rn)),
                out(reg) rval,
                options(nostack, preserves_flags)
            );
        }
        rval
    }};
}

/* Processor Version Register */
pub const SPRN_PVR: u32 = 0x11F;

/* Version field */
#[inline]
pub const fn PVR_VER(pvr: u32) -> u32 {
    (pvr >> 16) & 0xFFFF
}

/* Revision field */
#[inline]
pub const fn PVR_REV(pvr: u32) -> u32 {
    (pvr >> 0) & 0xFFFF
}
