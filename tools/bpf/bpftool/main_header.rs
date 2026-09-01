/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (C) 2017-2018 Netronome Systems, Inc. */

/* Translated from bpf/bpftool/main.h. C include directives, header guards,
 * GCC poison pragmas, and source feature defines are preserved here only as
 * dependency intent:
 * - BFD and kernel.h both define GCC_VERSION differently.
 * - _GNU_SOURCE is required by the C source.
 * - Kernel-only integer typedefs u8/u16/u32/u64/s8/s16/s32/s64 must not be used.
 */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};

pub type __u32 = u32;
pub type __u64 = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type FILE = c_void;
pub type json_writer_t = c_void;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_obj_get_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_line_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog_linfo {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn ptr_to_u64(ptr: *const c_void) -> __u64 {
    ptr as usize as __u64
}

#[inline]
pub unsafe fn u64_to_ptr(ptr: __u64) -> *mut c_void {
    ptr as usize as *mut c_void
}

/* The following C statement-expression macros mutate caller-local argc/argv:
 * NEXT_ARG, NEXT_ARGP, BAD_ARG, GET_ARG, and REQ_ARGS. Rust has no direct
 * header-level equivalent for those lvalue-capturing statement expressions.
 * Their behavior is preserved at call sites by translating uses explicitly.
 */

pub const ERR_MAX_LEN: c_int = 1024;
pub const MAX_SIG_SIZE: c_int = 4096;

pub const BPF_TAG_FMT: &[u8] = b"%02hhx%02hhx%02hhx%02hhx%02hhx%02hhx%02hhx%02hhx\0";

pub const HELP_SPEC_PROGRAM: &[u8] =
    b"PROG := { id PROG_ID | pinned FILE | tag PROG_TAG | name PROG_NAME }\0";
pub const HELP_SPEC_OPTIONS: &[u8] =
    b"OPTIONS := { {-j|--json} [{-p|--pretty}] | {-d|--debug}\0";
pub const HELP_SPEC_MAP: &[u8] = b"MAP := { id MAP_ID | pinned FILE | name MAP_NAME }\0";
pub const HELP_SPEC_LINK: &[u8] = b"LINK := { id LINK_ID | pinned FILE }\0";

/* keep in sync with the definition in skeleton/pid_iter.bpf.c */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bpf_obj_type {
    BPF_OBJ_UNKNOWN,
    BPF_OBJ_PROG,
    BPF_OBJ_MAP,
    BPF_OBJ_LINK,
    BPF_OBJ_BTF,
}

unsafe extern "C" {
    pub static bin_name: *const c_char;

    pub static mut json_wtr: *mut json_writer_t;
    pub static mut json_output: bool;
    pub static mut show_pinned: bool;
    pub static mut show_pids: bool;
    pub static mut block_mount: bool;
    pub static mut verifier_logs: bool;
    pub static mut relaxed_maps: bool;
    pub static mut use_loader: bool;
    pub static mut base_btf: *mut btf;
    pub static mut refs_table: *mut hashmap;
    pub static mut sign_progs: bool;
    pub static mut private_key_path: *const c_char;
    pub static mut cert_path: *const c_char;

    pub fn p_err(fmt: *const c_char, ...);
    pub fn p_info(fmt: *const c_char, ...);

    pub fn is_prefix(pfx: *const c_char, str_: *const c_char) -> bool;
    pub fn detect_common_prefix(arg: *const c_char, ...) -> c_int;
    pub fn fprint_hex(f: *mut FILE, arg: *mut c_void, n: c_uint, sep: *const c_char);
    pub fn usage() -> !;

    pub fn set_max_rlimit();

    pub fn mount_tracefs(target: *const c_char) -> c_int;
}

#[repr(C)]
pub struct obj_ref {
    pub pid: c_int,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct obj_refs {
    pub ref_cnt: c_int,
    pub has_bpf_cookie: bool,
    pub refs: *mut obj_ref,
    pub bpf_cookie: __u64,
}

unsafe extern "C" {
    pub fn build_pinned_obj_table(table: *mut hashmap, type_: bpf_obj_type) -> c_int;
    pub fn delete_pinned_obj_table(table: *mut hashmap);
    /* __weak in C */
    pub fn build_obj_refs_table(table: *mut *mut hashmap, type_: bpf_obj_type) -> c_int;
    /* __weak in C */
    pub fn delete_obj_refs_table(table: *mut hashmap);
    /* __weak in C */
    pub fn emit_obj_refs_json(table: *mut hashmap, id: __u32, json_wtr: *mut json_writer_t);
    /* __weak in C */
    pub fn emit_obj_refs_plain(table: *mut hashmap, id: __u32, prefix: *const c_char);
    pub fn print_dev_plain(ifindex: __u32, ns_dev: __u64, ns_inode: __u64);
    pub fn print_dev_json(ifindex: __u32, ns_dev: __u64, ns_inode: __u64);
}

#[repr(C)]
pub struct cmd {
    pub cmd: *const c_char,
    pub func: Option<unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int>,
}

unsafe extern "C" {
    pub fn cmd_select(
        cmds: *const cmd,
        argc: c_int,
        argv: *mut *mut c_char,
        help: Option<unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int>,
    ) -> c_int;
}

pub const MAX_PROG_FULL_NAME: c_int = 128;

unsafe extern "C" {
    pub fn get_prog_full_name(
        prog_info: *const bpf_prog_info,
        prog_fd: c_int,
        name_buff: *mut c_char,
        buff_len: size_t,
    );

    pub fn get_fd_type(fd: c_int) -> c_int;
    pub fn get_fd_type_name(type_: bpf_obj_type) -> *const c_char;
    pub fn get_fdinfo(fd: c_int, key: *const c_char) -> *mut c_char;
    pub fn open_obj_pinned(
        path: *const c_char,
        quiet: bool,
        opts: *const bpf_obj_get_opts,
    ) -> c_int;
    pub fn open_obj_pinned_any(
        path: *const c_char,
        exp_type: bpf_obj_type,
        opts: *const bpf_obj_get_opts,
    ) -> c_int;
    pub fn mount_bpffs_for_file(file_name: *const c_char) -> c_int;
    pub fn create_and_mount_bpffs_dir(dir_name: *const c_char) -> c_int;
    pub fn do_pin_any(
        argc: c_int,
        argv: *mut *mut c_char,
        get_fd_by_id: Option<
            unsafe extern "C" fn(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int,
        >,
    ) -> c_int;
    pub fn do_pin_fd(fd: c_int, name: *const c_char) -> c_int;

    /* commands available in bootstrap mode */
    pub fn do_gen(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn do_btf(argc: c_int, argv: *mut *mut c_char) -> c_int;

    /* non-bootstrap only commands; __weak in C */
    pub fn do_prog(argc: c_int, arg: *mut *mut c_char) -> c_int;
    pub fn do_map(argc: c_int, arg: *mut *mut c_char) -> c_int;
    pub fn do_link(argc: c_int, arg: *mut *mut c_char) -> c_int;
    pub fn do_event_pipe(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn do_cgroup(argc: c_int, arg: *mut *mut c_char) -> c_int;
    pub fn do_perf(argc: c_int, arg: *mut *mut c_char) -> c_int;
    pub fn do_net(argc: c_int, arg: *mut *mut c_char) -> c_int;
    pub fn do_tracelog(argc: c_int, arg: *mut *mut c_char) -> c_int;
    pub fn do_feature(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn do_struct_ops(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn do_iter(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn do_token(argc: c_int, argv: *mut *mut c_char) -> c_int;

    pub fn parse_u32_arg(
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        val: *mut __u32,
        what: *const c_char,
    ) -> c_int;
    pub fn prog_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int;
    pub fn prog_parse_fds(
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        fds: *mut *mut c_int,
    ) -> c_int;
    pub fn map_parse_fd(
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        open_flags: __u32,
    ) -> c_int;
    pub fn map_parse_fds(
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        fds: *mut *mut c_int,
        open_flags: __u32,
    ) -> c_int;
    pub fn map_parse_fd_and_info(
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        info: *mut bpf_map_info,
        info_len: *mut __u32,
        open_flags: __u32,
    ) -> c_int;
}

/* In C this declaration is conditional:
 * #if defined(HAVE_LLVM_SUPPORT) || defined(HAVE_LIBBFD_SUPPORT)
 *   extern declarations are supplied by another object;
 * #else
 *   inline fallback definitions return 0 and -1 after p_err().
 * The Rust translation exposes the declarations and preserves fallback intent
 * below as disabled code because the build-time condition is external.
 */
unsafe extern "C" {
    pub fn disasm_print_insn(
        image: *mut c_uchar,
        len: ssize_t,
        opcodes: c_int,
        arch: *const c_char,
        disassembler_options: *const c_char,
        btf: *const btf,
        prog_linfo: *const bpf_prog_linfo,
        func_ksym: __u64,
        func_idx: c_uint,
        linum: bool,
    ) -> c_int;
    pub fn disasm_init() -> c_int;
}

#[cfg(any())]
#[inline]
pub unsafe fn disasm_print_insn_fallback(
    image: *mut c_uchar,
    len: ssize_t,
    opcodes: c_int,
    arch: *const c_char,
    disassembler_options: *const c_char,
    btf: *const btf,
    prog_linfo: *const bpf_prog_linfo,
    func_ksym: __u64,
    func_idx: c_uint,
    linum: bool,
) -> c_int {
    let _ = (
        image,
        len,
        opcodes,
        arch,
        disassembler_options,
        btf,
        prog_linfo,
        func_ksym,
        func_idx,
        linum,
    );
    0
}

#[cfg(any())]
#[inline]
pub unsafe fn disasm_init_fallback() -> c_int {
    p_err(b"No JIT disassembly support\0".as_ptr() as *const c_char);
    -1
}

unsafe extern "C" {
    pub fn print_data_json(data: *mut u8, len: size_t);
    pub fn print_hex_data_json(data: *mut u8, len: size_t);

    pub fn get_page_size() -> c_uint;
    pub fn get_possible_cpus() -> c_uint;
    pub fn ifindex_to_arch(
        ifindex: __u32,
        ns_dev: __u64,
        ns_ino: __u64,
        opt: *mut *const c_char,
    ) -> *const c_char;
}

#[repr(C)]
pub struct btf_dumper {
    pub btf: *const btf,
    pub jw: *mut json_writer_t,
    pub is_plain_text: bool,
    pub prog_id_as_func_ptr: bool,
}

/* btf_dumper_type - print data along with type information
 * @d: an instance containing context for dumping types
 * @type_id: index in btf->types array. this points to the type to be dumped
 * @data: pointer the actual data, i.e. the values to be printed
 *
 * Returns zero on success and negative error code otherwise
 */
unsafe extern "C" {
    pub fn btf_dumper_type(d: *const btf_dumper, type_id: __u32, data: *const c_void) -> c_int;
    pub fn btf_dumper_type_only(
        btf: *const btf,
        func_type_id: __u32,
        func_only: *mut c_char,
        size: c_int,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
