// SPDX-License-Identifier: GPL-2.0-only
/*
 * NUMA support, based on the x86 implementation.
 *
 * Copyright (C) 2015 Cavium Inc.
 * Author: Ganapatrao Kulkarni <gkulkarni@cavium.com>
 */

// C dependencies and build-time configuration are supplied by the surrounding kernel.

static mut CPU_TO_NODE_MAP: [i32; NR_CPUS] = [NUMA_NO_NODE; NR_CPUS];
pub static mut numa_off: bool = false;

unsafe fn numa_parse_early_param(opt: *mut core::ffi::c_char) -> i32 {
    if opt.is_null() { return -EINVAL; }
    if str_has_prefix(opt, b"off\0".as_ptr() as *const _) { numa_off = true; }
    if !strncmp(opt, b"fake=\0".as_ptr() as *const _, 5) {
        return numa_emu_cmdline(opt.add(5));
    }
    0
}

pub static mut node_to_cpumask_map: [cpumask_var_t; MAX_NUMNODES] = [core::ptr::null_mut(); MAX_NUMNODES];

#[cfg(CONFIG_DEBUG_PER_CPU_MAPS)]
pub unsafe fn cpumask_of_node(node: i32) -> *const struct_cpumask {
    if node == NUMA_NO_NODE { return cpu_all_mask; }
    if WARN_ON(node < 0 || node >= nr_node_ids) { return cpu_none_mask; }
    if WARN_ON(node_to_cpumask_map[node as usize].is_null()) { return cpu_online_mask; }
    node_to_cpumask_map[node as usize]
}

#[cfg(not(CONFIG_NUMA_EMU))]
unsafe fn numa_update_cpu(cpu: u32, remove: bool) {
    let nid = cpu_to_node(cpu);
    if nid == NUMA_NO_NODE { return; }
    if remove { cpumask_clear_cpu(cpu, node_to_cpumask_map[nid as usize]); }
    else { cpumask_set_cpu(cpu, node_to_cpumask_map[nid as usize]); }
}

#[cfg(not(CONFIG_NUMA_EMU))]
pub unsafe fn numa_add_cpu(cpu: u32) { numa_update_cpu(cpu, false); }
#[cfg(not(CONFIG_NUMA_EMU))]
pub unsafe fn numa_remove_cpu(cpu: u32) { numa_update_cpu(cpu, true); }

pub unsafe fn numa_clear_node(cpu: u32) {
    numa_remove_cpu(cpu);
    set_cpu_numa_node(cpu, NUMA_NO_NODE);
}

unsafe fn setup_node_to_cpumask_map() {
    if nr_node_ids == MAX_NUMNODES { setup_nr_node_ids(); }
    if unlikely(nr_node_ids > MAX_NUMNODES) {
        pr_err!("nr_node_ids ({}) is larger than MAX_NUMNODES ({})\n", nr_node_ids, MAX_NUMNODES);
        return;
    }
    for node in 0..nr_node_ids {
        alloc_bootmem_cpumask_var(&mut node_to_cpumask_map[node as usize]);
        cpumask_clear(node_to_cpumask_map[node as usize]);
    }
    pr_debug!("Node to cpumask map for {} nodes\n", nr_node_ids);
}

pub unsafe fn numa_store_cpu_info(cpu: u32) { set_cpu_numa_node(cpu, CPU_TO_NODE_MAP[cpu as usize]); }

pub unsafe fn early_map_cpu_to_node(cpu: u32, mut nid: i32) {
    if nid < 0 || nid >= MAX_NUMNODES || numa_off { nid = 0; }
    CPU_TO_NODE_MAP[cpu as usize] = nid;
    if cpu == 0 { set_cpu_numa_node(cpu, nid); }
}

#[cfg(CONFIG_HAVE_SETUP_PER_CPU_AREA)]
pub static mut __per_cpu_offset: [usize; NR_CPUS] = [0; NR_CPUS];
#[cfg(CONFIG_HAVE_SETUP_PER_CPU_AREA)]
pub unsafe fn early_cpu_to_node(cpu: i32) -> i32 { CPU_TO_NODE_MAP[cpu as usize] }
#[cfg(CONFIG_HAVE_SETUP_PER_CPU_AREA)]
unsafe fn pcpu_cpu_distance(from: u32, to: u32) -> i32 { node_distance(early_cpu_to_node(from as i32), early_cpu_to_node(to as i32)) }

#[cfg(CONFIG_HAVE_SETUP_PER_CPU_AREA)]
pub unsafe fn setup_per_cpu_areas() {
    let mut rc = -EINVAL;
    if pcpu_chosen_fc != PCPU_FC_PAGE {
        rc = pcpu_embed_first_chunk(PERCPU_MODULE_RESERVE, PERCPU_DYNAMIC_RESERVE, PAGE_SIZE, pcpu_cpu_distance, early_cpu_to_node);
        #[cfg(CONFIG_NEED_PER_CPU_PAGE_FIRST_CHUNK)]
        if rc < 0 { pr_warn!("PERCPU: allocator failed ({}), falling back to page size\n", rc); }
    }
    #[cfg(CONFIG_NEED_PER_CPU_PAGE_FIRST_CHUNK)]
    if rc < 0 { rc = pcpu_page_first_chunk(PERCPU_MODULE_RESERVE, early_cpu_to_node); }
    if rc < 0 { panic!("Failed to initialize percpu areas (err={}).", rc); }
    let delta = pcpu_base_addr as usize - __per_cpu_start as usize;
    for_each_possible_cpu!(cpu, { __per_cpu_offset[cpu as usize] = delta + pcpu_unit_offsets[cpu as usize]; });
}

unsafe fn setup_node_data(nid: i32, start_pfn: u64, end_pfn: u64) {
    if start_pfn >= end_pfn { pr_info!("Initmem setup node {} [<memory-less node>]\n", nid); }
    alloc_node_data(nid);
    (*NODE_DATA(nid)).node_id = nid;
    (*NODE_DATA(nid)).node_start_pfn = start_pfn;
    (*NODE_DATA(nid)).node_spanned_pages = end_pfn - start_pfn;
}

unsafe fn numa_register_nodes() -> i32 {
    if !memblock_validate_numa_coverage(0) { return -EINVAL; }
    for_each_node_mask!(nid, numa_nodes_parsed, {
        let (mut start_pfn, mut end_pfn) = (0usize, 0usize);
        get_pfn_range_for_nid(nid, &mut start_pfn, &mut end_pfn);
        setup_node_data(nid, start_pfn as u64, end_pfn as u64);
        node_set_online(nid);
    });
    0
}

unsafe fn numa_init(init_func: unsafe fn() -> i32) -> i32 {
    let mut ret = numa_memblks_init(init_func, false);
    if ret < 0 { numa_reset_distance(); return ret; }
    if nodes_empty(numa_nodes_parsed) { pr_info!("No NUMA configuration found\n"); numa_reset_distance(); return -EINVAL; }
    ret = numa_register_nodes();
    if ret < 0 { numa_reset_distance(); return ret; }
    setup_node_to_cpumask_map();
    0
}

unsafe fn dummy_numa_init() -> i32 {
    let start = memblock_start_of_DRAM();
    let end = memblock_end_of_DRAM() - 1;
    if numa_off { pr_info!("NUMA disabled\n"); }
    pr_info!("Faking a node at [mem %pap-%pap]\n", &start, &end);
    let ret = numa_add_memblk(0, start, end + 1);
    if ret != 0 { pr_err!("NUMA init failed\n"); return ret; }
    numa_off = true; 0
}

#[cfg(CONFIG_ACPI_NUMA)]
unsafe fn arch_acpi_numa_init() -> i32 { let ret = acpi_numa_init(); if ret != 0 { pr_debug!("Failed to initialise from firmware\n"); return ret; } if srat_disabled() { -EINVAL } else { 0 } }
#[cfg(not(CONFIG_ACPI_NUMA))]
unsafe fn arch_acpi_numa_init() -> i32 { -EOPNOTSUPP }

pub unsafe fn arch_numa_init() {
    if !numa_off {
        if !acpi_disabled && numa_init(arch_acpi_numa_init) == 0 { return; }
        if acpi_disabled && numa_init(of_numa_init) == 0 { return; }
    }
    numa_init(dummy_numa_init);
}

#[cfg(CONFIG_NUMA_EMU)]
pub unsafe fn numa_emu_update_cpu_to_node(emu_nid_to_phys: *mut i32, nr_emu_nids: u32) {
    for i in 0..NR_CPUS { if CPU_TO_NODE_MAP[i] == NUMA_NO_NODE { continue; } let mut j = 0; while j < nr_emu_nids && CPU_TO_NODE_MAP[i] != *emu_nid_to_phys.add(j as usize) { j += 1; } CPU_TO_NODE_MAP[i] = if j < nr_emu_nids { j as i32 } else { 0 }; }
}
#[cfg(CONFIG_NUMA_EMU)]
pub unsafe fn numa_emu_dma_end() -> u64 { memblock_start_of_DRAM() + SZ_4G }
#[cfg(CONFIG_NUMA_EMU)]
pub unsafe fn debug_cpumask_set_cpu(cpu: u32, node: i32, enable: bool) {
    if node == NUMA_NO_NODE { return; }
    let mask = node_to_cpumask_map[node as usize];
    if !cpumask_available(mask) { pr_err!("node_to_cpumask_map[{}] NULL\n", node); dump_stack(); return; }
    if enable { cpumask_set_cpu(cpu, mask); } else { cpumask_clear_cpu(cpu, mask); }
    pr_debug!("{} cpu {} node {}: mask now %*pbl\n", if enable { "numa_add_cpu" } else { "numa_remove_cpu" }, cpu, node, cpumask_pr_args(mask));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
