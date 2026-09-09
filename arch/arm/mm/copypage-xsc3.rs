// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mm/copypage-xsc3.S
 *
 * Copyright (C) 2004 Intel Corp.
 *
 * Adapted for 3rd gen XScale core, no more mini-dcache
 * Author: Matt Gilbert (matthew.m.gilbert@intel.com)
 */

// Kernel-provided dependencies.
use core::arch::asm;

extern "C" {
    fn kmap_atomic(page: *mut page) -> *mut core::ffi::c_void;
    fn kunmap_atomic(addr: *mut core::ffi::c_void);
    fn flush_cache_page(vma: *mut vm_area_struct, vaddr: usize, pfn: usize);
    fn page_to_pfn(page: *mut page) -> usize;
}

#[repr(C)]
pub struct page { _private: [u8; 0] }

#[repr(C)]
pub struct vm_area_struct { _private: [u8; 0] }

extern "C" { static PAGE_SIZE: usize; }

unsafe fn xsc3_mc_copy_user_page(mut kto: *mut core::ffi::c_void,
                                  mut kfrom: *const core::ffi::c_void) {
    let mut tmp: i32;
    asm!(
        ".arch xscale
         pld [{1}, #0]
         pld [{1}, #32]
     1:  pld [{1}, #64]
         pld [{1}, #96]
     2:  ldrd r2, r3, [{1}], #8
         ldrd r4, r5, [{1}], #8
         mcr p15, 0, {0}, c7, c6, 1
         strd r2, r3, [{0}], #8
         ldrd r2, r3, [{1}], #8
         strd r4, r5, [{0}], #8
         ldrd r4, r5, [{1}], #8
         strd r2, r3, [{0}], #8
         strd r4, r5, [{0}], #8
         ldrd r2, r3, [{1}], #8
         ldrd r4, r5, [{1}], #8
         mcr p15, 0, {0}, c7, c6, 1
         strd r2, r3, [{0}], #8
         ldrd r2, r3, [{1}], #8
         subs {2}, {2}, #1
         strd r4, r5, [{0}], #8
         ldrd r4, r5, [{1}], #8
         strd r2, r3, [{0}], #8
         strd r4, r5, [{0}], #8
         bgt 1b
         beq 2b",
        inout(reg) kto,
        inout(reg) kfrom,
        inout(reg) (PAGE_SIZE / 64 - 1) => tmp,
        out("r2") _, out("r3") _, out("r4") _, out("r5") _,
    );
}

pub unsafe extern "C" fn xsc3_mc_copy_user_highpage(
    to: *mut page, from: *mut page, vaddr: usize, vma: *mut vm_area_struct,
) {
    let kto = kmap_atomic(to);
    let kfrom = kmap_atomic(from);
    flush_cache_page(vma, vaddr, page_to_pfn(from));
    xsc3_mc_copy_user_page(kto, kfrom as *const _);
    kunmap_atomic(kfrom);
    kunmap_atomic(kto);
}

pub unsafe extern "C" fn xsc3_mc_clear_user_highpage(page: *mut page, _vaddr: usize) {
    let kaddr = kmap_atomic(page);
    let mut ptr = kaddr;
    asm!(
        ".arch xscale
         mov r1, {1}
         mov r2, #0
         mov r3, #0
     1:  mcr p15, 0, {0}, c7, c6, 1
         strd r2, r3, [{0}], #8
         strd r2, r3, [{0}], #8
         strd r2, r3, [{0}], #8
         strd r2, r3, [{0}], #8
         subs r1, r1, #1
         bne 1b",
        inout(reg) ptr,
        in(reg) PAGE_SIZE / 32,
        out("r1") _, out("r2") _, out("r3") _,
    );
    kunmap_atomic(kaddr);
}

#[repr(C)]
pub struct cpu_user_fns {
    pub cpu_clear_user_highpage: Option<unsafe extern "C" fn(*mut page, usize)>,
    pub cpu_copy_user_highpage: Option<unsafe extern "C" fn(*mut page, *mut page, usize, *mut vm_area_struct)>,
}

#[no_mangle]
pub static mut xsc3_mc_user_fns: cpu_user_fns = cpu_user_fns {
    cpu_clear_user_highpage: Some(xsc3_mc_clear_user_highpage),
    cpu_copy_user_highpage: Some(xsc3_mc_copy_user_highpage),
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
