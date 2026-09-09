// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of trace/events/landlock.h.
// Linux tracepoint and kernel types are supplied by the surrounding crate.

use core::ffi::{c_char, c_void};

pub type AccessMaskT = u64;

#[repr(C)]
pub struct TraceSeq {
    pub seq: *mut c_void,
    pub full: i32,
}

#[repr(C)]
pub struct TracePrintFlags {
    pub mask: u64,
    pub name: *const c_char,
}

#[repr(C)]
pub struct LandlockLayer {
    pub level: usize,
    pub access: AccessMaskT,
}

#[repr(C)]
pub struct LandlockRule {
    pub num_layers: usize,
    pub layers: *const LandlockLayer,
}

unsafe extern "C" {
    fn seq_buf_get_buf(seq: *mut c_void, buf: *mut *mut c_char) -> usize;
    fn trace_seq_buffer_ptr(p: *mut TraceSeq) -> *const c_char;
    fn string_escape_mem(src: *const c_char, len: usize, dst: *mut c_char,
                         size: usize, flags: u32, only: *const c_char) -> i32;
    fn seq_buf_set_overflow(seq: *mut c_void);
    fn seq_buf_commit(seq: *mut c_void, len: usize);
    fn trace_seq_putc(p: *mut TraceSeq, c: i32);
    fn trace_seq_puts(p: *mut TraceSeq, s: *const c_char);
    fn warn_on_once(condition: bool) -> bool;
}

pub const ESCAPE_SPACE: u32 = 1 << 0;
pub const ESCAPE_SPECIAL: u32 = 1 << 1;
pub const ESCAPE_NAP: u32 = 1 << 2;
pub const ESCAPE_APPEND: u32 = 1 << 3;
pub const ESCAPE_OCTAL: u32 = 1 << 4;

pub unsafe fn trace_print_untrusted_str(
    p: *mut TraceSeq,
    src: *const c_char,
    len: usize,
) -> *const c_char {
    let mut buf: *mut c_char = core::ptr::null_mut();
    let buf_size = unsafe { seq_buf_get_buf((*p).seq, &mut buf) };
    let ret = unsafe { trace_seq_buffer_ptr(p) };
    if src.is_null() || buf_size == 0 {
        return core::ptr::null();
    }
    let escaped_size = unsafe {
        string_escape_mem(src, len, buf, buf_size,
            ESCAPE_SPACE | ESCAPE_SPECIAL | ESCAPE_NAP | ESCAPE_APPEND | ESCAPE_OCTAL,
            c" ='\"\\".as_ptr())
    };
    if escaped_size < 0 || escaped_size as usize >= buf_size {
        unsafe {
            seq_buf_set_overflow((*p).seq);
            (*p).full = 1;
        }
        return core::ptr::null();
    }
    unsafe {
        seq_buf_commit((*p).seq, escaped_size as usize);
        trace_seq_putc(p, 0);
    }
    ret
}

pub unsafe fn landlock_fill_layers(
    layers: *mut AccessMaskT,
    num_layers: usize,
    rule: *const LandlockRule,
    access_request: AccessMaskT,
) {
    let mut i = 0usize;
    for level in 1..=num_layers {
        let mut grants = 0;
        unsafe {
            if i < (*rule).num_layers && level == (*(*rule).layers.add(i)).level {
                grants = (*(*rule).layers.add(i)).access & access_request;
                i += 1;
            }
            *layers.add(level - 1) = grants;
        }
    }
    let _ = unsafe { warn_on_once(i < (*rule).num_layers) };
}

pub unsafe fn landlock_print_layers(
    p: *mut TraceSeq,
    layers: *const AccessMaskT,
    num_layers: usize,
    names: *const TracePrintFlags,
    names_size: usize,
) -> *const c_char {
    let ret = unsafe { trace_seq_buffer_ptr(p) };
    unsafe { trace_seq_putc(p, b'{' as i32); }
    for i in 0..num_layers {
        if i != 0 { unsafe { trace_seq_putc(p, b',' as i32); } }
        let mut mask = unsafe { *layers.add(i) };
        let mut first = true;
        for j in 0..names_size {
            if mask == 0 { break; }
            let entry = unsafe { &*names.add(j) };
            if mask & entry.mask != entry.mask { continue; }
            if !first { unsafe { trace_seq_putc(p, b'|' as i32); } }
            unsafe { trace_seq_puts(p, entry.name); }
            mask &= !entry.mask;
            first = false;
        }
    }
    unsafe { trace_seq_putc(p, b'}' as i32); trace_seq_putc(p, 0); }
    ret
}

// Tracepoint declarations retained as named interfaces. Their TP_PROTO,
// TP_STRUCT__entry, TP_fast_assign, and TP_printk bodies are kernel tracepoint
// DSL and are consumed by the surrounding tracepoint implementation.
pub const LANDLOCK_TRACE_EVENTS: &[&str] = &[
    "landlock_create_ruleset", "landlock_free_ruleset", "landlock_add_rule_fs",
    "landlock_add_rule_net", "landlock_create_domain", "landlock_enforce_domain",
    "landlock_free_domain", "landlock_check_rule_fs", "landlock_check_rule_net",
    "landlock_deny_access_fs", "landlock_deny_access_net", "landlock_deny_ptrace",
    "landlock_deny_scope_signal", "landlock_deny_scope_abstract_unix_socket",
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
