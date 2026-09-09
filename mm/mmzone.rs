// SPDX-License-Identifier: GPL-2.0
/*
 * linux/mm/mmzone.c
 *
 * management codes for pgdats, zones and page flags
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn first_online_pgdat() -> *mut pglist_data {
    NODE_DATA(first_online_node)
}

pub unsafe fn next_online_pgdat(pgdat: *mut pglist_data) -> *mut pglist_data {
    let nid: i32 = next_online_node((*pgdat).node_id);

    if nid == MAX_NUMNODES {
        core::ptr::null_mut()
    } else {
        NODE_DATA(nid)
    }
}

/*
 * next_zone - helper magic for for_each_zone()
 */
pub unsafe fn next_zone(mut zone: *mut zone) -> *mut zone {
    let mut pgdat: *mut pglist_data = (*zone).zone_pgdat;

    if zone < (*pgdat).node_zones.as_mut_ptr().add(MAX_NR_ZONES - 1) {
        zone = zone.add(1);
    } else {
        pgdat = next_online_pgdat(pgdat);
        if !pgdat.is_null() {
            zone = (*pgdat).node_zones.as_mut_ptr();
        } else {
            zone = core::ptr::null_mut();
        }
    }
    zone
}

unsafe fn zref_in_nodemask(zref: *mut zoneref, nodes: *const nodemask_t) -> i32 {
    // CONFIG_NUMA controls this branch in the C source.
    node_isset(zonelist_node_idx(zref), *nodes)
}

/* Returns the next zone at or below highest_zoneidx in a zonelist */
pub unsafe fn __next_zones_zonelist(
    mut z: *mut zoneref,
    highest_zoneidx: zone_type,
    nodes: *const nodemask_t,
) -> *mut zoneref {
    /*
     * Find the next suitable zone to use for the allocation.
     * Only filter based on nodemask if it's set
     */
    if nodes.is_null() {
        while zonelist_zone_idx(z) > highest_zoneidx {
            z = z.add(1);
        }
    } else {
        while zonelist_zone_idx(z) > highest_zoneidx
            || (zonelist_zone(z) != core::ptr::null_mut() && zref_in_nodemask(z, nodes) == 0)
        {
            z = z.add(1);
        }
    }

    z
}

pub unsafe fn lruvec_init(lruvec: *mut lruvec) {
    let mut lru: lru_list;

    core::ptr::write_bytes(lruvec.cast::<u8>(), 0, core::mem::size_of::<lruvec>());
    spin_lock_init(&mut (*lruvec).lru_lock);
    spin_lock_init(&mut (*lruvec).cost_lock);
    zswap_lruvec_state_init(lruvec);

    // for_each_lru(lru)
    lru = LRU_BASE;
    while lru <= LRU_UNEVICTABLE {
        INIT_LIST_HEAD(&mut (*lruvec).lists[lru as usize]);
        lru = lru + 1;
    }
    /*
     * The "Unevictable LRU" is imaginary: though its size is maintained,
     * it is never scanned, and unevictable pages are not threaded on it
     * (so that their lru fields can be reused to hold mlock_count).
     * Poison its list head, so that any operations on it would crash.
     */
    list_del(&mut (*lruvec).lists[LRU_UNEVICTABLE as usize]);

    lru_gen_init_lruvec(lruvec);
}

// This definition is present only when CONFIG_NUMA_BALANCING is enabled and
// LAST_CPUPID_NOT_IN_PAGE_FLAGS is not defined.
pub unsafe fn folio_xchg_last_cpupid(folio: *mut folio, cpupid: i32) -> i32 {
    let mut old_flags: usize = READ_ONCE((*folio).flags.f);
    let mut flags: usize;
    let mut last_cpupid: i32;

    loop {
        flags = old_flags;
        last_cpupid = ((flags >> LAST_CPUPID_PGSHIFT) & LAST_CPUPID_MASK) as i32;

        flags &= !(LAST_CPUPID_MASK << LAST_CPUPID_PGSHIFT);
        flags |= ((cpupid as usize) & LAST_CPUPID_MASK) << LAST_CPUPID_PGSHIFT;
        if try_cmpxchg(&mut (*folio).flags.f, &mut old_flags, flags) {
            break;
        }
    }

    last_cpupid
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
