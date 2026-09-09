// SPDX-License-Identifier: GPL-2.0

// Dependency intent: declarations supplied by <linux/trace_seq.h> and "trace.h"
// are referenced here but are not implemented in this translation unit.

use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct trace_seq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_iterator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

pub enum print_line_t {}

extern "C" {
    pub fn trace_print_bputs_msg_only(iter: *mut trace_iterator) -> print_line_t;
    pub fn trace_print_bprintk_msg_only(iter: *mut trace_iterator) -> print_line_t;
    pub fn trace_print_printk_msg_only(iter: *mut trace_iterator) -> print_line_t;

    pub fn seq_print_ip_sym(s: *mut trace_seq, ip: c_ulong, sym_flags: c_ulong) -> c_int;

    pub fn trace_seq_print_sym(s: *mut trace_seq, address: c_ulong, offset: bool);
    pub fn trace_print_context(iter: *mut trace_iterator) -> c_int;
    pub fn trace_print_lat_context(iter: *mut trace_iterator) -> c_int;
    pub fn print_event_fields(
        iter: *mut trace_iterator,
        event: *mut trace_event,
    ) -> print_line_t;

    pub fn trace_event_read_lock();
    pub fn trace_event_read_unlock();
    pub fn ftrace_find_event(type_: c_int) -> *mut trace_event;

    pub fn trace_nop_print(
        iter: *mut trace_iterator,
        flags: c_int,
        event: *mut trace_event,
    ) -> print_line_t;
    pub fn trace_print_lat_fmt(s: *mut trace_seq, entry: *mut trace_entry) -> c_int;

    // Used by module unregistering.
    pub fn __unregister_trace_event(event: *mut trace_event) -> c_int;
    pub static mut trace_event_sem: rw_semaphore;

    // Supplied by trace.h; retained here as external symbols for the inline
    // translations below.
    pub fn trace_seq_putmem(s: *mut trace_seq, mem: *const u8, len: usize);
    pub fn trace_seq_putmem_hex(s: *mut trace_seq, mem: *const u8, len: usize);
    pub fn trace_seq_puts(s: *mut trace_seq, str_: *const u8);
    pub fn TRACE_ITER(flag: c_ulong) -> c_ulong;
    pub static SYM_OFFSET: c_ulong;
}

pub unsafe fn seq_print_ip_sym_offset(
    s: *mut trace_seq,
    ip: c_ulong,
    sym_flags: c_ulong,
) -> c_int {
    seq_print_ip_sym(s, ip, sym_flags | TRACE_ITER(SYM_OFFSET))
}

pub unsafe fn seq_print_ip_sym_no_offset(
    s: *mut trace_seq,
    ip: c_ulong,
    sym_flags: c_ulong,
) -> c_int {
    seq_print_ip_sym(s, ip, sym_flags & !TRACE_ITER(SYM_OFFSET))
}

pub unsafe fn seq_put_field<T>(s: *mut trace_seq, x: *const T) {
    trace_seq_putmem(s, x as *const u8, core::mem::size_of::<T>());
}

pub unsafe fn seq_put_hex_field<T>(s: *mut trace_seq, x: *const T) {
    trace_seq_putmem_hex(s, x as *const u8, core::mem::size_of::<T>());
}

// CONFIG_FUNCTION_TRACE_ARGS controls whether arguments are printed.
#[cfg(CONFIG_FUNCTION_TRACE_ARGS)]
extern "C" {
    pub fn print_function_args(s: *mut trace_seq, args: *mut c_ulong, func: c_ulong);
}

#[cfg(not(CONFIG_FUNCTION_TRACE_ARGS))]
pub unsafe fn print_function_args(_s: *mut trace_seq, _args: *mut c_ulong, _func: c_ulong) {
    trace_seq_puts(_s, b"()\0".as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
