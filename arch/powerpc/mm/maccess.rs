// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the corresponding kernel headers:
// linux/uaccess.h, linux/kernel.h, asm/disassemble.h, asm/inst.h,
// and asm/ppc-opcode.h.

unsafe extern "C" {
    fn is_kernel_addr(addr: usize) -> bool;
}

pub unsafe fn copy_from_kernel_nofault_allowed(
    unsafe_src: *const core::ffi::c_void,
    size: usize,
) -> bool {
    let _ = size;
    unsafe { is_kernel_addr(unsafe_src as usize) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
