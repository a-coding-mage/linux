// SPDX-License-Identifier: GPL-2.0
/*
 * machine_kexec.c - handle transition of Linux booting another kernel
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static relocate_new_kernel: *const u8;
    static relocate_new_kernel_size: usize;

    fn page_address(page: *mut c_void) -> *mut c_void;
    fn local_irq_disable();
    fn pr_info(fmt: *const u8, ...);
    fn __flush_cache_all();

    static mut m68k_cputype: u32;
    static mut m68k_mmutype: u32;
}

#[repr(C)]
pub struct kimage {
    pub control_code_page: *mut c_void,
    pub start: usize,
    pub head: usize,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

pub const PAGE_MASK: usize = !((4096usize) - 1);

extern "C" {
    static __PAGE_MASK: usize;
}

pub unsafe fn machine_kexec_prepare(_kimage: *mut kimage) -> i32 {
    0
}

pub unsafe fn machine_kexec_cleanup(_kimage: *mut kimage) {}

pub unsafe fn machine_shutdown() {}

pub unsafe fn machine_crash_shutdown(_regs: *mut pt_regs) {}

type RelocateKernelT = unsafe extern "C" fn(
    ptr: usize,
    start: usize,
    cpu_mmu_flags: usize,
) -> !;

pub unsafe fn machine_kexec(image: *mut kimage) {
    let reboot_code_buffer: *mut c_void;
    let cpu_mmu_flags: usize;

    reboot_code_buffer = page_address((*image).control_code_page);

    core::ptr::copy_nonoverlapping(
        relocate_new_kernel,
        reboot_code_buffer as *mut u8,
        relocate_new_kernel_size,
    );

    /*
     * we do not want to be bothered.
     */
    local_irq_disable();

    pr_info(
        b"Will call new kernel at 0x%08lx. Bye...\n\0".as_ptr(),
        (*image).start,
    );
    __flush_cache_all();
    cpu_mmu_flags = m68k_cputype as usize | ((m68k_mmutype as usize) << 8);
    let relocate: RelocateKernelT = core::mem::transmute(reboot_code_buffer);
    relocate((*image).head & PAGE_MASK, (*image).start, cpu_mmu_flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
