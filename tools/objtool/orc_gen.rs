// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017 Josh Poimboeuf <jpoimboe@redhat.com>
 */

// Rust translation of objtool/orc_gen.c.
// C include dependencies:
// - <stdlib.h>
// - <string.h>
// - <linux/objtool_types.h>
// - <asm/orc_types.h>
// - <objtool/check.h>
// - <objtool/orc.h>
// - <objtool/warn.h>

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

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
pub struct shdr {
    pub sh_size: c_ulong,
}

#[repr(C)]
pub struct section {
    pub list: list_head,
    pub text: bool,
    pub sh: shdr,
}

#[repr(C)]
pub struct instruction {
    pub list: list_head,
    pub sec: *mut section,
    pub offset: c_ulong,
    pub len: c_ulong,
    pub cfi: *mut cfi_state,
    pub alt_group: *mut alt_group,
}

#[repr(C)]
pub struct alt_group {
    pub first_insn: *mut instruction,
    pub last_insn: *mut instruction,
    pub cfi: *mut *mut cfi_state,
}

#[repr(C)]
pub struct cfi_state {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orc_entry {
    pub sp_reg: i16,
    pub bp_reg: i16,
    pub sp_offset: i16,
    pub bp_offset: i16,
    pub type_: u8,
    pub end: u8,
}

#[repr(C)]
pub struct orc_list_entry {
    pub list: list_head,
    pub orc: orc_entry,
    pub insn_sec: *mut section,
    pub insn_off: c_ulong,
}

pub const ORC_TYPE_UNDEFINED: u8 = 0;
pub const SHT_PROGBITS: u32 = 1;
pub const SHF_ALLOC: c_ulong = 2;

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn WARN(fmt: *const c_char, ...);
    fn init_orc_entry(orc: *mut orc_entry, cfi: *mut cfi_state, insn: *mut instruction) -> c_int;
    fn find_section_by_name(elf: *mut elf, name: *const c_char) -> *mut section;
    fn elf_create_section(
        elf: *mut elf,
        name: *const c_char,
        size: usize,
        entsize: usize,
        sh_type: u32,
        nr: c_int,
        flags: c_ulong,
    ) -> *mut section;
    fn elf_create_section_pair(
        elf: *mut elf,
        name: *const c_char,
        entsize: usize,
        nr: c_uint,
        reloc_nr: c_uint,
    ) -> *mut section;
    fn write_orc_entry(
        elf: *mut elf,
        orc_sec: *mut section,
        ip_sec: *mut section,
        idx: c_uint,
        insn_sec: *mut section,
        insn_off: c_ulong,
        orc: *mut orc_entry,
    ) -> c_int;

    static mut section_list: list_head;
}

pub type c_uint = u32;

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    unsafe {
        let prev = (*head).prev;
        (*new).next = head;
        (*new).prev = prev;
        (*prev).next = new;
        (*head).prev = new;
    }
}

unsafe fn list_entry<T>(ptr: *mut list_head, offset: usize) -> *mut T {
    unsafe { (ptr as *mut u8).sub(offset) as *mut T }
}

const fn offset_of_orc_list_entry_list() -> usize {
    0
}

unsafe fn orc_list_add(
    orc_list: *mut list_head,
    orc: *mut orc_entry,
    sec: *mut section,
    offset: c_ulong,
) -> c_int {
    unsafe {
        let entry = malloc(mem::size_of::<orc_list_entry>()) as *mut orc_list_entry;

        if entry.is_null() {
            WARN(c"malloc failed".as_ptr());
            return -1;
        }

        (*entry).orc = *orc;
        (*entry).insn_sec = sec;
        (*entry).insn_off = offset;

        list_add_tail(&mut (*entry).list, orc_list);
        0
    }
}

unsafe fn alt_group_len(alt_group: *mut alt_group) -> c_ulong {
    unsafe {
        (*(*alt_group).last_insn).offset
            + (*(*alt_group).last_insn).len
            - (*(*alt_group).first_insn).offset
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn orc_create(file: *mut objtool_file) -> c_int {
    unsafe {
        let mut sec: *mut section;
        let orc_sec: *mut section;
        let mut nr: c_uint = 0;
        let mut idx: c_uint = 0;
        let mut entry: *mut orc_list_entry;
        let mut orc_list = list_head {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        let null = orc_entry {
            sp_reg: 0,
            bp_reg: 0,
            sp_offset: 0,
            bp_offset: 0,
            type_: ORC_TYPE_UNDEFINED,
            end: 0,
        };

        /* Build a deduplicated list of ORC entries: */
        INIT_LIST_HEAD(&mut orc_list);
        sec = section_list.next as *mut section;
        while &mut (*sec).list as *mut list_head != &mut section_list {
            let mut orc = mem::zeroed::<orc_entry>();
            let mut prev_orc = mem::zeroed::<orc_entry>();
            let mut insn: *mut instruction;
            let mut empty = true;

            if !(*sec).text {
                sec = (*sec).list.next as *mut section;
                continue;
            }

            insn = ptr::null_mut();
            while !insn.is_null() {
                let alt_group = (*insn).alt_group;
                let mut i: c_int;

                if alt_group.is_null() {
                    if init_orc_entry(&mut orc, (*insn).cfi, insn) != 0 {
                        return -1;
                    }
                    if memcmp(
                        &prev_orc as *const _ as *const c_void,
                        &orc as *const _ as *const c_void,
                        mem::size_of_val(&orc),
                    ) == 0
                    {
                        insn = (*insn).list.next as *mut instruction;
                        continue;
                    }
                    if orc_list_add(&mut orc_list, &mut orc, sec, (*insn).offset) != 0 {
                        return -1;
                    }
                    nr += 1;
                    prev_orc = orc;
                    empty = false;
                    insn = (*insn).list.next as *mut instruction;
                    continue;
                }

                /*
                 * Alternatives can have different stack layout
                 * possibilities (but they shouldn't conflict).
                 * Instead of traversing the instructions, use the
                 * alt_group's flattened byte-offset-addressed CFI
                 * array.
                 */
                i = 0;
                while (i as c_ulong) < alt_group_len(alt_group) {
                    let cfi = *(*alt_group).cfi.add(i as usize);
                    if cfi.is_null() {
                        i += 1;
                        continue;
                    }
                    /* errors are reported on the original insn */
                    if init_orc_entry(&mut orc, cfi, insn) != 0 {
                        return -1;
                    }
                    if memcmp(
                        &prev_orc as *const _ as *const c_void,
                        &orc as *const _ as *const c_void,
                        mem::size_of_val(&orc),
                    ) == 0
                    {
                        i += 1;
                        continue;
                    }
                    if orc_list_add(
                        &mut orc_list,
                        &mut orc,
                        (*insn).sec,
                        (*insn).offset + i as c_ulong,
                    ) != 0
                    {
                        return -1;
                    }
                    nr += 1;
                    prev_orc = orc;
                    empty = false;
                    i += 1;
                }

                /* Skip to the end of the alt_group */
                insn = (*alt_group).last_insn;
                insn = (*insn).list.next as *mut instruction;
            }

            /* Add a section terminator */
            if !empty {
                orc_list_add(&mut orc_list, &null as *const _ as *mut orc_entry, sec, (*sec).sh.sh_size);
                nr += 1;
            }

            sec = (*sec).list.next as *mut section;
        }
        if nr == 0 {
            return 0;
        }

        /* Create .orc_unwind, .orc_unwind_ip and .rela.orc_unwind_ip sections: */
        sec = find_section_by_name((*file).elf, c".orc_unwind".as_ptr());
        if !sec.is_null() {
            WARN(c"file already has .orc_unwind section, skipping".as_ptr());
            return -1;
        }
        orc_sec = elf_create_section(
            (*file).elf,
            c".orc_unwind".as_ptr(),
            nr as usize * mem::size_of::<orc_entry>(),
            mem::size_of::<orc_entry>(),
            SHT_PROGBITS,
            1,
            SHF_ALLOC,
        );
        if orc_sec.is_null() {
            return -1;
        }

        sec = elf_create_section_pair((*file).elf, c".orc_unwind_ip".as_ptr(), mem::size_of::<c_int>(), nr, nr);
        if sec.is_null() {
            return -1;
        }

        /* Write ORC entries to sections: */
        let mut pos = orc_list.next;
        while pos != &mut orc_list {
            entry = list_entry::<orc_list_entry>(pos, offset_of_orc_list_entry_list());
            if write_orc_entry(
                (*file).elf,
                orc_sec,
                sec,
                idx,
                (*entry).insn_sec,
                (*entry).insn_off,
                &mut (*entry).orc,
            ) != 0
            {
                return -1;
            }
            idx += 1;
            pos = (*pos).next;
        }

        0
    }
}
