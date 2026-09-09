/* SPDX-License-Identifier: GPL-2.0 */
/* Data Access Monitor Unit Tests */

// Translated from the C header under CONFIG_DAMON_VADDR_KUNIT_TEST.
// Required kernel, DAMON, maple-tree, and KUnit definitions are supplied externally.

unsafe fn __link_vmas(
    mt: *mut maple_tree,
    vmas: *mut vm_area_struct,
    nr_vmas: isize,
) -> i32 {
    let mut ret: i32 = -ENOMEM;
    let mut mas = MA_STATE!(mt, 0, 0);

    if nr_vmas == 0 {
        return 0;
    }

    mas_lock(&mut mas);
    for i in 0..nr_vmas {
        let vma = vmas.offset(i);
        mas_set_range(&mut mas, (*vma).vm_start, (*vma).vm_end - 1);
        if mas_store_gfp(&mut mas, vma, GFP_KERNEL) {
            goto_failed!();
        }
    }

    ret = 0;
    goto_failed!();
    mas_unlock(&mut mas);
    ret
}

/*
 * Test __damon_va_three_regions() function
 *
 * In case of virtual memory address spaces monitoring, DAMON converts the
 * complex and dynamic memory mappings of each target task to three discontiguous regions which cover every mapped areas. However, the three regions should not include the two biggest unmapped areas in the original mapping, because the two biggest areas are normally the areas between 1) heap and the mmap()-ed regions, and 2) the mmap()-ed regions and stack. Because these two unmapped areas are very huge but obviously never accessed, covering the region is just a waste.
 *
 * '__damon_va_three_regions() receives an address space of a process. It first identifies the start of mappings, end of mappings, and the two biggest unmapped areas. After that, based on the information, it constructs the three regions and returns. For more detail, refer to the comment of 'damon_init_regions_of()' function definition in 'mm/damon.c' file.
 *
 * For example, suppose virtual address ranges of 10-20, 20-25, 200-210,
 * 210-220, 300-305, and 307-330 (Other comments represent this mappings in
 * more short form: 10-20-25, 200-210-220, 300-305, 307-330) of a process are
 * mapped. To cover every mappings, the three regions should start with 10,
 * and end with 305. The process also has three unmapped areas, 25-200,
 * 220-300, and 305-307. Among those, 25-200 and 220-300 are the biggest two
 * unmapped areas, and thus it should be converted to three regions of 10-25,
 * 200-220, and 300-330.
 */
unsafe fn damon_test_three_regions_in_vmas(test: *mut kunit) {
    static mut MM: mm_struct = mm_struct::default();
    let mut regions: [damon_addr_range; 3] = [damon_addr_range::default(); 3];
    /* 10-20-25, 200-210-220, 300-305, 307-330 */
    static mut VMAS: [vm_area_struct; 6] = [
        vm_area_struct { vm_start: 10, vm_end: 20 },
        vm_area_struct { vm_start: 20, vm_end: 25 },
        vm_area_struct { vm_start: 200, vm_end: 210 },
        vm_area_struct { vm_start: 210, vm_end: 220 },
        vm_area_struct { vm_start: 300, vm_end: 305 },
        vm_area_struct { vm_start: 307, vm_end: 330 },
    ];

    mt_init_flags(&mut MM.mm_mt, MT_FLAGS_ALLOC_RANGE | MT_FLAGS_USE_RCU);
    if __link_vmas(&mut MM.mm_mt, VMAS.as_mut_ptr(), VMAS.len() as isize) != 0 {
        kunit_skip(test, "Failed to create VMA tree");
    }

    __damon_va_three_regions(&mut MM, regions.as_mut_ptr());

    KUNIT_EXPECT_EQ!(test, 10usize, regions[0].start);
    KUNIT_EXPECT_EQ!(test, 25usize, regions[0].end);
    KUNIT_EXPECT_EQ!(test, 200usize, regions[1].start);
    KUNIT_EXPECT_EQ!(test, 220usize, regions[1].end);
    KUNIT_EXPECT_EQ!(test, 300usize, regions[2].start);
    KUNIT_EXPECT_EQ!(test, 330usize, regions[2].end);
}

unsafe fn __nth_region_of(t: *mut damon_target, idx: i32) -> *mut damon_region {
    let mut i: u32 = 0;
    for r in damon_for_each_region!(t) {
        if { let equal = i as i32 == idx; i += 1; equal } {
            return r;
        }
    }
    core::ptr::null_mut()
}

/* Test 'damon_set_regions()'. */
unsafe fn damon_do_test_apply_three_regions(
    test: *mut kunit,
    regions: *mut usize,
    nr_regions: i32,
    three_regions: *mut damon_addr_range,
    expected: *mut usize,
    nr_expected: i32,
) {
    let t = damon_new_target();
    if t.is_null() { kunit_skip(test, "target alloc fail"); }

    let ranges = kmalloc_array((nr_regions / 2) as usize, core::mem::size_of::<damon_addr_range>(), GFP_KERNEL);
    if ranges.is_null() {
        damon_destroy_target(t, core::ptr::null_mut());
        kunit_skip(test, "ranges alloc fail");
    }
    for i in 0..(nr_regions / 2) {
        (*ranges.offset(i as isize)).start = *regions.offset((i * 2) as isize);
        (*ranges.offset(i as isize)).end = *regions.offset((i * 2 + 1) as isize);
    }
    if damon_set_regions(t, ranges, (nr_regions / 2) as usize, DAMON_MIN_REGION_SZ) != 0 {
        kfree(ranges);
        damon_destroy_target(t, core::ptr::null_mut());
        kunit_skip(test, "damon_set_regions() fail");
    }
    kfree(ranges);

    if damon_set_regions(t, three_regions, 3, DAMON_MIN_REGION_SZ) != 0 {
        damon_destroy_target(t, core::ptr::null_mut());
        kunit_skip(test, "second damon_set_regions() fail");
    }

    KUNIT_EXPECT_EQ!(test, damon_nr_regions(t), nr_expected / 2);
    if damon_nr_regions(t) != nr_expected / 2 { damon_destroy_target(t, core::ptr::null_mut()); return; }
    for i in 0..(nr_expected / 2) {
        let r = __nth_region_of(t, i);
        KUNIT_EXPECT_EQ!(test, (*r).ar.start, *expected.offset((i * 2) as isize));
        KUNIT_EXPECT_EQ!(test, (*r).ar.end, *expected.offset((i * 2 + 1) as isize));
    }
    damon_destroy_target(t, core::ptr::null_mut());
}

unsafe fn damon_test_apply_three_regions1(test: *mut kunit) {
    let regions: [usize; 16] = [10,20,20,30,50,55,55,57,57,59,70,80,80,90,90,100];
    let mut new_three_regions = [damon_addr_range { start: 5, end: 27 }, damon_addr_range { start: 45, end: 55 }, damon_addr_range { start: 73, end: 104 }];
    let expected: [usize; 12] = [5,20,20,27,45,55,73,80,80,90,90,104];
    damon_do_test_apply_three_regions(test, regions.as_ptr() as *mut _, 16, new_three_regions.as_mut_ptr(), expected.as_ptr() as *mut _, 12);
}

unsafe fn damon_test_apply_three_regions2(test: *mut kunit) {
    let regions: [usize; 16] = [10,20,20,30,50,55,55,57,57,59,70,80,80,90,90,100];
    let mut n = [damon_addr_range {start:5,end:27}, damon_addr_range {start:56,end:57}, damon_addr_range {start:65,end:104}];
    let e: [usize;12] = [5,20,20,27,56,57,65,80,80,90,90,104];
    damon_do_test_apply_three_regions(test, regions.as_ptr() as *mut _, 16, n.as_mut_ptr(), e.as_ptr() as *mut _, 12);
}

unsafe fn damon_test_apply_three_regions3(test: *mut kunit) {
    let regions: [usize;16] = [10,20,20,30,50,55,55,57,57,59,70,80,80,90,90,100];
    let mut n = [damon_addr_range {start:5,end:27}, damon_addr_range {start:61,end:63}, damon_addr_range {start:65,end:104}];
    let e: [usize;12] = [5,20,20,27,61,63,65,80,80,90,90,104];
    damon_do_test_apply_three_regions(test, regions.as_ptr() as *mut _, 16, n.as_mut_ptr(), e.as_ptr() as *mut _, 12);
}

unsafe fn damon_test_apply_three_regions4(test: *mut kunit) {
    let regions: [usize;16] = [10,20,20,30,50,55,55,57,57,59,70,80,80,90,90,100];
    let mut n = [damon_addr_range {start:5,end:7}, damon_addr_range {start:30,end:32}, damon_addr_range {start:65,end:68}];
    let e: [usize;6] = [5,7,30,32,65,68];
    damon_do_test_apply_three_regions(test, regions.as_ptr() as *mut _, 16, n.as_mut_ptr(), e.as_ptr() as *mut _, 6);
}

static mut DAMON_TEST_CASES: [kunit_case; 6] = [
    KUNIT_CASE!(damon_test_three_regions_in_vmas), KUNIT_CASE!(damon_test_apply_three_regions1),
    KUNIT_CASE!(damon_test_apply_three_regions2), KUNIT_CASE!(damon_test_apply_three_regions3),
    KUNIT_CASE!(damon_test_apply_three_regions4), kunit_case::default(),
];
static mut DAMON_TEST_SUITE: kunit_suite = kunit_suite { name: "damon-operations", test_cases: DAMON_TEST_CASES.as_mut_ptr() };
kunit_test_suite!(DAMON_TEST_SUITE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
