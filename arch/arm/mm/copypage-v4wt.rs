// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/copypage-v4wt.S
 *
 *  Copyright (C) 1995-1999 Russell King
 *
 *  This is for CPUs with a writethrough cache and 'flush ID cache' is
 *  the only supported cache operation.
 */

// Corresponds to <linux/init.h> and <linux/highmem.h>.

/*
 * ARMv4 optimised copy_user_highpage
 *
 * Since we have writethrough caches, we don't have to worry about
 * dirty data in the cache.  However, we do have to ensure that
 * subsequent reads are up to date.
 */
unsafe fn v4wt_copy_user_page(mut kto: *mut core::ffi::c_void,
                              mut kfrom: *const core::ffi::c_void) {
    let mut tmp: i32;

    core::arch::asm!(
        ".syntax unified",
        "ldmia {1}!, {{r3, r4, ip, lr}}",
        "1:",
        "stmia {0}!, {{r3, r4, ip, lr}}",
        "ldmia {1}!, {{r3, r4, ip, lr}}",
        "stmia {0}!, {{r3, r4, ip, lr}}",
        "ldmia {1}!, {{r3, r4, ip, lr}}",
        "stmia {0}!, {{r3, r4, ip, lr}}",
        "ldmia {1}!, {{r3, r4, ip, lr}}",
        "subs {2}, {2}, #1",
        "stmia {0}!, {{r3, r4, ip, lr}}",
        "ldmiane {1}!, {{r3, r4, ip, lr}}",
        "bne 1b",
        "mcr p15, 0, {2}, c7, c7, 0",
        inout(reg) kto,
        inout(reg) kfrom,
        inout(reg) tmp => _,
        in("r3") 0i32,
        in("r4") 0i32,
        in("ip") 0i32,
        in("lr") 0i32,
        options(nostack)
    );
}

#[repr(C)]
pub struct page;
#[repr(C)]
pub struct vm_area_struct;

extern "C" {
    fn kmap_atomic(page: *mut page) -> *mut core::ffi::c_void;
    fn kunmap_atomic(addr: *mut core::ffi::c_void);
}

pub unsafe extern "C" fn v4wt_copy_user_highpage(
    to: *mut page,
    from: *mut page,
    _vaddr: libc::c_ulong,
    _vma: *mut vm_area_struct,
) {
    let kto = kmap_atomic(to);
    let kfrom = kmap_atomic(from);
    v4wt_copy_user_page(kto, kfrom);
    kunmap_atomic(kfrom);
    kunmap_atomic(kto);
}

/*
 * ARMv4 optimised clear_user_page
 *
 * Same story as above.
 */
pub unsafe extern "C" fn v4wt_clear_user_highpage(page: *mut page, _vaddr: libc::c_ulong) {
    let kaddr = kmap_atomic(page);
    let mut ptr = kaddr;
    core::arch::asm!(
        "mov r1, {1}",
        "mov r2, #0",
        "mov r3, #0",
        "mov ip, #0",
        "mov lr, #0",
        "1:",
        "stmia {0}!, {{r2, r3, ip, lr}}",
        "stmia {0}!, {{r2, r3, ip, lr}}",
        "stmia {0}!, {{r2, r3, ip, lr}}",
        "stmia {0}!, {{r2, r3, ip, lr}}",
        "subs r1, r1, #1",
        "bne 1b",
        "mcr p15, 0, r2, c7, c7, 0",
        inout(reg) ptr,
        in(reg) PAGE_SIZE / 64,
        options(nostack)
    );
    kunmap_atomic(kaddr);
}

// PAGE_SIZE is supplied by the kernel headers.
extern "C" {
    static PAGE_SIZE: usize;
}

#[repr(C)]
pub struct cpu_user_fns {
    pub cpu_clear_user_highpage: unsafe extern "C" fn(*mut page, libc::c_ulong),
    pub cpu_copy_user_highpage: unsafe extern "C" fn(
        *mut page,
        *mut page,
        libc::c_ulong,
        *mut vm_area_struct,
    ),
}

#[link_section = ".init.data"]
#[no_mangle]
pub static mut v4wt_user_fns: cpu_user_fns = cpu_user_fns {
    cpu_clear_user_highpage: v4wt_clear_user_highpage,
    cpu_copy_user_highpage: v4wt_copy_user_highpage,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
