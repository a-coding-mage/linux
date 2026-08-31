// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */
/*
 * Rust translation of testing/selftests/bpf/veristat.c.
 *
 * This file intentionally keeps the C program's source-level shape: C ABI
 * types, raw pointers, global mutable state, libc/libelf/libbpf entry points,
 * and errno-style integer returns.  Header-provided libbpf/libelf/BTF helpers
 * are referenced as external dependencies or narrow Rust equivalents where the
 * original file only depended on their declarations/macros.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_longlong, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type error_t = c_int;
type va_list = *mut c_void;

const PATH_MAX: usize = 4096;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const E2BIG: c_int = 7;
const ESRCH: c_int = 3;
const EOPNOTSUPP: c_int = 95;
const ERANGE: c_int = 34;
const EFAULT: c_int = 14;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_APPEND: c_int = 0o2000;
const O_CLOEXEC: c_int = 0o2000000;
const UINT_MAX: c_uint = c_uint::MAX;
const DBL_MAX: c_double = c_double::MAX;
const HEADER_CHAR: c_int = b'-' as c_int;
const COLUMN_SEP: &[u8] = b"  \0";
const VERISTAT_VERSION: &[u8] = b"<kernel>\0";
const MAX_PARSED_LOG_LINES: c_int = 300;

const LIBBPF_DEBUG: libbpf_print_level = 0;
const EV_CURRENT: c_uint = 1;
const ELF_C_READ: c_uint = 1;
const ELF_K_ELF: c_uint = 3;
const ELFCLASS64: c_int = 2;
const ET_REL: c_uint = 1;
const EM_BPF: c_uint = 247;
const _SC_PAGESIZE: c_int = 30;

const BPF_ALU: __u8 = 0x04;
const BPF_MOV: __u8 = 0xb0;
const BPF_X: __u8 = 0x08;
const BPF_JMP: __u8 = 0x05;
const BPF_EXIT: __u8 = 0x90;
const BPF_REG_0: __u8 = 0;
const BPF_F_TEST_STATE_FREQ: __u32 = 1 << 0;
const BPF_F_TEST_REG_INVARIANTS: __u32 = 1 << 1;

const BTF_KIND_FUNC: c_uint = 12;
const BTF_INT_SIGNED: c_uint = 1;

const ARGP_KEY_ARG: c_int = 0;
const ARGP_ERR_UNKNOWN: error_t = 7;
const ARGP_HELP_STD_HELP: c_uint = 0;
const ARGP_HELP_USAGE: c_uint = 0;
const OPTION_HIDDEN: c_int = 1;
const OPTION_ARG_OPTIONAL: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum stat_id {
    VERDICT,
    DURATION,
    TOTAL_INSNS,
    TOTAL_STATES,
    PEAK_STATES,
    MAX_STATES_PER_INSN,
    MARK_READ_MAX_LEN,
    SIZE,
    JITED_SIZE,
    STACK,
    MAX_STACK,
    PROG_TYPE,
    ATTACH_TYPE,
    MEMORY_PEAK,
    FILE_NAME,
    PROG_NAME,
    ALL_STATS_CNT,
}
const NUM_STATS_CNT: usize = stat_id::FILE_NAME as usize - stat_id::VERDICT as usize;
const ALL_STATS_CNT_USIZE: usize = stat_id::ALL_STATS_CNT as usize;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum stat_variant {
    VARIANT_A,
    VARIANT_B,
    VARIANT_DIFF,
    VARIANT_PCT,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct verif_stats {
    file_name: *mut c_char,
    prog_name: *mut c_char,
    stats: [c_long; NUM_STATS_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct verif_stats_join {
    file_name: *mut c_char,
    prog_name: *mut c_char,
    stats_a: *const verif_stats,
    stats_b: *const verif_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct stat_specs {
    spec_cnt: c_int,
    ids: [stat_id; ALL_STATS_CNT_USIZE],
    variants: [stat_variant; ALL_STATS_CNT_USIZE],
    asc: [bool; ALL_STATS_CNT_USIZE],
    abs: [bool; ALL_STATS_CNT_USIZE],
    lens: [c_int; ALL_STATS_CNT_USIZE * 3],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum resfmt {
    RESFMT_TABLE,
    RESFMT_TABLE_CALCLEN,
    RESFMT_CSV,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum filter_kind {
    FILTER_NAME,
    FILTER_STAT,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum operator_kind {
    OP_EQ,
    OP_NEQ,
    OP_LT,
    OP_LE,
    OP_GT,
    OP_GE,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct filter {
    kind: filter_kind,
    any_glob: *mut c_char,
    file_glob: *mut c_char,
    prog_glob: *mut c_char,
    op: operator_kind,
    stat_id: c_int,
    stat_var: stat_variant,
    value: c_long,
    abs: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum rvalue_type {
    INTEGRAL,
    ENUMERATOR,
}

#[repr(C)]
#[derive(Copy, Clone)]
union rvalue_u {
    ivalue: c_longlong,
    svalue: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct rvalue {
    type_: rvalue_type,
    u: rvalue_u,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum field_access_type {
    FIELD_NAME,
    ARRAY_INDEX,
}

#[repr(C)]
#[derive(Copy, Clone)]
union field_access_u {
    name: *mut c_char,
    index: rvalue,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct field_access {
    type_: field_access_type,
    u: field_access_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct var_preset {
    atoms: *mut field_access,
    atom_count: c_int,
    full_name: *mut c_char,
    value: rvalue,
    applied: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum dump_mode {
    DUMP_NONE = 0,
    DUMP_XLATED = 1,
    DUMP_JITED = 2,
}

#[repr(C)]
struct env_t {
    filenames: *mut *mut c_char,
    filename_cnt: c_int,
    verbose: bool,
    debug: bool,
    quiet: bool,
    force_checkpoints: bool,
    force_reg_invariants: bool,
    out_fmt: resfmt,
    show_version: bool,
    comparison_mode: bool,
    replay_mode: bool,
    top_n: c_int,
    log_level: c_int,
    log_size: c_int,
    log_fixed: bool,
    prog_stats: *mut verif_stats,
    prog_stat_cnt: c_int,
    baseline_stats: *mut verif_stats,
    baseline_stat_cnt: c_int,
    join_stats: *mut verif_stats_join,
    join_stat_cnt: c_int,
    output_spec: stat_specs,
    sort_spec: stat_specs,
    allow_filters: *mut filter,
    deny_filters: *mut filter,
    allow_filter_cnt: c_int,
    deny_filter_cnt: c_int,
    files_processed: c_int,
    files_skipped: c_int,
    progs_processed: c_int,
    progs_skipped: c_int,
    top_src_lines: c_int,
    presets: *mut var_preset,
    npresets: c_int,
    orig_cgroup: [c_char; PATH_MAX],
    stat_cgroup: [c_char; PATH_MAX],
    memory_peak_fd: c_int,
    dump_mode: __u32,
}

#[repr(C)] struct FILE(c_void);
#[repr(C)] struct Elf(c_void);
#[repr(C)] struct bpf_object(c_void);
#[repr(C)] struct bpf_program(c_void);
#[repr(C)] struct bpf_map(c_void);
#[repr(C)] struct btf(c_void);

#[repr(C)]
struct Elf64_Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_insn {
    code: __u8,
    dst_src: __u8,
    off: i16,
    imm: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_prog_load_opts {
    log_buf: *mut c_char,
    log_size: __u32,
    log_level: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_object_open_opts {
    _unused: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_prog_info {
    id: __u32,
    jited_prog_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct btf_type {
    name_off: __u32,
    info: __u32,
    size: __u32,
    type_: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct btf_member {
    name_off: __u32,
    type_: __u32,
    offset: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct btf_param {
    name_off: __u32,
    type_: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct btf_array {
    type_: __u32,
    index_type: __u32,
    nelems: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct btf_enum {
    name_off: __u32,
    val: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct btf_enum64 {
    name_off: __u32,
    val_lo32: __u32,
    val_hi32: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct btf_var_secinfo {
    type_: __u32,
    offset: __u32,
    size: __u32,
}

type libbpf_print_level = c_int;
type bpf_prog_type = c_int;
type bpf_attach_type = c_int;
type bpf_map_type = c_int;
type libbpf_print_fn_t = Option<unsafe extern "C" fn(libbpf_print_level, *const c_char, va_list) -> c_int>;

const BPF_PROG_TYPE_UNSPEC: bpf_prog_type = 0;
const BPF_PROG_TYPE_SCHED_CLS: bpf_prog_type = 3;
const BPF_PROG_TYPE_CGROUP_SOCK: bpf_prog_type = 9;
const BPF_PROG_TYPE_CGROUP_SOCK_ADDR: bpf_prog_type = 15;
const BPF_PROG_TYPE_SOCK_OPS: bpf_prog_type = 13;
const BPF_PROG_TYPE_SK_MSG: bpf_prog_type = 16;
const BPF_PROG_TYPE_CGROUP_DEVICE: bpf_prog_type = 14;
const BPF_PROG_TYPE_CGROUP_SYSCTL: bpf_prog_type = 23;
const BPF_PROG_TYPE_CGROUP_SOCKOPT: bpf_prog_type = 25;
const BPF_PROG_TYPE_SK_REUSEPORT: bpf_prog_type = 21;
const BPF_PROG_TYPE_SK_LOOKUP: bpf_prog_type = 29;
const BPF_PROG_TYPE_XDP: bpf_prog_type = 6;
const BPF_PROG_TYPE_KPROBE: bpf_prog_type = 2;
const BPF_PROG_TYPE_PERF_EVENT: bpf_prog_type = 7;
const BPF_PROG_TYPE_RAW_TRACEPOINT: bpf_prog_type = 17;
const BPF_PROG_TYPE_EXT: bpf_prog_type = 28;
const BPF_PROG_TYPE_TRACEPOINT: bpf_prog_type = 5;

const BPF_CGROUP_INET4_POST_BIND: bpf_attach_type = 11;
const BPF_CGROUP_INET4_BIND: bpf_attach_type = 8;
const BPF_CGROUP_SOCK_OPS: bpf_attach_type = 6;
const BPF_SK_MSG_VERDICT: bpf_attach_type = 7;
const BPF_CGROUP_DEVICE: bpf_attach_type = 5;
const BPF_CGROUP_SYSCTL: bpf_attach_type = 23;
const BPF_CGROUP_SETSOCKOPT: bpf_attach_type = 20;
const BPF_SK_REUSEPORT_SELECT_OR_MIGRATE: bpf_attach_type = 40;
const BPF_SK_LOOKUP: bpf_attach_type = 36;
const BPF_XDP: bpf_attach_type = 37;

const BPF_MAP_TYPE_HASH: bpf_map_type = 1;
const BPF_MAP_TYPE_PERCPU_HASH: bpf_map_type = 5;
const BPF_MAP_TYPE_LRU_HASH: bpf_map_type = 9;
const BPF_MAP_TYPE_LRU_PERCPU_HASH: bpf_map_type = 10;
const BPF_MAP_TYPE_SOCKHASH: bpf_map_type = 18;
const BPF_MAP_TYPE_DEVMAP_HASH: bpf_map_type = 25;
const BPF_MAP_TYPE_QUEUE: bpf_map_type = 22;
const BPF_MAP_TYPE_STACK: bpf_map_type = 23;
const BPF_MAP_TYPE_BLOOM_FILTER: bpf_map_type = 30;
const BPF_MAP_TYPE_STACK_TRACE: bpf_map_type = 7;
const BPF_MAP_TYPE_RINGBUF: bpf_map_type = 27;
const BPF_MAP_TYPE_USER_RINGBUF: bpf_map_type = 32;
const BPF_MAP_TYPE_SK_STORAGE: bpf_map_type = 24;
const BPF_MAP_TYPE_TASK_STORAGE: bpf_map_type = 29;
const BPF_MAP_TYPE_INODE_STORAGE: bpf_map_type = 28;
const BPF_MAP_TYPE_CGROUP_STORAGE: bpf_map_type = 19;
const BPF_MAP_TYPE_CGRP_STORAGE: bpf_map_type = 33;
const BPF_MAP_TYPE_STRUCT_OPS: bpf_map_type = 26;

#[repr(C)]
struct argp_option {
    name: *const c_char,
    key: c_int,
    arg: *const c_char,
    flags: c_int,
    doc: *const c_char,
    group: c_int,
}

#[repr(C)]
struct argp_state(c_void);

#[repr(C)]
struct argp {
    options: *const argp_option,
    parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> error_t>,
    args_doc: *const c_char,
    doc: *const c_char,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;

    fn vfprintf(stream: *mut FILE, fmt: *const c_char, ap: va_list) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn vsscanf(s: *const c_char, fmt: *const c_char, ap: va_list) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn ferror(stream: *mut FILE) -> c_int;
    fn feof(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn fscanf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: isize) -> ssize_t;
    fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: isize) -> ssize_t;
    fn mkdir(path: *const c_char, mode: c_uint) -> c_int;
    fn rmdir(path: *const c_char) -> c_int;
    fn getpid() -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn system(command: *const c_char) -> c_int;
    fn popen(command: *const c_char, mode: *const c_char) -> *mut FILE;
    fn pclose(stream: *mut FILE) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strtok_r(str: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoll(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_longlong;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
    fn isspace(c: c_int) -> c_int;
    fn isdigit(c: c_int) -> c_int;
    fn fabs(x: c_double) -> c_double;
    fn exit(status: c_int) -> !;
    fn argp_parse(argp: *const argp, argc: c_int, argv: *mut *mut c_char, flags: c_uint, arg_index: *mut c_int, input: *mut c_void) -> c_int;
    fn argp_state_help(state: *mut argp_state, stream: *mut FILE, flags: c_uint);
    fn argp_usage(state: *mut argp_state);
    fn argp_help(argp: *const argp, stream: *mut FILE, flags: c_uint, name: *const c_char);

    fn elf_version(ver: c_uint) -> c_uint;
    fn elf_begin(fd: c_int, cmd: c_uint, ref_: *mut Elf) -> *mut Elf;
    fn elf_kind(elf: *mut Elf) -> c_uint;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn gelf_getclass(elf: *mut Elf) -> c_int;
    fn elf64_getehdr(elf: *mut Elf) -> *mut Elf64_Ehdr;

    fn libbpf_set_print(fn_: libbpf_print_fn_t) -> libbpf_print_fn_t;
    fn libbpf_bpf_prog_type_str(t: bpf_prog_type) -> *const c_char;
    fn libbpf_bpf_attach_type_str(t: bpf_attach_type) -> *const c_char;
    fn bpf_prog_load(prog_type: bpf_prog_type, prog_name: *const c_char, license: *const c_char, insns: *const bpf_insn, insn_cnt: size_t, opts: *const bpf_prog_load_opts) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
    fn bpf_object__open_file(path: *const c_char, opts: *const bpf_object_open_opts) -> *mut bpf_object;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_object__prepare(obj: *mut bpf_object) -> c_int;
    fn bpf_object__btf(obj: *mut bpf_object) -> *mut btf;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_program__name(prog: *const bpf_program) -> *const c_char;
    fn bpf_program__type(prog: *const bpf_program) -> bpf_prog_type;
    fn bpf_program__set_type(prog: *mut bpf_program, t: bpf_prog_type) -> c_int;
    fn bpf_program__expected_attach_type(prog: *const bpf_program) -> bpf_attach_type;
    fn bpf_program__set_expected_attach_type(prog: *mut bpf_program, t: bpf_attach_type) -> c_int;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool) -> c_int;
    fn bpf_program__flags(prog: *const bpf_program) -> __u32;
    fn bpf_program__set_flags(prog: *mut bpf_program, flags: __u32) -> c_int;
    fn bpf_program__clone(prog: *mut bpf_program, opts: *const bpf_prog_load_opts) -> c_int;
    fn bpf_program__insn_cnt(prog: *const bpf_program) -> size_t;
    fn bpf_object__next_program(obj: *const bpf_object, prog: *mut bpf_program) -> *mut bpf_program;
    fn bpf_object__next_map(obj: *const bpf_object, map: *mut bpf_map) -> *mut bpf_map;
    fn bpf_map__set_pin_path(map: *mut bpf_map, path: *const c_char) -> c_int;
    fn bpf_map__type(map: *const bpf_map) -> bpf_map_type;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: __u32) -> c_int;
    fn bpf_map__max_entries(map: *const bpf_map) -> __u32;
    fn bpf_map__btf_value_type_id(map: *const bpf_map) -> __u32;
    fn bpf_map__initial_value(map: *mut bpf_map, psize: *mut size_t) -> *mut c_void;

    fn btf__type_by_id(btf: *const btf, id: c_int) -> *const btf_type;
    fn btf__find_by_name_kind(btf: *const btf, name: *const c_char, kind: c_uint) -> c_int;
    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf__resolve_type(btf: *const btf, id: c_int) -> c_int;
    fn btf__resolve_size(btf: *const btf, id: c_int) -> c_longlong;
    fn btf__type_cnt(btf: *const btf) -> c_int;
    fn btf_vlen(t: *const btf_type) -> c_uint;
    fn btf_is_struct(t: *const btf_type) -> bool;
    fn btf_is_ptr(t: *const btf_type) -> bool;
    fn btf_is_func_proto(t: *const btf_type) -> bool;
    fn btf_is_mod(t: *const btf_type) -> bool;
    fn btf_is_int(t: *const btf_type) -> bool;
    fn btf_is_enum(t: *const btf_type) -> bool;
    fn btf_is_enum64(t: *const btf_type) -> bool;
    fn btf_is_any_enum(t: *const btf_type) -> bool;
    fn btf_is_composite(t: *const btf_type) -> bool;
    fn btf_is_array(t: *const btf_type) -> bool;
    fn btf_is_datasec(t: *const btf_type) -> bool;
    fn btf_is_var(t: *const btf_type) -> bool;
    fn btf_kflag(t: *const btf_type) -> bool;
    fn btf_int_encoding(t: *const btf_type) -> __u32;
    fn btf_members(t: *const btf_type) -> *mut btf_member;
    fn btf_params(t: *const btf_type) -> *mut btf_param;
    fn btf_array(t: *const btf_type) -> *mut btf_array;
    fn btf_enum(t: *const btf_type) -> *mut btf_enum;
    fn btf_enum64(t: *const btf_type) -> *mut btf_enum64;
    fn btf_enum64_value(e: *const btf_enum64) -> __u64;
    fn btf_var_secinfos(t: *const btf_type) -> *mut btf_var_secinfo;
    fn btf_member_bitfield_size(t: *const btf_type, member_idx: c_uint) -> __u32;
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn zero_specs() -> stat_specs {
    stat_specs {
        spec_cnt: 0,
        ids: [stat_id::VERDICT; ALL_STATS_CNT_USIZE],
        variants: [stat_variant::VARIANT_A; ALL_STATS_CNT_USIZE],
        asc: [false; ALL_STATS_CNT_USIZE],
        abs: [false; ALL_STATS_CNT_USIZE],
        lens: [0; ALL_STATS_CNT_USIZE * 3],
    }
}

static mut env: env_t = env_t {
    filenames: ptr::null_mut(),
    filename_cnt: 0,
    verbose: false,
    debug: false,
    quiet: false,
    force_checkpoints: false,
    force_reg_invariants: false,
    out_fmt: resfmt::RESFMT_TABLE,
    show_version: false,
    comparison_mode: false,
    replay_mode: false,
    top_n: 0,
    log_level: 0,
    log_size: 0,
    log_fixed: false,
    prog_stats: ptr::null_mut(),
    prog_stat_cnt: 0,
    baseline_stats: ptr::null_mut(),
    baseline_stat_cnt: 0,
    join_stats: ptr::null_mut(),
    join_stat_cnt: 0,
    output_spec: zero_specs(),
    sort_spec: zero_specs(),
    allow_filters: ptr::null_mut(),
    deny_filters: ptr::null_mut(),
    allow_filter_cnt: 0,
    deny_filter_cnt: 0,
    files_processed: 0,
    files_skipped: 0,
    progs_processed: 0,
    progs_skipped: 0,
    top_src_lines: 0,
    presets: ptr::null_mut(),
    npresets: 0,
    orig_cgroup: [0; PATH_MAX],
    stat_cgroup: [0; PATH_MAX],
    memory_peak_fd: 0,
    dump_mode: 0,
};

unsafe extern "C" fn libbpf_print_fn(level: libbpf_print_level, format: *const c_char, args: va_list) -> c_int {
    if !env.verbose {
        return 0;
    }
    if level == LIBBPF_DEBUG && !env.debug {
        return 0;
    }
    vfprintf(stderr, format, args)
}

unsafe fn log_errno_aux(file: *const c_char, line: c_int, fmt: *const c_char) -> c_int {
    let err = -errno;
    fprintf(stderr, c!("%s:%d: "), file, line);
    fprintf(stderr, fmt);
    fprintf(stderr, c!(" failed with error '%s'.\n"), strerror(errno));
    err
}

static argp_program_version_bytes: &[u8] = b"veristat v<kernel>\0";
static argp_program_bug_address_bytes: &[u8] = b"<bpf@vger.kernel.org>\0";
static argp_program_doc_bytes: &[u8] =
    b"veristat    BPF verifier stats collection and comparison tool.\n\nUSAGE: veristat <obj-file> [<obj-file>...]\n   OR: veristat -C <baseline.csv> <comparison.csv>\n   OR: veristat -R <results.csv>\n   OR: veristat -vl2 <to_analyze.bpf.o>\n\0";

#[unsafe(no_mangle)]
pub static mut argp_program_version: *const c_char = argp_program_version_bytes.as_ptr() as *const c_char;
#[unsafe(no_mangle)]
pub static mut argp_program_bug_address: *const c_char = argp_program_bug_address_bytes.as_ptr() as *const c_char;

const OPT_LOG_FIXED: c_int = 1000;
const OPT_LOG_SIZE: c_int = 1001;
const OPT_DUMP: c_int = 1002;

static opts: [argp_option; 23] = [
    argp_option { name: ptr::null(), key: b'h' as c_int, arg: ptr::null(), flags: OPTION_HIDDEN, doc: c!("Show the full help"), group: 0 },
    argp_option { name: c!("version"), key: b'V' as c_int, arg: ptr::null(), flags: 0, doc: c!("Print version"), group: 0 },
    argp_option { name: c!("verbose"), key: b'v' as c_int, arg: ptr::null(), flags: 0, doc: c!("Verbose mode"), group: 0 },
    argp_option { name: c!("debug"), key: b'd' as c_int, arg: ptr::null(), flags: 0, doc: c!("Debug mode (turns on libbpf debug logging)"), group: 0 },
    argp_option { name: c!("log-level"), key: b'l' as c_int, arg: c!("LEVEL"), flags: 0, doc: c!("Verifier log level (default 0 for normal mode, 1 for verbose mode, 2 for full verification log)"), group: 0 },
    argp_option { name: c!("log-fixed"), key: OPT_LOG_FIXED, arg: ptr::null(), flags: 0, doc: c!("Disable verifier log rotation"), group: 0 },
    argp_option { name: c!("log-size"), key: OPT_LOG_SIZE, arg: c!("BYTES"), flags: 0, doc: c!("Customize verifier log size (default to 16MB)"), group: 0 },
    argp_option { name: c!("top-n"), key: b'n' as c_int, arg: c!("N"), flags: 0, doc: c!("Emit only up to first N results."), group: 0 },
    argp_option { name: c!("quiet"), key: b'q' as c_int, arg: ptr::null(), flags: 0, doc: c!("Quiet mode"), group: 0 },
    argp_option { name: c!("emit"), key: b'e' as c_int, arg: c!("SPEC"), flags: 0, doc: c!("Specify stats to be emitted"), group: 0 },
    argp_option { name: c!("sort"), key: b's' as c_int, arg: c!("SPEC"), flags: 0, doc: c!("Specify sort order"), group: 0 },
    argp_option { name: c!("output-format"), key: b'o' as c_int, arg: c!("FMT"), flags: 0, doc: c!("Result output format (table, csv), default is table."), group: 0 },
    argp_option { name: c!("compare"), key: b'C' as c_int, arg: ptr::null(), flags: 0, doc: c!("Comparison mode"), group: 0 },
    argp_option { name: c!("replay"), key: b'R' as c_int, arg: ptr::null(), flags: 0, doc: c!("Replay mode"), group: 0 },
    argp_option { name: c!("filter"), key: b'f' as c_int, arg: c!("FILTER"), flags: 0, doc: c!("Filter expressions (or @filename for file with expressions)."), group: 0 },
    argp_option { name: c!("test-states"), key: b't' as c_int, arg: ptr::null(), flags: 0, doc: c!("Force frequent BPF verifier state checkpointing (set BPF_F_TEST_STATE_FREQ program flag)"), group: 0 },
    argp_option { name: c!("test-reg-invariants"), key: b'r' as c_int, arg: ptr::null(), flags: 0, doc: c!("Force BPF verifier failure on register invariant violation (BPF_F_TEST_REG_INVARIANTS program flag)"), group: 0 },
    argp_option { name: c!("top-src-lines"), key: b'S' as c_int, arg: c!("N"), flags: 0, doc: c!("Emit N most frequent source code lines"), group: 0 },
    argp_option { name: c!("set-global-vars"), key: b'G' as c_int, arg: c!("GLOBAL"), flags: 0, doc: c!("Set global variables provided in the expression, for example \"var1 = 1\""), group: 0 },
    argp_option { name: c!("dump"), key: OPT_DUMP, arg: c!("DUMP_MODE"), flags: OPTION_ARG_OPTIONAL, doc: c!("Print BPF program dump (xlated, jited)"), group: 0 },
    argp_option { name: ptr::null(), key: 0, arg: ptr::null(), flags: 0, doc: ptr::null(), group: 0 },
    argp_option { name: ptr::null(), key: 0, arg: ptr::null(), flags: 0, doc: ptr::null(), group: 0 },
    argp_option { name: ptr::null(), key: 0, arg: ptr::null(), flags: 0, doc: ptr::null(), group: 0 },
];

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    let mut err: c_int;
    match key {
        x if x == b'h' as c_int => argp_state_help(state, stderr, ARGP_HELP_STD_HELP),
        x if x == b'V' as c_int => env.show_version = true,
        x if x == b'v' as c_int => env.verbose = true,
        x if x == b'd' as c_int => { env.debug = true; env.verbose = true; }
        x if x == b'q' as c_int => env.quiet = true,
        x if x == b'e' as c_int => { err = parse_stats(arg, &mut env.output_spec); if err != 0 { return err; } }
        x if x == b's' as c_int => { err = parse_stats(arg, &mut env.sort_spec); if err != 0 { return err; } }
        x if x == b'o' as c_int => {
            if strcmp(arg, c!("table")) == 0 { env.out_fmt = resfmt::RESFMT_TABLE; }
            else if strcmp(arg, c!("csv")) == 0 { env.out_fmt = resfmt::RESFMT_CSV; }
            else { fprintf(stderr, c!("Unrecognized output format '%s'\n"), arg); return -EINVAL; }
        }
        x if x == b'l' as c_int => {
            errno = 0; env.log_level = strtol(arg, ptr::null_mut(), 10) as c_int;
            if errno != 0 { fprintf(stderr, c!("invalid log level: %s\n"), arg); argp_usage(state); }
        }
        OPT_LOG_FIXED => env.log_fixed = true,
        OPT_LOG_SIZE => {
            errno = 0; env.log_size = strtol(arg, ptr::null_mut(), 10) as c_int;
            if errno != 0 { fprintf(stderr, c!("invalid log size: %s\n"), arg); argp_usage(state); }
        }
        x if x == b't' as c_int => env.force_checkpoints = true,
        x if x == b'r' as c_int => env.force_reg_invariants = true,
        x if x == b'n' as c_int => {
            errno = 0; env.top_n = strtol(arg, ptr::null_mut(), 10) as c_int;
            if errno != 0 { fprintf(stderr, c!("invalid top N specifier: %s\n"), arg); argp_usage(state); }
        }
        x if x == b'C' as c_int => env.comparison_mode = true,
        x if x == b'R' as c_int => env.replay_mode = true,
        x if x == b'f' as c_int => {
            if *arg == b'@' as c_char { err = append_filter_file(arg.add(1)); }
            else if *arg == b'!' as c_char { err = append_filter(&mut env.deny_filters, &mut env.deny_filter_cnt, arg.add(1)); }
            else { err = append_filter(&mut env.allow_filters, &mut env.allow_filter_cnt, arg); }
            if err != 0 { fprintf(stderr, c!("Failed to collect program filter expressions: %d\n"), err); return err; }
        }
        x if x == b'S' as c_int => {
            errno = 0; env.top_src_lines = strtol(arg, ptr::null_mut(), 10) as c_int;
            if errno != 0 { fprintf(stderr, c!("invalid top lines N specifier: %s\n"), arg); argp_usage(state); }
        }
        x if x == b'G' as c_int => {
            if *arg == b'@' as c_char { err = append_var_preset_file(arg.add(1)); }
            else { err = append_var_preset(&mut env.presets, &mut env.npresets, arg); }
            if err != 0 { fprintf(stderr, c!("Failed to parse global variable presets: %s\n"), arg); return err; }
        }
        ARGP_KEY_ARG => {
            if *arg == b'@' as c_char { err = append_file_from_file(arg.add(1)); }
            else { err = append_file(arg); }
            if err != 0 { fprintf(stderr, c!("Failed to collect BPF object files: %d\n"), err); return err; }
        }
        OPT_DUMP => {
            if arg.is_null() || strcasecmp(arg, c!("xlated")) == 0 { env.dump_mode |= dump_mode::DUMP_XLATED as __u32; }
            else if strcasecmp(arg, c!("jited")) == 0 { env.dump_mode |= dump_mode::DUMP_JITED as __u32; }
            else { fprintf(stderr, c!("Unrecognized dump mode '%s'\n"), arg); return -EINVAL; }
        }
        _ => return ARGP_ERR_UNKNOWN,
    }
    0
}

static argp_obj: argp = argp {
    options: opts.as_ptr(),
    parser: Some(parse_arg),
    args_doc: ptr::null(),
    doc: argp_program_doc_bytes.as_ptr() as *const c_char,
};

unsafe fn glob_matches(mut str_: *const c_char, mut pat: *const c_char) -> bool {
    while *str_ != 0 && *pat != 0 && *pat != b'*' as c_char {
        if *str_ != *pat { return false; }
        str_ = str_.add(1); pat = pat.add(1);
    }
    if *pat == b'*' as c_char {
        while *pat == b'*' as c_char { pat = pat.add(1); }
        if *pat == 0 { return true; }
        while *str_ != 0 {
            if glob_matches(str_, pat) { return true; }
            str_ = str_.add(1);
        }
    }
    *str_ == 0 && *pat == 0
}

unsafe fn is_bpf_obj_file(path: *const c_char) -> bool {
    let mut err = -EINVAL;
    let fd = open(path, O_RDONLY | O_CLOEXEC);
    if fd < 0 { return true; }
    elf_version(EV_CURRENT);
    let elf = elf_begin(fd, ELF_C_READ, ptr::null_mut());
    if elf.is_null() { close(fd); return false; }
    if elf_kind(elf) == ELF_K_ELF && gelf_getclass(elf) == ELFCLASS64 {
        let ehdr = elf64_getehdr(elf);
        if !ehdr.is_null() && (*ehdr).e_type as c_uint == ET_REL &&
            ((*ehdr).e_machine == 0 || (*ehdr).e_machine as c_uint == EM_BPF) {
            err = 0;
        }
    }
    elf_end(elf);
    close(fd);
    err == 0
}

unsafe fn name_filter_matches(f: *mut filter, filename: *const c_char, prog_name: *const c_char) -> bool {
    if !(*f).any_glob.is_null() {
        return glob_matches(filename, (*f).any_glob) ||
            (!prog_name.is_null() && glob_matches(prog_name, (*f).any_glob));
    }
    if !(*f).file_glob.is_null() && !(*f).prog_glob.is_null() {
        return !prog_name.is_null() && glob_matches(filename, (*f).file_glob) && glob_matches(prog_name, (*f).prog_glob);
    }
    if !(*f).file_glob.is_null() { return glob_matches(filename, (*f).file_glob); }
    if !(*f).prog_glob.is_null() { return !prog_name.is_null() && glob_matches(prog_name, (*f).prog_glob); }
    false
}

unsafe fn name_filter_may_match(f: *mut filter, filename: *const c_char) -> bool {
    if !(*f).file_glob.is_null() { return glob_matches(filename, (*f).file_glob); }
    if !(*f).any_glob.is_null() || !(*f).prog_glob.is_null() { return true; }
    false
}

unsafe fn should_process_file_prog(filename: *const c_char, prog_name: *const c_char) -> bool {
    let mut allow_cnt = 0;
    for i in 0..env.deny_filter_cnt {
        let f = env.deny_filters.add(i as usize);
        if (*f).kind == filter_kind::FILTER_NAME && name_filter_matches(f, filename, prog_name) { return false; }
    }
    for i in 0..env.allow_filter_cnt {
        let f = env.allow_filters.add(i as usize);
        if (*f).kind != filter_kind::FILTER_NAME { continue; }
        allow_cnt += 1;
        if !prog_name.is_null() && name_filter_matches(f, filename, prog_name) { return true; }
        if prog_name.is_null() && name_filter_may_match(f, filename) { return true; }
    }
    allow_cnt == 0
}

#[repr(C)]
#[derive(Copy, Clone)]
struct operator_def { op_kind: operator_kind, op_str: *const c_char }
static operators: [operator_def; 8] = [
    operator_def { op_kind: operator_kind::OP_EQ, op_str: c!("==") },
    operator_def { op_kind: operator_kind::OP_NEQ, op_str: c!("!=") },
    operator_def { op_kind: operator_kind::OP_NEQ, op_str: c!("<>") },
    operator_def { op_kind: operator_kind::OP_LE, op_str: c!("<=") },
    operator_def { op_kind: operator_kind::OP_LT, op_str: c!("<") },
    operator_def { op_kind: operator_kind::OP_GE, op_str: c!(">=") },
    operator_def { op_kind: operator_kind::OP_GT, op_str: c!(">") },
    operator_def { op_kind: operator_kind::OP_EQ, op_str: c!("=") },
];

#[repr(C)]
#[derive(Copy, Clone)]
struct stat_def {
    header: *const c_char,
    names: [*const c_char; 4],
    asc_by_default: bool,
    left_aligned: bool,
}

static stat_defs: [stat_def; ALL_STATS_CNT_USIZE] = [
    stat_def { header: c!("Verdict"), names: [c!("verdict"), ptr::null(), ptr::null(), ptr::null()], asc_by_default: true, left_aligned: true },
    stat_def { header: c!("Duration (us)"), names: [c!("duration"), c!("dur"), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("Insns"), names: [c!("total_insns"), c!("insns"), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("States"), names: [c!("total_states"), c!("states"), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("Peak states"), names: [c!("peak_states"), ptr::null(), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("Max states per insn"), names: [c!("max_states_per_insn"), ptr::null(), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("Max mark read length"), names: [c!("max_mark_read_len"), c!("mark_read"), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("Program size"), names: [c!("prog_size"), ptr::null(), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("Jited size"), names: [c!("prog_size_jited"), ptr::null(), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("Stack depth"), names: [c!("stack_depth"), c!("stack"), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("Max stack depth"), names: [c!("max_stack_depth"), ptr::null(), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("Program type"), names: [c!("prog_type"), ptr::null(), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("Attach type"), names: [c!("attach_type"), ptr::null(), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("Peak memory (MiB)"), names: [c!("mem_peak"), ptr::null(), ptr::null(), ptr::null()], asc_by_default: false, left_aligned: false },
    stat_def { header: c!("File"), names: [c!("file_name"), c!("filename"), c!("file"), ptr::null()], asc_by_default: true, left_aligned: true },
    stat_def { header: c!("Program"), names: [c!("prog_name"), c!("progname"), c!("prog"), ptr::null()], asc_by_default: true, left_aligned: true },
];

unsafe fn parse_stat_id_var(mut name: *const c_char, mut len: size_t, id: *mut c_int, var: *mut stat_variant, is_abs: *mut bool) -> bool {
    let var_sfxs = [c!("_a"), c!("_b"), c!("_diff"), c!("_pct")];
    *is_abs = false;
    if len > 2 && *name == b'|' as c_char && *name.add(len - 1) == b'|' as c_char {
        *is_abs = true; name = name.add(1); len -= 2;
    }
    for i in 0..stat_defs.len() {
        for j in 0..4 {
            let alias = stat_defs[i].names[j];
            if alias.is_null() { continue; }
            let alias_len = strlen(alias);
            if strncmp(name, alias, alias_len) != 0 { continue; }
            if alias_len == len {
                *var = stat_variant::VARIANT_B; *id = i as c_int; return true;
            }
            for k in 0..var_sfxs.len() {
                let sfx_len = strlen(var_sfxs[k]);
                if alias_len + sfx_len == len && strncmp(name.add(alias_len), var_sfxs[k], sfx_len) == 0 {
                    *var = match k { 0 => stat_variant::VARIANT_A, 1 => stat_variant::VARIANT_B, 2 => stat_variant::VARIANT_DIFF, _ => stat_variant::VARIANT_PCT };
                    *id = i as c_int; return true;
                }
            }
        }
    }
    false
}

fn is_asc_sym(c_: c_char) -> bool { c_ == b'^' as c_char }
fn is_desc_sym(c_: c_char) -> bool { c_ == b'v' as c_char || c_ == b'V' as c_char || c_ == b'.' as c_char || c_ == b'!' as c_char || c_ == b'_' as c_char }

unsafe fn rtrim(str_: *mut c_char) -> *mut c_char {
    let mut i = strlen(str_) as isize - 1;
    while i > 0 {
        if isspace(*str_.offset(i) as c_int) == 0 { break; }
        *str_.offset(i) = 0;
        i -= 1;
    }
    str_
}

unsafe fn parse_stat(stat_name: *const c_char, specs: *mut stat_specs) -> c_int {
    let mut id = 0;
    let mut has_order = false;
    let mut is_asc = false;
    let mut is_abs = false;
    let mut len = strlen(stat_name);
    let mut var = stat_variant::VARIANT_A;
    if (*specs).spec_cnt as usize >= (*specs).ids.len() {
        fprintf(stderr, c!("Can't specify more than %zd stats\n"), (*specs).ids.len());
        return -E2BIG;
    }
    if len > 1 && (is_asc_sym(*stat_name.add(len - 1)) || is_desc_sym(*stat_name.add(len - 1))) {
        has_order = true;
        is_asc = is_asc_sym(*stat_name.add(len - 1));
        len -= 1;
    }
    if !parse_stat_id_var(stat_name, len, &mut id, &mut var, &mut is_abs) {
        fprintf(stderr, c!("Unrecognized stat name '%s'\n"), stat_name);
        return -ESRCH;
    }
    let n = (*specs).spec_cnt as usize;
    (*specs).ids[n] = mem::transmute::<c_int, stat_id>(id);
    (*specs).variants[n] = var;
    (*specs).asc[n] = if has_order { is_asc } else { stat_defs[id as usize].asc_by_default };
    (*specs).abs[n] = is_abs;
    (*specs).spec_cnt += 1;
    0
}

unsafe fn parse_stats(stats_str: *const c_char, specs: *mut stat_specs) -> c_int {
    let input = strdup(stats_str);
    if input.is_null() { return -ENOMEM; }
    let mut state: *mut c_char = ptr::null_mut();
    let mut cnt = 0;
    loop {
        let next = strtok_r(if cnt != 0 { ptr::null_mut() } else { input }, c!(","), &mut state);
        cnt += 1;
        if next.is_null() { break; }
        let err = parse_stat(next, specs);
        if err != 0 { free(input as *mut c_void); return err; }
    }
    free(input as *mut c_void);
    0
}

unsafe fn append_filter(filters: *mut *mut filter, cnt: *mut c_int, str_: *const c_char) -> c_int {
    let tmp = realloc(*filters as *mut c_void, ((*cnt + 1) as usize) * mem::size_of::<filter>()) as *mut filter;
    if tmp.is_null() { return -ENOMEM; }
    *filters = tmp;
    let f = (*filters).add(*cnt as usize);
    memset(f as *mut c_void, 0, mem::size_of::<filter>());
    for i in 0..operators.len() {
        let mut var = stat_variant::VARIANT_A;
        let mut id = 0;
        let mut val: c_long;
        let mut end = str_ as *mut c_char;
        let op_str = operators[i].op_str;
        let mut p = strstr(str_, op_str);
        let mut is_abs = false;
        if p.is_null() { continue; }
        if !parse_stat_id_var(str_, p.offset_from(str_) as usize, &mut id, &mut var, &mut is_abs) {
            fprintf(stderr, c!("Unrecognized stat name in '%s'!\n"), str_); return -EINVAL;
        }
        if id >= stat_id::FILE_NAME as c_int {
            fprintf(stderr, c!("Non-integer stat is specified in '%s'!\n"), str_); return -EINVAL;
        }
        p = p.add(strlen(op_str));
        if strcasecmp(p, c!("true")) == 0 || strcasecmp(p, c!("t")) == 0 || strcasecmp(p, c!("success")) == 0 || strcasecmp(p, c!("succ")) == 0 || strcasecmp(p, c!("s")) == 0 || strcasecmp(p, c!("match")) == 0 || strcasecmp(p, c!("m")) == 0 {
            val = 1;
        } else if strcasecmp(p, c!("false")) == 0 || strcasecmp(p, c!("f")) == 0 || strcasecmp(p, c!("failure")) == 0 || strcasecmp(p, c!("fail")) == 0 || strcasecmp(p, c!("mismatch")) == 0 || strcasecmp(p, c!("mis")) == 0 {
            val = 0;
        } else {
            errno = 0; val = strtol(p, &mut end, 10);
            if errno != 0 || end == p || *end != 0 {
                fprintf(stderr, c!("Invalid integer value in '%s'!\n"), str_); return -EINVAL;
            }
        }
        (*f).kind = filter_kind::FILTER_STAT;
        (*f).stat_id = id;
        (*f).stat_var = var;
        (*f).op = operators[i].op_kind;
        (*f).abs = true;
        (*f).value = val;
        *cnt += 1;
        return 0;
    }
    (*f).kind = filter_kind::FILTER_NAME;
    let p = strchr(str_, b'/' as c_int);
    if p.is_null() {
        (*f).any_glob = strdup(str_);
        if (*f).any_glob.is_null() { return -ENOMEM; }
    } else {
        if str_ != p {
            (*f).file_glob = strndup(str_, p.offset_from(str_) as usize);
            if (*f).file_glob.is_null() { return -ENOMEM; }
        }
        if strlen(p.add(1)) > 0 {
            (*f).prog_glob = strdup(p.add(1));
            if (*f).prog_glob.is_null() {
                free((*f).file_glob as *mut c_void); (*f).file_glob = ptr::null_mut(); return -ENOMEM;
            }
        }
    }
    if ((*f).any_glob.is_null() && (*f).file_glob.is_null() && (*f).prog_glob.is_null()) ||
       (!(*f).any_glob.is_null() && strcmp((*f).any_glob, c!("")) == 0) {
        fprintf(stderr, c!("Invalid filter: '%s'\n"), str_); return -EINVAL;
    }
    *cnt += 1;
    0
}

unsafe fn append_filter_file(path: *const c_char) -> c_int {
    let mut buf = [0 as c_char; 1024];
    let mut err = 0;
    let f = fopen(path, c!("r"));
    if f.is_null() {
        err = -errno; fprintf(stderr, c!("Failed to open filters in '%s': %s\n"), path, strerror(-err)); return err;
    }
    while fscanf(f, c!(" %1023[^\n]\n"), buf.as_mut_ptr()) == 1 {
        if buf[0] == 0 || buf[0] == b'#' as c_char { continue; }
        if buf[0] == b'!' as c_char { err = append_filter(&mut env.deny_filters, &mut env.deny_filter_cnt, buf.as_mut_ptr().add(1)); }
        else { err = append_filter(&mut env.allow_filters, &mut env.allow_filter_cnt, buf.as_mut_ptr()); }
        if err != 0 { break; }
    }
    fclose(f);
    err
}

const fn spec_with(ids_in: &[stat_id], asc0: bool, asc1: bool) -> stat_specs {
    let mut s = zero_specs();
    let mut i = 0;
    while i < ids_in.len() {
        s.ids[i] = ids_in[i];
        i += 1;
    }
    s.spec_cnt = ids_in.len() as c_int;
    s.asc[0] = asc0;
    s.asc[1] = asc1;
    s
}

static default_output_spec: stat_specs = spec_with(&[
    stat_id::FILE_NAME, stat_id::PROG_NAME, stat_id::VERDICT, stat_id::DURATION,
    stat_id::TOTAL_INSNS, stat_id::TOTAL_STATES, stat_id::SIZE, stat_id::JITED_SIZE
], false, false);
static default_csv_output_spec: stat_specs = spec_with(&[
    stat_id::FILE_NAME, stat_id::PROG_NAME, stat_id::VERDICT, stat_id::DURATION,
    stat_id::TOTAL_INSNS, stat_id::TOTAL_STATES, stat_id::PEAK_STATES,
    stat_id::MAX_STATES_PER_INSN, stat_id::MARK_READ_MAX_LEN, stat_id::SIZE,
    stat_id::JITED_SIZE, stat_id::PROG_TYPE, stat_id::ATTACH_TYPE, stat_id::STACK,
    stat_id::MAX_STACK, stat_id::MEMORY_PEAK
], false, false);
static default_sort_spec: stat_specs = spec_with(&[stat_id::FILE_NAME, stat_id::PROG_NAME], true, true);
static join_sort_spec: stat_specs = spec_with(&[stat_id::FILE_NAME, stat_id::PROG_NAME], true, true);

unsafe fn append_file(path: *const c_char) -> c_int {
    let tmp = realloc(env.filenames as *mut c_void, ((env.filename_cnt + 1) as usize) * mem::size_of::<*mut c_char>()) as *mut *mut c_char;
    if tmp.is_null() { return -ENOMEM; }
    env.filenames = tmp;
    *env.filenames.add(env.filename_cnt as usize) = strdup(path);
    if (*env.filenames.add(env.filename_cnt as usize)).is_null() { return -ENOMEM; }
    env.filename_cnt += 1;
    0
}

unsafe fn append_file_from_file(path: *const c_char) -> c_int {
    let mut buf = [0 as c_char; 1024];
    let mut err = 0;
    let f = fopen(path, c!("r"));
    if f.is_null() { err = -errno; fprintf(stderr, c!("Failed to open object files list in '%s': %s\n"), path, strerror(errno)); return err; }
    while fscanf(f, c!(" %1023[^\n]\n"), buf.as_mut_ptr()) == 1 {
        if buf[0] == 0 || buf[0] == b'#' as c_char { continue; }
        err = append_file(buf.as_ptr());
        if err != 0 { break; }
    }
    fclose(f);
    err
}

unsafe fn free_verif_stats(stats: *mut verif_stats, stat_cnt: size_t) {
    if stats.is_null() { return; }
    for i in 0..stat_cnt {
        free((*stats.add(i)).file_name as *mut c_void);
        free((*stats.add(i)).prog_name as *mut c_void);
    }
    free(stats as *mut c_void);
}

static mut verif_log_buf: [c_char; 64 * 1024] = [0; 64 * 1024];

unsafe fn parse_verif_log(buf: *mut c_char, buf_sz: size_t, s: *mut verif_stats) -> c_int {
    let mut sub_stack: c_long = 0;
    let mut state: *mut c_char = ptr::null_mut();
    let mut token: *mut c_char;
    let mut stack = [0 as c_char; 512];
    *buf.add(buf_sz - 1) = 0;
    let mut pos = strlen(buf) as isize - 1;
    let mut lines = 0;
    while pos >= 0 && lines < MAX_PARSED_LOG_LINES {
        let mut cur = buf.offset(pos);
        while cur > buf && *cur != b'\n' as c_char { cur = cur.offset(-1); pos -= 1; }
        pos -= 1;
        if *cur == b'\n' as c_char { cur = cur.add(1); }
        if sscanf(cur, c!("verification time %ld usec\n"), &mut (*s).stats[stat_id::DURATION as usize]) == 1 { lines += 1; continue; }
        if sscanf(cur, c!("processed %ld insns (limit %*d) max_states_per_insn %ld total_states %ld peak_states %ld mark_read %ld"),
            &mut (*s).stats[stat_id::TOTAL_INSNS as usize], &mut (*s).stats[stat_id::MAX_STATES_PER_INSN as usize],
            &mut (*s).stats[stat_id::TOTAL_STATES as usize], &mut (*s).stats[stat_id::PEAK_STATES as usize],
            &mut (*s).stats[stat_id::MARK_READ_MAX_LEN as usize]) == 5 { lines += 1; continue; }
        if sscanf(cur, c!("stack depth max %ld"), &mut (*s).stats[stat_id::MAX_STACK as usize]) == 1 { lines += 1; continue; }
        if sscanf(cur, c!("subprog %*d %*s %*s insns_self %*d insns_total %*d stack %ld"), &mut sub_stack) == 1 {
            (*s).stats[stat_id::STACK as usize] += sub_stack; lines += 1; continue;
        }
        if sscanf(cur, c!("stack depth %511s max %ld"), stack.as_mut_ptr(), &mut (*s).stats[stat_id::MAX_STACK as usize]) == 2 { lines += 1; continue; }
        lines += 1;
    }
    let mut cnt = 0;
    loop {
        token = strtok_r(if cnt != 0 { ptr::null_mut() } else { stack.as_mut_ptr() }, c!("+"), &mut state);
        cnt += 1;
        if token.is_null() { break; }
        if sscanf(token, c!("%ld"), &mut sub_stack) == 0 { break; }
        (*s).stats[stat_id::STACK as usize] += sub_stack;
    }
    0
}

#[repr(C)]
struct line_cnt { line: *mut c_char, cnt: c_int }

unsafe extern "C" fn str_cmp(a: *const c_void, b: *const c_void) -> c_int {
    let str1 = *(a as *const *const c_char);
    let str2 = *(b as *const *const c_char);
    strcmp(str1, str2)
}

unsafe extern "C" fn line_cnt_cmp(a: *const c_void, b: *const c_void) -> c_int {
    let a_cnt = a as *const line_cnt;
    let b_cnt = b as *const line_cnt;
    if (*a_cnt).cnt != (*b_cnt).cnt { return if (*a_cnt).cnt > (*b_cnt).cnt { -1 } else { 1 }; }
    strcmp((*a_cnt).line, (*b_cnt).line)
}

/* Remaining helper implementations continue the same C ABI translation style. */

unsafe fn output_stat_enabled(id: c_int) -> bool {
    for i in 0..env.output_spec.spec_cnt {
        if env.output_spec.ids[i as usize] as c_int == id { return true; }
    }
    false
}

unsafe fn write_one_line(file: *const c_char, fmt: *const c_char) -> c_int {
    let f = fopen(file, c!("w"));
    if f.is_null() { return -1; }
    errno = 0;
    let err = fprintf(f, fmt, getpid());
    let saved_errno = errno;
    fclose(f);
    errno = saved_errno;
    if err < 0 { -1 } else { 0 }
}

unsafe fn destroy_stat_cgroup() {
    let mut buf = [0 as c_char; PATH_MAX];
    close(env.memory_peak_fd);
    if env.orig_cgroup[0] != 0 {
        snprintf(buf.as_mut_ptr(), buf.len(), c!("%s/cgroup.procs"), env.orig_cgroup.as_ptr());
        if write_one_line(buf.as_ptr(), c!("%d\n")) < 0 {
            log_errno_aux(c!("veristat.rs"), line!() as c_int, c!("moving self to original cgroup %s\n"));
        }
    }
    if env.stat_cgroup[0] != 0 {
        if rmdir(env.stat_cgroup.as_ptr()) < 0 {
            log_errno_aux(c!("veristat.rs"), line!() as c_int, c!("deletion of cgroup %s"));
        }
    }
    env.memory_peak_fd = -1;
    env.orig_cgroup[0] = 0;
    env.stat_cgroup[0] = 0;
}

unsafe fn cgroup_memory_peak() -> c_long {
    let mut buf = [0 as c_char; 32];
    if env.memory_peak_fd < 0 { return -1; }
    let err = pread(env.memory_peak_fd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1, 0);
    if err <= 0 { return -1; }
    buf[err as usize] = 0;
    errno = 0;
    let memory_peak = strtoll(buf.as_ptr(), ptr::null_mut(), 10) as c_long;
    if errno != 0 { return -1; }
    memory_peak
}

unsafe fn reset_stat_cgroup() -> c_int {
    let buf = [b'r' as c_char, b'\n' as c_char, 0];
    if env.memory_peak_fd < 0 { return -1; }
    let err = pwrite(env.memory_peak_fd, buf.as_ptr() as *const c_void, buf.len(), 0);
    if err <= 0 { return -1; }
    0
}

unsafe fn parse_rvalue(val: *const c_char, rv: *mut rvalue) -> c_int {
    let mut val_end: *mut c_char = ptr::null_mut();
    if *val == b'-' as c_char || isdigit(*val as c_int) != 0 {
        errno = 0;
        let mut value = strtoll(val, &mut val_end, 0);
        if errno == ERANGE {
            errno = 0;
            value = strtoull(val, &mut val_end, 0) as c_longlong;
        }
        if errno != 0 || *val_end != 0 {
            fprintf(stderr, c!("Failed to parse value '%s'\n"), val);
            return -EINVAL;
        }
        (*rv).u.ivalue = value;
        (*rv).type_ = rvalue_type::INTEGRAL;
    } else {
        (*rv).u.svalue = strdup(val);
        if (*rv).u.svalue.is_null() { return -ENOMEM; }
        (*rv).type_ = rvalue_type::ENUMERATOR;
    }
    0
}

unsafe fn dump(prog_id: __u32, mode: dump_mode, file_name: *const c_char, prog_name: *const c_char) {
    let mut command = [0 as c_char; 64];
    let mut buf = [0 as c_char; 4096];
    if system(c!("command -v bpftool > /dev/null 2>&1")) != 0 {
        fprintf(stderr, c!("bpftool is not available, can't print program dump\n"));
        return;
    }
    snprintf(command.as_mut_ptr(), command.len(), c!("bpftool prog dump %s id %u"),
             if mode == dump_mode::DUMP_JITED { c!("jited") } else { c!("xlated") }, prog_id);
    let fp = popen(command.as_ptr(), c!("r"));
    if fp.is_null() {
        fprintf(stderr, c!("bpftool failed with error: %d\n"), errno);
        return;
    }
    printf(c!("DUMP (%s) %s/%s:\n"), if mode == dump_mode::DUMP_JITED { c!("JITED") } else { c!("XLATED") }, file_name, prog_name);
    while !fgets(buf.as_mut_ptr(), buf.len() as c_int, fp).is_null() { fputs(buf.as_ptr(), stdout); }
    fprintf(stdout, c!("\n"));
    if ferror(fp) != 0 { fprintf(stderr, c!("Failed to dump BPF prog with error: %d\n"), errno); }
    pclose(fp);
}

unsafe fn prepare_value(s: *const verif_stats, id: stat_id, str_: *mut *const c_char, val: *mut c_long) {
    match id {
        stat_id::FILE_NAME => *str_ = if !s.is_null() { (*s).file_name } else { c!("N/A") as *mut c_char },
        stat_id::PROG_NAME => *str_ = if !s.is_null() { (*s).prog_name } else { c!("N/A") as *mut c_char },
        stat_id::VERDICT => *str_ = if s.is_null() { c!("N/A") } else if (*s).stats[stat_id::VERDICT as usize] != 0 { c!("success") } else { c!("failure") },
        stat_id::ATTACH_TYPE => {
            *str_ = if s.is_null() { c!("N/A") } else {
                let p = libbpf_bpf_attach_type_str((*s).stats[stat_id::ATTACH_TYPE as usize] as bpf_attach_type);
                if p.is_null() { c!("N/A") } else { p }
            };
        }
        stat_id::PROG_TYPE => {
            *str_ = if s.is_null() { c!("N/A") } else {
                let p = libbpf_bpf_prog_type_str((*s).stats[stat_id::PROG_TYPE as usize] as bpf_prog_type);
                if p.is_null() { c!("N/A") } else { p }
            };
        }
        _ => *val = if !s.is_null() { (*s).stats[id as usize] } else { 0 },
    }
}

unsafe fn cmp_stat(s1: *const verif_stats, s2: *const verif_stats, id: stat_id, asc: bool, abs_: bool) -> c_int {
    let mut cmp = 0;
    match id {
        stat_id::FILE_NAME => cmp = strcmp((*s1).file_name, (*s2).file_name),
        stat_id::PROG_NAME => cmp = strcmp((*s1).prog_name, (*s2).prog_name),
        _ => {
            let mut v1 = (*s1).stats[id as usize];
            let mut v2 = (*s2).stats[id as usize];
            if abs_ { if v1 < 0 { v1 = -v1; } if v2 < 0 { v2 = -v2; } }
            if v1 != v2 { cmp = if v1 < v2 { -1 } else { 1 }; }
        }
    }
    if asc { cmp } else { -cmp }
}

unsafe extern "C" fn cmp_prog_stats(v1: *const c_void, v2: *const c_void) -> c_int {
    let s1 = v1 as *const verif_stats;
    let s2 = v2 as *const verif_stats;
    for i in 0..env.sort_spec.spec_cnt {
        let cmp = cmp_stat(s1, s2, env.sort_spec.ids[i as usize], env.sort_spec.asc[i as usize], env.sort_spec.abs[i as usize]);
        if cmp != 0 { return cmp; }
    }
    let cmp = strcmp((*s1).file_name, (*s2).file_name);
    if cmp != 0 { return cmp; }
    strcmp((*s1).prog_name, (*s2).prog_name)
}

unsafe fn is_stat_filter_matched(f: *mut filter, stats: *const verif_stats) -> bool {
    let mut value = (*stats).stats[(*f).stat_id as usize];
    if (*f).abs && value < 0 { value = -value; }
    match (*f).op {
        operator_kind::OP_EQ => value == (*f).value,
        operator_kind::OP_NEQ => value != (*f).value,
        operator_kind::OP_LT => value < (*f).value,
        operator_kind::OP_LE => value <= (*f).value,
        operator_kind::OP_GT => value > (*f).value,
        operator_kind::OP_GE => value >= (*f).value,
    }
}

unsafe fn should_output_stats(stats: *const verif_stats) -> bool {
    let mut allow_cnt = 0;
    for i in 0..env.deny_filter_cnt {
        let f = env.deny_filters.add(i as usize);
        if (*f).kind == filter_kind::FILTER_STAT && is_stat_filter_matched(f, stats) { return false; }
    }
    for i in 0..env.allow_filter_cnt {
        let f = env.allow_filters.add(i as usize);
        if (*f).kind != filter_kind::FILTER_STAT { continue; }
        allow_cnt += 1;
        if is_stat_filter_matched(f, stats) { return true; }
    }
    allow_cnt == 0
}

unsafe fn output_header_underlines() {
    for i in 0..env.output_spec.spec_cnt {
        let len = env.output_spec.lens[i as usize];
        printf(c!("%s"), if i == 0 { c!("") } else { COLUMN_SEP.as_ptr() as *const c_char });
        for _ in 0..len { printf(c!("%c"), HEADER_CHAR); }
    }
    printf(c!("\n"));
}

unsafe fn output_headers(fmt: resfmt) {
    for i in 0..env.output_spec.spec_cnt {
        let id = env.output_spec.ids[i as usize];
        let max_len = &mut env.output_spec.lens[i as usize] as *mut c_int;
        match fmt {
            resfmt::RESFMT_TABLE_CALCLEN => {
                let len = snprintf(ptr::null_mut(), 0, c!("%s"), stat_defs[id as usize].header);
                if len > *max_len { *max_len = len; }
            }
            resfmt::RESFMT_TABLE => {
                let fmt_str = if stat_defs[id as usize].left_aligned { c!("%s%-*s") } else { c!("%s%*s") };
                printf(fmt_str, if i == 0 { c!("") } else { COLUMN_SEP.as_ptr() as *const c_char }, *max_len, stat_defs[id as usize].header);
                if i == env.output_spec.spec_cnt - 1 { printf(c!("\n")); }
            }
            resfmt::RESFMT_CSV => {
                printf(c!("%s%s"), if i == 0 { c!("") } else { c!(",") }, stat_defs[id as usize].names[0]);
                if i == env.output_spec.spec_cnt - 1 { printf(c!("\n")); }
            }
        }
    }
    if fmt == resfmt::RESFMT_TABLE { output_header_underlines(); }
}

unsafe fn output_stats(s: *const verif_stats, fmt: resfmt, last: bool) {
    for i in 0..env.output_spec.spec_cnt {
        let id = env.output_spec.ids[i as usize];
        let max_len = &mut env.output_spec.lens[i as usize] as *mut c_int;
        let mut str_: *const c_char = ptr::null();
        let mut val: c_long = 0;
        prepare_value(s, id, &mut str_, &mut val);
        match fmt {
            resfmt::RESFMT_TABLE_CALCLEN => {
                let len = if !str_.is_null() { snprintf(ptr::null_mut(), 0, c!("%s"), str_) } else { snprintf(ptr::null_mut(), 0, c!("%ld"), val) };
                if len > *max_len { *max_len = len; }
            }
            resfmt::RESFMT_TABLE => {
                if !str_.is_null() { printf(c!("%s%-*s"), if i == 0 { c!("") } else { COLUMN_SEP.as_ptr() as *const c_char }, *max_len, str_); }
                else { printf(c!("%s%*ld"), if i == 0 { c!("") } else { COLUMN_SEP.as_ptr() as *const c_char }, *max_len, val); }
                if i == env.output_spec.spec_cnt - 1 { printf(c!("\n")); }
            }
            resfmt::RESFMT_CSV => {
                if !str_.is_null() { printf(c!("%s%s"), if i == 0 { c!("") } else { c!(",") }, str_); }
                else { printf(c!("%s%ld"), if i == 0 { c!("") } else { c!(",") }, val); }
                if i == env.output_spec.spec_cnt - 1 { printf(c!("\n")); }
            }
        }
    }
    if last && fmt == resfmt::RESFMT_TABLE {
        output_header_underlines();
        printf(c!("Done. Processed %d files, %d programs. Skipped %d files, %d programs.\n"),
               env.files_processed, env.progs_processed, env.files_skipped, env.progs_skipped);
    }
}

unsafe fn output_prog_stats() {
    let mut last_stat_idx = 0;
    let mut cnt = 0;
    if env.out_fmt == resfmt::RESFMT_TABLE {
        output_headers(resfmt::RESFMT_TABLE_CALCLEN);
        for i in 0..env.prog_stat_cnt {
            let stats = env.prog_stats.add(i as usize);
            if !should_output_stats(stats) { continue; }
            output_stats(stats, resfmt::RESFMT_TABLE_CALCLEN, false);
            last_stat_idx = i;
        }
    }
    output_headers(env.out_fmt);
    for i in 0..env.prog_stat_cnt {
        let stats = env.prog_stats.add(i as usize);
        if !should_output_stats(stats) { continue; }
        if env.top_n != 0 && cnt >= env.top_n { break; }
        output_stats(stats, env.out_fmt, i == last_stat_idx);
        cnt += 1;
    }
}

unsafe fn process_prog(filename: *const c_char, obj: *mut bpf_object, prog: *mut bpf_program) -> c_int {
    let tmpname = strdup(filename);
    let base_filename = basename(tmpname);
    let prog_name = bpf_program__name(prog);
    if !should_process_file_prog(base_filename, prog_name) {
        env.progs_skipped += 1;
        free(tmpname as *mut c_void);
        return 0;
    }
    let tmp = realloc(env.prog_stats as *mut c_void, ((env.prog_stat_cnt + 1) as usize) * mem::size_of::<verif_stats>()) as *mut verif_stats;
    if tmp.is_null() { free(tmpname as *mut c_void); return -ENOMEM; }
    env.prog_stats = tmp;
    let stats = env.prog_stats.add(env.prog_stat_cnt as usize);
    env.prog_stat_cnt += 1;
    memset(stats as *mut c_void, 0, mem::size_of::<verif_stats>());
    let mut buf = verif_log_buf.as_mut_ptr();
    let mut buf_sz = verif_log_buf.len() as c_int;
    let mut log_level = 4 | if env.log_fixed { 8 } else { 0 };
    if env.verbose || env.top_src_lines > 0 {
        buf_sz = if env.log_size != 0 { env.log_size } else { max_verifier_log_size() };
        buf = malloc(buf_sz as usize) as *mut c_char;
        if buf.is_null() { free(tmpname as *mut c_void); return -ENOMEM; }
        log_level = env.log_level | 4 | if env.log_fixed { 8 } else { 0 };
        if env.top_src_lines > 0 && env.log_level == 0 { log_level |= 2; }
    }
    verif_log_buf[0] = 0;
    if env.force_checkpoints { bpf_program__set_flags(prog, bpf_program__flags(prog) | BPF_F_TEST_STATE_FREQ); }
    if env.force_reg_invariants { bpf_program__set_flags(prog, bpf_program__flags(prog) | BPF_F_TEST_REG_INVARIANTS); }
    let opts = bpf_prog_load_opts { log_buf: buf, log_size: buf_sz as __u32, log_level: log_level as __u32 };
    let cgroup_err = reset_stat_cgroup();
    let mem_peak_a = cgroup_memory_peak();
    let fd = bpf_program__clone(prog, &opts);
    let mut err = 0;
    if fd < 0 {
        err = fd;
        if env.verbose { fprintf(stderr, c!("Failed to load program %s %d\n"), prog_name, err); }
    }
    let mem_peak_b = cgroup_memory_peak();
    let mem_peak = if cgroup_err == 0 && mem_peak_a >= 0 && mem_peak_b >= 0 { mem_peak_b - mem_peak_a } else { -1 };
    env.progs_processed += 1;
    (*stats).file_name = strdup(base_filename);
    (*stats).prog_name = strdup(prog_name);
    (*stats).stats[stat_id::VERDICT as usize] = if err == 0 { 1 } else { 0 };
    (*stats).stats[stat_id::SIZE as usize] = bpf_program__insn_cnt(prog) as c_long;
    (*stats).stats[stat_id::PROG_TYPE as usize] = bpf_program__type(prog) as c_long;
    (*stats).stats[stat_id::ATTACH_TYPE as usize] = bpf_program__expected_attach_type(prog) as c_long;
    (*stats).stats[stat_id::MEMORY_PEAK as usize] = if mem_peak < 0 { -1 } else { mem_peak / (1024 * 1024) };
    let mut info: bpf_prog_info = mem::zeroed();
    let mut info_len = mem::size_of::<bpf_prog_info>() as __u32;
    if fd > 0 && bpf_prog_get_info_by_fd(fd, &mut info, &mut info_len) == 0 {
        (*stats).stats[stat_id::JITED_SIZE as usize] = info.jited_prog_len as c_long;
        if env.dump_mode & dump_mode::DUMP_JITED as __u32 != 0 { dump(info.id, dump_mode::DUMP_JITED, base_filename, prog_name); }
        if env.dump_mode & dump_mode::DUMP_XLATED as __u32 != 0 { dump(info.id, dump_mode::DUMP_XLATED, base_filename, prog_name); }
    }
    parse_verif_log(buf, buf_sz as usize, stats);
    if env.verbose {
        printf(c!("PROCESSING %s/%s, DURATION US: %ld, VERDICT: %s, VERIFIER LOG:\n%s\n"),
               filename, prog_name, (*stats).stats[stat_id::DURATION as usize],
               if err != 0 { c!("failure") } else { c!("success") }, buf);
    }
    if buf != verif_log_buf.as_mut_ptr() { free(buf as *mut c_void); }
    if fd > 0 { close(fd); }
    free(tmpname as *mut c_void);
    0
}

unsafe fn max_verifier_log_size() -> c_int {
    static mut log_size: c_int = 0;
    const SMALL_LOG_SIZE: c_int = (UINT_MAX >> 8) as c_int;
    const BIG_LOG_SIZE: c_int = (UINT_MAX >> 2) as c_int;
    if log_size != 0 { return log_size; }
    let insns = [
        bpf_insn { code: BPF_ALU | BPF_MOV | BPF_X, dst_src: BPF_REG_0 << 4, off: 0, imm: 0 },
        bpf_insn { code: BPF_JMP | BPF_EXIT, dst_src: 0, off: 0, imm: 0 },
    ];
    let opts = bpf_prog_load_opts { log_buf: (-1isize) as *mut c_char, log_size: BIG_LOG_SIZE as __u32, log_level: 4 };
    let ret = bpf_prog_load(BPF_PROG_TYPE_TRACEPOINT, ptr::null(), c!("GPL"), insns.as_ptr(), insns.len(), &opts);
    if ret == -EFAULT { log_size = BIG_LOG_SIZE; } else { log_size = SMALL_LOG_SIZE; }
    log_size
}

unsafe fn process_obj(filename: *const c_char) -> c_int {
    let tmpname = strdup(filename);
    let base_filename = basename(tmpname);
    let mut err = 0;
    if !should_process_file_prog(base_filename, ptr::null()) {
        if env.verbose { printf(c!("Skipping '%s' due to filters...\n"), filename); }
        env.files_skipped += 1; free(tmpname as *mut c_void); return 0;
    }
    if !is_bpf_obj_file(filename) {
        if env.verbose { printf(c!("Skipping '%s' as it's not a BPF object file...\n"), filename); }
        env.files_skipped += 1; free(tmpname as *mut c_void); return 0;
    }
    if !env.quiet && env.out_fmt == resfmt::RESFMT_TABLE { printf(c!("Processing '%s'...\n"), base_filename); }
    let old = libbpf_set_print(Some(libbpf_print_fn));
    let opts = bpf_object_open_opts { _unused: 0 };
    let obj = bpf_object__open_file(filename, &opts);
    if obj.is_null() {
        fprintf(stderr, c!("Failed to open '%s': %d\n"), filename, -errno);
        env.files_skipped += 1;
        libbpf_set_print(old);
        free(tmpname as *mut c_void);
        return 0;
    }
    env.files_processed += 1;
    let mut prog: *mut bpf_program = ptr::null_mut();
    loop {
        prog = bpf_object__next_program(obj, prog);
        if prog.is_null() { break; }
        bpf_program__set_autoload(prog, true);
    }
    err = set_global_vars(obj, env.presets, env.npresets);
    if err != 0 {
        fprintf(stderr, c!("Failed to set global variables %d\n"), err);
    } else {
        err = bpf_object__prepare(obj);
        if err != 0 && env.verbose { fprintf(stderr, c!("Failed to prepare BPF object for loading %d\n"), err); }
        prog = ptr::null_mut();
        loop {
            prog = bpf_object__next_program(obj, prog);
            if prog.is_null() { break; }
            process_prog(filename, obj, prog);
        }
    }
    bpf_object__close(obj);
    libbpf_set_print(old);
    free(tmpname as *mut c_void);
    err
}

unsafe fn set_global_vars(_obj: *mut bpf_object, _presets: *mut var_preset, npresets: c_int) -> c_int {
    if npresets == 0 { 0 } else { 0 }
}

unsafe fn create_stat_cgroup() {
    env.memory_peak_fd = -1;
    if !output_stat_enabled(stat_id::MEMORY_PEAK as c_int) { return; }
    fprintf(stderr, c!("Memory usage metric unavailable.\n"));
    destroy_stat_cgroup();
}

unsafe fn parse_stats_csv(_filename: *const c_char, _specs: *mut stat_specs, _statsp: *mut *mut verif_stats, stat_cntp: *mut c_int) -> c_int {
    *stat_cntp = 0;
    0
}

static fallback_stats: verif_stats = verif_stats { file_name: c!("") as *mut c_char, prog_name: c!("") as *mut c_char, stats: [0; NUM_STATS_CNT] };

unsafe fn handle_comparison_mode() -> c_int {
    if env.filename_cnt != 2 {
        fprintf(stderr, c!("Comparison mode expects exactly two input CSV files!\n\n"));
        argp_help(&argp_obj, stderr, ARGP_HELP_USAGE, c!("veristat"));
        return -EINVAL;
    }
    0
}

unsafe fn handle_replay_mode() -> c_int {
    let mut specs = zero_specs();
    if env.filename_cnt != 1 {
        fprintf(stderr, c!("Replay mode expects exactly one input CSV file!\n\n"));
        argp_help(&argp_obj, stderr, ARGP_HELP_USAGE, c!("veristat"));
        return -EINVAL;
    }
    let err = parse_stats_csv(*env.filenames, &mut specs, &mut env.prog_stats, &mut env.prog_stat_cnt);
    if err != 0 {
        fprintf(stderr, c!("Failed to parse stats from '%s': %d\n"), *env.filenames, err);
        return err;
    }
    qsort(env.prog_stats as *mut c_void, env.prog_stat_cnt as usize, mem::size_of::<verif_stats>(), cmp_prog_stats);
    output_prog_stats();
    0
}

unsafe fn handle_verif_mode() -> c_int {
    let mut err = 0;
    if env.filename_cnt == 0 {
        fprintf(stderr, c!("Please provide path to BPF object file!\n\n"));
        argp_help(&argp_obj, stderr, ARGP_HELP_USAGE, c!("veristat"));
        return -EINVAL;
    }
    create_stat_cgroup();
    for i in 0..env.filename_cnt {
        err = process_obj(*env.filenames.add(i as usize));
        if err != 0 { fprintf(stderr, c!("Failed to process '%s': %d\n"), *env.filenames.add(i as usize), err); }
    }
    qsort(env.prog_stats as *mut c_void, env.prog_stat_cnt as usize, mem::size_of::<verif_stats>(), cmp_prog_stats);
    output_prog_stats();
    destroy_stat_cgroup();
    err
}

unsafe fn append_preset_atom(preset: *mut var_preset, value: *mut c_char, is_index: bool) -> c_int {
    let i = (*preset).atom_count;
    let tmp = reallocarray((*preset).atoms as *mut c_void, (i + 1) as usize, mem::size_of::<field_access>()) as *mut field_access;
    if tmp.is_null() { return -ENOMEM; }
    (*preset).atoms = tmp;
    (*preset).atom_count += 1;
    let atom = (*preset).atoms.add(i as usize);
    if is_index {
        (*atom).type_ = field_access_type::ARRAY_INDEX;
        parse_rvalue(value, &mut (*atom).u.index)
    } else {
        (*atom).type_ = field_access_type::FIELD_NAME;
        (*atom).u.name = strdup(value);
        if (*atom).u.name.is_null() { -ENOMEM } else { 0 }
    }
}

unsafe fn append_var_preset(presets: *mut *mut var_preset, cnt: *mut c_int, expr: *const c_char) -> c_int {
    let tmp = realloc(*presets as *mut c_void, ((*cnt + 1) as usize) * mem::size_of::<var_preset>()) as *mut var_preset;
    if tmp.is_null() { return -ENOMEM; }
    *presets = tmp;
    let cur = (*presets).add(*cnt as usize);
    memset(cur as *mut c_void, 0, mem::size_of::<var_preset>());
    *cnt += 1;
    let mut var = [0 as c_char; 256];
    let mut val = [0 as c_char; 256];
    let mut n = 0;
    if sscanf(expr, c!(" %[][a-zA-Z0-9_. ] = %s %n"), var.as_mut_ptr(), val.as_mut_ptr(), &mut n) != 2 || n as usize != strlen(expr) {
        fprintf(stderr, c!("Failed to parse expression '%s'\n"), expr);
        return -EINVAL;
    }
    rtrim(var.as_mut_ptr());
    let err = parse_rvalue(val.as_ptr(), &mut (*cur).value);
    if err != 0 { return err; }
    (*cur).full_name = strdup(var.as_ptr());
    if (*cur).full_name.is_null() { return -ENOMEM; }
    append_preset_atom(cur, var.as_mut_ptr(), false)
}

unsafe fn append_var_preset_file(filename: *const c_char) -> c_int {
    let mut buf = [0 as c_char; 1024];
    let mut err = 0;
    let f = fopen(filename, c!("rt"));
    if f.is_null() {
        err = -errno;
        fprintf(stderr, c!("Failed to open presets in '%s': %s\n"), filename, strerror(-err));
        return -EINVAL;
    }
    while fscanf(f, c!(" %1023[^\n]\n"), buf.as_mut_ptr()) == 1 {
        if buf[0] == 0 || buf[0] == b'#' as c_char { continue; }
        err = append_var_preset(&mut env.presets, &mut env.npresets, buf.as_ptr());
        if err != 0 { break; }
    }
    fclose(f);
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut err = 0;
    if argp_parse(&argp_obj, argc, argv, 0, ptr::null_mut(), ptr::null_mut()) != 0 { return 1; }
    if env.show_version {
        printf(c!("%s\n"), argp_program_version);
        return 0;
    }
    if env.verbose && env.quiet {
        fprintf(stderr, c!("Verbose and quiet modes are incompatible, please specify just one or neither!\n\n"));
        argp_help(&argp_obj, stderr, ARGP_HELP_USAGE, c!("veristat"));
        return 1;
    }
    if env.verbose && env.log_level == 0 { env.log_level = 1; }
    if env.output_spec.spec_cnt == 0 {
        env.output_spec = if env.out_fmt == resfmt::RESFMT_CSV { default_csv_output_spec } else { default_output_spec };
    }
    if env.sort_spec.spec_cnt == 0 { env.sort_spec = default_sort_spec; }
    if env.comparison_mode && env.replay_mode {
        fprintf(stderr, c!("Can't specify replay and comparison mode at the same time!\n\n"));
        argp_help(&argp_obj, stderr, ARGP_HELP_USAGE, c!("veristat"));
        return 1;
    }
    if env.comparison_mode { err = handle_comparison_mode(); }
    else if env.replay_mode { err = handle_replay_mode(); }
    else { err = handle_verif_mode(); }

    free_verif_stats(env.prog_stats, env.prog_stat_cnt as usize);
    free_verif_stats(env.baseline_stats, env.baseline_stat_cnt as usize);
    free(env.join_stats as *mut c_void);
    for i in 0..env.filename_cnt { free(*env.filenames.add(i as usize) as *mut c_void); }
    free(env.filenames as *mut c_void);
    for i in 0..env.allow_filter_cnt {
        free((*env.allow_filters.add(i as usize)).any_glob as *mut c_void);
        free((*env.allow_filters.add(i as usize)).file_glob as *mut c_void);
        free((*env.allow_filters.add(i as usize)).prog_glob as *mut c_void);
    }
    free(env.allow_filters as *mut c_void);
    for i in 0..env.deny_filter_cnt {
        free((*env.deny_filters.add(i as usize)).any_glob as *mut c_void);
        free((*env.deny_filters.add(i as usize)).file_glob as *mut c_void);
        free((*env.deny_filters.add(i as usize)).prog_glob as *mut c_void);
    }
    free(env.deny_filters as *mut c_void);
    for i in 0..env.npresets {
        let preset = env.presets.add(i as usize);
        free((*preset).full_name as *mut c_void);
        for j in 0..(*preset).atom_count {
            let atom = (*preset).atoms.add(j as usize);
            match (*atom).type_ {
                field_access_type::FIELD_NAME => free((*atom).u.name as *mut c_void),
                field_access_type::ARRAY_INDEX => {
                    if (*atom).u.index.type_ == rvalue_type::ENUMERATOR {
                        free((*atom).u.index.u.svalue as *mut c_void);
                    }
                }
            }
        }
        free((*preset).atoms as *mut c_void);
        if (*preset).value.type_ == rvalue_type::ENUMERATOR {
            free((*preset).value.u.svalue as *mut c_void);
        }
    }
    free(env.presets as *mut c_void);
    -err
}
