// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/copypage-v4wb.c
 *
 *  Copyright (C) 1995-1999 Russell King
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

extern "C" {
    fn kmap_atomic(page: *mut page) -> *mut c_void;
    fn kunmap_atomic(addr: *const c_void);
    fn flush_cache_page(vma: *mut vm_area_struct, vaddr: usize, pfn: usize);
    fn page_to_pfn(page: *mut page) -> usize;
}

const PAGE_SIZE: usize = 4096;

/*
 * ARMv4 optimised copy_user_highpage
 *
 * We flush the destination cache lines just before we write the data into the
 * corresponding address.  Since the Dcache is read-allocate, this removes the
 * Dcache aliasing issue.  The writes will be forwarded to the write buffer,
 * and merged as appropriate.
 *
 * Note: We rely on all ARMv4 processors implementing the "invalidate D line"
 * instruction.  If your processor does not supply this, you have to write
 * your own copy_user_highpage that does the right thing.
 */
unsafe fn v4wb_copy_user_page(mut kto: *mut c_void, mut kfrom: *const c_void) {
    let mut tmp: i32;

    // The following ARMv4 inline assembly is preserved verbatim.  It requires
    // the target kernel's ARM assembler/register conventions.
    core::arch::asm!(
        ".syntax unified
         ldmia {from}!, {{r3, r4, ip, lr}}
        1: mcr p15, 0, {to}, c7, c6, 1
         stmia {to}!, {{r3, r4, ip, lr}}
         ldmia {from}!, {{r3, r4, ip, lr}}
         stmia {to}!, {{r3, r4, ip, lr}}
         ldmia {from}!, {{r3, r4, ip, lr}}
         mcr p15, 0, {to}, c7, c6, 1
         stmia {to}!, {{r3, r4, ip, lr}}
         ldmia {from}!, {{r3, r4, ip, lr}}
         subs {count}, {count}, #1
         stmia {to}!, {{r3, r4, ip, lr}}
         ldmiane {from}!, {{r3, r4, ip, lr}}
         bne 1b
         mcr p15, 0, {from}, c7, c10, 4",
        to = inout(reg) kto,
        from = inout(reg) kfrom,
        count = inout(reg) tmp,
        in("r3") 0usize,
        in("r4") 0usize,
        in("ip") 0usize,
        in("lr") 0usize,
        options(nostack)
    );
}

pub unsafe fn v4wb_copy_user_highpage(
    to: *mut page,
    from: *mut page,
    vaddr: usize,
    vma: *mut vm_area_struct,
) {
    let kto = kmap_atomic(to);
    let kfrom = kmap_atomic(from);
    flush_cache_page(vma, vaddr, page_to_pfn(from));
    v4wb_copy_user_page(kto, kfrom as *const c_void);
    kunmap_atomic(kfrom);
    kunmap_atomic(kto);
}

/* ARMv4 optimised clear_user_page. */
pub unsafe fn v4wb_clear_user_highpage(page: *mut page, _vaddr: usize) {
    let kaddr = kmap_atomic(page);
    let mut ptr = kaddr;

    // ARMv4 cache-line invalidation and write-buffer-drain sequence from C.
    core::arch::asm!(
        "mov r1, {count}
         mov r2, #0
         mov r3, #0
         mov ip, #0
         mov lr, #0
        1: mcr p15, 0, {ptr}, c7, c6, 1
         stmia {ptr}!, {{r2, r3, ip, lr}}
         stmia {ptr}!, {{r2, r3, ip, lr}}
         mcr p15, 0, {ptr}, c7, c6, 1
         stmia {ptr}!, {{r2, r3, ip, lr}}
         stmia {ptr}!, {{r2, r3, ip, lr}}
         subs r1, r1, #1
         bne 1b
         mcr p15, 0, r1, c7, c10, 4",
        ptr = inout(reg) ptr,
        count = const PAGE_SIZE / 64,
        options(nostack)
    );
    kunmap_atomic(kaddr);
}

#[repr(C)]
pub struct cpu_user_fns {
    pub cpu_clear_user_highpage: unsafe extern "C" fn(*mut page, usize),
    pub cpu_copy_user_highpage: unsafe extern "C" fn(*mut page, *mut page, usize, *mut vm_area_struct),
}

pub static mut v4wb_user_fns: cpu_user_fns = cpu_user_fns {
    cpu_clear_user_highpage: v4wb_clear_user_highpage,
    cpu_copy_user_highpage: v4wb_copy_user_highpage,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
