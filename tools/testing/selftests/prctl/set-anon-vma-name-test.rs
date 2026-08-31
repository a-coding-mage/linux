// SPDX-License-Identifier: GPL-2.0
/*
 * This test covers the anonymous VMA naming functionality through prctl calls
 */

// C dependencies translated from:
// errno.h, fcntl.h, sys/prctl.h, stdio.h, stdlib.h, sys/mman.h, string.h,
// unistd.h, and "kselftest_harness.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type FILE = c_void;

const AREA_SIZE: c_ulong = 1024;

const GOOD_NAME: &[u8] = b"goodname\0";
const BAD_NAME: &[u8] = b"badname\x01\0";

// If PR_SET_VMA is not supplied by system headers:
const PR_SET_VMA: c_int = 0x53564d41;
const PR_SET_VMA_ANON_NAME: c_int = 0;

const EOF: c_int = -1;
const EINVAL: c_int = 22;
const EBADF: c_int = 9;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn ftruncate(fd: c_int, length: isize) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

fn rename_vma(addr: c_ulong, size: c_ulong, name: *mut c_char) -> c_int {
    let res: c_int;

    unsafe {
        res = prctl(PR_SET_VMA, PR_SET_VMA_ANON_NAME, addr, size, name);
        if res < 0 {
            return -errno();
        }
    }
    res
}

fn was_renaming_successful(target_name: *mut c_char, ptr: c_ulong) -> c_int {
    let maps_file: *mut FILE;

    let mut line_buf: [c_char; 512] = [0; 512];
    let mut name: [c_char; 128] = [0; 128];
    let mut mode: [c_char; 8] = [0; 8];
    let mut start_addr: c_ulong = 0;
    let mut end_addr: c_ulong = 0;
    let mut offset: c_ulong = 0;
    let mut major_id: c_uint = 0;
    let mut minor_id: c_uint = 0;
    let mut node_id: c_uint = 0;

    let mut target_buf: [c_char; 128] = [0; 128];
    let mut res: c_int = 0;
    let mut sscanf_res: c_int;

    unsafe {
        // The entry name in maps will be in format [anon:<target_name>]
        sprintf(
            target_buf.as_mut_ptr(),
            b"[anon:%s]\0".as_ptr() as *const c_char,
            target_name,
        );
        maps_file = fopen(
            b"/proc/self/maps\0".as_ptr() as *const c_char,
            b"r\0".as_ptr() as *const c_char,
        );
        if maps_file.is_null() {
            printf(b"## /proc/self/maps file opening error\n\0".as_ptr() as *const c_char);
            return 0;
        }

        // Parse the maps file to find the entry we renamed
        while !fgets(line_buf.as_mut_ptr(), line_buf.len() as c_int, maps_file).is_null() {
            sscanf_res = sscanf(
                line_buf.as_ptr(),
                b"%lx-%lx %7s %lx %u:%u %u %s\0".as_ptr() as *const c_char,
                &mut start_addr as *mut c_ulong,
                &mut end_addr as *mut c_ulong,
                mode.as_mut_ptr(),
                &mut offset as *mut c_ulong,
                &mut major_id as *mut c_uint,
                &mut minor_id as *mut c_uint,
                &mut node_id as *mut c_uint,
                name.as_mut_ptr(),
            );
            if sscanf_res == EOF {
                res = 0;
                printf(b"## EOF while parsing the maps file\n\0".as_ptr() as *const c_char);
                break;
            }
            if strcmp(name.as_ptr(), target_buf.as_ptr()) == 0 && start_addr == ptr {
                res = 1;
                break;
            }
        }
        fclose(maps_file);
    }
    res
}

#[repr(C)]
struct vma {
    ptr_anon: *mut c_void,
    ptr_not_anon: *mut c_void,
    fd_not_anon: c_int,
}

fn vma_setup(self_: *mut vma) {
    let mut template: [c_char; 28] = *b"./set-anon-vma-test-XXXXXX\0";

    unsafe {
        (*self_).ptr_anon = mmap(
            core::ptr::null_mut(),
            AREA_SIZE as usize,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        );
        ASSERT_NE!((*self_).ptr_anon, MAP_FAILED);

        (*self_).fd_not_anon = mkstemp(template.as_mut_ptr());
        ASSERT_NE!((*self_).fd_not_anon, -1);
        unlink(template.as_ptr());
        ASSERT_EQ!(ftruncate((*self_).fd_not_anon, AREA_SIZE as isize), 0);
        (*self_).ptr_not_anon = mmap(
            core::ptr::null_mut(),
            AREA_SIZE as usize,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE,
            (*self_).fd_not_anon,
            0,
        );
        ASSERT_NE!((*self_).ptr_not_anon, MAP_FAILED);
        close((*self_).fd_not_anon);
    }
}

fn vma_teardown(self_: *mut vma) {
    unsafe {
        munmap((*self_).ptr_anon, AREA_SIZE as usize);
        munmap((*self_).ptr_not_anon, AREA_SIZE as usize);
    }
}

fn vma_renaming(self_: *mut vma) {
    unsafe {
        TH_LOG!("Try to rename the VMA with correct parameters");
        EXPECT_GE!(
            rename_vma((*self_).ptr_anon as c_ulong, AREA_SIZE, GOOD_NAME.as_ptr() as *mut c_char),
            0
        );
        EXPECT_TRUE!(was_renaming_successful(
            GOOD_NAME.as_ptr() as *mut c_char,
            (*self_).ptr_anon as c_ulong
        ));

        TH_LOG!("Try to pass invalid name (with non-printable character \\1) to rename the VMA");
        EXPECT_EQ!(
            rename_vma((*self_).ptr_anon as c_ulong, AREA_SIZE, BAD_NAME.as_ptr() as *mut c_char),
            -EINVAL
        );

        TH_LOG!("Try to rename non-anonymous VMA");
        EXPECT_EQ!(
            rename_vma(
                (*self_).ptr_not_anon as c_ulong,
                AREA_SIZE,
                GOOD_NAME.as_ptr() as *mut c_char
            ),
            -EBADF
        );
    }
}

TEST_HARNESS_MAIN!();
