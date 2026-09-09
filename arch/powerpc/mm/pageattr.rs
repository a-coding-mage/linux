// SPDX-License-Identifier: GPL-2.0

/*
 * MMU-generic set_memory implementation for powerpc
 *
 * Copyright 2019-2021, IBM Corporation.
 */

// Dependencies supplied by the surrounding kernel translation unit.

unsafe extern "C" {
    static mut init_mm: mm_struct;

    fn pte_update(
        mm: *mut mm_struct,
        addr: c_ulong,
        ptep: *mut pte_t,
        clr: c_ulong,
        set: c_ulong,
        huge: c_int,
    ) -> pte_basic_t;
    fn radix_enabled() -> bool;
    fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong);
    fn is_vmalloc_or_module_addr(addr: *mut c_void) -> bool;
    fn is_vm_area_hugepages(addr: *mut c_void) -> bool;
    fn apply_to_existing_page_range(
        mm: *mut mm_struct,
        start: c_ulong,
        size: c_ulong,
        fn_: unsafe extern "C" fn(*mut pte_t, c_ulong, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn get_region_id(addr: c_ulong) -> c_int;
    fn page_address(page: *mut page) -> *mut c_void;
    fn PageHighMem(page: *mut page) -> bool;
    fn hash__kernel_map_pages(page: *mut page, numpages: c_int, enable: c_int) -> c_int;
    fn set_memory_p(addr: c_ulong, numpages: c_int) -> c_int;
    fn set_memory_np(addr: c_ulong, numpages: c_int) -> c_int;
    fn panic(fmt: *const c_char, ...);
}

type c_int = i32;
type c_long = isize;
type c_ulong = usize;
type c_void = core::ffi::c_void;
type c_char = i8;

// Types and constants below are supplied by the corresponding kernel headers.
#[allow(non_camel_case_types)]
type pte_basic_t = usize;
#[allow(non_camel_case_types)]
type pte_t = core::ffi::c_void;
#[allow(non_camel_case_types)]
type mm_struct = core::ffi::c_void;
#[allow(non_camel_case_types)]
type page = core::ffi::c_void;

unsafe fn pte_update_delta(
    ptep: *mut pte_t,
    addr: c_ulong,
    old: c_ulong,
    new: c_ulong,
) -> pte_basic_t {
    pte_update(&raw mut init_mm, addr, ptep, old & !new, new & !old, 0)
}

/*
 * Updates the attributes of a page atomically.
 *
 * This sequence is safe against concurrent updates, and also allows updating the
 * attributes of a page currently being executed or accessed.
 */
unsafe extern "C" fn change_page_attr(
    ptep: *mut pte_t,
    addr: c_ulong,
    data: *mut c_void,
) -> c_int {
    let action = data as c_long;

    let addr = addr & PAGE_MASK;
    /* modify the PTE bits as desired */
    match action {
        SET_MEMORY_RO => {
            /* Don't clear DIRTY bit */
            pte_update_delta(ptep, addr, _PAGE_KERNEL_RW & !_PAGE_DIRTY, _PAGE_KERNEL_RO);
        }
        SET_MEMORY_ROX => {
            /* Don't clear DIRTY bit */
            pte_update_delta(ptep, addr, _PAGE_KERNEL_RW & !_PAGE_DIRTY, _PAGE_KERNEL_ROX);
        }
        SET_MEMORY_RW => {
            pte_update_delta(ptep, addr, _PAGE_KERNEL_RO, _PAGE_KERNEL_RW);
        }
        SET_MEMORY_NX => {
            pte_update_delta(ptep, addr, _PAGE_KERNEL_ROX, _PAGE_KERNEL_RO);
        }
        SET_MEMORY_X => {
            pte_update_delta(ptep, addr, _PAGE_KERNEL_RO, _PAGE_KERNEL_ROX);
        }
        SET_MEMORY_NP => {
            pte_update(&raw mut init_mm, addr, ptep, _PAGE_PRESENT, 0, 0);
        }
        SET_MEMORY_P => {
            pte_update(&raw mut init_mm, addr, ptep, 0, _PAGE_PRESENT, 0);
        }
        _ => {
            // Equivalent to WARN_ON_ONCE(1).
            unsafe { core::hint::unreachable_unchecked() }
        }
    }

    /* See ptesync comment in radix__set_pte_at() */
    if radix_enabled() {
        core::arch::asm!("ptesync", options(nostack, preserves_flags));
    }

    flush_tlb_kernel_range(addr, addr + PAGE_SIZE);

    0
}

unsafe fn change_memory_attr(addr: c_ulong, numpages: c_int, action: c_long) -> c_int {
    let start = addr & !(PAGE_SIZE - 1);
    let size = (numpages as c_ulong).wrapping_mul(PAGE_SIZE);

    if numpages == 0 {
        return 0;
    }

    if is_vmalloc_or_module_addr(addr as *mut c_void)
        && is_vm_area_hugepages(addr as *mut c_void)
    {
        return -EINVAL;
    }

    // CONFIG_PPC_BOOK3S_64: on hash, the linear mapping is not in the Linux
    // page table, so apply_to_existing_page_range() will have no effect. If in
    // the future the set_memory_* functions are used on the linear map this
    // will need to be updated.
    if !radix_enabled() {
        let region = get_region_id(addr);
        if region != VMALLOC_REGION_ID && region != IO_REGION_ID {
            return -EINVAL;
        }
    }

    apply_to_existing_page_range(
        &raw mut init_mm,
        start,
        size,
        change_page_attr,
        action as *mut c_void,
    )
}

// CONFIG_DEBUG_PAGEALLOC || CONFIG_KFENCE
// CONFIG_ARCH_SUPPORTS_DEBUG_PAGEALLOC
unsafe fn __kernel_map_pages(page: *mut page, numpages: c_int, enable: c_int) {
    let addr = page_address(page) as c_ulong;

    if PageHighMem(page) {
        return;
    }

    let err = if CONFIG_PPC_BOOK3S_64 && !radix_enabled() {
        hash__kernel_map_pages(page, numpages, enable)
    } else if enable != 0 {
        set_memory_p(addr, numpages)
    } else {
        set_memory_np(addr, numpages)
    };

    if err != 0 {
        panic(c"%s: changing memory protections failed\n".as_ptr(), c"__kernel_map_pages".as_ptr());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
