// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
 */

/*
 * This file reads all the special sections which have alternate instructions
 * which can be patched in or redirected to at runtime.
 */

// C dependencies:
// <stdlib.h>, <string.h>
// <arch/special.h>, <objtool/builtin.h>, <objtool/special.h>, <objtool/warn.h>

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct special_entry {
    sec: *const c_char,
    group: bool,
    jump_or_nop: bool,
    size: u8,
    orig: u8,
    new: u8,
    orig_len: u8,
    new_len: u8, /* group only */
    feature: u8, /* ALTERNATIVE macro CPU feature */
    key: u8,     /* jump_label key */
}

extern "C" {
    static ALT_ENTRY_SIZE: u8;
    static ALT_ORIG_OFFSET: u8;
    static ALT_ORIG_LEN_OFFSET: u8;
    static ALT_NEW_OFFSET: u8;
    static ALT_NEW_LEN_OFFSET: u8;
    static ALT_FEATURE_OFFSET: u8;

    static JUMP_ENTRY_SIZE: u8;
    static JUMP_ORIG_OFFSET: u8;
    static JUMP_NEW_OFFSET: u8;
    static JUMP_KEY_OFFSET: u8;

    static EX_ENTRY_SIZE: u8;
    static EX_ORIG_OFFSET: u8;
    static EX_NEW_OFFSET: u8;
}

#[repr(C)]
pub struct elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct section {
    pub name: *const c_char,
    pub data: *mut elf_data,
}

#[repr(C)]
pub struct elf_data {
    pub d_buf: *mut c_void,
}

#[repr(C)]
pub struct symbol {
    pub sec: *mut section,
    pub offset: c_ulong,
}

#[repr(C)]
pub struct reloc {
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct special_alt {
    pub list: list_head,
    pub group: bool,
    pub jump_or_nop: bool,
    pub orig_len: u8,
    pub new_len: u8,
    pub feature: u32,
    pub orig_sec: *mut section,
    pub orig_off: c_ulong,
    pub new_sec: *mut section,
    pub new_off: c_ulong,
    pub key_addend: c_ulong,
}

extern "C" {
    fn reloc_addend(reloc: *mut reloc) -> c_ulong;
    fn find_reloc_by_dest(elf: *mut elf, sec: *mut section, offset: c_ulong) -> *mut reloc;
    fn find_section_by_name(elf: *mut elf, name: *const c_char) -> *mut section;
    fn sec_size(sec: *mut section) -> c_ulong;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn ERROR_FUNC(sec: *mut section, offset: c_ulong, fmt: *const c_char, ...);
    fn ERROR(fmt: *const c_char, ...);
    fn ERROR_GLIBC(fmt: *const c_char, ...);
}

unsafe extern "C" fn arch_handle_alternative(_alt: *mut special_alt) {}

static ENTRIES: [special_entry; 4] = [
    special_entry {
        sec: b".altinstructions\0".as_ptr() as *const c_char,
        group: true,
        jump_or_nop: false,
        size: unsafe { ALT_ENTRY_SIZE },
        orig: unsafe { ALT_ORIG_OFFSET },
        orig_len: unsafe { ALT_ORIG_LEN_OFFSET },
        new: unsafe { ALT_NEW_OFFSET },
        new_len: unsafe { ALT_NEW_LEN_OFFSET },
        feature: unsafe { ALT_FEATURE_OFFSET },
        key: 0,
    },
    special_entry {
        sec: b"__jump_table\0".as_ptr() as *const c_char,
        group: false,
        jump_or_nop: true,
        size: unsafe { JUMP_ENTRY_SIZE },
        orig: unsafe { JUMP_ORIG_OFFSET },
        orig_len: 0,
        new: unsafe { JUMP_NEW_OFFSET },
        new_len: 0,
        feature: 0,
        key: unsafe { JUMP_KEY_OFFSET },
    },
    special_entry {
        sec: b"__ex_table\0".as_ptr() as *const c_char,
        group: false,
        jump_or_nop: false,
        size: unsafe { EX_ENTRY_SIZE },
        orig: unsafe { EX_ORIG_OFFSET },
        orig_len: 0,
        new: unsafe { EX_NEW_OFFSET },
        new_len: 0,
        feature: 0,
        key: 0,
    },
    special_entry {
        sec: core::ptr::null(),
        group: false,
        jump_or_nop: false,
        size: 0,
        orig: 0,
        new: 0,
        orig_len: 0,
        new_len: 0,
        feature: 0,
        key: 0,
    },
];

unsafe fn reloc_to_sec_off(reloc: *mut reloc, sec: *mut *mut section, off: *mut c_ulong) {
    *sec = (*(*reloc).sym).sec;
    *off = (*(*reloc).sym).offset.wrapping_add(reloc_addend(reloc));
}

unsafe fn get_alt_entry(
    elf: *mut elf,
    entry: *const special_entry,
    sec: *mut section,
    idx: c_int,
    alt: *mut special_alt,
) -> c_int {
    let mut orig_reloc: *mut reloc;
    let mut new_reloc: *mut reloc;
    let offset: c_ulong;

    offset = (idx as c_ulong).wrapping_mul((*entry).size as c_ulong);

    (*alt).group = (*entry).group;
    (*alt).jump_or_nop = (*entry).jump_or_nop;

    if (*alt).group {
        (*alt).orig_len = *((*(*sec).data).d_buf as *mut u8)
            .add(offset.wrapping_add((*entry).orig_len as c_ulong) as usize);
        (*alt).new_len = *((*(*sec).data).d_buf as *mut u8)
            .add(offset.wrapping_add((*entry).new_len as c_ulong) as usize);
        (*alt).feature = *((*(*sec).data).d_buf as *mut u8)
            .add(offset.wrapping_add((*entry).feature as c_ulong) as usize)
            as *mut u32);
    }

    orig_reloc = find_reloc_by_dest(elf, sec, offset.wrapping_add((*entry).orig as c_ulong));
    if orig_reloc.is_null() {
        ERROR_FUNC(
            sec,
            offset.wrapping_add((*entry).orig as c_ulong),
            b"can't find orig reloc\0".as_ptr() as *const c_char,
        );
        return -1;
    }

    reloc_to_sec_off(orig_reloc, &mut (*alt).orig_sec, &mut (*alt).orig_off);

    arch_handle_alternative(alt);

    if !(*entry).group || (*alt).new_len != 0 {
        new_reloc = find_reloc_by_dest(elf, sec, offset.wrapping_add((*entry).new as c_ulong));
        if new_reloc.is_null() {
            ERROR_FUNC(
                sec,
                offset.wrapping_add((*entry).new as c_ulong),
                b"can't find new reloc\0".as_ptr() as *const c_char,
            );
            return -1;
        }

        reloc_to_sec_off(new_reloc, &mut (*alt).new_sec, &mut (*alt).new_off);

        /* _ASM_EXTABLE_EX hack */
        if (*alt).new_off >= 0x7ffffff0 {
            (*alt).new_off = (*alt).new_off.wrapping_sub(0x7ffffff0);
        }
    }

    if (*entry).key != 0 {
        let mut key_reloc: *mut reloc;

        key_reloc = find_reloc_by_dest(elf, sec, offset.wrapping_add((*entry).key as c_ulong));
        if key_reloc.is_null() {
            ERROR_FUNC(
                sec,
                offset.wrapping_add((*entry).key as c_ulong),
                b"can't find key reloc\0".as_ptr() as *const c_char,
            );
            return -1;
        }
        (*alt).key_addend = reloc_addend(key_reloc);
    }

    0
}

/*
 * Read all the special sections and create a list of special_alt structs which
 * describe all the alternate instructions which can be patched in or
 * redirected to at runtime.
 */
#[no_mangle]
pub unsafe extern "C" fn special_get_alts(elf: *mut elf, alts: *mut list_head) -> c_int {
    let mut sec: *mut section;
    let mut nr_entries: u32;
    let mut alt: *mut special_alt;
    let mut idx: c_int;

    INIT_LIST_HEAD(alts);

    let mut entry = ENTRIES.as_ptr();
    while !(*entry).sec.is_null() {
        sec = find_section_by_name(elf, (*entry).sec);
        if sec.is_null() {
            entry = entry.add(1);
            continue;
        }

        if sec_size(sec) % ((*entry).size as c_ulong) != 0 {
            ERROR(
                b"%s size not a multiple of %d\0".as_ptr() as *const c_char,
                (*sec).name,
                (*entry).size as c_int,
            );
            return -1;
        }

        nr_entries = (sec_size(sec) / ((*entry).size as c_ulong)) as u32;

        idx = 0;
        while idx < nr_entries as c_int {
            alt = malloc(core::mem::size_of::<special_alt>()) as *mut special_alt;
            if alt.is_null() {
                ERROR_GLIBC(b"malloc failed\0".as_ptr() as *const c_char);
                return -1;
            }
            memset(
                alt as *mut c_void,
                0,
                core::mem::size_of::<special_alt>(),
            );

            if get_alt_entry(elf, entry, sec, idx, alt) != 0 {
                return -1;
            }

            list_add_tail(&mut (*alt).list, alts);
            idx += 1;
        }

        entry = entry.add(1);
    }

    0
}
