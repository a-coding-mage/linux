/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */
/* Copyright (c) 2021 Facebook */

/*
 * C includes and the header guard from skel_internal.h are intentionally not
 * executable Rust. The translated items below depend on the same external BPF,
 * libc, and kernel symbols/types supplied by the surrounding build.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

pub type size_t = usize;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s32 = i32;
pub type __aligned_u64 = u64;

/*
 * Original C fallback:
 * #ifndef __NR_bpf
 * # if defined(__mips__) && defined(_ABIO32)
 * #  define __NR_bpf 4355
 * # elif defined(__mips__) && defined(_ABIN32)
 * #  define __NR_bpf 6319
 * # elif defined(__mips__) && defined(_ABI64)
 * #  define __NR_bpf 5315
 * # endif
 * #endif
 */
#[cfg(all(target_arch = "mips", target_pointer_width = "32"))]
pub const __NR_bpf: c_long = 4355;

/* This file is a base header for auto-generated *.lskel.h files.
 * Its contents will change and may become part of auto-generation in the future.
 *
 * The layout of bpf_[map|prog]_desc and bpf_loader_ctx is feature dependent
 * and will change from one version of libbpf to another and features
 * requested during loader program generation.
 */
#[repr(C)]
pub struct bpf_map_desc {
    /* output of the loader prog */
    pub map_fd: c_int,
    /* input for the loader prog */
    pub max_entries: __u32,
    pub initial_value: __aligned_u64,
}

#[repr(C)]
pub struct bpf_prog_desc {
    pub prog_fd: c_int,
}

pub const BPF_SKEL_KERNEL: __u64 = 1u64 << 0;

#[repr(C)]
pub struct bpf_loader_ctx {
    pub sz: __u32,
    pub flags: __u32,
    pub log_level: __u32,
    pub log_size: __u32,
    pub log_buf: __u64,
}

#[repr(C)]
pub struct bpf_load_and_run_opts {
    pub ctx: *mut bpf_loader_ctx,
    pub data: *const c_void,
    pub insns: *const c_void,
    pub data_sz: __u32,
    pub insns_sz: __u32,
    pub errstr: *const c_char,
    pub signature: *mut c_void,
    pub signature_sz: __u32,
    pub keyring_id: __s32,
    pub excl_prog_hash: *mut c_void,
    pub excl_prog_hash_sz: __u32,
}

extern "C" {
    pub fn kern_sys_bpf(cmd: __u32, attr: *mut c_void, attr_size: __u32) -> c_long;
    pub fn syscall(num: c_long, ...) -> c_long;
    pub fn close(fd: c_int) -> c_int;
    pub fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    pub fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
}

extern "C" {
    pub static mut errno: c_int;
}

/* External C/kernel/libbpf types and constants supplied by dependencies. */
pub type bpf_cmd = __u32;
pub type bpf_map_type = __u32;
pub type bpf_attach_type = __u32;
pub type bpf_prog_type = __u32;

#[repr(C)]
pub struct bpf_insn {
    pub _private: [u8; 0],
}

#[repr(C)]
pub union bpf_attr {
    pub map_type: __u32,
    pub map_fd: c_int,
    pub map_id: __u32,
    pub key: c_ulong,
    pub value: c_ulong,
    pub flags: __u64,
    pub raw_tracepoint: bpf_attr_raw_tracepoint,
    pub link_create: bpf_attr_link_create,
    pub test: bpf_attr_test,
    pub prog_type: __u32,
}

#[repr(C)]
pub struct bpf_attr_raw_tracepoint {
    pub name: c_ulong,
    pub prog_fd: c_int,
}

#[repr(C)]
pub struct bpf_attr_link_create {
    pub prog_fd: c_int,
    pub target_fd: c_int,
    pub attach_type: __u32,
    pub iter_info_len: __u32,
}

#[repr(C)]
pub struct bpf_attr_test {
    pub prog_fd: c_int,
    pub retval: __u32,
    pub ctx_in: c_ulong,
    pub ctx_size_in: __u32,
}

impl bpf_attr {
    pub unsafe fn map_name(&mut self) -> *mut c_char {
        self as *mut _ as *mut c_char
    }

    pub unsafe fn prog_name(&mut self) -> *mut c_char {
        self as *mut _ as *mut c_char
    }

    pub unsafe fn set_excl_prog_hash(&mut self, _value: c_ulong) {}
    pub unsafe fn set_excl_prog_hash_size(&mut self, _value: __u32) {}
    pub unsafe fn set_key_size(&mut self, _value: __u32) {}
    pub unsafe fn set_value_size(&mut self, _value: __u32) {}
    pub unsafe fn set_max_entries(&mut self, _value: __u32) {}
    pub unsafe fn set_insns(&mut self, _value: c_ulong) {}
    pub unsafe fn set_insn_cnt(&mut self, _value: __u32) {}
    pub unsafe fn set_license(&mut self, _value: c_ulong) {}
    pub unsafe fn set_signature(&mut self, _value: c_ulong) {}
    pub unsafe fn set_signature_size(&mut self, _value: __u32) {}
    pub unsafe fn set_fd_array_cnt(&mut self, _value: __u32) {}
    pub unsafe fn set_keyring_id(&mut self, _value: __s32) {}
    pub unsafe fn set_fd_array(&mut self, _value: c_ulong) {}
    pub unsafe fn set_log_level(&mut self, _value: __u32) {}
    pub unsafe fn set_log_size(&mut self, _value: __u32) {}
    pub unsafe fn set_log_buf(&mut self, _value: __u64) {}
    pub unsafe fn set_prog_flags(&mut self, _value: __u32) {}
}

extern "C" {
    pub static BPF_MAP_CREATE: bpf_cmd;
    pub static BPF_MAP_UPDATE_ELEM: bpf_cmd;
    pub static BPF_MAP_DELETE_ELEM: bpf_cmd;
    pub static BPF_MAP_GET_FD_BY_ID: bpf_cmd;
    pub static BPF_RAW_TRACEPOINT_OPEN: bpf_cmd;
    pub static BPF_LINK_CREATE: bpf_cmd;
    pub static BPF_MAP_FREEZE: bpf_cmd;
    pub static BPF_PROG_LOAD: bpf_cmd;
    pub static BPF_PROG_RUN: bpf_cmd;
    pub static BPF_MAP_TYPE_ARRAY: bpf_map_type;
    pub static BPF_PROG_TYPE_SYSCALL: bpf_prog_type;
    pub static BPF_F_SLEEPABLE: __u32;
    pub static EINVAL: c_int;
    pub static PROT_READ: c_int;
    pub static PROT_WRITE: c_int;
    pub static MAP_SHARED: c_int;
    pub static MAP_ANONYMOUS: c_int;
    pub static MAP_FIXED: c_int;
}

#[inline]
pub unsafe fn skel_sys_bpf(cmd: bpf_cmd, attr: *mut bpf_attr, size: c_uint) -> c_int {
    /*
     * Original C dispatches to kern_sys_bpf() under __KERNEL__ and syscall()
     * otherwise.
     */
    syscall(__NR_bpf, cmd, attr, size) as c_int
}

pub type c_uint = u32;

#[inline]
pub unsafe fn skel_alloc(size: size_t) -> *mut c_void {
    calloc(1, size)
}

#[inline]
pub unsafe fn skel_free(p: *mut c_void) {
    free(p);
}

/* skel->bss/rodata maps are populated the following way:
 *
 * For kernel use:
 * skel_prep_map_data() allocates kernel memory that kernel module can directly access.
 * Generated lskel stores the pointer in skel->rodata and in skel->maps.rodata.initial_value.
 * The loader program will perform probe_read_kernel() from maps.rodata.initial_value.
 * skel_finalize_map_data() sets skel->rodata to point to actual value in a bpf map and
 * does maps.rodata.initial_value = ~0ULL to signal skel_free_map_data() that kvfree
 * is not necessary.
 *
 * For user space:
 * skel_prep_map_data() mmaps anon memory into skel->rodata that can be accessed directly.
 * Generated lskel stores the pointer in skel->rodata and in skel->maps.rodata.initial_value.
 * The loader program will perform copy_from_user() from maps.rodata.initial_value.
 * skel_finalize_map_data() remaps bpf array map value from the kernel memory into
 * skel->rodata address.
 *
 * The "bpftool gen skeleton -L" command generates lskel.h that is suitable for
 * both kernel and user space. The generated loader program does
 * either bpf_probe_read_kernel() or bpf_copy_from_user() from initial_value
 * depending on bpf_loader_ctx->flags.
 */
#[inline]
pub unsafe fn skel_free_map_data(p: *mut c_void, _addr: __u64, sz: size_t) {
    munmap(p, sz);
}

#[inline]
pub unsafe fn skel_prep_map_data(
    val: *const c_void,
    mmap_sz: size_t,
    val_sz: size_t,
) -> *mut c_void {
    let mut addr: *mut c_void;

    addr = mmap(
        ptr::null_mut(),
        mmap_sz,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    );
    if addr == (-1isize) as *mut c_void {
        return ptr::null_mut();
    }
    memcpy(addr, val, val_sz);
    addr
}

#[inline]
pub unsafe fn skel_finalize_map_data(
    init_val: *mut __u64,
    mmap_sz: size_t,
    flags: c_int,
    fd: c_int,
) -> *mut c_void {
    let mut addr: *mut c_void;

    addr = mmap(
        (*init_val as c_long) as *mut c_void,
        mmap_sz,
        flags,
        MAP_SHARED | MAP_FIXED,
        fd,
        0,
    );
    if addr == (-1isize) as *mut c_void {
        return ptr::null_mut();
    }
    addr
}

#[inline]
pub unsafe fn skel_protect_map_data(
    p: *mut c_void,
    _init_val: *mut __u64,
    sz: size_t,
) -> c_int {
    if mprotect(p, sz, PROT_READ) != 0 {
        return -errno;
    }
    0
}

#[inline]
pub unsafe fn skel_closenz(fd: c_int) -> c_int {
    if fd > 0 {
        return close(fd);
    }
    -EINVAL
}

#[macro_export]
macro_rules! offsetofend {
    ($ty:ty, $member:tt) => {
        offset_of!($ty, $member) + size_of_val(&(*(core::ptr::null::<$ty>())).$member)
    };
}

#[inline]
pub unsafe fn skel_map_create(
    map_type: bpf_map_type,
    map_name: *const c_char,
    key_size: __u32,
    value_size: __u32,
    max_entries: __u32,
    excl_prog_hash: *const c_void,
    excl_prog_hash_sz: __u32,
) -> c_int {
    let attr_sz: size_t = size_of::<bpf_attr>();
    let mut attr: bpf_attr = core::mem::zeroed();

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);

    attr.map_type = map_type;
    attr.set_excl_prog_hash(excl_prog_hash as c_ulong);
    attr.set_excl_prog_hash_size(excl_prog_hash_sz);

    strncpy(attr.map_name(), map_name, 16);
    attr.set_key_size(key_size);
    attr.set_value_size(value_size);
    attr.set_max_entries(max_entries);

    skel_sys_bpf(BPF_MAP_CREATE, &mut attr, attr_sz as c_uint)
}

#[inline]
pub unsafe fn skel_map_update_elem(
    fd: c_int,
    key: *const c_void,
    value: *const c_void,
    flags: __u64,
) -> c_int {
    let attr_sz: size_t = size_of::<bpf_attr>();
    let mut attr: bpf_attr = core::mem::zeroed();

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.map_fd = fd;
    attr.key = key as c_long as c_ulong;
    attr.value = value as c_long as c_ulong;
    attr.flags = flags;

    skel_sys_bpf(BPF_MAP_UPDATE_ELEM, &mut attr, attr_sz as c_uint)
}

#[inline]
pub unsafe fn skel_map_delete_elem(fd: c_int, key: *const c_void) -> c_int {
    let attr_sz: size_t = size_of::<bpf_attr>();
    let mut attr: bpf_attr = core::mem::zeroed();

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.map_fd = fd;
    attr.key = key as c_long as c_ulong;

    skel_sys_bpf(BPF_MAP_DELETE_ELEM, &mut attr, attr_sz as c_uint)
}

#[inline]
pub unsafe fn skel_map_get_fd_by_id(id: __u32) -> c_int {
    let attr_sz: size_t = size_of::<bpf_attr>();
    let mut attr: bpf_attr = core::mem::zeroed();

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.map_id = id;

    skel_sys_bpf(BPF_MAP_GET_FD_BY_ID, &mut attr, attr_sz as c_uint)
}

#[inline]
pub unsafe fn skel_raw_tracepoint_open(name: *const c_char, prog_fd: c_int) -> c_int {
    let attr_sz: size_t = size_of::<bpf_attr>();
    let mut attr: bpf_attr = core::mem::zeroed();

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.raw_tracepoint.name = name as c_long as c_ulong;
    attr.raw_tracepoint.prog_fd = prog_fd;

    skel_sys_bpf(BPF_RAW_TRACEPOINT_OPEN, &mut attr, attr_sz as c_uint)
}

#[inline]
pub unsafe fn skel_link_create(
    prog_fd: c_int,
    target_fd: c_int,
    attach_type: bpf_attach_type,
) -> c_int {
    let attr_sz: size_t = size_of::<bpf_attr>();
    let mut attr: bpf_attr = core::mem::zeroed();

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.link_create.prog_fd = prog_fd;
    attr.link_create.target_fd = target_fd;
    attr.link_create.attach_type = attach_type;

    skel_sys_bpf(BPF_LINK_CREATE, &mut attr, attr_sz as c_uint)
}

#[inline]
pub unsafe fn skel_map_freeze(fd: c_int) -> c_int {
    let attr_sz: size_t = size_of::<bpf_attr>();
    let mut attr: bpf_attr = core::mem::zeroed();

    memset(&mut attr as *mut _ as *mut c_void, 0, attr_sz);
    attr.map_fd = fd;

    skel_sys_bpf(BPF_MAP_FREEZE, &mut attr, attr_sz as c_uint)
}

#[inline]
pub unsafe fn bpf_load_and_run(opts: *mut bpf_load_and_run_opts) -> c_int {
    let prog_load_attr_sz: size_t = size_of::<bpf_attr>();
    let test_run_attr_sz: size_t = size_of::<bpf_attr>();
    let mut map_fd: c_int = -1;
    let mut prog_fd: c_int = -1;
    let mut key: c_int = 0;
    let mut err: c_int;
    let mut attr: bpf_attr = core::mem::zeroed();

    map_fd = skel_map_create(
        BPF_MAP_TYPE_ARRAY,
        b"__loader.map\0".as_ptr() as *const c_char,
        4,
        (*opts).data_sz,
        1,
        (*opts).excl_prog_hash,
        (*opts).excl_prog_hash_sz,
    );
    err = map_fd;
    if map_fd < 0 {
        (*opts).errstr = b"failed to create loader map\0".as_ptr() as *const c_char;
        err = -errno;
        goto_out(map_fd, prog_fd);
        return err;
    }

    err = skel_map_update_elem(map_fd, &key as *const _ as *const c_void, (*opts).data, 0);
    if err < 0 {
        (*opts).errstr = b"failed to update loader map\0".as_ptr() as *const c_char;
        err = -errno;
        goto_out(map_fd, prog_fd);
        return err;
    }

    err = skel_map_freeze(map_fd);
    if err < 0 {
        (*opts).errstr = b"failed to freeze map\0".as_ptr() as *const c_char;
        err = -errno;
        goto_out(map_fd, prog_fd);
        return err;
    }

    memset(&mut attr as *mut _ as *mut c_void, 0, prog_load_attr_sz);
    attr.prog_type = BPF_PROG_TYPE_SYSCALL;
    attr.set_insns((*opts).insns as c_long as c_ulong);
    attr.set_insn_cnt((*opts).insns_sz / size_of::<bpf_insn>() as __u32);
    attr.set_license(b"Dual BSD/GPL\0".as_ptr() as c_long as c_ulong);
    attr.set_signature((*opts).signature as c_long as c_ulong);
    attr.set_signature_size((*opts).signature_sz);
    if !(*opts).signature.is_null() {
        attr.set_fd_array_cnt(1);
    }
    attr.set_keyring_id((*opts).keyring_id);
    memcpy(
        attr.prog_name() as *mut c_void,
        b"__loader.prog\0".as_ptr() as *const c_void,
        size_of_val(b"__loader.prog\0"),
    );
    attr.set_fd_array(&mut map_fd as *mut _ as c_long as c_ulong);
    attr.set_log_level((*(*opts).ctx).log_level);
    attr.set_log_size((*(*opts).ctx).log_size);
    attr.set_log_buf((*(*opts).ctx).log_buf);
    attr.set_prog_flags(BPF_F_SLEEPABLE);
    prog_fd = skel_sys_bpf(BPF_PROG_LOAD, &mut attr, prog_load_attr_sz as c_uint);
    err = prog_fd;
    if prog_fd < 0 {
        (*opts).errstr = b"failed to load loader prog\0".as_ptr() as *const c_char;
        err = -errno;
        goto_out(map_fd, prog_fd);
        return err;
    }

    memset(&mut attr as *mut _ as *mut c_void, 0, test_run_attr_sz);
    attr.test.prog_fd = prog_fd;
    attr.test.ctx_in = (*opts).ctx as c_long as c_ulong;
    attr.test.ctx_size_in = (*(*opts).ctx).sz;
    err = skel_sys_bpf(BPF_PROG_RUN, &mut attr, test_run_attr_sz as c_uint);
    if err < 0 || attr.test.retval as c_int < 0 {
        if err < 0 {
            (*opts).errstr = b"failed to execute loader prog\0".as_ptr() as *const c_char;
            err = -errno;
        } else {
            (*opts).errstr = b"error returned by loader prog\0".as_ptr() as *const c_char;
            err = attr.test.retval as c_int;
            errno = -err;
        }
        goto_out(map_fd, prog_fd);
        return err;
    }
    err = 0;

    goto_out(map_fd, prog_fd);
    err
}

#[inline]
unsafe fn goto_out(map_fd: c_int, prog_fd: c_int) {
    if map_fd >= 0 {
        close(map_fd);
    }
    if prog_fd >= 0 {
        close(prog_fd);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
