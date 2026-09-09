// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2010 Loongson Inc. & Lemote Inc. &
 *                    Institute of Computing Technology
 * Author:  Xiang Gao, gaoxiang@ict.ac.cn
 *          Huacai Chen, chenhc@lemote.com
 *          Xiaofu Meng, Shuangshuang Zhang
 */
// Linux and architecture headers from the C source provide the referenced
// constants, types, globals, macros, and functions.

pub static mut __node_distances: [[u8; MAX_NUMNODES]; MAX_NUMNODES] =
    [[0; MAX_NUMNODES]; MAX_NUMNODES];
pub static mut __node_cpumask: [cpumask_t; MAX_NUMNODES] =
    [cpumask_t::default(); MAX_NUMNODES];

unsafe fn cpu_node_probe() {
    let mut i: i32;

    nodes_clear(node_possible_map);
    nodes_clear(node_online_map);
    i = 0;
    while i < loongson_sysconf.nr_nodes {
        node_set_state(num_online_nodes(), N_POSSIBLE);
        node_set_online(num_online_nodes());
        i += 1;
    }

    pr_info!("NUMA: Discovered {} cpus on {} nodes\n",
        loongson_sysconf.nr_cpus, num_online_nodes());
}

unsafe fn compute_node_distance(row: i32, col: i32) -> i32 {
    let package_row = row * loongson_sysconf.cores_per_node /
        loongson_sysconf.cores_per_package;
    let package_col = col * loongson_sysconf.cores_per_node /
        loongson_sysconf.cores_per_package;

    if col == row {
        LOCAL_DISTANCE
    } else if package_row == package_col {
        40
    } else {
        100
    }
}

unsafe fn init_topology_matrix() {
    let mut row: i32;
    let mut col: i32;

    row = 0;
    while row < MAX_NUMNODES {
        col = 0;
        while col < MAX_NUMNODES {
            __node_distances[row as usize][col as usize] = u8::MAX;
            col += 1;
        }
        row += 1;
    }

    for_each_online_node!(row, {
        for_each_online_node!(col, {
            __node_distances[row as usize][col as usize] =
                compute_node_distance(row, col) as u8;
        });
    });
}

unsafe fn node_mem_init(node: u32) {
    let node_addrspace_offset: usize;
    let start_pfn: usize;
    let end_pfn: usize;

    node_addrspace_offset = nid_to_addrbase(node);
    pr_info!("Node{}'s addrspace_offset is 0x{:x}\n",
        node, node_addrspace_offset);

    get_pfn_range_for_nid(node, &mut start_pfn, &mut end_pfn);
    pr_info!("Node{}: start_pfn=0x{:x}, end_pfn=0x{:x}\n",
        node, start_pfn, end_pfn);

    alloc_node_data(node);

    NODE_DATA(node).node_start_pfn = start_pfn;
    NODE_DATA(node).node_spanned_pages = end_pfn - start_pfn;

    if node == 0 {
        /* kernel start address */
        let kernel_start_pfn = PFN_DOWN(__pa_symbol(&_text));

        /* kernel end address */
        let kernel_end_pfn = PFN_UP(__pa_symbol(&_end));

        /* used by finalize_initrd() */
        max_low_pfn = end_pfn;

        /* Reserve the kernel text/data/bss */
        memblock_reserve(kernel_start_pfn << PAGE_SHIFT,
            (kernel_end_pfn - kernel_start_pfn) << PAGE_SHIFT);

        /* Reserve 0xfe000000~0xffffffff for RS780E integrated GPU */
        if node_end_pfn(0) >= (0xffffffffusize >> PAGE_SHIFT) {
            memblock_reserve(node_addrspace_offset | 0xfe000000usize,
                32usize << 20);
        }

        /* Reserve pfn range 0~node[0]->node_start_pfn */
        memblock_reserve(0, PAGE_SIZE * start_pfn);
        /* set nid for reserved memory on node 0 */
        memblock_set_node(0, 1u64 << 44, &mut memblock.reserved, 0);
    }
}

unsafe fn prom_meminit() {
    let mut node: u32;
    let mut cpu: u32;
    let mut active_cpu: u32 = 0;

    cpu_node_probe();
    init_topology_matrix();

    node = 0;
    while node < loongson_sysconf.nr_nodes as u32 {
        if node_online(node) {
            szmem(node);
            node_mem_init(node);
            cpumask_clear(&mut __node_cpumask[node as usize]);
        }
        node += 1;
    }
    max_low_pfn = PHYS_PFN(memblock_end_of_DRAM());

    cpu = 0;
    while cpu < loongson_sysconf.nr_cpus as u32 {
        node = cpu / loongson_sysconf.cores_per_node as u32;
        if node >= num_online_nodes() {
            node = 0;
        }

        if loongson_sysconf.reserved_cpus_mask & (1u64 << cpu) != 0 {
            cpu += 1;
            continue;
        }

        cpumask_set_cpu(active_cpu, &mut __node_cpumask[node as usize]);
        pr_info!("NUMA: set cpumask cpu {} on node {}\n", active_cpu, node);

        active_cpu += 1;
        cpu += 1;
    }
}

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut usize) {
    *max_zone_pfns.add(ZONE_DMA32 as usize) = MAX_DMA32_PFN;
    *max_zone_pfns.add(ZONE_NORMAL as usize) = max_low_pfn;
}

/* All PCI device belongs to logical Node-0 */
pub unsafe fn pcibus_to_node(_bus: *mut pci_bus) -> i32 {
    0
}

pub unsafe fn prom_init_numa_memory() {
    pr_info!("CP0_Config3: CP0 16.3 (0x{:x})\n", read_c0_config3());
    pr_info!("CP0_PageGrain: CP0 5.1 (0x{:x})\n", read_c0_pagegrain());
    prom_meminit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
