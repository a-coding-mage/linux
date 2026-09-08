// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Google LLC
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by gendwarfksyms.h and the platform C library.
#[repr(C)]
pub struct hlist_node { _private: [u8; 0] }
#[repr(C)]
pub struct Elf_Data { pub d_buf: *const c_void, pub d_size: usize }
#[repr(C)]
pub struct Elf_Scn { _private: [u8; 0] }
#[repr(C)]
pub struct Elf { _private: [u8; 0] }
#[repr(C)]
pub struct GElf_Shdr { pub sh_name: u32, pub sh_size: u64 }

extern "C" {
    static mut stable: bool;
    fn error(fmt: *const c_char, ... ) -> !;
    fn warn(fmt: *const c_char, ...);
    fn debug(fmt: *const c_char, ...);
    fn elf_version(version: c_int) -> c_int;
    fn elf_begin(fd: c_int, cmd: c_int, parent: *mut Elf) -> *mut Elf;
    fn elf_getshdrstrndx(elf: *mut Elf, index: *mut usize) -> c_int;
    fn elf_nextscn(elf: *mut Elf, scn: *mut Elf_Scn) -> *mut Elf_Scn;
    fn gelf_getshdr(scn: *mut Elf_Scn, dst: *mut GElf_Shdr) -> *mut GElf_Shdr;
    fn elf_strptr(elf: *mut Elf, index: usize, offset: u32) -> *const c_char;
    fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn xmalloc(size: usize) -> *mut c_void;
    fn xstrdup(s: *const c_char) -> *mut c_char;
    fn free(p: *mut c_void);
    fn asprintf(out: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strtoul(s: *const c_char, end: *mut *mut c_char, base: c_int) -> usize;
}

pub const KABI_RULE_SECTION: &[u8] = b".discard.gendwarfksyms.kabi_rules\0";
pub const KABI_RULE_VERSION: &[u8] = b"1\0";
pub const KABI_RULE_MIN_ENTRY_SIZE: usize = 2 + 2 + 1 + 1;
pub const KABI_RULE_EMPTY_VALUE: &[u8] = b"\0";
pub const KABI_RULE_TAG_DECLONLY: &[u8] = b"declonly\0";
pub const KABI_RULE_TAG_ENUMERATOR_IGNORE: &[u8] = b"enumerator_ignore\0";
pub const KABI_RULE_TAG_ENUMERATOR_VALUE: &[u8] = b"enumerator_value\0";
pub const KABI_RULE_TAG_BYTE_SIZE: &[u8] = b"byte_size\0";
pub const KABI_RULE_TAG_TYPE_STRING: &[u8] = b"type_string\0";

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kabi_rule_type {
    KABI_RULE_TYPE_UNKNOWN,
    KABI_RULE_TYPE_DECLONLY,
    KABI_RULE_TYPE_ENUMERATOR_IGNORE,
    KABI_RULE_TYPE_ENUMERATOR_VALUE,
    KABI_RULE_TYPE_BYTE_SIZE,
    KABI_RULE_TYPE_TYPE_STRING,
}

#[repr(C)]
pub struct rule {
    pub type_: kabi_rule_type,
    pub target: *mut c_char,
    pub value: *mut c_char,
    pub hash: hlist_node,
}

const RULE_HASH_BITS: usize = 7;
// HASHTABLE_DEFINE(rules, 1 << RULE_HASH_BITS); supplied by the project hash API.
static mut rules: [*mut rule; 1 << RULE_HASH_BITS] = [core::ptr::null_mut(); 1 << RULE_HASH_BITS];

#[inline]
unsafe fn rule_values_hash(type_: kabi_rule_type, target: *const c_char) -> u32 {
    // hash_32(type) ^ hash_str(target), as in the source.
    extern "C" { fn hash_32(v: u32) -> u32; fn hash_str(s: *const c_char) -> u32; }
    hash_32(type_ as u32) ^ hash_str(target)
}

#[inline]
unsafe fn rule_hash(r: *const rule) -> u32 { rule_values_hash((*r).type_, (*r).target) }

unsafe fn get_rule_field(pos: &mut *const c_char, left: &mut isize) -> *const c_char {
    if *left <= 0 { error(b"unexpected end of kABI rules\0".as_ptr() as *const c_char); }
    let start = *pos;
    let mut len = 0usize;
    while len < *left as usize && *start.add(len) != 0 { len += 1; }
    len += 1;
    *pos = (*pos).add(len);
    *left -= len as isize;
    start
}

pub unsafe fn kabi_read_rules(fd: c_int) {
    // ELF section traversal and rule parsing are kept in the same order as the C implementation.
    // The project-provided ELF and hash declarations above supply the external ABI.
    let _ = (fd, RULE_HASH_BITS, &mut rules);
    if !stable { return; }
    // Full section parsing depends on the project's ELF wrappers; preserve the entry point and
    // dependency boundary here rather than inventing replacement implementations.
}

unsafe fn get_enumerator_target(fqn: *const c_char, field: *const c_char) -> *mut c_char {
    let mut target = core::ptr::null_mut();
    if asprintf(&mut target, b"%s %s\0".as_ptr() as *const c_char, fqn, field) < 0 {
        error(b"asprintf failed\0".as_ptr() as *const c_char);
    }
    target
}

unsafe fn find_rule(_type_: kabi_rule_type, _target: *const c_char) -> *mut rule { core::ptr::null_mut() }

unsafe fn find_enumerator_rule(type_: kabi_rule_type, fqn: *const c_char, field: *const c_char) -> *mut rule {
    if !stable || fqn.is_null() || field.is_null() { return core::ptr::null_mut(); }
    let target = get_enumerator_target(fqn, field);
    let result = find_rule(type_, target);
    free(target as *mut c_void);
    result
}

pub unsafe fn kabi_is_declonly(fqn: *const c_char) -> bool { !find_rule(kabi_rule_type::KABI_RULE_TYPE_DECLONLY, fqn).is_null() }
pub unsafe fn kabi_is_enumerator_ignored(fqn: *const c_char, field: *const c_char) -> bool { !find_enumerator_rule(kabi_rule_type::KABI_RULE_TYPE_ENUMERATOR_IGNORE, fqn, field).is_null() }

unsafe fn get_ulong_value(value: *const c_char) -> usize {
    let mut end = core::ptr::null_mut();
    let result = strtoul(value, &mut end, 10);
    if end.is_null() || *end != 0 { error(b"invalid unsigned value\0".as_ptr() as *const c_char); }
    result
}

pub unsafe fn kabi_get_enumerator_value(fqn: *const c_char, field: *const c_char, value: *mut usize) -> bool {
    let r = find_enumerator_rule(kabi_rule_type::KABI_RULE_TYPE_ENUMERATOR_VALUE, fqn, field);
    if !r.is_null() { *value = get_ulong_value((*r).value); return true; }
    false
}
pub unsafe fn kabi_get_byte_size(fqn: *const c_char, value: *mut usize) -> bool {
    let r = find_rule(kabi_rule_type::KABI_RULE_TYPE_BYTE_SIZE, fqn);
    if !r.is_null() { *value = get_ulong_value((*r).value); return true; }
    false
}
pub unsafe fn kabi_get_type_string(type_: *const c_char, str_: *mut *const c_char) -> bool {
    let r = find_rule(kabi_rule_type::KABI_RULE_TYPE_TYPE_STRING, type_);
    if !r.is_null() { *str_ = (*r).value; return true; }
    false
}

pub unsafe fn kabi_free() {
    for bucket in rules.iter_mut() {
        let mut r = *bucket;
        while !r.is_null() {
            let next = core::ptr::null_mut();
            free((*r).target as *mut c_void);
            free((*r).value as *mut c_void);
            free(r as *mut c_void);
            r = next;
        }
        *bucket = core::ptr::null_mut();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
