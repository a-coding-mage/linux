// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * acpi_numa.c - ACPI NUMA support
 * Rust source-level translation of the implementation source.
 */

// Includes and build-time configuration are supplied by the surrounding kernel translation.

static mut nodes_found_map: nodemask_t = NODE_MASK_NONE;
static mut pxm_to_node_map: [i32; MAX_PXM_DOMAINS] = [NUMA_NO_NODE; MAX_PXM_DOMAINS];
static mut node_to_pxm_map: [i32; MAX_NUMNODES] = [PXM_INVAL; MAX_NUMNODES];

pub static mut acpi_srat_revision: u8 = 0;
static mut acpi_numa: i32 = 0;
static mut last_real_pxm: i32 = 0;

pub unsafe fn disable_srat() { acpi_numa = -1; }

pub unsafe fn pxm_to_node(pxm: i32) -> i32 {
    if pxm < 0 || pxm >= MAX_PXM_DOMAINS || numa_off { NUMA_NO_NODE } else { pxm_to_node_map[pxm as usize] }
}

pub unsafe fn node_to_pxm(node: i32) -> i32 {
    if node < 0 { PXM_INVAL } else { node_to_pxm_map[node as usize] }
}

unsafe fn __acpi_map_pxm_to_node(pxm: i32, node: i32) {
    if pxm_to_node_map[pxm as usize] == NUMA_NO_NODE || node < pxm_to_node_map[pxm as usize] { pxm_to_node_map[pxm as usize] = node; }
    if node_to_pxm_map[node as usize] == PXM_INVAL || pxm < node_to_pxm_map[node as usize] { node_to_pxm_map[node as usize] = pxm; }
}

pub unsafe fn acpi_map_pxm_to_node(pxm: i32) -> i32 {
    if pxm < 0 || pxm >= MAX_PXM_DOMAINS || numa_off { return NUMA_NO_NODE; }
    let mut node = pxm_to_node_map[pxm as usize];
    if node == NUMA_NO_NODE {
        node = first_unset_node(nodes_found_map);
        if node >= MAX_NUMNODES { return NUMA_NO_NODE; }
        __acpi_map_pxm_to_node(pxm, node);
        node_set(node, nodes_found_map);
    }
    node
}

#[cfg(CONFIG_NUMA_EMU)]
pub unsafe fn fix_pxm_node_maps(max_nid: i32) -> i32 {
    let mut pxm_to_node_map_copy = [NUMA_NO_NODE; MAX_PXM_DOMAINS];
    let mut node_to_pxm_map_copy = [PXM_INVAL; MAX_NUMNODES];
    let mut index = -1;
    let mut count = 0;
    let mut nodes_to_enable: nodemask_t = core::mem::zeroed();
    if numa_off { return -1; }
    if srat_disabled() != 0 { return 0; }
    for i in 0..MAX_NUMNODES {
        if node_to_pxm_map[i] != PXM_INVAL {
            for j in 0..=max_nid {
                if emu_nid_to_phys[j as usize] == i as i32 && WARN(node_to_pxm_map_copy[j as usize] != PXM_INVAL, "Node %d is already binded to PXM %d\n", j, node_to_pxm_map_copy[j as usize]) { return -1; }
                if emu_nid_to_phys[j as usize] == i as i32 { node_to_pxm_map_copy[j as usize] = node_to_pxm_map[i]; if j > index { index = j; } count += 1; }
            }
        }
    }
    if index == -1 { pr_debug!("No node/PXM mapping has been set\n"); return 0; }
    if WARN(index != max_nid, "%d max nid  when expected %d\n", index, max_nid) { return -1; }
    nodes_clear(nodes_to_enable);
    for i in 0..MAX_NUMNODES {
        if node_to_pxm_map[i] != PXM_INVAL {
            let mut j = 0;
            while j <= max_nid { if emu_nid_to_phys[j as usize] == i as i32 { break; } j += 1; }
            if j <= max_nid { continue; }
            j = 0; while j < MAX_NUMNODES as i32 && node_to_pxm_map_copy[j as usize] != PXM_INVAL { j += 1; }
            if WARN(j == MAX_NUMNODES as i32, "Number of nodes exceeds MAX_NUMNODES\n") { return -1; }
            node_to_pxm_map_copy[j as usize] = node_to_pxm_map[i]; node_set(j, nodes_to_enable); count += 1;
        }
    }
    for i in 0..MAX_NUMNODES { if node_to_pxm_map_copy[i] != PXM_INVAL && pxm_to_node_map_copy[node_to_pxm_map_copy[i] as usize] == NUMA_NO_NODE { pxm_to_node_map_copy[node_to_pxm_map_copy[i] as usize] = i as i32; } }
    for i in 0..MAX_NUMNODES { node_to_pxm_map[i] = node_to_pxm_map_copy[i]; pxm_to_node_map[i] = pxm_to_node_map_copy[i]; }
    nodes_or(numa_nodes_parsed, nodes_to_enable, numa_nodes_parsed);
    pr_debug!("found %d total number of nodes\n", count); 0
}

unsafe fn slit_valid(slit: *mut acpi_table_slit) -> i32 {
    let d = (*slit).locality_count as usize;
    for i in 0..d { for j in 0..d { let val = (*slit).entry[d*i+j]; if i == j { if val != LOCAL_DISTANCE { return 0; } } else if val <= LOCAL_DISTANCE { return 0; } } }
    1
}

pub unsafe fn bad_srat() { pr_err!("SRAT: SRAT not used.\n"); disable_srat(); }
pub unsafe fn srat_disabled() -> i32 { (acpi_numa < 0) as i32 }
pub unsafe fn numa_fill_memblks(_start: u64, _end: u64) -> i32 { NUMA_NO_MEMBLK }

unsafe fn acpi_parse_slit(table: *mut acpi_table_header) -> i32 {
    let slit = table as *mut acpi_table_slit;
    if slit_valid(slit) == 0 { pr_info!("SLIT table looks invalid. Not used.\n"); return -EINVAL; }
    for i in 0..(*slit).locality_count as i32 { let from_node = pxm_to_node(i); if from_node == NUMA_NO_NODE { continue; } for j in 0..(*slit).locality_count as i32 { let to_node = pxm_to_node(j); if to_node != NUMA_NO_NODE { numa_set_distance(from_node, to_node, (*slit).entry[((*slit).locality_count as i32 * i + j) as usize]); } } }
    0
}

static mut parsed_numa_memblks: i32 = 0;

unsafe fn acpi_parse_memory_affinity(header: *mut acpi_subtable_headers, _table_end: usize) -> i32 {
    let ma = header as *mut acpi_srat_mem_affinity;
    acpi_table_print_srat_entry(&mut (*header).common);
    if srat_disabled() != 0 { return 0; }
    if (*ma).header.length < core::mem::size_of::<acpi_srat_mem_affinity>() as u32 { pr_err!("SRAT: Unexpected header length: %d\n", (*ma).header.length); bad_srat(); return 0; }
    if (*ma).flags & ACPI_SRAT_MEM_ENABLED == 0 { return 0; }
    let hotpluggable = IS_ENABLED(CONFIG_MEMORY_HOTPLUG) && ((*ma).flags & ACPI_SRAT_MEM_HOT_PLUGGABLE != 0);
    let start = (*ma).base_address; let end = start + (*ma).length; let mut pxm = (*ma).proximity_domain;
    if acpi_srat_revision <= 1 { pxm &= 0xff; }
    let node = acpi_map_pxm_to_node(pxm as i32); if node == NUMA_NO_NODE { pr_err!("SRAT: Too many proximity domains.\n"); bad_srat(); return 0; }
    if numa_add_memblk(node, start, end) < 0 { pr_err!("SRAT: Failed to add memblk to node %u [mem %#010Lx-%#010Lx]\n", node, start, end - 1); bad_srat(); return 0; }
    pr_info!("SRAT: Node %u PXM %u [mem %#010Lx-%#010Lx]%s%s\n", node, pxm, start, end - 1, if hotpluggable { " hotplug" } else { "" }, if (*ma).flags & ACPI_SRAT_MEM_NON_VOLATILE != 0 { " non-volatile" } else { "" });
    if hotpluggable && memblock_mark_hotplug(start, (*ma).length) != 0 { pr_warn!("SRAT: Failed to mark hotplug range [mem %#010Lx-%#010Lx] in memblock\n", start, end - 1); }
    max_possible_pfn = max(max_possible_pfn, PFN_UP(end - 1)); parsed_numa_memblks += 1; 0
}

unsafe fn acpi_table_print_srat_entry(header: *mut acpi_subtable_header) {
    match (*header).type_ {
        ACPI_SRAT_TYPE_CPU_AFFINITY => { let p = header as *mut acpi_srat_cpu_affinity; pr_debug!("SRAT Processor (id[0x%02x] eid[0x%02x]) in proximity domain %d %s\n", (*p).apic_id, (*p).local_sapic_eid, (*p).proximity_domain_lo, str_enabled_disabled((*p).flags & ACPI_SRAT_CPU_ENABLED)); }
        ACPI_SRAT_TYPE_MEMORY_AFFINITY => { let p = header as *mut acpi_srat_mem_affinity; pr_debug!("SRAT Memory (0x%llx length 0x%llx) in proximity domain %d %s%s%s\n", (*p).base_address, (*p).length, (*p).proximity_domain, str_enabled_disabled((*p).flags & ACPI_SRAT_MEM_ENABLED), if (*p).flags & ACPI_SRAT_MEM_HOT_PLUGGABLE != 0 { " hot-pluggable" } else { "" }, if (*p).flags & ACPI_SRAT_MEM_NON_VOLATILE != 0 { " non-volatile" } else { "" }); }
        ACPI_SRAT_TYPE_X2APIC_CPU_AFFINITY => { let p = header as *mut acpi_srat_x2apic_cpu_affinity; pr_debug!("SRAT Processor (x2apicid[0x%08x]) in proximity domain %d %s\n", (*p).apic_id, (*p).proximity_domain, str_enabled_disabled((*p).flags & ACPI_SRAT_CPU_ENABLED)); }
        ACPI_SRAT_TYPE_GICC_AFFINITY => { let p = header as *mut acpi_srat_gicc_affinity; pr_debug!("SRAT Processor (acpi id[0x%04x]) in proximity domain %d %s\n", (*p).acpi_processor_uid, (*p).proximity_domain, str_enabled_disabled((*p).flags & ACPI_SRAT_GICC_ENABLED)); }
        ACPI_SRAT_TYPE_RINTC_AFFINITY => { let p = header as *mut acpi_srat_rintc_affinity; pr_debug!("SRAT Processor (acpi id[0x%04x]) in proximity domain %d %s\n", (*p).acpi_processor_uid, (*p).proximity_domain, str_enabled_disabled((*p).flags & ACPI_SRAT_RINTC_ENABLED)); }
        _ => { pr_warn!("Found unsupported SRAT entry (type = 0x%x)\n", (*header).type_); }
    }
}

unsafe fn acpi_parse_x2apic_affinity(header: *mut acpi_subtable_headers, _end: usize) -> i32 { acpi_table_print_srat_entry(&mut (*header).common); acpi_numa_x2apic_affinity_init(header as *mut acpi_srat_x2apic_cpu_affinity); 0 }
unsafe fn acpi_parse_processor_affinity(header: *mut acpi_subtable_headers, _end: usize) -> i32 { acpi_table_print_srat_entry(&mut (*header).common); acpi_numa_processor_affinity_init(header as *mut acpi_srat_cpu_affinity); 0 }
unsafe fn acpi_parse_gicc_affinity(header: *mut acpi_subtable_headers, _end: usize) -> i32 { acpi_table_print_srat_entry(&mut (*header).common); acpi_numa_gicc_affinity_init(header as *mut acpi_srat_gicc_affinity); 0 }
unsafe fn acpi_parse_gi_affinity(header: *mut acpi_subtable_headers, _end: usize) -> i32 {
    let p = header as *mut acpi_srat_generic_affinity; if p.is_null() || (*p).flags & ACPI_SRAT_GENERIC_AFFINITY_ENABLED == 0 { return -EINVAL; }
    acpi_table_print_srat_entry(&mut (*header).common); let node = acpi_map_pxm_to_node((*p).proximity_domain); if node == NUMA_NO_NODE { return -EINVAL; } node_set(node, numa_nodes_parsed); node_set_state(node, N_GENERIC_INITIATOR); 0
}
unsafe fn acpi_parse_rintc_affinity(header: *mut acpi_subtable_headers, _end: usize) -> i32 { acpi_table_print_srat_entry(&mut (*header).common); acpi_numa_rintc_affinity_init(header as *mut acpi_srat_rintc_affinity); 0 }
unsafe fn acpi_parse_srat(table: *mut acpi_table_header) -> i32 { acpi_srat_revision = (*(table as *mut acpi_table_srat)).header.revision; 0 }

pub unsafe fn acpi_numa_init_full() -> i32 {
    if acpi_disabled { return -EINVAL; }
    if acpi_table_parse(ACPI_SIG_SRAT, acpi_parse_srat) == 0 { acpi_table_parse_entries(ACPI_SIG_SRAT, core::mem::size_of::<acpi_table_srat>(), ACPI_SRAT_TYPE_CPU_AFFINITY, acpi_parse_processor_affinity, 0); acpi_table_parse_entries(ACPI_SIG_SRAT, core::mem::size_of::<acpi_table_srat>(), ACPI_SRAT_TYPE_X2APIC_CPU_AFFINITY, acpi_parse_x2apic_affinity, 0); acpi_table_parse_entries(ACPI_SIG_SRAT, core::mem::size_of::<acpi_table_srat>(), ACPI_SRAT_TYPE_GICC_AFFINITY, acpi_parse_gicc_affinity, 0); acpi_table_parse_entries(ACPI_SIG_SRAT, core::mem::size_of::<acpi_table_srat>(), ACPI_SRAT_TYPE_MEMORY_AFFINITY, acpi_parse_memory_affinity, 0); }
    acpi_table_parse(ACPI_SIG_SLIT, acpi_parse_slit); if parsed_numa_memblks == 0 { -ENOENT } else { 0 }
}

unsafe fn acpi_parse_cfmws(header: *mut acpi_subtable_headers, _arg: *mut core::ffi::c_void, _table_end: usize) -> i32 {
    let cfmws = header as *mut acpi_cedt_cfmws; let start = (*cfmws).base_hpa; let end = start + (*cfmws).window_size;
    let align = 1u64 << __ffs(start | end); if align >= SZ_256M { let err = memory_block_advise_max_size(align); if err != 0 { pr_warn!("CFMWS: memblock size advise failed (%d)\n", err); } } else { pr_err!("CFMWS: [BIOS BUG] base/size alignment violates spec\n"); }
    if numa_fill_memblks(start, end) == 0 { return 0; }
    0
}

pub unsafe fn acpi_numa_x2apic_affinity_init(pa: *mut acpi_srat_x2apic_cpu_affinity) { pr_warn!("Found unsupported x2apic [0x%08x] SRAT entry\n", (*pa).apic_id); }

pub unsafe fn acpi_numa_init() -> i32 { if acpi_disabled { return -EINVAL; } acpi_table_parse(ACPI_SIG_SLIT, acpi_parse_slit); if parsed_numa_memblks == 0 { -ENOENT } else { 0 } }

pub unsafe fn acpi_node_backed_by_real_pxm(nid: i32) -> bool { node_to_pxm(nid) <= last_real_pxm }

unsafe fn acpi_get_pxm(mut h: acpi_handle) -> i32 {
    let mut pxm = 0u64; let mut handle: acpi_handle; let mut phandle = h;
    loop { handle = phandle; let status = acpi_evaluate_integer(handle, "_PXM", core::ptr::null_mut(), &mut pxm); if ACPI_SUCCESS(status) { return pxm as i32; } let status = acpi_get_parent(handle, &mut phandle); if !ACPI_SUCCESS(status) { break; } }
    -1
}

pub unsafe fn acpi_get_node(handle: acpi_handle) -> i32 { pxm_to_node(acpi_get_pxm(handle)) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
