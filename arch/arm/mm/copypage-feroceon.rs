// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/copypage-feroceon.S
 *
 *  Copyright (C) 2008 Marvell Semiconductors
 *
 * This handles copy_user_highpage and clear_user_page on Feroceon
 * more optimally than the generic implementations.
 */

// Supplied by the surrounding kernel translation unit.
use crate::{flush_cache_page, kmap_atomic, kunmap_atomic, page_to_pfn, page, vm_area_struct,
            cpu_user_fns, PAGE_SIZE};

unsafe fn feroceon_copy_user_page(mut kto: *mut core::ffi::c_void,
                                  mut kfrom: *const core::ffi::c_void) {
    let mut tmp: i32;
    core::arch::asm!(
        ".arch armv5te\n\
1:  ldmia {from}!, {{r2-r7, ip, lr}}\n\
    pld [{from}, #0]\n\
    pld [{from}, #32]\n\
    pld [{from}, #64]\n\
    pld [{from}, #96]\n\
    pld [{from}, #128]\n\
    pld [{from}, #160]\n\
    pld [{from}, #192]\n\
    stmia {to}, {{r2-r7, ip, lr}}\n\
    ldmia {from}!, {{r2-r7, ip, lr}}\n\
    mcr p15, 0, {to}, c7, c14, 1\n\
    add {to}, {to}, #32\n\
    stmia {to}, {{r2-r7, ip, lr}}\n\
    ldmia {from}!, {{r2-r7, ip, lr}}\n\
    mcr p15, 0, {to}, c7, c14, 1\n\
    add {to}, {to}, #32\n\
    stmia {to}, {{r2-r7, ip, lr}}\n\
    ldmia {from}!, {{r2-r7, ip, lr}}\n\
    mcr p15, 0, {to}, c7, c14, 1\n\
    add {to}, {to}, #32\n\
    stmia {to}, {{r2-r7, ip, lr}}\n\
    ldmia {from}!, {{r2-r7, ip, lr}}\n\
    mcr p15, 0, {to}, c7, c14, 1\n\
    add {to}, {to}, #32\n\
    stmia {to}, {{r2-r7, ip, lr}}\n\
    ldmia {from}!, {{r2-r7, ip, lr}}\n\
    mcr p15, 0, {to}, c7, c14, 1\n\
    add {to}, {to}, #32\n\
    stmia {to}, {{r2-r7, ip, lr}}\n\
    ldmia {from}!, {{r2-r7, ip, lr}}\n\
    mcr p15, 0, {to}, c7, c14, 1\n\
    add {to}, {to}, #32\n\
    stmia {to}, {{r2-r7, ip, lr}}\n\
    ldmia {from}!, {{r2-r7, ip, lr}}\n\
    mcr p15, 0, {to}, c7, c14, 1\n\
    add {to}, {to}, #32\n\
    stmia {to}, {{r2-r7, ip, lr}}\n\
    subs {count}, {count}, #(32 * 8)\n\
    mcr p15, 0, {to}, c7, c14, 1\n\
    add {to}, {to}, #32\n\
    bne 1b\n\
    mcr p15, 0, {count}, c7, c10, 4",
        to = inout(reg) kto,
        from = inout(reg) kfrom,
        count = lateout(reg) tmp,
        in(reg) PAGE_SIZE,
        out("r2") _, out("r3") _, out("r4") _, out("r5") _, out("r6") _, out("r7") _,
        out("ip") _, out("lr") _,
        options(nostack)
    );
}

pub unsafe fn feroceon_copy_user_highpage(to: *mut page, from: *mut page,
                                          vaddr: usize, vma: *mut vm_area_struct) {
    let kto = kmap_atomic(to);
    let kfrom = kmap_atomic(from);
    flush_cache_page(vma, vaddr, page_to_pfn(from));
    feroceon_copy_user_page(kto, kfrom);
    kunmap_atomic(kfrom);
    kunmap_atomic(kto);
}

pub unsafe fn feroceon_clear_user_highpage(page: *mut page, _vaddr: usize) {
    let kaddr = kmap_atomic(page);
    let mut ptr: *mut core::ffi::c_void;
    core::arch::asm!(
        "mov r1, {count}\n\
         mov r2, #0\n mov r3, #0\n mov r4, #0\n mov r5, #0\n mov r6, #0\n mov r7, #0\n\
         mov ip, #0\n mov lr, #0\n\
1:       stmia {ptr}, {{r2-r7, ip, lr}}\n\
         subs r1, r1, #1\n\
         mcr p15, 0, {ptr}, c7, c14, 1\n\
         add {ptr}, {ptr}, #32\n\
         bne 1b\n\
         mcr p15, 0, r1, c7, c10, 4",
        ptr = inout(reg) kaddr => ptr,
        count = const PAGE_SIZE / 32,
        out("r1") _, out("r2") _, out("r3") _, out("r4") _, out("r5") _, out("r6") _, out("r7") _,
        out("ip") _, out("lr") _, options(nostack)
    );
    kunmap_atomic(kaddr);
}

pub static mut feroceon_user_fns: cpu_user_fns = cpu_user_fns {
    cpu_clear_user_highpage: Some(feroceon_clear_user_highpage),
    cpu_copy_user_highpage: Some(feroceon_copy_user_highpage),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
