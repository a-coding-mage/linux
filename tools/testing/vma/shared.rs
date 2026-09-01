// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from shared.c. C include dependency:
// #include "shared.h"

pub static mut fail_prealloc: bool = false;
pub static mut mmap_min_addr: c_ulong = CONFIG_DEFAULT_MMAP_MIN_ADDR;
pub static mut dac_mmap_min_addr: c_ulong = CONFIG_DEFAULT_MMAP_MIN_ADDR;
pub static mut stack_guard_gap: c_ulong = 256u64 as c_ulong << PAGE_SHIFT;

pub static vma_dummy_vm_ops: vm_operations_struct = unsafe { core::mem::zeroed() };
pub static mut dummy_anon_vma: anon_vma = unsafe { core::mem::zeroed() };
pub static mut __current: task_struct = unsafe { core::mem::zeroed() };

#[no_mangle]
pub unsafe extern "C" fn alloc_vma(
    mm: *mut mm_struct,
    start: c_ulong,
    end: c_ulong,
    pgoff: pgoff_t,
    vma_flags: vma_flags_t,
) -> *mut vm_area_struct {
    let vma: *mut vm_area_struct = vm_area_alloc(mm);

    if vma.is_null() {
        return core::ptr::null_mut();
    }

    (*vma).vm_start = start;
    (*vma).vm_end = end;
    vma_set_pgoff(vma, pgoff);
    vma_set_anon_pgoff(vma, start >> PAGE_SHIFT);
    (*vma).flags = vma_flags;
    vma_assert_detached(vma);

    vma
}

#[no_mangle]
pub unsafe extern "C" fn detach_free_vma(vma: *mut vm_area_struct) {
    vma_mark_detached(vma);
    vm_area_free(vma);
}

#[no_mangle]
pub unsafe extern "C" fn alloc_and_link_vma(
    mm: *mut mm_struct,
    start: c_ulong,
    end: c_ulong,
    pgoff: pgoff_t,
    vma_flags: vma_flags_t,
) -> *mut vm_area_struct {
    let vma: *mut vm_area_struct = alloc_vma(mm, start, end, pgoff, vma_flags);

    if vma.is_null() {
        return core::ptr::null_mut();
    }

    if attach_vma(mm, vma) != 0 {
        detach_free_vma(vma);
        return core::ptr::null_mut();
    }

    /*
     * Reset this counter which we use to track whether writes have
     * begun. Linking to the tree will have caused this to be incremented,
     * which means we will get a false positive otherwise.
     */
    (*vma).vm_lock_seq = UINT_MAX as _;

    vma
}

#[no_mangle]
pub unsafe extern "C" fn reset_dummy_anon_vma() {
    dummy_anon_vma.was_cloned = false;
    dummy_anon_vma.was_unlinked = false;
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_mm(mm: *mut mm_struct, vmi: *mut vma_iterator) -> c_int {
    let mut vma: *mut vm_area_struct;
    let mut count: c_int = 0;

    fail_prealloc = false;
    reset_dummy_anon_vma();

    vma_iter_set(vmi, 0);

    // C source uses for_each_vma(*vmi, vma).
    for_each_vma!(*vmi, vma, {
        detach_free_vma(vma);
        count += 1;
    });

    mtree_destroy(&mut (*mm).mm_mt);
    (*mm).map_count = 0;
    count
}

#[no_mangle]
pub unsafe extern "C" fn vma_write_started(vma: *mut vm_area_struct) -> bool {
    let seq: c_int = (*vma).vm_lock_seq;

    /* We reset after each check. */
    (*vma).vm_lock_seq = UINT_MAX as _;

    /* The vma_start_write() stub simply increments this value. */
    seq > -1
}

#[no_mangle]
pub unsafe extern "C" fn __vma_set_dummy_anon_vma(
    vma: *mut vm_area_struct,
    avc: *mut anon_vma_chain,
    anon_vma: *mut anon_vma,
) {
    (*vma).anon_vma = anon_vma;
    INIT_LIST_HEAD(&mut (*vma).anon_vma_chain);
    list_add(&mut (*avc).same_vma, &mut (*vma).anon_vma_chain);
    (*avc).anon_vma = (*vma).anon_vma;
}

#[no_mangle]
pub unsafe extern "C" fn vma_set_dummy_anon_vma(
    vma: *mut vm_area_struct,
    avc: *mut anon_vma_chain,
) {
    __vma_set_dummy_anon_vma(vma, avc, &raw mut dummy_anon_vma);
}

#[no_mangle]
pub unsafe extern "C" fn get_current() -> *mut task_struct {
    &raw mut __current
}

#[no_mangle]
pub unsafe extern "C" fn rlimit(_limit: c_uint) -> c_ulong {
    -1isize as c_ulong
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
