// SPDX-License-Identifier: GPL-2.0-only
/*
 * System call table mapper
 *
 * (C) 2016 Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

// Translated from includes:
// "syscalltbl.h", <asm/bitsperlong.h>, <linux/compiler.h>,
// <linux/kernel.h>, <linux/zalloc.h>, <string.h>, "string2.h".
// "trace/beauty/generated/syscalltbl.c" is expected to provide syscalltbls.

extern "C" {
    static syscalltbls: [syscalltbl; 0];

    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: usize,
        size: usize,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strglobmatch(str_: *const c_char, pat: *const c_char) -> bool;
}

extern "C" {
    static syscalltbls_len: usize;
}

const EM_NONE: c_int = 0;
const EM_MIPS: c_int = 8;
const EM_SPARC: c_int = 2;
const EM_SPARCV9: c_int = 43;

#[repr(C)]
pub struct syscalltbl {
    pub e_machine: c_int,
    pub num_to_name: *const *const c_char,
    pub num_to_name_len: c_int,
    pub sorted_names: *const u16,
    pub sorted_names_len: c_int,
}

unsafe fn find_table(mut e_machine: c_int) -> *const syscalltbl {
    static mut LAST_TABLE: *const syscalltbl = ptr::null();
    static mut LAST_TABLE_MACHINE: c_int = EM_NONE;

    /* Tables only exist for EM_SPARC. */
    if e_machine == EM_SPARCV9 {
        e_machine = EM_SPARC;
    }

    if LAST_TABLE_MACHINE == e_machine && !LAST_TABLE.is_null() {
        return LAST_TABLE;
    }

    let mut i: usize = 0;
    while i < syscalltbls_len {
        let entry = syscalltbls.as_ptr().add(i);

        if (*entry).e_machine != e_machine && (*entry).e_machine != EM_NONE {
            i += 1;
            continue;
        }

        LAST_TABLE = entry;
        LAST_TABLE_MACHINE = e_machine;
        return entry;
    }
    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn syscalltbl__name(e_machine: c_int, mut id: c_int) -> *const c_char {
    let table = find_table(e_machine);

    if e_machine == EM_MIPS && id > 1000 {
        /*
         * MIPS may encode the N32/64/O32 type in the high part of
         * syscall number. Mask this off if present. See the values of
         * __NR_N32_Linux, __NR_64_Linux, __NR_O32_Linux and __NR_Linux.
         */
        id = id % 1000;
    }
    if !table.is_null() && id >= 0 && id < (*table).num_to_name_len {
        return *(*table).num_to_name.add(id as usize);
    }
    ptr::null()
}

#[repr(C)]
struct syscall_cmp_key {
    name: *const c_char,
    tbl: *const *const c_char,
}

unsafe extern "C" fn syscallcmpname(vkey: *const c_void, ventry: *const c_void) -> c_int {
    let key = vkey as *const syscall_cmp_key;
    let entry = ventry as *const u16;

    strcmp((*key).name, *(*key).tbl.add(*entry as usize))
}

#[no_mangle]
pub unsafe extern "C" fn syscalltbl__id(e_machine: c_int, name: *const c_char) -> c_int {
    let table = find_table(e_machine);
    let mut key: syscall_cmp_key = syscall_cmp_key {
        name: ptr::null(),
        tbl: ptr::null(),
    };
    let id: *const u16;

    if table.is_null() {
        return -1;
    }

    key.name = name;
    key.tbl = (*table).num_to_name;
    id = bsearch(
        &key as *const syscall_cmp_key as *const c_void,
        (*table).sorted_names as *const c_void,
        (*table).sorted_names_len as usize,
        size_of::<u16>(),
        Some(syscallcmpname),
    ) as *const u16;

    if !id.is_null() {
        *id as c_int
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn syscalltbl__num_idx(e_machine: c_int) -> c_int {
    let table = find_table(e_machine);

    if table.is_null() {
        return 0;
    }

    (*table).sorted_names_len
}

#[no_mangle]
pub unsafe extern "C" fn syscalltbl__id_at_idx(e_machine: c_int, idx: c_int) -> c_int {
    let table = find_table(e_machine);

    if table.is_null() {
        return -1;
    }

    assert!(idx >= 0 && idx < (*table).sorted_names_len);
    *(*table).sorted_names.add(idx as usize) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn syscalltbl__strglobmatch_next(
    e_machine: c_int,
    syscall_glob: *const c_char,
    idx: *mut c_int,
) -> c_int {
    let table = find_table(e_machine);

    let mut i = *idx + 1;
    while !table.is_null() && i < (*table).sorted_names_len {
        let name = *(*table)
            .num_to_name
            .add(*(*table).sorted_names.add(i as usize) as usize);

        if strglobmatch(name, syscall_glob) {
            *idx = i;
            return *(*table).sorted_names.add(i as usize) as c_int;
        }

        i += 1;
    }

    -1
}

#[no_mangle]
pub unsafe extern "C" fn syscalltbl__strglobmatch_first(
    e_machine: c_int,
    syscall_glob: *const c_char,
    idx: *mut c_int,
) -> c_int {
    *idx = -1;
    syscalltbl__strglobmatch_next(e_machine, syscall_glob, idx)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
