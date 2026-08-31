// SPDX-License-Identifier: GPL-2.0

// Translated from testing/selftests/bpf/map_tests/lpm_trie_map_batch_ops.c.
// C includes referenced libbpf, Linux BPF, networking, libc, and test_maps
// declarations. Those dependencies are expected to be supplied by the target
// harness.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u32 = u32;
type __u64 = u64;

const AF_INET: c_int = 2;
const ENOENT: c_int = 2;
const BPF_MAP_TYPE_LPM_TRIE: c_uint = 11;
const BPF_F_NO_PREALLOC: __u32 = 1;

#[repr(C)]
pub struct in_addr {
    pub s_addr: __u32,
}

#[repr(C)]
struct test_lpm_key {
    prefix: __u32,
    ipv4: in_addr,
}

#[repr(C)]
pub struct bpf_map_batch_opts {
    pub sz: usize,
    pub elem_flags: __u64,
    pub flags: __u64,
}

#[repr(C)]
pub struct bpf_map_create_opts {
    pub sz: usize,
    pub btf_fd: __u32,
    pub btf_key_type_id: __u32,
    pub btf_value_type_id: __u32,
    pub btf_vmlinux_value_type_id: __u32,
    pub inner_map_fd: __u32,
    pub map_flags: __u32,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn inet_ntop(
        af: c_int,
        src: *const c_void,
        dst: *mut c_char,
        size: c_uint,
    ) -> *const c_char;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

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
    fn bpf_map_delete_batch(
        fd: c_int,
        keys: *const c_void,
        count: *mut __u32,
        opts: *const bpf_map_batch_opts,
    ) -> c_int;
    fn bpf_map_create(
        map_type: c_uint,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
}

unsafe fn map_batch_update(
    map_fd: c_int,
    max_entries: __u32,
    keys: *mut test_lpm_key,
    values: *mut c_int,
) {
    let mut i: __u32;
    let mut err: c_int;
    let mut count: __u32 = max_entries;
    let mut buff: [c_char; 16] = [0; 16];
    let mut opts = bpf_map_batch_opts {
        sz: size_of::<bpf_map_batch_opts>(),
        elem_flags: 0,
        flags: 0,
    };

    i = 0;
    while i < max_entries {
        (*keys.add(i as usize)).prefix = 32;
        snprintf(
            buff.as_mut_ptr(),
            16,
            b"192.168.1.%d\0".as_ptr() as *const c_char,
            i + 1,
        );
        inet_pton(
            AF_INET,
            buff.as_ptr(),
            &mut (*keys.add(i as usize)).ipv4 as *mut in_addr as *mut c_void,
        );
        *values.add(i as usize) = (i + 1) as c_int;
        i += 1;
    }

    err = bpf_map_update_batch(
        map_fd,
        keys as *const c_void,
        values as *const c_void,
        &mut count,
        &opts,
    );
    CHECK!(
        err != 0,
        "bpf_map_update_batch()\0",
        "error:%s\n\0",
        strerror(errno)
    );
}

unsafe fn map_batch_verify(
    visited: *mut c_int,
    max_entries: __u32,
    keys: *mut test_lpm_key,
    values: *mut c_int,
) {
    let mut buff: [c_char; 16] = [0; 16];
    let mut lower_byte: c_int = 0;
    let mut i: __u32;

    memset(
        visited as *mut c_void,
        0,
        max_entries as usize * size_of::<c_int>(),
    );
    i = 0;
    while i < max_entries {
        inet_ntop(
            AF_INET,
            &(*keys.add(i as usize)).ipv4 as *const in_addr as *const c_void,
            buff.as_mut_ptr(),
            32,
        );
        CHECK!(
            sscanf(
                buff.as_ptr(),
                b"192.168.1.%d\0".as_ptr() as *const c_char,
                &mut lower_byte,
            ) == -1,
            "sscanf()\0",
            "error: i %d\n\0",
            i
        );
        CHECK!(
            lower_byte != *values.add(i as usize),
            "key/value checking\0",
            "error: i %d key %s value %d\n\0",
            i,
            buff.as_ptr(),
            *values.add(i as usize)
        );
        *visited.add(i as usize) = 1;
        i += 1;
    }
    i = 0;
    while i < max_entries {
        CHECK!(
            *visited.add(i as usize) != 1,
            "visited checking\0",
            "error: keys array at index %d missing\n\0",
            i
        );
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_lpm_trie_map_batch_ops() {
    let mut create_opts: bpf_map_create_opts = zeroed();
    create_opts.sz = size_of::<bpf_map_create_opts>();
    create_opts.map_flags = BPF_F_NO_PREALLOC;
    let mut keys: *mut test_lpm_key;
    let mut key: test_lpm_key = zeroed();
    let mut map_fd: c_int;
    let mut values: *mut c_int;
    let mut visited: *mut c_int;
    let mut step: __u32;
    let mut count: __u32;
    let mut total: __u32;
    let mut total_success: __u32;
    const max_entries: __u32 = 10;
    let mut batch: __u64 = 0;
    let mut err: c_int;
    let mut opts = bpf_map_batch_opts {
        sz: size_of::<bpf_map_batch_opts>(),
        elem_flags: 0,
        flags: 0,
    };

    map_fd = bpf_map_create(
        BPF_MAP_TYPE_LPM_TRIE,
        b"lpm_trie_map\0".as_ptr() as *const c_char,
        size_of::<test_lpm_key>() as __u32,
        size_of::<c_int>() as __u32,
        max_entries,
        &create_opts,
    );
    CHECK!(
        map_fd == -1,
        "bpf_map_create()\0",
        "error:%s\n\0",
        strerror(errno)
    );

    keys = malloc(max_entries as usize * size_of::<test_lpm_key>()) as *mut test_lpm_key;
    values = malloc(max_entries as usize * size_of::<c_int>()) as *mut c_int;
    visited = malloc(max_entries as usize * size_of::<c_int>()) as *mut c_int;
    CHECK!(
        keys.is_null() || values.is_null() || visited.is_null(),
        "malloc()\0",
        "error:%s\n\0",
        strerror(errno)
    );

    total_success = 0;
    step = 1;
    while step < max_entries {
        map_batch_update(map_fd, max_entries, keys, values);
        map_batch_verify(visited, max_entries, keys, values);
        memset(
            keys as *mut c_void,
            0,
            max_entries as usize * size_of::<test_lpm_key>(),
        );
        memset(
            values as *mut c_void,
            0,
            max_entries as usize * size_of::<c_int>(),
        );
        batch = 0;
        total = 0;
        /* iteratively lookup/delete elements with 'step'
         * elements each.
         */
        count = step;
        loop {
            err = bpf_map_lookup_batch(
                map_fd,
                if total != 0 {
                    &mut batch as *mut __u64 as *mut c_void
                } else {
                    ptr::null_mut()
                },
                &mut batch as *mut __u64 as *mut c_void,
                keys.add(total as usize) as *mut c_void,
                values.add(total as usize) as *mut c_void,
                &mut count,
                &opts,
            );

            CHECK!(
                err != 0 && errno != ENOENT,
                "lookup with steps\0",
                "error: %s\n\0",
                strerror(errno)
            );

            total += count;
            if err != 0 {
                break;
            }
        }

        CHECK!(
            total != max_entries,
            "lookup with steps\0",
            "total = %u, max_entries = %u\n\0",
            total,
            max_entries
        );

        map_batch_verify(visited, max_entries, keys, values);

        total = 0;
        count = step;
        while total < max_entries {
            if max_entries - total < step {
                count = max_entries - total;
            }
            err = bpf_map_delete_batch(
                map_fd,
                keys.add(total as usize) as *const c_void,
                &mut count,
                &opts,
            );
            CHECK!(
                err != 0 && errno != ENOENT,
                "delete batch\0",
                "error: %s\n\0",
                strerror(errno)
            );
            total += count;
            if err != 0 {
                break;
            }
        }
        CHECK!(
            total != max_entries,
            "delete with steps\0",
            "total = %u, max_entries = %u\n\0",
            total,
            max_entries
        );

        /* check map is empty, errno == ENOENT */
        err = bpf_map_get_next_key(
            map_fd,
            ptr::null(),
            &mut key as *mut test_lpm_key as *mut c_void,
        );
        CHECK!(
            err == 0 || errno != ENOENT,
            "bpf_map_get_next_key()\0",
            "error: %s\n\0",
            strerror(errno)
        );

        total_success += 1;
        step += 1;
    }

    CHECK!(
        total_success == 0,
        "check total_success\0",
        "unexpected failure\n\0"
    );

    printf(b"%s:PASS\n\0".as_ptr() as *const c_char, b"test_lpm_trie_map_batch_ops\0".as_ptr());

    free(keys as *mut c_void);
    free(values as *mut c_void);
    free(visited as *mut c_void);
    close(map_fd);
}
