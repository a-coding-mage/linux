// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Derived from arch/ppc/mm/extable.c and arch/i386/mm/extable.c.
 *
 * Copyright (C) 2004 Paul Mackerras, IBM Corp.
 */

// Required declarations are supplied by the corresponding kernel headers.

#[repr(C)]
pub struct exception_table_entry {
    pub insn: usize,
    pub fixup: usize,
}

#[repr(C)]
pub struct module {
    pub num_exentries: usize,
    pub extable: *mut exception_table_entry,
}

#[cfg(not(feature = "ARCH_HAS_RELATIVE_EXTABLE"))]
#[inline]
unsafe fn ex_to_insn(x: *const exception_table_entry) -> usize {
    (*x).insn
}

#[cfg(feature = "ARCH_HAS_RELATIVE_EXTABLE")]
#[inline]
unsafe fn ex_to_insn(x: *const exception_table_entry) -> usize {
    (core::ptr::addr_of!((*x).insn) as usize).wrapping_add((*x).insn)
}

#[cfg(feature = "ARCH_HAS_RELATIVE_EXTABLE")]
unsafe fn swap_ex(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, _size: i32) {
    let x = a as *mut exception_table_entry;
    let y = b as *mut exception_table_entry;
    let tmp = core::ptr::read(x);
    let delta = (b as isize).wrapping_sub(a as isize) as usize;

    (*x).insn = (*y).insn.wrapping_add(delta);
    (*y).insn = tmp.insn.wrapping_sub(delta);

    // swap_ex_entry_fixup is a build-time hook supplied by the architecture.
    (*x).fixup = (*y).fixup.wrapping_add(delta);
    (*y).fixup = tmp.fixup.wrapping_sub(delta);
}

/*
 * The exception table needs to be sorted so that the binary
 * search that we use to find entries in it works properly.
 * This is used both for the kernel exception table and for
 * the exception tables of modules that get loaded.
 */
unsafe extern "C" {
    fn sort(
        base: *mut core::ffi::c_void,
        num: usize,
        size: usize,
        cmp: unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> i32,
        swap: *const core::ffi::c_void,
    );
    fn bsearch(
        key: *const core::ffi::c_void,
        base: *const core::ffi::c_void,
        num: usize,
        size: usize,
        cmp: unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> i32,
    ) -> *mut core::ffi::c_void;
    fn within_module_init(addr: usize, m: *const module) -> bool;
}

unsafe extern "C" fn cmp_ex_sort(
    a: *const core::ffi::c_void,
    b: *const core::ffi::c_void,
) -> i32 {
    let x = a as *const exception_table_entry;
    let y = b as *const exception_table_entry;

    /* avoid overflow */
    if ex_to_insn(x) > ex_to_insn(y) {
        return 1;
    }
    if ex_to_insn(x) < ex_to_insn(y) {
        return -1;
    }
    0
}

pub unsafe fn sort_extable(
    start: *mut exception_table_entry,
    finish: *mut exception_table_entry,
) {
    let count = (finish as usize).wrapping_sub(start as usize)
        / core::mem::size_of::<exception_table_entry>();
    #[cfg(feature = "ARCH_HAS_RELATIVE_EXTABLE")]
    let swap = swap_ex as *const core::ffi::c_void;
    #[cfg(not(feature = "ARCH_HAS_RELATIVE_EXTABLE"))]
    let swap = core::ptr::null();
    sort(
        start as *mut core::ffi::c_void,
        count,
        core::mem::size_of::<exception_table_entry>(),
        cmp_ex_sort,
        swap,
    );
}

#[cfg(feature = "CONFIG_MODULES")]
pub unsafe fn trim_init_extable(m: *mut module) {
    /* trim the beginning */
    while (*m).num_exentries != 0
        && within_module_init(ex_to_insn((*m).extable), m)
    {
        (*m).extable = (*m).extable.add(1);
        (*m).num_exentries -= 1;
    }
    /* trim the end */
    while (*m).num_exentries != 0
        && within_module_init(
            ex_to_insn((*m).extable.add((*m).num_exentries - 1)),
            m,
        )
    {
        (*m).num_exentries -= 1;
    }
}

unsafe extern "C" fn cmp_ex_search(
    key: *const core::ffi::c_void,
    elt: *const core::ffi::c_void,
) -> i32 {
    let entry = elt as *const exception_table_entry;
    let key = *(key as *const usize);

    /* avoid overflow */
    if key > ex_to_insn(entry) {
        return 1;
    }
    if key < ex_to_insn(entry) {
        return -1;
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
pub unsafe fn search_extable(
    base: *const exception_table_entry,
    num: usize,
    value: usize,
) -> *const exception_table_entry {
    bsearch(
        &value as *const usize as *const core::ffi::c_void,
        base as *const core::ffi::c_void,
        num,
        core::mem::size_of::<exception_table_entry>(),
        cmp_ex_search,
    ) as *const exception_table_entry
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
