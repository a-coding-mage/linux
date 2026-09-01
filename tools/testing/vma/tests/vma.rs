// SPDX-License-Identifier: GPL-2.0-or-later

unsafe fn compare_legacy_flags(legacy_flags: vm_flags_t, flags: vma_flags_t) -> bool {
    let legacy_val: ::core::ffi::c_ulong = legacy_flags as ::core::ffi::c_ulong;
    /* The lower word should contain the precise same value. */
    let flags_lower: ::core::ffi::c_ulong = flags.__vma_flags[0];
    let converted_flags: vma_flags_t;

    // C preprocessor condition preserved: #if NUM_VMA_FLAG_BITS > BITS_PER_LONG
    if NUM_VMA_FLAG_BITS > BITS_PER_LONG {
        let mut i: ::core::ffi::c_int = 1;

        /* All bits in higher flag values should be zero. */
        while i < NUM_VMA_FLAG_BITS / BITS_PER_LONG {
            if flags.__vma_flags[i as usize] != 0 {
                return false;
            }
            i += 1;
        }
    }

    static_assert!(::core::mem::size_of_val(&legacy_flags) == ::core::mem::size_of::<::core::ffi::c_ulong>());

    /* Assert that legacy flag helpers work correctly. */
    converted_flags = legacy_to_vma_flags(legacy_flags);
    ASSERT_FLAGS_SAME_MASK!(&converted_flags, flags);
    ASSERT_EQ!(vma_flags_to_legacy(flags), legacy_flags);

    legacy_val == flags_lower
}

unsafe fn test_copy_vma() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags!(
        VMA_READ_BIT,
        VMA_WRITE_BIT,
        VMA_MAYREAD_BIT,
        VMA_MAYWRITE_BIT
    );
    let mut mm: mm_struct = ::core::mem::zeroed();
    let mut need_locks: bool = false;
    VMA_ITERATOR!(vmi, &mut mm, 0);
    let mut vma: *mut vm_area_struct;
    let mut vma_prev: *mut vm_area_struct;
    let mut vma_new: *mut vm_area_struct;
    let mut vma_next: *mut vm_area_struct;
    let mut vma_orig: *mut vm_area_struct;

    /* Move forwards, adjacent to old self - self-merge. */

    vma = alloc_and_link_vma(&mut mm, 0x1000, 0x2000, 1, vma_flags);
    vma_set_anonymous(vma);
    vma_orig = vma;
    vma_new = copy_vma(&mut vma, 0x2000, 0x1000, 1, 1, &mut need_locks);
    ASSERT_EQ!(vma_new, vma_orig);
    ASSERT_EQ!(vma, vma_orig);
    ASSERT_EQ!((*vma_new).vm_start, 0x1000);
    ASSERT_EQ!((*vma_new).vm_end, 0x3000);

    cleanup_mm(&mut mm, &mut vmi);

    /* Move backwards, adjacent to old self - self-merge. */

    vma = alloc_and_link_vma(&mut mm, 0x2000, 0x3000, 2, vma_flags);
    vma_set_anonymous(vma);
    vma_orig = vma;
    vma_new = copy_vma(&mut vma, 0x1000, 0x1000, 2, 2, &mut need_locks);
    ASSERT_EQ!(vma_new, vma_orig);
    ASSERT_EQ!(vma, vma_orig);
    ASSERT_EQ!((*vma_new).vm_start, 0x1000);
    ASSERT_EQ!((*vma_new).vm_end, 0x3000);

    cleanup_mm(&mut mm, &mut vmi);

    /*
     * Move backwards between prior VMA and old self - self-merge and vma
     * updated to a new VMA.
     */

    vma_prev = alloc_and_link_vma(&mut mm, 0x1000, 0x2000, 1, vma_flags);
    vma_set_anonymous(vma_prev);
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x4000, 3, vma_flags);
    vma_set_anonymous(vma);
    vma_orig = vma;
    vma_new = copy_vma(&mut vma, 0x2000, 0x1000, 3, 3, &mut need_locks);
    ASSERT_NE!(vma_new, vma_orig);
    ASSERT_EQ!(vma_new, vma);
    ASSERT_EQ!((*vma_new).vm_start, 0x1000);
    ASSERT_EQ!((*vma_new).vm_end, 0x4000);

    cleanup_mm(&mut mm, &mut vmi);

    /* Move backwards and do not merge. */

    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x5000, 3, vma_flags);
    vma_new = copy_vma(&mut vma, 0, 0x2000, 0, 3, &mut need_locks);
    ASSERT_NE!(vma_new, vma);
    ASSERT_EQ!((*vma_new).vm_start, 0);
    ASSERT_EQ!((*vma_new).vm_end, 0x2000);
    ASSERT_EQ!((*vma_new).vm_pgoff, 0);
    vma_assert_attached(vma_new);

    cleanup_mm(&mut mm, &mut vmi);

    /* Move a VMA into position next to another and merge the two. */

    vma = alloc_and_link_vma(&mut mm, 0, 0x2000, 0, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x6000, 0x8000, 6, vma_flags);
    vma_new = copy_vma(&mut vma, 0x4000, 0x2000, 4, 4, &mut need_locks);
    vma_assert_attached(vma_new);

    ASSERT_EQ!(vma_new, vma_next);

    cleanup_mm(&mut mm, &mut vmi);
    true
}

unsafe fn test_vma_flags_unchanged() -> bool {
    let mut flags: vma_flags_t = EMPTY_VMA_FLAGS;
    let mut legacy_flags: vm_flags_t = 0;
    let mut bit: ::core::ffi::c_int;
    let mut vma: vm_area_struct = ::core::mem::zeroed();
    let mut desc: vm_area_desc = ::core::mem::zeroed();

    vma.flags = EMPTY_VMA_FLAGS;
    desc.vma_flags = EMPTY_VMA_FLAGS;

    bit = 0;
    while bit < BITS_PER_LONG {
        let mask: vma_flags_t = mk_vma_flags!(bit);

        legacy_flags |= (1 as vm_flags_t) << bit;

        /* Individual flags. */
        vma_flags_set!(&mut flags, bit);
        ASSERT_TRUE!(compare_legacy_flags(legacy_flags, flags));

        /* Via mask. */
        vma_flags_set_mask(&mut flags, mask);
        ASSERT_TRUE!(compare_legacy_flags(legacy_flags, flags));

        /* Same for VMA. */
        vma_set_flags!(&mut vma, bit);
        ASSERT_TRUE!(compare_legacy_flags(legacy_flags, vma.flags));
        vma_set_flags_mask(&mut vma, mask);
        ASSERT_TRUE!(compare_legacy_flags(legacy_flags, vma.flags));

        /* Same for VMA descriptor. */
        vma_desc_set_flags!(&mut desc, bit);
        ASSERT_TRUE!(compare_legacy_flags(legacy_flags, desc.vma_flags));
        vma_desc_set_flags_mask(&mut desc, mask);
        ASSERT_TRUE!(compare_legacy_flags(legacy_flags, desc.vma_flags));

        bit += 1;
    }

    true
}

unsafe fn test_vma_flags_cleared() -> bool {
    let empty: vma_flags_t = EMPTY_VMA_FLAGS;
    let mut flags: vma_flags_t = ::core::mem::zeroed();
    let mut i: ::core::ffi::c_int;

    /* Set all bits high. */
    memset(&mut flags as *mut _ as *mut _, 1, ::core::mem::size_of_val(&flags));
    /* Try to clear. */
    vma_flags_clear_all(&mut flags);
    /* Equal to EMPTY_VMA_FLAGS? */
    ASSERT_EQ!(memcmp(&empty as *const _ as *const _, &flags as *const _ as *const _, ::core::mem::size_of_val(&flags)), 0);
    /* Make sure every unsigned long entry in bitmap array zero. */
    i = 0;
    while (i as usize) < ::core::mem::size_of_val(&flags) / (BITS_PER_LONG as usize) {
        let val: ::core::ffi::c_ulong = flags.__vma_flags[i as usize];

        ASSERT_EQ!(val, 0);
        i += 1;
    }

    true
}

/*
 * C preprocessor condition preserved: #if NUM_VMA_FLAG_BITS > 64
 *
 * Assert that VMA flag functions that operate at the system word level function
 * correctly.
 */
unsafe fn test_vma_flags_word() -> bool {
    let mut flags: vma_flags_t = EMPTY_VMA_FLAGS;
    let comparison: vma_flags_t = mk_vma_flags!(VMA_READ_BIT, VMA_WRITE_BIT, 64, 65);

    if NUM_VMA_FLAG_BITS <= 64 {
        return true;
    }

    /* Set some custom high flags. */
    vma_flags_set!(&mut flags, 64, 65);

    /* Now overwrite the first word. */
    vma_flags_overwrite_word(&mut flags, VM_READ | VM_WRITE);
    /* Ensure they are equal. */
    ASSERT_EQ!(memcmp(&flags as *const _ as *const _, &comparison as *const _ as *const _, ::core::mem::size_of_val(&flags)), 0);

    flags = EMPTY_VMA_FLAGS;
    vma_flags_set!(&mut flags, 64, 65);

    /* Do the same with the _once() equivalent. */
    vma_flags_overwrite_word_once(&mut flags, VM_READ | VM_WRITE);
    ASSERT_EQ!(memcmp(&flags as *const _ as *const _, &comparison as *const _ as *const _, ::core::mem::size_of_val(&flags)), 0);

    flags = EMPTY_VMA_FLAGS;
    vma_flags_set!(&mut flags, 64, 65);

    /* Make sure we can set a word without disturbing other bits. */
    vma_flags_set!(&mut flags, VMA_WRITE_BIT);
    vma_flags_set_word(&mut flags, VM_READ);
    ASSERT_EQ!(memcmp(&flags as *const _ as *const _, &comparison as *const _ as *const _, ::core::mem::size_of_val(&flags)), 0);

    flags = EMPTY_VMA_FLAGS;
    vma_flags_set!(&mut flags, 64, 65);

    /* Make sure we can clear a word without disturbing other bits. */
    vma_flags_set!(&mut flags, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
    vma_flags_clear_word(&mut flags, VM_EXEC);
    ASSERT_EQ!(memcmp(&flags as *const _ as *const _, &comparison as *const _ as *const _, ::core::mem::size_of_val(&flags)), 0);

    true
}

/* Ensure that vma_flags_test() and friends works correctly. */
unsafe fn test_vma_flags_test() -> bool {
    let mut flags: vma_flags_t = mk_vma_flags!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);
    let mut desc: vm_area_desc = ::core::mem::zeroed();
    let mut vma: vm_area_struct = ::core::mem::zeroed();

    desc.vma_flags = flags;
    vma.flags = flags;

    macro_rules! do_test {
        ($flag:expr) => {{
            ASSERT_TRUE!(vma_flags_test(&flags, $flag));
            ASSERT_TRUE!(vma_flags_test_single_mask(&flags, mk_vma_flags!($flag)));
            ASSERT_TRUE!(vma_test(&vma, $flag));
            ASSERT_TRUE!(vma_test_single_mask(&vma, mk_vma_flags!($flag)));
            ASSERT_TRUE!(vma_desc_test(&desc, $flag));
        }};
    }

    macro_rules! do_test_false {
        ($flag:expr) => {{
            ASSERT_FALSE!(vma_flags_test(&flags, $flag));
            ASSERT_FALSE!(vma_flags_test_single_mask(&flags, mk_vma_flags!($flag)));
            ASSERT_FALSE!(vma_test(&vma, $flag));
            ASSERT_FALSE!(vma_test_single_mask(&vma, mk_vma_flags!($flag)));
            ASSERT_FALSE!(vma_desc_test(&desc, $flag));
        }};
    }

    do_test!(VMA_READ_BIT);
    do_test!(VMA_WRITE_BIT);
    do_test!(VMA_EXEC_BIT);
    // C preprocessor condition preserved: #if NUM_VMA_FLAG_BITS > 64
    if NUM_VMA_FLAG_BITS > 64 {
        do_test!(64);
        do_test!(65);
    }
    do_test_false!(VMA_MAYWRITE_BIT);
    if NUM_VMA_FLAG_BITS > 64 {
        do_test_false!(66);
    }

    /* We define the _single_mask() variants to return false if empty. */
    ASSERT_FALSE!(vma_flags_test_single_mask(&flags, EMPTY_VMA_FLAGS));
    ASSERT_FALSE!(vma_test_single_mask(&vma, EMPTY_VMA_FLAGS));
    /* Even when both flags and tested flag mask are empty! */
    flags = EMPTY_VMA_FLAGS;
    vma.flags = EMPTY_VMA_FLAGS;
    ASSERT_FALSE!(vma_flags_test_single_mask(&flags, EMPTY_VMA_FLAGS));
    ASSERT_FALSE!(vma_test_single_mask(&vma, EMPTY_VMA_FLAGS));

    true
}

/* Ensure that vma_flags_test_any() and friends works correctly. */
unsafe fn test_vma_flags_test_any() -> bool {
    let flags: vma_flags_t = mk_vma_flags!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);
    let mut vma: vm_area_struct = ::core::mem::zeroed();
    let mut desc: vm_area_desc = ::core::mem::zeroed();

    vma.flags = flags;
    desc.vma_flags = flags;

    macro_rules! do_test {
        ($($arg:expr),+ $(,)?) => {{
            ASSERT_TRUE!(vma_flags_test_any!(&flags, $($arg),+));
            ASSERT_TRUE!(vma_desc_test_any!(&desc, $($arg),+));
            ASSERT_TRUE!(vma_test_any!(&vma, $($arg),+));
        }};
    }

    macro_rules! do_test_all_true {
        ($($arg:expr),+ $(,)?) => {{
            ASSERT_TRUE!(vma_flags_test_all!(&flags, $($arg),+));
            ASSERT_TRUE!(vma_test_all!(&vma, $($arg),+));
        }};
    }

    macro_rules! do_test_all_false {
        ($($arg:expr),+ $(,)?) => {{
            ASSERT_FALSE!(vma_flags_test_all!(&flags, $($arg),+));
            ASSERT_FALSE!(vma_test_all!(&vma, $($arg),+));
        }};
    }

    /*
     * Testing for some flags that are present, some that are not - should
     * pass. ANY flags matching should work.
     */
    do_test!(VMA_READ_BIT, VMA_MAYREAD_BIT, VMA_SEQ_READ_BIT);
    /* However, the ...test_all() variant should NOT pass. */
    do_test_all_false!(VMA_READ_BIT, VMA_MAYREAD_BIT, VMA_SEQ_READ_BIT);
    if NUM_VMA_FLAG_BITS > 64 {
        /* But should pass for flags present. */
        do_test_all_true!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);
        /* Also subsets... */
        do_test_all_true!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64);
    }
    do_test_all_true!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
    do_test_all_true!(VMA_READ_BIT, VMA_WRITE_BIT);
    do_test_all_true!(VMA_READ_BIT);
    /*
     * Check _mask variant. We don't need to test extensively as macro
     * helper is the equivalent.
     */
    ASSERT_TRUE!(vma_flags_test_any_mask(&flags, flags));
    ASSERT_TRUE!(vma_flags_test_all_mask(&flags, flags));

    /* Single bits. */
    do_test!(VMA_READ_BIT);
    do_test!(VMA_WRITE_BIT);
    do_test!(VMA_EXEC_BIT);
    if NUM_VMA_FLAG_BITS > 64 {
        do_test!(64);
        do_test!(65);
    }

    /* Two bits. */
    do_test!(VMA_READ_BIT, VMA_WRITE_BIT);
    do_test!(VMA_READ_BIT, VMA_EXEC_BIT);
    do_test!(VMA_WRITE_BIT, VMA_EXEC_BIT);
    /* Ordering shouldn't matter. */
    do_test!(VMA_WRITE_BIT, VMA_READ_BIT);
    do_test!(VMA_EXEC_BIT, VMA_READ_BIT);
    do_test!(VMA_EXEC_BIT, VMA_WRITE_BIT);
    if NUM_VMA_FLAG_BITS > 64 {
        do_test!(VMA_READ_BIT, 64);
        do_test!(VMA_WRITE_BIT, 64);
        do_test!(64, VMA_READ_BIT);
        do_test!(64, VMA_WRITE_BIT);
        do_test!(VMA_READ_BIT, 65);
        do_test!(VMA_WRITE_BIT, 65);
        do_test!(65, VMA_READ_BIT);
        do_test!(65, VMA_WRITE_BIT);
    }
    /* Three bits. */
    do_test!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
    if NUM_VMA_FLAG_BITS > 64 {
        /* No need to consider every single permutation. */
        do_test!(VMA_READ_BIT, VMA_WRITE_BIT, 64);
        do_test!(VMA_READ_BIT, VMA_WRITE_BIT, 65);

        /* Four bits. */
        do_test!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64);
        do_test!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 65);

        /* Five bits. */
        do_test!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);
    }

    /* Testing all flags against none trivially succeeds. */
    ASSERT_TRUE!(vma_flags_test_all_mask(&flags, EMPTY_VMA_FLAGS));
    ASSERT_TRUE!(vma_test_all_mask(&vma, EMPTY_VMA_FLAGS));

    true
}

/* Ensure that vma_flags_clear() and friends works correctly. */
unsafe fn test_vma_flags_clear() -> bool {
    let mut flags: vma_flags_t = mk_vma_flags!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);
    let mask: vma_flags_t = mk_vma_flags!(VMA_EXEC_BIT, 64);
    let mut vma: vm_area_struct = ::core::mem::zeroed();
    let mut desc: vm_area_desc = ::core::mem::zeroed();

    vma.flags = flags;
    desc.vma_flags = flags;

    /* Cursory check of _mask() variant, as the helper macros imply. */
    vma_flags_clear_mask(&mut flags, mask);
    vma_clear_flags_mask(&mut vma, mask);
    vma_desc_clear_flags_mask(&mut desc, mask);
    if NUM_VMA_FLAG_BITS > 64 {
        ASSERT_FALSE!(vma_flags_test_any!(&flags, VMA_EXEC_BIT, 64));
        ASSERT_FALSE!(vma_test_any!(&vma, VMA_EXEC_BIT, 64));
        ASSERT_FALSE!(vma_desc_test_any!(&desc, VMA_EXEC_BIT, 64));
        /* Reset. */
        vma_flags_set!(&mut flags, VMA_EXEC_BIT, 64);
        vma_set_flags!(&mut vma, VMA_EXEC_BIT, 64);
        vma_desc_set_flags!(&mut desc, VMA_EXEC_BIT, 64);
    }

    /*
     * Clear the flags and assert clear worked, then reset flags back to
     * include specified flags.
     */
    macro_rules! do_test_and_reset {
        ($($arg:expr),+ $(,)?) => {{
            vma_flags_clear!(&mut flags, $($arg),+);
            vma_clear_flags!(&mut vma, $($arg),+);
            vma_desc_clear_flags!(&mut desc, $($arg),+);
            ASSERT_FALSE!(vma_flags_test_any!(&flags, $($arg),+));
            ASSERT_FALSE!(vma_test_any!(&vma, $($arg),+));
            ASSERT_FALSE!(vma_desc_test_any!(&desc, $($arg),+));
            vma_flags_set!(&mut flags, $($arg),+);
            vma_set_flags!(&mut vma, $($arg),+);
            vma_desc_set_flags!(&mut desc, $($arg),+);
        }};
    }

    /* Single flags. */
    do_test_and_reset!(VMA_READ_BIT);
    do_test_and_reset!(VMA_WRITE_BIT);
    do_test_and_reset!(VMA_EXEC_BIT);
    if NUM_VMA_FLAG_BITS > 64 {
        do_test_and_reset!(64);
        do_test_and_reset!(65);
    }

    /* Two flags, in different orders. */
    do_test_and_reset!(VMA_READ_BIT, VMA_WRITE_BIT);
    do_test_and_reset!(VMA_READ_BIT, VMA_EXEC_BIT);
    if NUM_VMA_FLAG_BITS > 64 {
        do_test_and_reset!(VMA_READ_BIT, 64);
        do_test_and_reset!(VMA_READ_BIT, 65);
    }
    do_test_and_reset!(VMA_WRITE_BIT, VMA_READ_BIT);
    do_test_and_reset!(VMA_WRITE_BIT, VMA_EXEC_BIT);
    if NUM_VMA_FLAG_BITS > 64 {
        do_test_and_reset!(VMA_WRITE_BIT, 64);
        do_test_and_reset!(VMA_WRITE_BIT, 65);
    }
    do_test_and_reset!(VMA_EXEC_BIT, VMA_READ_BIT);
    do_test_and_reset!(VMA_EXEC_BIT, VMA_WRITE_BIT);
    if NUM_VMA_FLAG_BITS > 64 {
        do_test_and_reset!(VMA_EXEC_BIT, 64);
        do_test_and_reset!(VMA_EXEC_BIT, 65);
        do_test_and_reset!(64, VMA_READ_BIT);
        do_test_and_reset!(64, VMA_WRITE_BIT);
        do_test_and_reset!(64, VMA_EXEC_BIT);
        do_test_and_reset!(64, 65);
        do_test_and_reset!(65, VMA_READ_BIT);
        do_test_and_reset!(65, VMA_WRITE_BIT);
        do_test_and_reset!(65, VMA_EXEC_BIT);
        do_test_and_reset!(65, 64);
    }

    /* Three flags. */

    true
}

/* Ensure that vma_flags_empty() works correctly. */
unsafe fn test_vma_flags_empty() -> bool {
    let mut flags: vma_flags_t = mk_vma_flags!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);

    ASSERT_FLAGS_NONEMPTY!(&flags);
    vma_flags_clear!(&mut flags, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
    if NUM_VMA_FLAG_BITS > 64 {
        ASSERT_FLAGS_NONEMPTY!(&flags);
        vma_flags_clear!(&mut flags, 64, 65);
        ASSERT_FLAGS_EMPTY!(&flags);
    } else {
        ASSERT_FLAGS_EMPTY!(&flags);
    }

    true
}

/* Ensure that vma_flags_diff_pair() works correctly. */
unsafe fn test_vma_flags_diff() -> bool {
    let flags1: vma_flags_t = mk_vma_flags!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);

    let mut flags2: vma_flags_t = mk_vma_flags!(
        VMA_READ_BIT,
        VMA_WRITE_BIT,
        VMA_EXEC_BIT,
        VMA_MAYWRITE_BIT,
        VMA_MAYEXEC_BIT,
        64,
        65,
        66,
        67
    );
    let mut diff: vma_flags_t = vma_flags_diff_pair(&flags1, &flags2);

    if NUM_VMA_FLAG_BITS > 64 {
        ASSERT_FLAGS_SAME!(&diff, VMA_MAYWRITE_BIT, VMA_MAYEXEC_BIT, 66, 67);
    } else {
        ASSERT_FLAGS_SAME!(&diff, VMA_MAYWRITE_BIT, VMA_MAYEXEC_BIT);
    }
    /* Should be the same even if re-ordered. */
    diff = vma_flags_diff_pair(&flags2, &flags1);
    if NUM_VMA_FLAG_BITS > 64 {
        ASSERT_FLAGS_SAME!(&diff, VMA_MAYWRITE_BIT, VMA_MAYEXEC_BIT, 66, 67);
    } else {
        ASSERT_FLAGS_SAME!(&diff, VMA_MAYWRITE_BIT, VMA_MAYEXEC_BIT);
    }

    /* Should be no difference when applied against themselves. */
    diff = vma_flags_diff_pair(&flags1, &flags1);
    ASSERT_FLAGS_EMPTY!(&diff);
    diff = vma_flags_diff_pair(&flags2, &flags2);
    ASSERT_FLAGS_EMPTY!(&diff);

    /* One set of flags against an empty one should equal the original. */
    flags2 = EMPTY_VMA_FLAGS;
    diff = vma_flags_diff_pair(&flags1, &flags2);
    ASSERT_FLAGS_SAME_MASK!(&diff, flags1);

    /* A subset should work too. */
    flags2 = mk_vma_flags!(VMA_READ_BIT, VMA_WRITE_BIT);
    diff = vma_flags_diff_pair(&flags1, &flags2);
    if NUM_VMA_FLAG_BITS > 64 {
        ASSERT_FLAGS_SAME!(&diff, VMA_EXEC_BIT, 64, 65);
    } else {
        ASSERT_FLAGS_SAME!(&diff, VMA_EXEC_BIT);
    }

    true
}

/* Ensure that vma_flags_and() and friends work correctly. */
unsafe fn test_vma_flags_and() -> bool {
    let flags1: vma_flags_t = mk_vma_flags!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);
    let flags2: vma_flags_t = mk_vma_flags!(
        VMA_READ_BIT,
        VMA_WRITE_BIT,
        VMA_EXEC_BIT,
        VMA_MAYWRITE_BIT,
        VMA_MAYEXEC_BIT,
        64,
        65,
        66,
        67
    );
    let flags3: vma_flags_t = mk_vma_flags!(VMA_IO_BIT, VMA_MAYBE_GUARD_BIT, 68, 69);
    let mut and: vma_flags_t = vma_flags_and_mask(&flags1, flags2);

    if NUM_VMA_FLAG_BITS > 64 {
        ASSERT_FLAGS_SAME!(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);
    } else {
        ASSERT_FLAGS_SAME!(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
    }

    and = vma_flags_and_mask(&flags1, flags1);
    ASSERT_FLAGS_SAME_MASK!(&and, flags1);

    and = vma_flags_and_mask(&flags2, flags2);
    ASSERT_FLAGS_SAME_MASK!(&and, flags2);

    and = vma_flags_and_mask(&flags1, flags3);
    ASSERT_FLAGS_EMPTY!(&and);
    and = vma_flags_and_mask(&flags2, flags3);
    ASSERT_FLAGS_EMPTY!(&and);

    and = vma_flags_and!(&flags1, VMA_READ_BIT);
    ASSERT_FLAGS_SAME!(&and, VMA_READ_BIT);

    and = vma_flags_and!(&flags1, VMA_READ_BIT, VMA_WRITE_BIT);
    ASSERT_FLAGS_SAME!(&and, VMA_READ_BIT, VMA_WRITE_BIT);

    and = vma_flags_and!(&flags1, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
    ASSERT_FLAGS_SAME!(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);

    if NUM_VMA_FLAG_BITS > 64 {
        and = vma_flags_and!(&flags1, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64);
        ASSERT_FLAGS_SAME!(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64);

        and = vma_flags_and!(&flags1, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);
        ASSERT_FLAGS_SAME!(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);
    }

    /* And against some missing values. */

    and = vma_flags_and!(&flags1, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, VMA_IO_BIT);
    ASSERT_FLAGS_SAME!(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);

    and = vma_flags_and!(
        &flags1,
        VMA_READ_BIT,
        VMA_WRITE_BIT,
        VMA_EXEC_BIT,
        VMA_IO_BIT,
        VMA_RAND_READ_BIT
    );
    ASSERT_FLAGS_SAME!(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);

    if NUM_VMA_FLAG_BITS > 64 {
        and = vma_flags_and!(
            &flags1,
            VMA_READ_BIT,
            VMA_WRITE_BIT,
            VMA_EXEC_BIT,
            VMA_IO_BIT,
            VMA_RAND_READ_BIT,
            69
        );
        ASSERT_FLAGS_SAME!(&and, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
    }

    true
}

/* Ensure append_vma_flags() acts as expected. */
unsafe fn test_append_vma_flags() -> bool {
    let mut flags: vma_flags_t = append_vma_flags!(VMA_REMAP_FLAGS, VMA_READ_BIT, VMA_WRITE_BIT, 64, 65);

    ASSERT_FLAGS_SAME!(
        &flags,
        VMA_IO_BIT,
        VMA_PFNMAP_BIT,
        VMA_DONTEXPAND_BIT,
        VMA_DONTDUMP_BIT,
        VMA_READ_BIT,
        VMA_WRITE_BIT,
        64,
        65
    );

    flags = append_vma_flags!(EMPTY_VMA_FLAGS, VMA_READ_BIT, VMA_WRITE_BIT);
    ASSERT_FLAGS_SAME!(&flags, VMA_READ_BIT, VMA_WRITE_BIT);

    true
}

/* Assert that vma_flags_count() behaves as expected. */
unsafe fn test_vma_flags_count() -> bool {
    let mut flags: vma_flags_t = mk_vma_flags!(VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT, 64, 65);

    if NUM_VMA_FLAG_BITS > 64 {
        ASSERT_EQ!(vma_flags_count(&flags), 5);
        vma_flags_clear!(&mut flags, 64);
        ASSERT_EQ!(vma_flags_count(&flags), 4);
        vma_flags_clear!(&mut flags, 65);
    }
    ASSERT_EQ!(vma_flags_count(&flags), 3);
    vma_flags_clear!(&mut flags, VMA_EXEC_BIT);
    ASSERT_EQ!(vma_flags_count(&flags), 2);
    vma_flags_clear!(&mut flags, VMA_WRITE_BIT);
    ASSERT_EQ!(vma_flags_count(&flags), 1);
    vma_flags_clear!(&mut flags, VMA_READ_BIT);
    ASSERT_EQ!(vma_flags_count(&flags), 0);

    true
}

unsafe fn run_vma_tests(num_tests: *mut ::core::ffi::c_int, num_fail: *mut ::core::ffi::c_int) {
    TEST!(copy_vma);
    TEST!(vma_flags_unchanged);
    TEST!(vma_flags_cleared);
    // C preprocessor condition preserved: #if NUM_VMA_FLAG_BITS > 64
    if NUM_VMA_FLAG_BITS > 64 {
        TEST!(vma_flags_word);
    }
    TEST!(vma_flags_test);
    TEST!(vma_flags_test_any);
    TEST!(vma_flags_clear);
    TEST!(vma_flags_empty);
    TEST!(vma_flags_diff);
    TEST!(vma_flags_and);
    TEST!(append_vma_flags);
    TEST!(vma_flags_count);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
