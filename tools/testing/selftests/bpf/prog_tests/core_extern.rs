// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

/*
 * C source dependencies:
 *   <test_progs.h>
 *   <sys/mman.h>
 *   <sys/utsname.h>
 *   <linux/version.h>
 *   "test_core_extern.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

#[repr(C)]
pub struct bpf_object_open_opts {
    pub kconfig: *const c_char,
}

#[repr(C)]
pub struct test_core_extern__data {
    pub unkn_virt_val: u64,
    pub bpf_syscall: bool,
    pub tristate_val: i32,
    pub bool_val: bool,
    pub char_val: i8,
    pub ushort_val: u16,
    pub int_val: i32,
    pub ulong_val: u64,
    pub str_val: [c_char; 8],
    pub kern_ver: u32,
    pub missing_val: u64,
}

#[repr(C)]
pub struct test_core_extern {
    pub data: *mut test_core_extern__data,
}

unsafe extern "C" {
    fn uname(buf: *mut utsname) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn usleep(usec: c_uint) -> c_int;

    fn test__start_subtest(name: *const c_char) -> bool;
    fn test_core_extern__open_opts(opts: *const bpf_object_open_opts) -> *mut test_core_extern;
    fn test_core_extern__load(skel: *mut test_core_extern) -> c_int;
    fn test_core_extern__attach(skel: *mut test_core_extern) -> c_int;
    fn test_core_extern__destroy(skel: *mut test_core_extern);

    fn ASSERT_OK_PTR(ptr: *mut test_core_extern, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(got: u64, exp: u64, name: *const c_char) -> bool;
}

const TRI_NO: i32 = 0;
const TRI_YES: i32 = 1;
const TRI_MODULE: i32 = 2;

const fn KERNEL_VERSION(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 16) + (minor << 8) + if patch > 255 { 255 } else { patch }
}

unsafe fn get_kernel_version() -> u32 {
    let mut major: u32 = 0;
    let mut minor: u32 = 0;
    let mut patch: u32 = 0;
    let mut info: utsname = core::mem::zeroed();

    uname(&mut info);
    if sscanf(
        info.release.as_ptr(),
        c"%u.%u.%u".as_ptr(),
        &mut major as *mut u32,
        &mut minor as *mut u32,
        &mut patch as *mut u32,
    ) != 3
    {
        return 0;
    }
    KERNEL_VERSION(major, minor, patch)
}

const CFG: *const c_char = c"CONFIG_BPF_SYSCALL=n\n".as_ptr();

#[repr(C)]
struct test_case {
    name: *const c_char,
    cfg: *const c_char,
    fails: bool,
    data: test_core_extern__data,
}

const fn data_default() -> test_core_extern__data {
    test_core_extern__data {
        unkn_virt_val: 0,
        bpf_syscall: false,
        tristate_val: 0,
        bool_val: false,
        char_val: 0,
        ushort_val: 0,
        int_val: 0,
        ulong_val: 0,
        str_val: [0; 8],
        kern_ver: 0,
        missing_val: 0,
    }
}

const fn str8(bytes: &[u8; 8]) -> [c_char; 8] {
    [
        bytes[0] as c_char,
        bytes[1] as c_char,
        bytes[2] as c_char,
        bytes[3] as c_char,
        bytes[4] as c_char,
        bytes[5] as c_char,
        bytes[6] as c_char,
        bytes[7] as c_char,
    ]
}

static mut test_cases: [test_case; 36] = [
    test_case {
        name: c"default search path".as_ptr(),
        cfg: core::ptr::null(),
        fails: false,
        data: test_core_extern__data { bpf_syscall: true, ..data_default() },
    },
    test_case {
        name: c"custom values".as_ptr(),
        cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_TRISTATE=m\nCONFIG_BOOL=y\nCONFIG_CHAR=100\nCONFIG_USHORT=30000\nCONFIG_INT=123456\nCONFIG_ULONG=0xDEADBEEFC0DE\nCONFIG_STR=\"abracad\"\nCONFIG_MISSING=0".as_ptr(),
        fails: false,
        data: test_core_extern__data {
            unkn_virt_val: 0,
            bpf_syscall: false,
            tristate_val: TRI_MODULE,
            bool_val: true,
            char_val: 100,
            ushort_val: 30000,
            int_val: 123456,
            ulong_val: 0xDEADBEEFC0DE,
            str_val: str8(b"abracad\0"),
            ..data_default()
        },
    },
    /* TRISTATE */
    test_case { name: c"tristate (y)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_TRISTATE=y\n".as_ptr(), fails: false, data: test_core_extern__data { tristate_val: TRI_YES, ..data_default() } },
    test_case { name: c"tristate (n)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_TRISTATE=n\n".as_ptr(), fails: false, data: test_core_extern__data { tristate_val: TRI_NO, ..data_default() } },
    test_case { name: c"tristate (m)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_TRISTATE=m\n".as_ptr(), fails: false, data: test_core_extern__data { tristate_val: TRI_MODULE, ..data_default() } },
    test_case { name: c"tristate (int)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_TRISTATE=1".as_ptr(), data: data_default() },
    test_case { name: c"tristate (bad)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_TRISTATE=M".as_ptr(), data: data_default() },
    /* BOOL */
    test_case { name: c"bool (y)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_BOOL=y\n".as_ptr(), fails: false, data: test_core_extern__data { bool_val: true, ..data_default() } },
    test_case { name: c"bool (n)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_BOOL=n\n".as_ptr(), fails: false, data: test_core_extern__data { bool_val: false, ..data_default() } },
    test_case { name: c"bool (tristate)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_BOOL=m".as_ptr(), data: data_default() },
    test_case { name: c"bool (int)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_BOOL=1".as_ptr(), data: data_default() },
    /* CHAR */
    test_case { name: c"char (tristate)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_CHAR=m\n".as_ptr(), fails: false, data: test_core_extern__data { char_val: b'm' as i8, ..data_default() } },
    test_case { name: c"char (bad)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_CHAR=q\n".as_ptr(), data: data_default() },
    test_case { name: c"char (empty)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_CHAR=\n".as_ptr(), data: data_default() },
    test_case { name: c"char (str)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_CHAR=\"y\"\n".as_ptr(), data: data_default() },
    /* STRING */
    test_case { name: c"str (empty)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_STR=\"\"\n".as_ptr(), fails: false, data: test_core_extern__data { str_val: str8(b"\0\0\0\0\0\0\0\0"), ..data_default() } },
    test_case { name: c"str (padded)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_STR=\"abra\"\n".as_ptr(), fails: false, data: test_core_extern__data { str_val: str8(b"abra\0\0\0\0"), ..data_default() } },
    test_case { name: c"str (too long)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_STR=\"abracada\"\n".as_ptr(), fails: false, data: test_core_extern__data { str_val: str8(b"abracad\0"), ..data_default() } },
    test_case { name: c"str (no value)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_STR=\n".as_ptr(), data: data_default() },
    test_case { name: c"str (bad value)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_STR=bla\n".as_ptr(), data: data_default() },
    /* INTEGERS */
    test_case {
        name: c"integer forms".as_ptr(),
        cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_CHAR=0xA\nCONFIG_USHORT=0462\nCONFIG_INT=-100\nCONFIG_ULONG=+1000000000000".as_ptr(),
        fails: false,
        data: test_core_extern__data {
            char_val: 0xA,
            ushort_val: 0o462,
            int_val: -100,
            ulong_val: 1000000000000,
            ..data_default()
        },
    },
    test_case { name: c"int (bad)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_INT=abc".as_ptr(), data: data_default() },
    test_case { name: c"int (str)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_INT=\"abc\"".as_ptr(), data: data_default() },
    test_case { name: c"int (empty)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_INT=".as_ptr(), data: data_default() },
    test_case { name: c"int (mixed)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_INT=123abc".as_ptr(), data: data_default() },
    test_case { name: c"int (max)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_INT=2147483647".as_ptr(), fails: false, data: test_core_extern__data { int_val: 2147483647, ..data_default() } },
    test_case { name: c"int (min)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_INT=-2147483648".as_ptr(), fails: false, data: test_core_extern__data { int_val: -2147483648, ..data_default() } },
    test_case { name: c"int (max+1)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_INT=2147483648".as_ptr(), data: data_default() },
    test_case { name: c"int (min-1)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_INT=-2147483649".as_ptr(), data: data_default() },
    test_case { name: c"ushort (max)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_USHORT=65535".as_ptr(), fails: false, data: test_core_extern__data { ushort_val: 65535, ..data_default() } },
    test_case { name: c"ushort (min)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_USHORT=0".as_ptr(), fails: false, data: test_core_extern__data { ushort_val: 0, ..data_default() } },
    test_case { name: c"ushort (max+1)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_USHORT=65536".as_ptr(), data: data_default() },
    test_case { name: c"ushort (min-1)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_USHORT=-1".as_ptr(), data: data_default() },
    test_case { name: c"u64 (max)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_ULONG=0xffffffffffffffff".as_ptr(), fails: false, data: test_core_extern__data { ulong_val: 0xffffffffffffffff, ..data_default() } },
    test_case { name: c"u64 (min)".as_ptr(), cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_ULONG=0".as_ptr(), fails: false, data: test_core_extern__data { ulong_val: 0, ..data_default() } },
    test_case { name: c"u64 (max+1)".as_ptr(), fails: true, cfg: c"CONFIG_BPF_SYSCALL=n\nCONFIG_ULONG=0x10000000000000000".as_ptr(), data: data_default() },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_core_extern() {
    let kern_ver: u32 = get_kernel_version();
    let mut err: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut skel: *mut test_core_extern = core::ptr::null_mut();
    let mut got: *mut u64;
    let mut exp: *mut u64;
    let n: c_int = (core::mem::size_of::<test_core_extern__data>() / core::mem::size_of::<u64>()) as c_int;

    i = 0;
    while (i as usize) < test_cases.len() {
        let t: *mut test_case = &mut test_cases[i as usize];
        let opts = bpf_object_open_opts {
            kconfig: (*t).cfg,
        };

        if !test__start_subtest((*t).name) {
            i += 1;
            continue;
        }

        skel = test_core_extern__open_opts(&opts);
        if !ASSERT_OK_PTR(skel, c"skel_open".as_ptr()) {
            test_core_extern__destroy(skel);
            skel = core::ptr::null_mut();
            i += 1;
            continue;
        }
        err = test_core_extern__load(skel);
        if (*t).fails {
            ASSERT_ERR(err, c"skel_load_should_fail".as_ptr());
            test_core_extern__destroy(skel);
            skel = core::ptr::null_mut();
            i += 1;
            continue;
        } else if !ASSERT_OK(err, c"skel_load".as_ptr()) {
            test_core_extern__destroy(skel);
            skel = core::ptr::null_mut();
            i += 1;
            continue;
        }
        err = test_core_extern__attach(skel);
        if !ASSERT_OK(err, c"attach_raw_tp".as_ptr()) {
            test_core_extern__destroy(skel);
            skel = core::ptr::null_mut();
            i += 1;
            continue;
        }

        usleep(1);

        (*t).data.kern_ver = kern_ver;
        (*t).data.missing_val = 0xDEADC0DE;
        got = (*skel).data as *mut u64;
        exp = &mut (*t).data as *mut test_core_extern__data as *mut u64;
        j = 0;
        while j < n {
            ASSERT_EQ(*got.offset(j as isize), *exp.offset(j as isize), c"result".as_ptr());
            j += 1;
        }

        test_core_extern__destroy(skel);
        skel = core::ptr::null_mut();
        i += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
