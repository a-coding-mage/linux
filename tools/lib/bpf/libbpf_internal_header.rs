/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * Internal libbpf helpers.
 *
 * Copyright (c) 2019 Facebook
 */

/* C includes and header guards are intentionally omitted in this Rust translation.
 * External symbols from libbpf.h, btf.h, relo_core.h, libelf, libc, and Linux UAPI
 * are represented as FFI declarations or referenced by their original names.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

/* Android's libc doesn't support AT_EACCESS in faccessat() implementation.
 * In C this is redefined to 0 under __ANDROID__.
 */
#[cfg(target_os = "android")]
pub const AT_EACCESS: c_int = 0;

/* GCC poison pragmas for kernel-only typedefs and reallocarray have no Rust equivalent. */

pub const EM_BPF: c_int = 247;

pub const R_BPF_64_64: c_int = 1;
pub const R_BPF_64_ABS64: c_int = 2;
pub const R_BPF_64_ABS32: c_int = 3;
pub const R_BPF_64_32: c_int = 10;

pub const SHT_LLVM_ADDRSIG: c_uint = 0x6FFF4C03;

/* If libelf is old and doesn't support mmap(), C falls back to ELF_C_READ. */
pub const JUMPTABLES_SEC: &[u8] = b".jumptables\0";

pub const PROG_LOAD_ATTEMPTS: c_int = 5;
pub const SHA256_DIGEST_LENGTH: usize = 32;

#[inline]
pub const fn btf_info_enc(kind: u32, kind_flag: bool, vlen: u32) -> u32 {
    ((kind_flag as u32) << 31) | (kind << 24) | (vlen & BTF_MAX_VLEN)
}

#[inline]
pub const fn btf_int_enc(encoding: u32, bits_offset: u32, nr_bits: u32) -> u32 {
    (encoding << 24) | (bits_offset << 16) | nr_bits
}

#[inline]
pub unsafe fn str_has_sfx(str_: *const c_char, sfx: *const c_char) -> bool {
    let str_len = strlen(str_);
    let sfx_len = strlen(sfx);

    if sfx_len > str_len {
        return false;
    }
    strcmp(str_.add(str_len - sfx_len), sfx) == 0
}

/* Symbol versioning macros differ for shared/static C builds and have no direct
 * item-level Rust equivalent here.
 */

unsafe extern "C" {
    pub fn libbpf_print(level: libbpf_print_level, format: *const c_char, ...);
    pub fn libbpf_errstr(err: c_int) -> *const c_char;
}

#[inline]
pub unsafe fn errstr(err: c_int) -> *const c_char {
    libbpf_errstr(err)
}

#[repr(C)]
pub struct bpf_link {
    pub detach: Option<unsafe extern "C" fn(link: *mut bpf_link) -> c_int>,
    pub dealloc: Option<unsafe extern "C" fn(link: *mut bpf_link)>,
    pub pin_path: *mut c_char, /* NULL, if not pinned */
    pub fd: c_int,            /* hook FD, -1 if not applicable */
    pub disconnected: bool,
}

#[inline]
pub unsafe fn libbpf_reallocarray(ptr: *mut c_void, nmemb: usize, size: usize) -> *mut c_void {
    let Some(total) = nmemb.checked_mul(size) else {
        return core::ptr::null_mut();
    };
    realloc(ptr, total)
}

#[inline]
pub unsafe fn libbpf_strlcpy(dst: *mut c_char, src: *const c_char, mut sz: usize) {
    let mut i: usize;

    if sz == 0 {
        return;
    }

    sz -= 1;
    i = 0;
    while i < sz && *src.add(i) != 0 {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
    *dst.add(i) = b'\0' as c_char;
}

unsafe extern "C" {
    pub fn get_kernel_version() -> __u32;

    pub fn btf_type_by_id(btf: *const btf, type_id: __u32) -> *mut btf_type;
    pub fn btf_kind_str(t: *const btf_type) -> *const c_char;
    pub fn skip_mods_and_typedefs(btf: *const btf, id: __u32, res_id: *mut __u32) -> *const btf_type;
    pub fn btf_header(btf: *const btf) -> *const btf_header;
    pub fn btf_set_base_btf(btf: *mut btf, base_btf: *const btf);
    pub fn btf_relocate(btf: *mut btf, base_btf: *const btf, id_map: *mut *mut __u32) -> c_int;
    pub fn btf_type_is_traceable_func(btf: *const btf, t: *const btf_type) -> bool;
}

#[inline]
pub unsafe fn btf_func_linkage(t: *const btf_type) -> btf_func_linkage {
    btf_vlen(t) as c_int as btf_func_linkage
}

#[inline]
pub const fn btf_type_info(kind: c_int, vlen: c_int, kflag: c_int) -> __u32 {
    ((kflag as __u32) << 31) | ((kind as __u32) << 24) | (vlen as __u32)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum map_def_parts {
    MAP_DEF_MAP_TYPE = 0x001,
    MAP_DEF_KEY_TYPE = 0x002,
    MAP_DEF_KEY_SIZE = 0x004,
    MAP_DEF_VALUE_TYPE = 0x008,
    MAP_DEF_VALUE_SIZE = 0x010,
    MAP_DEF_MAX_ENTRIES = 0x020,
    MAP_DEF_MAP_FLAGS = 0x040,
    MAP_DEF_NUMA_NODE = 0x080,
    MAP_DEF_PINNING = 0x100,
    MAP_DEF_INNER_MAP = 0x200,
    MAP_DEF_MAP_EXTRA = 0x400,
    MAP_DEF_ALL = 0x7ff, /* combination of all above */
}

#[repr(C)]
pub struct btf_map_def {
    pub parts: map_def_parts,
    pub map_type: __u32,
    pub key_type_id: __u32,
    pub key_size: __u32,
    pub value_type_id: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
    pub numa_node: __u32,
    pub pinning: __u32,
    pub map_extra: __u64,
}

unsafe extern "C" {
    pub fn parse_btf_map_def(
        map_name: *const c_char,
        btf: *mut btf,
        def_t: *const btf_type,
        strict: bool,
        map_def: *mut btf_map_def,
        inner_def: *mut btf_map_def,
    ) -> c_int;

    pub fn libbpf_add_mem(
        data: *mut *mut c_void,
        cap_cnt: *mut usize,
        elem_sz: usize,
        cur_cnt: usize,
        max_cnt: usize,
        add_cnt: usize,
    ) -> *mut c_void;
    pub fn libbpf_ensure_mem(
        data: *mut *mut c_void,
        cap_cnt: *mut usize,
        elem_sz: usize,
        need_cnt: usize,
    ) -> c_int;
}

#[inline]
pub unsafe fn libbpf_is_mem_zeroed(mut p: *const c_char, mut len: isize) -> bool {
    while len > 0 {
        if *p != 0 {
            return false;
        }
        p = p.add(1);
        len -= 1;
    }
    true
}

#[inline]
pub unsafe fn libbpf_validate_opts(
    opts: *const c_char,
    opts_sz: usize,
    user_sz: usize,
    type_name: *const c_char,
) -> bool {
    if user_sz < core::mem::size_of::<usize>() {
        libbpf_print(LIBBPF_WARN, c"%s size (%zu) is too small\n".as_ptr(), type_name, user_sz);
        return false;
    }
    if !libbpf_is_mem_zeroed(opts.add(opts_sz), user_sz as isize - opts_sz as isize) {
        libbpf_print(LIBBPF_WARN, c"%s has non-zero extra bytes\n".as_ptr(), type_name);
        return false;
    }
    true
}

/* OPTS_VALID, OPTS_HAS, OPTS_GET, OPTS_SET, and OPTS_ZEROED depend on C typeof,
 * offsetofend, and caller-specific struct fields. Preserve their intent at call
 * sites with direct Rust field/offset checks.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kern_feature_id {
    /* v4.14: kernel support for program & map names. */
    FEAT_PROG_NAME,
    /* v5.2: kernel support for global data sections. */
    FEAT_GLOBAL_DATA,
    /* BTF support */
    FEAT_BTF,
    /* BTF_KIND_FUNC and BTF_KIND_FUNC_PROTO support */
    FEAT_BTF_FUNC,
    /* BTF_KIND_VAR and BTF_KIND_DATASEC support */
    FEAT_BTF_DATASEC,
    /* BTF_FUNC_GLOBAL is supported */
    FEAT_BTF_GLOBAL_FUNC,
    /* BPF_F_MMAPABLE is supported for arrays */
    FEAT_ARRAY_MMAP,
    /* kernel support for expected_attach_type in BPF_PROG_LOAD */
    FEAT_EXP_ATTACH_TYPE,
    /* bpf_probe_read_{kernel,user}[_str] helpers */
    FEAT_PROBE_READ_KERN,
    /* BPF_PROG_BIND_MAP is supported */
    FEAT_PROG_BIND_MAP,
    /* Kernel support for module BTFs */
    FEAT_MODULE_BTF,
    /* BTF_KIND_FLOAT support */
    FEAT_BTF_FLOAT,
    /* BPF perf link support */
    FEAT_PERF_LINK,
    /* BTF_KIND_DECL_TAG support */
    FEAT_BTF_DECL_TAG,
    /* BTF_KIND_TYPE_TAG support */
    FEAT_BTF_TYPE_TAG,
    /* memcg-based accounting for BPF maps and progs */
    FEAT_MEMCG_ACCOUNT,
    /* BPF cookie (bpf_get_attach_cookie() BPF helper) support */
    FEAT_BPF_COOKIE,
    /* BTF_KIND_ENUM64 support and BTF_KIND_ENUM kflag support */
    FEAT_BTF_ENUM64,
    /* Kernel uses syscall wrapper (CONFIG_ARCH_HAS_SYSCALL_WRAPPER) */
    FEAT_SYSCALL_WRAPPER,
    /* BPF multi-uprobe link support */
    FEAT_UPROBE_MULTI_LINK,
    /* Kernel supports arg:ctx tag (__arg_ctx) for global subprogs natively */
    FEAT_ARG_CTX_TAG,
    /* Kernel supports '?' at the front of datasec names */
    FEAT_BTF_QMARK_DATASEC,
    /* Kernel supports LDIMM64 imm offsets past 512 MiB. */
    FEAT_LDIMM64_FULL_RANGE_OFF,
    /* Kernel supports uprobe syscall */
    FEAT_UPROBE_SYSCALL,
    /* Kernel supports BTF layout information */
    FEAT_BTF_LAYOUT,
    /* Kernel supports BPF syscall common attributes */
    FEAT_BPF_SYSCALL_COMMON_ATTRS,
    /* Kernel supports percpu data */
    FEAT_PERCPU_DATA,
    __FEAT_CNT,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kern_feature_result {
    FEAT_UNKNOWN = 0,
    FEAT_SUPPORTED = 1,
    FEAT_MISSING = 2,
}

#[repr(C)]
pub struct kern_feature_cache {
    pub res: [kern_feature_result; kern_feature_id::__FEAT_CNT as usize],
    pub token_fd: c_int,
}

unsafe extern "C" {
    pub fn feat_supported(cache: *mut kern_feature_cache, feat_id: kern_feature_id) -> bool;
    pub fn kernel_supports(obj: *const bpf_object, feat_id: kern_feature_id) -> bool;
    pub fn bpf_object_set_feat_cache(obj: *mut bpf_object, cache: *mut kern_feature_cache);

    pub fn probe_kern_syscall_wrapper(token_fd: c_int) -> c_int;
    pub fn probe_memcg_account(token_fd: c_int) -> c_int;
    pub fn bump_rlimit_memlock() -> c_int;

    pub fn parse_cpu_mask_str(s: *const c_char, mask: *mut *mut bool, mask_sz: *mut c_int) -> c_int;
    pub fn parse_cpu_mask_file(fcpu: *const c_char, mask: *mut *mut bool, mask_sz: *mut c_int) -> c_int;
    pub fn libbpf__load_raw_btf(
        raw_types: *const c_char,
        types_len: usize,
        str_sec: *const c_char,
        str_len: usize,
        token_fd: c_int,
    ) -> c_int;
    pub fn libbpf__load_raw_btf_hdr(
        hdr: *const btf_header,
        raw_types: *const c_char,
        str_sec: *const c_char,
        layout_sec: *const c_char,
        token_fd: c_int,
    ) -> c_int;
    pub fn bpf_object__sanitize_btf(obj: *mut bpf_object, orig_btf: *mut btf) -> *mut btf;
    pub fn btf_load_into_kernel(
        btf: *mut btf,
        log_buf: *mut c_char,
        log_sz: usize,
        log_level: __u32,
        token_fd: c_int,
    ) -> c_int;
    pub fn btf_load_from_kernel(id: __u32, base_btf: *mut btf, token_fd: c_int) -> *mut btf;

    pub fn btf_get_from_fd(btf_fd: c_int, base_btf: *mut btf) -> *mut btf;
    pub fn btf_get_kernel_prefix_kind(
        attach_type: bpf_attach_type,
        prefix: *mut *const c_char,
        kind: *mut c_int,
    );
}

#[repr(C)]
pub struct btf_ext_info {
    /*
     * info points to the individual info section (e.g. func_info and
     * line_info) from the .BTF.ext. It does not include the __u32 rec_size.
     */
    pub info: *mut c_void,
    pub rec_size: __u32,
    pub len: __u32,
    /* optional (maintained internally by libbpf) mapping between .BTF.ext
     * section and corresponding ELF section. This is used to join
     * information like CO-RE relocation records with corresponding BPF
     * programs defined in ELF sections
     */
    pub sec_idxs: *mut __u32,
    pub sec_cnt: c_int,
}

/* for_each_btf_ext_sec and for_each_btf_ext_rec are C iterator macros over
 * variable-sized records. Translate call sites as pointer-walking loops using
 * btf_ext_info.info, btf_ext_info.len, btf_ext_info.rec_size, and
 * btf_ext_info_sec.num_info.
 */

#[repr(C)]
pub struct btf_ext_header {
    pub magic: __u16,
    pub version: __u8,
    pub flags: __u8,
    pub hdr_len: __u32,

    /* All offsets are in bytes relative to the end of this header */
    pub func_info_off: __u32,
    pub func_info_len: __u32,
    pub line_info_off: __u32,
    pub line_info_len: __u32,

    /* optional part of .BTF.ext header */
    pub core_relo_off: __u32,
    pub core_relo_len: __u32,
}

#[repr(C)]
pub union btf_ext_hdr_or_data {
    pub hdr: *mut btf_ext_header,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct btf_ext {
    pub hdr_or_data: btf_ext_hdr_or_data,
    pub data_swapped: *mut c_void,
    pub swapped_endian: bool,
    pub func_info: btf_ext_info,
    pub line_info: btf_ext_info,
    pub core_relo_info: btf_ext_info,
    pub data_size: __u32,
}

#[repr(C)]
pub struct btf_ext_info_sec {
    pub sec_name_off: __u32,
    pub num_info: __u32,
    /* Followed by num_info * record_size number of bytes */
    pub data: [__u8; 0],
}

/* The minimum bpf_func_info checked by the loader */
#[repr(C)]
pub struct bpf_func_info_min {
    pub insn_off: __u32,
    pub type_id: __u32,
}

/* The minimum bpf_line_info checked by the loader */
#[repr(C)]
pub struct bpf_line_info_min {
    pub insn_off: __u32,
    pub file_name_off: __u32,
    pub line_off: __u32,
    pub line_col: __u32,
}

/* Functions to byte-swap info records */
pub type info_rec_bswap_fn = Option<unsafe extern "C" fn(*mut c_void)>;

#[inline]
pub unsafe fn bpf_func_info_bswap(i: *mut bpf_func_info) {
    (*i).insn_off = (*i).insn_off.swap_bytes();
    (*i).type_id = (*i).type_id.swap_bytes();
}

#[inline]
pub unsafe fn bpf_line_info_bswap(i: *mut bpf_line_info) {
    (*i).insn_off = (*i).insn_off.swap_bytes();
    (*i).file_name_off = (*i).file_name_off.swap_bytes();
    (*i).line_off = (*i).line_off.swap_bytes();
    (*i).line_col = (*i).line_col.swap_bytes();
}

#[inline]
pub unsafe fn bpf_core_relo_bswap(i: *mut bpf_core_relo) {
    (*i).insn_off = (*i).insn_off.swap_bytes();
    (*i).type_id = (*i).type_id.swap_bytes();
    (*i).access_str_off = (*i).access_str_off.swap_bytes();
    (*i).kind = (*i).kind.swap_bytes();
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum btf_field_iter_kind {
    BTF_FIELD_ITER_IDS,
    BTF_FIELD_ITER_STRS,
}

#[repr(C)]
pub struct btf_field_desc {
    /* once-per-type offsets */
    pub t_off_cnt: c_int,
    pub t_offs: [c_int; 2],
    /* member struct size, or zero, if no members */
    pub m_sz: c_int,
    /* repeated per-member offsets */
    pub m_off_cnt: c_int,
    pub m_offs: [c_int; 1],
}

#[repr(C)]
pub struct btf_field_iter {
    pub desc: btf_field_desc,
    pub p: *mut c_void,
    pub m_idx: c_int,
    pub off_idx: c_int,
    pub vlen: c_int,
}

pub type type_id_visit_fn = Option<unsafe extern "C" fn(type_id: *mut __u32, ctx: *mut c_void) -> c_int>;
pub type str_off_visit_fn = Option<unsafe extern "C" fn(str_off: *mut __u32, ctx: *mut c_void) -> c_int>;

unsafe extern "C" {
    pub fn btf_field_iter_init(
        it: *mut btf_field_iter,
        t: *mut btf_type,
        iter_kind: btf_field_iter_kind,
    ) -> c_int;
    pub fn btf_field_iter_next(it: *mut btf_field_iter) -> *mut __u32;
    pub fn btf_ext_visit_type_ids(btf_ext: *mut btf_ext, visit: type_id_visit_fn, ctx: *mut c_void)
        -> c_int;
    pub fn btf_ext_visit_str_offs(btf_ext: *mut btf_ext, visit: str_off_visit_fn, ctx: *mut c_void)
        -> c_int;
}

#[inline]
pub unsafe fn libbpf_err(ret: c_int) -> c_int {
    if ret < 0 {
        errno = -ret;
    }
    ret
}

#[inline]
pub unsafe fn libbpf_err_errno(ret: c_int) -> c_int {
    if ret < 0 { -errno } else { ret }
}

#[inline]
pub unsafe fn libbpf_err_ptr(err: c_int) -> *mut c_void {
    errno = -err;
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn libbpf_ptr(ret: *mut c_void) -> *mut c_void {
    if IS_ERR(ret) {
        errno = -PTR_ERR(ret);
    }

    if IS_ERR(ret) {
        core::ptr::null_mut()
    } else {
        ret
    }
}

#[inline]
pub unsafe fn str_is_empty(s: *const c_char) -> bool {
    s.is_null() || *s == 0
}

#[inline]
pub unsafe fn is_ldimm64_insn(insn: *mut bpf_insn) -> bool {
    (*insn).code as c_int == (BPF_LD | BPF_IMM | BPF_DW)
}

#[inline]
pub unsafe fn bpf_insn_bswap(insn: *mut bpf_insn) {
    let tmp_reg: __u8 = (*insn).dst_reg;

    (*insn).dst_reg = (*insn).src_reg;
    (*insn).src_reg = tmp_reg;
    (*insn).off = (*insn).off.swap_bytes();
    (*insn).imm = (*insn).imm.swap_bytes();
}

#[inline]
pub unsafe fn dup_good_fd(fd: c_int) -> c_int {
    if fd < 0 {
        return fd;
    }
    fcntl(fd, F_DUPFD_CLOEXEC, 3)
}

#[inline]
pub unsafe fn ensure_good_fd(mut fd: c_int) -> c_int {
    let old_fd: c_int = fd;
    let saved_errno: c_int;

    if fd < 0 {
        return fd;
    }
    if fd < 3 {
        fd = dup_good_fd(fd);
        saved_errno = errno;
        close(old_fd);
        errno = saved_errno;
        if fd < 0 {
            libbpf_print(
                LIBBPF_WARN,
                c"failed to dup FD %d to FD > 2: %d\n".as_ptr(),
                old_fd,
                -saved_errno,
            );
            errno = saved_errno;
        }
    }
    fd
}

#[inline]
pub unsafe fn sys_dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int {
    syscall(__NR_dup3 as c_long, oldfd, newfd, flags) as c_int
}

#[inline]
pub unsafe fn sys_memfd_create(name: *const c_char, flags: c_uint) -> c_int {
    syscall(__NR_memfd_create as c_long, name, flags) as c_int
}

#[inline]
pub unsafe fn reuse_fd(fixed_fd: c_int, tmp_fd: c_int) -> c_int {
    let mut err: c_int;

    err = sys_dup3(tmp_fd, fixed_fd, O_CLOEXEC);
    err = if err < 0 { -errno } else { 0 };
    close(tmp_fd); /* clean up temporary FD */
    err
}

unsafe extern "C" {
    pub fn bpf_core_add_cands(
        local_cand: *mut bpf_core_cand,
        local_essent_len: usize,
        targ_btf: *const btf,
        targ_btf_name: *const c_char,
        targ_start_id: c_int,
        cands: *mut bpf_core_cand_list,
    ) -> c_int;
    pub fn bpf_core_free_cands(cands: *mut bpf_core_cand_list);

    pub fn usdt_manager_new(obj: *mut bpf_object) -> *mut usdt_manager;
    pub fn usdt_manager_free(man: *mut usdt_manager);
    pub fn usdt_manager_attach_usdt(
        man: *mut usdt_manager,
        prog: *const bpf_program,
        pid: pid_t,
        path: *const c_char,
        usdt_provider: *const c_char,
        usdt_name: *const c_char,
        usdt_cookie: __u64,
    ) -> *mut bpf_link;
}

#[inline]
pub const fn is_pow_of_2(x: usize) -> bool {
    x != 0 && (x & (x - 1)) == 0
}

#[inline]
pub const fn ror32(v: __u32, bits: c_int) -> __u32 {
    (v >> bits) | (v << (32 - bits))
}

unsafe extern "C" {
    pub fn sys_bpf_prog_load(attr: *mut bpf_attr, size: c_uint, attempts: c_int) -> c_int;

    pub fn glob_match(str_: *const c_char, pat: *const c_char) -> bool;

    pub fn elf_find_func_offset(elf: *mut Elf, binary_path: *const c_char, name: *const c_char) -> c_long;
    pub fn elf_find_func_offset_from_file(binary_path: *const c_char, name: *const c_char) -> c_long;
}

#[repr(C)]
pub struct elf_fd {
    pub elf: *mut Elf,
    pub fd: c_int,
}

unsafe extern "C" {
    pub fn elf_open(binary_path: *const c_char, elf_fd: *mut elf_fd) -> c_int;
    pub fn elf_close(elf_fd: *mut elf_fd);

    pub fn elf_resolve_syms_offsets(
        binary_path: *const c_char,
        cnt: c_int,
        syms: *mut *const c_char,
        poffsets: *mut *mut c_ulong,
        st_type: c_int,
    ) -> c_int;
    pub fn elf_resolve_pattern_offsets(
        binary_path: *const c_char,
        pattern: *const c_char,
        poffsets: *mut *mut c_ulong,
        pcnt: *mut usize,
    ) -> c_int;

    pub fn probe_fd(fd: c_int) -> c_int;

    pub fn libbpf_sha256(data: *const c_void, len: usize, out: *mut __u8);
    pub fn probe_sys_bpf_ext() -> c_int;
}

unsafe extern "C" {
    pub static mut errno: c_int;

    pub fn strlen(s: *const c_char) -> usize;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn syscall(num: c_long, ...) -> c_long;
}

/* Opaque or externally supplied C types and constants. */
pub enum btf {}
pub enum btf_type {}
pub enum btf_header {}
pub enum bpf_object {}
pub enum bpf_program {}
pub enum usdt_manager {}
pub enum Elf {}
pub enum bpf_attr {}
pub enum btf_func_linkage {}
pub enum bpf_attach_type {}

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type pid_t = c_int;

unsafe extern "C" {
    pub fn btf_vlen(t: *const btf_type) -> __u32;
    pub fn IS_ERR(ptr: *const c_void) -> bool;
    pub fn PTR_ERR(ptr: *const c_void) -> c_int;
}

unsafe extern "C" {
    pub static BTF_MAX_VLEN: __u32;
    pub static BTF_KIND_INT: __u32;
    pub static BTF_KIND_FLOAT: __u32;
    pub static BTF_KIND_DECL_TAG: __u32;
    pub static BTF_KIND_TYPE_TAG: __u32;
    pub static LIBBPF_WARN: libbpf_print_level;
    pub static LIBBPF_INFO: libbpf_print_level;
    pub static LIBBPF_DEBUG: libbpf_print_level;
    pub static BPF_LD: c_int;
    pub static BPF_IMM: c_int;
    pub static BPF_DW: c_int;
    pub static F_DUPFD_CLOEXEC: c_int;
    pub static O_CLOEXEC: c_int;
    pub static __NR_dup3: c_long;
    pub static __NR_memfd_create: c_long;
}

pub enum libbpf_print_level {}

#[repr(C)]
pub struct bpf_func_info {
    pub insn_off: __u32,
    pub type_id: __u32,
}

#[repr(C)]
pub struct bpf_line_info {
    pub insn_off: __u32,
    pub file_name_off: __u32,
    pub line_off: __u32,
    pub line_col: __u32,
}

#[repr(C)]
pub struct bpf_core_relo {
    pub insn_off: __u32,
    pub type_id: __u32,
    pub access_str_off: __u32,
    pub kind: __u32,
}

#[repr(C)]
pub struct bpf_insn {
    pub code: __u8,
    pub dst_reg: __u8,
    pub src_reg: __u8,
    pub off: i16,
    pub imm: i32,
}

pub enum bpf_core_cand {}
pub enum bpf_core_cand_list {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
