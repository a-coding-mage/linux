// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PS3 pagetable management routines.
 *
 *  Copyright (C) 2006 Sony Computer Entertainment Inc.
 *  Copyright 2006, 2007 Sony Corporation
 */

// Dependencies supplied by the surrounding kernel translation unit.
// #define PS3_VERBOSE_RESULT

/**
 * enum lpar_vas_id - id of LPAR virtual address space.
 * @lpar_vas_id_current: Current selected virtual address space
 *
 * Identify the target LPAR address space.
 */
#[repr(C)]
enum Ps3LparVasId {
    PS3_LPAR_VAS_ID_CURRENT = 0,
}

static mut ps3_htab_lock: u8 = 0;

unsafe fn ps3_hpte_insert(
    hpte_group: c_ulong,
    vpn: c_ulong,
    pa: c_ulong,
    rflags: c_ulong,
    mut vflags: c_ulong,
    psize: c_int,
    apsize: c_int,
    ssize: c_int,
) -> c_long {
    let mut result: c_int;
    let hpte_v: u64;
    let hpte_r: u64;
    let mut inserted_index: u64 = 0;
    let mut evicted_v: u64 = 0;
    let mut evicted_r: u64 = 0;
    let mut hpte_v_array = [0u64; 4];
    let mut hpte_rs: u64 = 0;
    let mut flags: c_ulong = 0;
    let mut ret: c_long = -1;

    /* lv1_insert_htab_entry() will search for victim entry in both primary and secondary pte group */
    vflags &= !HPTE_V_SECONDARY;

    hpte_v = hpte_encode_v(vpn, psize, apsize, ssize) | vflags | HPTE_V_VALID;
    hpte_r = hpte_encode_r(ps3_mm_phys_to_lpar(pa), psize, apsize) | rflags;

    spin_lock_irqsave(&raw mut ps3_htab_lock, &mut flags);

    /* talk hvc to replace entries BOLTED == 0 */
    result = lv1_insert_htab_entry(
        PS3_LPAR_VAS_ID_CURRENT as c_ulong,
        hpte_group,
        hpte_v,
        hpte_r,
        HPTE_V_BOLTED,
        0,
        &mut inserted_index,
        &mut evicted_v,
        &mut evicted_r,
    );

    if result != 0 {
        /* all entries bolted !*/
        pr_info!("{}:result={} vpn={:x} pa={:x} ix={:x} v={:x} r={:x}\n", __func__, ps3_result(result), vpn, pa, hpte_group, hpte_v, hpte_r);
        BUG!();
    }

    /* see if the entry is inserted into secondary pteg */
    result = lv1_read_htab_entries(
        PS3_LPAR_VAS_ID_CURRENT as c_ulong,
        inserted_index & !0x3UL,
        &mut hpte_v_array[0],
        &mut hpte_v_array[1],
        &mut hpte_v_array[2],
        &mut hpte_v_array[3],
        &mut hpte_rs,
    );
    BUG_ON!(result);

    if hpte_v_array[(inserted_index % 4) as usize] & HPTE_V_SECONDARY != 0 {
        ret = ((inserted_index & 7) | (1 << 3)) as c_long;
    } else {
        ret = (inserted_index & 7) as c_long;
    }

    spin_unlock_irqrestore(&raw mut ps3_htab_lock, flags);
    ret
}

unsafe fn ps3_hpte_remove(_hpte_group: c_ulong) -> c_long {
    panic!("ps3_hpte_remove() not implemented");
}

unsafe fn ps3_hpte_updatepp(
    slot: c_ulong, newpp: c_ulong, vpn: c_ulong, psize: c_int, apsize: c_int,
    ssize: c_int, inv_flags: c_ulong,
) -> c_long {
    let mut result: c_int;
    let mut hpte_v: u64;
    let want_v: u64;
    let mut hpte_rs: u64 = 0;
    let mut hpte_v_array = [0u64; 4];
    let mut flags: c_ulong = 0;
    let ret: c_long;

    let _ = (newpp, apsize, inv_flags);
    want_v = hpte_encode_avpn(vpn, psize, ssize);

    spin_lock_irqsave(&raw mut ps3_htab_lock, &mut flags);
    result = lv1_read_htab_entries(
        PS3_LPAR_VAS_ID_CURRENT as c_ulong, slot & !0x3UL,
        &mut hpte_v_array[0], &mut hpte_v_array[1], &mut hpte_v_array[2],
        &mut hpte_v_array[3], &mut hpte_rs,
    );

    if result != 0 {
        pr_info!("{}: result={} read vpn={:x} slot={:x} psize={}\n", __func__, ps3_result(result), vpn, slot, psize);
        BUG!();
    }

    hpte_v = hpte_v_array[(slot % 4) as usize];

    /* As lv1_read_htab_entries() does not give us the RPN, we cannot synthesize the new hpte_r value here. */
    if !HPTE_V_COMPARE(hpte_v, want_v) || hpte_v & HPTE_V_VALID == 0 {
        ret = -1;
    } else {
        result = lv1_write_htab_entry(PS3_LPAR_VAS_ID_CURRENT as c_ulong, slot, 0, 0);
        let _ = result;
        ret = -1;
    }

    spin_unlock_irqrestore(&raw mut ps3_htab_lock, flags);
    ret
}

unsafe fn ps3_hpte_updateboltedpp(_newpp: c_ulong, _ea: c_ulong, _psize: c_int, _ssize: c_int) {
    pr_info!("ps3_hpte_updateboltedpp() not implemented");
}

unsafe fn ps3_hpte_invalidate(_slot: c_ulong, vpn: c_ulong, psize: c_int, _apsize: c_int, _ssize: c_int, _local: c_int) {
    let mut flags: c_ulong = 0;
    let result: c_int;

    spin_lock_irqsave(&raw mut ps3_htab_lock, &mut flags);
    result = lv1_write_htab_entry(PS3_LPAR_VAS_ID_CURRENT as c_ulong, _slot, 0, 0);
    if result != 0 {
        pr_info!("{}: result={} vpn={:x} slot={:x} psize={}\n", __func__, ps3_result(result), vpn, _slot, psize);
        BUG!();
    }
    spin_unlock_irqrestore(&raw mut ps3_htab_lock, flags);
}

/* Called during kexec sequence with MMU off */
unsafe fn ps3_hpte_clear() {
    let hpte_count: c_ulong = (1UL << ppc64_pft_size) >> 4;
    let mut i: u64 = 0;
    while i < hpte_count as u64 {
        lv1_write_htab_entry(PS3_LPAR_VAS_ID_CURRENT as c_ulong, i as c_ulong, 0, 0);
        i += 1;
    }
    ps3_mm_shutdown();
    ps3_mm_vas_destroy();
}

unsafe fn ps3_hpte_init(htab_size: c_ulong) {
    mmu_hash_ops.hpte_invalidate = Some(ps3_hpte_invalidate);
    mmu_hash_ops.hpte_updatepp = Some(ps3_hpte_updatepp);
    mmu_hash_ops.hpte_updateboltedpp = Some(ps3_hpte_updateboltedpp);
    mmu_hash_ops.hpte_insert = Some(ps3_hpte_insert);
    mmu_hash_ops.hpte_remove = Some(ps3_hpte_remove);
    mmu_hash_ops.hpte_clear_all = Some(ps3_hpte_clear);
    ppc64_pft_size = __ilog2(htab_size);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
