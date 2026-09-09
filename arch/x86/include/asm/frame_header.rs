/* SPDX-License-Identifier: GPL-2.0 */

/*
 * These are stack frame creation macros. They should be used by every
 * callable non-leaf asm function to make kernel stack traces more reliable.
 *
 * The original header is also consumed by the assembler. The following
 * string constants preserve the inline-assembly forms of those macros.
 */

#[cfg(feature = "CONFIG_FRAME_POINTER")]
pub const FRAME_BEGIN: &str = "push %rbp\n\tmov %rsp, %rbp\n";

#[cfg(feature = "CONFIG_FRAME_POINTER")]
pub const FRAME_END: &str = "pop %rbp\n";

#[cfg(all(feature = "CONFIG_FRAME_POINTER", target_arch = "x86_64"))]
pub const ENCODE_FRAME_POINTER: &str = "lea 1(%rsp), %rbp\n\t";

#[cfg(all(feature = "CONFIG_FRAME_POINTER", target_arch = "x86"))]
pub const ENCODE_FRAME_POINTER: &str =
    "movl %esp, %ebp\n\tandl $0x7fffffff, %ebp\n\t";

/*
 * This is a sneaky trick to help the unwinder find pt_regs on the stack. The
 * frame pointer is replaced with an encoded pointer to pt_regs. The encoding
 * is just setting the LSB on x86-64, or clearing the MSB on 32-bit x86.
 *
 * `pt_regs` is supplied by the architecture's other headers.
 */
#[cfg(all(feature = "CONFIG_FRAME_POINTER", target_arch = "x86_64"))]
#[inline]
pub unsafe fn encode_frame_pointer(regs: *mut pt_regs) -> ::core::ffi::c_ulong {
    regs as ::core::ffi::c_ulong + 1
}

#[cfg(all(feature = "CONFIG_FRAME_POINTER", target_arch = "x86"))]
#[inline]
pub unsafe fn encode_frame_pointer(regs: *mut pt_regs) -> ::core::ffi::c_ulong {
    (regs as ::core::ffi::c_ulong) & 0x7fffffff
}

/* __ASM_SEL(4, 8): the selected frame offset follows the target word size. */
#[cfg(all(feature = "CONFIG_FRAME_POINTER", target_pointer_width = "64"))]
pub const FRAME_OFFSET: usize = 8;

#[cfg(all(feature = "CONFIG_FRAME_POINTER", target_pointer_width = "32"))]
pub const FRAME_OFFSET: usize = 4;

#[cfg(not(feature = "CONFIG_FRAME_POINTER"))]
pub const FRAME_BEGIN: &str = "";

#[cfg(not(feature = "CONFIG_FRAME_POINTER"))]
pub const FRAME_END: &str = "";

#[cfg(not(feature = "CONFIG_FRAME_POINTER"))]
pub const ENCODE_FRAME_POINTER: &str = "";

#[cfg(not(feature = "CONFIG_FRAME_POINTER"))]
pub const FRAME_OFFSET: usize = 0;

#[cfg(not(feature = "CONFIG_FRAME_POINTER"))]
#[inline]
pub unsafe fn encode_frame_pointer(_regs: *mut pt_regs) -> ::core::ffi::c_ulong {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
