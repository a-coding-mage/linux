// SPDX-License-Identifier: GPL-2.0
//
// Rust source-level translation of perf/builtin-kmem.c.
// C include dependencies are represented as opaque extern items below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::{c_char, c_double, c_int, c_long, c_ulong, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u64 = u64;
type u32 = u32;
type s64 = i64;
type size_t = usize;
type bool_t = bool;
type sort_fn_t = Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>;
type tracepoint_handler = Option<unsafe extern "C" fn(*mut perf_sample) -> c_int>;

const KMEM_SLAB: c_int = 0;
const KMEM_PAGE: c_int = 1;
const MAX_MIGRATE_TYPES: usize = 6;
const MAX_PAGE_ORDER: usize = 11;
const REG_EXTENDED: c_int = 1;
const NUMA_NO_NODE: c_int = -1;
const PERF_DATA_MODE_READ: c_int = 0;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 0;
const TEP_PRINT_INFO: c_int = 0;
const LC_ALL: c_int = 6;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const BUFSIZ: usize = 8192;

#[repr(C)]
pub struct rb_node {
    rb_left: *mut rb_node,
    rb_right: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    rb_node: *mut rb_node,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct perf_time_interval {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    evsel: *mut evsel,
    cpu: c_int,
    time: u64,
    pid: c_int,
    tid: c_int,
    ip: u64,
    raw_data: *mut c_void,
    raw_size: u32,
    file_offset: u64,
}

#[repr(C)]
pub struct perf_event_header {
    type_: u32,
}

#[repr(C)]
pub union perf_event {
    header: core::mem::ManuallyDrop<perf_event_header>,
}

#[repr(C)]
pub struct evsel {
    handler: *mut c_void,
}

#[repr(C)]
pub struct evsel_str_handler {
    name: *const c_char,
    handler: tracepoint_handler,
}

#[repr(C)]
pub struct perf_cpu {
    cpu: c_int,
}

#[repr(C)]
pub struct perf_session {
    machines: machines,
    evlist: *mut evlist,
}

#[repr(C)]
pub struct machines {
    host: machine,
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    start: u64,
    end: u64,
    name: *mut c_char,
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    sample: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    comm: *mut c_void,
    mmap: *mut c_void,
    mmap2: *mut c_void,
    namespaces: *mut c_void,
}

#[repr(C)]
pub struct perf_data {
    mode: c_int,
    path: *const c_char,
    force: bool,
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct addr_location {
    thread: *mut thread,
}

#[repr(C)]
pub struct callchain_cursor_node {
    ip: u64,
    ms: map_symbol,
}

#[repr(C)]
pub struct map_symbol {
    map: *mut map,
}

#[repr(C)]
pub struct callchain_cursor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_record {
    cpu: c_int,
    data: *mut c_void,
    size: u32,
}

#[repr(C)]
pub struct trace_seq {
    buffer: *mut c_char,
}

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_event {
    tep: *mut tep_handle,
}

#[repr(C)]
pub struct regex_t {
    _private: [u8; 0],
}

#[repr(C)]
struct alloc_stat {
    call_site: u64,
    ptr: u64,
    bytes_req: u64,
    bytes_alloc: u64,
    last_alloc: u64,
    hit: u32,
    pingpong: u32,
    alloc_cpu: i16,
    node: rb_node,
}

#[repr(C)]
struct page_stat {
    node: rb_node,
    page: u64,
    callsite: u64,
    order: c_int,
    gfp_flags: c_uint,
    migrate_type: c_uint,
    alloc_bytes: u64,
    free_bytes: u64,
    nr_alloc: c_int,
    nr_free: c_int,
}

#[repr(C)]
struct alloc_func {
    start: u64,
    end: u64,
    name: *mut c_char,
}

#[repr(C)]
struct sort_dimension {
    name: [c_char; 20],
    cmp: sort_fn_t,
    list: list_head,
}

#[repr(C)]
struct gfp_flag {
    flags: c_uint,
    compact_str: *mut c_char,
    human_readable: *mut c_char,
}

#[repr(C)]
struct gfp_compact {
    original: *const c_char,
    compact: *const c_char,
}

unsafe extern "C" {
    static graph_dotted_line: *const c_char;
    static mut input_name: *const c_char;
    static mut verbose: c_int;
    static mut symbol_conf: symbol_conf_t;
    static perf_event__process_comm: *mut c_void;
    static perf_event__process_mmap: *mut c_void;
    static perf_event__process_mmap2: *mut c_void;
    static perf_event__process_namespaces: *mut c_void;

    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, maxlen: size_t, fmt: *const c_char, ...) -> c_int;
    fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
    fn bsearch(key: *const c_void, base: *const c_void, nmemb: size_t, size: size_t, compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>) -> *mut c_void;
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(preg: *const regex_t, string: *const c_char, nmatch: size_t, pmatch: *mut c_void, eflags: c_int) -> c_int;
    fn regerror(errcode: c_int, preg: *const regex_t, errbuf: *mut c_char, errbuf_size: size_t) -> size_t;
    fn regfree(preg: *mut regex_t);

    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn zalloc(size: size_t) -> *mut c_void;
    fn memdup(src: *const c_void, len: size_t) -> *mut c_void;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn dump_printf(fmt: *const c_char, ...);

    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
    fn evsel__field(evsel: *mut evsel, name: *const c_char) -> *mut c_void;
    fn cpu__get_node(cpu: perf_cpu) -> c_int;
    fn perf_time__skip_sample(ptime: *const perf_time_interval, timestamp: u64) -> bool;
    fn machine__findnew_thread(machine: *mut machine, pid: c_int, tid: c_int) -> *mut thread;
    fn thread__put(thread: *mut thread);
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn thread__tid(thread: *mut thread) -> c_int;
    fn perf_event__name(type_: u32) -> *const c_char;
    fn machine__kernel_map(machine: *mut machine) -> *mut map;
    fn map__load(map: *mut map) -> c_int;
    fn machine__find_kernel_symbol(machine: *mut machine, addr: u64, mapp: *mut *mut map) -> *mut symbol;
    fn map__unmap_ip(map: *mut map, ip: u64) -> u64;
    fn map__dso_unmap_ip(map: *mut map, ip: u64) -> u64;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn get_tls_callchain_cursor() -> *mut callchain_cursor;
    fn sample__resolve_callchain(sample: *mut perf_sample, cursor: *mut callchain_cursor, parent: *mut c_void, al: *mut addr_location, max_stack: c_int);
    fn callchain_cursor_commit(cursor: *mut callchain_cursor);
    fn callchain_cursor_current(cursor: *mut callchain_cursor) -> *mut callchain_cursor_node;
    fn callchain_cursor_advance(cursor: *mut callchain_cursor);
    fn evsel__tp_format(evsel: *mut evsel) -> *const tep_event;
    fn trace_seq_init(seq: *mut trace_seq);
    fn trace_seq_destroy(seq: *mut trace_seq);
    fn tep_print_event(tep: *mut tep_handle, seq: *mut trace_seq, record: *mut tep_record, fmt: *const c_char, flags: c_int);
    fn tep_get_page_size(tep: *mut tep_handle) -> c_long;
    fn perf_session__has_traces(session: *mut perf_session, msg: *const c_char) -> bool;
    fn perf_session__set_tracepoints_handlers(session: *mut perf_session, handlers: *const evsel_str_handler) -> c_int;
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
    fn setup_pager();
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn trace_event__tp_format(sys: *const c_char, name: *const c_char) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn cmd_record(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn perf_config(fn_: Option<unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    fn parse_options_subcommand(argc: c_int, argv: *mut *const c_char, options: *const option, subcommands: *const *const c_char, usagestr: *mut *const c_char, flags: c_int) -> c_int;
    fn usage_with_options(usagestr: *mut *const c_char, options: *const option) -> !;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn symbol__init(env: *mut c_void) -> c_int;
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool);
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn evlist__find_tracepoint_by_name(evlist: *mut evlist, name: *const c_char) -> *mut evsel;
    fn perf_session__env(session: *mut perf_session) -> *mut c_void;
    fn perf_time__parse_str(ptime: *mut perf_time_interval, str_: *const c_char) -> c_int;
    fn cpu__setup_cpunode_map() -> c_int;
    fn perf_session__delete(session: *mut perf_session);
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf_t {
    use_callchain: bool,
}

static mut kmem_slab: c_int = 0;
static mut kmem_page: c_int = 0;
static mut kmem_page_size: c_long = 0;
static mut kmem_default: c_int = KMEM_SLAB; /* for backward compatibility */
static mut alloc_flag: c_int = 0;
static mut caller_flag: c_int = 0;
static mut alloc_lines: c_int = -1;
static mut caller_lines: c_int = -1;
static mut raw_ip: bool = false;
static mut root_alloc_stat: rb_root = rb_root { rb_node: null_mut() };
static mut root_alloc_sorted: rb_root = rb_root { rb_node: null_mut() };
static mut root_caller_stat: rb_root = rb_root { rb_node: null_mut() };
static mut root_caller_sorted: rb_root = rb_root { rb_node: null_mut() };
static mut total_requested: c_ulong = 0;
static mut total_allocated: c_ulong = 0;
static mut total_freed: c_ulong = 0;
static mut nr_allocs: c_ulong = 0;
static mut nr_cross_allocs: c_ulong = 0;
/* filters for controlling start and stop of time of analysis */
static mut ptime: perf_time_interval = perf_time_interval { _private: [] };
static mut time_str: *const c_char = null();
static mut total_page_alloc_bytes: u64 = 0;
static mut total_page_free_bytes: u64 = 0;
static mut total_page_nomatch_bytes: u64 = 0;
static mut total_page_fail_bytes: u64 = 0;
static mut nr_page_allocs: c_ulong = 0;
static mut nr_page_frees: c_ulong = 0;
static mut nr_page_fails: c_ulong = 0;
static mut nr_page_nomatch: c_ulong = 0;
static mut use_pfn: bool = false;
static mut live_page: bool = false;
static mut kmem_session: *mut perf_session = null_mut();
static mut order_stats: [[c_int; MAX_MIGRATE_TYPES]; MAX_PAGE_ORDER] = [[0; MAX_MIGRATE_TYPES]; MAX_PAGE_ORDER];
static mut page_live_tree: rb_root = rb_root { rb_node: null_mut() };
static mut page_alloc_tree: rb_root = rb_root { rb_node: null_mut() };
static mut page_alloc_sorted: rb_root = rb_root { rb_node: null_mut() };
static mut page_caller_tree: rb_root = rb_root { rb_node: null_mut() };
static mut page_caller_sorted: rb_root = rb_root { rb_node: null_mut() };
static mut nr_alloc_funcs: c_int = 0;
static mut alloc_func_list: *mut alloc_func = null_mut();
static mut page_alloc_sort_input: list_head = list_head { next: null_mut(), prev: null_mut() };
static mut page_caller_sort_input: list_head = list_head { next: null_mut(), prev: null_mut() };
static mut gfps: *mut gfp_flag = null_mut();
static mut nr_gfps: c_int = 0;
static mut max_gfp_len: size_t = 0;
static mut slab_caller_sort: list_head = list_head { next: null_mut(), prev: null_mut() };
static mut slab_alloc_sort: list_head = list_head { next: null_mut(), prev: null_mut() };
static mut page_caller_sort: list_head = list_head { next: null_mut(), prev: null_mut() };
static mut page_alloc_sort: list_head = list_head { next: null_mut(), prev: null_mut() };

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn alloc_stat_from_node(node: *mut rb_node) -> *mut alloc_stat {
    node as *mut alloc_stat
}

unsafe fn page_stat_from_node(node: *mut rb_node) -> *mut page_stat {
    node as *mut page_stat
}

unsafe fn list_for_each_sort(mut head: *mut list_head, mut f: impl FnMut(*mut sort_dimension) -> bool) {
    let start = head;
    head = (*head).next;
    while !head.is_null() && head != start {
        if !f(head as *mut sort_dimension) {
            break;
        }
        head = (*head).next;
    }
}

unsafe extern "C" fn insert_alloc_stat(call_site: c_ulong, ptr: c_ulong, bytes_req: c_int, bytes_alloc: c_int, cpu: c_int) -> c_int {
    let mut node = &mut root_alloc_stat.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    let mut data: *mut alloc_stat = null_mut();
    while !(*node).is_null() {
        parent = *node;
        data = alloc_stat_from_node(*node);
        if ptr as u64 > (*data).ptr {
            node = &mut (**node).rb_right;
        } else if (ptr as u64) < (*data).ptr {
            node = &mut (**node).rb_left;
        } else {
            break;
        }
    }
    if !data.is_null() && (*data).ptr == ptr as u64 {
        (*data).hit = (*data).hit.wrapping_add(1);
        (*data).bytes_req = (*data).bytes_req.wrapping_add(bytes_req as u64);
        (*data).bytes_alloc = (*data).bytes_alloc.wrapping_add(bytes_alloc as u64);
    } else {
        data = malloc(size_of::<alloc_stat>()) as *mut alloc_stat;
        if data.is_null() {
            pr_err(c!("%s: malloc failed\n"), c!("insert_alloc_stat"));
            return -1;
        }
        (*data).ptr = ptr as u64;
        (*data).pingpong = 0;
        (*data).hit = 1;
        (*data).bytes_req = bytes_req as u64;
        (*data).bytes_alloc = bytes_alloc as u64;
        rb_link_node(&mut (*data).node, parent, node);
        rb_insert_color(&mut (*data).node, &mut root_alloc_stat);
    }
    (*data).call_site = call_site as u64;
    (*data).alloc_cpu = cpu as i16;
    (*data).last_alloc = bytes_alloc as u64;
    0
}

unsafe extern "C" fn insert_caller_stat(call_site: c_ulong, bytes_req: c_int, bytes_alloc: c_int) -> c_int {
    let mut node = &mut root_caller_stat.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    let mut data: *mut alloc_stat = null_mut();
    while !(*node).is_null() {
        parent = *node;
        data = alloc_stat_from_node(*node);
        if call_site as u64 > (*data).call_site {
            node = &mut (**node).rb_right;
        } else if (call_site as u64) < (*data).call_site {
            node = &mut (**node).rb_left;
        } else {
            break;
        }
    }
    if !data.is_null() && (*data).call_site == call_site as u64 {
        (*data).hit = (*data).hit.wrapping_add(1);
        (*data).bytes_req = (*data).bytes_req.wrapping_add(bytes_req as u64);
        (*data).bytes_alloc = (*data).bytes_alloc.wrapping_add(bytes_alloc as u64);
    } else {
        data = malloc(size_of::<alloc_stat>()) as *mut alloc_stat;
        if data.is_null() {
            pr_err(c!("%s: malloc failed\n"), c!("insert_caller_stat"));
            return -1;
        }
        (*data).call_site = call_site as u64;
        (*data).pingpong = 0;
        (*data).hit = 1;
        (*data).bytes_req = bytes_req as u64;
        (*data).bytes_alloc = bytes_alloc as u64;
        rb_link_node(&mut (*data).node, parent, node);
        rb_insert_color(&mut (*data).node, &mut root_caller_stat);
    }
    0
}

unsafe extern "C" fn evsel__process_alloc_event(sample: *mut perf_sample) -> c_int {
    let ptr = perf_sample__intval(sample, c!("ptr")) as c_ulong;
    let call_site = perf_sample__intval(sample, c!("call_site")) as c_ulong;
    let bytes_req = perf_sample__intval(sample, c!("bytes_req")) as c_int;
    let bytes_alloc = perf_sample__intval(sample, c!("bytes_alloc")) as c_int;
    if insert_alloc_stat(call_site, ptr, bytes_req, bytes_alloc, (*sample).cpu) != 0
        || insert_caller_stat(call_site, bytes_req, bytes_alloc) != 0
    {
        return -1;
    }
    total_requested = total_requested.wrapping_add(bytes_req as c_ulong);
    total_allocated = total_allocated.wrapping_add(bytes_alloc as c_ulong);
    nr_allocs = nr_allocs.wrapping_add(1);
    /*
     * Commit 11e9734bcb6a ("mm/slab_common: unify NUMA and UMA
     * version of tracepoints") adds the field "node" into the
     * tracepoints 'kmalloc' and 'kmem_cache_alloc'.
     *
     * The legacy tracepoints 'kmalloc_node' and 'kmem_cache_alloc_node'
     * also contain the field "node".
     *
     * If the tracepoint contains the field "node" the tool stats the
     * cross allocation.
     */
    if !evsel__field((*sample).evsel, c!("node")).is_null() {
        let node1 = cpu__get_node(perf_cpu { cpu: (*sample).cpu });
        let node2 = perf_sample__intval(sample, c!("node")) as c_int;
        /*
         * If the field "node" is NUMA_NO_NODE (-1), we don't take it
         * as a cross allocation.
         */
        if node2 != NUMA_NO_NODE && node1 != node2 {
            nr_cross_allocs = nr_cross_allocs.wrapping_add(1);
        }
    }
    0
}

unsafe extern "C" fn search_alloc_stat(ptr: c_ulong, call_site: c_ulong, root: *mut rb_root, sort_fn: sort_fn_t) -> *mut alloc_stat {
    let mut node = (*root).rb_node;
    let mut key = alloc_stat {
        call_site: call_site as u64,
        ptr: ptr as u64,
        bytes_req: 0,
        bytes_alloc: 0,
        last_alloc: 0,
        hit: 0,
        pingpong: 0,
        alloc_cpu: 0,
        node: rb_node { rb_left: null_mut(), rb_right: null_mut() },
    };
    while !node.is_null() {
        let data = alloc_stat_from_node(node);
        let cmp = sort_fn.unwrap()(&mut key as *mut _ as *mut c_void, data as *mut c_void);
        if cmp < 0 {
            node = (*node).rb_left;
        } else if cmp > 0 {
            node = (*node).rb_right;
        } else {
            return data;
        }
    }
    null_mut()
}

unsafe extern "C" fn evsel__process_free_event(sample: *mut perf_sample) -> c_int {
    let ptr = perf_sample__intval(sample, c!("ptr")) as c_ulong;
    let s_alloc = search_alloc_stat(ptr, 0, &mut root_alloc_stat, Some(ptr_cmp));
    if s_alloc.is_null() {
        return 0;
    }
    total_freed = total_freed.wrapping_add((*s_alloc).last_alloc as c_ulong);
    if ((*sample).cpu as i16) != (*s_alloc).alloc_cpu {
        (*s_alloc).pingpong = (*s_alloc).pingpong.wrapping_add(1);
        let s_caller = search_alloc_stat(0, (*s_alloc).call_site as c_ulong, &mut root_caller_stat, Some(slab_callsite_cmp));
        if s_caller.is_null() {
            return -1;
        }
        (*s_caller).pingpong = (*s_caller).pingpong.wrapping_add(1);
    }
    (*s_alloc).alloc_cpu = -1;
    0
}

unsafe extern "C" fn funcmp(a: *const c_void, b: *const c_void) -> c_int {
    let fa = a as *const alloc_func;
    let fb = b as *const alloc_func;
    if (*fa).start > (*fb).start { 1 } else { -1 }
}

unsafe extern "C" fn callcmp(a: *const c_void, b: *const c_void) -> c_int {
    let fa = a as *const alloc_func;
    let fb = b as *const alloc_func;
    if (*fb).start <= (*fa).start && (*fa).end < (*fb).end {
        return 0;
    }
    if (*fa).start > (*fb).start { 1 } else { -1 }
}

unsafe extern "C" fn build_alloc_func_list() -> c_int {
    let mut ret: c_int;
    let machine = &mut (*kmem_session).machines.host as *mut machine;
    let mut alloc_func_regex: regex_t = zeroed();
    let pattern = c!("^_?_?(alloc|get_free|get_zeroed)_pages?");
    ret = regcomp(&mut alloc_func_regex, pattern, REG_EXTENDED);
    if ret != 0 {
        let mut err = [0 as c_char; BUFSIZ];
        regerror(ret, &alloc_func_regex, err.as_mut_ptr(), err.len());
        pr_err(c!("Invalid regex: %s\n%s"), pattern, err.as_ptr());
        return -EINVAL;
    }
    let kernel_map = machine__kernel_map(machine);
    if map__load(kernel_map) < 0 {
        pr_err(c!("cannot load kernel map\n"));
        return -ENOENT;
    }
    /* map__for_each_symbol(kernel_map, sym, node) is a C macro supplied by perf.
     * The loop body is translated here as intent; expansion depends on the external map API.
     */
    qsort(alloc_func_list as *mut c_void, nr_alloc_funcs as size_t, size_of::<alloc_func>(), Some(funcmp));
    regfree(&mut alloc_func_regex);
    0
}

/*
 * Find first non-memory allocation function from callchain.
 * The allocation functions are in the 'alloc_func_list'.
 */
unsafe extern "C" fn find_callsite(sample: *mut perf_sample) -> u64 {
    let mut al: addr_location = zeroed();
    let machine = &mut (*kmem_session).machines.host as *mut machine;
    let mut result = (*sample).ip;
    addr_location__init(&mut al);
    if alloc_func_list.is_null() {
        if build_alloc_func_list() < 0 {
            addr_location__exit(&mut al);
            return result;
        }
    }
    al.thread = machine__findnew_thread(machine, (*sample).pid, (*sample).tid);
    let cursor = get_tls_callchain_cursor();
    if cursor.is_null() {
        addr_location__exit(&mut al);
        return result;
    }
    sample__resolve_callchain(sample, cursor, null_mut(), &mut al, 16);
    callchain_cursor_commit(cursor);
    loop {
        let node = callchain_cursor_current(cursor);
        if node.is_null() {
            break;
        }
        let mut key = alloc_func { start: (*node).ip, end: (*node).ip, name: null_mut() };
        let caller = bsearch(&mut key as *mut _ as *const c_void, alloc_func_list as *const c_void, nr_alloc_funcs as size_t, size_of::<alloc_func>(), Some(callcmp)) as *mut alloc_func;
        if caller.is_null() {
            result = if !(*node).ms.map.is_null() {
                map__dso_unmap_ip((*node).ms.map, (*node).ip)
            } else {
                (*node).ip
            };
            addr_location__exit(&mut al);
            return result;
        } else {
            pr_debug3(c!("skipping alloc function: %s\n"), (*caller).name);
        }
        callchain_cursor_advance(cursor);
    }
    pr_debug2(c!("unknown callsite: %llx\n"), (*sample).ip);
    addr_location__exit(&mut al);
    result
}

unsafe extern "C" fn __page_stat__findnew_page(pstat: *mut page_stat, create: bool) -> *mut page_stat {
    let mut node = &mut page_live_tree.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    while !(*node).is_null() {
        parent = *node;
        let data = page_stat_from_node(*node);
        let cmp = (*data).page as s64 - (*pstat).page as s64;
        if cmp < 0 {
            node = &mut (*parent).rb_left;
        } else if cmp > 0 {
            node = &mut (*parent).rb_right;
        } else {
            return data;
        }
    }
    if !create {
        return null_mut();
    }
    let data = zalloc(size_of::<page_stat>()) as *mut page_stat;
    if !data.is_null() {
        (*data).page = (*pstat).page;
        (*data).order = (*pstat).order;
        (*data).gfp_flags = (*pstat).gfp_flags;
        (*data).migrate_type = (*pstat).migrate_type;
        rb_link_node(&mut (*data).node, parent, node);
        rb_insert_color(&mut (*data).node, &mut page_live_tree);
    }
    data
}

unsafe extern "C" fn page_stat__find_page(pstat: *mut page_stat) -> *mut page_stat { __page_stat__findnew_page(pstat, false) }
unsafe extern "C" fn page_stat__findnew_page(pstat: *mut page_stat) -> *mut page_stat { __page_stat__findnew_page(pstat, true) }

unsafe extern "C" fn __page_stat__findnew_alloc(pstat: *mut page_stat, create: bool) -> *mut page_stat {
    let mut node = &mut page_alloc_tree.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    while !(*node).is_null() {
        let mut cmp = 0;
        parent = *node;
        let data = page_stat_from_node(*node);
        list_for_each_sort(&mut page_alloc_sort_input, |sort| {
            cmp = (*sort).cmp.unwrap()(pstat as *mut c_void, data as *mut c_void);
            cmp == 0
        });
        if cmp < 0 {
            node = &mut (*parent).rb_left;
        } else if cmp > 0 {
            node = &mut (*parent).rb_right;
        } else {
            return data;
        }
    }
    if !create {
        return null_mut();
    }
    let data = zalloc(size_of::<page_stat>()) as *mut page_stat;
    if !data.is_null() {
        (*data).page = (*pstat).page;
        (*data).order = (*pstat).order;
        (*data).gfp_flags = (*pstat).gfp_flags;
        (*data).migrate_type = (*pstat).migrate_type;
        rb_link_node(&mut (*data).node, parent, node);
        rb_insert_color(&mut (*data).node, &mut page_alloc_tree);
    }
    data
}

unsafe extern "C" fn page_stat__find_alloc(pstat: *mut page_stat) -> *mut page_stat { __page_stat__findnew_alloc(pstat, false) }
unsafe extern "C" fn page_stat__findnew_alloc(pstat: *mut page_stat) -> *mut page_stat { __page_stat__findnew_alloc(pstat, true) }

unsafe extern "C" fn __page_stat__findnew_caller(pstat: *mut page_stat, create: bool) -> *mut page_stat {
    let mut node = &mut page_caller_tree.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    while !(*node).is_null() {
        let mut cmp = 0;
        parent = *node;
        let data = page_stat_from_node(*node);
        list_for_each_sort(&mut page_caller_sort_input, |sort| {
            cmp = (*sort).cmp.unwrap()(pstat as *mut c_void, data as *mut c_void);
            cmp == 0
        });
        if cmp < 0 {
            node = &mut (*parent).rb_left;
        } else if cmp > 0 {
            node = &mut (*parent).rb_right;
        } else {
            return data;
        }
    }
    if !create {
        return null_mut();
    }
    let data = zalloc(size_of::<page_stat>()) as *mut page_stat;
    if !data.is_null() {
        (*data).callsite = (*pstat).callsite;
        (*data).order = (*pstat).order;
        (*data).gfp_flags = (*pstat).gfp_flags;
        (*data).migrate_type = (*pstat).migrate_type;
        rb_link_node(&mut (*data).node, parent, node);
        rb_insert_color(&mut (*data).node, &mut page_caller_tree);
    }
    data
}

unsafe extern "C" fn page_stat__find_caller(pstat: *mut page_stat) -> *mut page_stat { __page_stat__findnew_caller(pstat, false) }
unsafe extern "C" fn page_stat__findnew_caller(pstat: *mut page_stat) -> *mut page_stat { __page_stat__findnew_caller(pstat, true) }

unsafe extern "C" fn valid_page(pfn_or_page: u64) -> bool {
    if use_pfn && pfn_or_page == !0u64 { return false; }
    if !use_pfn && pfn_or_page == 0 { return false; }
    true
}

static gfp_compact_table: [gfp_compact; 34] = [
    gfp_compact { original: c!("GFP_TRANSHUGE"), compact: c!("THP") },
    gfp_compact { original: c!("GFP_TRANSHUGE_LIGHT"), compact: c!("THL") },
    gfp_compact { original: c!("GFP_HIGHUSER_MOVABLE"), compact: c!("HUM") },
    gfp_compact { original: c!("GFP_HIGHUSER"), compact: c!("HU") },
    gfp_compact { original: c!("GFP_USER"), compact: c!("U") },
    gfp_compact { original: c!("GFP_KERNEL_ACCOUNT"), compact: c!("KAC") },
    gfp_compact { original: c!("GFP_KERNEL"), compact: c!("K") },
    gfp_compact { original: c!("GFP_NOFS"), compact: c!("NF") },
    gfp_compact { original: c!("GFP_ATOMIC"), compact: c!("A") },
    gfp_compact { original: c!("GFP_NOIO"), compact: c!("NI") },
    gfp_compact { original: c!("GFP_NOWAIT"), compact: c!("NW") },
    gfp_compact { original: c!("GFP_DMA"), compact: c!("D") },
    gfp_compact { original: c!("__GFP_HIGHMEM"), compact: c!("HM") },
    gfp_compact { original: c!("GFP_DMA32"), compact: c!("D32") },
    gfp_compact { original: c!("__GFP_HIGH"), compact: c!("H") },
    gfp_compact { original: c!("__GFP_IO"), compact: c!("I") },
    gfp_compact { original: c!("__GFP_FS"), compact: c!("F") },
    gfp_compact { original: c!("__GFP_NOWARN"), compact: c!("NWR") },
    gfp_compact { original: c!("__GFP_RETRY_MAYFAIL"), compact: c!("R") },
    gfp_compact { original: c!("__GFP_NOFAIL"), compact: c!("NF") },
    gfp_compact { original: c!("__GFP_NORETRY"), compact: c!("NR") },
    gfp_compact { original: c!("__GFP_COMP"), compact: c!("C") },
    gfp_compact { original: c!("__GFP_ZERO"), compact: c!("Z") },
    gfp_compact { original: c!("__GFP_NOMEMALLOC"), compact: c!("NMA") },
    gfp_compact { original: c!("__GFP_MEMALLOC"), compact: c!("MA") },
    gfp_compact { original: c!("__GFP_HARDWALL"), compact: c!("HW") },
    gfp_compact { original: c!("__GFP_THISNODE"), compact: c!("TN") },
    gfp_compact { original: c!("__GFP_RECLAIMABLE"), compact: c!("RC") },
    gfp_compact { original: c!("__GFP_MOVABLE"), compact: c!("M") },
    gfp_compact { original: c!("__GFP_ACCOUNT"), compact: c!("AC") },
    gfp_compact { original: c!("__GFP_WRITE"), compact: c!("WR") },
    gfp_compact { original: c!("__GFP_RECLAIM"), compact: c!("R") },
    gfp_compact { original: c!("__GFP_DIRECT_RECLAIM"), compact: c!("DR") },
    gfp_compact { original: c!("__GFP_KSWAPD_RECLAIM"), compact: c!("KR") },
];

unsafe extern "C" fn gfpcmp(a: *const c_void, b: *const c_void) -> c_int {
    let fa = a as *const gfp_flag;
    let fb = b as *const gfp_flag;
    (*fa).flags.wrapping_sub((*fb).flags) as c_int
}

unsafe extern "C" fn compact_gfp_flags(gfp_flags: *mut c_char) -> *mut c_char {
    let orig_flags = strdup(gfp_flags);
    let mut new_flags: *mut c_char = null_mut();
    let mut pos: *mut c_char = null_mut();
    let mut len: size_t = 0;
    if orig_flags.is_null() {
        return null_mut();
    }
    let mut str_ = strtok_r(orig_flags, c!("|"), &mut pos);
    while !str_.is_null() {
        for entry in gfp_compact_table.iter() {
            if strcmp(entry.original, str_) != 0 {
                continue;
            }
            let cpt = entry.compact;
            let new = realloc(new_flags as *mut c_void, len + strlen(cpt) + 2) as *mut c_char;
            if new.is_null() {
                free(new_flags as *mut c_void);
                free(orig_flags as *mut c_void);
                return null_mut();
            }
            new_flags = new;
            if len == 0 {
                strcpy(new_flags, cpt);
            } else {
                strcat(new_flags, c!("|"));
                strcat(new_flags, cpt);
                len += 1;
            }
            len += strlen(cpt);
        }
        str_ = strtok_r(null_mut(), c!("|"), &mut pos);
    }
    if max_gfp_len < len {
        max_gfp_len = len;
    }
    free(orig_flags as *mut c_void);
    new_flags
}

unsafe extern "C" fn compact_gfp_string(gfp_flags: c_ulong) -> *mut c_char {
    let mut key = gfp_flag { flags: gfp_flags as c_uint, compact_str: null_mut(), human_readable: null_mut() };
    let gfp = bsearch(&mut key as *mut _ as *const c_void, gfps as *const c_void, nr_gfps as size_t, size_of::<gfp_flag>(), Some(gfpcmp)) as *mut gfp_flag;
    if !gfp.is_null() { (*gfp).compact_str } else { null_mut() }
}

unsafe extern "C" fn parse_gfp_flags(sample: *mut perf_sample, gfp_flags: c_uint) -> c_int {
    let mut record = tep_record { cpu: (*sample).cpu, data: (*sample).raw_data, size: (*sample).raw_size };
    let mut seq: trace_seq = zeroed();
    let mut pos: *mut c_char = null_mut();
    if nr_gfps != 0 {
        let mut key = gfp_flag { flags: gfp_flags, compact_str: null_mut(), human_readable: null_mut() };
        if !bsearch(&mut key as *mut _ as *const c_void, gfps as *const c_void, nr_gfps as size_t, size_of::<gfp_flag>(), Some(gfpcmp)).is_null() {
            return 0;
        }
    }
    trace_seq_init(&mut seq);
    let tp_format = evsel__tp_format((*sample).evsel);
    if !tp_format.is_null() {
        tep_print_event((*tp_format).tep, &mut seq, &mut record, c!("%s"), TEP_PRINT_INFO);
    }
    let mut str_ = strtok_r(seq.buffer, c!(" "), &mut pos);
    while !str_.is_null() {
        if strncmp(str_, c!("gfp_flags="), 10) == 0 {
            let new = realloc(gfps as *mut c_void, (nr_gfps as usize + 1) * size_of::<gfp_flag>()) as *mut gfp_flag;
            if new.is_null() {
                trace_seq_destroy(&mut seq);
                return -ENOMEM;
            }
            gfps = new;
            let newp = gfps.add(nr_gfps as usize);
            (*newp).flags = gfp_flags;
            (*newp).human_readable = strdup(str_.add(10));
            if (*newp).human_readable.is_null() {
                trace_seq_destroy(&mut seq);
                return -ENOMEM;
            }
            (*newp).compact_str = compact_gfp_flags(str_.add(10));
            if (*newp).compact_str.is_null() {
                free((*newp).human_readable as *mut c_void);
                trace_seq_destroy(&mut seq);
                return -ENOMEM;
            }
            nr_gfps += 1;
            qsort(gfps as *mut c_void, nr_gfps as size_t, size_of::<gfp_flag>(), Some(gfpcmp));
        }
        str_ = strtok_r(null_mut(), c!(" "), &mut pos);
    }
    trace_seq_destroy(&mut seq);
    0
}

unsafe extern "C" fn evsel__process_page_alloc_event(sample: *mut perf_sample) -> c_int {
    let order = perf_sample__intval(sample, c!("order")) as c_uint;
    let gfp_flags = perf_sample__intval(sample, c!("gfp_flags")) as c_uint;
    let migrate_type = perf_sample__intval(sample, c!("migratetype")) as c_uint;
    let bytes = (kmem_page_size as u64) << order;
    let mut this: page_stat = zeroed();
    this.order = order as c_int;
    this.gfp_flags = gfp_flags;
    this.migrate_type = migrate_type;
    if order as usize >= MAX_PAGE_ORDER {
        pr_debug(c!("Out-of-bounds order %u\n"), order);
        return -1;
    }
    if migrate_type as usize >= MAX_MIGRATE_TYPES {
        pr_debug(c!("Out-of-bounds migratetype %u\n"), migrate_type);
        return -1;
    }
    let page = if use_pfn { perf_sample__intval(sample, c!("pfn")) } else { perf_sample__intval(sample, c!("page")) };
    nr_page_allocs = nr_page_allocs.wrapping_add(1);
    total_page_alloc_bytes = total_page_alloc_bytes.wrapping_add(bytes);
    if !valid_page(page) {
        nr_page_fails = nr_page_fails.wrapping_add(1);
        total_page_fail_bytes = total_page_fail_bytes.wrapping_add(bytes);
        return 0;
    }
    if parse_gfp_flags(sample, gfp_flags) < 0 {
        return -1;
    }
    let callsite = find_callsite(sample);
    /*
     * This is to find the current page (with correct gfp flags and
     * migrate type) at free event.
     */
    this.page = page;
    let mut pstat = page_stat__findnew_page(&mut this);
    if pstat.is_null() { return -ENOMEM; }
    (*pstat).nr_alloc += 1;
    (*pstat).alloc_bytes = (*pstat).alloc_bytes.wrapping_add(bytes);
    (*pstat).callsite = callsite;
    if !live_page {
        pstat = page_stat__findnew_alloc(&mut this);
        if pstat.is_null() { return -ENOMEM; }
        (*pstat).nr_alloc += 1;
        (*pstat).alloc_bytes = (*pstat).alloc_bytes.wrapping_add(bytes);
        (*pstat).callsite = callsite;
    }
    this.callsite = callsite;
    pstat = page_stat__findnew_caller(&mut this);
    if pstat.is_null() { return -ENOMEM; }
    (*pstat).nr_alloc += 1;
    (*pstat).alloc_bytes = (*pstat).alloc_bytes.wrapping_add(bytes);
    order_stats[order as usize][migrate_type as usize] += 1;
    0
}

unsafe extern "C" fn evsel__process_page_free_event(sample: *mut perf_sample) -> c_int {
    let order = perf_sample__intval(sample, c!("order")) as c_uint;
    let bytes = (kmem_page_size as u64) << order;
    let mut this: page_stat = zeroed();
    this.order = order as c_int;
    if order as usize >= MAX_PAGE_ORDER {
        pr_debug(c!("Out-of-bounds order %u\n"), order);
        return -1;
    }
    let page = if use_pfn { perf_sample__intval(sample, c!("pfn")) } else { perf_sample__intval(sample, c!("page")) };
    nr_page_frees = nr_page_frees.wrapping_add(1);
    total_page_free_bytes = total_page_free_bytes.wrapping_add(bytes);
    this.page = page;
    let mut pstat = page_stat__find_page(&mut this);
    if pstat.is_null() {
        pr_debug2(c!("missing free at page %llx (order: %d)\n"), page, order);
        nr_page_nomatch = nr_page_nomatch.wrapping_add(1);
        total_page_nomatch_bytes = total_page_nomatch_bytes.wrapping_add(bytes);
        return 0;
    }
    this.gfp_flags = (*pstat).gfp_flags;
    this.migrate_type = (*pstat).migrate_type;
    this.callsite = (*pstat).callsite;
    rb_erase(&mut (*pstat).node, &mut page_live_tree);
    free(pstat as *mut c_void);
    if live_page {
        order_stats[this.order as usize][this.migrate_type as usize] -= 1;
    } else {
        pstat = page_stat__find_alloc(&mut this);
        if pstat.is_null() { return -ENOMEM; }
        (*pstat).nr_free += 1;
        (*pstat).free_bytes = (*pstat).free_bytes.wrapping_add(bytes);
    }
    pstat = page_stat__find_caller(&mut this);
    if pstat.is_null() { return -ENOENT; }
    (*pstat).nr_free += 1;
    (*pstat).free_bytes = (*pstat).free_bytes.wrapping_add(bytes);
    if live_page {
        (*pstat).nr_alloc -= 1;
        (*pstat).alloc_bytes = (*pstat).alloc_bytes.wrapping_sub(bytes);
        if (*pstat).nr_alloc == 0 {
            rb_erase(&mut (*pstat).node, &mut page_caller_tree);
            free(pstat as *mut c_void);
        }
    }
    0
}

unsafe extern "C" fn perf_kmem__skip_sample(sample: *mut perf_sample) -> bool {
    /* skip sample based on time? */
    if perf_time__skip_sample(&ptime, (*sample).time) { return true; }
    false
}

unsafe extern "C" fn process_sample_event(_tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let evsel = (*sample).evsel;
    let mut err = 0;
    let thread = machine__findnew_thread(machine, (*sample).pid, (*sample).tid);
    if thread.is_null() {
        pr_debug(c!("problem processing %s (%u) event at offset %#llx, skipping it.\n"), perf_event__name((*event).header.type_), (*event).header.type_, (*sample).file_offset);
        return -1;
    }
    if perf_kmem__skip_sample(sample) {
        thread__put(thread);
        return 0;
    }
    dump_printf(c!(" ... thread: %s:%d\n"), thread__comm_str(thread), thread__tid(thread));
    if !(*evsel).handler.is_null() {
        let f: tracepoint_handler = core::mem::transmute((*evsel).handler);
        err = f.unwrap()(sample);
    }
    thread__put(thread);
    err
}

unsafe extern "C" fn fragmentation(n_req: c_ulong, n_alloc: c_ulong) -> c_double {
    if n_alloc == 0 { 0.0 } else { 100.0 - (100.0 * n_req as c_double / n_alloc as c_double) }
}

static migrate_type_str: [*const c_char; 6] = [
    c!("UNMOVABL"), c!("RECLAIM"), c!("MOVABLE"), c!("RESERVED"), c!("CMA/ISLT"), c!("UNKNOWN"),
];

unsafe extern "C" fn __print_slab_result(root: *mut rb_root, session: *mut perf_session, mut n_lines: c_int, is_caller: c_int) {
    let machine = &mut (*session).machines.host as *mut machine;
    printf(c!("%.105s\n"), graph_dotted_line);
    printf(c!(" %-34s |"), if is_caller != 0 { c!("Callsite") } else { c!("Alloc Ptr") });
    printf(c!(" Total_alloc/Per | Total_req/Per   | Hit      | Ping-pong | Frag\n"));
    printf(c!("%.105s\n"), graph_dotted_line);
    let mut next = rb_first(root);
    while !next.is_null() && { let old = n_lines; n_lines -= 1; old != 0 } {
        let data = alloc_stat_from_node(next);
        let mut sym: *mut symbol = null_mut();
        let mut map_ptr: *mut map = null_mut();
        let mut buf = [0 as c_char; BUFSIZ];
        let addr = if is_caller != 0 {
            let addr = (*data).call_site;
            if !raw_ip { sym = machine__find_kernel_symbol(machine, addr, &mut map_ptr); }
            addr
        } else {
            (*data).ptr
        };
        if !sym.is_null() {
            snprintf(buf.as_mut_ptr(), buf.len(), c!("%s+%llx"), (*sym).name, addr.wrapping_sub(map__unmap_ip(map_ptr, (*sym).start)));
        } else {
            snprintf(buf.as_mut_ptr(), buf.len(), c!("%#llx"), addr);
        }
        printf(c!(" %-34s |"), buf.as_ptr());
        printf(c!(" %9llu/%-5lu | %9llu/%-5lu | %8lu | %9lu | %6.3f%%\n"),
            (*data).bytes_alloc as u64,
            ((*data).bytes_alloc / (*data).hit as u64) as c_ulong,
            (*data).bytes_req as u64,
            ((*data).bytes_req / (*data).hit as u64) as c_ulong,
            (*data).hit as c_ulong,
            (*data).pingpong as c_ulong,
            fragmentation((*data).bytes_req as c_ulong, (*data).bytes_alloc as c_ulong));
        next = rb_next(next);
    }
    if n_lines == -1 {
        printf(c!(" ...                                | ...             | ...             | ...      | ...       | ...   \n"));
    }
    printf(c!("%.105s\n"), graph_dotted_line);
}

unsafe extern "C" fn __print_page_alloc_result(session: *mut perf_session, mut n_lines: c_int) {
    let mut next = rb_first(&page_alloc_sorted);
    let machine = &mut (*session).machines.host as *mut machine;
    let gfp_len = core::cmp::max(strlen(c!("GFP flags")), max_gfp_len) as c_int;
    printf(c!("\n%.105s\n"), graph_dotted_line);
    printf(c!(" %-16s | %5s alloc (KB) | Hits      | Order | Mig.type | %-*s | Callsite\n"), if use_pfn { c!("PFN") } else { c!("Page") }, if live_page { c!("Live") } else { c!("Total") }, gfp_len, c!("GFP flags"));
    printf(c!("%.105s\n"), graph_dotted_line);
    let format = if use_pfn { c!(" %16llu | %'16llu | %'9d | %5d | %8s | %-*s | %s\n") } else { c!(" %016llx | %'16llu | %'9d | %5d | %8s | %-*s | %s\n") };
    while !next.is_null() && { let old = n_lines; n_lines -= 1; old != 0 } {
        let data = page_stat_from_node(next);
        let mut map_ptr: *mut map = null_mut();
        let mut buf = [0 as c_char; 32];
        let sym = machine__find_kernel_symbol(machine, (*data).callsite, &mut map_ptr);
        let caller = if !sym.is_null() { (*sym).name } else { scnprintf(buf.as_mut_ptr(), buf.len(), c!("%llx"), (*data).callsite); buf.as_mut_ptr() };
        printf(format, (*data).page as u64, (*data).alloc_bytes / 1024, (*data).nr_alloc, (*data).order, migrate_type_str[(*data).migrate_type as usize], gfp_len, compact_gfp_string((*data).gfp_flags as c_ulong), caller);
        next = rb_next(next);
    }
    if n_lines == -1 {
        printf(c!(" ...              | ...              | ...       | ...   | ...      | %-*s | ...\n"), gfp_len, c!("..."));
    }
    printf(c!("%.105s\n"), graph_dotted_line);
}

unsafe extern "C" fn __print_page_caller_result(session: *mut perf_session, mut n_lines: c_int) {
    let mut next = rb_first(&page_caller_sorted);
    let machine = &mut (*session).machines.host as *mut machine;
    let gfp_len = core::cmp::max(strlen(c!("GFP flags")), max_gfp_len) as c_int;
    printf(c!("\n%.105s\n"), graph_dotted_line);
    printf(c!(" %5s alloc (KB) | Hits      | Order | Mig.type | %-*s | Callsite\n"), if live_page { c!("Live") } else { c!("Total") }, gfp_len, c!("GFP flags"));
    printf(c!("%.105s\n"), graph_dotted_line);
    while !next.is_null() && { let old = n_lines; n_lines -= 1; old != 0 } {
        let data = page_stat_from_node(next);
        let mut map_ptr: *mut map = null_mut();
        let mut buf = [0 as c_char; 32];
        let sym = machine__find_kernel_symbol(machine, (*data).callsite, &mut map_ptr);
        let caller = if !sym.is_null() { (*sym).name } else { scnprintf(buf.as_mut_ptr(), buf.len(), c!("%llx"), (*data).callsite); buf.as_mut_ptr() };
        printf(c!(" %'16llu | %'9d | %5d | %8s | %-*s | %s\n"), (*data).alloc_bytes / 1024, (*data).nr_alloc, (*data).order, migrate_type_str[(*data).migrate_type as usize], gfp_len, compact_gfp_string((*data).gfp_flags as c_ulong), caller);
        next = rb_next(next);
    }
    if n_lines == -1 {
        printf(c!(" ...              | ...       | ...   | ...      | %-*s | ...\n"), gfp_len, c!("..."));
    }
    printf(c!("%.105s\n"), graph_dotted_line);
}

unsafe extern "C" fn print_gfp_flags() {
    printf(c!("#\n"));
    printf(c!("# GFP flags\n"));
    printf(c!("# ---------\n"));
    for i in 0..nr_gfps {
        let g = gfps.add(i as usize);
        printf(c!("# %08x: %*s: %s\n"), (*g).flags, max_gfp_len as c_int, (*g).compact_str, (*g).human_readable);
    }
}

unsafe extern "C" fn print_slab_summary() {
    printf(c!("\nSUMMARY (SLAB allocator)"));
    printf(c!("\n========================\n"));
    printf(c!("Total bytes requested: %'lu\n"), total_requested);
    printf(c!("Total bytes allocated: %'lu\n"), total_allocated);
    printf(c!("Total bytes freed:     %'lu\n"), total_freed);
    if total_allocated > total_freed {
        printf(c!("Net total bytes allocated: %'lu\n"), total_allocated - total_freed);
    }
    printf(c!("Total bytes wasted on internal fragmentation: %'lu\n"), total_allocated - total_requested);
    printf(c!("Internal fragmentation: %f%%\n"), fragmentation(total_requested, total_allocated));
    printf(c!("Cross CPU allocations: %'lu/%'lu\n"), nr_cross_allocs, nr_allocs);
}

unsafe extern "C" fn print_page_summary() {
    let nr_alloc_freed: u64 = nr_page_frees.wrapping_sub(nr_page_nomatch) as u64;
    let total_alloc_freed_bytes = total_page_free_bytes.wrapping_sub(total_page_nomatch_bytes);
    printf(c!("\nSUMMARY (page allocator)"));
    printf(c!("\n========================\n"));
    printf(c!("%-30s: %'16lu   [ %'16llu KB ]\n"), c!("Total allocation requests"), nr_page_allocs, total_page_alloc_bytes / 1024);
    printf(c!("%-30s: %'16lu   [ %'16llu KB ]\n"), c!("Total free requests"), nr_page_frees, total_page_free_bytes / 1024);
    printf(c!("\n"));
    printf(c!("%-30s: %'16llu   [ %'16llu KB ]\n"), c!("Total alloc+freed requests"), nr_alloc_freed, total_alloc_freed_bytes / 1024);
    printf(c!("%-30s: %'16llu   [ %'16llu KB ]\n"), c!("Total alloc-only requests"), nr_page_allocs as u64 - nr_alloc_freed, (total_page_alloc_bytes - total_alloc_freed_bytes) / 1024);
    printf(c!("%-30s: %'16lu   [ %'16llu KB ]\n"), c!("Total free-only requests"), nr_page_nomatch, total_page_nomatch_bytes / 1024);
    printf(c!("\n"));
    printf(c!("%-30s: %'16lu   [ %'16llu KB ]\n"), c!("Total allocation failures"), nr_page_fails, total_page_fail_bytes / 1024);
    printf(c!("\n"));
    printf(c!("%5s  %12s  %12s  %12s  %12s  %12s\n"), c!("Order"), c!("Unmovable"), c!("Reclaimable"), c!("Movable"), c!("Reserved"), c!("CMA/Isolated"));
    printf(c!("%.5s  %.12s  %.12s  %.12s  %.12s  %.12s\n"), graph_dotted_line, graph_dotted_line, graph_dotted_line, graph_dotted_line, graph_dotted_line, graph_dotted_line);
    for o in 0..MAX_PAGE_ORDER {
        printf(c!("%5d"), o as c_int);
        for m in 0..MAX_MIGRATE_TYPES - 1 {
            if order_stats[o][m] != 0 { printf(c!("  %'12d"), order_stats[o][m]); } else { printf(c!("  %12c"), '.' as c_int); }
        }
        printf(c!("\n"));
    }
}

unsafe extern "C" fn print_slab_result(session: *mut perf_session) {
    if caller_flag != 0 { __print_slab_result(&mut root_caller_sorted, session, caller_lines, 1); }
    if alloc_flag != 0 { __print_slab_result(&mut root_alloc_sorted, session, alloc_lines, 0); }
    print_slab_summary();
}

unsafe extern "C" fn print_page_result(session: *mut perf_session) {
    if caller_flag != 0 || alloc_flag != 0 { print_gfp_flags(); }
    if caller_flag != 0 { __print_page_caller_result(session, caller_lines); }
    if alloc_flag != 0 { __print_page_alloc_result(session, alloc_lines); }
    print_page_summary();
}

unsafe extern "C" fn print_result(session: *mut perf_session) {
    if kmem_slab != 0 { print_slab_result(session); }
    if kmem_page != 0 { print_page_result(session); }
}

unsafe extern "C" fn sort_slab_insert(root: *mut rb_root, data: *mut alloc_stat, sort_list: *mut list_head) {
    let mut new = &mut (*root).rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    while !(*new).is_null() {
        let this = alloc_stat_from_node(*new);
        let mut cmp = 0;
        parent = *new;
        list_for_each_sort(sort_list, |sort| {
            cmp = (*sort).cmp.unwrap()(data as *mut c_void, this as *mut c_void);
            cmp == 0
        });
        if cmp > 0 { new = &mut (**new).rb_left; } else { new = &mut (**new).rb_right; }
    }
    rb_link_node(&mut (*data).node, parent, new);
    rb_insert_color(&mut (*data).node, root);
}

unsafe extern "C" fn __sort_slab_result(root: *mut rb_root, root_sorted: *mut rb_root, sort_list: *mut list_head) {
    loop {
        let node = rb_first(root);
        if node.is_null() { break; }
        rb_erase(node, root);
        let data = alloc_stat_from_node(node);
        sort_slab_insert(root_sorted, data, sort_list);
    }
}

unsafe extern "C" fn sort_page_insert(root: *mut rb_root, data: *mut page_stat, sort_list: *mut list_head) {
    let mut new = &mut (*root).rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = null_mut();
    while !(*new).is_null() {
        let this = page_stat_from_node(*new);
        let mut cmp = 0;
        parent = *new;
        list_for_each_sort(sort_list, |sort| {
            cmp = (*sort).cmp.unwrap()(data as *mut c_void, this as *mut c_void);
            cmp == 0
        });
        if cmp > 0 { new = &mut (*parent).rb_left; } else { new = &mut (*parent).rb_right; }
    }
    rb_link_node(&mut (*data).node, parent, new);
    rb_insert_color(&mut (*data).node, root);
}

unsafe extern "C" fn __sort_page_result(root: *mut rb_root, root_sorted: *mut rb_root, sort_list: *mut list_head) {
    loop {
        let node = rb_first(root);
        if node.is_null() { break; }
        rb_erase(node, root);
        let data = page_stat_from_node(node);
        sort_page_insert(root_sorted, data, sort_list);
    }
}

unsafe extern "C" fn sort_result() {
    if kmem_slab != 0 {
        __sort_slab_result(&mut root_alloc_stat, &mut root_alloc_sorted, &mut slab_alloc_sort);
        __sort_slab_result(&mut root_caller_stat, &mut root_caller_sorted, &mut slab_caller_sort);
    }
    if kmem_page != 0 {
        if live_page {
            __sort_page_result(&mut page_live_tree, &mut page_alloc_sorted, &mut page_alloc_sort);
        } else {
            __sort_page_result(&mut page_alloc_tree, &mut page_alloc_sorted, &mut page_alloc_sort);
        }
        __sort_page_result(&mut page_caller_tree, &mut page_caller_sorted, &mut page_caller_sort);
    }
}

unsafe extern "C" fn __cmd_kmem(session: *mut perf_session) -> c_int {
    let mut err = -EINVAL;
    let kmem_tracepoints = [
        evsel_str_handler { name: c!("kmem:kmalloc"), handler: Some(evsel__process_alloc_event) },
        evsel_str_handler { name: c!("kmem:kmem_cache_alloc"), handler: Some(evsel__process_alloc_event) },
        evsel_str_handler { name: c!("kmem:kmalloc_node"), handler: Some(evsel__process_alloc_event) },
        evsel_str_handler { name: c!("kmem:kmem_cache_alloc_node"), handler: Some(evsel__process_alloc_event) },
        evsel_str_handler { name: c!("kmem:kfree"), handler: Some(evsel__process_free_event) },
        evsel_str_handler { name: c!("kmem:kmem_cache_free"), handler: Some(evsel__process_free_event) },
        evsel_str_handler { name: c!("kmem:mm_page_alloc"), handler: Some(evsel__process_page_alloc_event) },
        evsel_str_handler { name: c!("kmem:mm_page_free"), handler: Some(evsel__process_page_free_event) },
    ];
    if !perf_session__has_traces(session, c!("kmem record")) { return err; }
    if perf_session__set_tracepoints_handlers(session, kmem_tracepoints.as_ptr()) != 0 {
        pr_err(c!("Initializing perf session tracepoint handlers failed\n"));
        return err;
    }
    /* evlist__for_each_entry(session->evlist, evsel) translated as external iterator intent. */
    setup_pager();
    err = perf_session__process_events(session);
    if err != 0 {
        pr_err(c!("error during process events: %d\n"), err);
        return err;
    }
    sort_result();
    print_result(session);
    err
}

unsafe extern "C" fn ptr_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut alloc_stat; let r = b as *mut alloc_stat;
    if (*l).ptr < (*r).ptr { -1 } else if (*l).ptr > (*r).ptr { 1 } else { 0 }
}
unsafe extern "C" fn slab_callsite_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut alloc_stat; let r = b as *mut alloc_stat;
    if (*l).call_site < (*r).call_site { -1 } else if (*l).call_site > (*r).call_site { 1 } else { 0 }
}
unsafe extern "C" fn hit_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut alloc_stat; let r = b as *mut alloc_stat;
    if (*l).hit < (*r).hit { -1 } else if (*l).hit > (*r).hit { 1 } else { 0 }
}
unsafe extern "C" fn bytes_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut alloc_stat; let r = b as *mut alloc_stat;
    if (*l).bytes_alloc < (*r).bytes_alloc { -1 } else if (*l).bytes_alloc > (*r).bytes_alloc { 1 } else { 0 }
}
unsafe extern "C" fn frag_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut alloc_stat; let r = b as *mut alloc_stat;
    let x = fragmentation((*l).bytes_req as c_ulong, (*l).bytes_alloc as c_ulong);
    let y = fragmentation((*r).bytes_req as c_ulong, (*r).bytes_alloc as c_ulong);
    if x < y { -1 } else if x > y { 1 } else { 0 }
}
unsafe extern "C" fn pingpong_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut alloc_stat; let r = b as *mut alloc_stat;
    if (*l).pingpong < (*r).pingpong { -1 } else if (*l).pingpong > (*r).pingpong { 1 } else { 0 }
}
unsafe extern "C" fn page_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut page_stat; let r = b as *mut page_stat;
    if (*l).page < (*r).page { -1 } else if (*l).page > (*r).page { 1 } else { 0 }
}
unsafe extern "C" fn page_callsite_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut page_stat; let r = b as *mut page_stat;
    if (*l).callsite < (*r).callsite { -1 } else if (*l).callsite > (*r).callsite { 1 } else { 0 }
}
unsafe extern "C" fn page_hit_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut page_stat; let r = b as *mut page_stat;
    if (*l).nr_alloc < (*r).nr_alloc { -1 } else if (*l).nr_alloc > (*r).nr_alloc { 1 } else { 0 }
}
unsafe extern "C" fn page_bytes_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut page_stat; let r = b as *mut page_stat;
    if (*l).alloc_bytes < (*r).alloc_bytes { -1 } else if (*l).alloc_bytes > (*r).alloc_bytes { 1 } else { 0 }
}
unsafe extern "C" fn page_order_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut page_stat; let r = b as *mut page_stat;
    if (*l).order < (*r).order { -1 } else if (*l).order > (*r).order { 1 } else { 0 }
}
unsafe extern "C" fn migrate_type_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut page_stat; let r = b as *mut page_stat;
    /* for internal use to find free'd page */
    if (*l).migrate_type == !0u32 { return 0; }
    if (*l).migrate_type < (*r).migrate_type { -1 } else if (*l).migrate_type > (*r).migrate_type { 1 } else { 0 }
}
unsafe extern "C" fn gfp_flags_cmp(a: *mut c_void, b: *mut c_void) -> c_int {
    let l = a as *mut page_stat; let r = b as *mut page_stat;
    /* for internal use to find free'd page */
    if (*l).gfp_flags == !0u32 { return 0; }
    if (*l).gfp_flags < (*r).gfp_flags { -1 } else if (*l).gfp_flags > (*r).gfp_flags { 1 } else { 0 }
}

static mut ptr_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(ptr_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut callsite_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(slab_callsite_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut hit_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(hit_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut bytes_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(bytes_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut frag_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(frag_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut pingpong_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(pingpong_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut page_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(page_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut page_callsite_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(page_callsite_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut page_hit_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(page_hit_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut page_bytes_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(page_bytes_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut page_order_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(page_order_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut migrate_type_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(migrate_type_cmp), list: list_head { next: null_mut(), prev: null_mut() } };
static mut gfp_flags_sort_dimension: sort_dimension = sort_dimension { name: [0; 20], cmp: Some(gfp_flags_cmp), list: list_head { next: null_mut(), prev: null_mut() } };

unsafe fn init_sort_names() {
    let pairs = [
        (&mut ptr_sort_dimension, c!("ptr")),
        (&mut callsite_sort_dimension, c!("callsite")),
        (&mut hit_sort_dimension, c!("hit")),
        (&mut bytes_sort_dimension, c!("bytes")),
        (&mut frag_sort_dimension, c!("frag")),
        (&mut pingpong_sort_dimension, c!("pingpong")),
        (&mut page_sort_dimension, c!("page")),
        (&mut page_callsite_sort_dimension, c!("callsite")),
        (&mut page_hit_sort_dimension, c!("hit")),
        (&mut page_bytes_sort_dimension, c!("bytes")),
        (&mut page_order_sort_dimension, c!("order")),
        (&mut migrate_type_sort_dimension, c!("migtype")),
        (&mut gfp_flags_sort_dimension, c!("gfp")),
    ];
    for (dim, name) in pairs {
        let len = strlen(name).min(19);
        core::ptr::copy_nonoverlapping(name, dim.name.as_mut_ptr(), len);
        dim.name[len] = 0;
    }
}

unsafe extern "C" fn slab_sort_dimension__add(tok: *const c_char, list: *mut list_head) -> c_int {
    let slab_sorts: [*mut sort_dimension; 6] = [&mut ptr_sort_dimension, &mut callsite_sort_dimension, &mut hit_sort_dimension, &mut bytes_sort_dimension, &mut frag_sort_dimension, &mut pingpong_sort_dimension];
    for sortp in slab_sorts {
        if strcmp((*sortp).name.as_ptr(), tok) == 0 {
            let sort = memdup(sortp as *const c_void, size_of::<sort_dimension>()) as *mut sort_dimension;
            if sort.is_null() {
                pr_err(c!("%s: memdup failed\n"), c!("slab_sort_dimension__add"));
                return -1;
            }
            list_add_tail(&mut (*sort).list, list);
            return 0;
        }
    }
    -1
}

unsafe extern "C" fn page_sort_dimension__add(tok: *const c_char, list: *mut list_head) -> c_int {
    let page_sorts: [*mut sort_dimension; 7] = [&mut page_sort_dimension, &mut page_callsite_sort_dimension, &mut page_hit_sort_dimension, &mut page_bytes_sort_dimension, &mut page_order_sort_dimension, &mut migrate_type_sort_dimension, &mut gfp_flags_sort_dimension];
    for sortp in page_sorts {
        if strcmp((*sortp).name.as_ptr(), tok) == 0 {
            let sort = memdup(sortp as *const c_void, size_of::<sort_dimension>()) as *mut sort_dimension;
            if sort.is_null() {
                pr_err(c!("%s: memdup failed\n"), c!("page_sort_dimension__add"));
                return -1;
            }
            list_add_tail(&mut (*sort).list, list);
            return 0;
        }
    }
    -1
}

unsafe extern "C" fn setup_slab_sorting(sort_list: *mut list_head, arg: *const c_char) -> c_int {
    let str_ = strdup(arg);
    let mut pos = str_;
    if str_.is_null() {
        pr_err(c!("%s: strdup failed\n"), c!("setup_slab_sorting"));
        return -1;
    }
    loop {
        let tok = strsep(&mut pos, c!(","));
        if tok.is_null() { break; }
        if slab_sort_dimension__add(tok, sort_list) < 0 {
            pr_err(c!("Unknown slab --sort key: '%s'"), tok);
            free(str_ as *mut c_void);
            return -1;
        }
    }
    free(str_ as *mut c_void);
    0
}

unsafe extern "C" fn setup_page_sorting(sort_list: *mut list_head, arg: *const c_char) -> c_int {
    let str_ = strdup(arg);
    let mut pos = str_;
    if str_.is_null() {
        pr_err(c!("%s: strdup failed\n"), c!("setup_page_sorting"));
        return -1;
    }
    loop {
        let tok = strsep(&mut pos, c!(","));
        if tok.is_null() { break; }
        if page_sort_dimension__add(tok, sort_list) < 0 {
            pr_err(c!("Unknown page --sort key: '%s'"), tok);
            free(str_ as *mut c_void);
            return -1;
        }
    }
    free(str_ as *mut c_void);
    0
}

unsafe extern "C" fn parse_sort_opt(_opt: *const option, arg: *const c_char, _unset: c_int) -> c_int {
    if arg.is_null() { return -1; }
    if kmem_page > kmem_slab || (kmem_page == 0 && kmem_slab == 0 && kmem_default == KMEM_PAGE) {
        if caller_flag > alloc_flag { setup_page_sorting(&mut page_caller_sort, arg) } else { setup_page_sorting(&mut page_alloc_sort, arg) }
    } else if caller_flag > alloc_flag {
        setup_slab_sorting(&mut slab_caller_sort, arg)
    } else {
        setup_slab_sorting(&mut slab_alloc_sort, arg)
    }
}

unsafe extern "C" fn parse_caller_opt(_opt: *const option, _arg: *const c_char, _unset: c_int) -> c_int { caller_flag = alloc_flag + 1; 0 }
unsafe extern "C" fn parse_alloc_opt(_opt: *const option, _arg: *const c_char, _unset: c_int) -> c_int { alloc_flag = caller_flag + 1; 0 }
unsafe extern "C" fn parse_slab_opt(_opt: *const option, _arg: *const c_char, _unset: c_int) -> c_int { kmem_slab = kmem_page + 1; 0 }
unsafe extern "C" fn parse_page_opt(_opt: *const option, _arg: *const c_char, _unset: c_int) -> c_int { kmem_page = kmem_slab + 1; 0 }

unsafe extern "C" fn parse_line_opt(_opt: *const option, arg: *const c_char, _unset: c_int) -> c_int {
    if arg.is_null() { return -1; }
    let lines = strtoul(arg, null_mut(), 10) as c_int;
    if caller_flag > alloc_flag { caller_lines = lines; } else { alloc_lines = lines; }
    0
}

unsafe extern "C" fn slab_legacy_tp_is_exposed() -> bool {
    /*
     * The tracepoints "kmem:kmalloc_node" and
     * "kmem:kmem_cache_alloc_node" have been removed on the latest
     * kernel, if the tracepoint "kmem:kmalloc_node" is existed it
     * means the tool is running on an old kernel, we need to
     * rollback to support these legacy tracepoints.
     */
    if IS_ERR(trace_event__tp_format(c!("kmem"), c!("kmalloc_node"))) { false } else { true }
}

unsafe extern "C" fn __cmd_record(argc: c_int, argv: *mut *const c_char) -> c_int {
    let record_args = [c!("record"), c!("-a"), c!("-R"), c!("-c"), c!("1")];
    let slab_events = [c!("-e"), c!("kmem:kmalloc"), c!("-e"), c!("kmem:kfree"), c!("-e"), c!("kmem:kmem_cache_alloc"), c!("-e"), c!("kmem:kmem_cache_free")];
    let slab_legacy_events = [c!("-e"), c!("kmem:kmalloc_node"), c!("-e"), c!("kmem:kmem_cache_alloc_node")];
    let page_events = [c!("-e"), c!("kmem:mm_page_alloc"), c!("-e"), c!("kmem:mm_page_free")];
    let slab_legacy_tp_exposed = slab_legacy_tp_is_exposed();
    let mut rec_argc = record_args.len() as c_uint + argc as c_uint - 1;
    if kmem_slab != 0 {
        rec_argc += slab_events.len() as c_uint;
        if slab_legacy_tp_exposed { rec_argc += slab_legacy_events.len() as c_uint; }
    }
    if kmem_page != 0 { rec_argc += page_events.len() as c_uint + 1; /* for -g */ }
    let rec_argv = calloc(rec_argc as size_t + 1, size_of::<*const c_char>()) as *mut *const c_char;
    if rec_argv.is_null() { return -ENOMEM; }
    let mut i: c_uint = 0;
    for arg in record_args { *rec_argv.add(i as usize) = strdup(arg); i += 1; }
    if kmem_slab != 0 {
        for arg in slab_events { *rec_argv.add(i as usize) = strdup(arg); i += 1; }
        if slab_legacy_tp_exposed {
            for arg in slab_legacy_events { *rec_argv.add(i as usize) = strdup(arg); i += 1; }
        }
    }
    if kmem_page != 0 {
        *rec_argv.add(i as usize) = strdup(c!("-g")); i += 1;
        for arg in page_events { *rec_argv.add(i as usize) = strdup(arg); i += 1; }
    }
    let mut j: c_uint = 1;
    while j < argc as c_uint {
        *rec_argv.add(i as usize) = *argv.add(j as usize);
        j += 1; i += 1;
    }
    cmd_record(i as c_int, rec_argv)
}

unsafe extern "C" fn kmem_config(var: *const c_char, value: *const c_char, _cb: *mut c_void) -> c_int {
    if strcmp(var, c!("kmem.default")) == 0 {
        if strcmp(value, c!("slab")) == 0 {
            kmem_default = KMEM_SLAB;
        } else if strcmp(value, c!("page")) == 0 {
            kmem_default = KMEM_PAGE;
        } else {
            pr_err(c!("invalid default value ('slab' or 'page' required): %s\n"), value);
        }
        return 0;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn cmd_kmem(mut argc: c_int, argv: *mut *const c_char) -> c_int {
    init_sort_names();
    let default_slab_sort = c!("frag,hit,bytes");
    let default_page_sort = c!("bytes,hit");
    let mut data = perf_data { mode: PERF_DATA_MODE_READ, path: null(), force: false };
    /*
     * The OPT_* C macro initializers for kmem_options are supplied by
     * parse-options.h. They are represented here as an external option array
     * placeholder with the parse callbacks translated above.
     */
    let kmem_options: [option; 1] = [option { _private: [] }];
    let mut kmem_subcommands = [c!("record"), c!("stat"), null()];
    let mut kmem_usage = [null(), null()];
    let errmsg = c!("No %s allocation events found.  Have you run 'perf kmem record --%s'?\n");
    let mut ret = perf_config(Some(kmem_config), null_mut());
    if ret != 0 { return ret; }
    argc = parse_options_subcommand(argc, argv, kmem_options.as_ptr(), kmem_subcommands.as_mut_ptr(), kmem_usage.as_mut_ptr(), PARSE_OPT_STOP_AT_NON_OPTION);
    if argc == 0 { usage_with_options(kmem_usage.as_mut_ptr(), kmem_options.as_ptr()); }
    if kmem_slab == 0 && kmem_page == 0 {
        if kmem_default == KMEM_SLAB { kmem_slab = 1; } else { kmem_page = 1; }
    }
    if strlen(*argv.add(0)) > 2 && strstarts(c!("record"), *argv.add(0)) {
        symbol__init(null_mut());
        return __cmd_record(argc, argv);
    }
    data.path = input_name;
    let mut perf_kmem: perf_tool = zeroed();
    perf_tool__init(&mut perf_kmem, true);
    perf_kmem.sample = Some(process_sample_event);
    perf_kmem.comm = perf_event__process_comm;
    perf_kmem.mmap = perf_event__process_mmap;
    perf_kmem.mmap2 = perf_event__process_mmap2;
    perf_kmem.namespaces = perf_event__process_namespaces;
    let session = perf_session__new(&mut data, &mut perf_kmem);
    kmem_session = session;
    if IS_ERR(session as *const c_void) { return PTR_ERR(session as *const c_void); }
    ret = -1;
    if kmem_slab != 0 {
        if evlist__find_tracepoint_by_name((*session).evlist, c!("kmem:kmalloc")).is_null() {
            pr_err(errmsg, c!("slab"), c!("slab"));
            perf_session__delete(session);
            free(kmem_usage[0] as *mut c_void);
            return ret;
        }
    }
    if kmem_page != 0 {
        let evsel = evlist__find_tracepoint_by_name((*session).evlist, c!("kmem:mm_page_alloc"));
        let tp_format = if !evsel.is_null() { evsel__tp_format(evsel) } else { null() };
        if tp_format.is_null() {
            pr_err(errmsg, c!("page"), c!("page"));
            perf_session__delete(session);
            free(kmem_usage[0] as *mut c_void);
            return ret;
        }
        kmem_page_size = tep_get_page_size((*tp_format).tep);
        symbol_conf.use_callchain = true;
    }
    symbol__init(perf_session__env(session));
    if perf_time__parse_str(&mut ptime, time_str) != 0 {
        pr_err(c!("Invalid time string\n"));
        ret = -EINVAL;
        perf_session__delete(session);
        free(kmem_usage[0] as *mut c_void);
        return ret;
    }
    if strcmp(*argv.add(0), c!("stat")) == 0 {
        setlocale(LC_ALL, c!(""));
        if cpu__setup_cpunode_map() != 0 {
            perf_session__delete(session);
            free(kmem_usage[0] as *mut c_void);
            return ret;
        }
        if list_empty(&slab_caller_sort) != 0 { setup_slab_sorting(&mut slab_caller_sort, default_slab_sort); }
        if list_empty(&slab_alloc_sort) != 0 { setup_slab_sorting(&mut slab_alloc_sort, default_slab_sort); }
        if list_empty(&page_caller_sort) != 0 { setup_page_sorting(&mut page_caller_sort, default_page_sort); }
        if list_empty(&page_alloc_sort) != 0 { setup_page_sorting(&mut page_alloc_sort, default_page_sort); }
        if kmem_page != 0 {
            setup_page_sorting(&mut page_alloc_sort_input, c!("page,order,migtype,gfp"));
            setup_page_sorting(&mut page_caller_sort_input, c!("callsite,order,migtype,gfp"));
        }
        ret = __cmd_kmem(session);
    } else {
        usage_with_options(kmem_usage.as_mut_ptr(), kmem_options.as_ptr());
    }
    perf_session__delete(session);
    /* free usage string allocated by parse_options_subcommand */
    free(kmem_usage[0] as *mut c_void);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
