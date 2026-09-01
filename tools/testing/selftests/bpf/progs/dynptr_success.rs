// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Facebook */

// Rust translation of dynptr_success.c. Includes from vmlinux.h, bpf helpers,
// bpf_misc.h, and errno.h are represented as external dependencies below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type u32 = u32;
type u64 = u64;

const PAGE_SIZE_64K: __u32 = 65536;
const DYNPTR_MEMSET_VAL: i32 = 42;

const BPF_MAP_TYPE_RINGBUF: u32 = 27;
const BPF_MAP_TYPE_ARRAY: u32 = 2;
const EINVAL: i32 = 22;
const ERANGE: i32 = 34;
const E2BIG: i32 = 7;
const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;
const __PAGE_SIZE: __u32 = 4096;

#[repr(C)]
pub struct bpf_dynptr {
    _data: [u64; 2],
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xdp_md {
    _private: [u8; 0],
}

#[repr(C)]
pub struct skb_shared_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct ringbuf_sample {
    pid: i32,
    seq: i32,
    value: isize,
    comm: [i8; 16],
}

#[repr(C)]
struct ringbuf_map_def {
    type_: u32,
    max_entries: u32,
}

#[repr(C)]
struct array_map_def {
    type_: u32,
    max_entries: u32,
    key_size: u32,
    value_size: u32,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut pid: i32 = 0;
#[no_mangle]
pub static mut err: i32 = 0;
#[no_mangle]
pub static mut val: i32 = 0;

#[no_mangle]
#[link_section = ".maps"]
static mut ringbuf: ringbuf_map_def = ringbuf_map_def {
    type_: BPF_MAP_TYPE_RINGBUF,
    max_entries: 4096,
};

#[no_mangle]
#[link_section = ".maps"]
static mut array_map: array_map_def = array_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u32>() as u32,
};

type bpf_read_dynptr_fn_t = unsafe extern "C" fn(
    dptr: *const bpf_dynptr,
    off: u64,
    size: u64,
    unsafe_ptr: *const core::ffi::c_void,
) -> i32;

extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_ringbuf_reserve_dynptr(
        ringbuf: *mut core::ffi::c_void,
        size: __u64,
        flags: __u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_ringbuf_discard_dynptr(ptr: *mut bpf_dynptr, flags: __u64);
    fn bpf_ringbuf_submit_dynptr(ptr: *mut bpf_dynptr, flags: __u64);
    fn bpf_dynptr_write(
        ptr: *mut bpf_dynptr,
        offset: __u32,
        data: *const core::ffi::c_void,
        len: __u32,
        flags: __u64,
    ) -> i32;
    fn bpf_dynptr_read(
        data: *mut core::ffi::c_void,
        len: __u32,
        ptr: *const bpf_dynptr,
        offset: __u32,
        flags: __u64,
    ) -> i32;
    fn bpf_dynptr_data(
        ptr: *const bpf_dynptr,
        offset: __u32,
        len: __u32,
    ) -> *mut core::ffi::c_void;
    fn bpf_dynptr_from_mem(
        data: *mut core::ffi::c_void,
        size: __u32,
        flags: __u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_dynptr_from_skb(skb: *mut core::ffi::c_void, flags: __u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_from_skb_meta(
        skb: *mut __sk_buff,
        flags: __u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_dynptr_from_xdp(xdp: *mut xdp_md, flags: __u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_slice(
        ptr: *const bpf_dynptr,
        offset: __u32,
        buffer: *mut core::ffi::c_void,
        len: __u32,
    ) -> *mut core::ffi::c_void;
    fn bpf_dynptr_adjust(ptr: *mut bpf_dynptr, start: __u32, end: __u32) -> i32;
    fn bpf_dynptr_size(ptr: *const bpf_dynptr) -> __u32;
    fn bpf_dynptr_is_null(ptr: *const bpf_dynptr) -> bool;
    fn bpf_dynptr_is_rdonly(ptr: *const bpf_dynptr) -> bool;
    fn bpf_dynptr_clone(src: *const bpf_dynptr, dst: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_copy(
        dst: *mut bpf_dynptr,
        dst_offset: __u32,
        src: *const bpf_dynptr,
        src_offset: __u32,
        len: __u32,
    ) -> i32;
    fn bpf_dynptr_memset(ptr: *mut bpf_dynptr, offset: __u32, len: __u32, value: i32) -> i32;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i32;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_probe_read_kernel(
        dst: *mut core::ffi::c_void,
        size: __u32,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i32;
    fn bpf_loop(
        nr_loops: __u32,
        callback_fn: unsafe extern "C" fn(__u32, *mut core::ffi::c_void) -> i32,
        callback_ctx: *mut core::ffi::c_void,
        flags: __u64,
    ) -> i32;
    fn bpf_strncmp(s1: *const i8, s1_sz: __u32, s2: *const i8) -> i32;
    fn bpf_probe_read_user_dynptr(
        dptr: *const bpf_dynptr,
        off: u64,
        size: u64,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i32;
    fn bpf_probe_read_kernel_dynptr(
        dptr: *const bpf_dynptr,
        off: u64,
        size: u64,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i32;
    fn bpf_probe_read_user_str_dynptr(
        dptr: *const bpf_dynptr,
        off: u64,
        size: u64,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i32;
    fn bpf_probe_read_kernel_str_dynptr(
        dptr: *const bpf_dynptr,
        off: u64,
        size: u64,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i32;
    fn bpf_copy_from_user_dynptr(
        dptr: *const bpf_dynptr,
        off: u64,
        size: u64,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i32;
    fn bpf_copy_from_user_str_dynptr(
        dptr: *const bpf_dynptr,
        off: u64,
        size: u64,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i32;
    fn bpf_copy_from_user_task_dynptr(
        dptr: *const bpf_dynptr,
        off: u64,
        size: u64,
        unsafe_ptr: *const core::ffi::c_void,
        task: *mut task_struct,
    ) -> i32;
    fn bpf_copy_from_user_task_str_dynptr(
        dptr: *const bpf_dynptr,
        off: u64,
        size: u64,
        unsafe_ptr: *const core::ffi::c_void,
        task: *mut task_struct,
    ) -> i32;
    fn bpf_get_current_task_btf() -> *mut task_struct;
}

#[inline(always)]
unsafe fn ternary_err(next: i32) {
    if err == 0 {
        err = next;
    }
}

// SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_read_write(_ctx: *mut core::ffi::c_void) -> i32 {
    let write_data: [i8; 64] = {
        let mut a = [0i8; 64];
        let s = b"hello there, world!!\0";
        let mut i = 0usize;
        while i < s.len() {
            a[i] = s[i] as i8;
            i += 1;
        }
        a
    };
    let mut read_data = [0i8; 64];
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut i: i32;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return 0;
    }

    bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, write_data.len() as __u64, 0, &mut ptr);
    err = bpf_dynptr_write(&mut ptr, 0, write_data.as_ptr() as *const _, write_data.len() as __u32, 0);
    ternary_err(bpf_dynptr_read(read_data.as_mut_ptr() as *mut _, read_data.len() as __u32, &ptr, 0, 0));

    i = 0;
    while (i as usize) < read_data.len() {
        if read_data[i as usize] != write_data[i as usize] {
            err = 1;
            break;
        }
        i += 1;
    }

    bpf_ringbuf_discard_dynptr(&mut ptr, 0);
    0
}

// SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_data(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: __u32 = 0;
    let mut local_val: __u32 = 235;
    let mut map_val: *mut __u32;
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let map_val_size: __u32;
    let mut data: *mut core::ffi::c_void;

    map_val_size = core::mem::size_of::<__u32>() as __u32;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return 0;
    }

    bpf_map_update_elem(&mut array_map as *mut _ as *mut _, &mut key as *mut _ as *const _, &mut local_val as *mut _ as *const _, 0);

    map_val = bpf_map_lookup_elem(&mut array_map as *mut _ as *mut _, &mut key as *mut _ as *const _) as *mut __u32;
    if map_val.is_null() {
        err = 1;
        return 0;
    }

    bpf_dynptr_from_mem(map_val as *mut _, map_val_size, 0, &mut ptr);

    data = bpf_dynptr_data(&ptr, map_val_size + 1, 1);
    if !data.is_null() {
        err = 2;
        return 0;
    }

    data = bpf_dynptr_data(&ptr, 0, map_val_size + 1);
    if !data.is_null() {
        err = 3;
        return 0;
    }

    data = bpf_dynptr_data(&ptr, 0, core::mem::size_of::<__u32>() as __u32);
    if data.is_null() {
        err = 4;
        return 0;
    }

    *(data as *mut __u32) = 999;

    err = bpf_probe_read_kernel(&mut local_val as *mut _ as *mut _, core::mem::size_of_val(&local_val) as __u32, data);
    if err != 0 {
        return 0;
    }

    if local_val != *(data as *mut i32) as __u32 {
        err = 5;
    }

    0
}

static unsafe extern "C" fn ringbuf_callback(index: __u32, data: *mut core::ffi::c_void) -> i32 {
    let mut sample: *mut ringbuf_sample;
    let ptr: *mut bpf_dynptr = data as *mut bpf_dynptr;

    sample = bpf_dynptr_data(ptr, 0, core::mem::size_of::<ringbuf_sample>() as __u32) as *mut ringbuf_sample;
    if sample.is_null() {
        err = 2;
    } else {
        (*sample).pid += index as i32;
    }

    0
}

// SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_ringbuf(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut sample: *mut ringbuf_sample;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return 0;
    }

    val = 100;
    err = bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, val as __u64, 0, &mut ptr);
    sample = if err != 0 {
        core::ptr::null_mut()
    } else {
        bpf_dynptr_data(&ptr, 0, core::mem::size_of::<ringbuf_sample>() as __u32) as *mut ringbuf_sample
    };
    if sample.is_null() {
        err = 1;
    } else {
        (*sample).pid = 10;
        bpf_loop(10, ringbuf_callback, &mut ptr as *mut _ as *mut _, 0);
        if (*sample).pid != 55 {
            err = 2;
        }
    }

    bpf_ringbuf_discard_dynptr(&mut ptr, 0);
    0
}

// SEC("?cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn test_skb_readonly(skb: *mut __sk_buff) -> i32 {
    let write_data: [__u8; 2] = [1, 2];
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let ret: i32;

    if bpf_dynptr_from_skb(skb as *mut _, 0, &mut ptr) != 0 {
        err = 1;
        return 1;
    }

    ret = bpf_dynptr_write(&mut ptr, 0, write_data.as_ptr() as *const _, write_data.len() as __u32, 0);
    if ret != -EINVAL {
        err = 2;
        return 1;
    }

    1
}

// SEC("?cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_skb_data(skb: *mut __sk_buff) -> i32 {
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let data: *mut __u64;

    if bpf_dynptr_from_skb(skb as *mut _, 0, &mut ptr) != 0 {
        err = 1;
        return 1;
    }

    data = bpf_dynptr_data(&ptr, 0, 1) as *mut __u64;
    if !data.is_null() {
        err = 2;
        return 1;
    }

    1
}

// SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_skb_meta_data(skb: *mut __sk_buff) -> i32 {
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let md: *mut __u8;
    let ret: i32;

    err = 1;
    ret = bpf_dynptr_from_skb_meta(skb, 0, &mut meta);
    if ret != 0 {
        return 1;
    }

    err = 2;
    md = bpf_dynptr_data(&meta, 0, core::mem::size_of::<__u8>() as __u32) as *mut __u8;
    if !md.is_null() {
        return 1;
    }

    err = 0;
    1
}

/* Check that skb metadata dynptr ops don't accept any flags. */
// SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_skb_meta_flags(skb: *mut __sk_buff) -> i32 {
    const INVALID_FLAGS: __u64 = !0u64;
    let mut meta = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut buf: __u8 = 0;
    let mut ret: i32;

    err = 1;
    ret = bpf_dynptr_from_skb_meta(skb, INVALID_FLAGS, &mut meta);
    if ret != -EINVAL {
        return 1;
    }

    err = 2;
    ret = bpf_dynptr_from_skb_meta(skb, 0, &mut meta);
    if ret != 0 {
        return 1;
    }

    err = 3;
    ret = bpf_dynptr_read(&mut buf as *mut _ as *mut _, 0, &meta, 0, INVALID_FLAGS);
    if ret != -EINVAL {
        return 1;
    }

    err = 4;
    ret = bpf_dynptr_write(&mut meta, 0, &mut buf as *mut _ as *const _, 0, INVALID_FLAGS);
    if ret != -EINVAL {
        return 1;
    }

    err = 0;
    1
}

// SEC("tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_adjust(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let bytes: __u32 = 64;
    let off: __u32 = 10;
    let trim: __u32 = 15;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return 0;
    }

    err = bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, bytes as __u64, 0, &mut ptr);
    if err != 0 {
        err = 1;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_size(&ptr) != bytes {
        err = 2;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    err = bpf_dynptr_adjust(&mut ptr, off, bpf_dynptr_size(&ptr));
    if err != 0 {
        err = 3;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_size(&ptr) != bytes - off {
        err = 4;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    err = bpf_dynptr_adjust(&mut ptr, off, 15);
    if err != 0 {
        err = 5;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_size(&ptr) != trim - off {
        err = 6;
    }

    bpf_ringbuf_discard_dynptr(&mut ptr, 0);
    0
}

// SEC("tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_adjust_err(_ctx: *mut core::ffi::c_void) -> i32 {
    let write_data: [i8; 45] = {
        let mut a = [0i8; 45];
        let s = b"hello there, world!!\0";
        let mut i = 0usize;
        while i < s.len() {
            a[i] = s[i] as i8;
            i += 1;
        }
        a
    };
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let size: __u32 = 64;
    let off: __u32 = 20;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return 0;
    }

    if bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, size as __u64, 0, &mut ptr) != 0 {
        err = 1;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_adjust(&mut ptr, 5, 1) != -EINVAL {
        err = 2;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_adjust(&mut ptr, size + 1, size + 1) != -ERANGE {
        err = 3;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_adjust(&mut ptr, 0, size + 1) != -ERANGE {
        err = 4;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_adjust(&mut ptr, off, size) != 0 {
        err = 5;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_write(&mut ptr, 0, write_data.as_ptr() as *const _, write_data.len() as __u32, 0) != -E2BIG {
        err = 6;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    bpf_ringbuf_submit_dynptr(&mut ptr, 0);
    0
}

// SEC("tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_zero_size_dynptr(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut write_data: i8 = b'x' as i8;
    let mut read_data: i8 = 0;
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let size: __u32 = 64;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return 0;
    }

    if bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, size as __u64, 0, &mut ptr) != 0 {
        err = 1;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_adjust(&mut ptr, size, size) != 0 {
        err = 2;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_read(&mut read_data as *mut _ as *mut _, core::mem::size_of_val(&read_data) as __u32, &ptr, 0, 0) != -E2BIG {
        err = 3;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_write(&mut ptr, 0, &mut write_data as *mut _ as *const _, core::mem::size_of_val(&write_data) as __u32, 0) != -E2BIG {
        err = 4;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_read(&mut read_data as *mut _ as *mut _, 0, &ptr, 0, 0) != 0 {
        err = 5;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    if bpf_dynptr_write(&mut ptr, 0, &mut write_data as *mut _ as *const _, 0, 0) != 0 {
        err = 6;
        bpf_ringbuf_discard_dynptr(&mut ptr, 0);
        return 0;
    }

    err = 0;
    bpf_ringbuf_discard_dynptr(&mut ptr, 0);
    0
}

// SEC("tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_is_null(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut ptr1 = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut ptr2 = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let size: __u64 = 4;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return 0;
    }

    if bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, size, 123, &mut ptr1) != -EINVAL {
        err = 1;
        bpf_ringbuf_discard_dynptr(&mut ptr1, 0);
        return 0;
    }

    if !bpf_dynptr_is_null(&ptr1) {
        err = 2;
        bpf_ringbuf_discard_dynptr(&mut ptr1, 0);
        return 0;
    }

    if bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, size, 0, &mut ptr2) != 0 {
        err = 3;
        bpf_ringbuf_discard_dynptr(&mut ptr2, 0);
        bpf_ringbuf_discard_dynptr(&mut ptr1, 0);
        return 0;
    }

    if bpf_dynptr_is_null(&ptr2) {
        err = 4;
    }

    bpf_ringbuf_discard_dynptr(&mut ptr2, 0);
    bpf_ringbuf_discard_dynptr(&mut ptr1, 0);
    0
}

// SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_is_rdonly(skb: *mut __sk_buff) -> i32 {
    let mut ptr1 = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut ptr2 = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut ptr3 = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();

    if bpf_dynptr_from_skb(skb as *mut _, 123, &mut ptr1) != -EINVAL {
        err = 1;
        return 0;
    }

    if bpf_dynptr_is_rdonly(&ptr1) {
        err = 2;
        return 0;
    }

    if bpf_dynptr_from_skb(skb as *mut _, 0, &mut ptr2) != 0 {
        err = 3;
        return 0;
    }

    if !bpf_dynptr_is_rdonly(&ptr2) {
        err = 4;
        return 0;
    }

    if bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, 64, 0, &mut ptr3) != 0 {
        err = 5;
        bpf_ringbuf_discard_dynptr(&mut ptr3, 0);
        return 0;
    }

    if bpf_dynptr_is_rdonly(&ptr3) {
        err = 6;
    }

    bpf_ringbuf_discard_dynptr(&mut ptr3, 0);
    0
}

// SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_clone(skb: *mut __sk_buff) -> i32 {
    let mut ptr1 = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut ptr2 = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let off: __u32 = 2;
    let size: __u32;

    if bpf_dynptr_from_skb(skb as *mut _, 0, &mut ptr1) != 0 {
        err = 1;
        return 0;
    }

    if bpf_dynptr_adjust(&mut ptr1, off, bpf_dynptr_size(&ptr1)) != 0 {
        err = 2;
        return 0;
    }

    if bpf_dynptr_clone(&ptr1, &mut ptr2) != 0 {
        err = 3;
        return 0;
    }

    size = bpf_dynptr_size(&ptr1);

    if bpf_dynptr_size(&ptr2) != size {
        err = 4;
        return 0;
    }

    if bpf_dynptr_is_rdonly(&ptr2) != bpf_dynptr_is_rdonly(&ptr1) {
        err = 5;
        return 0;
    }

    bpf_dynptr_adjust(&mut ptr1, 5, 5);

    if bpf_dynptr_size(&ptr2) != size {
        err = 6;
        return 0;
    }

    0
}

// SEC("?cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_skb_no_buff(skb: *mut __sk_buff) -> i32 {
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let data: *mut __u64;

    if bpf_dynptr_from_skb(skb as *mut _, 0, &mut ptr) != 0 {
        err = 1;
        return 1;
    }

    data = bpf_dynptr_slice(&ptr, 0, core::ptr::null_mut(), 1) as *mut __u64;
    (!data.is_null()) as i32
}

// SEC("?cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_skb_strcmp(skb: *mut __sk_buff) -> i32 {
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let data: *mut i8;

    if bpf_dynptr_from_skb(skb as *mut _, 0, &mut ptr) != 0 {
        err = 1;
        return 1;
    }

    data = bpf_dynptr_slice(&ptr, 0, core::ptr::null_mut(), 10) as *mut i8;
    if !data.is_null() {
        bpf_strncmp(data, 10, c"foo".as_ptr());
        return 1;
    }

    1
}

// SEC("tp_btf/kfree_skb")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_skb_tp_btf(
    skb: *mut core::ffi::c_void,
    _location: *mut core::ffi::c_void,
) -> i32 {
    let write_data: [__u8; 2] = [1, 2];
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let ret: i32;

    if bpf_dynptr_from_skb(skb, 0, &mut ptr) != 0 {
        err = 1;
        return 1;
    }

    ret = bpf_dynptr_write(&mut ptr, 0, write_data.as_ptr() as *const _, write_data.len() as __u32, 0);
    if ret != -EINVAL {
        err = 2;
        return 1;
    }

    1
}

#[inline(always)]
unsafe fn bpf_memcmp(a: *const i8, b: *const i8, size: u32) -> i32 {
    let mut i: i32 = 0;
    while (i as u32) < size {
        let av = *a.add(i as usize);
        let bv = *b.add(i as usize);
        if av != bv {
            return if av < bv { -1 } else { 1 };
        }
        i += 1;
    }
    0
}

// SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_copy(_ctx: *mut core::ffi::c_void) -> i32 {
    let data: [i8; 21] = *b"hello there, world!!\0".as_ptr().cast::<[i8; 21]>();
    let mut buf = [0i8; 32];
    let sz: __u32 = data.len() as __u32;
    let mut src = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut dst = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();

    bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, sz as __u64, 0, &mut src);
    bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, sz as __u64, 0, &mut dst);

    err = bpf_dynptr_write(&mut src, 0, data.as_ptr() as *const _, sz, 0);
    ternary_err(bpf_dynptr_copy(&mut dst, 0, &src, 0, sz));
    ternary_err(bpf_dynptr_read(buf.as_mut_ptr() as *mut _, sz, &dst, 0, 0));
    ternary_err(bpf_memcmp(data.as_ptr(), buf.as_ptr(), sz));

    ternary_err(bpf_dynptr_copy(&mut dst, 3, &src, 5, sz - 5));
    ternary_err(bpf_dynptr_read(buf.as_mut_ptr() as *mut _, sz - 5, &dst, 3, 0));
    ternary_err(bpf_memcmp(data.as_ptr().add(5), buf.as_ptr(), sz - 5));

    bpf_ringbuf_discard_dynptr(&mut src, 0);
    bpf_ringbuf_discard_dynptr(&mut dst, 0);
    0
}

// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_copy_xdp(xdp: *mut xdp_md) -> i32 {
    let mut ptr_buf = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut ptr_xdp = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let data: [i8; 20] = *b"qwertyuiopasdfghjkl\0".as_ptr().cast::<[i8; 20]>();
    let mut buf = [0i8; 32];
    let len: __u32 = data.len() as __u32;
    let xdp_data_size: __u32;
    let mut i: i32;
    let chunks: i32 = 200;

    bpf_dynptr_from_xdp(xdp, 0, &mut ptr_xdp);
    xdp_data_size = bpf_dynptr_size(&ptr_xdp);
    bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, (len as i32 * chunks) as __u64, 0, &mut ptr_buf);

    i = 0;
    while i < chunks {
        err = bpf_dynptr_write(&mut ptr_buf, (i as __u32) * len, data.as_ptr() as *const _, len, 0);
        if err != 0 {
            bpf_ringbuf_discard_dynptr(&mut ptr_buf, 0);
            return XDP_DROP;
        }
        i += 1;
    }

    err = bpf_dynptr_copy(&mut ptr_xdp, 0, &ptr_buf, 0, len * chunks as __u32);
    if err != 0 {
        bpf_ringbuf_discard_dynptr(&mut ptr_buf, 0);
        return XDP_DROP;
    }

    i = 0;
    while i < chunks {
        core::ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len());
        err = bpf_dynptr_read(buf.as_mut_ptr() as *mut _, len, &ptr_xdp, (i as __u32) * len, 0);
        if err != 0 || bpf_memcmp(data.as_ptr(), buf.as_ptr(), len) != 0 {
            bpf_ringbuf_discard_dynptr(&mut ptr_buf, 0);
            return XDP_DROP;
        }
        i += 1;
    }

    core::ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len());
    i = 0;
    while i < chunks {
        err = bpf_dynptr_write(&mut ptr_buf, (i as __u32) * len, buf.as_ptr() as *const _, len, 0);
        if err != 0 {
            bpf_ringbuf_discard_dynptr(&mut ptr_buf, 0);
            return XDP_DROP;
        }
        i += 1;
    }

    err = bpf_dynptr_copy(&mut ptr_buf, 0, &ptr_xdp, 0, len * chunks as __u32);
    if err != 0 {
        bpf_ringbuf_discard_dynptr(&mut ptr_buf, 0);
        return XDP_DROP;
    }

    i = 0;
    while i < chunks {
        core::ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len());
        err = bpf_dynptr_read(buf.as_mut_ptr() as *mut _, len, &ptr_buf, (i as __u32) * len, 0);
        if err != 0 || bpf_memcmp(data.as_ptr(), buf.as_ptr(), len) != 0 {
            bpf_ringbuf_discard_dynptr(&mut ptr_buf, 0);
            return XDP_DROP;
        }
        i += 1;
    }

    err = bpf_dynptr_copy(&mut ptr_xdp, 2, &ptr_xdp, len, len * ((chunks - 1) as __u32));
    if err != 0 {
        bpf_ringbuf_discard_dynptr(&mut ptr_buf, 0);
        return XDP_DROP;
    }

    i = 0;
    while i < chunks - 1 {
        core::ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len());
        err = bpf_dynptr_read(buf.as_mut_ptr() as *mut _, len, &ptr_xdp, 2 + (i as __u32) * len, 0);
        if err != 0 || bpf_memcmp(data.as_ptr(), buf.as_ptr(), len) != 0 {
            bpf_ringbuf_discard_dynptr(&mut ptr_buf, 0);
            return XDP_DROP;
        }
        i += 1;
    }

    if bpf_dynptr_copy(&mut ptr_xdp, xdp_data_size - 3000, &ptr_xdp, 0, len * chunks as __u32) != -E2BIG {
        err = 1;
    }

    bpf_ringbuf_discard_dynptr(&mut ptr_buf, 0);
    XDP_DROP
}

#[no_mangle]
pub static mut memset_zero_data: [i8; 18] = *b"data to be zeroed\0".as_ptr().cast::<[i8; 18]>();

// SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_memset_zero(_ctx: *mut core::ffi::c_void) -> i32 {
    let data_sz: __u32 = core::mem::size_of_val(&memset_zero_data) as __u32;
    let zeroes = [0i8; 32];
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();

    err = bpf_dynptr_from_mem(memset_zero_data.as_mut_ptr() as *mut _, data_sz, 0, &mut ptr);
    ternary_err(bpf_dynptr_memset(&mut ptr, 0, data_sz, 0));
    ternary_err(bpf_memcmp(zeroes.as_ptr(), memset_zero_data.as_ptr(), data_sz));

    0
}

#[no_mangle]
pub static mut memset_notzero_data: [i8; 23] = *b"data to be overwritten\0".as_ptr().cast::<[i8; 23]>();

// SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_memset_notzero(_ctx: *mut core::ffi::c_void) -> i32 {
    let data_sz: u32 = core::mem::size_of_val(&memset_notzero_data) as u32;
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut expected = [0i8; 32];

    core::ptr::write_bytes(expected.as_mut_ptr(), DYNPTR_MEMSET_VAL as u8, data_sz as usize);

    err = bpf_dynptr_from_mem(memset_notzero_data.as_mut_ptr() as *mut _, data_sz, 0, &mut ptr);
    ternary_err(bpf_dynptr_memset(&mut ptr, 0, data_sz, DYNPTR_MEMSET_VAL));
    ternary_err(bpf_memcmp(expected.as_ptr(), memset_notzero_data.as_ptr(), data_sz));

    0
}

#[no_mangle]
pub static mut memset_zero_offset_data: [i8; 28] = *b"data to be zeroed partially\0".as_ptr().cast::<[i8; 28]>();

// SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_memset_zero_offset(_ctx: *mut core::ffi::c_void) -> i32 {
    let expected: [i8; 28] = *b"data to \0\0\0\0eroed partially\0".as_ptr().cast::<[i8; 28]>();
    let data_sz: __u32 = core::mem::size_of_val(&memset_zero_offset_data) as __u32;
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();

    err = bpf_dynptr_from_mem(memset_zero_offset_data.as_mut_ptr() as *mut _, data_sz, 0, &mut ptr);
    ternary_err(bpf_dynptr_memset(&mut ptr, 8, 4, 0));
    ternary_err(bpf_memcmp(expected.as_ptr(), memset_zero_offset_data.as_ptr(), data_sz));

    0
}

#[no_mangle]
pub static mut memset_zero_adjusted_data: [i8; 28] = *b"data to be zeroed partially\0".as_ptr().cast::<[i8; 28]>();

// SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_memset_zero_adjusted(_ctx: *mut core::ffi::c_void) -> i32 {
    let expected: [i8; 28] = *b"data\0\0\0\0be zeroed partially\0".as_ptr().cast::<[i8; 28]>();
    let data_sz: __u32 = core::mem::size_of_val(&memset_zero_adjusted_data) as __u32;
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();

    err = bpf_dynptr_from_mem(memset_zero_adjusted_data.as_mut_ptr() as *mut _, data_sz, 0, &mut ptr);
    ternary_err(bpf_dynptr_adjust(&mut ptr, 4, 8));
    ternary_err(bpf_dynptr_memset(&mut ptr, 0, bpf_dynptr_size(&ptr), 0));
    ternary_err(bpf_memcmp(expected.as_ptr(), memset_zero_adjusted_data.as_ptr(), data_sz));

    0
}

#[no_mangle]
pub static mut memset_overflow_data: [i8; 21] = *b"memset overflow data\0".as_ptr().cast::<[i8; 21]>();

// SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_memset_overflow(_ctx: *mut core::ffi::c_void) -> i32 {
    let data_sz: __u32 = core::mem::size_of_val(&memset_overflow_data) as __u32;
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let ret: i32;

    err = bpf_dynptr_from_mem(memset_overflow_data.as_mut_ptr() as *mut _, data_sz, 0, &mut ptr);
    ret = bpf_dynptr_memset(&mut ptr, 0, data_sz + 1, 0);
    if ret != -E2BIG {
        err = 1;
    }

    0
}

// SEC("?tp/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_memset_overflow_offset(_ctx: *mut core::ffi::c_void) -> i32 {
    let data_sz: __u32 = core::mem::size_of_val(&memset_overflow_data) as __u32;
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let ret: i32;

    err = bpf_dynptr_from_mem(memset_overflow_data.as_mut_ptr() as *mut _, data_sz, 0, &mut ptr);
    ret = bpf_dynptr_memset(&mut ptr, 1, data_sz, 0);
    if ret != -E2BIG {
        err = 1;
    }

    0
}

// SEC("?cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_memset_readonly(skb: *mut __sk_buff) -> i32 {
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let ret: i32;

    err = bpf_dynptr_from_skb(skb as *mut _, 0, &mut ptr);
    ret = bpf_dynptr_memset(&mut ptr, 0, bpf_dynptr_size(&ptr), 0);
    if ret != -EINVAL {
        err = 1;
    }

    0
}

#[inline(always)]
fn min_t_u32(x: u32, y: u32) -> u32 {
    if x < y { x } else { y }
}

// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn test_dynptr_memset_xdp_chunks(xdp: *mut xdp_md) -> i32 {
    let mut data_sz: u32;
    let mut chunk_sz: u32;
    let mut offset: u32 = 0;
    const max_chunks: i32 = 200;
    let mut ptr_xdp = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut expected_buf = [0i8; 32];
    let mut buf = [0i8; 32];
    let mut i: i32;

    core::ptr::write_bytes(expected_buf.as_mut_ptr(), DYNPTR_MEMSET_VAL as u8, expected_buf.len());

    bpf_dynptr_from_xdp(xdp, 0, &mut ptr_xdp);
    data_sz = bpf_dynptr_size(&ptr_xdp);

    err = bpf_dynptr_memset(&mut ptr_xdp, 0, data_sz, DYNPTR_MEMSET_VAL);
    if err != 0 {
        if data_sz >= PAGE_SIZE_64K {
            err = 0;
        }
        return XDP_DROP;
    }

    i = 0;
    while i < max_chunks {
        offset = (i as u32) * core::mem::size_of_val(&buf) as u32;
        if offset >= data_sz {
            return XDP_DROP;
        }
        chunk_sz = min_t_u32(core::mem::size_of_val(&buf) as u32, data_sz - offset);
        err = bpf_dynptr_read(buf.as_mut_ptr() as *mut _, chunk_sz, &ptr_xdp, offset, 0);
        if err != 0 {
            return XDP_DROP;
        }
        err = bpf_memcmp(buf.as_ptr(), expected_buf.as_ptr(), core::mem::size_of_val(&buf) as u32);
        if err != 0 {
            return XDP_DROP;
        }
        i += 1;
    }

    XDP_DROP
}

#[no_mangle]
pub static mut user_ptr: *mut core::ffi::c_void = core::ptr::null_mut();

/* Contains the copy of the data pointed by user_ptr.
 * Size 384 to make it not fit into a single kernel chunk when copying
 * but less than the maximum bpf stack size (512).
 */
#[no_mangle]
pub static mut expected_str: [i8; 384] = [0; 384];

#[no_mangle]
pub static mut test_len: [__u32; 7] = [0, 0, 1, 2, 255, 256, 257];

/* Returns the offset just before the end of the maximum sized xdp fragment.
 * Any write larger than 32 bytes will be split between 2 fragments.
 */
#[no_mangle]
pub unsafe extern "C" fn xdp_near_frag_end_offset() -> __u32 {
    const headroom: __u32 = 256;
    let max_frag_size: __u32 = __PAGE_SIZE - headroom - core::mem::size_of::<skb_shared_info>() as __u32;

    max_frag_size - 32
}

/* Use __always_inline on test_dynptr_probe[_str][_xdp]() and callbacks
 * of type bpf_read_dynptr_fn_t to prevent compiler from generating
 * indirect calls that make program fail to load with "unknown opcode" error.
 */
#[inline(always)]
unsafe fn test_dynptr_probe(ptr: *mut core::ffi::c_void, bpf_read_dynptr_fn: bpf_read_dynptr_fn_t) {
    let mut buf = [0i8; 384];
    let mut ptr_buf = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut i: i32;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return;
    }

    err = bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, buf.len() as __u64, 0, &mut ptr_buf);

    i = 0;
    while (i as usize) < test_len.len() {
        let len: __u32 = test_len[i as usize];

        ternary_err(bpf_read_dynptr_fn(&ptr_buf, 0, test_len[i as usize] as u64, ptr));
        if len > buf.len() as __u32 {
            break;
        }
        ternary_err(bpf_dynptr_read(buf.as_mut_ptr() as *mut _, len, &ptr_buf, 0, 0));

        if err != 0 || bpf_memcmp(expected_str.as_ptr(), buf.as_ptr(), len) != 0 {
            err = 1;
        }

        core::ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len());
        ternary_err(bpf_dynptr_write(&mut ptr_buf, 0, buf.as_ptr() as *const _, len, 0));
        i += 1;
    }
    bpf_ringbuf_discard_dynptr(&mut ptr_buf, 0);
}

#[inline(always)]
unsafe fn test_dynptr_probe_str(ptr: *mut core::ffi::c_void, bpf_read_dynptr_fn: bpf_read_dynptr_fn_t) {
    let mut buf = [0i8; 384];
    let mut ptr_buf = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut cnt: __u32;
    let mut i: __u32;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return;
    }

    bpf_ringbuf_reserve_dynptr(&mut ringbuf as *mut _ as *mut _, buf.len() as __u64, 0, &mut ptr_buf);

    i = 0;
    while (i as usize) < test_len.len() {
        let len: __u32 = test_len[i as usize];

        cnt = bpf_read_dynptr_fn(&ptr_buf, 0, len as u64, ptr) as __u32;
        if cnt != len {
            err = 1;
        }

        if len > buf.len() as __u32 {
            i += 1;
            continue;
        }
        ternary_err(bpf_dynptr_read(buf.as_mut_ptr() as *mut _, len, &ptr_buf, 0, 0));
        if len == 0 {
            i += 1;
            continue;
        }
        if err != 0
            || bpf_memcmp(expected_str.as_ptr(), buf.as_ptr(), len - 1) != 0
            || buf[(len - 1) as usize] != 0
        {
            err = 1;
        }
        i += 1;
    }
    bpf_ringbuf_discard_dynptr(&mut ptr_buf, 0);
}

#[inline(always)]
unsafe fn test_dynptr_probe_xdp(
    xdp: *mut xdp_md,
    ptr: *mut core::ffi::c_void,
    bpf_read_dynptr_fn: bpf_read_dynptr_fn_t,
) {
    let mut ptr_xdp = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut buf = [0i8; 384];
    let off: __u32;
    let mut i: __u32;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return;
    }

    off = xdp_near_frag_end_offset();
    err = bpf_dynptr_from_xdp(xdp, 0, &mut ptr_xdp);

    i = 0;
    while (i as usize) < test_len.len() {
        let len: __u32 = test_len[i as usize];

        ternary_err(bpf_read_dynptr_fn(&ptr_xdp, off as u64, len as u64, ptr));
        if len > buf.len() as __u32 {
            i += 1;
            continue;
        }
        ternary_err(bpf_dynptr_read(buf.as_mut_ptr() as *mut _, len, &ptr_xdp, off, 0));
        if err != 0 || bpf_memcmp(expected_str.as_ptr(), buf.as_ptr(), len) != 0 {
            err = 1;
        }
        core::ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len());
        ternary_err(bpf_dynptr_write(&mut ptr_xdp, off, buf.as_ptr() as *const _, len, 0));
        i += 1;
    }
}

#[inline(always)]
unsafe fn test_dynptr_probe_str_xdp(
    xdp: *mut xdp_md,
    ptr: *mut core::ffi::c_void,
    bpf_read_dynptr_fn: bpf_read_dynptr_fn_t,
) {
    let mut ptr_xdp = core::mem::MaybeUninit::<bpf_dynptr>::uninit().assume_init();
    let mut buf = [0i8; 384];
    let mut cnt: __u32;
    let off: __u32;
    let mut i: __u32;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return;
    }

    off = xdp_near_frag_end_offset();
    err = bpf_dynptr_from_xdp(xdp, 0, &mut ptr_xdp);
    if err != 0 {
        return;
    }

    i = 0;
    while (i as usize) < test_len.len() {
        let len: __u32 = test_len[i as usize];

        cnt = bpf_read_dynptr_fn(&ptr_xdp, off as u64, len as u64, ptr) as __u32;
        if cnt != len {
            err = 1;
        }

        if len > buf.len() as __u32 {
            i += 1;
            continue;
        }
        ternary_err(bpf_dynptr_read(buf.as_mut_ptr() as *mut _, len, &ptr_xdp, off, 0));

        if len == 0 {
            i += 1;
            continue;
        }
        if err != 0
            || bpf_memcmp(expected_str.as_ptr(), buf.as_ptr(), len - 1) != 0
            || buf[(len - 1) as usize] != 0
        {
            err = 1;
        }

        core::ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len());
        ternary_err(bpf_dynptr_write(&mut ptr_xdp, off, buf.as_ptr() as *const _, len, 0));
        i += 1;
    }
}

// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn test_probe_read_user_dynptr(xdp: *mut xdp_md) -> i32 {
    test_dynptr_probe(user_ptr, bpf_probe_read_user_dynptr);
    if err == 0 {
        test_dynptr_probe_xdp(xdp, user_ptr, bpf_probe_read_user_dynptr);
    }
    XDP_PASS
}

// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn test_probe_read_kernel_dynptr(xdp: *mut xdp_md) -> i32 {
    test_dynptr_probe(expected_str.as_mut_ptr() as *mut _, bpf_probe_read_kernel_dynptr);
    if err == 0 {
        test_dynptr_probe_xdp(xdp, expected_str.as_mut_ptr() as *mut _, bpf_probe_read_kernel_dynptr);
    }
    XDP_PASS
}

// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn test_probe_read_user_str_dynptr(xdp: *mut xdp_md) -> i32 {
    test_dynptr_probe_str(user_ptr, bpf_probe_read_user_str_dynptr);
    if err == 0 {
        test_dynptr_probe_str_xdp(xdp, user_ptr, bpf_probe_read_user_str_dynptr);
    }
    XDP_PASS
}

// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn test_probe_read_kernel_str_dynptr(xdp: *mut xdp_md) -> i32 {
    test_dynptr_probe_str(expected_str.as_mut_ptr() as *mut _, bpf_probe_read_kernel_str_dynptr);
    if err == 0 {
        test_dynptr_probe_str_xdp(xdp, expected_str.as_mut_ptr() as *mut _, bpf_probe_read_kernel_str_dynptr);
    }
    XDP_PASS
}

// SEC("fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_copy_from_user_dynptr(_ctx: *mut core::ffi::c_void) -> i32 {
    test_dynptr_probe(user_ptr, bpf_copy_from_user_dynptr);
    0
}

// SEC("fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_copy_from_user_str_dynptr(_ctx: *mut core::ffi::c_void) -> i32 {
    test_dynptr_probe_str(user_ptr, bpf_copy_from_user_str_dynptr);
    0
}

static unsafe extern "C" fn bpf_copy_data_from_user_task(
    dptr: *const bpf_dynptr,
    off: u64,
    size: u64,
    unsafe_ptr: *const core::ffi::c_void,
) -> i32 {
    let task: *mut task_struct = bpf_get_current_task_btf();

    bpf_copy_from_user_task_dynptr(dptr, off, size, unsafe_ptr, task)
}

static unsafe extern "C" fn bpf_copy_data_from_user_task_str(
    dptr: *const bpf_dynptr,
    off: u64,
    size: u64,
    unsafe_ptr: *const core::ffi::c_void,
) -> i32 {
    let task: *mut task_struct = bpf_get_current_task_btf();

    bpf_copy_from_user_task_str_dynptr(dptr, off, size, unsafe_ptr, task)
}

// SEC("fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_copy_from_user_task_dynptr(_ctx: *mut core::ffi::c_void) -> i32 {
    test_dynptr_probe(user_ptr, bpf_copy_data_from_user_task);
    0
}

// SEC("fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_copy_from_user_task_str_dynptr(_ctx: *mut core::ffi::c_void) -> i32 {
    test_dynptr_probe_str(user_ptr, bpf_copy_data_from_user_task_str);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
