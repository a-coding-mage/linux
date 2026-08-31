// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/trace-event.c.
// C includes removed; external symbols are declared below as future dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use std::ffi::{c_char, c_int, c_long, c_void};
use std::ptr;

const PATH_MAX: usize = 4096;
const TEP_NSEC_OUTPUT: c_int = 1;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_plugin_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

pub type tep_func_resolver_t = c_void;

#[repr(C)]
pub struct trace_event {
    pub pevent: *mut tep_handle,
    pub plugin_list: *mut tep_plugin_list,
}

unsafe extern "C" {
    fn tep_alloc() -> *mut tep_handle;
    fn tep_load_plugins(pevent: *mut tep_handle) -> *mut tep_plugin_list;
    fn tep_is_bigendian() -> c_int;
    fn tep_set_flag(pevent: *mut tep_handle, flag: c_int);
    fn tep_set_file_bigendian(pevent: *mut tep_handle, endian: c_int);
    fn tep_set_local_bigendian(pevent: *mut tep_handle, endian: c_int);
    fn tep_set_function_resolver(
        pevent: *mut tep_handle,
        func: *mut tep_func_resolver_t,
        machine: *mut machine,
    ) -> c_int;
    fn tep_unload_plugins(plugin_list: *mut tep_plugin_list, pevent: *mut tep_handle);
    fn tep_free(pevent: *mut tep_handle);
    fn get_events_file(sys: *const c_char) -> *mut c_char;
    fn put_events_file(file: *mut c_char);
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn filename__read_str(path: *const c_char, data: *mut *mut c_char, size: *mut usize) -> c_int;
    fn tep_parse_format(
        pevent: *mut tep_handle,
        event: *mut *mut tep_event,
        data: *mut c_char,
        size: usize,
        sys: *const c_char,
    ) -> c_int;
    fn tep_find_event(pevent: *mut tep_handle, id: c_int) -> *mut tep_event;
    fn free(ptr: *mut c_void);
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno_value() -> c_int {
    *__errno_location()
}

/*
 * <linux/err.h> ERR_PTR interface.
 */
fn ERR_PTR<T>(error: c_long) -> *mut T {
    error as isize as *mut T
}

/*
 * global trace_event object used by trace_event__tp_format
 *
 * TODO There's no cleanup call for this. Add some sort of
 * __exit function support and call trace_event__cleanup
 * there.
 */
static mut tevent: trace_event = trace_event {
    pevent: ptr::null_mut(),
    plugin_list: ptr::null_mut(),
};
static mut tevent_initialized: bool = false;

#[no_mangle]
pub unsafe extern "C" fn trace_event__init(t: *mut trace_event) -> c_int {
    let pevent: *mut tep_handle;

    trace_event__cleanup(t);

    pevent = tep_alloc();

    if !pevent.is_null() {
        (*t).plugin_list = tep_load_plugins(pevent);
        (*t).pevent = pevent;
    }

    if !pevent.is_null() {
        0
    } else {
        -1
    }
}

unsafe fn trace_event__init2() -> c_int {
    let be: c_int = tep_is_bigendian();
    let pevent: *mut tep_handle;

    if trace_event__init(&raw mut tevent) != 0 {
        return -1;
    }

    pevent = tevent.pevent;
    tep_set_flag(pevent, TEP_NSEC_OUTPUT);
    tep_set_file_bigendian(pevent, be);
    tep_set_local_bigendian(pevent, be);
    tevent_initialized = true;
    0
}

#[no_mangle]
pub unsafe extern "C" fn trace_event__register_resolver(
    machine: *mut machine,
    func: *mut tep_func_resolver_t,
) -> c_int {
    if !tevent_initialized && trace_event__init2() != 0 {
        return -1;
    }

    tep_set_function_resolver(tevent.pevent, func, machine)
}

#[no_mangle]
pub unsafe extern "C" fn trace_event__cleanup(t: *mut trace_event) {
    if (*t).pevent.is_null() {
        return;
    }

    tep_unload_plugins((*t).plugin_list, (*t).pevent);
    tep_free((*t).pevent);
    (*t).pevent = ptr::null_mut();
    (*t).plugin_list = ptr::null_mut();
}

/*
 * Returns pointer with encoded error via <linux/err.h> interface.
 */
unsafe fn tp_format(sys: *const c_char, name: *const c_char) -> *mut tep_event {
    let tp_dir: *mut c_char = get_events_file(sys);
    let pevent: *mut tep_handle = tevent.pevent;
    let mut event: *mut tep_event = ptr::null_mut();
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut size: usize = 0;
    let mut data: *mut c_char = ptr::null_mut();
    let err: c_int;

    if tp_dir.is_null() {
        return ERR_PTR(-(errno_value() as c_long));
    }

    scnprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        c"%s/%s/format".as_ptr(),
        tp_dir,
        name,
    );
    put_events_file(tp_dir);

    err = filename__read_str(path.as_ptr(), &mut data, &mut size);
    if err != 0 {
        return ERR_PTR(err as c_long);
    }

    tep_parse_format(pevent, &mut event, data, size, sys);

    free(data as *mut c_void);
    event
}

/*
 * Returns pointer with encoded error via <linux/err.h> interface.
 */
#[no_mangle]
pub unsafe extern "C" fn trace_event__tp_format(
    sys: *const c_char,
    name: *const c_char,
) -> *mut tep_event {
    if !tevent_initialized && trace_event__init2() != 0 {
        return ERR_PTR(-(ENOMEM as c_long));
    }

    tp_format(sys, name)
}

#[no_mangle]
pub unsafe extern "C" fn trace_event__tp_format_id(id: c_int) -> *mut tep_event {
    if !tevent_initialized && trace_event__init2() != 0 {
        return ERR_PTR(-(ENOMEM as c_long));
    }

    tep_find_event(tevent.pevent, id)
}
