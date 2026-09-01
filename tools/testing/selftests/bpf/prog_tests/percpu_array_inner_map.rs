// SPDX-License-Identifier: GPL-2.0
// Dependency intent from C source: #include <test_progs.h>

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

unsafe extern "C" {
    static BPF_MAP_TYPE_PERCPU_ARRAY: c_int;
    static BPF_MAP_TYPE_ARRAY_OF_MAPS: c_int;

    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: c_uint,
        value_size: c_uint,
        max_entries: c_uint,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
}

// Provided by libbpf/test_progs bindings. This file needs the field used by
// LIBBPF_OPTS(bpf_map_create_opts, opts) and opts.inner_map_fd.
#[repr(C)]
pub struct bpf_map_create_opts {
    pub inner_map_fd: c_int,
}

/*
 * Test that replacing an inner percpu array map with one that has different
 * max_entries is rejected.  percpu_array_map_gen_lookup() inlines the
 * template's index_mask, so allowing a smaller replacement would cause OOB.
 */
#[no_mangle]
pub unsafe extern "C" fn test_percpu_array_inner_map() {
    let mut opts: bpf_map_create_opts = unsafe { core::mem::zeroed() };
    let mut outer_fd: c_int;
    let tmpl_fd: c_int;
    let mut good_fd: c_int;
    let mut bad_fd: c_int;
    let mut err: c_int;
    let zero: c_int = 0;

    /* Create template: percpu array with 8 entries */
    tmpl_fd = unsafe {
        bpf_map_create(
            BPF_MAP_TYPE_PERCPU_ARRAY,
            b"tmpl\0".as_ptr() as *const c_char,
            core::mem::size_of::<c_int>() as c_uint,
            core::mem::size_of::<c_long>() as c_uint,
            8,
            core::ptr::null(),
        )
    };
    if !unsafe { ASSERT_OK_FD(tmpl_fd, b"create_tmpl\0".as_ptr() as *const c_char) } {
        return;
    }

    'close_tmpl: {
        /* Create outer array-of-maps using template */
        opts.inner_map_fd = tmpl_fd;
        outer_fd = unsafe {
            bpf_map_create(
                BPF_MAP_TYPE_ARRAY_OF_MAPS,
                b"outer\0".as_ptr() as *const c_char,
                core::mem::size_of::<c_int>() as c_uint,
                core::mem::size_of::<c_int>() as c_uint,
                1,
                &opts,
            )
        };
        if !unsafe { ASSERT_OK_FD(outer_fd, b"create_outer\0".as_ptr() as *const c_char) } {
            break 'close_tmpl;
        }

        'close_outer: {
            /* Insert template as initial inner map */
            err = unsafe {
                bpf_map_update_elem(
                    outer_fd,
                    &zero as *const c_int as *const c_void,
                    &tmpl_fd as *const c_int as *const c_void,
                    0,
                )
            };
            if !unsafe { ASSERT_OK(err, b"insert_tmpl\0".as_ptr() as *const c_char) } {
                break 'close_outer;
            }

            /* Replacement with same max_entries should succeed */
            good_fd = unsafe {
                bpf_map_create(
                    BPF_MAP_TYPE_PERCPU_ARRAY,
                    b"good\0".as_ptr() as *const c_char,
                    core::mem::size_of::<c_int>() as c_uint,
                    core::mem::size_of::<c_long>() as c_uint,
                    8,
                    core::ptr::null(),
                )
            };
            if !unsafe { ASSERT_OK_FD(good_fd, b"create_good\0".as_ptr() as *const c_char) } {
                break 'close_outer;
            }

            err = unsafe {
                bpf_map_update_elem(
                    outer_fd,
                    &zero as *const c_int as *const c_void,
                    &good_fd as *const c_int as *const c_void,
                    0,
                )
            };
            unsafe {
                ASSERT_OK(
                    err,
                    b"replace_same_max_entries\0".as_ptr() as *const c_char,
                );
                close(good_fd);
            }

            /* Replacement with fewer max_entries must fail */
            bad_fd = unsafe {
                bpf_map_create(
                    BPF_MAP_TYPE_PERCPU_ARRAY,
                    b"bad\0".as_ptr() as *const c_char,
                    core::mem::size_of::<c_int>() as c_uint,
                    core::mem::size_of::<c_long>() as c_uint,
                    2,
                    core::ptr::null(),
                )
            };
            if !unsafe { ASSERT_OK_FD(bad_fd, b"create_bad\0".as_ptr() as *const c_char) } {
                break 'close_outer;
            }

            err = unsafe {
                bpf_map_update_elem(
                    outer_fd,
                    &zero as *const c_int as *const c_void,
                    &bad_fd as *const c_int as *const c_void,
                    0,
                )
            };
            unsafe {
                ASSERT_ERR(
                    err,
                    b"replace_smaller_max_entries\0".as_ptr() as *const c_char,
                );
                close(bad_fd);
            }
        }

        unsafe {
            close(outer_fd);
        }
    }

    unsafe {
        close(tmpl_fd);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
