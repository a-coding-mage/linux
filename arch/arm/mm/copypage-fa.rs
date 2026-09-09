// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/lib/copypage-fa.S
 *
 *  Copyright (C) 2005 Faraday Corp.
 *  Copyright (C) 2008-2009 Paulius Zaleckas <paulius.zaleckas@teltonika.lt>
 *
 * Based on copypage-v4wb.S:
 *  Copyright (C) 1995-1999 Russell King
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/highmem.h

use core::arch::asm;

const PAGE_SIZE: usize = 4096;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_user_fns {
    pub cpu_clear_user_highpage: Option<unsafe extern "C" fn(*mut page, libc::c_ulong)>,
    pub cpu_copy_user_highpage:
        Option<unsafe extern "C" fn(*mut page, *mut page, libc::c_ulong, *mut vm_area_struct)>,
}

extern "C" {
    fn kmap_atomic(page: *mut page) -> *mut core::ffi::c_void;
    fn kunmap_atomic(addr: *const core::ffi::c_void);
}

/* Faraday optimised copy_user_page */
unsafe fn fa_copy_user_page(mut kto: *mut core::ffi::c_void, mut kfrom: *const core::ffi::c_void) {
    let mut tmp: i32;

    asm!(
        "1:",
        "ldmia {from}!, {{r3, r4, ip, lr}}",
        "stmia {to}, {{r3, r4, ip, lr}}",
        "mcr p15, 0, {to}, c7, c14, 1",
        "add {to}, {to}, #16",
        "ldmia {from}!, {{r3, r4, ip, lr}}",
        "stmia {to}, {{r3, r4, ip, lr}}",
        "mcr p15, 0, {to}, c7, c14, 1",
        "add {to}, {to}, #16",
        "subs {count}, {count}, #1",
        "bne 1b",
        "mcr p15, 0, {count}, c7, c10, 4",
        to = inout(reg) kto,
        from = inout(reg) kfrom,
        count = inout(reg) (PAGE_SIZE / 32) as i32 => tmp,
        out("r3") _, out("r4") _, out("ip") _, out("lr") _,
        options(nostack)
    );
}

pub unsafe extern "C" fn fa_copy_user_highpage(
    to: *mut page,
    from: *mut page,
    _vaddr: libc::c_ulong,
    _vma: *mut vm_area_struct,
) {
    let kto = kmap_atomic(to);
    let kfrom = kmap_atomic(from);
    fa_copy_user_page(kto, kfrom);
    kunmap_atomic(kfrom);
    kunmap_atomic(kto);
}

/*
 * Faraday optimised clear_user_page
 *
 * Same story as above.
 */
pub unsafe extern "C" fn fa_clear_user_highpage(page: *mut page, _vaddr: libc::c_ulong) {
    let kaddr = kmap_atomic(page);
    let mut ptr = kaddr;

    asm!(
        "mov r1, {count}",
        "mov r2, #0",
        "mov r3, #0",
        "mov ip, #0",
        "mov lr, #0",
        "1:",
        "stmia {ptr}, {{r2, r3, ip, lr}}",
        "mcr p15, 0, {ptr}, c7, c14, 1",
        "add {ptr}, {ptr}, #16",
        "stmia {ptr}, {{r2, r3, ip, lr}}",
        "mcr p15, 0, {ptr}, c7, c14, 1",
        "add {ptr}, {ptr}, #16",
        "subs r1, r1, #1",
        "bne 1b",
        "mcr p15, 0, r1, c7, c10, 4",
        ptr = inout(reg) ptr,
        count = const (PAGE_SIZE / 32),
        out("r1") _, out("r2") _, out("r3") _, out("ip") _, out("lr") _,
        options(nostack)
    );
    kunmap_atomic(kaddr);
}

#[no_mangle]
pub static mut fa_user_fns: cpu_user_fns = cpu_user_fns {
    cpu_clear_user_highpage: Some(fa_clear_user_highpage),
    cpu_copy_user_highpage: Some(fa_copy_user_highpage),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
