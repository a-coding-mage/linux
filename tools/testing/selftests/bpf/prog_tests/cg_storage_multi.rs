// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2020 Google LLC.
 */

// C dependencies:
// <test_progs.h>
// <cgroup_helpers.h>
// <network_helpers.h>
// "progs/cg_storage_multi.h"
// "cg_storage_multi_egress_only.skel.h"
// "cg_storage_multi_isolated.skel.h"
// "cg_storage_multi_shared.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const PARENT_CGROUP: &[u8] = b"/cgroup_storage\0";
const CHILD_CGROUP: &[u8] = b"/cgroup_storage/child\0";

static mut duration: c_int = 0;

const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const ENOENT: c_int = 2;
const BPF_CGROUP_INET_INGRESS: c_uint = 0;
const BPF_CGROUP_INET_EGRESS: c_uint = 1;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_value {
    pub egress_pkts: u64,
    pub ingress_pkts: u64,
}

#[repr(C)]
pub struct bpf_cgroup_storage_key {
    pub cgroup_inode_id: u64,
    pub attach_type: c_uint,
}

#[repr(C)]
pub struct cg_storage_multi_egress_only_progs {
    pub egress: *mut bpf_program,
}

#[repr(C)]
pub struct cg_storage_multi_egress_only_maps {
    pub cgroup_storage: *mut bpf_map,
}

#[repr(C)]
pub struct cg_storage_multi_egress_only_bss {
    pub invocations: c_int,
}

#[repr(C)]
pub struct cg_storage_multi_egress_only {
    pub progs: cg_storage_multi_egress_only_progs,
    pub maps: cg_storage_multi_egress_only_maps,
    pub bss: *mut cg_storage_multi_egress_only_bss,
}

#[repr(C)]
pub struct cg_storage_multi_isolated_progs {
    pub egress1: *mut bpf_program,
    pub egress2: *mut bpf_program,
    pub ingress: *mut bpf_program,
}

#[repr(C)]
pub struct cg_storage_multi_isolated_maps {
    pub cgroup_storage: *mut bpf_map,
}

#[repr(C)]
pub struct cg_storage_multi_isolated_bss {
    pub invocations: c_int,
}

#[repr(C)]
pub struct cg_storage_multi_isolated {
    pub progs: cg_storage_multi_isolated_progs,
    pub maps: cg_storage_multi_isolated_maps,
    pub bss: *mut cg_storage_multi_isolated_bss,
}

#[repr(C)]
pub struct cg_storage_multi_shared_progs {
    pub egress1: *mut bpf_program,
    pub egress2: *mut bpf_program,
    pub ingress: *mut bpf_program,
}

#[repr(C)]
pub struct cg_storage_multi_shared_maps {
    pub cgroup_storage: *mut bpf_map,
}

#[repr(C)]
pub struct cg_storage_multi_shared_bss {
    pub invocations: c_int,
}

#[repr(C)]
pub struct cg_storage_multi_shared {
    pub progs: cg_storage_multi_shared_progs,
    pub maps: cg_storage_multi_shared_maps,
    pub bss: *mut cg_storage_multi_shared_bss,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn join_cgroup(cgroup_path: *const c_char) -> c_int;
    fn start_server(family: c_int, type_: c_int, addr: *const c_void, port: c_int, timeout_ms: c_int) -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;

    fn get_cgroup_id(path: *const c_char) -> u64;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn create_and_get_cgroup(path: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn cg_storage_multi_egress_only__open_and_load() -> *mut cg_storage_multi_egress_only;
    fn cg_storage_multi_egress_only__destroy(obj: *mut cg_storage_multi_egress_only);
    fn cg_storage_multi_isolated__open_and_load() -> *mut cg_storage_multi_isolated;
    fn cg_storage_multi_isolated__destroy(obj: *mut cg_storage_multi_isolated);
    fn cg_storage_multi_shared__open_and_load() -> *mut cg_storage_multi_shared;
    fn cg_storage_multi_shared__destroy(obj: *mut cg_storage_multi_shared);

    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn assert_storage(map: *mut bpf_map, key: *const c_void, expected: *mut cgroup_value) -> bool {
    let mut value = cgroup_value {
        egress_pkts: 0,
        ingress_pkts: 0,
    };
    let map_fd: c_int;

    map_fd = bpf_map__fd(map);

    if CHECK(
        bpf_map_lookup_elem(map_fd, key, &mut value as *mut _ as *mut c_void) < 0,
        c"map-lookup".as_ptr(),
        c"errno %d".as_ptr(),
        errno,
    ) {
        return true;
    }
    if CHECK(
        value.egress_pkts != (*expected).egress_pkts || value.ingress_pkts != (*expected).ingress_pkts,
        c"assert-storage".as_ptr(),
        c"storages differ".as_ptr(),
    ) {
        return true;
    }

    false
}

unsafe fn assert_storage_noexist(map: *mut bpf_map, key: *const c_void) -> bool {
    let mut value = cgroup_value {
        egress_pkts: 0,
        ingress_pkts: 0,
    };
    let map_fd: c_int;

    map_fd = bpf_map__fd(map);

    if CHECK(
        bpf_map_lookup_elem(map_fd, key, &mut value as *mut _ as *mut c_void) == 0,
        c"map-lookup".as_ptr(),
        c"succeeded, expected ENOENT".as_ptr(),
    ) {
        return true;
    }
    if CHECK(
        errno != ENOENT,
        c"map-lookup".as_ptr(),
        c"errno %d, expected ENOENT".as_ptr(),
        errno,
    ) {
        return true;
    }

    false
}

unsafe fn connect_send(cgroup_path: *const c_char) -> bool {
    let mut server_fd: c_int = -1;
    let mut client_fd: c_int = -1;
    let mut message = *b"message\0";
    let mut res = true;

    'out_clean: {
        if join_cgroup(cgroup_path) != 0 {
            break 'out_clean;
        }

        server_fd = start_server(AF_INET, SOCK_DGRAM, ptr::null(), 0, 0);
        if server_fd < 0 {
            break 'out_clean;
        }

        client_fd = connect_to_fd(server_fd, 0);
        if client_fd < 0 {
            break 'out_clean;
        }

        if send(client_fd, message.as_ptr() as *const c_void, mem::size_of_val(&message), 0) < 0 {
            break 'out_clean;
        }

        if read(server_fd, message.as_mut_ptr() as *mut c_void, mem::size_of_val(&message)) < 0 {
            break 'out_clean;
        }

        res = false;
    }

    close(client_fd);
    close(server_fd);
    res
}

unsafe fn test_egress_only(parent_cgroup_fd: c_int, child_cgroup_fd: c_int) {
    let obj: *mut cg_storage_multi_egress_only;
    let mut expected_cgroup_value: cgroup_value;
    let mut key = bpf_cgroup_storage_key {
        cgroup_inode_id: 0,
        attach_type: 0,
    };
    let mut parent_link: *mut bpf_link = ptr::null_mut();
    let mut child_link: *mut bpf_link = ptr::null_mut();
    let err: bool;

    key.attach_type = BPF_CGROUP_INET_EGRESS;

    obj = cg_storage_multi_egress_only__open_and_load();
    if CHECK(obj.is_null(), c"skel-load".as_ptr(), c"errno %d".as_ptr(), errno) {
        return;
    }

    /*
     * Attach to parent cgroup, trigger packet from child.
     * Assert that there is only one run and in that run the storage is
     * parent cgroup's storage.
     * Also assert that child cgroup's storage does not exist
     */
    'close_bpf_object: {
        parent_link = bpf_program__attach_cgroup((*obj).progs.egress, parent_cgroup_fd);
        if !ASSERT_OK_PTR(parent_link as *const c_void, c"parent-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        err = connect_send(CHILD_CGROUP.as_ptr() as *const c_char);
        if CHECK(err, c"first-connect-send".as_ptr(), c"errno %d".as_ptr(), errno) {
            break 'close_bpf_object;
        }
        if CHECK((*(*obj).bss).invocations != 1, c"first-invoke".as_ptr(), c"invocations=%d".as_ptr(), (*(*obj).bss).invocations) {
            break 'close_bpf_object;
        }
        key.cgroup_inode_id = get_cgroup_id(PARENT_CGROUP.as_ptr() as *const c_char);
        expected_cgroup_value = cgroup_value {
            egress_pkts: 1,
            ingress_pkts: 0,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
        key.cgroup_inode_id = get_cgroup_id(CHILD_CGROUP.as_ptr() as *const c_char);
        if assert_storage_noexist((*obj).maps.cgroup_storage, &key as *const _ as *const c_void) {
            break 'close_bpf_object;
        }

        /*
         * Attach to parent and child cgroup, trigger packet from child.
         * Assert that there are two additional runs, one that run with parent
         * cgroup's storage and one with child cgroup's storage.
         */
        child_link = bpf_program__attach_cgroup((*obj).progs.egress, child_cgroup_fd);
        if !ASSERT_OK_PTR(child_link as *const c_void, c"child-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        err = connect_send(CHILD_CGROUP.as_ptr() as *const c_char);
        if CHECK(err, c"second-connect-send".as_ptr(), c"errno %d".as_ptr(), errno) {
            break 'close_bpf_object;
        }
        if CHECK((*(*obj).bss).invocations != 3, c"second-invoke".as_ptr(), c"invocations=%d".as_ptr(), (*(*obj).bss).invocations) {
            break 'close_bpf_object;
        }
        key.cgroup_inode_id = get_cgroup_id(PARENT_CGROUP.as_ptr() as *const c_char);
        expected_cgroup_value = cgroup_value {
            egress_pkts: 2,
            ingress_pkts: 0,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
        key.cgroup_inode_id = get_cgroup_id(CHILD_CGROUP.as_ptr() as *const c_char);
        expected_cgroup_value = cgroup_value {
            egress_pkts: 1,
            ingress_pkts: 0,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
    }

    bpf_link__destroy(parent_link);
    bpf_link__destroy(child_link);

    cg_storage_multi_egress_only__destroy(obj);
}

unsafe fn test_isolated(parent_cgroup_fd: c_int, child_cgroup_fd: c_int) {
    let obj: *mut cg_storage_multi_isolated;
    let mut expected_cgroup_value: cgroup_value;
    let mut key = bpf_cgroup_storage_key {
        cgroup_inode_id: 0,
        attach_type: 0,
    };
    let mut parent_egress1_link: *mut bpf_link = ptr::null_mut();
    let mut parent_egress2_link: *mut bpf_link = ptr::null_mut();
    let mut child_egress1_link: *mut bpf_link = ptr::null_mut();
    let mut child_egress2_link: *mut bpf_link = ptr::null_mut();
    let mut parent_ingress_link: *mut bpf_link = ptr::null_mut();
    let mut child_ingress_link: *mut bpf_link = ptr::null_mut();
    let err: bool;

    obj = cg_storage_multi_isolated__open_and_load();
    if CHECK(obj.is_null(), c"skel-load".as_ptr(), c"errno %d".as_ptr(), errno) {
        return;
    }

    /*
     * Attach to parent cgroup, trigger packet from child.
     * Assert that there is three runs, two with parent cgroup egress and
     * one with parent cgroup ingress, stored in separate parent storages.
     * Also assert that child cgroup's storages does not exist
     */
    'close_bpf_object: {
        parent_egress1_link = bpf_program__attach_cgroup((*obj).progs.egress1, parent_cgroup_fd);
        if !ASSERT_OK_PTR(parent_egress1_link as *const c_void, c"parent-egress1-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        parent_egress2_link = bpf_program__attach_cgroup((*obj).progs.egress2, parent_cgroup_fd);
        if !ASSERT_OK_PTR(parent_egress2_link as *const c_void, c"parent-egress2-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        parent_ingress_link = bpf_program__attach_cgroup((*obj).progs.ingress, parent_cgroup_fd);
        if !ASSERT_OK_PTR(parent_ingress_link as *const c_void, c"parent-ingress-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        err = connect_send(CHILD_CGROUP.as_ptr() as *const c_char);
        if CHECK(err, c"first-connect-send".as_ptr(), c"errno %d".as_ptr(), errno) {
            break 'close_bpf_object;
        }
        if CHECK((*(*obj).bss).invocations != 3, c"first-invoke".as_ptr(), c"invocations=%d".as_ptr(), (*(*obj).bss).invocations) {
            break 'close_bpf_object;
        }
        key.cgroup_inode_id = get_cgroup_id(PARENT_CGROUP.as_ptr() as *const c_char);
        key.attach_type = BPF_CGROUP_INET_EGRESS;
        expected_cgroup_value = cgroup_value {
            egress_pkts: 2,
            ingress_pkts: 0,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
        key.attach_type = BPF_CGROUP_INET_INGRESS;
        expected_cgroup_value = cgroup_value {
            egress_pkts: 0,
            ingress_pkts: 1,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
        key.cgroup_inode_id = get_cgroup_id(CHILD_CGROUP.as_ptr() as *const c_char);
        key.attach_type = BPF_CGROUP_INET_EGRESS;
        if assert_storage_noexist((*obj).maps.cgroup_storage, &key as *const _ as *const c_void) {
            break 'close_bpf_object;
        }
        key.attach_type = BPF_CGROUP_INET_INGRESS;
        if assert_storage_noexist((*obj).maps.cgroup_storage, &key as *const _ as *const c_void) {
            break 'close_bpf_object;
        }

        /*
         * Attach to parent and child cgroup, trigger packet from child.
         * Assert that there is six additional runs, parent cgroup egresses and
         * ingress, child cgroup egresses and ingress.
         * Assert that egress and ingress storages are separate.
         */
        child_egress1_link = bpf_program__attach_cgroup((*obj).progs.egress1, child_cgroup_fd);
        if !ASSERT_OK_PTR(child_egress1_link as *const c_void, c"child-egress1-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        child_egress2_link = bpf_program__attach_cgroup((*obj).progs.egress2, child_cgroup_fd);
        if !ASSERT_OK_PTR(child_egress2_link as *const c_void, c"child-egress2-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        child_ingress_link = bpf_program__attach_cgroup((*obj).progs.ingress, child_cgroup_fd);
        if !ASSERT_OK_PTR(child_ingress_link as *const c_void, c"child-ingress-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        err = connect_send(CHILD_CGROUP.as_ptr() as *const c_char);
        if CHECK(err, c"second-connect-send".as_ptr(), c"errno %d".as_ptr(), errno) {
            break 'close_bpf_object;
        }
        if CHECK((*(*obj).bss).invocations != 9, c"second-invoke".as_ptr(), c"invocations=%d".as_ptr(), (*(*obj).bss).invocations) {
            break 'close_bpf_object;
        }
        key.cgroup_inode_id = get_cgroup_id(PARENT_CGROUP.as_ptr() as *const c_char);
        key.attach_type = BPF_CGROUP_INET_EGRESS;
        expected_cgroup_value = cgroup_value {
            egress_pkts: 4,
            ingress_pkts: 0,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
        key.attach_type = BPF_CGROUP_INET_INGRESS;
        expected_cgroup_value = cgroup_value {
            egress_pkts: 0,
            ingress_pkts: 2,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
        key.cgroup_inode_id = get_cgroup_id(CHILD_CGROUP.as_ptr() as *const c_char);
        key.attach_type = BPF_CGROUP_INET_EGRESS;
        expected_cgroup_value = cgroup_value {
            egress_pkts: 2,
            ingress_pkts: 0,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
        key.attach_type = BPF_CGROUP_INET_INGRESS;
        expected_cgroup_value = cgroup_value {
            egress_pkts: 0,
            ingress_pkts: 1,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
    }

    bpf_link__destroy(parent_egress1_link);
    bpf_link__destroy(parent_egress2_link);
    bpf_link__destroy(parent_ingress_link);
    bpf_link__destroy(child_egress1_link);
    bpf_link__destroy(child_egress2_link);
    bpf_link__destroy(child_ingress_link);

    cg_storage_multi_isolated__destroy(obj);
}

unsafe fn test_shared(parent_cgroup_fd: c_int, child_cgroup_fd: c_int) {
    let obj: *mut cg_storage_multi_shared;
    let mut expected_cgroup_value: cgroup_value;
    let mut key: u64;
    let mut parent_egress1_link: *mut bpf_link = ptr::null_mut();
    let mut parent_egress2_link: *mut bpf_link = ptr::null_mut();
    let mut child_egress1_link: *mut bpf_link = ptr::null_mut();
    let mut child_egress2_link: *mut bpf_link = ptr::null_mut();
    let mut parent_ingress_link: *mut bpf_link = ptr::null_mut();
    let mut child_ingress_link: *mut bpf_link = ptr::null_mut();
    let err: bool;

    obj = cg_storage_multi_shared__open_and_load();
    if CHECK(obj.is_null(), c"skel-load".as_ptr(), c"errno %d".as_ptr(), errno) {
        return;
    }

    /*
     * Attach to parent cgroup, trigger packet from child.
     * Assert that there is three runs, two with parent cgroup egress and
     * one with parent cgroup ingress.
     * Also assert that child cgroup's storage does not exist
     */
    'close_bpf_object: {
        parent_egress1_link = bpf_program__attach_cgroup((*obj).progs.egress1, parent_cgroup_fd);
        if !ASSERT_OK_PTR(parent_egress1_link as *const c_void, c"parent-egress1-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        parent_egress2_link = bpf_program__attach_cgroup((*obj).progs.egress2, parent_cgroup_fd);
        if !ASSERT_OK_PTR(parent_egress2_link as *const c_void, c"parent-egress2-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        parent_ingress_link = bpf_program__attach_cgroup((*obj).progs.ingress, parent_cgroup_fd);
        if !ASSERT_OK_PTR(parent_ingress_link as *const c_void, c"parent-ingress-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        err = connect_send(CHILD_CGROUP.as_ptr() as *const c_char);
        if CHECK(err, c"first-connect-send".as_ptr(), c"errno %d".as_ptr(), errno) {
            break 'close_bpf_object;
        }
        if CHECK((*(*obj).bss).invocations != 3, c"first-invoke".as_ptr(), c"invocations=%d".as_ptr(), (*(*obj).bss).invocations) {
            break 'close_bpf_object;
        }
        key = get_cgroup_id(PARENT_CGROUP.as_ptr() as *const c_char);
        expected_cgroup_value = cgroup_value {
            egress_pkts: 2,
            ingress_pkts: 1,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
        key = get_cgroup_id(CHILD_CGROUP.as_ptr() as *const c_char);
        if assert_storage_noexist((*obj).maps.cgroup_storage, &key as *const _ as *const c_void) {
            break 'close_bpf_object;
        }

        /*
         * Attach to parent and child cgroup, trigger packet from child.
         * Assert that there is six additional runs, parent cgroup egresses and
         * ingress, child cgroup egresses and ingress.
         */
        child_egress1_link = bpf_program__attach_cgroup((*obj).progs.egress1, child_cgroup_fd);
        if !ASSERT_OK_PTR(child_egress1_link as *const c_void, c"child-egress1-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        child_egress2_link = bpf_program__attach_cgroup((*obj).progs.egress2, child_cgroup_fd);
        if !ASSERT_OK_PTR(child_egress2_link as *const c_void, c"child-egress2-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        child_ingress_link = bpf_program__attach_cgroup((*obj).progs.ingress, child_cgroup_fd);
        if !ASSERT_OK_PTR(child_ingress_link as *const c_void, c"child-ingress-cg-attach".as_ptr()) {
            break 'close_bpf_object;
        }
        err = connect_send(CHILD_CGROUP.as_ptr() as *const c_char);
        if CHECK(err, c"second-connect-send".as_ptr(), c"errno %d".as_ptr(), errno) {
            break 'close_bpf_object;
        }
        if CHECK((*(*obj).bss).invocations != 9, c"second-invoke".as_ptr(), c"invocations=%d".as_ptr(), (*(*obj).bss).invocations) {
            break 'close_bpf_object;
        }
        key = get_cgroup_id(PARENT_CGROUP.as_ptr() as *const c_char);
        expected_cgroup_value = cgroup_value {
            egress_pkts: 4,
            ingress_pkts: 2,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
        key = get_cgroup_id(CHILD_CGROUP.as_ptr() as *const c_char);
        expected_cgroup_value = cgroup_value {
            egress_pkts: 2,
            ingress_pkts: 1,
        };
        if assert_storage((*obj).maps.cgroup_storage, &key as *const _ as *const c_void, &mut expected_cgroup_value) {
            break 'close_bpf_object;
        }
    }

    bpf_link__destroy(parent_egress1_link);
    bpf_link__destroy(parent_egress2_link);
    bpf_link__destroy(parent_ingress_link);
    bpf_link__destroy(child_egress1_link);
    bpf_link__destroy(child_egress2_link);
    bpf_link__destroy(child_ingress_link);

    cg_storage_multi_shared__destroy(obj);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_cg_storage_multi() {
    let mut parent_cgroup_fd: c_int = -1;
    let mut child_cgroup_fd: c_int = -1;

    'close_cgroup_fd: {
        parent_cgroup_fd = test__join_cgroup(PARENT_CGROUP.as_ptr() as *const c_char);
        if CHECK(parent_cgroup_fd < 0, c"cg-create-parent".as_ptr(), c"errno %d".as_ptr(), errno) {
            break 'close_cgroup_fd;
        }
        child_cgroup_fd = create_and_get_cgroup(CHILD_CGROUP.as_ptr() as *const c_char);
        if CHECK(child_cgroup_fd < 0, c"cg-create-child".as_ptr(), c"errno %d".as_ptr(), errno) {
            break 'close_cgroup_fd;
        }

        if test__start_subtest(c"egress_only".as_ptr()) {
            test_egress_only(parent_cgroup_fd, child_cgroup_fd);
        }

        if test__start_subtest(c"isolated".as_ptr()) {
            test_isolated(parent_cgroup_fd, child_cgroup_fd);
        }

        if test__start_subtest(c"shared".as_ptr()) {
            test_shared(parent_cgroup_fd, child_cgroup_fd);
        }
    }

    close(child_cgroup_fd);
    close(parent_cgroup_fd);
}
