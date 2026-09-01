// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from testing/memblock/tests/common.c.
// Dependencies originally provided by:
// "tests/common.h", <string.h>, <getopt.h>, <linux/memory_hotplug.h>,
// and <linux/build_bug.h>.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void, VaList};
use core::mem::size_of;
use core::ptr;

const PREFIXES_MAX: usize = 15;
const DELIM: *const c_char = b": \0".as_ptr() as *const c_char;
const BASIS: phys_addr_t = 10000;

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

unsafe impl Sync for option {}

unsafe extern "C" {
    static mut memblock: memblock;
    static mut errno: c_int;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn vprintf(fmt: *const c_char, arg: VaList<'_, '_>) -> c_int;
    fn exit(status: c_int) -> !;
    fn getopt_long_only(
        argc: c_int,
        argv: *mut *mut c_char,
        shortopts: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    fn memblock_add(base: phys_addr_t, size: phys_addr_t) -> c_int;
    fn memblock_add_node(
        base: phys_addr_t,
        size: phys_addr_t,
        nid: c_int,
        flags: c_int,
    ) -> c_int;
    fn movable_node_is_enabled() -> bool;
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
}

static mut memory_block: test_memory = test_memory {
    base: ptr::null_mut(),
};
static mut prefixes: [*const c_char; PREFIXES_MAX] = [ptr::null(); PREFIXES_MAX];
static mut nr_prefixes: c_int = 0;

static short_opts: *const c_char = b"hmv\0".as_ptr() as *const c_char;
static long_opts: [option; 4] = [
    option {
        name: b"help\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'h' as c_int,
    },
    option {
        name: b"movable-node\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'm' as c_int,
    },
    option {
        name: b"verbose\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: b'v' as c_int,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

static help_opts: [*const c_char; 3] = [
    b"display this help message and exit\0".as_ptr() as *const c_char,
    b"disallow allocations from regions marked as hotplugged\n\t\t\tby simulating enabling the \"movable_node\" kernel\n\t\t\tparameter\0"
        .as_ptr() as *const c_char,
    b"enable verbose output, which includes the name of the\n\t\t\tmemblock function being tested, the name of the test,\n\t\t\tand whether the test passed or failed.\0"
        .as_ptr() as *const c_char,
];

static mut verbose: c_int = 0;

/* sets global variable returned by movable_node_is_enabled() stub */
#[no_mangle]
pub static mut movable_node_enabled: bool = false;

#[no_mangle]
pub unsafe extern "C" fn reset_memblock_regions() {
    unsafe {
        memset(
            memblock.memory.regions as *mut c_void,
            0,
            memblock.memory.cnt as usize * size_of::<memblock_region>(),
        );
        memblock.memory.cnt = 0;
        memblock.memory.max = INIT_MEMBLOCK_REGIONS;
        memblock.memory.total_size = 0;

        memset(
            memblock.reserved.regions as *mut c_void,
            0,
            memblock.reserved.cnt as usize * size_of::<memblock_region>(),
        );
        memblock.reserved.cnt = 0;
        memblock.reserved.max = INIT_MEMBLOCK_RESERVED_REGIONS;
        memblock.reserved.total_size = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn reset_memblock_attributes() {
    unsafe {
        memblock.memory.name = b"memory\0".as_ptr() as *const c_char;
        memblock.reserved.name = b"reserved\0".as_ptr() as *const c_char;
        memblock.bottom_up = false;
        memblock.current_limit = MEMBLOCK_ALLOC_ANYWHERE;
    }
}

#[inline]
unsafe fn fill_memblock() {
    unsafe {
        memset((*(&raw mut memory_block)).base, 1, PHYS_MEM_SIZE as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn setup_memblock() {
    unsafe {
        reset_memblock_regions();
        memblock_add((*(&raw mut memory_block)).base as phys_addr_t, MEM_SIZE);
        fill_memblock();
    }
}

/**
 * setup_numa_memblock:
 * Set up a memory layout with multiple NUMA nodes in a previously allocated
 * dummy physical memory.
 * @node_fracs: an array representing the fraction of MEM_SIZE contained in
 *              each node in basis point units (one hundredth of 1% or 1/10000).
 *              For example, if node 0 should contain 1/8 of MEM_SIZE,
 *              node_fracs[0] = 1250.
 *
 * The nids will be set to 0 through NUMA_NODES - 1.
 */
#[no_mangle]
pub unsafe extern "C" fn setup_numa_memblock(node_fracs: *const c_uint) {
    unsafe {
        let mut base: phys_addr_t;
        let flags: c_int;

        reset_memblock_regions();
        base = (*(&raw mut memory_block)).base as phys_addr_t;
        flags = if movable_node_is_enabled() {
            MEMBLOCK_NONE
        } else {
            MEMBLOCK_HOTPLUG
        };

        for i in 0..NUMA_NODES {
            assert!(*node_fracs.add(i as usize) <= BASIS as c_uint);
            let size: phys_addr_t = MEM_SIZE * *node_fracs.add(i as usize) as phys_addr_t / BASIS;

            memblock_add_node(base, size, i as c_int, flags);
            base += size;
        }
        fill_memblock();
    }
}

#[no_mangle]
pub unsafe extern "C" fn dummy_physical_memory_init() {
    unsafe {
        (*(&raw mut memory_block)).base = malloc(PHYS_MEM_SIZE as usize);
        assert!(!(*(&raw mut memory_block)).base.is_null());
        fill_memblock();
    }
}

#[no_mangle]
pub unsafe extern "C" fn dummy_physical_memory_cleanup() {
    unsafe {
        free((*(&raw mut memory_block)).base);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dummy_physical_memory_base() -> phys_addr_t {
    unsafe { (*(&raw mut memory_block)).base as phys_addr_t }
}

unsafe fn usage(prog: *const c_char) {
    unsafe {
        // C BUILD_BUG_ON checked ARRAY_SIZE(help_opts) == ARRAY_SIZE(long_opts) - 1.
        let _ = [(); help_opts.len()];
        let _ = [(); long_opts.len() - 1];

        printf(b"Usage: %s [-%s]\n\0".as_ptr() as *const c_char, prog, short_opts);

        let mut i: usize = 0;
        while !long_opts[i].name.is_null() {
            printf(
                b"  -%c, --%-12s\t%s\n\0".as_ptr() as *const c_char,
                long_opts[i].val,
                long_opts[i].name,
                help_opts[i],
            );
            i += 1;
        }

        exit(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn parse_args(argc: c_int, argv: *mut *mut c_char) {
    unsafe {
        let mut c: c_int;

        loop {
            c = getopt_long_only(argc, argv, short_opts, long_opts.as_ptr(), ptr::null_mut());
            if c == -1 {
                break;
            }

            match c {
                x if x == b'm' as c_int => {
                    movable_node_enabled = true;
                }
                x if x == b'v' as c_int => {
                    verbose = 1;
                }
                _ => {
                    usage(*argv.add(0));
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_prefixes(postfix: *const c_char) {
    unsafe {
        for i in 0..nr_prefixes {
            test_print(
                b"%s%s\0".as_ptr() as *const c_char,
                prefixes[i as usize],
                DELIM,
            );
        }
        test_print(postfix);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_fail() {
    unsafe {
        if verbose != 0 {
            ksft_test_result_fail(b": \0".as_ptr() as *const c_char);
            print_prefixes(b"failed\n\0".as_ptr() as *const c_char);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_pass() {
    unsafe {
        if verbose != 0 {
            ksft_test_result_pass(b": \0".as_ptr() as *const c_char);
            print_prefixes(b"passed\n\0".as_ptr() as *const c_char);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_print(fmt: *const c_char, mut args: ...) {
    unsafe {
        if verbose != 0 {
            let saved_errno: c_int = errno;

            errno = saved_errno;
            vprintf(fmt, args.as_va_list());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn prefix_reset() {
    unsafe {
        memset(
            prefixes.as_mut_ptr() as *mut c_void,
            0,
            PREFIXES_MAX * size_of::<*const c_char>(),
        );
        nr_prefixes = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn prefix_push(prefix: *const c_char) {
    unsafe {
        assert!(nr_prefixes < PREFIXES_MAX as c_int);
        prefixes[nr_prefixes as usize] = prefix;
        nr_prefixes += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn prefix_pop() {
    unsafe {
        if nr_prefixes > 0 {
            prefixes[(nr_prefixes - 1) as usize] = ptr::null();
            nr_prefixes -= 1;
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
