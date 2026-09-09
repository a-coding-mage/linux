/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _ASM_POWERPC_SECTIONS_H */
/* The declarations below are kernel-only (__KERNEL__). */

/* C dependencies: linux/elf.h, linux/uaccess.h, and asm-generic/sections.h. */

#[cfg(feature = "CONFIG_HAVE_FUNCTION_DESCRIPTORS")]
pub type func_desc_t = func_desc;

extern "C" {
    pub static mut __head_end: [core::ffi::c_char; 0];
    pub static mut __srwx_boundary: [core::ffi::c_char; 0];
    pub static mut __exittext_begin: [core::ffi::c_char; 0];
    pub static mut __exittext_end: [core::ffi::c_char; 0];

    /* Patch sites */
    pub static mut patch__call_flush_branch_caches1: i32;
    pub static mut patch__call_flush_branch_caches2: i32;
    pub static mut patch__call_flush_branch_caches3: i32;
    pub static mut patch__flush_count_cache_return: i32;
    pub static mut patch__flush_link_stack_return: i32;
    pub static mut patch__call_kvm_flush_link_stack: i32;
    pub static mut patch__call_kvm_flush_link_stack_p9: i32;
    pub static mut patch__memset_nocache: i32;
    pub static mut patch__memcpy_nocache: i32;

    pub static mut flush_branch_caches: core::ffi::c_long;
    pub static mut kvm_flush_link_stack: core::ffi::c_long;

    #[cfg(target_pointer_width = "64")]
    pub static mut __start_interrupts: [core::ffi::c_char; 0];
    #[cfg(target_pointer_width = "64")]
    pub static mut __end_interrupts: [core::ffi::c_char; 0];

    #[cfg(all(target_pointer_width = "64", feature = "CONFIG_PPC_POWERNV"))]
    pub static mut start_real_trampolines: [core::ffi::c_char; 0];
    #[cfg(all(target_pointer_width = "64", feature = "CONFIG_PPC_POWERNV"))]
    pub static mut end_real_trampolines: [core::ffi::c_char; 0];
    #[cfg(all(target_pointer_width = "64", feature = "CONFIG_PPC_POWERNV"))]
    pub static mut start_virt_trampolines: [core::ffi::c_char; 0];
    #[cfg(all(target_pointer_width = "64", feature = "CONFIG_PPC_POWERNV"))]
    pub static mut end_virt_trampolines: [core::ffi::c_char; 0];
}

/* This assumes the kernel is never compiled -mcmodel=small or the total
 * .toc is always less than 64k. */
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn kernel_toc_addr() -> usize {
    #[cfg(feature = "CONFIG_PPC_KERNEL_PCREL")]
    {
        /* BUILD_BUG(); */
        return usize::MAX;
    }
    #[cfg(not(feature = "CONFIG_PPC_KERNEL_PCREL"))]
    {
        let toc_ptr: usize;
        core::arch::asm!("mr {0}, 2", out(reg) toc_ptr);
        toc_ptr
    }
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn overlaps_interrupt_vector_text(start: usize, end: usize) -> i32 {
    let real_start = (&__start_interrupts as *const _ as usize)
        .wrapping_sub(&_stext as *const _ as usize);
    let real_end = (&__end_interrupts as *const _ as usize)
        .wrapping_sub(&_stext as *const _ as usize);

    (start < __va(real_end) && __va(real_start) < end) as i32
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn overlaps_kernel_text(start: usize, end: usize) -> i32 {
    (start < (&__init_end as *const _ as usize) && (&_stext as *const _ as usize) < end) as i32
}

#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn kernel_toc_addr() -> usize {
    /* BUILD_BUG(); */
    usize::MAX
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
