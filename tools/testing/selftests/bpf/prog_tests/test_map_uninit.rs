// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, "map_kptr.skel.h"

use core::ffi::{c_int, c_void};

type size_t = usize;
type __u8 = u8;

const BPF_ANY: u64 = 0;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_kptr__maps {
    pub pcpu_array: *mut bpf_map,
}

#[repr(C)]
pub struct map_kptr {
    pub obj: *mut bpf_object,
    pub maps: map_kptr__maps,
}

unsafe extern "C" {
    fn libbpf_num_possible_cpus() -> c_int;
    fn map_kptr__open() -> *mut map_kptr;
    fn map_kptr__load(skel: *mut map_kptr) -> c_int;
    fn map_kptr__destroy(skel: *mut map_kptr);
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool) -> c_int;
    fn bpf_map__value_size(map: *mut bpf_map) -> size_t;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
}

#[inline]
fn roundup(value: size_t, align: size_t) -> size_t {
    (value + align - 1) & !(align - 1)
}

pub unsafe fn test_map_uninit_mem_exposure() {
    let mut value_sz: size_t;
    let mut slot_sz: size_t;
    let mut lookup_sz: size_t;
    let mut tail_sz: size_t;
    let mut err: c_int;
    let mut key: c_int;
    let mut nr_cpus: c_int;
    let mut cpu: c_int;
    let mut map_fd: c_int;
    let mut value: *mut __u8 = core::ptr::null_mut();
    let mut zero: *mut __u8 = core::ptr::null_mut();
    let mut prog: *mut bpf_program;
    let mut skel: *mut map_kptr;

    nr_cpus = libbpf_num_possible_cpus();
    if !ASSERT_GT!(nr_cpus, 0, "libbpf_num_possible_cpus") {
        return;
    }

    skel = map_kptr__open();
    if !ASSERT_OK_PTR!(skel, "map_kptr__open") {
        return;
    }

    // Original C uses bpf_object__for_each_program(prog, skel->obj).
    bpf_object__for_each_program!(prog, (*skel).obj, {
        err = bpf_program__set_autoload(prog, false);
        if !ASSERT_OK!(err, "bpf_program__set_autoload") {
            goto_out!();
        }
    });

    err = map_kptr__load(skel);
    if !ASSERT_OK!(err, "map_kptr__load") {
        free(zero as *mut c_void);
        free(value as *mut c_void);
        map_kptr__destroy(skel);
        return;
    }

    value_sz = bpf_map__value_size((*skel).maps.pcpu_array);
    slot_sz = roundup(value_sz, 8);
    tail_sz = slot_sz - value_sz;
    if !ASSERT_NEQ!(tail_sz, 0, "tail_sz") {
        free(zero as *mut c_void);
        free(value as *mut c_void);
        map_kptr__destroy(skel);
        return;
    }

    lookup_sz = slot_sz * nr_cpus as size_t;
    map_fd = bpf_map__fd((*skel).maps.pcpu_array);

    value = malloc(lookup_sz) as *mut __u8;
    zero = calloc(1, tail_sz) as *mut __u8;
    if !ASSERT_OK_PTR!(value, "malloc value") || !ASSERT_OK_PTR!(zero, "calloc zero") {
        free(zero as *mut c_void);
        free(value as *mut c_void);
        map_kptr__destroy(skel);
        return;
    }

    key = 0;
    memset(value as *mut c_void, 0x2B, lookup_sz);
    err = bpf_map_update_elem(
        map_fd,
        &key as *const c_int as *const c_void,
        value as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_OK!(err, "bpf_map_update_elem") {
        free(zero as *mut c_void);
        free(value as *mut c_void);
        map_kptr__destroy(skel);
        return;
    }

    memset(value as *mut c_void, 0xFF, lookup_sz);
    err = bpf_map_lookup_elem(
        map_fd,
        &key as *const c_int as *const c_void,
        value as *mut c_void,
    );
    if !ASSERT_OK!(err, "bpf_map_lookup_elem") {
        free(zero as *mut c_void);
        free(value as *mut c_void);
        map_kptr__destroy(skel);
        return;
    }

    cpu = 0;
    while cpu < nr_cpus {
        let tail: *mut __u8 = value.add(cpu as size_t * slot_sz + value_sz);

        if !ASSERT_MEMEQ!(tail, zero, tail_sz, "zeroed tail bytes") {
            free(zero as *mut c_void);
            free(value as *mut c_void);
            map_kptr__destroy(skel);
            return;
        }
        cpu += 1;
    }

    free(zero as *mut c_void);
    free(value as *mut c_void);
    map_kptr__destroy(skel);
}
