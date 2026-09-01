// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/evsel_fprintf.c.
// C includes become external dependencies supplied by the surrounding tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void, VaList};

type bool_ = bool;
type u64 = u64;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_attr_details {
    pub event_group: bool_,
    pub verbose: bool_,
    pub freq: bool_,
    pub trace_fields: bool_,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub sample_freq: u64,
    pub freq: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub nr_members: c_int,
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub group_name: *const c_char,
}

#[repr(C)]
pub struct perf_sample {
    pub callchain: *mut c_void,
    pub deferred_callchain: bool_,
    pub deferred_cookie: u64,
    pub ip: u64,
}

#[repr(C)]
pub struct callchain_cursor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct callchain_cursor_node {
    pub ms: map_symbol,
    pub ip: u64,
    pub srcline: *const c_char,
}

#[repr(C)]
pub struct addr_location {
    pub addr: u64,
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct tep_format_field {
    pub next: *mut tep_format_field,
    pub name: *const c_char,
}

#[repr(C)]
pub struct tep_event_format {
    pub fields: *mut tep_format_field,
}

#[repr(C)]
pub struct tep_event {
    pub format: tep_event_format,
}

unsafe extern "C" {
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn vfprintf(fp: *mut FILE, fmt: *const c_char, args: VaList<'_, '_>) -> c_int;
    fn fputc(c: c_int, fp: *mut FILE) -> c_int;

    fn perf_event_attr__fprintf(
        fp: *mut FILE,
        attr: *const perf_event_attr,
        cb: unsafe extern "C" fn(*mut FILE, *const c_char, *const c_char, *mut c_void) -> c_int,
        priv_: *mut c_void,
    ) -> c_int;

    fn evsel__is_group_leader(evsel: *mut evsel) -> bool_;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__tp_format(evsel: *mut evsel) -> *const tep_event;
    fn evsel__next_group_member(pos: *mut evsel, leader: *mut evsel) -> *mut evsel;

    fn callchain_cursor_commit(cursor: *mut callchain_cursor);
    fn callchain_cursor_current(cursor: *mut callchain_cursor) -> *mut callchain_cursor_node;
    fn callchain_cursor_advance(cursor: *mut callchain_cursor);

    fn symbol__ignore(sym: *mut symbol) -> bool_;
    fn symbol__inlined(sym: *mut symbol) -> bool_;
    fn __symbol__fprintf_symname_offs(
        sym: *mut symbol,
        al: *mut addr_location,
        print_unknown_as_addr: c_int,
        print_offsets: bool_,
        fp: *mut FILE,
    ) -> c_int;
    fn __symbol__fprintf_symname(
        sym: *mut symbol,
        al: *mut addr_location,
        print_unknown_as_addr: c_int,
        fp: *mut FILE,
    ) -> c_int;

    fn map__map_ip(map: *mut map, ip: u64) -> u64;
    fn map__get(map: *mut map) -> *mut map;
    fn map__fprintf_dsoname_dsoff(map: *mut map, print_dsoff: c_int, addr: u64, fp: *mut FILE) -> c_int;
    fn map__fprintf_srcline(map: *mut map, addr: u64, prefix: *const c_char, fp: *mut FILE) -> c_int;

    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);

    fn strlist__has_entry(list: *mut strlist, entry: *const c_char) -> bool_;
}

unsafe extern "C" {
    static EVSEL__PRINT_IP: c_uint;
    static EVSEL__PRINT_SYM: c_uint;
    static EVSEL__PRINT_DSO: c_uint;
    static EVSEL__PRINT_DSOFF: c_uint;
    static EVSEL__PRINT_SYMOFFSET: c_uint;
    static EVSEL__PRINT_ONELINE: c_uint;
    static EVSEL__PRINT_SRCLINE: c_uint;
    static EVSEL__PRINT_UNKNOWN_AS_ADDR: c_uint;
    static EVSEL__PRINT_CALLCHAIN_ARROW: c_uint;
    static EVSEL__PRINT_SKIP_IGNORED: c_uint;
    static PERF_TYPE_TRACEPOINT: u32;
}

const EMPTY: *const c_char = b"\0".as_ptr() as *const c_char;

unsafe fn comma_fprintf_impl(fp: *mut FILE, first: *mut bool_, fmt: *const c_char, args: VaList<'_, '_>) -> c_int {
    let mut ret: c_int = 0;

    if !*first {
        ret += fprintf(fp, c", ".as_ptr());
    } else {
        ret += fprintf(fp, c":".as_ptr());
        *first = false;
    }

    ret += vfprintf(fp, fmt, args);
    ret
}

// C variadic helper. Rust support for defining C-variadic functions is a direct
// source-level match for the original va_start/vfprintf/va_end sequence.
unsafe extern "C" fn comma_fprintf(fp: *mut FILE, first: *mut bool_, fmt: *const c_char, mut args: ...) -> c_int {
    comma_fprintf_impl(fp, first, fmt, args.as_va_list())
}

unsafe extern "C" fn __print_attr__fprintf(
    fp: *mut FILE,
    name: *const c_char,
    val: *const c_char,
    priv_: *mut c_void,
) -> c_int {
    comma_fprintf(fp, priv_ as *mut bool_, c" %s: %s".as_ptr(), name, val)
}

#[no_mangle]
pub unsafe extern "C" fn evsel__fprintf(evsel: *mut evsel, details: *mut perf_attr_details, fp: *mut FILE) -> c_int {
    let mut first: bool_ = true;
    let mut printed: c_int = 0;

    if (*details).event_group {
        let mut pos: *mut evsel;

        if !evsel__is_group_leader(evsel) {
            return 0;
        }

        if (*evsel).core.nr_members > 1 {
            let group_name = if (*evsel).group_name.is_null() { EMPTY } else { (*evsel).group_name };
            printed += fprintf(fp, c"%s{".as_ptr(), group_name);
        }

        printed += fprintf(fp, c"%s".as_ptr(), evsel__name(evsel));
        pos = evsel__next_group_member(core::ptr::null_mut(), evsel);
        while !pos.is_null() {
            printed += fprintf(fp, c",%s".as_ptr(), evsel__name(pos));
            pos = evsel__next_group_member(pos, evsel);
        }

        if (*evsel).core.nr_members > 1 {
            printed += fprintf(fp, c"}".as_ptr());
        }
    } else {
        printed += fprintf(fp, c"%s".as_ptr(), evsel__name(evsel));

        if (*details).verbose {
            printed += perf_event_attr__fprintf(
                fp,
                &(*evsel).core.attr,
                __print_attr__fprintf,
                &mut first as *mut bool_ as *mut c_void,
            );
        } else if (*details).freq {
            let mut term: *const c_char = c"sample_freq".as_ptr();

            if (*evsel).core.attr.freq == 0 {
                term = c"sample_period".as_ptr();
            }

            printed += comma_fprintf(
                fp,
                &mut first,
                c" %s=%llu".as_ptr(),
                term,
                (*evsel).core.attr.sample_freq as u64,
            );
        }

        // #ifdef HAVE_LIBTRACEEVENT
        if (*details).trace_fields {
            let mut field: *mut tep_format_field;
            let tp_format: *const tep_event;

            if (*evsel).core.attr.type_ != PERF_TYPE_TRACEPOINT {
                printed += comma_fprintf(fp, &mut first, c" (not a tracepoint)".as_ptr());
                fputc('\n' as c_int, fp);
                return printed + 1;
            }

            tp_format = evsel__tp_format(evsel);
            field = if !tp_format.is_null() {
                (*tp_format).format.fields
            } else {
                core::ptr::null_mut()
            };
            if field.is_null() {
                printed += comma_fprintf(fp, &mut first, c" (no trace field)".as_ptr());
                fputc('\n' as c_int, fp);
                return printed + 1;
            }

            printed += comma_fprintf(fp, &mut first, c" trace_fields: %s".as_ptr(), (*field).name);

            field = (*field).next;
            while !field.is_null() {
                printed += comma_fprintf(fp, &mut first, c"%s".as_ptr(), (*field).name);
                field = (*field).next;
            }
        }
        // #endif
    }

    fputc('\n' as c_int, fp);
    printed += 1;
    printed
}

#[no_mangle]
pub unsafe extern "C" fn sample__fprintf_callchain(
    sample: *mut perf_sample,
    left_alignment: c_int,
    print_opts: c_uint,
    cursor: *mut callchain_cursor,
    bt_stop_list: *mut strlist,
    fp: *mut FILE,
) -> c_int {
    let mut printed: c_int = 0;
    let mut node: *mut callchain_cursor_node;
    let print_ip: c_int = (print_opts & EVSEL__PRINT_IP) as c_int;
    let print_sym: c_int = (print_opts & EVSEL__PRINT_SYM) as c_int;
    let print_dso: c_int = (print_opts & EVSEL__PRINT_DSO) as c_int;
    let print_dsoff: c_int = (print_opts & EVSEL__PRINT_DSOFF) as c_int;
    let print_symoffset: c_int = (print_opts & EVSEL__PRINT_SYMOFFSET) as c_int;
    let print_oneline: c_int = (print_opts & EVSEL__PRINT_ONELINE) as c_int;
    let print_srcline: c_int = (print_opts & EVSEL__PRINT_SRCLINE) as c_int;
    let print_unknown_as_addr: c_int = (print_opts & EVSEL__PRINT_UNKNOWN_AS_ADDR) as c_int;
    let print_arrow: c_int = (print_opts & EVSEL__PRINT_CALLCHAIN_ARROW) as c_int;
    let print_skip_ignored: c_int = (print_opts & EVSEL__PRINT_SKIP_IGNORED) as c_int;
    let s: c_char = if print_oneline != 0 { b' ' as c_char } else { b'\t' as c_char };
    let mut first: bool_ = true;

    if cursor.is_null() {
        return fprintf(
            fp,
            c"<not enough memory for the callchain cursor>%s".as_ptr(),
            if print_oneline != 0 { c"".as_ptr() } else { c"\n".as_ptr() },
        );
    }

    if !(*sample).callchain.is_null() {
        callchain_cursor_commit(cursor);

        loop {
            let sym: *mut symbol;
            let map: *mut map;
            let mut addr: u64 = 0;

            node = callchain_cursor_current(cursor);
            if node.is_null() {
                break;
            }

            sym = (*node).ms.sym;
            map = (*node).ms.map;

            if !sym.is_null() && symbol__ignore(sym) && print_skip_ignored != 0 {
                callchain_cursor_advance(cursor);
                continue;
            }

            printed += fprintf(fp, c"%-*.*s".as_ptr(), left_alignment, left_alignment, c" ".as_ptr());

            if print_arrow != 0 && !first {
                printed += fprintf(fp, c" <-".as_ptr());
            }

            if !map.is_null() {
                addr = map__map_ip(map, (*node).ip);
            }

            if print_ip != 0 {
                printed += fprintf(fp, c"%c%16llx".as_ptr(), s as c_int, (*node).ip);
            }

            if print_sym != 0 {
                let mut node_al = core::mem::MaybeUninit::<addr_location>::uninit();

                addr_location__init(node_al.as_mut_ptr());
                printed += fprintf(fp, c" ".as_ptr());
                (*node_al.as_mut_ptr()).addr = addr;
                (*node_al.as_mut_ptr()).map = map__get(map);

                if (*sample).deferred_callchain && (*sample).deferred_cookie == (*node).ip {
                    printed += fprintf(fp, c"(cookie)".as_ptr());
                } else if print_symoffset != 0 {
                    printed += __symbol__fprintf_symname_offs(
                        sym,
                        node_al.as_mut_ptr(),
                        print_unknown_as_addr,
                        true,
                        fp,
                    );
                } else {
                    printed += __symbol__fprintf_symname(sym, node_al.as_mut_ptr(), print_unknown_as_addr, fp);
                }
                addr_location__exit(node_al.as_mut_ptr());
            }

            if print_dso != 0 && (sym.is_null() || !symbol__inlined(sym)) {
                printed += map__fprintf_dsoname_dsoff(map, print_dsoff, addr, fp);
            }

            if print_srcline != 0 {
                if !(*node).srcline.is_null() {
                    printed += fprintf(fp, c"\n  %s".as_ptr(), (*node).srcline);
                } else {
                    printed += map__fprintf_srcline(map, addr, c"\n  ".as_ptr(), fp);
                }
            }

            if !sym.is_null() && symbol__inlined(sym) {
                printed += fprintf(fp, c" (inlined)".as_ptr());
            }

            if print_oneline == 0 {
                printed += fprintf(fp, c"\n".as_ptr());
            }

            /* Add srccode here too? */
            if !bt_stop_list.is_null()
                && !sym.is_null()
                && strlist__has_entry(bt_stop_list, (*sym).name)
            {
                break;
            }

            first = false;
            callchain_cursor_advance(cursor);
        }
    }

    printed
}

#[no_mangle]
pub unsafe extern "C" fn sample__fprintf_sym(
    sample: *mut perf_sample,
    al: *mut addr_location,
    left_alignment: c_int,
    print_opts: c_uint,
    cursor: *mut callchain_cursor,
    bt_stop_list: *mut strlist,
    fp: *mut FILE,
) -> c_int {
    let mut printed: c_int = 0;
    let print_ip: c_int = (print_opts & EVSEL__PRINT_IP) as c_int;
    let print_sym: c_int = (print_opts & EVSEL__PRINT_SYM) as c_int;
    let print_dso: c_int = (print_opts & EVSEL__PRINT_DSO) as c_int;
    let print_dsoff: c_int = (print_opts & EVSEL__PRINT_DSOFF) as c_int;
    let print_symoffset: c_int = (print_opts & EVSEL__PRINT_SYMOFFSET) as c_int;
    let print_srcline: c_int = (print_opts & EVSEL__PRINT_SRCLINE) as c_int;
    let print_unknown_as_addr: c_int = (print_opts & EVSEL__PRINT_UNKNOWN_AS_ADDR) as c_int;

    if !cursor.is_null() {
        printed += sample__fprintf_callchain(sample, left_alignment, print_opts, cursor, bt_stop_list, fp);
    } else {
        printed += fprintf(fp, c"%-*.*s".as_ptr(), left_alignment, left_alignment, c" ".as_ptr());

        if print_ip != 0 {
            printed += fprintf(fp, c"%16llx".as_ptr(), (*sample).ip);
        }

        if print_sym != 0 {
            printed += fprintf(fp, c" ".as_ptr());
            if print_symoffset != 0 {
                printed += __symbol__fprintf_symname_offs(
                    (*al).sym,
                    al,
                    print_unknown_as_addr,
                    true,
                    fp,
                );
            } else {
                printed += __symbol__fprintf_symname((*al).sym, al, print_unknown_as_addr, fp);
            }
        }

        if print_dso != 0 {
            printed += map__fprintf_dsoname_dsoff((*al).map, print_dsoff, (*al).addr, fp);
        }

        if print_srcline != 0 {
            printed += map__fprintf_srcline((*al).map, (*al).addr, c"\n  ".as_ptr(), fp);
        }
    }

    printed
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
