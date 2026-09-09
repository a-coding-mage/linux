/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies provided by the corresponding architecture and generic headers:
// asm/ftrace.h, linux/uaccess.h, linux/pgtable.h, asm/string.h, asm/page.h,
// asm/checksum.h, asm/mce.h, asm-generic/asm-prototypes.h,
// asm/special_insns.h, asm/preempt.h, asm/asm.h, asm/fred.h, asm/gsseg.h,
// asm/nospec-branch.h

// !CONFIG_X86_CX8
#[cfg(not(feature = "CONFIG_X86_CX8"))]
unsafe extern "C" {
    pub fn cmpxchg8b_emu();
}

// CONFIG_STACKPROTECTOR
#[cfg(feature = "CONFIG_STACKPROTECTOR")]
unsafe extern "C" {
    pub static mut __ref_stack_chk_guard: ::core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
