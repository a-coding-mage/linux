// SPDX-License-Identifier: GPL-2.0
/*
 * swsusp.c - SuperH hibernation support
 *
 * Copyright (C) 2009 Magnus Damm
 */

// Linux and SuperH headers supplying the declarations below.

#[repr(C)]
pub struct swsusp_arch_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    pub static mut swsusp_arch_regs_cpu0: swsusp_arch_regs;

    static __nosave_begin: u8;
    static __nosave_end: u8;

    // These correspond to the kernel's __pa(), PAGE_ALIGN(), and PAGE_SHIFT.
    fn __pa(addr: *const u8) -> usize;
    fn PAGE_ALIGN(addr: usize) -> usize;
    static PAGE_SHIFT: usize;

    static mut current: *mut task_struct;
    fn init_fpu(task: *mut task_struct);
    fn local_flush_tlb_all();
}

pub unsafe fn pfn_is_nosave(pfn: usize) -> i32 {
    let begin_pfn: usize = __pa(&__nosave_begin as *const u8) >> PAGE_SHIFT;
    let end_pfn: usize = PAGE_ALIGN(__pa(&__nosave_end as *const u8)) >> PAGE_SHIFT;

    ((pfn >= begin_pfn) && (pfn < end_pfn)) as i32
}

pub unsafe fn save_processor_state() {
    init_fpu(current);
}

pub unsafe fn restore_processor_state() {
    local_flush_tlb_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
