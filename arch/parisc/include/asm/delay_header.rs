/* SPDX-License-Identifier: GPL-2.0 */

/// Delay by the specified number of loop iterations.
#[inline]
pub unsafe fn __delay(mut loops: ::core::ffi::c_ulong) {
    ::core::arch::asm!(
        ".balignl 64,0x34000034\n",
        "addib,UV -1,{0},.\n",
        "nop\n",
        inout(reg) loops,
        options(nostack, preserves_flags),
    );
}

unsafe extern "C" {
    pub fn __udelay(usecs: ::core::ffi::c_ulong);
    pub fn __udelay_bad(usecs: ::core::ffi::c_ulong);
}

#[inline]
pub unsafe fn udelay(usecs: ::core::ffi::c_ulong) {
    // Rust has no direct equivalent of C's __builtin_constant_p for a normal
    // function parameter; retain the value guard as the closest local mapping.
    if usecs > 20000 {
        __udelay_bad(usecs);
    }
    __udelay(usecs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
