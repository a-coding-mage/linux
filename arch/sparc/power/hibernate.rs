// SPDX-License-Identifier: GPL-2.0
/*
 * hibernate.c:  Hibernaton support specific for sparc64.
 *
 * Copyright (C) 2013 Kirill V Tkhai (tkhai@yandex.ru)
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct saved_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    pub active_mm: *mut mm_struct,
    pub context: context,
}

#[repr(C)]
pub struct context {
    _private: [u8; 0],
}

extern "C" {
    pub static mut __nosave_begin: u8;
    pub static mut __nosave_end: u8;
    pub static mut current: *mut mm_struct;

    pub fn save_and_clear_fpu();
    pub fn tsb_context_switch_ctx(mm: *mut mm_struct, ctx: usize);
    pub fn CTX_HWBITS(ctx: *const context) -> usize;
    pub fn PFN_DOWN(addr: usize) -> usize;
}

pub static mut saved_context: saved_context = saved_context { _private: [] };

/*
 *\tpfn_is_nosave - check if given pfn is in the 'nosave' section
 */
pub unsafe fn pfn_is_nosave(pfn: usize) -> i32 {
    let nosave_begin_pfn = PFN_DOWN(&__nosave_begin as *const u8 as usize);
    let nosave_end_pfn = PFN_DOWN(&__nosave_end as *const u8 as usize);

    if (pfn >= nosave_begin_pfn) && (pfn < nosave_end_pfn) {
        1
    } else {
        0
    }
}

pub unsafe fn save_processor_state() {
    save_and_clear_fpu();
}

pub unsafe fn restore_processor_state() {
    let mm = (*current).active_mm;

    tsb_context_switch_ctx(mm, CTX_HWBITS(&(*mm).context));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
