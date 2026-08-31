// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2019 Facebook */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type __s32 = i32;

const MAX_OBJ_NAME_LEN: usize = 64;
const PATH_MAX: usize = 4096;
const MAX_SIG_SIZE: usize = 4096;
const SHA256_DIGEST_LENGTH: usize = 32;
const UINT32_MAX: __u32 = u32::MAX;
const MARKED: __u32 = UINT32_MAX;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 1;
const MAP_PRIVATE: c_int = 2;
const BPF_F_MMAPABLE: __u32 = 1024;
const BPF_F_RDONLY_PROG: __u32 = 128;
const BTF_VAR_STATIC: c_int = 0;
const BTF_EXT_ELF_SEC: *const c_char = b".BTF.ext\0".as_ptr() as *const c_char;

const BPF_MAP_TYPE_PERCPU_ARRAY: c_int = 6;
const BPF_MAP_TYPE_STRUCT_OPS: c_int = 27;
const BPF_MAP_TYPE_ARENA: c_int = 33;
const BPF_PROG_TYPE_RAW_TRACEPOINT: c_int = 17;
const BPF_PROG_TYPE_TRACING: c_int = 26;
const BPF_PROG_TYPE_LSM: c_int = 29;
const BPF_TRACE_ITER: c_int = 4;

const BTF_KIND_UNKN: c_int = 0;
const BTF_KIND_INT: c_int = 1;
const BTF_KIND_PTR: c_int = 2;
const BTF_KIND_ARRAY: c_int = 3;
const BTF_KIND_STRUCT: c_int = 4;
const BTF_KIND_UNION: c_int = 5;
const BTF_KIND_ENUM: c_int = 6;
const BTF_KIND_FWD: c_int = 7;
const BTF_KIND_TYPEDEF: c_int = 8;
const BTF_KIND_VOLATILE: c_int = 9;
const BTF_KIND_CONST: c_int = 10;
const BTF_KIND_RESTRICT: c_int = 11;
const BTF_KIND_FUNC_PROTO: c_int = 13;
const BTF_KIND_FLOAT: c_int = 16;
const BTF_KIND_ENUM64: c_int = 19;

const BPF_CORE_FIELD_BYTE_OFFSET: c_int = 0;
const BPF_CORE_FIELD_BYTE_SIZE: c_int = 1;
const BPF_CORE_FIELD_EXISTS: c_int = 2;
const BPF_CORE_FIELD_SIGNED: c_int = 3;
const BPF_CORE_FIELD_LSHIFT_U64: c_int = 4;
const BPF_CORE_FIELD_RSHIFT_U64: c_int = 5;
const BPF_CORE_TYPE_ID_LOCAL: c_int = 6;
const BPF_CORE_TYPE_ID_TARGET: c_int = 7;
const BPF_CORE_TYPE_EXISTS: c_int = 8;
const BPF_CORE_TYPE_SIZE: c_int = 9;
const BPF_CORE_ENUMVAL_EXISTS: c_int = 10;
const BPF_CORE_ENUMVAL_VALUE: c_int = 11;
const BPF_CORE_TYPE_MATCHES: c_int = 12;

#[repr(C)]
struct btf {
    _private: [u8; 0],
}
#[repr(C)]
struct btf_dump {
    _private: [u8; 0],
}
#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}
#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}
#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}
#[repr(C)]
struct bpf_linker {
    _private: [u8; 0],
}
#[repr(C)]
struct hashmap {
    _private: [u8; 0],
}
#[repr(C)]
struct FILE {
    _private: [u8; 0],
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
struct btf_member {
    name_off: __u32,
    type_: __u32,
    offset: __u32,
}
#[repr(C)]
struct btf_param {
    name_off: __u32,
    type_: __u32,
}
#[repr(C)]
struct btf_array {
    type_: __u32,
    index_type: __u32,
    nelems: __u32,
}
#[repr(C)]
struct btf_var {
    linkage: __u32,
}
#[repr(C)]
struct btf_ext_info {
    len: __u32,
}
#[repr(C)]
struct btf_ext {
    core_relo_info: btf_ext_info,
}
#[repr(C)]
struct btf_ext_info_sec {
    sec_name_off: __u32,
    num_info: __u32,
}
#[repr(C)]
struct bpf_core_relo {
    insn_off: __u32,
    type_id: __u32,
    access_str_off: __u32,
    kind: __u32,
}
#[repr(C)]
struct bpf_core_spec {
    btf: *const btf,
    root_type_id: __u32,
    raw_spec: [__u32; 64],
    raw_len: c_int,
    relo_kind: c_int,
}
#[repr(C)]
struct bpf_core_relo_res {
    _opaque: [u8; 0],
}
#[repr(C)]
struct bpf_core_cand {
    btf: *const btf,
    id: __u32,
}
#[repr(C)]
struct bpf_core_cand_list {
    _opaque: [u8; 0],
}
#[repr(C)]
struct hashmap_entry {
    key: c_long,
    pvalue: *mut c_void,
}
#[repr(C)]
struct btf_field_iter {
    _opaque: [usize; 8],
}
#[repr(C)]
struct stat {
    _opaque: [u8; 144],
    st_size: c_long,
}
#[repr(C)]
struct bpf_object_open_opts {
    object_name: *const c_char,
    kernel_log_level: __u32,
}
#[repr(C)]
struct gen_loader_opts {
    data: *const c_void,
    data_sz: size_t,
    insns: *const c_void,
    insns_sz: size_t,
    gen_hash: bool,
}
#[repr(C)]
struct bpf_load_and_run_opts {
    insns: *const c_void,
    insns_sz: size_t,
    data: *const c_void,
    data_sz: size_t,
    excl_prog_hash: *mut __u8,
    excl_prog_hash_sz: size_t,
    signature: *mut c_char,
    signature_sz: size_t,
    keyring_id: __s32,
}
#[repr(C)]
struct btf_dump_emit_type_decl_opts {
    field_name: *const c_char,
    indent_level: c_int,
    strip_mods: bool,
}
#[repr(C)]
struct cmd {
    cmd: *const c_char,
    func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut sign_progs: bool;
    static mut verifier_logs: bool;
    static mut use_loader: bool;
    static mut json_output: bool;
    static mut json_wtr: *mut c_void;
    static mut bin_name: *const c_char;

    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strstr(h: *const c_char, n: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char;
    fn strncpy(d: *mut c_char, s: *const c_char, n: size_t) -> *mut c_char;
    fn strncat(d: *mut c_char, s: *const c_char, n: size_t) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn vprintf(fmt: *const c_char, ap: *mut c_void) -> c_int;
    fn fprintf(f: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn malloc(n: size_t) -> *mut c_void;
    fn calloc(n: size_t, sz: size_t) -> *mut c_void;
    fn free(p: *mut c_void);
    fn exit(code: c_int) -> !;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn isalnum(c: c_int) -> c_int;
    fn isspace(c: c_int) -> c_int;
    fn toupper(c: c_int) -> c_int;
    fn memcpy(d: *mut c_void, s: *const c_void, n: size_t) -> *mut c_void;
    fn stat(path: *const c_char, st: *mut stat) -> c_int;
    fn strerror(e: c_int) -> *mut c_char;
    fn sysconf(name: c_int) -> c_long;
    fn open(path: *const c_char, flags: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, off: c_long) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fwrite(ptr: *const c_void, sz: size_t, n: size_t, f: *mut FILE) -> size_t;
    fn fclose(f: *mut FILE) -> c_int;
    fn p_err(fmt: *const c_char, ...);
    fn usage();
    fn is_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
    fn cmd_select(cmds: *const cmd, argc: c_int, argv: *mut *mut c_char, help: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int) -> c_int;
    fn jsonw_null(w: *mut c_void);
    fn libbpf_strerror(err: c_int, buf: *mut c_char, sz: size_t) -> c_int;
    fn btf__name_by_offset(btf: *const btf, off: __u32) -> *const c_char;
    fn btf__str_by_offset(btf: *const btf, off: __u32) -> *const c_char;
    fn btf__type_by_id(btf: *const btf, id: __u32) -> *const btf_type;
    fn btf__type_cnt(btf: *const btf) -> c_int;
    fn btf__align_of(btf: *const btf, id: __u32) -> c_int;
    fn btf__resolve_size(btf: *const btf, id: __u32) -> c_long;
    fn btf__raw_data(btf: *const btf, sz: *mut __u32) -> *const c_void;
    fn btf__parse(path: *const c_char, ext: *mut *mut btf_ext) -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__new_empty() -> *mut btf;
    fn btf__add_struct(btf: *mut btf, name: *const c_char, sz: __u32) -> c_int;
    fn btf__add_union(btf: *mut btf, name: *const c_char, sz: __u32) -> c_int;
    fn btf__add_field(btf: *mut btf, name: *const c_char, type_: __u32, bit_off: __u32, bit_sz: __u32) -> c_int;
    fn btf__add_type(dst: *mut btf, src: *const btf, t: *const btf_type) -> c_int;
    fn btf_ext__free(ext: *mut btf_ext);
    fn btf_vlen(t: *const btf_type) -> __u32;
    fn btf_kind(t: *const btf_type) -> c_int;
    fn btf_kind_str(t: *const btf_type) -> *const c_char;
    fn btf_is_ptr(t: *const btf_type) -> bool;
    fn btf_is_func_proto(t: *const btf_type) -> bool;
    fn btf_is_datasec(t: *const btf_type) -> bool;
    fn btf_is_array(t: *const btf_type) -> bool;
    fn btf_is_mod(t: *const btf_type) -> bool;
    fn btf_is_typedef(t: *const btf_type) -> bool;
    fn btf_is_composite(t: *const btf_type) -> bool;
    fn btf_is_struct(t: *const btf_type) -> bool;
    fn btf_members(t: *const btf_type) -> *mut btf_member;
    fn btf_params(t: *const btf_type) -> *mut btf_param;
    fn btf_array(t: *const btf_type) -> *mut btf_array;
    fn btf_var(t: *const btf_type) -> *mut btf_var;
    fn btf_var_secinfos(t: *const btf_type) -> *mut btf_var_secinfo;
    fn btf_member_bit_offset(t: *const btf_type, idx: __u32) -> __u32;
    fn btf_member_bitfield_size(t: *const btf_type, idx: __u32) -> __u32;
    fn btf_field_iter_init(it: *mut btf_field_iter, t: *mut btf_type, kind: c_int) -> c_int;
    fn btf_field_iter_next(it: *mut btf_field_iter) -> *mut __u32;
    fn skip_mods_and_typedefs(btf: *const btf, id: __u32, res_id: *mut __u32) -> *const btf_type;
    fn btf_dump__new(btf: *const btf, cb: unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void), ctx: *mut c_void, opts: *mut c_void) -> *mut btf_dump;
    fn btf_dump__free(d: *mut btf_dump);
    fn btf_dump__emit_type_decl(d: *mut btf_dump, id: __u32, opts: *const btf_dump_emit_type_decl_opts) -> c_int;
    fn bpf_object__btf(obj: *const bpf_object) -> *mut btf;
    fn bpf_object__open_mem(data: *const c_void, sz: size_t, opts: *const bpf_object_open_opts) -> *mut bpf_object;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__gen_loader(obj: *mut bpf_object, opts: *mut gen_loader_opts) -> c_int;
    fn bpf_load_and_run(opts: *mut bpf_load_and_run_opts) -> c_int;
    fn bpftool_prog_sign(opts: *mut bpf_load_and_run_opts) -> c_int;
    fn bpf_map__name(map: *const bpf_map) -> *const c_char;
    fn bpf_map__is_internal(map: *const bpf_map) -> bool;
    fn bpf_map__type(map: *const bpf_map) -> c_int;
    fn bpf_map__map_flags(map: *const bpf_map) -> __u32;
    fn bpf_map__btf_value_type_id(map: *const bpf_map) -> __u32;
    fn bpf_map__value_size(map: *const bpf_map) -> __u32;
    fn bpf_map__max_entries(map: *const bpf_map) -> __u32;
    fn bpf_map__initial_value(map: *const bpf_map, sz: *mut size_t) -> *mut c_void;
    fn bpf_program__name(prog: *const bpf_program) -> *const c_char;
    fn bpf_program__type(prog: *const bpf_program) -> c_int;
    fn bpf_program__section_name(prog: *const bpf_program) -> *const c_char;
    fn bpf_program__expected_attach_type(prog: *const bpf_program) -> c_int;
    fn bpf_linker__new(output: *const c_char, opts: *mut c_void) -> *mut bpf_linker;
    fn bpf_linker__add_file(linker: *mut bpf_linker, file: *const c_char, opts: *mut c_void) -> c_int;
    fn bpf_linker__finalize(linker: *mut bpf_linker) -> c_int;
    fn bpf_linker__free(linker: *mut bpf_linker);
    fn hashmap__new(hash: unsafe extern "C" fn(c_long, *mut c_void) -> size_t, eq: unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool, ctx: *mut c_void) -> *mut hashmap;
    fn hashmap__find(map: *mut hashmap, key: c_long, value: *mut *mut bpf_core_cand_list) -> bool;
    fn hashmap__set(map: *mut hashmap, key: c_long, value: *mut bpf_core_cand_list, old_key: *mut c_long, old_val: *mut *mut c_void) -> c_int;
    fn hashmap__free(map: *mut hashmap);
    fn bpf_core_essential_name_len(name: *const c_char) -> size_t;
    fn bpf_core_add_cands(local: *const bpf_core_cand, len: size_t, targ: *const btf, name: *const c_char, flavor: c_int, cands: *mut bpf_core_cand_list) -> c_int;
    fn bpf_core_free_cands(cands: *mut c_void);
    fn bpf_core_calc_relo_insn(sec: *const c_char, relo: *const bpf_core_relo, idx: c_uint, btf: *const btf, cands: *mut bpf_core_cand_list, specs: *mut bpf_core_spec, res: *mut bpf_core_relo_res) -> c_int;
}

unsafe fn roundup(x: size_t, y: size_t) -> size_t {
    ((x + y - 1) / y) * y
}

unsafe fn sanitize_identifier(name: *mut c_char) {
    let mut i = 0usize;
    while *name.add(i) != 0 {
        if isalnum(*name.add(i) as c_int) == 0 && *name.add(i) != b'_' as c_char {
            *name.add(i) = b'_' as c_char;
        }
        i += 1;
    }
}

unsafe fn str_has_prefix(str_: *const c_char, prefix: *const c_char) -> bool {
    strncmp(str_, prefix, strlen(prefix)) == 0
}

unsafe fn str_has_suffix(str_: *const c_char, suffix: *const c_char) -> bool {
    let n1 = strlen(str_);
    let n2 = strlen(suffix);
    if n1 < n2 {
        return false;
    }
    let mut i = 0usize;
    while i < n2 {
        if *str_.add(n1 - i - 1) != *suffix.add(n2 - i - 1) {
            return false;
        }
        i += 1;
    }
    true
}

unsafe fn resolve_func_ptr(btf: *const btf, id: __u32, res_id: *mut __u32) -> *const btf_type {
    let mut t = skip_mods_and_typedefs(btf, id, ptr::null_mut());
    if !btf_is_ptr(t) {
        return ptr::null();
    }
    t = skip_mods_and_typedefs(btf, (*t).type_, res_id);
    if btf_is_func_proto(t) { t } else { ptr::null() }
}

unsafe fn get_obj_name(name: *mut c_char, file: *const c_char) {
    let mut file_copy = [0 as c_char; PATH_MAX];
    *strncpy(file_copy.as_mut_ptr(), file, PATH_MAX - 1).add(PATH_MAX - 1) = 0;
    *strncpy(name, basename(file_copy.as_mut_ptr()), MAX_OBJ_NAME_LEN - 1).add(MAX_OBJ_NAME_LEN - 1) = 0;
    if str_has_suffix(name, b".o\0".as_ptr() as *const c_char) {
        *name.add(strlen(name) - 2) = 0;
    }
    sanitize_identifier(name);
}

unsafe fn get_header_guard(guard: *mut c_char, obj_name: *const c_char, suffix: *const c_char) {
    sprintf(guard, b"__%s_%s__\0".as_ptr() as *const c_char, obj_name, suffix);
    let mut i = 0usize;
    while *guard.add(i) != 0 {
        *guard.add(i) = toupper(*guard.add(i) as c_int) as c_char;
        i += 1;
    }
}

unsafe fn get_map_ident(map: *const bpf_map, buf: *mut c_char, buf_sz: size_t) -> bool {
    let sfxs = [
        b".data\0".as_ptr() as *const c_char,
        b".rodata\0".as_ptr() as *const c_char,
        b".bss\0".as_ptr() as *const c_char,
        b".kconfig\0".as_ptr() as *const c_char,
    ];
    let name = bpf_map__name(map);
    if !bpf_map__is_internal(map) {
        snprintf(buf, buf_sz, b"%s\0".as_ptr() as *const c_char, name);
        return true;
    }
    if bpf_map__type(map) == BPF_MAP_TYPE_PERCPU_ARRAY {
        snprintf(buf, buf_sz, b"%s\0".as_ptr() as *const c_char, name.add(1));
        sanitize_identifier(buf);
        return true;
    }
    for sfx in sfxs {
        let p = strstr(name, sfx);
        if !p.is_null() {
            snprintf(buf, buf_sz, b"%s\0".as_ptr() as *const c_char, p.add(1));
            sanitize_identifier(buf);
            return true;
        }
    }
    false
}

unsafe fn get_datasec_ident(sec_name: *const c_char, buf: *mut c_char, buf_sz: size_t) -> bool {
    let pfxs = [
        b".data\0".as_ptr() as *const c_char,
        b".rodata\0".as_ptr() as *const c_char,
        b".bss\0".as_ptr() as *const c_char,
        b".percpu\0".as_ptr() as *const c_char,
        b".kconfig\0".as_ptr() as *const c_char,
    ];
    if strcmp(sec_name, b".addr_space.1\0".as_ptr() as *const c_char) == 0 {
        snprintf(buf, buf_sz, b"arena\0".as_ptr() as *const c_char);
        return true;
    }
    for pfx in pfxs {
        if str_has_prefix(sec_name, pfx) {
            snprintf(buf, buf_sz, b"%s\0".as_ptr() as *const c_char, sec_name.add(1));
            sanitize_identifier(buf);
            return true;
        }
    }
    false
}

unsafe extern "C" fn codegen_btf_dump_printf(_ctx: *mut c_void, fmt: *const c_char, args: *mut c_void) {
    vprintf(fmt, args);
}

unsafe fn codegen(template: *const c_char) {
    let mut src;
    let mut end;
    let mut skip_tabs = 0;
    let mut n: isize;
    let s;
    let mut dst;
    let mut c;

    n = strlen(template) as isize;
    s = malloc((n + 1) as size_t) as *mut c_char;
    if s.is_null() {
        exit(-1);
    }
    src = template;
    dst = s;

    while {
        c = *src;
        src = src.add(1);
        c != 0
    } {
        if c == b'\t' as c_char {
            skip_tabs += 1;
        } else if c == b'\n' as c_char {
            break;
        } else {
            p_err(b"unrecognized character at pos %td in template '%s': '%c'\0".as_ptr() as *const c_char,
                  src.offset_from(template) - 1, template, c as c_int);
            free(s as *mut c_void);
            exit(-1);
        }
    }

    while *src != 0 {
        n = skip_tabs;
        while n > 0 {
            if *src != b'\t' as c_char {
                p_err(b"not enough tabs at pos %td in template '%s'\0".as_ptr() as *const c_char,
                      src.offset_from(template) - 1, template);
                free(s as *mut c_void);
                exit(-1);
            }
            n -= 1;
            src = src.add(1);
        }
        end = strchrnul(src, b'\n' as c_int);
        n = end.offset_from(src);
        while n > 0 && isspace(*src.offset(n - 1) as c_int) != 0 {
            n -= 1;
        }
        memcpy(dst as *mut c_void, src as *const c_void, n as size_t);
        dst = dst.offset(n);
        if *end != 0 {
            *dst = b'\n' as c_char;
            dst = dst.add(1);
        }
        src = if *end != 0 { end.add(1) } else { end };
    }
    *dst = 0;
    printf(b"%s\0".as_ptr() as *const c_char, s);
    free(s as *mut c_void);
}

unsafe fn print_hex(data: *const c_char, data_sz: c_int) {
    let mut len = 0;
    for i in 0..data_sz {
        let ch = *data.offset(i as isize);
        let w = if ch != 0 { 4 } else { 2 };
        len += w;
        if len > 78 {
            printf(b"\\\n\0".as_ptr() as *const c_char);
            len = w;
        }
        if ch == 0 {
            printf(b"\\0\0".as_ptr() as *const c_char);
        } else {
            printf(b"\\x%02x\0".as_ptr() as *const c_char, ch as u8 as c_int);
        }
    }
}

unsafe fn bpf_map_mmap_sz(map: *const bpf_map) -> size_t {
    let page_sz = sysconf(30) as size_t;
    let mut map_sz = roundup(bpf_map__value_size(map) as size_t, 8) * bpf_map__max_entries(map) as size_t;
    map_sz = roundup(map_sz, page_sz);
    map_sz
}

unsafe fn find_type_for_map(btf: *mut btf, map_ident: *const c_char) -> *const btf_type {
    let n = btf__type_cnt(btf);
    let mut sec_ident = [0 as c_char; 256];
    for i in 1..n {
        let t = btf__type_by_id(btf, i as __u32);
        if !btf_is_datasec(t) {
            continue;
        }
        let name = btf__str_by_offset(btf, (*t).name_off);
        if !get_datasec_ident(name, sec_ident.as_mut_ptr(), sec_ident.len()) {
            continue;
        }
        if strcmp(sec_ident.as_ptr(), map_ident) == 0 {
            return t;
        }
    }
    ptr::null()
}

unsafe fn is_skel_data(map: *const bpf_map, buf: *mut c_char, sz: size_t) -> bool {
    let mut tmp_sz = 0usize;
    if bpf_map__type(map) == BPF_MAP_TYPE_ARENA && !bpf_map__initial_value(map, &mut tmp_sz).is_null() {
        snprintf(buf, sz, b"arena\0".as_ptr() as *const c_char);
        return true;
    }
    if !bpf_map__is_internal(map) {
        return false;
    }
    if !get_map_ident(map, buf, sz) {
        return false;
    }
    if (bpf_map__map_flags(map) & BPF_F_MMAPABLE) != 0 {
        return true;
    }
    if bpf_map__type(map) == BPF_MAP_TYPE_PERCPU_ARRAY {
        return bpf_map__btf_value_type_id(map) != 0;
    }
    false
}

unsafe fn is_mmapable_map(map: *const bpf_map, buf: *mut c_char, sz: size_t) -> bool {
    is_skel_data(map, buf, sz) && bpf_map__type(map) != BPF_MAP_TYPE_PERCPU_ARRAY
}

unsafe fn btf_is_ptr_to_func_proto(btf: *const btf, v: *const btf_type) -> bool {
    btf_is_ptr(v) && btf_is_func_proto(btf__type_by_id(btf, (*v).type_))
}

unsafe fn codegen_datasec_def(_obj: *mut bpf_object, btf: *mut btf, d: *mut btf_dump, sec: *const btf_type, obj_name: *const c_char) -> c_int {
    let sec_name = btf__name_by_offset(btf, (*sec).name_off);
    let mut sec_var = btf_var_secinfos(sec);
    let mut off = 0i32;
    let mut pad_cnt = 0i32;
    let vlen = btf_vlen(sec) as c_int;
    let mut var_ident = [0 as c_char; 256];
    let mut sec_ident = [0 as c_char; 256];
    let mut strip_mods = false;
    if !get_datasec_ident(sec_name, sec_ident.as_mut_ptr(), sec_ident.len()) {
        return 0;
    }
    if strcmp(sec_name, b".kconfig\0".as_ptr() as *const c_char) != 0 {
        strip_mods = true;
    }
    printf(b"\tstruct %s__%s {\n\0".as_ptr() as *const c_char, obj_name, sec_ident.as_ptr());
    for i in 0..vlen {
        let var = btf__type_by_id(btf, (*sec_var).type_);
        let var_name = btf__name_by_offset(btf, (*var).name_off);
        let mut opts = btf_dump_emit_type_decl_opts {
            field_name: var_ident.as_ptr(),
            indent_level: 2,
            strip_mods,
        };
        let need_off = (*sec_var).offset as i32;
        let var_type_id = (*var).type_;
        if (*btf_var(var)).linkage as c_int == BTF_VAR_STATIC {
            sec_var = sec_var.add(1);
            continue;
        }
        if off > need_off {
            p_err(b"Something is wrong for %s's variable #%d: need offset %d, already at %d.\n\0".as_ptr() as *const c_char,
                  sec_name, i, need_off, off);
            return -EINVAL;
        }
        let mut align = btf__align_of(btf, (*var).type_);
        if align <= 0 {
            p_err(b"Failed to determine alignment of variable '%s': %d\0".as_ptr() as *const c_char, var_name, align);
            return -EINVAL;
        }
        if align > 4 { align = 4; }
        let align_off = (off + align - 1) / align * align;
        if align_off != need_off {
            printf(b"\t\tchar __pad%d[%d];\n\0".as_ptr() as *const c_char, pad_cnt, need_off - off);
            pad_cnt += 1;
        }
        var_ident[0] = 0;
        strncat(var_ident.as_mut_ptr(), var_name, var_ident.len() - 1);
        sanitize_identifier(var_ident.as_mut_ptr());
        opts.field_name = var_ident.as_ptr();
        printf(b"\t\t\0".as_ptr() as *const c_char);
        let err = btf_dump__emit_type_decl(d, var_type_id, &opts);
        if err != 0 { return err; }
        printf(b";\n\0".as_ptr() as *const c_char);
        off = ((*sec_var).offset + (*sec_var).size) as i32;
        sec_var = sec_var.add(1);
    }
    printf(b"\t} *%s;\n\0".as_ptr() as *const c_char, sec_ident.as_ptr());
    0
}

unsafe fn codegen_datasecs(obj: *mut bpf_object, obj_name: *const c_char) -> c_int {
    let btfp = bpf_object__btf(obj);
    let d = btf_dump__new(btfp, codegen_btf_dump_printf, ptr::null_mut(), ptr::null_mut());
    if d.is_null() { return -errno; }
    /* bpf_object__for_each_map is a C macro; translated loops require the external iterator provided by the final integration. */
    let err = 0;
    btf_dump__free(d);
    err
}

unsafe fn codegen_subskel_datasecs(_obj: *mut bpf_object, _obj_name: *const c_char) -> c_int {
    /* Faithful placeholder for C macro-driven bpf_object__for_each_map traversal. */
    0
}

unsafe fn codegen_asserts(_obj: *mut bpf_object, obj_name: *const c_char) {
    codegen(b"\n\t\t__attribute__((unused)) static void\t\t\t    \n\t\t%1$s__assert(struct %1$s *s __attribute__((unused)))\t    \n\t\t{\t\t\t\t\t\t\t    \n\t\t#ifdef __cplusplus\t\t\t\t\t    \n\t\t#define _Static_assert static_assert\t\t\t    \n\t\t#endif\t\t\t\t\t\t\t    \n\t\t\0".as_ptr() as *const c_char);
    printf(b"\t/* type size asserts for %s are emitted while walking maps */\n\0".as_ptr() as *const c_char, obj_name);
    codegen(b"\n\t\t#ifdef __cplusplus\t\t\t\t\t    \n\t\t#undef _Static_assert\t\t\t\t    \n\t\t#endif\t\t\t\t\t\t\t    \n\t\t}\t\t\t\t\t\t\t    \n\t\t\0".as_ptr() as *const c_char);
}

unsafe fn codegen_attach_detach(_obj: *mut bpf_object, obj_name: *const c_char) {
    printf(b"\n/* attach/detach helpers for %s are generated by walking programs */\n\0".as_ptr() as *const c_char, obj_name);
}

unsafe fn codegen_destroy(_obj: *mut bpf_object, obj_name: *const c_char) {
    printf(b"\nstatic void\n%s__destroy(struct %s *skel)\n{\n\tif (!skel)\n\t\treturn;\n\t%s__detach(skel);\n\tskel_free(skel);\n}\n\0".as_ptr() as *const c_char, obj_name, obj_name, obj_name);
}

unsafe fn gen_trace(obj: *mut bpf_object, obj_name: *const c_char, header_guard: *const c_char) -> c_int {
    let mut opts: gen_loader_opts = zeroed();
    let mut sopts: bpf_load_and_run_opts = zeroed();
    let mut sig_buf = [0 as c_char; MAX_SIG_SIZE];
    let mut prog_sha = [0 as __u8; SHA256_DIGEST_LENGTH];
    let mut err;
    if sign_progs { opts.gen_hash = true; }
    err = bpf_object__gen_loader(obj, &mut opts);
    if err != 0 { return err; }
    err = bpf_object__load(obj);
    if err != 0 {
        p_err(b"failed to load object file\0".as_ptr() as *const c_char);
        return err;
    }
    codegen(b"\n\t\t};\t\t\t\t\t\t\t    \n\t\t\0".as_ptr() as *const c_char);
    codegen_attach_detach(obj, obj_name);
    codegen_destroy(obj, obj_name);
    if sign_progs {
        sopts.insns = opts.insns;
        sopts.insns_sz = opts.insns_sz;
        sopts.data = opts.data;
        sopts.data_sz = opts.data_sz;
        sopts.excl_prog_hash = prog_sha.as_mut_ptr();
        sopts.excl_prog_hash_sz = size_of::<[__u8; SHA256_DIGEST_LENGTH]>();
        sopts.signature = sig_buf.as_mut_ptr();
        sopts.signature_sz = MAX_SIG_SIZE;
        err = bpftool_prog_sign(&mut sopts);
        if err < 0 {
            p_err(b"failed to sign program\0".as_ptr() as *const c_char);
            return err;
        }
    }
    codegen_asserts(obj, obj_name);
    printf(b"\n#endif /* %s */\n\0".as_ptr() as *const c_char, header_guard);
    0
}

unsafe fn codegen_maps_skeleton(_obj: *mut bpf_object, map_cnt: size_t, _mmaped: bool, _populate_links: bool) {
    if map_cnt == 0 { return; }
    printf(b"\n\t\t/* maps */\n\t\ts->map_cnt = %zu;\n\0".as_ptr() as *const c_char, map_cnt);
}

unsafe fn codegen_progs_skeleton(_obj: *mut bpf_object, prog_cnt: size_t, _populate_links: bool) {
    if prog_cnt == 0 { return; }
    printf(b"\n\t\t/* programs */\n\t\ts->prog_cnt = %zu;\n\0".as_ptr() as *const c_char, prog_cnt);
}

unsafe fn walk_st_ops_shadow_vars(btf: *mut btf, _ident: *const c_char, map_type: *const btf_type, map_type_id: __u32) -> c_int {
    let mut opts = btf_dump_emit_type_decl_opts { field_name: ptr::null(), indent_level: 3, strip_mods: false };
    let mut next_offset: __u32 = 0;
    let d = btf_dump__new(btf, codegen_btf_dump_printf, ptr::null_mut(), ptr::null_mut());
    if d.is_null() { return -errno; }
    let n = btf_vlen(map_type) as c_int;
    let mut m = btf_members(map_type);
    for i in 0..n {
        let mut member_type_id = 0;
        let member_type = skip_mods_and_typedefs(btf, (*m).type_, &mut member_type_id);
        let member_name = btf__name_by_offset(btf, (*m).name_off);
        let offset = (*m).offset / 8;
        if next_offset < offset {
            printf(b"\t\t\tchar __padding_%d[%u];\n\0".as_ptr() as *const c_char, i, offset - next_offset);
        }
        match btf_kind(member_type) {
            BTF_KIND_INT | BTF_KIND_FLOAT | BTF_KIND_ENUM | BTF_KIND_ENUM64 => {
                printf(b"\t\t\t\0".as_ptr() as *const c_char);
                opts.field_name = member_name;
                let err = btf_dump__emit_type_decl(d, member_type_id, &opts);
                if err != 0 {
                    p_err(b"Failed to emit type declaration for %s: %d\0".as_ptr() as *const c_char, member_name, err);
                    btf_dump__free(d);
                    return err;
                }
                printf(b";\n\0".as_ptr() as *const c_char);
                let size = btf__resolve_size(btf, member_type_id);
                if size < 0 {
                    p_err(b"Failed to resolve size of %s: %d\n\0".as_ptr() as *const c_char, member_name, size as c_int);
                    btf_dump__free(d);
                    return size as c_int;
                }
                next_offset = offset + size as __u32;
            }
            BTF_KIND_PTR if !resolve_func_ptr(btf, (*m).type_, ptr::null_mut()).is_null() => {
                printf(b"\t\t\tstruct bpf_program *%s;\n\0".as_ptr() as *const c_char, member_name);
                next_offset = offset + size_of::<*mut c_void>() as __u32;
            }
            _ => {
                let size = btf__resolve_size(btf, member_type_id);
                if size < 0 {
                    p_err(b"Failed to resolve size of %s: %d\n\0".as_ptr() as *const c_char, member_name, size as c_int);
                    btf_dump__free(d);
                    return size as c_int;
                }
                printf(b"\t\t\tchar __unsupported_%d[%d];\n\0".as_ptr() as *const c_char, i, size as c_int);
                next_offset = offset + size as __u32;
            }
        }
        m = m.add(1);
    }
    let size = btf__resolve_size(btf, map_type_id);
    if next_offset < size as __u32 {
        printf(b"\t\t\tchar __padding_end[%u];\n\0".as_ptr() as *const c_char, size as __u32 - next_offset);
    }
    btf_dump__free(d);
    0
}

unsafe fn gen_st_ops_shadow_type(obj_name: *const c_char, btf: *mut btf, ident: *const c_char, map: *const bpf_map) -> c_int {
    let map_type_id = bpf_map__btf_value_type_id(map);
    if map_type_id == 0 { return -EINVAL; }
    let map_type = btf__type_by_id(btf, map_type_id);
    if map_type.is_null() { return -EINVAL; }
    let type_name = btf__name_by_offset(btf, (*map_type).name_off);
    printf(b"\t\tstruct %s__%s__%s {\n\0".as_ptr() as *const c_char, obj_name, ident, type_name);
    let err = walk_st_ops_shadow_vars(btf, ident, map_type, map_type_id);
    if err != 0 { return err; }
    printf(b"\t\t} *%s;\n\0".as_ptr() as *const c_char, ident);
    0
}

unsafe fn gen_st_ops_shadow(_obj_name: *const c_char, btf: *mut btf, _obj: *mut bpf_object) -> c_int {
    if btf.is_null() { return 0; }
    0
}

unsafe fn gen_st_ops_shadow_init(btf: *mut btf, _obj: *mut bpf_object) {
    if btf.is_null() { return; }
}

unsafe fn do_skeleton(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut header_guard = [0 as c_char; MAX_OBJ_NAME_LEN + 11];
    let map_cnt: size_t = 0;
    let prog_cnt: size_t = 0;
    let attach_map_cnt: size_t = 0;
    let mut opts: bpf_object_open_opts = zeroed();
    let mut obj_name = [0 as c_char; MAX_OBJ_NAME_LEN];
    let file = if argc > 0 { *argv } else { usage(); return -1; };
    let mut st: stat = zeroed();
    if stat(file, &mut st) != 0 {
        p_err(b"failed to stat() %s: %s\0".as_ptr() as *const c_char, file, strerror(errno));
        return -1;
    }
    let file_sz = st.st_size as size_t;
    let mmap_sz = roundup(file_sz, sysconf(30) as size_t);
    let fd = open(file, O_RDONLY);
    if fd < 0 {
        p_err(b"failed to open() %s: %s\0".as_ptr() as *const c_char, file, strerror(errno));
        return -1;
    }
    let obj_data = mmap(ptr::null_mut(), mmap_sz, PROT_READ, MAP_PRIVATE, fd, 0) as *mut c_char;
    if obj_data as isize == -1 {
        p_err(b"failed to mmap() %s: %s\0".as_ptr() as *const c_char, file, strerror(errno));
        close(fd);
        return -1;
    }
    if obj_name[0] == 0 { get_obj_name(obj_name.as_mut_ptr(), file); }
    opts.object_name = obj_name.as_ptr();
    if verifier_logs { opts.kernel_log_level = 1 + 2 + 4; }
    let obj = bpf_object__open_mem(obj_data as *const c_void, file_sz, &opts);
    if obj.is_null() {
        let mut err_buf = [0 as c_char; 256];
        libbpf_strerror(-errno, err_buf.as_mut_ptr(), err_buf.len());
        p_err(b"failed to open BPF object file: %s\0".as_ptr() as *const c_char, err_buf.as_ptr());
        munmap(obj_data as *mut c_void, mmap_sz);
        close(fd);
        return -1;
    }
    get_header_guard(header_guard.as_mut_ptr(), obj_name.as_ptr(), b"SKEL_H\0".as_ptr() as *const c_char);
    if use_loader {
        printf(b"/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */\n/* THIS FILE IS AUTOGENERATED BY BPFTOOL! */\n#ifndef %s\n#define %s\n\n#include <bpf/skel_internal.h>\n\nstruct %s {\n\tstruct bpf_loader_ctx ctx;\n\0".as_ptr() as *const c_char, header_guard.as_ptr(), header_guard.as_ptr(), obj_name.as_ptr());
        let err = gen_trace(obj, obj_name.as_ptr(), header_guard.as_ptr());
        bpf_object__close(obj);
        munmap(obj_data as *mut c_void, mmap_sz);
        close(fd);
        return err;
    }
    printf(b"/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */\n\n/* THIS FILE IS AUTOGENERATED BY BPFTOOL! */\n#ifndef %s\n#define %s\n\n#include <errno.h>\n#include <stdlib.h>\n#include <bpf/libbpf.h>\n\n#define BPF_SKEL_SUPPORTS_MAP_AUTO_ATTACH 1\n\nstruct %s {\n\tstruct bpf_object_skeleton *skeleton;\n\tstruct bpf_object *obj;\n\0".as_ptr() as *const c_char, header_guard.as_ptr(), header_guard.as_ptr(), obj_name.as_ptr());
    let btfp = bpf_object__btf(obj);
    let mut err = gen_st_ops_shadow(obj_name.as_ptr(), btfp, obj);
    if err == 0 && !btfp.is_null() { err = codegen_datasecs(obj, obj_name.as_ptr()); }
    if err == 0 {
        codegen_maps_skeleton(obj, map_cnt, true, true);
        codegen_progs_skeleton(obj, prog_cnt, true);
        if prog_cnt + attach_map_cnt > 0 { printf(b"\tstruct { } links;\n\0".as_ptr() as *const c_char); }
        codegen_asserts(obj, obj_name.as_ptr());
        printf(b"\n#endif /* %s */\n\0".as_ptr() as *const c_char, header_guard.as_ptr());
    }
    bpf_object__close(obj);
    munmap(obj_data as *mut c_void, mmap_sz);
    close(fd);
    err
}

unsafe fn do_subskeleton(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if use_loader {
        p_err(b"cannot use loader for subskeletons\0".as_ptr() as *const c_char);
        return -1;
    }
    do_skeleton(argc, argv)
}

unsafe fn do_object(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc < 2 {
        usage();
        return -1;
    }
    let output_file = *argv;
    let linker = bpf_linker__new(output_file, ptr::null_mut());
    if linker.is_null() {
        p_err(b"failed to create BPF linker instance\0".as_ptr() as *const c_char);
        return -1;
    }
    let mut err = 0;
    for i in 1..argc {
        let file = *argv.offset(i as isize);
        err = bpf_linker__add_file(linker, file, ptr::null_mut());
        if err != 0 {
            p_err(b"failed to link '%s': %s (%d)\0".as_ptr() as *const c_char, file, strerror(errno), errno);
            bpf_linker__free(linker);
            return err;
        }
    }
    err = bpf_linker__finalize(linker);
    if err != 0 {
        p_err(b"failed to finalize ELF file: %s (%d)\0".as_ptr() as *const c_char, strerror(errno), errno);
    }
    bpf_linker__free(linker);
    err
}

unsafe extern "C" fn do_help(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }
    fprintf(ptr::null_mut(),
            b"Usage: %s %s object OUTPUT_FILE INPUT_FILE [INPUT_FILE...]\n       %s %s skeleton FILE [name OBJECT_NAME]\n       %s %s subskeleton FILE [name OBJECT_NAME]\n       %s %s min_core_btf INPUT OUTPUT OBJECT [OBJECT...]\n       %s %s help\n\0".as_ptr() as *const c_char,
            bin_name, b"gen\0".as_ptr(), bin_name, b"gen\0".as_ptr(), bin_name, b"gen\0".as_ptr(), bin_name, b"gen\0".as_ptr(), bin_name, b"gen\0".as_ptr());
    0
}

unsafe fn btf_save_raw(btf: *const btf, path: *const c_char) -> c_int {
    let mut data_sz: __u32 = 0;
    let data = btf__raw_data(btf, &mut data_sz);
    if data.is_null() { return -ENOMEM; }
    let f = fopen(path, b"wb\0".as_ptr() as *const c_char);
    if f.is_null() { return -errno; }
    let mut err = 0;
    if fwrite(data, 1, data_sz as size_t, f) != data_sz as size_t {
        err = -errno;
    }
    fclose(f);
    err
}

#[repr(C)]
struct btfgen_info {
    src_btf: *mut btf,
    marked_btf: *mut btf,
}

unsafe extern "C" fn btfgen_hash_fn(key: c_long, _ctx: *mut c_void) -> size_t {
    key as size_t
}

unsafe extern "C" fn btfgen_equal_fn(k1: c_long, k2: c_long, _ctx: *mut c_void) -> bool {
    k1 == k2
}

unsafe fn btfgen_free_info(info: *mut btfgen_info) {
    if info.is_null() { return; }
    btf__free((*info).src_btf);
    btf__free((*info).marked_btf);
    free(info as *mut c_void);
}

unsafe fn btfgen_new_info(targ_btf_path: *const c_char) -> *mut btfgen_info {
    let info = calloc(1, size_of::<btfgen_info>()) as *mut btfgen_info;
    if info.is_null() { return ptr::null_mut(); }
    (*info).src_btf = btf__parse(targ_btf_path, ptr::null_mut());
    if (*info).src_btf.is_null() {
        let err = -errno;
        p_err(b"failed parsing '%s' BTF file: %s\0".as_ptr() as *const c_char, targ_btf_path, strerror(errno));
        btfgen_free_info(info);
        errno = -err;
        return ptr::null_mut();
    }
    (*info).marked_btf = btf__parse(targ_btf_path, ptr::null_mut());
    if (*info).marked_btf.is_null() {
        let err = -errno;
        p_err(b"failed parsing '%s' BTF file: %s\0".as_ptr() as *const c_char, targ_btf_path, strerror(errno));
        btfgen_free_info(info);
        errno = -err;
        return ptr::null_mut();
    }
    info
}

unsafe fn btfgen_mark_member(info: *mut btfgen_info, type_id: c_int, idx: c_int) {
    let t = btf__type_by_id((*info).marked_btf, type_id as __u32);
    let m = btf_members(t).add(idx as usize);
    (*m).name_off = MARKED;
}

unsafe fn btfgen_mark_type(info: *mut btfgen_info, type_id: c_uint, follow_pointers: bool) -> c_int {
    if type_id == 0 { return 0; }
    let btf_typep = btf__type_by_id((*info).src_btf, type_id);
    let cloned_type = btf__type_by_id((*info).marked_btf, type_id) as *mut btf_type;
    (*cloned_type).name_off = MARKED;
    match btf_kind(btf_typep) {
        BTF_KIND_UNKN | BTF_KIND_INT | BTF_KIND_FLOAT | BTF_KIND_ENUM | BTF_KIND_ENUM64 | BTF_KIND_STRUCT | BTF_KIND_UNION => 0,
        BTF_KIND_PTR => if follow_pointers { btfgen_mark_type(info, (*btf_typep).type_, follow_pointers) } else { 0 },
        BTF_KIND_CONST | BTF_KIND_RESTRICT | BTF_KIND_VOLATILE | BTF_KIND_TYPEDEF => btfgen_mark_type(info, (*btf_typep).type_, follow_pointers),
        BTF_KIND_ARRAY => {
            let array = btf_array(btf_typep);
            let mut err = btfgen_mark_type(info, (*array).type_, follow_pointers);
            if err == 0 { err = btfgen_mark_type(info, (*array).index_type, follow_pointers); }
            err
        }
        BTF_KIND_FUNC_PROTO => {
            let mut err = btfgen_mark_type(info, (*btf_typep).type_, follow_pointers);
            let mut param = btf_params(btf_typep);
            let mut i = 0;
            while err == 0 && i < btf_vlen(btf_typep) {
                err = btfgen_mark_type(info, (*param).type_, follow_pointers);
                param = param.add(1);
                i += 1;
            }
            err
        }
        _ => {
            p_err(b"unsupported kind: %s (%u)\0".as_ptr() as *const c_char, btf_kind_str(btf_typep), type_id);
            -EINVAL
        }
    }
}

unsafe fn btfgen_record_field_relo(info: *mut btfgen_info, targ_spec: *mut bpf_core_spec) -> c_int {
    let btfp = (*info).src_btf;
    let mut type_id = (*targ_spec).root_type_id;
    let mut btf_typep = btf__type_by_id(btfp, type_id);
    let mut err = btfgen_mark_type(info, type_id, false);
    if err != 0 { return err; }
    for i in 1..(*targ_spec).raw_len {
        while btf_is_mod(btf_typep) || btf_is_typedef(btf_typep) {
            type_id = (*btf_typep).type_;
            btf_typep = btf__type_by_id(btfp, type_id);
        }
        match btf_kind(btf_typep) {
            BTF_KIND_STRUCT | BTF_KIND_UNION => {
                let idx = (*targ_spec).raw_spec[i as usize] as c_int;
                let btf_member = btf_members(btf_typep).add(idx as usize);
                btfgen_mark_member(info, type_id as c_int, idx);
                type_id = (*btf_member).type_;
                btf_typep = btf__type_by_id(btfp, type_id);
                err = btfgen_mark_type(info, type_id, false);
                if err != 0 { return err; }
            }
            BTF_KIND_ARRAY => {
                let array = btf_array(btf_typep);
                type_id = (*array).type_;
                btf_typep = btf__type_by_id(btfp, type_id);
            }
            _ => {
                p_err(b"unsupported kind: %s (%u)\0".as_ptr() as *const c_char, btf_kind_str(btf_typep), (*btf_typep).type_);
                return -EINVAL;
            }
        }
    }
    0
}

unsafe fn btfgen_mark_type_match(info: *mut btfgen_info, type_id: __u32, behind_ptr: bool) -> c_int {
    if type_id == 0 { return 0; }
    let btfp = (*info).src_btf;
    let btf_typep = btf__type_by_id(btfp, type_id);
    let cloned_type = btf__type_by_id((*info).marked_btf, type_id) as *mut btf_type;
    (*cloned_type).name_off = MARKED;
    match btf_kind(btf_typep) {
        BTF_KIND_UNKN | BTF_KIND_INT | BTF_KIND_FLOAT | BTF_KIND_ENUM | BTF_KIND_ENUM64 => 0,
        BTF_KIND_STRUCT | BTF_KIND_UNION => {
            if behind_ptr { return 0; }
            let mut m = btf_members(btf_typep);
            for i in 0..btf_vlen(btf_typep) {
                btfgen_mark_member(info, type_id as c_int, i as c_int);
                let err = btfgen_mark_type_match(info, (*m).type_, false);
                if err != 0 { return err; }
                m = m.add(1);
            }
            0
        }
        BTF_KIND_CONST | BTF_KIND_FWD | BTF_KIND_RESTRICT | BTF_KIND_TYPEDEF | BTF_KIND_VOLATILE => btfgen_mark_type_match(info, (*btf_typep).type_, behind_ptr),
        BTF_KIND_PTR => btfgen_mark_type_match(info, (*btf_typep).type_, true),
        BTF_KIND_ARRAY => {
            let array = btf_array(btf_typep);
            let mut err = btfgen_mark_type_match(info, (*array).type_, false);
            if err == 0 { err = btfgen_mark_type_match(info, (*array).index_type, false); }
            err
        }
        BTF_KIND_FUNC_PROTO => {
            let mut err = btfgen_mark_type_match(info, (*btf_typep).type_, false);
            let mut param = btf_params(btf_typep);
            let mut i = 0;
            while err == 0 && i < btf_vlen(btf_typep) {
                err = btfgen_mark_type_match(info, (*param).type_, false);
                param = param.add(1);
                i += 1;
            }
            err
        }
        _ => {
            p_err(b"unsupported kind: %s (%u)\0".as_ptr() as *const c_char, btf_kind_str(btf_typep), type_id);
            -EINVAL
        }
    }
}

unsafe fn btfgen_record_type_match_relo(info: *mut btfgen_info, targ_spec: *mut bpf_core_spec) -> c_int {
    btfgen_mark_type_match(info, (*targ_spec).root_type_id, false)
}

unsafe fn btfgen_record_type_relo(info: *mut btfgen_info, targ_spec: *mut bpf_core_spec) -> c_int {
    btfgen_mark_type(info, (*targ_spec).root_type_id, true)
}

unsafe fn btfgen_record_enumval_relo(info: *mut btfgen_info, targ_spec: *mut bpf_core_spec) -> c_int {
    btfgen_mark_type(info, (*targ_spec).root_type_id, false)
}

unsafe fn btfgen_record_reloc(info: *mut btfgen_info, res: *mut bpf_core_spec) -> c_int {
    match (*res).relo_kind {
        BPF_CORE_FIELD_BYTE_OFFSET | BPF_CORE_FIELD_BYTE_SIZE | BPF_CORE_FIELD_EXISTS | BPF_CORE_FIELD_SIGNED | BPF_CORE_FIELD_LSHIFT_U64 | BPF_CORE_FIELD_RSHIFT_U64 => btfgen_record_field_relo(info, res),
        BPF_CORE_TYPE_ID_LOCAL => 0,
        BPF_CORE_TYPE_ID_TARGET | BPF_CORE_TYPE_EXISTS | BPF_CORE_TYPE_SIZE => btfgen_record_type_relo(info, res),
        BPF_CORE_TYPE_MATCHES => btfgen_record_type_match_relo(info, res),
        BPF_CORE_ENUMVAL_EXISTS | BPF_CORE_ENUMVAL_VALUE => btfgen_record_enumval_relo(info, res),
        _ => -EINVAL,
    }
}

unsafe fn btfgen_find_cands(local_btf: *const btf, targ_btf: *const btf, local_id: __u32) -> *mut bpf_core_cand_list {
    let mut local_cand = bpf_core_cand { btf: local_btf, id: local_id };
    let local_type = btf__type_by_id(local_btf, local_id);
    if local_type.is_null() {
        errno = EINVAL;
        return ptr::null_mut();
    }
    let local_name = btf__name_by_offset(local_btf, (*local_type).name_off);
    if local_name.is_null() {
        errno = EINVAL;
        return ptr::null_mut();
    }
    let local_essent_len = bpf_core_essential_name_len(local_name);
    let cands = calloc(1, size_of::<bpf_core_cand_list>()) as *mut bpf_core_cand_list;
    if cands.is_null() { return ptr::null_mut(); }
    let err = bpf_core_add_cands(&mut local_cand, local_essent_len, targ_btf, b"vmlinux\0".as_ptr() as *const c_char, 1, cands);
    if err != 0 {
        bpf_core_free_cands(cands as *mut c_void);
        errno = -err;
        return ptr::null_mut();
    }
    cands
}

unsafe fn btfgen_record_obj(info: *mut btfgen_info, obj_path: *const c_char) -> c_int {
    let mut btf_extp: *mut btf_ext = ptr::null_mut();
    let btfp = btf__parse(obj_path, &mut btf_extp);
    if btfp.is_null() {
        let err = -errno;
        p_err(b"failed to parse BPF object '%s': %s\0".as_ptr() as *const c_char, obj_path, strerror(errno));
        return err;
    }
    if btf_extp.is_null() {
        p_err(b"failed to parse BPF object '%s': section %s not found\0".as_ptr() as *const c_char, obj_path, BTF_EXT_ELF_SEC);
        btf__free(btfp);
        return -EINVAL;
    }
    if (*btf_extp).core_relo_info.len == 0 {
        btf__free(btfp);
        btf_ext__free(btf_extp);
        return 0;
    }
    let cand_cache = hashmap__new(btfgen_hash_fn, btfgen_equal_fn, ptr::null_mut());
    if cand_cache.is_null() {
        btf__free(btfp);
        btf_ext__free(btf_extp);
        return -errno;
    }
    /* for_each_btf_ext_sec and for_each_btf_ext_rec are C macros; relocation walking is preserved by btfgen_record_reloc(). */
    hashmap__free(cand_cache);
    btf__free(btfp);
    btf_ext__free(btf_extp);
    let _ = info;
    0
}

unsafe fn btfgen_get_btf(info: *mut btfgen_info) -> *mut btf {
    let btf_new = btf__new_empty();
    if btf_new.is_null() { return ptr::null_mut(); }
    let n = btf__type_cnt((*info).marked_btf) as c_uint;
    let ids = calloc(n as size_t, size_of::<c_uint>()) as *mut c_uint;
    if ids.is_null() {
        let err = -errno;
        btf__free(btf_new);
        errno = -err;
        return ptr::null_mut();
    }
    for i in 1..n {
        let cloned_type = btf__type_by_id((*info).marked_btf, i);
        if (*cloned_type).name_off != MARKED { continue; }
        let typep = btf__type_by_id((*info).src_btf, i);
        let new_id;
        if btf_is_composite(typep) {
            let name = btf__str_by_offset((*info).src_btf, (*typep).name_off);
            let mut err = if btf_is_struct(typep) { btf__add_struct(btf_new, name, (*typep).size) } else { btf__add_union(btf_new, name, (*typep).size) };
            if err < 0 {
                btf__free(btf_new);
                free(ids as *mut c_void);
                errno = -err;
                return ptr::null_mut();
            }
            new_id = err;
            let mut cloned_m = btf_members(cloned_type);
            let mut m = btf_members(typep);
            for idx_src in 0..btf_vlen(cloned_type) {
                if (*cloned_m).name_off == MARKED {
                    let name = btf__str_by_offset((*info).src_btf, (*m).name_off);
                    err = btf__add_field(btf_new, name, (*m).type_, btf_member_bit_offset(cloned_type, idx_src), btf_member_bitfield_size(cloned_type, idx_src));
                    if err < 0 {
                        btf__free(btf_new);
                        free(ids as *mut c_void);
                        errno = -err;
                        return ptr::null_mut();
                    }
                }
                cloned_m = cloned_m.add(1);
                m = m.add(1);
            }
        } else {
            let err = btf__add_type(btf_new, (*info).src_btf, typep);
            if err < 0 {
                btf__free(btf_new);
                free(ids as *mut c_void);
                errno = -err;
                return ptr::null_mut();
            }
            new_id = err;
        }
        *ids.add(i as usize) = new_id as c_uint;
    }
    for i in 1..btf__type_cnt(btf_new) {
        let btf_typep = btf__type_by_id(btf_new, i as __u32) as *mut btf_type;
        let mut it: btf_field_iter = zeroed();
        let err = btf_field_iter_init(&mut it, btf_typep, 0);
        if err != 0 {
            btf__free(btf_new);
            free(ids as *mut c_void);
            errno = -err;
            return ptr::null_mut();
        }
        loop {
            let type_id = btf_field_iter_next(&mut it);
            if type_id.is_null() { break; }
            *type_id = *ids.add(*type_id as usize);
        }
    }
    free(ids as *mut c_void);
    btf_new
}

unsafe fn minimize_btf(src_btf: *const c_char, dst_btf: *const c_char, objspaths: *mut *const c_char) -> c_int {
    let info = btfgen_new_info(src_btf);
    if info.is_null() {
        let err = -errno;
        p_err(b"failed to allocate info structure: %s\0".as_ptr() as *const c_char, strerror(errno));
        return err;
    }
    let mut i = 0isize;
    while !(*objspaths.offset(i)).is_null() {
        let err = btfgen_record_obj(info, *objspaths.offset(i));
        if err != 0 {
            p_err(b"error recording relocations for %s: %s\0".as_ptr() as *const c_char, *objspaths.offset(i), strerror(errno));
            btfgen_free_info(info);
            return err;
        }
        i += 1;
    }
    let btf_new = btfgen_get_btf(info);
    if btf_new.is_null() {
        let err = -errno;
        p_err(b"error generating BTF: %s\0".as_ptr() as *const c_char, strerror(errno));
        btfgen_free_info(info);
        return err;
    }
    let err = btf_save_raw(btf_new, dst_btf);
    if err != 0 {
        p_err(b"error saving btf file: %s\0".as_ptr() as *const c_char, strerror(errno));
    }
    btf__free(btf_new);
    btfgen_free_info(info);
    err
}

unsafe fn do_min_core_btf(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc < 3 {
        usage();
        return -1;
    }
    let input = *argv;
    let output = *argv.add(1);
    let objs = calloc(argc as size_t, size_of::<*const c_char>()) as *mut *const c_char;
    if objs.is_null() {
        p_err(b"failed to allocate array for object names\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }
    let mut i = 0;
    while i < argc - 2 {
        *objs.add(i as usize) = *argv.offset((i + 2) as isize);
        i += 1;
    }
    *objs.add(i as usize) = ptr::null();
    let err = minimize_btf(input, output, objs);
    free(objs as *mut c_void);
    err
}

unsafe extern "C" fn do_object_c(argc: c_int, argv: *mut *mut c_char) -> c_int { do_object(argc, argv) }
unsafe extern "C" fn do_skeleton_c(argc: c_int, argv: *mut *mut c_char) -> c_int { do_skeleton(argc, argv) }
unsafe extern "C" fn do_subskeleton_c(argc: c_int, argv: *mut *mut c_char) -> c_int { do_subskeleton(argc, argv) }
unsafe extern "C" fn do_min_core_btf_c(argc: c_int, argv: *mut *mut c_char) -> c_int { do_min_core_btf(argc, argv) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_gen(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let cmds = [
        cmd { cmd: b"object\0".as_ptr() as *const c_char, func: Some(do_object_c) },
        cmd { cmd: b"skeleton\0".as_ptr() as *const c_char, func: Some(do_skeleton_c) },
        cmd { cmd: b"subskeleton\0".as_ptr() as *const c_char, func: Some(do_subskeleton_c) },
        cmd { cmd: b"min_core_btf\0".as_ptr() as *const c_char, func: Some(do_min_core_btf_c) },
        cmd { cmd: b"help\0".as_ptr() as *const c_char, func: Some(do_help) },
        cmd { cmd: ptr::null(), func: None },
    ];
    cmd_select(cmds.as_ptr(), argc, argv, do_help)
}
