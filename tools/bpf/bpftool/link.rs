// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2020 Facebook */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type bool_ = bool;

const PERF_HW_CACHE_LEN: usize = 128;

const IF_NAMESIZE: usize = 16;
const PATH_MAX: usize = 4096;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const BPF_OBJ_LINK: c_int = 4;
const BPF_F_KPROBE_MULTI_RETURN: __u64 = 1;
const BPF_F_UPROBE_MULTI_RETURN: __u64 = 1;

/* External constants from Linux/libbpf headers. */
const PERF_TYPE_HARDWARE: usize = 0;
const PERF_TYPE_SOFTWARE: usize = 1;
const PERF_TYPE_TRACEPOINT: usize = 2;
const PERF_TYPE_HW_CACHE: usize = 3;
const PERF_TYPE_RAW: usize = 4;
const PERF_TYPE_BREAKPOINT: usize = 5;
const PERF_TYPE_MAX: usize = 6;

const PERF_COUNT_HW_CPU_CYCLES: usize = 0;
const PERF_COUNT_HW_INSTRUCTIONS: usize = 1;
const PERF_COUNT_HW_CACHE_REFERENCES: usize = 2;
const PERF_COUNT_HW_CACHE_MISSES: usize = 3;
const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: usize = 4;
const PERF_COUNT_HW_BRANCH_MISSES: usize = 5;
const PERF_COUNT_HW_BUS_CYCLES: usize = 6;
const PERF_COUNT_HW_STALLED_CYCLES_FRONTEND: usize = 7;
const PERF_COUNT_HW_STALLED_CYCLES_BACKEND: usize = 8;
const PERF_COUNT_HW_REF_CPU_CYCLES: usize = 9;
const PERF_COUNT_HW_MAX: usize = 10;

const PERF_COUNT_SW_CPU_CLOCK: usize = 0;
const PERF_COUNT_SW_TASK_CLOCK: usize = 1;
const PERF_COUNT_SW_PAGE_FAULTS: usize = 2;
const PERF_COUNT_SW_CONTEXT_SWITCHES: usize = 3;
const PERF_COUNT_SW_CPU_MIGRATIONS: usize = 4;
const PERF_COUNT_SW_PAGE_FAULTS_MIN: usize = 5;
const PERF_COUNT_SW_PAGE_FAULTS_MAJ: usize = 6;
const PERF_COUNT_SW_ALIGNMENT_FAULTS: usize = 7;
const PERF_COUNT_SW_EMULATION_FAULTS: usize = 8;
const PERF_COUNT_SW_DUMMY: usize = 9;
const PERF_COUNT_SW_BPF_OUTPUT: usize = 10;
const PERF_COUNT_SW_CGROUP_SWITCHES: usize = 11;
const PERF_COUNT_SW_MAX: usize = 12;

const PERF_COUNT_HW_CACHE_L1D: usize = 0;
const PERF_COUNT_HW_CACHE_L1I: usize = 1;
const PERF_COUNT_HW_CACHE_LL: usize = 2;
const PERF_COUNT_HW_CACHE_DTLB: usize = 3;
const PERF_COUNT_HW_CACHE_ITLB: usize = 4;
const PERF_COUNT_HW_CACHE_BPU: usize = 5;
const PERF_COUNT_HW_CACHE_NODE: usize = 6;
const PERF_COUNT_HW_CACHE_MAX: usize = 7;

const PERF_COUNT_HW_CACHE_OP_READ: usize = 0;
const PERF_COUNT_HW_CACHE_OP_WRITE: usize = 1;
const PERF_COUNT_HW_CACHE_OP_PREFETCH: usize = 2;
const PERF_COUNT_HW_CACHE_OP_MAX: usize = 3;

const PERF_COUNT_HW_CACHE_RESULT_ACCESS: usize = 0;
const PERF_COUNT_HW_CACHE_RESULT_MISS: usize = 1;
const PERF_COUNT_HW_CACHE_RESULT_MAX: usize = 2;

const BPF_CGROUP_ITER_ORDER_UNSPEC: __u32 = 0;
const BPF_CGROUP_ITER_SELF_ONLY: __u32 = 1;
const BPF_CGROUP_ITER_DESCENDANTS_PRE: __u32 = 2;
const BPF_CGROUP_ITER_DESCENDANTS_POST: __u32 = 3;
const BPF_CGROUP_ITER_ANCESTORS_UP: __u32 = 4;

const BPF_LINK_TYPE_RAW_TRACEPOINT: __u32 = 1;
const BPF_LINK_TYPE_TRACING: __u32 = 2;
const BPF_LINK_TYPE_CGROUP: __u32 = 3;
const BPF_LINK_TYPE_ITER: __u32 = 4;
const BPF_LINK_TYPE_NETNS: __u32 = 5;
const BPF_LINK_TYPE_XDP: __u32 = 6;
const BPF_LINK_TYPE_PERF_EVENT: __u32 = 7;
const BPF_LINK_TYPE_KPROBE_MULTI: __u32 = 8;
const BPF_LINK_TYPE_STRUCT_OPS: __u32 = 9;
const BPF_LINK_TYPE_NETFILTER: __u32 = 10;
const BPF_LINK_TYPE_TCX: __u32 = 11;
const BPF_LINK_TYPE_UPROBE_MULTI: __u32 = 12;
const BPF_LINK_TYPE_NETKIT: __u32 = 13;
const BPF_LINK_TYPE_SOCKMAP: __u32 = 14;
const BPF_LINK_TYPE_TRACING_MULTI: __u32 = 15;

const BPF_PERF_EVENT_EVENT: __u32 = 0;
const BPF_PERF_EVENT_TRACEPOINT: __u32 = 1;
const BPF_PERF_EVENT_KPROBE: __u32 = 2;
const BPF_PERF_EVENT_KRETPROBE: __u32 = 3;
const BPF_PERF_EVENT_UPROBE: __u32 = 4;
const BPF_PERF_EVENT_URETPROBE: __u32 = 5;

const NFPROTO_INET: usize = 1;
const NFPROTO_IPV4: usize = 2;
const NFPROTO_ARP: usize = 3;
const NFPROTO_NETDEV: usize = 5;
const NFPROTO_BRIDGE: usize = 7;
const NFPROTO_IPV6: usize = 10;
const NF_INET_PRE_ROUTING: usize = 0;
const NF_INET_LOCAL_IN: usize = 1;
const NF_INET_FORWARD: usize = 2;
const NF_INET_LOCAL_OUT: usize = 3;
const NF_INET_POST_ROUTING: usize = 4;
const NF_ARP_IN: usize = 0;
const NF_ARP_OUT: usize = 1;

#[repr(C)]
struct hashmap;
#[repr(C)]
struct json_writer_t;
#[repr(C)]
struct hashmap_entry {
    key: *const c_void,
    pvalue: *mut c_void,
}
#[repr(C)]
struct kernel_config_option {
    name: *const c_char,
}
#[repr(C)]
struct kernel_sym {
    address: __u64,
    name: *const c_char,
    module: [c_char; 128],
}
#[repr(C)]
struct dump_data {
    sym_count: __u32,
    sym_mapping: *mut kernel_sym,
}
#[repr(C)]
struct bpf_prog_info {
    type_: __u32,
}

#[repr(C)]
struct bpf_link_info {
    type_: __u32,
    id: __u32,
    prog_id: __u32,
    raw_tracepoint: raw_tracepoint_info,
    tracing: tracing_info,
    cgroup: cgroup_info,
    iter: iter_info,
    netns: netns_info,
    netfilter: netfilter_info,
    tcx: if_attach_info,
    netkit: if_attach_info,
    sockmap: sockmap_info,
    xdp: xdp_info,
    struct_ops: struct_ops_info,
    kprobe_multi: kprobe_multi_info,
    uprobe_multi: uprobe_multi_info,
    tracing_multi: tracing_multi_info,
    perf_event: perf_event_info,
}

#[repr(C)]
struct raw_tracepoint_info {
    tp_name: __u64,
    tp_name_len: __u32,
    cookie: __u64,
}
#[repr(C)]
struct tracing_info {
    attach_type: __u32,
    target_obj_id: __u32,
    target_btf_id: __u32,
    cookie: __u64,
}
#[repr(C)]
struct cgroup_info {
    cgroup_id: __u64,
    attach_type: __u32,
}
#[repr(C)]
struct iter_info {
    target_name: __u64,
    target_name_len: __u32,
    map: iter_map_info,
    task: iter_task_info,
    cgroup: iter_cgroup_info,
}
#[repr(C)]
struct iter_map_info {
    map_id: __u32,
}
#[repr(C)]
struct iter_task_info {
    tid: __u32,
    pid: __u32,
}
#[repr(C)]
struct iter_cgroup_info {
    cgroup_id: __u64,
    order: __u32,
}
#[repr(C)]
struct netns_info {
    netns_ino: __u32,
    attach_type: __u32,
}
#[repr(C)]
struct netfilter_info {
    pf: __u32,
    hooknum: __u32,
    priority: c_int,
    flags: __u32,
}
#[repr(C)]
struct if_attach_info {
    ifindex: __u32,
    attach_type: __u32,
}
#[repr(C)]
struct sockmap_info {
    map_id: __u32,
    attach_type: __u32,
}
#[repr(C)]
struct xdp_info {
    ifindex: __u32,
}
#[repr(C)]
struct struct_ops_info {
    map_id: __u32,
}
#[repr(C)]
struct kprobe_multi_info {
    flags: __u64,
    count: __u32,
    missed: __u64,
    addrs: __u64,
    cookies: __u64,
}
#[repr(C)]
struct uprobe_multi_info {
    flags: __u64,
    path: __u64,
    path_size: __u32,
    count: __u32,
    pid: __u32,
    offsets: __u64,
    ref_ctr_offsets: __u64,
    cookies: __u64,
}
#[repr(C)]
struct tracing_multi_info {
    attach_type: __u32,
    count: __u32,
    btf_obj_id: __u32,
    ids: __u64,
    addrs: __u64,
    cookies: __u64,
}
#[repr(C)]
struct perf_event_info {
    type_: __u32,
    kprobe: perf_event_kprobe,
    uprobe: perf_event_uprobe,
    tracepoint: perf_event_tracepoint,
    event: perf_event_event,
}
#[repr(C)]
struct perf_event_kprobe {
    addr: __u64,
    func_name: __u64,
    name_len: __u32,
    offset: __u32,
    missed: __u64,
    cookie: __u64,
}
#[repr(C)]
struct perf_event_uprobe {
    file_name: __u64,
    name_len: __u32,
    offset: __u64,
    cookie: __u64,
    ref_ctr_offset: __u64,
}
#[repr(C)]
struct perf_event_tracepoint {
    tp_name: __u64,
    name_len: __u32,
    cookie: __u64,
}
#[repr(C)]
struct perf_event_event {
    type_: __u32,
    config: __u64,
    cookie: __u64,
}
#[repr(C)]
struct cmd {
    cmd: *const c_char,
    func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}
#[repr(C)]
struct addr_cookie {
    addr: __u64,
    cookie: __u64,
}

static mut link_table: *mut hashmap = null_mut();
static mut dd: dump_data = dump_data {
    sym_count: 0,
    sym_mapping: null_mut(),
};

static perf_type_name: [*const c_char; PERF_TYPE_MAX] = [
    b"hardware\0".as_ptr() as *const c_char,
    b"software\0".as_ptr() as *const c_char,
    b"tracepoint\0".as_ptr() as *const c_char,
    b"hw-cache\0".as_ptr() as *const c_char,
    b"raw\0".as_ptr() as *const c_char,
    b"breakpoint\0".as_ptr() as *const c_char,
];

static event_symbols_hw: [*const c_char; PERF_COUNT_HW_MAX] = [
    b"cpu-cycles\0".as_ptr() as *const c_char,
    b"instructions\0".as_ptr() as *const c_char,
    b"cache-references\0".as_ptr() as *const c_char,
    b"cache-misses\0".as_ptr() as *const c_char,
    b"branch-instructions\0".as_ptr() as *const c_char,
    b"branch-misses\0".as_ptr() as *const c_char,
    b"bus-cycles\0".as_ptr() as *const c_char,
    b"stalled-cycles-frontend\0".as_ptr() as *const c_char,
    b"stalled-cycles-backend\0".as_ptr() as *const c_char,
    b"ref-cycles\0".as_ptr() as *const c_char,
];

static event_symbols_sw: [*const c_char; PERF_COUNT_SW_MAX] = [
    b"cpu-clock\0".as_ptr() as *const c_char,
    b"task-clock\0".as_ptr() as *const c_char,
    b"page-faults\0".as_ptr() as *const c_char,
    b"context-switches\0".as_ptr() as *const c_char,
    b"cpu-migrations\0".as_ptr() as *const c_char,
    b"minor-faults\0".as_ptr() as *const c_char,
    b"major-faults\0".as_ptr() as *const c_char,
    b"alignment-faults\0".as_ptr() as *const c_char,
    b"emulation-faults\0".as_ptr() as *const c_char,
    b"dummy\0".as_ptr() as *const c_char,
    b"bpf-output\0".as_ptr() as *const c_char,
    b"cgroup-switches\0".as_ptr() as *const c_char,
];

static evsel__hw_cache: [*const c_char; PERF_COUNT_HW_CACHE_MAX] = [
    b"L1-dcache\0".as_ptr() as *const c_char,
    b"L1-icache\0".as_ptr() as *const c_char,
    b"LLC\0".as_ptr() as *const c_char,
    b"dTLB\0".as_ptr() as *const c_char,
    b"iTLB\0".as_ptr() as *const c_char,
    b"branch\0".as_ptr() as *const c_char,
    b"node\0".as_ptr() as *const c_char,
];

static evsel__hw_cache_op: [*const c_char; PERF_COUNT_HW_CACHE_OP_MAX] = [
    b"load\0".as_ptr() as *const c_char,
    b"store\0".as_ptr() as *const c_char,
    b"prefetch\0".as_ptr() as *const c_char,
];

static evsel__hw_cache_result: [*const c_char; PERF_COUNT_HW_CACHE_RESULT_MAX] = [
    b"refs\0".as_ptr() as *const c_char,
    b"misses\0".as_ptr() as *const c_char,
];

static pf2name: [*const c_char; 11] = [
    null(),
    b"inet\0".as_ptr() as *const c_char,
    b"ip\0".as_ptr() as *const c_char,
    b"arp\0".as_ptr() as *const c_char,
    null(),
    b"netdev\0".as_ptr() as *const c_char,
    null(),
    b"bridge\0".as_ptr() as *const c_char,
    null(),
    null(),
    b"ip6\0".as_ptr() as *const c_char,
];
static inethook2name: [*const c_char; 5] = [
    b"prerouting\0".as_ptr() as *const c_char,
    b"input\0".as_ptr() as *const c_char,
    b"forward\0".as_ptr() as *const c_char,
    b"output\0".as_ptr() as *const c_char,
    b"postrouting\0".as_ptr() as *const c_char,
];
static arphook2name: [*const c_char; 2] = [
    b"input\0".as_ptr() as *const c_char,
    b"output\0".as_ptr() as *const c_char,
];

unsafe extern "C" {
    static mut errno: c_int;
    static mut json_wtr: *mut json_writer_t;
    static mut json_output: bool;
    static mut show_pinned: bool;
    static mut refs_table: *mut hashmap;
    static mut bin_name: *const c_char;
    static mut hash_fn_for_key_as_id: *mut c_void;
    static mut equal_fn_for_key_as_id: *mut c_void;

    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;

    fn is_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
    fn p_err(format: *const c_char, ...);
    fn open_obj_pinned_any(path: *const c_char, expected_type: c_int, info: *mut c_void) -> c_int;
    fn bpf_link_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_prog_get_fd_by_id(id: c_int) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, len: *mut __u32) -> c_int;
    fn bpf_link_get_info_by_fd(fd: c_int, info: *mut bpf_link_info, len: *mut __u32) -> c_int;
    fn bpf_link_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_link_detach(fd: c_int) -> c_int;
    fn libbpf_bpf_link_type_str(type_: __u32) -> *const c_char;
    fn libbpf_bpf_attach_type_str(attach_type: __u32) -> *const c_char;
    fn libbpf_bpf_prog_type_str(type_: __u32) -> *const c_char;
    fn jsonw_uint_field(w: *mut json_writer_t, name: *const c_char, val: __u64);
    fn jsonw_int_field(w: *mut json_writer_t, name: *const c_char, val: c_int);
    fn jsonw_lluint_field(w: *mut json_writer_t, name: *const c_char, val: __u64);
    fn jsonw_string_field(w: *mut json_writer_t, name: *const c_char, val: *const c_char);
    fn jsonw_bool_field(w: *mut json_writer_t, name: *const c_char, val: bool);
    fn jsonw_name(w: *mut json_writer_t, name: *const c_char);
    fn jsonw_start_array(w: *mut json_writer_t);
    fn jsonw_end_array(w: *mut json_writer_t);
    fn jsonw_start_object(w: *mut json_writer_t);
    fn jsonw_end_object(w: *mut json_writer_t);
    fn jsonw_string(w: *mut json_writer_t, val: *const c_char);
    fn jsonw_null(w: *mut json_writer_t);
    fn hashmap__new(hash_fn: *mut c_void, equal_fn: *mut c_void, ctx: *mut c_void) -> *mut hashmap;
    fn hashmap__empty(map: *mut hashmap) -> bool;
    fn hashmap__for_each_key_entry(map: *mut hashmap, entry: *mut *mut hashmap_entry, key: __u32) -> bool;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn build_pinned_obj_table(map: *mut hashmap, obj_type: c_int);
    fn delete_pinned_obj_table(map: *mut hashmap);
    fn build_obj_refs_table(table: *mut *mut hashmap, obj_type: c_int);
    fn delete_obj_refs_table(table: *mut hashmap);
    fn emit_obj_refs_json(table: *mut hashmap, id: __u32, w: *mut json_writer_t);
    fn emit_obj_refs_plain(table: *mut hashmap, id: __u32, prefix: *const c_char);
    fn do_pin_any(argc: c_int, argv: *mut *mut c_char, parse_fd: unsafe fn(*mut c_int, *mut *mut *mut c_char) -> c_int) -> c_int;
    fn read_kernel_config(options: *mut kernel_config_option, cnt: size_t, values: *mut *mut c_char, err: *mut c_void) -> c_int;
    fn kernel_syms_load(dd: *mut dump_data);
    fn kernel_syms_destroy(dd: *mut dump_data);
    fn kernel_syms_search(dd: *mut dump_data, addr: __u64) -> *mut kernel_sym;
    fn cmd_select(cmds: *const cmd, argc: c_int, argv: *mut *mut c_char, help: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int) -> c_int;
}

unsafe fn u64_to_ptr<T>(val: __u64) -> *mut T {
    val as usize as *mut T
}

unsafe fn ptr_to_u64<T>(ptr: *mut T) -> __u64 {
    ptr as usize as __u64
}

unsafe fn perf_event_name(array: &[*const c_char], id: __u64) -> *const c_char {
    let mut event_str: *const c_char = null();
    if (id as usize) < array.len() {
        event_str = array[id as usize];
    }
    event_str
}

unsafe fn NEXT_ARGP(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    *argc -= 1;
    *argv = (*argv).add(1);
}

unsafe fn BAD_ARG() -> c_int {
    p_err(b"bad argument\0".as_ptr() as *const c_char);
    -1
}

unsafe fn link_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int {
    let mut fd: c_int;

    if is_prefix(**argv, b"id\0".as_ptr() as *const c_char) {
        let id: c_uint;
        let mut endptr: *mut c_char = null_mut();

        NEXT_ARGP(argc, argv);
        id = strtoul(**argv, &mut endptr, 0) as c_uint;
        if *endptr != 0 {
            p_err(b"can't parse %s as ID\0".as_ptr() as *const c_char, **argv);
            return -1;
        }
        NEXT_ARGP(argc, argv);

        fd = bpf_link_get_fd_by_id(id);
        if fd < 0 {
            p_err(
                b"failed to get link with ID %u: %s\0".as_ptr() as *const c_char,
                id,
                strerror(errno),
            );
        }
        return fd;
    } else if is_prefix(**argv, b"pinned\0".as_ptr() as *const c_char) {
        let path: *mut c_char;

        NEXT_ARGP(argc, argv);
        path = **argv;
        NEXT_ARGP(argc, argv);

        return open_obj_pinned_any(path, BPF_OBJ_LINK, null_mut());
    }

    p_err(b"expected 'id' or 'pinned', got: '%s'?\0".as_ptr() as *const c_char, **argv);
    -1
}

unsafe fn show_link_header_json(info: *mut bpf_link_info, wtr: *mut json_writer_t) {
    let link_type_str: *const c_char;

    jsonw_uint_field(wtr, b"id\0".as_ptr() as *const c_char, (*info).id as __u64);
    link_type_str = libbpf_bpf_link_type_str((*info).type_);
    if !link_type_str.is_null() {
        jsonw_string_field(wtr, b"type\0".as_ptr() as *const c_char, link_type_str);
    } else {
        jsonw_uint_field(wtr, b"type\0".as_ptr() as *const c_char, (*info).type_ as __u64);
    }

    jsonw_uint_field(json_wtr, b"prog_id\0".as_ptr() as *const c_char, (*info).prog_id as __u64);
}

unsafe fn show_link_attach_type_json(attach_type: __u32, wtr: *mut json_writer_t) {
    let attach_type_str = libbpf_bpf_attach_type_str(attach_type);
    if !attach_type_str.is_null() {
        jsonw_string_field(wtr, b"attach_type\0".as_ptr() as *const c_char, attach_type_str);
    } else {
        jsonw_uint_field(wtr, b"attach_type\0".as_ptr() as *const c_char, attach_type as __u64);
    }
}

unsafe fn show_link_ifindex_json(ifindex: __u32, wtr: *mut json_writer_t) {
    let mut devname = [0 as c_char; IF_NAMESIZE];
    let unknown = b"(unknown)\0";
    core::ptr::copy_nonoverlapping(unknown.as_ptr() as *const c_char, devname.as_mut_ptr(), unknown.len());

    if ifindex != 0 {
        if_indextoname(ifindex, devname.as_mut_ptr());
    } else {
        snprintf(devname.as_mut_ptr(), devname.len(), b"(detached)\0".as_ptr() as *const c_char);
    }
    jsonw_string_field(wtr, b"devname\0".as_ptr() as *const c_char, devname.as_ptr());
    jsonw_uint_field(wtr, b"ifindex\0".as_ptr() as *const c_char, ifindex as __u64);
}

unsafe fn is_iter_map_target(target_name: *const c_char) -> bool {
    strcmp(target_name, b"bpf_map_elem\0".as_ptr() as *const c_char) == 0
        || strcmp(target_name, b"bpf_sk_storage_map\0".as_ptr() as *const c_char) == 0
}

unsafe fn is_iter_cgroup_target(target_name: *const c_char) -> bool {
    strcmp(target_name, b"cgroup\0".as_ptr() as *const c_char) == 0
}

unsafe fn cgroup_order_string(order: __u32) -> *const c_char {
    match order {
        BPF_CGROUP_ITER_ORDER_UNSPEC => b"order_unspec\0".as_ptr() as *const c_char,
        BPF_CGROUP_ITER_SELF_ONLY => b"self_only\0".as_ptr() as *const c_char,
        BPF_CGROUP_ITER_DESCENDANTS_PRE => b"descendants_pre\0".as_ptr() as *const c_char,
        BPF_CGROUP_ITER_DESCENDANTS_POST => b"descendants_post\0".as_ptr() as *const c_char,
        BPF_CGROUP_ITER_ANCESTORS_UP => b"ancestors_up\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

unsafe fn is_iter_task_target(target_name: *const c_char) -> bool {
    strcmp(target_name, b"task\0".as_ptr() as *const c_char) == 0
        || strcmp(target_name, b"task_file\0".as_ptr() as *const c_char) == 0
        || strcmp(target_name, b"task_vma\0".as_ptr() as *const c_char) == 0
}

unsafe fn show_iter_json(info: *mut bpf_link_info, wtr: *mut json_writer_t) {
    let target_name = u64_to_ptr::<c_char>((*info).iter.target_name) as *const c_char;

    jsonw_string_field(wtr, b"target_name\0".as_ptr() as *const c_char, target_name);
    if is_iter_map_target(target_name) {
        jsonw_uint_field(wtr, b"map_id\0".as_ptr() as *const c_char, (*info).iter.map.map_id as __u64);
    } else if is_iter_task_target(target_name) {
        if (*info).iter.task.tid != 0 {
            jsonw_uint_field(wtr, b"tid\0".as_ptr() as *const c_char, (*info).iter.task.tid as __u64);
        } else if (*info).iter.task.pid != 0 {
            jsonw_uint_field(wtr, b"pid\0".as_ptr() as *const c_char, (*info).iter.task.pid as __u64);
        }
    }
    if is_iter_cgroup_target(target_name) {
        jsonw_lluint_field(wtr, b"cgroup_id\0".as_ptr() as *const c_char, (*info).iter.cgroup.cgroup_id);
        jsonw_string_field(wtr, b"order\0".as_ptr() as *const c_char, cgroup_order_string((*info).iter.cgroup.order));
    }
}

pub unsafe fn netfilter_dump_json(info: *const bpf_link_info, _wtr: *mut json_writer_t) {
    jsonw_uint_field(json_wtr, b"pf\0".as_ptr() as *const c_char, (*info).netfilter.pf as __u64);
    jsonw_uint_field(json_wtr, b"hook\0".as_ptr() as *const c_char, (*info).netfilter.hooknum as __u64);
    jsonw_int_field(json_wtr, b"prio\0".as_ptr() as *const c_char, (*info).netfilter.priority);
    jsonw_uint_field(json_wtr, b"flags\0".as_ptr() as *const c_char, (*info).netfilter.flags as __u64);
}

unsafe fn get_prog_info(prog_id: c_int, info: *mut bpf_prog_info) -> c_int {
    let mut len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let err: c_int;
    let prog_fd = bpf_prog_get_fd_by_id(prog_id);
    if prog_fd < 0 {
        return prog_fd;
    }
    core::ptr::write_bytes(info as *mut u8, 0, size_of::<bpf_prog_info>());
    err = bpf_prog_get_info_by_fd(prog_fd, info, &mut len);
    if err != 0 {
        p_err(b"can't get prog info: %s\0".as_ptr() as *const c_char, strerror(errno));
    }
    close(prog_fd);
    err
}

unsafe extern "C" fn cmp_addr_cookie(A: *const c_void, B: *const c_void) -> c_int {
    let a = A as *const addr_cookie;
    let b = B as *const addr_cookie;
    if (*a).addr == (*b).addr {
        return 0;
    }
    if (*a).addr < (*b).addr { -1 } else { 1 }
}

unsafe fn get_addr_cookie_array(addrs: *mut __u64, cookies: *mut __u64, count: __u32) -> *mut addr_cookie {
    let data = calloc(count as size_t, size_of::<addr_cookie>()) as *mut addr_cookie;
    if data.is_null() {
        p_err(b"mem alloc failed\0".as_ptr() as *const c_char);
        return null_mut();
    }
    for i in 0..count {
        (*data.add(i as usize)).addr = *addrs.add(i as usize);
        (*data.add(i as usize)).cookie = *cookies.add(i as usize);
    }
    qsort(data as *mut c_void, count as size_t, size_of::<addr_cookie>(), cmp_addr_cookie);
    data
}

unsafe fn is_x86_ibt_enabled() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        let mut options = [kernel_config_option { name: b"CONFIG_X86_KERNEL_IBT\0".as_ptr() as *const c_char }];
        let mut values: [*mut c_char; 1] = [null_mut()];
        let ret: bool;
        if read_kernel_config(options.as_mut_ptr(), options.len(), values.as_mut_ptr(), null_mut()) != 0 {
            return false;
        }
        ret = !values[0].is_null();
        free(values[0] as *mut c_void);
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

unsafe fn symbol_matches_target(sym_addr: __u64, target_addr: __u64, is_ibt_enabled: bool) -> bool {
    if sym_addr == target_addr {
        return true;
    }
    /*
     * On x86_64 architectures with CET (Control-flow Enforcement Technology),
     * function entry points have a 4-byte 'endbr' instruction prefix.
     * This causes kprobe hooks to target the address *after* 'endbr'
     * (symbol address + 4), preserving the CET instruction.
     * Here we check if the symbol address matches the hook target address
     * minus 4, indicating a CET-enabled function entry point.
     */
    if is_ibt_enabled && sym_addr == target_addr.wrapping_sub(4) {
        return true;
    }
    false
}

unsafe fn u64_to_arr(val: __u64) -> *mut __u64 {
    u64_to_ptr::<__u64>(val)
}

unsafe fn u64_to_u32_arr(val: __u64) -> *mut __u32 {
    u64_to_ptr::<__u32>(val)
}

unsafe fn find_kernel_sym_by_addr(addr: __u64, is_ibt_enabled: bool) -> *mut kernel_sym {
    let mut sym: *mut kernel_sym;
    if addr == 0 {
        return null_mut();
    }
    sym = kernel_syms_search(&raw mut dd, addr);
    if sym.is_null() && is_ibt_enabled && addr >= 4 {
        sym = kernel_syms_search(&raw mut dd, addr - 4);
    }
    sym
}

unsafe fn show_kprobe_multi_json(info: *mut bpf_link_info, _wtr: *mut json_writer_t) {
    let mut j: __u32 = 0;
    jsonw_bool_field(json_wtr, b"retprobe\0".as_ptr() as *const c_char, ((*info).kprobe_multi.flags & BPF_F_KPROBE_MULTI_RETURN) != 0);
    jsonw_uint_field(json_wtr, b"func_cnt\0".as_ptr() as *const c_char, (*info).kprobe_multi.count as __u64);
    jsonw_uint_field(json_wtr, b"missed\0".as_ptr() as *const c_char, (*info).kprobe_multi.missed);
    jsonw_name(json_wtr, b"funcs\0".as_ptr() as *const c_char);
    jsonw_start_array(json_wtr);
    let data = get_addr_cookie_array(u64_to_ptr((*info).kprobe_multi.addrs), u64_to_ptr((*info).kprobe_multi.cookies), (*info).kprobe_multi.count);
    if data.is_null() {
        return;
    }
    if dd.sym_count == 0 {
        kernel_syms_load(&raw mut dd);
    }
    if dd.sym_count != 0 {
        let is_ibt_enabled = is_x86_ibt_enabled();
        for i in 0..dd.sym_count {
            let sym = dd.sym_mapping.add(i as usize);
            if !symbol_matches_target((*sym).address, (*data.add(j as usize)).addr, is_ibt_enabled) {
                continue;
            }
            jsonw_start_object(json_wtr);
            jsonw_uint_field(json_wtr, b"addr\0".as_ptr() as *const c_char, (*data.add(j as usize)).addr);
            jsonw_string_field(json_wtr, b"func\0".as_ptr() as *const c_char, (*sym).name);
            if (*sym).module[0] == 0 {
                jsonw_name(json_wtr, b"module\0".as_ptr() as *const c_char);
                jsonw_null(json_wtr);
            } else {
                jsonw_string_field(json_wtr, b"module\0".as_ptr() as *const c_char, (*sym).module.as_ptr());
            }
            jsonw_uint_field(json_wtr, b"cookie\0".as_ptr() as *const c_char, (*data.add(j as usize)).cookie);
            jsonw_end_object(json_wtr);
            if j == (*info).kprobe_multi.count {
                break;
            }
            j += 1;
        }
        jsonw_end_array(json_wtr);
    }
    free(data as *mut c_void);
}

unsafe fn show_uprobe_multi_json(info: *mut bpf_link_info, _wtr: *mut json_writer_t) {
    jsonw_bool_field(json_wtr, b"retprobe\0".as_ptr() as *const c_char, ((*info).uprobe_multi.flags & BPF_F_UPROBE_MULTI_RETURN) != 0);
    jsonw_string_field(json_wtr, b"path\0".as_ptr() as *const c_char, u64_to_ptr::<c_char>((*info).uprobe_multi.path));
    jsonw_uint_field(json_wtr, b"func_cnt\0".as_ptr() as *const c_char, (*info).uprobe_multi.count as __u64);
    jsonw_int_field(json_wtr, b"pid\0".as_ptr() as *const c_char, (*info).uprobe_multi.pid as c_int);
    jsonw_name(json_wtr, b"funcs\0".as_ptr() as *const c_char);
    jsonw_start_array(json_wtr);
    for i in 0..(*info).uprobe_multi.count {
        jsonw_start_object(json_wtr);
        jsonw_uint_field(json_wtr, b"offset\0".as_ptr() as *const c_char, *u64_to_arr((*info).uprobe_multi.offsets).add(i as usize));
        jsonw_uint_field(json_wtr, b"ref_ctr_offset\0".as_ptr() as *const c_char, *u64_to_arr((*info).uprobe_multi.ref_ctr_offsets).add(i as usize));
        jsonw_uint_field(json_wtr, b"cookie\0".as_ptr() as *const c_char, *u64_to_arr((*info).uprobe_multi.cookies).add(i as usize));
        jsonw_end_object(json_wtr);
    }
    jsonw_end_array(json_wtr);
}

unsafe fn show_tracing_multi_json(info: *mut bpf_link_info, wtr: *mut json_writer_t) {
    let is_ibt_enabled = is_x86_ibt_enabled();
    if dd.sym_count == 0 {
        kernel_syms_load(&raw mut dd);
    }
    let show_symbol = dd.sym_count != 0;
    show_link_attach_type_json((*info).tracing_multi.attach_type, wtr);
    jsonw_uint_field(wtr, b"func_cnt\0".as_ptr() as *const c_char, (*info).tracing_multi.count as __u64);
    jsonw_uint_field(wtr, b"btf_obj_id\0".as_ptr() as *const c_char, (*info).tracing_multi.btf_obj_id as __u64);
    jsonw_name(wtr, b"funcs\0".as_ptr() as *const c_char);
    jsonw_start_array(wtr);
    let ids = u64_to_u32_arr((*info).tracing_multi.ids);
    let addrs = u64_to_arr((*info).tracing_multi.addrs);
    let cookies = u64_to_arr((*info).tracing_multi.cookies);
    for i in 0..(*info).tracing_multi.count {
        let addr = *addrs.add(i as usize);
        let sym = if show_symbol { find_kernel_sym_by_addr(addr, is_ibt_enabled) } else { null_mut() };
        jsonw_start_object(wtr);
        jsonw_uint_field(wtr, b"id\0".as_ptr() as *const c_char, *ids.add(i as usize) as __u64);
        jsonw_uint_field(wtr, b"addr\0".as_ptr() as *const c_char, addr);
        if !sym.is_null() {
            jsonw_string_field(wtr, b"func\0".as_ptr() as *const c_char, (*sym).name);
            if (*sym).module[0] == 0 {
                jsonw_name(wtr, b"module\0".as_ptr() as *const c_char);
                jsonw_null(wtr);
            } else {
                jsonw_string_field(wtr, b"module\0".as_ptr() as *const c_char, (*sym).module.as_ptr());
            }
        }
        jsonw_uint_field(wtr, b"cookie\0".as_ptr() as *const c_char, *cookies.add(i as usize));
        jsonw_end_object(wtr);
    }
    jsonw_end_array(wtr);
}

unsafe fn show_perf_event_kprobe_json(info: *mut bpf_link_info, wtr: *mut json_writer_t) {
    jsonw_bool_field(wtr, b"retprobe\0".as_ptr() as *const c_char, (*info).perf_event.type_ == BPF_PERF_EVENT_KRETPROBE);
    jsonw_uint_field(wtr, b"addr\0".as_ptr() as *const c_char, (*info).perf_event.kprobe.addr);
    jsonw_string_field(wtr, b"func\0".as_ptr() as *const c_char, u64_to_ptr::<c_char>((*info).perf_event.kprobe.func_name));
    jsonw_uint_field(wtr, b"offset\0".as_ptr() as *const c_char, (*info).perf_event.kprobe.offset as __u64);
    jsonw_uint_field(wtr, b"missed\0".as_ptr() as *const c_char, (*info).perf_event.kprobe.missed);
    jsonw_uint_field(wtr, b"cookie\0".as_ptr() as *const c_char, (*info).perf_event.kprobe.cookie);
}

unsafe fn show_perf_event_uprobe_json(info: *mut bpf_link_info, wtr: *mut json_writer_t) {
    jsonw_bool_field(wtr, b"retprobe\0".as_ptr() as *const c_char, (*info).perf_event.type_ == BPF_PERF_EVENT_URETPROBE);
    jsonw_string_field(wtr, b"file\0".as_ptr() as *const c_char, u64_to_ptr::<c_char>((*info).perf_event.uprobe.file_name));
    jsonw_uint_field(wtr, b"offset\0".as_ptr() as *const c_char, (*info).perf_event.uprobe.offset);
    jsonw_uint_field(wtr, b"cookie\0".as_ptr() as *const c_char, (*info).perf_event.uprobe.cookie);
    jsonw_uint_field(wtr, b"ref_ctr_offset\0".as_ptr() as *const c_char, (*info).perf_event.uprobe.ref_ctr_offset);
}

unsafe fn show_perf_event_tracepoint_json(info: *mut bpf_link_info, wtr: *mut json_writer_t) {
    jsonw_string_field(wtr, b"tracepoint\0".as_ptr() as *const c_char, u64_to_ptr::<c_char>((*info).perf_event.tracepoint.tp_name));
    jsonw_uint_field(wtr, b"cookie\0".as_ptr() as *const c_char, (*info).perf_event.tracepoint.cookie);
}

unsafe fn perf_config_hw_cache_str(config: __u64) -> *mut c_char {
    let str_ = malloc(PERF_HW_CACHE_LEN) as *mut c_char;
    if str_.is_null() {
        p_err(b"mem alloc failed\0".as_ptr() as *const c_char);
        return null_mut();
    }
    let hw_cache = perf_event_name(&evsel__hw_cache, config & 0xff);
    if !hw_cache.is_null() {
        snprintf(str_, PERF_HW_CACHE_LEN, b"%s-\0".as_ptr() as *const c_char, hw_cache);
    } else {
        snprintf(str_, PERF_HW_CACHE_LEN, b"%llu-\0".as_ptr() as *const c_char, config & 0xff);
    }
    let op = perf_event_name(&evsel__hw_cache_op, (config >> 8) & 0xff);
    if !op.is_null() {
        snprintf(str_.add(strlen(str_)), PERF_HW_CACHE_LEN - strlen(str_), b"%s-\0".as_ptr() as *const c_char, op);
    } else {
        snprintf(str_.add(strlen(str_)), PERF_HW_CACHE_LEN - strlen(str_), b"%llu-\0".as_ptr() as *const c_char, (config >> 8) & 0xff);
    }
    let result = perf_event_name(&evsel__hw_cache_result, config >> 16);
    if !result.is_null() {
        snprintf(str_.add(strlen(str_)), PERF_HW_CACHE_LEN - strlen(str_), b"%s\0".as_ptr() as *const c_char, result);
    } else {
        snprintf(str_.add(strlen(str_)), PERF_HW_CACHE_LEN - strlen(str_), b"%llu\0".as_ptr() as *const c_char, config >> 16);
    }
    str_
}

unsafe fn perf_config_str(type_: __u32, config: __u64) -> *const c_char {
    match type_ as usize {
        PERF_TYPE_HARDWARE => perf_event_name(&event_symbols_hw, config),
        PERF_TYPE_SOFTWARE => perf_event_name(&event_symbols_sw, config),
        PERF_TYPE_HW_CACHE => perf_config_hw_cache_str(config) as *const c_char,
        _ => null(),
    }
}

unsafe fn show_perf_event_event_json(info: *mut bpf_link_info, wtr: *mut json_writer_t) {
    let config = (*info).perf_event.event.config;
    let type_ = (*info).perf_event.event.type_;
    let perf_type = perf_event_name(&perf_type_name, type_ as __u64);
    if !perf_type.is_null() {
        jsonw_string_field(wtr, b"event_type\0".as_ptr() as *const c_char, perf_type);
    } else {
        jsonw_uint_field(wtr, b"event_type\0".as_ptr() as *const c_char, type_ as __u64);
    }
    let perf_config = perf_config_str(type_, config);
    if !perf_config.is_null() {
        jsonw_string_field(wtr, b"event_config\0".as_ptr() as *const c_char, perf_config);
    } else {
        jsonw_uint_field(wtr, b"event_config\0".as_ptr() as *const c_char, config);
    }
    jsonw_uint_field(wtr, b"cookie\0".as_ptr() as *const c_char, (*info).perf_event.event.cookie);
    if type_ as usize == PERF_TYPE_HW_CACHE && !perf_config.is_null() {
        free(perf_config as *mut c_void);
    }
}

unsafe fn show_link_close_json(_fd: c_int, info: *mut bpf_link_info) -> c_int {
    let mut prog_info: bpf_prog_info = zeroed();
    jsonw_start_object(json_wtr);
    show_link_header_json(info, json_wtr);
    match (*info).type_ {
        BPF_LINK_TYPE_RAW_TRACEPOINT => {
            jsonw_string_field(json_wtr, b"tp_name\0".as_ptr() as *const c_char, u64_to_ptr((*info).raw_tracepoint.tp_name));
            jsonw_uint_field(json_wtr, b"cookie\0".as_ptr() as *const c_char, (*info).raw_tracepoint.cookie);
        }
        BPF_LINK_TYPE_TRACING => {
            let err = get_prog_info((*info).prog_id as c_int, &mut prog_info);
            if err != 0 { return err; }
            let prog_type_str = libbpf_bpf_prog_type_str(prog_info.type_);
            if !prog_type_str.is_null() {
                jsonw_string_field(json_wtr, b"prog_type\0".as_ptr() as *const c_char, prog_type_str);
            } else {
                jsonw_uint_field(json_wtr, b"prog_type\0".as_ptr() as *const c_char, prog_info.type_ as __u64);
            }
            show_link_attach_type_json((*info).tracing.attach_type, json_wtr);
            jsonw_uint_field(json_wtr, b"target_obj_id\0".as_ptr() as *const c_char, (*info).tracing.target_obj_id as __u64);
            jsonw_uint_field(json_wtr, b"target_btf_id\0".as_ptr() as *const c_char, (*info).tracing.target_btf_id as __u64);
            jsonw_uint_field(json_wtr, b"cookie\0".as_ptr() as *const c_char, (*info).tracing.cookie);
        }
        BPF_LINK_TYPE_CGROUP => {
            jsonw_lluint_field(json_wtr, b"cgroup_id\0".as_ptr() as *const c_char, (*info).cgroup.cgroup_id);
            show_link_attach_type_json((*info).cgroup.attach_type, json_wtr);
        }
        BPF_LINK_TYPE_ITER => show_iter_json(info, json_wtr),
        BPF_LINK_TYPE_NETNS => {
            jsonw_uint_field(json_wtr, b"netns_ino\0".as_ptr() as *const c_char, (*info).netns.netns_ino as __u64);
            show_link_attach_type_json((*info).netns.attach_type, json_wtr);
        }
        BPF_LINK_TYPE_NETFILTER => netfilter_dump_json(info, json_wtr),
        BPF_LINK_TYPE_TCX => { show_link_ifindex_json((*info).tcx.ifindex, json_wtr); show_link_attach_type_json((*info).tcx.attach_type, json_wtr); }
        BPF_LINK_TYPE_NETKIT => { show_link_ifindex_json((*info).netkit.ifindex, json_wtr); show_link_attach_type_json((*info).netkit.attach_type, json_wtr); }
        BPF_LINK_TYPE_SOCKMAP => { jsonw_uint_field(json_wtr, b"map_id\0".as_ptr() as *const c_char, (*info).sockmap.map_id as __u64); show_link_attach_type_json((*info).sockmap.attach_type, json_wtr); }
        BPF_LINK_TYPE_XDP => show_link_ifindex_json((*info).xdp.ifindex, json_wtr),
        BPF_LINK_TYPE_STRUCT_OPS => jsonw_uint_field(json_wtr, b"map_id\0".as_ptr() as *const c_char, (*info).struct_ops.map_id as __u64),
        BPF_LINK_TYPE_KPROBE_MULTI => show_kprobe_multi_json(info, json_wtr),
        BPF_LINK_TYPE_UPROBE_MULTI => show_uprobe_multi_json(info, json_wtr),
        BPF_LINK_TYPE_TRACING_MULTI => show_tracing_multi_json(info, json_wtr),
        BPF_LINK_TYPE_PERF_EVENT => match (*info).perf_event.type_ {
            BPF_PERF_EVENT_EVENT => show_perf_event_event_json(info, json_wtr),
            BPF_PERF_EVENT_TRACEPOINT => show_perf_event_tracepoint_json(info, json_wtr),
            BPF_PERF_EVENT_KPROBE | BPF_PERF_EVENT_KRETPROBE => show_perf_event_kprobe_json(info, json_wtr),
            BPF_PERF_EVENT_UPROBE | BPF_PERF_EVENT_URETPROBE => show_perf_event_uprobe_json(info, json_wtr),
            _ => {}
        },
        _ => {}
    }
    if !hashmap__empty(link_table) {
        let mut entry: *mut hashmap_entry = null_mut();
        jsonw_name(json_wtr, b"pinned\0".as_ptr() as *const c_char);
        jsonw_start_array(json_wtr);
        while hashmap__for_each_key_entry(link_table, &mut entry, (*info).id) {
            jsonw_string(json_wtr, (*entry).pvalue as *const c_char);
        }
        jsonw_end_array(json_wtr);
    }
    emit_obj_refs_json(refs_table, (*info).id, json_wtr);
    jsonw_end_object(json_wtr);
    0
}

unsafe fn show_link_header_plain(info: *mut bpf_link_info) {
    printf(b"%u: \0".as_ptr() as *const c_char, (*info).id);
    let link_type_str = libbpf_bpf_link_type_str((*info).type_);
    if !link_type_str.is_null() {
        printf(b"%s  \0".as_ptr() as *const c_char, link_type_str);
    } else {
        printf(b"type %u  \0".as_ptr() as *const c_char, (*info).type_);
    }
    if (*info).type_ == BPF_LINK_TYPE_STRUCT_OPS {
        printf(b"map %u  \0".as_ptr() as *const c_char, (*info).struct_ops.map_id);
    } else {
        printf(b"prog %u  \0".as_ptr() as *const c_char, (*info).prog_id);
    }
}

unsafe fn show_link_attach_type_plain(attach_type: __u32) {
    let attach_type_str = libbpf_bpf_attach_type_str(attach_type);
    if !attach_type_str.is_null() {
        printf(b"attach_type %s  \0".as_ptr() as *const c_char, attach_type_str);
    } else {
        printf(b"attach_type %u  \0".as_ptr() as *const c_char, attach_type);
    }
}

unsafe fn show_link_ifindex_plain(ifindex: __u32) {
    let mut devname = [0 as c_char; IF_NAMESIZE * 2];
    let mut tmpname = [0 as c_char; IF_NAMESIZE];
    let mut ret: *mut c_char = null_mut();
    if ifindex != 0 {
        ret = if_indextoname(ifindex, tmpname.as_mut_ptr());
    } else {
        snprintf(devname.as_mut_ptr(), devname.len(), b"(detached)\0".as_ptr() as *const c_char);
    }
    if !ret.is_null() {
        snprintf(devname.as_mut_ptr(), devname.len(), b"%s(%u)\0".as_ptr() as *const c_char, tmpname.as_ptr(), ifindex);
    }
    printf(b"ifindex %s  \0".as_ptr() as *const c_char, devname.as_ptr());
}

unsafe fn show_iter_plain(info: *mut bpf_link_info) {
    let target_name = u64_to_ptr::<c_char>((*info).iter.target_name) as *const c_char;
    printf(b"target_name %s  \0".as_ptr() as *const c_char, target_name);
    if is_iter_map_target(target_name) {
        printf(b"map_id %u  \0".as_ptr() as *const c_char, (*info).iter.map.map_id);
    } else if is_iter_task_target(target_name) {
        if (*info).iter.task.tid != 0 {
            printf(b"tid %u \0".as_ptr() as *const c_char, (*info).iter.task.tid);
        } else if (*info).iter.task.pid != 0 {
            printf(b"pid %u \0".as_ptr() as *const c_char, (*info).iter.task.pid);
        }
    }
    if is_iter_cgroup_target(target_name) {
        printf(b"cgroup_id %llu  \0".as_ptr() as *const c_char, (*info).iter.cgroup.cgroup_id);
        printf(b"order %s  \0".as_ptr() as *const c_char, cgroup_order_string((*info).iter.cgroup.order));
    }
}

pub unsafe fn netfilter_dump_plain(info: *const bpf_link_info) {
    let mut hookname: *const c_char = null();
    let mut pfname: *const c_char = null();
    let hook = (*info).netfilter.hooknum as usize;
    let pf = (*info).netfilter.pf as usize;
    if pf < pf2name.len() {
        pfname = pf2name[pf];
    }
    match pf {
        NFPROTO_BRIDGE | NFPROTO_IPV4 | NFPROTO_IPV6 | NFPROTO_INET => {
            if hook < inethook2name.len() { hookname = inethook2name[hook]; }
        }
        NFPROTO_ARP => {
            if hook < arphook2name.len() { hookname = arphook2name[hook]; }
        }
        _ => {}
    }
    if !pfname.is_null() { printf(b"\n\t%s\0".as_ptr() as *const c_char, pfname); } else { printf(b"\n\tpf: %u\0".as_ptr() as *const c_char, pf); }
    if !hookname.is_null() { printf(b" %s\0".as_ptr() as *const c_char, hookname); } else { printf(b", hook %u,\0".as_ptr() as *const c_char, hook); }
    printf(b" prio %d\0".as_ptr() as *const c_char, (*info).netfilter.priority);
    if (*info).netfilter.flags != 0 {
        printf(b" flags 0x%x\0".as_ptr() as *const c_char, (*info).netfilter.flags);
    }
}

unsafe fn show_kprobe_multi_plain(info: *mut bpf_link_info) {
    let mut j: __u32 = 0;
    if (*info).kprobe_multi.count == 0 { return; }
    if ((*info).kprobe_multi.flags & BPF_F_KPROBE_MULTI_RETURN) != 0 { printf(b"\n\tkretprobe.multi  \0".as_ptr() as *const c_char); } else { printf(b"\n\tkprobe.multi  \0".as_ptr() as *const c_char); }
    printf(b"func_cnt %u  \0".as_ptr() as *const c_char, (*info).kprobe_multi.count);
    if (*info).kprobe_multi.missed != 0 { printf(b"missed %llu  \0".as_ptr() as *const c_char, (*info).kprobe_multi.missed); }
    let data = get_addr_cookie_array(u64_to_ptr((*info).kprobe_multi.addrs), u64_to_ptr((*info).kprobe_multi.cookies), (*info).kprobe_multi.count);
    if data.is_null() { return; }
    if dd.sym_count == 0 { kernel_syms_load(&raw mut dd); }
    if dd.sym_count != 0 {
        let is_ibt_enabled = is_x86_ibt_enabled();
        printf(b"\n\t%-16s %-16s %s\0".as_ptr() as *const c_char, b"addr\0".as_ptr(), b"cookie\0".as_ptr(), b"func [module]\0".as_ptr());
        for i in 0..dd.sym_count {
            let sym = dd.sym_mapping.add(i as usize);
            if !symbol_matches_target((*sym).address, (*data.add(j as usize)).addr, is_ibt_enabled) { continue; }
            printf(b"\n\t%016lx %-16llx %s\0".as_ptr() as *const c_char, (*data.add(j as usize)).addr as c_ulong, (*data.add(j as usize)).cookie, (*sym).name);
            if (*sym).module[0] != 0 { printf(b" [%s]  \0".as_ptr() as *const c_char, (*sym).module.as_ptr()); } else { printf(b"  \0".as_ptr() as *const c_char); }
            if j == (*info).kprobe_multi.count { break; }
            j += 1;
        }
    }
    free(data as *mut c_void);
}

unsafe fn show_uprobe_multi_plain(info: *mut bpf_link_info) {
    if (*info).uprobe_multi.count == 0 { return; }
    if ((*info).uprobe_multi.flags & BPF_F_UPROBE_MULTI_RETURN) != 0 { printf(b"\n\turetprobe.multi  \0".as_ptr() as *const c_char); } else { printf(b"\n\tuprobe.multi  \0".as_ptr() as *const c_char); }
    printf(b"path %s  \0".as_ptr() as *const c_char, u64_to_ptr::<c_char>((*info).uprobe_multi.path));
    printf(b"func_cnt %u  \0".as_ptr() as *const c_char, (*info).uprobe_multi.count);
    if (*info).uprobe_multi.pid != 0 { printf(b"pid %u  \0".as_ptr() as *const c_char, (*info).uprobe_multi.pid); }
    printf(b"\n\t%-16s   %-16s   %-16s\0".as_ptr() as *const c_char, b"offset\0".as_ptr(), b"ref_ctr_offset\0".as_ptr(), b"cookies\0".as_ptr());
    for i in 0..(*info).uprobe_multi.count {
        printf(b"\n\t0x%-16llx 0x%-16llx 0x%-16llx\0".as_ptr() as *const c_char, *u64_to_arr((*info).uprobe_multi.offsets).add(i as usize), *u64_to_arr((*info).uprobe_multi.ref_ctr_offsets).add(i as usize), *u64_to_arr((*info).uprobe_multi.cookies).add(i as usize));
    }
}

unsafe fn show_tracing_multi_plain(info: *mut bpf_link_info) {
    if (*info).tracing_multi.count == 0 { return; }
    let is_ibt_enabled = is_x86_ibt_enabled();
    if dd.sym_count == 0 { kernel_syms_load(&raw mut dd); }
    let show_symbol = dd.sym_count != 0;
    printf(b"\n\t\0".as_ptr() as *const c_char);
    show_link_attach_type_plain((*info).tracing_multi.attach_type);
    printf(b"btf_obj_id %u  \0".as_ptr() as *const c_char, (*info).tracing_multi.btf_obj_id);
    printf(b"count %u  \0".as_ptr() as *const c_char, (*info).tracing_multi.count);
    printf(b"\n\t%-16s %-16s %-16s %s\0".as_ptr() as *const c_char, b"btf_id\0".as_ptr(), b"addr\0".as_ptr(), b"cookie\0".as_ptr(), b"func [module]\0".as_ptr());
    let ids = u64_to_u32_arr((*info).tracing_multi.ids);
    let addrs = u64_to_arr((*info).tracing_multi.addrs);
    let cookies = u64_to_arr((*info).tracing_multi.cookies);
    for i in 0..(*info).tracing_multi.count {
        let addr = *addrs.add(i as usize);
        let sym = if show_symbol { find_kernel_sym_by_addr(addr, is_ibt_enabled) } else { null_mut() };
        printf(b"\n\t%-16u %016llx %-16llu\0".as_ptr() as *const c_char, *ids.add(i as usize), addr, *cookies.add(i as usize));
        if !sym.is_null() {
            printf(b" %s\0".as_ptr() as *const c_char, (*sym).name);
            if (*sym).module[0] != 0 { printf(b" [%s]\0".as_ptr() as *const c_char, (*sym).module.as_ptr()); }
        }
    }
}

unsafe fn show_perf_event_kprobe_plain(info: *mut bpf_link_info) {
    let buf = u64_to_ptr::<c_char>((*info).perf_event.kprobe.func_name) as *const c_char;
    if *buf == 0 && (*info).perf_event.kprobe.addr == 0 { return; }
    if (*info).perf_event.type_ == BPF_PERF_EVENT_KRETPROBE { printf(b"\n\tkretprobe \0".as_ptr() as *const c_char); } else { printf(b"\n\tkprobe \0".as_ptr() as *const c_char); }
    if (*info).perf_event.kprobe.addr != 0 { printf(b"%llx \0".as_ptr() as *const c_char, (*info).perf_event.kprobe.addr); }
    printf(b"%s\0".as_ptr() as *const c_char, buf);
    if (*info).perf_event.kprobe.offset != 0 { printf(b"+%#x\0".as_ptr() as *const c_char, (*info).perf_event.kprobe.offset); }
    if (*info).perf_event.kprobe.missed != 0 { printf(b"  missed %llu\0".as_ptr() as *const c_char, (*info).perf_event.kprobe.missed); }
    if (*info).perf_event.kprobe.cookie != 0 { printf(b"  cookie %llu\0".as_ptr() as *const c_char, (*info).perf_event.kprobe.cookie); }
    printf(b"  \0".as_ptr() as *const c_char);
}

unsafe fn show_perf_event_uprobe_plain(info: *mut bpf_link_info) {
    let buf = u64_to_ptr::<c_char>((*info).perf_event.uprobe.file_name) as *const c_char;
    if *buf == 0 { return; }
    if (*info).perf_event.type_ == BPF_PERF_EVENT_URETPROBE { printf(b"\n\turetprobe \0".as_ptr() as *const c_char); } else { printf(b"\n\tuprobe \0".as_ptr() as *const c_char); }
    printf(b"%s+%#x  \0".as_ptr() as *const c_char, buf, (*info).perf_event.uprobe.offset);
    if (*info).perf_event.uprobe.cookie != 0 { printf(b"cookie %llu  \0".as_ptr() as *const c_char, (*info).perf_event.uprobe.cookie); }
    if (*info).perf_event.uprobe.ref_ctr_offset != 0 { printf(b"ref_ctr_offset 0x%llx  \0".as_ptr() as *const c_char, (*info).perf_event.uprobe.ref_ctr_offset); }
}

unsafe fn show_perf_event_tracepoint_plain(info: *mut bpf_link_info) {
    let buf = u64_to_ptr::<c_char>((*info).perf_event.tracepoint.tp_name) as *const c_char;
    if *buf == 0 { return; }
    printf(b"\n\ttracepoint %s  \0".as_ptr() as *const c_char, buf);
    if (*info).perf_event.tracepoint.cookie != 0 { printf(b"cookie %llu  \0".as_ptr() as *const c_char, (*info).perf_event.tracepoint.cookie); }
}

unsafe fn show_perf_event_event_plain(info: *mut bpf_link_info) {
    let config = (*info).perf_event.event.config;
    let type_ = (*info).perf_event.event.type_;
    printf(b"\n\tevent \0".as_ptr() as *const c_char);
    let perf_type = perf_event_name(&perf_type_name, type_ as __u64);
    if !perf_type.is_null() { printf(b"%s:\0".as_ptr() as *const c_char, perf_type); } else { printf(b"%u :\0".as_ptr() as *const c_char, type_); }
    let perf_config = perf_config_str(type_, config);
    if !perf_config.is_null() { printf(b"%s  \0".as_ptr() as *const c_char, perf_config); } else { printf(b"%llu  \0".as_ptr() as *const c_char, config); }
    if (*info).perf_event.event.cookie != 0 { printf(b"cookie %llu  \0".as_ptr() as *const c_char, (*info).perf_event.event.cookie); }
    if type_ as usize == PERF_TYPE_HW_CACHE && !perf_config.is_null() { free(perf_config as *mut c_void); }
}

unsafe fn show_link_close_plain(_fd: c_int, info: *mut bpf_link_info) -> c_int {
    let mut prog_info: bpf_prog_info = zeroed();
    show_link_header_plain(info);
    match (*info).type_ {
        BPF_LINK_TYPE_RAW_TRACEPOINT => {
            printf(b"\n\ttp '%s'  \0".as_ptr() as *const c_char, u64_to_ptr::<c_char>((*info).raw_tracepoint.tp_name));
            if (*info).raw_tracepoint.cookie != 0 { printf(b"cookie %llu  \0".as_ptr() as *const c_char, (*info).raw_tracepoint.cookie); }
        }
        BPF_LINK_TYPE_TRACING => {
            let err = get_prog_info((*info).prog_id as c_int, &mut prog_info);
            if err != 0 { return err; }
            let prog_type_str = libbpf_bpf_prog_type_str(prog_info.type_);
            if !prog_type_str.is_null() { printf(b"\n\tprog_type %s  \0".as_ptr() as *const c_char, prog_type_str); } else { printf(b"\n\tprog_type %u  \0".as_ptr() as *const c_char, prog_info.type_); }
            show_link_attach_type_plain((*info).tracing.attach_type);
            if (*info).tracing.target_obj_id != 0 || (*info).tracing.target_btf_id != 0 {
                printf(b"\n\ttarget_obj_id %u  target_btf_id %u  \0".as_ptr() as *const c_char, (*info).tracing.target_obj_id, (*info).tracing.target_btf_id);
            }
            if (*info).tracing.cookie != 0 { printf(b"\n\tcookie %llu  \0".as_ptr() as *const c_char, (*info).tracing.cookie); }
        }
        BPF_LINK_TYPE_CGROUP => { printf(b"\n\tcgroup_id %zu  \0".as_ptr() as *const c_char, (*info).cgroup.cgroup_id as size_t); show_link_attach_type_plain((*info).cgroup.attach_type); }
        BPF_LINK_TYPE_ITER => show_iter_plain(info),
        BPF_LINK_TYPE_NETNS => { printf(b"\n\tnetns_ino %u  \0".as_ptr() as *const c_char, (*info).netns.netns_ino); show_link_attach_type_plain((*info).netns.attach_type); }
        BPF_LINK_TYPE_NETFILTER => netfilter_dump_plain(info),
        BPF_LINK_TYPE_TCX => { printf(b"\n\t\0".as_ptr() as *const c_char); show_link_ifindex_plain((*info).tcx.ifindex); show_link_attach_type_plain((*info).tcx.attach_type); }
        BPF_LINK_TYPE_NETKIT => { printf(b"\n\t\0".as_ptr() as *const c_char); show_link_ifindex_plain((*info).netkit.ifindex); show_link_attach_type_plain((*info).netkit.attach_type); }
        BPF_LINK_TYPE_SOCKMAP => { printf(b"\n\t\0".as_ptr() as *const c_char); printf(b"map_id %u  \0".as_ptr() as *const c_char, (*info).sockmap.map_id); show_link_attach_type_plain((*info).sockmap.attach_type); }
        BPF_LINK_TYPE_XDP => { printf(b"\n\t\0".as_ptr() as *const c_char); show_link_ifindex_plain((*info).xdp.ifindex); }
        BPF_LINK_TYPE_KPROBE_MULTI => show_kprobe_multi_plain(info),
        BPF_LINK_TYPE_UPROBE_MULTI => show_uprobe_multi_plain(info),
        BPF_LINK_TYPE_TRACING_MULTI => show_tracing_multi_plain(info),
        BPF_LINK_TYPE_PERF_EVENT => match (*info).perf_event.type_ {
            BPF_PERF_EVENT_EVENT => show_perf_event_event_plain(info),
            BPF_PERF_EVENT_TRACEPOINT => show_perf_event_tracepoint_plain(info),
            BPF_PERF_EVENT_KPROBE | BPF_PERF_EVENT_KRETPROBE => show_perf_event_kprobe_plain(info),
            BPF_PERF_EVENT_UPROBE | BPF_PERF_EVENT_URETPROBE => show_perf_event_uprobe_plain(info),
            _ => {}
        },
        _ => {}
    }
    if !hashmap__empty(link_table) {
        let mut entry: *mut hashmap_entry = null_mut();
        while hashmap__for_each_key_entry(link_table, &mut entry, (*info).id) {
            printf(b"\n\tpinned %s\0".as_ptr() as *const c_char, (*entry).pvalue as *mut c_char);
        }
    }
    emit_obj_refs_plain(refs_table, (*info).id, b"\n\tpids \0".as_ptr() as *const c_char);
    printf(b"\n\0".as_ptr() as *const c_char);
    0
}

unsafe fn do_show_link(fd: c_int) -> c_int {
    let mut ref_ctr_offsets: *mut __u64 = null_mut();
    let mut offsets: *mut __u64 = null_mut();
    let mut cookies: *mut __u64 = null_mut();
    let mut ids: *mut __u32 = null_mut();
    let mut info: bpf_link_info = zeroed();
    let mut len = size_of::<bpf_link_info>() as __u32;
    let mut path_buf = [0 as c_char; PATH_MAX];
    let mut addrs: *mut __u64 = null_mut();
    let mut buf = [0 as c_char; PATH_MAX];
    let mut count: c_int;
    loop {
        let err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
        if err != 0 {
            p_err(b"can't get link info: %s\0".as_ptr() as *const c_char, strerror(errno));
            close(fd);
            return err;
        }
        if info.type_ == BPF_LINK_TYPE_RAW_TRACEPOINT && info.raw_tracepoint.tp_name == 0 {
            info.raw_tracepoint.tp_name = ptr_to_u64(buf.as_mut_ptr());
            info.raw_tracepoint.tp_name_len = size_of::<[c_char; PATH_MAX]>() as __u32;
            continue;
        }
        if info.type_ == BPF_LINK_TYPE_ITER && info.iter.target_name == 0 {
            info.iter.target_name = ptr_to_u64(buf.as_mut_ptr());
            info.iter.target_name_len = size_of::<[c_char; PATH_MAX]>() as __u32;
            continue;
        }
        if info.type_ == BPF_LINK_TYPE_KPROBE_MULTI && info.kprobe_multi.addrs == 0 {
            count = info.kprobe_multi.count as c_int;
            if count != 0 {
                addrs = calloc(count as size_t, size_of::<__u64>()) as *mut __u64;
                if addrs.is_null() { p_err(b"mem alloc failed\0".as_ptr() as *const c_char); close(fd); return -ENOMEM; }
                info.kprobe_multi.addrs = ptr_to_u64(addrs);
                cookies = calloc(count as size_t, size_of::<__u64>()) as *mut __u64;
                if cookies.is_null() { p_err(b"mem alloc failed\0".as_ptr() as *const c_char); free(addrs as *mut c_void); close(fd); return -ENOMEM; }
                info.kprobe_multi.cookies = ptr_to_u64(cookies);
                continue;
            }
        }
        if info.type_ == BPF_LINK_TYPE_UPROBE_MULTI && info.uprobe_multi.offsets == 0 {
            count = info.uprobe_multi.count as c_int;
            if count != 0 {
                offsets = calloc(count as size_t, size_of::<__u64>()) as *mut __u64;
                if offsets.is_null() { p_err(b"mem alloc failed\0".as_ptr() as *const c_char); close(fd); return -ENOMEM; }
                info.uprobe_multi.offsets = ptr_to_u64(offsets);
                ref_ctr_offsets = calloc(count as size_t, size_of::<__u64>()) as *mut __u64;
                if ref_ctr_offsets.is_null() { p_err(b"mem alloc failed\0".as_ptr() as *const c_char); free(offsets as *mut c_void); close(fd); return -ENOMEM; }
                info.uprobe_multi.ref_ctr_offsets = ptr_to_u64(ref_ctr_offsets);
                cookies = calloc(count as size_t, size_of::<__u64>()) as *mut __u64;
                if cookies.is_null() { p_err(b"mem alloc failed\0".as_ptr() as *const c_char); free(ref_ctr_offsets as *mut c_void); free(offsets as *mut c_void); close(fd); return -ENOMEM; }
                info.uprobe_multi.cookies = ptr_to_u64(cookies);
                info.uprobe_multi.path = ptr_to_u64(path_buf.as_mut_ptr());
                info.uprobe_multi.path_size = size_of::<[c_char; PATH_MAX]>() as __u32;
                continue;
            }
        }
        if info.type_ == BPF_LINK_TYPE_TRACING_MULTI && info.tracing_multi.ids == 0 {
            count = info.tracing_multi.count as c_int;
            if count != 0 {
                ids = calloc(count as size_t, size_of::<__u32>()) as *mut __u32;
                addrs = calloc(count as size_t, size_of::<__u64>()) as *mut __u64;
                cookies = calloc(count as size_t, size_of::<__u64>()) as *mut __u64;
                if ids.is_null() || addrs.is_null() || cookies.is_null() {
                    p_err(b"mem alloc failed\0".as_ptr() as *const c_char);
                    close(fd); free(cookies as *mut c_void); free(addrs as *mut c_void); free(ids as *mut c_void);
                    return -ENOMEM;
                }
                info.tracing_multi.ids = ptr_to_u64(ids);
                info.tracing_multi.addrs = ptr_to_u64(addrs);
                info.tracing_multi.cookies = ptr_to_u64(cookies);
                continue;
            }
        }
        if info.type_ == BPF_LINK_TYPE_PERF_EVENT {
            match info.perf_event.type_ {
                BPF_PERF_EVENT_TRACEPOINT if info.perf_event.tracepoint.tp_name == 0 => {
                    info.perf_event.tracepoint.tp_name = ptr_to_u64(buf.as_mut_ptr());
                    info.perf_event.tracepoint.name_len = size_of::<[c_char; PATH_MAX]>() as __u32;
                    continue;
                }
                BPF_PERF_EVENT_KPROBE | BPF_PERF_EVENT_KRETPROBE if info.perf_event.kprobe.func_name == 0 => {
                    info.perf_event.kprobe.func_name = ptr_to_u64(buf.as_mut_ptr());
                    info.perf_event.kprobe.name_len = size_of::<[c_char; PATH_MAX]>() as __u32;
                    continue;
                }
                BPF_PERF_EVENT_UPROBE | BPF_PERF_EVENT_URETPROBE if info.perf_event.uprobe.file_name == 0 => {
                    info.perf_event.uprobe.file_name = ptr_to_u64(buf.as_mut_ptr());
                    info.perf_event.uprobe.name_len = size_of::<[c_char; PATH_MAX]>() as __u32;
                    continue;
                }
                _ => {}
            }
        }
        break;
    }
    if json_output { show_link_close_json(fd, &mut info); } else { show_link_close_plain(fd, &mut info); }
    free(ref_ctr_offsets as *mut c_void);
    free(cookies as *mut c_void);
    free(offsets as *mut c_void);
    free(addrs as *mut c_void);
    free(ids as *mut c_void);
    close(fd);
    0
}

unsafe extern "C" fn do_show(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut id: __u32 = 0;
    let mut fd: c_int;
    if show_pinned {
        link_table = hashmap__new(hash_fn_for_key_as_id, equal_fn_for_key_as_id, null_mut());
        if IS_ERR(link_table as *const c_void) {
            p_err(b"failed to create hashmap for pinned paths\0".as_ptr() as *const c_char);
            return -1;
        }
        build_pinned_obj_table(link_table, BPF_OBJ_LINK);
    }
    build_obj_refs_table(&raw mut refs_table, BPF_OBJ_LINK);
    if argc == 2 {
        fd = link_parse_fd(&mut argc, &mut argv);
        if fd < 0 { return fd; }
        do_show_link(fd);
        if dd.sym_count != 0 { kernel_syms_destroy(&raw mut dd); }
        return if errno == ENOENT { 0 } else { -1 };
    }
    if argc != 0 { return BAD_ARG(); }
    if json_output { jsonw_start_array(json_wtr); }
    loop {
        let err = bpf_link_get_next_id(id, &mut id);
        if err != 0 {
            if errno == ENOENT { break; }
            p_err(b"can't get next link: %s%s\0".as_ptr() as *const c_char, strerror(errno), if errno == EINVAL { b" -- kernel too old?\0".as_ptr() } else { b"\0".as_ptr() });
            break;
        }
        fd = bpf_link_get_fd_by_id(id);
        if fd < 0 {
            if errno == ENOENT { continue; }
            p_err(b"can't get link by id (%u): %s\0".as_ptr() as *const c_char, id, strerror(errno));
            break;
        }
        let err = do_show_link(fd);
        if err != 0 { break; }
    }
    if json_output { jsonw_end_array(json_wtr); }
    delete_obj_refs_table(refs_table);
    if show_pinned { delete_pinned_obj_table(link_table); }
    if dd.sym_count != 0 { kernel_syms_destroy(&raw mut dd); }
    if errno == ENOENT { 0 } else { -1 }
}

unsafe extern "C" fn do_pin(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let err = do_pin_any(argc, argv, link_parse_fd);
    if err == 0 && json_output { jsonw_null(json_wtr); }
    err
}

unsafe extern "C" fn do_detach(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    if argc != 2 {
        p_err(b"link specifier is invalid or missing\n\0".as_ptr() as *const c_char);
        return 1;
    }
    let fd = link_parse_fd(&mut argc, &mut argv);
    if fd < 0 { return 1; }
    let mut err = bpf_link_detach(fd);
    if err != 0 { err = -errno; }
    close(fd);
    if err != 0 {
        p_err(b"failed link detach: %s\0".as_ptr() as *const c_char, strerror(-err));
        return 1;
    }
    if json_output { jsonw_null(json_wtr); }
    0
}

unsafe extern "C" fn do_help(_argc: c_int, argv: *mut *mut c_char) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }
    fprintf(
        stderr,
        b"Usage: %1$s %2$s { show | list }   [LINK]\n       %1$s %2$s pin        LINK  FILE\n       %1$s %2$s detach     LINK\n       %1$s %2$s help\n\n       LINK := { id LINK_ID | pinned FILE }\n       OPTIONS := { {-j|--json} [{-p|--pretty}] | {-d|--debug} |\n                    {-f|--bpffs} | {-n|--nomount} }\n\0".as_ptr() as *const c_char,
        bin_name,
        *argv.offset(-2),
    );
    0
}

static cmds: [cmd; 6] = [
    cmd { cmd: b"show\0".as_ptr() as *const c_char, func: Some(do_show) },
    cmd { cmd: b"list\0".as_ptr() as *const c_char, func: Some(do_show) },
    cmd { cmd: b"help\0".as_ptr() as *const c_char, func: Some(do_help) },
    cmd { cmd: b"pin\0".as_ptr() as *const c_char, func: Some(do_pin) },
    cmd { cmd: b"detach\0".as_ptr() as *const c_char, func: Some(do_detach) },
    cmd { cmd: null(), func: None },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_link(argc: c_int, argv: *mut *mut c_char) -> c_int {
    cmd_select(cmds.as_ptr(), argc, argv, do_help)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
