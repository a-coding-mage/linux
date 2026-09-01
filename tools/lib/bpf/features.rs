// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */
/* Translated from lib/bpf/features.c. Header-provided libbpf, Linux, and BTF
 * definitions are treated as external dependencies.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u32 = u32;
type __u64 = u64;

extern "C" {
    static mut errno: c_int;

    fn close(fd: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn libbpf_strlcpy(dst: *mut c_char, src: *const c_char, siz: usize) -> usize;
    fn sys_bpf_prog_load(attr: *mut bpf_attr, attr_sz: usize, attempts: c_int) -> c_int;
    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn bpf_prog_load(
        prog_type: c_int,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: c_int,
        opts: *const bpf_prog_load_opts,
    ) -> c_int;
    fn libbpf__load_raw_btf(
        raw_types: *mut c_char,
        types_len: usize,
        strs: *const c_char,
        strs_len: usize,
        token_fd: c_int,
    ) -> c_int;
    fn libbpf__load_raw_btf_hdr(
        hdr: *const btf_header,
        raw_types: *mut c_char,
        strs: *const c_char,
        layout: *mut c_char,
        token_fd: c_int,
    ) -> c_int;
    fn bpf_prog_bind_map(prog_fd: c_int, map_fd: c_int, opts: *const c_void) -> c_int;
    fn bpf_btf_get_info_by_fd(fd: c_int, info: *mut bpf_btf_info, len: *mut __u32) -> c_int;
    fn bpf_link_create(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_int,
        opts: *const bpf_link_create_opts,
    ) -> c_int;
    fn probe_memcg_account(token_fd: c_int) -> c_int;
    fn probe_kern_syscall_wrapper(token_fd: c_int) -> c_int;
    fn probe_sys_bpf_ext() -> c_int;
    fn errstr(err: c_int) -> *const c_char;
    fn pr_warn(fmt: *const c_char, ...);

    fn BPF_MOV64_IMM(dst: c_int, imm: c_int) -> bpf_insn;
    fn BPF_EXIT_INSN() -> bpf_insn;
    fn BPF_LD_MAP_VALUE(dst: c_int, map_fd: c_int, off: c_ulong) -> bpf_insn;
    fn BPF_ST_MEM(sz: c_int, dst: c_int, off: c_int, imm: c_int) -> bpf_insn;
    fn BPF_MOV64_REG(dst: c_int, src: c_int) -> bpf_insn;
    fn BPF_ALU64_IMM(op: c_int, dst: c_int, imm: c_int) -> bpf_insn;
    fn BPF_RAW_INSN(code: c_int, dst: c_int, src: c_int, off: c_int, imm: c_int) -> bpf_insn;
    fn BPF_CALL_REL(off: c_int) -> bpf_insn;
    fn BPF_EMIT_CALL(func: c_int) -> bpf_insn;
    fn BPF_LDX_MEM(sz: c_int, dst: c_int, src: c_int, off: c_int) -> bpf_insn;

    fn BTF_TYPE_INT_ENC(name: __u32, encoding: __u32, bits_offset: __u32, nr_bits: __u32, sz: __u32) -> __u32;
    fn BTF_TYPE_ENC(name: __u32, info: __u32, size_or_type: __u32) -> __u32;
    fn BTF_INFO_ENC(kind: __u32, kind_flag: __u32, vlen: __u32) -> __u32;
    fn BTF_PARAM_ENC(name: __u32, type_id: __u32) -> __u32;
    fn BTF_VAR_SECINFO_ENC(type_id: __u32, offset: __u32, size: __u32) -> __u32;
    fn BTF_TYPE_FLOAT_ENC(name: __u32, sz: __u32) -> __u32;
    fn BTF_TYPE_DECL_TAG_ENC(value: __u32, type_id: __u32, component_idx: c_int) -> __u32;
    fn BTF_TYPE_TYPE_TAG_ENC(value: __u32, type_id: __u32) -> __u32;
}

#[repr(C)]
pub struct bpf_insn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_attr {
    pub prog_type: __u32,
    pub insn_cnt: __u32,
    pub insns: __u64,
    pub license: __u64,
    pub prog_flags: __u32,
    pub prog_name: [c_char; 16],
    pub prog_token_fd: __u32,
}

#[repr(C)]
pub struct bpf_map_create_opts {
    pub token_fd: c_int,
    pub map_flags: __u32,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub expected_attach_type: c_int,
    pub token_fd: c_int,
    pub prog_flags: __u32,
    pub log_buf: *mut c_char,
    pub log_size: __u32,
    pub prog_btf_fd: c_int,
    pub func_info: *const bpf_func_info_min,
    pub func_info_cnt: __u32,
    pub func_info_rec_size: __u32,
}

#[repr(C)]
pub struct bpf_link_create_opts {
    pub uprobe_multi: bpf_uprobe_multi_opts,
}

#[repr(C)]
pub struct bpf_uprobe_multi_opts {
    pub path: *const c_char,
    pub offsets: *mut c_ulong,
    pub cnt: __u32,
    pub pid: c_int,
}

#[repr(C)]
pub struct bpf_btf_info {
    pub name: __u64,
    pub name_len: __u32,
}

#[repr(C)]
pub struct bpf_func_info_min {
    pub insn_off: __u32,
    pub type_id: __u32,
}

#[repr(C)]
pub struct btf_layout {
    pub type_off: __u32,
    pub type_len: __u32,
    pub flags: __u32,
}

#[repr(C)]
pub struct btf_header {
    pub magic: __u16,
    pub version: __u8,
    pub flags: __u8,
    pub hdr_len: __u32,
    pub type_off: __u32,
    pub type_len: __u32,
    pub str_off: __u32,
    pub str_len: __u32,
    pub layout_off: __u32,
    pub layout_len: __u32,
}

type __u8 = u8;
type __u16 = u16;

#[repr(C)]
pub struct kern_feature_cache {
    pub token_fd: c_int,
    pub res: [c_int; __FEAT_CNT as usize],
}

#[repr(C)]
pub enum kern_feature_id {
    FEAT_PROG_NAME,
    FEAT_GLOBAL_DATA,
    FEAT_BTF,
    FEAT_BTF_FUNC,
    FEAT_BTF_GLOBAL_FUNC,
    FEAT_BTF_DATASEC,
    FEAT_ARRAY_MMAP,
    FEAT_EXP_ATTACH_TYPE,
    FEAT_PROBE_READ_KERN,
    FEAT_PROG_BIND_MAP,
    FEAT_MODULE_BTF,
    FEAT_BTF_FLOAT,
    FEAT_PERF_LINK,
    FEAT_BTF_DECL_TAG,
    FEAT_BTF_TYPE_TAG,
    FEAT_MEMCG_ACCOUNT,
    FEAT_BPF_COOKIE,
    FEAT_BTF_ENUM64,
    FEAT_SYSCALL_WRAPPER,
    FEAT_UPROBE_MULTI_LINK,
    FEAT_ARG_CTX_TAG,
    FEAT_BTF_QMARK_DATASEC,
    FEAT_LDIMM64_FULL_RANGE_OFF,
    FEAT_UPROBE_SYSCALL,
    FEAT_BTF_LAYOUT,
    FEAT_BPF_SYSCALL_COMMON_ATTRS,
    FEAT_PERCPU_DATA,
}

const __FEAT_CNT: c_int = 27;
const FEAT_UNKNOWN: c_int = 0;
const FEAT_SUPPORTED: c_int = 1;
const FEAT_MISSING: c_int = 2;

extern "C" {
    static BPF_PROG_TYPE_SOCKET_FILTER: c_int;
    static BPF_PROG_TYPE_CGROUP_SOCK: c_int;
    static BPF_PROG_TYPE_TRACEPOINT: c_int;
    static BPF_PROG_TYPE_KPROBE: c_int;
    static BPF_MAP_TYPE_ARRAY: c_int;
    static BPF_MAP_TYPE_PERCPU_ARRAY: c_int;
    static BPF_F_TOKEN_FD: __u32;
    static BPF_F_MMAPABLE: __u32;
    static BPF_REG_0: c_int;
    static BPF_REG_1: c_int;
    static BPF_REG_2: c_int;
    static BPF_REG_3: c_int;
    static BPF_REG_10: c_int;
    static BPF_DW: c_int;
    static BPF_ADD: c_int;
    static BPF_JMP: c_int;
    static BPF_CALL: c_int;
    static BPF_FUNC_probe_read_kernel: c_int;
    static BPF_FUNC_get_attach_cookie: c_int;
    static BPF_FUNC_get_func_ip: c_int;
    static BPF_CGROUP_INET_SOCK_CREATE: c_int;
    static BPF_PERF_EVENT: c_int;
    static BPF_TRACE_UPROBE_MULTI: c_int;
    static BTF_INT_SIGNED: __u32;
    static BTF_KIND_FUNC_PROTO: __u32;
    static BTF_KIND_FUNC: __u32;
    static BTF_KIND_VAR: __u32;
    static BTF_KIND_DATASEC: __u32;
    static BTF_KIND_PTR: __u32;
    static BTF_KIND_ENUM64: __u32;
    static BTF_FUNC_GLOBAL: __u32;
    static BTF_VAR_STATIC: __u32;
    static BTF_MAGIC: __u16;
    static BTF_VERSION: __u8;
    static PROG_LOAD_ATTEMPTS: c_int;
    static EBADF: c_int;
    static EINVAL: c_int;
    static EPROTO: c_int;
}

#[inline]
unsafe fn ptr_to_u64(ptr: *const c_void) -> __u64 {
    ptr as c_ulong as __u64
}

#[no_mangle]
pub unsafe extern "C" fn probe_fd(fd: c_int) -> c_int {
    if fd >= 0 {
        close(fd);
    }
    (fd >= 0) as c_int
}

unsafe fn probe_kern_prog_name(token_fd: c_int) -> c_int {
    let attr_sz = size_of::<bpf_attr>();
    let mut insns = [BPF_MOV64_IMM(BPF_REG_0, 0), BPF_EXIT_INSN()];
    let mut attr: bpf_attr = zeroed();
    let ret: c_int;

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.prog_type = BPF_PROG_TYPE_SOCKET_FILTER as __u32;
    attr.license = ptr_to_u64(c"GPL".as_ptr() as *const c_void);
    attr.insns = ptr_to_u64(insns.as_mut_ptr() as *const c_void);
    attr.insn_cnt = insns.len() as __u32;
    attr.prog_token_fd = token_fd as __u32;
    if token_fd != 0 {
        attr.prog_flags |= BPF_F_TOKEN_FD;
    }
    libbpf_strlcpy(attr.prog_name.as_mut_ptr(), c"libbpf_nametest".as_ptr(), attr.prog_name.len());

    /* make sure loading with name works */
    ret = sys_bpf_prog_load(&mut attr, attr_sz, PROG_LOAD_ATTEMPTS);
    probe_fd(ret)
}

unsafe fn probe_kern_global_data(token_fd: c_int) -> c_int {
    let mut insns = [
        BPF_LD_MAP_VALUE(BPF_REG_1, 0, 16),
        BPF_ST_MEM(BPF_DW, BPF_REG_1, 0, 42),
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let map_opts = bpf_map_create_opts {
        token_fd,
        map_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
    };
    let prog_opts = bpf_prog_load_opts {
        token_fd,
        prog_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
        ..zeroed()
    };
    let mut ret: c_int;
    let map: c_int;
    let insn_cnt = insns.len() as c_int;

    map = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"libbpf_global".as_ptr(), size_of::<c_int>() as __u32, 32, 1, &map_opts);
    if map < 0 {
        ret = -errno;
        pr_warn(c"Error in %s(): %s. Couldn't create simple array map.\n".as_ptr(), c"probe_kern_global_data".as_ptr(), errstr(ret));
        return ret;
    }

    (*insns.as_mut_ptr()).imm = map;

    ret = bpf_prog_load(BPF_PROG_TYPE_SOCKET_FILTER, ptr::null(), c"GPL".as_ptr(), insns.as_ptr(), insn_cnt, &prog_opts);
    close(map);
    probe_fd(ret)
}

unsafe fn load_raw_btf_probe(types: *mut __u32, types_len: usize, strs: *const c_char, strs_len: usize, token_fd: c_int) -> c_int {
    probe_fd(libbpf__load_raw_btf(types as *mut c_char, types_len, strs, strs_len, token_fd))
}

unsafe fn probe_kern_btf(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0int\0";
    let mut types = [BTF_TYPE_INT_ENC(1, BTF_INT_SIGNED, 0, 32, 4)];

    load_raw_btf_probe(types.as_mut_ptr(), size_of_val(&types), STRS.as_ptr() as *const c_char, STRS.len(), token_fd)
}

unsafe fn probe_kern_btf_func(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0int\0x\0a\0";
    /* void x(int a) {} */
    let mut types = [
        /* int */
        BTF_TYPE_INT_ENC(1, BTF_INT_SIGNED, 0, 32, 4), /* [1] */
        /* FUNC_PROTO */ /* [2] */
        BTF_TYPE_ENC(0, BTF_INFO_ENC(BTF_KIND_FUNC_PROTO, 0, 1), 0),
        BTF_PARAM_ENC(7, 1),
        /* FUNC x */ /* [3] */
        BTF_TYPE_ENC(5, BTF_INFO_ENC(BTF_KIND_FUNC, 0, 0), 2),
    ];

    load_raw_btf_probe(types.as_mut_ptr(), size_of_val(&types), STRS.as_ptr() as *const c_char, STRS.len(), token_fd)
}

unsafe fn probe_kern_btf_func_global(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0int\0x\0a\0";
    /* static void x(int a) {} */
    let mut types = [
        /* int */
        BTF_TYPE_INT_ENC(1, BTF_INT_SIGNED, 0, 32, 4), /* [1] */
        /* FUNC_PROTO */ /* [2] */
        BTF_TYPE_ENC(0, BTF_INFO_ENC(BTF_KIND_FUNC_PROTO, 0, 1), 0),
        BTF_PARAM_ENC(7, 1),
        /* FUNC x BTF_FUNC_GLOBAL */ /* [3] */
        BTF_TYPE_ENC(5, BTF_INFO_ENC(BTF_KIND_FUNC, 0, BTF_FUNC_GLOBAL), 2),
    ];

    load_raw_btf_probe(types.as_mut_ptr(), size_of_val(&types), STRS.as_ptr() as *const c_char, STRS.len(), token_fd)
}

unsafe fn probe_kern_btf_datasec(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0x\0.data\0";
    /* static int a; */
    let mut types = [
        /* int */
        BTF_TYPE_INT_ENC(0, BTF_INT_SIGNED, 0, 32, 4), /* [1] */
        /* VAR x */ /* [2] */
        BTF_TYPE_ENC(1, BTF_INFO_ENC(BTF_KIND_VAR, 0, 0), 1),
        BTF_VAR_STATIC,
        /* DATASEC val */ /* [3] */
        BTF_TYPE_ENC(3, BTF_INFO_ENC(BTF_KIND_DATASEC, 0, 1), 4),
        BTF_VAR_SECINFO_ENC(2, 0, 4),
    ];

    load_raw_btf_probe(types.as_mut_ptr(), size_of_val(&types), STRS.as_ptr() as *const c_char, STRS.len(), token_fd)
}

unsafe fn probe_kern_btf_qmark_datasec(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0x\0?.data\0";
    /* static int a; */
    let mut types = [
        /* int */
        BTF_TYPE_INT_ENC(0, BTF_INT_SIGNED, 0, 32, 4), /* [1] */
        /* VAR x */ /* [2] */
        BTF_TYPE_ENC(1, BTF_INFO_ENC(BTF_KIND_VAR, 0, 0), 1),
        BTF_VAR_STATIC,
        /* DATASEC ?.data */ /* [3] */
        BTF_TYPE_ENC(3, BTF_INFO_ENC(BTF_KIND_DATASEC, 0, 1), 4),
        BTF_VAR_SECINFO_ENC(2, 0, 4),
    ];

    load_raw_btf_probe(types.as_mut_ptr(), size_of_val(&types), STRS.as_ptr() as *const c_char, STRS.len(), token_fd)
}

unsafe fn probe_kern_btf_float(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0float\0";
    let mut types = [BTF_TYPE_FLOAT_ENC(1, 4)];

    load_raw_btf_probe(types.as_mut_ptr(), size_of_val(&types), STRS.as_ptr() as *const c_char, STRS.len(), token_fd)
}

unsafe fn probe_kern_btf_decl_tag(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0tag\0";
    let mut types = [
        /* int */
        BTF_TYPE_INT_ENC(0, BTF_INT_SIGNED, 0, 32, 4), /* [1] */
        /* VAR x */ /* [2] */
        BTF_TYPE_ENC(1, BTF_INFO_ENC(BTF_KIND_VAR, 0, 0), 1),
        BTF_VAR_STATIC,
        /* attr */
        BTF_TYPE_DECL_TAG_ENC(1, 2, -1),
    ];

    load_raw_btf_probe(types.as_mut_ptr(), size_of_val(&types), STRS.as_ptr() as *const c_char, STRS.len(), token_fd)
}

unsafe fn probe_kern_btf_type_tag(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0tag\0";
    let mut types = [
        /* int */
        BTF_TYPE_INT_ENC(0, BTF_INT_SIGNED, 0, 32, 4), /* [1] */
        /* attr */
        BTF_TYPE_TYPE_TAG_ENC(1, 1), /* [2] */
        /* ptr */
        BTF_TYPE_ENC(0, BTF_INFO_ENC(BTF_KIND_PTR, 0, 0), 2), /* [3] */
    ];

    load_raw_btf_probe(types.as_mut_ptr(), size_of_val(&types), STRS.as_ptr() as *const c_char, STRS.len(), token_fd)
}

unsafe fn probe_kern_array_mmap(token_fd: c_int) -> c_int {
    let opts = bpf_map_create_opts {
        map_flags: BPF_F_MMAPABLE | if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
        token_fd,
    };
    let fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"libbpf_mmap".as_ptr(), size_of::<c_int>() as __u32, size_of::<c_int>() as __u32, 1, &opts);
    probe_fd(fd)
}

unsafe fn probe_kern_exp_attach_type(token_fd: c_int) -> c_int {
    let opts = bpf_prog_load_opts {
        expected_attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        token_fd,
        prog_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
        ..zeroed()
    };
    let insns = [BPF_MOV64_IMM(BPF_REG_0, 0), BPF_EXIT_INSN()];
    let fd: c_int;
    let insn_cnt = insns.len() as c_int;

    /* use any valid combination of program type and (optional)
     * non-zero expected attach type (i.e., not a BPF_CGROUP_INET_INGRESS)
     * to see if kernel supports expected_attach_type field for
     * BPF_PROG_LOAD command
     */
    fd = bpf_prog_load(BPF_PROG_TYPE_CGROUP_SOCK, ptr::null(), c"GPL".as_ptr(), insns.as_ptr(), insn_cnt, &opts);
    probe_fd(fd)
}

unsafe fn probe_kern_probe_read_kernel(token_fd: c_int) -> c_int {
    let opts = bpf_prog_load_opts {
        token_fd,
        prog_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
        ..zeroed()
    };
    let insns = [
        BPF_MOV64_REG(BPF_REG_1, BPF_REG_10), /* r1 = r10 (fp) */
        BPF_ALU64_IMM(BPF_ADD, BPF_REG_1, -8), /* r1 += -8 */
        BPF_MOV64_IMM(BPF_REG_2, 8),          /* r2 = 8 */
        BPF_MOV64_IMM(BPF_REG_3, 0),          /* r3 = 0 */
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_probe_read_kernel),
        BPF_EXIT_INSN(),
    ];
    let fd = bpf_prog_load(BPF_PROG_TYPE_TRACEPOINT, ptr::null(), c"GPL".as_ptr(), insns.as_ptr(), insns.len() as c_int, &opts);
    probe_fd(fd)
}

unsafe fn probe_prog_bind_map(token_fd: c_int) -> c_int {
    let insns = [BPF_MOV64_IMM(BPF_REG_0, 0), BPF_EXIT_INSN()];
    let map_opts = bpf_map_create_opts {
        token_fd,
        map_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
    };
    let prog_opts = bpf_prog_load_opts {
        token_fd,
        prog_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
        ..zeroed()
    };
    let ret: c_int;
    let map: c_int;
    let prog: c_int;
    let insn_cnt = insns.len() as c_int;

    map = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"libbpf_det_bind".as_ptr(), size_of::<c_int>() as __u32, 32, 1, &map_opts);
    if map < 0 {
        let e = -errno;
        pr_warn(c"Error in %s(): %s. Couldn't create simple array map.\n".as_ptr(), c"probe_prog_bind_map".as_ptr(), errstr(e));
        return e;
    }

    prog = bpf_prog_load(BPF_PROG_TYPE_SOCKET_FILTER, ptr::null(), c"GPL".as_ptr(), insns.as_ptr(), insn_cnt, &prog_opts);
    if prog < 0 {
        close(map);
        return 0;
    }

    ret = bpf_prog_bind_map(prog, map, ptr::null());

    close(map);
    close(prog);

    (ret >= 0) as c_int
}

unsafe fn probe_module_btf(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0int\0";
    let mut types = [BTF_TYPE_INT_ENC(1, BTF_INT_SIGNED, 0, 32, 4)];
    let mut info: bpf_btf_info = zeroed();
    let mut len = size_of::<bpf_btf_info>() as __u32;
    let mut name = [0 as c_char; 16];
    let fd: c_int;
    let err: c_int;

    fd = libbpf__load_raw_btf(types.as_mut_ptr() as *mut c_char, size_of_val(&types), STRS.as_ptr() as *const c_char, STRS.len(), token_fd);
    if fd < 0 {
        return 0; /* BTF not supported at all */
    }

    memset(&mut info as *mut _ as *mut c_void, 0, size_of::<bpf_btf_info>());
    info.name = ptr_to_u64(name.as_mut_ptr() as *const c_void);
    info.name_len = name.len() as __u32;

    /* check that BPF_OBJ_GET_INFO_BY_FD supports specifying name pointer;
     * kernel's module BTF support coincides with support for
     * name/name_len fields in struct bpf_btf_info.
     */
    err = bpf_btf_get_info_by_fd(fd, &mut info, &mut len);
    close(fd);
    (err == 0) as c_int
}

unsafe fn probe_perf_link(token_fd: c_int) -> c_int {
    let insns = [BPF_MOV64_IMM(BPF_REG_0, 0), BPF_EXIT_INSN()];
    let opts = bpf_prog_load_opts {
        token_fd,
        prog_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
        ..zeroed()
    };
    let prog_fd: c_int;
    let link_fd: c_int;
    let err: c_int;

    prog_fd = bpf_prog_load(BPF_PROG_TYPE_TRACEPOINT, ptr::null(), c"GPL".as_ptr(), insns.as_ptr(), insns.len() as c_int, &opts);
    if prog_fd < 0 {
        return -errno;
    }

    /* use invalid perf_event FD to get EBADF, if link is supported;
     * otherwise EINVAL should be returned
     */
    link_fd = bpf_link_create(prog_fd, -1, BPF_PERF_EVENT, ptr::null());
    err = -errno; /* close() can clobber errno */

    if link_fd >= 0 {
        close(link_fd);
    }
    close(prog_fd);

    (link_fd < 0 && err == -EBADF) as c_int
}

unsafe fn probe_uprobe_multi_link(token_fd: c_int) -> c_int {
    let load_opts = bpf_prog_load_opts {
        expected_attach_type: BPF_TRACE_UPROBE_MULTI,
        token_fd,
        prog_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
        ..zeroed()
    };
    let mut link_opts: bpf_link_create_opts = zeroed();
    let insns = [BPF_MOV64_IMM(BPF_REG_0, 0), BPF_EXIT_INSN()];
    let prog_fd: c_int;
    let mut link_fd: c_int;
    let mut err: c_int;
    let mut offset: c_ulong = 0;

    prog_fd = bpf_prog_load(BPF_PROG_TYPE_KPROBE, ptr::null(), c"GPL".as_ptr(), insns.as_ptr(), insns.len() as c_int, &load_opts);
    if prog_fd < 0 {
        return -errno;
    }

    /* Creating uprobe in '/' binary should fail with -EBADF. */
    link_opts.uprobe_multi.path = c"/".as_ptr();
    link_opts.uprobe_multi.offsets = &mut offset;
    link_opts.uprobe_multi.cnt = 1;

    link_fd = bpf_link_create(prog_fd, -1, BPF_TRACE_UPROBE_MULTI, &link_opts);
    err = -errno; /* close() can clobber errno */

    if link_fd >= 0 || err != -EBADF {
        if link_fd >= 0 {
            close(link_fd);
        }
        close(prog_fd);
        return 0;
    }

    /* Initial multi-uprobe support in kernel didn't handle PID filtering
     * correctly (it was doing thread filtering, not process filtering).
     * So now we'll detect if PID filtering logic was fixed, and, if not,
     * we'll pretend multi-uprobes are not supported, if not.
     * Multi-uprobes are used in USDT attachment logic, and we need to be
     * conservative here, because multi-uprobe selection happens early at
     * load time, while the use of PID filtering is known late at
     * attachment time, at which point it's too late to undo multi-uprobe
     * selection.
     *
     * Creating uprobe with pid == -1 for (invalid) '/' binary will fail
     * early with -EINVAL on kernels with fixed PID filtering logic;
     * otherwise -ESRCH would be returned if passed correct binary path
     * (but we'll just get -BADF, of course).
     */
    link_opts.uprobe_multi.pid = -1; /* invalid PID */
    link_opts.uprobe_multi.path = c"/".as_ptr(); /* invalid path */
    link_opts.uprobe_multi.offsets = &mut offset;
    link_opts.uprobe_multi.cnt = 1;

    link_fd = bpf_link_create(prog_fd, -1, BPF_TRACE_UPROBE_MULTI, &link_opts);
    err = -errno; /* close() can clobber errno */

    if link_fd >= 0 {
        close(link_fd);
    }
    close(prog_fd);

    (link_fd < 0 && err == -EINVAL) as c_int
}

unsafe fn probe_kern_bpf_cookie(token_fd: c_int) -> c_int {
    let insns = [
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_attach_cookie),
        BPF_EXIT_INSN(),
    ];
    let opts = bpf_prog_load_opts {
        token_fd,
        prog_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
        ..zeroed()
    };
    let ret = bpf_prog_load(BPF_PROG_TYPE_TRACEPOINT, ptr::null(), c"GPL".as_ptr(), insns.as_ptr(), insns.len() as c_int, &opts);
    probe_fd(ret)
}

unsafe fn probe_kern_btf_enum64(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0enum64\0";
    let mut types = [BTF_TYPE_ENC(1, BTF_INFO_ENC(BTF_KIND_ENUM64, 0, 0), 8)];

    load_raw_btf_probe(types.as_mut_ptr(), size_of_val(&types), STRS.as_ptr() as *const c_char, STRS.len(), token_fd)
}

unsafe fn probe_kern_arg_ctx_tag(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0a\0b\0arg:ctx\0";
    let mut types = [
        /* [1] INT */
        BTF_TYPE_INT_ENC(1 /* "a" */, BTF_INT_SIGNED, 0, 32, 4),
        /* [2] PTR -> VOID */
        BTF_TYPE_ENC(0, BTF_INFO_ENC(BTF_KIND_PTR, 0, 0), 0),
        /* [3] FUNC_PROTO `int(void *a)` */
        BTF_TYPE_ENC(0, BTF_INFO_ENC(BTF_KIND_FUNC_PROTO, 0, 1), 1),
        BTF_PARAM_ENC(1 /* "a" */, 2),
        /* [4] FUNC 'a' -> FUNC_PROTO (main prog) */
        BTF_TYPE_ENC(1 /* "a" */, BTF_INFO_ENC(BTF_KIND_FUNC, 0, BTF_FUNC_GLOBAL), 3),
        /* [5] FUNC_PROTO `int(void *b __arg_ctx)` */
        BTF_TYPE_ENC(0, BTF_INFO_ENC(BTF_KIND_FUNC_PROTO, 0, 1), 1),
        BTF_PARAM_ENC(3 /* "b" */, 2),
        /* [6] FUNC 'b' -> FUNC_PROTO (subprog) */
        BTF_TYPE_ENC(3 /* "b" */, BTF_INFO_ENC(BTF_KIND_FUNC, 0, BTF_FUNC_GLOBAL), 5),
        /* [7] DECL_TAG 'arg:ctx' -> func 'b' arg 'b' */
        BTF_TYPE_DECL_TAG_ENC(5 /* "arg:ctx" */, 6, 0),
    ];
    let insns = [
        /* main prog */
        BPF_CALL_REL(1),
        BPF_EXIT_INSN(),
        /* global subprog */
        BPF_EMIT_CALL(BPF_FUNC_get_func_ip), /* needs PTR_TO_CTX */
        BPF_EXIT_INSN(),
    ];
    let func_infos = [
        bpf_func_info_min { insn_off: 0, type_id: 4 }, /* main prog -> FUNC 'a' */
        bpf_func_info_min { insn_off: 2, type_id: 6 }, /* subprog -> FUNC 'b' */
    ];
    let mut opts = bpf_prog_load_opts {
        token_fd,
        prog_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
        ..zeroed()
    };
    let prog_fd: c_int;
    let btf_fd: c_int;
    let insn_cnt = insns.len() as c_int;

    btf_fd = libbpf__load_raw_btf(types.as_mut_ptr() as *mut c_char, size_of_val(&types), STRS.as_ptr() as *const c_char, STRS.len(), token_fd);
    if btf_fd < 0 {
        return 0;
    }

    opts.prog_btf_fd = btf_fd;
    opts.func_info = func_infos.as_ptr();
    opts.func_info_cnt = func_infos.len() as __u32;
    opts.func_info_rec_size = size_of::<bpf_func_info_min>() as __u32;

    prog_fd = bpf_prog_load(BPF_PROG_TYPE_KPROBE, c"det_arg_ctx".as_ptr(), c"GPL".as_ptr(), insns.as_ptr(), insn_cnt, &opts);
    close(btf_fd);

    probe_fd(prog_fd)
}

unsafe fn probe_ldimm64_full_range_off(token_fd: c_int) -> c_int {
    let mut log_buf = [0 as c_char; 1024];
    let prog_fd: c_int;
    let map_fd: c_int;
    let mut ret: c_int;
    let map_opts = bpf_map_create_opts {
        token_fd,
        map_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
    };
    let prog_opts = bpf_prog_load_opts {
        token_fd,
        prog_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
        log_buf: log_buf.as_mut_ptr(),
        log_size: log_buf.len() as __u32,
        ..zeroed()
    };
    let mut insns = [BPF_LD_MAP_VALUE(BPF_REG_1, 0, 1u64.wrapping_shl(30) as c_ulong), BPF_EXIT_INSN()];
    let insn_cnt = insns.len() as c_int;

    map_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"arr".as_ptr(), size_of::<c_int>() as __u32, 1, 1, &map_opts);
    if map_fd < 0 {
        ret = -errno;
        pr_warn(c"Error in %s(): %s. Couldn't create simple array map.\n".as_ptr(), c"probe_ldimm64_full_range_off".as_ptr(), errstr(ret));
        return ret;
    }
    (*insns.as_mut_ptr()).imm = map_fd;

    log_buf[0] = 0;
    prog_fd = bpf_prog_load(BPF_PROG_TYPE_SOCKET_FILTER, c"global_reloc".as_ptr(), c"GPL".as_ptr(), insns.as_ptr(), insn_cnt, &prog_opts);
    ret = -errno;

    close(map_fd);

    if prog_fd >= 0 {
        pr_warn(c"Error in %s(): Program loading unexpectedly succeeded.\n".as_ptr(), c"probe_ldimm64_full_range_off".as_ptr());
        close(prog_fd);
        return -EINVAL;
    }

    /*
     * Feature is allowed if we're not failing with the error message
     * "direct value offset of %u is not allowed" removed in
     * 12a1fe6e12db ("bpf/verifier: Do not limit maximum direct offset into arena map").
     * We should instead fail with "invalid access to map value pointer".
     * Ensure we match with one of the two and we're not failing with a
     * different, unexpected message.
     */
    if !strstr(log_buf.as_ptr(), c"direct value offset of".as_ptr()).is_null() {
        return 0;
    }

    if strstr(log_buf.as_ptr(), c"invalid access to map value pointer".as_ptr()).is_null() {
        pr_warn(c"Error in %s(): Program unexpectedly failed with message: %s.\n".as_ptr(), c"probe_ldimm64_full_range_off".as_ptr(), log_buf.as_ptr());
        return ret;
    }

    1
}

/* C source uses #ifdef __x86_64__; preserve the architecture-specific syscall probe. */
#[cfg(target_arch = "x86_64")]
const __NR_uprobe: c_long = 336;

#[cfg(target_arch = "x86_64")]
unsafe fn probe_uprobe_syscall(_token_fd: c_int) -> c_int {
    /*
     * If kernel supports uprobe() syscall, it will return -EPROTO when called
     * from the outside of a kernel-generated uprobe trampoline.
     */
    (syscall(__NR_uprobe) < 0 && errno == EPROTO) as c_int
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn probe_uprobe_syscall(_token_fd: c_int) -> c_int {
    0
}

unsafe fn probe_kern_btf_layout(token_fd: c_int) -> c_int {
    static STRS: &[u8] = b"\0int\0";
    let mut types = [BTF_TYPE_INT_ENC(1, BTF_INT_SIGNED, 0, 32, 4)];
    let mut layout = [
        btf_layout { type_off: 0, type_len: 0, flags: 0 },
        btf_layout { type_off: size_of::<__u32>() as __u32, type_len: 0, flags: 0 },
    ];
    let hdr = btf_header {
        magic: BTF_MAGIC,
        version: BTF_VERSION,
        flags: 0,
        hdr_len: size_of::<btf_header>() as __u32,
        type_off: 0,
        type_len: size_of_val(&types) as __u32,
        str_off: (size_of_val(&types) + size_of_val(&layout)) as __u32,
        str_len: STRS.len() as __u32,
        layout_off: size_of_val(&types) as __u32,
        layout_len: size_of_val(&layout) as __u32,
    };

    probe_fd(libbpf__load_raw_btf_hdr(&hdr, types.as_mut_ptr() as *mut c_char, STRS.as_ptr() as *const c_char, layout.as_mut_ptr() as *mut c_char, token_fd))
}

unsafe fn probe_bpf_syscall_common_attrs(_token_fd: c_int) -> c_int {
    probe_sys_bpf_ext()
}

unsafe fn probe_kern_percpu_data(token_fd: c_int) -> c_int {
    let mut insns = [
        BPF_LD_MAP_VALUE(BPF_REG_1, 0, 0),
        BPF_LDX_MEM(BPF_DW, BPF_REG_0, BPF_REG_1, 0),
        BPF_EXIT_INSN(),
    ];
    let map_opts = bpf_map_create_opts {
        token_fd,
        map_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
    };
    let prog_opts = bpf_prog_load_opts {
        token_fd,
        prog_flags: if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 },
        ..zeroed()
    };
    let ret: c_int;
    let map: c_int;
    let insn_cnt = insns.len() as c_int;

    map = bpf_map_create(BPF_MAP_TYPE_PERCPU_ARRAY, c"libbpf_percpu".as_ptr(), size_of::<c_int>() as __u32, 8, 1, &map_opts);
    if map < 0 {
        pr_warn(c"Error in %s(): %s. Couldn't create simple percpu_array map.\n".as_ptr(), c"probe_kern_percpu_data".as_ptr(), errstr(map));
        return map;
    }

    (*insns.as_mut_ptr()).imm = map;

    ret = bpf_prog_load(BPF_PROG_TYPE_SOCKET_FILTER, ptr::null(), c"GPL".as_ptr(), insns.as_ptr(), insn_cnt, &prog_opts);
    close(map);
    probe_fd(ret)
}

type feature_probe_fn = unsafe fn(c_int) -> c_int;

static mut feature_cache: kern_feature_cache = kern_feature_cache {
    token_fd: 0,
    res: [FEAT_UNKNOWN; __FEAT_CNT as usize],
};

#[repr(C)]
struct kern_feature_desc {
    desc: *const c_char,
    probe: feature_probe_fn,
}

static mut feature_probes: [kern_feature_desc; __FEAT_CNT as usize] = [
    kern_feature_desc { desc: c"BPF program name".as_ptr(), probe: probe_kern_prog_name },
    kern_feature_desc { desc: c"global variables".as_ptr(), probe: probe_kern_global_data },
    kern_feature_desc { desc: c"minimal BTF".as_ptr(), probe: probe_kern_btf },
    kern_feature_desc { desc: c"BTF functions".as_ptr(), probe: probe_kern_btf_func },
    kern_feature_desc { desc: c"BTF global function".as_ptr(), probe: probe_kern_btf_func_global },
    kern_feature_desc { desc: c"BTF data section and variable".as_ptr(), probe: probe_kern_btf_datasec },
    kern_feature_desc { desc: c"ARRAY map mmap()".as_ptr(), probe: probe_kern_array_mmap },
    kern_feature_desc { desc: c"BPF_PROG_LOAD expected_attach_type attribute".as_ptr(), probe: probe_kern_exp_attach_type },
    kern_feature_desc { desc: c"bpf_probe_read_kernel() helper".as_ptr(), probe: probe_kern_probe_read_kernel },
    kern_feature_desc { desc: c"BPF_PROG_BIND_MAP support".as_ptr(), probe: probe_prog_bind_map },
    kern_feature_desc { desc: c"module BTF support".as_ptr(), probe: probe_module_btf },
    kern_feature_desc { desc: c"BTF_KIND_FLOAT support".as_ptr(), probe: probe_kern_btf_float },
    kern_feature_desc { desc: c"BPF perf link support".as_ptr(), probe: probe_perf_link },
    kern_feature_desc { desc: c"BTF_KIND_DECL_TAG support".as_ptr(), probe: probe_kern_btf_decl_tag },
    kern_feature_desc { desc: c"BTF_KIND_TYPE_TAG support".as_ptr(), probe: probe_kern_btf_type_tag },
    kern_feature_desc { desc: c"memcg-based memory accounting".as_ptr(), probe: probe_memcg_account },
    kern_feature_desc { desc: c"BPF cookie support".as_ptr(), probe: probe_kern_bpf_cookie },
    kern_feature_desc { desc: c"BTF_KIND_ENUM64 support".as_ptr(), probe: probe_kern_btf_enum64 },
    kern_feature_desc { desc: c"Kernel using syscall wrapper".as_ptr(), probe: probe_kern_syscall_wrapper },
    kern_feature_desc { desc: c"BPF multi-uprobe link support".as_ptr(), probe: probe_uprobe_multi_link },
    kern_feature_desc { desc: c"kernel-side __arg_ctx tag".as_ptr(), probe: probe_kern_arg_ctx_tag },
    kern_feature_desc { desc: c"BTF DATASEC names starting from '?'".as_ptr(), probe: probe_kern_btf_qmark_datasec },
    kern_feature_desc { desc: c"full range LDIMM64 support".as_ptr(), probe: probe_ldimm64_full_range_off },
    kern_feature_desc { desc: c"kernel supports uprobe syscall".as_ptr(), probe: probe_uprobe_syscall },
    kern_feature_desc { desc: c"kernel supports BTF layout".as_ptr(), probe: probe_kern_btf_layout },
    kern_feature_desc { desc: c"BPF syscall common attributes support".as_ptr(), probe: probe_bpf_syscall_common_attrs },
    kern_feature_desc { desc: c"kernel supports percpu data".as_ptr(), probe: probe_kern_percpu_data },
];

unsafe fn READ_ONCE(p: *const c_int) -> c_int {
    core::ptr::read_volatile(p)
}

unsafe fn WRITE_ONCE(p: *mut c_int, v: c_int) {
    core::ptr::write_volatile(p, v);
}

#[no_mangle]
pub unsafe extern "C" fn feat_supported(mut cache: *mut kern_feature_cache, feat_id: kern_feature_id) -> bool {
    let idx = feat_id as usize;
    let feat = &mut feature_probes[idx] as *mut kern_feature_desc;
    let ret: c_int;

    /* assume global feature cache, unless custom one is provided */
    if cache.is_null() {
        cache = &mut feature_cache;
    }

    if READ_ONCE((*cache).res.as_ptr().add(idx)) == FEAT_UNKNOWN {
        ret = ((*feat).probe)((*cache).token_fd);
        if ret > 0 {
            WRITE_ONCE((*cache).res.as_mut_ptr().add(idx), FEAT_SUPPORTED);
        } else if ret == 0 {
            WRITE_ONCE((*cache).res.as_mut_ptr().add(idx), FEAT_MISSING);
        } else {
            pr_warn(c"Detection of kernel %s support failed: %s\n".as_ptr(), (*feat).desc, errstr(ret));
            WRITE_ONCE((*cache).res.as_mut_ptr().add(idx), FEAT_MISSING);
        }
    }

    READ_ONCE((*cache).res.as_ptr().add(idx)) == FEAT_SUPPORTED
}

fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
