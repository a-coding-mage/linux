// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook  */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type bool_ = bool;
type __u8 = u8;
type __u32 = u32;
type size_t = usize;
type pthread_t = c_ulong;
type rlim_t = c_ulong;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

const BPF_F_NO_PREALLOC: __u32 = 1;
const BPF_NOEXIST: u64 = 1;
const BPF_EXIST: u64 = 2;
const BPF_F_LOCK: u64 = 4;
const BPF_MAP_TYPE_SK_STORAGE: c_int = 24;

const BTF_MAGIC: __u32 = 0xeB9F;
const BTF_VERSION: __u32 = 1;
const BTF_KIND_STRUCT: __u32 = 4;
const BTF_INT_SIGNED: __u32 = 1;

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SIGALRM: c_int = 14;
const SIGTERM: c_int = 15;
const SIGINT: c_int = 2;
const RLIMIT_NOFILE: c_int = 7;
const ENOMEM: c_int = 12;
const EAGAIN: c_int = 11;
const ENOENT: c_int = 2;
const EEXIST: c_int = 17;
const EINVAL: c_int = 22;

const SIG_DFL: sighandler_t = None;

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_map_create_opts {
    sz: size_t,
    btf_fd: c_int,
    btf_key_type_id: __u32,
    btf_value_type_id: __u32,
    btf_vmlinux_value_type_id: __u32,
    inner_map_fd: c_int,
    map_flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct btf_header {
    magic: __u32,
    version: __u8,
    flags: __u8,
    hdr_len: __u32,
    type_off: __u32,
    type_len: __u32,
    str_off: __u32,
    str_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct rlimit {
    rlim_cur: rlim_t,
    rlim_max: rlim_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct storage_value {
    cnt: c_int,
    lock: c_int,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn __errno_location() -> *mut c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn usleep(usec: c_uint) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn alarm(seconds: c_uint) -> c_uint;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn bpf_btf_load(raw_btf: *const c_void, raw_btf_size: size_t, opts: *const c_void) -> c_int;
    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_lookup_elem_flags(fd: c_int, key: *const c_void, value: *mut c_void, flags: u64) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;

    fn CHECK(condition: bool_, name: *const c_char, format: *const c_char, ...);
}

static mut map_opts: bpf_map_create_opts = bpf_map_create_opts {
    sz: mem::size_of::<bpf_map_create_opts>(),
    btf_fd: -1,
    btf_key_type_id: 1,
    btf_value_type_id: 3,
    btf_vmlinux_value_type_id: 0,
    inner_map_fd: 0,
    map_flags: BPF_F_NO_PREALLOC,
};

static mut nr_sk_threads_done: c_uint = 0;
static mut nr_sk_threads_err: c_uint = 0;
static mut nr_sk_per_thread: c_uint = 4096;
static mut nr_sk_threads: c_uint = 4;
static mut sk_storage_map: c_int = -1;
static mut stop: c_uint = 0;
static mut runtime_s: c_int = 5;

#[inline]
unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

#[inline]
unsafe fn ERR_PTR(err: c_int) -> *mut c_void {
    err as isize as *mut c_void
}

#[inline]
unsafe fn IS_ERR(ptr: *mut c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

#[inline]
unsafe fn PTR_ERR(ptr: *mut c_void) -> c_int {
    ptr as isize as c_int
}

const fn BTF_INFO_ENC(kind: __u32, kind_flag: __u32, vlen: __u32) -> __u32 {
    (kind << 24) | (kind_flag << 31) | vlen
}

const fn BTF_TYPE_ENC(name: __u32, info: __u32, size_or_type: __u32) -> __u32 {
    name | (info << 16) | (size_or_type << 0)
}

const fn BTF_MEMBER_ENC(name: __u32, type_: __u32, bits_offset: __u32) -> __u32 {
    name | (type_ << 16) | bits_offset
}

const fn BTF_TYPE_INT_ENC(name: __u32, encoding: __u32, offset: __u32, bits: __u32, size: __u32) -> __u32 {
    let info = BTF_INFO_ENC(1, 0, 0);
    let int_data = (encoding << 24) | (offset << 16) | bits;
    name | (info << 16) | size ^ int_data
}

unsafe fn is_stopped() -> bool {
    unsafe { ptr::read_volatile(ptr::addr_of!(stop)) != 0 }
}

unsafe fn threads_err() -> c_uint {
    unsafe { ptr::read_volatile(ptr::addr_of!(nr_sk_threads_err)) }
}

unsafe fn notify_thread_err() {
    unsafe {
        nr_sk_threads_err = nr_sk_threads_err.wrapping_add(1);
    }
}

unsafe fn wait_for_threads_err() -> bool {
    unsafe {
        while !is_stopped() && threads_err() == 0 {
            usleep(500);
        }

        !is_stopped()
    }
}

unsafe fn threads_done() -> c_uint {
    unsafe { ptr::read_volatile(ptr::addr_of!(nr_sk_threads_done)) }
}

unsafe fn notify_thread_done() {
    unsafe {
        nr_sk_threads_done = nr_sk_threads_done.wrapping_add(1);
    }
}

unsafe fn notify_thread_redo() {
    unsafe {
        nr_sk_threads_done = nr_sk_threads_done.wrapping_sub(1);
    }
}

unsafe fn wait_for_threads_done() -> bool {
    unsafe {
        while threads_done() != nr_sk_threads && !is_stopped() && threads_err() == 0 {
            usleep(50);
        }

        !is_stopped() && threads_err() == 0
    }
}

unsafe fn wait_for_threads_redo() -> bool {
    unsafe {
        while threads_done() != 0 && !is_stopped() && threads_err() == 0 {
            usleep(50);
        }

        !is_stopped() && threads_err() == 0
    }
}

unsafe fn wait_for_map() -> bool {
    unsafe {
        while ptr::read_volatile(ptr::addr_of!(sk_storage_map)) == -1 && !is_stopped() {
            usleep(50);
        }

        !is_stopped()
    }
}

unsafe fn wait_for_map_close() -> bool {
    unsafe {
        while ptr::read_volatile(ptr::addr_of!(sk_storage_map)) != -1 && !is_stopped() {}

        !is_stopped()
    }
}

unsafe fn load_btf() -> c_int {
    let btf_str_sec: [c_char; 26] = [
        0, b'b' as c_char, b'p' as c_char, b'f' as c_char, b'_' as c_char,
        b's' as c_char, b'p' as c_char, b'i' as c_char, b'n' as c_char,
        b'_' as c_char, b'l' as c_char, b'o' as c_char, b'c' as c_char,
        b'k' as c_char, 0, b'v' as c_char, b'a' as c_char, b'l' as c_char,
        0, b'c' as c_char, b'n' as c_char, b't' as c_char, 0,
        b'l' as c_char, 0, 0,
    ];
    let btf_raw_types: [__u32; 6] = [
        /* int */
        BTF_TYPE_INT_ENC(0, BTF_INT_SIGNED, 0, 32, 4), /* [1] */
        /* struct bpf_spin_lock */ /* [2] */
        BTF_TYPE_ENC(1, BTF_INFO_ENC(BTF_KIND_STRUCT, 0, 1), 4),
        BTF_MEMBER_ENC(15, 1, 0), /* int val; */
        /* struct val */ /* [3] */
        BTF_TYPE_ENC(15, BTF_INFO_ENC(BTF_KIND_STRUCT, 0, 2), 8),
        BTF_MEMBER_ENC(19, 1, 0),  /* int cnt; */
        BTF_MEMBER_ENC(23, 2, 32), /* struct bpf_spin_lock l; */
    ];
    let btf_hdr = btf_header {
        magic: BTF_MAGIC,
        version: BTF_VERSION as __u8,
        flags: 0,
        hdr_len: mem::size_of::<btf_header>() as __u32,
        type_off: 0,
        type_len: mem::size_of_val(&btf_raw_types) as __u32,
        str_off: mem::size_of_val(&btf_raw_types) as __u32,
        str_len: mem::size_of_val(&btf_str_sec) as __u32,
    };
    let mut raw_btf =
        [0u8; mem::size_of::<btf_header>() + mem::size_of::<[__u32; 6]>() + 26];

    unsafe {
        memcpy(raw_btf.as_mut_ptr() as *mut c_void, &btf_hdr as *const _ as *const c_void, mem::size_of_val(&btf_hdr));
        memcpy(
            raw_btf.as_mut_ptr().add(mem::size_of_val(&btf_hdr)) as *mut c_void,
            btf_raw_types.as_ptr() as *const c_void,
            mem::size_of_val(&btf_raw_types),
        );
        memcpy(
            raw_btf.as_mut_ptr().add(mem::size_of_val(&btf_hdr) + mem::size_of_val(&btf_raw_types)) as *mut c_void,
            btf_str_sec.as_ptr() as *const c_void,
            mem::size_of_val(&btf_str_sec),
        );

        bpf_btf_load(raw_btf.as_ptr() as *const c_void, mem::size_of_val(&raw_btf), ptr::null())
    }
}

unsafe fn create_sk_storage_map() -> c_int {
    unsafe {
        let btf_fd = load_btf();
        CHECK(
            btf_fd == -1,
            c"bpf_load_btf".as_ptr(),
            c"btf_fd:%d errno:%d\n".as_ptr(),
            btf_fd,
            errno(),
        );
        map_opts.btf_fd = btf_fd;

        let map_fd = bpf_map_create(BPF_MAP_TYPE_SK_STORAGE, c"sk_storage_map".as_ptr(), 4, 8, 0, ptr::addr_of!(map_opts));
        map_opts.btf_fd = -1;
        close(btf_fd);
        CHECK(map_fd == -1, c"bpf_map_create()".as_ptr(), c"errno:%d\n".as_ptr(), errno());

        map_fd
    }
}

unsafe extern "C" fn insert_close_thread(_arg: *mut c_void) -> *mut c_void {
    unsafe {
        let value = storage_value { cnt: 0xeB9F, lock: 0 };
        let mut err: c_int;
        let sk_fds = malloc(mem::size_of::<c_int>() * nr_sk_per_thread as usize) as *mut c_int;
        if sk_fds.is_null() {
            notify_thread_err();
            return ERR_PTR(-ENOMEM);
        }

        for i in 0..nr_sk_per_thread as isize {
            *sk_fds.offset(i) = -1;
        }

        while !is_stopped() {
            if !wait_for_map() {
                for i in 0..nr_sk_per_thread as isize {
                    close(*sk_fds.offset(i));
                    *sk_fds.offset(i) = -1;
                }
                notify_thread_redo();
                continue;
            }

            let map_fd = ptr::read_volatile(ptr::addr_of!(sk_storage_map));
            let mut i: c_uint = 0;
            while i < nr_sk_per_thread && !is_stopped() {
                *sk_fds.add(i as usize) = socket(AF_INET6, SOCK_STREAM, 0);
                if *sk_fds.add(i as usize) == -1 {
                    err = -errno();
                    fprintf(stderr, c"socket(): errno:%d\n".as_ptr(), errno());
                    while i < nr_sk_per_thread && *sk_fds.add(i as usize) != -1 {
                        close(*sk_fds.add(i as usize));
                        i += 1;
                    }
                    free(sk_fds as *mut c_void);
                    notify_thread_err();
                    return ERR_PTR(err);
                }
                err = bpf_map_update_elem(
                    map_fd,
                    sk_fds.add(i as usize) as *const c_void,
                    &value as *const _ as *const c_void,
                    BPF_NOEXIST,
                );
                if err != 0 {
                    err = -errno();
                    fprintf(stderr, c"bpf_map_update_elem(): errno:%d\n".as_ptr(), errno());
                    while i < nr_sk_per_thread && *sk_fds.add(i as usize) != -1 {
                        close(*sk_fds.add(i as usize));
                        i += 1;
                    }
                    free(sk_fds as *mut c_void);
                    notify_thread_err();
                    return ERR_PTR(err);
                }
                i += 1;
            }

            notify_thread_done();
            wait_for_map_close();

            for i in 0..nr_sk_per_thread as isize {
                close(*sk_fds.offset(i));
                *sk_fds.offset(i) = -1;
            }

            notify_thread_redo();
        }

        free(sk_fds as *mut c_void);
        ptr::null_mut()
    }
}

unsafe fn do_sk_storage_map_stress_free() -> c_int {
    unsafe {
        let mut map_fd: c_int = -1;
        let mut err: c_int = 0;
        let mut nr_threads_created: c_int = 0;
        let mut thread_ret: *mut c_void = ptr::null_mut();

        let sk_thread_ids = malloc(mem::size_of::<pthread_t>() * nr_sk_threads as usize) as *mut pthread_t;
        if sk_thread_ids.is_null() {
            fprintf(stderr, c"malloc(sk_threads): NULL\n".as_ptr());
            return -ENOMEM;
        }

        for i in 0..nr_sk_threads as isize {
            err = pthread_create(sk_thread_ids.offset(i), ptr::null(), insert_close_thread, ptr::null_mut());
            if err != 0 {
                err = -errno();
                break;
            }
            nr_threads_created += 1;
        }

        if err == 0 {
            while !is_stopped() {
                map_fd = create_sk_storage_map();
                ptr::write_volatile(ptr::addr_of_mut!(sk_storage_map), map_fd);

                if !wait_for_threads_done() {
                    break;
                }

                ptr::write_volatile(ptr::addr_of_mut!(sk_storage_map), -1);
                close(map_fd);
                map_fd = -1;

                if !wait_for_threads_redo() {
                    break;
                }
            }
        }

        ptr::write_volatile(ptr::addr_of_mut!(stop), 1);
        for i in 0..nr_threads_created as isize {
            pthread_join(*sk_thread_ids.offset(i), &mut thread_ret);
            if IS_ERR(thread_ret) && err == 0 {
                err = PTR_ERR(thread_ret);
                fprintf(stderr, c"threads#%u: err:%d\n".as_ptr(), i as c_uint, err);
            }
        }
        free(sk_thread_ids as *mut c_void);

        if map_fd != -1 {
            close(map_fd);
        }

        err
    }
}

unsafe extern "C" fn update_thread(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let value = storage_value { cnt: 0xeB9F, lock: 0 };
        let map_fd = ptr::read_volatile(ptr::addr_of!(sk_storage_map));
        let sk_fd = *(arg as *mut c_int);
        let mut err: c_int = 0; /* Suppress compiler false alarm */

        while !is_stopped() {
            err = bpf_map_update_elem(map_fd, &sk_fd as *const _ as *const c_void, &value as *const _ as *const c_void, 0);
            if err != 0 && errno() != EAGAIN {
                err = -errno();
                fprintf(stderr, c"bpf_map_update_elem: %d %d\n".as_ptr(), err, errno());
                break;
            }
        }

        if !is_stopped() {
            notify_thread_err();
            return ERR_PTR(err);
        }

        ptr::null_mut()
    }
}

unsafe extern "C" fn delete_thread(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let map_fd = ptr::read_volatile(ptr::addr_of!(sk_storage_map));
        let sk_fd = *(arg as *mut c_int);
        let mut err: c_int = 0; /* Suppress compiler false alarm */

        while !is_stopped() {
            err = bpf_map_delete_elem(map_fd, &sk_fd as *const _ as *const c_void);
            if err != 0 && errno() != ENOENT {
                err = -errno();
                fprintf(stderr, c"bpf_map_delete_elem: %d %d\n".as_ptr(), err, errno());
                break;
            }
        }

        if !is_stopped() {
            notify_thread_err();
            return ERR_PTR(err);
        }

        ptr::null_mut()
    }
}

unsafe fn do_sk_storage_map_stress_change() -> c_int {
    unsafe {
        let mut sk_fd: c_int = -1;
        let mut map_fd: c_int = -1;
        let mut err: c_int = 0;
        let mut nr_threads_created: c_int = 0;
        let mut thread_ret: *mut c_void = ptr::null_mut();

        let sk_thread_ids = malloc(mem::size_of::<pthread_t>() * nr_sk_threads as usize) as *mut pthread_t;
        if sk_thread_ids.is_null() {
            fprintf(stderr, c"malloc(sk_threads): NULL\n".as_ptr());
            return -ENOMEM;
        }

        sk_fd = socket(AF_INET6, SOCK_STREAM, 0);
        if sk_fd == -1 {
            err = -errno();
        } else {
            map_fd = create_sk_storage_map();
            ptr::write_volatile(ptr::addr_of_mut!(sk_storage_map), map_fd);

            for i in 0..nr_sk_threads as isize {
                if (i & 0x1) != 0 {
                    err = pthread_create(sk_thread_ids.offset(i), ptr::null(), update_thread, &mut sk_fd as *mut _ as *mut c_void);
                } else {
                    err = pthread_create(sk_thread_ids.offset(i), ptr::null(), delete_thread, &mut sk_fd as *mut _ as *mut c_void);
                }
                if err != 0 {
                    err = -errno();
                    break;
                }
                nr_threads_created += 1;
            }

            if err == 0 {
                wait_for_threads_err();
            }
        }

        ptr::write_volatile(ptr::addr_of_mut!(stop), 1);
        for i in 0..nr_threads_created as isize {
            pthread_join(*sk_thread_ids.offset(i), &mut thread_ret);
            if IS_ERR(thread_ret) && err == 0 {
                err = PTR_ERR(thread_ret);
                fprintf(stderr, c"threads#%u: err:%d\n".as_ptr(), i as c_uint, err);
            }
        }
        free(sk_thread_ids as *mut c_void);

        if sk_fd != -1 {
            close(sk_fd);
        }
        close(map_fd);

        err
    }
}

unsafe extern "C" fn stop_handler(signum: c_int) {
    unsafe {
        if signum != SIGALRM {
            printf(c"stopping...\n".as_ptr());
        }
        ptr::write_volatile(ptr::addr_of_mut!(stop), 1);
    }
}

const BPF_SK_STORAGE_MAP_TEST_NR_THREADS: *const c_char = c"BPF_SK_STORAGE_MAP_TEST_NR_THREADS".as_ptr();
const BPF_SK_STORAGE_MAP_TEST_SK_PER_THREAD: *const c_char = c"BPF_SK_STORAGE_MAP_TEST_SK_PER_THREAD".as_ptr();
const BPF_SK_STORAGE_MAP_TEST_RUNTIME_S: *const c_char = c"BPF_SK_STORAGE_MAP_TEST_RUNTIME_S".as_ptr();
const BPF_SK_STORAGE_MAP_TEST_NAME: *const c_char = c"BPF_SK_STORAGE_MAP_TEST_NAME".as_ptr();

unsafe fn test_sk_storage_map_stress_free() {
    unsafe {
        let mut rlim_old: rlimit = mem::zeroed();
        let mut rlim_new: rlimit = mem::zeroed();
        let mut err: c_int;

        getrlimit(RLIMIT_NOFILE, &mut rlim_old);

        signal(SIGTERM, Some(stop_handler));
        signal(SIGINT, Some(stop_handler));
        if runtime_s > 0 {
            signal(SIGALRM, Some(stop_handler));
            alarm(runtime_s as c_uint);
        }

        if rlim_old.rlim_cur < (nr_sk_threads * nr_sk_per_thread) as rlim_t {
            rlim_new.rlim_cur = (nr_sk_threads * nr_sk_per_thread + 128) as rlim_t;
            rlim_new.rlim_max = rlim_new.rlim_cur + 128;
            err = setrlimit(RLIMIT_NOFILE, &rlim_new);
            CHECK(
                err != 0,
                c"setrlimit(RLIMIT_NOFILE)".as_ptr(),
                c"rlim_new:%lu errno:%d".as_ptr(),
                rlim_new.rlim_cur as c_ulong,
                errno(),
            );
        }

        err = do_sk_storage_map_stress_free();

        signal(SIGTERM, SIG_DFL);
        signal(SIGINT, SIG_DFL);
        if runtime_s > 0 {
            signal(SIGALRM, SIG_DFL);
            alarm(0);
        }

        if rlim_new.rlim_cur != 0 {
            setrlimit(RLIMIT_NOFILE, &rlim_old);
        }

        CHECK(err != 0, c"test_sk_storage_map_stress_free".as_ptr(), c"err:%d\n".as_ptr(), err);
    }
}

unsafe fn test_sk_storage_map_stress_change() {
    unsafe {
        let mut err: c_int;

        signal(SIGTERM, Some(stop_handler));
        signal(SIGINT, Some(stop_handler));
        if runtime_s > 0 {
            signal(SIGALRM, Some(stop_handler));
            alarm(runtime_s as c_uint);
        }

        err = do_sk_storage_map_stress_change();

        signal(SIGTERM, SIG_DFL);
        signal(SIGINT, SIG_DFL);
        if runtime_s > 0 {
            signal(SIGALRM, SIG_DFL);
            alarm(0);
        }

        CHECK(err != 0, c"test_sk_storage_map_stress_change".as_ptr(), c"err:%d\n".as_ptr(), err);
    }
}

unsafe fn test_sk_storage_map_basic() {
    unsafe {
        let mut value = storage_value { cnt: 0xeB9f, lock: 1 };
        let mut lookup_value: storage_value = mem::zeroed();
        let mut bad_xattr: bpf_map_create_opts = mem::zeroed();
        let mut err: c_int;

        let btf_fd = load_btf();
        CHECK(btf_fd == -1, c"bpf_load_btf".as_ptr(), c"btf_fd:%d errno:%d\n".as_ptr(), btf_fd, errno());
        map_opts.btf_fd = btf_fd;

        let sk_fd = socket(AF_INET6, SOCK_STREAM, 0);
        CHECK(sk_fd == -1, c"socket()".as_ptr(), c"sk_fd:%d errno:%d\n".as_ptr(), sk_fd, errno());

        let map_fd = bpf_map_create(BPF_MAP_TYPE_SK_STORAGE, c"sk_storage_map".as_ptr(), 4, 8, 0, ptr::addr_of!(map_opts));
        CHECK(map_fd == -1, c"bpf_map_create(good_xattr)".as_ptr(), c"map_fd:%d errno:%d\n".as_ptr(), map_fd, errno());

        /* Add new elem */
        memcpy(&mut lookup_value as *mut _ as *mut c_void, &value as *const _ as *const c_void, mem::size_of_val(&value));
        err = bpf_map_update_elem(map_fd, &sk_fd as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST | BPF_F_LOCK);
        CHECK(err != 0, c"bpf_map_update_elem(BPF_NOEXIST|BPF_F_LOCK)".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());
        err = bpf_map_lookup_elem_flags(map_fd, &sk_fd as *const _ as *const c_void, &mut lookup_value as *mut _ as *mut c_void, BPF_F_LOCK);
        CHECK(err != 0 || lookup_value.lock != 0 || lookup_value.cnt != value.cnt, c"bpf_map_lookup_elem_flags(BPF_F_LOCK)".as_ptr(), c"err:%d errno:%d lock:%x cnt:%x(%x)\n".as_ptr(), err, errno(), lookup_value.lock, lookup_value.cnt, value.cnt);

        /* Bump the cnt and update with BPF_EXIST | BPF_F_LOCK */
        value.cnt += 1;
        value.lock = 2;
        err = bpf_map_update_elem(map_fd, &sk_fd as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_EXIST | BPF_F_LOCK);
        CHECK(err != 0, c"bpf_map_update_elem(BPF_EXIST|BPF_F_LOCK)".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());
        err = bpf_map_lookup_elem_flags(map_fd, &sk_fd as *const _ as *const c_void, &mut lookup_value as *mut _ as *mut c_void, BPF_F_LOCK);
        CHECK(err != 0 || lookup_value.lock != 0 || lookup_value.cnt != value.cnt, c"bpf_map_lookup_elem_flags(BPF_F_LOCK)".as_ptr(), c"err:%d errno:%d lock:%x cnt:%x(%x)\n".as_ptr(), err, errno(), lookup_value.lock, lookup_value.cnt, value.cnt);

        /* Bump the cnt and update with BPF_EXIST */
        value.cnt += 1;
        value.lock = 2;
        err = bpf_map_update_elem(map_fd, &sk_fd as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_EXIST);
        CHECK(err != 0, c"bpf_map_update_elem(BPF_EXIST)".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());
        err = bpf_map_lookup_elem_flags(map_fd, &sk_fd as *const _ as *const c_void, &mut lookup_value as *mut _ as *mut c_void, BPF_F_LOCK);
        CHECK(err != 0 || lookup_value.lock != 0 || lookup_value.cnt != value.cnt, c"bpf_map_lookup_elem_flags(BPF_F_LOCK)".as_ptr(), c"err:%d errno:%d lock:%x cnt:%x(%x)\n".as_ptr(), err, errno(), lookup_value.lock, lookup_value.cnt, value.cnt);

        /* Update with BPF_NOEXIST */
        value.cnt += 1;
        value.lock = 2;
        err = bpf_map_update_elem(map_fd, &sk_fd as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST | BPF_F_LOCK);
        CHECK(err == 0 || errno() != EEXIST, c"bpf_map_update_elem(BPF_NOEXIST|BPF_F_LOCK)".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());
        err = bpf_map_update_elem(map_fd, &sk_fd as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
        CHECK(err == 0 || errno() != EEXIST, c"bpf_map_update_elem(BPF_NOEXIST)".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());
        value.cnt -= 1;
        err = bpf_map_lookup_elem_flags(map_fd, &sk_fd as *const _ as *const c_void, &mut lookup_value as *mut _ as *mut c_void, BPF_F_LOCK);
        CHECK(err != 0 || lookup_value.lock != 0 || lookup_value.cnt != value.cnt, c"bpf_map_lookup_elem_flags(BPF_F_LOCK)".as_ptr(), c"err:%d errno:%d lock:%x cnt:%x(%x)\n".as_ptr(), err, errno(), lookup_value.lock, lookup_value.cnt, value.cnt);

        /* Bump the cnt again and update with map_flags == 0 */
        value.cnt += 1;
        value.lock = 2;
        err = bpf_map_update_elem(map_fd, &sk_fd as *const _ as *const c_void, &value as *const _ as *const c_void, 0);
        CHECK(err != 0, c"bpf_map_update_elem()".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());
        err = bpf_map_lookup_elem_flags(map_fd, &sk_fd as *const _ as *const c_void, &mut lookup_value as *mut _ as *mut c_void, BPF_F_LOCK);
        CHECK(err != 0 || lookup_value.lock != 0 || lookup_value.cnt != value.cnt, c"bpf_map_lookup_elem_flags(BPF_F_LOCK)".as_ptr(), c"err:%d errno:%d lock:%x cnt:%x(%x)\n".as_ptr(), err, errno(), lookup_value.lock, lookup_value.cnt, value.cnt);

        /* Test delete elem */
        err = bpf_map_delete_elem(map_fd, &sk_fd as *const _ as *const c_void);
        CHECK(err != 0, c"bpf_map_delete_elem()".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());
        err = bpf_map_lookup_elem_flags(map_fd, &sk_fd as *const _ as *const c_void, &mut lookup_value as *mut _ as *mut c_void, BPF_F_LOCK);
        CHECK(err == 0 || errno() != ENOENT, c"bpf_map_lookup_elem_flags(BPF_F_LOCK)".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());
        err = bpf_map_delete_elem(map_fd, &sk_fd as *const _ as *const c_void);
        CHECK(err == 0 || errno() != ENOENT, c"bpf_map_delete_elem()".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());

        memcpy(&mut bad_xattr as *mut _ as *mut c_void, ptr::addr_of!(map_opts) as *const c_void, mem::size_of_val(&map_opts));
        bad_xattr.btf_key_type_id = 0;
        err = bpf_map_create(BPF_MAP_TYPE_SK_STORAGE, c"sk_storage_map".as_ptr(), 4, 8, 0, &bad_xattr);
        CHECK(err == 0 || errno() != EINVAL, c"bpf_map_create(bad_xattr)".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());

        memcpy(&mut bad_xattr as *mut _ as *mut c_void, ptr::addr_of!(map_opts) as *const c_void, mem::size_of_val(&map_opts));
        bad_xattr.btf_key_type_id = 3;
        err = bpf_map_create(BPF_MAP_TYPE_SK_STORAGE, c"sk_storage_map".as_ptr(), 4, 8, 0, &bad_xattr);
        CHECK(err == 0 || errno() != EINVAL, c"bpf_map_create(bad_xattr)".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());

        err = bpf_map_create(BPF_MAP_TYPE_SK_STORAGE, c"sk_storage_map".as_ptr(), 4, 8, 1, ptr::addr_of!(map_opts));
        CHECK(err == 0 || errno() != EINVAL, c"bpf_map_create(bad_xattr)".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());

        memcpy(&mut bad_xattr as *mut _ as *mut c_void, ptr::addr_of!(map_opts) as *const c_void, mem::size_of_val(&map_opts));
        bad_xattr.map_flags = 0;
        err = bpf_map_create(BPF_MAP_TYPE_SK_STORAGE, c"sk_storage_map".as_ptr(), 4, 8, 0, &bad_xattr);
        CHECK(err == 0 || errno() != EINVAL, c"bap_create_map_xattr(bad_xattr)".as_ptr(), c"err:%d errno:%d\n".as_ptr(), err, errno());

        map_opts.btf_fd = -1;
        close(btf_fd);
        close(map_fd);
        close(sk_fd);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_sk_storage_map() {
    unsafe {
        let mut test_ran = false;

        let test_name = getenv(BPF_SK_STORAGE_MAP_TEST_NAME);

        let mut env_opt = getenv(BPF_SK_STORAGE_MAP_TEST_NR_THREADS);
        if !env_opt.is_null() {
            nr_sk_threads = atoi(env_opt) as c_uint;
        }

        env_opt = getenv(BPF_SK_STORAGE_MAP_TEST_SK_PER_THREAD);
        if !env_opt.is_null() {
            nr_sk_per_thread = atoi(env_opt) as c_uint;
        }

        env_opt = getenv(BPF_SK_STORAGE_MAP_TEST_RUNTIME_S);
        if !env_opt.is_null() {
            runtime_s = atoi(env_opt);
        }

        if test_name.is_null() || strcmp(test_name, c"basic".as_ptr()) == 0 {
            test_sk_storage_map_basic();
            test_ran = true;
        }
        if test_name.is_null() || strcmp(test_name, c"stress_free".as_ptr()) == 0 {
            test_sk_storage_map_stress_free();
            test_ran = true;
        }
        if test_name.is_null() || strcmp(test_name, c"stress_change".as_ptr()) == 0 {
            test_sk_storage_map_stress_change();
            test_ran = true;
        }

        if test_ran {
            printf(c"%s:PASS\n".as_ptr(), c"test_sk_storage_map".as_ptr());
        } else {
            CHECK(true, c"Invalid test_name".as_ptr(), c"%s\n".as_ptr(), test_name);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
