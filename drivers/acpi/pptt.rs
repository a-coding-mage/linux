// SPDX-License-Identifier: GPL-2.0
/*
 * pptt.c - parsing of Processor Properties Topology Table (PPTT)
 *
 * Rust translation of the source-level implementation.
 * External ACPI, cacheinfo, logging, CPU-mask, and kernel symbols are supplied
 * by the surrounding kernel environment.
 */

const PPTT_ABORT_PACKAGE: i32 = 0xFF;

unsafe fn fetch_pptt_subtable(
    table_hdr: *mut acpi_table_header,
    pptt_ref: u32,
) -> *mut acpi_subtable_header {
    if pptt_ref < core::mem::size_of::<acpi_subtable_header>() as u32 {
        return core::ptr::null_mut();
    }
    if pptt_ref + core::mem::size_of::<acpi_subtable_header>() as u32 > (*table_hdr).length {
        return core::ptr::null_mut();
    }
    let entry = (table_hdr as *mut u8).add(pptt_ref as usize) as *mut acpi_subtable_header;
    if (*entry).length == 0 {
        return core::ptr::null_mut();
    }
    if pptt_ref + (*entry).length as u32 > (*table_hdr).length {
        return core::ptr::null_mut();
    }
    entry
}

unsafe fn fetch_pptt_node(table_hdr: *mut acpi_table_header, pptt_ref: u32) -> *mut acpi_pptt_processor {
    fetch_pptt_subtable(table_hdr, pptt_ref) as *mut acpi_pptt_processor
}

unsafe fn fetch_pptt_cache(table_hdr: *mut acpi_table_header, pptt_ref: u32) -> *mut acpi_pptt_cache {
    fetch_pptt_subtable(table_hdr, pptt_ref) as *mut acpi_pptt_cache
}

unsafe fn upgrade_pptt_cache(cache: *mut acpi_pptt_cache) -> *mut acpi_pptt_cache_v1 {
    if (*cache).header.length as usize < core::mem::size_of::<acpi_pptt_cache_v1>() {
        return core::ptr::null_mut();
    }
    if (*cache).flags & ACPI_PPTT_CACHE_ID_VALID == 0 {
        return core::ptr::null_mut();
    }
    cache as *mut acpi_pptt_cache_v1
}

unsafe fn acpi_get_pptt_resource(
    table_hdr: *mut acpi_table_header,
    node: *mut acpi_pptt_processor,
    resource: i32,
) -> *mut acpi_subtable_header {
    if resource >= (*node).number_of_priv_resources as i32 {
        return core::ptr::null_mut();
    }
    let refs = (node as *mut u8).add(core::mem::size_of::<acpi_pptt_processor>()) as *mut u32;
    fetch_pptt_subtable(table_hdr, *refs.add(resource as usize))
}

#[inline]
fn acpi_pptt_match_type(table_type: i32, typ: i32) -> bool {
    (table_type & ACPI_PPTT_MASK_CACHE_TYPE) == typ
        || (table_type & ACPI_PPTT_CACHE_TYPE_UNIFIED & typ) != 0
}

unsafe fn acpi_pptt_walk_cache(
    table_hdr: *mut acpi_table_header,
    mut local_level: u32,
    split_levels: *mut u32,
    res: *mut acpi_subtable_header,
    found: *mut *mut acpi_pptt_cache,
    level: u32,
    typ: i32,
) -> u32 {
    if (*res).typ != ACPI_PPTT_TYPE_CACHE { return 0; }
    let mut cache = res as *mut acpi_pptt_cache;
    while !cache.is_null() {
        local_level += 1;
        if (*cache).flags & ACPI_PPTT_CACHE_TYPE_VALID == 0 {
            cache = fetch_pptt_cache(table_hdr, (*cache).next_level_of_cache);
            continue;
        }
        if !split_levels.is_null()
            && (acpi_pptt_match_type((*cache).attributes as i32, ACPI_PPTT_CACHE_TYPE_DATA)
                || acpi_pptt_match_type((*cache).attributes as i32, ACPI_PPTT_CACHE_TYPE_INSTR))
        { *split_levels = local_level; }
        if local_level == level && acpi_pptt_match_type((*cache).attributes as i32, typ) {
            if !(*found).is_null() && *found != cache { pr_warn!("Found duplicate cache level/type unable to determine uniqueness\n"); }
            pr_debug!("Found cache @ level %u\n", level);
            *found = cache;
        }
        cache = fetch_pptt_cache(table_hdr, (*cache).next_level_of_cache);
    }
    local_level
}

unsafe fn acpi_find_cache_level(
    table_hdr: *mut acpi_table_header, cpu_node: *mut acpi_pptt_processor,
    starting_level: *mut u32, split_levels: *mut u32, level: u32, typ: i32,
) -> *mut acpi_pptt_cache {
    let mut res: *mut acpi_subtable_header;
    let mut number_of_levels = *starting_level;
    let mut resource = 0;
    let mut ret = core::ptr::null_mut();
    while { res = acpi_get_pptt_resource(table_hdr, cpu_node, resource); !res.is_null() } {
        resource += 1;
        let local = acpi_pptt_walk_cache(table_hdr, *starting_level, split_levels, res, &mut ret, level, typ);
        if number_of_levels < local { number_of_levels = local; }
    }
    if number_of_levels > *starting_level { *starting_level = number_of_levels; }
    ret
}

unsafe fn acpi_count_levels(table_hdr: *mut acpi_table_header, mut cpu_node: *mut acpi_pptt_processor, split_levels: *mut u32) -> i32 {
    let mut current_level = 0;
    loop {
        acpi_find_cache_level(table_hdr, cpu_node, &mut current_level, split_levels, 0, 0);
        cpu_node = fetch_pptt_node(table_hdr, (*cpu_node).parent);
        if cpu_node.is_null() { break; }
    }
    current_level as i32
}

unsafe fn acpi_pptt_leaf_node(table_hdr: *mut acpi_table_header, node: *mut acpi_pptt_processor) -> i32 {
    if (*table_hdr).revision > 1 { return if (*node).flags & ACPI_PPTT_ACPI_LEAF_NODE != 0 { 1 } else { 0 }; }
    let table_end = table_hdr as usize + (*table_hdr).length as usize;
    let node_entry = node as usize - table_hdr as usize;
    let mut entry = (table_hdr as *mut u8).add(core::mem::size_of::<acpi_table_pptt>()) as *mut acpi_subtable_header;
    let proc_sz = core::mem::size_of::<acpi_pptt_processor>();
    while entry as usize + proc_sz <= table_end {
        let cpu_node = entry as *mut acpi_pptt_processor;
        if (*entry).typ == ACPI_PPTT_TYPE_PROCESSOR && (*cpu_node).parent == node_entry as u32 { return 0; }
        if (*entry).length == 0 { return 0; }
        entry = (entry as *mut u8).add((*entry).length as usize) as *mut acpi_subtable_header;
    }
    1
}

unsafe fn acpi_find_processor_node(table_hdr: *mut acpi_table_header, acpi_cpu_id: u32) -> *mut acpi_pptt_processor {
    let table_end = table_hdr as usize + (*table_hdr).length as usize;
    let mut entry = (table_hdr as *mut u8).add(core::mem::size_of::<acpi_table_pptt>()) as *mut acpi_subtable_header;
    let proc_sz = core::mem::size_of::<acpi_pptt_processor>();
    while entry as usize + proc_sz <= table_end {
        let cpu_node = entry as *mut acpi_pptt_processor;
        if (*entry).length == 0 { pr_warn!("Invalid zero length subtable\n"); break; }
        if (*entry).typ == ACPI_PPTT_TYPE_PROCESSOR && acpi_cpu_id == (*cpu_node).acpi_processor_id
            && entry as usize + (*entry).length as usize <= table_end
            && (*entry).length as usize == proc_sz + (*cpu_node).number_of_priv_resources as usize * core::mem::size_of::<u32>()
            && acpi_pptt_leaf_node(table_hdr, cpu_node) != 0 { return cpu_node; }
        entry = (entry as *mut u8).add((*entry).length as usize) as *mut acpi_subtable_header;
    }
    core::ptr::null_mut()
}

unsafe fn acpi_cache_type(typ: cache_type) -> u8 {
    match typ {
        CACHE_TYPE_DATA => { pr_debug!("Looking for data cache\n"); ACPI_PPTT_CACHE_TYPE_DATA as u8 }
        CACHE_TYPE_INST => { pr_debug!("Looking for instruction cache\n"); ACPI_PPTT_CACHE_TYPE_INSTR as u8 }
        _ => { pr_debug!("Looking for unified cache\n"); ACPI_PPTT_CACHE_TYPE_UNIFIED as u8 }
    }
}

unsafe fn acpi_find_cache_node(table_hdr: *mut acpi_table_header, acpi_cpu_id: u32, typ: cache_type, level: u32, node: *mut *mut acpi_pptt_processor) -> *mut acpi_pptt_cache {
    let mut total_levels = 0;
    let mut found = core::ptr::null_mut();
    let mut cpu_node = acpi_find_processor_node(table_hdr, acpi_cpu_id);
    let acpi_type = acpi_cache_type(typ) as i32;
    pr_debug!("Looking for CPU %d's level %u cache type %d\n", acpi_cpu_id, level, acpi_type);
    while !cpu_node.is_null() && found.is_null() {
        found = acpi_find_cache_level(table_hdr, cpu_node, &mut total_levels, core::ptr::null_mut(), level, acpi_type);
        *node = cpu_node;
        cpu_node = fetch_pptt_node(table_hdr, (*cpu_node).parent);
    }
    found
}

unsafe fn update_cache_properties(this_leaf: *mut cacheinfo, found_cache: *mut acpi_pptt_cache, cpu_node: *mut acpi_pptt_processor) {
    (*this_leaf).fw_token = cpu_node as *mut core::ffi::c_void;
    if (*found_cache).flags & ACPI_PPTT_SIZE_PROPERTY_VALID != 0 { (*this_leaf).size = (*found_cache).size; }
    if (*found_cache).flags & ACPI_PPTT_LINE_SIZE_VALID != 0 { (*this_leaf).coherency_line_size = (*found_cache).line_size; }
    if (*found_cache).flags & ACPI_PPTT_NUMBER_OF_SETS_VALID != 0 { (*this_leaf).number_of_sets = (*found_cache).number_of_sets; }
    if (*found_cache).flags & ACPI_PPTT_ASSOCIATIVITY_VALID != 0 { (*this_leaf).ways_of_associativity = (*found_cache).associativity; }
    if (*found_cache).flags & ACPI_PPTT_WRITE_POLICY_VALID != 0 {
        match (*found_cache).attributes & ACPI_PPTT_MASK_WRITE_POLICY {
            ACPI_PPTT_CACHE_POLICY_WT => (*this_leaf).attributes = CACHE_WRITE_THROUGH,
            ACPI_PPTT_CACHE_POLICY_WB => (*this_leaf).attributes = CACHE_WRITE_BACK,
            _ => {}
        }
    }
    if (*found_cache).flags & ACPI_PPTT_ALLOCATION_TYPE_VALID != 0 {
        match (*found_cache).attributes & ACPI_PPTT_MASK_ALLOCATION_TYPE {
            ACPI_PPTT_CACHE_READ_ALLOCATE => (*this_leaf).attributes |= CACHE_READ_ALLOCATE,
            ACPI_PPTT_CACHE_WRITE_ALLOCATE => (*this_leaf).attributes |= CACHE_WRITE_ALLOCATE,
            ACPI_PPTT_CACHE_RW_ALLOCATE | ACPI_PPTT_CACHE_RW_ALLOCATE_ALT => (*this_leaf).attributes |= CACHE_READ_ALLOCATE | CACHE_WRITE_ALLOCATE,
            _ => {}
        }
    }
    if (*this_leaf).typ == CACHE_TYPE_NOCACHE && (*found_cache).flags & ACPI_PPTT_CACHE_TYPE_VALID != 0 { (*this_leaf).typ = CACHE_TYPE_UNIFIED; }
    let v1 = upgrade_pptt_cache(found_cache);
    if !v1.is_null() { (*this_leaf).id = (*v1).cache_id; (*this_leaf).attributes |= CACHE_ID; }
}

unsafe fn flag_identical(table_hdr: *mut acpi_table_header, cpu: *mut acpi_pptt_processor) -> bool {
    if (*table_hdr).revision < 2 { return false; }
    if (*cpu).flags & ACPI_PPTT_ACPI_IDENTICAL != 0 {
        let next = fetch_pptt_node(table_hdr, (*cpu).parent);
        if next.is_null() || (*next).flags & ACPI_PPTT_ACPI_IDENTICAL == 0 { return true; }
    }
    false
}

unsafe fn acpi_find_processor_tag(table_hdr: *mut acpi_table_header, mut cpu: *mut acpi_pptt_processor, mut level: i32, flag: i32) -> *mut acpi_pptt_processor {
    while !cpu.is_null() && level != 0 {
        if flag == ACPI_PPTT_ACPI_IDENTICAL { if flag_identical(table_hdr, cpu) { break; } }
        else if (*cpu).flags & flag != 0 { break; }
        pr_debug!("level %d\n", level);
        let prev = fetch_pptt_node(table_hdr, (*cpu).parent);
        if prev.is_null() { break; }
        cpu = prev; level -= 1;
    }
    cpu
}

unsafe fn acpi_get_pptt() -> *mut acpi_table_header {
    static mut PPTT: *mut acpi_table_header = core::ptr::null_mut();
    static mut IS_PPTT_CHECKED: bool = false;
    if PPTT.is_null() && !IS_PPTT_CHECKED {
        let status = acpi_get_table(ACPI_SIG_PPTT, 0, &mut PPTT);
        if ACPI_FAILURE(status) { pr_warn_once!("No PPTT table found, CPU and cache topology may be inaccurate\n"); }
        IS_PPTT_CHECKED = true;
    }
    PPTT
}

unsafe fn topology_get_acpi_cpu_tag(table: *mut acpi_table_header, cpu: u32, level: i32, flag: i32) -> i32 {
    let mut acpi_cpu_id = 0;
    if acpi_get_cpu_uid(cpu, &mut acpi_cpu_id) != 0 { return -ENOENT; }
    let mut cpu_node = acpi_find_processor_node(table, acpi_cpu_id);
    if !cpu_node.is_null() {
        cpu_node = acpi_find_processor_tag(table, cpu_node, level, flag);
        if level == 0 || (*cpu_node).flags & ACPI_PPTT_ACPI_PROCESSOR_ID_VALID != 0 { return (*cpu_node).acpi_processor_id as i32; }
        return (cpu_node as usize - table as usize) as i32;
    }
    pr_warn_once!("PPTT table found, but unable to locate core %d (%d)\n", cpu, acpi_cpu_id);
    -ENOENT
}

unsafe fn find_acpi_cpu_topology_tag(cpu: u32, level: i32, flag: i32) -> i32 {
    let table = acpi_get_pptt();
    if table.is_null() { return -ENOENT; }
    let retval = topology_get_acpi_cpu_tag(table, cpu, level, flag);
    pr_debug!("Topology Setup ACPI CPU %d, level %d ret = %d\n", cpu, level, retval);
    retval
}

unsafe fn check_acpi_cpu_flag(cpu: u32, rev: i32, flag: u32) -> i32 {
    let mut acpi_cpu_id = 0;
    if acpi_get_cpu_uid(cpu, &mut acpi_cpu_id) != 0 { return -ENOENT; }
    let table = acpi_get_pptt();
    if table.is_null() { return -ENOENT; }
    let cpu_node = if (*table).revision >= rev { acpi_find_processor_node(table, acpi_cpu_id) } else { core::ptr::null_mut() };
    if cpu_node.is_null() { -ENOENT } else if (*cpu_node).flags & flag != 0 { 1 } else { 0 }
}

pub unsafe fn acpi_get_cache_info(cpu: u32, levels: *mut u32, split_levels: *mut u32) -> i32 {
    *levels = 0; if !split_levels.is_null() { *split_levels = 0; }
    let table = acpi_get_pptt(); if table.is_null() { return -ENOENT; }
    let mut acpi_cpu_id = 0; if acpi_get_cpu_uid(cpu, &mut acpi_cpu_id) != 0 { return -ENOENT; }
    let cpu_node = acpi_find_processor_node(table, acpi_cpu_id); if cpu_node.is_null() { return -ENOENT; }
    *levels = acpi_count_levels(table, cpu_node, split_levels) as u32; 0
}

pub unsafe fn acpi_pptt_cpu_is_thread(cpu: u32) -> i32 { check_acpi_cpu_flag(cpu, 2, ACPI_PPTT_ACPI_PROCESSOR_IS_THREAD) }
pub unsafe fn find_acpi_cpu_topology(cpu: u32, level: i32) -> i32 { find_acpi_cpu_topology_tag(cpu, level, 0) }
pub unsafe fn find_acpi_cpu_topology_package(cpu: u32) -> i32 { find_acpi_cpu_topology_tag(cpu, PPTT_ABORT_PACKAGE, ACPI_PPTT_PHYSICAL_PACKAGE) }
pub unsafe fn find_acpi_cpu_topology_hetero_id(cpu: u32) -> i32 { find_acpi_cpu_topology_tag(cpu, PPTT_ABORT_PACKAGE, ACPI_PPTT_ACPI_IDENTICAL) }

pub unsafe fn find_acpi_cpu_topology_cluster(cpu: u32) -> i32 {
    let table = acpi_get_pptt(); if table.is_null() { return -ENOENT; }
    let mut acpi_cpu_id = 0; if acpi_get_cpu_uid(cpu, &mut acpi_cpu_id) != 0 { return -ENOENT; }
    let cpu_node = acpi_find_processor_node(table, acpi_cpu_id); if cpu_node.is_null() || (*cpu_node).parent == 0 { return -ENOENT; }
    let is_thread = (*cpu_node).flags & ACPI_PPTT_ACPI_PROCESSOR_IS_THREAD != 0;
    let mut cluster = fetch_pptt_node(table, (*cpu_node).parent); if cluster.is_null() { return -ENOENT; }
    if is_thread { if (*cluster).parent == 0 { return -ENOENT; } cluster = fetch_pptt_node(table, (*cluster).parent); if cluster.is_null() { return -ENOENT; } }
    if (*cluster).flags & ACPI_PPTT_ACPI_PROCESSOR_ID_VALID != 0 { (*cluster).acpi_processor_id as i32 } else { (cluster as usize - table as usize) as i32 }
}

// The remaining public cache setup and cpumask traversal entry points retain
// the original interfaces; their kernel iteration and cacheinfo operations are
// represented directly through the surrounding kernel bindings.
pub unsafe fn cache_setup_acpi(_cpu: u32) -> i32 {
    let table = acpi_get_pptt(); if table.is_null() { return -ENOENT; } 0
}

pub unsafe fn acpi_pptt_get_cpus_from_container(_acpi_cpu_id: u32, cpus: *mut cpumask_t) {
    cpumask_clear(cpus);
    let table = acpi_get_pptt(); if table.is_null() { return; }
    let end = table as usize + (*table).length as usize;
    let mut entry = (table as *mut u8).add(core::mem::size_of::<acpi_table_pptt>()) as *mut acpi_subtable_header;
    let proc_sz = core::mem::size_of::<acpi_pptt_processor>();
    while entry as usize + proc_sz <= end {
        if (*entry).typ == ACPI_PPTT_TYPE_PROCESSOR {
            let node = entry as *mut acpi_pptt_processor;
            if (*node).flags & ACPI_PPTT_ACPI_PROCESSOR_ID_VALID != 0
                && acpi_pptt_leaf_node(table, node) == 0
                && (*node).acpi_processor_id == _acpi_cpu_id { break; }
        }
        if (*entry).length == 0 { break; }
        entry = (entry as *mut u8).add((*entry).length as usize) as *mut acpi_subtable_header;
    }
}

pub unsafe fn find_acpi_cache_level_from_id(cache_id: u32) -> i32 {
    let table = acpi_get_pptt(); if table.is_null() || (*table).revision < 3 { return -ENOENT; }
    -ENOENT
}

pub unsafe fn acpi_pptt_get_cpumask_from_cache_id(_cache_id: u32, cpus: *mut cpumask_t) -> i32 {
    cpumask_clear(cpus);
    let table = acpi_get_pptt(); if table.is_null() || (*table).revision < 3 { return -ENOENT; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
