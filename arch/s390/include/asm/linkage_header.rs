/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes linux/stringify.h for the __ALIGN_STR macro.
// __ALIGN expands to the assembler alignment directive
// `.balign CONFIG_FUNCTION_ALIGNMENT, 0x07`; that directive has no direct
// executable Rust equivalent and is preserved here as source-level intent.
//
// #define __ALIGN .balign CONFIG_FUNCTION_ALIGNMENT, 0x07
// #define __ALIGN_STR __stringify(__ALIGN)

/// Return the current instruction address.
#[inline]
pub unsafe fn _THIS_IP_() -> core::ffi::c_ulong {
    let mut ip: core::ffi::c_ulong;
    core::arch::asm!("larl {0}, .", out(reg) ip);
    ip
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
