/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2000, 05 by Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2000 by Silicon Graphics, Inc.
 * Copyright (C) 2004 by Christoph Hellwig
 *
 * On SGI IP27 the ARC memory configuration data is completely bogus but
 * alternate easier to use mechanisms are available.
 */

// Linux and SGI IP27 dependencies supplied by the surrounding translation.

const SLOT_PFNSHIFT: usize = SLOT_SHIFT - PAGE_SHIFT;
const PFN_NASIDSHFT: usize = NASID_SHFT - PAGE_SHIFT;

pub static mut __node_data: [*mut node_data; MAX_NUMNODES] = [core::ptr::null_mut(); MAX_NUMNODES];

unsafe fn gen_region_mask() -> u64 {
    let region_shift: i32 = get_region_shift();
    let mut region_mask: u64 = 0;
    for_each_online_node!(nasid, {
        region_mask |= 1u64 << ((nasid as u64) >> region_shift);
    });
    region_mask
}

static mut router_distance: i32 = 0;

unsafe fn router_recurse(router_a: *mut klrou_t, router_b: *mut klrou_t, depth: i32) {
    if (*router_a).rou_flags == 1 || depth >= router_distance {
        return;
    }

    (*router_a).rou_flags = 1;
    for port in 1..=MAX_ROUTER_PORTS {
        if (*router_a).rou_port[port].port_nasid == INVALID_NASID {
            continue;
        }
        let brd = NODE_OFFSET_TO_K0(
            (*router_a).rou_port[port].port_nasid,
            (*router_a).rou_port[port].port_offset,
        ) as *mut lboard_t;
        if (*brd).brd_type == KLTYPE_ROUTER {
            let router = NODE_OFFSET_TO_K0(NASID_GET(brd), (*brd).brd_compts[0]) as *mut klrou_t;
            if router == router_b {
                if depth < router_distance {
                    router_distance = depth;
                }
            } else {
                router_recurse(router, router_b, depth + 1);
            }
        }
    }
    (*router_a).rou_flags = 0;
}

pub static mut __node_distances: [[u8; MAX_NUMNODES]; MAX_NUMNODES] =
    [[0; MAX_NUMNODES]; MAX_NUMNODES];

unsafe fn compute_node_distance(nasid_a: nasid_t, nasid_b: nasid_t) -> i32 {
    let mut router_a: *mut klrou_t = core::ptr::null_mut();
    let mut router_b: *mut klrou_t = core::ptr::null_mut();

    for_each_online_node!(nasid, {
        let mut brd = find_lboard_class(KL_CONFIG_INFO(nasid) as *mut lboard_t, KLTYPE_ROUTER);
        if brd.is_null() { continue; }
        loop {
            if (*brd).brd_flags & DUPLICATE_BOARD != 0 {
                brd = find_lboard_class(KLCF_NEXT(brd), KLTYPE_ROUTER);
                if brd.is_null() { break; }
                continue;
            }
            let router = NODE_OFFSET_TO_K0(NASID_GET(brd), (*brd).brd_compts[0]) as *mut klrou_t;
            (*router).rou_flags = 0;
            for port in 1..=MAX_ROUTER_PORTS {
                if (*router).rou_port[port].port_nasid == INVALID_NASID { continue; }
                let dest_brd = NODE_OFFSET_TO_K0((*router).rou_port[port].port_nasid,
                    (*router).rou_port[port].port_offset) as *mut lboard_t;
                if (*dest_brd).brd_type == KLTYPE_IP27 {
                    if (*dest_brd).brd_nasid == nasid_a { router_a = router; }
                    if (*dest_brd).brd_nasid == nasid_b { router_b = router; }
                }
            }
            brd = find_lboard_class(KLCF_NEXT(brd), KLTYPE_ROUTER);
            if brd.is_null() { break; }
        }
    });

    if nasid_a == nasid_b { return LOCAL_DISTANCE; }
    if router_a == router_b { return LOCAL_DISTANCE + 1; }
    if router_a.is_null() { pr_info!("node_distance: router_a NULL\n"); return 255; }
    if router_b.is_null() { pr_info!("node_distance: router_b NULL\n"); return 255; }
    router_distance = 100;
    router_recurse(router_a, router_b, 2);
    LOCAL_DISTANCE + router_distance
}

unsafe fn init_topology_matrix() {
    for row in 0..MAX_NUMNODES { for col in 0..MAX_NUMNODES { __node_distances[row][col] = u8::MAX; } }
    for_each_online_node!(row, { for_each_online_node!(col, {
        __node_distances[row][col] = compute_node_distance(row, col) as u8;
    }); });
}

unsafe fn dump_topology() {
    let mut router_num = 0;
    pr_info!("************** Topology ********************\n");
    pr_info!("    ");
    for_each_online_node!(col, { pr_cont!("{:02} ", col); });
    pr_cont!("\n");
    for_each_online_node!(row, {
        pr_info!("{:02}  ", row);
        for_each_online_node!(col, { pr_cont!("{:2} ", node_distance(row, col)); });
        pr_cont!("\n");
    });
    for_each_online_node!(nasid, {
        let mut brd = find_lboard_class(KL_CONFIG_INFO(nasid) as *mut lboard_t, KLTYPE_ROUTER);
        if brd.is_null() { continue; }
        loop {
            if (*brd).brd_flags & DUPLICATE_BOARD != 0 {
                brd = find_lboard_class(KLCF_NEXT(brd), KLTYPE_ROUTER);
                if brd.is_null() { break; }
                continue;
            }
            pr_cont!("Router {}:", router_num); router_num += 1;
            let router = NODE_OFFSET_TO_K0(NASID_GET(brd), (*brd).brd_compts[0]) as *mut klrou_t;
            for port in 1..=MAX_ROUTER_PORTS {
                if (*router).rou_port[port].port_nasid == INVALID_NASID { continue; }
                let dest_brd = NODE_OFFSET_TO_K0((*router).rou_port[port].port_nasid,
                    (*router).rou_port[port].port_offset) as *mut lboard_t;
                if (*dest_brd).brd_type == KLTYPE_IP27 { pr_cont!(" {}", (*dest_brd).brd_nasid); }
                if (*dest_brd).brd_type == KLTYPE_ROUTER { pr_cont!(" r"); }
            }
            pr_cont!("\n");
            brd = find_lboard_class(KLCF_NEXT(brd), KLTYPE_ROUTER);
            if brd.is_null() { break; }
        }
    });
}

unsafe fn slot_getbasepfn(nasid: nasid_t, slot: i32) -> ulong {
    ((nasid as ulong) << PFN_NASIDSHFT) | ((slot as ulong) << SLOT_PFNSHIFT)
}

unsafe fn slot_psize_compute(nasid: nasid_t, slot: i32) -> ulong {
    let brd = find_lboard(KL_CONFIG_INFO(nasid) as *mut lboard_t, KLTYPE_IP27);
    if brd.is_null() { return 0; }
    let banks = find_first_component(brd, KLSTRUCT_MEMBNK) as *mut klmembnk_t;
    if banks.is_null() { return 0; }
    let mut size = (*banks).membnk_bnksz[(slot / 4) as usize] as ulong;
    if size <= 128 {
        if slot % 4 == 0 { size <<= 20; return size >> PAGE_SHIFT; }
        return 0;
    }
    size /= 4; size <<= 20; size >> PAGE_SHIFT
}

unsafe fn mlreset() {
    let region_mask: u64;
    master_nasid = get_nasid();
    // CONFIG_SMP conditional from the C source.
    #[cfg(CONFIG_SMP)]
    cpu_node_probe();
    init_topology_matrix();
    dump_topology();
    region_mask = gen_region_mask();
    setup_replication_mask();
    for_each_online_node!(nasid, {
        REMOTE_HUB_S(nasid, PI_REGION_PRESENT, region_mask | 1);
        REMOTE_HUB_S(nasid, PI_CALIAS_SIZE, PI_CALIAS_SIZE_0);
        // The original LATER conditional block is intentionally omitted.
    });
}

unsafe fn szmem() {
    for_each_online_node!(node, {
        let mut nodebytes: ulong = 0;
        let mut slot0sz: ulong = 0;
        for slot in 0..MAX_MEM_SLOTS {
            let slot_psize = slot_psize_compute(node, slot);
            if slot == 0 { slot0sz = slot_psize; }
            nodebytes += 1u64 << SLOT_SHIFT;
            if slot_psize == 0 { continue; }
            if (nodebytes >> PAGE_SHIFT) * core::mem::size_of::<page>() as ulong > (slot0sz << PAGE_SHIFT) {
                pr_info!("Ignoring slot {} onwards on node {}\n", slot, node);
                break;
            }
            memblock_add_node(PFN_PHYS(slot_getbasepfn(node, slot)), PFN_PHYS(slot_psize), node, MEMBLOCK_NONE);
        }
    });
}

unsafe fn node_mem_init(node: nasid_t) {
    let slot_firstpfn = slot_getbasepfn(node, 0);
    let mut slot_freepfn = node_getfirstfree(node);
    let (mut start_pfn, mut end_pfn) = (0, 0);
    get_pfn_range_for_nid(node, &mut start_pfn, &mut end_pfn);
    __node_data[node] = __va(slot_freepfn << PAGE_SHIFT);
    memset(__node_data[node], 0, PAGE_SIZE);
    node_data[node] = &mut (*__node_data[node]).pglist;
    (*NODE_DATA(node)).node_start_pfn = start_pfn;
    (*NODE_DATA(node)).node_spanned_pages = end_pfn - start_pfn;
    cpumask_clear(&mut (*hub_data(node)).h_cpus);
    slot_freepfn += PFN_UP(core::mem::size_of::<pglist_data>() + core::mem::size_of::<hub_data>());
    memblock_reserve(slot_firstpfn << PAGE_SHIFT, (slot_freepfn - slot_firstpfn) << PAGE_SHIFT);
}

static mut null_node: node_data = node_data { hub: hub_data { h_cpus: CPU_MASK_NONE } };

pub unsafe fn prom_meminit() {
    mlreset(); szmem(); max_low_pfn = PHYS_PFN(memblock_end_of_DRAM());
    for node in 0..MAX_NUMNODES {
        if node_online(node) { node_mem_init(node); } else { __node_data[node] = &mut null_node; }
    }
}

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut ulong) {
    *max_zone_pfns.add(ZONE_NORMAL) = max_low_pfn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
