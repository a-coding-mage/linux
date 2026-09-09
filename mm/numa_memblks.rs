// SPDX-License-Identifier: GPL-2.0-or-later

// Linux header dependencies are supplied by the surrounding translation unit.

pub static mut numa_distance_cnt: i32 = 0;
static mut numa_distance: *mut u8 = core::ptr::null_mut();

pub static mut numa_nodes_parsed: nodemask_t = NODE_MASK_NONE;

static mut numa_meminfo: numa_meminfo = numa_meminfo::default();
static mut numa_reserved_meminfo: numa_meminfo = numa_meminfo::default();

/**
 * numa_reset_distance - Reset NUMA distance table
 *
 * The current table is freed.  The next numa_set_distance() call will
 * create a new one.
 */
pub unsafe fn numa_reset_distance() {
    let size = (numa_distance_cnt as usize) * (numa_distance_cnt as usize)
        * core::mem::size_of::<u8>();

    /* numa_distance could be 1LU marking allocation failure, test cnt */
    if numa_distance_cnt != 0 {
        memblock_free(numa_distance, size);
    }
    numa_distance_cnt = 0;
    numa_distance = core::ptr::null_mut(); /* enable table creation */
}

unsafe fn numa_alloc_distance() -> i32 {
    let nodes_parsed = numa_nodes_parsed;
    let mut size: usize;
    let mut cnt: i32 = 0;

    /* size the new table and allocate it */
    for i in for_each_node_mask(nodes_parsed) {
        cnt = i;
    }
    cnt += 1;
    size = (cnt as usize) * (cnt as usize) * core::mem::size_of::<u8>();

    numa_distance = memblock_alloc(size, PAGE_SIZE);
    if numa_distance.is_null() {
        pr_warn!("Warning: can't allocate distance table!\n");
        /* don't retry until explicitly reset */
        numa_distance = 1usize as *mut u8;
        return -ENOMEM;
    }

    numa_distance_cnt = cnt;

    /* fill with the default distances */
    for i in 0..cnt {
        for j in 0..cnt {
            *numa_distance.add((i * cnt + j) as usize) =
                if i == j { LOCAL_DISTANCE } else { REMOTE_DISTANCE };
        }
    }
    pr_debug!("NUMA: Initialized distance table, cnt=%d\n", cnt);

    0
}

pub unsafe fn numa_set_distance(from: i32, to: i32, distance: i32) {
    if numa_distance.is_null() && numa_alloc_distance() < 0 {
        return;
    }

    if from >= numa_distance_cnt || to >= numa_distance_cnt || from < 0 || to < 0 {
        pr_warn_once!("Warning: node ids are out of bound, from=%d to=%d distance=%d\n", from, to, distance);
        return;
    }

    if (distance as u8) as i32 != distance || (from == to && distance != LOCAL_DISTANCE) {
        pr_warn_once!("Warning: invalid distance parameter, from=%d to=%d distance=%d\n", from, to, distance);
        return;
    }

    *numa_distance.add((from * numa_distance_cnt + to) as usize) = distance as u8;
}

pub unsafe fn __node_distance(from: i32, to: i32) -> i32 {
    if from >= numa_distance_cnt || to >= numa_distance_cnt {
        return if from == to { LOCAL_DISTANCE } else { REMOTE_DISTANCE };
    }
    *numa_distance.add((from * numa_distance_cnt + to) as usize) as i32
}

unsafe fn numa_add_memblk_to(nid: i32, start: u64, end: u64, mi: *mut numa_meminfo) -> i32 {
    /* whine about and ignore invalid nid */
    if nid < 0 || nid >= MAX_NUMNODES {
        pr_warn!("Warning: invalid memblk node id %d [mem %#010Lx-%#010Lx]\n", nid, start, end - 1);
        return -EINVAL;
    }
    /* ignore zero length blks */
    if start == end { return 0; }
    /* whine about and ignore invalid ranges */
    if start > end {
        pr_warn!("Warning: invalid memblk range for node %d [mem %#010Lx-%#010Lx]\n", nid, start, end - 1);
        return 0;
    }
    if (*mi).nr_blks >= NR_NODE_MEMBLKS {
        pr_err!("too many memblk ranges\n");
        return -EINVAL;
    }
    (*mi).blk[(*mi).nr_blks as usize].start = start;
    (*mi).blk[(*mi).nr_blks as usize].end = end;
    (*mi).blk[(*mi).nr_blks as usize].nid = nid;
    (*mi).nr_blks += 1;
    0
}

pub unsafe fn numa_remove_memblk_from(idx: i32, mi: *mut numa_meminfo) {
    (*mi).nr_blks -= 1;
    let n = ((*mi).nr_blks - idx) as usize;
    core::ptr::copy(
        (*mi).blk.as_ptr().add((idx + 1) as usize),
        (*mi).blk.as_mut_ptr().add(idx as usize),
        n,
    );
}

unsafe fn numa_move_tail_memblk(dst: *mut numa_meminfo, idx: i32, src: *mut numa_meminfo) {
    (*dst).blk[(*dst).nr_blks as usize] = (*src).blk[idx as usize];
    (*dst).nr_blks += 1;
    numa_remove_memblk_from(idx, src);
}

pub unsafe fn numa_add_memblk(nid: i32, start: u64, end: u64) -> i32 {
    let ret = numa_add_memblk_to(nid, start, end, &raw mut numa_meminfo);
    if ret == 0 { node_set(nid, &raw mut numa_nodes_parsed); }
    ret
}

pub unsafe fn numa_add_reserved_memblk(nid: i32, start: u64, end: u64) -> i32 {
    numa_add_memblk_to(nid, start, end, &raw mut numa_reserved_meminfo)
}

pub unsafe fn numa_cleanup_meminfo(mi: *mut numa_meminfo) -> i32 {
    let low = memblock_start_of_DRAM();
    let high = memblock_end_of_DRAM();
    let mut i: i32 = 0;
    while i < (*mi).nr_blks {
        let bi = &mut (*mi).blk[i as usize];
        if !memblock_overlaps_region(&raw const memblock.memory, bi.start, bi.end - bi.start) {
            numa_move_tail_memblk(&raw mut numa_reserved_meminfo, i, mi); i -= 1;
        } else {
            bi.start = core::cmp::max(bi.start, low);
            if bi.end > high { numa_add_reserved_memblk(bi.nid, high, bi.end); bi.end = high; }
            if bi.start >= bi.end { numa_remove_memblk_from(i, mi); i -= 1; }
        }
        i += 1;
    }
    i = 0;
    while i < (*mi).nr_blks {
        let mut j = i + 1;
        while j < (*mi).nr_blks {
            let (bi, bj) = (&mut (*mi).blk[i as usize], (*mi).blk[j as usize]);
            if bi.end > bj.start && bi.start < bj.end {
                if bi.nid != bj.nid { pr_err!("node %d overlaps with node %d\n", bi.nid, bj.nid); return -EINVAL; }
                pr_warn!("Warning: node %d overlaps with itself\n", bi.nid);
            }
            if bi.nid == bj.nid {
                let start = core::cmp::min(bi.start, bj.start);
                let end = core::cmp::max(bi.end, bj.end);
                let mut k = 0;
                while k < (*mi).nr_blks {
                    let bk = (*mi).blk[k as usize];
                    if bi.nid != bk.nid && start < bk.end && end > bk.start { break; }
                    k += 1;
                }
                if k == (*mi).nr_blks { bi.start = start; bi.end = end; numa_remove_memblk_from(j, mi); j -= 1; }
            }
            j += 1;
        }
        i += 1;
    }
    while i < ARRAY_SIZE((*mi).blk) as i32 {
        (*mi).blk[i as usize].start = 0;
        (*mi).blk[i as usize].end = 0;
        (*mi).blk[i as usize].nid = NUMA_NO_NODE;
        i += 1;
    }
    0
}

static mut numa_memblk_list: [*mut numa_memblk; NR_NODE_MEMBLKS as usize] = [core::ptr::null_mut(); NR_NODE_MEMBLKS as usize];

unsafe fn numa_clear_kernel_node_hotplug() {
    let mut reserved_nodemask = NODE_MASK_NONE;
    for i in 0..numa_meminfo.nr_blks as usize {
        let mb = &numa_meminfo.blk[i];
        let ret = memblock_set_node(mb.start, mb.end - mb.start, &raw mut memblock.reserved, mb.nid);
        WARN_ON_ONCE(ret);
    }
    for mb_region in for_each_reserved_mem_region() {
        let nid = memblock_get_region_node(mb_region);
        if numa_valid_node(nid) { node_set(nid, &raw mut reserved_nodemask); }
    }
    for i in 0..numa_meminfo.nr_blks as usize {
        let mb = &numa_meminfo.blk[i];
        if node_isset(mb.nid, reserved_nodemask) { memblock_clear_hotplug(mb.start, mb.end - mb.start); }
    }
}

unsafe fn numa_register_meminfo(mi: *mut numa_meminfo) -> i32 {
    node_possible_map = numa_nodes_parsed;
    if WARN_ON(nodes_empty(node_possible_map)) { return -EINVAL; }
    for i in 0..(*mi).nr_blks as usize {
        let mb = &(*mi).blk[i];
        memblock_set_node(mb.start, mb.end - mb.start, &raw mut memblock.memory, mb.nid);
    }
    numa_clear_kernel_node_hotplug();
    if IS_ENABLED!(NODE_NOT_IN_PAGE_FLAGS) {
        let pfn_align = node_map_pfn_alignment();
        if pfn_align != 0 && pfn_align < PAGES_PER_SECTION {
            let node_align_mb = PFN_PHYS(pfn_align) / SZ_1M;
            let sect_align_mb = PFN_PHYS(PAGES_PER_SECTION) / SZ_1M;
            pr_warn!("Node alignment %luMB < min %luMB, rejecting NUMA config\n", node_align_mb, sect_align_mb);
            return -EINVAL;
        }
    }
    0
}

pub unsafe fn numa_memblks_init(init_func: Option<unsafe extern "C" fn() -> i32>, memblock_force_top_down: bool) -> i32 {
    let max_addr: phys_addr_t = u64::MAX as phys_addr_t;
    nodes_clear(&raw mut numa_nodes_parsed);
    nodes_clear(&raw mut node_possible_map);
    nodes_clear(&raw mut node_online_map);
    core::ptr::write_bytes(&raw mut numa_meminfo as *mut numa_meminfo as *mut u8, 0, core::mem::size_of::<numa_meminfo>());
    WARN_ON(memblock_set_node(0, max_addr, &raw mut memblock.memory, NUMA_NO_NODE));
    WARN_ON(memblock_set_node(0, max_addr, &raw mut memblock.reserved, NUMA_NO_NODE));
    WARN_ON(memblock_clear_hotplug(0, max_addr));
    numa_reset_distance();
    let ret = init_func.unwrap()();
    if ret < 0 { return ret; }
    if memblock_force_top_down { memblock_set_bottom_up(false); }
    let ret = numa_cleanup_meminfo(&raw mut numa_meminfo);
    if ret < 0 { return ret; }
    numa_emulation(&raw mut numa_meminfo, numa_distance_cnt);
    numa_register_meminfo(&raw mut numa_meminfo)
}

unsafe fn cmp_memblk(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let ma = *(a as *const *mut numa_memblk);
    let mb = *(b as *const *mut numa_memblk);
    ((*ma).start > (*mb).start) as i32 - ((*ma).start < (*mb).start) as i32
}

pub unsafe fn numa_fill_memblks(start: u64, end: u64) -> i32 {
    let mi = &raw mut numa_meminfo;
    let mut count = 0usize;
    for i in 0..(*mi).nr_blks as usize {
        let bi = &mut (*mi).blk[i];
        if memblock_addrs_overlap(start, end - start, bi.start, bi.end - bi.start) {
            numa_memblk_list[count] = bi; count += 1;
        }
    }
    if count == 0 { return NUMA_NO_MEMBLK; }
    numa_memblk_list[0].as_mut().unwrap().start = core::cmp::min((*numa_memblk_list[0]).start, start);
    numa_memblk_list[count - 1].as_mut().unwrap().end = core::cmp::max((*numa_memblk_list[count - 1]).end, end);
    let mut prev_end = (*numa_memblk_list[0]).end;
    for i in 1..count {
        let curr = &mut *numa_memblk_list[i];
        if prev_end >= curr.start { if prev_end < curr.end { prev_end = curr.end; } }
        else { curr.start = prev_end; prev_end = curr.end; }
    }
    0
}

#[cfg(CONFIG_NUMA_KEEP_MEMINFO)]
unsafe fn meminfo_to_nid(mi: *mut numa_meminfo, start: u64) -> i32 {
    for i in 0..(*mi).nr_blks as usize {
        if (*mi).blk[i].start <= start && (*mi).blk[i].end > start { return (*mi).blk[i].nid; }
    }
    NUMA_NO_NODE
}

#[cfg(CONFIG_NUMA_KEEP_MEMINFO)]
pub unsafe fn phys_to_target_node(start: u64) -> i32 {
    let nid = meminfo_to_nid(&raw mut numa_meminfo, start);
    let reserved_nid = meminfo_to_nid(&raw mut numa_reserved_meminfo, start);
    if nid != NUMA_NO_NODE && reserved_nid == NUMA_NO_NODE { nid } else { reserved_nid }
}

#[cfg(CONFIG_NUMA_KEEP_MEMINFO)]
pub unsafe fn memory_add_physaddr_to_nid(start: u64) -> i32 {
    let mut nid = meminfo_to_nid(&raw mut numa_meminfo, start);
    if nid == NUMA_NO_NODE { nid = numa_meminfo.blk[0].nid; }
    nid
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
