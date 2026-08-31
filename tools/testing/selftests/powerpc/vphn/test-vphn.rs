// SPDX-License-Identifier: GPL-2.0
//
// C dependencies from the original source:
// #include <stdio.h>
// #include <byteswap.h>
// #include "utils.h"
// #include "subunit.h"
// #include "vphn.c"

use std::os::raw::{c_char, c_int, c_long};

type U32 = u32;
type Be32 = u32;

const VPHN_REGISTER_COUNT: usize = 6;
const VPHN_ASSOC_BUFSIZE: usize = 25;

#[cfg(target_endian = "little")]
fn cpu_to_be32(x: U32) -> U32 {
    x.swap_bytes()
}

#[cfg(target_endian = "little")]
fn be32_to_cpu(x: Be32) -> U32 {
    x.swap_bytes()
}

#[cfg(target_endian = "little")]
unsafe fn be16_to_cpup(x: *const u16) -> u16 {
    (*x).swap_bytes()
}

#[cfg(target_endian = "little")]
fn cpu_to_be64(x: u64) -> u64 {
    x.swap_bytes()
}

#[cfg(not(target_endian = "little"))]
fn cpu_to_be32(x: U32) -> U32 {
    x
}

#[cfg(not(target_endian = "little"))]
fn be32_to_cpu(x: Be32) -> U32 {
    x
}

#[cfg(not(target_endian = "little"))]
unsafe fn be16_to_cpup(x: *const u16) -> u16 {
    *x
}

#[cfg(not(target_endian = "little"))]
fn cpu_to_be64(x: u64) -> u64 {
    x
}

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn vphn_unpack_associativity(input: *const c_long, output: *mut Be32);
    fn test_finish(name: *const c_char, ret: c_int);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

#[repr(C)]
struct test {
    descr: *const c_char,
    input: [c_long; VPHN_REGISTER_COUNT],
    expected: [U32; VPHN_ASSOC_BUFSIZE],
}

macro_rules! arr6 {
    ($($x:expr),* $(,)?) => {
        [$($x as u64 as c_long),*]
    };
}

macro_rules! exp {
    ($($x:expr),* $(,)?) => {
        {
            let mut a = [0_u32; VPHN_ASSOC_BUFSIZE];
            let vals = [$($x as U32),*];
            let mut i = 0;
            while i < vals.len() {
                a[i] = vals[i];
                i += 1;
            }
            a
        }
    };
}

static mut all_tests: [test; 17] = [
    test {
        descr: b"vphn: no data\0".as_ptr() as *const c_char,
        input: arr6![
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![0x00000000],
    },
    test {
        descr: b"vphn: 1 x 16-bit value\0".as_ptr() as *const c_char,
        input: arr6![
            0x8001ffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![0x00000001, 0x00000001],
    },
    test {
        descr: b"vphn: 2 x 16-bit values\0".as_ptr() as *const c_char,
        input: arr6![
            0x80018002ffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![0x00000002, 0x00000001, 0x00000002],
    },
    test {
        descr: b"vphn: 3 x 16-bit values\0".as_ptr() as *const c_char,
        input: arr6![
            0x800180028003ffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![0x00000003, 0x00000001, 0x00000002, 0x00000003],
    },
    test {
        descr: b"vphn: 4 x 16-bit values\0".as_ptr() as *const c_char,
        input: arr6![
            0x8001800280038004,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![
            0x00000004,
            0x00000001,
            0x00000002,
            0x00000003,
            0x00000004
        ],
    },
    test {
        /* Parsing the next 16-bit value out of the next 64-bit input
         * value.
         */
        descr: b"vphn: 5 x 16-bit values\0".as_ptr() as *const c_char,
        input: arr6![
            0x8001800280038004,
            0x8005ffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![
            0x00000005,
            0x00000001,
            0x00000002,
            0x00000003,
            0x00000004,
            0x00000005
        ],
    },
    test {
        /* Parse at most 6 x 64-bit input values */
        descr: b"vphn: 24 x 16-bit values\0".as_ptr() as *const c_char,
        input: arr6![
            0x8001800280038004,
            0x8005800680078008,
            0x8009800a800b800c,
            0x800d800e800f8010,
            0x8011801280138014,
            0x8015801680178018,
        ],
        expected: exp![
            0x00000018, 0x00000001, 0x00000002, 0x00000003, 0x00000004, 0x00000005,
            0x00000006, 0x00000007, 0x00000008, 0x00000009, 0x0000000a, 0x0000000b,
            0x0000000c, 0x0000000d, 0x0000000e, 0x0000000f, 0x00000010, 0x00000011,
            0x00000012, 0x00000013, 0x00000014, 0x00000015, 0x00000016, 0x00000017,
            0x00000018
        ],
    },
    test {
        descr: b"vphn: 1 x 32-bit value\0".as_ptr() as *const c_char,
        input: arr6![
            0x00000001ffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![0x00000001, 0x00000001],
    },
    test {
        descr: b"vphn: 2 x 32-bit values\0".as_ptr() as *const c_char,
        input: arr6![
            0x0000000100000002,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![0x00000002, 0x00000001, 0x00000002],
    },
    test {
        /* Parsing the next 32-bit value out of the next 64-bit input
         * value.
         */
        descr: b"vphn: 3 x 32-bit values\0".as_ptr() as *const c_char,
        input: arr6![
            0x0000000100000002,
            0x00000003ffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![0x00000003, 0x00000001, 0x00000002, 0x00000003],
    },
    test {
        /* Parse at most 6 x 64-bit input values */
        descr: b"vphn: 12 x 32-bit values\0".as_ptr() as *const c_char,
        input: arr6![
            0x0000000100000002,
            0x0000000300000004,
            0x0000000500000006,
            0x0000000700000008,
            0x000000090000000a,
            0x0000000b0000000c,
        ],
        expected: exp![
            0x0000000c, 0x00000001, 0x00000002, 0x00000003, 0x00000004, 0x00000005,
            0x00000006, 0x00000007, 0x00000008, 0x00000009, 0x0000000a, 0x0000000b,
            0x0000000c
        ],
    },
    test {
        descr: b"vphn: 16-bit value followed by 32-bit value\0".as_ptr() as *const c_char,
        input: arr6![
            0x800100000002ffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![0x00000002, 0x00000001, 0x00000002],
    },
    test {
        descr: b"vphn: 32-bit value followed by 16-bit value\0".as_ptr() as *const c_char,
        input: arr6![
            0x000000018002ffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![0x00000002, 0x00000001, 0x00000002],
    },
    test {
        /* Parse a 32-bit value split across two consecutives 64-bit
         * input values.
         */
        descr: b"vphn: 16-bit value followed by 2 x 32-bit values\0".as_ptr() as *const c_char,
        input: arr6![
            0x8001000000020000,
            0x0003ffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![
            0x00000003,
            0x00000001,
            0x00000002,
            0x00000003,
            0x00000004,
            0x00000005
        ],
    },
    test {
        /* The lower bits in 0x0001ffff don't get mixed up with the
         * 0xffff terminator.
         */
        descr: b"vphn: 32-bit value has all ones in 16 lower bits\0".as_ptr() as *const c_char,
        input: arr6![
            0x0001ffff80028003,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ],
        expected: exp![0x00000003, 0x0001ffff, 0x00000002, 0x00000003],
    },
    test {
        /* The following input doesn't follow the specification.
         */
        descr: b"vphn: last 32-bit value is truncated\0".as_ptr() as *const c_char,
        input: arr6![
            0x0000000100000002,
            0x0000000300000004,
            0x0000000500000006,
            0x0000000700000008,
            0x000000090000000a,
            0x0000000b800c2bad,
        ],
        expected: exp![
            0x0000000c, 0x00000001, 0x00000002, 0x00000003, 0x00000004, 0x00000005,
            0x00000006, 0x00000007, 0x00000008, 0x00000009, 0x0000000a, 0x0000000b,
            0x0000000c
        ],
    },
    test {
        descr: b"vphn: garbage after terminator\0".as_ptr() as *const c_char,
        input: arr6![
            0xffff2bad2bad2bad,
            0x2bad2bad2bad2bad,
            0x2bad2bad2bad2bad,
            0x2bad2bad2bad2bad,
            0x2bad2bad2bad2bad,
            0x2bad2bad2bad2bad,
        ],
        expected: exp![0x00000000],
    },
    test {
        descr: std::ptr::null(),
        input: [0; VPHN_REGISTER_COUNT],
        expected: [0; VPHN_ASSOC_BUFSIZE],
    },
];

unsafe extern "C" fn test_one(test: *mut test) -> c_int {
    let mut output: [Be32; VPHN_ASSOC_BUFSIZE] = [0; VPHN_ASSOC_BUFSIZE];
    let mut i: c_int;
    let len: c_int;

    vphn_unpack_associativity((*test).input.as_ptr(), output.as_mut_ptr());

    len = be32_to_cpu(output[0]) as c_int;
    if len != (*test).expected[0] as c_int {
        printf(
            b"expected %d elements, got %d\n\0".as_ptr() as *const c_char,
            (*test).expected[0],
            len,
        );
        return 1;
    }

    i = 1;
    while i < len {
        let val: U32 = be32_to_cpu(output[i as usize]);
        if val != (*test).expected[i as usize] {
            printf(
                b"element #%d is 0x%x, should be 0x%x\n\0".as_ptr() as *const c_char,
                i,
                val,
                (*test).expected[i as usize],
            );
            return 1;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn test_vphn() -> c_int {
    static mut TEST: *mut test = std::ptr::null_mut();

    TEST = all_tests.as_mut_ptr();
    while !(*TEST).descr.is_null() {
        let ret: c_int;

        ret = test_one(TEST);
        test_finish((*TEST).descr, ret);
        if ret != 0 {
            return ret;
        }

        TEST = TEST.add(1);
    }

    0
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = argc;
    let _ = argv;

    test_harness(test_vphn, b"test-vphn\0".as_ptr() as *const c_char)
}
