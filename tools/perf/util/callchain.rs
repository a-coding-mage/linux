// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2009-2011, Frederic Weisbecker <fweisbec@gmail.com>
 *
 * Handle the callchains from the stream in an ad-hoc radix tree and then
 * sort them in an rbtree.
 *
 * Using a radix for code path provides a fast retrieval and factorizes
 * memory use. Also that lets us use the paths in a hierarchical graph view.
 *
 */

use core::ffi::{c_char, c_double, c_float, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u64 = u64;
type s64 = i64;
type size_t = usize;
type uint16_t = u16;
type pthread_key_t = c_ulong;
type pthread_once_t = c_int;
type FILE = c_void;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const USHRT_MAX: c_ulong = 65535;
const PERF_RECORD_MISC_KERNEL: c_int = 1;
const PERF_RECORD_MISC_USER: c_int = 2;
const PERF_RECORD_MISC_HYPERVISOR: c_int = 3;
const PERF_RECORD_MISC_GUEST_KERNEL: c_int = 4;
const PERF_RECORD_MISC_GUEST_USER: c_int = 5;
const PERF_SAMPLE_REGS_USER: u64 = 1 << 12;
const PERF_SAMPLE_STACK_USER: u64 = 1 << 13;
const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
const EM_AARCH64: uint16_t = 183;
const PTHREAD_ONCE_INIT: pthread_once_t = 0;

#[repr(C)]
pub struct rb_node {
    pub rb_left: *mut rb_node,
    pub rb_right: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub name: *const c_char,
}

#[repr(C)]
pub struct map_symbol {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_flags {
    pub cycles: u64,
    pub predicted: bool,
    pub abort: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_type_stat {
    pub branch_to: bool,
}

#[repr(C)]
pub struct callchain_list {
    pub list: list_head,
    pub ip: u64,
    pub ms: map_symbol,
    pub srcline: *const c_char,
    pub branch_count: u64,
    pub predicted_count: u64,
    pub abort_count: u64,
    pub cycles_count: u64,
    pub iter_count: u64,
    pub iter_cycles: u64,
    pub from_count: u64,
    pub brtype_stat: *mut branch_type_stat,
    pub has_children: bool,
}

#[repr(C)]
pub struct callchain_node {
    pub rb_node: rb_node,
    pub rb_node_in: rb_node,
    pub rb_root: rb_root,
    pub rb_root_in: rb_root,
    pub parent: *mut callchain_node,
    pub val: list_head,
    pub parent_val: list_head,
    pub val_nr: u64,
    pub hit: u64,
    pub children_hit: u64,
    pub count: u32,
    pub children_count: u32,
}

#[repr(C)]
pub struct callchain_root {
    pub node: callchain_node,
    pub max_depth: u64,
}

#[repr(C)]
pub struct callchain_cursor_node {
    pub next: *mut callchain_cursor_node,
    pub ip: u64,
    pub ms: map_symbol,
    pub branch: bool,
    pub branch_flags: branch_flags,
    pub nr_loop_iter: c_int,
    pub iter_cycles: u64,
    pub branch_from: u64,
    pub srcline: *const c_char,
}

#[repr(C)]
pub struct callchain_cursor {
    pub first: *mut callchain_cursor_node,
    pub last: *mut *mut callchain_cursor_node,
    pub curr: *mut callchain_cursor_node,
    pub nr: u64,
    pub pos: u64,
}

#[repr(C)]
pub struct record_opts {
    pub record_data_mmap_set: bool,
    pub record_data_mmap: bool,
}

#[repr(C)]
pub struct callchain_param {
    pub mode: chain_mode,
    pub min_percent: c_double,
    pub order: chain_order,
    pub key: callchain_key,
    pub value: callchain_value,
    pub enabled: bool,
    pub order_set: bool,
    pub branch_callstack: c_int,
    pub record_mode: callchain_record_mode,
    pub dump_size: c_ulong,
    pub defer: bool,
    pub max_stack: c_ulong,
    pub print_limit: c_ulong,
    pub sort: Option<unsafe extern "C" fn(*mut rb_root, *mut callchain_root, u64, *mut callchain_param)>,
}

#[repr(C)]
pub struct perf_sample {
    pub callchain: *mut ip_callchain,
    pub period: u64,
    pub deferred_callchain: bool,
    pub merged_callchain: bool,
}

#[repr(C)]
pub struct ip_callchain {
    pub nr: u64,
    pub ips: [u64; 0],
}

#[repr(C)]
pub struct addr_location {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
    pub srcline: *const c_char,
    pub addr: u64,
    pub cpumode: c_int,
    pub level: c_char,
}

#[repr(C)]
pub struct hist_entry {
    pub callchain: *mut callchain_root,
    pub sorted_chain: rb_root,
    pub rb_node: rb_node,
}

#[repr(C)]
pub struct hists {
    pub entries: rb_root_cached,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub use_callchain: bool,
    pub cumulate_callchain: bool,
    pub show_branchflag_count: bool,
}

#[repr(C)]
pub struct perf_hpp_list_t {
    pub parent: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chain_mode {
    CHAIN_NONE = 0,
    CHAIN_GRAPH_ABS,
    CHAIN_GRAPH_REL,
    CHAIN_FLAT,
    CHAIN_FOLDED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chain_order {
    ORDER_CALLER = 0,
    ORDER_CALLEE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum callchain_key {
    CCKEY_FUNCTION = 0,
    CCKEY_ADDRESS,
    CCKEY_SRCLINE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum callchain_value {
    CCVAL_PERCENT = 0,
    CCVAL_PERIOD,
    CCVAL_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum callchain_record_mode {
    CALLCHAIN_NONE = 0,
    CALLCHAIN_FP,
    CALLCHAIN_DWARF,
    CALLCHAIN_LBR,
}

const CALLCHAIN_MAX: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum match_result {
    MATCH_ERROR = -1,
    MATCH_EQ = 0,
    MATCH_LT = 1,
    MATCH_GT = 2,
}

type callchain_iter_fn = unsafe extern "C" fn(*mut callchain_cursor_node, *mut c_void) -> c_int;

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut perf_hpp_list: perf_hpp_list_t;
    static mut perf_guest: bool;

    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn perror(s: *const c_char);
    fn abort() -> !;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn ceil(x: c_double) -> c_double;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn percent_color_fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn WARN_ONCE(condition: bool, fmt: *const c_char, ...);

    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_prev(node: *const rb_node) -> *mut rb_node;
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);

    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn sysctl__max_stack() -> c_int;
    fn branch_type_count(stat: *mut branch_type_stat, flags: *const branch_flags, from: u64, to: u64);
    fn branch_type_str(stat: *const branch_type_stat, bf: *mut c_char, bfsize: c_int) -> c_int;
    fn map_symbol__copy(dst: *mut map_symbol, src: *const map_symbol);
    fn map_symbol__exit(ms: *mut map_symbol);
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__get(map: *mut map) -> *mut map;
    fn map__put(map: *mut map);
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn symbol__inlined(sym: *mut symbol) -> bool;
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn maps__equal(a: *mut maps, b: *mut maps) -> bool;
    fn machine__kernel_maps(machine: *mut machine) -> *mut maps;
    fn machine__is_host(machine: *mut machine) -> bool;
    fn thread__resolve_callchain(thread: *mut thread, cursor: *mut callchain_cursor, sample: *mut perf_sample, parent: *mut *mut symbol, al: *mut addr_location, max_stack: c_int) -> c_int;
    fn __thread__resolve_callchain(thread: *mut thread, cursor: *mut callchain_cursor, sample: *mut perf_sample, parent: *mut *mut symbol, root_al: *mut addr_location, max_stack: c_int, symbols: bool) -> c_int;
    fn pthread_key_create(key: *mut pthread_key_t, destructor: Option<unsafe extern "C" fn(*mut c_void)>) -> c_int;
    fn pthread_once(once: *mut pthread_once_t, init: unsafe extern "C" fn());
    fn pthread_getspecific(key: pthread_key_t) -> *mut c_void;
    fn pthread_setspecific(key: pthread_key_t, value: *const c_void) -> c_int;
}

const fn rb_root_empty() -> rb_root {
    rb_root { rb_node: ptr::null_mut() }
}

#[inline]
unsafe fn init_list_head(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

#[inline]
unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

#[inline]
unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    let prev = (*head).prev;
    (*new).next = head;
    (*new).prev = prev;
    (*prev).next = new;
    (*head).prev = new;
}

#[inline]
unsafe fn list_del_init(entry: *mut list_head) {
    let prev = (*entry).prev;
    let next = (*entry).next;
    (*next).prev = prev;
    (*prev).next = next;
    init_list_head(entry);
}

#[inline]
unsafe fn list_del_range(first: *mut list_head, last: *mut list_head) {
    (*(*first).prev).next = (*last).next;
    (*(*last).next).prev = (*first).prev;
}

#[inline]
unsafe fn list_move_tail(list: *mut list_head, head: *mut list_head) {
    list_del_init(list);
    list_add_tail(list, head);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! container_of {
    ($ptr:expr, $type:ty, $field:ident) => {{
        let base = ptr::null::<$type>();
        let offset = unsafe { &(*base).$field as *const _ as usize };
        ($ptr as *mut u8).wrapping_sub(offset) as *mut $type
    }};
}

macro_rules! rb_entry {
    ($ptr:expr, $type:ty, $field:ident) => {
        container_of!($ptr, $type, $field)
    };
}

macro_rules! list_entry {
    ($ptr:expr, $type:ty, $field:ident) => {
        container_of!($ptr, $type, $field)
    };
}

unsafe fn callchain_cumul_hits(node: *mut callchain_node) -> u64 {
    (*node).hit.wrapping_add((*node).children_hit)
}

unsafe fn callchain_cumul_counts(node: *mut callchain_node) -> u32 {
    (*node).count.wrapping_add((*node).children_count)
}

#[no_mangle]
pub static mut callchain_param: callchain_param = callchain_param {
    mode: chain_mode::CHAIN_GRAPH_ABS,
    min_percent: 0.5,
    order: chain_order::ORDER_CALLEE,
    key: callchain_key::CCKEY_FUNCTION,
    value: callchain_value::CCVAL_PERCENT,
    enabled: false,
    order_set: false,
    branch_callstack: 0,
    record_mode: callchain_record_mode::CALLCHAIN_NONE,
    dump_size: 0,
    defer: false,
    max_stack: 0,
    print_limit: 0,
    sort: None,
};

/*
 * Are there any events usind DWARF callchains?
 *
 * I.e.
 *
 * -e cycles/call-graph=dwarf/
 */
#[no_mangle]
pub static mut dwarf_callchain_users: bool = false;

#[no_mangle]
pub static mut callchain_param_default: callchain_param = callchain_param {
    mode: chain_mode::CHAIN_GRAPH_ABS,
    min_percent: 0.5,
    order: chain_order::ORDER_CALLEE,
    key: callchain_key::CCKEY_FUNCTION,
    value: callchain_value::CCVAL_PERCENT,
    enabled: false,
    order_set: false,
    branch_callstack: 0,
    record_mode: callchain_record_mode::CALLCHAIN_NONE,
    dump_size: 0,
    defer: false,
    max_stack: 0,
    print_limit: 0,
    sort: None,
};

/* Used for thread-local struct callchain_cursor. */
static mut callchain_cursor: pthread_key_t = 0;

#[no_mangle]
pub unsafe extern "C" fn parse_callchain_record_opt(arg: *const c_char, param: *mut callchain_param) -> c_int {
    parse_callchain_record(arg, param)
}

unsafe fn parse_callchain_mode(value: *const c_char) -> c_int {
    if strncmp(value, cstr!("graph"), strlen(value)) == 0 {
        callchain_param.mode = chain_mode::CHAIN_GRAPH_ABS;
        return 0;
    }
    if strncmp(value, cstr!("flat"), strlen(value)) == 0 {
        callchain_param.mode = chain_mode::CHAIN_FLAT;
        return 0;
    }
    if strncmp(value, cstr!("fractal"), strlen(value)) == 0 {
        callchain_param.mode = chain_mode::CHAIN_GRAPH_REL;
        return 0;
    }
    if strncmp(value, cstr!("folded"), strlen(value)) == 0 {
        callchain_param.mode = chain_mode::CHAIN_FOLDED;
        return 0;
    }
    -1
}

unsafe fn parse_callchain_order(value: *const c_char) -> c_int {
    if strncmp(value, cstr!("caller"), strlen(value)) == 0 {
        callchain_param.order = chain_order::ORDER_CALLER;
        callchain_param.order_set = true;
        return 0;
    }
    if strncmp(value, cstr!("callee"), strlen(value)) == 0 {
        callchain_param.order = chain_order::ORDER_CALLEE;
        callchain_param.order_set = true;
        return 0;
    }
    -1
}

unsafe fn parse_callchain_sort_key(value: *const c_char) -> c_int {
    if strncmp(value, cstr!("function"), strlen(value)) == 0 {
        callchain_param.key = callchain_key::CCKEY_FUNCTION;
        return 0;
    }
    if strncmp(value, cstr!("address"), strlen(value)) == 0 {
        callchain_param.key = callchain_key::CCKEY_ADDRESS;
        return 0;
    }
    if strncmp(value, cstr!("srcline"), strlen(value)) == 0 {
        callchain_param.key = callchain_key::CCKEY_SRCLINE;
        return 0;
    }
    if strncmp(value, cstr!("branch"), strlen(value)) == 0 {
        callchain_param.branch_callstack = 1;
        return 0;
    }
    -1
}

unsafe fn parse_callchain_value(value: *const c_char) -> c_int {
    if strncmp(value, cstr!("percent"), strlen(value)) == 0 {
        callchain_param.value = callchain_value::CCVAL_PERCENT;
        return 0;
    }
    if strncmp(value, cstr!("period"), strlen(value)) == 0 {
        callchain_param.value = callchain_value::CCVAL_PERIOD;
        return 0;
    }
    if strncmp(value, cstr!("count"), strlen(value)) == 0 {
        callchain_param.value = callchain_value::CCVAL_COUNT;
        return 0;
    }
    -1
}

#[inline]
fn round_down(value: c_ulong, align: usize) -> c_ulong {
    value & !((align as c_ulong) - 1)
}

#[inline]
fn round_up(value: c_ulong, align: usize) -> c_ulong {
    (value.wrapping_add(align as c_ulong - 1)) & !((align as c_ulong) - 1)
}

unsafe fn get_stack_size(str_: *const c_char, size_out: *mut c_ulong) -> c_int {
    let mut endptr: *mut c_char = ptr::null_mut();
    let mut size: c_ulong;
    let max_size = round_down(USHRT_MAX, size_of::<u64>());

    size = strtoul(str_, &mut endptr, 0);

    loop {
        if *endptr != 0 {
            break;
        }

        size = round_up(size, size_of::<u64>());
        if size == 0 || size > max_size {
            break;
        }

        *size_out = size;
        return 0;
    }

    pr_err(cstr!("callchain: Incorrect stack dump size (max %ld): %s\n"), max_size, str_);
    -1
}

unsafe fn __parse_callchain_report_opt(arg: *const c_char, allow_record_opt: bool) -> c_int {
    let mut saveptr: *mut c_char = ptr::null_mut();
    let mut endptr: *mut c_char = ptr::null_mut();
    let mut minpcnt_set = false;
    let mut record_opt_set = false;
    let mut try_stack_size = false;

    callchain_param.enabled = true;
    symbol_conf.use_callchain = true;

    if arg.is_null() {
        return 0;
    }

    let arg_copy = strdup(arg);
    if arg_copy.is_null() {
        return -ENOMEM;
    }

    let mut tok = strtok_r(arg_copy, cstr!(","), &mut saveptr);
    while !tok.is_null() {
        if strncmp(tok, cstr!("none"), strlen(tok)) == 0 {
            callchain_param.mode = chain_mode::CHAIN_NONE;
            callchain_param.enabled = false;
            symbol_conf.use_callchain = false;
            free(arg_copy as *mut c_void);
            return 0;
        }

        if parse_callchain_mode(tok) == 0
            || parse_callchain_order(tok) == 0
            || parse_callchain_sort_key(tok) == 0
            || parse_callchain_value(tok) == 0
        {
            /* parsing ok - move on to the next */
            try_stack_size = false;
        } else if allow_record_opt && !record_opt_set {
            if parse_callchain_record(tok, &raw mut callchain_param) != 0 {
                if try_stack_size {
                    let mut size: c_ulong = 0;
                    if get_stack_size(tok, &mut size) < 0 {
                        free(arg_copy as *mut c_void);
                        return -1;
                    }
                    callchain_param.dump_size = size;
                    try_stack_size = false;
                } else if !minpcnt_set {
                    callchain_param.min_percent = strtod(tok, &mut endptr);
                    if tok == endptr {
                        free(arg_copy as *mut c_void);
                        return -1;
                    }
                    minpcnt_set = true;
                } else {
                    callchain_param.print_limit = strtoul(tok, &mut endptr, 0);
                    if tok == endptr {
                        free(arg_copy as *mut c_void);
                        return -1;
                    }
                }
            } else {
                /* assume that number followed by 'dwarf' is stack size */
                if callchain_param.record_mode == callchain_record_mode::CALLCHAIN_DWARF {
                    try_stack_size = true;
                }
                record_opt_set = true;
            }
        } else if try_stack_size {
            let mut size: c_ulong = 0;
            if get_stack_size(tok, &mut size) < 0 {
                free(arg_copy as *mut c_void);
                return -1;
            }
            callchain_param.dump_size = size;
            try_stack_size = false;
        } else if !minpcnt_set {
            /* try to get the min percent */
            callchain_param.min_percent = strtod(tok, &mut endptr);
            if tok == endptr {
                free(arg_copy as *mut c_void);
                return -1;
            }
            minpcnt_set = true;
        } else {
            /* try print limit at last */
            callchain_param.print_limit = strtoul(tok, &mut endptr, 0);
            if tok == endptr {
                free(arg_copy as *mut c_void);
                return -1;
            }
        }

        tok = strtok_r(ptr::null_mut(), cstr!(","), &mut saveptr);
    }

    if callchain_register_param(&raw mut callchain_param) < 0 {
        pr_err(cstr!("Can't register callchain params\n"));
        free(arg_copy as *mut c_void);
        return -1;
    }
    free(arg_copy as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn parse_callchain_report_opt(arg: *const c_char) -> c_int {
    __parse_callchain_report_opt(arg, false)
}

#[no_mangle]
pub unsafe extern "C" fn parse_callchain_top_opt(arg: *const c_char) -> c_int {
    __parse_callchain_report_opt(arg, true)
}

#[no_mangle]
pub unsafe extern "C" fn parse_callchain_record(arg: *const c_char, param: *mut callchain_param) -> c_int {
    let mut saveptr: *mut c_char = ptr::null_mut();
    let mut ret = -1;

    /* We need buffer that we know we can write to. */
    let buf = strdup(arg);
    if buf.is_null() {
        return -ENOMEM;
    }

    let mut tok = strtok_r(buf, cstr!(","), &mut saveptr);
    let mut name = if !tok.is_null() { tok } else { buf };

    loop {
        /* Framepointer style */
        if strncmp(name, cstr!("fp"), size_of!("fp")) == 0 {
            ret = 0;
            (*param).record_mode = callchain_record_mode::CALLCHAIN_FP;

            tok = strtok_r(ptr::null_mut(), cstr!(","), &mut saveptr);
            if !tok.is_null() {
                if strncmp(tok, cstr!("defer"), size_of!("defer")) == 0 {
                    (*param).defer = true;
                } else {
                    let size = strtoul(tok, &mut name, 0);
                    if size < sysctl__max_stack() as c_ulong {
                        (*param).max_stack = size;
                    }
                }
            }
            break;
        } else if strncmp(name, cstr!("dwarf"), size_of!("dwarf")) == 0 {
            /* Dwarf style */
            let default_stack_dump_size: c_ulong = 8192;

            ret = 0;
            (*param).record_mode = callchain_record_mode::CALLCHAIN_DWARF;
            (*param).dump_size = default_stack_dump_size;
            dwarf_callchain_users = true;

            tok = strtok_r(ptr::null_mut(), cstr!(","), &mut saveptr);
            if !tok.is_null() {
                let mut size: c_ulong = 0;
                ret = get_stack_size(tok, &mut size);
                (*param).dump_size = size;
            }
        } else if strncmp(name, cstr!("lbr"), size_of!("lbr")) == 0 {
            if strtok_r(ptr::null_mut(), cstr!(","), &mut saveptr).is_null() {
                (*param).record_mode = callchain_record_mode::CALLCHAIN_LBR;
                ret = 0;
            } else {
                pr_err(cstr!("callchain: No more arguments needed for --call-graph lbr\n"));
            }
            break;
        } else {
            pr_err(cstr!("callchain: Unknown --call-graph option value: %s\n"), arg);
            break;
        }
        break;
    }

    free(buf as *mut c_void);

    if (*param).defer && (*param).record_mode != callchain_record_mode::CALLCHAIN_FP {
        pr_err(cstr!("callchain: deferred callchain only works with FP\n"));
        return -EINVAL;
    }

    ret
}

unsafe fn callchain_debug(callchain: *const callchain_param) {
    static STRS: [*const c_char; CALLCHAIN_MAX] = [
        cstr!("NONE"),
        cstr!("FP"),
        cstr!("DWARF"),
        cstr!("LBR"),
    ];

    pr_debug(cstr!("callchain: type %s\n"), STRS[(*callchain).record_mode as usize]);

    if (*callchain).record_mode == callchain_record_mode::CALLCHAIN_DWARF {
        pr_debug(cstr!("callchain: stack dump size %d\n"), (*callchain).dump_size);
    }
}

#[no_mangle]
pub unsafe extern "C" fn record_opts__parse_callchain(
    record: *mut record_opts,
    callchain: *mut callchain_param,
    arg: *const c_char,
    unset: bool,
) -> c_int {
    (*callchain).enabled = !unset;

    /* --no-call-graph */
    if unset {
        (*callchain).record_mode = callchain_record_mode::CALLCHAIN_NONE;
        pr_debug(cstr!("callchain: disabled\n"));
        return 0;
    }

    let ret = parse_callchain_record_opt(arg, callchain);
    if ret == 0 {
        /* Enable data address sampling for DWARF unwind. */
        if (*callchain).record_mode == callchain_record_mode::CALLCHAIN_DWARF && !(*record).record_data_mmap_set {
            (*record).record_data_mmap = true;
        }
        callchain_debug(callchain);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_callchain_config(var: *const c_char, value: *const c_char) -> c_int {
    let mut endptr: *mut c_char = ptr::null_mut();

    if strncmp(var, cstr!("call-graph."), strlen(cstr!("call-graph.")) - 1) != 0 {
        return 0;
    }
    let var = var.add(size_of!("call-graph.") - 1);

    if strcmp(var, cstr!("record-mode")) == 0 {
        return parse_callchain_record_opt(value, &raw mut callchain_param);
    }
    if strcmp(var, cstr!("dump-size")) == 0 {
        let mut size: c_ulong = 0;
        let ret = get_stack_size(value, &mut size);
        callchain_param.dump_size = size;
        return ret;
    }
    if strcmp(var, cstr!("print-type")) == 0 {
        let ret = parse_callchain_mode(value);
        if ret == -1 {
            pr_err(cstr!("Invalid callchain mode: %s\n"), value);
        }
        return ret;
    }
    if strcmp(var, cstr!("order")) == 0 {
        let ret = parse_callchain_order(value);
        if ret == -1 {
            pr_err(cstr!("Invalid callchain order: %s\n"), value);
        }
        return ret;
    }
    if strcmp(var, cstr!("sort-key")) == 0 {
        let ret = parse_callchain_sort_key(value);
        if ret == -1 {
            pr_err(cstr!("Invalid callchain sort key: %s\n"), value);
        }
        return ret;
    }
    if strcmp(var, cstr!("threshold")) == 0 {
        callchain_param.min_percent = strtod(value, &mut endptr);
        if value == endptr {
            pr_err(cstr!("Invalid callchain threshold: %s\n"), value);
            return -1;
        }
    }
    if strcmp(var, cstr!("print-limit")) == 0 {
        callchain_param.print_limit = strtod(value, &mut endptr) as c_ulong;
        if value == endptr {
            pr_err(cstr!("Invalid callchain print limit: %s\n"), value);
            return -1;
        }
    }

    0
}

macro_rules! size_of {
    ($s:literal) => {
        $s.len() + 1
    };
}

unsafe extern "C" fn rb_insert_callchain(root: *mut rb_root, chain: *mut callchain_node, mode: chain_mode) {
    let mut p: *mut *mut rb_node = &mut (*root).rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    let chain_cumul = callchain_cumul_hits(chain);

    while !(*p).is_null() {
        parent = *p;
        let rnode = rb_entry!(parent, callchain_node, rb_node);
        let rnode_cumul = callchain_cumul_hits(rnode);

        match mode {
            chain_mode::CHAIN_FLAT | chain_mode::CHAIN_FOLDED => {
                if (*rnode).hit < (*chain).hit {
                    p = &mut (**p).rb_left;
                } else {
                    p = &mut (**p).rb_right;
                }
            }
            chain_mode::CHAIN_GRAPH_ABS | chain_mode::CHAIN_GRAPH_REL => {
                if rnode_cumul < chain_cumul {
                    p = &mut (**p).rb_left;
                } else {
                    p = &mut (**p).rb_right;
                }
            }
            chain_mode::CHAIN_NONE => {}
        }
    }

    rb_link_node(&mut (*chain).rb_node, parent, p);
    rb_insert_color(&mut (*chain).rb_node, root);
}

unsafe extern "C" fn __sort_chain_flat(rb_root: *mut rb_root, node: *mut callchain_node, min_hit: u64) {
    let mut n = rb_first(&(*node).rb_root_in);
    while !n.is_null() {
        let child = rb_entry!(n, callchain_node, rb_node_in);
        n = rb_next(n);
        __sort_chain_flat(rb_root, child, min_hit);
    }

    if (*node).hit != 0 && (*node).hit >= min_hit {
        rb_insert_callchain(rb_root, node, chain_mode::CHAIN_FLAT);
    }
}

/*
 * Once we get every callchains from the stream, we can now
 * sort them by hit
 */
unsafe extern "C" fn sort_chain_flat(rb_root: *mut rb_root, root: *mut callchain_root, min_hit: u64, _param: *mut callchain_param) {
    *rb_root = rb_root_empty();
    __sort_chain_flat(rb_root, &mut (*root).node, min_hit);
}

unsafe extern "C" fn __sort_chain_graph_abs(node: *mut callchain_node, min_hit: u64) {
    (*node).rb_root = rb_root_empty();
    let mut n = rb_first(&(*node).rb_root_in);

    while !n.is_null() {
        let child = rb_entry!(n, callchain_node, rb_node_in);
        n = rb_next(n);

        __sort_chain_graph_abs(child, min_hit);
        if callchain_cumul_hits(child) >= min_hit {
            rb_insert_callchain(&mut (*node).rb_root, child, chain_mode::CHAIN_GRAPH_ABS);
        }
    }
}

unsafe extern "C" fn sort_chain_graph_abs(rb_root: *mut rb_root, chain_root: *mut callchain_root, min_hit: u64, _param: *mut callchain_param) {
    __sort_chain_graph_abs(&mut (*chain_root).node, min_hit);
    (*rb_root).rb_node = (*chain_root).node.rb_root.rb_node;
}

unsafe extern "C" fn __sort_chain_graph_rel(node: *mut callchain_node, min_percent: c_double) {
    (*node).rb_root = rb_root_empty();
    let min_hit = ceil((*node).children_hit as c_double * min_percent) as u64;

    let mut n = rb_first(&(*node).rb_root_in);
    while !n.is_null() {
        let child = rb_entry!(n, callchain_node, rb_node_in);
        n = rb_next(n);

        __sort_chain_graph_rel(child, min_percent);
        if callchain_cumul_hits(child) >= min_hit {
            rb_insert_callchain(&mut (*node).rb_root, child, chain_mode::CHAIN_GRAPH_REL);
        }
    }
}

unsafe extern "C" fn sort_chain_graph_rel(rb_root: *mut rb_root, chain_root: *mut callchain_root, _min_hit: u64, param: *mut callchain_param) {
    __sort_chain_graph_rel(&mut (*chain_root).node, (*param).min_percent / 100.0);
    (*rb_root).rb_node = (*chain_root).node.rb_root.rb_node;
}

#[no_mangle]
pub unsafe extern "C" fn callchain_register_param(param: *mut callchain_param) -> c_int {
    match (*param).mode {
        chain_mode::CHAIN_GRAPH_ABS => (*param).sort = Some(sort_chain_graph_abs),
        chain_mode::CHAIN_GRAPH_REL => (*param).sort = Some(sort_chain_graph_rel),
        chain_mode::CHAIN_FLAT | chain_mode::CHAIN_FOLDED => (*param).sort = Some(sort_chain_flat),
        chain_mode::CHAIN_NONE => return -1,
    }
    0
}

/*
 * Create a child for a parent. If inherit_children, then the new child
 * will become the new parent of it's parent children
 */
unsafe fn create_child(parent: *mut callchain_node, inherit_children: bool) -> *mut callchain_node {
    let new = zalloc(size_of::<callchain_node>()) as *mut callchain_node;
    if new.is_null() {
        perror(cstr!("not enough memory to create child for code path tree"));
        return ptr::null_mut();
    }
    (*new).parent = parent;
    init_list_head(&mut (*new).val);
    init_list_head(&mut (*new).parent_val);

    if inherit_children {
        (*new).rb_root_in = (*parent).rb_root_in;
        (*parent).rb_root_in = rb_root_empty();

        let mut n = rb_first(&(*new).rb_root_in);
        while !n.is_null() {
            let child = rb_entry!(n, callchain_node, rb_node_in);
            (*child).parent = new;
            n = rb_next(n);
        }

        /* make it the first child */
        rb_link_node(&mut (*new).rb_node_in, ptr::null_mut(), &mut (*parent).rb_root_in.rb_node);
        rb_insert_color(&mut (*new).rb_node_in, &mut (*parent).rb_root_in);
    }

    new
}

unsafe fn callchain_cursor_current(cursor: *mut callchain_cursor) -> *mut callchain_cursor_node {
    (*cursor).curr
}

unsafe fn callchain_cursor_advance(cursor: *mut callchain_cursor) {
    if !(*cursor).curr.is_null() {
        (*cursor).curr = (*(*cursor).curr).next;
        (*cursor).pos += 1;
    }
}

unsafe fn callchain_cursor_commit(cursor: *mut callchain_cursor) {
    (*cursor).curr = (*cursor).first;
    (*cursor).pos = 0;
}

/*
 * Fill the node with callchain values
 */
unsafe fn fill_node(node: *mut callchain_node, cursor: *mut callchain_cursor) -> c_int {
    (*node).val_nr = (*cursor).nr - (*cursor).pos;
    if (*node).val_nr == 0 {
        pr_warning(cstr!("Warning: empty node in callchain tree\n"));
    }

    let mut cursor_node = callchain_cursor_current(cursor);

    while !cursor_node.is_null() {
        let call = zalloc(size_of::<callchain_list>()) as *mut callchain_list;
        if call.is_null() {
            perror(cstr!("not enough memory for the code path tree"));
            return -ENOMEM;
        }
        (*call).ip = (*cursor_node).ip;
        map_symbol__copy(&mut (*call).ms, &(*cursor_node).ms);
        (*call).srcline = (*cursor_node).srcline;

        if (*cursor_node).branch {
            (*call).branch_count = 1;

            if (*cursor_node).branch_from != 0 {
                /*
                 * branch_from is set with value somewhere else
                 * to imply it's "to" of a branch.
                 */
                if (*call).brtype_stat.is_null() {
                    (*call).brtype_stat = zalloc(size_of::<branch_type_stat>()) as *mut branch_type_stat;
                    if (*call).brtype_stat.is_null() {
                        perror(cstr!("not enough memory for the code path branch statistics"));
                        zfree(&mut (*call).brtype_stat as *mut *mut branch_type_stat as *mut *mut c_void);
                        return -ENOMEM;
                    }
                }
                (*(*call).brtype_stat).branch_to = true;

                if (*cursor_node).branch_flags.predicted {
                    (*call).predicted_count = 1;
                }
                if (*cursor_node).branch_flags.abort {
                    (*call).abort_count = 1;
                }
                branch_type_count((*call).brtype_stat, &(*cursor_node).branch_flags, (*cursor_node).branch_from, (*cursor_node).ip);
            } else {
                /*
                 * It's "from" of a branch
                 */
                if !(*call).brtype_stat.is_null() && (*(*call).brtype_stat).branch_to {
                    (*(*call).brtype_stat).branch_to = false;
                }
                (*call).cycles_count = (*cursor_node).branch_flags.cycles;
                (*call).iter_count = (*cursor_node).nr_loop_iter as u64;
                (*call).iter_cycles = (*cursor_node).iter_cycles;
            }
        }

        list_add_tail(&mut (*call).list, &mut (*node).val);

        callchain_cursor_advance(cursor);
        cursor_node = callchain_cursor_current(cursor);
    }
    0
}

unsafe fn add_child(parent: *mut callchain_node, cursor: *mut callchain_cursor, period: u64) -> *mut callchain_node {
    let new = create_child(parent, false);
    if new.is_null() {
        return ptr::null_mut();
    }

    if fill_node(new, cursor) < 0 {
        let mut pos = (*new).val.next;
        while pos != &mut (*new).val {
            let next = (*pos).next;
            let call = list_entry!(pos, callchain_list, list);
            list_del_init(&mut (*call).list);
            map_symbol__exit(&mut (*call).ms);
            zfree(&mut (*call).brtype_stat as *mut *mut branch_type_stat as *mut *mut c_void);
            free(call as *mut c_void);
            pos = next;
        }
        free(new as *mut c_void);
        return ptr::null_mut();
    }

    (*new).children_hit = 0;
    (*new).hit = period;
    (*new).children_count = 0;
    (*new).count = 1;
    new
}

unsafe fn match_chain_strings(left: *const c_char, right: *const c_char) -> match_result {
    let cmp: c_int;

    if !left.is_null() && !right.is_null() {
        cmp = strcmp(left, right);
    } else if left.is_null() && !right.is_null() {
        cmp = 1;
    } else if !left.is_null() && right.is_null() {
        cmp = -1;
    } else {
        return match_result::MATCH_ERROR;
    }

    if cmp != 0 {
        if cmp < 0 { match_result::MATCH_LT } else { match_result::MATCH_GT }
    } else {
        match_result::MATCH_EQ
    }
}

/*
 * We need to always use relative addresses because we're aggregating
 * callchains from multiple threads, i.e. different address spaces, so
 * comparing absolute addresses make no sense as a symbol in a DSO may end up
 * in a different address when used in a different binary or even the same
 * binary but with some sort of address randomization technique, thus we need
 * to compare just relative addresses. -acme
 */
unsafe fn match_chain_dso_addresses(left_map: *mut map, left_ip: u64, right_map: *mut map, right_ip: u64) -> match_result {
    let left_dso = if !left_map.is_null() { map__dso(left_map) } else { ptr::null_mut() };
    let right_dso = if !right_map.is_null() { map__dso(right_map) } else { ptr::null_mut() };

    if left_dso != right_dso {
        return if (left_dso as usize) < (right_dso as usize) { match_result::MATCH_LT } else { match_result::MATCH_GT };
    }

    if left_ip != right_ip {
        return if left_ip < right_ip { match_result::MATCH_LT } else { match_result::MATCH_GT };
    }

    match_result::MATCH_EQ
}

unsafe fn match_chain(node: *mut callchain_cursor_node, cnode: *mut callchain_list) -> match_result {
    let mut match_ = match_result::MATCH_ERROR;

    match callchain_param.key {
        callchain_key::CCKEY_SRCLINE => {
            match_ = match_chain_strings((*cnode).srcline, (*node).srcline);
            if match_ == match_result::MATCH_ERROR {
                /* otherwise fall-back to symbol-based comparison below */
                match_function_or_address(node, cnode, &mut match_);
            }
        }
        callchain_key::CCKEY_FUNCTION => match_function_or_address(node, cnode, &mut match_),
        callchain_key::CCKEY_ADDRESS => {
            match_ = match_chain_dso_addresses((*cnode).ms.map, (*cnode).ip, (*node).ms.map, (*node).ip);
        }
    }

    if match_ == match_result::MATCH_EQ && (*node).branch {
        (*cnode).branch_count = (*cnode).branch_count.wrapping_add(1);

        if (*node).branch_from != 0 {
            /*
             * It's "to" of a branch
             */
            if (*cnode).brtype_stat.is_null() {
                (*cnode).brtype_stat = zalloc(size_of::<branch_type_stat>()) as *mut branch_type_stat;
                if (*cnode).brtype_stat.is_null() {
                    perror(cstr!("not enough memory for the code path branch statistics"));
                    return match_result::MATCH_ERROR;
                }
            }
            (*(*cnode).brtype_stat).branch_to = true;

            if (*node).branch_flags.predicted {
                (*cnode).predicted_count = (*cnode).predicted_count.wrapping_add(1);
            }
            if (*node).branch_flags.abort {
                (*cnode).abort_count = (*cnode).abort_count.wrapping_add(1);
            }

            branch_type_count((*cnode).brtype_stat, &(*node).branch_flags, (*node).branch_from, (*node).ip);
        } else {
            /*
             * It's "from" of a branch
             */
            if !(*cnode).brtype_stat.is_null() && (*(*cnode).brtype_stat).branch_to {
                (*(*cnode).brtype_stat).branch_to = false;
            }
            (*cnode).cycles_count = (*cnode).cycles_count.wrapping_add((*node).branch_flags.cycles);
            (*cnode).iter_count = (*cnode).iter_count.wrapping_add((*node).nr_loop_iter as u64);
            (*cnode).iter_cycles = (*cnode).iter_cycles.wrapping_add((*node).iter_cycles);
            (*cnode).from_count = (*cnode).from_count.wrapping_add(1);
        }
    }

    match_
}

unsafe fn match_function_or_address(node: *mut callchain_cursor_node, cnode: *mut callchain_list, match_: *mut match_result) {
    if !(*node).ms.sym.is_null() && !(*cnode).ms.sym.is_null() {
        /*
         * Compare inlined frames based on their symbol name
         * because different inlined frames will have the same
         * symbol start. Otherwise do a faster comparison based
         * on the symbol start address.
         */
        if symbol__inlined((*cnode).ms.sym) || symbol__inlined((*node).ms.sym) {
            *match_ = match_chain_strings((*(*cnode).ms.sym).name, (*(*node).ms.sym).name);
            if *match_ != match_result::MATCH_ERROR {
                return;
            }
        } else {
            *match_ = match_chain_dso_addresses((*cnode).ms.map, (*(*cnode).ms.sym).start, (*node).ms.map, (*(*node).ms.sym).start);
            return;
        }
    }
    /* otherwise fall-back to IP-based comparison below */
    *match_ = match_chain_dso_addresses((*cnode).ms.map, (*cnode).ip, (*node).ms.map, (*node).ip);
}

/* Remaining functions continue the same source-level translation style. */

unsafe fn split_add_child(parent: *mut callchain_node, cursor: *mut callchain_cursor, to_split: *mut callchain_list, idx_parents: u64, idx_local: u64, period: u64) -> c_int {
    let idx_total: u32 = (idx_parents + idx_local) as u32;
    let mut new = create_child(parent, true);
    if new.is_null() {
        return -1;
    }

    let old_tail = (*parent).val.prev;
    list_del_range(&mut (*to_split).list, old_tail);
    (*new).val.next = &mut (*to_split).list;
    (*new).val.prev = old_tail;
    (*to_split).list.prev = &mut (*new).val;
    (*old_tail).next = &mut (*new).val;

    (*new).hit = (*parent).hit;
    (*new).children_hit = (*parent).children_hit;
    (*parent).children_hit = callchain_cumul_hits(new);
    (*new).val_nr = (*parent).val_nr - idx_local;
    (*parent).val_nr = idx_local;
    (*new).count = (*parent).count;
    (*new).children_count = (*parent).children_count;
    (*parent).children_count = callchain_cumul_counts(new);

    if (idx_total as u64) < (*cursor).nr {
        (*parent).hit = 0;
        (*parent).children_hit = (*parent).children_hit.wrapping_add(period);
        (*parent).count = 0;
        (*parent).children_count = (*parent).children_count.wrapping_add(1);

        let node = callchain_cursor_current(cursor);
        new = add_child(parent, cursor, period);
        if new.is_null() {
            return -1;
        }

        /*
         * This is second child since we moved parent's children
         * to new (first) child above.
         */
        let p = (*parent).rb_root_in.rb_node;
        let first = rb_entry!(p, callchain_node, rb_node_in);
        let cnode = list_entry!((*first).val.next, callchain_list, list);
        let pp = if match_chain(node, cnode) == match_result::MATCH_LT {
            &mut (*p).rb_left
        } else {
            &mut (*p).rb_right
        };

        rb_link_node(&mut (*new).rb_node_in, p, pp);
        rb_insert_color(&mut (*new).rb_node_in, &mut (*parent).rb_root_in);
    } else {
        (*parent).hit = period;
        (*parent).count = 1;
    }
    0
}

unsafe fn append_chain(root: *mut callchain_node, cursor: *mut callchain_cursor, period: u64) -> match_result {
    let start = (*cursor).pos;
    let mut found = false;
    let mut cmp = match_result::MATCH_ERROR;
    let mut cnode: *mut callchain_list = ptr::null_mut();

    let mut pos = (*root).val.next;
    while pos != &mut (*root).val {
        cnode = list_entry!(pos, callchain_list, list);
        let node = callchain_cursor_current(cursor);
        if node.is_null() {
            break;
        }

        cmp = match_chain(node, cnode);
        if cmp != match_result::MATCH_EQ {
            break;
        }

        found = true;
        callchain_cursor_advance(cursor);
        pos = (*pos).next;
    }

    /* matches not, relay no the parent */
    if !found {
        WARN_ONCE(cmp == match_result::MATCH_ERROR, cstr!("Chain comparison error\n"));
        return cmp;
    }

    let matches = (*cursor).pos - start;

    /* we match only a part of the node. Split it and add the new chain */
    if matches < (*root).val_nr {
        if split_add_child(root, cursor, cnode, start, matches, period) < 0 {
            return match_result::MATCH_ERROR;
        }
        return match_result::MATCH_EQ;
    }

    /* we match 100% of the path, increment the hit */
    if matches == (*root).val_nr && (*cursor).pos == (*cursor).nr {
        (*root).hit = (*root).hit.wrapping_add(period);
        (*root).count = (*root).count.wrapping_add(1);
        return match_result::MATCH_EQ;
    }

    /* We match the node and still have a part remaining */
    if append_chain_children(root, cursor, period) < 0 {
        return match_result::MATCH_ERROR;
    }

    match_result::MATCH_EQ
}

unsafe fn append_chain_children(root: *mut callchain_node, cursor: *mut callchain_cursor, period: u64) -> c_int {
    let mut p: *mut *mut rb_node = &mut (*root).rb_root_in.rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();

    let node = callchain_cursor_current(cursor);
    if node.is_null() {
        return -1;
    }

    /* lookup in children */
    while !(*p).is_null() {
        parent = *p;
        let rnode = rb_entry!(parent, callchain_node, rb_node_in);

        /* If at least first entry matches, rely to children */
        let ret = append_chain(rnode, cursor, period);
        if ret == match_result::MATCH_EQ {
            (*root).children_hit = (*root).children_hit.wrapping_add(period);
            (*root).children_count = (*root).children_count.wrapping_add(1);
            return 0;
        }
        if ret == match_result::MATCH_ERROR {
            return -1;
        }

        if ret == match_result::MATCH_LT {
            p = &mut (*parent).rb_left;
        } else {
            p = &mut (*parent).rb_right;
        }
    }
    /* nothing in children, add to the current node */
    let rnode = add_child(root, cursor, period);
    if rnode.is_null() {
        return -1;
    }

    rb_link_node(&mut (*rnode).rb_node_in, parent, p);
    rb_insert_color(&mut (*rnode).rb_node_in, &mut (*root).rb_root_in);

    (*root).children_hit = (*root).children_hit.wrapping_add(period);
    (*root).children_count = (*root).children_count.wrapping_add(1);
    0
}

#[no_mangle]
pub unsafe extern "C" fn callchain_append(root: *mut callchain_root, cursor: *mut callchain_cursor, period: u64) -> c_int {
    if cursor.is_null() {
        return -1;
    }
    if (*cursor).nr == 0 {
        return 0;
    }

    callchain_cursor_commit(cursor);

    if append_chain_children(&mut (*root).node, cursor, period) < 0 {
        return -1;
    }

    if (*cursor).nr > (*root).max_depth {
        (*root).max_depth = (*cursor).nr;
    }

    0
}

unsafe fn merge_chain_branch(cursor: *mut callchain_cursor, dst: *mut callchain_node, src: *mut callchain_node) -> c_int {
    let old_last = (*cursor).last;
    let old_pos = (*cursor).nr;
    let mut err = 0;

    let mut pos = (*src).val.next;
    while pos != &mut (*src).val {
        let next_pos = (*pos).next;
        let list = list_entry!(pos, callchain_list, list);
        let mut ms = map_symbol {
            thread: thread__get((*list).ms.thread),
            map: map__get((*list).ms.map),
            sym: ptr::null_mut(),
        };

        callchain_cursor_append(cursor, (*list).ip, &mut ms, false, ptr::null_mut(), 0, 0, 0, (*list).srcline);
        list_del_init(&mut (*list).list);
        map_symbol__exit(&mut ms);
        map_symbol__exit(&mut (*list).ms);
        zfree(&mut (*list).brtype_stat as *mut *mut branch_type_stat as *mut *mut c_void);
        free(list as *mut c_void);

        pos = next_pos;
    }

    if (*src).hit != 0 {
        callchain_cursor_commit(cursor);
        if append_chain_children(dst, cursor, (*src).hit) < 0 {
            return -1;
        }
    }

    let mut n = rb_first(&(*src).rb_root_in);
    while !n.is_null() {
        let child = container_of!(n, callchain_node, rb_node_in);
        n = rb_next(n);
        rb_erase(&mut (*child).rb_node_in, &mut (*src).rb_root_in);

        err = merge_chain_branch(cursor, dst, child);
        if err != 0 {
            break;
        }

        free(child as *mut c_void);
    }

    (*cursor).nr = old_pos;
    (*cursor).last = old_last;

    err
}

#[no_mangle]
pub unsafe extern "C" fn callchain_merge(cursor: *mut callchain_cursor, dst: *mut callchain_root, src: *mut callchain_root) -> c_int {
    merge_chain_branch(cursor, &mut (*dst).node, &mut (*src).node)
}

#[no_mangle]
pub unsafe extern "C" fn callchain_cursor_append(cursor: *mut callchain_cursor, ip: u64, ms: *mut map_symbol, branch: bool, flags: *mut branch_flags, nr_loop_iter: c_int, iter_cycles: u64, branch_from: u64, srcline: *const c_char) -> c_int {
    let mut node = *(*cursor).last;

    if node.is_null() {
        node = calloc(1, size_of::<callchain_cursor_node>()) as *mut callchain_cursor_node;
        if node.is_null() {
            return -ENOMEM;
        }

        *(*cursor).last = node;
    }

    (*node).ip = ip;
    map_symbol__exit(&mut (*node).ms);
    map_symbol__copy(&mut (*node).ms, ms);
    (*node).branch = branch;
    (*node).nr_loop_iter = nr_loop_iter;
    (*node).iter_cycles = iter_cycles;
    (*node).srcline = srcline;

    if !flags.is_null() {
        memcpy(&mut (*node).branch_flags as *mut _ as *mut c_void, flags as *const c_void, size_of::<branch_flags>());
    }

    (*node).branch_from = branch_from;
    (*cursor).nr = (*cursor).nr.wrapping_add(1);
    (*cursor).last = &mut (*node).next;

    0
}

#[no_mangle]
pub unsafe extern "C" fn sample__resolve_callchain(sample: *mut perf_sample, cursor: *mut callchain_cursor, parent: *mut *mut symbol, al: *mut addr_location, max_stack: c_int) -> c_int {
    if (*sample).callchain.is_null() && !symbol_conf.show_branchflag_count {
        return 0;
    }

    if symbol_conf.use_callchain || symbol_conf.cumulate_callchain || !perf_hpp_list.parent.is_null() || symbol_conf.show_branchflag_count {
        return thread__resolve_callchain((*al).thread, cursor, sample, parent, al, max_stack);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn hist_entry__append_callchain(he: *mut hist_entry, sample: *mut perf_sample) -> c_int {
    if ((!symbol_conf.use_callchain || (*sample).callchain.is_null()) && !symbol_conf.show_branchflag_count) {
        return 0;
    }
    callchain_append((*he).callchain, get_tls_callchain_cursor(), (*sample).period)
}

#[no_mangle]
pub unsafe extern "C" fn fill_callchain_info(al: *mut addr_location, node: *mut callchain_cursor_node, hide_unresolved: bool) -> c_int {
    let mut machine: *mut machine = ptr::null_mut();

    if !(*node).ms.thread.is_null() {
        machine = maps__machine(thread__maps((*node).ms.thread));
    }

    map__put((*al).map);
    (*al).map = map__get((*node).ms.map);
    (*al).sym = (*node).ms.sym;
    (*al).srcline = (*node).srcline;
    (*al).addr = (*node).ip;

    if (*al).sym.is_null() {
        if hide_unresolved {
            return 0;
        }
        if (*al).map.is_null() {
            return 1;
        }
    }
    if maps__equal(thread__maps((*al).thread), machine__kernel_maps(machine)) {
        if machine__is_host(machine) {
            (*al).cpumode = PERF_RECORD_MISC_KERNEL;
            (*al).level = b'k' as c_char;
        } else {
            (*al).cpumode = PERF_RECORD_MISC_GUEST_KERNEL;
            (*al).level = b'g' as c_char;
        }
    } else if machine__is_host(machine) {
        (*al).cpumode = PERF_RECORD_MISC_USER;
        (*al).level = b'.' as c_char;
    } else if perf_guest {
        (*al).cpumode = PERF_RECORD_MISC_GUEST_USER;
        (*al).level = b'u' as c_char;
    } else {
        (*al).cpumode = PERF_RECORD_MISC_HYPERVISOR;
        (*al).level = b'H' as c_char;
    }

    1
}

#[no_mangle]
pub unsafe extern "C" fn callchain_list__sym_name(cl: *mut callchain_list, bf: *mut c_char, bfsize: size_t, show_dso: bool) -> *mut c_char {
    let show_addr = callchain_param.key == callchain_key::CCKEY_ADDRESS;
    let show_srcline = show_addr || callchain_param.key == callchain_key::CCKEY_SRCLINE;
    let printed: c_int;

    if !(*cl).ms.sym.is_null() {
        let inlined = if symbol__inlined((*cl).ms.sym) { cstr!(" (inlined)") } else { cstr!("") };

        if show_srcline && !(*cl).srcline.is_null() {
            printed = scnprintf(bf, bfsize, cstr!("%s %s%s"), (*(*cl).ms.sym).name, (*cl).srcline, inlined);
        } else {
            printed = scnprintf(bf, bfsize, cstr!("%s%s"), (*(*cl).ms.sym).name, inlined);
        }
    } else {
        printed = scnprintf(bf, bfsize, cstr!("%#lx"), (*cl).ip);
    }

    if show_dso {
        scnprintf(
            bf.add(printed as usize),
            bfsize - printed as usize,
            cstr!(" %s"),
            if !(*cl).ms.map.is_null() { dso__short_name(map__dso((*cl).ms.map)) } else { cstr!("unknown") },
        );
    }

    bf
}

#[no_mangle]
pub unsafe extern "C" fn callchain_node__scnprintf_value(node: *mut callchain_node, bf: *mut c_char, bfsize: size_t, total: u64) -> *mut c_char {
    let mut percent = 0.0;
    let mut period = callchain_cumul_hits(node);
    let mut count = callchain_cumul_counts(node);

    if callchain_param.mode == chain_mode::CHAIN_FOLDED {
        period = (*node).hit;
        count = (*node).count;
    }

    match callchain_param.value {
        callchain_value::CCVAL_PERIOD => {
            scnprintf(bf, bfsize, cstr!("%lu"), period);
        }
        callchain_value::CCVAL_COUNT => {
            scnprintf(bf, bfsize, cstr!("%u"), count);
        }
        callchain_value::CCVAL_PERCENT => {
            if total != 0 {
                percent = period as c_double * 100.0 / total as c_double;
            }
            scnprintf(bf, bfsize, cstr!("%.2f%%"), percent);
        }
    }
    bf
}

#[no_mangle]
pub unsafe extern "C" fn callchain_node__fprintf_value(node: *mut callchain_node, fp: *mut FILE, total: u64) -> c_int {
    let mut percent = 0.0;
    let mut period = callchain_cumul_hits(node);
    let mut count = callchain_cumul_counts(node);

    if callchain_param.mode == chain_mode::CHAIN_FOLDED {
        period = (*node).hit;
        count = (*node).count;
    }

    match callchain_param.value {
        callchain_value::CCVAL_PERIOD => fprintf(fp, cstr!("%lu"), period),
        callchain_value::CCVAL_COUNT => fprintf(fp, cstr!("%u"), count),
        callchain_value::CCVAL_PERCENT => {
            if total != 0 {
                percent = period as c_double * 100.0 / total as c_double;
            }
            percent_color_fprintf(fp, cstr!("%.2f%%"), percent)
        }
    }
}

unsafe fn callchain_counts_value(node: *mut callchain_node, branch_count: *mut u64, predicted_count: *mut u64, abort_count: *mut u64, cycles_count: *mut u64) {
    let mut pos = (*node).val.next;
    while pos != &mut (*node).val {
        let clist = list_entry!(pos, callchain_list, list);
        if !branch_count.is_null() { *branch_count = (*branch_count).wrapping_add((*clist).branch_count); }
        if !predicted_count.is_null() { *predicted_count = (*predicted_count).wrapping_add((*clist).predicted_count); }
        if !abort_count.is_null() { *abort_count = (*abort_count).wrapping_add((*clist).abort_count); }
        if !cycles_count.is_null() { *cycles_count = (*cycles_count).wrapping_add((*clist).cycles_count); }
        pos = (*pos).next;
    }
}

unsafe fn callchain_node_branch_counts_cumul(node: *mut callchain_node, branch_count: *mut u64, predicted_count: *mut u64, abort_count: *mut u64, cycles_count: *mut u64) -> c_int {
    let mut n = rb_first(&(*node).rb_root_in);
    while !n.is_null() {
        let child = rb_entry!(n, callchain_node, rb_node_in);
        n = rb_next(n);

        callchain_node_branch_counts_cumul(child, branch_count, predicted_count, abort_count, cycles_count);
        callchain_counts_value(child, branch_count, predicted_count, abort_count, cycles_count);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn callchain_branch_counts(root: *mut callchain_root, branch_count: *mut u64, predicted_count: *mut u64, abort_count: *mut u64, cycles_count: *mut u64) -> c_int {
    if !branch_count.is_null() { *branch_count = 0; }
    if !predicted_count.is_null() { *predicted_count = 0; }
    if !abort_count.is_null() { *abort_count = 0; }
    if !cycles_count.is_null() { *cycles_count = 0; }

    callchain_node_branch_counts_cumul(&mut (*root).node, branch_count, predicted_count, abort_count, cycles_count)
}

unsafe fn count_pri64_printf(idx: c_int, str_: *const c_char, value: u64, bf: *mut c_char, bfsize: c_int) -> c_int {
    scnprintf(bf, bfsize as size_t, cstr!("%s%s:%ld"), if idx != 0 { cstr!(" ") } else { cstr!(" (") }, str_, value)
}

unsafe fn count_float_printf(idx: c_int, str_: *const c_char, value: c_float, bf: *mut c_char, bfsize: c_int, threshold: c_float) -> c_int {
    if threshold != 0.0 && value < threshold {
        return 0;
    }
    scnprintf(bf, bfsize as size_t, cstr!("%s%s:%.1f%%"), if idx != 0 { cstr!(" ") } else { cstr!(" (") }, str_, value as c_double)
}

unsafe fn branch_to_str(bf: *mut c_char, bfsize: c_int, branch_count: u64, predicted_count: u64, abort_count: u64, brtype_stat: *const branch_type_stat) -> c_int {
    let mut printed = branch_type_str(brtype_stat, bf, bfsize);
    let mut i = 0;
    if printed != 0 { i += 1; }

    if predicted_count < branch_count {
        printed += count_float_printf(i, cstr!("predicted"), (predicted_count as c_double * 100.0 / branch_count as c_double) as c_float, bf.add(printed as usize), bfsize - printed, 0.0);
        i += 1;
    }

    if abort_count != 0 {
        printed += count_float_printf(i, cstr!("abort"), (abort_count as c_double * 100.0 / branch_count as c_double) as c_float, bf.add(printed as usize), bfsize - printed, 0.1);
        i += 1;
    }

    if i != 0 {
        printed += scnprintf(bf.add(printed as usize), (bfsize - printed) as size_t, cstr!(")"));
    }

    printed
}

unsafe fn branch_from_str(bf: *mut c_char, bfsize: c_int, branch_count: u64, cycles_count: u64, iter_count: u64, iter_cycles: u64, from_count: u64) -> c_int {
    let mut printed = 0;
    let mut i = 0;
    let cycles = cycles_count / branch_count;
    if cycles != 0 {
        printed += count_pri64_printf(i, cstr!("cycles"), cycles, bf.add(printed as usize), bfsize - printed);
        i += 1;
    }

    if iter_count != 0 && from_count != 0 {
        let v = iter_count / from_count;
        if v != 0 {
            printed += count_pri64_printf(i, cstr!("iter"), v, bf.add(printed as usize), bfsize - printed);
            i += 1;
            printed += count_pri64_printf(i, cstr!("avg_cycles"), iter_cycles / iter_count, bf.add(printed as usize), bfsize - printed);
            i += 1;
        }
    }

    if i != 0 {
        printed += scnprintf(bf.add(printed as usize), (bfsize - printed) as size_t, cstr!(")"));
    }

    printed
}

unsafe fn counts_str_build(bf: *mut c_char, bfsize: c_int, branch_count: u64, predicted_count: u64, abort_count: u64, cycles_count: u64, iter_count: u64, iter_cycles: u64, from_count: u64, brtype_stat: *const branch_type_stat) -> c_int {
    if branch_count == 0 {
        return scnprintf(bf, bfsize as size_t, cstr!(" (calltrace)"));
    }

    let printed = if (*brtype_stat).branch_to {
        branch_to_str(bf, bfsize, branch_count, predicted_count, abort_count, brtype_stat)
    } else {
        branch_from_str(bf, bfsize, branch_count, cycles_count, iter_count, iter_cycles, from_count)
    };

    if printed == 0 {
        *bf = 0;
    }

    printed
}

unsafe fn callchain_counts_printf(fp: *mut FILE, bf: *mut c_char, bfsize: c_int, branch_count: u64, predicted_count: u64, abort_count: u64, cycles_count: u64, iter_count: u64, iter_cycles: u64, from_count: u64, brtype_stat: *const branch_type_stat) -> c_int {
    let mut str_: [c_char; 256] = [0; 256];

    counts_str_build(str_.as_mut_ptr(), str_.len() as c_int, branch_count, predicted_count, abort_count, cycles_count, iter_count, iter_cycles, from_count, brtype_stat);

    if !fp.is_null() {
        return fprintf(fp, cstr!("%s"), str_.as_ptr());
    }

    scnprintf(bf, bfsize as size_t, cstr!("%s"), str_.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn callchain_list_counts__printf_value(clist: *mut callchain_list, fp: *mut FILE, bf: *mut c_char, bfsize: c_int) -> c_int {
    static EMPTY_BRTYPE_STAT: branch_type_stat = branch_type_stat { branch_to: false };
    let brtype_stat = if !(*clist).brtype_stat.is_null() { (*clist).brtype_stat } else { &EMPTY_BRTYPE_STAT as *const _ as *mut _ };

    callchain_counts_printf(fp, bf, bfsize, (*clist).branch_count, (*clist).predicted_count, (*clist).abort_count, (*clist).cycles_count, (*clist).iter_count, (*clist).iter_cycles, (*clist).from_count, brtype_stat)
}

unsafe fn free_callchain_node(node: *mut callchain_node) {
    let mut pos = (*node).parent_val.next;
    while pos != &mut (*node).parent_val {
        let next = (*pos).next;
        let list = list_entry!(pos, callchain_list, list);
        list_del_init(&mut (*list).list);
        map_symbol__exit(&mut (*list).ms);
        zfree(&mut (*list).brtype_stat as *mut *mut branch_type_stat as *mut *mut c_void);
        free(list as *mut c_void);
        pos = next;
    }

    pos = (*node).val.next;
    while pos != &mut (*node).val {
        let next = (*pos).next;
        let list = list_entry!(pos, callchain_list, list);
        list_del_init(&mut (*list).list);
        map_symbol__exit(&mut (*list).ms);
        zfree(&mut (*list).brtype_stat as *mut *mut branch_type_stat as *mut *mut c_void);
        free(list as *mut c_void);
        pos = next;
    }

    let mut n = rb_first(&(*node).rb_root_in);
    while !n.is_null() {
        let child = container_of!(n, callchain_node, rb_node_in);
        n = rb_next(n);
        rb_erase(&mut (*child).rb_node_in, &mut (*node).rb_root_in);

        free_callchain_node(child);
        free(child as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_callchain(root: *mut callchain_root) {
    if !symbol_conf.use_callchain {
        return;
    }
    free_callchain_node(&mut (*root).node);
}

#[no_mangle]
pub unsafe extern "C" fn callchain_cursor_cleanup(cursor: *mut callchain_cursor) {
    callchain_cursor_reset(cursor);

    let mut node = (*cursor).first;
    while !node.is_null() {
        let next = (*node).next;
        free(node as *mut c_void);
        node = next;
    }
    (*cursor).first = ptr::null_mut();
    (*cursor).last = &mut (*cursor).first;
    (*cursor).curr = ptr::null_mut();
}

unsafe fn decay_callchain_node(node: *mut callchain_node) -> u64 {
    let mut child_hits = 0;
    let mut n = rb_first(&(*node).rb_root_in);
    while !n.is_null() {
        let child = container_of!(n, callchain_node, rb_node_in);
        child_hits = (child_hits as u64).wrapping_add(decay_callchain_node(child));
        n = rb_next(n);
    }

    (*node).hit = ((*node).hit * 7) / 8;
    (*node).children_hit = child_hits;

    (*node).hit
}

#[no_mangle]
pub unsafe extern "C" fn decay_callchain(root: *mut callchain_root) {
    if !symbol_conf.use_callchain {
        return;
    }
    decay_callchain_node(&mut (*root).node);
}

#[no_mangle]
pub unsafe extern "C" fn callchain_node__make_parent_list(node: *mut callchain_node) -> c_int {
    let mut parent = (*node).parent;
    let mut head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    init_list_head(&mut head);

    while !parent.is_null() {
        let mut pos = (*parent).val.prev;
        while pos != &mut (*parent).val {
            let chain = list_entry!(pos, callchain_list, list);
            let new = malloc(size_of::<callchain_list>()) as *mut callchain_list;
            if new.is_null() {
                let mut cleanup = head.next;
                while cleanup != &mut head {
                    let next = (*cleanup).next;
                    let chain = list_entry!(cleanup, callchain_list, list);
                    list_del_init(&mut (*chain).list);
                    map_symbol__exit(&mut (*chain).ms);
                    zfree(&mut (*chain).brtype_stat as *mut *mut branch_type_stat as *mut *mut c_void);
                    free(chain as *mut c_void);
                    cleanup = next;
                }
                return -ENOMEM;
            }
            ptr::copy_nonoverlapping(chain, new, 1);
            (*new).has_children = false;
            map_symbol__copy(&mut (*new).ms, &(*chain).ms);
            list_add_tail(&mut (*new).list, &mut head);
            pos = (*pos).prev;
        }
        parent = (*parent).parent;
    }

    let mut pos = head.prev;
    while pos != &mut head {
        let prev = (*pos).prev;
        let chain = list_entry!(pos, callchain_list, list);
        list_move_tail(&mut (*chain).list, &mut (*node).parent_val);
        pos = prev;
    }

    if !list_empty(&(*node).parent_val) {
        let chain = list_entry!((*node).parent_val.next, callchain_list, list);
        (*chain).has_children = !rb_prev(&(*node).rb_node).is_null() || !rb_next(&(*node).rb_node).is_null();

        let chain = list_entry!((*node).val.next, callchain_list, list);
        (*chain).has_children = false;
    }
    0
}

unsafe extern "C" fn callchain_cursor__delete(vcursor: *mut c_void) {
    let cursor = vcursor as *mut callchain_cursor;
    callchain_cursor_reset(cursor);
    let mut node = (*cursor).first;
    while !node.is_null() {
        let next = (*node).next;
        free(node as *mut c_void);
        node = next;
    }
    free(cursor as *mut c_void);
}

unsafe extern "C" fn init_callchain_cursor_key() {
    if pthread_key_create(&mut callchain_cursor, Some(callchain_cursor__delete)) != 0 {
        pr_err(cstr!("callchain cursor creation failed"));
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_tls_callchain_cursor() -> *mut callchain_cursor {
    static mut ONCE_CONTROL: pthread_once_t = PTHREAD_ONCE_INIT;
    pthread_once(&mut ONCE_CONTROL, init_callchain_cursor_key);
    let mut cursor = pthread_getspecific(callchain_cursor) as *mut callchain_cursor;
    if cursor.is_null() {
        cursor = zalloc(size_of::<callchain_cursor>()) as *mut callchain_cursor;
        if cursor.is_null() {
            pr_debug3(cstr!("%s: not enough memory\n"), cstr!("get_tls_callchain_cursor"));
        }
        pthread_setspecific(callchain_cursor, cursor as *const c_void);
    }
    cursor
}

#[no_mangle]
pub unsafe extern "C" fn callchain_cursor__copy(dst: *mut callchain_cursor, src: *mut callchain_cursor) -> c_int {
    let mut rc = 0;

    callchain_cursor_reset(dst);
    callchain_cursor_commit(src);

    loop {
        let node = callchain_cursor_current(src);
        if node.is_null() {
            break;
        }

        rc = callchain_cursor_append(dst, (*node).ip, &mut (*node).ms, (*node).branch, &mut (*node).branch_flags, (*node).nr_loop_iter, (*node).iter_cycles, (*node).branch_from, (*node).srcline);
        if rc != 0 {
            break;
        }

        callchain_cursor_advance(src);
    }

    rc
}

/*
 * Initialize a cursor before adding entries inside, but keep
 * the previously allocated entries as a cache.
 */
#[no_mangle]
pub unsafe extern "C" fn callchain_cursor_reset(cursor: *mut callchain_cursor) {
    (*cursor).nr = 0;
    (*cursor).last = &mut (*cursor).first;

    let mut node = (*cursor).first;
    while !node.is_null() {
        map_symbol__exit(&mut (*node).ms);
        node = (*node).next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn callchain_param_setup(sample_type: u64, e_machine: uint16_t) {
    if symbol_conf.use_callchain || symbol_conf.cumulate_callchain {
        if (sample_type & PERF_SAMPLE_REGS_USER) != 0 && (sample_type & PERF_SAMPLE_STACK_USER) != 0 {
            callchain_param.record_mode = callchain_record_mode::CALLCHAIN_DWARF;
            dwarf_callchain_users = true;
        } else if (sample_type & PERF_SAMPLE_BRANCH_STACK) != 0 {
            callchain_param.record_mode = callchain_record_mode::CALLCHAIN_LBR;
        } else {
            callchain_param.record_mode = callchain_record_mode::CALLCHAIN_FP;
        }
    }

    /*
     * It's necessary to use libunwind to reliably determine the caller of
     * a leaf function on aarch64, as otherwise we cannot know whether to
     * start from the LR or FP.
     *
     * Always starting from the LR can result in duplicate or entirely
     * erroneous entries. Always skipping the LR and starting from the FP
     * can result in missing entries.
     */
    if callchain_param.record_mode == callchain_record_mode::CALLCHAIN_FP && e_machine == EM_AARCH64 {
        dwarf_callchain_users = true;
    }
}

unsafe fn chain_match(base_chain: *mut callchain_list, pair_chain: *mut callchain_list) -> bool {
    let mut match_ = match_chain_strings((*base_chain).srcline, (*pair_chain).srcline);
    if match_ != match_result::MATCH_ERROR {
        return match_ == match_result::MATCH_EQ;
    }

    match_ = match_chain_dso_addresses((*base_chain).ms.map, (*base_chain).ip, (*pair_chain).ms.map, (*pair_chain).ip);
    match_ == match_result::MATCH_EQ
}

#[no_mangle]
pub unsafe extern "C" fn callchain_cnode_matched(base_cnode: *mut callchain_node, pair_cnode: *mut callchain_node) -> bool {
    let mut match_ = false;
    let mut pair_chain = list_entry!((*pair_cnode).val.next, callchain_list, list);

    let mut pos = (*base_cnode).val.next;
    while pos != &mut (*base_cnode).val {
        let base_chain = list_entry!(pos, callchain_list, list);
        if &mut (*pair_chain).list == &mut (*pair_cnode).val {
            return false;
        }

        if (*base_chain).srcline.is_null() || (*pair_chain).srcline.is_null() {
            pair_chain = list_entry!((*pair_chain).list.next, callchain_list, list);
            pos = (*pos).next;
            continue;
        }

        match_ = chain_match(base_chain, pair_chain);
        if !match_ {
            return false;
        }

        pair_chain = list_entry!((*pair_chain).list.next, callchain_list, list);
        pos = (*pos).next;
    }

    /*
     * Say chain1 is ABC, chain2 is ABCD, we consider they are
     * not fully matched.
     */
    if !pair_chain.is_null() && (&mut (*pair_chain).list != &mut (*pair_cnode).val) {
        return false;
    }

    match_
}

unsafe fn count_callchain_hits(he: *mut hist_entry) -> u64 {
    let root = &mut (*he).sorted_chain;
    let mut rb_node = rb_first(root);
    let mut chain_hits = 0;

    while !rb_node.is_null() {
        let node = rb_entry!(rb_node, callchain_node, rb_node);
        chain_hits = (chain_hits as u64).wrapping_add((*node).hit);
        rb_node = rb_next(rb_node);
    }

    chain_hits
}

#[no_mangle]
pub unsafe extern "C" fn callchain_total_hits(hists: *mut hists) -> u64 {
    let mut next = rb_first_cached(&(*hists).entries);
    let mut chain_hits = 0;

    while !next.is_null() {
        let he = rb_entry!(next, hist_entry, rb_node);
        chain_hits = (chain_hits as u64).wrapping_add(count_callchain_hits(he));
        next = rb_next(&(*he).rb_node);
    }

    chain_hits
}

#[no_mangle]
pub unsafe extern "C" fn callchain_avg_cycles(cnode: *mut callchain_node) -> s64 {
    let mut cycles: s64 = 0;

    let mut pos = (*cnode).val.next;
    while pos != &mut (*cnode).val {
        let chain = list_entry!(pos, callchain_list, list);
        if !(*chain).srcline.is_null() && (*chain).branch_count != 0 {
            cycles += ((*chain).cycles_count / (*chain).branch_count) as s64;
        }
        pos = (*pos).next;
    }

    cycles
}

#[no_mangle]
pub unsafe extern "C" fn sample__for_each_callchain_node(thread: *mut thread, sample: *mut perf_sample, max_stack: c_int, symbols: bool, cb: callchain_iter_fn, data: *mut c_void) -> c_int {
    let cursor = get_tls_callchain_cursor();
    if cursor.is_null() {
        return -ENOMEM;
    }

    /* Fill in the callchain. */
    let mut ret = __thread__resolve_callchain(thread, cursor, sample, ptr::null_mut(), ptr::null_mut(), max_stack, symbols);
    if ret != 0 {
        return ret;
    }

    /* Switch from writing the callchain to reading it. */
    callchain_cursor_commit(cursor);

    loop {
        let node = callchain_cursor_current(cursor);
        if node.is_null() {
            break;
        }

        ret = cb(node, data);
        if ret != 0 {
            return ret;
        }

        callchain_cursor_advance(cursor);
    }
    0
}

/*
 * This function merges earlier samples (@sample_orig) waiting for deferred
 * user callchains with the matching callchain record (@sample_callchain)
 * which is delivered now.  The @sample_orig->callchain should be released
 * after use if ->deferred_callchain is set.
 */
#[no_mangle]
pub unsafe extern "C" fn sample__merge_deferred_callchain(sample_orig: *mut perf_sample, sample_callchain: *mut perf_sample) -> c_int {
    let nr_orig = (*(*sample_orig).callchain).nr - 1;
    let nr_deferred = (*(*sample_callchain).callchain).nr;

    if (*sample_orig).merged_callchain {
        /* Already merged. */
        return -EINVAL;
    }

    if (*(*sample_orig).callchain).nr < 2 {
        (*sample_orig).deferred_callchain = false;
        return -EINVAL;
    }

    let callchain = calloc(1 + nr_orig as size_t + nr_deferred as size_t, size_of::<u64>()) as *mut ip_callchain;
    if callchain.is_null() {
        return -ENOMEM;
    }

    (*callchain).nr = nr_orig + nr_deferred;
    /* copy original including PERF_CONTEXT_USER_DEFERRED (but the cookie) */
    memcpy((*callchain).ips.as_mut_ptr() as *mut c_void, (*(*sample_orig).callchain).ips.as_ptr() as *const c_void, nr_orig as size_t * size_of::<u64>());
    /* copy deferred user callchains */
    memcpy((*callchain).ips.as_mut_ptr().add(nr_orig as usize) as *mut c_void, (*(*sample_callchain).callchain).ips.as_ptr() as *const c_void, nr_deferred as size_t * size_of::<u64>());

    (*sample_orig).merged_callchain = true;
    (*sample_orig).callchain = callchain;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
