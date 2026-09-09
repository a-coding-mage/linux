// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cache management functions for Hexagon
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

use core::arch::asm;
use core::ffi::c_void;

// Supplied by the Linux Hexagon headers/build environment.
extern "C" {
    static LINESIZE: usize;
    static LINEBITS: usize;

    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn __vmcache_ickill();
    fn __vmcache_dckill();
    fn __vmcache_l2kill();
    fn mb();
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_flags: usize,
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

// VM_EXEC is supplied by <linux/mm.h>.
extern "C" {
    static VM_EXEC: usize;
}

#[inline]
unsafe fn spanlines(start: usize, end: usize) -> usize {
    ((end.wrapping_sub(start & !(LINESIZE.wrapping_sub(1))) >> LINEBITS) + 1)
}

pub unsafe fn flush_dcache_range(mut start: usize, end: usize) {
    let lines = spanlines(start, end.wrapping_sub(1));
    let mut flags: usize = 0;

    start &= !(LINESIZE.wrapping_sub(1));

    local_irq_save(&mut flags);

    for _ in 0..lines {
        asm!("dccleaninva({0});", in(reg) start);
        start = start.wrapping_add(LINESIZE);
    }
    local_irq_restore(flags);
}

pub unsafe fn flush_icache_range(mut start: usize, end: usize) {
    let lines = spanlines(start, end.wrapping_sub(1));
    let mut flags: usize = 0;

    start &= !(LINESIZE.wrapping_sub(1));

    local_irq_save(&mut flags);

    for _ in 0..lines {
        asm!("dccleana({0});", "icinva({0});", in(reg) start);
        start = start.wrapping_add(LINESIZE);
    }
    asm!("isync");
    local_irq_restore(flags);
}

// EXPORT_SYMBOL(flush_icache_range);

pub unsafe fn hexagon_clean_dcache_range(mut start: usize, end: usize) {
    let lines = spanlines(start, end.wrapping_sub(1));
    let mut flags: usize = 0;

    start &= !(LINESIZE.wrapping_sub(1));

    local_irq_save(&mut flags);

    for _ in 0..lines {
        asm!("dccleana({0});", in(reg) start);
        start = start.wrapping_add(LINESIZE);
    }
    local_irq_restore(flags);
}

pub unsafe fn hexagon_inv_dcache_range(mut start: usize, end: usize) {
    let lines = spanlines(start, end.wrapping_sub(1));
    let mut flags: usize = 0;

    start &= !(LINESIZE.wrapping_sub(1));

    local_irq_save(&mut flags);

    for _ in 0..lines {
        asm!("dcinva({0});", in(reg) start);
        start = start.wrapping_add(LINESIZE);
    }
    local_irq_restore(flags);
}

/*
 * This is just really brutal and shouldn't be used anyways,
 * especially on V2.  Left here just in case.
 */
pub unsafe fn flush_cache_all_hexagon() {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    __vmcache_ickill();
    __vmcache_dckill();
    __vmcache_l2kill();
    local_irq_restore(flags);
    mb();
}

pub unsafe fn copy_to_user_page(
    vma: *mut vm_area_struct,
    _page: *mut page,
    vaddr: usize,
    dst: *mut c_void,
    src: *mut c_void,
    len: i32,
) {
    let _ = vaddr;
    memcpy(dst, src, len as usize);
    if (*vma).vm_flags & VM_EXEC != 0 {
        flush_icache_range(dst as usize, (dst as usize).wrapping_add(len as usize));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
