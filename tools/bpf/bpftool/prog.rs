// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2017-2018 Netronome Systems, Inc. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type ssize_t = isize;
type time_t = i64;
type FILE = c_void;
type DIR = c_void;

const BPF_METADATA_PREFIX: &[u8] = b"bpf_metadata_\0";
const BPF_METADATA_PREFIX_LEN: usize = 13;
const MAX_NUM_PROFILE_METRICS: usize = 4;

const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const CLOCK_REALTIME: c_int = 0;
const CLOCK_BOOTTIME: c_int = 7;
const UINT_MAX: c_ulong = c_uint::MAX as c_ulong;
const UINT32_MAX: c_uint = c_uint::MAX;
const SZ_32K: c_uint = 32 * 1024;
const BPF_ANY: __u64 = 0;
const KEY_SPEC_SESSION_KEYRING: c_int = -3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum dump_mode {
    DUMP_JITED,
    DUMP_XLATED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum prog_tracelog_mode {
    TRACE_STDOUT,
    TRACE_STDERR,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct tm {
    _priv: [u8; 0],
}

#[repr(C)]
struct bpf_prog_info {
    type_: __u32,
    id: __u32,
    tag: [__u8; 8],
    jited_prog_len: __u32,
    xlated_prog_len: __u32,
    jited_prog_insns: __u64,
    xlated_prog_insns: __u64,
    load_time: __u64,
    created_by_uid: __u32,
    nr_map_ids: __u32,
    map_ids: __u64,
    name: [c_char; 16],
    ifindex: __u32,
    gpl_compatible: __u32,
    netns_dev: __u64,
    netns_ino: __u64,
    nr_jited_ksyms: __u32,
    nr_jited_func_lens: __u32,
    jited_ksyms: __u64,
    jited_func_lens: __u64,
    btf_id: __u32,
    func_info_rec_size: __u32,
    func_info: __u64,
    nr_func_info: __u32,
    nr_line_info: __u32,
    line_info: __u64,
    jited_line_info: __u64,
    nr_jited_line_info: __u32,
    line_info_rec_size: __u32,
    jited_line_info_rec_size: __u32,
    run_time_ns: __u64,
    run_cnt: __u64,
    recursion_misses: __u64,
}

#[repr(C)]
struct bpf_map_info {
    type_: __u32,
    id: __u32,
    key_size: __u32,
    value_size: __u32,
    max_entries: __u32,
    name: [c_char; 16],
    btf_id: __u32,
    btf_value_type_id: __u32,
}

#[repr(C)]
struct btf_type {
    name_off: __u32,
    info: __u32,
    size: __u32,
    type_: __u32,
}

#[repr(C)]
struct btf_var_secinfo {
    type_: __u32,
    offset: __u32,
    size: __u32,
}

#[repr(C)]
struct btf;
#[repr(C)]
struct btf_dumper {
    btf: *mut btf,
    jw: *mut json_writer_t,
    is_plain_text: bool,
}
#[repr(C)]
struct json_writer_t;
#[repr(C)]
struct hashmap;
#[repr(C)]
struct hashmap_entry {
    pvalue: *mut c_void,
}
#[repr(C)]
struct bpf_prog_linfo;
#[repr(C)]
struct dump_data {
    nr_jited_ksyms: __u32,
    jited_ksyms: *mut __u64,
    btf: *mut btf,
    func_info: *mut c_void,
    finfo_rec_size: __u32,
    prog_linfo: *mut bpf_prog_linfo,
}
#[repr(C)]
struct kernel_sym {
    name: *const c_char,
}
#[repr(C)]
struct bpf_func_info {
    insn_off: __u32,
    type_id: __u32,
}
#[repr(C)]
struct bpf_perf_event_value {
    counter: __u64,
    enabled: __u64,
    running: __u64,
}
#[repr(C)]
struct perf_event_attr {
    type_: __u32,
    config: __u64,
    exclude_user: __u64,
}
#[repr(C)]
struct bpf_program;
#[repr(C)]
struct bpf_object;
#[repr(C)]
struct bpf_map;
#[repr(C)]
struct bpf_link;
#[repr(C)]
struct gen_loader_opts {
    data: *mut c_void,
    data_sz: __u32,
    insns: *mut c_void,
    insns_sz: __u32,
    gen_hash: bool,
}
#[repr(C)]
struct bpf_load_and_run_opts {
    ctx: *mut bpf_loader_ctx,
    data: *mut c_void,
    data_sz: __u32,
    insns: *mut c_void,
    insns_sz: __u32,
    errstr: *const c_char,
    excl_prog_hash: *mut __u8,
    excl_prog_hash_sz: __u32,
    signature: *mut c_char,
    signature_sz: __u32,
    keyring_id: c_int,
}
#[repr(C)]
struct bpf_loader_ctx {
    sz: c_int,
    log_level: c_int,
    log_size: c_int,
    log_buf: c_long,
}
#[repr(C)]
struct bpf_object_open_opts {
    relaxed_maps: bool,
    kernel_log_level: c_int,
    btf_custom_path: *const c_char,
}
#[repr(C)]
struct bpf_test_run_opts {
    data_in: *mut c_void,
    data_out: *mut c_void,
    data_size_in: c_uint,
    data_size_out: c_uint,
    ctx_in: *mut c_void,
    ctx_out: *mut c_void,
    ctx_size_in: c_uint,
    ctx_size_out: c_uint,
    retval: c_uint,
    duration: c_uint,
    repeat: c_uint,
}
#[repr(C)]
struct profiler_bpf {
    obj: *mut bpf_object,
    maps: profiler_bpf_maps,
    rodata: *mut profiler_bpf_rodata,
}
#[repr(C)]
struct profiler_bpf_maps {
    events: *mut bpf_map,
    fentry_readings: *mut bpf_map,
    accum_readings: *mut bpf_map,
    counts: *mut bpf_map,
}
#[repr(C)]
struct profiler_bpf_rodata {
    num_cpu: __u32,
    num_metric: __u32,
}
#[repr(C)]
struct cmd {
    cmd: *const c_char,
    func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}
#[repr(C)]
struct map_replace {
    idx: c_int,
    fd: c_int,
    name: *mut c_char,
}
#[repr(C)]
struct profile_metric {
    name: *const c_char,
    val: bpf_perf_event_value,
    attr: perf_event_attr,
    selected: bool,
    /* calculate ratios like instructions per cycle */
    ratio_metric: c_int, /* 0 for N/A, 1 for index 0 (cycles) */
    ratio_desc: *const c_char,
    ratio_mul: f32,
}

unsafe extern "C" {
    static mut json_output: bool;
    static mut json_wtr: *mut json_writer_t;
    static mut refs_table: *mut c_void;
    static mut show_pinned: bool;
    static mut relaxed_maps: bool;
    static mut verifier_logs: bool;
    static mut use_loader: bool;
    static mut sign_progs: bool;
    static mut cert_path: *const c_char;
    static mut bin_name: *const c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut stdin: *mut FILE;
    static mut errno: c_int;

    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strstr(a: *const c_char, b: *const c_char) -> *mut c_char;
    fn strlen(a: *const c_char) -> size_t;
    fn strcpy(a: *mut c_char, b: *const c_char) -> *mut c_char;
    fn strcat(a: *mut c_char, b: *const c_char) -> *mut c_char;
    fn strdup(a: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn malloc(n: size_t) -> *mut c_void;
    fn calloc(n: size_t, sz: size_t) -> *mut c_void;
    fn realloc(p: *mut c_void, n: size_t) -> *mut c_void;
    fn free(p: *mut c_void);
    fn alloca(n: size_t) -> *mut c_void;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
    fn clock_gettime(clk: c_int, ts: *mut timespec) -> c_int;
    fn localtime_r(t: *const time_t, tm: *mut tm) -> *mut tm;
    fn strftime(s: *mut c_char, max: size_t, fmt: *const c_char, tm: *const tm) -> size_t;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(f: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn feof(stream: *mut FILE) -> c_int;
    fn ferror(stream: *mut FILE) -> c_int;
    fn perror(s: *const c_char);
    fn open(path: *const c_char, flags: c_int, mode: c_uint) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn unlink(path: *const c_char) -> c_int;
    fn sleep(secs: c_uint) -> c_uint;
    fn signal(sig: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn syscall(num: c_long, ...) -> c_long;
    fn ioctl(fd: c_int, req: c_ulong, ...) -> c_int;
    fn if_nametoindex(name: *const c_char) -> c_uint;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut c_void;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn atoll(nptr: *const c_char) -> i64;
    fn strerror(errnum: c_int) -> *mut c_char;

    fn ptr_to_u64(ptr: *const c_void) -> __u64;
    fn u64_to_ptr(val: __u64) -> *mut c_void;
    fn is_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
    fn detect_common_prefix(arg: *const c_char, ...) -> bool;
    fn usage() -> !;
    fn BAD_ARG() -> c_int;
    fn REQ_ARGS(n: c_int) -> bool;
    fn NEXT_ARG();
    fn GET_ARG() -> *mut c_char;
    fn p_err(fmt: *const c_char, ...);
    fn p_info(fmt: *const c_char, ...);

    fn libbpf_bpf_attach_type_str(t: __u32) -> *const c_char;
    fn libbpf_bpf_prog_type_str(t: __u32) -> *const c_char;
    fn libbpf_prog_type_by_name(name: *const c_char, prog_type: *mut __u32, expected: *mut __u32) -> c_int;
    fn libbpf_set_print(cb: *mut c_void) -> *mut c_void;
    fn print_all_levels() -> c_int;
    fn libbpf_reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn libbpf_num_possible_cpus() -> c_int;

    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, len: *mut __u32) -> c_int;
    fn bpf_prog_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, len: *mut __u32) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_prog_attach(progfd: c_int, targetfd: c_int, attach_type: __u32, flags: __u32) -> c_int;
    fn bpf_prog_detach2(progfd: c_int, targetfd: c_int, attach_type: __u32) -> c_int;
    fn bpf_prog_stream_read(prog_fd: c_int, stream_id: c_int, buf: *mut c_void, size: size_t, flags: *mut c_void) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_load_and_run(opts: *mut bpf_load_and_run_opts) -> c_int;

    fn btf__load_from_kernel_by_id(id: __u32) -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__type_by_id(btf: *mut btf, id: __u32) -> *const btf_type;
    fn btf__name_by_offset(btf: *mut btf, off: __u32) -> *const c_char;
    fn btf_is_datasec(t: *const btf_type) -> bool;
    fn btf_vlen(t: *const btf_type) -> c_uint;
    fn btf_var_secinfos(t: *const btf_type) -> *mut btf_var_secinfo;
    fn btf_dumper_type(d: *mut btf_dumper, type_id: __u32, data: *mut c_void) -> c_int;
    fn btf_dumper_type_only(btf: *mut btf, type_id: __u32, buf: *mut c_char, sz: size_t) -> c_int;

    fn hashmap__new(hash: *mut c_void, equal: *mut c_void, ctx: *mut c_void) -> *mut hashmap;
    fn hashmap__empty(map: *mut hashmap) -> bool;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn hashmap__for_each_key_entry(map: *mut hashmap, entry: *mut *mut hashmap_entry, key: __u32) -> bool;

    fn jsonw_name(w: *mut json_writer_t, name: *const c_char);
    fn jsonw_start_array(w: *mut json_writer_t);
    fn jsonw_end_array(w: *mut json_writer_t);
    fn jsonw_start_object(w: *mut json_writer_t);
    fn jsonw_end_object(w: *mut json_writer_t);
    fn jsonw_uint(w: *mut json_writer_t, v: c_uint);
    fn jsonw_uint_field(w: *mut json_writer_t, name: *const c_char, v: __u64);
    fn jsonw_int_field(w: *mut json_writer_t, name: *const c_char, v: i64);
    fn jsonw_lluint_field(w: *mut json_writer_t, name: *const c_char, v: __u64);
    fn jsonw_string(w: *mut json_writer_t, s: *const c_char);
    fn jsonw_string_field(w: *mut json_writer_t, name: *const c_char, s: *const c_char);
    fn jsonw_bool_field(w: *mut json_writer_t, name: *const c_char, b: bool);
    fn jsonw_printf(w: *mut json_writer_t, fmt: *const c_char, ...);
    fn jsonw_null(w: *mut json_writer_t);
    fn jsonw_new(f: *mut FILE) -> *mut json_writer_t;
    fn jsonw_reset(w: *mut json_writer_t);
    fn jsonw_destroy(w: *mut *mut json_writer_t);

    fn print_dev_json(ifindex: __u32, netns_dev: __u64, netns_ino: __u64);
    fn print_dev_plain(ifindex: __u32, netns_dev: __u64, netns_ino: __u64);
    fn get_prog_full_name(info: *mut bpf_prog_info, fd: c_int, buf: *mut c_char, len: size_t);
    fn fprint_hex(f: *mut FILE, data: *const __u8, len: size_t, sep: *const c_char);
    fn get_fdinfo(fd: c_int, key: *const c_char) -> *mut c_char;
    fn emit_obj_refs_json(table: *mut c_void, id: __u32, w: *mut json_writer_t);
    fn emit_obj_refs_plain(table: *mut c_void, id: __u32, prefix: *const c_char);
    fn build_pinned_obj_table(map: *mut hashmap, obj_type: c_int);
    fn delete_pinned_obj_table(map: *mut hashmap);
    fn build_obj_refs_table(table: *mut *mut c_void, obj_type: c_int);
    fn delete_obj_refs_table(table: *mut c_void);
    fn prog_parse_fds(argc: *mut c_int, argv: *mut *mut *mut c_char, fds: *mut *mut c_int) -> c_int;
    fn prog_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int;
    fn map_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char, inner: c_int) -> c_int;
    fn do_pin_any(argc: c_int, argv: *mut *mut c_char, parser: unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char) -> c_int) -> c_int;
    fn do_tracelog(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn print_data_json(data: *mut c_void, size: c_uint);
    fn set_max_rlimit();
    fn ifindex_to_arch(ifindex: __u32, netns_dev: __u64, netns_ino: __u64, opt: *mut *const c_char) -> *const c_char;
    fn disasm_init() -> c_int;
    fn disasm_print_insn(img: *mut __u8, len: __u32, opcodes: bool, name: *const c_char, opt: *const c_char, btf: *mut btf, linfo: *mut bpf_prog_linfo, ksym: __u64, idx: __u32, linum: bool) -> c_int;
    fn bpf_prog_linfo__new(info: *mut bpf_prog_info) -> *mut bpf_prog_linfo;
    fn bpf_prog_linfo__free(info: *mut bpf_prog_linfo);
    fn kernel_syms_load(dd: *mut dump_data);
    fn kernel_syms_destroy(dd: *mut dump_data);
    fn kernel_syms_search(dd: *mut dump_data, addr: __u64) -> *mut kernel_sym;
    fn dump_xlated_json(dd: *mut dump_data, buf: *mut __u8, len: __u32, opcodes: bool, linum: bool);
    fn dump_xlated_cfg(dd: *mut dump_data, buf: *mut __u8, len: __u32, opcodes: bool, linum: bool);
    fn dump_xlated_plain(dd: *mut dump_data, buf: *mut __u8, len: __u32, opcodes: bool, linum: bool);
    fn pathname_concat(buf: *mut c_char, size: size_t, path: *const c_char, name: *const c_char) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_program__name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__section_name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_program__set_flags(prog: *mut bpf_program, flags: __u32);
    fn bpf_program__set_ifindex(prog: *mut bpf_program, ifindex: __u32);
    fn bpf_program__type(prog: *mut bpf_program) -> __u32;
    fn bpf_program__set_type(prog: *mut bpf_program, type_: __u32);
    fn bpf_program__set_expected_attach_type(prog: *mut bpf_program, type_: __u32);
    fn bpf_program__set_attach_target(prog: *mut bpf_program, fd: c_int, name: *const c_char) -> c_int;
    fn bpf_program__unpin(prog: *mut bpf_program, path: *const c_char) -> c_int;
    fn bpf_link__pin(link: *mut bpf_link, path: *const c_char) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_obj_pin(fd: c_int, path: *const c_char) -> c_int;
    fn bpf_object__open_file(file: *const c_char, opts: *mut bpf_object_open_opts) -> *mut bpf_object;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_object__next_program(obj: *mut bpf_object, prog: *mut bpf_program) -> *mut bpf_program;
    fn bpf_object__prev_program(obj: *mut bpf_object, prog: *mut bpf_program) -> *mut bpf_program;
    fn bpf_object__next_map(obj: *mut bpf_object, map: *mut bpf_map) -> *mut bpf_map;
    fn bpf_object__pin_programs(obj: *mut bpf_object, path: *const c_char) -> c_int;
    fn bpf_object__unpin_programs(obj: *mut bpf_object, path: *const c_char) -> c_int;
    fn bpf_object__pin_maps(obj: *mut bpf_object, path: *const c_char) -> c_int;
    fn bpf_object__gen_loader(obj: *mut bpf_object, gen: *mut gen_loader_opts) -> c_int;
    fn bpf_map__name(map: *mut bpf_map) -> *const c_char;
    fn bpf_map__type(map: *mut bpf_map) -> __u32;
    fn bpf_map__set_ifindex(map: *mut bpf_map, ifindex: __u32);
    fn bpf_map__reuse_fd(map: *mut bpf_map, fd: c_int) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, cnt: __u32) -> c_int;
    fn mount_bpffs_for_file(path: *const c_char) -> c_int;
    fn create_and_mount_bpffs_dir(path: *const c_char) -> c_int;
    fn bpftool_prog_sign(opts: *mut bpf_load_and_run_opts) -> c_int;
    fn register_session_key(path: *const c_char) -> c_int;
    fn profiler_bpf__open() -> *mut profiler_bpf;
    fn profiler_bpf__load(obj: *mut profiler_bpf) -> c_int;
    fn profiler_bpf__attach(obj: *mut profiler_bpf) -> c_int;
    fn profiler_bpf__destroy(obj: *mut profiler_bpf);
    fn cmd_select(cmds: *const cmd, argc: c_int, argv: *mut *mut c_char, help: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int) -> c_int;
}

const __MAX_BPF_ATTACH_TYPE: usize = 64;
const BPF_SK_SKB_STREAM_PARSER: usize = 1;
const BPF_SK_SKB_STREAM_VERDICT: usize = 2;
const BPF_SK_SKB_VERDICT: usize = 3;
const BPF_SK_MSG_VERDICT: usize = 4;
const BPF_FLOW_DISSECTOR: usize = 5;
const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const BPF_MAP_TYPE_PERF_EVENT_ARRAY: __u32 = 4;
const BPF_OBJ_PROG: c_int = 0;
const BPF_PROG_TYPE_UNSPEC: __u32 = 0;
const BPF_PROG_TYPE_XDP: __u32 = 6;
const BPF_F_XDP_DEV_BOUND_ONLY: __u32 = 1 << 6;
const PERF_TYPE_HARDWARE: __u32 = 0;
const PERF_TYPE_HW_CACHE: __u32 = 3;
const PERF_COUNT_HW_CPU_CYCLES: __u64 = 0;
const PERF_COUNT_HW_INSTRUCTIONS: __u64 = 1;
const PERF_COUNT_HW_CACHE_L1D: __u64 = 0;
const PERF_COUNT_HW_CACHE_LL: __u64 = 2;
const PERF_COUNT_HW_CACHE_DTLB: __u64 = 3;
const PERF_COUNT_HW_CACHE_ITLB: __u64 = 4;
const PERF_COUNT_HW_CACHE_OP_READ: __u64 = 0;
const PERF_COUNT_HW_CACHE_RESULT_ACCESS: __u64 = 0;
const PERF_COUNT_HW_CACHE_RESULT_MISS: __u64 = 1;
const PERF_EVENT_IOC_ENABLE: c_ulong = 0x2400;
const SIGINT: c_int = 2;
const MAX_PROG_FULL_NAME: usize = 128;
const BPF_TAG_SIZE: usize = 8;
const SYM_MAX_NAME: usize = 256;
const PATH_MAX: usize = 4096;
const MAX_SIG_SIZE: usize = 4096;
const SHA256_DIGEST_LENGTH: usize = 32;
const __NR_perf_event_open: c_long = 298;

static attach_types: [bool; __MAX_BPF_ATTACH_TYPE + 1] = {
    let mut a = [false; __MAX_BPF_ATTACH_TYPE + 1];
    a[BPF_SK_SKB_STREAM_PARSER] = true;
    a[BPF_SK_SKB_STREAM_VERDICT] = true;
    a[BPF_SK_SKB_VERDICT] = true;
    a[BPF_SK_MSG_VERDICT] = true;
    a[BPF_FLOW_DISSECTOR] = true;
    a
};

/* Textual representations traditionally used by the program and kept around
 * for the sake of backwards compatibility.
 */
static attach_type_strings: [*const c_char; __MAX_BPF_ATTACH_TYPE + 1] = {
    let mut a = [null(); __MAX_BPF_ATTACH_TYPE + 1];
    a[BPF_SK_SKB_STREAM_PARSER] = b"stream_parser\0".as_ptr() as *const c_char;
    a[BPF_SK_SKB_STREAM_VERDICT] = b"stream_verdict\0".as_ptr() as *const c_char;
    a[BPF_SK_SKB_VERDICT] = b"skb_verdict\0".as_ptr() as *const c_char;
    a[BPF_SK_MSG_VERDICT] = b"msg_verdict\0".as_ptr() as *const c_char;
    a
};

static mut prog_table: *mut hashmap = null_mut();

unsafe extern "C" fn parse_attach_type(str_: *const c_char) -> __u32 {
    let mut type_: __u32 = 0;
    while (type_ as usize) < __MAX_BPF_ATTACH_TYPE {
        if attach_types[type_ as usize] {
            let attach_type_str = libbpf_bpf_attach_type_str(type_);
            if strcmp(str_, attach_type_str) == 0 {
                return type_;
            }
        }
        if !attach_type_strings[type_ as usize].is_null()
            && is_prefix(str_, attach_type_strings[type_ as usize])
        {
            return type_;
        }
        type_ += 1;
    }
    __MAX_BPF_ATTACH_TYPE as __u32
}

unsafe extern "C" fn prep_prog_info(
    info: *mut bpf_prog_info,
    mode: dump_mode,
    info_data: *mut *mut c_void,
    info_data_sz: *mut size_t,
) -> c_int {
    let mut holder: bpf_prog_info = zeroed();
    let mut needed: size_t = 0;
    let mut ptr: *mut c_void;

    if mode == dump_mode::DUMP_JITED {
        holder.jited_prog_len = (*info).jited_prog_len;
        needed += (*info).jited_prog_len as size_t;
    } else {
        holder.xlated_prog_len = (*info).xlated_prog_len;
        needed += (*info).xlated_prog_len as size_t;
    }
    holder.nr_jited_ksyms = (*info).nr_jited_ksyms;
    needed += holder.nr_jited_ksyms as size_t * size_of::<__u64>();
    holder.nr_jited_func_lens = (*info).nr_jited_func_lens;
    needed += holder.nr_jited_func_lens as size_t * size_of::<__u32>();
    holder.nr_func_info = (*info).nr_func_info;
    holder.func_info_rec_size = (*info).func_info_rec_size;
    needed += holder.nr_func_info as size_t * holder.func_info_rec_size as size_t;
    holder.nr_line_info = (*info).nr_line_info;
    holder.line_info_rec_size = (*info).line_info_rec_size;
    needed += holder.nr_line_info as size_t * holder.line_info_rec_size as size_t;
    holder.nr_jited_line_info = (*info).nr_jited_line_info;
    holder.jited_line_info_rec_size = (*info).jited_line_info_rec_size;
    needed += holder.nr_jited_line_info as size_t * holder.jited_line_info_rec_size as size_t;

    if needed > *info_data_sz {
        ptr = realloc(*info_data, needed);
        if ptr.is_null() {
            return -1;
        }
        *info_data = ptr;
        *info_data_sz = needed;
    }
    ptr = *info_data;
    if mode == dump_mode::DUMP_JITED {
        holder.jited_prog_insns = ptr_to_u64(ptr);
        ptr = (ptr as *mut u8).add(holder.jited_prog_len as usize) as *mut c_void;
    } else {
        holder.xlated_prog_insns = ptr_to_u64(ptr);
        ptr = (ptr as *mut u8).add(holder.xlated_prog_len as usize) as *mut c_void;
    }
    holder.jited_ksyms = ptr_to_u64(ptr);
    ptr = (ptr as *mut u8).add(holder.nr_jited_ksyms as usize * size_of::<__u64>()) as *mut c_void;
    holder.jited_func_lens = ptr_to_u64(ptr);
    ptr = (ptr as *mut u8).add(holder.nr_jited_func_lens as usize * size_of::<__u32>()) as *mut c_void;
    holder.func_info = ptr_to_u64(ptr);
    ptr = (ptr as *mut u8).add(holder.nr_func_info as usize * holder.func_info_rec_size as usize) as *mut c_void;
    holder.line_info = ptr_to_u64(ptr);
    ptr = (ptr as *mut u8).add(holder.nr_line_info as usize * holder.line_info_rec_size as usize) as *mut c_void;
    holder.jited_line_info = ptr_to_u64(ptr);
    *info = holder;
    0
}

unsafe extern "C" fn print_boot_time(nsecs: __u64, buf: *mut c_char, mut size: c_uint) {
    let mut real_time_ts: timespec = zeroed();
    let mut boot_time_ts: timespec = zeroed();
    let mut load_tm: tm = zeroed();
    size -= 1;
    *buf.add(size as usize) = 0;
    if clock_gettime(CLOCK_REALTIME, &mut real_time_ts) != 0
        || clock_gettime(CLOCK_BOOTTIME, &mut boot_time_ts) != 0
    {
        perror(b"Can't read clocks\0".as_ptr() as *const c_char);
        snprintf(buf, size as usize, b"%llu\0".as_ptr() as *const c_char, nsecs / 1000000000);
        return;
    }
    let wallclock_secs: time_t = (real_time_ts.tv_sec - boot_time_ts.tv_sec)
        + ((real_time_ts.tv_nsec - boot_time_ts.tv_nsec) as __u64 + nsecs) as time_t / 1000000000;
    if localtime_r(&wallclock_secs, &mut load_tm).is_null() {
        snprintf(buf, size as usize, b"%llu\0".as_ptr() as *const c_char, nsecs / 1000000000);
        return;
    }
    if json_output {
        strftime(buf, size as usize, b"%s\0".as_ptr() as *const c_char, &load_tm);
    } else {
        strftime(buf, size as usize, b"%FT%T%z\0".as_ptr() as *const c_char, &load_tm);
    }
}

unsafe extern "C" fn show_prog_maps(fd: c_int, num_maps: __u32) {
    let mut info: bpf_prog_info = zeroed();
    let mut len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let mut map_ids = vec![0u32; num_maps as usize];
    info.nr_map_ids = num_maps;
    info.map_ids = ptr_to_u64(map_ids.as_mut_ptr() as *mut c_void);
    if bpf_prog_get_info_by_fd(fd, &mut info, &mut len) != 0 || info.nr_map_ids == 0 {
        return;
    }
    if json_output {
        jsonw_name(json_wtr, b"map_ids\0".as_ptr() as *const c_char);
        jsonw_start_array(json_wtr);
        for i in 0..info.nr_map_ids as usize {
            jsonw_uint(json_wtr, map_ids[i]);
        }
        jsonw_end_array(json_wtr);
    } else {
        printf(b"  map_ids \0".as_ptr() as *const c_char);
        for i in 0..info.nr_map_ids as usize {
            printf(
                b"%u%s\0".as_ptr() as *const c_char,
                map_ids[i],
                if i == info.nr_map_ids as usize - 1 { b"\0".as_ptr() } else { b",\0".as_ptr() },
            );
        }
    }
}

unsafe extern "C" fn find_metadata(prog_fd: c_int, map_info: *mut bpf_map_info) -> *mut c_void {
    let mut prog_info: bpf_prog_info = zeroed();
    let mut prog_info_len = size_of::<bpf_prog_info>() as __u32;
    let mut key: c_int = 0;
    if bpf_prog_get_info_by_fd(prog_fd, &mut prog_info, &mut prog_info_len) != 0 || prog_info.nr_map_ids == 0 {
        return null_mut();
    }
    let map_ids = calloc(prog_info.nr_map_ids as usize, size_of::<__u32>()) as *mut __u32;
    if map_ids.is_null() {
        return null_mut();
    }
    let nr_maps = prog_info.nr_map_ids;
    memset(&mut prog_info as *mut _ as *mut c_void, 0, size_of::<bpf_prog_info>());
    prog_info.nr_map_ids = nr_maps;
    prog_info.map_ids = ptr_to_u64(map_ids as *mut c_void);
    prog_info_len = size_of::<bpf_prog_info>() as __u32;
    let mut value: *mut c_void = null_mut();
    if bpf_prog_get_info_by_fd(prog_fd, &mut prog_info, &mut prog_info_len) == 0 {
        for i in 0..prog_info.nr_map_ids {
            let map_fd = bpf_map_get_fd_by_id(*map_ids.add(i as usize));
            if map_fd < 0 {
                break;
            }
            memset(map_info as *mut c_void, 0, size_of::<bpf_map_info>());
            let mut map_info_len = size_of::<bpf_map_info>() as __u32;
            if bpf_map_get_info_by_fd(map_fd, map_info, &mut map_info_len) < 0 {
                close(map_fd);
                break;
            }
            if (*map_info).type_ != BPF_MAP_TYPE_ARRAY
                || (*map_info).key_size as usize != size_of::<c_int>()
                || (*map_info).max_entries != 1
                || (*map_info).btf_value_type_id == 0
                || strstr((*map_info).name.as_ptr(), b".rodata\0".as_ptr() as *const c_char).is_null()
            {
                close(map_fd);
                continue;
            }
            value = malloc((*map_info).value_size as usize);
            if value.is_null() {
                close(map_fd);
                break;
            }
            if bpf_map_lookup_elem(map_fd, &mut key as *mut _ as *const c_void, value) != 0 {
                close(map_fd);
                free(value);
                value = null_mut();
                break;
            }
            close(map_fd);
            break;
        }
    }
    free(map_ids as *mut c_void);
    value
}

unsafe extern "C" fn has_metadata_prefix(s: *const c_char) -> bool {
    strncmp(s, BPF_METADATA_PREFIX.as_ptr() as *const c_char, BPF_METADATA_PREFIX_LEN) == 0
}

unsafe extern "C" fn show_prog_metadata(fd: c_int, num_maps: __u32) {
    if num_maps == 0 {
        return;
    }
    let mut map_info: bpf_map_info = zeroed();
    let value = find_metadata(fd, &mut map_info);
    if value.is_null() {
        return;
    }
    let btfp = btf__load_from_kernel_by_id(map_info.btf_id);
    if btfp.is_null() {
        free(value);
        return;
    }
    let t_datasec = btf__type_by_id(btfp, map_info.btf_value_type_id);
    if !btf_is_datasec(t_datasec) {
        btf__free(btfp);
        free(value);
        return;
    }
    let vlen = btf_vlen(t_datasec);
    let mut vsi = btf_var_secinfos(t_datasec);
    let mut printed_header = false;
    if json_output {
        let mut d = btf_dumper { btf: btfp, jw: json_wtr, is_plain_text: false };
        for _ in 0..vlen {
            let t_var = btf__type_by_id(btfp, (*vsi).type_);
            let name = btf__name_by_offset(btfp, (*t_var).name_off);
            if has_metadata_prefix(name) {
                if !printed_header {
                    jsonw_name(json_wtr, b"metadata\0".as_ptr() as *const c_char);
                    jsonw_start_object(json_wtr);
                    printed_header = true;
                }
                jsonw_name(json_wtr, name.add(BPF_METADATA_PREFIX_LEN));
                let err = btf_dumper_type(&mut d, (*t_var).type_, (value as *mut u8).add((*vsi).offset as usize) as *mut c_void);
                if err != 0 {
                    p_err(b"btf dump failed: %d\0".as_ptr() as *const c_char, err);
                    break;
                }
            }
            vsi = vsi.add(1);
        }
        if printed_header {
            jsonw_end_object(json_wtr);
        }
    } else {
        let mut btf_wtr: *mut json_writer_t = null_mut();
        let mut d = btf_dumper { btf: btfp, jw: null_mut(), is_plain_text: true };
        for _ in 0..vlen {
            let t_var = btf__type_by_id(btfp, (*vsi).type_);
            let name = btf__name_by_offset(btfp, (*t_var).name_off);
            if has_metadata_prefix(name) {
                if !printed_header {
                    printf(b"\tmetadata:\0".as_ptr() as *const c_char);
                    btf_wtr = jsonw_new(stdout);
                    if btf_wtr.is_null() {
                        p_err(b"jsonw alloc failed\0".as_ptr() as *const c_char);
                        break;
                    }
                    d.jw = btf_wtr;
                    printed_header = true;
                }
                printf(b"\n\t\t%s = \0".as_ptr() as *const c_char, name.add(BPF_METADATA_PREFIX_LEN));
                jsonw_reset(btf_wtr);
                let err = btf_dumper_type(&mut d, (*t_var).type_, (value as *mut u8).add((*vsi).offset as usize) as *mut c_void);
                if err != 0 {
                    p_err(b"btf dump failed: %d\0".as_ptr() as *const c_char, err);
                    break;
                }
            }
            vsi = vsi.add(1);
        }
        if printed_header {
            jsonw_destroy(&mut btf_wtr);
        }
    }
    btf__free(btfp);
    free(value);
}

unsafe extern "C" fn print_prog_header_json(info: *mut bpf_prog_info, fd: c_int) {
    let mut prog_name = [0 as c_char; MAX_PROG_FULL_NAME];
    jsonw_uint_field(json_wtr, b"id\0".as_ptr() as *const c_char, (*info).id as __u64);
    let prog_type_str = libbpf_bpf_prog_type_str((*info).type_);
    if !prog_type_str.is_null() {
        jsonw_string_field(json_wtr, b"type\0".as_ptr() as *const c_char, prog_type_str);
    } else {
        jsonw_uint_field(json_wtr, b"type\0".as_ptr() as *const c_char, (*info).type_ as __u64);
    }
    if (*info).name[0] != 0 {
        get_prog_full_name(info, fd, prog_name.as_mut_ptr(), prog_name.len());
        jsonw_string_field(json_wtr, b"name\0".as_ptr() as *const c_char, prog_name.as_ptr());
    }
    jsonw_name(json_wtr, b"tag\0".as_ptr() as *const c_char);
    jsonw_printf(json_wtr, b"\"%02hhx%02hhx%02hhx%02hhx%02hhx%02hhx%02hhx%02hhx\"\0".as_ptr() as *const c_char,
                 (*info).tag[0] as c_int, (*info).tag[1] as c_int, (*info).tag[2] as c_int, (*info).tag[3] as c_int,
                 (*info).tag[4] as c_int, (*info).tag[5] as c_int, (*info).tag[6] as c_int, (*info).tag[7] as c_int);
    jsonw_bool_field(json_wtr, b"gpl_compatible\0".as_ptr() as *const c_char, (*info).gpl_compatible != 0);
    if (*info).run_time_ns != 0 {
        jsonw_uint_field(json_wtr, b"run_time_ns\0".as_ptr() as *const c_char, (*info).run_time_ns);
        jsonw_uint_field(json_wtr, b"run_cnt\0".as_ptr() as *const c_char, (*info).run_cnt);
    }
    if (*info).recursion_misses != 0 {
        jsonw_uint_field(json_wtr, b"recursion_misses\0".as_ptr() as *const c_char, (*info).recursion_misses);
    }
}

unsafe extern "C" fn print_prog_json(info: *mut bpf_prog_info, fd: c_int, orphaned: bool) {
    jsonw_start_object(json_wtr);
    print_prog_header_json(info, fd);
    print_dev_json((*info).ifindex, (*info).netns_dev, (*info).netns_ino);
    if (*info).load_time != 0 {
        let mut buf = [0 as c_char; 32];
        print_boot_time((*info).load_time, buf.as_mut_ptr(), buf.len() as c_uint);
        jsonw_name(json_wtr, b"loaded_at\0".as_ptr() as *const c_char);
        jsonw_printf(json_wtr, b"%s\0".as_ptr() as *const c_char, buf.as_ptr());
        jsonw_uint_field(json_wtr, b"uid\0".as_ptr() as *const c_char, (*info).created_by_uid as __u64);
    }
    jsonw_bool_field(json_wtr, b"orphaned\0".as_ptr() as *const c_char, orphaned);
    jsonw_uint_field(json_wtr, b"bytes_xlated\0".as_ptr() as *const c_char, (*info).xlated_prog_len as __u64);
    if (*info).jited_prog_len != 0 {
        jsonw_bool_field(json_wtr, b"jited\0".as_ptr() as *const c_char, true);
        jsonw_uint_field(json_wtr, b"bytes_jited\0".as_ptr() as *const c_char, (*info).jited_prog_len as __u64);
    } else {
        jsonw_bool_field(json_wtr, b"jited\0".as_ptr() as *const c_char, false);
    }
    let memlock = get_fdinfo(fd, b"memlock\0".as_ptr() as *const c_char);
    if !memlock.is_null() {
        jsonw_int_field(json_wtr, b"bytes_memlock\0".as_ptr() as *const c_char, atoll(memlock));
    }
    free(memlock as *mut c_void);
    if (*info).nr_map_ids != 0 { show_prog_maps(fd, (*info).nr_map_ids); }
    if (*info).btf_id != 0 { jsonw_int_field(json_wtr, b"btf_id\0".as_ptr() as *const c_char, (*info).btf_id as i64); }
    if !hashmap__empty(prog_table) {
        let mut entry: *mut hashmap_entry = null_mut();
        jsonw_name(json_wtr, b"pinned\0".as_ptr() as *const c_char);
        jsonw_start_array(json_wtr);
        while hashmap__for_each_key_entry(prog_table, &mut entry, (*info).id) {
            jsonw_string(json_wtr, (*entry).pvalue as *const c_char);
        }
        jsonw_end_array(json_wtr);
    }
    emit_obj_refs_json(refs_table, (*info).id, json_wtr);
    show_prog_metadata(fd, (*info).nr_map_ids);
    jsonw_end_object(json_wtr);
}

unsafe extern "C" fn print_prog_header_plain(info: *mut bpf_prog_info, fd: c_int) {
    let mut prog_name = [0 as c_char; MAX_PROG_FULL_NAME];
    printf(b"%u: \0".as_ptr() as *const c_char, (*info).id);
    let prog_type_str = libbpf_bpf_prog_type_str((*info).type_);
    if !prog_type_str.is_null() {
        printf(b"%s  \0".as_ptr() as *const c_char, prog_type_str);
    } else {
        printf(b"type %u  \0".as_ptr() as *const c_char, (*info).type_);
    }
    if (*info).name[0] != 0 {
        get_prog_full_name(info, fd, prog_name.as_mut_ptr(), prog_name.len());
        printf(b"name %s  \0".as_ptr() as *const c_char, prog_name.as_ptr());
    }
    printf(b"tag \0".as_ptr() as *const c_char);
    fprint_hex(stdout, (*info).tag.as_ptr(), BPF_TAG_SIZE, b"\0".as_ptr() as *const c_char);
    print_dev_plain((*info).ifindex, (*info).netns_dev, (*info).netns_ino);
    printf(if (*info).gpl_compatible != 0 { b"  gpl\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char });
    if (*info).run_time_ns != 0 {
        printf(b" run_time_ns %llu run_cnt %llu\0".as_ptr() as *const c_char, (*info).run_time_ns, (*info).run_cnt);
    }
    if (*info).recursion_misses != 0 {
        printf(b" recursion_misses %llu\0".as_ptr() as *const c_char, (*info).recursion_misses);
    }
    printf(b"\n\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn print_prog_plain(info: *mut bpf_prog_info, fd: c_int, orphaned: bool) {
    print_prog_header_plain(info, fd);
    if (*info).load_time != 0 {
        let mut buf = [0 as c_char; 32];
        print_boot_time((*info).load_time, buf.as_mut_ptr(), buf.len() as c_uint);
        printf(b"\tloaded_at %s  uid %u\n\0".as_ptr() as *const c_char, buf.as_ptr(), (*info).created_by_uid);
    }
    printf(b"\txlated %uB\0".as_ptr() as *const c_char, (*info).xlated_prog_len);
    if (*info).jited_prog_len != 0 {
        printf(b"  jited %uB\0".as_ptr() as *const c_char, (*info).jited_prog_len);
    } else {
        printf(b"  not jited\0".as_ptr() as *const c_char);
    }
    let memlock = get_fdinfo(fd, b"memlock\0".as_ptr() as *const c_char);
    if !memlock.is_null() { printf(b"  memlock %sB\0".as_ptr() as *const c_char, memlock); }
    free(memlock as *mut c_void);
    if orphaned { printf(b"  orphaned\0".as_ptr() as *const c_char); }
    if (*info).nr_map_ids != 0 { show_prog_maps(fd, (*info).nr_map_ids); }
    if !hashmap__empty(prog_table) {
        let mut entry: *mut hashmap_entry = null_mut();
        while hashmap__for_each_key_entry(prog_table, &mut entry, (*info).id) {
            printf(b"\n\tpinned %s\0".as_ptr() as *const c_char, (*entry).pvalue as *mut c_char);
        }
    }
    if (*info).btf_id != 0 { printf(b"\n\tbtf_id %u\0".as_ptr() as *const c_char, (*info).btf_id); }
    emit_obj_refs_plain(refs_table, (*info).id, b"\n\tpids \0".as_ptr() as *const c_char);
    printf(b"\n\0".as_ptr() as *const c_char);
    show_prog_metadata(fd, (*info).nr_map_ids);
}

unsafe extern "C" fn show_prog(fd: c_int) -> c_int {
    let mut info: bpf_prog_info = zeroed();
    let mut len = size_of::<bpf_prog_info>() as __u32;
    let err = bpf_prog_get_info_by_fd(fd, &mut info, &mut len);
    if err != 0 && err != -ENODEV {
        p_err(b"can't get prog info: %s\0".as_ptr() as *const c_char, strerror(errno));
        return -1;
    }
    if json_output { print_prog_json(&mut info, fd, err == -ENODEV); } else { print_prog_plain(&mut info, fd, err == -ENODEV); }
    0
}

unsafe extern "C" fn do_show_subset(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut fds = malloc(size_of::<c_int>()) as *mut c_int;
    let mut err = -1;
    if fds.is_null() { p_err(b"mem alloc failed\0".as_ptr() as *const c_char); return -1; }
    let nb_fds = prog_parse_fds(&mut argc, &mut argv, &mut fds);
    if nb_fds >= 1 {
        if json_output && nb_fds > 1 { jsonw_start_array(json_wtr); }
        let mut i = 0;
        while i < nb_fds {
            err = show_prog(*fds.add(i as usize));
            if err != 0 {
                while i < nb_fds { close(*fds.add(i as usize)); i += 1; }
                break;
            }
            close(*fds.add(i as usize));
            i += 1;
        }
        if json_output && nb_fds > 1 { jsonw_end_array(json_wtr); }
    }
    free(fds as *mut c_void);
    err
}

unsafe extern "C" fn do_show(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut id: __u32 = 0;
    let mut err: c_int;
    if show_pinned {
        prog_table = hashmap__new(null_mut(), null_mut(), null_mut());
        if IS_ERR(prog_table as *const c_void) {
            p_err(b"failed to create hashmap for pinned paths\0".as_ptr() as *const c_char);
            return -1;
        }
        build_pinned_obj_table(prog_table, BPF_OBJ_PROG);
    }
    build_obj_refs_table(&mut refs_table, BPF_OBJ_PROG);
    if argc == 2 { return do_show_subset(argc, argv); }
    if argc != 0 { return BAD_ARG(); }
    if json_output { jsonw_start_array(json_wtr); }
    loop {
        err = bpf_prog_get_next_id(id, &mut id);
        if err != 0 {
            if errno == ENOENT { err = 0; break; }
            p_err(b"can't get next program: %s%s\0".as_ptr() as *const c_char, strerror(errno),
                  if errno == EINVAL { b" -- kernel too old?\0".as_ptr() } else { b"\0".as_ptr() });
            err = -1; break;
        }
        let fd = bpf_prog_get_fd_by_id(id);
        if fd < 0 {
            if errno == ENOENT { continue; }
            p_err(b"can't get prog by id (%u): %s\0".as_ptr() as *const c_char, id, strerror(errno));
            err = -1; break;
        }
        err = show_prog(fd);
        close(fd);
        if err != 0 { break; }
    }
    if json_output { jsonw_end_array(json_wtr); }
    delete_obj_refs_table(refs_table);
    if show_pinned { delete_pinned_obj_table(prog_table); }
    err
}

unsafe extern "C" fn prog_dump(info: *mut bpf_prog_info, mode: dump_mode, filepath: *mut c_char, opcodes: bool, visual: bool, linum: bool) -> c_int {
    let mut prog_linfo: *mut bpf_prog_linfo = null_mut();
    let mut disasm_opt: *const c_char = null();
    let mut dd: dump_data = zeroed();
    let mut btfp: *mut btf = null_mut();
    let mut err = -1;
    let (buf, member_len) = if mode == dump_mode::DUMP_JITED {
        if (*info).jited_prog_len == 0 || (*info).jited_prog_insns == 0 {
            p_err(b"error retrieving jit dump: no instructions returned or kernel.kptr_restrict set?\0".as_ptr() as *const c_char);
            return -1;
        }
        (u64_to_ptr((*info).jited_prog_insns) as *mut __u8, (*info).jited_prog_len)
    } else {
        if (*info).xlated_prog_len == 0 || (*info).xlated_prog_insns == 0 {
            p_err(b"error retrieving insn dump: kernel.kptr_restrict set?\0".as_ptr() as *const c_char);
            return -1;
        }
        (u64_to_ptr((*info).xlated_prog_insns) as *mut __u8, (*info).xlated_prog_len)
    };
    if (*info).btf_id != 0 {
        btfp = btf__load_from_kernel_by_id((*info).btf_id);
        if btfp.is_null() { p_err(b"failed to get btf\0".as_ptr() as *const c_char); return -1; }
    }
    let func_info = u64_to_ptr((*info).func_info);
    if (*info).nr_line_info != 0 {
        prog_linfo = bpf_prog_linfo__new(info);
        if prog_linfo.is_null() { p_info(b"error in processing bpf_line_info.  continue without it.\0".as_ptr() as *const c_char); }
    }
    if !filepath.is_null() {
        let fd = open(filepath, O_WRONLY | O_CREAT | O_TRUNC, 0o600);
        if fd < 0 {
            p_err(b"can't open file %s: %s\0".as_ptr() as *const c_char, filepath, strerror(errno));
        } else {
            let n = write(fd, buf as *const c_void, member_len as size_t);
            close(fd);
            if n != member_len as ssize_t {
                p_err(b"error writing output file: %s\0".as_ptr() as *const c_char,
                      if n < 0 { strerror(errno) as *const c_char } else { b"short write\0".as_ptr() as *const c_char });
            } else {
                if json_output { jsonw_null(json_wtr); }
                err = 0;
            }
        }
    } else if mode == dump_mode::DUMP_JITED {
        let mut name: *const c_char = null();
        if (*info).ifindex != 0 {
            name = ifindex_to_arch((*info).ifindex, (*info).netns_dev, (*info).netns_ino, &mut disasm_opt);
        }
        if !name.is_null() || (*info).ifindex == 0 {
            if (*info).nr_jited_func_lens != 0 && (*info).jited_func_lens != 0 {
                let mut img = buf;
                let mut ksyms: *mut __u64 = null_mut();
                if (*info).nr_jited_ksyms != 0 { kernel_syms_load(&mut dd); ksyms = u64_to_ptr((*info).jited_ksyms) as *mut __u64; }
                if json_output { jsonw_start_array(json_wtr); }
                let lens = u64_to_ptr((*info).jited_func_lens) as *mut __u32;
                for i in 0..(*info).nr_jited_func_lens {
                    let mut sym_name = [0 as c_char; SYM_MAX_NAME];
                    let mut func_sig = [0 as c_char; 1024];
                    if !ksyms.is_null() {
                        let sym = kernel_syms_search(&mut dd, *ksyms.add(i as usize));
                        if !sym.is_null() { sprintf(sym_name.as_mut_ptr(), b"%s\0".as_ptr() as *const c_char, (*sym).name); }
                        else { sprintf(sym_name.as_mut_ptr(), b"0x%016llx\0".as_ptr() as *const c_char, *ksyms.add(i as usize)); }
                    } else {
                        strcpy(sym_name.as_mut_ptr(), b"unknown\0".as_ptr() as *const c_char);
                    }
                    if !func_info.is_null() {
                        let record = (func_info as *mut u8).add(i as usize * (*info).func_info_rec_size as usize) as *mut bpf_func_info;
                        btf_dumper_type_only(btfp, (*record).type_id, func_sig.as_mut_ptr(), func_sig.len());
                    }
                    if json_output {
                        jsonw_start_object(json_wtr);
                        if !func_info.is_null() && func_sig[0] != 0 {
                            jsonw_name(json_wtr, b"proto\0".as_ptr() as *const c_char);
                            jsonw_string(json_wtr, func_sig.as_ptr());
                        }
                        jsonw_name(json_wtr, b"name\0".as_ptr() as *const c_char);
                        jsonw_string(json_wtr, sym_name.as_ptr());
                        jsonw_name(json_wtr, b"insns\0".as_ptr() as *const c_char);
                    } else {
                        if !func_info.is_null() && func_sig[0] != 0 { printf(b"%s:\n\0".as_ptr() as *const c_char, func_sig.as_ptr()); }
                        printf(b"%s:\n\0".as_ptr() as *const c_char, sym_name.as_ptr());
                    }
                    let derr = if !ksyms.is_null() {
                        disasm_print_insn(img, *lens.add(i as usize), opcodes, name, disasm_opt, btfp, prog_linfo, *ksyms.add(i as usize), i, linum)
                    } else {
                        disasm_print_insn(img, *lens.add(i as usize), opcodes, name, disasm_opt, btfp, null_mut(), 0, 0, false)
                    };
                    if derr != 0 { break; }
                    img = img.add(*lens.add(i as usize) as usize);
                    if json_output { jsonw_end_object(json_wtr); } else { printf(b"\n\0".as_ptr() as *const c_char); }
                }
                if json_output { jsonw_end_array(json_wtr); }
                err = 0;
            } else if disasm_print_insn(buf, member_len, opcodes, name, disasm_opt, btfp, null_mut(), 0, 0, false) == 0 {
                err = 0;
            }
        }
    } else {
        kernel_syms_load(&mut dd);
        dd.nr_jited_ksyms = (*info).nr_jited_ksyms;
        dd.jited_ksyms = u64_to_ptr((*info).jited_ksyms) as *mut __u64;
        dd.btf = btfp;
        dd.func_info = func_info;
        dd.finfo_rec_size = (*info).func_info_rec_size;
        dd.prog_linfo = prog_linfo;
        if json_output { dump_xlated_json(&mut dd, buf, member_len, opcodes, linum); }
        else if visual { dump_xlated_cfg(&mut dd, buf, member_len, opcodes, linum); }
        else { dump_xlated_plain(&mut dd, buf, member_len, opcodes, linum); }
        kernel_syms_destroy(&mut dd);
        err = 0;
    }
    btf__free(btfp);
    bpf_prog_linfo__free(prog_linfo);
    err
}

unsafe extern "C" fn do_dump(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut info: bpf_prog_info = zeroed();
    let mut info_len = size_of::<bpf_prog_info>() as __u32;
    let mut info_data_sz: size_t = 0;
    let mut info_data: *mut c_void = null_mut();
    let mut filepath: *mut c_char = null_mut();
    let mut opcodes = false;
    let mut visual = false;
    let mut linum = false;
    let mode: dump_mode;
    let mut fds = malloc(size_of::<c_int>()) as *mut c_int;
    let mut err = -1;
    if is_prefix(*argv, b"jited\0".as_ptr() as *const c_char) {
        if disasm_init() != 0 { return -1; }
        mode = dump_mode::DUMP_JITED;
    } else if is_prefix(*argv, b"xlated\0".as_ptr() as *const c_char) {
        mode = dump_mode::DUMP_XLATED;
    } else {
        p_err(b"expected 'xlated' or 'jited', got: %s\0".as_ptr() as *const c_char, *argv);
        return -1;
    }
    NEXT_ARG();
    if argc < 2 { usage(); }
    if fds.is_null() { p_err(b"mem alloc failed\0".as_ptr() as *const c_char); return -1; }
    let nb_fds = prog_parse_fds(&mut argc, &mut argv, &mut fds);
    if nb_fds < 1 { free(fds as *mut c_void); return err; }
    while argc != 0 {
        if is_prefix(*argv, b"file\0".as_ptr() as *const c_char) {
            NEXT_ARG(); if argc == 0 { p_err(b"expected file path\0".as_ptr() as *const c_char); break; }
            if nb_fds > 1 { p_err(b"several programs matched\0".as_ptr() as *const c_char); break; }
            filepath = *argv; NEXT_ARG();
        } else if is_prefix(*argv, b"opcodes\0".as_ptr() as *const c_char) { opcodes = true; NEXT_ARG(); }
        else if is_prefix(*argv, b"visual\0".as_ptr() as *const c_char) {
            if nb_fds > 1 { p_err(b"several programs matched\0".as_ptr() as *const c_char); break; }
            visual = true; NEXT_ARG();
        } else if is_prefix(*argv, b"linum\0".as_ptr() as *const c_char) { linum = true; NEXT_ARG(); }
        else { usage(); }
    }
    if !filepath.is_null() && (opcodes || visual || linum) {
        p_err(b"'file' is not compatible with 'opcodes', 'visual', or 'linum'\0".as_ptr() as *const c_char);
    } else if json_output && visual {
        p_err(b"'visual' is not compatible with JSON output\0".as_ptr() as *const c_char);
    } else {
        if json_output && nb_fds > 1 { jsonw_start_array(json_wtr); }
        let mut i = 0;
        while i < nb_fds {
            memset(&mut info as *mut _ as *mut c_void, 0, size_of::<bpf_prog_info>());
            err = bpf_prog_get_info_by_fd(*fds.add(i as usize), &mut info, &mut info_len);
            if err != 0 { p_err(b"can't get prog info: %s\0".as_ptr() as *const c_char, strerror(errno)); break; }
            err = prep_prog_info(&mut info, mode, &mut info_data, &mut info_data_sz);
            if err != 0 { p_err(b"can't grow prog info_data\0".as_ptr() as *const c_char); break; }
            err = bpf_prog_get_info_by_fd(*fds.add(i as usize), &mut info, &mut info_len);
            if err != 0 { p_err(b"can't get prog info: %s\0".as_ptr() as *const c_char, strerror(errno)); break; }
            if json_output && nb_fds > 1 {
                jsonw_start_object(json_wtr); print_prog_header_json(&mut info, *fds.add(i as usize)); jsonw_name(json_wtr, b"insns\0".as_ptr() as *const c_char);
            } else if nb_fds > 1 { print_prog_header_plain(&mut info, *fds.add(i as usize)); }
            err = prog_dump(&mut info, mode, filepath, opcodes, visual, linum);
            if json_output && nb_fds > 1 { jsonw_end_object(json_wtr); }
            else if i != nb_fds - 1 && nb_fds > 1 { printf(b"\n\0".as_ptr() as *const c_char); }
            if err != 0 { break; }
            close(*fds.add(i as usize));
            i += 1;
        }
        if json_output && nb_fds > 1 { jsonw_end_array(json_wtr); }
        while i < nb_fds { close(*fds.add(i as usize)); i += 1; }
    }
    free(info_data);
    free(fds as *mut c_void);
    err
}

unsafe extern "C" fn do_pin(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let err = do_pin_any(argc, argv, prog_parse_fd);
    if err == 0 && json_output { jsonw_null(json_wtr); }
    err
}

unsafe extern "C" fn map_replace_compar(p1: *const c_void, p2: *const c_void) -> c_int {
    let a = p1 as *const map_replace;
    let b = p2 as *const map_replace;
    (*a).idx - (*b).idx
}

unsafe extern "C" fn parse_attach_detach_args(mut argc: c_int, mut argv: *mut *mut c_char, progfd: *mut c_int, attach_type: *mut __u32, mapfd: *mut c_int) -> c_int {
    if !REQ_ARGS(3) { return -EINVAL; }
    *progfd = prog_parse_fd(&mut argc, &mut argv);
    if *progfd < 0 { return *progfd; }
    *attach_type = parse_attach_type(*argv);
    if *attach_type == __MAX_BPF_ATTACH_TYPE as __u32 {
        p_err(b"invalid attach/detach type\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if *attach_type == BPF_FLOW_DISSECTOR as __u32 {
        *mapfd = 0;
        return 0;
    }
    NEXT_ARG();
    if !REQ_ARGS(2) { return -EINVAL; }
    *mapfd = map_parse_fd(&mut argc, &mut argv, 0);
    if *mapfd < 0 { return *mapfd; }
    0
}

unsafe extern "C" fn do_attach(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut attach_type = 0;
    let mut progfd = 0;
    let mut mapfd = 0;
    let mut err = parse_attach_detach_args(argc, argv, &mut progfd, &mut attach_type, &mut mapfd);
    if err != 0 { return err; }
    err = bpf_prog_attach(progfd, mapfd, attach_type, 0);
    if err != 0 { p_err(b"failed prog attach to map\0".as_ptr() as *const c_char); return -EINVAL; }
    if json_output { jsonw_null(json_wtr); }
    0
}

unsafe extern "C" fn do_detach(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut attach_type = 0;
    let mut progfd = 0;
    let mut mapfd = 0;
    let mut err = parse_attach_detach_args(argc, argv, &mut progfd, &mut attach_type, &mut mapfd);
    if err != 0 { return err; }
    err = bpf_prog_detach2(progfd, mapfd, attach_type);
    if err != 0 { p_err(b"failed prog detach from map\0".as_ptr() as *const c_char); return -EINVAL; }
    if json_output { jsonw_null(json_wtr); }
    0
}

unsafe extern "C" fn prog_tracelog_stream(prog_fd: c_int, mode: prog_tracelog_mode) -> c_int {
    let file = if mode == prog_tracelog_mode::TRACE_STDOUT { stdout } else { stderr };
    let stream_id = if mode == prog_tracelog_mode::TRACE_STDOUT { 1 } else { 2 };
    let mut buf = [0 as c_char; 512];
    let mut ret;
    loop {
        ret = bpf_prog_stream_read(prog_fd, stream_id, buf.as_mut_ptr() as *mut c_void, buf.len(), null_mut());
        if ret > 0 { fwrite(buf.as_ptr() as *const c_void, size_of::<c_char>(), ret as usize, file); }
        if ret <= 0 { break; }
    }
    fflush(file);
    if ret != 0 { -1 } else { 0 }
}

unsafe extern "C" fn do_tracelog_any(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    if argc == 0 { return do_tracelog(argc, argv); }
    if !is_prefix(*argv, b"stdout\0".as_ptr() as *const c_char) && !is_prefix(*argv, b"stderr\0".as_ptr() as *const c_char) { usage(); }
    let mode = if is_prefix(*argv, b"stdout\0".as_ptr() as *const c_char) { prog_tracelog_mode::TRACE_STDOUT } else { prog_tracelog_mode::TRACE_STDERR };
    NEXT_ARG();
    if !REQ_ARGS(2) { return -1; }
    let fd = prog_parse_fd(&mut argc, &mut argv);
    if fd < 0 { return -1; }
    prog_tracelog_stream(fd, mode)
}

unsafe extern "C" fn check_single_stdin(file_data_in: *mut c_char, file_ctx_in: *mut c_char) -> c_int {
    if !file_data_in.is_null() && !file_ctx_in.is_null()
        && strcmp(file_data_in, b"-\0".as_ptr() as *const c_char) == 0
        && strcmp(file_ctx_in, b"-\0".as_ptr() as *const c_char) == 0
    {
        p_err(b"cannot use standard input for both data_in and ctx_in\0".as_ptr() as *const c_char);
        return -1;
    }
    0
}

unsafe extern "C" fn get_run_data(fname: *const c_char, data_ptr: *mut *mut c_void, size: *mut c_uint) -> c_int {
    let block_size: size_t = 256;
    let mut buf_size = block_size;
    let mut nb_read: size_t = 0;
    if fname.is_null() { *data_ptr = null_mut(); *size = 0; return 0; }
    let f = if strcmp(fname, b"-\0".as_ptr() as *const c_char) == 0 { stdin } else { fopen(fname, b"r\0".as_ptr() as *const c_char) };
    if f.is_null() { p_err(b"failed to open %s: %s\0".as_ptr() as *const c_char, fname, strerror(errno)); return -1; }
    *data_ptr = malloc(block_size);
    if (*data_ptr).is_null() { p_err(b"failed to allocate memory for data_in/ctx_in: %s\0".as_ptr() as *const c_char, strerror(errno)); if f != stdin { fclose(f); } return -1; }
    loop {
        nb_read += fread((*data_ptr as *mut u8).add(nb_read) as *mut c_void, 1, block_size, f);
        if nb_read == 0 || feof(f) != 0 { break; }
        if ferror(f) != 0 {
            p_err(b"failed to read data_in/ctx_in from %s: %s\0".as_ptr() as *const c_char, fname, strerror(errno));
            free(*data_ptr); *data_ptr = null_mut(); if f != stdin { fclose(f); } return -1;
        }
        if nb_read > buf_size - block_size {
            if buf_size == UINT32_MAX as usize { p_err(b"data_in/ctx_in is too long (max: %u)\0".as_ptr() as *const c_char, UINT32_MAX); free(*data_ptr); *data_ptr = null_mut(); if f != stdin { fclose(f); } return -1; }
            /* No space for fread()-ing next chunk; realloc() */
            buf_size *= 2;
            let tmp = realloc(*data_ptr, buf_size);
            if tmp.is_null() { p_err(b"failed to reallocate data_in/ctx_in: %s\0".as_ptr() as *const c_char, strerror(errno)); free(*data_ptr); *data_ptr = null_mut(); if f != stdin { fclose(f); } return -1; }
            *data_ptr = tmp;
        }
    }
    if f != stdin { fclose(f); }
    *size = nb_read as c_uint;
    0
}

unsafe extern "C" fn hex_print(data: *mut c_void, size: c_uint, f: *mut FILE) {
    let mut i = 0usize;
    while i < size as usize {
        fprintf(f, b"%07zx\t\0".as_ptr() as *const c_char, i);
        let mut j = i;
        while j < i + 16 && j < size as usize {
            fprintf(f, b"%02x%s\0".as_ptr() as *const c_char, *((data as *mut u8).add(j)) as c_int,
                    if j % 2 != 0 { b" \0".as_ptr() } else { b"\0".as_ptr() });
            j += 1;
        }
        while j < i + 16 {
            fprintf(f, b"  %s\0".as_ptr() as *const c_char, if j % 2 != 0 { b" \0".as_ptr() } else { b"\0".as_ptr() });
            j += 1;
        }
        fprintf(f, b"| \0".as_ptr() as *const c_char);
        j = i;
        while j < i + 16 && j < size as usize {
            let mut c = *((data as *mut c_char).add(j));
            if c < b' ' as c_char || c > b'~' as c_char { c = b'.' as c_char; }
            fprintf(f, b"%c%s\0".as_ptr() as *const c_char, c as c_int, if j == i + 7 { b" \0".as_ptr() } else { b"\0".as_ptr() });
            j += 1;
        }
        fprintf(f, b"\n\0".as_ptr() as *const c_char);
        i += 16;
    }
}

unsafe extern "C" fn print_run_output(data: *mut c_void, size: c_uint, fname: *const c_char, json_key: *const c_char) -> c_int {
    if fname.is_null() { return 0; }
    if strcmp(fname, b"-\0".as_ptr() as *const c_char) == 0 {
        if json_output { jsonw_name(json_wtr, json_key); print_data_json(data, size); } else { hex_print(data, size, stdout); }
        return 0;
    }
    let f = fopen(fname, b"w\0".as_ptr() as *const c_char);
    if f.is_null() { p_err(b"failed to open %s: %s\0".as_ptr() as *const c_char, fname, strerror(errno)); return -1; }
    let nb_written = fwrite(data, 1, size as usize, f);
    fclose(f);
    if nb_written != size as usize { p_err(b"failed to write output data/ctx: %s\0".as_ptr() as *const c_char, strerror(errno)); return -1; }
    0
}

unsafe extern "C" fn alloc_run_data(data_ptr: *mut *mut c_void, size_out: c_uint) -> c_int {
    *data_ptr = calloc(size_out as usize, 1);
    if (*data_ptr).is_null() {
        p_err(b"failed to allocate memory for output data/ctx: %s\0".as_ptr() as *const c_char, strerror(errno));
        return -1;
    }
    0
}

unsafe extern "C" fn do_run(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut data_fname_in: *mut c_char = null_mut();
    let mut data_fname_out: *mut c_char = null_mut();
    let mut ctx_fname_in: *mut c_char = null_mut();
    let mut ctx_fname_out: *mut c_char = null_mut();
    let default_size: c_uint = SZ_32K;
    let mut data_in: *mut c_void = null_mut();
    let mut data_out: *mut c_void = null_mut();
    let mut ctx_in: *mut c_void = null_mut();
    let mut ctx_out: *mut c_void = null_mut();
    let mut repeat: c_uint = 1;
    let mut test_attr: bpf_test_run_opts = zeroed();
    if !REQ_ARGS(4) { return -1; }
    let fd = prog_parse_fd(&mut argc, &mut argv);
    if fd < 0 { return -1; }
    while argc != 0 {
        if detect_common_prefix(*argv, b"data_in\0".as_ptr() as *const c_char, b"data_out\0".as_ptr() as *const c_char, b"data_size_out\0".as_ptr() as *const c_char, null::<c_char>()) { return -1; }
        if detect_common_prefix(*argv, b"ctx_in\0".as_ptr() as *const c_char, b"ctx_out\0".as_ptr() as *const c_char, b"ctx_size_out\0".as_ptr() as *const c_char, null::<c_char>()) { return -1; }
        if is_prefix(*argv, b"data_in\0".as_ptr() as *const c_char) { NEXT_ARG(); if !REQ_ARGS(1) { return -1; } data_fname_in = GET_ARG(); if check_single_stdin(data_fname_in, ctx_fname_in) != 0 { return -1; } }
        else if is_prefix(*argv, b"data_out\0".as_ptr() as *const c_char) { NEXT_ARG(); if !REQ_ARGS(1) { return -1; } data_fname_out = GET_ARG(); }
        else if is_prefix(*argv, b"data_size_out\0".as_ptr() as *const c_char) { let mut endptr: *mut c_char = null_mut(); NEXT_ARG(); if !REQ_ARGS(1) { return -1; } test_attr.data_size_out = strtoul(*argv, &mut endptr, 0) as c_uint; if *endptr != 0 { p_err(b"can't parse %s as output data size\0".as_ptr() as *const c_char, *argv); return -1; } NEXT_ARG(); }
        else if is_prefix(*argv, b"ctx_in\0".as_ptr() as *const c_char) { NEXT_ARG(); if !REQ_ARGS(1) { return -1; } ctx_fname_in = GET_ARG(); if check_single_stdin(data_fname_in, ctx_fname_in) != 0 { return -1; } }
        else if is_prefix(*argv, b"ctx_out\0".as_ptr() as *const c_char) { NEXT_ARG(); if !REQ_ARGS(1) { return -1; } ctx_fname_out = GET_ARG(); }
        else if is_prefix(*argv, b"ctx_size_out\0".as_ptr() as *const c_char) { let mut endptr: *mut c_char = null_mut(); NEXT_ARG(); if !REQ_ARGS(1) { return -1; } test_attr.ctx_size_out = strtoul(*argv, &mut endptr, 0) as c_uint; if *endptr != 0 { p_err(b"can't parse %s as output context size\0".as_ptr() as *const c_char, *argv); return -1; } NEXT_ARG(); }
        else if is_prefix(*argv, b"repeat\0".as_ptr() as *const c_char) { let mut endptr: *mut c_char = null_mut(); NEXT_ARG(); if !REQ_ARGS(1) { return -1; } repeat = strtoul(*argv, &mut endptr, 0) as c_uint; if *endptr != 0 { p_err(b"can't parse %s as repeat number\0".as_ptr() as *const c_char, *argv); return -1; } NEXT_ARG(); }
        else { p_err(b"expected no more arguments, 'data_in', 'data_out', 'data_size_out', 'ctx_in', 'ctx_out', 'ctx_size_out' or 'repeat', got: '%s'?\0".as_ptr() as *const c_char, *argv); return -1; }
    }
    let mut err = get_run_data(data_fname_in, &mut data_in, &mut test_attr.data_size_in);
    if err != 0 { return -1; }
    if !data_in.is_null() {
        if test_attr.data_size_out == 0 { test_attr.data_size_out = default_size; }
        err = alloc_run_data(&mut data_out, test_attr.data_size_out);
        if err != 0 { free(data_in); return err; }
    }
    err = get_run_data(ctx_fname_in, &mut ctx_in, &mut test_attr.ctx_size_in);
    if err != 0 { free(data_out); free(data_in); return err; }
    if !ctx_in.is_null() {
        if test_attr.ctx_size_out == 0 { test_attr.ctx_size_out = default_size; }
        err = alloc_run_data(&mut ctx_out, test_attr.ctx_size_out);
        if err != 0 { free(ctx_in); free(data_out); free(data_in); return err; }
    }
    test_attr.repeat = repeat; test_attr.data_in = data_in; test_attr.data_out = data_out; test_attr.ctx_in = ctx_in; test_attr.ctx_out = ctx_out;
    err = bpf_prog_test_run_opts(fd, &mut test_attr);
    if err != 0 { p_err(b"failed to run program: %s\0".as_ptr() as *const c_char, strerror(errno)); }
    else {
        if json_output { jsonw_start_object(json_wtr); }
        if test_attr.data_size_out != 0 { err += print_run_output(test_attr.data_out, test_attr.data_size_out, data_fname_out, b"data_out\0".as_ptr() as *const c_char); }
        if test_attr.ctx_size_out != 0 { err += print_run_output(test_attr.ctx_out, test_attr.ctx_size_out, ctx_fname_out, b"ctx_out\0".as_ptr() as *const c_char); }
        if json_output {
            jsonw_uint_field(json_wtr, b"retval\0".as_ptr() as *const c_char, test_attr.retval as __u64);
            jsonw_uint_field(json_wtr, b"duration\0".as_ptr() as *const c_char, test_attr.duration as __u64);
            jsonw_end_object(json_wtr);
        } else {
            fprintf(stdout, b"Return value: %u, duration%s: %uns\n\0".as_ptr() as *const c_char, test_attr.retval, if repeat > 1 { b" (average)\0".as_ptr() } else { b"\0".as_ptr() }, test_attr.duration);
        }
    }
    free(ctx_out); free(ctx_in); free(data_out); free(data_in);
    err
}

unsafe extern "C" fn get_prog_type_by_name(name: *const c_char, prog_type: *mut __u32, expected_attach_type: *mut __u32) -> c_int {
    let mut ret = libbpf_prog_type_by_name(name, prog_type, expected_attach_type);
    if ret == 0 { return ret; }
    /* libbpf_prog_type_by_name() failed, let's re-run with debug level */
    let print_backup = libbpf_set_print(print_all_levels as *mut c_void);
    ret = libbpf_prog_type_by_name(name, prog_type, expected_attach_type);
    libbpf_set_print(print_backup);
    ret
}

unsafe extern "C" fn auto_attach_program(prog: *mut bpf_program, path: *const c_char) -> c_int {
    let link = bpf_program__attach(prog);
    if link.is_null() {
        p_info(b"Program %s does not support autoattach, falling back to pinning\0".as_ptr() as *const c_char, bpf_program__name(prog));
        return bpf_obj_pin(bpf_program__fd(prog), path);
    }
    let err = bpf_link__pin(link, path);
    bpf_link__destroy(link);
    err
}

unsafe extern "C" fn auto_attach_programs(obj: *mut bpf_object, path: *const c_char) -> c_int {
    let mut prog: *mut bpf_program = null_mut();
    let mut buf = [0 as c_char; PATH_MAX];
    loop {
        prog = bpf_object__next_program(obj, prog);
        if prog.is_null() { break; }
        let mut err = pathname_concat(buf.as_mut_ptr(), buf.len(), path, bpf_program__name(prog));
        if err == 0 { err = auto_attach_program(prog, buf.as_ptr()); }
        if err != 0 {
            while { prog = bpf_object__prev_program(obj, prog); !prog.is_null() } {
                if pathname_concat(buf.as_mut_ptr(), buf.len(), path, bpf_program__name(prog)) == 0 {
                    bpf_program__unpin(prog, buf.as_ptr());
                }
            }
            return err;
        }
    }
    0
}

unsafe extern "C" fn load_with_options(mut argc: c_int, mut argv: *mut *mut c_char, first_prog_only: bool) -> c_int {
    let mut common_prog_type: __u32 = BPF_PROG_TYPE_UNSPEC;
    let mut open_opts: bpf_object_open_opts = zeroed();
    open_opts.relaxed_maps = relaxed_maps;
    let mut expected_attach_type: __u32 = 0;
    let mut map_replace_arr: *mut map_replace = null_mut();
    let mut old_map_fds: c_uint = 0;
    let mut pinmaps: *const c_char = null();
    let mut xdpmeta_ifindex: __u32 = 0;
    let mut offload_ifindex: __u32 = 0;
    let mut auto_attach = false;
    let mut i: c_uint;
    let mut j: c_uint;
    let mut idx: c_int;
    if !REQ_ARGS(2) { return -1; }
    let file = GET_ARG();
    let pinfile = GET_ARG();
    while argc != 0 {
        if is_prefix(*argv, b"type\0".as_ptr() as *const c_char) {
            NEXT_ARG();
            if common_prog_type != BPF_PROG_TYPE_UNSPEC { p_err(b"program type already specified\0".as_ptr() as *const c_char); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            if !REQ_ARGS(1) { goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            let mut err = libbpf_prog_type_by_name(*argv, &mut common_prog_type, &mut expected_attach_type);
            if err < 0 {
                let type_buf = malloc(strlen(*argv) + 2) as *mut c_char;
                if type_buf.is_null() { p_err(b"mem alloc failed\0".as_ptr() as *const c_char); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
                *type_buf = 0; strcat(type_buf, *argv); strcat(type_buf, b"/\0".as_ptr() as *const c_char);
                err = get_prog_type_by_name(type_buf, &mut common_prog_type, &mut expected_attach_type);
                free(type_buf as *mut c_void);
                if err < 0 { goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            }
            NEXT_ARG();
        } else if is_prefix(*argv, b"map\0".as_ptr() as *const c_char) {
            let mut endptr: *mut c_char = null_mut();
            let mut name: *mut c_char = null_mut();
            NEXT_ARG();
            if !REQ_ARGS(4) { goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            if is_prefix(*argv, b"idx\0".as_ptr() as *const c_char) {
                NEXT_ARG(); idx = strtoul(*argv, &mut endptr, 0) as c_int;
                if *endptr != 0 { p_err(b"can't parse %s as IDX\0".as_ptr() as *const c_char, *argv); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            } else if is_prefix(*argv, b"name\0".as_ptr() as *const c_char) {
                NEXT_ARG(); name = *argv; idx = -1;
            } else {
                p_err(b"expected 'idx' or 'name', got: '%s'?\0".as_ptr() as *const c_char, *argv); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1;
            }
            NEXT_ARG();
            let fd = map_parse_fd(&mut argc, &mut argv, 0);
            if fd < 0 { goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            let new_map_replace = libbpf_reallocarray(map_replace_arr as *mut c_void, (old_map_fds + 1) as usize, size_of::<map_replace>()) as *mut map_replace;
            if new_map_replace.is_null() { p_err(b"mem alloc failed\0".as_ptr() as *const c_char); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            map_replace_arr = new_map_replace;
            (*map_replace_arr.add(old_map_fds as usize)).idx = idx;
            (*map_replace_arr.add(old_map_fds as usize)).name = name;
            (*map_replace_arr.add(old_map_fds as usize)).fd = fd;
            old_map_fds += 1;
        } else if is_prefix(*argv, b"dev\0".as_ptr() as *const c_char) || is_prefix(*argv, b"offload_dev\0".as_ptr() as *const c_char) {
            if is_prefix(*argv, b"dev\0".as_ptr() as *const c_char) {
                p_info(b"Warning: 'bpftool prog load [...] dev <ifname>' syntax is deprecated.\nGoing further, please use 'offload_dev <ifname>' to offload program to device.\nFor applications using XDP hints only, use 'xdpmeta_dev <ifname>'.\0".as_ptr() as *const c_char);
            }
            NEXT_ARG();
            if offload_ifindex != 0 { p_err(b"offload_dev already specified\0".as_ptr() as *const c_char); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            else if xdpmeta_ifindex != 0 { p_err(b"xdpmeta_dev and offload_dev are mutually exclusive\0".as_ptr() as *const c_char); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            if !REQ_ARGS(1) { goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            offload_ifindex = if_nametoindex(*argv);
            if offload_ifindex == 0 { p_err(b"unrecognized netdevice '%s': %s\0".as_ptr() as *const c_char, *argv, strerror(errno)); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            NEXT_ARG();
        } else if is_prefix(*argv, b"xdpmeta_dev\0".as_ptr() as *const c_char) {
            NEXT_ARG();
            if xdpmeta_ifindex != 0 { p_err(b"xdpmeta_dev already specified\0".as_ptr() as *const c_char); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            else if offload_ifindex != 0 { p_err(b"xdpmeta_dev and offload_dev are mutually exclusive\0".as_ptr() as *const c_char); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            if !REQ_ARGS(1) { goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            xdpmeta_ifindex = if_nametoindex(*argv);
            if xdpmeta_ifindex == 0 { p_err(b"unrecognized netdevice '%s': %s\0".as_ptr() as *const c_char, *argv, strerror(errno)); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            NEXT_ARG();
        } else if is_prefix(*argv, b"pinmaps\0".as_ptr() as *const c_char) {
            NEXT_ARG(); if !REQ_ARGS(1) { goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; } pinmaps = GET_ARG();
        } else if is_prefix(*argv, b"autoattach\0".as_ptr() as *const c_char) {
            auto_attach = true; NEXT_ARG();
        } else if is_prefix(*argv, b"kernel_btf\0".as_ptr() as *const c_char) {
            NEXT_ARG(); if !REQ_ARGS(1) { goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; } open_opts.btf_custom_path = GET_ARG();
        } else {
            p_err(b"expected no more arguments, 'type', 'map', 'offload_dev', 'xdpmeta_dev', 'pinmaps', 'autoattach', or 'kernel_btf', got: '%s'?\0".as_ptr() as *const c_char, *argv);
            goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1;
        }
    }
    set_max_rlimit();
    if verifier_logs { open_opts.kernel_log_level = 1 + 2 + 4; }
    let obj = bpf_object__open_file(file, &mut open_opts);
    if obj.is_null() { p_err(b"failed to open object file\0".as_ptr() as *const c_char); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
    let mut pos: *mut bpf_program = null_mut();
    loop {
        pos = bpf_object__next_program(obj, pos);
        if pos.is_null() { break; }
        let mut prog_type = common_prog_type;
        if prog_type == BPF_PROG_TYPE_UNSPEC {
            let sec_name = bpf_program__section_name(pos);
            if get_prog_type_by_name(sec_name, &mut prog_type, &mut expected_attach_type) < 0 { bpf_object__close(obj); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
        }
        if prog_type == BPF_PROG_TYPE_XDP && xdpmeta_ifindex != 0 {
            bpf_program__set_flags(pos, BPF_F_XDP_DEV_BOUND_ONLY);
            bpf_program__set_ifindex(pos, xdpmeta_ifindex);
        } else {
            bpf_program__set_ifindex(pos, offload_ifindex);
        }
        if bpf_program__type(pos) != prog_type { bpf_program__set_type(pos, prog_type); }
        bpf_program__set_expected_attach_type(pos, expected_attach_type);
    }
    qsort(map_replace_arr as *mut c_void, old_map_fds as usize, size_of::<map_replace>(), Some(map_replace_compar));
    j = 0;
    while j < old_map_fds && !(*map_replace_arr.add(j as usize)).name.is_null() {
        i = 0;
        let mut map: *mut bpf_map = null_mut();
        loop {
            map = bpf_object__next_map(obj, map);
            if map.is_null() { break; }
            if strcmp(bpf_map__name(map), (*map_replace_arr.add(j as usize)).name) == 0 {
                (*map_replace_arr.add(j as usize)).idx = i as c_int;
                break;
            }
            i += 1;
        }
        if (*map_replace_arr.add(j as usize)).idx == -1 {
            p_err(b"unable to find map '%s'\0".as_ptr() as *const c_char, (*map_replace_arr.add(j as usize)).name);
            bpf_object__close(obj); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1;
        }
        j += 1;
    }
    if j != 0 { qsort(map_replace_arr as *mut c_void, old_map_fds as usize, size_of::<map_replace>(), Some(map_replace_compar)); }
    j = 0; idx = 0;
    let mut map: *mut bpf_map = null_mut();
    loop {
        map = bpf_object__next_map(obj, map);
        if map.is_null() { break; }
        if bpf_map__type(map) != BPF_MAP_TYPE_PERF_EVENT_ARRAY { bpf_map__set_ifindex(map, offload_ifindex); }
        if j < old_map_fds && idx == (*map_replace_arr.add(j as usize)).idx {
            let err = bpf_map__reuse_fd(map, (*map_replace_arr.add(j as usize)).fd);
            j += 1;
            if err != 0 { p_err(b"unable to set up map reuse: %d\0".as_ptr() as *const c_char, err); bpf_object__close(obj); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
            if j < old_map_fds && (*map_replace_arr.add(j as usize)).idx == idx {
                p_err(b"replacement for map idx %d specified more than once\0".as_ptr() as *const c_char, idx);
                bpf_object__close(obj); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1;
            }
        }
        idx += 1;
    }
    if j < old_map_fds { p_err(b"map idx '%d' not used\0".as_ptr() as *const c_char, (*map_replace_arr.add(j as usize)).idx); bpf_object__close(obj); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
    let mut err = bpf_object__load(obj);
    if err != 0 { p_err(b"failed to load object file\0".as_ptr() as *const c_char); bpf_object__close(obj); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
    err = if first_prog_only { mount_bpffs_for_file(pinfile) } else { create_and_mount_bpffs_dir(pinfile) };
    if err != 0 { bpf_object__close(obj); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
    if first_prog_only {
        let prog = bpf_object__next_program(obj, null_mut());
        if prog.is_null() { p_err(b"object file doesn't contain any bpf program\0".as_ptr() as *const c_char); bpf_object__close(obj); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
        err = if auto_attach { auto_attach_program(prog, pinfile) } else { bpf_obj_pin(bpf_program__fd(prog), pinfile) };
        if err != 0 { p_err(b"failed to pin program %s\0".as_ptr() as *const c_char, bpf_program__section_name(prog)); bpf_object__close(obj); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
    } else {
        err = if auto_attach { auto_attach_programs(obj, pinfile) } else { bpf_object__pin_programs(obj, pinfile) };
        if err != 0 { p_err(b"failed to pin all programs\0".as_ptr() as *const c_char); bpf_object__close(obj); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1; }
    }
    if !pinmaps.is_null() {
        err = create_and_mount_bpffs_dir(pinmaps);
        if err == 0 { err = bpf_object__pin_maps(obj, pinmaps); }
        if err != 0 {
            if first_prog_only { unlink(pinfile); } else { bpf_object__unpin_programs(obj, pinfile); }
            bpf_object__close(obj); goto_err_free_reuse_maps(map_replace_arr, old_map_fds); return -1;
        }
    }
    if json_output { jsonw_null(json_wtr); }
    bpf_object__close(obj);
    goto_err_free_reuse_maps(map_replace_arr, old_map_fds);
    0
}

unsafe fn goto_err_free_reuse_maps(map_replace_arr: *mut map_replace, old_map_fds: c_uint) {
    for i in 0..old_map_fds {
        close((*map_replace_arr.add(i as usize)).fd);
    }
    free(map_replace_arr as *mut c_void);
}

unsafe extern "C" fn count_open_fds() -> c_int {
    let dp = opendir(b"/proc/self/fd\0".as_ptr() as *const c_char);
    let mut cnt = -3;
    if dp.is_null() { return -1; }
    while !readdir(dp).is_null() { cnt += 1; }
    closedir(dp);
    cnt
}

unsafe extern "C" fn try_loader(gen: *mut gen_loader_opts) -> c_int {
    let mut opts: bpf_load_and_run_opts = zeroed();
    let mut sig_buf = [0 as c_char; MAX_SIG_SIZE];
    let mut prog_sha = [0 as __u8; SHA256_DIGEST_LENGTH];
    let ctx_sz = size_of::<bpf_loader_ctx>() + 64 * size_of::<usize>();
    let log_buf_sz = (1u32 << 24) - 1;
    let ctx = alloca(ctx_sz) as *mut bpf_loader_ctx;
    memset(ctx as *mut c_void, 0, ctx_sz);
    (*ctx).sz = ctx_sz as c_int;
    let mut log_buf: *mut c_char = null_mut();
    if verifier_logs {
        (*ctx).log_level = 1 + 2 + 4;
        (*ctx).log_size = log_buf_sz as c_int;
        log_buf = malloc(log_buf_sz as usize) as *mut c_char;
        if log_buf.is_null() { return -ENOMEM; }
        (*ctx).log_buf = log_buf as c_long;
    }
    opts.ctx = ctx;
    opts.data = (*gen).data;
    opts.data_sz = (*gen).data_sz;
    opts.insns = (*gen).insns;
    opts.insns_sz = (*gen).insns_sz;
    let fds_before = count_open_fds();
    let mut err;
    if sign_progs {
        opts.excl_prog_hash = prog_sha.as_mut_ptr();
        opts.excl_prog_hash_sz = prog_sha.len() as __u32;
        opts.signature = sig_buf.as_mut_ptr();
        opts.signature_sz = MAX_SIG_SIZE as __u32;
        opts.keyring_id = KEY_SPEC_SESSION_KEYRING;
        err = bpftool_prog_sign(&mut opts);
        if err < 0 { p_err(b"failed to sign program\0".as_ptr() as *const c_char); free(log_buf as *mut c_void); return err; }
        err = register_session_key(cert_path);
        if err < 0 { p_err(b"failed to add session key\0".as_ptr() as *const c_char); free(log_buf as *mut c_void); return err; }
    }
    err = bpf_load_and_run(&mut opts);
    let fd_delta = count_open_fds() - fds_before;
    if err < 0 || verifier_logs {
        fprintf(stderr, b"err %d\n%s\n%s\0".as_ptr() as *const c_char, err, opts.errstr, log_buf);
        if fd_delta != 0 && err < 0 { fprintf(stderr, b"loader prog leaked %d FDs\n\0".as_ptr() as *const c_char, fd_delta); }
    }
    free(log_buf as *mut c_void);
    err
}

unsafe extern "C" fn do_loader(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut open_opts: bpf_object_open_opts = zeroed();
    let mut gen: gen_loader_opts = zeroed();
    if !REQ_ARGS(1) { return -1; }
    let file = GET_ARG();
    if verifier_logs { open_opts.kernel_log_level = 1 + 2 + 4; }
    let obj = bpf_object__open_file(file, &mut open_opts);
    if obj.is_null() { p_err(b"failed to open object file\0".as_ptr() as *const c_char); return -1; }
    if sign_progs { gen.gen_hash = true; }
    let mut err = bpf_object__gen_loader(obj, &mut gen);
    if err == 0 { err = bpf_object__load(obj); if err != 0 { p_err(b"failed to load object file\0".as_ptr() as *const c_char); } }
    if err == 0 && verifier_logs {
        let mut dd: dump_data = zeroed();
        kernel_syms_load(&mut dd);
        dump_xlated_plain(&mut dd, gen.insns as *mut c_void as *mut __u8, gen.insns_sz, false, false);
        kernel_syms_destroy(&mut dd);
    }
    if err == 0 { err = try_loader(&mut gen); }
    bpf_object__close(obj);
    err
}

unsafe extern "C" fn do_load(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if use_loader { return do_loader(argc, argv); }
    load_with_options(argc, argv, true)
}

unsafe extern "C" fn do_loadall(argc: c_int, argv: *mut *mut c_char) -> c_int {
    load_with_options(argc, argv, false)
}

/* Build-time condition from C preserved:
 * #ifdef BPFTOOL_WITHOUT_SKELETONS uses this fallback do_profile().
 * Otherwise the profiler skeleton-backed implementation below is used.
 */
unsafe extern "C" fn do_profile_without_skeletons(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    p_err(b"bpftool prog profile command is not supported. Please build bpftool with clang >= 10.0.0\0".as_ptr() as *const c_char);
    0
}

static mut metrics: [profile_metric; 6] = [
    profile_metric { name: b"cycles\0".as_ptr() as *const c_char, val: bpf_perf_event_value { counter: 0, enabled: 0, running: 0 }, attr: perf_event_attr { type_: PERF_TYPE_HARDWARE, config: PERF_COUNT_HW_CPU_CYCLES, exclude_user: 1 }, selected: false, ratio_metric: 0, ratio_desc: null(), ratio_mul: 0.0 },
    profile_metric { name: b"instructions\0".as_ptr() as *const c_char, val: bpf_perf_event_value { counter: 0, enabled: 0, running: 0 }, attr: perf_event_attr { type_: PERF_TYPE_HARDWARE, config: PERF_COUNT_HW_INSTRUCTIONS, exclude_user: 1 }, selected: false, ratio_metric: 1, ratio_desc: b"insns per cycle\0".as_ptr() as *const c_char, ratio_mul: 1.0 },
    profile_metric { name: b"l1d_loads\0".as_ptr() as *const c_char, val: bpf_perf_event_value { counter: 0, enabled: 0, running: 0 }, attr: perf_event_attr { type_: PERF_TYPE_HW_CACHE, config: PERF_COUNT_HW_CACHE_L1D | (PERF_COUNT_HW_CACHE_OP_READ << 8) | (PERF_COUNT_HW_CACHE_RESULT_ACCESS << 16), exclude_user: 1 }, selected: false, ratio_metric: 0, ratio_desc: null(), ratio_mul: 0.0 },
    profile_metric { name: b"llc_misses\0".as_ptr() as *const c_char, val: bpf_perf_event_value { counter: 0, enabled: 0, running: 0 }, attr: perf_event_attr { type_: PERF_TYPE_HW_CACHE, config: PERF_COUNT_HW_CACHE_LL | (PERF_COUNT_HW_CACHE_OP_READ << 8) | (PERF_COUNT_HW_CACHE_RESULT_MISS << 16), exclude_user: 1 }, selected: false, ratio_metric: 2, ratio_desc: b"LLC misses per million insns\0".as_ptr() as *const c_char, ratio_mul: 1e6 },
    profile_metric { name: b"itlb_misses\0".as_ptr() as *const c_char, val: bpf_perf_event_value { counter: 0, enabled: 0, running: 0 }, attr: perf_event_attr { type_: PERF_TYPE_HW_CACHE, config: PERF_COUNT_HW_CACHE_ITLB | (PERF_COUNT_HW_CACHE_OP_READ << 8) | (PERF_COUNT_HW_CACHE_RESULT_MISS << 16), exclude_user: 1 }, selected: false, ratio_metric: 2, ratio_desc: b"itlb misses per million insns\0".as_ptr() as *const c_char, ratio_mul: 1e6 },
    profile_metric { name: b"dtlb_misses\0".as_ptr() as *const c_char, val: bpf_perf_event_value { counter: 0, enabled: 0, running: 0 }, attr: perf_event_attr { type_: PERF_TYPE_HW_CACHE, config: PERF_COUNT_HW_CACHE_DTLB | (PERF_COUNT_HW_CACHE_OP_READ << 8) | (PERF_COUNT_HW_CACHE_RESULT_MISS << 16), exclude_user: 1 }, selected: false, ratio_metric: 2, ratio_desc: b"dtlb misses per million insns\0".as_ptr() as *const c_char, ratio_mul: 1e6 },
];

static mut profile_total_count: __u64 = 0;
static mut profile_obj: *mut profiler_bpf = null_mut();
static mut profile_tgt_fd: c_int = -1;
static mut profile_tgt_name: *mut c_char = null_mut();
static mut profile_perf_events: *mut c_int = null_mut();
static mut profile_perf_event_cnt: c_int = 0;

unsafe extern "C" fn profile_parse_metrics(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let metric_cnt = metrics.len();
    let mut selected_cnt = 0;
    while argc > 0 {
        let mut i = 0;
        while i < metric_cnt {
            if is_prefix(*argv, metrics[i].name) {
                if !metrics[i].selected { selected_cnt += 1; }
                metrics[i].selected = true;
                break;
            }
            i += 1;
        }
        if i == metric_cnt {
            p_err(b"unknown metric %s\0".as_ptr() as *const c_char, *argv);
            return -1;
        }
        NEXT_ARG();
    }
    if selected_cnt > MAX_NUM_PROFILE_METRICS as c_int {
        p_err(b"too many (%d) metrics, please specify no more than %d metrics at a time\0".as_ptr() as *const c_char, selected_cnt, MAX_NUM_PROFILE_METRICS as c_int);
        return -1;
    }
    selected_cnt
}

unsafe extern "C" fn profile_read_values(obj: *mut profiler_bpf) {
    let num_cpu = (*(*obj).rodata).num_cpu;
    let reading_map_fd = bpf_map__fd((*obj).maps.accum_readings);
    let count_map_fd = bpf_map__fd((*obj).maps.counts);
    if reading_map_fd < 0 || count_map_fd < 0 { p_err(b"failed to get fd for map\0".as_ptr() as *const c_char); return; }
    let mut counts = vec![0u64; num_cpu as usize];
    let mut key: __u32 = 0;
    if bpf_map_lookup_elem(count_map_fd, &mut key as *mut _ as *const c_void, counts.as_mut_ptr() as *mut c_void) != 0 {
        p_err(b"failed to read count_map: %s\0".as_ptr() as *const c_char, strerror(errno));
        return;
    }
    profile_total_count = 0;
    for cpu in 0..num_cpu as usize { profile_total_count += counts[cpu]; }
    for m in 0..metrics.len() {
        if !metrics[m].selected { continue; }
        let mut values = vec![bpf_perf_event_value { counter: 0, enabled: 0, running: 0 }; num_cpu as usize];
        if bpf_map_lookup_elem(reading_map_fd, &mut key as *mut _ as *const c_void, values.as_mut_ptr() as *mut c_void) != 0 {
            p_err(b"failed to read reading_map: %s\0".as_ptr() as *const c_char, strerror(errno));
            return;
        }
        for cpu in 0..num_cpu as usize {
            metrics[m].val.counter += values[cpu].counter;
            metrics[m].val.enabled += values[cpu].enabled;
            metrics[m].val.running += values[cpu].running;
        }
        key += 1;
    }
}

unsafe extern "C" fn profile_print_readings_json() {
    jsonw_start_array(json_wtr);
    for m in 0..metrics.len() {
        if !metrics[m].selected { continue; }
        jsonw_start_object(json_wtr);
        jsonw_string_field(json_wtr, b"metric\0".as_ptr() as *const c_char, metrics[m].name);
        jsonw_lluint_field(json_wtr, b"run_cnt\0".as_ptr() as *const c_char, profile_total_count);
        jsonw_lluint_field(json_wtr, b"value\0".as_ptr() as *const c_char, metrics[m].val.counter);
        jsonw_lluint_field(json_wtr, b"enabled\0".as_ptr() as *const c_char, metrics[m].val.enabled);
        jsonw_lluint_field(json_wtr, b"running\0".as_ptr() as *const c_char, metrics[m].val.running);
        jsonw_end_object(json_wtr);
    }
    jsonw_end_array(json_wtr);
}

unsafe extern "C" fn profile_print_readings_plain() {
    printf(b"\n%18llu %-20s\n\0".as_ptr() as *const c_char, profile_total_count, b"run_cnt\0".as_ptr());
    for m in 0..metrics.len() {
        if !metrics[m].selected { continue; }
        let val = &metrics[m].val;
        printf(b"%18llu %-20s\0".as_ptr() as *const c_char, val.counter, metrics[m].name);
        let r = metrics[m].ratio_metric - 1;
        if r >= 0 && metrics[r as usize].selected && metrics[r as usize].val.counter > 0 {
            printf(b"# %8.2f %-30s\0".as_ptr() as *const c_char,
                   val.counter as f64 * metrics[m].ratio_mul as f64 / metrics[r as usize].val.counter as f64,
                   metrics[m].ratio_desc);
        } else {
            printf(b"%-41s\0".as_ptr() as *const c_char, b"\0".as_ptr());
        }
        if val.enabled > val.running {
            printf(b"(%4.2f%%)\0".as_ptr() as *const c_char, val.running as f64 * 100.0 / val.enabled as f64);
        }
        printf(b"\n\0".as_ptr() as *const c_char);
    }
}

unsafe extern "C" fn profile_print_readings() {
    if json_output { profile_print_readings_json(); } else { profile_print_readings_plain(); }
}

unsafe extern "C" fn profile_target_name(tgt_fd: c_int) -> *mut c_char {
    let mut func_info: bpf_func_info = zeroed();
    let mut info: bpf_prog_info = zeroed();
    let mut info_len = size_of::<bpf_prog_info>() as __u32;
    let mut btfp: *mut btf = null_mut();
    let mut name: *mut c_char = null_mut();
    if bpf_prog_get_info_by_fd(tgt_fd, &mut info, &mut info_len) != 0 { p_err(b"failed to get info for prog FD %d\0".as_ptr() as *const c_char, tgt_fd); return null_mut(); }
    if info.btf_id == 0 { p_err(b"prog FD %d doesn't have valid btf\0".as_ptr() as *const c_char, tgt_fd); return null_mut(); }
    let func_info_rec_size = info.func_info_rec_size;
    if info.nr_func_info == 0 { p_err(b"found 0 func_info for prog FD %d\0".as_ptr() as *const c_char, tgt_fd); return null_mut(); }
    memset(&mut info as *mut _ as *mut c_void, 0, size_of::<bpf_prog_info>());
    info.nr_func_info = 1;
    info.func_info_rec_size = func_info_rec_size;
    info.func_info = ptr_to_u64(&mut func_info as *mut _ as *mut c_void);
    if bpf_prog_get_info_by_fd(tgt_fd, &mut info, &mut info_len) != 0 { p_err(b"failed to get func_info for prog FD %d\0".as_ptr() as *const c_char, tgt_fd); return null_mut(); }
    btfp = btf__load_from_kernel_by_id(info.btf_id);
    if btfp.is_null() { p_err(b"failed to load btf for prog FD %d\0".as_ptr() as *const c_char, tgt_fd); return null_mut(); }
    let t = btf__type_by_id(btfp, func_info.type_id);
    if t.is_null() { p_err(b"btf %u doesn't have type %u\0".as_ptr() as *const c_char, info.btf_id, func_info.type_id); }
    else { name = strdup(btf__name_by_offset(btfp, (*t).name_off)); }
    btf__free(btfp);
    name
}

unsafe extern "C" fn profile_close_perf_events(_obj: *mut profiler_bpf) {
    let mut i = profile_perf_event_cnt - 1;
    while i >= 0 {
        close(*profile_perf_events.add(i as usize));
        if i == 0 { break; }
        i -= 1;
    }
    free(profile_perf_events as *mut c_void);
    profile_perf_event_cnt = 0;
}

unsafe extern "C" fn profile_open_perf_event(mid: c_int, cpu: c_int, map_fd: c_int) -> c_int {
    let pmu_fd = syscall(__NR_perf_event_open, &mut metrics[mid as usize].attr, -1, cpu, -1, 0) as c_int;
    if pmu_fd < 0 {
        if errno == ENODEV {
            p_info(b"cpu %d may be offline, skip %s profiling.\0".as_ptr() as *const c_char, cpu, metrics[mid as usize].name);
            profile_perf_event_cnt += 1;
            return 0;
        }
        return -1;
    }
    if bpf_map_update_elem(map_fd, &profile_perf_event_cnt as *const _ as *const c_void, &pmu_fd as *const _ as *const c_void, BPF_ANY) != 0
        || ioctl(pmu_fd, PERF_EVENT_IOC_ENABLE, 0) != 0
    {
        close(pmu_fd);
        return -1;
    }
    *profile_perf_events.add(profile_perf_event_cnt as usize) = pmu_fd;
    profile_perf_event_cnt += 1;
    0
}

unsafe extern "C" fn profile_open_perf_events(obj: *mut profiler_bpf) -> c_int {
    profile_perf_events = calloc(((*(*obj).rodata).num_cpu * (*(*obj).rodata).num_metric) as usize, size_of::<c_int>()) as *mut c_int;
    if profile_perf_events.is_null() { p_err(b"failed to allocate memory for perf_event array: %s\0".as_ptr() as *const c_char, strerror(errno)); return -1; }
    let map_fd = bpf_map__fd((*obj).maps.events);
    if map_fd < 0 { p_err(b"failed to get fd for events map\0".as_ptr() as *const c_char); return -1; }
    for m in 0..metrics.len() {
        if !metrics[m].selected { continue; }
        for cpu in 0..(*(*obj).rodata).num_cpu {
            if profile_open_perf_event(m as c_int, cpu as c_int, map_fd) != 0 {
                p_err(b"failed to create event %s on cpu %u\0".as_ptr() as *const c_char, metrics[m].name, cpu);
                return -1;
            }
        }
    }
    0
}

unsafe extern "C" fn profile_print_and_cleanup() {
    profile_close_perf_events(profile_obj);
    profile_read_values(profile_obj);
    profile_print_readings();
    profiler_bpf__destroy(profile_obj);
    close(profile_tgt_fd);
    free(profile_tgt_name as *mut c_void);
}

unsafe extern "C" fn int_exit(_signo: c_int) {
    profile_print_and_cleanup();
    extern "C" { fn exit(status: c_int) -> !; }
    exit(0);
}

unsafe extern "C" fn do_profile(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut err = -1;
    let mut duration: c_ulong;
    let mut endptr: *mut c_char = null_mut();
    if !REQ_ARGS(3) { return -EINVAL; }
    profile_tgt_fd = prog_parse_fd(&mut argc, &mut argv);
    if profile_tgt_fd < 0 { p_err(b"failed to parse fd\0".as_ptr() as *const c_char); return -1; }
    if argc > 2 && is_prefix(*argv, b"duration\0".as_ptr() as *const c_char) {
        NEXT_ARG();
        duration = strtoul(*argv, &mut endptr, 0);
        if *endptr != 0 { usage(); }
        NEXT_ARG();
    } else {
        duration = UINT_MAX;
    }
    let num_metric = profile_parse_metrics(argc, argv);
    if num_metric <= 0 { goto_profile_out(err); return err; }
    let num_cpu = libbpf_num_possible_cpus();
    if num_cpu <= 0 { p_err(b"failed to identify number of CPUs\0".as_ptr() as *const c_char); goto_profile_out(err); return err; }
    profile_obj = profiler_bpf__open();
    if profile_obj.is_null() { p_err(b"failed to open and/or load BPF object\0".as_ptr() as *const c_char); goto_profile_out(err); return err; }
    (*(*profile_obj).rodata).num_cpu = num_cpu as __u32;
    (*(*profile_obj).rodata).num_metric = num_metric as __u32;
    bpf_map__set_max_entries((*profile_obj).maps.events, (num_metric * num_cpu) as __u32);
    bpf_map__set_max_entries((*profile_obj).maps.fentry_readings, num_metric as __u32);
    bpf_map__set_max_entries((*profile_obj).maps.accum_readings, num_metric as __u32);
    bpf_map__set_max_entries((*profile_obj).maps.counts, 1);
    profile_tgt_name = profile_target_name(profile_tgt_fd);
    if profile_tgt_name.is_null() { goto_profile_out(err); return err; }
    let mut prog: *mut bpf_program = null_mut();
    loop {
        prog = bpf_object__next_program((*profile_obj).obj, prog);
        if prog.is_null() { break; }
        err = bpf_program__set_attach_target(prog, profile_tgt_fd, profile_tgt_name);
        if err != 0 { p_err(b"failed to set attach target\n\0".as_ptr() as *const c_char); goto_profile_out(err); return err; }
    }
    set_max_rlimit();
    err = profiler_bpf__load(profile_obj);
    if err != 0 { p_err(b"failed to load profile_obj\0".as_ptr() as *const c_char); goto_profile_out(err); return err; }
    err = profile_open_perf_events(profile_obj);
    if err != 0 { goto_profile_out(err); return err; }
    err = profiler_bpf__attach(profile_obj);
    if err != 0 { p_err(b"failed to attach profile_obj\0".as_ptr() as *const c_char); goto_profile_out(err); return err; }
    signal(SIGINT, int_exit);
    sleep(duration as c_uint);
    profile_print_and_cleanup();
    0
}

unsafe fn goto_profile_out(err: c_int) {
    profile_close_perf_events(profile_obj);
    if !profile_obj.is_null() { profiler_bpf__destroy(profile_obj); }
    close(profile_tgt_fd);
    free(profile_tgt_name as *mut c_void);
}

unsafe extern "C" fn do_help(_argc: c_int, argv: *mut *mut c_char) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }
    fprintf(stderr,
        b"Usage: %1$s %2$s { show | list } [PROG]\n       %1$s %2$s dump xlated PROG [{ file FILE | [opcodes] [linum] [visual] }]\n       %1$s %2$s dump jited  PROG [{ file FILE | [opcodes] [linum] }]\n       %1$s %2$s pin   PROG FILE\n       %1$s %2$s { load | loadall } OBJ  PATH \\\n                         [type TYPE] [{ offload_dev | xdpmeta_dev } NAME] \\\n                         [map { idx IDX | name NAME } MAP]\\\n                         [pinmaps MAP_DIR]\n                         [autoattach]\n                         [kernel_btf BTF_FILE]\n       %1$s %2$s attach PROG ATTACH_TYPE [MAP]\n       %1$s %2$s detach PROG ATTACH_TYPE [MAP]\n       %1$s %2$s run PROG \\\n                         data_in FILE \\\n                         [data_out FILE [data_size_out L]] \\\n                         [ctx_in FILE [ctx_out FILE [ctx_size_out M]]] \\\n                         [repeat N]\n       %1$s %2$s profile PROG [duration DURATION] METRICs\n       %1$s %2$s tracelog\n       %1$s %2$s tracelog { stdout | stderr } PROG\n       %1$s %2$s help\n\n       HELP_SPEC_MAP\n       HELP_SPEC_PROGRAM\n       TYPE := { socket | kprobe | kretprobe | classifier | action |\n                 tracepoint | raw_tracepoint | xdp | perf_event | cgroup/skb |\n                 cgroup/sock | cgroup/dev | lwt_in | lwt_out | lwt_xmit |\n                 lwt_seg6local | sockops | sk_skb | sk_msg | lirc_mode2 |\n                 sk_reuseport | flow_dissector | cgroup/sysctl |\n                 cgroup/bind4 | cgroup/bind6 | cgroup/post_bind4 |\n                 cgroup/post_bind6 | cgroup/connect4 | cgroup/connect6 |\n                 cgroup/connect_unix | cgroup/getpeername4 | cgroup/getpeername6 |\n                 cgroup/getpeername_unix | cgroup/getsockname4 | cgroup/getsockname6 |\n                 cgroup/getsockname_unix | cgroup/sendmsg4 | cgroup/sendmsg6 |\n                 cgroup/sendmsg_unix | cgroup/recvmsg4 | cgroup/recvmsg6 | cgroup/recvmsg_unix |\n                 cgroup/getsockopt | cgroup/setsockopt | cgroup/sock_release |\n                 struct_ops | fentry | fexit | fsession | freplace | sk_lookup }\n       ATTACH_TYPE := { sk_msg_verdict | sk_skb_verdict | sk_skb_stream_verdict |\n                        sk_skb_stream_parser | flow_dissector }\n       METRIC := { cycles | instructions | l1d_loads | llc_misses | itlb_misses | dtlb_misses }\n       HELP_SPEC_OPTIONS |\n                    {-f|--bpffs} | {-m|--mapcompat} | {-n|--nomount} |\n                    {-L|--use-loader} | [ {-S|--sign } {-k} <private_key.pem> {-i} <certificate.x509> ] \n\0".as_ptr() as *const c_char,
        bin_name, *argv.offset(-2));
    0
}

static cmds: [cmd; 12] = [
    cmd { cmd: b"show\0".as_ptr() as *const c_char, func: Some(do_show) },
    cmd { cmd: b"list\0".as_ptr() as *const c_char, func: Some(do_show) },
    cmd { cmd: b"help\0".as_ptr() as *const c_char, func: Some(do_help) },
    cmd { cmd: b"dump\0".as_ptr() as *const c_char, func: Some(do_dump) },
    cmd { cmd: b"pin\0".as_ptr() as *const c_char, func: Some(do_pin) },
    cmd { cmd: b"load\0".as_ptr() as *const c_char, func: Some(do_load) },
    cmd { cmd: b"loadall\0".as_ptr() as *const c_char, func: Some(do_loadall) },
    cmd { cmd: b"attach\0".as_ptr() as *const c_char, func: Some(do_attach) },
    cmd { cmd: b"detach\0".as_ptr() as *const c_char, func: Some(do_detach) },
    cmd { cmd: b"tracelog\0".as_ptr() as *const c_char, func: Some(do_tracelog_any) },
    cmd { cmd: b"run\0".as_ptr() as *const c_char, func: Some(do_run) },
    cmd { cmd: b"profile\0".as_ptr() as *const c_char, func: Some(do_profile) },
];

#[no_mangle]
pub unsafe extern "C" fn do_prog(argc: c_int, argv: *mut *mut c_char) -> c_int {
    cmd_select(cmds.as_ptr(), argc, argv, do_help)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
