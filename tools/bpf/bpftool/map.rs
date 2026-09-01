// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2017-2018 Netronome Systems, Inc. */

/*
 * Source-level Rust translation of bpf/bpftool/map.c.
 * C headers and project-local declarations are represented as external
 * dependencies or opaque C-compatible types.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_longlong, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type bool_ = bool;

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub key: *const c_void,
    pub pvalue: *mut c_void,
}

#[repr(C)]
pub struct json_writer_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_dumper {
    pub btf: *mut btf,
    pub jw: *mut json_writer_t,
    pub is_plain_text: bool,
}

#[repr(C)]
pub struct bpf_map_info {
    pub type_: __u32,
    pub id: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
    pub name: [c_char; 16],
    pub ifindex: __u32,
    pub netns_dev: u64,
    pub netns_ino: u64,
    pub btf_id: __u32,
    pub btf_key_type_id: __u32,
    pub btf_value_type_id: __u32,
    pub btf_vmlinux_value_type_id: __u32,
}

#[repr(C)]
pub struct bpf_get_fd_by_id_opts {
    pub sz: usize,
    pub open_flags: __u32,
}

#[repr(C)]
pub struct bpf_map_create_opts {
    pub sz: usize,
    pub btf_fd: __u32,
    pub btf_key_type_id: __u32,
    pub btf_value_type_id: __u32,
    pub btf_vmlinux_value_type_id: __u32,
    pub inner_map_fd: c_int,
    pub map_flags: __u32,
    pub map_ifindex: __u32,
}

#[repr(C)]
pub struct cmd {
    pub cmd: *const c_char,
    pub func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

const ENOENT: c_int = 2;
const ENOSPC: c_int = 28;
const EINVAL: c_int = 22;
const BPF_ANY: __u32 = 0;
const BPF_NOEXIST: __u32 = 1;
const BPF_EXIST: __u32 = 2;
const BPF_F_RDONLY: __u32 = 8;
const BPF_OBJ_MAP: c_int = 1;
const BPF_OBJ_NAME_LEN: usize = 16;
type bpf_map_type = __u32;
const BPF_MAP_TYPE_UNSPEC: bpf_map_type = 0;
const BPF_MAP_TYPE_HASH: __u32 = 1;
const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const BPF_MAP_TYPE_PROG_ARRAY: __u32 = 3;
const BPF_MAP_TYPE_PERCPU_HASH: __u32 = 5;
const BPF_MAP_TYPE_PERCPU_ARRAY: __u32 = 6;
const BPF_MAP_TYPE_LRU_PERCPU_HASH: __u32 = 10;
const BPF_MAP_TYPE_ARRAY_OF_MAPS: __u32 = 12;
const BPF_MAP_TYPE_HASH_OF_MAPS: __u32 = 13;
const BPF_MAP_TYPE_REUSEPORT_SOCKARRAY: __u32 = 20;
const BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE: __u32 = 21;

unsafe extern "C" {
    static mut json_wtr: *mut json_writer_t;
    static mut json_output: bool;
    static mut show_pinned: bool;
    static mut refs_table: *mut c_void;
    static mut bin_name: *const c_char;
    static mut stdout: *mut c_void;
    static mut stderr: *mut c_void;
    static mut errno: c_int;

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn atoi(nptr: *const c_char) -> c_int;
    fn atoll(nptr: *const c_char) -> c_longlong;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;

    fn libbpf_bpf_map_type_str(t: c_uint) -> *const c_char;
    fn libbpf_bpf_prog_type_str(t: c_uint) -> *const c_char;
    fn libbpf_find_kernel_btf() -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__load_from_kernel_by_id(id: __u32) -> *mut btf;
    fn btf_dumper_type(d: *const btf_dumper, type_id: __u32, data: *mut c_void) -> c_int;

    fn jsonw_new(stream: *mut c_void) -> *mut json_writer_t;
    fn jsonw_destroy(w: *mut *mut json_writer_t);
    fn jsonw_pretty(w: *mut json_writer_t, pretty: bool);
    fn jsonw_start_object(w: *mut json_writer_t);
    fn jsonw_end_object(w: *mut json_writer_t);
    fn jsonw_start_array(w: *mut json_writer_t);
    fn jsonw_end_array(w: *mut json_writer_t);
    fn jsonw_name(w: *mut json_writer_t, name: *const c_char);
    fn jsonw_printf(w: *mut json_writer_t, fmt: *const c_char, ...);
    fn jsonw_uint_field(w: *mut json_writer_t, name: *const c_char, value: c_uint);
    fn jsonw_int_field(w: *mut json_writer_t, name: *const c_char, value: c_int);
    fn jsonw_string_field(w: *mut json_writer_t, name: *const c_char, value: *const c_char);
    fn jsonw_bool_field(w: *mut json_writer_t, name: *const c_char, value: bool);
    fn jsonw_null(w: *mut json_writer_t);
    fn jsonw_null_field(w: *mut json_writer_t, name: *const c_char);
    fn jsonw_string(w: *mut json_writer_t, value: *const c_char);

    fn print_hex_data_json(data: *const c_void, len: __u32);
    fn fprint_hex(stream: *mut c_void, data: *const c_void, len: __u32, sep: *const c_char);
    fn print_dev_json(ifindex: __u32, netns_dev: u64, netns_ino: u64);
    fn print_dev_plain(ifindex: __u32, netns_dev: u64, netns_ino: u64);
    fn emit_obj_refs_json(refs: *mut c_void, id: __u32, w: *mut json_writer_t);
    fn emit_obj_refs_plain(refs: *mut c_void, id: __u32, prefix: *const c_char);

    fn hashmap__new(hash: *mut c_void, equal: *mut c_void, ctx: *mut c_void) -> *mut hashmap;
    fn hashmap__empty(map: *mut hashmap) -> bool;
    fn hashmap__for_each_key_entry(map: *mut hashmap, entry: *mut *mut hashmap_entry, key: __u32) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    static mut hash_fn_for_key_as_id: *mut c_void;
    static mut equal_fn_for_key_as_id: *mut c_void;

    fn get_possible_cpus() -> c_uint;
    fn round_up(x: __u32, y: __u32) -> __u32;
    fn is_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
    fn p_err(fmt: *const c_char, ...);
    fn p_info(fmt: *const c_char, ...);
    fn usage() -> !;
    fn BAD_ARG() -> c_int;
    fn REQ_ARGS(n: c_int) -> bool;
    fn GET_ARG() -> *mut c_char;
    fn NEXT_ARG();
    fn parse_u32_arg(argc: *mut c_int, argv: *mut *mut *mut c_char, val: *mut __u32, what: *const c_char) -> c_int;
    fn set_max_rlimit();

    fn get_fdinfo(fd: c_int, key: *const c_char) -> *mut c_char;
    fn map_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char, open_flags: __u32) -> c_int;
    fn map_parse_fds(argc: *mut c_int, argv: *mut *mut *mut c_char, fds: *mut *mut c_int, open_flags: __u32) -> c_int;
    fn map_parse_fd_and_info(argc: *mut c_int, argv: *mut *mut *mut c_char, info: *mut bpf_map_info, len: *mut __u32, open_flags: __u32) -> c_int;
    fn prog_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int;
    fn do_pin_any(argc: c_int, argv: *mut *mut c_char, parse_fd: unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char) -> c_int) -> c_int;
    fn do_pin_fd(fd: c_int, name: *const c_char) -> c_int;
    fn do_event_pipe(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn cmd_select(cmds: *const cmd, argc: c_int, argv: *mut *mut c_char, help: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int) -> c_int;

    fn build_pinned_obj_table(table: *mut hashmap, obj_type: c_int);
    fn delete_pinned_obj_table(table: *mut hashmap);
    fn build_obj_refs_table(table: *mut *mut c_void, obj_type: c_int);
    fn delete_obj_refs_table(table: *mut c_void);

    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, len: *mut __u32) -> c_int;
    fn bpf_map_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_map_get_fd_by_id_opts(id: __u32, opts: *const bpf_get_fd_by_id_opts) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u32) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_lookup_and_delete_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_freeze(fd: c_int) -> c_int;
    fn bpf_map_create(t: bpf_map_type, name: *const c_char, key_size: __u32, value_size: __u32, max_entries: __u32, opts: *const bpf_map_create_opts) -> c_int;
}

static mut map_table: *mut hashmap = ptr::null_mut();
static mut btf_vmlinux: *mut btf = ptr::null_mut();

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn argv_at(argv: *mut *mut c_char, i: isize) -> *mut c_char {
    *argv.offset(i)
}

unsafe fn map_is_per_cpu(type_: __u32) -> bool {
    type_ == BPF_MAP_TYPE_PERCPU_HASH
        || type_ == BPF_MAP_TYPE_PERCPU_ARRAY
        || type_ == BPF_MAP_TYPE_LRU_PERCPU_HASH
        || type_ == BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE
}

unsafe fn map_is_map_of_maps(type_: __u32) -> bool {
    type_ == BPF_MAP_TYPE_ARRAY_OF_MAPS || type_ == BPF_MAP_TYPE_HASH_OF_MAPS
}

unsafe fn map_is_map_of_progs(type_: __u32) -> bool {
    type_ == BPF_MAP_TYPE_PROG_ARRAY
}

unsafe fn map_type_from_str(type_: *const c_char) -> c_int {
    let mut i: c_uint = 0;
    loop {
        let map_type_str = libbpf_bpf_map_type_str(i);
        if map_type_str.is_null() {
            break;
        }
        /* Don't allow prefixing in case of possible future shadowing */
        if strcmp(map_type_str, type_) == 0 {
            return i as c_int;
        }
        i += 1;
    }
    -1
}

unsafe fn alloc_value(info: *mut bpf_map_info) -> *mut c_void {
    if map_is_per_cpu((*info).type_) {
        malloc((round_up((*info).value_size, 8) * get_possible_cpus()) as usize)
    } else {
        malloc((*info).value_size as usize)
    }
}

unsafe fn do_dump_btf(d: *const btf_dumper, map_info: *mut bpf_map_info, key: *mut c_void, value: *mut c_void) -> c_int {
    let mut ret: c_int = 0;
    jsonw_start_object((*d).jw);
    if (*map_info).btf_key_type_id != 0 {
        jsonw_name((*d).jw, c!("key"));
        ret = btf_dumper_type(d, (*map_info).btf_key_type_id, key);
        if ret != 0 {
            jsonw_end_object((*d).jw);
            return ret;
        }
    }
    let value_id = if (*map_info).btf_vmlinux_value_type_id != 0 {
        (*map_info).btf_vmlinux_value_type_id
    } else {
        (*map_info).btf_value_type_id
    };
    if !map_is_per_cpu((*map_info).type_) {
        jsonw_name((*d).jw, c!("value"));
        ret = btf_dumper_type(d, value_id, value);
    } else {
        jsonw_name((*d).jw, c!("values"));
        jsonw_start_array((*d).jw);
        let n = get_possible_cpus();
        let step = round_up((*map_info).value_size, 8);
        let mut i = 0;
        while i < n {
            jsonw_start_object((*d).jw);
            jsonw_int_field((*d).jw, c!("cpu"), i as c_int);
            jsonw_name((*d).jw, c!("value"));
            ret = btf_dumper_type(d, value_id, (value as *mut u8).add((i * step) as usize) as *mut c_void);
            jsonw_end_object((*d).jw);
            if ret != 0 {
                break;
            }
            i += 1;
        }
        jsonw_end_array((*d).jw);
    }
    jsonw_end_object((*d).jw);
    ret
}

unsafe fn get_btf_writer() -> *mut json_writer_t {
    let jw = jsonw_new(stdout);
    if jw.is_null() {
        return ptr::null_mut();
    }
    jsonw_pretty(jw, true);
    jw
}

unsafe fn print_entry_json(info: *mut bpf_map_info, key: *mut u8, value: *mut u8, btfp: *mut btf) {
    jsonw_start_object(json_wtr);
    if !map_is_per_cpu((*info).type_) {
        jsonw_name(json_wtr, c!("key"));
        print_hex_data_json(key as *const c_void, (*info).key_size);
        jsonw_name(json_wtr, c!("value"));
        print_hex_data_json(value as *const c_void, (*info).value_size);
        if map_is_map_of_maps((*info).type_) {
            jsonw_uint_field(json_wtr, c!("inner_map_id"), *(value as *mut c_uint));
        }
        if !btfp.is_null() {
            let d = btf_dumper { btf: btfp, jw: json_wtr, is_plain_text: false };
            jsonw_name(json_wtr, c!("formatted"));
            do_dump_btf(&d, info, key as *mut c_void, value as *mut c_void);
        }
    } else {
        let n = get_possible_cpus();
        let step = round_up((*info).value_size, 8);
        jsonw_name(json_wtr, c!("key"));
        print_hex_data_json(key as *const c_void, (*info).key_size);
        jsonw_name(json_wtr, c!("values"));
        jsonw_start_array(json_wtr);
        let mut i = 0;
        while i < n {
            jsonw_start_object(json_wtr);
            jsonw_int_field(json_wtr, c!("cpu"), i as c_int);
            jsonw_name(json_wtr, c!("value"));
            print_hex_data_json(value.add((i * step) as usize) as *const c_void, (*info).value_size);
            jsonw_end_object(json_wtr);
            i += 1;
        }
        jsonw_end_array(json_wtr);
        if !btfp.is_null() {
            let d = btf_dumper { btf: btfp, jw: json_wtr, is_plain_text: false };
            jsonw_name(json_wtr, c!("formatted"));
            do_dump_btf(&d, info, key as *mut c_void, value as *mut c_void);
        }
    }
    jsonw_end_object(json_wtr);
}

unsafe fn print_entry_error_msg(info: *mut bpf_map_info, key: *mut u8, error_msg: *const c_char) {
    let msg_size = strlen(error_msg) as __u32;
    let break_names = (*info).key_size > 16 || msg_size > 16;
    let single_line = (*info).key_size + msg_size <= 24 && !break_names;
    printf(c!("key:%c"), if break_names { '\n' as c_int } else { ' ' as c_int });
    fprint_hex(stdout, key as *const c_void, (*info).key_size, c!(" "));
    printf(if single_line { c!("  ") } else { c!("\n") });
    printf(c!("value:%c%s"), if break_names { '\n' as c_int } else { ' ' as c_int }, error_msg);
    printf(c!("\n"));
}

unsafe fn print_entry_error(map_info: *mut bpf_map_info, key: *mut c_void, lookup_errno: c_int) {
    if (map_is_map_of_maps((*map_info).type_) || map_is_map_of_progs((*map_info).type_)) && lookup_errno == ENOENT {
        return;
    }
    if json_output {
        jsonw_start_object(json_wtr);
        jsonw_name(json_wtr, c!("key"));
        print_hex_data_json(key, (*map_info).key_size);
        jsonw_name(json_wtr, c!("value"));
        jsonw_start_object(json_wtr);
        jsonw_string_field(json_wtr, c!("error"), strerror(lookup_errno));
        jsonw_end_object(json_wtr);
        jsonw_end_object(json_wtr);
    } else {
        let mut msg: *const c_char = ptr::null();
        if lookup_errno == ENOENT {
            msg = c!("<no entry>");
        } else if lookup_errno == ENOSPC && (*map_info).type_ == BPF_MAP_TYPE_REUSEPORT_SOCKARRAY {
            msg = c!("<cannot read>");
        }
        print_entry_error_msg(map_info, key as *mut u8, if msg.is_null() { strerror(lookup_errno) } else { msg as *mut c_char });
    }
}

unsafe fn print_entry_plain(info: *mut bpf_map_info, key: *mut u8, value: *mut u8) {
    if !map_is_per_cpu((*info).type_) {
        let break_names = (*info).key_size > 16 || (*info).value_size > 16;
        let single_line = (*info).key_size + (*info).value_size <= 24 && !break_names;
        if (*info).key_size != 0 {
            printf(c!("key:%c"), if break_names { '\n' as c_int } else { ' ' as c_int });
            fprint_hex(stdout, key as *const c_void, (*info).key_size, c!(" "));
            printf(if single_line { c!("  ") } else { c!("\n") });
        }
        if (*info).value_size != 0 {
            if map_is_map_of_maps((*info).type_) {
                printf(c!("inner_map_id:%c"), if break_names { '\n' as c_int } else { ' ' as c_int });
                printf(c!("%u "), *(value as *mut c_uint));
            } else {
                printf(c!("value:%c"), if break_names { '\n' as c_int } else { ' ' as c_int });
                fprint_hex(stdout, value as *const c_void, (*info).value_size, c!(" "));
            }
        }
        printf(c!("\n"));
    } else {
        let n = get_possible_cpus();
        let step = round_up((*info).value_size, 8);
        if (*info).key_size != 0 {
            printf(c!("key:\n"));
            fprint_hex(stdout, key as *const c_void, (*info).key_size, c!(" "));
            printf(c!("\n"));
        }
        if (*info).value_size != 0 {
            let mut i = 0;
            while i < n {
                printf(c!("value (CPU %02u):%c"), i, if (*info).value_size > 16 { '\n' as c_int } else { ' ' as c_int });
                fprint_hex(stdout, value.add((i * step) as usize) as *const c_void, (*info).value_size, c!(" "));
                printf(c!("\n"));
                i += 1;
            }
        }
    }
}

unsafe fn parse_bytes(mut argv: *mut *mut c_char, name: *const c_char, val: *mut u8, n: c_uint) -> *mut *mut c_char {
    let mut i: c_uint = 0;
    let mut base: c_uint = 0;
    let mut endptr: *mut c_char = ptr::null_mut();
    if is_prefix(argv_at(argv, 0), c!("hex")) {
        base = 16;
        argv = argv.add(1);
    }
    while i < n && !argv_at(argv, i as isize).is_null() {
        *val.add(i as usize) = strtoul(argv_at(argv, i as isize), &mut endptr, base as c_int) as u8;
        if *endptr != 0 {
            p_err(c!("error parsing byte: %s"), argv_at(argv, i as isize));
            return ptr::null_mut();
        }
        i += 1;
    }
    if i != n {
        p_err(c!("%s expected %u bytes got %u"), name, n, i);
        return ptr::null_mut();
    }
    argv.add(i as usize)
}

/* on per cpu maps we must copy the provided value on all value instances */
unsafe fn fill_per_cpu_value(info: *mut bpf_map_info, value: *mut c_void) {
    if !map_is_per_cpu((*info).type_) {
        return;
    }
    let n = get_possible_cpus();
    let step = round_up((*info).value_size, 8);
    let mut i = 1;
    while i < n {
        memcpy((value as *mut u8).add((i * step) as usize) as *mut c_void, value, (*info).value_size as usize);
        i += 1;
    }
}

unsafe fn parse_elem(mut argv: *mut *mut c_char, info: *mut bpf_map_info, key: *mut c_void, value: *mut c_void, key_size: __u32, value_size: __u32, flags: *mut __u32, value_fd: *mut *mut __u32, open_flags: __u32) -> c_int {
    if argv_at(argv, 0).is_null() {
        if key.is_null() && value.is_null() {
            return 0;
        }
        p_err(c!("did not find %s"), if !key.is_null() { c!("key") } else { c!("value") });
        return -1;
    }
    if is_prefix(argv_at(argv, 0), c!("key")) {
        if key.is_null() {
            p_err(if key_size != 0 { c!("duplicate key") } else { c!("unnecessary key") });
            return -1;
        }
        argv = parse_bytes(argv.add(1), c!("key"), key as *mut u8, key_size);
        if argv.is_null() {
            return -1;
        }
        return parse_elem(argv, info, ptr::null_mut(), value, key_size, value_size, flags, value_fd, open_flags);
    } else if is_prefix(argv_at(argv, 0), c!("value")) {
        if value.is_null() {
            p_err(if value_size != 0 { c!("duplicate value") } else { c!("unnecessary value") });
            return -1;
        }
        argv = argv.add(1);
        if map_is_map_of_maps((*info).type_) {
            let mut argc = 2;
            if value_size != 4 {
                p_err(c!("value smaller than 4B for map in map?"));
                return -1;
            }
            if argv_at(argv, 0).is_null() || argv_at(argv, 1).is_null() {
                p_err(c!("not enough value arguments for map in map"));
                return -1;
            }
            let fd = map_parse_fd(&mut argc, &mut argv, open_flags);
            if fd < 0 {
                return -1;
            }
            *value_fd = value as *mut __u32;
            **value_fd = fd as __u32;
        } else if map_is_map_of_progs((*info).type_) {
            let mut argc = 2;
            if value_size != 4 {
                p_err(c!("value smaller than 4B for map of progs?"));
                return -1;
            }
            if argv_at(argv, 0).is_null() || argv_at(argv, 1).is_null() {
                p_err(c!("not enough value arguments for map of progs"));
                return -1;
            }
            if is_prefix(argv_at(argv, 0), c!("id")) {
                p_info(c!("Warning: updating program array via MAP_ID, make sure this map is kept open\n         by some process or pinned otherwise update will be lost"));
            }
            let fd = prog_parse_fd(&mut argc, &mut argv);
            if fd < 0 {
                return -1;
            }
            *value_fd = value as *mut __u32;
            **value_fd = fd as __u32;
        } else {
            argv = parse_bytes(argv, c!("value"), value as *mut u8, value_size);
            if argv.is_null() {
                return -1;
            }
            fill_per_cpu_value(info, value);
        }
        return parse_elem(argv, info, key, ptr::null_mut(), key_size, value_size, flags, ptr::null_mut(), open_flags);
    } else if is_prefix(argv_at(argv, 0), c!("any")) || is_prefix(argv_at(argv, 0), c!("noexist")) || is_prefix(argv_at(argv, 0), c!("exist")) {
        if flags.is_null() {
            p_err(c!("flags specified multiple times: %s"), argv_at(argv, 0));
            return -1;
        }
        if is_prefix(argv_at(argv, 0), c!("any")) {
            *flags = BPF_ANY;
        } else if is_prefix(argv_at(argv, 0), c!("noexist")) {
            *flags = BPF_NOEXIST;
        } else if is_prefix(argv_at(argv, 0), c!("exist")) {
            *flags = BPF_EXIST;
        }
        return parse_elem(argv.add(1), info, key, value, key_size, value_size, ptr::null_mut(), value_fd, open_flags);
    }
    p_err(c!("expected key or value, got: %s"), argv_at(argv, 0));
    -1
}

unsafe fn show_map_header_json(info: *mut bpf_map_info, wtr: *mut json_writer_t) {
    jsonw_uint_field(wtr, c!("id"), (*info).id);
    let map_type_str = libbpf_bpf_map_type_str((*info).type_);
    if !map_type_str.is_null() {
        jsonw_string_field(wtr, c!("type"), map_type_str);
    } else {
        jsonw_uint_field(wtr, c!("type"), (*info).type_);
    }
    if (*info).name[0] != 0 {
        jsonw_string_field(wtr, c!("name"), (*info).name.as_ptr());
    }
    jsonw_name(wtr, c!("flags"));
    jsonw_printf(wtr, c!("%u"), (*info).map_flags);
}

unsafe fn show_map_close_json(fd: c_int, info: *mut bpf_map_info) -> c_int {
    let memlock = get_fdinfo(fd, c!("memlock"));
    let frozen_str = get_fdinfo(fd, c!("frozen"));
    let mut frozen = 0;
    jsonw_start_object(json_wtr);
    show_map_header_json(info, json_wtr);
    print_dev_json((*info).ifindex, (*info).netns_dev, (*info).netns_ino);
    jsonw_uint_field(json_wtr, c!("bytes_key"), (*info).key_size);
    jsonw_uint_field(json_wtr, c!("bytes_value"), (*info).value_size);
    jsonw_uint_field(json_wtr, c!("max_entries"), (*info).max_entries);
    if !memlock.is_null() {
        jsonw_int_field(json_wtr, c!("bytes_memlock"), atoll(memlock) as c_int);
    }
    free(memlock as *mut c_void);
    if (*info).type_ == BPF_MAP_TYPE_PROG_ARRAY {
        let owner_prog_type = get_fdinfo(fd, c!("owner_prog_type"));
        let owner_jited = get_fdinfo(fd, c!("owner_jited"));
        if !owner_prog_type.is_null() {
            let prog_type = atoi(owner_prog_type) as c_uint;
            let prog_type_str = libbpf_bpf_prog_type_str(prog_type);
            if !prog_type_str.is_null() {
                jsonw_string_field(json_wtr, c!("owner_prog_type"), prog_type_str);
            } else {
                jsonw_uint_field(json_wtr, c!("owner_prog_type"), prog_type);
            }
        }
        if !owner_jited.is_null() {
            jsonw_bool_field(json_wtr, c!("owner_jited"), atoi(owner_jited) != 0);
        }
        free(owner_prog_type as *mut c_void);
        free(owner_jited as *mut c_void);
    }
    close(fd);
    if !frozen_str.is_null() {
        frozen = atoi(frozen_str);
        free(frozen_str as *mut c_void);
    }
    jsonw_int_field(json_wtr, c!("frozen"), frozen);
    if (*info).btf_id != 0 {
        jsonw_int_field(json_wtr, c!("btf_id"), (*info).btf_id as c_int);
    }
    if !hashmap__empty(map_table) {
        let mut entry: *mut hashmap_entry = ptr::null_mut();
        jsonw_name(json_wtr, c!("pinned"));
        jsonw_start_array(json_wtr);
        while hashmap__for_each_key_entry(map_table, &mut entry, (*info).id) != 0 {
            jsonw_string(json_wtr, (*entry).pvalue as *const c_char);
        }
        jsonw_end_array(json_wtr);
    }
    emit_obj_refs_json(refs_table, (*info).id, json_wtr);
    jsonw_end_object(json_wtr);
    0
}

unsafe fn show_map_header_plain(info: *mut bpf_map_info) {
    printf(c!("%u: "), (*info).id);
    let map_type_str = libbpf_bpf_map_type_str((*info).type_);
    if !map_type_str.is_null() {
        printf(c!("%s  "), map_type_str);
    } else {
        printf(c!("type %u  "), (*info).type_);
    }
    if (*info).name[0] != 0 {
        printf(c!("name %s  "), (*info).name.as_ptr());
    }
    printf(c!("flags 0x%x"), (*info).map_flags);
    print_dev_plain((*info).ifindex, (*info).netns_dev, (*info).netns_ino);
    printf(c!("\n"));
}

unsafe fn show_map_close_plain(fd: c_int, info: *mut bpf_map_info) -> c_int {
    let memlock = get_fdinfo(fd, c!("memlock"));
    let frozen_str = get_fdinfo(fd, c!("frozen"));
    let mut frozen = 0;
    show_map_header_plain(info);
    printf(c!("\tkey %uB  value %uB  max_entries %u"), (*info).key_size, (*info).value_size, (*info).max_entries);
    if !memlock.is_null() {
        printf(c!("  memlock %sB"), memlock);
    }
    free(memlock as *mut c_void);
    if (*info).type_ == BPF_MAP_TYPE_PROG_ARRAY {
        let owner_prog_type = get_fdinfo(fd, c!("owner_prog_type"));
        let owner_jited = get_fdinfo(fd, c!("owner_jited"));
        if !owner_prog_type.is_null() || !owner_jited.is_null() {
            printf(c!("\n\t"));
        }
        if !owner_prog_type.is_null() {
            let prog_type = atoi(owner_prog_type) as c_uint;
            let prog_type_str = libbpf_bpf_prog_type_str(prog_type);
            if !prog_type_str.is_null() {
                printf(c!("owner_prog_type %s  "), prog_type_str);
            } else {
                printf(c!("owner_prog_type %u  "), prog_type);
            }
        }
        if !owner_jited.is_null() {
            printf(c!("owner%s jited"), if atoi(owner_jited) != 0 { c!("") } else { c!(" not") });
        }
        free(owner_prog_type as *mut c_void);
        free(owner_jited as *mut c_void);
    }
    close(fd);
    if !hashmap__empty(map_table) {
        let mut entry: *mut hashmap_entry = ptr::null_mut();
        while hashmap__for_each_key_entry(map_table, &mut entry, (*info).id) != 0 {
            printf(c!("\n\tpinned %s"), (*entry).pvalue as *mut c_char);
        }
    }
    if !frozen_str.is_null() {
        frozen = atoi(frozen_str);
        free(frozen_str as *mut c_void);
    }
    if (*info).btf_id != 0 || frozen != 0 {
        printf(c!("\n\t"));
    }
    if (*info).btf_id != 0 {
        printf(c!("btf_id %u"), (*info).btf_id);
    }
    if frozen != 0 {
        printf(c!("%sfrozen"), if (*info).btf_id != 0 { c!("  ") } else { c!("") });
    }
    emit_obj_refs_plain(refs_table, (*info).id, c!("\n\tpids "));
    printf(c!("\n"));
    0
}

unsafe fn do_show_subset(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut info: bpf_map_info = core::mem::zeroed();
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
    let mut fds: *mut c_int = malloc(size_of::<c_int>()) as *mut c_int;
    let mut i = 0;
    let mut err = -1;
    if fds.is_null() {
        p_err(c!("mem alloc failed"));
        return -1;
    }
    let nb_fds = map_parse_fds(&mut argc, &mut argv, &mut fds, BPF_F_RDONLY);
    if nb_fds < 1 {
        free(fds as *mut c_void);
        return err;
    }
    if json_output && nb_fds > 1 {
        jsonw_start_array(json_wtr);
    }
    while i < nb_fds {
        err = bpf_map_get_info_by_fd(*fds.add(i as usize), &mut info, &mut len);
        if err != 0 {
            p_err(c!("can't get map info: %s"), strerror(errno));
            while i < nb_fds {
                close(*fds.add(i as usize));
                i += 1;
            }
            break;
        }
        if json_output { show_map_close_json(*fds.add(i as usize), &mut info); } else { show_map_close_plain(*fds.add(i as usize), &mut info); }
        i += 1;
    }
    if json_output && nb_fds > 1 {
        jsonw_end_array(json_wtr);
    }
    free(fds as *mut c_void);
    err
}

unsafe extern "C" fn do_show(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut opts = bpf_get_fd_by_id_opts { sz: size_of::<bpf_get_fd_by_id_opts>(), open_flags: BPF_F_RDONLY };
    let mut info: bpf_map_info = core::mem::zeroed();
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
    let mut id: __u32 = 0;
    let mut err: c_int;
    if show_pinned {
        map_table = hashmap__new(hash_fn_for_key_as_id, equal_fn_for_key_as_id, ptr::null_mut());
        if IS_ERR(map_table as *const c_void) {
            p_err(c!("failed to create hashmap for pinned paths"));
            return -1;
        }
        build_pinned_obj_table(map_table, BPF_OBJ_MAP);
    }
    build_obj_refs_table(&mut refs_table, BPF_OBJ_MAP);
    if argc == 2 {
        return do_show_subset(argc, argv);
    }
    if argc != 0 {
        return BAD_ARG();
    }
    if json_output { jsonw_start_array(json_wtr); }
    loop {
        err = bpf_map_get_next_id(id, &mut id);
        if err != 0 {
            if errno == ENOENT { break; }
            p_err(c!("can't get next map: %s%s"), strerror(errno), if errno == EINVAL { c!(" -- kernel too old?") } else { c!("") });
            break;
        }
        let fd = bpf_map_get_fd_by_id_opts(id, &opts);
        if fd < 0 {
            if errno == ENOENT { continue; }
            p_err(c!("can't get map by id (%u): %s"), id, strerror(errno));
            break;
        }
        err = bpf_map_get_info_by_fd(fd, &mut info, &mut len);
        if err != 0 {
            p_err(c!("can't get map info: %s"), strerror(errno));
            close(fd);
            break;
        }
        if json_output { show_map_close_json(fd, &mut info); } else { show_map_close_plain(fd, &mut info); }
    }
    if json_output { jsonw_end_array(json_wtr); }
    delete_obj_refs_table(refs_table);
    if show_pinned { delete_pinned_obj_table(map_table); }
    if errno == ENOENT { 0 } else { -1 }
}

unsafe fn dump_map_elem(fd: c_int, key: *mut c_void, value: *mut c_void, map_info: *mut bpf_map_info, btfp: *mut btf, btf_wtr: *mut json_writer_t) -> c_int {
    if bpf_map_lookup_elem(fd, key, value) != 0 {
        print_entry_error(map_info, key, errno);
        return -1;
    }
    if json_output {
        print_entry_json(map_info, key as *mut u8, value as *mut u8, btfp);
    } else if !btfp.is_null() {
        let d = btf_dumper { btf: btfp, jw: btf_wtr, is_plain_text: true };
        do_dump_btf(&d, map_info, key, value);
    } else {
        print_entry_plain(map_info, key as *mut u8, value as *mut u8);
    }
    0
}

unsafe fn maps_have_btf(fds: *mut c_int, nb_fds: c_int) -> c_int {
    let mut info: bpf_map_info = core::mem::zeroed();
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
    let mut i = 0;
    while i < nb_fds {
        if bpf_map_get_info_by_fd(*fds.add(i as usize), &mut info, &mut len) != 0 {
            p_err(c!("can't get map info: %s"), strerror(errno));
            return -1;
        }
        if info.btf_id == 0 {
            return 0;
        }
        i += 1;
    }
    1
}

unsafe fn free_btf_vmlinux() {
    btf__free(btf_vmlinux);
    btf_vmlinux = ptr::null_mut();
}

unsafe fn get_map_kv_btf(info: *const bpf_map_info, btf_out: *mut *mut btf) -> c_int {
    let mut err = 0;
    if (*info).btf_vmlinux_value_type_id != 0 {
        if btf_vmlinux.is_null() {
            btf_vmlinux = libbpf_find_kernel_btf();
            if btf_vmlinux.is_null() {
                p_err(c!("failed to get kernel btf"));
                return -errno;
            }
        }
        *btf_out = btf_vmlinux;
    } else if (*info).btf_value_type_id != 0 {
        *btf_out = btf__load_from_kernel_by_id((*info).btf_id);
        if (*btf_out).is_null() {
            err = -errno;
            p_err(c!("failed to get btf"));
        }
    } else {
        *btf_out = ptr::null_mut();
    }
    err
}

unsafe fn free_map_kv_btf(btfp: *mut btf) {
    if btfp != btf_vmlinux {
        btf__free(btfp);
    }
}

unsafe fn map_dump(fd: c_int, info: *mut bpf_map_info, wtr: *mut json_writer_t, show_header: bool) -> c_int {
    let key = malloc((*info).key_size as usize);
    let value = alloc_value(info);
    let mut prev_key: *mut c_void = ptr::null_mut();
    let mut btfp: *mut btf = ptr::null_mut();
    let mut num_elems: c_uint = 0;
    let mut err: c_int;
    if key.is_null() || value.is_null() {
        p_err(c!("mem alloc failed"));
        free(key);
        free(value);
        free_map_kv_btf(btfp);
        return -1;
    }
    if !wtr.is_null() {
        err = get_map_kv_btf(info, &mut btfp);
        if err != 0 { free(key); free(value); free_map_kv_btf(btfp); return err; }
        if show_header {
            jsonw_start_object(wtr);
            show_map_header_json(info, wtr);
            jsonw_name(wtr, c!("elements"));
        }
        jsonw_start_array(wtr);
    } else if show_header {
        show_map_header_plain(info);
    }
    if (*info).type_ == BPF_MAP_TYPE_REUSEPORT_SOCKARRAY && (*info).value_size != 8 {
        let map_type_str = libbpf_bpf_map_type_str((*info).type_);
        p_info(c!("Warning: cannot read values from %s map with value_size != 8"), map_type_str);
    }
    loop {
        err = bpf_map_get_next_key(fd, prev_key, key);
        if err != 0 {
            if errno == ENOENT { err = 0; }
            break;
        }
        if dump_map_elem(fd, key, value, info, btfp, wtr) == 0 {
            num_elems += 1;
        }
        prev_key = key;
    }
    if !wtr.is_null() {
        jsonw_end_array(wtr);
        if show_header { jsonw_end_object(wtr); }
    } else {
        printf(c!("Found %u element%s\n"), num_elems, if num_elems != 1 { c!("s") } else { c!("") });
    }
    free(key);
    free(value);
    free_map_kv_btf(btfp);
    err
}

unsafe extern "C" fn do_dump(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut wtr: *mut json_writer_t = ptr::null_mut();
    let mut btf_wtr: *mut json_writer_t = ptr::null_mut();
    let mut info: bpf_map_info = core::mem::zeroed();
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
    let mut fds: *mut c_int = ptr::null_mut();
    let mut i = 0;
    let mut err = -1;
    if argc != 2 { usage(); }
    fds = malloc(size_of::<c_int>()) as *mut c_int;
    if fds.is_null() { p_err(c!("mem alloc failed")); return -1; }
    let nb_fds = map_parse_fds(&mut argc, &mut argv, &mut fds, BPF_F_RDONLY);
    if nb_fds < 1 { free(fds as *mut c_void); free_btf_vmlinux(); return err; }
    if json_output { wtr = json_wtr; } else {
        let do_plain_btf = maps_have_btf(fds, nb_fds);
        if do_plain_btf < 0 { while i < nb_fds { close(*fds.add(i as usize)); i += 1; } free(fds as *mut c_void); free_btf_vmlinux(); return err; }
        if do_plain_btf != 0 {
            btf_wtr = get_btf_writer();
            wtr = btf_wtr;
            if btf_wtr.is_null() { p_info(c!("failed to create json writer for btf. falling back to plain output")); }
        }
    }
    if !wtr.is_null() && nb_fds > 1 { jsonw_start_array(wtr); }
    while i < nb_fds {
        if bpf_map_get_info_by_fd(*fds.add(i as usize), &mut info, &mut len) != 0 {
            p_err(c!("can't get map info: %s"), strerror(errno));
            err = -1;
            break;
        }
        err = map_dump(*fds.add(i as usize), &mut info, wtr, nb_fds > 1);
        if wtr.is_null() && i != nb_fds - 1 { printf(c!("\n")); }
        if err != 0 { break; }
        close(*fds.add(i as usize));
        i += 1;
    }
    if !wtr.is_null() && nb_fds > 1 { jsonw_end_array(wtr); }
    if !btf_wtr.is_null() { jsonw_destroy(&mut btf_wtr); }
    while i < nb_fds { close(*fds.add(i as usize)); i += 1; }
    free(fds as *mut c_void);
    free_btf_vmlinux();
    err
}

unsafe fn alloc_key_value(info: *mut bpf_map_info, key: *mut *mut c_void, value: *mut *mut c_void) -> c_int {
    *key = ptr::null_mut();
    *value = ptr::null_mut();
    if (*info).key_size != 0 {
        *key = malloc((*info).key_size as usize);
        if (*key).is_null() { p_err(c!("key mem alloc failed")); return -1; }
    }
    if (*info).value_size != 0 {
        *value = alloc_value(info);
        if (*value).is_null() {
            p_err(c!("value mem alloc failed"));
            free(*key);
            *key = ptr::null_mut();
            return -1;
        }
    }
    0
}

unsafe extern "C" fn do_update(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut info: bpf_map_info = core::mem::zeroed();
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
    let mut value_fd: *mut __u32 = ptr::null_mut();
    let mut flags = BPF_ANY;
    let mut key: *mut c_void = ptr::null_mut();
    let mut value: *mut c_void = ptr::null_mut();
    if argc < 2 { usage(); }
    let fd = map_parse_fd_and_info(&mut argc, &mut argv, &mut info, &mut len, 0);
    if fd < 0 { return -1; }
    let mut err = alloc_key_value(&mut info, &mut key, &mut value);
    if err == 0 { err = parse_elem(argv, &mut info, key, value, info.key_size, info.value_size, &mut flags, &mut value_fd, 0); }
    if err == 0 {
        err = bpf_map_update_elem(fd, key, value, flags);
        if err != 0 { p_err(c!("update failed: %s"), strerror(errno)); }
    }
    if !value_fd.is_null() { close(*value_fd as c_int); }
    free(key);
    free(value);
    close(fd);
    if err == 0 && json_output { jsonw_null(json_wtr); }
    err
}

unsafe fn print_key_value(info: *mut bpf_map_info, key: *mut c_void, value: *mut c_void) {
    let mut btfp: *mut btf = ptr::null_mut();
    if get_map_kv_btf(info, &mut btfp) != 0 { return; }
    if json_output {
        print_entry_json(info, key as *mut u8, value as *mut u8, btfp);
    } else if !btfp.is_null() {
        let mut btf_wtr = get_btf_writer();
        if btf_wtr.is_null() {
            p_info(c!("failed to create json writer for btf. falling back to plain output"));
            free_map_kv_btf(btfp);
            btfp = ptr::null_mut();
            print_entry_plain(info, key as *mut u8, value as *mut u8);
        } else {
            let d = btf_dumper { btf: btfp, jw: btf_wtr, is_plain_text: true };
            do_dump_btf(&d, info, key, value);
            jsonw_destroy(&mut btf_wtr);
        }
    } else {
        print_entry_plain(info, key as *mut u8, value as *mut u8);
    }
    free_map_kv_btf(btfp);
}

unsafe extern "C" fn do_lookup(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut info: bpf_map_info = core::mem::zeroed();
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
    let mut key: *mut c_void = ptr::null_mut();
    let mut value: *mut c_void = ptr::null_mut();
    if argc < 2 { usage(); }
    let fd = map_parse_fd_and_info(&mut argc, &mut argv, &mut info, &mut len, BPF_F_RDONLY);
    if fd < 0 { return -1; }
    let mut err = alloc_key_value(&mut info, &mut key, &mut value);
    if err == 0 { err = parse_elem(argv, &mut info, key, ptr::null_mut(), info.key_size, 0, ptr::null_mut(), ptr::null_mut(), BPF_F_RDONLY); }
    if err == 0 {
        err = bpf_map_lookup_elem(fd, key, value);
        if err != 0 {
            if errno == ENOENT {
                if json_output { jsonw_null(json_wtr); } else {
                    printf(c!("key:\n")); fprint_hex(stdout, key, info.key_size, c!(" ")); printf(c!("\n\nNot found\n"));
                }
            } else { p_err(c!("lookup failed: %s"), strerror(errno)); }
        } else {
            print_key_value(&mut info, key, value);
        }
    }
    free(key); free(value); close(fd); err
}

unsafe extern "C" fn do_getnext(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut info: bpf_map_info = core::mem::zeroed();
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
    if argc < 2 { usage(); }
    let fd = map_parse_fd_and_info(&mut argc, &mut argv, &mut info, &mut len, BPF_F_RDONLY);
    if fd < 0 { return -1; }
    let mut key = malloc(info.key_size as usize);
    let nextkey = malloc(info.key_size as usize);
    let mut err;
    if key.is_null() || nextkey.is_null() { p_err(c!("mem alloc failed")); err = -1; }
    else {
        if argc != 0 {
            err = parse_elem(argv, &mut info, key, ptr::null_mut(), info.key_size, 0, ptr::null_mut(), ptr::null_mut(), BPF_F_RDONLY);
            if err != 0 { free(nextkey); free(key); close(fd); return err; }
        } else { free(key); key = ptr::null_mut(); }
        err = bpf_map_get_next_key(fd, key, nextkey);
        if err != 0 { p_err(c!("can't get next key: %s"), strerror(errno)); }
        else if json_output {
            jsonw_start_object(json_wtr);
            if !key.is_null() { jsonw_name(json_wtr, c!("key")); print_hex_data_json(key, info.key_size); } else { jsonw_null_field(json_wtr, c!("key")); }
            jsonw_name(json_wtr, c!("next_key")); print_hex_data_json(nextkey, info.key_size); jsonw_end_object(json_wtr);
        } else {
            if !key.is_null() { printf(c!("key:\n")); fprint_hex(stdout, key, info.key_size, c!(" ")); printf(c!("\n")); } else { printf(c!("key: None\n")); }
            printf(c!("next key:\n")); fprint_hex(stdout, nextkey, info.key_size, c!(" ")); printf(c!("\n"));
        }
    }
    free(nextkey); free(key); close(fd); err
}

unsafe extern "C" fn do_delete(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut info: bpf_map_info = core::mem::zeroed();
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
    if argc < 2 { usage(); }
    let fd = map_parse_fd_and_info(&mut argc, &mut argv, &mut info, &mut len, 0);
    if fd < 0 { return -1; }
    let key = malloc(info.key_size as usize);
    let mut err;
    if key.is_null() { p_err(c!("mem alloc failed")); err = -1; }
    else {
        err = parse_elem(argv, &mut info, key, ptr::null_mut(), info.key_size, 0, ptr::null_mut(), ptr::null_mut(), 0);
        if err == 0 {
            err = bpf_map_delete_elem(fd, key);
            if err != 0 { p_err(c!("delete failed: %s"), strerror(errno)); }
        }
    }
    free(key); close(fd);
    if err == 0 && json_output { jsonw_null(json_wtr); }
    err
}

unsafe extern "C" fn map_parse_read_only_fd(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int {
    map_parse_fd(argc, argv, BPF_F_RDONLY)
}

unsafe extern "C" fn do_pin(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let err = do_pin_any(argc, argv, map_parse_read_only_fd);
    if err == 0 && json_output { jsonw_null(json_wtr); }
    err
}

unsafe extern "C" fn do_create(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut attr: bpf_map_create_opts = core::mem::zeroed();
    attr.sz = size_of::<bpf_map_create_opts>();
    let mut map_type: bpf_map_type = BPF_MAP_TYPE_UNSPEC;
    let mut key_size: __u32 = 0;
    let mut value_size: __u32 = 0;
    let mut max_entries: __u32 = 0;
    let mut map_name: *const c_char = ptr::null();
    let mut err = -1;
    if !REQ_ARGS(7) { return -1; }
    let pinfile = GET_ARG();
    while argc != 0 {
        if !REQ_ARGS(2) { return -1; }
        if is_prefix(argv_at(argv, 0), c!("type")) {
            NEXT_ARG();
            if map_type != 0 { p_err(c!("map type already specified")); break; }
            map_type = map_type_from_str(argv_at(argv, 0)) as bpf_map_type;
            if (map_type as c_int) < 0 { p_err(c!("unrecognized map type: %s"), argv_at(argv, 0)); break; }
            NEXT_ARG();
        } else if is_prefix(argv_at(argv, 0), c!("name")) {
            NEXT_ARG();
            map_name = GET_ARG();
            if strlen(map_name) > BPF_OBJ_NAME_LEN - 1 {
                p_info(c!("Warning: map name is longer than %u characters, it will be truncated."), (BPF_OBJ_NAME_LEN - 1) as c_uint);
            }
        } else if is_prefix(argv_at(argv, 0), c!("key")) {
            if parse_u32_arg(&mut argc, &mut argv, &mut key_size, c!("key size")) != 0 { break; }
        } else if is_prefix(argv_at(argv, 0), c!("value")) {
            if parse_u32_arg(&mut argc, &mut argv, &mut value_size, c!("value size")) != 0 { break; }
        } else if is_prefix(argv_at(argv, 0), c!("entries")) {
            if parse_u32_arg(&mut argc, &mut argv, &mut max_entries, c!("max entries")) != 0 { break; }
        } else if is_prefix(argv_at(argv, 0), c!("flags")) {
            if parse_u32_arg(&mut argc, &mut argv, &mut attr.map_flags, c!("flags")) != 0 { break; }
        } else if is_prefix(argv_at(argv, 0), c!("dev")) || is_prefix(argv_at(argv, 0), c!("offload_dev")) {
            if is_prefix(argv_at(argv, 0), c!("dev")) {
                p_info(c!("Warning: 'bpftool map create [...] dev <ifname>' syntax is deprecated.\nGoing further, please use 'offload_dev <ifname>' to request hardware offload for the map."));
            }
            NEXT_ARG();
            if attr.map_ifindex != 0 { p_err(c!("offload device already specified")); break; }
            attr.map_ifindex = if_nametoindex(argv_at(argv, 0));
            if attr.map_ifindex == 0 { p_err(c!("unrecognized netdevice '%s': %s"), argv_at(argv, 0), strerror(errno)); break; }
            NEXT_ARG();
        } else if is_prefix(argv_at(argv, 0), c!("inner_map")) {
            let mut info: bpf_map_info = core::mem::zeroed();
            let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
            NEXT_ARG();
            if !REQ_ARGS(2) { usage(); }
            let inner_map_fd = map_parse_fd_and_info(&mut argc, &mut argv, &mut info, &mut len, BPF_F_RDONLY);
            if inner_map_fd < 0 { return -1; }
            attr.inner_map_fd = inner_map_fd;
        } else {
            p_err(c!("unknown arg %s"), argv_at(argv, 0));
            break;
        }
    }
    if map_name.is_null() { p_err(c!("map name not specified")); }
    else {
        set_max_rlimit();
        let fd = bpf_map_create(map_type, map_name, key_size, value_size, max_entries, &attr);
        if fd < 0 { p_err(c!("map create failed: %s"), strerror(errno)); }
        else {
            err = do_pin_fd(fd, pinfile);
            close(fd);
            if err == 0 && json_output { jsonw_null(json_wtr); }
        }
    }
    if attr.inner_map_fd > 0 { close(attr.inner_map_fd); }
    err
}

unsafe extern "C" fn do_pop_dequeue(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut info: bpf_map_info = core::mem::zeroed();
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;
    let mut key: *mut c_void = ptr::null_mut();
    let mut value: *mut c_void = ptr::null_mut();
    if argc < 2 { usage(); }
    let fd = map_parse_fd_and_info(&mut argc, &mut argv, &mut info, &mut len, 0);
    if fd < 0 { return -1; }
    let mut err = alloc_key_value(&mut info, &mut key, &mut value);
    if err == 0 {
        err = bpf_map_lookup_and_delete_elem(fd, key, value);
        if err != 0 {
            if errno == ENOENT {
                if json_output { jsonw_null(json_wtr); } else { printf(c!("Error: empty map\n")); }
            } else { p_err(c!("pop failed: %s"), strerror(errno)); }
        } else { print_key_value(&mut info, key, value); }
    }
    free(key); free(value); close(fd); err
}

unsafe extern "C" fn do_freeze(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    if !REQ_ARGS(2) { return -1; }
    let fd = map_parse_fd(&mut argc, &mut argv, 0);
    if fd < 0 { return -1; }
    if argc != 0 { close(fd); return BAD_ARG(); }
    let err = bpf_map_freeze(fd);
    close(fd);
    if err != 0 {
        p_err(c!("failed to freeze map: %s"), strerror(errno));
        return err;
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
        c!("Usage: %1$s %2$s { show | list }   [MAP]\n       %1$s %2$s create     FILE type TYPE key KEY_SIZE value VALUE_SIZE \\\n                                  entries MAX_ENTRIES name NAME [flags FLAGS] \\\n                                  [inner_map MAP] [offload_dev NAME]\n       %1$s %2$s dump       MAP\n       %1$s %2$s update     MAP [key DATA] [value VALUE] [UPDATE_FLAGS]\n       %1$s %2$s lookup     MAP [key DATA]\n       %1$s %2$s getnext    MAP [key DATA]\n       %1$s %2$s delete     MAP  key DATA\n       %1$s %2$s pin        MAP  FILE\n       %1$s %2$s event_pipe MAP [cpu N index M]\n       %1$s %2$s peek       MAP\n       %1$s %2$s push       MAP value VALUE\n       %1$s %2$s pop        MAP\n       %1$s %2$s enqueue    MAP value VALUE\n       %1$s %2$s dequeue    MAP\n       %1$s %2$s freeze     MAP\n       %1$s %2$s help\n\n       HELP_SPEC_MAP\n       DATA := { [hex] BYTES }\n       HELP_SPEC_PROGRAM\n       VALUE := { DATA | MAP | PROG }\n       UPDATE_FLAGS := { any | exist | noexist }\n       TYPE := { hash | array | prog_array | perf_event_array | percpu_hash |\n                 percpu_array | stack_trace | cgroup_array | lru_hash |\n                 lru_percpu_hash | lpm_trie | array_of_maps | hash_of_maps |\n                 devmap | devmap_hash | sockmap | cpumap | xskmap | sockhash |\n                 cgroup_storage | reuseport_sockarray | percpu_cgroup_storage |\n                 queue | stack | sk_storage | struct_ops | ringbuf | inode_storage |\n                 task_storage | bloom_filter | user_ringbuf | cgrp_storage | arena |\n                 insn_array | rhash }\n       HELP_SPEC_OPTIONS |\n                    {-f|--bpffs} | {-n|--nomount} }\n"),
        bin_name,
        argv_at(argv, -2),
    );
    0
}

static cmds: [cmd; 17] = [
    cmd { cmd: c!("show"), func: Some(do_show) },
    cmd { cmd: c!("list"), func: Some(do_show) },
    cmd { cmd: c!("help"), func: Some(do_help) },
    cmd { cmd: c!("dump"), func: Some(do_dump) },
    cmd { cmd: c!("update"), func: Some(do_update) },
    cmd { cmd: c!("lookup"), func: Some(do_lookup) },
    cmd { cmd: c!("getnext"), func: Some(do_getnext) },
    cmd { cmd: c!("delete"), func: Some(do_delete) },
    cmd { cmd: c!("pin"), func: Some(do_pin) },
    cmd { cmd: c!("event_pipe"), func: Some(do_event_pipe) },
    cmd { cmd: c!("create"), func: Some(do_create) },
    cmd { cmd: c!("peek"), func: Some(do_lookup) },
    cmd { cmd: c!("push"), func: Some(do_update) },
    cmd { cmd: c!("enqueue"), func: Some(do_update) },
    cmd { cmd: c!("pop"), func: Some(do_pop_dequeue) },
    cmd { cmd: c!("dequeue"), func: Some(do_pop_dequeue) },
    cmd { cmd: ptr::null(), func: None },
];

#[no_mangle]
pub unsafe extern "C" fn do_map(argc: c_int, argv: *mut *mut c_char) -> c_int {
    cmd_select(cmds.as_ptr(), argc, argv, do_help)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
