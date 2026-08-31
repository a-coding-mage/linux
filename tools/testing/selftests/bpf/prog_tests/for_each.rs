// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
// C dependencies translated as external symbols:
// <test_progs.h>, <network_helpers.h>,
// "for_each_hash_map_elem.skel.h", "for_each_array_map_elem.skel.h",
// "for_each_map_elem_write_key.skel.h", "for_each_multi_maps.skel.h",
// "for_each_hash_modify.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type __u32 = u32;
type __u64 = u64;

const BPF_ANY: __u64 = 0;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: size_t,
    pub data_in: *const c_void,
    pub data_size_in: __u32,
    pub repeat: __u32,
    pub duration: __u32,
    pub retval: __u32,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct for_each_hash_map_elem_maps {
    pub hashmap: *mut bpf_map,
    pub percpu_map: *mut bpf_map,
}

#[repr(C)]
pub struct for_each_hash_map_elem_progs {
    pub test_pkt_access: *mut bpf_program,
}

#[repr(C)]
pub struct for_each_hash_map_elem_bss {
    pub hashmap_output: __u64,
    pub hashmap_elems: __u32,
    pub percpu_called: __u32,
    pub cpu: __u32,
    pub percpu_map_elems: __u32,
    pub percpu_key: __u32,
    pub percpu_val: __u64,
    pub percpu_output: __u64,
}

#[repr(C)]
pub struct for_each_hash_map_elem {
    pub maps: for_each_hash_map_elem_maps,
    pub progs: for_each_hash_map_elem_progs,
    pub bss: *mut for_each_hash_map_elem_bss,
}

#[repr(C)]
pub struct for_each_array_map_elem_maps {
    pub arraymap: *mut bpf_map,
    pub percpu_map: *mut bpf_map,
}

#[repr(C)]
pub struct for_each_array_map_elem_progs {
    pub test_pkt_access: *mut bpf_program,
}

#[repr(C)]
pub struct for_each_array_map_elem_bss {
    pub arraymap_output: __u64,
    pub cpu: __u32,
    pub percpu_val: __u64,
}

#[repr(C)]
pub struct for_each_array_map_elem {
    pub maps: for_each_array_map_elem_maps,
    pub progs: for_each_array_map_elem_progs,
    pub bss: *mut for_each_array_map_elem_bss,
}

#[repr(C)]
pub struct for_each_map_elem_write_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct for_each_multi_maps_maps {
    pub arraymap: *mut bpf_map,
    pub hashmap: *mut bpf_map,
}

#[repr(C)]
pub struct for_each_multi_maps_progs {
    pub test_pkt_access: *mut bpf_program,
}

#[repr(C)]
pub struct for_each_multi_maps_bss {
    pub data_output: __u64,
    pub use_array: __u32,
}

#[repr(C)]
pub struct for_each_multi_maps {
    pub maps: for_each_multi_maps_maps,
    pub progs: for_each_multi_maps_progs,
    pub bss: *mut for_each_multi_maps_bss,
}

#[repr(C)]
pub struct for_each_hash_modify_maps {
    pub hashmap: *mut bpf_map,
}

#[repr(C)]
pub struct for_each_hash_modify_progs {
    pub test_pkt_access: *mut bpf_program,
}

#[repr(C)]
pub struct for_each_hash_modify {
    pub maps: for_each_hash_modify_maps,
    pub progs: for_each_hash_modify_progs,
}

unsafe extern "C" {
    static pkt_v4: c_void;
    static mut errno: c_int;

    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn bpf_map__max_entries(map: *const bpf_map) -> c_uint;
    fn bpf_map__update_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: size_t,
        value: *const c_void,
        value_sz: size_t,
        flags: __u64,
    ) -> c_int;
    fn bpf_map__lookup_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: size_t,
        value: *mut c_void,
        value_sz: size_t,
        flags: __u64,
    ) -> c_int;
    fn bpf_num_possible_cpus() -> c_int;
    fn bpf_program__fd(prog: *const bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: __u64, expected: __u64, name: *const c_char) -> bool;
    fn ASSERT_LT(actual: __u64, expected: __u64, name: *const c_char) -> bool;
    fn CHECK(
        condition: bool,
        tag: *const c_char,
        format: *const c_char,
        ...
    ) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn for_each_hash_map_elem__open_and_load() -> *mut for_each_hash_map_elem;
    fn for_each_hash_map_elem__destroy(skel: *mut for_each_hash_map_elem);
    fn for_each_array_map_elem__open_and_load() -> *mut for_each_array_map_elem;
    fn for_each_array_map_elem__destroy(skel: *mut for_each_array_map_elem);
    fn for_each_map_elem_write_key__open_and_load() -> *mut for_each_map_elem_write_key;
    fn for_each_map_elem_write_key__destroy(skel: *mut for_each_map_elem_write_key);
    fn for_each_multi_maps__open_and_load() -> *mut for_each_multi_maps;
    fn for_each_multi_maps__destroy(skel: *mut for_each_multi_maps);
    fn for_each_hash_modify__open_and_load() -> *mut for_each_hash_modify;
    fn for_each_hash_modify__destroy(skel: *mut for_each_hash_modify);
}

static mut duration: c_uint = 0;

unsafe fn test_hash_map() {
    let mut i: c_int;
    let mut err: c_int;
    let max_entries: c_int;
    let skel: *mut for_each_hash_map_elem;
    let mut percpu_valbuf: *mut __u64 = core::ptr::null_mut();
    let percpu_val_sz: size_t;
    let mut key: __u32;
    let num_cpus: __u32;
    let mut val: __u64;
    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: &pkt_v4 as *const _ as *const c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as __u32,
        repeat: 1,
        duration: 0,
        retval: 0,
    };

    skel = for_each_hash_map_elem__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"for_each_hash_map_elem__open_and_load".as_ptr()) {
        return;
    }

    max_entries = bpf_map__max_entries((*skel).maps.hashmap) as c_int;
    i = 0;
    while i < max_entries {
        key = i as __u32;
        val = (i + 1) as __u64;
        err = bpf_map__update_elem(
            (*skel).maps.hashmap,
            &key as *const _ as *const c_void,
            core::mem::size_of_val(&key),
            &val as *const _ as *const c_void,
            core::mem::size_of_val(&val),
            BPF_ANY,
        );
        if !ASSERT_OK(err, c"map_update".as_ptr()) {
            goto_out_hash_map(percpu_valbuf, skel);
            return;
        }
        i += 1;
    }

    num_cpus = bpf_num_possible_cpus() as __u32;
    percpu_val_sz = core::mem::size_of::<__u64>() * num_cpus as size_t;
    percpu_valbuf = malloc(percpu_val_sz) as *mut __u64;
    if !ASSERT_OK_PTR(percpu_valbuf as *const c_void, c"percpu_valbuf".as_ptr()) {
        goto_out_hash_map(percpu_valbuf, skel);
        return;
    }

    key = 1;
    i = 0;
    while i < num_cpus as c_int {
        *percpu_valbuf.add(i as usize) = (i + 1) as __u64;
        i += 1;
    }
    err = bpf_map__update_elem(
        (*skel).maps.percpu_map,
        &key as *const _ as *const c_void,
        core::mem::size_of_val(&key),
        percpu_valbuf as *const c_void,
        percpu_val_sz,
        BPF_ANY,
    );
    if !ASSERT_OK(err, c"percpu_map_update".as_ptr()) {
        goto_out_hash_map(percpu_valbuf, skel);
        return;
    }

    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.test_pkt_access), &mut topts);
    duration = topts.duration;
    if CHECK(
        err != 0 || topts.retval != 0,
        c"ipv4".as_ptr(),
        c"err %d errno %d retval %d\n".as_ptr(),
        err,
        errno,
        topts.retval,
    ) {
        goto_out_hash_map(percpu_valbuf, skel);
        return;
    }

    ASSERT_EQ((*(*skel).bss).hashmap_output, 4, c"hashmap_output".as_ptr());
    ASSERT_EQ((*(*skel).bss).hashmap_elems as __u64, max_entries as __u64, c"hashmap_elems".as_ptr());

    key = 1;
    err = bpf_map__lookup_elem(
        (*skel).maps.hashmap,
        &key as *const _ as *const c_void,
        core::mem::size_of_val(&key),
        &mut val as *mut _ as *mut c_void,
        core::mem::size_of_val(&val),
        0,
    );
    ASSERT_ERR(err, c"hashmap_lookup".as_ptr());

    ASSERT_EQ((*(*skel).bss).percpu_called as __u64, 1, c"percpu_called".as_ptr());
    ASSERT_LT((*(*skel).bss).cpu as __u64, num_cpus as __u64, c"num_cpus".as_ptr());
    ASSERT_EQ((*(*skel).bss).percpu_map_elems as __u64, 1, c"percpu_map_elems".as_ptr());
    ASSERT_EQ((*(*skel).bss).percpu_key as __u64, 1, c"percpu_key".as_ptr());
    ASSERT_EQ((*(*skel).bss).percpu_val, ((*(*skel).bss).cpu + 1) as __u64, c"percpu_val".as_ptr());
    ASSERT_EQ((*(*skel).bss).percpu_output, 100, c"percpu_output".as_ptr());

    goto_out_hash_map(percpu_valbuf, skel);
}

unsafe fn goto_out_hash_map(percpu_valbuf: *mut __u64, skel: *mut for_each_hash_map_elem) {
    free(percpu_valbuf as *mut c_void);
    for_each_hash_map_elem__destroy(skel);
}

unsafe fn test_array_map() {
    let mut key: __u32;
    let num_cpus: __u32;
    let max_entries: __u32;
    let mut i: c_int;
    let mut err: c_int;
    let skel: *mut for_each_array_map_elem;
    let mut percpu_valbuf: *mut __u64 = core::ptr::null_mut();
    let percpu_val_sz: size_t;
    let mut val: __u64;
    let mut expected_total: __u64;
    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: &pkt_v4 as *const _ as *const c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as __u32,
        repeat: 1,
        duration: 0,
        retval: 0,
    };

    skel = for_each_array_map_elem__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"for_each_array_map_elem__open_and_load".as_ptr()) {
        return;
    }

    expected_total = 0;
    max_entries = bpf_map__max_entries((*skel).maps.arraymap);
    i = 0;
    while i < max_entries as c_int {
        key = i as __u32;
        val = (i + 1) as __u64;
        /* skip the last iteration for expected total */
        if i != max_entries as c_int - 1 {
            expected_total += val;
        }
        err = bpf_map__update_elem(
            (*skel).maps.arraymap,
            &key as *const _ as *const c_void,
            core::mem::size_of_val(&key),
            &val as *const _ as *const c_void,
            core::mem::size_of_val(&val),
            BPF_ANY,
        );
        if !ASSERT_OK(err, c"map_update".as_ptr()) {
            goto_out_array_map(percpu_valbuf, skel);
            return;
        }
        i += 1;
    }

    num_cpus = bpf_num_possible_cpus() as __u32;
    percpu_val_sz = core::mem::size_of::<__u64>() * num_cpus as size_t;
    percpu_valbuf = malloc(percpu_val_sz) as *mut __u64;
    if !ASSERT_OK_PTR(percpu_valbuf as *const c_void, c"percpu_valbuf".as_ptr()) {
        goto_out_array_map(percpu_valbuf, skel);
        return;
    }

    key = 0;
    i = 0;
    while i < num_cpus as c_int {
        *percpu_valbuf.add(i as usize) = (i + 1) as __u64;
        i += 1;
    }
    err = bpf_map__update_elem(
        (*skel).maps.percpu_map,
        &key as *const _ as *const c_void,
        core::mem::size_of_val(&key),
        percpu_valbuf as *const c_void,
        percpu_val_sz,
        BPF_ANY,
    );
    if !ASSERT_OK(err, c"percpu_map_update".as_ptr()) {
        goto_out_array_map(percpu_valbuf, skel);
        return;
    }

    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.test_pkt_access), &mut topts);
    duration = topts.duration;
    if CHECK(
        err != 0 || topts.retval != 0,
        c"ipv4".as_ptr(),
        c"err %d errno %d retval %d\n".as_ptr(),
        err,
        errno,
        topts.retval,
    ) {
        goto_out_array_map(percpu_valbuf, skel);
        return;
    }

    ASSERT_EQ((*(*skel).bss).arraymap_output, expected_total, c"array_output".as_ptr());
    ASSERT_EQ(((*(*skel).bss).cpu + 1) as __u64, (*(*skel).bss).percpu_val, c"percpu_val".as_ptr());

    goto_out_array_map(percpu_valbuf, skel);
}

unsafe fn goto_out_array_map(percpu_valbuf: *mut __u64, skel: *mut for_each_array_map_elem) {
    free(percpu_valbuf as *mut c_void);
    for_each_array_map_elem__destroy(skel);
}

unsafe fn test_write_map_key() {
    let skel: *mut for_each_map_elem_write_key;

    skel = for_each_map_elem_write_key__open_and_load();
    if !ASSERT_ERR_PTR(skel as *const c_void, c"for_each_map_elem_write_key__open_and_load".as_ptr()) {
        for_each_map_elem_write_key__destroy(skel);
    }
}

unsafe fn test_multi_maps() {
    let skel: *mut for_each_multi_maps;
    let mut val: __u64;
    let mut array_total: __u64;
    let mut hash_total: __u64;
    let mut key: __u32;
    let mut max_entries: __u32;
    let mut i: c_int;
    let mut err: c_int;

    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: &pkt_v4 as *const _ as *const c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as __u32,
        repeat: 1,
        duration: 0,
        retval: 0,
    };

    skel = for_each_multi_maps__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"for_each_multi_maps__open_and_load".as_ptr()) {
        return;
    }

    array_total = 0;
    max_entries = bpf_map__max_entries((*skel).maps.arraymap);
    i = 0;
    while i < max_entries as c_int {
        key = i as __u32;
        val = (i + 1) as __u64;
        array_total += val;
        err = bpf_map__update_elem(
            (*skel).maps.arraymap,
            &key as *const _ as *const c_void,
            core::mem::size_of_val(&key),
            &val as *const _ as *const c_void,
            core::mem::size_of_val(&val),
            BPF_ANY,
        );
        if !ASSERT_OK(err, c"array_map_update".as_ptr()) {
            for_each_multi_maps__destroy(skel);
            return;
        }
        i += 1;
    }

    hash_total = 0;
    max_entries = bpf_map__max_entries((*skel).maps.hashmap);
    i = 0;
    while i < max_entries as c_int {
        key = (i + 100) as __u32;
        val = (i + 1) as __u64;
        hash_total += val;
        err = bpf_map__update_elem(
            (*skel).maps.hashmap,
            &key as *const _ as *const c_void,
            core::mem::size_of_val(&key),
            &val as *const _ as *const c_void,
            core::mem::size_of_val(&val),
            BPF_ANY,
        );
        if !ASSERT_OK(err, c"hash_map_update".as_ptr()) {
            for_each_multi_maps__destroy(skel);
            return;
        }
        i += 1;
    }

    (*(*skel).bss).data_output = 0;
    (*(*skel).bss).use_array = 1;
    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.test_pkt_access), &mut topts);
    ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());
    ASSERT_OK(topts.retval as c_int, c"retval".as_ptr());
    ASSERT_EQ((*(*skel).bss).data_output, array_total, c"array output".as_ptr());

    (*(*skel).bss).data_output = 0;
    (*(*skel).bss).use_array = 0;
    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.test_pkt_access), &mut topts);
    ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());
    ASSERT_OK(topts.retval as c_int, c"retval".as_ptr());
    ASSERT_EQ((*(*skel).bss).data_output, hash_total, c"hash output".as_ptr());

    for_each_multi_maps__destroy(skel);
}

unsafe fn test_hash_modify() {
    let skel: *mut for_each_hash_modify;
    let max_entries: c_int;
    let mut i: c_int;
    let mut err: c_int;
    let mut key: __u64;
    let mut val: __u64;

    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: &pkt_v4 as *const _ as *const c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as __u32,
        repeat: 1,
        duration: 0,
        retval: 0,
    };

    skel = for_each_hash_modify__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"for_each_hash_modify__open_and_load".as_ptr()) {
        return;
    }

    max_entries = bpf_map__max_entries((*skel).maps.hashmap) as c_int;
    i = 0;
    while i < max_entries {
        key = i as __u64;
        val = i as __u64;
        err = bpf_map__update_elem(
            (*skel).maps.hashmap,
            &key as *const _ as *const c_void,
            core::mem::size_of_val(&key),
            &val as *const _ as *const c_void,
            core::mem::size_of_val(&val),
            BPF_ANY,
        );
        if !ASSERT_OK(err, c"map_update".as_ptr()) {
            for_each_hash_modify__destroy(skel);
            return;
        }
        i += 1;
    }

    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.test_pkt_access), &mut topts);
    ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());
    ASSERT_OK(topts.retval as c_int, c"retval".as_ptr());

    for_each_hash_modify__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_for_each() {
    if test__start_subtest(c"hash_map".as_ptr()) {
        test_hash_map();
    }
    if test__start_subtest(c"array_map".as_ptr()) {
        test_array_map();
    }
    if test__start_subtest(c"write_map_key".as_ptr()) {
        test_write_map_key();
    }
    if test__start_subtest(c"multi_maps".as_ptr()) {
        test_multi_maps();
    }
    if test__start_subtest(c"hash_modify".as_ptr()) {
        test_hash_modify();
    }
}
