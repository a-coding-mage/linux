// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates */
// C dependencies translated as external Rust dependencies:
// <test_progs.h>
// "csum_diff_test.skel.h"

use core::ffi::{c_int, c_void};
use core::mem;
use core::ptr;

const BUFF_SZ: usize = 512;

#[repr(C)]
pub struct testcase {
    pub to_buff: [u64; BUFF_SZ / 8],
    pub to_buff_len: u32,
    pub from_buff: [u64; BUFF_SZ / 8],
    pub from_buff_len: u32,
    pub seed: u16,
    pub result: u16,
}

const NUM_PUSH_TESTS: usize = 4;

static mut push_tests: [testcase; NUM_PUSH_TESTS] = [
    testcase {
        to_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xdeadbeefdeadbeef;
            a
        },
        to_buff_len: 8,
        from_buff: [0u64; BUFF_SZ / 8],
        from_buff_len: 0,
        seed: 0,
        result: 0x3b3b,
    },
    testcase {
        to_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xdeadbeefdeadbeef;
            a[1] = 0xbeefdeadbeefdead;
            a
        },
        to_buff_len: 16,
        from_buff: [0u64; BUFF_SZ / 8],
        from_buff_len: 0,
        seed: 0x1234,
        result: 0x88aa,
    },
    testcase {
        to_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xdeadbeefdeadbeef;
            a[1] = 0xbeefdeadbeefdead;
            a
        },
        to_buff_len: 15,
        from_buff: [0u64; BUFF_SZ / 8],
        from_buff_len: 0,
        seed: 0x1234,
        #[cfg(target_endian = "little")]
        result: 0xcaa9,
        #[cfg(not(target_endian = "little"))]
        result: 0x87fd,
    },
    testcase {
        to_buff: [
            0x327b23c66b8b4567, 0x66334873643c9869, 0x19495cff74b0dc51, 0x625558ec2ae8944a,
            0x46e87ccd238e1f29, 0x507ed7ab3d1b58ba, 0x41b71efb2eb141f2, 0x7545e14679e2a9e3,
            0x5bd062c2515f007c, 0x4db127f812200854, 0x1f16e9e80216231b, 0x66ef438d1190cde7,
            0x3352255a140e0f76, 0x0ded7263109cf92e, 0x1befd79f7fdcc233, 0x6b68079a41a7c4c9,
            0x25e45d324e6afb66, 0x431bd7b7519b500d, 0x7c83e4583f2dba31, 0x62bbd95a257130a3,
            0x628c895d436c6125, 0x721da317333ab105, 0x2d1d5ae92443a858, 0x75a2a8d46763845e,
            0x79838cb208edbdab, 0x0b03e0c64353d0cd, 0x54e49eb4189a769b, 0x2ca8861171f32454,
            0x02901d820836c40e, 0x081386413a95f874, 0x7c3dbd3d1e7ff521, 0x6ceaf087737b8ddc,
            0x4516dde922221a70, 0x614fd4a13006c83e, 0x5577f8e1419ac241, 0x05072367440badfc,
            0x77465f013804823e, 0x5c482a977724c67e, 0x5e884adc2463b9ea, 0x2d51779651ead36b,
            0x153ea438580bd78f, 0x70a64e2a3855585c, 0x2a487cb06a2342ec, 0x725a06fb1d4ed43b,
            0x57e4ccaf2cd89a32, 0x4b588f547a6d8d3c, 0x6de91b18542289ec, 0x7644a45c38437fdb,
            0x684a481a32fff902, 0x749abb43579478fe, 0x1ba026fa3dc240fb, 0x75c6c33a79a1deaa,
            0x70c6a52912e685fb, 0x374a3fe6520eedd1, 0x23f9c13c4f4ef005, 0x275ac794649bb77c,
            0x1cf10fd839386575, 0x235ba861180115be, 0x354fe9f947398c89, 0x741226bb15b5af5c,
            0x10233c990d34b6a8, 0x615740953f6ab60f, 0x77ae35eb7e0c57b1, 0x310c50b3579be4f1,
        ],
        to_buff_len: 512,
        from_buff: [0u64; BUFF_SZ / 8],
        from_buff_len: 0,
        seed: 0xffff,
        result: 0xca45,
    },
];

const NUM_PULL_TESTS: usize = 4;

static mut pull_tests: [testcase; NUM_PULL_TESTS] = [
    testcase {
        to_buff: [0u64; BUFF_SZ / 8],
        to_buff_len: 0,
        from_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xdeadbeefdeadbeef;
            a
        },
        from_buff_len: 8,
        seed: 0,
        result: 0xc4c4,
    },
    testcase {
        to_buff: [0u64; BUFF_SZ / 8],
        to_buff_len: 0,
        from_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xdeadbeefdeadbeef;
            a[1] = 0xbeefdeadbeefdead;
            a
        },
        from_buff_len: 16,
        seed: 0x1234,
        result: 0x9bbd,
    },
    testcase {
        to_buff: [0u64; BUFF_SZ / 8],
        to_buff_len: 0,
        from_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xdeadbeefdeadbeef;
            a[1] = 0xbeefdeadbeefdead;
            a
        },
        from_buff_len: 15,
        seed: 0x1234,
        #[cfg(target_endian = "little")]
        result: 0x59be,
        #[cfg(not(target_endian = "little"))]
        result: 0x9c6a,
    },
    testcase {
        to_buff: [0u64; BUFF_SZ / 8],
        to_buff_len: 0,
        from_buff: [
            0x327b23c66b8b4567, 0x66334873643c9869, 0x19495cff74b0dc51, 0x625558ec2ae8944a,
            0x46e87ccd238e1f29, 0x507ed7ab3d1b58ba, 0x41b71efb2eb141f2, 0x7545e14679e2a9e3,
            0x5bd062c2515f007c, 0x4db127f812200854, 0x1f16e9e80216231b, 0x66ef438d1190cde7,
            0x3352255a140e0f76, 0x0ded7263109cf92e, 0x1befd79f7fdcc233, 0x6b68079a41a7c4c9,
            0x25e45d324e6afb66, 0x431bd7b7519b500d, 0x7c83e4583f2dba31, 0x62bbd95a257130a3,
            0x628c895d436c6125, 0x721da317333ab105, 0x2d1d5ae92443a858, 0x75a2a8d46763845e,
            0x79838cb208edbdab, 0x0b03e0c64353d0cd, 0x54e49eb4189a769b, 0x2ca8861171f32454,
            0x02901d820836c40e, 0x081386413a95f874, 0x7c3dbd3d1e7ff521, 0x6ceaf087737b8ddc,
            0x4516dde922221a70, 0x614fd4a13006c83e, 0x5577f8e1419ac241, 0x05072367440badfc,
            0x77465f013804823e, 0x5c482a977724c67e, 0x5e884adc2463b9ea, 0x2d51779651ead36b,
            0x153ea438580bd78f, 0x70a64e2a3855585c, 0x2a487cb06a2342ec, 0x725a06fb1d4ed43b,
            0x57e4ccaf2cd89a32, 0x4b588f547a6d8d3c, 0x6de91b18542289ec, 0x7644a45c38437fdb,
            0x684a481a32fff902, 0x749abb43579478fe, 0x1ba026fa3dc240fb, 0x75c6c33a79a1deaa,
            0x70c6a52912e685fb, 0x374a3fe6520eedd1, 0x23f9c13c4f4ef005, 0x275ac794649bb77c,
            0x1cf10fd839386575, 0x235ba861180115be, 0x354fe9f947398c89, 0x741226bb15b5af5c,
            0x10233c990d34b6a8, 0x615740953f6ab60f, 0x77ae35eb7e0c57b1, 0x310c50b3579be4f1,
        ],
        from_buff_len: 512,
        seed: 0xffff,
        result: 0x35ba,
    },
];

const NUM_DIFF_TESTS: usize = 4;

static mut diff_tests: [testcase; NUM_DIFF_TESTS] = [
    testcase {
        from_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xdeadbeefdeadbeef;
            a
        },
        from_buff_len: 8,
        to_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xabababababababab;
            a
        },
        to_buff_len: 8,
        seed: 0,
        result: 0x7373,
    },
    testcase {
        from_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xdeadbeefdeadbeef;
            a
        },
        from_buff_len: 7,
        to_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xabababababababab;
            a
        },
        to_buff_len: 7,
        seed: 0,
        #[cfg(target_endian = "little")]
        result: 0xa673,
        #[cfg(not(target_endian = "little"))]
        result: 0x73b7,
    },
    testcase {
        from_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0;
            a
        },
        from_buff_len: 8,
        to_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xabababababababab;
            a
        },
        to_buff_len: 8,
        seed: 0,
        result: 0xaeae,
    },
    testcase {
        from_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0xdeadbeefdeadbeef;
            a
        },
        from_buff_len: 8,
        to_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0;
            a
        },
        to_buff_len: 8,
        seed: 0xffff,
        result: 0xc4c4,
    },
];

const NUM_EDGE_TESTS: usize = 4;

static mut edge_tests: [testcase; NUM_EDGE_TESTS] = [
    testcase {
        from_buff: [0u64; BUFF_SZ / 8],
        from_buff_len: 0,
        to_buff: [0u64; BUFF_SZ / 8],
        to_buff_len: 0,
        seed: 0,
        result: 0,
    },
    testcase {
        from_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0x1234;
            a
        },
        from_buff_len: 0,
        to_buff: {
            let mut a = [0u64; BUFF_SZ / 8];
            a[0] = 0x1234;
            a
        },
        to_buff_len: 0,
        seed: 0,
        result: 0,
    },
    testcase {
        from_buff: [0u64; BUFF_SZ / 8],
        from_buff_len: 0,
        to_buff: [0u64; BUFF_SZ / 8],
        to_buff_len: 0,
        seed: 0x1234,
        result: 0x1234,
    },
    testcase {
        from_buff: [0u64; BUFF_SZ / 8],
        from_buff_len: 512,
        to_buff: [0u64; BUFF_SZ / 8],
        to_buff_len: 0,
        seed: 0xffff,
        result: 0xffff,
    },
];

#[repr(C)]
pub struct csum_diff_test {
    pub progs: *mut csum_diff_test_progs,
    pub rodata: *mut csum_diff_test_rodata,
    pub bss: *mut csum_diff_test_bss,
}

#[repr(C)]
pub struct csum_diff_test_progs {
    pub compute_checksum: *mut bpf_program,
}

#[repr(C)]
pub struct csum_diff_test_rodata {
    pub to_buff_len: u32,
    pub from_buff_len: u32,
}

#[repr(C)]
pub struct csum_diff_test_bss {
    pub to_buff: [u64; BUFF_SZ / 8],
    pub from_buff: [u64; BUFF_SZ / 8],
    pub seed: u16,
    pub result: u16,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *mut c_void,
    pub data_size_in: u32,
    pub data_out: *mut c_void,
    pub data_size_out: u32,
    pub repeat: u32,
}

extern "C" {
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn csum_diff_test__open() -> *mut csum_diff_test;
    fn csum_diff_test__load(obj: *mut csum_diff_test) -> c_int;
    fn csum_diff_test__destroy(obj: *mut csum_diff_test);
    fn test__start_subtest(name: *const i8) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const i8) -> bool;
    fn ASSERT_EQ(actual: u64, expected: u64, name: *const i8) -> bool;
}

unsafe fn trigger_csum_diff(skel: *const csum_diff_test) -> u16 {
    let mut tmp_out = [0u8; 64 << 2];
    let mut tmp_in = [0u8; 64];
    let err: c_int;
    let pfd: c_int;

    let mut topts = bpf_test_run_opts {
        sz: mem::size_of::<bpf_test_run_opts>(),
        data_in: tmp_in.as_mut_ptr() as *mut c_void,
        data_size_in: mem::size_of_val(&tmp_in) as u32,
        data_out: tmp_out.as_mut_ptr() as *mut c_void,
        data_size_out: mem::size_of_val(&tmp_out) as u32,
        repeat: 1,
    };
    pfd = bpf_program__fd((*(*skel).progs).compute_checksum);
    err = bpf_prog_test_run_opts(pfd, &mut topts);
    if err != 0 {
        return -1i16 as u16;
    }

    (*(*skel).bss).result
}

unsafe fn test_csum_diff(tests: *mut testcase, num_tests: c_int) {
    let mut skel: *mut csum_diff_test;
    let got: u16;
    let mut err: c_int;

    let mut i = 0;
    while i < num_tests {
        skel = csum_diff_test__open();
        if !ASSERT_OK_PTR(skel as *const c_void, c"csum_diff_test open".as_ptr()) {
            return;
        }

        (*(*skel).rodata).to_buff_len = (*tests.offset(i as isize)).to_buff_len;
        (*(*skel).rodata).from_buff_len = (*tests.offset(i as isize)).from_buff_len;

        err = csum_diff_test__load(skel);
        if !ASSERT_EQ(err as u64, 0, c"csum_diff_test load".as_ptr()) {
            csum_diff_test__destroy(skel);
            return;
        }

        ptr::copy_nonoverlapping(
            (*tests.offset(i as isize)).to_buff.as_ptr() as *const u8,
            (*(*skel).bss).to_buff.as_mut_ptr() as *mut u8,
            (*tests.offset(i as isize)).to_buff_len as usize,
        );
        ptr::copy_nonoverlapping(
            (*tests.offset(i as isize)).from_buff.as_ptr() as *const u8,
            (*(*skel).bss).from_buff.as_mut_ptr() as *mut u8,
            (*tests.offset(i as isize)).from_buff_len as usize,
        );
        (*(*skel).bss).seed = (*tests.offset(i as isize)).seed;

        got = trigger_csum_diff(skel);
        ASSERT_EQ(got as u64, (*tests.offset(i as isize)).result as u64, c"csum_diff result".as_ptr());

        csum_diff_test__destroy(skel);
        i += 1;
    }

    return;
}

#[no_mangle]
pub unsafe extern "C" fn test_test_csum_diff() {
    if test__start_subtest(c"csum_diff_push".as_ptr()) {
        test_csum_diff(push_tests.as_mut_ptr(), NUM_PUSH_TESTS as c_int);
    }
    if test__start_subtest(c"csum_diff_pull".as_ptr()) {
        test_csum_diff(pull_tests.as_mut_ptr(), NUM_PULL_TESTS as c_int);
    }
    if test__start_subtest(c"csum_diff_diff".as_ptr()) {
        test_csum_diff(diff_tests.as_mut_ptr(), NUM_DIFF_TESTS as c_int);
    }
    if test__start_subtest(c"csum_diff_edge".as_ptr()) {
        test_csum_diff(edge_tests.as_mut_ptr(), NUM_EDGE_TESTS as c_int);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
