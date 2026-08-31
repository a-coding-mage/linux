// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

// Translated from testing/selftests/bpf/prog_tests/skeleton.c.
// External declarations from test_progs.h, sys/mman.h, and test_skeleton.skel.h
// are intentionally left as dependencies.

use core::ffi::{c_char, c_int, c_longlong, c_uint, c_void};

#[repr(C, packed)]
pub struct s {
    pub a: c_int,
    pub b: c_longlong,
}

#[repr(C)]
pub struct test_skeleton {
    pub bss: *mut test_skeleton__bss,
    pub data: *mut test_skeleton__data,
    pub data_dyn: *mut test_skeleton__data_dyn,
    pub rodata: *mut test_skeleton__rodata,
    pub rodata_dyn: *mut test_skeleton__rodata_dyn,
    pub kconfig: *mut test_skeleton__kconfig,
    pub data_read_mostly: *mut test_skeleton__data_read_mostly,
    pub maps: test_skeleton__maps,
}

#[repr(C)]
pub struct test_skeleton__maps {
    pub rodata_dyn: *mut bpf_map,
    pub data_dyn: *mut bpf_map,
    pub data_non_mmapable: *mut bpf_map,
}

#[repr(C)]
pub struct test_skeleton__bss {
    pub in3: c_int,
    pub out3: c_int,
    pub in4: c_longlong,
    pub out4: c_longlong,
    pub in5: s,
    pub out5: s,
    pub out6: c_int,
    pub bpf_syscall: c_int,
    pub kern_ver: c_int,
    pub out_mostly_var: c_int,
    pub huge_arr: [c_int; 1],
}

#[repr(C)]
pub struct test_skeleton__data {
    pub in1: c_int,
    pub out1: c_int,
    pub in2: c_longlong,
    pub out2: c_longlong,
}

#[repr(C)]
pub struct test_skeleton__data_dyn {
    pub out_dynarr: [c_int; 4],
}

#[repr(C)]
pub struct test_skeleton__rodata_in {
    pub in6: c_int,
}

#[repr(C)]
pub struct test_skeleton__rodata {
    pub in_: test_skeleton__rodata_in,
}

#[repr(C)]
pub struct test_skeleton__rodata_dyn {
    pub in_dynarr_sz: c_int,
    pub in_dynarr: [c_int; 4],
}

#[repr(C)]
pub struct test_skeleton__kconfig {
    pub CONFIG_BPF_SYSCALL: c_int,
    pub LINUX_KERNEL_VERSION: c_int,
}

#[repr(C)]
pub struct test_skeleton__data_read_mostly {
    pub read_mostly_var: c_int,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn test_skeleton__open() -> *mut test_skeleton;
    fn test_skeleton__load(skel: *mut test_skeleton) -> c_int;
    fn test_skeleton__attach(skel: *mut test_skeleton) -> c_int;
    fn test_skeleton__destroy(skel: *mut test_skeleton);
    fn test_skeleton__elf_bytes(sz: *mut usize) -> *const c_void;

    fn bpf_map__name(map: *mut bpf_map) -> *const c_char;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__map_flags(map: *mut bpf_map) -> c_uint;

    fn usleep(usec: c_uint) -> c_int;
    fn getpagesize() -> c_int;
    fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> c_int;

    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char);
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char);
    fn ASSERT_GE<T>(actual: T, expected: T, name: *const c_char);
}

const PROT_READ: c_int = 0x1;
const MAP_SHARED: c_int = 0x01;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        ($array).len()
    };
}

pub unsafe fn test_skeleton() {
    let duration: c_int = 0;
    let mut err: c_int;
    let mut skel: *mut test_skeleton;
    let mut bss: *mut test_skeleton__bss;
    let mut data: *mut test_skeleton__data;
    let mut data_dyn: *mut test_skeleton__data_dyn;
    let mut rodata: *mut test_skeleton__rodata;
    let mut rodata_dyn: *mut test_skeleton__rodata_dyn;
    let mut kcfg: *mut test_skeleton__kconfig;
    let mut elf_bytes: *const c_void;
    let mut elf_bytes_sz: usize = 0;
    let mut m: *mut c_void;
    let mut i: c_int;
    let mut fd: c_int;

    skel = test_skeleton__open();
    if CHECK(
        skel.is_null(),
        cstr!("skel_open"),
        cstr!("failed to open skeleton\n"),
    ) {
        return;
    }

    if CHECK(
        !(*skel).kconfig.is_null(),
        cstr!("skel_kconfig"),
        cstr!("kconfig is mmaped()!\n"),
    ) {
        goto_cleanup(skel);
        return;
    }

    bss = (*skel).bss;
    data = (*skel).data;
    data_dyn = (*skel).data_dyn;
    rodata = (*skel).rodata;
    rodata_dyn = (*skel).rodata_dyn;

    ASSERT_STREQ(
        bpf_map__name((*skel).maps.rodata_dyn),
        cstr!(".rodata.dyn"),
        cstr!("rodata_dyn_name"),
    );
    ASSERT_STREQ(
        bpf_map__name((*skel).maps.data_dyn),
        cstr!(".data.dyn"),
        cstr!("data_dyn_name"),
    );

    /* validate values are pre-initialized correctly */
    CHECK((*data).in1 != -1, cstr!("in1"), cstr!("got %d != exp %d\n"), (*data).in1, -1);
    CHECK((*data).out1 != -1, cstr!("out1"), cstr!("got %d != exp %d\n"), (*data).out1, -1);
    CHECK((*data).in2 != -1, cstr!("in2"), cstr!("got %lld != exp %lld\n"), (*data).in2, -1i64);
    CHECK((*data).out2 != -1, cstr!("out2"), cstr!("got %lld != exp %lld\n"), (*data).out2, -1i64);

    CHECK((*bss).in3 != 0, cstr!("in3"), cstr!("got %d != exp %d\n"), (*bss).in3, 0);
    CHECK((*bss).out3 != 0, cstr!("out3"), cstr!("got %d != exp %d\n"), (*bss).out3, 0);
    CHECK((*bss).in4 != 0, cstr!("in4"), cstr!("got %lld != exp %lld\n"), (*bss).in4, 0i64);
    CHECK((*bss).out4 != 0, cstr!("out4"), cstr!("got %lld != exp %lld\n"), (*bss).out4, 0i64);

    CHECK((*rodata).in_.in6 != 0, cstr!("in6"), cstr!("got %d != exp %d\n"), (*rodata).in_.in6, 0);
    CHECK((*bss).out6 != 0, cstr!("out6"), cstr!("got %d != exp %d\n"), (*bss).out6, 0);

    ASSERT_EQ((*rodata_dyn).in_dynarr_sz, 0, cstr!("in_dynarr_sz"));
    i = 0;
    while i < 4 {
        ASSERT_EQ((*rodata_dyn).in_dynarr[i as usize], -(i + 1), cstr!("in_dynarr"));
        i += 1;
    }
    i = 0;
    while i < 4 {
        ASSERT_EQ((*data_dyn).out_dynarr[i as usize], i + 1, cstr!("out_dynarr"));
        i += 1;
    }

    /* validate we can pre-setup global variables, even in .bss */
    (*data).in1 = 10;
    (*data).in2 = 11;
    (*bss).in3 = 12;
    (*bss).in4 = 13;
    (*rodata).in_.in6 = 14;

    (*rodata_dyn).in_dynarr_sz = 4;
    i = 0;
    while i < 4 {
        (*rodata_dyn).in_dynarr[i as usize] = i + 10;
        i += 1;
    }

    err = test_skeleton__load(skel);
    if CHECK(err != 0, cstr!("skel_load"), cstr!("failed to load skeleton: %d\n"), err) {
        goto_cleanup(skel);
        return;
    }

    /* validate pre-setup values are still there */
    CHECK((*data).in1 != 10, cstr!("in1"), cstr!("got %d != exp %d\n"), (*data).in1, 10);
    CHECK((*data).in2 != 11, cstr!("in2"), cstr!("got %lld != exp %lld\n"), (*data).in2, 11i64);
    CHECK((*bss).in3 != 12, cstr!("in3"), cstr!("got %d != exp %d\n"), (*bss).in3, 12);
    CHECK((*bss).in4 != 13, cstr!("in4"), cstr!("got %lld != exp %lld\n"), (*bss).in4, 13i64);
    CHECK((*rodata).in_.in6 != 14, cstr!("in6"), cstr!("got %d != exp %d\n"), (*rodata).in_.in6, 14);

    ASSERT_EQ((*rodata_dyn).in_dynarr_sz, 4, cstr!("in_dynarr_sz"));
    i = 0;
    while i < 4 {
        ASSERT_EQ((*rodata_dyn).in_dynarr[i as usize], i + 10, cstr!("in_dynarr"));
        i += 1;
    }

    /* now set new values and attach to get them into outX variables */
    (*data).in1 = 1;
    (*data).in2 = 2;
    (*bss).in3 = 3;
    (*bss).in4 = 4;
    (*bss).in5.a = 5;
    (*bss).in5.b = 6;
    kcfg = (*skel).kconfig;

    (*(*skel).data_read_mostly).read_mostly_var = 123;

    err = test_skeleton__attach(skel);
    if CHECK(
        err != 0,
        cstr!("skel_attach"),
        cstr!("skeleton attach failed: %d\n"),
        err,
    ) {
        goto_cleanup(skel);
        return;
    }

    /* trigger tracepoint */
    usleep(1);

    CHECK((*data).out1 != 1, cstr!("res1"), cstr!("got %d != exp %d\n"), (*data).out1, 1);
    CHECK((*data).out2 != 2, cstr!("res2"), cstr!("got %lld != exp %d\n"), (*data).out2, 2);
    CHECK((*bss).out3 != 3, cstr!("res3"), cstr!("got %d != exp %d\n"), (*bss).out3 as c_int, 3);
    CHECK((*bss).out4 != 4, cstr!("res4"), cstr!("got %lld != exp %d\n"), (*bss).out4, 4);
    CHECK((*bss).out5.a != 5, cstr!("res5"), cstr!("got %d != exp %d\n"), (*bss).out5.a, 5);
    CHECK((*bss).out5.b != 6, cstr!("res6"), cstr!("got %lld != exp %d\n"), (*bss).out5.b, 6);
    CHECK((*bss).out6 != 14, cstr!("res7"), cstr!("got %d != exp %d\n"), (*bss).out6, 14);

    CHECK(
        (*bss).bpf_syscall != (*kcfg).CONFIG_BPF_SYSCALL,
        cstr!("ext1"),
        cstr!("got %d != exp %d\n"),
        (*bss).bpf_syscall,
        (*kcfg).CONFIG_BPF_SYSCALL,
    );
    CHECK(
        (*bss).kern_ver != (*kcfg).LINUX_KERNEL_VERSION,
        cstr!("ext2"),
        cstr!("got %d != exp %d\n"),
        (*bss).kern_ver,
        (*kcfg).LINUX_KERNEL_VERSION,
    );

    i = 0;
    while i < 4 {
        ASSERT_EQ((*data_dyn).out_dynarr[i as usize], i + 10, cstr!("out_dynarr"));
        i += 1;
    }

    ASSERT_EQ((*(*skel).bss).out_mostly_var, 123, cstr!("out_mostly_var"));

    ASSERT_EQ(
        (*bss).huge_arr[ARRAY_SIZE!((*bss).huge_arr) - 1],
        123,
        cstr!("huge_arr"),
    );

    fd = bpf_map__fd((*skel).maps.data_non_mmapable);
    m = mmap(core::ptr::null_mut(), getpagesize() as usize, PROT_READ, MAP_SHARED, fd, 0);
    if !ASSERT_EQ(m, MAP_FAILED, cstr!("unexpected_mmap_success")) {
        munmap(m, getpagesize() as usize);
    }

    ASSERT_EQ(
        bpf_map__map_flags((*skel).maps.data_non_mmapable),
        0u32,
        cstr!("non_mmap_flags"),
    );

    elf_bytes = test_skeleton__elf_bytes(&mut elf_bytes_sz);
    ASSERT_OK_PTR(elf_bytes, cstr!("elf_bytes"));
    ASSERT_GE(elf_bytes_sz, 0usize, cstr!("elf_bytes_sz"));

    goto_cleanup(skel);

    let _ = duration;
}

unsafe fn goto_cleanup(skel: *mut test_skeleton) {
    test_skeleton__destroy(skel);
}
