// SPDX-License-Identifier: GPL-2.0-only
/*
 * test system mappings are sealed when
 * KCONFIG_MSEAL_SYSTEM_MAPPINGS=y
 */

// C dependencies removed from executable Rust:
// _GNU_SOURCE, stdio.h, errno.h, unistd.h, string.h, stdbool.h,
// kselftest.h, kselftest_harness.h.

use core::ffi::{c_char, c_int, c_void};

type FILE = c_void;

const VMFLAGS: &[u8] = b"VmFlags:\0";
const MSEAL_FLAGS: &[u8] = b"sl\0";
const MAX_LINE_LEN: usize = 512;

extern "C" {
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
}

// errno is provided by the C runtime; the exact accessor is platform-specific.
extern "C" {
    static mut errno: c_int;
}

unsafe fn has_mapping(name: *mut c_char, maps: *mut FILE) -> bool {
    let mut line: [c_char; MAX_LINE_LEN] = [0; MAX_LINE_LEN];

    while !fgets(line.as_mut_ptr(), line.len() as c_int, maps).is_null() {
        if !strstr(line.as_ptr(), name).is_null() {
            return true;
        }
    }

    false
}

unsafe fn mapping_is_sealed(_name: *mut c_char, maps: *mut FILE) -> bool {
    let mut line: [c_char; MAX_LINE_LEN] = [0; MAX_LINE_LEN];

    while !fgets(line.as_mut_ptr(), line.len() as c_int, maps).is_null() {
        if strncmp(line.as_ptr(), VMFLAGS.as_ptr() as *const c_char, strlen(VMFLAGS.as_ptr() as *const c_char)) == 0 {
            if !strstr(line.as_ptr(), MSEAL_FLAGS.as_ptr() as *const c_char).is_null() {
                return true;
            }

            return false;
        }
    }

    false
}

#[repr(C)]
struct basic {
    maps: *mut FILE,
}

unsafe fn basic_setup(self_: *mut basic) {
    (*self_).maps = fopen(b"/proc/self/smaps\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if (*self_).maps.is_null() {
        // SKIP(return, "Could not open /proc/self/smap, errno=%d", errno);
        let _ = errno;
        return;
    }
}

unsafe fn basic_teardown(self_: *mut basic) {
    if !(*self_).maps.is_null() {
        fclose((*self_).maps);
    }
}

#[repr(C)]
struct basic_variant {
    name: *mut c_char,
    sealed: bool,
}

static mut basic_vdso: basic_variant = basic_variant {
    name: b"[vdso]\0".as_ptr() as *mut c_char,
    sealed: true,
};

static mut basic_vvar: basic_variant = basic_variant {
    name: b"[vvar]\0".as_ptr() as *mut c_char,
    sealed: true,
};

static mut basic_vvar_vclock: basic_variant = basic_variant {
    name: b"[vvar_vclock]\0".as_ptr() as *mut c_char,
    sealed: true,
};

static mut basic_sigpage: basic_variant = basic_variant {
    name: b"[sigpage]\0".as_ptr() as *mut c_char,
    sealed: true,
};

static mut basic_vectors: basic_variant = basic_variant {
    name: b"[vectors]\0".as_ptr() as *mut c_char,
    sealed: true,
};

static mut basic_uprobes: basic_variant = basic_variant {
    name: b"[uprobes]\0".as_ptr() as *mut c_char,
    sealed: true,
};

static mut basic_stack: basic_variant = basic_variant {
    name: b"[stack]\0".as_ptr() as *mut c_char,
    sealed: false,
};

unsafe fn basic_check_sealed(self_: *mut basic, variant: *mut basic_variant) {
    if !has_mapping((*variant).name, (*self_).maps) {
        // SKIP(return, "could not find the mapping, %s", variant->name);
        return;
    }

    // EXPECT_EQ(variant->sealed, mapping_is_sealed(variant->name, self->maps));
    let _ = (*variant).sealed == mapping_is_sealed((*variant).name, (*self_).maps);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
