// SPDX-License-Identifier: GPL-2.0-only
// Translated from crash_core_test.c. Kernel/KUnit symbols are supplied by the
// surrounding build environment.

use core::{mem, ptr};

unsafe fn create_crash_mem(test: *mut kunit, max_ranges: u32, nr_initial_ranges: u32,
                           initial_ranges: *const range) -> *mut crash_mem {
    if max_ranges < nr_initial_ranges {
        kunit_err(test, "max_ranges (%u) < nr_initial_ranges (%u)\n", max_ranges, nr_initial_ranges);
        return ptr::null_mut();
    }
    let alloc_size = mem::size_of::<crash_mem>() + max_ranges as usize * mem::size_of::<range>();
    let mem = kunit_kzalloc(test, alloc_size, GFP_KERNEL);
    if mem.is_null() {
        kunit_err(test, "Failed to allocate crash_mem\n");
        return ptr::null_mut();
    }
    (*mem).max_nr_ranges = max_ranges;
    (*mem).nr_ranges = nr_initial_ranges;
    if !initial_ranges.is_null() && nr_initial_ranges > 0 {
        ptr::copy_nonoverlapping(initial_ranges, (*mem).ranges.as_mut_ptr(), nr_initial_ranges as usize);
    }
    mem
}

unsafe fn assert_ranges_equal(test: *mut kunit, actual_ranges: *const range, actual_nr_ranges: u32,
                              expected_ranges: *const range, expected_nr_ranges: u32,
                              case_name: *const i8) {
    kunit_assert_eq_msg(test, expected_nr_ranges, actual_nr_ranges, "%s: Number of ranges mismatch.", case_name);
    for i in 0..expected_nr_ranges as usize {
        kunit_assert_eq_msg(test, (*expected_ranges.add(i)).start, (*actual_ranges.add(i)).start,
                            "%s: Range %u start mismatch.", case_name, i as u32);
        kunit_assert_eq_msg(test, (*expected_ranges.add(i)).end, (*actual_ranges.add(i)).end,
                            "%s: Range %u end mismatch.", case_name, i as u32);
    }
}

#[repr(C)]
struct exclude_test_param {
    description: *const i8,
    exclude_start: u64,
    exclude_end: u64,
    initial_max_ranges: u32,
    initial_ranges: *const range,
    initial_nr_ranges: u32,
    expected_ranges: *const range,
    expected_nr_ranges: u32,
    expected_ret: i32,
}

unsafe fn run_exclude_test_case(test: *mut kunit, params: *const exclude_test_param) {
    kunit_info(test, "%s", (*params).description);
    let mem = create_crash_mem(test, (*params).initial_max_ranges, (*params).initial_nr_ranges,
                               (*params).initial_ranges);
    if mem.is_null() { return; }
    let ret = crash_exclude_mem_range(mem, (*params).exclude_start, (*params).exclude_end);
    kunit_assert_eq_msg(test, (*params).expected_ret, ret, "%s: Return value mismatch.", (*params).description);
    if (*params).expected_ret == 0 {
        assert_ranges_equal(test, (*mem).ranges.as_ptr(), (*mem).nr_ranges,
                            (*params).expected_ranges, (*params).expected_nr_ranges, (*params).description);
    } else {
        kunit_assert_eq_msg(test, (*params).initial_nr_ranges, (*mem).nr_ranges,
                            "%s: Number of ranges mismatch on error.", (*params).description);
    }
}

static single_range_b: range = range { start: 100, end: 199 };

static r_1_3: [range; 1] = [range { start: 150, end: 199 }];
static r_1_4: [range; 2] = [range { start: 100, end: 119 }, range { start: 180, end: 199 }];
static r_1_5: [range; 1] = [range { start: 100, end: 149 }];
static r_1_13: [range; 1] = [range { start: 101, end: 199 }];
static r_1_14: [range; 2] = [range { start: 100, end: 149 }, range { start: 151, end: 199 }];
static r_1_15: [range; 1] = [range { start: 100, end: 198 }];
static initial_2_1: [range; 3] = [range { start: 0, end: 0x3efff }, range { start: 0x3f000, end: 0x3ffff }, range { start: 0x40000, end: 0x9ffff }];
static initial_2_2: [range; 3] = [range { start: 1, end: 299 }, range { start: 401, end: 1000 }, range { start: 1001, end: 2000 }];

static exclude_single_range_test_data: [exclude_test_param; 17] = [
    exclude_test_param { description: b"1.1: A is left of B, no overlap\0".as_ptr() as _, exclude_start: 10, exclude_end: 50, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &single_range_b, expected_nr_ranges: 1, expected_ret: 0 },
    exclude_test_param { description: b"1.2: A's right boundary touches B's left boundary\0".as_ptr() as _, exclude_start: 10, exclude_end: 99, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &single_range_b, expected_nr_ranges: 1, expected_ret: 0 },
    exclude_test_param { description: b"1.3: A overlaps B's left part\0".as_ptr() as _, exclude_start: 50, exclude_end: 149, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &r_1_3, expected_nr_ranges: 1, expected_ret: 0 },
    exclude_test_param { description: b"1.4: A is completely inside B\0".as_ptr() as _, exclude_start: 120, exclude_end: 179, initial_max_ranges: 2, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &r_1_4, expected_nr_ranges: 2, expected_ret: 0 },
    exclude_test_param { description: b"1.5: A overlaps B's right part\0".as_ptr() as _, exclude_start: 150, exclude_end: 249, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &r_1_5, expected_nr_ranges: 1, expected_ret: 0 },
    exclude_test_param { description: b"1.6: A's left boundary touches B's right boundary\0".as_ptr() as _, exclude_start: 200, exclude_end: 250, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &single_range_b, expected_nr_ranges: 1, expected_ret: 0 },
    exclude_test_param { description: b"1.7: A is right of B, no overlap\0".as_ptr() as _, exclude_start: 250, exclude_end: 300, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &single_range_b, expected_nr_ranges: 1, expected_ret: 0 },
    exclude_test_param { description: b"1.8: A completely covers B and extends beyond\0".as_ptr() as _, exclude_start: 50, exclude_end: 250, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: ptr::null(), expected_nr_ranges: 0, expected_ret: 0 },
    exclude_test_param { description: b"1.9: A covers B and extends to the left\0".as_ptr() as _, exclude_start: 50, exclude_end: 199, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: ptr::null(), expected_nr_ranges: 0, expected_ret: 0 },
    exclude_test_param { description: b"1.10: A covers B and extends to the right\0".as_ptr() as _, exclude_start: 100, exclude_end: 250, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: ptr::null(), expected_nr_ranges: 0, expected_ret: 0 },
    exclude_test_param { description: b"1.11: A is identical to B\0".as_ptr() as _, exclude_start: 100, exclude_end: 199, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: ptr::null(), expected_nr_ranges: 0, expected_ret: 0 },
    exclude_test_param { description: b"1.12: A is a point, left of B, no overlap\0".as_ptr() as _, exclude_start: 10, exclude_end: 10, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &single_range_b, expected_nr_ranges: 1, expected_ret: 0 },
    exclude_test_param { description: b"1.13: A is a point, at start of B\0".as_ptr() as _, exclude_start: 100, exclude_end: 100, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &r_1_13, expected_nr_ranges: 1, expected_ret: 0 },
    exclude_test_param { description: b"1.14: A is a point, in middle of B (causes split)\0".as_ptr() as _, exclude_start: 150, exclude_end: 150, initial_max_ranges: 2, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &r_1_14, expected_nr_ranges: 2, expected_ret: 0 },
    exclude_test_param { description: b"1.15: A is a point, at end of B\0".as_ptr() as _, exclude_start: 199, exclude_end: 199, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &r_1_15, expected_nr_ranges: 1, expected_ret: 0 },
    exclude_test_param { description: b"1.16: A is a point, right of B, no overlap\0".as_ptr() as _, exclude_start: 250, exclude_end: 250, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: &single_range_b, expected_nr_ranges: 1, expected_ret: 0 },
    exclude_test_param { description: b"1.17: A completely inside B (split), no space (ENOMEM)\0".as_ptr() as _, exclude_start: 120, exclude_end: 179, initial_max_ranges: 1, initial_ranges: &single_range_b, initial_nr_ranges: 1, expected_ranges: ptr::null(), expected_nr_ranges: 1, expected_ret: -ENOMEM },
];

unsafe fn exclude_single_range_test(test: *mut kunit) {
    for p in &exclude_single_range_test_data { kunit_log(KERN_INFO, test, "Running: %s", p.description); run_exclude_test_case(test, p); }
}

static exclude_range_regression_test_data: [exclude_test_param; 2] = [
    exclude_test_param { description: b"2.1: exclude low 1M\0".as_ptr() as _, exclude_start: 0, exclude_end: (1 << 20) - 1, initial_max_ranges: 3, initial_ranges: &initial_2_1, initial_nr_ranges: 3, expected_ranges: ptr::null(), expected_nr_ranges: 0, expected_ret: 0 },
    exclude_test_param { description: b"2.2: when range out of bound\0".as_ptr() as _, exclude_start: 100, exclude_end: 200, initial_max_ranges: 3, initial_ranges: &initial_2_2, initial_nr_ranges: 3, expected_ranges: ptr::null(), expected_nr_ranges: 3, expected_ret: -ENOMEM },
];

unsafe fn exclude_range_regression_test(test: *mut kunit) {
    for p in &exclude_range_regression_test_data { kunit_log(KERN_INFO, test, "Running: %s", p.description); run_exclude_test_case(test, p); }
}

// KUnit suite registration: crash_exclude_mem_range_tests, containing the two
// test functions above. MODULE_DESCRIPTION and MODULE_LICENSE are retained as
// build-time metadata supplied by the kernel integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
