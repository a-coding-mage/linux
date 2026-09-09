// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/lib/uaccess_with_memcpy.c
 *
 * Written by: Lennert Buytenhek and Nicolas Pitre
 * Copyright (C) 2009 Marvell Semiconductor
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

extern "C" {
    fn pgd_offset(mm: *mut MmStruct, addr: usize) -> *mut PgdT;
    fn pgd_none(x: PgdT) -> bool;
    fn pgd_bad(x: PgdT) -> bool;
    fn p4d_offset(pgd: *mut PgdT, addr: usize) -> *mut P4dT;
    fn p4d_none(x: P4dT) -> bool;
    fn p4d_bad(x: P4dT) -> bool;
    fn pud_offset(p4d: *mut P4dT, addr: usize) -> *mut PudT;
    fn pud_none(x: PudT) -> bool;
    fn pud_bad(x: PudT) -> bool;
    fn pmd_offset(pud: *mut PudT, addr: usize) -> *mut PmdT;
    fn pmd_none(x: PmdT) -> bool;
    fn pmd_bad(x: PmdT) -> bool;
    fn pmd_leaf(x: PmdT) -> bool;
    fn pmd_hugewillfault(x: PmdT) -> bool;
    fn pte_offset_map_lock(mm: *mut MmStruct, pmd: *mut PmdT, addr: usize, ptl: *mut *mut SpinlockT) -> *mut PteT;
    fn pte_present(x: PteT) -> bool;
    fn pte_young(x: PteT) -> bool;
    fn pte_write(x: PteT) -> bool;
    fn pte_dirty(x: PteT) -> bool;
    fn pte_unmap_unlock(pte: *mut PteT, ptl: *mut SpinlockT);
    fn spin_lock(ptl: *mut SpinlockT);
    fn spin_unlock(ptl: *mut SpinlockT);
    fn faulthandler_disabled() -> i32;
    fn mmap_read_lock(mm: *mut MmStruct);
    fn mmap_read_unlock(mm: *mut MmStruct);
    fn __put_user(value: u8, ptr: *mut u8) -> i32;
    fn uaccess_save_and_enable() -> usize;
    fn uaccess_restore(flags: usize);
    fn __memcpy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize);
    fn __memset(to: *mut core::ffi::c_void, value: i32, n: usize);
    fn __copy_to_user_std(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn uaccess_mask_range_ptr(to: *mut core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn __clear_user_std(addr: *mut core::ffi::c_void, n: usize) -> usize;
}

#[repr(C)] pub struct MmStruct { pub page_table_lock: SpinlockT }
#[repr(C)] pub struct PgdT { _private: [u8; 0] }
#[repr(C)] pub struct P4dT { _private: [u8; 0] }
#[repr(C)] pub struct PudT { _private: [u8; 0] }
#[repr(C)] pub struct PmdT { _private: [u8; 0] }
#[repr(C)] pub struct PteT { _private: [u8; 0] }
#[repr(C)] pub struct SpinlockT { _private: [u8; 0] }

extern "C" {
    static mut current_mm: *mut MmStruct;
}

const PAGE_MASK: usize = !4095usize;

unsafe fn pin_page_for_write(addr_ptr: *const core::ffi::c_void, ptep: *mut *mut PteT, ptlp: *mut *mut SpinlockT) -> i32 {
    let addr = addr_ptr as usize;
    let mm = current_mm;
    let pgd = pgd_offset(mm, addr);
    if pgd_none(*pgd) || pgd_bad(*pgd) { return 0; }
    let p4d = p4d_offset(pgd, addr);
    if p4d_none(*p4d) || p4d_bad(*p4d) { return 0; }
    let pud = pud_offset(p4d, addr);
    if pud_none(*pud) || pud_bad(*pud) { return 0; }
    let pmd = pmd_offset(pud, addr);
    if pmd_none(*pmd) { return 0; }

    if pmd_leaf(*pmd) {
        let ptl = &mut (*mm).page_table_lock as *mut SpinlockT;
        spin_lock(ptl);
        if !pmd_leaf(*pmd) || pmd_hugewillfault(*pmd) {
            spin_unlock(ptl);
            return 0;
        }
        *ptep = core::ptr::null_mut();
        *ptlp = ptl;
        return 1;
    }
    if pmd_bad(*pmd) { return 0; }
    let mut ptl = core::ptr::null_mut();
    let pte = pte_offset_map_lock(mm, pmd, addr, &mut ptl);
    if pte.is_null() { return 0; }
    if !pte_present(*pte) || !pte_young(*pte) || !pte_write(*pte) || !pte_dirty(*pte) {
        pte_unmap_unlock(pte, ptl);
        return 0;
    }
    *ptep = pte;
    *ptlp = ptl;
    1
}

unsafe fn __copy_to_user_memcpy(mut to: *mut core::ffi::c_void, mut from: *const core::ffi::c_void, mut n: usize) -> usize {
    let atomic = faulthandler_disabled() != 0;
    if !atomic { mmap_read_lock(current_mm); }
    while n != 0 {
        let mut pte = core::ptr::null_mut();
        let mut ptl = core::ptr::null_mut();
        while pin_page_for_write(to, &mut pte, &mut ptl) == 0 {
            if !atomic { mmap_read_unlock(current_mm); }
            if __put_user(0, to as *mut u8) != 0 { return n; }
            if !atomic { mmap_read_lock(current_mm); }
        }
        let mut tocopy = ((!((to as usize)) & !PAGE_MASK).wrapping_add(1));
        if tocopy > n { tocopy = n; }
        let flags = uaccess_save_and_enable();
        __memcpy(to, from, tocopy);
        uaccess_restore(flags);
        to = to.add(tocopy); from = from.add(tocopy); n -= tocopy;
        if !pte.is_null() { pte_unmap_unlock(pte, ptl); } else { spin_unlock(ptl); }
    }
    if !atomic { mmap_read_unlock(current_mm); }
    n
}

#[no_mangle]
pub unsafe extern "C" fn arm_copy_to_user(mut to: *mut core::ffi::c_void, from: *const core::ffi::c_void, mut n: usize) -> usize {
    if n < 64 {
        let flags = uaccess_save_and_enable(); n = __copy_to_user_std(to, from, n); uaccess_restore(flags);
    } else { n = __copy_to_user_memcpy(uaccess_mask_range_ptr(to, n), from, n); }
    n
}

unsafe fn __clear_user_memset(mut addr: *mut core::ffi::c_void, mut n: usize) -> usize {
    mmap_read_lock(current_mm);
    while n != 0 {
        let mut pte = core::ptr::null_mut(); let mut ptl = core::ptr::null_mut();
        while pin_page_for_write(addr, &mut pte, &mut ptl) == 0 {
            mmap_read_unlock(current_mm);
            if __put_user(0, addr as *mut u8) != 0 { return n; }
            mmap_read_lock(current_mm);
        }
        let mut tocopy = ((!((addr as usize)) & !PAGE_MASK).wrapping_add(1));
        if tocopy > n { tocopy = n; }
        let flags = uaccess_save_and_enable(); __memset(addr, 0, tocopy); uaccess_restore(flags);
        addr = addr.add(tocopy); n -= tocopy;
        if !pte.is_null() { pte_unmap_unlock(pte, ptl); } else { spin_unlock(ptl); }
    }
    mmap_read_unlock(current_mm);
    n
}

#[no_mangle]
pub unsafe extern "C" fn arm_clear_user(mut addr: *mut core::ffi::c_void, mut n: usize) -> usize {
    if n < 64 {
        let flags = uaccess_save_and_enable(); n = __clear_user_std(addr, n); uaccess_restore(flags);
    } else { n = __clear_user_memset(addr, n); }
    n
}

// The #if 0 benchmark and its disabled dependencies are intentionally retained as disabled source.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
