/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <asm/page_types.h>
// #include <linux/linkage.h>
// #include <linux/init.h>
// #include <linux/mm_types.h>

use core::ffi::c_void;

#[repr(C)]
pub struct vdso_image {
    pub data: *mut c_void,
    pub size: usize, // Always a multiple of PAGE_SIZE

    pub alt: usize,
    pub alt_len: usize,
    pub extable_base: usize,
    pub extable_len: usize,
    pub extable: *const c_void,

    pub sym___kernel_sigreturn: isize,
    pub sym___kernel_rt_sigreturn: isize,
    pub sym___kernel_vsyscall: isize,
    pub sym_int80_landing_pad: isize,
    pub sym_vdso32_sigreturn_landing_pad: isize,
    pub sym_vdso32_rt_sigreturn_landing_pad: isize,
    pub sym___futex_list64_try_unlock_cs_start: isize,
    pub sym___futex_list64_try_unlock_cs_end: isize,
    pub sym___futex_list32_try_unlock_cs_start: isize,
    pub sym___futex_list32_try_unlock_cs_end: isize,
}

extern "C" {
    pub static vdso64_image: vdso_image;
    pub static vdsox32_image: vdso_image;
    pub static vdso32_image: vdso_image;

    // C attribute: __init
    pub fn init_vdso_image(image: *const vdso_image) -> i32;

    pub fn map_vdso_once(image: *const vdso_image, addr: usize) -> i32;

    pub fn fixup_vdso_exception(
        regs: *mut pt_regs,
        trapnr: i32,
        error_code: usize,
        fault_addr: usize,
    ) -> bool;
}

// Supplied by linux/mm_types.h.
pub type pt_regs = crate::pt_regs;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
