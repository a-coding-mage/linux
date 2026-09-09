// SPDX-License-Identifier: GPL-2.0-only
/* Common code for 32 and 64-bit NUMA */

// Kernel and architecture dependencies supplied by other translation units.

pub static mut numa_off: i32 = 0;

#[init]
unsafe fn numa_setup(opt: *mut u8) -> i32 {
    if opt.is_null() { return -EINVAL; }
    if !strncmp(opt, b"off\0".as_ptr(), 3) { numa_off = 1; }
    if !strncmp(opt, b"fake=\0".as_ptr(), 5) { return numa_emu_cmdline(opt.add(5)); }
    if !strncmp(opt, b"noacpi\0".as_ptr(), 6) { disable_srat(); }
    if !strncmp(opt, b"nohmat\0".as_ptr(), 6) { disable_hmat(); }
    0
}
// early_param("numa", numa_setup);

/* apicid, cpu, node mappings */
#[no_mangle]
pub static mut __apicid_to_node: [i16; MAX_LOCAL_APIC as usize] = [NUMA_NO_NODE as i16; MAX_LOCAL_APIC as usize];

pub static mut numa_phys_nodes_parsed: nodemask_t = nodemask_t { bits: [0; MAX_NUMNODES as usize / (usize::BITS as usize)] };

pub unsafe fn numa_cpu_node(cpu: i32) -> i32 {
    let apicid: u32 = early_per_cpu(x86_cpu_to_apicid, cpu);
    if apicid != BAD_APICID { __apicid_to_node[apicid as usize] as i32 } else { NUMA_NO_NODE }
}

pub unsafe fn num_phys_nodes() -> i32 { bitmap_weight(numa_phys_nodes_parsed.bits.as_ptr(), MAX_NUMNODES) }

pub static mut node_to_cpumask_map: [cpumask_var_t; MAX_NUMNODES as usize] = [core::ptr::null_mut(); MAX_NUMNODES as usize];

/* Map cpu index to node index */
// DEFINE_EARLY_PER_CPU(int, x86_cpu_to_node_map, NUMA_NO_NODE);

pub unsafe fn numa_set_node(cpu: i32, node: i32) {
    let cpu_to_node_map = early_per_cpu_ptr(x86_cpu_to_node_map);
    if !cpu_to_node_map.is_null() { *cpu_to_node_map.add(cpu as usize) = node; return; }
    // CONFIG_DEBUG_PER_CPU_MAPS validation is preserved by the conditional build below.
    #[cfg(CONFIG_DEBUG_PER_CPU_MAPS)]
    if cpu >= nr_cpu_ids || !cpu_possible(cpu) { printk(KERN_ERR, b"numa_set_node: invalid cpu# (%d)\n\0".as_ptr(), cpu); dump_stack(); return; }
    per_cpu!(x86_cpu_to_node_map, cpu) = node;
    set_cpu_numa_node(cpu, node);
}

pub unsafe fn numa_clear_node(cpu: i32) { numa_set_node(cpu, NUMA_NO_NODE); }

pub unsafe fn setup_node_to_cpumask_map() {
    let mut node: u32;
    if nr_node_ids == MAX_NUMNODES { setup_nr_node_ids(); }
    node = 0;
    while node < nr_node_ids {
        alloc_bootmem_cpumask_var(&mut node_to_cpumask_map[node as usize]);
        node += 1;
    }
    pr_debug(b"Node to cpumask map for %u nodes\n\0".as_ptr(), nr_node_ids);
}

unsafe fn numa_register_nodes() -> i32 {
    if !memblock_validate_numa_coverage(SZ_1M) { return -EINVAL; }
    let mut nid: i32;
    for_each_node_mask!(nid, node_possible_map, {
        let mut start_pfn = 0UL; let mut end_pfn = 0UL;
        get_pfn_range_for_nid(nid, &mut start_pfn, &mut end_pfn);
        if start_pfn < end_pfn { alloc_node_data(nid); node_set_online(nid); }
    });
    memblock_dump_all(); 0
}

unsafe fn numa_init_array() {
    let mut rr = first_node(node_online_map);
    for i in 0..nr_cpu_ids { if early_cpu_to_node(i) == NUMA_NO_NODE { numa_set_node(i, rr); rr = next_node_in(rr, node_online_map); } }
}

unsafe fn numa_init(init_func: Option<unsafe fn() -> i32>) -> i32 {
    for i in 0..MAX_LOCAL_APIC { set_apicid_to_node(i, NUMA_NO_NODE); }
    let mut ret = numa_memblks_init(init_func, true); if ret < 0 { return ret; }
    ret = numa_register_nodes(); if ret < 0 { return ret; }
    for i in 0..nr_cpu_ids { let nid = early_cpu_to_node(i); if nid != NUMA_NO_NODE && !node_online(nid) { numa_clear_node(i); } }
    numa_init_array(); 0
}

unsafe fn dummy_numa_init() -> i32 {
    printk(KERN_INFO, b"%s\n\0".as_ptr(), if numa_off != 0 { b"NUMA turned off\0".as_ptr() } else { b"No NUMA configuration found\0".as_ptr() });
    printk(KERN_INFO, b"Faking a node at [mem %#018Lx-%#018Lx]\n\0".as_ptr(), 0u64, PFN_PHYS(max_pfn) - 1);
    node_set(0, &mut numa_phys_nodes_parsed); numa_add_memblk(0, 0, PFN_PHYS(max_pfn)); 0
}

pub unsafe fn x86_numa_init() {
    if numa_off == 0 {
        #[cfg(CONFIG_ACPI_NUMA)] if numa_init(Some(x86_acpi_numa_init)) == 0 { return; }
        #[cfg(CONFIG_AMD_NUMA)] if numa_init(Some(amd_numa_init)) == 0 { return; }
        if acpi_disabled && numa_init(Some(of_numa_init)) == 0 { return; }
    }
    numa_init(Some(dummy_numa_init));
}

pub unsafe fn init_gi_nodes() {
    let mut nid: i32;
    for_each_node_state!(nid, N_GENERIC_INITIATOR, { if !node_online(nid) { node_set_online(nid); } });
}

pub unsafe fn init_cpu_to_node() {
    let cpu_to_apicid = early_per_cpu_ptr(x86_cpu_to_apicid); BUG_ON(cpu_to_apicid.is_null());
    for_each_possible_cpu!(cpu, { let node = numa_cpu_node(cpu); if node != NUMA_NO_NODE { if !node_online(node) { node_set_online(node); } numa_set_node(cpu, node); } });
}

// CONFIG_DEBUG_PER_CPU_MAPS-dependent CPU mask helpers and NUMA emulation helpers
// are represented below with the same externally visible operations.
#[cfg(not(CONFIG_DEBUG_PER_CPU_MAPS))]
pub unsafe fn numa_add_cpu(cpu: u32) { cpumask_set_cpu(cpu, node_to_cpumask_map[early_cpu_to_node(cpu as i32) as usize]); }
#[cfg(not(CONFIG_DEBUG_PER_CPU_MAPS))]
pub unsafe fn numa_remove_cpu(cpu: u32) { cpumask_clear_cpu(cpu, node_to_cpumask_map[early_cpu_to_node(cpu as i32) as usize]); }

#[cfg(CONFIG_NUMA_EMU)]
pub unsafe fn numa_emu_update_cpu_to_node(emu_nid_to_phys: *mut i32, nr_emu_nids: u32) {
    for i in 0..MAX_LOCAL_APIC as usize { if __apicid_to_node[i] != NUMA_NO_NODE as i16 { let mut j = 0; while j < nr_emu_nids && __apicid_to_node[i] as i32 != *emu_nid_to_phys.add(j as usize) { j += 1; } __apicid_to_node[i] = if j < nr_emu_nids { j as i16 } else { 0 }; } }
}
#[cfg(CONFIG_NUMA_EMU)]
pub unsafe fn numa_emu_dma_end() -> u64 { PFN_PHYS(MAX_DMA32_PFN) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
