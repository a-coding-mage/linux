// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Derived from arch/ppc/mm/extable.c and arch/i386/mm/extable.c.
 *
 * Copyright (C) 2004 Paul Mackerras, IBM Corp.
 */

// Dependencies: linux/bsearch.h, linux/module.h, linux/init.h, linux/sort.h,
// linux/uaccess.h, linux/extable.h.
//
// lib/Kconfig limits RUST_EXTABLE to x86, whose relative entries carry
// insn, fixup and data and whose swap_ex_entry_fixup() also swaps data.

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::addr_of;
use kernel::bindings::{exception_table_entry, module};
use kernel::ffi::{c_int, c_ulong};

type CmpFn = unsafe extern "C" fn(*const c_void, *const c_void) -> c_int;
type SwapFn = unsafe extern "C" fn(*mut c_void, *mut c_void, c_int);

// `struct module` embeds zero-sized lockdep keys; only pointers cross here.
#[allow(improper_ctypes)]
extern "C" {
    fn sort(base: *mut c_void, num: usize, size: usize, cmp_func: Option<CmpFn>, swap_func: Option<SwapFn>);
    fn bsearch(key: *const c_void, base: *const c_void, num: usize, size: usize, cmp: Option<CmpFn>) -> *mut c_void;
    fn lupos_extable_within_module_init(addr: c_ulong, m: *const module) -> bool;
}

/// ex_to_insn(): the ARCH_HAS_RELATIVE_EXTABLE instruction address.
///
/// # Safety
/// `x` points to a valid exception table entry.
unsafe fn ex_to_insn(x: *const exception_table_entry) -> c_ulong {
    // SAFETY: x is valid; the displacement is relative to its own field.
    unsafe {
        let insn = addr_of!((*x).insn);
        (insn as c_ulong).wrapping_add((*insn) as c_ulong)
    }
}

/// swap_ex(): exchange two relative entries, rebasing their displacements.
///
/// # Safety
/// `a` and `b` point to entries of one exception table.
unsafe extern "C" fn swap_ex(a: *mut c_void, b: *mut c_void, _size: c_int) {
    // SAFETY: both entries are valid and distinct.
    unsafe {
        let x = a.cast::<exception_table_entry>();
        let y = b.cast::<exception_table_entry>();
        let delta = (b as isize).wrapping_sub(a as isize) as c_int;

        let tmp = *x;
        (*x).insn = (*y).insn.wrapping_add(delta);
        (*y).insn = tmp.insn.wrapping_sub(delta);

        // swap_ex_entry_fixup(x, y, tmp, delta)
        (*x).fixup = (*y).fixup.wrapping_add(delta);
        (*y).fixup = tmp.fixup.wrapping_sub(delta);
        (*x).data = (*y).data;
        (*y).data = tmp.data;
    }
}

/*
 * The exception table needs to be sorted so that the binary
 * search that we use to find entries in it works properly.
 * This is used both for the kernel exception table and for
 * the exception tables of modules that get loaded.
 */
/// # Safety
/// `a` and `b` point to exception table entries.
unsafe extern "C" fn cmp_ex_sort(a: *const c_void, b: *const c_void) -> c_int {
    // SAFETY: forwarded sort() contract.
    unsafe {
        let x = ex_to_insn(a.cast());
        let y = ex_to_insn(b.cast());

        /* avoid overflow */
        if x > y {
            return 1;
        }
        if x < y {
            return -1;
        }
    }
    0
}

/// Sorts an exception table by instruction address.
///
/// # Safety
/// `start..finish` is a writable exception table.
#[no_mangle]
pub unsafe extern "C" fn sort_extable(start: *mut exception_table_entry, finish: *mut exception_table_entry) {
    // SAFETY: forwarded caller contract.
    unsafe {
        sort(
            start.cast::<c_void>(),
            finish.offset_from(start) as usize,
            size_of::<exception_table_entry>(),
            Some(cmp_ex_sort),
            Some(swap_ex),
        );
    }
}

/*
 * If the exception table is sorted, any referring to the module init
 * will be at the beginning or the end.
 */
/// # Safety
/// `m` is a module whose exception table is sorted.
#[cfg(CONFIG_MODULES)]
#[no_mangle]
pub unsafe extern "C" fn trim_init_extable(m: *mut module) {
    // SAFETY: only entries inside the module's table are inspected.
    unsafe {
        /*trim the beginning*/
        while (*m).num_exentries != 0 && lupos_extable_within_module_init(ex_to_insn((*m).extable), m) {
            (*m).extable = (*m).extable.add(1);
            (*m).num_exentries = (*m).num_exentries.wrapping_sub(1);
        }
        /*trim the end*/
        while (*m).num_exentries != 0
            && lupos_extable_within_module_init(
                ex_to_insn((*m).extable.add((*m).num_exentries.wrapping_sub(1) as usize)),
                m,
            )
        {
            (*m).num_exentries = (*m).num_exentries.wrapping_sub(1);
        }
    }
}

/// # Safety
/// `key` points to an address and `elt` to an exception table entry.
unsafe extern "C" fn cmp_ex_search(key: *const c_void, elt: *const c_void) -> c_int {
    // SAFETY: forwarded bsearch() contract.
    unsafe {
        let key = *key.cast::<c_ulong>();
        let insn = ex_to_insn(elt.cast());

        /* avoid overflow */
        if key > insn {
            return 1;
        }
        if key < insn {
            return -1;
        }
    }
    0
}

/*
 * Search one exception table for an entry corresponding to the
 * given instruction address, and return the address of the entry,
 * or NULL if none is found.
 * We use a binary search, and thus we assume that the table is
 * already sorted.
 */
/// # Safety
/// `base` points to `num` sorted exception table entries.
#[no_mangle]
pub unsafe extern "C" fn search_extable(
    base: *const exception_table_entry,
    num: usize,
    value: c_ulong,
) -> *const exception_table_entry {
    // SAFETY: forwarded caller contract.
    unsafe {
        bsearch(
            addr_of!(value).cast::<c_void>(),
            base.cast::<c_void>(),
            num,
            size_of::<exception_table_entry>(),
            Some(cmp_ex_search),
        )
        .cast_const()
        .cast::<exception_table_entry>()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
