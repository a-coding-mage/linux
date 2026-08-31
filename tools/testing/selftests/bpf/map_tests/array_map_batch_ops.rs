// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_void};

type __u32 = u32;
type __u64 = u64;
type __s64 = i64;

static mut nr_cpus: c_int = 0;

extern "C" {
    static mut errno: c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn close(fd: c_int) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn libbpf_num_possible_cpus() -> c_int;
    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_map_update_batch(
        fd: c_int,
        keys: *const c_void,
        values: *const c_void,
        count: *mut __u32,
        opts: *const bpf_map_batch_opts,
    ) -> c_int;
    fn bpf_map_lookup_batch(
        fd: c_int,
        in_batch: *mut c_void,
        out_batch: *mut c_void,
        keys: *mut c_void,
        values: *mut c_void,
        count: *mut __u32,
        opts: *const bpf_map_batch_opts,
    ) -> c_int;
}

extern "Rust" {
    fn CHECK(condition: bool, tag: *const c_char, format: *const c_char, ...);
}

const BPF_MAP_TYPE_ARRAY: c_int = 2;
const BPF_MAP_TYPE_PERCPU_ARRAY: c_int = 6;
const ENOENT: c_int = 2;

#[repr(C)]
struct bpf_map_batch_opts {
    sz: usize,
    elem_flags: __u64,
    flags: __u64,
}

unsafe fn map_batch_update(
    map_fd: c_int,
    max_entries: __u32,
    keys: *mut c_int,
    values: *mut __s64,
    is_pcpu: bool,
) {
    let mut i: c_int;
    let mut j: c_int;
    let mut err: c_int;
    let mut cpu_offset: c_int = 0;
    let opts = bpf_map_batch_opts {
        sz: core::mem::size_of::<bpf_map_batch_opts>(),
        elem_flags: 0,
        flags: 0,
    };

    i = 0;
    while i < max_entries as c_int {
        *keys.add(i as usize) = i;
        if is_pcpu {
            cpu_offset = i * nr_cpus;
            j = 0;
            while j < nr_cpus {
                *values.add((cpu_offset + j) as usize) = (i + 1 + j) as __s64;
                j += 1;
            }
        } else {
            *values.add(i as usize) = (i + 1) as __s64;
        }
        i += 1;
    }

    err = bpf_map_update_batch(
        map_fd,
        keys as *const c_void,
        values as *const c_void,
        &mut (max_entries as __u32),
        &opts,
    );
    CHECK(
        err != 0,
        b"bpf_map_update_batch()\0".as_ptr() as *const c_char,
        b"error:%s\n\0".as_ptr() as *const c_char,
        strerror(errno),
    );
}

unsafe fn map_batch_verify(
    visited: *mut c_int,
    max_entries: __u32,
    keys: *mut c_int,
    values: *mut __s64,
    is_pcpu: bool,
) {
    let mut i: c_int;
    let mut j: c_int;
    let mut cpu_offset: c_int = 0;

    memset(
        visited as *mut c_void,
        0,
        max_entries as usize * core::mem::size_of::<c_int>(),
    );
    i = 0;
    while i < max_entries as c_int {
        if is_pcpu {
            cpu_offset = i * nr_cpus;
            j = 0;
            while j < nr_cpus {
                let value: __s64 = *values.add((cpu_offset + j) as usize);
                CHECK(
                    (*keys.add(i as usize) + j + 1) as __s64 != value,
                    b"key/value checking\0".as_ptr() as *const c_char,
                    b"error: i %d j %d key %d value %lld\n\0".as_ptr() as *const c_char,
                    i,
                    j,
                    *keys.add(i as usize),
                    value,
                );
                j += 1;
            }
        } else {
            CHECK(
                (*keys.add(i as usize) + 1) as __s64 != *values.add(i as usize),
                b"key/value checking\0".as_ptr() as *const c_char,
                b"error: i %d key %d value %lld\n\0".as_ptr() as *const c_char,
                i,
                *keys.add(i as usize),
                *values.add(i as usize),
            );
        }
        *visited.add(i as usize) = 1;
        i += 1;
    }
    i = 0;
    while i < max_entries as c_int {
        CHECK(
            *visited.add(i as usize) != 1,
            b"visited checking\0".as_ptr() as *const c_char,
            b"error: keys array at index %d missing\n\0".as_ptr() as *const c_char,
            i,
        );
        i += 1;
    }
}

unsafe fn __test_map_lookup_and_update_batch(is_pcpu: bool) {
    let mut map_fd: c_int;
    let mut keys: *mut c_int;
    let mut visited: *mut c_int;
    let mut count: __u32;
    let mut total: __u32;
    let mut total_success: __u32;
    const max_entries: __u32 = 10;
    let mut batch: __u64 = 0;
    let mut err: c_int;
    let mut step: c_int;
    let mut value_size: c_int;
    let mut values: *mut c_void;
    let opts = bpf_map_batch_opts {
        sz: core::mem::size_of::<bpf_map_batch_opts>(),
        elem_flags: 0,
        flags: 0,
    };

    map_fd = bpf_map_create(
        if is_pcpu {
            BPF_MAP_TYPE_PERCPU_ARRAY
        } else {
            BPF_MAP_TYPE_ARRAY
        },
        b"array_map\0".as_ptr() as *const c_char,
        core::mem::size_of::<c_int>() as __u32,
        core::mem::size_of::<__s64>() as __u32,
        max_entries,
        core::ptr::null(),
    );
    CHECK(
        map_fd == -1,
        b"bpf_map_create()\0".as_ptr() as *const c_char,
        b"error:%s\n\0".as_ptr() as *const c_char,
        strerror(errno),
    );

    value_size = core::mem::size_of::<__s64>() as c_int;
    if is_pcpu {
        value_size *= nr_cpus;
    }

    keys = calloc(max_entries as usize, core::mem::size_of::<c_int>()) as *mut c_int;
    values = calloc(max_entries as usize, value_size as usize);
    visited = calloc(max_entries as usize, core::mem::size_of::<c_int>()) as *mut c_int;
    CHECK(
        keys.is_null() || values.is_null() || visited.is_null(),
        b"malloc()\0".as_ptr() as *const c_char,
        b"error:%s\n\0".as_ptr() as *const c_char,
        strerror(errno),
    );

    /* test 1: lookup in a loop with various steps. */
    total_success = 0;
    step = 1;
    while step < max_entries as c_int {
        map_batch_update(map_fd, max_entries, keys, values as *mut __s64, is_pcpu);
        map_batch_verify(visited, max_entries, keys, values as *mut __s64, is_pcpu);
        memset(
            keys as *mut c_void,
            0,
            max_entries as usize * core::mem::size_of::<c_int>(),
        );
        memset(values, 0, max_entries as usize * value_size as usize);
        batch = 0;
        total = 0;
        /* iteratively lookup/delete elements with 'step'
         * elements each.
         */
        count = step as __u32;
        loop {
            err = bpf_map_lookup_batch(
                map_fd,
                if total != 0 {
                    &mut batch as *mut __u64 as *mut c_void
                } else {
                    core::ptr::null_mut()
                },
                &mut batch as *mut __u64 as *mut c_void,
                keys.add(total as usize) as *mut c_void,
                (values as *mut u8).add(total as usize * value_size as usize) as *mut c_void,
                &mut count,
                &opts,
            );

            CHECK(
                err != 0 && errno != ENOENT,
                b"lookup with steps\0".as_ptr() as *const c_char,
                b"error: %s\n\0".as_ptr() as *const c_char,
                strerror(errno),
            );

            total += count;
            if err != 0 {
                break;
            }
        }

        CHECK(
            total != max_entries,
            b"lookup with steps\0".as_ptr() as *const c_char,
            b"total = %u, max_entries = %u\n\0".as_ptr() as *const c_char,
            total,
            max_entries,
        );

        map_batch_verify(visited, max_entries, keys, values as *mut __s64, is_pcpu);

        total_success += 1;
        step += 1;
    }

    CHECK(
        total_success == 0,
        b"check total_success\0".as_ptr() as *const c_char,
        b"unexpected failure\n\0".as_ptr() as *const c_char,
    );

    free(keys as *mut c_void);
    free(values);
    free(visited as *mut c_void);
    close(map_fd);
}

unsafe fn array_map_batch_ops() {
    __test_map_lookup_and_update_batch(false);
    printf(
        b"test_%s:PASS\n\0".as_ptr() as *const c_char,
        b"array_map_batch_ops\0".as_ptr() as *const c_char,
    );
}

unsafe fn array_percpu_map_batch_ops() {
    __test_map_lookup_and_update_batch(true);
    printf(
        b"test_%s:PASS\n\0".as_ptr() as *const c_char,
        b"array_percpu_map_batch_ops\0".as_ptr() as *const c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn test_array_map_batch_ops() {
    nr_cpus = libbpf_num_possible_cpus();

    CHECK(
        nr_cpus < 0,
        b"nr_cpus checking\0".as_ptr() as *const c_char,
        b"error: get possible cpus failed\0".as_ptr() as *const c_char,
    );

    array_map_batch_ops();
    array_percpu_map_batch_ops();
}
