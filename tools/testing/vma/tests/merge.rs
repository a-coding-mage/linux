// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from testing/vma/tests/merge.c.
// External kernel-test types, constants, assertions, and helpers are expected
// to be supplied by the surrounding translated test harness.

/* Helper function which provides a wrapper around a merge new VMA operation. */
unsafe fn merge_new(vmg: *mut vma_merge_struct) -> *mut vm_area_struct {
    let vma: *mut vm_area_struct;
    /*
     * For convenience, get prev and next VMAs. Which the new VMA operation
     * requires.
     */
    (*vmg).next = vma_next((*vmg).vmi);
    (*vmg).prev = vma_prev((*vmg).vmi);
    vma_iter_next_range((*vmg).vmi);

    vma = vma_merge_new_range(vmg);
    if !vma.is_null() {
        vma_assert_attached(vma);
    }

    vma
}

/*
 * Helper function which provides a wrapper around the expansion of an existing
 * VMA.
 */
unsafe fn expand_existing(vmg: *mut vma_merge_struct) -> i32 {
    vma_expand(vmg)
}

/*
 * Helper function to reset merge state the associated VMA iterator to a
 * specified new range.
 */
pub unsafe fn vmg_set_range(
    vmg: *mut vma_merge_struct,
    start: c_ulong,
    end: c_ulong,
    pgoff: pgoff_t,
    vma_flags: vma_flags_t,
) {
    vma_iter_set((*vmg).vmi, start);

    (*vmg).prev = core::ptr::null_mut();
    (*vmg).middle = core::ptr::null_mut();
    (*vmg).next = core::ptr::null_mut();
    (*vmg).target = core::ptr::null_mut();

    (*vmg).start = start;
    (*vmg).end = end;
    (*vmg).pgoff = pgoff;
    (*vmg).anon_pgoff = start >> PAGE_SHIFT;
    (*vmg).vma_flags = vma_flags;

    (*vmg).just_expand = false;
    (*vmg).__remove_middle = false;
    (*vmg).__remove_next = false;
    (*vmg).__adjust_middle_start = false;
    (*vmg).__adjust_next_start = false;
}

/* Helper function to set both the VMG range and its anon_vma. */
unsafe fn vmg_set_range_anon_vma(
    vmg: *mut vma_merge_struct,
    start: c_ulong,
    end: c_ulong,
    pgoff: pgoff_t,
    vma_flags: vma_flags_t,
    anon_vma: *mut anon_vma,
) {
    vmg_set_range(vmg, start, end, pgoff, vma_flags);
    (*vmg).anon_vma = anon_vma;
}

/*
 * Helper function to try to merge a new VMA.
 *
 * Update vmg and the iterator for it and try to merge, otherwise allocate a new
 * VMA, link it to the maple tree and return it.
 */
unsafe fn try_merge_new_vma(
    mm: *mut mm_struct,
    vmg: *mut vma_merge_struct,
    start: c_ulong,
    end: c_ulong,
    pgoff: pgoff_t,
    vma_flags: vma_flags_t,
    was_merged: *mut bool,
) -> *mut vm_area_struct {
    let merged: *mut vm_area_struct;

    vmg_set_range(vmg, start, end, pgoff, vma_flags);

    merged = merge_new(vmg);
    if !merged.is_null() {
        *was_merged = true;
        ASSERT_EQ((*vmg).state, VMA_MERGE_SUCCESS);
        return merged;
    }

    *was_merged = false;

    ASSERT_EQ((*vmg).state, VMA_MERGE_NOMERGE);

    alloc_and_link_vma(mm, start, end, pgoff, vma_flags)
}

unsafe fn test_simple_merge() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let vma_left: *mut vm_area_struct = alloc_vma(&mut mm, 0, 0x1000, 0, vma_flags);
    let vma_right: *mut vm_area_struct = alloc_vma(&mut mm, 0x2000, 0x3000, 2, vma_flags);
    let mut vmi = VMA_ITERATOR(&mut mm, 0x1000);
    let mut vmg = vma_merge_struct {
        mm: &mut mm,
        vmi: &mut vmi,
        start: 0x1000,
        end: 0x2000,
        vma_flags,
        pgoff: 1,
        anon_pgoff: 1,
        ..core::mem::zeroed()
    };
    let vma: *mut vm_area_struct;

    ASSERT_FALSE(attach_vma(&mut mm, vma_left));
    ASSERT_FALSE(attach_vma(&mut mm, vma_right));

    vma = merge_new(&mut vmg);
    ASSERT_NE(vma, core::ptr::null_mut());

    ASSERT_EQ((*vma).vm_start, 0);
    ASSERT_EQ((*vma).vm_end, 0x3000);
    ASSERT_EQ(vma_start_pgoff(vma), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 0);
    ASSERT_FLAGS_SAME_MASK(&mut (*vma).flags, vma_flags);

    detach_free_vma(vma);
    mtree_destroy(&mut mm.mm_mt);

    true
}

unsafe fn test_simple_modify() -> bool {
    let mut vma: *mut vm_area_struct;
    let mut vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let init_vma: *mut vm_area_struct = alloc_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    let mut vmi = VMA_ITERATOR(&mut mm, 0x1000);

    ASSERT_FALSE(attach_vma(&mut mm, init_vma));

    /*
     * The flags will not be changed, the vma_modify_flags() function
     * performs the merge/split only.
     */
    vma = vma_modify_flags(&mut vmi, init_vma, init_vma, 0x1000, 0x2000, &mut vma_flags);
    ASSERT_NE(vma, core::ptr::null_mut());
    /* We modify the provided VMA, and on split allocate new VMAs. */
    ASSERT_EQ(vma, init_vma);

    ASSERT_EQ((*vma).vm_start, 0x1000);
    ASSERT_EQ((*vma).vm_end, 0x2000);
    ASSERT_EQ(vma_start_pgoff(vma), 1);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 1);

    /*
     * Now walk through the three split VMAs and make sure they are as
     * expected.
     */
    vma_iter_set(&mut vmi, 0);
    vma = vma_iter_load(&mut vmi);

    ASSERT_EQ((*vma).vm_start, 0);
    ASSERT_EQ((*vma).vm_end, 0x1000);
    ASSERT_EQ(vma_start_pgoff(vma), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 0);

    detach_free_vma(vma);
    vma_iter_clear(&mut vmi);

    vma = vma_next(&mut vmi);

    ASSERT_EQ((*vma).vm_start, 0x1000);
    ASSERT_EQ((*vma).vm_end, 0x2000);
    ASSERT_EQ(vma_start_pgoff(vma), 1);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 1);

    detach_free_vma(vma);
    vma_iter_clear(&mut vmi);

    vma = vma_next(&mut vmi);

    ASSERT_EQ((*vma).vm_start, 0x2000);
    ASSERT_EQ((*vma).vm_end, 0x3000);
    ASSERT_EQ(vma_start_pgoff(vma), 2);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 2);

    detach_free_vma(vma);
    mtree_destroy(&mut mm.mm_mt);

    true
}

unsafe fn test_simple_expand() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let vma: *mut vm_area_struct = alloc_vma(&mut mm, 0, 0x1000, 0, vma_flags);
    let mut vmi = VMA_ITERATOR(&mut mm, 0);
    let mut vmg = vma_merge_struct {
        vmi: &mut vmi,
        target: vma,
        start: 0,
        end: 0x3000,
        pgoff: 0,
        ..core::mem::zeroed()
    };

    ASSERT_FALSE(attach_vma(&mut mm, vma));

    ASSERT_FALSE(expand_existing(&mut vmg));

    ASSERT_EQ((*vma).vm_start, 0);
    ASSERT_EQ((*vma).vm_end, 0x3000);
    ASSERT_EQ(vma_start_pgoff(vma), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 0);

    detach_free_vma(vma);
    mtree_destroy(&mut mm.mm_mt);

    true
}

unsafe fn test_simple_shrink() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let vma: *mut vm_area_struct = alloc_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    let mut vmi = VMA_ITERATOR(&mut mm, 0);

    ASSERT_FALSE(attach_vma(&mut mm, vma));

    ASSERT_FALSE(vma_shrink(&mut vmi, vma, 0x1000));

    ASSERT_EQ((*vma).vm_start, 0);
    ASSERT_EQ((*vma).vm_end, 0x1000);
    ASSERT_EQ(vma_start_pgoff(vma), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 0);

    detach_free_vma(vma);
    mtree_destroy(&mut mm.mm_mt);

    true
}

unsafe fn __test_merge_new(is_sticky: bool, a_is_sticky: bool, b_is_sticky: bool, c_is_sticky: bool) -> bool {
    let mut vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let mut vmi = VMA_ITERATOR(&mut mm, 0);
    let mut vmg = vma_merge_struct { mm: &mut mm, vmi: &mut vmi, ..core::mem::zeroed() };
    let mut dummy_anon_vma_chain_a = anon_vma_chain { anon_vma: &mut dummy_anon_vma, ..core::mem::zeroed() };
    let mut dummy_anon_vma_chain_b = anon_vma_chain { anon_vma: &mut dummy_anon_vma, ..core::mem::zeroed() };
    let mut dummy_anon_vma_chain_c = anon_vma_chain { anon_vma: &mut dummy_anon_vma, ..core::mem::zeroed() };
    let mut dummy_anon_vma_chain_d = anon_vma_chain { anon_vma: &mut dummy_anon_vma, ..core::mem::zeroed() };
    let vm_ops = vm_operations_struct { close: dummy_close, ..core::mem::zeroed() };
    let mut count: i32;
    let mut vma: *mut vm_area_struct;
    let mut vma_a: *mut vm_area_struct;
    let mut vma_b: *mut vm_area_struct;
    let mut vma_c: *mut vm_area_struct;
    let vma_d: *mut vm_area_struct;
    let mut merged = false;

    if is_sticky {
        vma_flags_set_mask(&mut vma_flags, VMA_STICKY_FLAGS);
    }

    /*
     * 0123456789abc
     * AA B       CC
     */
    vma_a = alloc_and_link_vma(&mut mm, 0, 0x2000, 0, vma_flags);
    ASSERT_NE(vma_a, core::ptr::null_mut());
    if a_is_sticky {
        vma_flags_set_mask(&mut (*vma_a).flags, VMA_STICKY_FLAGS);
    }
    /* We give each VMA a single avc so we can test anon_vma duplication. */
    INIT_LIST_HEAD(&mut (*vma_a).anon_vma_chain);
    list_add(&mut dummy_anon_vma_chain_a.same_vma, &mut (*vma_a).anon_vma_chain);

    vma_b = alloc_and_link_vma(&mut mm, 0x3000, 0x4000, 3, vma_flags);
    ASSERT_NE(vma_b, core::ptr::null_mut());
    if b_is_sticky {
        vma_flags_set_mask(&mut (*vma_b).flags, VMA_STICKY_FLAGS);
    }
    INIT_LIST_HEAD(&mut (*vma_b).anon_vma_chain);
    list_add(&mut dummy_anon_vma_chain_b.same_vma, &mut (*vma_b).anon_vma_chain);

    vma_c = alloc_and_link_vma(&mut mm, 0xb000, 0xc000, 0xb, vma_flags);
    ASSERT_NE(vma_c, core::ptr::null_mut());
    if c_is_sticky {
        vma_flags_set_mask(&mut (*vma_c).flags, VMA_STICKY_FLAGS);
    }
    INIT_LIST_HEAD(&mut (*vma_c).anon_vma_chain);
    list_add(&mut dummy_anon_vma_chain_c.same_vma, &mut (*vma_c).anon_vma_chain);

    /*
     * NO merge.
     *
     * 0123456789abc
     * AA B   **  CC
     */
    vma_d = try_merge_new_vma(&mut mm, &mut vmg, 0x7000, 0x9000, 7, vma_flags, &mut merged);
    ASSERT_NE(vma_d, core::ptr::null_mut());
    INIT_LIST_HEAD(&mut (*vma_d).anon_vma_chain);
    list_add(&mut dummy_anon_vma_chain_d.same_vma, &mut (*vma_d).anon_vma_chain);
    ASSERT_FALSE(merged);
    ASSERT_EQ(mm.map_count, 4);

    /*
     * Merge BOTH sides.
     *
     * 0123456789abc
     * AA*B   DD  CC
     */
    (*vma_a).vm_ops = &vm_ops as *const _;
    (*vma_b).anon_vma = &mut dummy_anon_vma;
    vma = try_merge_new_vma(&mut mm, &mut vmg, 0x2000, 0x3000, 2, vma_flags, &mut merged);
    ASSERT_EQ(vma, vma_a);
    /* Merge with A, delete B. */
    ASSERT_TRUE(merged);
    ASSERT_EQ((*vma).vm_start, 0);
    ASSERT_EQ((*vma).vm_end, 0x4000);
    ASSERT_EQ(vma_start_pgoff(vma), 0);
    ASSERT_EQ((*vma).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE(vma_write_started(vma));
    ASSERT_EQ(mm.map_count, 3);
    if is_sticky || a_is_sticky || b_is_sticky {
        ASSERT_TRUE(vma_flags_test_any_mask(&mut (*vma).flags, VMA_STICKY_FLAGS));
    }

    /*
     * Merge to PREVIOUS VMA.
     *
     * 0123456789abc
     * AAAA*  DD  CC
     */
    vma = try_merge_new_vma(&mut mm, &mut vmg, 0x4000, 0x5000, 4, vma_flags, &mut merged);
    ASSERT_EQ(vma, vma_a);
    /* Extend A. */
    ASSERT_TRUE(merged);
    ASSERT_EQ((*vma).vm_start, 0);
    ASSERT_EQ((*vma).vm_end, 0x5000);
    ASSERT_EQ(vma_start_pgoff(vma), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 0);
    ASSERT_EQ((*vma).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE(vma_write_started(vma));
    ASSERT_EQ(mm.map_count, 3);
    if is_sticky || a_is_sticky {
        ASSERT_TRUE(vma_flags_test_any_mask(&mut (*vma).flags, VMA_STICKY_FLAGS));
    }

    /*
     * Merge to NEXT VMA.
     *
     * 0123456789abc
     * AAAAA *DD  CC
     */
    (*vma_d).anon_vma = &mut dummy_anon_vma;
    (*vma_d).vm_ops = &vm_ops as *const _;
    vma = try_merge_new_vma(&mut mm, &mut vmg, 0x6000, 0x7000, 6, vma_flags, &mut merged);
    ASSERT_EQ(vma, vma_d);
    /* Prepend. */
    ASSERT_TRUE(merged);
    ASSERT_EQ((*vma).vm_start, 0x6000);
    ASSERT_EQ((*vma).vm_end, 0x9000);
    ASSERT_EQ(vma_start_pgoff(vma), 6);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 6);
    ASSERT_EQ((*vma).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE(vma_write_started(vma));
    ASSERT_EQ(mm.map_count, 3);
    if is_sticky {
        ASSERT_TRUE(vma_flags_test_any_mask(&mut (*vma).flags, VMA_STICKY_FLAGS));
    }

    /*
     * Merge BOTH sides.
     *
     * 0123456789abc
     * AAAAA*DDD  CC
     */
    (*vma_d).vm_ops = core::ptr::null();
    vma = try_merge_new_vma(&mut mm, &mut vmg, 0x5000, 0x6000, 5, vma_flags, &mut merged);
    ASSERT_EQ(vma, vma_a);
    /* Merge with A, delete D. */
    ASSERT_TRUE(merged);
    ASSERT_EQ((*vma).vm_start, 0);
    ASSERT_EQ((*vma).vm_end, 0x9000);
    ASSERT_EQ(vma_start_pgoff(vma), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 0);
    ASSERT_EQ((*vma).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE(vma_write_started(vma));
    ASSERT_EQ(mm.map_count, 2);
    if is_sticky || a_is_sticky {
        ASSERT_TRUE(vma_flags_test_any_mask(&mut (*vma).flags, VMA_STICKY_FLAGS));
    }

    /*
     * Merge to NEXT VMA.
     *
     * 0123456789abc
     * AAAAAAAAA *CC
     */
    (*vma_c).anon_vma = &mut dummy_anon_vma;
    vma = try_merge_new_vma(&mut mm, &mut vmg, 0xa000, 0xb000, 0xa, vma_flags, &mut merged);
    ASSERT_EQ(vma, vma_c);
    /* Prepend C. */
    ASSERT_TRUE(merged);
    ASSERT_EQ((*vma).vm_start, 0xa000);
    ASSERT_EQ((*vma).vm_end, 0xc000);
    ASSERT_EQ(vma_start_pgoff(vma), 0xa);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 0xa);
    ASSERT_EQ((*vma).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE(vma_write_started(vma));
    ASSERT_EQ(mm.map_count, 2);
    if is_sticky || c_is_sticky {
        ASSERT_TRUE(vma_flags_test_any_mask(&mut (*vma).flags, VMA_STICKY_FLAGS));
    }

    /*
     * Merge BOTH sides.
     *
     * 0123456789abc
     * AAAAAAAAA*CCC
     */
    vma = try_merge_new_vma(&mut mm, &mut vmg, 0x9000, 0xa000, 0x9, vma_flags, &mut merged);
    ASSERT_EQ(vma, vma_a);
    /* Extend A and delete C. */
    ASSERT_TRUE(merged);
    ASSERT_EQ((*vma).vm_start, 0);
    ASSERT_EQ((*vma).vm_end, 0xc000);
    ASSERT_EQ(vma_start_pgoff(vma), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 0);
    ASSERT_EQ((*vma).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE(vma_write_started(vma));
    ASSERT_EQ(mm.map_count, 1);
    if is_sticky || a_is_sticky || c_is_sticky {
        ASSERT_TRUE(vma_flags_test_any_mask(&mut (*vma).flags, VMA_STICKY_FLAGS));
    }

    /*
     * Final state.
     *
     * 0123456789abc
     * AAAAAAAAAAAAA
     */
    count = 0;
    vma_iter_set(&mut vmi, 0);
    loop {
        vma = vma_next(&mut vmi);
        if vma.is_null() {
            break;
        }
        ASSERT_NE(vma, core::ptr::null_mut());
        ASSERT_EQ((*vma).vm_start, 0);
        ASSERT_EQ((*vma).vm_end, 0xc000);
        ASSERT_EQ(vma_start_pgoff(vma), 0);
        ASSERT_EQ(vma_start_anon_pgoff(vma), 0);
        ASSERT_EQ((*vma).anon_vma, &mut dummy_anon_vma);

        detach_free_vma(vma);
        count += 1;
    }

    /* Should only have one VMA left (though freed) after all is done.*/
    ASSERT_EQ(count, 1);

    mtree_destroy(&mut mm.mm_mt);
    true
}

unsafe fn test_merge_new() -> bool {
    let mut i: i32 = 0;
    while i < 2 {
        let mut j: i32 = 0;
        while j < 2 {
            let mut k: i32 = 0;
            while k < 2 {
                let mut l: i32 = 0;
                while l < 2 {
                    ASSERT_TRUE(__test_merge_new(i != 0, j != 0, k != 0, l != 0));
                    l += 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    true
}

unsafe fn test_vma_merge_special_flags() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let mut vmi = VMA_ITERATOR(&mut mm, 0);
    let mut vmg = vma_merge_struct { mm: &mut mm, vmi: &mut vmi, ..core::mem::zeroed() };
    let special_flags: [vma_flag_t; 4] = [VMA_IO_BIT, VMA_DONTEXPAND_BIT, VMA_PFNMAP_BIT, VMA_MIXEDMAP_BIT];
    let mut all_special_flags: vma_flags_t = EMPTY_VMA_FLAGS;
    let mut i: usize;
    let vma_left: *mut vm_area_struct;
    let mut vma: *mut vm_area_struct;

    /* Make sure there aren't new VM_SPECIAL flags. */
    i = 0;
    while i < special_flags.len() {
        vma_flags_set(&mut all_special_flags, special_flags[i]);
        i += 1;
    }
    ASSERT_FLAGS_SAME_MASK(&mut all_special_flags, VMA_SPECIAL_FLAGS);

    /*
     * 01234
     * AAA
     */
    vma_left = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    ASSERT_NE(vma_left, core::ptr::null_mut());

    /* 1. Set up new VMA with special flag that would otherwise merge. */
    /*
     * 01234
     * AAA*
     *
     * This should merge if not for the VM_SPECIAL flag.
     */
    vmg_set_range(&mut vmg, 0x3000, 0x4000, 3, vma_flags);
    i = 0;
    while i < special_flags.len() {
        let special_flag: vma_flag_t = special_flags[i];
        let mut flags: vma_flags_t = vma_flags;

        vma_flags_set(&mut flags, special_flag);
        (*vma_left).flags = flags;
        vmg.vma_flags = flags;
        vma = merge_new(&mut vmg);
        ASSERT_EQ(vma, core::ptr::null_mut());
        ASSERT_EQ(vmg.state, VMA_MERGE_NOMERGE);
        i += 1;
    }

    /* 2. Modify VMA with special flag that would otherwise merge. */
    /*
     * 01234
     * AAAB
     *
     * Create a VMA to modify.
     */
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x4000, 3, vma_flags);
    ASSERT_NE(vma, core::ptr::null_mut());
    vmg.middle = vma;

    i = 0;
    while i < special_flags.len() {
        let special_flag: vma_flag_t = special_flags[i];
        let mut flags: vma_flags_t = vma_flags;

        vma_flags_set(&mut flags, special_flag);
        (*vma_left).flags = flags;
        vmg.vma_flags = flags;
        vma = merge_existing(&mut vmg);
        ASSERT_EQ(vma, core::ptr::null_mut());
        ASSERT_EQ(vmg.state, VMA_MERGE_NOMERGE);
        i += 1;
    }

    cleanup_mm(&mut mm, &mut vmi);
    true
}

unsafe fn test_vma_merge_with_close() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let mut vmi = VMA_ITERATOR(&mut mm, 0);
    let mut vmg = vma_merge_struct { mm: &mut mm, vmi: &mut vmi, ..core::mem::zeroed() };
    let vm_ops = vm_operations_struct { close: dummy_close, ..core::mem::zeroed() };
    let mut vma_prev: *mut vm_area_struct;
    let mut vma_next: *mut vm_area_struct;
    let mut vma: *mut vm_area_struct;

    /*
     * When merging VMAs we are not permitted to remove any VMA that has a
     * vm_ops->close() hook.
     */
    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x5000, 0x9000, 5, vma_flags);
    (*vma_next).vm_ops = &vm_ops as *const _;

    vmg_set_range(&mut vmg, 0x3000, 0x5000, 3, vma_flags);
    ASSERT_EQ(merge_new(&mut vmg), vma_prev);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);
    ASSERT_EQ((*vma_prev).vm_start, 0);
    ASSERT_EQ((*vma_prev).vm_end, 0x5000);
    ASSERT_EQ(vma_start_pgoff(vma_prev), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma_prev), 0);

    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 2);

    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x5000, 3, vma_flags);
    (*vma).vm_ops = &vm_ops as *const _;

    vmg_set_range(&mut vmg, 0x3000, 0x5000, 3, vma_flags);
    vmg.prev = vma_prev;
    vmg.middle = vma;

    ASSERT_EQ(merge_existing(&mut vmg), core::ptr::null_mut());
    ASSERT_EQ(vmg.state, VMA_MERGE_NOMERGE);

    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 2);

    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x5000, 3, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x5000, 0x9000, 5, vma_flags);
    (*vma).vm_ops = &vm_ops as *const _;

    vmg_set_range(&mut vmg, 0x3000, 0x5000, 3, vma_flags);
    vmg.middle = vma;
    ASSERT_EQ(merge_existing(&mut vmg), core::ptr::null_mut());
    ASSERT_EQ(vmg.state, VMA_MERGE_NOMERGE);

    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 2);

    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x5000, 3, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x5000, 0x9000, 5, vma_flags);
    (*vma).vm_ops = &vm_ops as *const _;

    vmg_set_range(&mut vmg, 0x3000, 0x5000, 3, vma_flags);
    vmg.prev = vma_prev;
    vmg.middle = vma;

    ASSERT_EQ(merge_existing(&mut vmg), core::ptr::null_mut());
    ASSERT_EQ(vmg.state, VMA_MERGE_NOMERGE);

    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 3);

    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x5000, 3, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x5000, 0x9000, 5, vma_flags);
    (*vma_next).vm_ops = &vm_ops as *const _;

    vmg_set_range(&mut vmg, 0x3000, 0x5000, 3, vma_flags);
    vmg.prev = vma_prev;
    vmg.middle = vma;

    ASSERT_EQ(merge_existing(&mut vmg), vma_prev);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);
    ASSERT_EQ((*vma_prev).vm_start, 0);
    ASSERT_EQ((*vma_prev).vm_end, 0x5000);
    ASSERT_EQ(vma_start_pgoff(vma_prev), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma_prev), 0);

    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 2);

    true
}

unsafe fn test_vma_merge_new_with_close() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let mut vmi = VMA_ITERATOR(&mut mm, 0);
    let mut vmg = vma_merge_struct { mm: &mut mm, vmi: &mut vmi, ..core::mem::zeroed() };
    let vma_prev: *mut vm_area_struct = alloc_and_link_vma(&mut mm, 0, 0x2000, 0, vma_flags);
    let vma_next: *mut vm_area_struct = alloc_and_link_vma(&mut mm, 0x5000, 0x7000, 5, vma_flags);
    let vm_ops = vm_operations_struct { close: dummy_close, ..core::mem::zeroed() };
    let vma: *mut vm_area_struct;

    /*
     * We should allow the partial merge of a proposed new VMA if the
     * surrounding VMAs have vm_ops->close() hooks (but are otherwise
     * compatible).
     */
    (*vma_prev).vm_ops = &vm_ops as *const _;
    (*vma_next).vm_ops = &vm_ops as *const _;

    vmg_set_range(&mut vmg, 0x2000, 0x5000, 2, vma_flags);
    vma = merge_new(&mut vmg);
    ASSERT_NE(vma, core::ptr::null_mut());
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);
    ASSERT_EQ((*vma).vm_start, 0);
    ASSERT_EQ((*vma).vm_end, 0x5000);
    ASSERT_EQ(vma_start_pgoff(vma), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 0);
    ASSERT_EQ((*vma).vm_ops, &vm_ops as *const _);
    ASSERT_TRUE(vma_write_started(vma));
    ASSERT_EQ(mm.map_count, 2);

    cleanup_mm(&mut mm, &mut vmi);
    true
}

unsafe fn __test_merge_existing(prev_is_sticky: bool, middle_is_sticky: bool, next_is_sticky: bool) -> bool {
    let mut vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut prev_flags: vma_flags_t = vma_flags;
    let mut next_flags: vma_flags_t = vma_flags;
    let mut mm: mm_struct = core::mem::zeroed();
    let mut vmi = VMA_ITERATOR(&mut mm, 0);
    let mut vma: *mut vm_area_struct;
    let mut vma_prev: *mut vm_area_struct;
    let mut vma_next: *mut vm_area_struct;
    let mut vmg = vma_merge_struct { mm: &mut mm, vmi: &mut vmi, ..core::mem::zeroed() };
    let vm_ops = vm_operations_struct { close: dummy_close, ..core::mem::zeroed() };
    let mut avc: anon_vma_chain = core::mem::zeroed();

    if prev_is_sticky { vma_flags_set_mask(&mut prev_flags, VMA_STICKY_FLAGS); }
    if middle_is_sticky { vma_flags_set_mask(&mut vma_flags, VMA_STICKY_FLAGS); }
    if next_is_sticky { vma_flags_set_mask(&mut next_flags, VMA_STICKY_FLAGS); }

    /*
     * Merge right case - partial span.
     */
    vma = alloc_and_link_vma(&mut mm, 0x2000, 0x6000, 2, vma_flags);
    (*vma).vm_ops = &vm_ops as *const _;
    vma_next = alloc_and_link_vma(&mut mm, 0x6000, 0x9000, 6, next_flags);
    (*vma_next).vm_ops = &vm_ops as *const _;
    vmg_set_range_anon_vma(&mut vmg, 0x3000, 0x6000, 3, vma_flags, &mut dummy_anon_vma);
    vmg.middle = vma;
    vmg.prev = vma;
    vma_set_dummy_anon_vma(vma, &mut avc);
    ASSERT_EQ(merge_existing(&mut vmg), vma_next);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);
    ASSERT_EQ((*vma_next).vm_start, 0x3000);
    ASSERT_EQ((*vma_next).vm_end, 0x9000);
    ASSERT_EQ(vma_start_pgoff(vma_next), 3);
    ASSERT_EQ(vma_start_anon_pgoff(vma_next), 3);
    ASSERT_EQ((*vma_next).anon_vma, &mut dummy_anon_vma);
    ASSERT_EQ((*vma).vm_start, 0x2000);
    ASSERT_EQ((*vma).vm_end, 0x3000);
    ASSERT_EQ(vma_start_pgoff(vma), 2);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 2);
    ASSERT_TRUE(vma_write_started(vma));
    ASSERT_TRUE(vma_write_started(vma_next));
    ASSERT_EQ(mm.map_count, 2);
    if middle_is_sticky || next_is_sticky {
        ASSERT_TRUE(vma_flags_test_any_mask(&mut (*vma_next).flags, VMA_STICKY_FLAGS));
    }
    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 2);

    /*
     * Merge right case - full span.
     */
    vma = alloc_and_link_vma(&mut mm, 0x2000, 0x6000, 2, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x6000, 0x9000, 6, next_flags);
    (*vma_next).vm_ops = &vm_ops as *const _;
    vmg_set_range_anon_vma(&mut vmg, 0x2000, 0x6000, 2, vma_flags, &mut dummy_anon_vma);
    vmg.middle = vma;
    vma_set_dummy_anon_vma(vma, &mut avc);
    ASSERT_EQ(merge_existing(&mut vmg), vma_next);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);
    ASSERT_EQ((*vma_next).vm_start, 0x2000);
    ASSERT_EQ((*vma_next).vm_end, 0x9000);
    ASSERT_EQ(vma_start_pgoff(vma_next), 2);
    ASSERT_EQ(vma_start_anon_pgoff(vma_next), 2);
    ASSERT_EQ((*vma_next).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE(vma_write_started(vma_next));
    ASSERT_EQ(mm.map_count, 1);
    if middle_is_sticky || next_is_sticky {
        ASSERT_TRUE(vma_flags_test_any_mask(&mut (*vma_next).flags, VMA_STICKY_FLAGS));
    }
    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 1);

    /*
     * Merge left case - partial span.
     */
    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, prev_flags);
    (*vma_prev).vm_ops = &vm_ops as *const _;
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x7000, 3, vma_flags);
    (*vma).vm_ops = &vm_ops as *const _;
    vmg_set_range_anon_vma(&mut vmg, 0x3000, 0x6000, 3, vma_flags, &mut dummy_anon_vma);
    vmg.prev = vma_prev;
    vmg.middle = vma;
    vma_set_dummy_anon_vma(vma, &mut avc);
    ASSERT_EQ(merge_existing(&mut vmg), vma_prev);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);
    ASSERT_EQ((*vma_prev).vm_start, 0);
    ASSERT_EQ((*vma_prev).vm_end, 0x6000);
    ASSERT_EQ(vma_start_pgoff(vma_prev), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma_prev), 0);
    ASSERT_EQ((*vma_prev).anon_vma, &mut dummy_anon_vma);
    ASSERT_EQ((*vma).vm_start, 0x6000);
    ASSERT_EQ((*vma).vm_end, 0x7000);
    ASSERT_EQ(vma_start_pgoff(vma), 6);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 6);
    ASSERT_TRUE(vma_write_started(vma_prev));
    ASSERT_TRUE(vma_write_started(vma));
    ASSERT_EQ(mm.map_count, 2);
    if prev_is_sticky || middle_is_sticky {
        ASSERT_TRUE(vma_flags_test_any_mask(&mut (*vma_prev).flags, VMA_STICKY_FLAGS));
    }
    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 2);

    /*
     * Merge left case - full span.
     */
    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, prev_flags);
    (*vma_prev).vm_ops = &vm_ops as *const _;
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x7000, 3, vma_flags);
    vmg_set_range_anon_vma(&mut vmg, 0x3000, 0x7000, 3, vma_flags, &mut dummy_anon_vma);
    vmg.prev = vma_prev;
    vmg.middle = vma;
    vma_set_dummy_anon_vma(vma, &mut avc);
    ASSERT_EQ(merge_existing(&mut vmg), vma_prev);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);
    ASSERT_EQ((*vma_prev).vm_start, 0);
    ASSERT_EQ((*vma_prev).vm_end, 0x7000);
    ASSERT_EQ(vma_start_pgoff(vma_prev), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma_prev), 0);
    ASSERT_EQ((*vma_prev).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE(vma_write_started(vma_prev));
    ASSERT_EQ(mm.map_count, 1);
    if prev_is_sticky || middle_is_sticky {
        ASSERT_TRUE(vma_flags_test_any_mask(&mut (*vma_prev).flags, VMA_STICKY_FLAGS));
    }
    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 1);

    /*
     * Merge both case.
     */
    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, prev_flags);
    (*vma_prev).vm_ops = &vm_ops as *const _;
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x7000, 3, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x7000, 0x9000, 7, next_flags);
    vmg_set_range_anon_vma(&mut vmg, 0x3000, 0x7000, 3, vma_flags, &mut dummy_anon_vma);
    vmg.prev = vma_prev;
    vmg.middle = vma;
    vma_set_dummy_anon_vma(vma, &mut avc);
    ASSERT_EQ(merge_existing(&mut vmg), vma_prev);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);
    ASSERT_EQ((*vma_prev).vm_start, 0);
    ASSERT_EQ((*vma_prev).vm_end, 0x9000);
    ASSERT_EQ(vma_start_pgoff(vma_prev), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma_prev), 0);
    ASSERT_EQ((*vma_prev).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE(vma_write_started(vma_prev));
    ASSERT_EQ(mm.map_count, 1);
    if prev_is_sticky || middle_is_sticky || next_is_sticky {
        ASSERT_TRUE(vma_flags_test_any_mask(&mut (*vma_prev).flags, VMA_STICKY_FLAGS));
    }
    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 1);

    /*
     * Non-merge ranges.
     */
    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, prev_flags);
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x8000, 3, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x8000, 0xa000, 8, next_flags);

    vmg_set_range(&mut vmg, 0x4000, 0x5000, 4, vma_flags);
    vmg.prev = vma;
    vmg.middle = vma;
    ASSERT_EQ(merge_existing(&mut vmg), core::ptr::null_mut());
    ASSERT_EQ(vmg.state, VMA_MERGE_NOMERGE);

    vmg_set_range(&mut vmg, 0x5000, 0x6000, 5, vma_flags);
    vmg.prev = vma;
    vmg.middle = vma;
    ASSERT_EQ(merge_existing(&mut vmg), core::ptr::null_mut());
    ASSERT_EQ(vmg.state, VMA_MERGE_NOMERGE);

    vmg_set_range(&mut vmg, 0x6000, 0x7000, 6, vma_flags);
    vmg.prev = vma;
    vmg.middle = vma;
    ASSERT_EQ(merge_existing(&mut vmg), core::ptr::null_mut());
    ASSERT_EQ(vmg.state, VMA_MERGE_NOMERGE);

    vmg_set_range(&mut vmg, 0x4000, 0x7000, 4, vma_flags);
    vmg.prev = vma;
    vmg.middle = vma;
    ASSERT_EQ(merge_existing(&mut vmg), core::ptr::null_mut());
    ASSERT_EQ(vmg.state, VMA_MERGE_NOMERGE);

    vmg_set_range(&mut vmg, 0x4000, 0x6000, 4, vma_flags);
    vmg.prev = vma;
    vmg.middle = vma;
    ASSERT_EQ(merge_existing(&mut vmg), core::ptr::null_mut());
    ASSERT_EQ(vmg.state, VMA_MERGE_NOMERGE);

    vmg_set_range(&mut vmg, 0x5000, 0x6000, 5, vma_flags);
    vmg.prev = vma;
    vmg.middle = vma;
    ASSERT_EQ(merge_existing(&mut vmg), core::ptr::null_mut());
    ASSERT_EQ(vmg.state, VMA_MERGE_NOMERGE);

    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 3);

    true
}

unsafe fn test_merge_existing() -> bool {
    let mut i: i32 = 0;
    while i < 2 {
        let mut j: i32 = 0;
        while j < 2 {
            let mut k: i32 = 0;
            while k < 2 {
                ASSERT_TRUE(__test_merge_existing(i != 0, j != 0, k != 0));
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    true
}

unsafe fn test_anon_vma_non_mergeable() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let mut vmi = VMA_ITERATOR(&mut mm, 0);
    let mut vma: *mut vm_area_struct;
    let mut vma_prev: *mut vm_area_struct;
    let mut vma_next: *mut vm_area_struct;
    let mut vmg = vma_merge_struct { mm: &mut mm, vmi: &mut vmi, ..core::mem::zeroed() };
    let mut dummy_anon_vma_chain_1: anon_vma_chain = core::mem::zeroed();
    let mut dummy_anon_vma_chain_2: anon_vma_chain = core::mem::zeroed();
    let mut dummy_anon_vma_2: anon_vma = core::mem::zeroed();

    /*
     * In the case of modified VMA merge, merging both left and right VMAs
     * but where prev and next have incompatible anon_vma objects, we revert
     * to a merge of prev and VMA.
     */
    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x7000, 3, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x7000, 0x9000, 7, vma_flags);

    vmg_set_range_anon_vma(&mut vmg, 0x3000, 0x7000, 3, vma_flags, core::ptr::null_mut());
    vmg.prev = vma_prev;
    vmg.middle = vma;
    vma_set_dummy_anon_vma(vma_prev, &mut dummy_anon_vma_chain_1);
    __vma_set_dummy_anon_vma(vma_next, &mut dummy_anon_vma_chain_2, &mut dummy_anon_vma_2);

    ASSERT_EQ(merge_existing(&mut vmg), vma_prev);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);
    ASSERT_EQ((*vma_prev).vm_start, 0);
    ASSERT_EQ((*vma_prev).vm_end, 0x7000);
    ASSERT_EQ(vma_start_pgoff(vma_prev), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma_prev), 0);
    ASSERT_TRUE(vma_write_started(vma_prev));
    ASSERT_FALSE(vma_write_started(vma_next));

    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 2);

    /*
     * Now consider the new VMA case. This is equivalent, only adding a new
     * VMA in a gap between prev and next.
     */
    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x7000, 0x9000, 7, vma_flags);

    vmg_set_range_anon_vma(&mut vmg, 0x3000, 0x7000, 3, vma_flags, core::ptr::null_mut());
    vmg.prev = vma_prev;
    vma_set_dummy_anon_vma(vma_prev, &mut dummy_anon_vma_chain_1);
    __vma_set_dummy_anon_vma(vma_next, &mut dummy_anon_vma_chain_2, &mut dummy_anon_vma_2);

    vmg.anon_vma = core::ptr::null_mut();
    ASSERT_EQ(merge_new(&mut vmg), vma_prev);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);
    ASSERT_EQ((*vma_prev).vm_start, 0);
    ASSERT_EQ((*vma_prev).vm_end, 0x7000);
    ASSERT_EQ(vma_start_pgoff(vma_prev), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma_prev), 0);
    ASSERT_TRUE(vma_write_started(vma_prev));
    ASSERT_FALSE(vma_write_started(vma_next));

    ASSERT_EQ(cleanup_mm(&mut mm, &mut vmi), 2);

    true
}

unsafe fn test_dup_anon_vma() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let mut vmi = VMA_ITERATOR(&mut mm, 0);
    let mut vmg = vma_merge_struct { mm: &mut mm, vmi: &mut vmi, ..core::mem::zeroed() };
    let mut dummy_anon_vma_chain = anon_vma_chain { anon_vma: &mut dummy_anon_vma, ..core::mem::zeroed() };
    let mut vma_prev: *mut vm_area_struct;
    let mut vma_next: *mut vm_area_struct;
    let mut vma: *mut vm_area_struct;

    reset_dummy_anon_vma();

    /*
     * Expanding a VMA delete the next one duplicates next's anon_vma and
     * assigns it to the expanded VMA.
     */
    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x3000, 0x5000, 3, vma_flags);
    (*vma_next).anon_vma = &mut dummy_anon_vma;

    vmg_set_range(&mut vmg, 0, 0x5000, 0, vma_flags);
    vmg.target = vma_prev;
    vmg.next = vma_next;

    ASSERT_EQ(expand_existing(&mut vmg), 0);

    /* Will have been cloned. */
    ASSERT_EQ((*vma_prev).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE((*(*vma_prev).anon_vma).was_cloned);

    cleanup_mm(&mut mm, &mut vmi);

    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x5000, 3, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x5000, 0x8000, 5, vma_flags);

    /* Initialise avc so mergeability check passes. */
    INIT_LIST_HEAD(&mut (*vma_next).anon_vma_chain);
    list_add(&mut dummy_anon_vma_chain.same_vma, &mut (*vma_next).anon_vma_chain);

    (*vma_next).anon_vma = &mut dummy_anon_vma;
    vmg_set_range(&mut vmg, 0x3000, 0x5000, 3, vma_flags);
    vmg.prev = vma_prev;
    vmg.middle = vma;

    ASSERT_EQ(merge_existing(&mut vmg), vma_prev);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);

    ASSERT_EQ((*vma_prev).vm_start, 0);
    ASSERT_EQ((*vma_prev).vm_end, 0x8000);

    ASSERT_EQ((*vma_prev).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE((*(*vma_prev).anon_vma).was_cloned);

    cleanup_mm(&mut mm, &mut vmi);

    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x5000, 3, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x5000, 0x8000, 5, vma_flags);
    vmg.anon_vma = &mut dummy_anon_vma;
    vma_set_dummy_anon_vma(vma, &mut dummy_anon_vma_chain);
    vmg_set_range(&mut vmg, 0x3000, 0x5000, 3, vma_flags);
    vmg.prev = vma_prev;
    vmg.middle = vma;

    ASSERT_EQ(merge_existing(&mut vmg), vma_prev);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);

    ASSERT_EQ((*vma_prev).vm_start, 0);
    ASSERT_EQ((*vma_prev).vm_end, 0x8000);

    ASSERT_EQ((*vma_prev).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE((*(*vma_prev).anon_vma).was_cloned);

    cleanup_mm(&mut mm, &mut vmi);

    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x8000, 3, vma_flags);

    vma_set_dummy_anon_vma(vma, &mut dummy_anon_vma_chain);
    vmg_set_range(&mut vmg, 0x3000, 0x5000, 3, vma_flags);
    vmg.prev = vma_prev;
    vmg.middle = vma;

    ASSERT_EQ(merge_existing(&mut vmg), vma_prev);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);

    ASSERT_EQ((*vma_prev).vm_start, 0);
    ASSERT_EQ((*vma_prev).vm_end, 0x5000);

    ASSERT_EQ((*vma_prev).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE((*(*vma_prev).anon_vma).was_cloned);

    cleanup_mm(&mut mm, &mut vmi);

    vma = alloc_and_link_vma(&mut mm, 0, 0x5000, 0, vma_flags);
    vma_next = alloc_and_link_vma(&mut mm, 0x5000, 0x8000, 5, vma_flags);

    vma_set_dummy_anon_vma(vma, &mut dummy_anon_vma_chain);
    vmg_set_range(&mut vmg, 0x3000, 0x5000, 3, vma_flags);
    vmg.prev = vma;
    vmg.middle = vma;

    ASSERT_EQ(merge_existing(&mut vmg), vma_next);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);

    ASSERT_EQ((*vma_next).vm_start, 0x3000);
    ASSERT_EQ((*vma_next).vm_end, 0x8000);

    ASSERT_EQ((*vma_next).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE((*(*vma_next).anon_vma).was_cloned);

    cleanup_mm(&mut mm, &mut vmi);
    true
}

unsafe fn test_vmi_prealloc_fail() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let mut vmi = VMA_ITERATOR(&mut mm, 0);
    let mut vmg = vma_merge_struct { mm: &mut mm, vmi: &mut vmi, ..core::mem::zeroed() };
    let mut avc: anon_vma_chain = core::mem::zeroed();
    let mut vma_prev: *mut vm_area_struct;
    let mut vma: *mut vm_area_struct;

    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x5000, 3, vma_flags);
    (*vma).anon_vma = &mut dummy_anon_vma;

    vmg_set_range_anon_vma(&mut vmg, 0x3000, 0x5000, 3, vma_flags, &mut dummy_anon_vma);
    vmg.prev = vma_prev;
    vmg.middle = vma;
    vma_set_dummy_anon_vma(vma, &mut avc);

    fail_prealloc = true;

    /* This will cause the merge to fail. */
    ASSERT_EQ(merge_existing(&mut vmg), core::ptr::null_mut());
    ASSERT_EQ(vmg.state, VMA_MERGE_ERROR_NOMEM);
    /* We will already have assigned the anon_vma. */
    ASSERT_EQ((*vma_prev).anon_vma, &mut dummy_anon_vma);
    /* And it was both cloned and unlinked. */
    ASSERT_TRUE(dummy_anon_vma.was_cloned);
    ASSERT_TRUE(dummy_anon_vma.was_unlinked);

    cleanup_mm(&mut mm, &mut vmi); /* Resets fail_prealloc too. */

    vma_prev = alloc_and_link_vma(&mut mm, 0, 0x3000, 0, vma_flags);
    vma = alloc_and_link_vma(&mut mm, 0x3000, 0x5000, 3, vma_flags);
    (*vma).anon_vma = &mut dummy_anon_vma;

    vmg_set_range(&mut vmg, 0, 0x5000, 3, vma_flags);
    vmg.target = vma_prev;
    vmg.next = vma;

    fail_prealloc = true;
    ASSERT_EQ(expand_existing(&mut vmg), -ENOMEM);
    ASSERT_EQ(vmg.state, VMA_MERGE_ERROR_NOMEM);

    ASSERT_EQ((*vma_prev).anon_vma, &mut dummy_anon_vma);
    ASSERT_TRUE(dummy_anon_vma.was_cloned);
    ASSERT_TRUE(dummy_anon_vma.was_unlinked);

    cleanup_mm(&mut mm, &mut vmi);
    true
}

unsafe fn test_merge_extend() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let mut vmi = VMA_ITERATOR(&mut mm, 0x1000);
    let vma: *mut vm_area_struct;

    vma = alloc_and_link_vma(&mut mm, 0, 0x1000, 0, vma_flags);
    alloc_and_link_vma(&mut mm, 0x3000, 0x4000, 3, vma_flags);

    /*
     * Extend a VMA into the gap between itself and the following VMA.
     * This should result in a merge.
     */
    ASSERT_EQ(vma_merge_extend(&mut vmi, vma, 0x2000), vma);
    ASSERT_EQ((*vma).vm_start, 0);
    ASSERT_EQ((*vma).vm_end, 0x4000);
    ASSERT_EQ(vma_start_pgoff(vma), 0);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 0);
    ASSERT_TRUE(vma_write_started(vma));
    ASSERT_EQ(mm.map_count, 1);

    cleanup_mm(&mut mm, &mut vmi);
    true
}

unsafe fn test_expand_only_mode() -> bool {
    let vma_flags: vma_flags_t = mk_vma_flags(VMA_READ_BIT, VMA_WRITE_BIT, VMA_MAYREAD_BIT, VMA_MAYWRITE_BIT);
    let mut mm: mm_struct = core::mem::zeroed();
    let mut vmi = VMA_ITERATOR(&mut mm, 0);
    let vma_prev: *mut vm_area_struct;
    let vma: *mut vm_area_struct;
    let mut vmg = VMG_STATE(&mut mm, &mut vmi, 0x5000, 0x9000, vma_flags, 5, 5);

    /*
     * Place a VMA prior to the one we're expanding so we assert that we do
     * not erroneously try to traverse to the previous VMA even though we
     * have, through the use of the just_expand flag, indicated we do not
     * need to do so.
     */
    alloc_and_link_vma(&mut mm, 0, 0x2000, 0, vma_flags);

    /*
     * We will be positioned at the prev VMA, but looking to expand to
     * 0x9000.
     */
    vma_iter_set(&mut vmi, 0x3000);
    vma_prev = alloc_and_link_vma(&mut mm, 0x3000, 0x5000, 3, vma_flags);
    vmg.prev = vma_prev;
    vmg.just_expand = true;

    vma = vma_merge_new_range(&mut vmg);
    ASSERT_NE(vma, core::ptr::null_mut());
    ASSERT_EQ(vma, vma_prev);
    ASSERT_EQ(vmg.state, VMA_MERGE_SUCCESS);
    ASSERT_EQ((*vma).vm_start, 0x3000);
    ASSERT_EQ((*vma).vm_end, 0x9000);
    ASSERT_EQ(vma_start_pgoff(vma), 3);
    ASSERT_EQ(vma_start_anon_pgoff(vma), 3);
    ASSERT_TRUE(vma_write_started(vma));
    ASSERT_EQ(vma_iter_addr(&mut vmi), 0x3000);
    vma_assert_attached(vma);

    cleanup_mm(&mut mm, &mut vmi);
    true
}

unsafe fn run_merge_tests(num_tests: *mut i32, num_fail: *mut i32) {
    /* Very simple tests to kick the tyres. */
    TEST(simple_merge);
    TEST(simple_modify);
    TEST(simple_expand);
    TEST(simple_shrink);

    TEST(merge_new);
    TEST(vma_merge_special_flags);
    TEST(vma_merge_with_close);
    TEST(vma_merge_new_with_close);
    TEST(merge_existing);
    TEST(anon_vma_non_mergeable);
    TEST(dup_anon_vma);
    TEST(vmi_prealloc_fail);
    TEST(merge_extend);
    TEST(expand_only_mode);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
