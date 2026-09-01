/*
 * trace-event-perl.  Feed perf script events to an embedded Perl interpreter.
 *
 * Copyright (C) 2009 Tom Zanussi <tzanussi@gmail.com>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_ulonglong, c_void};

type u64 = u64;
type size_t = usize;
type INTERP = *mut PerlInterpreter;

const TRACE_EVENT_TYPE_MAX: usize = (1usize << (core::mem::size_of::<c_ushort>() * 8)) - 1;
const PERF_TYPE_TRACEPOINT: u32 = 2;
const NSEC_PER_SEC: c_ulonglong = 1_000_000_000;
const TEP_EVENT_SORT_ID: c_int = 1;
const TEP_FIELD_IS_STRING: c_int = 1 << 0;
const TEP_FIELD_IS_DYNAMIC: c_int = 1 << 1;
const TEP_FIELD_IS_SIGNED: c_int = 1 << 2;
const TEP_FIELD_IS_FLAG: c_int = 1 << 3;
const TEP_FIELD_IS_SYMBOLIC: c_int = 1 << 4;
const ENOMEM: c_int = 12;
const G_SCALAR: c_int = 2;
const G_DISCARD: c_int = 4;
const G_NOARGS: c_int = 8;
const PATH_MAX: usize = 4096;

type c_ushort = u16;

#[repr(C)]
pub struct PerlInterpreter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CV {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SV {
    _private: [u8; 0],
}

#[repr(C)]
pub struct AV {
    _private: [u8; 0],
}

#[repr(C)]
pub struct HV {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scripting_context {
    pub session: *mut perf_session,
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct perf_sample {
    pub cpu: c_int,
    pub raw_data: *mut c_void,
    pub raw_size: size_t,
    pub time: c_ulonglong,
    pub evsel: *mut evsel,
    pub callchain: *mut c_void,
}

#[repr(C)]
pub struct addr_location {
    pub thread: *mut thread,
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct callchain_cursor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub start: c_ulonglong,
    pub end: c_ulonglong,
    pub name: *const c_char,
    pub namelen: size_t,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct callchain_cursor_node {
    pub ip: c_ulonglong,
    pub ms: map_symbol,
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf_t {
    pub use_callchain: bool,
    pub show_kernel_path: bool,
}

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_print_flag_sym {
    pub value: *const c_char,
    pub str_: *const c_char,
    pub next: *mut tep_print_flag_sym,
}

#[repr(C)]
pub struct tep_format_field {
    pub name: *const c_char,
    pub offset: c_int,
    pub size: c_int,
    pub flags: c_int,
    pub next: *mut tep_format_field,
}

#[repr(C)]
pub struct tep_format {
    pub fields: *mut tep_format_field,
}

#[repr(C)]
pub struct tep_print_fmt {
    pub args: *mut tep_print_arg,
}

#[repr(C)]
pub struct tep_event {
    pub system: *const c_char,
    pub name: *const c_char,
    pub id: c_int,
    pub format: tep_format,
    pub print_fmt: tep_print_fmt,
}

#[repr(C)]
pub struct tep_print_arg_atom {
    pub atom: *const c_char,
}

#[repr(C)]
pub struct tep_print_arg_field {
    pub name: *const c_char,
}

#[repr(C)]
pub struct tep_print_arg_flags {
    pub field: *mut tep_print_arg,
    pub delim: *const c_char,
    pub flags: *mut tep_print_flag_sym,
}

#[repr(C)]
pub struct tep_print_arg_symbol {
    pub field: *mut tep_print_arg,
    pub symbols: *mut tep_print_flag_sym,
}

#[repr(C)]
pub struct tep_print_arg_hex {
    pub field: *mut tep_print_arg,
    pub size: *mut tep_print_arg,
}

#[repr(C)]
pub struct tep_print_arg_int_array {
    pub field: *mut tep_print_arg,
    pub count: *mut tep_print_arg,
    pub el_size: *mut tep_print_arg,
}

#[repr(C)]
pub struct tep_print_arg_typecast {
    pub item: *mut tep_print_arg,
}

#[repr(C)]
pub struct tep_print_arg_op {
    pub op: *const c_char,
    pub left: *mut tep_print_arg,
    pub right: *mut tep_print_arg,
}

#[repr(C)]
pub union tep_print_arg_u {
    pub atom: core::mem::ManuallyDrop<tep_print_arg_atom>,
    pub field: core::mem::ManuallyDrop<tep_print_arg_field>,
    pub flags: core::mem::ManuallyDrop<tep_print_arg_flags>,
    pub symbol: core::mem::ManuallyDrop<tep_print_arg_symbol>,
    pub hex: core::mem::ManuallyDrop<tep_print_arg_hex>,
    pub int_array: core::mem::ManuallyDrop<tep_print_arg_int_array>,
    pub typecast: core::mem::ManuallyDrop<tep_print_arg_typecast>,
    pub op: core::mem::ManuallyDrop<tep_print_arg_op>,
}

#[repr(C)]
pub struct tep_print_arg {
    pub type_: c_int,
    pub next: *mut tep_print_arg,
    pub u: tep_print_arg_u,
}

const TEP_PRINT_NULL: c_int = 0;
const TEP_PRINT_ATOM: c_int = 1;
const TEP_PRINT_FIELD: c_int = 2;
const TEP_PRINT_FLAGS: c_int = 3;
const TEP_PRINT_SYMBOL: c_int = 4;
const TEP_PRINT_HEX: c_int = 5;
const TEP_PRINT_HEX_STR: c_int = 6;
const TEP_PRINT_INT_ARRAY: c_int = 7;
const TEP_PRINT_BSTRING: c_int = 8;
const TEP_PRINT_DYNAMIC_ARRAY: c_int = 9;
const TEP_PRINT_DYNAMIC_ARRAY_LEN: c_int = 10;
const TEP_PRINT_STRING: c_int = 11;
const TEP_PRINT_BITMASK: c_int = 12;
const TEP_PRINT_TYPE: c_int = 13;
const TEP_PRINT_OP: c_int = 14;
const TEP_PRINT_FUNC: c_int = 15;

#[repr(C)]
pub struct scripting_ops {
    pub name: *const c_char,
    pub dirname: *const c_char,
    pub start_script: Option<unsafe extern "C" fn(*const c_char, c_int, *mut *const c_char, *mut perf_session) -> c_int>,
    pub flush_script: Option<unsafe extern "C" fn() -> c_int>,
    pub stop_script: Option<unsafe extern "C" fn() -> c_int>,
    pub process_event: Option<unsafe extern "C" fn(*mut perf_event, *mut perf_sample, *mut addr_location, *mut addr_location)>,
    pub generate_script: Option<unsafe extern "C" fn(*mut tep_handle, *const c_char) -> c_int>,
}

unsafe extern "C" {
    fn boot_Perf__Trace__Context(cv: *mut CV);
    fn boot_DynaLoader(cv: *mut CV);
    fn newXS(name: *const c_char, subaddr: unsafe extern "C" fn(*mut CV), filename: *const c_char);
    fn eval_flag(str_: *const c_char) -> c_ulonglong;
    fn newSVpv(s: *const c_char, len: size_t) -> *mut SV;
    fn newSVpvn(s: *const c_char, len: size_t) -> *mut SV;
    fn newSVuv(u: c_ulonglong) -> *mut SV;
    fn newSViv(i: c_long) -> *mut SV;
    fn sv_2mortal(sv: *mut SV) -> *mut SV;
    fn get_cv(name: *const c_char, flags: c_int) -> *mut CV;
    fn call_pv(sub_name: *const c_char, flags: c_int) -> c_int;
    fn newAV() -> *mut AV;
    fn newHV() -> *mut HV;
    fn newRV_noinc(sv: *mut SV) -> *mut SV;
    fn hv_stores(hv: *mut HV, key: *const c_char, val: *mut SV) -> *mut SV;
    fn hv_undef(hv: *mut HV);
    fn av_push(av: *mut AV, val: *mut SV);
    fn perl_alloc() -> INTERP;
    fn perl_construct(perl: INTERP);
    fn perl_parse(perl: INTERP, xsinit: unsafe extern "C" fn(), argc: c_int, argv: *mut *mut c_char, env: *mut *mut c_char) -> c_int;
    fn perl_run(perl: INTERP) -> c_int;
    fn perl_destruct(perl: INTERP) -> c_int;
    fn perl_free(perl: INTERP);
    static mut ERRSV: *mut SV;
    fn SvTRUE(sv: *mut SV) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    static mut stderr: *mut FILE;
    static mut scripting_context: *mut scripting_context;
    static mut symbol_conf: symbol_conf_t;
    static mut scripting_max_stack: c_int;
    fn get_tls_callchain_cursor() -> *mut callchain_cursor;
    fn thread__resolve_callchain(thread: *mut thread, cursor: *mut callchain_cursor, sample: *mut perf_sample, parent: *mut c_void, root_al: *mut c_void, max_stack: c_int) -> c_int;
    fn callchain_cursor_commit(cursor: *mut callchain_cursor);
    fn callchain_cursor_current(cursor: *mut callchain_cursor) -> *mut callchain_cursor_node;
    fn callchain_cursor_advance(cursor: *mut callchain_cursor);
    fn symbol__binding(sym: *mut symbol) -> c_int;
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__name(dso: *mut dso) -> *const c_char;
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn evsel__tp_format(evsel: *mut evsel) -> *mut tep_event;
    fn raw_field_value(event: *mut tep_event, name: *const c_char, data: *mut c_void) -> c_int;
    fn __test_and_set_bit(nr: c_int, addr: *mut c_ulong) -> c_int;
    fn read_size(event: *mut tep_event, addr: *mut c_void, size: c_int) -> c_ulonglong;
    fn tep_field_is_relative(flags: c_int) -> bool;
    fn scripting_context__update(context: *mut scripting_context, event: *mut perf_event, sample: *mut perf_sample, al: *mut addr_location, addr_al: *mut addr_location);
    fn tep_get_events_count(pevent: *mut tep_handle) -> c_int;
    fn tep_list_events(pevent: *mut tep_handle, sort_type: c_int) -> *mut *mut tep_event;
    fn pr_err(format: *const c_char, ...);
    fn pr_debug(format: *const c_char, ...);
}

static mut my_perl: INTERP = core::ptr::null_mut();
static mut cur_field_name: *mut c_char = core::ptr::null_mut();
static mut zero_flag_atom: c_int = 0;

unsafe extern "C" fn xs_init() {
    let file = b"trace-event-perl.rs\0".as_ptr() as *const c_char;

    newXS(
        b"Perf::Trace::Context::bootstrap\0".as_ptr() as *const c_char,
        boot_Perf__Trace__Context,
        file,
    );
    newXS(
        b"DynaLoader::boot_DynaLoader\0".as_ptr() as *const c_char,
        boot_DynaLoader,
        file,
    );
}

unsafe fn define_symbolic_value(
    ev_name: *const c_char,
    field_name: *const c_char,
    field_value: *const c_char,
    field_str: *const c_char,
) {
    let value: c_ulonglong = eval_flag(field_value);

    let _ = sv_2mortal(newSVpv(ev_name, 0));
    let _ = sv_2mortal(newSVpv(field_name, 0));
    let _ = sv_2mortal(newSVuv(value));
    let _ = sv_2mortal(newSVpv(field_str, 0));

    if !get_cv(b"main::define_symbolic_value\0".as_ptr() as *const c_char, 0).is_null() {
        call_pv(b"main::define_symbolic_value\0".as_ptr() as *const c_char, G_SCALAR);
    }
}

unsafe fn define_symbolic_values(
    field: *mut tep_print_flag_sym,
    ev_name: *const c_char,
    field_name: *const c_char,
) {
    define_symbolic_value(ev_name, field_name, (*field).value, (*field).str_);
    if !(*field).next.is_null() {
        define_symbolic_values((*field).next, ev_name, field_name);
    }
}

unsafe fn define_symbolic_field(ev_name: *const c_char, field_name: *const c_char) {
    let _ = sv_2mortal(newSVpv(ev_name, 0));
    let _ = sv_2mortal(newSVpv(field_name, 0));

    if !get_cv(b"main::define_symbolic_field\0".as_ptr() as *const c_char, 0).is_null() {
        call_pv(b"main::define_symbolic_field\0".as_ptr() as *const c_char, G_SCALAR);
    }
}

unsafe fn define_flag_value(
    ev_name: *const c_char,
    field_name: *const c_char,
    field_value: *const c_char,
    field_str: *const c_char,
) {
    let value: c_ulonglong = eval_flag(field_value);

    let _ = sv_2mortal(newSVpv(ev_name, 0));
    let _ = sv_2mortal(newSVpv(field_name, 0));
    let _ = sv_2mortal(newSVuv(value));
    let _ = sv_2mortal(newSVpv(field_str, 0));

    if !get_cv(b"main::define_flag_value\0".as_ptr() as *const c_char, 0).is_null() {
        call_pv(b"main::define_flag_value\0".as_ptr() as *const c_char, G_SCALAR);
    }
}

unsafe fn define_flag_values(
    field: *mut tep_print_flag_sym,
    ev_name: *const c_char,
    field_name: *const c_char,
) {
    define_flag_value(ev_name, field_name, (*field).value, (*field).str_);
    if !(*field).next.is_null() {
        define_flag_values((*field).next, ev_name, field_name);
    }
}

unsafe fn define_flag_field(
    ev_name: *const c_char,
    field_name: *const c_char,
    delim: *const c_char,
) {
    let _ = sv_2mortal(newSVpv(ev_name, 0));
    let _ = sv_2mortal(newSVpv(field_name, 0));
    let _ = sv_2mortal(newSVpv(delim, 0));

    if !get_cv(b"main::define_flag_field\0".as_ptr() as *const c_char, 0).is_null() {
        call_pv(b"main::define_flag_field\0".as_ptr() as *const c_char, G_SCALAR);
    }
}

unsafe fn define_event_symbols(
    event: *mut tep_event,
    ev_name: *const c_char,
    args: *mut tep_print_arg,
) {
    if args.is_null() {
        return;
    }

    match (*args).type_ {
        TEP_PRINT_NULL => {}
        TEP_PRINT_ATOM => {
            define_flag_value(ev_name, cur_field_name, b"0\0".as_ptr() as *const c_char, (*args).u.atom.atom);
            zero_flag_atom = 0;
        }
        TEP_PRINT_FIELD => {
            free(cur_field_name as *mut c_void);
            cur_field_name = strdup((*args).u.field.name);
        }
        TEP_PRINT_FLAGS => {
            define_event_symbols(event, ev_name, (*args).u.flags.field);
            define_flag_field(ev_name, cur_field_name, (*args).u.flags.delim);
            define_flag_values((*args).u.flags.flags, ev_name, cur_field_name);
        }
        TEP_PRINT_SYMBOL => {
            define_event_symbols(event, ev_name, (*args).u.symbol.field);
            define_symbolic_field(ev_name, cur_field_name);
            define_symbolic_values((*args).u.symbol.symbols, ev_name, cur_field_name);
        }
        TEP_PRINT_HEX | TEP_PRINT_HEX_STR => {
            define_event_symbols(event, ev_name, (*args).u.hex.field);
            define_event_symbols(event, ev_name, (*args).u.hex.size);
        }
        TEP_PRINT_INT_ARRAY => {
            define_event_symbols(event, ev_name, (*args).u.int_array.field);
            define_event_symbols(event, ev_name, (*args).u.int_array.count);
            define_event_symbols(event, ev_name, (*args).u.int_array.el_size);
        }
        TEP_PRINT_BSTRING | TEP_PRINT_DYNAMIC_ARRAY | TEP_PRINT_DYNAMIC_ARRAY_LEN | TEP_PRINT_STRING | TEP_PRINT_BITMASK => {}
        TEP_PRINT_TYPE => {
            define_event_symbols(event, ev_name, (*args).u.typecast.item);
        }
        TEP_PRINT_OP => {
            if strcmp((*args).u.op.op, b":\0".as_ptr() as *const c_char) == 0 {
                zero_flag_atom = 1;
            }
            define_event_symbols(event, ev_name, (*args).u.op.left);
            define_event_symbols(event, ev_name, (*args).u.op.right);
        }
        TEP_PRINT_FUNC | _ => {
            pr_err(b"Unsupported print arg type\n\0".as_ptr() as *const c_char);
            /* we should warn... */
            return;
        }
    }

    if !(*args).next.is_null() {
        define_event_symbols(event, ev_name, (*args).next);
    }
}

unsafe fn perl_process_callchain(sample: *mut perf_sample, al: *mut addr_location) -> *mut SV {
    let mut cursor: *mut callchain_cursor;
    let list: *mut AV;

    list = newAV();
    if list.is_null() {
        return newRV_noinc(list as *mut SV);
    }

    if !symbol_conf.use_callchain || (*sample).callchain.is_null() {
        return newRV_noinc(list as *mut SV);
    }

    cursor = get_tls_callchain_cursor();

    if thread__resolve_callchain(
        (*al).thread,
        cursor,
        sample,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        scripting_max_stack,
    ) != 0
    {
        pr_err(b"Failed to resolve callchain. Skipping\n\0".as_ptr() as *const c_char);
        return newRV_noinc(list as *mut SV);
    }
    callchain_cursor_commit(cursor);

    loop {
        let elem: *mut HV;
        let node: *mut callchain_cursor_node;
        node = callchain_cursor_current(cursor);
        if node.is_null() {
            break;
        }

        elem = newHV();
        if elem.is_null() {
            return newRV_noinc(list as *mut SV);
        }

        if hv_stores(elem, b"ip\0".as_ptr() as *const c_char, newSVuv((*node).ip)).is_null() {
            hv_undef(elem);
            return newRV_noinc(list as *mut SV);
        }

        if !(*node).ms.sym.is_null() {
            let sym = newHV();
            if sym.is_null() {
                hv_undef(elem);
                return newRV_noinc(list as *mut SV);
            }
            if hv_stores(sym, b"start\0".as_ptr() as *const c_char, newSVuv((*(*node).ms.sym).start)).is_null()
                || hv_stores(sym, b"end\0".as_ptr() as *const c_char, newSVuv((*(*node).ms.sym).end)).is_null()
                || hv_stores(sym, b"binding\0".as_ptr() as *const c_char, newSVuv(symbol__binding((*node).ms.sym) as c_ulonglong)).is_null()
                || hv_stores(sym, b"name\0".as_ptr() as *const c_char, newSVpvn((*(*node).ms.sym).name, (*(*node).ms.sym).namelen)).is_null()
                || hv_stores(elem, b"sym\0".as_ptr() as *const c_char, newRV_noinc(sym as *mut SV)).is_null()
            {
                hv_undef(sym);
                hv_undef(elem);
                return newRV_noinc(list as *mut SV);
            }
        }

        if !(*node).ms.map.is_null() {
            let map = (*node).ms.map;
            let dso = if !map.is_null() { map__dso(map) } else { core::ptr::null_mut() };
            let mut dsoname = b"[unknown]\0".as_ptr() as *const c_char;

            if !dso.is_null() {
                if symbol_conf.show_kernel_path && !dso__long_name(dso).is_null() {
                    dsoname = dso__long_name(dso);
                } else {
                    dsoname = dso__name(dso);
                }
            }
            if hv_stores(elem, b"dso\0".as_ptr() as *const c_char, newSVpv(dsoname, 0)).is_null() {
                hv_undef(elem);
                return newRV_noinc(list as *mut SV);
            }
        }

        callchain_cursor_advance(cursor);
        av_push(list, newRV_noinc(elem as *mut SV));
    }

    newRV_noinc(list as *mut SV)
}

unsafe fn perl_process_tracepoint(sample: *mut perf_sample, al: *mut addr_location) {
    let thread = (*al).thread;
    let mut field: *mut tep_format_field;
    static mut handler: [c_char; 256] = [0; 256];
    let mut val: c_ulonglong;
    let s: c_ulonglong;
    let ns: c_ulonglong;
    let pid: c_int;
    let cpu: c_int = (*sample).cpu;
    let data: *mut c_void = (*sample).raw_data;
    let nsecs: c_ulonglong = (*sample).time;
    let comm = thread__comm_str(thread);
    let mut events_defined: [c_ulong; (TRACE_EVENT_TYPE_MAX + c_ulong::BITS as usize - 1) / c_ulong::BITS as usize] =
        [0; (TRACE_EVENT_TYPE_MAX + c_ulong::BITS as usize - 1) / c_ulong::BITS as usize];
    let evsel = (*sample).evsel;
    let event: *mut tep_event;

    events_defined.fill(0);

    if (*evsel).core.attr.type_ != PERF_TYPE_TRACEPOINT {
        return;
    }

    event = evsel__tp_format(evsel);
    if event.is_null() {
        pr_debug(
            b"ug! no event found for type %llu\0".as_ptr() as *const c_char,
            (*evsel).core.attr.config,
        );
        return;
    }

    pid = raw_field_value(event, b"common_pid\0".as_ptr() as *const c_char, data);

    sprintf(
        handler.as_mut_ptr(),
        b"%s::%s\0".as_ptr() as *const c_char,
        (*event).system,
        (*event).name,
    );

    if __test_and_set_bit((*event).id, events_defined.as_mut_ptr()) == 0 {
        define_event_symbols(event, handler.as_ptr(), (*event).print_fmt.args);
    }

    s = nsecs / NSEC_PER_SEC;
    ns = nsecs - s * NSEC_PER_SEC;

    let _ = sv_2mortal(newSVpv(handler.as_ptr(), 0));
    let _ = sv_2mortal(newSViv(scripting_context as c_long));
    let _ = sv_2mortal(newSVuv(cpu as c_ulonglong));
    let _ = sv_2mortal(newSVuv(s));
    let _ = sv_2mortal(newSVuv(ns));
    let _ = sv_2mortal(newSViv(pid as c_long));
    let _ = sv_2mortal(newSVpv(comm, 0));
    let _ = sv_2mortal(perl_process_callchain(sample, al));

    /* common fields other than pid can be accessed via xsub fns */

    field = (*event).format.fields;
    while !field.is_null() {
        if (*field).flags & TEP_FIELD_IS_STRING != 0 {
            let mut offset: c_int;
            if (*field).flags & TEP_FIELD_IS_DYNAMIC != 0 {
                offset = *(data.add((*field).offset as usize) as *mut c_int);
                offset &= 0xffff;
                if tep_field_is_relative((*field).flags) {
                    offset += (*field).offset + (*field).size;
                }
            } else {
                offset = (*field).offset;
            }
            let _ = sv_2mortal(newSVpv(data.add(offset as usize) as *mut c_char, 0));
        } else {
            /* FIELD_IS_NUMERIC */
            val = read_size(event, data.add((*field).offset as usize), (*field).size);
            if (*field).flags & TEP_FIELD_IS_SIGNED != 0 {
                let _ = sv_2mortal(newSViv(val as c_long));
            } else {
                let _ = sv_2mortal(newSVuv(val));
            }
        }
        field = (*field).next;
    }

    if !get_cv(handler.as_ptr(), 0).is_null() {
        call_pv(handler.as_ptr(), G_SCALAR);
    } else if !get_cv(b"main::trace_unhandled\0".as_ptr() as *const c_char, 0).is_null() {
        let _ = sv_2mortal(newSVpv(handler.as_ptr(), 0));
        let _ = sv_2mortal(newSViv(scripting_context as c_long));
        let _ = sv_2mortal(newSVuv(cpu as c_ulonglong));
        let _ = sv_2mortal(newSVuv(nsecs));
        let _ = sv_2mortal(newSViv(pid as c_long));
        let _ = sv_2mortal(newSVpv(comm, 0));
        let _ = sv_2mortal(perl_process_callchain(sample, al));
        call_pv(b"main::trace_unhandled\0".as_ptr() as *const c_char, G_SCALAR);
    }
}

unsafe fn perl_process_event_generic(event: *mut perf_event, sample: *mut perf_sample) {
    if get_cv(b"process_event\0".as_ptr() as *const c_char, 0).is_null() {
        return;
    }

    let _ = sv_2mortal(newSVpvn(event as *const c_char, (*event).header.size as size_t));
    let _ = sv_2mortal(newSVpvn(
        &(*(*sample).evsel).core.attr as *const perf_event_attr as *const c_char,
        core::mem::size_of_val(&(*(*sample).evsel).core.attr),
    ));
    let _ = sv_2mortal(newSVpvn(sample as *const c_char, core::mem::size_of::<perf_sample>()));
    let _ = sv_2mortal(newSVpvn((*sample).raw_data as *const c_char, (*sample).raw_size));
    call_pv(b"process_event\0".as_ptr() as *const c_char, G_SCALAR);
}

unsafe extern "C" fn perl_process_event(
    event: *mut perf_event,
    sample: *mut perf_sample,
    al: *mut addr_location,
    addr_al: *mut addr_location,
) {
    scripting_context__update(scripting_context, event, sample, al, addr_al);
    perl_process_tracepoint(sample, al);
    perl_process_event_generic(event, sample);
}

unsafe fn run_start_sub() {
    if !get_cv(b"main::trace_begin\0".as_ptr() as *const c_char, 0).is_null() {
        call_pv(b"main::trace_begin\0".as_ptr() as *const c_char, G_DISCARD | G_NOARGS);
    }
}

/*
 * Start trace script
 */
unsafe extern "C" fn perl_start_script(
    script: *const c_char,
    argc: c_int,
    argv: *mut *const c_char,
    session: *mut perf_session,
) -> c_int {
    let command_line: *mut *const c_char;
    let mut i: c_int;
    let mut err: c_int = 0;

    (*scripting_context).session = session;

    command_line = malloc(((argc + 2) as usize) * core::mem::size_of::<*const c_char>()) as *mut *const c_char;
    if command_line.is_null() {
        return -ENOMEM;
    }

    *command_line.add(0) = b"\0".as_ptr() as *const c_char;
    *command_line.add(1) = script;
    i = 2;
    while i < argc + 2 {
        *command_line.add(i as usize) = *argv.add((i - 2) as usize);
        i += 1;
    }

    my_perl = perl_alloc();
    perl_construct(my_perl);

    if perl_parse(
        my_perl,
        xs_init,
        argc + 2,
        command_line as *mut *mut c_char,
        core::ptr::null_mut(),
    ) != 0
    {
        err = -1;
        perl_free(my_perl);
        free(command_line as *mut c_void);
        return err;
    }

    if perl_run(my_perl) != 0 {
        err = -1;
        perl_free(my_perl);
        free(command_line as *mut c_void);
        return err;
    }

    if SvTRUE(ERRSV) != 0 {
        err = -1;
        perl_free(my_perl);
        free(command_line as *mut c_void);
        return err;
    }

    run_start_sub();

    free(command_line as *mut c_void);
    0
}

unsafe extern "C" fn perl_flush_script() -> c_int {
    0
}

/*
 * Stop trace script
 */
unsafe extern "C" fn perl_stop_script() -> c_int {
    if !get_cv(b"main::trace_end\0".as_ptr() as *const c_char, 0).is_null() {
        call_pv(b"main::trace_end\0".as_ptr() as *const c_char, G_DISCARD | G_NOARGS);
    }

    perl_destruct(my_perl);
    perl_free(my_perl);

    0
}

unsafe extern "C" fn perl_generate_script(pevent: *mut tep_handle, outfile: *const c_char) -> c_int {
    let mut i: c_int;
    let mut not_first: c_int;
    let mut count: c_int;
    let nr_events: c_int;
    let all_events: *mut *mut tep_event;
    let mut event: *mut tep_event;
    let mut f: *mut tep_format_field;
    let mut fname: [c_char; PATH_MAX] = [0; PATH_MAX];
    let ofp: *mut FILE;

    sprintf(fname.as_mut_ptr(), b"%s.pl\0".as_ptr() as *const c_char, outfile);
    ofp = fopen(fname.as_ptr(), b"w\0".as_ptr() as *const c_char);
    if ofp.is_null() {
        fprintf(stderr, b"couldn't open %s\n\0".as_ptr() as *const c_char, fname.as_ptr());
        return -1;
    }

    fprintf(ofp, b"# perf script event handlers, generated by perf script -g perl\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"# Licensed under the terms of the GNU GPL License version 2\n\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"# The common_* event handler fields are the most useful fields common to\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"# all events.  They don't necessarily correspond to the 'common_*' fields\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"# in the format files.  Those fields not available as handler params can\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"# be retrieved using Perl functions of the form common_*($context).\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"# See Context.pm for the list of available functions.\n\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"use lib \"$ENV{'PERF_EXEC_PATH'}/scripts/perl/Perf-Trace-Util/lib\";\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"use lib \"./Perf-Trace-Util/lib\";\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"use Perf::Trace::Core;\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"use Perf::Trace::Context;\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"use Perf::Trace::Util;\n\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"sub trace_begin\n{\n\t# optional\n}\n\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"sub trace_end\n{\n\t# optional\n}\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"\nsub print_backtrace\n{\n\tmy $callchain = shift;\n\tfor my $node (@$callchain)\n\t{\n\t\tif(exists $node->{sym})\n\t\t{\n\t\t\tprintf( \"\\t[\\%x] \\%s\\n\", $node->{ip}, $node->{sym}{name});\n\t\t}\n\t\telse\n\t\t{\n\t\t\tprintf( \"\\t[\\%x]\\n\", $node{ip});\n\t\t}\n\t}\n}\n\n\0".as_ptr() as *const c_char);

    nr_events = tep_get_events_count(pevent);
    all_events = tep_list_events(pevent, TEP_EVENT_SORT_ID);

    i = 0;
    while !all_events.is_null() && i < nr_events {
        event = *all_events.add(i as usize);
        fprintf(ofp, b"sub %s::%s\n{\n\0".as_ptr() as *const c_char, (*event).system, (*event).name);
        fprintf(ofp, b"\tmy (\0".as_ptr() as *const c_char);
        fprintf(ofp, b"$event_name, \0".as_ptr() as *const c_char);
        fprintf(ofp, b"$context, \0".as_ptr() as *const c_char);
        fprintf(ofp, b"$common_cpu, \0".as_ptr() as *const c_char);
        fprintf(ofp, b"$common_secs, \0".as_ptr() as *const c_char);
        fprintf(ofp, b"$common_nsecs,\n\0".as_ptr() as *const c_char);
        fprintf(ofp, b"\t    $common_pid, \0".as_ptr() as *const c_char);
        fprintf(ofp, b"$common_comm, \0".as_ptr() as *const c_char);
        fprintf(ofp, b"$common_callchain,\n\t    \0".as_ptr() as *const c_char);

        not_first = 0;
        count = 0;
        f = (*event).format.fields;
        while !f.is_null() {
            if not_first != 0 {
                fprintf(ofp, b", \0".as_ptr() as *const c_char);
            }
            not_first += 1;
            count += 1;
            if count % 5 == 0 {
                fprintf(ofp, b"\n\t    \0".as_ptr() as *const c_char);
            }
            fprintf(ofp, b"$%s\0".as_ptr() as *const c_char, (*f).name);
            f = (*f).next;
        }
        fprintf(ofp, b") = @_;\n\n\0".as_ptr() as *const c_char);
        fprintf(ofp, b"\tprint_header($event_name, $common_cpu, $common_secs, $common_nsecs,\n\t             $common_pid, $common_comm, $common_callchain);\n\n\0".as_ptr() as *const c_char);
        fprintf(ofp, b"\tprintf(\"\0".as_ptr() as *const c_char);

        not_first = 0;
        count = 0;
        f = (*event).format.fields;
        while !f.is_null() {
            if not_first != 0 {
                fprintf(ofp, b", \0".as_ptr() as *const c_char);
            }
            not_first += 1;
            if count != 0 && count % 4 == 0 {
                fprintf(ofp, b"\".\n\t       \"\0".as_ptr() as *const c_char);
            }
            count += 1;
            fprintf(ofp, b"%s=\0".as_ptr() as *const c_char, (*f).name);
            if (*f).flags & TEP_FIELD_IS_STRING != 0
                || (*f).flags & TEP_FIELD_IS_FLAG != 0
                || (*f).flags & TEP_FIELD_IS_SYMBOLIC != 0
            {
                fprintf(ofp, b"%%s\0".as_ptr() as *const c_char);
            } else if (*f).flags & TEP_FIELD_IS_SIGNED != 0 {
                fprintf(ofp, b"%%d\0".as_ptr() as *const c_char);
            } else {
                fprintf(ofp, b"%%u\0".as_ptr() as *const c_char);
            }
            f = (*f).next;
        }

        fprintf(ofp, b"\\n\",\n\t       \0".as_ptr() as *const c_char);

        not_first = 0;
        count = 0;
        f = (*event).format.fields;
        while !f.is_null() {
            if not_first != 0 {
                fprintf(ofp, b", \0".as_ptr() as *const c_char);
            }
            not_first += 1;
            count += 1;
            if count % 5 == 0 {
                fprintf(ofp, b"\n\t       \0".as_ptr() as *const c_char);
            }
            if (*f).flags & TEP_FIELD_IS_FLAG != 0 {
                if (count - 1) % 5 != 0 {
                    fprintf(ofp, b"\n\t       \0".as_ptr() as *const c_char);
                    count = 4;
                }
                fprintf(ofp, b"flag_str(\"\0".as_ptr() as *const c_char);
                fprintf(ofp, b"%s::%s\", \0".as_ptr() as *const c_char, (*event).system, (*event).name);
                fprintf(ofp, b"\"%s\", $%s)\0".as_ptr() as *const c_char, (*f).name, (*f).name);
            } else if (*f).flags & TEP_FIELD_IS_SYMBOLIC != 0 {
                if (count - 1) % 5 != 0 {
                    fprintf(ofp, b"\n\t       \0".as_ptr() as *const c_char);
                    count = 4;
                }
                fprintf(ofp, b"symbol_str(\"\0".as_ptr() as *const c_char);
                fprintf(ofp, b"%s::%s\", \0".as_ptr() as *const c_char, (*event).system, (*event).name);
                fprintf(ofp, b"\"%s\", $%s)\0".as_ptr() as *const c_char, (*f).name, (*f).name);
            } else {
                fprintf(ofp, b"$%s\0".as_ptr() as *const c_char, (*f).name);
            }
            f = (*f).next;
        }

        fprintf(ofp, b");\n\n\0".as_ptr() as *const c_char);
        fprintf(ofp, b"\tprint_backtrace($common_callchain);\n\0".as_ptr() as *const c_char);
        fprintf(ofp, b"}\n\n\0".as_ptr() as *const c_char);
        i += 1;
    }

    fprintf(ofp, b"sub trace_unhandled\n{\n\tmy ($event_name, $context, $common_cpu, $common_secs, $common_nsecs,\n\t    $common_pid, $common_comm, $common_callchain) = @_;\n\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"\tprint_header($event_name, $common_cpu, $common_secs, $common_nsecs,\n\t             $common_pid, $common_comm, $common_callchain);\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"\tprint_backtrace($common_callchain);\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"}\n\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"sub print_header\n{\n\tmy ($event_name, $cpu, $secs, $nsecs, $pid, $comm) = @_;\n\n\tprintf(\"%%-20s %%5u %%05u.%%09u %%8u %%-20s \",\n\t       $event_name, $cpu, $secs, $nsecs, $pid, $comm);\n}\n\0".as_ptr() as *const c_char);
    fprintf(ofp, b"\n# Packed byte string args of process_event():\n#\n# $event:\tunion perf_event\tutil/event.h\n# $attr:\tstruct perf_event_attr\tlinux/perf_event.h\n# $sample:\tstruct perf_sample\tutil/event.h\n# $raw_data:\tperf_sample->raw_data\tutil/event.h\n\nsub process_event\n{\n\tmy ($event, $attr, $sample, $raw_data) = @_;\n\n\tmy @event\t= unpack(\"LSS\", $event);\n\tmy @attr\t= unpack(\"LLQQQQQLLQQ\", $attr);\n\tmy @sample\t= unpack(\"QLLQQQQQLL\", $sample);\n\tmy @raw_data\t= unpack(\"C*\", $raw_data);\n\n\tuse Data::Dumper;\n\tprint Dumper \\@event, \\@attr, \\@sample, \\@raw_data;\n}\n\0".as_ptr() as *const c_char);

    fclose(ofp);

    fprintf(stderr, b"generated Perl script: %s\n\0".as_ptr() as *const c_char, fname.as_ptr());

    0
}

#[unsafe(no_mangle)]
pub static mut perl_scripting_ops: scripting_ops = scripting_ops {
    name: b"Perl\0".as_ptr() as *const c_char,
    dirname: b"perl\0".as_ptr() as *const c_char,
    start_script: Some(perl_start_script),
    flush_script: Some(perl_flush_script),
    stop_script: Some(perl_stop_script),
    process_event: Some(perl_process_event),
    generate_script: Some(perl_generate_script),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
