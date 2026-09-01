// SPDX-License-Identifier: GPL-2.0
//
// Translated from C. Original dependency intent:
// <linux/compiler.h>, <linux/bitmap.h>, <linux/kernel.h>, <linux/zalloc.h>,
// <perf/cpumap.h>, <internal/cpumap.h>, "debug.h", "env.h", "mem2node.h",
// and "tests.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong};

#[repr(C)]
pub struct node {
    pub node: c_int,
    pub map: *const c_char,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mem2node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct memory_node {
    pub node: c_int,
    pub size: u64,
    pub set: *mut c_ulong,
}

#[repr(C)]
pub struct perf_env {
    pub memory_nodes: *mut memory_node,
    pub nr_memory_nodes: c_uint,
    pub memory_bsize: u64,
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_int) -> perf_cpu;

    fn bitmap_zalloc(nbits: c_uint) -> *mut c_ulong;
    fn __set_bit(nr: c_int, addr: *mut c_ulong);
    fn free(ptr: *mut core::ffi::c_void);
    fn zfree(ptr: *mut *mut c_ulong);

    fn mem2node__init(map: *mut mem2node, env: *mut perf_env) -> c_int;
    fn mem2node__node(map: *mut mem2node, addr: u64) -> c_int;
    fn mem2node__exit(map: *mut mem2node);

    fn TEST_ASSERT_VAL(msg: *const c_char, cond: c_int) -> c_int;
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

static mut test_nodes: [node; 3] = [
    node {
        node: 0,
        map: c_str!("0"),
    },
    node {
        node: 1,
        map: c_str!("1-2"),
    },
    node {
        node: 3,
        map: c_str!("5-7,9"),
    },
];

macro_rules! T {
    ($msg:literal, $cond:expr) => {{
        let __cond = $cond;
        if unsafe { TEST_ASSERT_VAL(c_str!($msg), (__cond != 0) as c_int) } != 0 {
            return -1;
        }
    }};
}

unsafe fn get_bitmap(str_: *const c_char, nbits: c_int) -> *mut c_ulong {
    let map = unsafe { perf_cpu_map__new(str_) };
    let mut bm: *mut c_ulong = core::ptr::null_mut();

    bm = unsafe { bitmap_zalloc(nbits as c_uint) };

    if !map.is_null() && !bm.is_null() {
        let mut i: c_uint = 0;

        while i < unsafe { perf_cpu_map__nr(map) } as c_uint {
            let cpu = unsafe { perf_cpu_map__cpu(map, i as c_int) };
            unsafe { __set_bit(cpu.cpu, bm) };
            i = i.wrapping_add(1);
        }
    }

    if !map.is_null() {
        unsafe { perf_cpu_map__put(map) };
    } else {
        unsafe { free(bm as *mut core::ffi::c_void) };
    }

    if !bm.is_null() && !map.is_null() {
        bm
    } else {
        core::ptr::null_mut()
    }
}

pub unsafe extern "C" fn test__mem2node(
    t: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = t;
    let _ = subtest;

    let mut map = core::mem::MaybeUninit::<mem2node>::uninit();
    let mut nodes: [memory_node; 3] = [
        memory_node {
            node: 0,
            size: 0,
            set: core::ptr::null_mut(),
        },
        memory_node {
            node: 0,
            size: 0,
            set: core::ptr::null_mut(),
        },
        memory_node {
            node: 0,
            size: 0,
            set: core::ptr::null_mut(),
        },
    ];
    let mut env = perf_env {
        memory_nodes: nodes.as_mut_ptr(),
        nr_memory_nodes: array_size(&nodes),
        memory_bsize: 0x100,
    };
    let mut i: c_uint;

    i = 0;
    while i < array_size(&nodes) {
        nodes[i as usize].node = unsafe { test_nodes[i as usize].node };
        nodes[i as usize].size = 10;

        nodes[i as usize].set =
            unsafe { get_bitmap(test_nodes[i as usize].map, 10) };
        T!("failed: alloc bitmap", !nodes[i as usize].set.is_null() as c_int);

        i = i.wrapping_add(1);
    }

    T!(
        "failed: mem2node__init",
        (unsafe { mem2node__init(map.as_mut_ptr(), &mut env) } == 0) as c_int
    );
    T!(
        "failed: mem2node__node",
        (0 == unsafe { mem2node__node(map.as_mut_ptr(), 0x50) }) as c_int
    );
    T!(
        "failed: mem2node__node",
        (1 == unsafe { mem2node__node(map.as_mut_ptr(), 0x100) }) as c_int
    );
    T!(
        "failed: mem2node__node",
        (1 == unsafe { mem2node__node(map.as_mut_ptr(), 0x250) }) as c_int
    );
    T!(
        "failed: mem2node__node",
        (3 == unsafe { mem2node__node(map.as_mut_ptr(), 0x500) }) as c_int
    );
    T!(
        "failed: mem2node__node",
        (3 == unsafe { mem2node__node(map.as_mut_ptr(), 0x650) }) as c_int
    );
    T!(
        "failed: mem2node__node",
        (-1 == unsafe { mem2node__node(map.as_mut_ptr(), 0x450) }) as c_int
    );
    T!(
        "failed: mem2node__node",
        (-1 == unsafe { mem2node__node(map.as_mut_ptr(), 0x1050) }) as c_int
    );

    i = 0;
    while i < array_size(&nodes) {
        unsafe { zfree(&mut nodes[i as usize].set) };
        i = i.wrapping_add(1);
    }

    unsafe { mem2node__exit(map.as_mut_ptr()) };
    0
}

// DEFINE_SUITE("mem2node", mem2node);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
