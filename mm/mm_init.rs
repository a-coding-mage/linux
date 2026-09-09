// SPDX-License-Identifier: GPL-2.0-only
/*
 * mm_init.c - Memory initialisation verification and debugging
 *
 * Copyright 2008 IBM Corporation, 2008
 * Author Mel Gorman <mel@csn.ul.ie>
 *
 */
// C dependencies supplied by the surrounding kernel translation unit.

#[cfg(not(CONFIG_NUMA))]
pub static mut max_mapnr: ::core::ffi::c_ulong = 0;

#[cfg(not(CONFIG_NUMA))]
pub static mut mem_map: *mut page = ::core::ptr::null_mut();

/*
 * high_memory defines the upper bound on direct map memory, then end
 * of ZONE_NORMAL.
 */
pub static mut high_memory: *mut ::core::ffi::c_void = ::core::ptr::null_mut();

pub static mut zero_page_pfn: ::core::ffi::c_ulong = 0;

#[cfg(not(__HAVE_COLOR_ZERO_PAGE))]
#[repr(align(4096))]
pub static empty_zero_page: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

#[cfg(not(__HAVE_COLOR_ZERO_PAGE))]
pub static mut __zero_page: *mut page = ::core::ptr::null_mut();

#[cfg(CONFIG_DEBUG_MEMORY_INIT)]
pub static mut mminit_loglevel: ::core::ffi::c_int = 0;

#[cfg(CONFIG_DEBUG_MEMORY_INIT)]
pub unsafe extern "C" fn mminit_verify_zonelist() {
    let nid: ::core::ffi::c_int;

    if mminit_loglevel < MMINIT_VERIFY {
        return;
    }

    // for_each_online_node(nid)
    for nid in online_nodes() {
        let pgdat: *mut pg_data_t = NODE_DATA(nid);

        for i in 0..(MAX_ZONELISTS * MAX_NR_ZONES) {
            /* Identify the zone and nodelist */
            let zoneid = i % MAX_NR_ZONES;
            let listid = i / MAX_NR_ZONES;
            let zonelist: *mut zonelist = &mut (*pgdat).node_zonelists[listid];
            let zone: *mut zone = &mut (*pgdat).node_zones[zoneid];
            if !populated_zone(zone) {
                continue;
            }

            /* Print information about the zonelist */
            printk(
                KERN_DEBUG,
                "mminit::zonelist %s %d:%s = ",
                if listid > 0 { "thisnode" } else { "general" },
                nid,
                (*zone).name,
            );

            /* Iterate the zonelist */
            // for_each_zone_zonelist(zone, z, zonelist, zoneid)
            for zone in zones_in_zonelist(zone, zonelist, zoneid) {
                pr_cont("%d:%s ", zone_to_nid(zone), (*zone).name);
            }
            pr_cont("\n");
        }
    }
}

#[cfg(CONFIG_DEBUG_MEMORY_INIT)]
pub unsafe extern "C" fn mminit_verify_pageflags_layout() {
    let mut shift: ::core::ffi::c_int = BITS_PER_LONG;
    let width = shift - NR_NON_PAGEFLAG_BITS;
    let mut or_mask: ::core::ffi::c_ulong;
    let mut add_mask: ::core::ffi::c_ulong;

    mminit_dprintk(MMINIT_TRACE, "pageflags_layout_widths",
        "Section %d Node %d Zone %d Lastcpupid %d Kasantag %d Gen %d Tier %d Flags %d\n",
        SECTIONS_WIDTH, NODES_WIDTH, ZONES_WIDTH, LAST_CPUPID_WIDTH,
        KASAN_TAG_WIDTH, LRU_GEN_WIDTH, LRU_REFS_WIDTH, NR_PAGEFLAGS);
    mminit_dprintk(MMINIT_TRACE, "pageflags_layout_shifts",
        "Section %d Node %d Zone %d Lastcpupid %d Kasantag %d\n",
        SECTIONS_SHIFT, NODES_SHIFT, ZONES_SHIFT, LAST_CPUPID_SHIFT,
        KASAN_TAG_WIDTH);
    mminit_dprintk(MMINIT_TRACE, "pageflags_layout_pgshifts",
        "Section %lu Node %lu Zone %lu Lastcpupid %lu Kasantag %lu\n",
        SECTIONS_PGSHIFT as ::core::ffi::c_ulong,
        NODES_PGSHIFT as ::core::ffi::c_ulong,
        ZONES_PGSHIFT as ::core::ffi::c_ulong,
        LAST_CPUPID_PGSHIFT as ::core::ffi::c_ulong,
        KASAN_TAG_PGSHIFT as ::core::ffi::c_ulong);
    mminit_dprintk(MMINIT_TRACE, "pageflags_layout_nodezoneid",
        "Node/Zone ID: %lu -> %lu\n",
        (ZONEID_PGOFF + ZONEID_SHIFT) as ::core::ffi::c_ulong,
        ZONEID_PGOFF as ::core::ffi::c_ulong);
    mminit_dprintk(MMINIT_TRACE, "pageflags_layout_usage",
        "location: %d -> %d layout %d -> %d unused %d -> %d page-flags\n",
        shift, width, width, NR_PAGEFLAGS, NR_PAGEFLAGS, 0);

    if SECTIONS_WIDTH != 0 {
        shift -= SECTIONS_WIDTH;
        BUG_ON(shift != SECTIONS_PGSHIFT);
    }
    if NODES_WIDTH != 0 {
        shift -= NODES_WIDTH;
        BUG_ON(shift != NODES_PGSHIFT);
    }
    if ZONES_WIDTH != 0 {
        shift -= ZONES_WIDTH;
        BUG_ON(shift != ZONES_PGSHIFT);
    }

    /* Check for bitmask overlaps */
    or_mask = (ZONES_MASK << ZONES_PGSHIFT) |
        (NODES_MASK << NODES_PGSHIFT) |
        (SECTIONS_MASK << SECTIONS_PGSHIFT);
    add_mask = (ZONES_MASK << ZONES_PGSHIFT) +
        (NODES_MASK << NODES_PGSHIFT) +
        (SECTIONS_MASK << SECTIONS_PGSHIFT);
    BUG_ON(or_mask != add_mask);
}

#[cfg(CONFIG_DEBUG_MEMORY_INIT)]
unsafe extern "C" fn set_mminit_loglevel(str_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    get_option(&mut str_, &mut mminit_loglevel);
    0
}

pub static mut mm_kobj: *mut kobject = ::core::ptr::null_mut();

#[cfg(CONFIG_SMP)]
pub static mut vm_committed_as_batch: s32 = 32;

#[cfg(CONFIG_SMP)]
pub unsafe extern "C" fn mm_compute_batch(overcommit_policy: ::core::ffi::c_int) {
    let memsized_batch: u64;
    let nr: s32 = num_present_cpus();
    let batch: s32 = core::cmp::max(nr.wrapping_mul(2), 32);
    let ram_pages: ::core::ffi::c_ulong = totalram_pages();

    /*
     * For policy OVERCOMMIT_NEVER, set batch size to 0.4% of
     * (total memory/#cpus), and lift it to 25% for other policies
     * to ease the possible lock contention for percpu_counter
     * vm_committed_as, while the max limit is INT_MAX
     */
    if overcommit_policy == OVERCOMMIT_NEVER {
        memsized_batch = core::cmp::min(ram_pages / nr as ::core::ffi::c_ulong / 256, INT_MAX as u64);
    } else {
        memsized_batch = core::cmp::min(ram_pages / nr as ::core::ffi::c_ulong / 4, INT_MAX as u64);
    }

    vm_committed_as_batch = core::cmp::max(memsized_batch as s32, batch);
}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn mm_compute_batch_notifier(
    _self: *mut notifier_block,
    action: ::core::ffi::c_ulong,
    _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    match action {
        MEM_ONLINE | MEM_OFFLINE => mm_compute_batch(sysctl_overcommit_memory),
        _ => {}
    }
    NOTIFY_OK
}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn mm_compute_batch_init() -> ::core::ffi::c_int {
    mm_compute_batch(sysctl_overcommit_memory);
    hotplug_memory_notifier(mm_compute_batch_notifier, MM_COMPUTE_BATCH_PRI);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
