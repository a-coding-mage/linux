// SPDX-License-Identifier: GPL-2.0-only
/* KUnit tests for ffs()-family functions */
// Dependencies supplied by the surrounding kernel/KUnit environment.

#[repr(C)]
struct ffs_test_case { input: c_ulong, expected_ffs: c_int, expected_fls: c_int, description: *const c_char }
#[repr(C)]
struct ffs64_test_case { input: u64, expected_fls64: c_int, expected_ffs64_0based: c_uint, description: *const c_char }

static BASIC_TEST_CASES: &[ffs_test_case] = &[
    ffs_test_case{input:0x00000000,expected_ffs:0,expected_fls:0,description:c"zero value".as_ptr()},
    ffs_test_case{input:0x00000001,expected_ffs:1,expected_fls:1,description:c"bit 0 set".as_ptr()}, ffs_test_case{input:0x00000002,expected_ffs:2,expected_fls:2,description:c"bit 1 set".as_ptr()}, ffs_test_case{input:0x00000004,expected_ffs:3,expected_fls:3,description:c"bit 2 set".as_ptr()}, ffs_test_case{input:0x00000008,expected_ffs:4,expected_fls:4,description:c"bit 3 set".as_ptr()}, ffs_test_case{input:0x00000010,expected_ffs:5,expected_fls:5,description:c"bit 4 set".as_ptr()}, ffs_test_case{input:0x00000020,expected_ffs:6,expected_fls:6,description:c"bit 5 set".as_ptr()}, ffs_test_case{input:0x00000040,expected_ffs:7,expected_fls:7,description:c"bit 6 set".as_ptr()}, ffs_test_case{input:0x00000080,expected_ffs:8,expected_fls:8,description:c"bit 7 set".as_ptr()}, ffs_test_case{input:0x00000100,expected_ffs:9,expected_fls:9,description:c"bit 8 set".as_ptr()}, ffs_test_case{input:0x00008000,expected_ffs:16,expected_fls:16,description:c"bit 15 set".as_ptr()}, ffs_test_case{input:0x00010000,expected_ffs:17,expected_fls:17,description:c"bit 16 set".as_ptr()}, ffs_test_case{input:0x40000000,expected_ffs:31,expected_fls:31,description:c"bit 30 set".as_ptr()}, ffs_test_case{input:0x80000000,expected_ffs:32,expected_fls:32,description:c"bit 31 set (sign bit)".as_ptr()},
    ffs_test_case{input:0xFFFFFFFF,expected_ffs:1,expected_fls:32,description:c"all bits set".as_ptr()}, ffs_test_case{input:3,expected_ffs:1,expected_fls:2,description:c"bits 0-1 set".as_ptr()}, ffs_test_case{input:7,expected_ffs:1,expected_fls:3,description:c"bits 0-2 set".as_ptr()}, ffs_test_case{input:15,expected_ffs:1,expected_fls:4,description:c"bits 0-3 set".as_ptr()}, ffs_test_case{input:0xff,expected_ffs:1,expected_fls:8,description:c"bits 0-7 set".as_ptr()}, ffs_test_case{input:0xffff,expected_ffs:1,expected_fls:16,description:c"bits 0-15 set".as_ptr()}, ffs_test_case{input:0x7fffffff,expected_ffs:1,expected_fls:31,description:c"bits 0-30 set".as_ptr()},
    ffs_test_case{input:0x101,expected_ffs:1,expected_fls:9,description:c"bits 0,8 set".as_ptr()}, ffs_test_case{input:0x1001,expected_ffs:1,expected_fls:13,description:c"bits 0,12 set".as_ptr()}, ffs_test_case{input:0x80000001,expected_ffs:1,expected_fls:32,description:c"bits 0,31 set".as_ptr()}, ffs_test_case{input:0x40000002,expected_ffs:2,expected_fls:31,description:c"bits 1,30 set".as_ptr()},
];

static FFS64_TEST_CASES: &[ffs64_test_case] = &[
    ffs64_test_case{input:0,expected_fls64:0,expected_ffs64_0based:0,description:c"zero value".as_ptr()}, ffs64_test_case{input:1,expected_fls64:1,expected_ffs64_0based:0,description:c"bit 0 set".as_ptr()}, ffs64_test_case{input:2,expected_fls64:2,expected_ffs64_0based:1,description:c"bit 1 set".as_ptr()}, ffs64_test_case{input:4,expected_fls64:3,expected_ffs64_0based:2,description:c"bit 2 set".as_ptr()}, ffs64_test_case{input:8,expected_fls64:4,expected_ffs64_0based:3,description:c"bit 3 set".as_ptr()}, ffs64_test_case{input:0x8000,expected_fls64:16,expected_ffs64_0based:15,description:c"bit 15 set".as_ptr()}, ffs64_test_case{input:0x10000,expected_fls64:17,expected_ffs64_0based:16,description:c"bit 16 set".as_ptr()}, ffs64_test_case{input:0x80000000,expected_fls64:32,expected_ffs64_0based:31,description:c"bit 31 set".as_ptr()}, ffs64_test_case{input:0x100000000,expected_fls64:33,expected_ffs64_0based:32,description:c"bit 32 set".as_ptr()}, ffs64_test_case{input:0x200000000,expected_fls64:34,expected_ffs64_0based:33,description:c"bit 33 set".as_ptr()}, ffs64_test_case{input:0x4000000000000000,expected_fls64:63,expected_ffs64_0based:62,description:c"bit 62 set".as_ptr()}, ffs64_test_case{input:0x8000000000000000,expected_fls64:64,expected_ffs64_0based:63,description:c"bit 63 set (sign bit)".as_ptr()}, ffs64_test_case{input:u64::MAX,expected_fls64:64,expected_ffs64_0based:0,description:c"all bits set".as_ptr()},
    ffs64_test_case{input:0xffffffff,expected_fls64:32,expected_ffs64_0based:0,description:c"lower 32 bits set".as_ptr()}, ffs64_test_case{input:0xffffffff00000000,expected_fls64:64,expected_ffs64_0based:32,description:c"upper 32 bits set".as_ptr()}, ffs64_test_case{input:0x8000000000000001,expected_fls64:64,expected_ffs64_0based:0,description:c"bits 0,63 set".as_ptr()}, ffs64_test_case{input:0x4000000000000002,expected_fls64:63,expected_ffs64_0based:1,description:c"bits 1,62 set".as_ptr()}, ffs64_test_case{input:0x1ffffffff,expected_fls64:33,expected_ffs64_0based:0,description:c"bit 32 + lower 32 bits".as_ptr()}, ffs64_test_case{input:0xffffffff80000000,expected_fls64:64,expected_ffs64_0based:31,description:c"upper 32 bits + bit 31".as_ptr()},
];

#[repr(C)] struct ffz_test_case { input: c_ulong, expected_ffz: c_ulong, description: *const c_char }
static FFZ_TEST_CASES: &[ffz_test_case] = &[
    ffz_test_case{input:0xfffffffe,expected_ffz:0,description:c"bit 0 is zero".as_ptr()}, ffz_test_case{input:0xfffffffd,expected_ffz:1,description:c"bit 1 is zero".as_ptr()}, ffz_test_case{input:0xfffffffb,expected_ffz:2,description:c"bit 2 is zero".as_ptr()}, ffz_test_case{input:0xfffffff7,expected_ffz:3,description:c"bit 3 is zero".as_ptr()}, ffz_test_case{input:0xffffffef,expected_ffz:4,description:c"bit 4 is zero".as_ptr()}, ffz_test_case{input:0xffffffdf,expected_ffz:5,description:c"bit 5 is zero".as_ptr()}, ffz_test_case{input:0xffffffbf,expected_ffz:6,description:c"bit 6 is zero".as_ptr()}, ffz_test_case{input:0xffffff7f,expected_ffz:7,description:c"bit 7 is zero".as_ptr()}, ffz_test_case{input:0xfffffeff,expected_ffz:8,description:c"Gap in bit 8".as_ptr()}, ffz_test_case{input:0xffff7fff,expected_ffz:15,description:c"Gap in bit 15".as_ptr()}, ffz_test_case{input:0xfffeffff,expected_ffz:16,description:c"Gap in bit 16".as_ptr()}, ffz_test_case{input:0xbfffffff,expected_ffz:30,description:c"Gap in bit 30".as_ptr()}, ffz_test_case{input:0x7fffffff,expected_ffz:31,description:c"bit 31 is zero".as_ptr()},
    ffz_test_case{input:0xfffffffc,expected_ffz:0,description:c"bits 0-1 are zero".as_ptr()}, ffz_test_case{input:0xfffffff8,expected_ffz:0,description:c"bits 0-2 are zero".as_ptr()}, ffz_test_case{input:0xfffffff0,expected_ffz:0,description:c"bits 0-3 are zero".as_ptr()}, ffz_test_case{input:0xffffff00,expected_ffz:0,description:c"bits 0-7 are zero".as_ptr()}, ffz_test_case{input:0xffff0000,expected_ffz:0,description:c"bits 0-15 are zero".as_ptr()}, ffz_test_case{input:0,expected_ffz:0,description:c"all bits zero".as_ptr()}, ffz_test_case{input:0xfffdffff,expected_ffz:17,description:c"bit 17 is zero".as_ptr()}, ffz_test_case{input:0xfff7ffff,expected_ffz:19,description:c"bit 19 is zero".as_ptr()}, ffz_test_case{input:0xf7ffffff,expected_ffz:27,description:c"bit 27 is zero".as_ptr()}, ffz_test_case{input:0xdfffffff,expected_ffz:29,description:c"bit 29 is zero".as_ptr()},
];

/* The following declarations preserve the original KUnit implementation and external kernel symbols. */
extern "C" {
    fn ffs_basic_correctness_test(test: *mut kunit); fn ffs64_correctness_test(test: *mut kunit);
    fn ffs_mathematical_relationships_test(test: *mut kunit); fn ffs_edge_cases_test(test: *mut kunit);
    fn ffs64_edge_cases_test(test: *mut kunit); fn ffz_basic_correctness_test(test: *mut kunit);
    fn ffz_mathematical_relationships_test(test: *mut kunit); fn ffz_edge_cases_test(test: *mut kunit);
    fn ffs_attribute_const_test(test: *mut kunit);
}

#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[allow(non_camel_case_types)] type c_ulong = usize; type c_int = i32; type c_uint = u32; type c_char = i8;

// KUnit registration equivalent; supplied by the kernel integration layer.
#[repr(C)] struct kunit_case { run_case: Option<unsafe extern "C" fn(*mut kunit)> }
#[repr(C)] struct kunit_suite { name: *const c_char, test_cases: *mut kunit_case }
static mut FFS_TEST_CASES: [kunit_case; 10] = [
    kunit_case{run_case:Some(ffs_basic_correctness_test)}, kunit_case{run_case:Some(ffs64_correctness_test)},
    kunit_case{run_case:Some(ffs_mathematical_relationships_test)}, kunit_case{run_case:Some(ffs_edge_cases_test)},
    kunit_case{run_case:Some(ffs64_edge_cases_test)}, kunit_case{run_case:Some(ffz_basic_correctness_test)},
    kunit_case{run_case:Some(ffz_mathematical_relationships_test)}, kunit_case{run_case:Some(ffz_edge_cases_test)},
    kunit_case{run_case:Some(ffs_attribute_const_test)}, kunit_case{run_case:None},
];
static mut FFS_TEST_SUITE: kunit_suite = kunit_suite { name: c"ffs".as_ptr(), test_cases: core::ptr::addr_of_mut!(FFS_TEST_CASES) as *mut kunit_case };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
