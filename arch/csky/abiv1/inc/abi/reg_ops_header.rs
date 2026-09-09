/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm/reg_ops.h>

/// Read a control processor register.
#[macro_export]
macro_rules! cprcr {
    ($reg:literal) => {{
        let mut tmp: u32;
        unsafe {
            core::arch::asm!(
                concat!("cprcr {tmp}, ", $reg),
                tmp = out(reg) tmp,
                options(nostack)
            );
        }
        tmp
    }};
}

/// Write a control processor register.
#[macro_export]
macro_rules! cpwcr {
    ($reg:literal, $val:expr) => {{
        unsafe {
            core::arch::asm!(
                concat!("cpwcr {val}, ", $reg),
                val = in(reg) ($val),
                options(nostack)
            );
        }
    }};
}

/// External register-read operation supplied by `asm/reg_ops.h`.
unsafe extern "C" {
    fn mfcr(reg: *const u8) -> u32;
}

#[inline]
pub unsafe fn mfcr_hint() -> u32 {
    unsafe { mfcr(b"cr30\0".as_ptr()) }
}

#[inline]
pub fn mfcr_ccr2() -> u32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
