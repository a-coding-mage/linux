// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external declarations:
// linux/bpf.h, stdio.h, stdbool.h, unistd.h, errno.h, stdlib.h, string.h,
// pthread.h, bpf/bpf.h, bpf/libbpf.h, test_maps.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type pthread_t = usize;

const BPF_MAP_TYPE_LPM_TRIE: c_int = 11;
const BPF_F_NO_PREALLOC: __u32 = 1;
const BPF_ANY: u64 = 0;

#[repr(C)]
struct bpf_map_create_opts {
    sz: usize,
    map_flags: __u32,
}

#[repr(C)]
struct test_lpm_key {
    prefix: __u32,
    data: __u32,
}

#[repr(C)]
struct get_next_key_ctx {
    key: test_lpm_key,
    start: bool,
    stop: bool,
    map_fd: c_int,
    loop_: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn usleep(usec: c_uint) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;

    fn CHECK(condition: bool, tag: *const c_char, format: *const c_char, ...);
}

static mut __func___test_lpm_trie_map_get_next_key: [c_char; 31] =
    *b"test_lpm_trie_map_get_next_key\0";

unsafe extern "C" fn get_next_key_fn(arg: *mut c_void) -> *mut c_void {
    let ctx: *mut get_next_key_ctx = arg as *mut get_next_key_ctx;
    let mut next_key: test_lpm_key = test_lpm_key { prefix: 0, data: 0 };
    let mut i: c_int = 0;

    while !(*ctx).start {
        usleep(1);
    }

    while !(*ctx).stop && {
        let old = i;
        i += 1;
        old < (*ctx).loop_
    } {
        bpf_map_get_next_key(
            (*ctx).map_fd,
            &(*ctx).key as *const test_lpm_key as *const c_void,
            &mut next_key as *mut test_lpm_key as *mut c_void,
        );
    }

    ptr::null_mut()
}

unsafe fn abort_get_next_key(ctx: *mut get_next_key_ctx, tids: *mut pthread_t, nr: c_uint) {
    let mut i: c_uint;

    (*ctx).stop = true;
    (*ctx).start = true;
    i = 0;
    while i < nr {
        pthread_join(*tids.add(i as usize), ptr::null_mut());
        i += 1;
    }
}

/* This test aims to prevent regression of future. As long as the kernel does
 * not panic, it is considered as success.
 */
pub unsafe fn test_lpm_trie_map_get_next_key() {
    const MAX_NR_THREADS: usize = 8;
    let create_opts: bpf_map_create_opts = bpf_map_create_opts {
        sz: size_of::<bpf_map_create_opts>(),
        map_flags: BPF_F_NO_PREALLOC,
    };
    let mut key: test_lpm_key = test_lpm_key { prefix: 0, data: 0 };
    let val: __u32 = 0;
    let map_fd: c_int;
    let max_prefixlen: __u32 =
        (8 * (size_of::<test_lpm_key>() - size_of::<__u32>())) as __u32;
    let max_entries: __u32 = max_prefixlen + 1;
    let mut i: c_uint;
    let nr: c_uint = MAX_NR_THREADS as c_uint;
    let loop_: c_uint = 65536;
    let mut tids: [pthread_t; MAX_NR_THREADS] = [0; MAX_NR_THREADS];
    let mut ctx: get_next_key_ctx = get_next_key_ctx {
        key: test_lpm_key { prefix: 0, data: 0 },
        start: false,
        stop: false,
        map_fd: 0,
        loop_: 0,
    };
    let mut err: c_int;

    map_fd = bpf_map_create(
        BPF_MAP_TYPE_LPM_TRIE,
        b"lpm_trie_map\0".as_ptr() as *const c_char,
        size_of::<test_lpm_key>() as __u32,
        size_of::<__u32>() as __u32,
        max_entries,
        &create_opts as *const bpf_map_create_opts,
    );
    CHECK(
        map_fd == -1,
        b"bpf_map_create()\0".as_ptr() as *const c_char,
        b"error:%s\n\0".as_ptr() as *const c_char,
        strerror(errno),
    );

    i = 0;
    while i <= max_prefixlen {
        key.prefix = i;
        err = bpf_map_update_elem(
            map_fd,
            &key as *const test_lpm_key as *const c_void,
            &val as *const __u32 as *const c_void,
            BPF_ANY,
        );
        CHECK(
            err != 0,
            b"bpf_map_update_elem()\0".as_ptr() as *const c_char,
            b"error:%s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        i += 1;
    }

    ctx.start = false;
    ctx.stop = false;
    ctx.map_fd = map_fd;
    ctx.loop_ = loop_ as c_int;
    memcpy(
        &mut ctx.key as *mut test_lpm_key as *mut c_void,
        &key as *const test_lpm_key as *const c_void,
        size_of::<test_lpm_key>(),
    );

    i = 0;
    while i < nr {
        err = pthread_create(
            &mut tids[i as usize] as *mut pthread_t,
            ptr::null(),
            get_next_key_fn,
            &mut ctx as *mut get_next_key_ctx as *mut c_void,
        );
        if err != 0 {
            abort_get_next_key(&mut ctx as *mut get_next_key_ctx, tids.as_mut_ptr(), i);
            CHECK(
                err != 0,
                b"pthread_create\0".as_ptr() as *const c_char,
                b"error %d\n\0".as_ptr() as *const c_char,
                err,
            );
        }
        i += 1;
    }

    ctx.start = true;
    i = 0;
    while i < nr {
        pthread_join(tids[i as usize], ptr::null_mut());
        i += 1;
    }

    printf(
        b"%s:PASS\n\0".as_ptr() as *const c_char,
        __func___test_lpm_trie_map_get_next_key.as_ptr(),
    );

    close(map_fd);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
