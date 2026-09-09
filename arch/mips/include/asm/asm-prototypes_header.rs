/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the corresponding architecture and generic headers:
// asm/checksum.h, asm/page.h, asm/fpu.h, asm-generic/asm-prototypes.h,
// linux/uaccess.h, asm/ftrace.h, and asm/mmu_context.h.

extern "C" {
    pub fn clear_page_cpu(page: *mut core::ffi::c_void);
    pub fn copy_page_cpu(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
