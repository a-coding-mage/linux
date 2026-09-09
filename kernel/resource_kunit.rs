// SPDX-License-Identifier: GPL-2.0+
/*
 * Test cases for API provided by resource.c and ioport.h
 */

// External Linux/KUnit dependencies are supplied by other translation units.

const R0_START: resource_size_t = 0x0000;
const R0_END: resource_size_t = 0xffff;
const R1_START: resource_size_t = 0x1234;
const R1_END: resource_size_t = 0x2345;
const R2_START: resource_size_t = 0x4567;
const R2_END: resource_size_t = 0x5678;
const R3_START: resource_size_t = 0x6789;
const R3_END: resource_size_t = 0x789a;
const R4_START: resource_size_t = 0x2000;
const R4_END: resource_size_t = 0x7000;

static mut r0: resource = resource { start: R0_START, end: R0_END, ..resource::zeroed() };
static mut r1: resource = resource { start: R1_START, end: R1_END, ..resource::zeroed() };
static mut r2: resource = resource { start: R2_START, end: R2_END, ..resource::zeroed() };
static mut r3: resource = resource { start: R3_START, end: R3_END, ..resource::zeroed() };
static mut r4: resource = resource { start: R4_START, end: R4_END, ..resource::zeroed() };

#[repr(C)]
struct result {
    r1: *mut resource,
    r2: *mut resource,
    r: resource,
    ret: bool,
}

static mut results_for_union: [result; 10] = [
    result { r1: unsafe { &raw mut r1 }, r2: unsafe { &raw mut r0 }, r: resource { start: R0_START, end: R0_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r2 }, r2: unsafe { &raw mut r0 }, r: resource { start: R0_START, end: R0_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r3 }, r2: unsafe { &raw mut r0 }, r: resource { start: R0_START, end: R0_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r4 }, r2: unsafe { &raw mut r0 }, r: resource { start: R0_START, end: R0_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r2 }, r2: unsafe { &raw mut r1 }, r: resource::zeroed(), ret: false },
    result { r1: unsafe { &raw mut r3 }, r2: unsafe { &raw mut r1 }, r: resource::zeroed(), ret: false },
    result { r1: unsafe { &raw mut r4 }, r2: unsafe { &raw mut r1 }, r: resource { start: R1_START, end: R4_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r2 }, r2: unsafe { &raw mut r3 }, r: resource::zeroed(), ret: false },
    result { r1: unsafe { &raw mut r2 }, r2: unsafe { &raw mut r4 }, r: resource { start: R4_START, end: R4_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r3 }, r2: unsafe { &raw mut r4 }, r: resource { start: R4_START, end: R3_END, ..resource::zeroed() }, ret: true },
];

static mut results_for_intersection: [result; 10] = [
    result { r1: unsafe { &raw mut r1 }, r2: unsafe { &raw mut r0 }, r: resource { start: R1_START, end: R1_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r2 }, r2: unsafe { &raw mut r0 }, r: resource { start: R2_START, end: R2_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r3 }, r2: unsafe { &raw mut r0 }, r: resource { start: R3_START, end: R3_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r4 }, r2: unsafe { &raw mut r0 }, r: resource { start: R4_START, end: R4_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r2 }, r2: unsafe { &raw mut r1 }, r: resource::zeroed(), ret: false },
    result { r1: unsafe { &raw mut r3 }, r2: unsafe { &raw mut r1 }, r: resource::zeroed(), ret: false },
    result { r1: unsafe { &raw mut r4 }, r2: unsafe { &raw mut r1 }, r: resource { start: R4_START, end: R1_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r2 }, r2: unsafe { &raw mut r3 }, r: resource::zeroed(), ret: false },
    result { r1: unsafe { &raw mut r2 }, r2: unsafe { &raw mut r4 }, r: resource { start: R2_START, end: R2_END, ..resource::zeroed() }, ret: true },
    result { r1: unsafe { &raw mut r3 }, r2: unsafe { &raw mut r4 }, r: resource { start: R3_START, end: R4_END, ..resource::zeroed() }, ret: true },
];

unsafe fn resource_do_test(test: *mut kunit, ret: bool, r: *mut resource,
                           exp_ret: bool, exp_r: *const resource,
                           r1: *mut resource, r2: *mut resource) {
    KUNIT_EXPECT_EQ_MSG(test, ret, exp_ret, "Resources %pR %pR", r1, r2);
    KUNIT_EXPECT_EQ_MSG(test, (*r).start, (*exp_r).start, "Start elements are not equal");
    KUNIT_EXPECT_EQ_MSG(test, (*r).end, (*exp_r).end, "End elements are not equal");
}

unsafe fn resource_do_union_test(test: *mut kunit, r: *mut result) {
    let mut result = resource::zeroed();
    let mut ret = resource_union((*r).r1, (*r).r2, &mut result);
    resource_do_test(test, ret, &mut result, (*r).ret, &(*r).r, (*r).r1, (*r).r2);
    result = resource::zeroed();
    ret = resource_union((*r).r2, (*r).r1, &mut result);
    resource_do_test(test, ret, &mut result, (*r).ret, &(*r).r, (*r).r2, (*r).r1);
}

unsafe fn resource_test_union(test: *mut kunit) {
    let mut i: usize = 0;
    loop {
        resource_do_union_test(test, &mut results_for_union[i]);
        i = i.wrapping_add(1);
        if i >= results_for_union.len() { break; }
    }
}

unsafe fn resource_do_intersection_test(test: *mut kunit, r: *mut result) {
    let mut result = resource::zeroed();
    let mut ret = resource_intersection((*r).r1, (*r).r2, &mut result);
    resource_do_test(test, ret, &mut result, (*r).ret, &(*r).r, (*r).r1, (*r).r2);
    result = resource::zeroed();
    ret = resource_intersection((*r).r2, (*r).r1, &mut result);
    resource_do_test(test, ret, &mut result, (*r).ret, &(*r).r, (*r).r2, (*r).r1);
}

unsafe fn resource_test_intersection(test: *mut kunit) {
    let mut i: usize = 0;
    loop {
        resource_do_intersection_test(test, &mut results_for_intersection[i]);
        i = i.wrapping_add(1);
        if i >= results_for_intersection.len() { break; }
    }
}

/*
 * The test resource tree for region_intersects() test:
 *
 * BASE-BASE+1M-1 : Test System RAM 0
 *             # hole 0 (BASE+1M-BASE+2M)
 * BASE+2M-BASE+3M-1 : Test CXL Window 0
 * BASE+3M-BASE+4M-1 : Test System RAM 1
 * BASE+4M-BASE+7M-1 : Test CXL Window 1
 *   BASE+4M-BASE+5M-1 : Test System RAM 2
 *     BASE+4M+128K-BASE+4M+256K-1: Test Code
 *   BASE+5M-BASE+6M-1 : Test System RAM 3
 */
const RES_TEST_RAM0_OFFSET: resource_size_t = 0;
const RES_TEST_RAM0_SIZE: resource_size_t = SZ_1M;
const RES_TEST_HOLE0_OFFSET: resource_size_t = RES_TEST_RAM0_OFFSET + RES_TEST_RAM0_SIZE;
const RES_TEST_HOLE0_SIZE: resource_size_t = SZ_1M;
const RES_TEST_WIN0_OFFSET: resource_size_t = RES_TEST_HOLE0_OFFSET + RES_TEST_HOLE0_SIZE;
const RES_TEST_WIN0_SIZE: resource_size_t = SZ_1M;
const RES_TEST_RAM1_OFFSET: resource_size_t = RES_TEST_WIN0_OFFSET + RES_TEST_WIN0_SIZE;
const RES_TEST_RAM1_SIZE: resource_size_t = SZ_1M;
const RES_TEST_WIN1_OFFSET: resource_size_t = RES_TEST_RAM1_OFFSET + RES_TEST_RAM1_SIZE;
const RES_TEST_WIN1_SIZE: resource_size_t = SZ_1M * 3;
const RES_TEST_RAM2_OFFSET: resource_size_t = RES_TEST_WIN1_OFFSET;
const RES_TEST_RAM2_SIZE: resource_size_t = SZ_1M;
const RES_TEST_CODE_OFFSET: resource_size_t = RES_TEST_RAM2_OFFSET + SZ_128K;
const RES_TEST_CODE_SIZE: resource_size_t = SZ_128K;
const RES_TEST_RAM3_OFFSET: resource_size_t = RES_TEST_RAM2_OFFSET + RES_TEST_RAM2_SIZE;
const RES_TEST_RAM3_SIZE: resource_size_t = SZ_1M;
const RES_TEST_TOTAL_SIZE: resource_size_t = RES_TEST_WIN1_OFFSET + RES_TEST_WIN1_SIZE;

unsafe fn remove_free_resource(ctx: *mut core::ffi::c_void) {
    let res = ctx as *mut resource;
    remove_resource(res);
    kfree(res);
}

unsafe fn kfree_wrapper(ctx: *mut core::ffi::c_void) {
    kfree(ctx as *const core::ffi::c_void);
}

unsafe fn resource_test_add_action_or_abort(test: *mut kunit,
                                             action: unsafe fn(*mut core::ffi::c_void),
                                             ctx: *mut core::ffi::c_void) {
    KUNIT_ASSERT_EQ_MSG(test, 0, kunit_add_action_or_reset(test, action, ctx), "Fail to add action");
}

// The remaining KUnit/resource operations retain their C interfaces through external dependencies.
unsafe fn resource_test_request_region(test: *mut kunit, parent: *mut resource,
                                       start: resource_size_t, size: resource_size_t,
                                       name: *const core::ffi::c_char, flags: c_ulong) {
    let res = __request_region(parent, start, size, name, flags);
    KUNIT_ASSERT_NOT_NULL(test, res);
    resource_test_add_action_or_abort(test, remove_free_resource, res as *mut core::ffi::c_void);
}

unsafe fn resource_test_insert_resource(test: *mut kunit, parent: *mut resource,
                                        start: resource_size_t, size: resource_size_t,
                                        name: *const core::ffi::c_char, flags: c_ulong) {
    let res = kzalloc_resource();
    KUNIT_ASSERT_NOT_NULL(test, res);
    (*res).name = name;
    (*res).start = start;
    (*res).end = start + size - 1;
    (*res).flags = flags;
    if insert_resource(parent, res) != 0 {
        resource_test_add_action_or_abort(test, kfree_wrapper, res as *mut core::ffi::c_void);
        KUNIT_FAIL_AND_ABORT(test, "Fail to insert resource %pR\n", res);
    }
    resource_test_add_action_or_abort(test, remove_free_resource, res as *mut core::ffi::c_void);
}

unsafe fn resource_test_region_intersects(test: *mut kunit) {
    let flags: c_ulong = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
    let parent = alloc_free_mem_region(&mut iomem_resource, RES_TEST_TOTAL_SIZE, SZ_1M, c"test resources".as_ptr());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, parent);
    let start = (*parent).start;
    resource_test_add_action_or_abort(test, remove_free_resource, parent as *mut core::ffi::c_void);
    resource_test_request_region(test, parent, start + RES_TEST_RAM0_OFFSET, RES_TEST_RAM0_SIZE, c"Test System RAM 0".as_ptr(), flags);
    resource_test_insert_resource(test, parent, start + RES_TEST_WIN0_OFFSET, RES_TEST_WIN0_SIZE, c"Test CXL Window 0".as_ptr(), IORESOURCE_MEM);
    resource_test_request_region(test, parent, start + RES_TEST_RAM1_OFFSET, RES_TEST_RAM1_SIZE, c"Test System RAM 1".as_ptr(), flags);
    resource_test_insert_resource(test, parent, start + RES_TEST_WIN1_OFFSET, RES_TEST_WIN1_SIZE, c"Test CXL Window 1".as_ptr(), IORESOURCE_MEM);
    resource_test_request_region(test, parent, start + RES_TEST_RAM2_OFFSET, RES_TEST_RAM2_SIZE, c"Test System RAM 2".as_ptr(), flags);
    resource_test_insert_resource(test, parent, start + RES_TEST_CODE_OFFSET, RES_TEST_CODE_SIZE, c"Test Code".as_ptr(), flags);
    resource_test_request_region(test, parent, start + RES_TEST_RAM3_OFFSET, RES_TEST_RAM3_SIZE, c"Test System RAM 3".as_ptr(), flags);
    kunit_release_action(test, remove_free_resource, parent as *mut core::ffi::c_void);

    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS, region_intersects(start + RES_TEST_RAM0_OFFSET, PAGE_SIZE, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS, region_intersects(start + RES_TEST_RAM0_OFFSET + RES_TEST_RAM0_SIZE - PAGE_SIZE, 2 * PAGE_SIZE, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_DISJOINT, region_intersects(start + RES_TEST_HOLE0_OFFSET, PAGE_SIZE, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_DISJOINT, region_intersects(start + RES_TEST_HOLE0_OFFSET + RES_TEST_HOLE0_SIZE - PAGE_SIZE, 2 * PAGE_SIZE, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_MIXED, region_intersects(start + RES_TEST_WIN0_OFFSET + RES_TEST_WIN0_SIZE - PAGE_SIZE, 2 * PAGE_SIZE, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS, region_intersects(start + RES_TEST_RAM1_OFFSET + RES_TEST_RAM1_SIZE - PAGE_SIZE, 2 * PAGE_SIZE, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS, region_intersects(start + RES_TEST_RAM2_OFFSET + RES_TEST_RAM2_SIZE - PAGE_SIZE, 2 * PAGE_SIZE, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS, region_intersects(start + RES_TEST_CODE_OFFSET, PAGE_SIZE, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS, region_intersects(start + RES_TEST_RAM2_OFFSET, RES_TEST_RAM2_SIZE + PAGE_SIZE, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_MIXED, region_intersects(start + RES_TEST_RAM3_OFFSET, RES_TEST_RAM3_SIZE + PAGE_SIZE, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
}

#[allow(non_upper_case_globals)]
static mut resource_test_cases: [*const core::ffi::c_void; 4] = [
    resource_test_union as *const core::ffi::c_void,
    resource_test_intersection as *const core::ffi::c_void,
    resource_test_region_intersects as *const core::ffi::c_void,
    core::ptr::null(),
];

// KUnit suite registration and module metadata correspond to the C macros.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
