// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook  */

/* Translated from C. Original includes:
 * <stdio.h>, <errno.h>, <string.h>, <unistd.h>
 * <bpf/bpf.h>, <bpf/libbpf.h>
 * <bpf_util.h>, <test_maps.h>
 */

use core::ffi::{c_char, c_int, c_void};

type __u32 = u32;

const ENOENT: c_int = 2;
const ENOSPC: c_int = 28;
const BPF_MAP_TYPE_HASH: c_int = 1;
const BPF_MAP_TYPE_PERCPU_HASH: c_int = 5;

#[repr(C)]
pub struct bpf_map_batch_opts {
    pub sz: usize,
    pub elem_flags: u64,
    pub flags: u64,
}

/* Rust stand-in for BPF_DECLARE_PERCPU(int, value). */
type value = [c_int; 128];

unsafe extern "C" {
    static mut errno: c_int;

    fn strerror(errnum: c_int) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;

    fn bpf_num_possible_cpus() -> c_int;
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
        keys: *mut c_void,
        values: *mut c_void,
        count: *mut __u32,
        opts: *const bpf_map_batch_opts,
    ) -> c_int;
    fn bpf_map_lookup_and_delete_batch(
        fd: c_int,
        in_batch: *mut c_void,
        out_batch: *mut c_void,
        keys: *mut c_void,
        values: *mut c_void,
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
    fn bpf_map_delete_batch(
        fd: c_int,
        keys: *mut c_void,
        count: *mut __u32,
        opts: *const bpf_map_batch_opts,
    ) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
}

unsafe fn map_batch_update(
    map_fd: c_int,
    mut max_entries: __u32,
    keys: *mut c_int,
    values: *mut c_void,
    is_pcpu: bool,
) {
    let mut v: *mut value = core::ptr::null_mut();
    let mut i: c_int;
    let mut j: c_int;
    let mut err: c_int;
    let opts = bpf_map_batch_opts {
        sz: core::mem::size_of::<bpf_map_batch_opts>(),
        elem_flags: 0,
        flags: 0,
    };

    if is_pcpu {
        v = values as *mut value;
    }

    i = 0;
    while i < max_entries as c_int {
        *keys.add(i as usize) = i + 1;
        if is_pcpu {
            j = 0;
            while j < bpf_num_possible_cpus() {
                (*v.add(i as usize))[j as usize] = i + 2 + j;
                j += 1;
            }
        } else {
            *((values as *mut c_int).add(i as usize)) = i + 2;
        }
        i += 1;
    }

    err = bpf_map_update_batch(
        map_fd,
        keys as *mut c_void,
        values,
        &mut max_entries,
        &opts,
    );
    CHECK!(
        err != 0,
        c"bpf_map_update_batch()".as_ptr(),
        c"error:%s\n".as_ptr(),
        strerror(errno)
    );
}

unsafe fn map_batch_verify(
    visited: *mut c_int,
    max_entries: __u32,
    keys: *mut c_int,
    values: *mut c_void,
    is_pcpu: bool,
) {
    let mut v: *mut value = core::ptr::null_mut();
    let mut i: c_int;
    let mut j: c_int;

    if is_pcpu {
        v = values as *mut value;
    }

    memset(
        visited as *mut c_void,
        0,
        max_entries as usize * core::mem::size_of_val(&*visited),
    );
    i = 0;
    while i < max_entries as c_int {
        if is_pcpu {
            j = 0;
            while j < bpf_num_possible_cpus() {
                CHECK!(
                    *keys.add(i as usize) + 1 + j != (*v.add(i as usize))[j as usize],
                    c"key/value checking".as_ptr(),
                    c"error: i %d j %d key %d value %d\n".as_ptr(),
                    i,
                    j,
                    *keys.add(i as usize),
                    (*v.add(i as usize))[j as usize]
                );
                j += 1;
            }
        } else {
            CHECK!(
                *keys.add(i as usize) + 1 != *((values as *mut c_int).add(i as usize)),
                c"key/value checking".as_ptr(),
                c"error: i %d key %d value %d\n".as_ptr(),
                i,
                *keys.add(i as usize),
                *((values as *mut c_int).add(i as usize))
            );
        }

        *visited.add(i as usize) = 1;

        i += 1;
    }
    i = 0;
    while i < max_entries as c_int {
        CHECK!(
            *visited.add(i as usize) != 1,
            c"visited checking".as_ptr(),
            c"error: keys array at index %d missing\n".as_ptr(),
            i
        );
        i += 1;
    }
}

pub unsafe fn __test_map_lookup_and_delete_batch(is_pcpu: bool) {
    let mut batch: __u32 = 0;
    let mut count: __u32;
    let mut total: __u32;
    let mut total_success: __u32;
    let mut map_fd: c_int;
    let mut keys: *mut c_int;
    let mut visited: *mut c_int;
    let mut key: c_int = 0;
    const max_entries: __u32 = 10;
    let mut pcpu_values: [value; max_entries as usize] = [[0; 128]; max_entries as usize];
    let mut err: c_int;
    let mut step: c_int;
    let value_size: c_int;
    let mut nospace_err: bool;
    let mut values: *mut c_void;
    let opts = bpf_map_batch_opts {
        sz: core::mem::size_of::<bpf_map_batch_opts>(),
        elem_flags: 0,
        flags: 0,
    };

    map_fd = bpf_map_create(
        if is_pcpu {
            BPF_MAP_TYPE_PERCPU_HASH
        } else {
            BPF_MAP_TYPE_HASH
        },
        c"hash_map".as_ptr(),
        core::mem::size_of::<c_int>() as __u32,
        core::mem::size_of::<c_int>() as __u32,
        max_entries,
        core::ptr::null(),
    );
    CHECK!(
        map_fd == -1,
        c"bpf_map_create()".as_ptr(),
        c"error:%s\n".as_ptr(),
        strerror(errno)
    );

    value_size = if is_pcpu {
        core::mem::size_of::<value>() as c_int
    } else {
        core::mem::size_of::<c_int>() as c_int
    };
    keys = malloc(max_entries as usize * core::mem::size_of::<c_int>()) as *mut c_int;
    if is_pcpu {
        values = pcpu_values.as_mut_ptr() as *mut c_void;
    } else {
        values = malloc(max_entries as usize * core::mem::size_of::<c_int>());
    }
    visited = malloc(max_entries as usize * core::mem::size_of::<c_int>()) as *mut c_int;
    CHECK!(
        keys.is_null() || values.is_null() || visited.is_null(),
        c"malloc()".as_ptr(),
        c"error:%s\n".as_ptr(),
        strerror(errno)
    );

    /* test 1: lookup/delete an empty hash table, -ENOENT */
    count = max_entries;
    err = bpf_map_lookup_and_delete_batch(
        map_fd,
        core::ptr::null_mut(),
        &mut batch as *mut __u32 as *mut c_void,
        keys as *mut c_void,
        values,
        &mut count,
        &opts,
    );
    CHECK!(
        err != 0 && errno != ENOENT,
        c"empty map".as_ptr(),
        c"error: %s\n".as_ptr(),
        strerror(errno)
    );

    /* populate elements to the map */
    map_batch_update(map_fd, max_entries, keys, values, is_pcpu);

    /* test 2: lookup/delete with count = 0, success */
    count = 0;
    err = bpf_map_lookup_and_delete_batch(
        map_fd,
        core::ptr::null_mut(),
        &mut batch as *mut __u32 as *mut c_void,
        keys as *mut c_void,
        values,
        &mut count,
        &opts,
    );
    CHECK!(
        err != 0,
        c"count = 0".as_ptr(),
        c"error: %s\n".as_ptr(),
        strerror(errno)
    );

    /* test 3: lookup/delete with count = max_entries, success */
    memset(
        keys as *mut c_void,
        0,
        max_entries as usize * core::mem::size_of_val(&*keys),
    );
    memset(values, 0, max_entries as usize * value_size as usize);
    count = max_entries;
    err = bpf_map_lookup_and_delete_batch(
        map_fd,
        core::ptr::null_mut(),
        &mut batch as *mut __u32 as *mut c_void,
        keys as *mut c_void,
        values,
        &mut count,
        &opts,
    );
    CHECK!(
        err != 0 && errno != ENOENT,
        c"count = max_entries".as_ptr(),
        c"error: %s\n".as_ptr(),
        strerror(errno)
    );
    CHECK!(
        count != max_entries,
        c"count = max_entries".as_ptr(),
        c"count = %u, max_entries = %u\n".as_ptr(),
        count,
        max_entries
    );
    map_batch_verify(visited, max_entries, keys, values, is_pcpu);

    /* bpf_map_get_next_key() should return -ENOENT for an empty map. */
    err = bpf_map_get_next_key(map_fd, core::ptr::null(), &mut key as *mut c_int as *mut c_void);
    CHECK!(
        err == 0,
        c"bpf_map_get_next_key()".as_ptr(),
        c"error: %s\n".as_ptr(),
        strerror(errno)
    );

    /* test 4: lookup/delete in a loop with various steps. */
    total_success = 0;
    step = 1;
    while step < max_entries as c_int {
        map_batch_update(map_fd, max_entries, keys, values, is_pcpu);
        memset(
            keys as *mut c_void,
            0,
            max_entries as usize * core::mem::size_of_val(&*keys),
        );
        memset(values, 0, max_entries as usize * value_size as usize);
        total = 0;
        /* iteratively lookup/delete elements with 'step'
         * elements each
         */
        count = step as __u32;
        nospace_err = false;
        loop {
            err = bpf_map_lookup_batch(
                map_fd,
                if total != 0 {
                    &mut batch as *mut __u32 as *mut c_void
                } else {
                    core::ptr::null_mut()
                },
                &mut batch as *mut __u32 as *mut c_void,
                keys.add(total as usize) as *mut c_void,
                (values as *mut u8).add(total as usize * value_size as usize) as *mut c_void,
                &mut count,
                &opts,
            );
            /* It is possible that we are failing due to buffer size
             * not big enough. In such cases, let us just exit and
             * go with large steps. Not that a buffer size with
             * max_entries should always work.
             */
            if err != 0 && errno == ENOSPC {
                nospace_err = true;
                break;
            }

            CHECK!(
                err != 0 && errno != ENOENT,
                c"lookup with steps".as_ptr(),
                c"error: %s\n".as_ptr(),
                strerror(errno)
            );

            total += count;
            if err != 0 {
                break;
            }
        }
        if nospace_err == true {
            step += 1;
            continue;
        }

        CHECK!(
            total != max_entries,
            c"lookup with steps".as_ptr(),
            c"total = %u, max_entries = %u\n".as_ptr(),
            total,
            max_entries
        );
        map_batch_verify(visited, max_entries, keys, values, is_pcpu);

        total = 0;
        count = step as __u32;
        while total < max_entries {
            if max_entries - total < step as __u32 {
                count = max_entries - total;
            }
            err = bpf_map_delete_batch(
                map_fd,
                keys.add(total as usize) as *mut c_void,
                &mut count,
                &opts,
            );
            CHECK!(
                err != 0 && errno != ENOENT,
                c"delete batch".as_ptr(),
                c"error: %s\n".as_ptr(),
                strerror(errno)
            );
            total += count;
            if err != 0 {
                break;
            }
        }
        CHECK!(
            total != max_entries,
            c"delete with steps".as_ptr(),
            c"total = %u, max_entries = %u\n".as_ptr(),
            total,
            max_entries
        );

        /* check map is empty, errno == ENOENT */
        err = bpf_map_get_next_key(map_fd, core::ptr::null(), &mut key as *mut c_int as *mut c_void);
        CHECK!(
            err == 0 || errno != ENOENT,
            c"bpf_map_get_next_key()".as_ptr(),
            c"error: %s\n".as_ptr(),
            strerror(errno)
        );

        /* iteratively lookup/delete elements with 'step'
         * elements each
         */
        map_batch_update(map_fd, max_entries, keys, values, is_pcpu);
        memset(
            keys as *mut c_void,
            0,
            max_entries as usize * core::mem::size_of_val(&*keys),
        );
        memset(values, 0, max_entries as usize * value_size as usize);
        total = 0;
        count = step as __u32;
        nospace_err = false;
        loop {
            err = bpf_map_lookup_and_delete_batch(
                map_fd,
                if total != 0 {
                    &mut batch as *mut __u32 as *mut c_void
                } else {
                    core::ptr::null_mut()
                },
                &mut batch as *mut __u32 as *mut c_void,
                keys.add(total as usize) as *mut c_void,
                (values as *mut u8).add(total as usize * value_size as usize) as *mut c_void,
                &mut count,
                &opts,
            );
            /* It is possible that we are failing due to buffer size
             * not big enough. In such cases, let us just exit and
             * go with large steps. Not that a buffer size with
             * max_entries should always work.
             */
            if err != 0 && errno == ENOSPC {
                nospace_err = true;
                break;
            }

            CHECK!(
                err != 0 && errno != ENOENT,
                c"lookup with steps".as_ptr(),
                c"error: %s\n".as_ptr(),
                strerror(errno)
            );

            total += count;
            if err != 0 {
                break;
            }
        }

        if nospace_err == true {
            step += 1;
            continue;
        }

        CHECK!(
            total != max_entries,
            c"lookup/delete with steps".as_ptr(),
            c"total = %u, max_entries = %u\n".as_ptr(),
            total,
            max_entries
        );

        map_batch_verify(visited, max_entries, keys, values, is_pcpu);
        err = bpf_map_get_next_key(map_fd, core::ptr::null(), &mut key as *mut c_int as *mut c_void);
        CHECK!(
            err == 0,
            c"bpf_map_get_next_key()".as_ptr(),
            c"error: %s\n".as_ptr(),
            strerror(errno)
        );

        total_success += 1;
        step += 1;
    }

    CHECK!(
        total_success == 0,
        c"check total_success".as_ptr(),
        c"unexpected failure\n".as_ptr()
    );
    free(keys as *mut c_void);
    free(visited as *mut c_void);
    if !is_pcpu {
        free(values);
    }
    close(map_fd);
}

pub unsafe fn htab_map_batch_ops() {
    __test_map_lookup_and_delete_batch(false);
    printf(c"test_%s:PASS\n".as_ptr(), c"htab_map_batch_ops".as_ptr());
}

pub unsafe fn htab_percpu_map_batch_ops() {
    __test_map_lookup_and_delete_batch(true);
    printf(c"test_%s:PASS\n".as_ptr(), c"htab_percpu_map_batch_ops".as_ptr());
}

pub unsafe fn test_htab_map_batch_ops() {
    htab_map_batch_ops();
    htab_percpu_map_batch_ops();
}
