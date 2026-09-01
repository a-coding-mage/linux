// SPDX-License-Identifier: GPL-2.0-or-later

// C dependencies removed from executable Rust. This translation expects the
// surrounding test/VMA support to provide the referenced types, constants,
// globals, functions, and macros.

unsafe fn test_mmap_region_basic() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(
        VMA_READ_BIT,
        VMA_WRITE_BIT,
        VMA_MAYREAD_BIT,
        VMA_MAYWRITE_BIT,
    );
    let mut mm: mm_struct = core::mem::zeroed();
    let mut addr: libc::c_ulong;
    let mut vma: *mut vm_area_struct;
    VMA_ITERATOR!(vmi, &mut mm, 0);

    (*current).mm = &mut mm;

    /* Map at 0x300000, length 0x3000. */
    addr = __mmap_region(
        core::ptr::null_mut(),
        0x300000,
        0x3000,
        vma_flags,
        0x300,
        core::ptr::null_mut(),
    );
    ASSERT_EQ!(addr, 0x300000);

    /* Map at 0x250000, length 0x3000. */
    addr = __mmap_region(
        core::ptr::null_mut(),
        0x250000,
        0x3000,
        vma_flags,
        0x250,
        core::ptr::null_mut(),
    );
    ASSERT_EQ!(addr, 0x250000);

    /* Map at 0x303000, merging to 0x300000 of length 0x6000. */
    addr = __mmap_region(
        core::ptr::null_mut(),
        0x303000,
        0x3000,
        vma_flags,
        0x303,
        core::ptr::null_mut(),
    );
    ASSERT_EQ!(addr, 0x303000);

    /* Map at 0x24d000, merging to 0x250000 of length 0x6000. */
    addr = __mmap_region(
        core::ptr::null_mut(),
        0x24d000,
        0x3000,
        vma_flags,
        0x24d,
        core::ptr::null_mut(),
    );
    ASSERT_EQ!(addr, 0x24d000);

    ASSERT_EQ!(mm.map_count, 2);

    for_each_vma!(vmi, vma, {
        if (*vma).vm_start == 0x300000 {
            ASSERT_EQ!((*vma).vm_end, 0x306000);
            ASSERT_EQ!((*vma).vm_pgoff, 0x300);
        } else if (*vma).vm_start == 0x24d000 {
            ASSERT_EQ!((*vma).vm_end, 0x253000);
            ASSERT_EQ!((*vma).vm_pgoff, 0x24d);
        } else {
            ASSERT_FALSE!(true);
        }
    });

    cleanup_mm(&mut mm, &mut vmi);
    true
}

unsafe fn run_mmap_tests(num_tests: *mut libc::c_int, num_fail: *mut libc::c_int) {
    TEST!(mmap_region_basic);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
