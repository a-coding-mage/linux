// SPDX-License-Identifier: GPL-2.0
/*
 * trace_events_inject - trace event injection
 *
 * Copyright (C) 2019 Cong Wang <cwang@twitter.com>
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// Definitions and functions supplied by the kernel tracing dependencies.
#[repr(C)] pub struct trace_event_buffer { _private: [u8; 0] }
#[repr(C)] pub struct trace_event_file { pub event_call: *mut trace_event_call, _private: [u8; 0] }
#[repr(C)] pub struct trace_event_call { pub event: trace_event, _private: [u8; 0] }
#[repr(C)] pub struct trace_event { pub type_: u16 }
#[repr(C)] pub struct ftrace_event_field {
    pub link: list_head, pub size: c_int, pub offset: c_int,
    pub is_signed: bool, pub filter_type: c_int,
}
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct file { _private: [u8; 0] }
pub type ssize_t = isize;
pub type loff_t = i64;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const ENODEV: c_int = 19;
const EPERM: c_int = 1;
const PAGE_SIZE: usize = 4096;
const MAX_FILTER_STR_VAL: usize = 256;
const FILTER_STATIC_STRING: c_int = 0;
const FILTER_DYN_STRING: c_int = 1;
const FILTER_RDYN_STRING: c_int = 2;
const INJECT_STRING: &[u8] = b"STATIC STRING CAN NOT BE INJECTED\0";

extern "C" {
    fn rcu_read_lock_sched();
    fn rcu_read_unlock_sched();
    fn trace_event_buffer_reserve(buffer: *mut trace_event_buffer, file: *mut trace_event_file, len: c_int) -> *mut c_void;
    fn trace_event_buffer_commit(buffer: *mut trace_event_buffer);
    fn trace_find_event_field(call: *mut trace_event_call, name: *const c_char) -> *mut ftrace_event_field;
    fn is_string_field(field: *mut ftrace_event_field) -> bool;
    fn is_function_field(field: *mut ftrace_event_field) -> bool;
    fn trace_get_fields(call: *mut trace_event_call) -> *mut list_head;
    fn tracing_gen_ctx() -> *mut c_void;
    fn tracing_generic_entry_update(entry: *mut c_void, typ: u16, ctx: *mut c_void);
    fn event_file_file(filp: *mut file) -> *mut trace_event_file;
    fn tracing_open_file_tr(filp: *mut file) -> c_int;
    fn tracing_release_file_tr(filp: *mut file) -> c_int;
    static mut event_mutex: c_void;
}

unsafe fn trace_inject_entry(file: *mut trace_event_file, rec: *mut c_void, len: c_int) -> c_int {
    let mut fbuffer = core::mem::MaybeUninit::<trace_event_buffer>::uninit();
    let mut written = 0;
    rcu_read_lock_sched();
    let entry = trace_event_buffer_reserve(fbuffer.as_mut_ptr(), file, len);
    if !entry.is_null() {
        core::ptr::copy_nonoverlapping(rec as *const u8, entry as *mut u8, len as usize);
        written = len;
        trace_event_buffer_commit(fbuffer.as_mut_ptr());
    }
    rcu_read_unlock_sched();
    written
}

unsafe fn parse_field(str_: *mut c_char, call: *mut trace_event_call, pf: *mut *mut ftrace_event_field, pv: *mut u64) -> c_int {
    let mut i = 0isize;
    if *str_.offset(i) == 0 { return 0; }
    while (*str_.offset(i) as u8).is_ascii_whitespace() { i += 1; }
    let s = i;
    while { let c = *str_.offset(i) as u8; c.is_ascii_alphanumeric() || c == b'_' } { i += 1; }
    let len = i - s;
    if len == 0 { return -EINVAL; }
    let mut name = Vec::with_capacity(len as usize + 1);
    for n in 0..len { name.push(*str_.offset(s + n) as u8); }
    name.push(0);
    let field = trace_find_event_field(call, name.as_ptr() as *const c_char);
    if field.is_null() { return -ENOENT; }
    *pf = field;
    while (*str_.offset(i) as u8).is_ascii_whitespace() { i += 1; }
    if *str_.offset(i) != b'=' as c_char { return -EINVAL; }
    i += 1;
    while (*str_.offset(i) as u8).is_ascii_whitespace() { i += 1; }
    let s = i;
    let c = *str_.offset(i) as u8;
    if c.is_ascii_digit() || c == b'-' {
        if is_string_field(field) { return -EINVAL; }
        if c == b'-' { i += 1; }
        while (*str_.offset(i) as u8).is_ascii_alphanumeric() { i += 1; }
        let end = *str_.offset(i) as u8;
        if end != 0 && !end.is_ascii_whitespace() { return -EINVAL; }
        *str_.offset(i) = 0;
        let text = core::slice::from_raw_parts(str_.offset(s) as *const u8, (i-s) as usize);
        let parsed = core::str::from_utf8(text).ok().and_then(|v| if (*field).is_signed { v.parse::<i64>().ok().map(|x| x as u64) } else { u64::from_str_radix(v.trim_start_matches("0x"), if v.starts_with("0x") {16} else {10}).ok() });
        *str_.offset(i) = end as c_char;
        match parsed { Some(v) => { *pv = v; i as c_int }, None => -EINVAL }
    } else if c == b'\'' || c == b'"' {
        if !is_string_field(field) { return -EINVAL; }
        let q = c; i += 1;
        while *str_.offset(i) != 0 { if *str_.offset(i) as u8 == b'\\' && *str_.offset(i+1) != 0 { i += 1; } else if *str_.offset(i) as u8 == q { break; } i += 1; }
        if *str_.offset(i) == 0 { return -EINVAL; }
        let start = s + 1; let len = i - start;
        if len as usize >= MAX_FILTER_STR_VAL { return -EINVAL; }
        *pv = str_.offset(start) as usize as u64;
        *str_.offset(i) = 0;
        (i + 1) as c_int
    } else { -EINVAL }
}

unsafe fn trace_get_entry_size(call: *mut trace_event_call) -> c_int {
    // list_for_each_entry(field, head, link)
    let head = trace_get_fields(call); let mut field = (*head).next; let mut size = 0;
    while field != head { let f = field as *mut ftrace_event_field; if (*f).size + (*f).offset > size { size = (*f).size + (*f).offset; } field = (*field).next; }
    size
}

unsafe fn trace_alloc_entry(call: *mut trace_event_call, size: *mut c_int) -> *mut c_void {
    let entry_size = trace_get_entry_size(call); let entry = libc::calloc((entry_size + 1) as usize, 1); if entry.is_null() { return core::ptr::null_mut(); }
    let head = trace_get_fields(call); let mut link = (*head).next;
    while link != head { let field = link as *mut ftrace_event_field; if is_string_field(field) && (*field).filter_type != FILTER_STATIC_STRING { let p = (entry as *mut u8).offset((*field).offset as isize); if (*field).filter_type == FILTER_DYN_STRING || (*field).filter_type == FILTER_RDYN_STRING { *(p as *mut u32) = (entry_size & 0xffff) as u32; } else { *(p as *mut *const c_char) = b"\0".as_ptr() as *const c_char; } } link = (*link).next; }
    *size = entry_size + 1; entry
}

unsafe fn parse_entry(str_: *mut c_char, call: *mut trace_event_call, pentry: *mut *mut c_void) -> c_int {
    let mut entry_size = 0; let mut entry = trace_alloc_entry(call, &mut entry_size); *pentry = entry; if entry.is_null() { return -ENOMEM; }
    tracing_generic_entry_update(entry, (*call).event.type_, tracing_gen_ctx());
    let mut val = 0u64; let mut field = core::ptr::null_mut(); let mut len;
    while { len = parse_field(str_, call, &mut field, &mut val); len > 0 } {
        if is_function_field(field) { return -EINVAL; }
        let dst = (entry as *mut u8).offset((*field).offset as isize);
        if is_string_field(field) {
            let addr = val as *const c_char;
            if (*field).filter_type == FILTER_STATIC_STRING { core::ptr::copy_nonoverlapping(addr as *const u8, dst, (*field).size as usize); }
            else if (*field).filter_type == FILTER_DYN_STRING || (*field).filter_type == FILTER_RDYN_STRING {
                let mut n = 0usize; while *(addr.add(n) as *const u8) != 0 { n += 1; } let str_len = n + 1; let old = entry;
                entry = libc::realloc(entry, (entry_size as usize + str_len) as usize); *pentry = entry; if entry.is_null() { libc::free(old); return -ENOMEM; }
                core::ptr::copy_nonoverlapping(addr as *const u8, (entry as *mut u8).add(entry_size as usize), str_len); entry_size += str_len as c_int;
                let mut str_loc = (entry_size - str_len as c_int) & 0xffff; if (*field).filter_type == FILTER_RDYN_STRING { str_loc -= (*field).offset + (*field).size; }
                *( (entry as *mut u8).offset((*field).offset as isize) as *mut u32) = ((str_len as u32) << 16) | str_loc as u32;
            } else { *(dst as *mut *const c_char) = INJECT_STRING.as_ptr() as *const c_char; }
        } else { match (*field).size { 1 => *(dst as *mut u8) = val as u8, 2 => *(dst as *mut u16) = val as u16, 4 => *(dst as *mut u32) = val as u32, 8 => *(dst as *mut u64) = val, _ => return -EINVAL } }
        str_ = str_.offset(len as isize);
    }
    if len < 0 { return len; } entry_size
}

// The file-operation wrapper and field-population logic retain the kernel ABI.
pub static mut event_inject_fops: *const c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
