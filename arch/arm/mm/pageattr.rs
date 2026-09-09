// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014, The Linux Foundation. All rights reserved.
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct page_change_data {
    pub set_mask: pgprot_t,
    pub clear_mask: pgprot_t,
}

extern "C" {
    static mut init_mm: mm_struct;

    fn clear_pte_bit(pte: pte_t, mask: pgprot_t) -> pte_t;
    fn set_pte_bit(pte: pte_t, mask: pgprot_t) -> pte_t;
    fn set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: u32);
    fn apply_to_page_range(
        mm: *mut mm_struct,
        start: c_ulong,
        size: c_ulong,
        fn_: unsafe extern "C" fn(*mut pte_t, c_ulong, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong);
    fn __pgprot(value: c_ulong) -> pgprot_t;
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

// External kernel types.
type c_ulong = usize;
type c_int = i32;
type c_void = core::ffi::c_void;
type pgprot_t = __pgprot_t;
type pte_t = __pte_t;
type mm_struct = __mm_struct;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct __pgprot_t {
    pub val: c_ulong,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct __pte_t {
    pub val: c_ulong,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct __mm_struct {
    _private: [u8; 0],
}

extern "C" {
    static PAGE_MASK: c_ulong;
    static PAGE_SIZE: c_ulong;
    static MODULES_VADDR: c_ulong;
    static MODULES_END: c_ulong;
    static VMALLOC_START: c_ulong;
    static VMALLOC_END: c_ulong;
    static L_PTE_RDONLY: c_ulong;
    static L_PTE_XN: c_ulong;
    static L_PTE_VALID: c_ulong;
}

unsafe extern "C" fn change_page_range(
    ptep: *mut pte_t,
    _addr: c_ulong,
    data: *mut c_void,
) -> c_int {
    let cdata = data as *mut page_change_data;
    let mut pte = core::ptr::read(ptep);

    pte = clear_pte_bit(pte, (*cdata).clear_mask);
    pte = set_pte_bit(pte, (*cdata).set_mask);

    set_pte_ext(ptep, pte, 0);
    0
}

unsafe fn range_in_range(
    start: c_ulong,
    size: c_ulong,
    range_start: c_ulong,
    range_end: c_ulong,
) -> bool {
    start >= range_start && start < range_end && size <= range_end.wrapping_sub(start)
}

/*
 * This function assumes that the range is mapped with PAGE_SIZE pages.
 */
unsafe fn __change_memory_common(
    start: c_ulong,
    size: c_ulong,
    set_mask: pgprot_t,
    clear_mask: pgprot_t,
) -> c_int {
    let mut data = page_change_data { set_mask, clear_mask };

    let ret = apply_to_page_range(
        &mut init_mm,
        start,
        size,
        change_page_range,
        &mut data as *mut _ as *mut c_void,
    );

    flush_tlb_kernel_range(start, start.wrapping_add(size));
    ret
}

unsafe fn change_memory_common(
    addr: c_ulong,
    numpages: c_int,
    set_mask: pgprot_t,
    clear_mask: pgprot_t,
) -> c_int {
    let start = addr & PAGE_MASK;
    let end = (addr.wrapping_add(PAGE_SIZE).wrapping_sub(1) & PAGE_MASK)
        .wrapping_add((numpages as c_ulong).wrapping_mul(PAGE_SIZE));
    let size = end.wrapping_sub(start);

    WARN_ON_ONCE(start != addr);

    if size == 0 {
        return 0;
    }

    if !(range_in_range(start, size, MODULES_VADDR, MODULES_END)
        || range_in_range(start, size, VMALLOC_START, VMALLOC_END))
    {
        return -22;
    }

    __change_memory_common(start, size, set_mask, clear_mask)
}

pub unsafe fn set_memory_ro(addr: c_ulong, numpages: c_int) -> c_int {
    change_memory_common(addr, numpages, __pgprot(L_PTE_RDONLY), __pgprot(0))
}

pub unsafe fn set_memory_rw(addr: c_ulong, numpages: c_int) -> c_int {
    change_memory_common(addr, numpages, __pgprot(0), __pgprot(L_PTE_RDONLY))
}

pub unsafe fn set_memory_nx(addr: c_ulong, numpages: c_int) -> c_int {
    change_memory_common(addr, numpages, __pgprot(L_PTE_XN), __pgprot(0))
}

pub unsafe fn set_memory_x(addr: c_ulong, numpages: c_int) -> c_int {
    change_memory_common(addr, numpages, __pgprot(0), __pgprot(L_PTE_XN))
}

pub unsafe fn set_memory_valid(addr: c_ulong, numpages: c_int, enable: c_int) -> c_int {
    if enable != 0 {
        __change_memory_common(
            addr,
            PAGE_SIZE.wrapping_mul(numpages as c_ulong),
            __pgprot(L_PTE_VALID),
            __pgprot(0),
        )
    } else {
        __change_memory_common(
            addr,
            PAGE_SIZE.wrapping_mul(numpages as c_ulong),
            __pgprot(0),
            __pgprot(L_PTE_VALID),
        )
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
