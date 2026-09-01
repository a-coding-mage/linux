// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from objtool/arch/loongarch/special.c.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const C_JUMP_TABLE_SECTION: *const c_char = b".rodata..c_jump_table\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objtool_file {
    pub elf: *mut elf,
}

#[repr(C)]
pub struct section {
    pub name: *const c_char,
    pub rsec: *mut section,
    pub rodata: bool,
    pub relocs: *mut reloc,
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
pub struct instruction {
    pub sec: *mut section,
    pub offset: c_ulong,
}

#[repr(C)]
pub struct special_alt {
    _private: [u8; 0],
}

#[repr(C)]
struct table_info {
    jump_info: list_head,
    insn_offset: c_ulong,
    rodata_offset: c_ulong,
}

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn malloc(size: usize) -> *mut c_void;

    fn WARN(fmt: *const c_char, ...);

    fn find_section_by_name(elf: *mut elf, name: *const c_char) -> *mut section;
    fn reloc_addend(reloc: *mut reloc) -> c_ulong;
    fn reloc_idx(reloc: *mut reloc) -> c_ulong;
    fn sec_num_entries(sec: *mut section) -> c_ulong;
    fn reloc_offset(reloc: *mut reloc) -> c_ulong;
    fn find_reloc_by_dest(elf: *mut elf, sec: *mut section, offset: c_ulong) -> *mut reloc;
}

#[inline]
unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

#[inline]
unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = new;
        (*new).next = next;
        (*new).prev = prev;
        (*prev).next = new;
    }
}

#[inline]
unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    unsafe {
        __list_add(new, (*head).prev, head);
    }
}

#[inline]
unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = prev;
        (*prev).next = next;
    }
}

#[inline]
unsafe fn list_del_init(entry: *mut list_head) {
    unsafe {
        __list_del((*entry).prev, (*entry).next);
        INIT_LIST_HEAD(entry);
    }
}

#[inline]
unsafe fn list_entry_table_info(ptr: *mut list_head) -> *mut table_info {
    ptr as *mut table_info
}

#[inline]
unsafe fn list_next_entry_table_info(pos: *mut table_info) -> *mut table_info {
    unsafe { list_entry_table_info((*pos).jump_info.next) }
}

#[inline]
unsafe fn first_reloc(sec: *mut section) -> *mut reloc {
    unsafe { (*sec).relocs }
}

#[inline]
unsafe fn reloc_at(sec: *mut section, idx: c_ulong) -> *mut reloc {
    unsafe { first_reloc(sec).add(idx as usize) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_support_alt_relocation(
    _special_alt: *mut special_alt,
    _insn: *mut instruction,
    _reloc: *mut reloc,
) -> bool {
    false
}

unsafe fn get_rodata_table_size_by_table_annotate(
    file: *mut objtool_file,
    insn: *mut instruction,
    table_size: *mut c_ulong,
) {
    unsafe {
        let mut rsec: *mut section;
        let mut reloc: *mut reloc;
        let mut table_list = list_head {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let mut orig_table: *mut table_info;
        let mut next_table: *mut table_info;
        let mut tmp_insn_offset: c_ulong;
        let mut tmp_rodata_offset: c_ulong;
        let mut is_valid_list = false;

        rsec = find_section_by_name(
            (*file).elf,
            b".rela.discard.tablejump_annotate\0".as_ptr() as *const c_char,
        );
        if rsec.is_null() {
            return;
        }

        INIT_LIST_HEAD(&mut table_list);

        let mut i: c_ulong = 0;
        while i < sec_num_entries(rsec) {
            reloc = reloc_at(rsec, i);

            if (*(*(*reloc).sym).sec).rodata {
                i += 1;
                continue;
            }

            if strcmp((*(*insn).sec).name, (*(*(*reloc).sym).sec).name) != 0 {
                i += 1;
                continue;
            }

            orig_table = malloc(size_of::<table_info>()) as *mut table_info;
            if orig_table.is_null() {
                WARN(b"malloc failed\0".as_ptr() as *const c_char);
                return;
            }

            (*orig_table).insn_offset = (*(*reloc).sym).offset.wrapping_add(reloc_addend(reloc));
            reloc = reloc.add(1);
            i += 1;
            (*orig_table).rodata_offset = (*(*reloc).sym).offset.wrapping_add(reloc_addend(reloc));

            list_add_tail(&mut (*orig_table).jump_info, &mut table_list);

            if reloc_idx(reloc).wrapping_add(1) == sec_num_entries(rsec) {
                break;
            }

            if strcmp((*(*insn).sec).name, (*(*(*reloc.add(1)).sym).sec).name) != 0 {
                let mut pos = table_list.next;
                while pos != &mut table_list {
                    orig_table = list_entry_table_info(pos);
                    if (*orig_table).insn_offset == (*insn).offset {
                        is_valid_list = true;
                        break;
                    }
                    pos = (*pos).next;
                }

                if !is_valid_list {
                    list_del_init(&mut table_list);
                    i += 1;
                    continue;
                }

                break;
            }

            i += 1;
        }

        let mut pos = table_list.next;
        while pos != &mut table_list {
            orig_table = list_entry_table_info(pos);
            next_table = list_next_entry_table_info(orig_table);
            let mut from = &mut (*next_table).jump_info as *mut list_head;
            while from != &mut table_list {
                if (*next_table).rodata_offset < (*orig_table).rodata_offset {
                    tmp_insn_offset = (*next_table).insn_offset;
                    tmp_rodata_offset = (*next_table).rodata_offset;
                    (*next_table).insn_offset = (*orig_table).insn_offset;
                    (*next_table).rodata_offset = (*orig_table).rodata_offset;
                    (*orig_table).insn_offset = tmp_insn_offset;
                    (*orig_table).rodata_offset = tmp_rodata_offset;
                }
                from = (*from).next;
                next_table = list_entry_table_info(from);
            }
            pos = (*pos).next;
        }

        pos = table_list.next;
        while pos != &mut table_list {
            orig_table = list_entry_table_info(pos);
            if (*insn).offset == (*orig_table).insn_offset {
                next_table = list_next_entry_table_info(orig_table);
                if &mut (*next_table).jump_info as *mut list_head == &mut table_list {
                    *table_size = 0;
                    return;
                }

                while (*next_table).rodata_offset == (*orig_table).rodata_offset {
                    next_table = list_next_entry_table_info(next_table);
                    if &mut (*next_table).jump_info as *mut list_head == &mut table_list {
                        *table_size = 0;
                        return;
                    }
                }

                *table_size = (*next_table)
                    .rodata_offset
                    .wrapping_sub((*orig_table).rodata_offset);
            }
            pos = (*pos).next;
        }
    }
}

unsafe fn find_reloc_by_table_annotate(
    file: *mut objtool_file,
    insn: *mut instruction,
    table_size: *mut c_ulong,
) -> *mut reloc {
    unsafe {
        let mut rsec: *mut section;
        let mut reloc: *mut reloc;
        let offset: c_ulong;

        rsec = find_section_by_name(
            (*file).elf,
            b".rela.discard.tablejump_annotate\0".as_ptr() as *const c_char,
        );
        if rsec.is_null() {
            return ptr::null_mut();
        }

        let mut i: c_ulong = 0;
        while i < sec_num_entries(rsec) {
            reloc = reloc_at(rsec, i);
            if (*(*(*reloc).sym).sec).rodata {
                i += 1;
                continue;
            }

            if strcmp((*(*insn).sec).name, (*(*(*reloc).sym).sec).name) != 0 {
                i += 1;
                continue;
            }

            offset = (*(*reloc).sym).offset.wrapping_add(reloc_addend(reloc));
            if (*insn).offset == offset {
                get_rodata_table_size_by_table_annotate(file, insn, table_size);
                reloc = reloc.add(1);
                return reloc;
            }

            i += 1;
        }

        ptr::null_mut()
    }
}

unsafe fn find_reloc_of_rodata_c_jump_table(
    sec: *mut section,
    offset: c_ulong,
    table_size: *mut c_ulong,
) -> *mut reloc {
    unsafe {
        let mut rsec: *mut section;
        let mut reloc: *mut reloc;

        rsec = (*sec).rsec;
        if rsec.is_null() {
            return ptr::null_mut();
        }

        let mut i: c_ulong = 0;
        while i < sec_num_entries(rsec) {
            reloc = reloc_at(rsec, i);
            if reloc_offset(reloc) > offset {
                break;
            }

            if strcmp((*(*(*reloc).sym).sec).name, C_JUMP_TABLE_SECTION) == 0 {
                *table_size = 0;
                return reloc;
            }

            i += 1;
        }

        ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_find_switch_table(
    file: *mut objtool_file,
    insn: *mut instruction,
    table_size: *mut c_ulong,
) -> *mut reloc {
    unsafe {
        let mut annotate_reloc: *mut reloc;
        let rodata_reloc: *mut reloc;
        let table_sec: *mut section;
        let table_offset: c_ulong;

        annotate_reloc = find_reloc_by_table_annotate(file, insn, table_size);
        if annotate_reloc.is_null() {
            annotate_reloc = find_reloc_of_rodata_c_jump_table((*insn).sec, (*insn).offset, table_size);
            if annotate_reloc.is_null() {
                return ptr::null_mut();
            }
        }

        table_sec = (*(*annotate_reloc).sym).sec;
        table_offset = (*(*annotate_reloc).sym)
            .offset
            .wrapping_add(reloc_addend(annotate_reloc));

        /*
         * Each table entry has a rela associated with it.  The rela
         * should reference text in the same function as the original
         * instruction.
         */
        rodata_reloc = find_reloc_by_dest((*file).elf, table_sec, table_offset);
        if rodata_reloc.is_null() {
            return ptr::null_mut();
        }

        rodata_reloc
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_cpu_feature_name(_feature_number: c_int) -> *const c_char {
    ptr::null()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
