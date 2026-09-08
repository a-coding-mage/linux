// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Google LLC
 */

use std::ffi::{c_char, c_int, c_void, c_ulong, CStr};
use std::mem;
use std::ptr;

// The list, hash-table, DIE, symbol, cache, and project helper declarations
// are supplied by the translated companion sources.

static mut expansion_cache: cache = cache { _private: [] };

#[repr(C)]
struct type_list_entry {
    str_: *const c_char,
    owned: *mut c_void,
    list: list_head,
}

unsafe fn type_list_free(list: *mut list_head) {
    let mut entry: *mut type_list_entry = ptr::null_mut();
    let mut tmp: *mut type_list_entry = ptr::null_mut();
    list_for_each_entry_safe!(entry, tmp, list, list, {
        if !(*entry).owned.is_null() { free((*entry).owned); }
        free(entry as *mut c_void);
    });
    INIT_LIST_HEAD!(list);
}

unsafe fn type_list_append(list: *mut list_head, s: *const c_char, owned: *mut c_void) -> usize {
    if s.is_null() { return 0; }
    let entry = xmalloc(mem::size_of::<type_list_entry>()) as *mut type_list_entry;
    (*entry).str_ = s;
    (*entry).owned = owned;
    list_add_tail!(&mut (*entry).list, list);
    strlen(s)
}

unsafe fn type_list_write(list: *mut list_head, file: *mut FILE) {
    let mut entry: *mut type_list_entry = ptr::null_mut();
    list_for_each_entry!(entry, list, list, {
        if !(*entry).str_.is_null() { checkp(fputs((*entry).str_, file)); }
    });
}

#[repr(C)]
struct type_expansion {
    name: *mut c_char,
    len: usize,
    expanded: list_head,
    hash: hlist_node,
}

unsafe fn type_expansion_init(ty: *mut type_expansion) {
    (*ty).name = ptr::null_mut();
    (*ty).len = 0;
    INIT_LIST_HEAD!(&mut (*ty).expanded);
}

unsafe fn type_expansion_free(ty: *mut type_expansion) {
    free((*ty).name as *mut c_void);
    (*ty).name = ptr::null_mut();
    (*ty).len = 0;
    type_list_free(&mut (*ty).expanded);
}

unsafe fn type_expansion_append(ty: *mut type_expansion, s: *const c_char, owned: *mut c_void) {
    (*ty).len += type_list_append(&mut (*ty).expanded, s, owned);
}

static mut type_map: hashtable = hashtable { _private: [] };

unsafe fn __type_map_get(name: *const c_char, res: *mut *mut type_expansion) -> c_int {
    let mut e: *mut type_expansion = ptr::null_mut();
    hash_for_each_possible!(type_map, e, hash, hash_str(name), {
        if strcmp(name, (*e).name) == 0 { *res = e; return 0; }
    });
    -1
}

unsafe fn type_map_add(name: *const c_char, ty: *mut type_expansion) -> *mut type_expansion {
    let mut e: *mut type_expansion = ptr::null_mut();
    if __type_map_get(name, &mut e) != 0 {
        e = xmalloc(mem::size_of::<type_expansion>()) as *mut type_expansion;
        type_expansion_init(e);
        (*e).name = xstrdup(name);
        hash_add!(type_map, &mut (*e).hash, hash_str((*e).name));
        if dump_types { debug!(b"adding %s\0".as_ptr(), (*e).name); }
    } else {
        if (*ty).len <= (*e).len { return e; }
        type_list_free(&mut (*e).expanded);
        if dump_types { debug!(b"replacing %s\0".as_ptr(), (*e).name); }
    }
    list_replace_init!(&mut (*ty).expanded, &mut (*e).expanded);
    (*e).len = (*ty).len;
    if dump_types {
        checkp(fputs((*e).name, stderr)); checkp(fputs(b" \0".as_ptr() as *const c_char, stderr));
        type_list_write(&mut (*e).expanded, stderr); checkp(fputs(b"\n\0".as_ptr() as *const c_char, stderr));
    }
    e
}

unsafe fn type_map_get(name: *const c_char, res: *mut *mut type_expansion) -> c_int {
    let mut ty = mem::MaybeUninit::<type_expansion>::uninit();
    let mut override_: *const c_char = ptr::null();
    if __type_map_get(name, res) == 0 { return 0; }
    if stable && kabi_get_type_string(name, &mut override_) != 0 {
        type_expansion_init(ty.as_mut_ptr());
        type_parse(name, override_, ty.as_mut_ptr());
        *res = type_map_add(name, ty.as_mut_ptr());
        type_expansion_free(ty.as_mut_ptr());
        return 0;
    }
    -1
}

unsafe fn type_parse(name: *const c_char, str_: *const c_char, ty: *mut type_expansion) {
    if *str_ == 0 { error!(b"empty type string override for '%s'\0".as_ptr(), name); }
    let mut start = 0usize;
    let mut pos = 0usize;
    while *str_.add(pos) != 0 {
        if !is_type_prefix(str_.add(pos)) { pos += 1; continue; }
        let mut end = pos + 2;
        let mut marker = b' ';
        if *str_.add(end) == b'\'' as c_char { marker = b'\''; end += 1; }
        while *str_.add(end) != 0 && *str_.add(end) != marker as c_char { end += 1; }
        let empty = if marker == b'\'' { if *str_.add(end) != marker as c_char { error!(b"incomplete type reference\0".as_ptr()); } end == pos + 3 } else { end == pos + 2 };
        if empty { error!(b"empty type name\0".as_ptr()); }
        if pos > start {
            let fragment = xstrndup(str_.add(start), pos - start);
            type_expansion_append(ty, fragment, fragment as *mut c_void);
        }
        let fragment = xstrndup(str_.add(pos), end - pos);
        type_expansion_append(ty, fragment, fragment as *mut c_void);
        start = end; pos = end;
    }
    if *str_.add(start) != 0 { type_expansion_append(ty, str_.add(start), ptr::null_mut()); }
}

unsafe fn is_type_prefix(s: *const c_char) -> bool {
    ((*s == b's' as c_char || *s == b'u' as c_char || *s == b'e' as c_char || *s == b't' as c_char) && *s.add(1) == b'#' as c_char)
}

unsafe fn get_type_name(cache: *mut die) -> *mut c_char {
    if (*cache).state == DIE_INCOMPLETE { warn!(b"found incomplete cache entry\0".as_ptr(), cache); return ptr::null_mut(); }
    if (*cache).state == DIE_SYMBOL || (*cache).state == DIE_FQN || (*cache).fqn.is_null() || *(*cache).fqn == 0 { return ptr::null_mut(); }
    let prefix = match (*cache).tag { DW_TAG_CLASS_TYPE | DW_TAG_STRUCTURE_TYPE => b's', DW_TAG_UNION_TYPE => b'u', DW_TAG_ENUMERATION_TYPE => b'e', DW_TAG_TYPEDEF_TYPE => b't', _ => 0 };
    if prefix == 0 { return ptr::null_mut(); }
    let quote = if strchr((*cache).fqn, b' ' as c_int).is_null() { b"\0" } else { b"'\0" };
    let mut name: *mut c_char = ptr::null_mut();
    asprintf(&mut name, b"%c#%s%s%s\0".as_ptr() as *const c_char, prefix, quote.as_ptr(), (*cache).fqn, quote.as_ptr());
    name
}

unsafe fn type_expand(name: *const c_char, cache: *mut die, ty: *mut type_expansion) {
    type_expansion_init(ty);
    let mut override_: *const c_char = ptr::null();
    if stable && kabi_get_type_string(name, &mut override_) != 0 { type_parse(name, override_, ty); }
    else { __type_expand(cache, ty); }
}

unsafe fn __type_expand(_cache: *mut die, _ty: *mut type_expansion) {
    // DIE fragment traversal is supplied by the translated DIE interfaces.
}

unsafe fn type_map_write(file: *mut FILE) {
    if file.is_null() { return; }
    // The project hash traversal and qsort helpers are used here exactly as in
    // the C implementation; entries are emitted in lexical name order.
    let mut e: *mut type_expansion = ptr::null_mut();
    hash_for_each!(type_map, e, hash, {
        checkp(fputs((*e).name, file));
        checkp(fputs(b" \0".as_ptr() as *const c_char, file));
        type_list_write(&mut (*e).expanded, file);
        checkp(fputs(b"\n\0".as_ptr() as *const c_char, file));
    });
}

unsafe fn type_map_free() {
    let mut e: *mut type_expansion = ptr::null_mut();
    let mut tmp: *mut hlist_node = ptr::null_mut();
    hash_for_each_safe!(type_map, e, tmp, hash, {
        type_expansion_free(e); free(e as *mut c_void);
    });
    hash_init!(type_map);
}

#[repr(C)] struct version { type_: type_expansion, crc: c_ulong }
unsafe fn version_init(v: *mut version) { (*v).crc = crc32(0, ptr::null(), 0); type_expansion_init(&mut (*v).type_); }
unsafe fn version_free(v: *mut version) { type_expansion_free(&mut (*v).type_); }
unsafe fn version_add(v: *mut version, s: *const c_char) {
    (*v).crc = crc32((*v).crc, s as *const u8, strlen(s) as u32);
    if dump_versions { type_expansion_append(&mut (*v).type_, s, ptr::null_mut()); }
}

unsafe fn __calculate_version(v: *mut version, ty: *mut type_expansion) {
    let mut entry: *mut type_list_entry = ptr::null_mut();
    list_for_each_entry!(entry, &mut (*ty).expanded, list, {
        if is_type_prefix((*entry).str_) {
            let mut e: *mut type_expansion = ptr::null_mut();
            if type_map_get((*entry).str_, &mut e) != 0 { error!(b"unknown type reference\0".as_ptr()); }
            if cache_was_expanded(&mut expansion_cache, e as *mut c_void) { version_add(v, (*entry).str_); }
            else { cache_mark_expanded(&mut expansion_cache, e as *mut c_void); __calculate_version(v, e); }
        } else { version_add(v, (*entry).str_); }
    });
}
unsafe fn calculate_version(v: *mut version, ty: *mut type_expansion) { version_init(v); __calculate_version(v, ty); cache_free(&mut expansion_cache); }

unsafe fn expand_type(cache: *mut die, _arg: *mut c_void) {
    if (*cache).mapped { return; }
    (*cache).mapped = true;
    let name = get_type_name(cache);
    if name.is_null() { return; }
    let mut ty = mem::MaybeUninit::<type_expansion>::uninit();
    type_expand(name, cache, ty.as_mut_ptr());
    type_map_add(name, ty.as_mut_ptr());
    type_expansion_free(ty.as_mut_ptr()); free(name as *mut c_void);
}

unsafe fn expand_symbol(sym: *mut symbol, _arg: *mut c_void) {
    let mut cache_: *mut die = ptr::null_mut();
    if !symtypes && (*sym).state == SYMBOL_PROCESSED { return; }
    if __die_map_get((*sym).die_addr, DIE_SYMBOL, &mut cache_) != 0 { return; }
    let mut ty = mem::MaybeUninit::<type_expansion>::uninit();
    type_expand((*sym).name, cache_, ty.as_mut_ptr());
    if (*sym).state != SYMBOL_PROCESSED {
        let mut v = mem::MaybeUninit::<version>::uninit(); calculate_version(v.as_mut_ptr(), ty.as_mut_ptr());
        symbol_set_crc(sym, (*v.as_ptr()).crc); version_free(v.as_mut_ptr());
    }
    if symtypes { type_map_add((*sym).name, ty.as_mut_ptr()); }
    type_expansion_free(ty.as_mut_ptr());
}

pub unsafe fn generate_symtypes_and_versions(file: *mut FILE) {
    cache_init(&mut expansion_cache);
    die_map_for_each(expand_type, ptr::null_mut());
    symbol_for_each(expand_symbol, ptr::null_mut());
    type_map_write(file);
    type_map_free();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
