// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2022 Huawei Technologies Duesseldorf GmbH
 *
 * Author: Roberto Sassu <roberto.sassu@huawei.com>
 */

// C dependencies:
// #include <test_progs.h>
// #include "test_libbpf_get_fd_by_id_opts.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type __u32 = u32;

const BPF_F_RDONLY: __u32 = 1 << 3;
const BPF_ANY: __u64 = 0;
const EINVAL: c_int = 22;

type __u64 = u64;

#[repr(C)]
pub struct bpf_map_info {
    pub type_: __u32,
    pub id: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
    pub name: [c_char; 16],
    pub ifindex: __u32,
    pub btf_vmlinux_value_type_id: __u32,
    pub netns_dev: __u64,
    pub netns_ino: __u64,
    pub btf_id: __u32,
    pub btf_key_type_id: __u32,
    pub btf_value_type_id: __u32,
    pub map_extra: __u64,
}

#[repr(C)]
pub struct bpf_get_fd_by_id_opts {
    pub sz: usize,
    pub open_flags: __u32,
}

#[repr(C)]
pub struct test_libbpf_get_fd_by_id_opts {
    pub maps: test_libbpf_get_fd_by_id_opts_maps,
}

#[repr(C)]
pub struct test_libbpf_get_fd_by_id_opts_maps {
    pub data_input: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn test_libbpf_get_fd_by_id_opts__open_and_load() -> *mut test_libbpf_get_fd_by_id_opts;
    fn test_libbpf_get_fd_by_id_opts__attach(
        skel: *mut test_libbpf_get_fd_by_id_opts,
    ) -> c_int;
    fn test_libbpf_get_fd_by_id_opts__destroy(skel: *mut test_libbpf_get_fd_by_id_opts);

    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *mut __u32) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_map_get_fd_by_id_opts(id: __u32, opts: *const bpf_get_fd_by_id_opts) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> c_int;
    fn bpf_prog_get_fd_by_id_opts(id: __u32, opts: *const bpf_get_fd_by_id_opts) -> c_int;
    fn bpf_link_get_fd_by_id_opts(id: __u32, opts: *const bpf_get_fd_by_id_opts) -> c_int;
    fn bpf_btf_get_fd_by_id_opts(id: __u32, opts: *const bpf_get_fd_by_id_opts) -> c_int;

    fn close(fd: c_int) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_LT(ret: c_int, val: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(ret: c_int, val: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(ret: c_int, val: c_int, name: *const c_char) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn test_libbpf_get_fd_by_id_opts() {
    let mut skel: *mut test_libbpf_get_fd_by_id_opts;
    let mut info_m: bpf_map_info = mem::zeroed();
    let mut len: __u32 = mem::size_of_val(&info_m) as __u32;
    let mut value: __u32 = 0;
    let mut ret: c_int;
    let zero: c_int = 0;
    let mut fd: c_int = -1;
    let fd_opts_rdonly = bpf_get_fd_by_id_opts {
        sz: mem::size_of::<bpf_get_fd_by_id_opts>(),
        open_flags: BPF_F_RDONLY,
    };

    skel = test_libbpf_get_fd_by_id_opts__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"test_libbpf_get_fd_by_id_opts__open_and_load".as_ptr(),
    ) {
        return;
    }

    ret = test_libbpf_get_fd_by_id_opts__attach(skel);
    if !ASSERT_OK(ret, c"test_libbpf_get_fd_by_id_opts__attach".as_ptr()) {
        goto_close_prog(skel, fd);
        return;
    }

    ret = bpf_map_get_info_by_fd(
        bpf_map__fd((*skel).maps.data_input),
        &mut info_m,
        &mut len,
    );
    if !ASSERT_OK(ret, c"bpf_map_get_info_by_fd".as_ptr()) {
        goto_close_prog(skel, fd);
        return;
    }

    fd = bpf_map_get_fd_by_id(info_m.id);
    if !ASSERT_LT(fd, 0, c"bpf_map_get_fd_by_id".as_ptr()) {
        goto_close_prog(skel, fd);
        return;
    }

    fd = bpf_map_get_fd_by_id_opts(info_m.id, ptr::null());
    if !ASSERT_LT(fd, 0, c"bpf_map_get_fd_by_id_opts".as_ptr()) {
        goto_close_prog(skel, fd);
        return;
    }

    fd = bpf_map_get_fd_by_id_opts(info_m.id, &fd_opts_rdonly);
    if !ASSERT_GE(fd, 0, c"bpf_map_get_fd_by_id_opts".as_ptr()) {
        goto_close_prog(skel, fd);
        return;
    }

    /* Map lookup should work with read-only fd. */
    ret = bpf_map_lookup_elem(
        fd,
        &zero as *const _ as *const c_void,
        &mut value as *mut _ as *mut c_void,
    );
    if !ASSERT_OK(ret, c"bpf_map_lookup_elem".as_ptr()) {
        goto_close_prog(skel, fd);
        return;
    }

    if !ASSERT_EQ(value as c_int, 0, c"map value mismatch".as_ptr()) {
        goto_close_prog(skel, fd);
        return;
    }

    /* Map update should not work with read-only fd. */
    ret = bpf_map_update_elem(
        fd,
        &zero as *const _ as *const c_void,
        &len as *const _ as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_LT(ret, 0, c"bpf_map_update_elem".as_ptr()) {
        goto_close_prog(skel, fd);
        return;
    }

    /* Map update should work with read-write fd. */
    ret = bpf_map_update_elem(
        bpf_map__fd((*skel).maps.data_input),
        &zero as *const _ as *const c_void,
        &len as *const _ as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_OK(ret, c"bpf_map_update_elem".as_ptr()) {
        goto_close_prog(skel, fd);
        return;
    }

    /* Prog get fd with opts set should not work (no kernel support). */
    ret = bpf_prog_get_fd_by_id_opts(0, &fd_opts_rdonly);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_prog_get_fd_by_id_opts".as_ptr()) {
        goto_close_prog(skel, fd);
        return;
    }

    /* Link get fd with opts set should not work (no kernel support). */
    ret = bpf_link_get_fd_by_id_opts(0, &fd_opts_rdonly);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_link_get_fd_by_id_opts".as_ptr()) {
        goto_close_prog(skel, fd);
        return;
    }

    /* BTF get fd with opts set should not work (no kernel support). */
    ret = bpf_btf_get_fd_by_id_opts(0, &fd_opts_rdonly);
    ASSERT_EQ(ret, -EINVAL, c"bpf_btf_get_fd_by_id_opts".as_ptr());

    goto_close_prog(skel, fd);
}

unsafe fn goto_close_prog(skel: *mut test_libbpf_get_fd_by_id_opts, fd: c_int) {
    if fd >= 0 {
        close(fd);
    }

    test_libbpf_get_fd_by_id_opts__destroy(skel);
}
