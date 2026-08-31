// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;

const OUTER_MAP_ENTRIES: c_int = 10;
const ENOENT: c_int = 2;
const ENOSPC: c_int = 28;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_map_type {
    BPF_MAP_TYPE_ARRAY = 2,
    BPF_MAP_TYPE_HASH = 1,
    BPF_MAP_TYPE_ARRAY_OF_MAPS = 12,
    BPF_MAP_TYPE_HASH_OF_MAPS = 13,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_info {
    pub id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_create_opts {
    pub inner_map_fd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_batch_opts {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn close(fd: c_int) -> c_int;

    fn bpf_map_get_info_by_fd(
        fd: c_int,
        info: *mut bpf_map_info,
        info_len: *mut u32,
    ) -> c_int;
    fn bpf_map_create(
        map_type: bpf_map_type,
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
    fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_lookup_and_delete_batch(
        fd: c_int,
        in_batch: *mut c_void,
        out_batch: *mut c_void,
        keys: *mut c_void,
        values: *mut c_void,
        count: *mut __u32,
        opts: *mut bpf_map_batch_opts,
    ) -> c_int;
    fn bpf_map_lookup_batch(
        fd: c_int,
        in_batch: *mut c_void,
        out_batch: *mut c_void,
        keys: *mut c_void,
        values: *mut c_void,
        count: *mut __u32,
        opts: *mut bpf_map_batch_opts,
    ) -> c_int;
    fn bpf_map_update_batch(
        fd: c_int,
        keys: *const c_void,
        values: *const c_void,
        count: *mut __u32,
        opts: *mut bpf_map_batch_opts,
    ) -> c_int;
}

macro_rules! CHECK {
    ($cond:expr, $name:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        if $cond {
            panic!("{}", $name);
        }
    }};
}

unsafe fn get_map_id_from_fd(map_fd: c_int) -> __u32 {
    let mut map_info: bpf_map_info = core::mem::zeroed();
    let mut info_len: u32 = size_of::<bpf_map_info>() as u32;
    let ret: c_int;

    ret = bpf_map_get_info_by_fd(map_fd, &mut map_info, &mut info_len);
    CHECK!(
        ret < 0,
        "Finding map info failed",
        "error:%s\n",
        strerror(errno)
    );

    map_info.id
}

/* This creates number of OUTER_MAP_ENTRIES maps that will be stored
 * in outer map and return the created map_fds
 */
unsafe fn create_inner_maps(map_type: bpf_map_type, inner_map_fds: *mut __u32) {
    let mut map_fd: c_int;
    let mut map_index: c_int;
    let mut ret: c_int;
    let map_key: __u32 = 0;
    let mut map_id: __u32;
    let mut map_name: [c_char; 16] = [0; 16];

    map_index = 0;
    while map_index < OUTER_MAP_ENTRIES {
        memset(
            map_name.as_mut_ptr() as *mut c_void,
            0,
            size_of::<[c_char; 16]>(),
        );
        snprintf(
            map_name.as_mut_ptr(),
            size_of::<[c_char; 16]>(),
            b"inner_map_fd_%d\0".as_ptr() as *const c_char,
            map_index,
        );
        map_fd = bpf_map_create(
            map_type,
            map_name.as_ptr(),
            size_of::<__u32>() as __u32,
            size_of::<__u32>() as __u32,
            1,
            ptr::null(),
        );
        CHECK!(
            map_fd < 0,
            "inner bpf_map_create() failed",
            "map_type=(%d) map_name(%s), error:%s\n",
            map_type as c_int,
            map_name.as_ptr(),
            strerror(errno)
        );

        /* keep track of the inner map fd as it is required
         * to add records in outer map
         */
        *inner_map_fds.add(map_index as usize) = map_fd as __u32;

        /* Add entry into this created map
         * eg: map1 key = 0, value = map1's map id
         *     map2 key = 0, value = map2's map id
         */
        map_id = get_map_id_from_fd(map_fd);
        ret = bpf_map_update_elem(
            map_fd,
            &map_key as *const __u32 as *const c_void,
            &map_id as *const __u32 as *const c_void,
            0,
        );
        CHECK!(
            ret != 0,
            "bpf_map_update_elem failed",
            "map_type=(%d) map_name(%s), error:%s\n",
            map_type as c_int,
            map_name.as_ptr(),
            strerror(errno)
        );

        map_index += 1;
    }
}

unsafe fn create_outer_map(map_type: bpf_map_type, inner_map_fd: __u32) -> c_int {
    let outer_map_fd: c_int;
    let mut attr: bpf_map_create_opts = core::mem::zeroed();

    attr.inner_map_fd = inner_map_fd;
    outer_map_fd = bpf_map_create(
        map_type,
        b"outer_map\0".as_ptr() as *const c_char,
        size_of::<__u32>() as __u32,
        size_of::<__u32>() as __u32,
        OUTER_MAP_ENTRIES as __u32,
        &attr,
    );
    CHECK!(
        outer_map_fd < 0,
        "outer bpf_map_create()",
        "map_type=(%d), error:%s\n",
        map_type as c_int,
        strerror(errno)
    );

    outer_map_fd
}

unsafe fn validate_fetch_results(
    outer_map_fd: c_int,
    fetched_keys: *mut __u32,
    fetched_values: *mut __u32,
    max_entries_fetched: __u32,
) {
    let mut inner_map_key: __u32 = 0;
    let mut inner_map_value: __u32 = 0;
    let mut inner_map_fd: c_int;
    let mut entry: c_int;
    let mut err: c_int;
    let mut outer_map_value: __u32;

    entry = 0;
    while entry < max_entries_fetched as c_int {
        outer_map_value = *fetched_values.add(entry as usize);
        inner_map_fd = bpf_map_get_fd_by_id(outer_map_value);
        CHECK!(
            inner_map_fd < 0,
            "Failed to get inner map fd",
            "from id(%d), error=%s\n",
            outer_map_value,
            strerror(errno)
        );
        err = bpf_map_get_next_key(
            inner_map_fd,
            ptr::null(),
            &mut inner_map_key as *mut __u32 as *mut c_void,
        );
        CHECK!(
            err != 0,
            "Failed to get inner map key",
            "error=%s\n",
            strerror(errno)
        );

        err = bpf_map_lookup_elem(
            inner_map_fd,
            &inner_map_key as *const __u32 as *const c_void,
            &mut inner_map_value as *mut __u32 as *mut c_void,
        );

        close(inner_map_fd);

        CHECK!(
            err != 0,
            "Failed to get inner map value",
            "for key(%d), error=%s\n",
            inner_map_key,
            strerror(errno)
        );

        /* Actual value validation */
        CHECK!(
            outer_map_value != inner_map_value,
            "Failed to validate inner map value",
            "fetched(%d) and lookedup(%d)!\n",
            outer_map_value,
            inner_map_value
        );

        entry += 1;
    }
}

unsafe fn fetch_and_validate(
    outer_map_fd: c_int,
    opts: *mut bpf_map_batch_opts,
    batch_size: __u32,
    delete_entries: bool,
    has_holes: bool,
) {
    let mut err: c_int;
    let max_entries: c_int = OUTER_MAP_ENTRIES - if has_holes { 1 } else { 0 };
    let fetched_keys: *mut __u32;
    let fetched_values: *mut __u32;
    let mut total_fetched: __u32 = 0;
    let mut i: __u32;
    let mut batch_key: __u32 = 0;
    let mut fetch_count: __u32;
    let mut step_size: __u32 = batch_size;
    let value_size: __u32 = size_of::<__u32>() as __u32;

    /* Total entries needs to be fetched */
    fetched_keys = calloc(max_entries as usize, value_size as usize) as *mut __u32;
    fetched_values = calloc(max_entries as usize, value_size as usize) as *mut __u32;
    CHECK!(
        fetched_keys.is_null() || fetched_values.is_null(),
        "Memory allocation failed for fetched_keys or fetched_values",
        "error=%s\n",
        strerror(errno)
    );

    /* hash map may not always return full batch */
    i = 0;
    while i < OUTER_MAP_ENTRIES as __u32 {
        fetch_count = step_size;
        err = if delete_entries {
            bpf_map_lookup_and_delete_batch(
                outer_map_fd,
                if total_fetched != 0 {
                    &mut batch_key as *mut __u32 as *mut c_void
                } else {
                    ptr::null_mut()
                },
                &mut batch_key as *mut __u32 as *mut c_void,
                fetched_keys.add(total_fetched as usize) as *mut c_void,
                fetched_values.add(total_fetched as usize) as *mut c_void,
                &mut fetch_count,
                opts,
            )
        } else {
            bpf_map_lookup_batch(
                outer_map_fd,
                if total_fetched != 0 {
                    &mut batch_key as *mut __u32 as *mut c_void
                } else {
                    ptr::null_mut()
                },
                &mut batch_key as *mut __u32 as *mut c_void,
                fetched_keys.add(total_fetched as usize) as *mut c_void,
                fetched_values.add(total_fetched as usize) as *mut c_void,
                &mut fetch_count,
                opts,
            )
        };

        if err != 0 && errno == ENOSPC {
            /* Fetch again with higher batch size */
            total_fetched = 0;
            step_size += batch_size;
            i += 1;
            continue;
        }

        CHECK!(
            err < 0 && errno != ENOENT,
            "lookup with steps failed",
            "error: %s\n",
            strerror(errno)
        );

        /* Update the total fetched number */
        total_fetched += fetch_count;
        if err != 0 {
            break;
        }

        i += 1;
    }

    CHECK!(
        total_fetched != max_entries as __u32,
        "Unable to fetch expected entries !",
        "total_fetched(%d) and max_entries(%d) error: (%d):%s\n",
        total_fetched,
        max_entries,
        errno,
        strerror(errno)
    );

    /* validate the fetched entries */
    validate_fetch_results(outer_map_fd, fetched_keys, fetched_values, total_fetched);
    printf(
        b"batch_op(%s) is successful with batch_size(%d)\n\0".as_ptr() as *const c_char,
        if delete_entries {
            b"LOOKUP_AND_DELETE\0".as_ptr()
        } else {
            b"LOOKUP\0".as_ptr()
        } as *const c_char,
        batch_size,
    );

    free(fetched_keys as *mut c_void);
    free(fetched_values as *mut c_void);
}

unsafe fn _map_in_map_batch_ops(
    outer_map_type: bpf_map_type,
    inner_map_type: bpf_map_type,
    has_holes: bool,
) {
    let mut max_entries: __u32 = (OUTER_MAP_ENTRIES - if has_holes { 1 } else { 0 }) as __u32;
    let outer_map_keys: *mut __u32;
    let inner_map_fds: *mut __u32;
    let mut opts: bpf_map_batch_opts = core::mem::zeroed();
    let value_size: __u32 = size_of::<__u32>() as __u32;
    let batch_size: [c_int; 2] = [5, 10];
    let mut map_index: __u32;
    let mut op_index: __u32;
    let outer_map_fd: c_int;
    let ret: c_int;

    outer_map_keys = calloc(OUTER_MAP_ENTRIES as usize, value_size as usize) as *mut __u32;
    inner_map_fds = calloc(OUTER_MAP_ENTRIES as usize, value_size as usize) as *mut __u32;
    CHECK!(
        outer_map_keys.is_null() || inner_map_fds.is_null(),
        "Memory allocation failed for outer_map_keys or inner_map_fds",
        "error=%s\n",
        strerror(errno)
    );

    create_inner_maps(inner_map_type, inner_map_fds);

    outer_map_fd = create_outer_map(outer_map_type, *inner_map_fds);
    /* create outer map keys */
    map_index = 0;
    while map_index < max_entries {
        *outer_map_keys.add(map_index as usize) = (if outer_map_type
            == bpf_map_type::BPF_MAP_TYPE_ARRAY_OF_MAPS
        {
            9
        } else {
            1000
        }) - map_index;
        map_index += 1;
    }

    /* This condition is only meaningful for array of maps.
     *
     * max_entries == OUTER_MAP_ENTRIES - 1 if it is true. Say
     * max_entries is short for n, then outer_map_keys looks like:
     *
     *   [n, n-1, ... 2, 1]
     *
     * We change it to
     *
     *   [n, n-1, ... 2, 0]
     *
     * So it will leave key 1 as a hole. It will serve to test the
     * correctness when batch on an array: a "non-exist" key might be
     * actually allocated and returned from key iteration.
     */
    if has_holes {
        *outer_map_keys.add((max_entries - 1) as usize) -= 1;
    }

    /* batch operation - map_update */
    ret = bpf_map_update_batch(
        outer_map_fd,
        outer_map_keys as *const c_void,
        inner_map_fds as *const c_void,
        &mut max_entries,
        &mut opts,
    );
    CHECK!(
        ret != 0,
        "Failed to update the outer map batch ops",
        "error=%s\n",
        strerror(errno)
    );

    /* batch operation - map_lookup */
    op_index = 0;
    while op_index < 2 {
        fetch_and_validate(
            outer_map_fd,
            &mut opts,
            batch_size[op_index as usize] as __u32,
            false,
            has_holes,
        );
        op_index += 1;
    }

    /* batch operation - map_lookup_delete */
    if outer_map_type == bpf_map_type::BPF_MAP_TYPE_HASH_OF_MAPS {
        fetch_and_validate(
            outer_map_fd,
            &mut opts,
            max_entries,
            true, /*delete*/
            has_holes,
        );
    }

    /* close all map fds */
    map_index = 0;
    while map_index < OUTER_MAP_ENTRIES as __u32 {
        close(*inner_map_fds.add(map_index as usize) as c_int);
        map_index += 1;
    }
    close(outer_map_fd);

    free(inner_map_fds as *mut c_void);
    free(outer_map_keys as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn test_map_in_map_batch_ops_array() {
    _map_in_map_batch_ops(
        bpf_map_type::BPF_MAP_TYPE_ARRAY_OF_MAPS,
        bpf_map_type::BPF_MAP_TYPE_ARRAY,
        false,
    );
    printf(
        b"%s:PASS with inner ARRAY map\n\0".as_ptr() as *const c_char,
        b"test_map_in_map_batch_ops_array\0".as_ptr() as *const c_char,
    );
    _map_in_map_batch_ops(
        bpf_map_type::BPF_MAP_TYPE_ARRAY_OF_MAPS,
        bpf_map_type::BPF_MAP_TYPE_HASH,
        false,
    );
    printf(
        b"%s:PASS with inner HASH map\n\0".as_ptr() as *const c_char,
        b"test_map_in_map_batch_ops_array\0".as_ptr() as *const c_char,
    );
    _map_in_map_batch_ops(
        bpf_map_type::BPF_MAP_TYPE_ARRAY_OF_MAPS,
        bpf_map_type::BPF_MAP_TYPE_ARRAY,
        true,
    );
    printf(
        b"%s:PASS with inner ARRAY map with holes\n\0".as_ptr() as *const c_char,
        b"test_map_in_map_batch_ops_array\0".as_ptr() as *const c_char,
    );
    _map_in_map_batch_ops(
        bpf_map_type::BPF_MAP_TYPE_ARRAY_OF_MAPS,
        bpf_map_type::BPF_MAP_TYPE_HASH,
        true,
    );
    printf(
        b"%s:PASS with inner HASH map with holes\n\0".as_ptr() as *const c_char,
        b"test_map_in_map_batch_ops_array\0".as_ptr() as *const c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn test_map_in_map_batch_ops_hash() {
    _map_in_map_batch_ops(
        bpf_map_type::BPF_MAP_TYPE_HASH_OF_MAPS,
        bpf_map_type::BPF_MAP_TYPE_ARRAY,
        false,
    );
    printf(
        b"%s:PASS with inner ARRAY map\n\0".as_ptr() as *const c_char,
        b"test_map_in_map_batch_ops_hash\0".as_ptr() as *const c_char,
    );
    _map_in_map_batch_ops(
        bpf_map_type::BPF_MAP_TYPE_HASH_OF_MAPS,
        bpf_map_type::BPF_MAP_TYPE_HASH,
        false,
    );
    printf(
        b"%s:PASS with inner HASH map\n\0".as_ptr() as *const c_char,
        b"test_map_in_map_batch_ops_hash\0".as_ptr() as *const c_char,
    );
}
