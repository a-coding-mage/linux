// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023-2025 Christoph Hellwig.
 * Copyright (c) 2024-2025, Western Digital Corporation or its affiliates.
 */
// Dependencies supplied by the surrounding XFS translation.

static XFS_WRITE_HINT_SHORTHAND: [[u8; 16]; 6] = [
    *b"NOT_SET\0\0\0\0\0\0\0\0\0\0",
    *b"NONE\0\0\0\0\0\0\0\0\0\0\0\0",
    *b"SHORT\0\0\0\0\0\0\0\0\0\0\0",
    *b"MEDIUM\0\0\0\0\0\0\0\0\0\0\0",
    *b"LONG\0\0\0\0\0\0\0\0\0\0\0\0\0",
    *b"EXTREME\0\0\0\0\0\0\0\0\0",
];

#[inline]
unsafe fn xfs_write_hint_to_str(write_hint: u8) -> *const u8 {
    if write_hint > WRITE_LIFE_EXTREME {
        b"UNKNOWN\0".as_ptr()
    } else {
        XFS_WRITE_HINT_SHORTHAND[write_hint as usize].as_ptr()
    }
}

unsafe fn xfs_show_open_zone(m: *mut seq_file, oz: *mut xfs_open_zone) {
    seq_printf(
        m,
        b"\t  zone %d, wp %u, written %u, used %u, hint %s %s\n\0".as_ptr(),
        rtg_rgno((*oz).oz_rtg),
        (*oz).oz_allocated,
        (*oz).oz_written,
        (*rtg_rmap((*oz).oz_rtg)).i_used_blocks,
        xfs_write_hint_to_str((*oz).oz_write_hint),
        if (*oz).oz_is_gc { b"(GC)\0".as_ptr() } else { b"\0".as_ptr() },
    );
}

unsafe fn xfs_show_full_zone_used_distribution(
    m: *mut seq_file,
    mp: *mut xfs_mount,
) {
    let zi = (*mp).m_zone_info;
    let mut reclaimable: c_uint = 0;
    let mut full: c_uint;

    spin_lock(&mut (*zi).zi_used_buckets_lock);
    for i in 0..XFS_ZONE_USED_BUCKETS {
        let entries = (*zi).zi_used_bucket_entries[i as usize];
        seq_printf(
            m,
            b"\t  %2u..%2u%%: %u\n\0".as_ptr(),
            i * (100 / XFS_ZONE_USED_BUCKETS),
            (i + 1) * (100 / XFS_ZONE_USED_BUCKETS) - 1,
            entries,
        );
        reclaimable += entries;
    }
    spin_unlock(&mut (*zi).zi_used_buckets_lock);

    full = (*mp).m_sb.sb_rgcount;
    full -= (*zi).zi_nr_open_zones;
    full -= (*zi).zi_nr_open_gc_zones;
    full -= atomic_read(&(*zi).zi_nr_free_zones);
    full -= reclaimable;

    seq_printf(m, b"\t     100%%: %u\n\0".as_ptr(), full);
}

pub unsafe fn xfs_zoned_show_stats(m: *mut seq_file, mp: *mut xfs_mount) {
    let zi = (*mp).m_zone_info;
    let mut oz: *mut xfs_open_zone;

    seq_puts(m, b"\n\0".as_ptr());
    seq_printf(m, b"\tuser free RT blocks: %lld\n\0".as_ptr(), xfs_sum_freecounter(mp, XC_FREE_RTEXTENTS));
    seq_printf(m, b"\treserved free RT blocks: %lld\n\0".as_ptr(), (*mp).m_free[XC_FREE_RTEXTENTS].res_avail);
    seq_printf(m, b"\tuser available RT blocks: %lld\n\0".as_ptr(), xfs_sum_freecounter(mp, XC_FREE_RTAVAILABLE));
    seq_printf(m, b"\treserved available RT blocks: %lld\n\0".as_ptr(), (*mp).m_free[XC_FREE_RTAVAILABLE].res_avail);
    seq_printf(m, b"\tRT reservations required: %d\n\0".as_ptr(), !list_empty_careful(&(*zi).zi_reclaim_reservations));
    seq_printf(m, b"\tRT GC required: %d\n\0".as_ptr(), xfs_zoned_need_gc(mp));
    seq_printf(m, b"\ttotal number of zones: %u\n\0".as_ptr(), (*mp).m_sb.sb_rgcount);
    seq_printf(m, b"\tfree zones: %d\n\0".as_ptr(), atomic_read(&(*zi).zi_nr_free_zones));

    spin_lock(&mut (*zi).zi_open_zones_lock);
    seq_printf(m, b"\tmax open zones: %u\n\0".as_ptr(), (*mp).m_max_open_zones);
    seq_printf(m, b"\tnr open zones: %u\n\0".as_ptr(), (*zi).zi_nr_open_zones);
    seq_printf(m, b"\tnr open GC zones: %u\n\0".as_ptr(), (*zi).zi_nr_open_gc_zones);
    seq_puts(m, b"\topen zones:\n\0".as_ptr());
    list_for_each_entry!(oz, &mut (*zi).zi_open_zones, oz_entry, {
        xfs_show_open_zone(m, oz);
    });
    spin_unlock(&mut (*zi).zi_open_zones_lock);
    seq_puts(m, b"\tused blocks distribution (fully written zones):\n\0".as_ptr());
    xfs_show_full_zone_used_distribution(m, mp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
