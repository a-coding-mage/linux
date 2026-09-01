// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2021 Facebook */

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const MAX_USED_MAPS: usize = 64;
const MAX_USED_PROGS: usize = 32;
const MAX_KFUNC_DESCS: usize = 256;
const MAX_FD_ARRAY_SZ: usize = MAX_USED_MAPS + MAX_KFUNC_DESCS;

/* The following structure describes the stack layout of the loader program.
 * In addition R6 contains the pointer to context.
 * R7 contains the result of the last sys_bpf command (typically error or FD).
 * R9 contains the result of the last sys_close command.
 *
 * Naming convention:
 * ctx - bpf program context
 * stack - bpf program stack
 * blob - bpf_attr-s, strings, insns, map data.
 *        All the bytes that loader prog will use for read/write.
 */
#[repr(C)]
struct loader_stack {
    btf_fd: __u32,
    inner_map_fd: __u32,
    prog_fd: [__u32; MAX_USED_PROGS],
}

type __u8 = u8;
type __u16 = u16;
type __s16 = i16;
type __u32 = u32;
type __s32 = i32;
type __u64 = u64;
type __s64 = i64;

const ERANGE: c_int = 34;
const ENOMEM: c_int = 12;
const E2BIG: c_int = 7;
const EFAULT: c_int = 14;
const ENOSPC: c_int = 28;
const EDOM: c_int = 33;
const ENOENT: c_int = 2;
const INT16_MAX_: c_int = i16::MAX as c_int;
const INT32_MAX_: usize = i32::MAX as usize;

#[allow(non_camel_case_types)]
type size_t = usize;

extern "C" {
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn vsnprintf(s: *mut c_char, n: size_t, fmt: *const c_char, args: va_list) -> c_int;

    fn libbpf_reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn libbpf_strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn btf_get_kernel_prefix_kind(
        ty: bpf_attach_type,
        prefix: *mut *const c_char,
        kind: *mut c_int,
    );
    fn bpf_insn_bswap(insn: *mut bpf_insn);
    fn bpf_func_info_bswap(fi: *mut bpf_func_info);
    fn bpf_line_info_bswap(li: *mut bpf_line_info);
    fn bpf_core_relo_bswap(cr: *mut bpf_core_relo);
    fn errstr(err: c_int) -> *const c_char;
    fn bswap_16(v: __u16) -> __u16;
    fn bswap_32(v: __u32) -> __u32;
    fn bswap_64(v: __u64) -> __u64;
}

type va_list = *mut c_void;

extern "C" {
    fn va_start(args: *mut va_list, fmt: *const c_char);
    fn va_end(args: va_list);
}

extern "C" {
    static BPF_REG_0: c_int;
    static BPF_REG_1: c_int;
    static BPF_REG_2: c_int;
    static BPF_REG_3: c_int;
    static BPF_REG_4: c_int;
    static BPF_REG_6: c_int;
    static BPF_REG_7: c_int;
    static BPF_REG_8: c_int;
    static BPF_REG_9: c_int;
    static BPF_REG_10: c_int;

    static BPF_W: c_int;
    static BPF_H: c_int;
    static BPF_B: c_int;
    static BPF_DW: c_int;
    static BPF_ADD: c_int;
    static BPF_RSH: c_int;
    static BPF_AND: c_int;
    static BPF_JA: c_int;
    static BPF_JEQ: c_int;
    static BPF_JNE: c_int;
    static BPF_JSET: c_int;
    static BPF_JSLT: c_int;
    static BPF_JSLE: c_int;
    static BPF_JSGE: c_int;
    static BPF_PSEUDO_MAP_IDX_VALUE: c_int;
    static BPF_PSEUDO_BTF_ID: c_int;
    static BPF_SKEL_KERNEL: c_int;

    static BPF_FUNC_probe_read_kernel: c_int;
    static BPF_FUNC_sys_close: c_int;
    static BPF_FUNC_sys_bpf: c_int;
    static BPF_FUNC_trace_printk: c_int;
    static BPF_FUNC_btf_find_by_name_kind: c_int;
    static BPF_FUNC_kallsyms_lookup_name: c_int;
    static BPF_FUNC_copy_from_user: c_int;

    static BPF_BTF_LOAD: c_int;
    static BPF_MAP_CREATE: c_int;
    static BPF_PROG_LOAD: c_int;
    static BPF_MAP_UPDATE_ELEM: c_int;
    static BPF_MAP_FREEZE: c_int;
    static BPF_MAP_TYPE_ARRAY_OF_MAPS: bpf_map_type;
    static BPF_MAP_TYPE_HASH_OF_MAPS: bpf_map_type;
}

extern "C" {
    fn BPF_MOV64_REG(dst: c_int, src: c_int) -> bpf_insn;
    fn BPF_MOV64_IMM(dst: c_int, imm: c_int) -> bpf_insn;
    fn BPF_ALU64_IMM(op: c_int, dst: c_int, imm: c_int) -> bpf_insn;
    fn BPF_ALU32_IMM(op: c_int, dst: c_int, imm: c_int) -> bpf_insn;
    fn BPF_EMIT_CALL(func: c_int) -> bpf_insn;
    fn BPF_JMP_IMM(op: c_int, dst: c_int, imm: c_int, off: c_int) -> bpf_insn;
    fn BPF_LDX_MEM(sz: c_int, dst: c_int, src: c_int, off: c_int) -> bpf_insn;
    fn BPF_STX_MEM(sz: c_int, dst: c_int, src: c_int, off: c_int) -> bpf_insn;
    fn BPF_ST_MEM(sz: c_int, dst: c_int, off: c_int, imm: c_int) -> bpf_insn;
    fn BPF_EXIT_INSN() -> bpf_insn;
    fn BPF_LD_IMM64_RAW_FULL(
        dst: c_int,
        src: c_int,
        off1: c_int,
        off2: c_int,
        imm1: c_int,
        imm2: c_int,
    ) -> bpf_insn_pair;
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct bpf_insn {
    code: __u8,
    regs: __u8,
    off: __s16,
    imm: __s32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct bpf_insn_pair {
    insn: [bpf_insn; 2],
}

#[repr(C)]
struct gen_loader_opts {
    insns: *mut c_void,
    insns_sz: size_t,
    data: *mut c_void,
    data_sz: size_t,
    gen_hash: bool,
}

#[repr(C)]
struct bpf_gen {
    fd_array: c_int,
    log_level: c_int,
    error: c_int,
    insn_start: *mut c_void,
    insn_cur: *mut c_void,
    data_start: *mut c_void,
    data_cur: *mut c_void,
    cleanup_label: isize,
    nr_maps: c_int,
    nr_fd_array: c_int,
    nr_progs: c_int,
    opts: *mut gen_loader_opts,
    swapped_endian: bool,
    attach_kind: c_int,
    attach_target: [c_char; 128],
    relos: *mut ksym_relo_desc,
    relo_cnt: c_int,
    ksyms: *mut ksym_desc,
    nr_ksyms: c_int,
    core_relos: *mut bpf_core_relo,
    core_relo_cnt: c_int,
}

#[repr(C)]
struct ksym_relo_desc {
    name: *const c_char,
    is_weak: bool,
    is_typeless: bool,
    is_ld64: bool,
    kind: c_int,
    insn_idx: c_int,
}

#[repr(C)]
struct ksym_desc {
    name: *const c_char,
    kind: c_int,
    ref_: c_int,
    off: c_int,
    insn: c_int,
    is_ld64: bool,
    typeless: bool,
}

#[repr(C)]
struct bpf_loader_ctx {
    log_level: __u32,
    log_size: __u32,
    log_buf: __u64,
    flags: __u32,
}

#[repr(C)]
struct bpf_map_desc {
    map_fd: __u32,
    max_entries: __u32,
    initial_value: __u64,
}

#[repr(C)]
struct bpf_prog_desc {
    prog_fd: __u32,
}

#[repr(C)]
struct bpf_map_create_opts {
    map_flags: __u32,
    map_extra: __u64,
    numa_node: __u32,
    map_ifindex: __u32,
    btf_key_type_id: __u32,
    btf_value_type_id: __u32,
}

#[repr(C)]
struct bpf_prog_load_opts {
    expected_attach_type: bpf_attach_type,
    attach_btf_id: __u32,
    prog_ifindex: __u32,
    prog_flags: __u32,
    func_info_rec_size: __u32,
    func_info_cnt: __u32,
    func_info: *const c_void,
    line_info_rec_size: __u32,
    line_info_cnt: __u32,
    line_info: *const c_void,
}

#[repr(C)]
struct bpf_func_info {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_line_info {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_core_relo {
    _private: [u8; 0],
}

type bpf_map_type = c_int;
type bpf_prog_type = c_int;
type bpf_attach_type = c_int;

#[repr(C)]
union bpf_attr {
    raw: [u8; 256],
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! emit_ld_imm64 {
    ($gen:expr, $dst:expr, $src:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {{
        let pair = BPF_LD_IMM64_RAW_FULL($dst, $src, $a, $b, $c, $d);
        emit2($gen, pair.insn[0], pair.insn[1]);
    }};
}

const fn offsetof_loader_stack_btf_fd() -> usize {
    0
}
const fn offsetof_loader_stack_inner_map_fd() -> usize {
    size_of::<__u32>()
}
const fn offsetof_loader_stack_prog_fd() -> usize {
    size_of::<__u32>() * 2
}
const fn stack_off_btf_fd() -> c_int {
    -(size_of::<loader_stack>() as c_int) + offsetof_loader_stack_btf_fd() as c_int
}
const fn stack_off_inner_map_fd() -> c_int {
    -(size_of::<loader_stack>() as c_int) + offsetof_loader_stack_inner_map_fd() as c_int
}
const fn stack_off_prog_fd(i: c_int) -> c_int {
    -(size_of::<loader_stack>() as c_int)
        + offsetof_loader_stack_prog_fd() as c_int
        + i * size_of::<__u32>() as c_int
}

const fn offsetof_bpf_insn_code() -> usize {
    0
}
const fn offsetof_bpf_insn_off() -> usize {
    2
}
const fn offsetof_bpf_insn_imm() -> usize {
    4
}
const fn offsetofend_bpf_insn_code() -> usize {
    offsetof_bpf_insn_code() + size_of::<__u8>()
}

const fn offsetof_bpf_loader_ctx_log_level() -> usize {
    0
}
const fn offsetof_bpf_loader_ctx_log_size() -> usize {
    4
}
const fn offsetof_bpf_loader_ctx_log_buf() -> usize {
    8
}
const fn offsetof_bpf_loader_ctx_flags() -> usize {
    16
}

const fn offsetof_bpf_map_desc_map_fd() -> usize {
    0
}
const fn offsetof_bpf_map_desc_max_entries() -> usize {
    4
}
const fn offsetof_bpf_map_desc_initial_value() -> usize {
    8
}
const fn offsetof_bpf_prog_desc_prog_fd() -> usize {
    0
}

/* bpf_attr field offsets are supplied by the original C dependency layout. */
extern "C" {
    static offsetof_bpf_attr_btf_log_level: c_int;
    static offsetofend_bpf_attr_btf_log_level: c_int;
    static offsetof_bpf_attr_btf_log_size: c_int;
    static offsetof_bpf_attr_btf_log_buf: c_int;
    static offsetof_bpf_attr_btf: c_int;
    static offsetof_bpf_attr_btf_size: c_int;
    static offsetofend_bpf_attr_map_extra: c_int;
    static offsetof_bpf_attr_map_type: c_int;
    static offsetof_bpf_attr_key_size: c_int;
    static offsetof_bpf_attr_value_size: c_int;
    static offsetof_bpf_attr_map_flags: c_int;
    static offsetof_bpf_attr_map_extra: c_int;
    static offsetof_bpf_attr_map_name: c_int;
    static offsetof_bpf_attr_numa_node: c_int;
    static offsetof_bpf_attr_map_ifindex: c_int;
    static offsetof_bpf_attr_max_entries: c_int;
    static offsetof_bpf_attr_btf_key_type_id: c_int;
    static offsetof_bpf_attr_btf_value_type_id: c_int;
    static offsetof_bpf_attr_btf_fd: c_int;
    static offsetof_bpf_attr_inner_map_fd: c_int;
    static offsetofend_bpf_attr_core_relo_rec_size: c_int;
    static offsetof_bpf_attr_license: c_int;
    static offsetof_bpf_attr_insns: c_int;
    static offsetof_bpf_attr_func_info: c_int;
    static offsetof_bpf_attr_line_info: c_int;
    static offsetof_bpf_attr_core_relos: c_int;
    static offsetof_bpf_attr_fd_array: c_int;
    static offsetof_bpf_attr_log_level: c_int;
    static offsetof_bpf_attr_log_size: c_int;
    static offsetof_bpf_attr_log_buf: c_int;
    static offsetof_bpf_attr_prog_btf_fd: c_int;
    static offsetof_bpf_attr_attach_btf_id: c_int;
    static offsetof_bpf_attr_attach_btf_obj_fd: c_int;
    static offsetof_bpf_attr_map_fd: c_int;
    static offsetofend_bpf_attr_flags: c_int;
    static offsetofend_bpf_attr_map_fd: c_int;
    static offsetof_bpf_attr_flags: c_int;
    static offsetof_bpf_attr_key: c_int;
    static offsetof_bpf_attr_value: c_int;
    static offsetof_bpf_attr_prog_name: c_int;
    static offsetof_bpf_attr_prog_type: c_int;
    static offsetof_bpf_attr_expected_attach_type: c_int;
    static offsetof_bpf_attr_prog_ifindex: c_int;
    static offsetof_bpf_attr_kern_version: c_int;
    static offsetof_bpf_attr_insn_cnt: c_int;
    static offsetof_bpf_attr_prog_flags: c_int;
    static offsetof_bpf_attr_func_info_rec_size: c_int;
    static offsetof_bpf_attr_func_info_cnt: c_int;
    static offsetof_bpf_attr_line_info_rec_size: c_int;
    static offsetof_bpf_attr_line_info_cnt: c_int;
    static offsetof_bpf_attr_core_relo_rec_size: c_int;
    static offsetof_bpf_attr_core_relo_cnt: c_int;
}

unsafe fn attr_field(attr: c_int, field: c_int) -> c_int {
    attr + field
}

fn roundup(x: __u32, y: __u32) -> __u32 {
    ((x + y - 1) / y) * y
}

unsafe fn opts_get_gen_hash(opts: *mut gen_loader_opts, default_value: bool) -> bool {
    if opts.is_null() {
        default_value
    } else {
        (*opts).gen_hash
    }
}

unsafe fn tgt_endian_u16(gen: *mut bpf_gen, rval: __u16) -> __u16 {
    let mut val = rval;
    if (*gen).swapped_endian {
        val = bswap_16(val);
    }
    val
}

unsafe fn tgt_endian_u32(gen: *mut bpf_gen, rval: __u32) -> __u32 {
    let mut val = rval;
    if (*gen).swapped_endian {
        val = bswap_32(val);
    }
    val
}

unsafe fn tgt_endian_u64(gen: *mut bpf_gen, rval: __u64) -> __u64 {
    let mut val = rval;
    if (*gen).swapped_endian {
        val = bswap_64(val);
    }
    val
}

unsafe fn blob_fd_array_off(gen: *mut bpf_gen, index: c_int) -> c_int {
    (*gen).fd_array + index * size_of::<c_int>() as c_int
}

unsafe fn realloc_insn_buf(gen: *mut bpf_gen, size: __u32) -> c_int {
    let off = ((*gen).insn_cur as usize).wrapping_sub((*gen).insn_start as usize);
    let mut insn_start: *mut c_void;

    if (*gen).error != 0 {
        return (*gen).error;
    }
    if size as usize > INT32_MAX_ || off + size as usize > INT32_MAX_ {
        (*gen).error = -ERANGE;
        return -ERANGE;
    }
    insn_start = realloc((*gen).insn_start, off + size as usize);
    if insn_start.is_null() {
        (*gen).error = -ENOMEM;
        free((*gen).insn_start);
        (*gen).insn_start = ptr::null_mut();
        (*gen).insn_cur = ptr::null_mut();
        return -ENOMEM;
    }
    (*gen).insn_start = insn_start;
    (*gen).insn_cur = (insn_start as *mut u8).add(off) as *mut c_void;
    0
}

unsafe fn realloc_data_buf(gen: *mut bpf_gen, size: __u32) -> c_int {
    let off = ((*gen).data_cur as usize).wrapping_sub((*gen).data_start as usize);
    let mut data_start: *mut c_void;

    if (*gen).error != 0 {
        return (*gen).error;
    }
    if size as usize > INT32_MAX_ || off + size as usize > INT32_MAX_ {
        (*gen).error = -ERANGE;
        return -ERANGE;
    }
    data_start = realloc((*gen).data_start, off + size as usize);
    if data_start.is_null() {
        (*gen).error = -ENOMEM;
        free((*gen).data_start);
        (*gen).data_start = ptr::null_mut();
        (*gen).data_cur = ptr::null_mut();
        return -ENOMEM;
    }
    (*gen).data_start = data_start;
    (*gen).data_cur = (data_start as *mut u8).add(off) as *mut c_void;
    0
}

unsafe fn emit(gen: *mut bpf_gen, insn: bpf_insn) {
    if realloc_insn_buf(gen, size_of::<bpf_insn>() as __u32) != 0 {
        return;
    }
    memcpy(
        (*gen).insn_cur,
        &insn as *const _ as *const c_void,
        size_of::<bpf_insn>(),
    );
    (*gen).insn_cur = ((*gen).insn_cur as *mut u8).add(size_of::<bpf_insn>()) as *mut c_void;
}

unsafe fn emit2(gen: *mut bpf_gen, insn1: bpf_insn, insn2: bpf_insn) {
    emit(gen, insn1);
    emit(gen, insn2);
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__init(
    gen: *mut bpf_gen,
    log_level: c_int,
    nr_progs: c_int,
    nr_maps: c_int,
) {
    let stack_sz = size_of::<loader_stack>();
    let nr_progs_sz: usize;
    let mut i: c_int;

    (*gen).fd_array = add_data(gen, ptr::null(), (MAX_FD_ARRAY_SZ * size_of::<c_int>()) as __u32);
    (*gen).log_level = log_level;
    /* save ctx pointer into R6 */
    emit(gen, BPF_MOV64_REG(BPF_REG_6, BPF_REG_1));

    /* bzero stack */
    emit(gen, BPF_MOV64_REG(BPF_REG_1, BPF_REG_10));
    emit(gen, BPF_ALU64_IMM(BPF_ADD, BPF_REG_1, -(stack_sz as c_int)));
    emit(gen, BPF_MOV64_IMM(BPF_REG_2, stack_sz as c_int));
    emit(gen, BPF_MOV64_IMM(BPF_REG_3, 0));
    emit(gen, BPF_EMIT_CALL(BPF_FUNC_probe_read_kernel));

    /* amount of stack actually used, only used to calculate iterations, not stack offset */
    nr_progs_sz = offsetof_loader_stack_prog_fd() + nr_progs as usize * size_of::<__u32>();
    /* jump over cleanup code */
    emit(
        gen,
        BPF_JMP_IMM(
            BPF_JA,
            0,
            0,
            ((nr_progs_sz / 4) * 3 + 2
                + nr_maps as usize * (6 + if (*gen).log_level != 0 { 6 } else { 0 }))
                as c_int,
        ),
    );

    /* remember the label where all error branches will jump to */
    (*gen).cleanup_label = ((*gen).insn_cur as isize) - ((*gen).insn_start as isize);
    /* emit cleanup code: close all temp FDs */
    i = 0;
    while (i as usize) < nr_progs_sz {
        emit(gen, BPF_LDX_MEM(BPF_W, BPF_REG_1, BPF_REG_10, -(stack_sz as c_int) + i));
        emit(gen, BPF_JMP_IMM(BPF_JSLE, BPF_REG_1, 0, 1));
        emit(gen, BPF_EMIT_CALL(BPF_FUNC_sys_close));
        i += 4;
    }
    i = 0;
    while i < nr_maps {
        emit_sys_close_blob(gen, blob_fd_array_off(gen, i));
        i += 1;
    }
    /* R7 contains the error code from sys_bpf. Copy it into R0 and exit. */
    emit(gen, BPF_MOV64_REG(BPF_REG_0, BPF_REG_7));
    emit(gen, BPF_EXIT_INSN());
}

unsafe fn add_data(gen: *mut bpf_gen, data: *const c_void, size: __u32) -> c_int {
    let zero: __u64 = 0;
    let size8: __u32;
    let prev: *mut c_void;

    if size as usize > INT32_MAX_ {
        (*gen).error = -ERANGE;
        return 0;
    }
    size8 = roundup(size, 8);

    if realloc_data_buf(gen, size8) != 0 {
        return 0;
    }
    prev = (*gen).data_cur;
    if !data.is_null() {
        memcpy((*gen).data_cur, data, size as usize);
        memcpy(
            ((*gen).data_cur as *mut u8).add(size as usize) as *mut c_void,
            &zero as *const _ as *const c_void,
            (size8 - size) as usize,
        );
    } else {
        memset((*gen).data_cur, 0, size8 as usize);
    }
    (*gen).data_cur = ((*gen).data_cur as *mut u8).add(size8 as usize) as *mut c_void;
    (prev as isize - (*gen).data_start as isize) as c_int
}

/* Get index for map_fd/btf_fd slot in reserved fd_array, or in data relative
 * to start of fd_array. Caller can decide if it is usable or not.
 */
unsafe fn add_map_fd(gen: *mut bpf_gen) -> c_int {
    if (*gen).nr_maps == MAX_USED_MAPS as c_int {
        pr_warn(cstr!("Total maps exceeds %d\n"), MAX_USED_MAPS as c_int);
        (*gen).error = -E2BIG;
        return 0;
    }
    let cur = (*gen).nr_maps;
    (*gen).nr_maps += 1;
    cur
}

unsafe fn add_kfunc_btf_fd(gen: *mut bpf_gen) -> c_int {
    let cur: c_int;

    if (*gen).nr_fd_array == MAX_KFUNC_DESCS as c_int {
        cur = add_data(gen, ptr::null(), size_of::<c_int>() as __u32);
        return (cur - (*gen).fd_array) / size_of::<c_int>() as c_int;
    }
    let ret = MAX_USED_MAPS as c_int + (*gen).nr_fd_array;
    (*gen).nr_fd_array += 1;
    ret
}

fn insn_bytes_to_bpf_size(sz: __u32) -> c_int {
    unsafe {
        match sz {
            8 => BPF_DW,
            4 => BPF_W,
            2 => BPF_H,
            1 => BPF_B,
            _ => -1,
        }
    }
}

/* *(u64 *)(blob + off) = (u64)(void *)(blob + data) */
unsafe fn emit_rel_store(gen: *mut bpf_gen, off: c_int, data: c_int) {
    emit_ld_imm64!(gen, BPF_REG_0, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, data);
    emit_ld_imm64!(gen, BPF_REG_1, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, off);
    emit(gen, BPF_STX_MEM(BPF_DW, BPF_REG_1, BPF_REG_0, 0));
}

unsafe fn move_blob2blob(gen: *mut bpf_gen, off: c_int, size: c_int, blob_off: c_int) {
    emit_ld_imm64!(gen, BPF_REG_2, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, blob_off);
    emit(gen, BPF_LDX_MEM(insn_bytes_to_bpf_size(size as __u32), BPF_REG_0, BPF_REG_2, 0));
    emit_ld_imm64!(gen, BPF_REG_1, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, off);
    emit(gen, BPF_STX_MEM(insn_bytes_to_bpf_size(size as __u32), BPF_REG_1, BPF_REG_0, 0));
}

unsafe fn move_blob2ctx(gen: *mut bpf_gen, ctx_off: c_int, size: c_int, blob_off: c_int) {
    emit_ld_imm64!(gen, BPF_REG_1, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, blob_off);
    emit(gen, BPF_LDX_MEM(insn_bytes_to_bpf_size(size as __u32), BPF_REG_0, BPF_REG_1, 0));
    emit(gen, BPF_STX_MEM(insn_bytes_to_bpf_size(size as __u32), BPF_REG_6, BPF_REG_0, ctx_off));
}

unsafe fn move_ctx2blob(
    gen: *mut bpf_gen,
    off: c_int,
    size: c_int,
    ctx_off: c_int,
    check_non_zero: bool,
) {
    emit(gen, BPF_LDX_MEM(insn_bytes_to_bpf_size(size as __u32), BPF_REG_0, BPF_REG_6, ctx_off));
    if check_non_zero {
        /* If value in ctx is zero don't update the blob.
         * For example: when ctx->map.max_entries == 0, keep default max_entries from bpf.c
         */
        emit(gen, BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 0, 3));
    }
    emit_ld_imm64!(gen, BPF_REG_1, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, off);
    emit(gen, BPF_STX_MEM(insn_bytes_to_bpf_size(size as __u32), BPF_REG_1, BPF_REG_0, 0));
}

unsafe fn move_stack2blob(gen: *mut bpf_gen, off: c_int, size: c_int, stack_off: c_int) {
    emit(gen, BPF_LDX_MEM(insn_bytes_to_bpf_size(size as __u32), BPF_REG_0, BPF_REG_10, stack_off));
    emit_ld_imm64!(gen, BPF_REG_1, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, off);
    emit(gen, BPF_STX_MEM(insn_bytes_to_bpf_size(size as __u32), BPF_REG_1, BPF_REG_0, 0));
}

unsafe fn move_stack2ctx(gen: *mut bpf_gen, ctx_off: c_int, size: c_int, stack_off: c_int) {
    emit(gen, BPF_LDX_MEM(insn_bytes_to_bpf_size(size as __u32), BPF_REG_0, BPF_REG_10, stack_off));
    emit(gen, BPF_STX_MEM(insn_bytes_to_bpf_size(size as __u32), BPF_REG_6, BPF_REG_0, ctx_off));
}

unsafe fn emit_sys_bpf(gen: *mut bpf_gen, cmd: c_int, attr: c_int, attr_size: c_int) {
    emit(gen, BPF_MOV64_IMM(BPF_REG_1, cmd));
    emit_ld_imm64!(gen, BPF_REG_2, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, attr);
    emit(gen, BPF_MOV64_IMM(BPF_REG_3, attr_size));
    emit(gen, BPF_EMIT_CALL(BPF_FUNC_sys_bpf));
    /* remember the result in R7 */
    emit(gen, BPF_MOV64_REG(BPF_REG_7, BPF_REG_0));
}

fn is_simm16(value: __s64) -> bool {
    value == value as __s16 as __s64
}

unsafe fn emit_check_err(gen: *mut bpf_gen) {
    let off: __s64 = -(((*gen).insn_cur as isize - (*gen).insn_start as isize - (*gen).cleanup_label)
        / 8) as __s64
        - 1;

    /* R7 contains result of last sys_bpf command.
     * if (R7 < 0) goto cleanup;
     */
    if is_simm16(off) {
        emit(gen, BPF_JMP_IMM(BPF_JSLT, BPF_REG_7, 0, off as c_int));
    } else {
        (*gen).error = -ERANGE;
    }
}

/* reg1 and reg2 should not be R1 - R5. They can be R0, R6 - R10 */
unsafe fn emit_debug(
    gen: *mut bpf_gen,
    reg1: c_int,
    reg2: c_int,
    fmt: *const c_char,
    args: va_list,
) {
    let mut buf = [0 as c_char; 1024];
    let addr: c_int;
    let len: c_int;
    let ret: c_int;

    if (*gen).log_level == 0 {
        return;
    }
    ret = vsnprintf(buf.as_mut_ptr(), buf.len(), fmt, args);
    if ret < 1024 - 7 && reg1 >= 0 && reg2 < 0 {
        /* The special case to accommodate common debug_ret():
         * to avoid specifying BPF_REG_7 and adding " r=%%d" to
         * prints explicitly.
         */
        strcat(buf.as_mut_ptr(), cstr!(" r=%d"));
    }
    len = strlen(buf.as_ptr()) as c_int + 1;
    addr = add_data(gen, buf.as_ptr() as *const c_void, len as __u32);

    emit_ld_imm64!(gen, BPF_REG_1, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, addr);
    emit(gen, BPF_MOV64_IMM(BPF_REG_2, len));
    if reg1 >= 0 {
        emit(gen, BPF_MOV64_REG(BPF_REG_3, reg1));
    }
    if reg2 >= 0 {
        emit(gen, BPF_MOV64_REG(BPF_REG_4, reg2));
    }
    emit(gen, BPF_EMIT_CALL(BPF_FUNC_trace_printk));
}

unsafe fn debug_regs(gen: *mut bpf_gen, reg1: c_int, reg2: c_int, fmt: *const c_char, mut args: ...) {
    let mut ap: va_list = ptr::null_mut();
    va_start(&mut ap, fmt);
    emit_debug(gen, reg1, reg2, fmt, ap);
    va_end(ap);
}

unsafe fn debug_ret(gen: *mut bpf_gen, fmt: *const c_char, mut args: ...) {
    let mut ap: va_list = ptr::null_mut();
    va_start(&mut ap, fmt);
    emit_debug(gen, BPF_REG_7, -1, fmt, ap);
    va_end(ap);
}

unsafe fn __emit_sys_close(gen: *mut bpf_gen) {
    emit(
        gen,
        BPF_JMP_IMM(
            BPF_JSLE,
            BPF_REG_1,
            0,
            2 + if (*gen).log_level != 0 { 6 } else { 0 },
        ),
    );
    emit(gen, BPF_MOV64_REG(BPF_REG_9, BPF_REG_1));
    emit(gen, BPF_EMIT_CALL(BPF_FUNC_sys_close));
    debug_regs(gen, BPF_REG_9, BPF_REG_0, cstr!("close(%%d) = %%d"));
}

unsafe fn emit_sys_close_stack(gen: *mut bpf_gen, stack_off: c_int) {
    emit(gen, BPF_LDX_MEM(BPF_W, BPF_REG_1, BPF_REG_10, stack_off));
    __emit_sys_close(gen);
}

unsafe fn emit_sys_close_blob(gen: *mut bpf_gen, blob_off: c_int) {
    emit_ld_imm64!(gen, BPF_REG_0, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, blob_off);
    emit(gen, BPF_LDX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, 0));
    __emit_sys_close(gen);
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__finish(
    gen: *mut bpf_gen,
    nr_progs: c_int,
    nr_maps: c_int,
) -> c_int {
    let mut i: c_int;

    if nr_progs < (*gen).nr_progs || nr_maps != (*gen).nr_maps {
        pr_warn(
            cstr!("nr_progs %d/%u nr_maps %d/%u mismatch\n"),
            nr_progs,
            (*gen).nr_progs as __u32,
            nr_maps,
            (*gen).nr_maps as __u32,
        );
        (*gen).error = -EFAULT;
        return (*gen).error;
    }
    emit_sys_close_stack(gen, stack_off_btf_fd());
    i = 0;
    while i < (*gen).nr_progs {
        move_stack2ctx(
            gen,
            (size_of::<bpf_loader_ctx>()
                + size_of::<bpf_map_desc>() * (*gen).nr_maps as usize
                + size_of::<bpf_prog_desc>() * i as usize
                + offsetof_bpf_prog_desc_prog_fd()) as c_int,
            4,
            stack_off_prog_fd(i),
        );
        i += 1;
    }
    i = 0;
    while i < (*gen).nr_maps {
        move_blob2ctx(
            gen,
            (size_of::<bpf_loader_ctx>()
                + size_of::<bpf_map_desc>() * i as usize
                + offsetof_bpf_map_desc_map_fd()) as c_int,
            4,
            blob_fd_array_off(gen, i),
        );
        i += 1;
    }
    emit(gen, BPF_MOV64_IMM(BPF_REG_0, 0));
    emit(gen, BPF_EXIT_INSN());
    if (*gen).error == 0 {
        let opts = (*gen).opts;

        (*opts).insns = (*gen).insn_start;
        (*opts).insns_sz = ((*gen).insn_cur as usize).wrapping_sub((*gen).insn_start as usize);
        (*opts).data = (*gen).data_start;
        (*opts).data_sz = ((*gen).data_cur as usize).wrapping_sub((*gen).data_start as usize);

        /* use target endianness for embedded loader */
        if (*gen).swapped_endian {
            let mut insn = (*opts).insns as *mut bpf_insn;
            let insn_cnt = (*opts).insns_sz / size_of::<bpf_insn>();
            i = 0;
            while (i as usize) < insn_cnt {
                bpf_insn_bswap(insn);
                insn = insn.add(1);
                i += 1;
            }
        }
    }
    pr_debug(cstr!("gen: finish %s\n"), errstr((*gen).error));
    (*gen).error
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__free(gen: *mut bpf_gen) {
    if gen.is_null() {
        return;
    }
    free((*gen).data_start);
    free((*gen).insn_start);
    free(gen as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__load_btf(
    gen: *mut bpf_gen,
    btf_raw_data: *const c_void,
    btf_raw_size: __u32,
) {
    let attr_size = offsetofend_bpf_attr_btf_log_level;
    let btf_data: c_int;
    let btf_load_attr: c_int;
    let mut attr: bpf_attr = zeroed();

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_size as usize);
    btf_data = add_data(gen, btf_raw_data, btf_raw_size);
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_btf_size as usize) as *mut __u32, tgt_endian_u32(gen, btf_raw_size));
    btf_load_attr = add_data(gen, &attr as *const _ as *const c_void, attr_size as __u32);
    pr_debug(cstr!("gen: load_btf: off %d size %u, attr: off %d size %d\n"), btf_data, btf_raw_size, btf_load_attr, attr_size);

    /* populate union bpf_attr with user provided log details */
    move_ctx2blob(gen, attr_field(btf_load_attr, offsetof_bpf_attr_btf_log_level), 4, offsetof_bpf_loader_ctx_log_level() as c_int, false);
    move_ctx2blob(gen, attr_field(btf_load_attr, offsetof_bpf_attr_btf_log_size), 4, offsetof_bpf_loader_ctx_log_size() as c_int, false);
    move_ctx2blob(gen, attr_field(btf_load_attr, offsetof_bpf_attr_btf_log_buf), 8, offsetof_bpf_loader_ctx_log_buf() as c_int, false);
    /* populate union bpf_attr with a pointer to the BTF data */
    emit_rel_store(gen, attr_field(btf_load_attr, offsetof_bpf_attr_btf), btf_data);
    /* emit BTF_LOAD command */
    emit_sys_bpf(gen, BPF_BTF_LOAD, btf_load_attr, attr_size);
    debug_ret(gen, cstr!("btf_load size %d"), btf_raw_size);
    emit_check_err(gen);
    /* remember btf_fd in the stack, if successful */
    emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_7, stack_off_btf_fd()));
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__map_create(
    gen: *mut bpf_gen,
    map_type: bpf_map_type,
    map_name: *const c_char,
    key_size: __u32,
    value_size: __u32,
    max_entries: __u32,
    map_attr: *mut bpf_map_create_opts,
    map_idx: c_int,
) {
    let attr_size = offsetofend_bpf_attr_map_extra;
    let mut close_inner_map_fd = false;
    let map_create_attr: c_int;
    let idx: c_int;
    let mut attr: bpf_attr = zeroed();

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_size as usize);
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_map_type as usize) as *mut __u32, tgt_endian_u32(gen, map_type as __u32));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_key_size as usize) as *mut __u32, tgt_endian_u32(gen, key_size));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_value_size as usize) as *mut __u32, tgt_endian_u32(gen, value_size));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_map_flags as usize) as *mut __u32, tgt_endian_u32(gen, (*map_attr).map_flags));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_map_extra as usize) as *mut __u64, tgt_endian_u64(gen, (*map_attr).map_extra));
    if !map_name.is_null() {
        libbpf_strlcpy((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_map_name as usize) as *mut c_char, map_name, 16);
    }
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_numa_node as usize) as *mut __u32, tgt_endian_u32(gen, (*map_attr).numa_node));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_map_ifindex as usize) as *mut __u32, tgt_endian_u32(gen, (*map_attr).map_ifindex));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_max_entries as usize) as *mut __u32, tgt_endian_u32(gen, max_entries));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_btf_key_type_id as usize) as *mut __u32, tgt_endian_u32(gen, (*map_attr).btf_key_type_id));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_btf_value_type_id as usize) as *mut __u32, tgt_endian_u32(gen, (*map_attr).btf_value_type_id));

    map_create_attr = add_data(gen, &attr as *const _ as *const c_void, attr_size as __u32);
    pr_debug(cstr!("gen: map_create: %s idx %d type %u value_type_id %u, attr: off %d size %d\n"), map_name, map_idx, map_type as __u32, (*map_attr).btf_value_type_id, map_create_attr, attr_size);

    if (*map_attr).btf_value_type_id != 0 {
        /* populate union bpf_attr with btf_fd saved in the stack earlier */
        move_stack2blob(gen, attr_field(map_create_attr, offsetof_bpf_attr_btf_fd), 4, stack_off_btf_fd());
    }
    if map_type == BPF_MAP_TYPE_ARRAY_OF_MAPS || map_type == BPF_MAP_TYPE_HASH_OF_MAPS {
        move_stack2blob(gen, attr_field(map_create_attr, offsetof_bpf_attr_inner_map_fd), 4, stack_off_inner_map_fd());
        close_inner_map_fd = true;
    }

    /*
     * Conditionally update max_entries from the host-supplied loader
     * ctx. This sizes the map at runtime, but for a signed loader
     * (gen_hash) it would let an untrusted host re-dimension the
     * program's maps, outside what the signature attests to: the
     * metadata blob is covered by the program signature and verified
     * by the kernel at load time. Keep the signer-provided max_entries
     * baked into the blob in that case.
     */
    if map_idx >= 0 && !opts_get_gen_hash((*gen).opts, false) {
        move_ctx2blob(
            gen,
            attr_field(map_create_attr, offsetof_bpf_attr_max_entries),
            4,
            (size_of::<bpf_loader_ctx>()
                + size_of::<bpf_map_desc>() * map_idx as usize
                + offsetof_bpf_map_desc_max_entries()) as c_int,
            true,
        );
    }

    /* emit MAP_CREATE command */
    emit_sys_bpf(gen, BPF_MAP_CREATE, map_create_attr, attr_size);
    debug_ret(gen, cstr!("map_create %s idx %d type %d value_size %d value_btf_id %d"), map_name, map_idx, map_type, value_size, (*map_attr).btf_value_type_id);
    emit_check_err(gen);
    /* remember map_fd in the stack, if successful */
    if map_idx < 0 {
        /* This bpf_gen__map_create() function is called with map_idx >= 0
         * for all maps that libbpf loading logic tracks.
         * It's called with -1 to create an inner map.
         */
        emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_7, stack_off_inner_map_fd()));
    } else if map_idx != (*gen).nr_maps {
        (*gen).error = -EDOM; /* internal bug */
        return;
    } else {
        /* add_map_fd does gen->nr_maps++ */
        idx = add_map_fd(gen);
        emit_ld_imm64!(gen, BPF_REG_1, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, blob_fd_array_off(gen, idx));
        emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_7, 0));
    }
    if close_inner_map_fd {
        emit_sys_close_stack(gen, stack_off_inner_map_fd());
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__record_attach_target(
    gen: *mut bpf_gen,
    attach_name: *const c_char,
    ty: bpf_attach_type,
) {
    let mut prefix: *const c_char = ptr::null();
    let mut kind: c_int = 0;
    let ret: c_int;

    btf_get_kernel_prefix_kind(ty, &mut prefix, &mut kind);
    (*gen).attach_kind = kind;
    ret = snprintf((*gen).attach_target.as_mut_ptr(), (*gen).attach_target.len(), cstr!("%s%s"), prefix, attach_name);
    if ret as usize >= (*gen).attach_target.len() {
        (*gen).error = -ENOSPC;
    }
}

unsafe fn emit_find_attach_target(gen: *mut bpf_gen) {
    let len = strlen((*gen).attach_target.as_ptr()) as c_int + 1;
    let name: c_int;

    pr_debug(cstr!("gen: find_attach_tgt %s %d\n"), (*gen).attach_target.as_ptr(), (*gen).attach_kind);
    name = add_data(gen, (*gen).attach_target.as_ptr() as *const c_void, len as __u32);

    emit_ld_imm64!(gen, BPF_REG_1, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, name);
    emit(gen, BPF_MOV64_IMM(BPF_REG_2, len));
    emit(gen, BPF_MOV64_IMM(BPF_REG_3, (*gen).attach_kind));
    emit(gen, BPF_MOV64_IMM(BPF_REG_4, 0));
    emit(gen, BPF_EMIT_CALL(BPF_FUNC_btf_find_by_name_kind));
    emit(gen, BPF_MOV64_REG(BPF_REG_7, BPF_REG_0));
    debug_ret(gen, cstr!("find_by_name_kind(%s,%d)"), (*gen).attach_target.as_ptr(), (*gen).attach_kind);
    emit_check_err(gen);
    /* if successful, btf_id is in lower 32-bit of R7 and
     * btf_obj_fd is in upper 32-bit
     */
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__record_extern(
    gen: *mut bpf_gen,
    name: *const c_char,
    is_weak: bool,
    is_typeless: bool,
    is_ld64: bool,
    kind: c_int,
    insn_idx: c_int,
) {
    let mut relo: *mut ksym_relo_desc;

    relo = libbpf_reallocarray((*gen).relos as *mut c_void, ((*gen).relo_cnt + 1) as size_t, size_of::<ksym_relo_desc>()) as *mut ksym_relo_desc;
    if relo.is_null() {
        (*gen).error = -ENOMEM;
        return;
    }
    (*gen).relos = relo;
    relo = relo.add((*gen).relo_cnt as usize);
    (*relo).name = name;
    (*relo).is_weak = is_weak;
    (*relo).is_typeless = is_typeless;
    (*relo).is_ld64 = is_ld64;
    (*relo).kind = kind;
    (*relo).insn_idx = insn_idx;
    (*gen).relo_cnt += 1;
}

/* returns existing ksym_desc with ref incremented, or inserts a new one */
unsafe fn get_ksym_desc(gen: *mut bpf_gen, relo: *mut ksym_relo_desc) -> *mut ksym_desc {
    let mut kdesc: *mut ksym_desc;
    let mut i: c_int = 0;

    while i < (*gen).nr_ksyms {
        kdesc = (*gen).ksyms.add(i as usize);
        if (*kdesc).kind == (*relo).kind
            && (*kdesc).is_ld64 == (*relo).is_ld64
            && strcmp((*kdesc).name, (*relo).name) == 0
        {
            (*kdesc).ref_ += 1;
            return kdesc;
        }
        i += 1;
    }
    kdesc = libbpf_reallocarray((*gen).ksyms as *mut c_void, ((*gen).nr_ksyms + 1) as size_t, size_of::<ksym_desc>()) as *mut ksym_desc;
    if kdesc.is_null() {
        (*gen).error = -ENOMEM;
        return ptr::null_mut();
    }
    (*gen).ksyms = kdesc;
    let idx = (*gen).nr_ksyms;
    (*gen).nr_ksyms += 1;
    kdesc = (*gen).ksyms.add(idx as usize);
    (*kdesc).name = (*relo).name;
    (*kdesc).kind = (*relo).kind;
    (*kdesc).ref_ = 1;
    (*kdesc).off = 0;
    (*kdesc).insn = 0;
    (*kdesc).is_ld64 = (*relo).is_ld64;
    (*kdesc).typeless = false;
    kdesc
}

/* Overwrites BPF_REG_{0, 1, 2, 3, 4, 7}
 * Returns result in BPF_REG_7
 */
unsafe fn emit_bpf_find_by_name_kind(gen: *mut bpf_gen, relo: *mut ksym_relo_desc) {
    let len = strlen((*relo).name) as c_int + 1;
    let name_off = add_data(gen, (*relo).name as *const c_void, len as __u32);
    emit_ld_imm64!(gen, BPF_REG_1, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, name_off);
    emit(gen, BPF_MOV64_IMM(BPF_REG_2, len));
    emit(gen, BPF_MOV64_IMM(BPF_REG_3, (*relo).kind));
    emit(gen, BPF_MOV64_IMM(BPF_REG_4, 0));
    emit(gen, BPF_EMIT_CALL(BPF_FUNC_btf_find_by_name_kind));
    emit(gen, BPF_MOV64_REG(BPF_REG_7, BPF_REG_0));
    debug_ret(gen, cstr!("find_by_name_kind(%s,%d)"), (*relo).name, (*relo).kind);
}

/* Overwrites BPF_REG_{0, 1, 2, 3, 4, 7}
 * Returns result in BPF_REG_7
 * Returns u64 symbol addr in BPF_REG_9
 */
unsafe fn emit_bpf_kallsyms_lookup_name(gen: *mut bpf_gen, relo: *mut ksym_relo_desc) {
    let len = strlen((*relo).name) as c_int + 1;
    let name_off = add_data(gen, (*relo).name as *const c_void, len as __u32);
    let res_off = add_data(gen, ptr::null(), 8); /* res is u64 */
    emit_ld_imm64!(gen, BPF_REG_1, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, name_off);
    emit(gen, BPF_MOV64_IMM(BPF_REG_2, len));
    emit(gen, BPF_MOV64_IMM(BPF_REG_3, 0));
    emit_ld_imm64!(gen, BPF_REG_4, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, res_off);
    emit(gen, BPF_MOV64_REG(BPF_REG_7, BPF_REG_4));
    emit(gen, BPF_EMIT_CALL(BPF_FUNC_kallsyms_lookup_name));
    emit(gen, BPF_LDX_MEM(BPF_DW, BPF_REG_9, BPF_REG_7, 0));
    emit(gen, BPF_MOV64_REG(BPF_REG_7, BPF_REG_0));
    debug_ret(gen, cstr!("kallsyms_lookup_name(%s,%d)"), (*relo).name, (*relo).kind);
}

unsafe fn emit_relo_kfunc_btf(gen: *mut bpf_gen, relo: *mut ksym_relo_desc, insn: c_int) {
    let kdesc = get_ksym_desc(gen, relo);
    let btf_fd_idx: c_int;

    if kdesc.is_null() {
        return;
    }
    /* try to copy from existing bpf_insn */
    if (*kdesc).ref_ > 1 {
        move_blob2blob(gen, insn + offsetof_bpf_insn_imm() as c_int, 4, (*kdesc).insn + offsetof_bpf_insn_imm() as c_int);
        move_blob2blob(gen, insn + offsetof_bpf_insn_off() as c_int, 2, (*kdesc).insn + offsetof_bpf_insn_off() as c_int);
    } else {
        /* remember insn offset, so we can copy BTF ID and FD later */
        (*kdesc).insn = insn;
        emit_bpf_find_by_name_kind(gen, relo);
        if !(*relo).is_weak {
            emit_check_err(gen);
        }
        btf_fd_idx = add_kfunc_btf_fd(gen);
        if btf_fd_idx > INT16_MAX_ {
            pr_warn(cstr!("BTF fd off %d for kfunc %s exceeds INT16_MAX, cannot process relocation\n"), btf_fd_idx, (*relo).name);
            (*gen).error = -E2BIG;
            return;
        }
        (*kdesc).off = btf_fd_idx;
        emit(gen, BPF_JMP_IMM(BPF_JSGE, BPF_REG_7, 0, 3));
        emit(gen, BPF_ST_MEM(BPF_W, BPF_REG_8, offsetof_bpf_insn_imm() as c_int, 0));
        emit(gen, BPF_ST_MEM(BPF_H, BPF_REG_8, offsetof_bpf_insn_off() as c_int, 0));
        emit(gen, BPF_JMP_IMM(BPF_JA, 0, 0, 10));
        emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_8, BPF_REG_7, offsetof_bpf_insn_imm() as c_int));
        emit(gen, BPF_MOV64_REG(BPF_REG_9, BPF_REG_7));
        emit(gen, BPF_ALU64_IMM(BPF_RSH, BPF_REG_9, 32));
        emit_ld_imm64!(gen, BPF_REG_0, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, blob_fd_array_off(gen, btf_fd_idx));
        emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_0, BPF_REG_9, 0));
        emit(gen, BPF_JMP_IMM(BPF_JNE, BPF_REG_9, 0, 2));
        emit(gen, BPF_ST_MEM(BPF_H, BPF_REG_8, offsetof_bpf_insn_off() as c_int, 0));
        emit(gen, BPF_JMP_IMM(BPF_JA, 0, 0, 1));
        emit(gen, BPF_ST_MEM(BPF_H, BPF_REG_8, offsetof_bpf_insn_off() as c_int, btf_fd_idx));
    }
    if (*gen).log_level == 0 {
        return;
    }
    emit(gen, BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_8, offsetof_bpf_insn_imm() as c_int));
    emit(gen, BPF_LDX_MEM(BPF_H, BPF_REG_9, BPF_REG_8, offsetof_bpf_insn_off() as c_int));
    debug_regs(gen, BPF_REG_7, BPF_REG_9, cstr!(" func (%s:count=%d): imm: %%d, off: %%d"), (*relo).name, (*kdesc).ref_);
    emit_ld_imm64!(gen, BPF_REG_0, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, blob_fd_array_off(gen, (*kdesc).off));
    emit(gen, BPF_LDX_MEM(BPF_W, BPF_REG_9, BPF_REG_0, 0));
    debug_regs(gen, BPF_REG_9, -1, cstr!(" func (%s:count=%d): btf_fd"), (*relo).name, (*kdesc).ref_);
}

unsafe fn emit_ksym_relo_log(gen: *mut bpf_gen, relo: *mut ksym_relo_desc, ref_: c_int) {
    if (*gen).log_level == 0 {
        return;
    }
    emit(gen, BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_8, offsetof_bpf_insn_imm() as c_int));
    emit(gen, BPF_LDX_MEM(BPF_H, BPF_REG_9, BPF_REG_8, (size_of::<bpf_insn>() + offsetof_bpf_insn_imm()) as c_int));
    debug_regs(gen, BPF_REG_7, BPF_REG_9, cstr!(" var t=%d w=%d (%s:count=%d): imm[0]: %%d, imm[1]: %%d"), (*relo).is_typeless as c_int, (*relo).is_weak as c_int, (*relo).name, ref_);
    emit(gen, BPF_LDX_MEM(BPF_B, BPF_REG_9, BPF_REG_8, offsetofend_bpf_insn_code() as c_int));
    debug_regs(gen, BPF_REG_9, -1, cstr!(" var t=%d w=%d (%s:count=%d): insn.reg"), (*relo).is_typeless as c_int, (*relo).is_weak as c_int, (*relo).name, ref_);
}

unsafe fn emit_relo_ksym_typeless(gen: *mut bpf_gen, relo: *mut ksym_relo_desc, insn: c_int) {
    let kdesc = get_ksym_desc(gen, relo);
    if kdesc.is_null() {
        return;
    }
    if (*kdesc).ref_ > 1 {
        move_blob2blob(gen, insn + offsetof_bpf_insn_imm() as c_int, 4, (*kdesc).insn + offsetof_bpf_insn_imm() as c_int);
        move_blob2blob(gen, insn + size_of::<bpf_insn>() as c_int + offsetof_bpf_insn_imm() as c_int, 4, (*kdesc).insn + size_of::<bpf_insn>() as c_int + offsetof_bpf_insn_imm() as c_int);
    } else {
        (*kdesc).insn = insn;
        (*kdesc).typeless = true;
        emit_bpf_kallsyms_lookup_name(gen, relo);
        emit(gen, BPF_JMP_IMM(BPF_JEQ, BPF_REG_7, -ENOENT, 1));
        emit_check_err(gen);
        emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_8, BPF_REG_9, offsetof_bpf_insn_imm() as c_int));
        emit(gen, BPF_ALU64_IMM(BPF_RSH, BPF_REG_9, 32));
        emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_8, BPF_REG_9, (size_of::<bpf_insn>() + offsetof_bpf_insn_imm()) as c_int));
    }
    emit_ksym_relo_log(gen, relo, (*kdesc).ref_);
}

unsafe fn src_reg_mask(gen: *mut bpf_gen) -> __u32 {
    /* Mirrors the original __LITTLE_ENDIAN_BITFIELD/__BIG_ENDIAN_BITFIELD
     * preprocessor condition. Rust compilation target cfg determines the
     * native bit-endian assumption for this source-level translation.
     */
    #[cfg(target_endian = "little")]
    {
        if (*gen).swapped_endian { 0xf0 } else { 0x0f }
    }
    #[cfg(target_endian = "big")]
    {
        if (*gen).swapped_endian { 0x0f } else { 0xf0 }
    }
}

unsafe fn emit_relo_ksym_btf(gen: *mut bpf_gen, relo: *mut ksym_relo_desc, insn: c_int) {
    let kdesc = get_ksym_desc(gen, relo);
    let reg_mask: __u32;

    if kdesc.is_null() {
        return;
    }
    if (*kdesc).ref_ > 1 {
        move_blob2blob(gen, insn + size_of::<bpf_insn>() as c_int + offsetof_bpf_insn_imm() as c_int, 4, (*kdesc).insn + size_of::<bpf_insn>() as c_int + offsetof_bpf_insn_imm() as c_int);
        move_blob2blob(gen, insn + offsetof_bpf_insn_imm() as c_int, 4, (*kdesc).insn + offsetof_bpf_insn_imm() as c_int);
        emit(gen, BPF_JMP_IMM(BPF_JNE, BPF_REG_0, 0, 3));
    } else {
        (*kdesc).insn = insn;
        emit_bpf_find_by_name_kind(gen, relo);
        if !(*relo).is_weak {
            emit_check_err(gen);
        }
        emit(gen, BPF_JMP_IMM(BPF_JSGE, BPF_REG_7, 0, 3));
        emit(gen, BPF_ST_MEM(BPF_W, BPF_REG_8, offsetof_bpf_insn_imm() as c_int, 0));
        emit(gen, BPF_ST_MEM(BPF_W, BPF_REG_8, (size_of::<bpf_insn>() + offsetof_bpf_insn_imm()) as c_int, 0));
        emit(gen, BPF_JMP_IMM(BPF_JA, 0, 0, 4));
        emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_8, BPF_REG_7, offsetof_bpf_insn_imm() as c_int));
        emit(gen, BPF_ALU64_IMM(BPF_RSH, BPF_REG_7, 32));
        emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_8, BPF_REG_7, (size_of::<bpf_insn>() + offsetof_bpf_insn_imm()) as c_int));
        emit(gen, BPF_JMP_IMM(BPF_JA, 0, 0, 3));
    }
    /* clear bpf_object__relocate_data's src_reg assignment, otherwise we get a verifier failure */
    reg_mask = src_reg_mask(gen);
    emit(gen, BPF_LDX_MEM(BPF_B, BPF_REG_9, BPF_REG_8, offsetofend_bpf_insn_code() as c_int));
    emit(gen, BPF_ALU32_IMM(BPF_AND, BPF_REG_9, reg_mask as c_int));
    emit(gen, BPF_STX_MEM(BPF_B, BPF_REG_8, BPF_REG_9, offsetofend_bpf_insn_code() as c_int));

    emit_ksym_relo_log(gen, relo, (*kdesc).ref_);
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__record_relo_core(
    gen: *mut bpf_gen,
    core_relo: *const bpf_core_relo,
) {
    let mut relos: *mut bpf_core_relo;

    relos = libbpf_reallocarray((*gen).core_relos as *mut c_void, ((*gen).core_relo_cnt + 1) as size_t, size_of::<bpf_core_relo>()) as *mut bpf_core_relo;
    if relos.is_null() {
        (*gen).error = -ENOMEM;
        return;
    }
    (*gen).core_relos = relos;
    relos = relos.add((*gen).core_relo_cnt as usize);
    memcpy(relos as *mut c_void, core_relo as *const c_void, size_of::<bpf_core_relo>());
    (*gen).core_relo_cnt += 1;
}

unsafe fn emit_relo(gen: *mut bpf_gen, relo: *mut ksym_relo_desc, insns: c_int) {
    let insn: c_int;

    pr_debug(cstr!("gen: emit_relo (%d): %s at %d %s\n"), (*relo).kind, (*relo).name, (*relo).insn_idx, if (*relo).is_ld64 { cstr!("ld64") } else { cstr!("call") });
    insn = insns + size_of::<bpf_insn>() as c_int * (*relo).insn_idx;
    emit_ld_imm64!(gen, BPF_REG_8, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, insn);
    if (*relo).is_ld64 {
        if (*relo).is_typeless {
            emit_relo_ksym_typeless(gen, relo, insn);
        } else {
            emit_relo_ksym_btf(gen, relo, insn);
        }
    } else {
        emit_relo_kfunc_btf(gen, relo, insn);
    }
}

unsafe fn emit_relos(gen: *mut bpf_gen, insns: c_int) {
    let mut i: c_int = 0;
    while i < (*gen).relo_cnt {
        emit_relo(gen, (*gen).relos.add(i as usize), insns);
        i += 1;
    }
}

unsafe fn cleanup_core_relo(gen: *mut bpf_gen) {
    if (*gen).core_relo_cnt == 0 {
        return;
    }
    free((*gen).core_relos as *mut c_void);
    (*gen).core_relo_cnt = 0;
    (*gen).core_relos = ptr::null_mut();
}

unsafe fn cleanup_relos(gen: *mut bpf_gen, _insns: c_int) {
    let mut kdesc: *mut ksym_desc;
    let mut i: c_int = 0;
    let mut insn: c_int;

    while i < (*gen).nr_ksyms {
        kdesc = (*gen).ksyms.add(i as usize);
        /* only close fds for typed ksyms and kfuncs */
        if (*kdesc).is_ld64 && !(*kdesc).typeless {
            /* close fd recorded in insn[insn_idx + 1].imm */
            insn = (*kdesc).insn;
            insn += size_of::<bpf_insn>() as c_int + offsetof_bpf_insn_imm() as c_int;
            emit_sys_close_blob(gen, insn);
        } else if !(*kdesc).is_ld64 {
            emit_sys_close_blob(gen, blob_fd_array_off(gen, (*kdesc).off));
            if (*kdesc).off < MAX_FD_ARRAY_SZ as c_int {
                (*gen).nr_fd_array -= 1;
            }
        }
        i += 1;
    }
    if (*gen).nr_ksyms != 0 {
        free((*gen).ksyms as *mut c_void);
        (*gen).nr_ksyms = 0;
        (*gen).ksyms = ptr::null_mut();
    }
    if (*gen).relo_cnt != 0 {
        free((*gen).relos as *mut c_void);
        (*gen).relo_cnt = 0;
        (*gen).relos = ptr::null_mut();
    }
    cleanup_core_relo(gen);
}

/* Convert func, line, and core relo info blobs to target endianness */
unsafe fn info_blob_bswap(
    gen: *mut bpf_gen,
    func_info: c_int,
    line_info: c_int,
    core_relos: c_int,
    load_attr: *mut bpf_prog_load_opts,
) {
    let mut fi = ((*gen).data_start as *mut u8).add(func_info as usize) as *mut bpf_func_info;
    let mut li = ((*gen).data_start as *mut u8).add(line_info as usize) as *mut bpf_line_info;
    let mut cr = ((*gen).data_start as *mut u8).add(core_relos as usize) as *mut bpf_core_relo;
    let mut i: c_int = 0;

    while i < (*load_attr).func_info_cnt as c_int {
        bpf_func_info_bswap(fi);
        fi = fi.add(1);
        i += 1;
    }
    i = 0;
    while i < (*load_attr).line_info_cnt as c_int {
        bpf_line_info_bswap(li);
        li = li.add(1);
        i += 1;
    }
    i = 0;
    while i < (*gen).core_relo_cnt {
        bpf_core_relo_bswap(cr);
        cr = cr.add(1);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__prog_load(
    gen: *mut bpf_gen,
    prog_type: bpf_prog_type,
    prog_name: *const c_char,
    license: *const c_char,
    insns: *mut bpf_insn,
    insn_cnt: size_t,
    load_attr: *mut bpf_prog_load_opts,
    prog_idx: c_int,
) {
    let func_info_tot_sz = (*load_attr).func_info_cnt * (*load_attr).func_info_rec_size;
    let line_info_tot_sz = (*load_attr).line_info_cnt * (*load_attr).line_info_rec_size;
    let core_relo_tot_sz = (*gen).core_relo_cnt as usize * size_of::<bpf_core_relo>();
    let prog_load_attr: c_int;
    let license_off: c_int;
    let insns_off: c_int;
    let func_info: c_int;
    let line_info: c_int;
    let core_relos: c_int;
    let attr_size = offsetofend_bpf_attr_core_relo_rec_size;
    let mut attr: bpf_attr = zeroed();
    let mut i: c_int;

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_size as usize);
    /* add license string to blob of bytes */
    license_off = add_data(gen, license as *const c_void, (strlen(license) + 1) as __u32);
    /* add insns to blob of bytes */
    insns_off = add_data(gen, insns as *const c_void, (insn_cnt * size_of::<bpf_insn>()) as __u32);
    pr_debug(cstr!("gen: prog_load: prog_idx %d type %u insn off %d insns_cnt %zu license off %d\n"), prog_idx, prog_type as __u32, insns_off, insn_cnt, license_off);

    /* convert blob insns to target endianness */
    if (*gen).swapped_endian && (*gen).error == 0 {
        let mut insn = ((*gen).data_start as *mut u8).add(insns_off as usize) as *mut bpf_insn;
        i = 0;
        while (i as usize) < insn_cnt {
            bpf_insn_bswap(insn);
            insn = insn.add(1);
            i += 1;
        }
    }

    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_prog_type as usize) as *mut __u32, tgt_endian_u32(gen, prog_type as __u32));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_expected_attach_type as usize) as *mut __u32, tgt_endian_u32(gen, (*load_attr).expected_attach_type as __u32));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_attach_btf_id as usize) as *mut __u32, tgt_endian_u32(gen, (*load_attr).attach_btf_id));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_prog_ifindex as usize) as *mut __u32, tgt_endian_u32(gen, (*load_attr).prog_ifindex));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_kern_version as usize) as *mut __u32, 0);
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_insn_cnt as usize) as *mut __u32, tgt_endian_u32(gen, insn_cnt as __u32));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_prog_flags as usize) as *mut __u32, tgt_endian_u32(gen, (*load_attr).prog_flags));

    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_func_info_rec_size as usize) as *mut __u32, tgt_endian_u32(gen, (*load_attr).func_info_rec_size));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_func_info_cnt as usize) as *mut __u32, tgt_endian_u32(gen, (*load_attr).func_info_cnt));
    func_info = add_data(gen, (*load_attr).func_info, func_info_tot_sz);
    pr_debug(cstr!("gen: prog_load: func_info: off %d cnt %u rec size %u\n"), func_info, (*load_attr).func_info_cnt, (*load_attr).func_info_rec_size);

    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_line_info_rec_size as usize) as *mut __u32, tgt_endian_u32(gen, (*load_attr).line_info_rec_size));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_line_info_cnt as usize) as *mut __u32, tgt_endian_u32(gen, (*load_attr).line_info_cnt));
    line_info = add_data(gen, (*load_attr).line_info, line_info_tot_sz);
    pr_debug(cstr!("gen: prog_load: line_info: off %d cnt %u rec size %u\n"), line_info, (*load_attr).line_info_cnt, (*load_attr).line_info_rec_size);

    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_core_relo_rec_size as usize) as *mut __u32, tgt_endian_u32(gen, size_of::<bpf_core_relo>() as __u32));
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_core_relo_cnt as usize) as *mut __u32, tgt_endian_u32(gen, (*gen).core_relo_cnt as __u32));
    core_relos = add_data(gen, (*gen).core_relos as *const c_void, core_relo_tot_sz as __u32);
    pr_debug(cstr!("gen: prog_load: core_relos: off %d cnt %d rec size %zu\n"), core_relos, (*gen).core_relo_cnt, size_of::<bpf_core_relo>());

    /* convert all info blobs to target endianness */
    if (*gen).swapped_endian && (*gen).error == 0 {
        info_blob_bswap(gen, func_info, line_info, core_relos, load_attr);
    }

    libbpf_strlcpy((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_prog_name as usize) as *mut c_char, prog_name, 16);
    prog_load_attr = add_data(gen, &attr as *const _ as *const c_void, attr_size as __u32);
    pr_debug(cstr!("gen: prog_load: attr: off %d size %d\n"), prog_load_attr, attr_size);

    emit_rel_store(gen, attr_field(prog_load_attr, offsetof_bpf_attr_license), license_off);
    emit_rel_store(gen, attr_field(prog_load_attr, offsetof_bpf_attr_insns), insns_off);
    emit_rel_store(gen, attr_field(prog_load_attr, offsetof_bpf_attr_func_info), func_info);
    emit_rel_store(gen, attr_field(prog_load_attr, offsetof_bpf_attr_line_info), line_info);
    emit_rel_store(gen, attr_field(prog_load_attr, offsetof_bpf_attr_core_relos), core_relos);
    emit_rel_store(gen, attr_field(prog_load_attr, offsetof_bpf_attr_fd_array), (*gen).fd_array);

    move_ctx2blob(gen, attr_field(prog_load_attr, offsetof_bpf_attr_log_level), 4, offsetof_bpf_loader_ctx_log_level() as c_int, false);
    move_ctx2blob(gen, attr_field(prog_load_attr, offsetof_bpf_attr_log_size), 4, offsetof_bpf_loader_ctx_log_size() as c_int, false);
    move_ctx2blob(gen, attr_field(prog_load_attr, offsetof_bpf_attr_log_buf), 8, offsetof_bpf_loader_ctx_log_buf() as c_int, false);
    move_stack2blob(gen, attr_field(prog_load_attr, offsetof_bpf_attr_prog_btf_fd), 4, stack_off_btf_fd());
    if (*gen).attach_kind != 0 {
        emit_find_attach_target(gen);
        emit_ld_imm64!(gen, BPF_REG_0, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, prog_load_attr);
        emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_0, BPF_REG_7, offsetof_bpf_attr_attach_btf_id));
        emit(gen, BPF_ALU64_IMM(BPF_RSH, BPF_REG_7, 32));
        emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_0, BPF_REG_7, offsetof_bpf_attr_attach_btf_obj_fd));
    }
    emit_relos(gen, insns_off);
    /* emit PROG_LOAD command */
    emit_sys_bpf(gen, BPF_PROG_LOAD, prog_load_attr, attr_size);
    debug_ret(gen, cstr!("prog_load %s insn_cnt %d"), (&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_prog_name as usize) as *mut c_char, ptr::read_unaligned((&attr as *const _ as *const u8).add(offsetof_bpf_attr_insn_cnt as usize) as *const __u32));
    /* successful or not, close btf module FDs used in extern ksyms and attach_btf_obj_fd */
    cleanup_relos(gen, insns_off);
    if (*gen).attach_kind != 0 {
        emit_sys_close_blob(gen, attr_field(prog_load_attr, offsetof_bpf_attr_attach_btf_obj_fd));
        (*gen).attach_kind = 0;
    }
    emit_check_err(gen);
    /* remember prog_fd in the stack, if successful */
    emit(gen, BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_7, stack_off_prog_fd((*gen).nr_progs)));
    (*gen).nr_progs += 1;
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__map_update_elem(
    gen: *mut bpf_gen,
    map_idx: c_int,
    pvalue: *mut c_void,
    value_size: __u32,
    flags: __u64,
) {
    let attr_size = offsetofend_bpf_attr_flags;
    let map_update_attr: c_int;
    let value: c_int;
    let key: c_int;
    let mut attr: bpf_attr = zeroed();
    let zero: c_int = 0;

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_size as usize);
    ptr::write_unaligned((&mut attr as *mut _ as *mut u8).add(offsetof_bpf_attr_flags as usize) as *mut __u64, tgt_endian_u64(gen, flags));

    value = add_data(gen, pvalue as *const c_void, value_size);
    key = add_data(gen, &zero as *const _ as *const c_void, size_of::<c_int>() as __u32);

    /*
     * if (map_desc[map_idx].initial_value) {
     *    if (ctx->flags & BPF_SKEL_KERNEL)
     *        bpf_probe_read_kernel(value, value_size, initial_value);
     *    else
     *        bpf_copy_from_user(value, value_size, initial_value);
     * }
     *
     * The runtime initial_value comes from the host-supplied loader
     * ctx and would overwrite the blob value that the program signature
     * covers and the kernel verifies at load time. For a signed loader
     * (gen_hash) the attested blob value must be authoritative, so skip
     * the override and leave the signed value in place.
     */
    if !opts_get_gen_hash((*gen).opts, false) {
        emit(gen, BPF_LDX_MEM(BPF_DW, BPF_REG_3, BPF_REG_6, (size_of::<bpf_loader_ctx>() + size_of::<bpf_map_desc>() * map_idx as usize + offsetof_bpf_map_desc_initial_value()) as c_int));
        emit(gen, BPF_JMP_IMM(BPF_JEQ, BPF_REG_3, 0, 8));
        emit_ld_imm64!(gen, BPF_REG_1, BPF_PSEUDO_MAP_IDX_VALUE, 0, 0, 0, value);
        emit(gen, BPF_MOV64_IMM(BPF_REG_2, value_size as c_int));
        emit(gen, BPF_LDX_MEM(BPF_W, BPF_REG_0, BPF_REG_6, offsetof_bpf_loader_ctx_flags() as c_int));
        emit(gen, BPF_JMP_IMM(BPF_JSET, BPF_REG_0, BPF_SKEL_KERNEL, 2));
        emit(gen, BPF_EMIT_CALL(BPF_FUNC_copy_from_user));
        emit(gen, BPF_JMP_IMM(BPF_JA, 0, 0, 1));
        emit(gen, BPF_EMIT_CALL(BPF_FUNC_probe_read_kernel));
    }

    map_update_attr = add_data(gen, &attr as *const _ as *const c_void, attr_size as __u32);
    pr_debug(cstr!("gen: map_update_elem: idx %d, value: off %d size %u, attr: off %d size %d\n"), map_idx, value, value_size, map_update_attr, attr_size);
    move_blob2blob(gen, attr_field(map_update_attr, offsetof_bpf_attr_map_fd), 4, blob_fd_array_off(gen, map_idx));
    emit_rel_store(gen, attr_field(map_update_attr, offsetof_bpf_attr_key), key);
    emit_rel_store(gen, attr_field(map_update_attr, offsetof_bpf_attr_value), value);
    /* emit MAP_UPDATE_ELEM command */
    emit_sys_bpf(gen, BPF_MAP_UPDATE_ELEM, map_update_attr, attr_size);
    debug_ret(gen, cstr!("update_elem idx %d value_size %d"), map_idx, value_size);
    emit_check_err(gen);
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__populate_outer_map(
    gen: *mut bpf_gen,
    outer_map_idx: c_int,
    slot: c_int,
    inner_map_idx: c_int,
) {
    let attr_size = offsetofend_bpf_attr_flags;
    let map_update_attr: c_int;
    let key: c_int;
    let mut attr: bpf_attr = zeroed();
    let tgt_slot: c_int;

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_size as usize);
    tgt_slot = tgt_endian_u32(gen, slot as __u32) as c_int;
    key = add_data(gen, &tgt_slot as *const _ as *const c_void, size_of::<c_int>() as __u32);

    map_update_attr = add_data(gen, &attr as *const _ as *const c_void, attr_size as __u32);
    pr_debug(cstr!("gen: populate_outer_map: outer %d key %d inner %d, attr: off %d size %d\n"), outer_map_idx, slot, inner_map_idx, map_update_attr, attr_size);
    move_blob2blob(gen, attr_field(map_update_attr, offsetof_bpf_attr_map_fd), 4, blob_fd_array_off(gen, outer_map_idx));
    emit_rel_store(gen, attr_field(map_update_attr, offsetof_bpf_attr_key), key);
    emit_rel_store(gen, attr_field(map_update_attr, offsetof_bpf_attr_value), blob_fd_array_off(gen, inner_map_idx));

    /* emit MAP_UPDATE_ELEM command */
    emit_sys_bpf(gen, BPF_MAP_UPDATE_ELEM, map_update_attr, attr_size);
    debug_ret(gen, cstr!("populate_outer_map outer %d key %d inner %d"), outer_map_idx, slot, inner_map_idx);
    emit_check_err(gen);
}

#[no_mangle]
pub unsafe extern "C" fn bpf_gen__map_freeze(gen: *mut bpf_gen, map_idx: c_int) {
    let attr_size = offsetofend_bpf_attr_map_fd;
    let map_freeze_attr: c_int;
    let mut attr: bpf_attr = zeroed();

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_size as usize);
    map_freeze_attr = add_data(gen, &attr as *const _ as *const c_void, attr_size as __u32);
    pr_debug(cstr!("gen: map_freeze: idx %d, attr: off %d size %d\n"), map_idx, map_freeze_attr, attr_size);
    move_blob2blob(gen, attr_field(map_freeze_attr, offsetof_bpf_attr_map_fd), 4, blob_fd_array_off(gen, map_idx));
    /* emit MAP_FREEZE command */
    emit_sys_bpf(gen, BPF_MAP_FREEZE, map_freeze_attr, attr_size);
    debug_ret(gen, cstr!("map_freeze"));
    emit_check_err(gen);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
