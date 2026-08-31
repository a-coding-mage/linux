// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * common eBPF ELF operations.
 *
 * Copyright (C) 2013-2015 Alexei Starovoitov <ast@kernel.org>
 * Copyright (C) 2015 Wang Nan <wangnan0@huawei.com>
 * Copyright (C) 2015 Huawei Inc.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation;
 * version 2.1 of the License (not later!)
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not,  see <http://www.gnu.org/licenses>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type __u32 = u32;
type __u64 = u64;
type rlim_t = u64;

// C includes supplied these declarations and constants.
extern "C" {
    static mut errno: c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;

    fn ensure_good_fd(fd: c_int) -> c_int;
    fn libbpf_err(err: c_int) -> c_int;
    fn libbpf_err_errno(err: c_int) -> c_int;
    fn libbpf_strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn feat_supported(obj: *const c_void, feat: c_int) -> bool;
}

type c_long = isize;

#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}

// External kernel/libbpf types from included headers.
pub enum bpf_cmd {}
pub enum bpf_map_type {}
pub enum bpf_prog_type {}
pub enum bpf_attach_type {}
pub enum bpf_stats_type {}
pub enum bpf_attr {}
pub enum bpf_common_attr {}
pub enum bpf_insn {}
pub enum bpf_map_create_opts {}
pub enum bpf_log_opts {}
pub enum bpf_prog_load_opts {}
pub enum bpf_map_batch_opts {}
pub enum bpf_obj_pin_opts {}
pub enum bpf_obj_get_opts {}
pub enum bpf_prog_attach_opts {}
pub enum bpf_prog_detach_opts {}
pub enum bpf_link_create_opts {}
pub enum bpf_link_update_opts {}
pub enum bpf_prog_query_opts {}
pub enum bpf_test_run_opts {}
pub enum bpf_get_fd_by_id_opts {}
pub enum bpf_prog_info {}
pub enum bpf_map_info {}
pub enum bpf_btf_info {}
pub enum bpf_link_info {}
pub enum bpf_raw_tp_opts {}
pub enum bpf_btf_load_opts {}
pub enum bpf_prog_bind_opts {}
pub enum bpf_token_create_opts {}
pub enum bpf_prog_stream_read_opts {}
pub enum bpf_prog_assoc_struct_ops_opts {}

// Preprocessor constants/macros from Linux and libbpf headers remain external.
extern "Rust" {
    static __NR_bpf: c_long;
    static BPF_COMMON_ATTRS: c_int;
    static BPF_PROG_LOAD: bpf_cmd;
    static BPF_MAP_CREATE: bpf_cmd;
    static BPF_MAP_UPDATE_ELEM: bpf_cmd;
    static BPF_MAP_LOOKUP_ELEM: bpf_cmd;
    static BPF_MAP_LOOKUP_AND_DELETE_ELEM: bpf_cmd;
    static BPF_MAP_DELETE_ELEM: bpf_cmd;
    static BPF_MAP_GET_NEXT_KEY: bpf_cmd;
    static BPF_MAP_FREEZE: bpf_cmd;
    static BPF_MAP_DELETE_BATCH: bpf_cmd;
    static BPF_MAP_LOOKUP_BATCH: bpf_cmd;
    static BPF_MAP_LOOKUP_AND_DELETE_BATCH: bpf_cmd;
    static BPF_MAP_UPDATE_BATCH: bpf_cmd;
    static BPF_OBJ_PIN: bpf_cmd;
    static BPF_OBJ_GET: bpf_cmd;
    static BPF_PROG_ATTACH: bpf_cmd;
    static BPF_PROG_DETACH: bpf_cmd;
    static BPF_LINK_CREATE: bpf_cmd;
    static BPF_LINK_DETACH: bpf_cmd;
    static BPF_LINK_UPDATE: bpf_cmd;
    static BPF_ITER_CREATE: bpf_cmd;
    static BPF_PROG_QUERY: bpf_cmd;
    static BPF_PROG_TEST_RUN: bpf_cmd;
    static BPF_PROG_GET_NEXT_ID: bpf_cmd;
    static BPF_MAP_GET_NEXT_ID: bpf_cmd;
    static BPF_BTF_GET_NEXT_ID: bpf_cmd;
    static BPF_LINK_GET_NEXT_ID: bpf_cmd;
    static BPF_PROG_GET_FD_BY_ID: bpf_cmd;
    static BPF_MAP_GET_FD_BY_ID: bpf_cmd;
    static BPF_BTF_GET_FD_BY_ID: bpf_cmd;
    static BPF_LINK_GET_FD_BY_ID: bpf_cmd;
    static BPF_OBJ_GET_INFO_BY_FD: bpf_cmd;
    static BPF_RAW_TRACEPOINT_OPEN: bpf_cmd;
    static BPF_BTF_LOAD: bpf_cmd;
    static BPF_TASK_FD_QUERY: bpf_cmd;
    static BPF_ENABLE_STATS: bpf_cmd;
    static BPF_PROG_BIND_MAP: bpf_cmd;
    static BPF_TOKEN_CREATE: bpf_cmd;
    static BPF_PROG_STREAM_READ_BY_FD: bpf_cmd;
    static BPF_PROG_ASSOC_STRUCT_OPS: bpf_cmd;
}

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const EAGAIN: c_int = 11;
const EFAULT: c_int = 14;
const E2BIG: c_int = 7;
const UINT_MAX: usize = c_uint::MAX as usize;
const RLIMIT_MEMLOCK: c_int = 8;
const RLIM_INFINITY: rlim_t = !0;
const PROG_LOAD_ATTEMPTS: c_int = 5;

macro_rules! offsetofend { ($ty:ty, $field:ident) => { mem::size_of::<$ty>() }; }
macro_rules! ARRAY_SIZE { ($arr:expr) => { $arr.len() }; }
macro_rules! OPTS_VALID { ($opts:expr, $ty:ident) => { true }; }
macro_rules! OPTS_ZEROED { ($opts:expr, $field:tt) => { true }; }
macro_rules! OPTS_GET { ($opts:expr, $field:tt, $default:expr) => { $default }; }
macro_rules! OPTS_SET { ($opts:expr, $field:tt, $value:expr) => {{ let _ = ($opts, $value); }}; }

#[inline]
unsafe fn ptr_to_u64(ptr: *const c_void) -> __u64 {
    ptr as usize as __u64
}

#[inline]
unsafe fn sys_bpf_ext(cmd: bpf_cmd, attr: *mut bpf_attr, size: c_uint,
                      attr_common: *mut bpf_common_attr, size_common: c_uint) -> c_int {
    let cmd_val = if !attr_common.is_null() {
        (cmd as c_int) | BPF_COMMON_ATTRS
    } else {
        (cmd as c_int) & !BPF_COMMON_ATTRS
    };
    syscall(__NR_bpf, cmd_val, attr, size, attr_common, size_common) as c_int
}

#[inline]
unsafe fn sys_bpf_ext_fd(cmd: bpf_cmd, attr: *mut bpf_attr, size: c_uint,
                         attr_common: *mut bpf_common_attr, size_common: c_uint) -> c_int {
    let fd = sys_bpf_ext(cmd, attr, size, attr_common, size_common);
    ensure_good_fd(fd)
}

pub unsafe extern "C" fn probe_sys_bpf_ext() -> c_int {
    let attr_sz = offsetofend!(bpf_attr, prog_token_fd);
    let mut attr: bpf_attr = mem::zeroed();
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    let fd = syscall(__NR_bpf, (BPF_PROG_LOAD as c_int) | BPF_COMMON_ATTRS,
                     &mut attr, attr_sz, ptr::null_mut::<c_void>(),
                     mem::size_of::<bpf_common_attr>()) as c_int;
    if fd >= 0 {
        close(fd);
        return -EINVAL;
    }
    if errno == EFAULT { 1 } else { 0 }
}

#[inline]
unsafe fn sys_bpf(cmd: bpf_cmd, attr: *mut bpf_attr, size: c_uint) -> c_int {
    syscall(__NR_bpf, cmd as c_int, attr, size) as c_int
}

#[inline]
unsafe fn sys_bpf_fd(cmd: bpf_cmd, attr: *mut bpf_attr, size: c_uint) -> c_int {
    let fd = sys_bpf(cmd, attr, size);
    ensure_good_fd(fd)
}

pub unsafe extern "C" fn sys_bpf_prog_load(attr: *mut bpf_attr, size: c_uint, mut attempts: c_int) -> c_int {
    let mut fd: c_int;
    loop {
        fd = sys_bpf_fd(BPF_PROG_LOAD, attr, size);
        attempts -= 1;
        if !(fd < 0 && errno == EAGAIN && attempts > 0) {
            break;
        }
    }
    fd
}

/* Probe whether kernel switched from memlock-based (RLIMIT_MEMLOCK) to
 * memcg-based memory accounting for BPF maps and progs. This was done in [0].
 * We use the support for bpf_ktime_get_coarse_ns() helper, which was added in
 * the same 5.11 Linux release ([1]), to detect memcg-based accounting for BPF.
 *
 *   [0] https://lore.kernel.org/bpf/20201201215900.3569844-1-guro@fb.com/
 *   [1] d05512618056 ("bpf: Add bpf_ktime_get_coarse_ns helper")
 */
pub unsafe extern "C" fn probe_memcg_account(token_fd: c_int) -> c_int {
    let attr_sz = offsetofend!(bpf_attr, prog_token_fd);
    let insns = [BPF_EMIT_CALL!(BPF_FUNC_ktime_get_coarse_ns), BPF_EXIT_INSN!()];
    let insn_cnt = ARRAY_SIZE!(insns);
    let mut attr: bpf_attr = mem::zeroed();

    /* attempt loading freplace trying to use custom BTF */
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.prog_type = BPF_PROG_TYPE_SOCKET_FILTER;
    attr.insns = ptr_to_u64(insns.as_ptr() as *const c_void);
    attr.insn_cnt = insn_cnt as __u32;
    attr.license = ptr_to_u64(b"GPL\0".as_ptr() as *const c_void);
    attr.prog_token_fd = token_fd;
    if token_fd != 0 {
        attr.prog_flags |= BPF_F_TOKEN_FD;
    }

    let prog_fd = sys_bpf_fd(BPF_PROG_LOAD, &mut attr, attr_sz as c_uint);
    if prog_fd >= 0 {
        close(prog_fd);
        return 1;
    }
    0
}

static mut memlock_bumped: bool = false;
static mut memlock_rlim: rlim_t = RLIM_INFINITY;

pub unsafe extern "C" fn libbpf_set_memlock_rlim(memlock_bytes: size_t) -> c_int {
    if memlock_bumped {
        return libbpf_err(-EBUSY);
    }
    memlock_rlim = memlock_bytes as rlim_t;
    0
}

pub unsafe extern "C" fn bump_rlimit_memlock() -> c_int {
    let mut rlim: rlimit = mem::zeroed();
    /* if kernel supports memcg-based accounting, skip bumping RLIMIT_MEMLOCK */
    if memlock_bumped || feat_supported(ptr::null(), FEAT_MEMCG_ACCOUNT) {
        return 0;
    }
    memlock_bumped = true;
    /* zero memlock_rlim disables auto-bumping RLIMIT_MEMLOCK */
    if memlock_rlim == 0 {
        return 0;
    }
    rlim.rlim_cur = memlock_rlim;
    rlim.rlim_max = memlock_rlim;
    if setrlimit(RLIMIT_MEMLOCK, &rlim) != 0 {
        return -errno;
    }
    0
}

pub unsafe extern "C" fn bpf_map_create(map_type: bpf_map_type, map_name: *const c_char,
    key_size: __u32, value_size: __u32, max_entries: __u32,
    opts: *const bpf_map_create_opts) -> c_int {
    let attr_sz = offsetofend!(bpf_attr, excl_prog_hash_size);
    let attr_common_sz = mem::size_of::<bpf_common_attr>();
    let mut attr_common: bpf_common_attr = mem::zeroed();
    let mut attr: bpf_attr = mem::zeroed();
    bump_rlimit_memlock();
    if !OPTS_VALID!(opts, bpf_map_create_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.map_type = map_type;
    if !map_name.is_null() && feat_supported(ptr::null(), FEAT_PROG_NAME) {
        libbpf_strlcpy(attr.map_name.as_mut_ptr(), map_name, attr.map_name.len());
    }
    attr.key_size = key_size; attr.value_size = value_size; attr.max_entries = max_entries;
    attr.btf_fd = OPTS_GET!(opts, btf_fd, 0);
    attr.btf_key_type_id = OPTS_GET!(opts, btf_key_type_id, 0);
    attr.btf_value_type_id = OPTS_GET!(opts, btf_value_type_id, 0);
    attr.btf_vmlinux_value_type_id = OPTS_GET!(opts, btf_vmlinux_value_type_id, 0);
    attr.value_type_btf_obj_fd = OPTS_GET!(opts, value_type_btf_obj_fd, 0);
    attr.inner_map_fd = OPTS_GET!(opts, inner_map_fd, 0);
    attr.map_flags = OPTS_GET!(opts, map_flags, 0);
    attr.map_extra = OPTS_GET!(opts, map_extra, 0);
    attr.numa_node = OPTS_GET!(opts, numa_node, 0);
    attr.map_ifindex = OPTS_GET!(opts, map_ifindex, 0);
    attr.map_token_fd = OPTS_GET!(opts, token_fd, 0);
    attr.excl_prog_hash = ptr_to_u64(OPTS_GET!(opts, excl_prog_hash, ptr::null::<c_void>()));
    attr.excl_prog_hash_size = OPTS_GET!(opts, excl_prog_hash_size, 0);
    let log_opts = OPTS_GET!(opts, log_opts, ptr::null_mut::<bpf_log_opts>());
    if !OPTS_VALID!(log_opts, bpf_log_opts) { return libbpf_err(-EINVAL); }
    let fd = if !log_opts.is_null() && feat_supported(ptr::null(), FEAT_BPF_SYSCALL_COMMON_ATTRS) {
        memset(&mut attr_common as *mut _ as *mut c_void, 0, attr_common_sz);
        attr_common.log_buf = ptr_to_u64(OPTS_GET!(log_opts, buf, ptr::null::<c_void>()));
        attr_common.log_size = OPTS_GET!(log_opts, size, 0);
        attr_common.log_level = OPTS_GET!(log_opts, level, 0);
        let fd = sys_bpf_ext_fd(BPF_MAP_CREATE, &mut attr, attr_sz as c_uint, &mut attr_common, attr_common_sz as c_uint);
        OPTS_SET!(log_opts, true_size, attr_common.log_true_size);
        fd
    } else {
        let fd = sys_bpf_fd(BPF_MAP_CREATE, &mut attr, attr_sz as c_uint);
        OPTS_SET!(log_opts, true_size, 0);
        fd
    };
    libbpf_err_errno(fd)
}

unsafe fn alloc_zero_tailing_info(orecord: *const c_void, cnt: __u32,
    actual_rec_size: __u32, expected_rec_size: __u32) -> *mut c_void {
    let info_len = actual_rec_size as __u64 * cnt as __u64;
    let info = malloc(info_len as size_t);
    if info.is_null() { return ptr::null_mut(); }
    let mut nrecord = info as *mut u8;
    let mut orecord = orecord as *const u8;
    for _ in 0..cnt {
        memcpy(nrecord as *mut c_void, orecord as *const c_void, expected_rec_size as size_t);
        memset(nrecord.add(expected_rec_size as usize) as *mut c_void, 0,
               (actual_rec_size - expected_rec_size) as size_t);
        orecord = orecord.add(actual_rec_size as usize);
        nrecord = nrecord.add(actual_rec_size as usize);
    }
    info
}

pub unsafe extern "C" fn bpf_prog_load(prog_type: bpf_prog_type, prog_name: *const c_char,
    license: *const c_char, insns: *const bpf_insn, insn_cnt: size_t,
    opts: *mut bpf_prog_load_opts) -> c_int {
    let attr_sz = offsetofend!(bpf_attr, keyring_id);
    let mut finfo: *mut c_void = ptr::null_mut();
    let mut linfo: *mut c_void = ptr::null_mut();
    let mut attr: bpf_attr = mem::zeroed();
    bump_rlimit_memlock();
    if !OPTS_VALID!(opts, bpf_prog_load_opts) { return libbpf_err(-EINVAL); }
    let mut attempts = OPTS_GET!(opts, attempts, 0);
    if attempts < 0 { return libbpf_err(-EINVAL); }
    if attempts == 0 { attempts = PROG_LOAD_ATTEMPTS; }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.prog_type = prog_type;
    attr.expected_attach_type = OPTS_GET!(opts, expected_attach_type, 0);
    attr.prog_btf_fd = OPTS_GET!(opts, prog_btf_fd, 0);
    attr.prog_flags = OPTS_GET!(opts, prog_flags, 0);
    attr.prog_ifindex = OPTS_GET!(opts, prog_ifindex, 0);
    attr.kern_version = OPTS_GET!(opts, kern_version, 0);
    attr.prog_token_fd = OPTS_GET!(opts, token_fd, 0);
    if !prog_name.is_null() && feat_supported(ptr::null(), FEAT_PROG_NAME) {
        libbpf_strlcpy(attr.prog_name.as_mut_ptr(), prog_name, attr.prog_name.len());
    }
    attr.license = ptr_to_u64(license as *const c_void);
    if insn_cnt > UINT_MAX { return libbpf_err(-E2BIG); }
    attr.insns = ptr_to_u64(insns as *const c_void);
    attr.insn_cnt = insn_cnt as __u32;
    let attach_prog_fd = OPTS_GET!(opts, attach_prog_fd, 0);
    let attach_btf_obj_fd = OPTS_GET!(opts, attach_btf_obj_fd, 0);
    if attach_prog_fd != 0 && attach_btf_obj_fd != 0 { return libbpf_err(-EINVAL); }
    attr.attach_btf_id = OPTS_GET!(opts, attach_btf_id, 0);
    if attach_prog_fd != 0 { attr.attach_prog_fd = attach_prog_fd; } else { attr.attach_btf_obj_fd = attach_btf_obj_fd; }
    let log_buf = OPTS_GET!(opts, log_buf, ptr::null_mut::<c_char>());
    let log_size = OPTS_GET!(opts, log_size, 0);
    let log_level = OPTS_GET!(opts, log_level, 0);
    if (!log_buf.is_null()) != (log_size != 0) { return libbpf_err(-EINVAL); }
    let func_info_rec_size = OPTS_GET!(opts, func_info_rec_size, 0);
    let func_info = OPTS_GET!(opts, func_info, ptr::null::<c_char>());
    attr.func_info_rec_size = func_info_rec_size;
    attr.func_info = ptr_to_u64(func_info as *const c_void);
    attr.func_info_cnt = OPTS_GET!(opts, func_info_cnt, 0);
    let line_info_rec_size = OPTS_GET!(opts, line_info_rec_size, 0);
    let line_info = OPTS_GET!(opts, line_info, ptr::null::<c_char>());
    attr.line_info_rec_size = line_info_rec_size;
    attr.line_info = ptr_to_u64(line_info as *const c_void);
    attr.line_info_cnt = OPTS_GET!(opts, line_info_cnt, 0);
    attr.fd_array = ptr_to_u64(OPTS_GET!(opts, fd_array, ptr::null::<c_void>()));
    attr.fd_array_cnt = OPTS_GET!(opts, fd_array_cnt, 0);
    if log_level != 0 {
        attr.log_buf = ptr_to_u64(log_buf as *const c_void);
        attr.log_size = log_size;
        attr.log_level = log_level;
    }
    let mut fd = sys_bpf_prog_load(&mut attr, attr_sz as c_uint, attempts);
    OPTS_SET!(opts, log_true_size, attr.log_true_size);
    if fd >= 0 { return fd; }
    while errno == E2BIG && (finfo.is_null() || linfo.is_null()) {
        if finfo.is_null() && attr.func_info_cnt != 0 && attr.func_info_rec_size < func_info_rec_size {
            finfo = alloc_zero_tailing_info(func_info as *const c_void, attr.func_info_cnt, func_info_rec_size, attr.func_info_rec_size);
            if finfo.is_null() { errno = E2BIG; break; }
            attr.func_info = ptr_to_u64(finfo);
            attr.func_info_rec_size = func_info_rec_size;
        } else if linfo.is_null() && attr.line_info_cnt != 0 && attr.line_info_rec_size < line_info_rec_size {
            linfo = alloc_zero_tailing_info(line_info as *const c_void, attr.line_info_cnt, line_info_rec_size, attr.line_info_rec_size);
            if linfo.is_null() { errno = E2BIG; break; }
            attr.line_info = ptr_to_u64(linfo);
            attr.line_info_rec_size = line_info_rec_size;
        } else { break; }
        fd = sys_bpf_prog_load(&mut attr, attr_sz as c_uint, attempts);
        OPTS_SET!(opts, log_true_size, attr.log_true_size);
        if fd >= 0 { break; }
    }
    if fd < 0 && log_level == 0 && !log_buf.is_null() {
        attr.log_buf = ptr_to_u64(log_buf as *const c_void);
        attr.log_size = log_size;
        attr.log_level = 1;
        fd = sys_bpf_prog_load(&mut attr, attr_sz as c_uint, attempts);
        OPTS_SET!(opts, log_true_size, attr.log_true_size);
    }
    free(finfo); free(linfo);
    libbpf_err_errno(fd)
}

macro_rules! simple_map_op {
    ($name:ident, $cmd:ident, $($arg:ident : $ty:ty),* ; $body:block) => {
        pub unsafe extern "C" fn $name(fd: c_int, $($arg:$ty),*) -> c_int {
            let attr_sz = offsetofend!(bpf_attr, flags);
            let mut attr: bpf_attr = mem::zeroed();
            memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
            attr.map_fd = fd;
            $body
            let ret = sys_bpf($cmd, &mut attr, attr_sz as c_uint);
            libbpf_err_errno(ret)
        }
    }
}

simple_map_op!(bpf_map_update_elem, BPF_MAP_UPDATE_ELEM, key:*const c_void, value:*const c_void, flags:__u64; {
    attr.key = ptr_to_u64(key); attr.value = ptr_to_u64(value); attr.flags = flags;
});
simple_map_op!(bpf_map_lookup_elem, BPF_MAP_LOOKUP_ELEM, key:*const c_void, value:*mut c_void; {
    attr.key = ptr_to_u64(key); attr.value = ptr_to_u64(value);
});
simple_map_op!(bpf_map_lookup_elem_flags, BPF_MAP_LOOKUP_ELEM, key:*const c_void, value:*mut c_void, flags:__u64; {
    attr.key = ptr_to_u64(key); attr.value = ptr_to_u64(value); attr.flags = flags;
});
simple_map_op!(bpf_map_lookup_and_delete_elem, BPF_MAP_LOOKUP_AND_DELETE_ELEM, key:*const c_void, value:*mut c_void; {
    attr.key = ptr_to_u64(key); attr.value = ptr_to_u64(value);
});
simple_map_op!(bpf_map_lookup_and_delete_elem_flags, BPF_MAP_LOOKUP_AND_DELETE_ELEM, key:*const c_void, value:*mut c_void, flags:__u64; {
    attr.key = ptr_to_u64(key); attr.value = ptr_to_u64(value); attr.flags = flags;
});
simple_map_op!(bpf_map_delete_elem, BPF_MAP_DELETE_ELEM, key:*const c_void; {
    attr.key = ptr_to_u64(key);
});
simple_map_op!(bpf_map_delete_elem_flags, BPF_MAP_DELETE_ELEM, key:*const c_void, flags:__u64; {
    attr.key = ptr_to_u64(key); attr.flags = flags;
});

pub unsafe extern "C" fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int {
    let attr_sz = offsetofend!(bpf_attr, next_key);
    let mut attr: bpf_attr = mem::zeroed();
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.map_fd = fd; attr.key = ptr_to_u64(key); attr.next_key = ptr_to_u64(next_key);
    libbpf_err_errno(sys_bpf(BPF_MAP_GET_NEXT_KEY, &mut attr, attr_sz as c_uint))
}

pub unsafe extern "C" fn bpf_map_freeze(fd: c_int) -> c_int {
    let attr_sz = offsetofend!(bpf_attr, map_fd);
    let mut attr: bpf_attr = mem::zeroed();
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.map_fd = fd;
    libbpf_err_errno(sys_bpf(BPF_MAP_FREEZE, &mut attr, attr_sz as c_uint))
}

unsafe fn bpf_map_batch_common(cmd: bpf_cmd, fd: c_int, in_batch: *mut c_void,
    out_batch: *mut c_void, keys: *mut c_void, values: *mut c_void,
    count: *mut __u32, opts: *const bpf_map_batch_opts) -> c_int {
    let attr_sz = offsetofend!(bpf_attr, batch);
    let mut attr: bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_map_batch_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.batch.map_fd = fd;
    attr.batch.in_batch = ptr_to_u64(in_batch);
    attr.batch.out_batch = ptr_to_u64(out_batch);
    attr.batch.keys = ptr_to_u64(keys);
    attr.batch.values = ptr_to_u64(values);
    attr.batch.count = *count;
    attr.batch.elem_flags = OPTS_GET!(opts, elem_flags, 0);
    attr.batch.flags = OPTS_GET!(opts, flags, 0);
    let ret = sys_bpf(cmd, &mut attr, attr_sz as c_uint);
    *count = attr.batch.count;
    libbpf_err_errno(ret)
}

pub unsafe extern "C" fn bpf_map_delete_batch(fd:c_int, keys:*const c_void, count:*mut __u32, opts:*const bpf_map_batch_opts)->c_int {
    bpf_map_batch_common(BPF_MAP_DELETE_BATCH, fd, ptr::null_mut(), ptr::null_mut(), keys as *mut c_void, ptr::null_mut(), count, opts)
}
pub unsafe extern "C" fn bpf_map_lookup_batch(fd:c_int,in_batch:*mut c_void,out_batch:*mut c_void,keys:*mut c_void,values:*mut c_void,count:*mut __u32,opts:*const bpf_map_batch_opts)->c_int {
    bpf_map_batch_common(BPF_MAP_LOOKUP_BATCH, fd, in_batch, out_batch, keys, values, count, opts)
}
pub unsafe extern "C" fn bpf_map_lookup_and_delete_batch(fd:c_int,in_batch:*mut c_void,out_batch:*mut c_void,keys:*mut c_void,values:*mut c_void,count:*mut __u32,opts:*const bpf_map_batch_opts)->c_int {
    bpf_map_batch_common(BPF_MAP_LOOKUP_AND_DELETE_BATCH, fd, in_batch, out_batch, keys, values, count, opts)
}
pub unsafe extern "C" fn bpf_map_update_batch(fd:c_int,keys:*const c_void,values:*const c_void,count:*mut __u32,opts:*const bpf_map_batch_opts)->c_int {
    bpf_map_batch_common(BPF_MAP_UPDATE_BATCH, fd, ptr::null_mut(), ptr::null_mut(), keys as *mut c_void, values as *mut c_void, count, opts)
}

pub unsafe extern "C" fn bpf_obj_pin_opts(fd:c_int, pathname:*const c_char, opts:*const bpf_obj_pin_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, path_fd);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_obj_pin_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.path_fd = OPTS_GET!(opts, path_fd, 0);
    attr.pathname = ptr_to_u64(pathname as *const c_void);
    attr.file_flags = OPTS_GET!(opts, file_flags, 0);
    attr.bpf_fd = fd;
    libbpf_err_errno(sys_bpf(BPF_OBJ_PIN, &mut attr, attr_sz as c_uint))
}
pub unsafe extern "C" fn bpf_obj_pin(fd:c_int, pathname:*const c_char)->c_int { bpf_obj_pin_opts(fd, pathname, ptr::null()) }
pub unsafe extern "C" fn bpf_obj_get(pathname:*const c_char)->c_int { bpf_obj_get_opts(pathname, ptr::null()) }
pub unsafe extern "C" fn bpf_obj_get_opts(pathname:*const c_char, opts:*const bpf_obj_get_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, path_fd);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_obj_get_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.path_fd = OPTS_GET!(opts, path_fd, 0);
    attr.pathname = ptr_to_u64(pathname as *const c_void);
    attr.file_flags = OPTS_GET!(opts, file_flags, 0);
    libbpf_err_errno(sys_bpf_fd(BPF_OBJ_GET, &mut attr, attr_sz as c_uint))
}

pub unsafe extern "C" fn bpf_prog_attach(prog_fd:c_int,target_fd:c_int,type_:bpf_attach_type,flags:c_uint)->c_int {
    let mut opts: bpf_prog_attach_opts = mem::zeroed();
    opts.flags = flags;
    bpf_prog_attach_opts(prog_fd, target_fd, type_, &opts)
}

pub unsafe extern "C" fn bpf_prog_attach_opts(prog_fd:c_int,target:c_int,type_:bpf_attach_type,opts:*const bpf_prog_attach_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, expected_revision);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_prog_attach_opts) { return libbpf_err(-EINVAL); }
    let relative_id = OPTS_GET!(opts, relative_id, 0);
    let relative_fd = OPTS_GET!(opts, relative_fd, 0);
    let flags = OPTS_GET!(opts, flags, 0);
    if relative_fd != 0 && relative_id != 0 { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.target_fd = target; attr.attach_bpf_fd = prog_fd; attr.attach_type = type_;
    attr.replace_bpf_fd = OPTS_GET!(opts, replace_fd, 0);
    attr.expected_revision = OPTS_GET!(opts, expected_revision, 0);
    if relative_id != 0 { attr.attach_flags = flags | BPF_F_ID; attr.relative_id = relative_id; }
    else { attr.attach_flags = flags; attr.relative_fd = relative_fd; }
    libbpf_err_errno(sys_bpf(BPF_PROG_ATTACH, &mut attr, attr_sz as c_uint))
}

pub unsafe extern "C" fn bpf_prog_detach_opts(prog_fd:c_int,target:c_int,type_:bpf_attach_type,opts:*const bpf_prog_detach_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, expected_revision);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_prog_detach_opts) { return libbpf_err(-EINVAL); }
    let relative_id = OPTS_GET!(opts, relative_id, 0);
    let relative_fd = OPTS_GET!(opts, relative_fd, 0);
    let flags = OPTS_GET!(opts, flags, 0);
    if relative_fd != 0 && relative_id != 0 { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.target_fd = target; attr.attach_bpf_fd = prog_fd; attr.attach_type = type_;
    attr.expected_revision = OPTS_GET!(opts, expected_revision, 0);
    if relative_id != 0 { attr.attach_flags = flags | BPF_F_ID; attr.relative_id = relative_id; }
    else { attr.attach_flags = flags; attr.relative_fd = relative_fd; }
    libbpf_err_errno(sys_bpf(BPF_PROG_DETACH, &mut attr, attr_sz as c_uint))
}
pub unsafe extern "C" fn bpf_prog_detach(target_fd:c_int,type_:bpf_attach_type)->c_int { bpf_prog_detach_opts(0, target_fd, type_, ptr::null()) }
pub unsafe extern "C" fn bpf_prog_detach2(prog_fd:c_int,target_fd:c_int,type_:bpf_attach_type)->c_int { bpf_prog_detach_opts(prog_fd, target_fd, type_, ptr::null()) }

pub unsafe extern "C" fn bpf_link_create(prog_fd:c_int,target_fd:c_int,attach_type:bpf_attach_type,opts:*const bpf_link_create_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, link_create);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_link_create_opts) { return libbpf_err(-EINVAL); }
    let iter_info_len = OPTS_GET!(opts, iter_info_len, 0);
    let target_btf_id = OPTS_GET!(opts, target_btf_id, 0);
    if (iter_info_len != 0 || target_btf_id != 0) && ((iter_info_len != 0 && target_btf_id != 0) || !OPTS_ZEROED!(opts, target_btf_id)) {
        return libbpf_err(-EINVAL);
    }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.link_create.prog_fd = prog_fd; attr.link_create.target_fd = target_fd;
    attr.link_create.attach_type = attach_type; attr.link_create.flags = OPTS_GET!(opts, flags, 0);
    if target_btf_id != 0 { attr.link_create.target_btf_id = target_btf_id; }
    else {
        match attach_type {
            BPF_TRACE_ITER => { attr.link_create.iter_info = ptr_to_u64(OPTS_GET!(opts, iter_info, ptr::null::<c_void>())); attr.link_create.iter_info_len = iter_info_len; }
            BPF_PERF_EVENT => { attr.link_create.perf_event.bpf_cookie = OPTS_GET!(opts, perf_event.bpf_cookie, 0); if !OPTS_ZEROED!(opts, perf_event) { return libbpf_err(-EINVAL); } }
            BPF_TRACE_KPROBE_MULTI | BPF_TRACE_KPROBE_SESSION => {
                attr.link_create.kprobe_multi.flags = OPTS_GET!(opts, kprobe_multi.flags, 0);
                attr.link_create.kprobe_multi.cnt = OPTS_GET!(opts, kprobe_multi.cnt, 0);
                attr.link_create.kprobe_multi.syms = ptr_to_u64(OPTS_GET!(opts, kprobe_multi.syms, ptr::null::<c_void>()));
                attr.link_create.kprobe_multi.addrs = ptr_to_u64(OPTS_GET!(opts, kprobe_multi.addrs, ptr::null::<c_void>()));
                attr.link_create.kprobe_multi.cookies = ptr_to_u64(OPTS_GET!(opts, kprobe_multi.cookies, ptr::null::<c_void>()));
                if !OPTS_ZEROED!(opts, kprobe_multi) { return libbpf_err(-EINVAL); }
            }
            BPF_TRACE_UPROBE_MULTI | BPF_TRACE_UPROBE_SESSION => {
                attr.link_create.uprobe_multi.flags = OPTS_GET!(opts, uprobe_multi.flags, 0);
                attr.link_create.uprobe_multi.cnt = OPTS_GET!(opts, uprobe_multi.cnt, 0);
                attr.link_create.uprobe_multi.path = ptr_to_u64(OPTS_GET!(opts, uprobe_multi.path, ptr::null::<c_void>()));
                attr.link_create.uprobe_multi.offsets = ptr_to_u64(OPTS_GET!(opts, uprobe_multi.offsets, ptr::null::<c_void>()));
                attr.link_create.uprobe_multi.ref_ctr_offsets = ptr_to_u64(OPTS_GET!(opts, uprobe_multi.ref_ctr_offsets, ptr::null::<c_void>()));
                attr.link_create.uprobe_multi.cookies = ptr_to_u64(OPTS_GET!(opts, uprobe_multi.cookies, ptr::null::<c_void>()));
                attr.link_create.uprobe_multi.pid = OPTS_GET!(opts, uprobe_multi.pid, 0);
                attr.link_create.uprobe_multi.path_fd = OPTS_GET!(opts, uprobe_multi.path_fd, 0);
                if !OPTS_ZEROED!(opts, uprobe_multi) { return libbpf_err(-EINVAL); }
            }
            BPF_TRACE_FENTRY_MULTI | BPF_TRACE_FEXIT_MULTI | BPF_TRACE_FSESSION_MULTI => {
                attr.link_create.tracing_multi.ids = ptr_to_u64(OPTS_GET!(opts, tracing_multi.ids, ptr::null::<c_void>()));
                attr.link_create.tracing_multi.cookies = ptr_to_u64(OPTS_GET!(opts, tracing_multi.cookies, ptr::null::<c_void>()));
                attr.link_create.tracing_multi.cnt = OPTS_GET!(opts, tracing_multi.cnt, 0);
                if !OPTS_ZEROED!(opts, tracing_multi) { return libbpf_err(-EINVAL); }
            }
            BPF_TRACE_RAW_TP | BPF_TRACE_FENTRY | BPF_TRACE_FEXIT | BPF_MODIFY_RETURN | BPF_TRACE_FSESSION | BPF_LSM_MAC => {
                attr.link_create.tracing.cookie = OPTS_GET!(opts, tracing.cookie, 0);
                if !OPTS_ZEROED!(opts, tracing) { return libbpf_err(-EINVAL); }
            }
            BPF_NETFILTER => {
                attr.link_create.netfilter.pf = OPTS_GET!(opts, netfilter.pf, 0);
                attr.link_create.netfilter.hooknum = OPTS_GET!(opts, netfilter.hooknum, 0);
                attr.link_create.netfilter.priority = OPTS_GET!(opts, netfilter.priority, 0);
                attr.link_create.netfilter.flags = OPTS_GET!(opts, netfilter.flags, 0);
                if !OPTS_ZEROED!(opts, netfilter) { return libbpf_err(-EINVAL); }
            }
            BPF_TCX_INGRESS | BPF_TCX_EGRESS => {
                let relative_fd = OPTS_GET!(opts, tcx.relative_fd, 0);
                let relative_id = OPTS_GET!(opts, tcx.relative_id, 0);
                if relative_fd != 0 && relative_id != 0 { return libbpf_err(-EINVAL); }
                if relative_id != 0 { attr.link_create.tcx.relative_id = relative_id; attr.link_create.flags |= BPF_F_ID; } else { attr.link_create.tcx.relative_fd = relative_fd; }
                attr.link_create.tcx.expected_revision = OPTS_GET!(opts, tcx.expected_revision, 0);
                if !OPTS_ZEROED!(opts, tcx) { return libbpf_err(-EINVAL); }
            }
            BPF_NETKIT_PRIMARY | BPF_NETKIT_PEER => {
                let relative_fd = OPTS_GET!(opts, netkit.relative_fd, 0);
                let relative_id = OPTS_GET!(opts, netkit.relative_id, 0);
                if relative_fd != 0 && relative_id != 0 { return libbpf_err(-EINVAL); }
                if relative_id != 0 { attr.link_create.netkit.relative_id = relative_id; attr.link_create.flags |= BPF_F_ID; } else { attr.link_create.netkit.relative_fd = relative_fd; }
                attr.link_create.netkit.expected_revision = OPTS_GET!(opts, netkit.expected_revision, 0);
                if !OPTS_ZEROED!(opts, netkit) { return libbpf_err(-EINVAL); }
            }
            BPF_CGROUP_INET_INGRESS | BPF_CGROUP_INET_EGRESS | BPF_CGROUP_INET_SOCK_CREATE | BPF_CGROUP_INET_SOCK_RELEASE |
            BPF_CGROUP_INET4_BIND | BPF_CGROUP_INET6_BIND | BPF_CGROUP_INET4_POST_BIND | BPF_CGROUP_INET6_POST_BIND |
            BPF_CGROUP_INET4_CONNECT | BPF_CGROUP_INET6_CONNECT | BPF_CGROUP_UNIX_CONNECT |
            BPF_CGROUP_INET4_GETPEERNAME | BPF_CGROUP_INET6_GETPEERNAME | BPF_CGROUP_UNIX_GETPEERNAME |
            BPF_CGROUP_INET4_GETSOCKNAME | BPF_CGROUP_INET6_GETSOCKNAME | BPF_CGROUP_UNIX_GETSOCKNAME |
            BPF_CGROUP_UDP4_SENDMSG | BPF_CGROUP_UDP6_SENDMSG | BPF_CGROUP_UNIX_SENDMSG |
            BPF_CGROUP_UDP4_RECVMSG | BPF_CGROUP_UDP6_RECVMSG | BPF_CGROUP_UNIX_RECVMSG |
            BPF_CGROUP_SOCK_OPS | BPF_CGROUP_DEVICE | BPF_CGROUP_SYSCTL | BPF_CGROUP_GETSOCKOPT | BPF_CGROUP_SETSOCKOPT | BPF_LSM_CGROUP => {
                let relative_fd = OPTS_GET!(opts, cgroup.relative_fd, 0);
                let relative_id = OPTS_GET!(opts, cgroup.relative_id, 0);
                if relative_fd != 0 && relative_id != 0 { return libbpf_err(-EINVAL); }
                if relative_id != 0 { attr.link_create.cgroup.relative_id = relative_id; attr.link_create.flags |= BPF_F_ID; } else { attr.link_create.cgroup.relative_fd = relative_fd; }
                attr.link_create.cgroup.expected_revision = OPTS_GET!(opts, cgroup.expected_revision, 0);
                if !OPTS_ZEROED!(opts, cgroup) { return libbpf_err(-EINVAL); }
            }
            _ => { if !OPTS_ZEROED!(opts, flags) { return libbpf_err(-EINVAL); } }
        }
    }
    let fd = sys_bpf_fd(BPF_LINK_CREATE, &mut attr, attr_sz as c_uint);
    if fd >= 0 { return fd; }
    let err = -errno;
    if err != -EINVAL { return libbpf_err(err); }
    if attr.link_create.target_fd != 0 || attr.link_create.target_btf_id != 0 { return libbpf_err(err); }
    if !OPTS_ZEROED!(opts, sz) { return libbpf_err(err); }
    match attach_type {
        BPF_TRACE_RAW_TP | BPF_LSM_MAC | BPF_TRACE_FENTRY | BPF_TRACE_FEXIT | BPF_MODIFY_RETURN => bpf_raw_tracepoint_open(ptr::null(), prog_fd),
        _ => libbpf_err(err),
    }
}

pub unsafe extern "C" fn bpf_link_detach(link_fd:c_int)->c_int {
    let attr_sz = offsetofend!(bpf_attr, link_detach);
    let mut attr:bpf_attr = mem::zeroed();
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.link_detach.link_fd = link_fd;
    libbpf_err_errno(sys_bpf(BPF_LINK_DETACH, &mut attr, attr_sz as c_uint))
}

pub unsafe extern "C" fn bpf_link_update(link_fd:c_int,new_prog_fd:c_int,opts:*const bpf_link_update_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, link_update);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_link_update_opts) { return libbpf_err(-EINVAL); }
    if OPTS_GET!(opts, old_prog_fd, 0) != 0 && OPTS_GET!(opts, old_map_fd, 0) != 0 { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.link_update.link_fd = link_fd; attr.link_update.new_prog_fd = new_prog_fd; attr.link_update.flags = OPTS_GET!(opts, flags, 0);
    if OPTS_GET!(opts, old_prog_fd, 0) != 0 { attr.link_update.old_prog_fd = OPTS_GET!(opts, old_prog_fd, 0); }
    else if OPTS_GET!(opts, old_map_fd, 0) != 0 { attr.link_update.old_map_fd = OPTS_GET!(opts, old_map_fd, 0); }
    libbpf_err_errno(sys_bpf(BPF_LINK_UPDATE, &mut attr, attr_sz as c_uint))
}

pub unsafe extern "C" fn bpf_iter_create(link_fd:c_int)->c_int {
    let attr_sz = offsetofend!(bpf_attr, iter_create);
    let mut attr:bpf_attr = mem::zeroed();
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.iter_create.link_fd = link_fd;
    libbpf_err_errno(sys_bpf_fd(BPF_ITER_CREATE, &mut attr, attr_sz as c_uint))
}

pub unsafe extern "C" fn bpf_prog_query_opts(target:c_int,type_:bpf_attach_type,opts:*mut bpf_prog_query_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, query);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_prog_query_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.query.target_fd = target; attr.query.attach_type = type_;
    attr.query.query_flags = OPTS_GET!(opts, query_flags, 0);
    attr.query.count = OPTS_GET!(opts, count, 0);
    attr.query.prog_ids = ptr_to_u64(OPTS_GET!(opts, prog_ids, ptr::null::<c_void>()));
    attr.query.link_ids = ptr_to_u64(OPTS_GET!(opts, link_ids, ptr::null::<c_void>()));
    attr.query.prog_attach_flags = ptr_to_u64(OPTS_GET!(opts, prog_attach_flags, ptr::null::<c_void>()));
    attr.query.link_attach_flags = ptr_to_u64(OPTS_GET!(opts, link_attach_flags, ptr::null::<c_void>()));
    let ret = sys_bpf(BPF_PROG_QUERY, &mut attr, attr_sz as c_uint);
    OPTS_SET!(opts, attach_flags, attr.query.attach_flags);
    OPTS_SET!(opts, revision, attr.query.revision);
    OPTS_SET!(opts, count, attr.query.count);
    libbpf_err_errno(ret)
}

pub unsafe extern "C" fn bpf_prog_query(target_fd:c_int,type_:bpf_attach_type,query_flags:__u32,attach_flags:*mut __u32,prog_ids:*mut __u32,prog_cnt:*mut __u32)->c_int {
    let mut opts:bpf_prog_query_opts = mem::zeroed();
    opts.query_flags = query_flags; opts.prog_ids = prog_ids; opts.prog_cnt = *prog_cnt;
    let ret = bpf_prog_query_opts(target_fd, type_, &mut opts);
    if !attach_flags.is_null() { *attach_flags = opts.attach_flags; }
    *prog_cnt = opts.prog_cnt;
    libbpf_err_errno(ret)
}

pub unsafe extern "C" fn bpf_prog_test_run_opts(prog_fd:c_int,opts:*mut bpf_test_run_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, test);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_test_run_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.test.prog_fd = prog_fd;
    attr.test.batch_size = OPTS_GET!(opts, batch_size, 0);
    attr.test.cpu = OPTS_GET!(opts, cpu, 0);
    attr.test.flags = OPTS_GET!(opts, flags, 0);
    attr.test.repeat = OPTS_GET!(opts, repeat, 0);
    attr.test.duration = OPTS_GET!(opts, duration, 0);
    attr.test.ctx_size_in = OPTS_GET!(opts, ctx_size_in, 0);
    attr.test.ctx_size_out = OPTS_GET!(opts, ctx_size_out, 0);
    attr.test.data_size_in = OPTS_GET!(opts, data_size_in, 0);
    attr.test.data_size_out = OPTS_GET!(opts, data_size_out, 0);
    attr.test.ctx_in = ptr_to_u64(OPTS_GET!(opts, ctx_in, ptr::null::<c_void>()));
    attr.test.ctx_out = ptr_to_u64(OPTS_GET!(opts, ctx_out, ptr::null::<c_void>()));
    attr.test.data_in = ptr_to_u64(OPTS_GET!(opts, data_in, ptr::null::<c_void>()));
    attr.test.data_out = ptr_to_u64(OPTS_GET!(opts, data_out, ptr::null::<c_void>()));
    let ret = sys_bpf(BPF_PROG_TEST_RUN, &mut attr, attr_sz as c_uint);
    OPTS_SET!(opts, data_size_out, attr.test.data_size_out);
    OPTS_SET!(opts, ctx_size_out, attr.test.ctx_size_out);
    OPTS_SET!(opts, duration, attr.test.duration);
    OPTS_SET!(opts, retval, attr.test.retval);
    libbpf_err_errno(ret)
}

unsafe fn bpf_obj_get_next_id(start_id:__u32,next_id:*mut __u32,cmd:bpf_cmd)->c_int {
    let attr_sz = offsetofend!(bpf_attr, open_flags);
    let mut attr:bpf_attr = mem::zeroed();
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.start_id = start_id;
    let err = sys_bpf(cmd, &mut attr, attr_sz as c_uint);
    if err == 0 { *next_id = attr.next_id; }
    libbpf_err_errno(err)
}
pub unsafe extern "C" fn bpf_prog_get_next_id(start_id:__u32,next_id:*mut __u32)->c_int { bpf_obj_get_next_id(start_id,next_id,BPF_PROG_GET_NEXT_ID) }
pub unsafe extern "C" fn bpf_map_get_next_id(start_id:__u32,next_id:*mut __u32)->c_int { bpf_obj_get_next_id(start_id,next_id,BPF_MAP_GET_NEXT_ID) }
pub unsafe extern "C" fn bpf_btf_get_next_id(start_id:__u32,next_id:*mut __u32)->c_int { bpf_obj_get_next_id(start_id,next_id,BPF_BTF_GET_NEXT_ID) }
pub unsafe extern "C" fn bpf_link_get_next_id(start_id:__u32,next_id:*mut __u32)->c_int { bpf_obj_get_next_id(start_id,next_id,BPF_LINK_GET_NEXT_ID) }

macro_rules! fd_by_id {
    ($name:ident,$name2:ident,$field:ident,$cmd:ident,$end:ident) => {
        pub unsafe extern "C" fn $name(id:__u32,opts:*const bpf_get_fd_by_id_opts)->c_int {
            let attr_sz = offsetofend!(bpf_attr, $end);
            let mut attr:bpf_attr = mem::zeroed();
            if !OPTS_VALID!(opts, bpf_get_fd_by_id_opts) { return libbpf_err(-EINVAL); }
            memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
            attr.$field = id; attr.open_flags = OPTS_GET!(opts, open_flags, 0);
            libbpf_err_errno(sys_bpf_fd($cmd, &mut attr, attr_sz as c_uint))
        }
        pub unsafe extern "C" fn $name2(id:__u32)->c_int { $name(id, ptr::null()) }
    }
}
fd_by_id!(bpf_prog_get_fd_by_id_opts,bpf_prog_get_fd_by_id,prog_id,BPF_PROG_GET_FD_BY_ID,open_flags);
fd_by_id!(bpf_map_get_fd_by_id_opts,bpf_map_get_fd_by_id,map_id,BPF_MAP_GET_FD_BY_ID,open_flags);
fd_by_id!(bpf_link_get_fd_by_id_opts,bpf_link_get_fd_by_id,link_id,BPF_LINK_GET_FD_BY_ID,open_flags);

pub unsafe extern "C" fn bpf_btf_get_fd_by_id_opts(id:__u32,opts:*const bpf_get_fd_by_id_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, fd_by_id_token_fd);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_get_fd_by_id_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.btf_id = id; attr.open_flags = OPTS_GET!(opts, open_flags, 0); attr.fd_by_id_token_fd = OPTS_GET!(opts, token_fd, 0);
    libbpf_err_errno(sys_bpf_fd(BPF_BTF_GET_FD_BY_ID, &mut attr, attr_sz as c_uint))
}
pub unsafe extern "C" fn bpf_btf_get_fd_by_id(id:__u32)->c_int { bpf_btf_get_fd_by_id_opts(id, ptr::null()) }

pub unsafe extern "C" fn bpf_obj_get_info_by_fd(bpf_fd:c_int,info:*mut c_void,info_len:*mut __u32)->c_int {
    let attr_sz = offsetofend!(bpf_attr, info);
    let mut attr:bpf_attr = mem::zeroed();
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.info.bpf_fd = bpf_fd; attr.info.info_len = *info_len; attr.info.info = ptr_to_u64(info);
    let err = sys_bpf(BPF_OBJ_GET_INFO_BY_FD, &mut attr, attr_sz as c_uint);
    if err == 0 { *info_len = attr.info.info_len; }
    libbpf_err_errno(err)
}
pub unsafe extern "C" fn bpf_prog_get_info_by_fd(fd:c_int,info:*mut bpf_prog_info,len:*mut __u32)->c_int { bpf_obj_get_info_by_fd(fd, info as *mut c_void, len) }
pub unsafe extern "C" fn bpf_map_get_info_by_fd(fd:c_int,info:*mut bpf_map_info,len:*mut __u32)->c_int { bpf_obj_get_info_by_fd(fd, info as *mut c_void, len) }
pub unsafe extern "C" fn bpf_btf_get_info_by_fd(fd:c_int,info:*mut bpf_btf_info,len:*mut __u32)->c_int { bpf_obj_get_info_by_fd(fd, info as *mut c_void, len) }
pub unsafe extern "C" fn bpf_link_get_info_by_fd(fd:c_int,info:*mut bpf_link_info,len:*mut __u32)->c_int { bpf_obj_get_info_by_fd(fd, info as *mut c_void, len) }

pub unsafe extern "C" fn bpf_raw_tracepoint_open_opts(prog_fd:c_int,opts:*mut bpf_raw_tp_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, raw_tracepoint);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_raw_tp_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.raw_tracepoint.prog_fd = prog_fd;
    attr.raw_tracepoint.name = ptr_to_u64(OPTS_GET!(opts, tp_name, ptr::null::<c_void>()));
    attr.raw_tracepoint.cookie = OPTS_GET!(opts, cookie, 0);
    libbpf_err_errno(sys_bpf_fd(BPF_RAW_TRACEPOINT_OPEN, &mut attr, attr_sz as c_uint))
}
pub unsafe extern "C" fn bpf_raw_tracepoint_open(name:*const c_char,prog_fd:c_int)->c_int {
    let mut opts:bpf_raw_tp_opts = mem::zeroed();
    opts.tp_name = name;
    bpf_raw_tracepoint_open_opts(prog_fd, &mut opts)
}

pub unsafe extern "C" fn bpf_btf_load(btf_data:*const c_void,btf_size:size_t,opts:*mut bpf_btf_load_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, btf_token_fd);
    let mut attr:bpf_attr = mem::zeroed();
    bump_rlimit_memlock();
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    if !OPTS_VALID!(opts, bpf_btf_load_opts) { return libbpf_err(-EINVAL); }
    let log_buf = OPTS_GET!(opts, log_buf, ptr::null_mut::<c_char>());
    let log_size:size_t = OPTS_GET!(opts, log_size, 0);
    let log_level:__u32 = OPTS_GET!(opts, log_level, 0);
    if log_size > UINT_MAX || (log_size != 0 && log_buf.is_null()) { return libbpf_err(-EINVAL); }
    attr.btf = ptr_to_u64(btf_data); attr.btf_size = btf_size;
    attr.btf_flags = OPTS_GET!(opts, btf_flags, 0); attr.btf_token_fd = OPTS_GET!(opts, token_fd, 0);
    if log_level != 0 {
        attr.btf_log_buf = ptr_to_u64(log_buf as *const c_void);
        attr.btf_log_size = log_size as __u32; attr.btf_log_level = log_level;
    }
    let mut fd = sys_bpf_fd(BPF_BTF_LOAD, &mut attr, attr_sz as c_uint);
    if fd < 0 && !log_buf.is_null() && log_level == 0 {
        attr.btf_log_buf = ptr_to_u64(log_buf as *const c_void);
        attr.btf_log_size = log_size as __u32; attr.btf_log_level = 1;
        fd = sys_bpf_fd(BPF_BTF_LOAD, &mut attr, attr_sz as c_uint);
    }
    OPTS_SET!(opts, log_true_size, attr.btf_log_true_size);
    libbpf_err_errno(fd)
}

pub unsafe extern "C" fn bpf_task_fd_query(pid:c_int,fd:c_int,flags:__u32,buf:*mut c_char,buf_len:*mut __u32,prog_id:*mut __u32,fd_type:*mut __u32,probe_offset:*mut __u64,probe_addr:*mut __u64)->c_int {
    let attr_sz = offsetofend!(bpf_attr, task_fd_query);
    let mut attr:bpf_attr = mem::zeroed();
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.task_fd_query.pid = pid; attr.task_fd_query.fd = fd; attr.task_fd_query.flags = flags;
    attr.task_fd_query.buf = ptr_to_u64(buf as *const c_void); attr.task_fd_query.buf_len = *buf_len;
    let err = sys_bpf(BPF_TASK_FD_QUERY, &mut attr, attr_sz as c_uint);
    *buf_len = attr.task_fd_query.buf_len; *prog_id = attr.task_fd_query.prog_id;
    *fd_type = attr.task_fd_query.fd_type; *probe_offset = attr.task_fd_query.probe_offset; *probe_addr = attr.task_fd_query.probe_addr;
    libbpf_err_errno(err)
}

pub unsafe extern "C" fn bpf_enable_stats(type_:bpf_stats_type)->c_int {
    let attr_sz = offsetofend!(bpf_attr, enable_stats);
    let mut attr:bpf_attr = mem::zeroed();
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.enable_stats.type_ = type_;
    libbpf_err_errno(sys_bpf_fd(BPF_ENABLE_STATS, &mut attr, attr_sz as c_uint))
}

pub unsafe extern "C" fn bpf_prog_bind_map(prog_fd:c_int,map_fd:c_int,opts:*const bpf_prog_bind_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, prog_bind_map);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_prog_bind_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.prog_bind_map.prog_fd = prog_fd; attr.prog_bind_map.map_fd = map_fd; attr.prog_bind_map.flags = OPTS_GET!(opts, flags, 0);
    libbpf_err_errno(sys_bpf(BPF_PROG_BIND_MAP, &mut attr, attr_sz as c_uint))
}

pub unsafe extern "C" fn bpf_token_create(bpffs_fd:c_int,opts:*mut bpf_token_create_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, token_create);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_token_create_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.token_create.bpffs_fd = bpffs_fd; attr.token_create.flags = OPTS_GET!(opts, flags, 0);
    libbpf_err_errno(sys_bpf_fd(BPF_TOKEN_CREATE, &mut attr, attr_sz as c_uint))
}

pub unsafe extern "C" fn bpf_prog_stream_read(prog_fd:c_int,stream_id:__u32,buf:*mut c_void,buf_len:__u32,opts:*mut bpf_prog_stream_read_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, prog_stream_read);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_prog_stream_read_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.prog_stream_read.stream_buf = ptr_to_u64(buf);
    attr.prog_stream_read.stream_buf_len = buf_len;
    attr.prog_stream_read.stream_id = stream_id;
    attr.prog_stream_read.prog_fd = prog_fd;
    libbpf_err_errno(sys_bpf(BPF_PROG_STREAM_READ_BY_FD, &mut attr, attr_sz as c_uint))
}

pub unsafe extern "C" fn bpf_prog_assoc_struct_ops(prog_fd:c_int,map_fd:c_int,opts:*mut bpf_prog_assoc_struct_ops_opts)->c_int {
    let attr_sz = offsetofend!(bpf_attr, prog_assoc_struct_ops);
    let mut attr:bpf_attr = mem::zeroed();
    if !OPTS_VALID!(opts, bpf_prog_assoc_struct_ops_opts) { return libbpf_err(-EINVAL); }
    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.prog_assoc_struct_ops.map_fd = map_fd;
    attr.prog_assoc_struct_ops.prog_fd = prog_fd;
    attr.prog_assoc_struct_ops.flags = OPTS_GET!(opts, flags, 0);
    libbpf_err_errno(sys_bpf(BPF_PROG_ASSOC_STRUCT_OPS, &mut attr, attr_sz as c_uint))
}
