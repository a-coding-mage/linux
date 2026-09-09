// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * inventory.c
 *
 * Copyright (c) 1999 The Puffin Group (David Kennedy and Alex deVries)
 * Copyright (c) 2001 Matthew Wilcox for Hewlett-Packard
 *
 * These are the routines to discover what hardware exists in this box.
 */

// Linux and architecture headers supply the types, constants, globals, and
// external functions referenced below.

static mut pdc_type: i32 = PDC_TYPE_ILLEGAL;
static mut parisc_cell_num: usize = 0;
static mut parisc_cell_loc: usize = 0;
static mut parisc_pat_pdc_cap: usize = 0;

pub unsafe fn setup_pdc() {
    let mut status: isize;
    let mut bus_id: u32;
    let mut module_result = core::mem::zeroed::<pdc_system_map_mod_info>();
    let mut module_path = core::mem::zeroed::<pdc_module_path>();
    let mut model = core::mem::zeroed::<pdc_model>();

    printk(KERN_INFO as _, b"Determining PDC firmware type: \0".as_ptr() as _);
    status = pdc_system_map_find_mods(&mut module_result, &mut module_path, 0);
    if status == PDC_OK as _ { pdc_type = PDC_TYPE_SYSTEM_MAP; pr_cont(b"System Map.\n\0".as_ptr() as _); return; }

    // All 64-bit capable machines are either PAT boxes or support SYSTEM_MAP.
    #[cfg(target_pointer_width = "64")]
    {
        let mut cell_info = core::mem::zeroed::<pdc_pat_cell_num>();
        status = pdc_pat_cell_get_number(&mut cell_info);
        if status == PDC_OK as _ {
            let (mut legacy_rev, mut pat_rev) = (0usize, 0usize);
            pdc_type = PDC_TYPE_PAT;
            pr_cont(b"64 bit PAT.\n\0".as_ptr() as _);
            parisc_cell_num = cell_info.cell_num; parisc_cell_loc = cell_info.cell_loc;
            pr_info(b"PAT: Running on cell %lu and location %lu.\n\0".as_ptr() as _, parisc_cell_num, parisc_cell_loc);
            status = pdc_pat_pd_get_pdc_revisions(&mut legacy_rev, &mut pat_rev, &mut parisc_pat_pdc_cap);
            pr_info(b"PAT: legacy revision 0x%lx, pat_rev 0x%lx, pdc_cap 0x%lx, S-PTLB %d, HPMC_RENDEZ %d.\n\0".as_ptr() as _, legacy_rev, pat_rev, parisc_pat_pdc_cap, if parisc_pat_pdc_cap & PDC_PAT_CAPABILITY_BIT_SIMULTANEOUS_PTLB != 0 {1} else {0}, if parisc_pat_pdc_cap & PDC_PAT_CAPABILITY_BIT_PDC_HPMC_RENDEZ != 0 {1} else {0});
            return;
        }
    }
    status = pdc_model_info(&mut model);
    bus_id = ((model.hversion >> (4 + 7)) & 0x1f) as u32;
    match bus_id {
        0x4 | 0x6 | 0x7 | 0x8 | 0xa | 0xc => { pdc_type = PDC_TYPE_SNAKE; pr_cont(b"Snake.\n\0".as_ptr() as _); }
        _ => { pr_cont(b"Unsupported.\n\0".as_ptr() as _); panic!("If this is a 64-bit machine, please try a 64-bit kernel.\n"); }
    }
}

const PDC_PAGE_ADJ_SHIFT: usize = PAGE_SHIFT - 12;

unsafe fn set_pmem_entry(pmem_ptr: *mut physmem_range_t, start: usize, pages4k: usize) {
    if start & (PAGE_SIZE - 1) != 0 || pages4k & ((1usize << PDC_PAGE_ADJ_SHIFT) - 1) != 0 { panic!("Memory range doesn't align with page size!\n"); }
    (*pmem_ptr).start_pfn = start >> PAGE_SHIFT;
    (*pmem_ptr).pages = pages4k >> PDC_PAGE_ADJ_SHIFT;
}

unsafe fn pagezero_memconfig() {
    let npages = (PAGE_ALIGN((*PAGE0).imm_max_mem) >> PAGE_SHIFT) as usize;
    set_pmem_entry(pmem_ranges, 0, npages);
    npmem_ranges = 1;
}

#[cfg(target_pointer_width = "64")]
unsafe fn pat_query_module(pcell_loc: usize, mod_index: usize) -> i32 {
    let pa_pdc_cell = kmalloc_obj::<pdc_pat_cell_mod_maddr_block_t>();
    if pa_pdc_cell.is_null() { panic!("couldn't allocate memory for PDC_PAT_CELL!"); }
    let mut bytecnt = 0usize;
    let status = pdc_pat_cell_module(&mut bytecnt, pcell_loc, mod_index, PA_VIEW, pa_pdc_cell);
    if status != PDC_OK as _ { kfree(pa_pdc_cell as _); return status as i32; }
    let dev = alloc_pa_dev(PAT_GET_CBA((*pa_pdc_cell).cba), &(*pa_pdc_cell).mod_path);
    if dev.is_null() { kfree(pa_pdc_cell as _); return PDC_OK as i32; }
    (*dev).pcell_loc = pcell_loc; (*dev).mod_index = mod_index;
    (*dev).mod_info = (*pa_pdc_cell).mod_info; (*dev).pmod_loc = (*pa_pdc_cell).mod_location; (*dev).mod0 = (*pa_pdc_cell).mod_[0];
    register_parisc_device(dev); kfree(pa_pdc_cell as _); PDC_OK as i32
}

#[cfg(target_pointer_width = "64")]
const PAT_MAX_RANGES: usize = 4 * MAX_PHYSMEM_RANGES;

#[cfg(target_pointer_width = "64")]
unsafe fn pat_memconfig() {
    let mut actual_len = 0usize;
    let mut mem_table = [core::mem::zeroed::<pdc_pat_pd_addr_map_entry>(); PAT_MAX_RANGES + 1];
    let status = pdc_pat_pd_get_addr_map(&mut actual_len, mem_table.as_mut_ptr(), core::mem::size_of_val(&mem_table), 0);
    if status != PDC_OK as _ || actual_len % core::mem::size_of::<pdc_pat_pd_addr_map_entry>() != 0 { pagezero_memconfig(); return; }
    let entries = actual_len / core::mem::size_of::<pdc_pat_pd_addr_map_entry>();
    npmem_ranges = 0;
    for e in mem_table.iter().take(entries) {
        if e.entry_type != PAT_MEMORY_DESCRIPTOR || e.memory_type != PAT_MEMTYPE_MEMORY || e.pages == 0 || (e.memory_usage != PAT_MEMUSE_GENERAL && e.memory_usage != PAT_MEMUSE_GI && e.memory_usage != PAT_MEMUSE_GNI) { continue; }
        if npmem_ranges == MAX_PHYSMEM_RANGES { break; }
        set_pmem_entry(pmem_ranges.add(npmem_ranges), e.paddr, e.pages); npmem_ranges += 1;
    }
}

#[cfg(target_pointer_width = "64")]
unsafe fn pat_inventory() -> usize {
    let mut ci = core::mem::zeroed::<pdc_pat_cell_num>();
    if pdc_pat_cell_get_number(&mut ci) != PDC_OK as _ { return 0; }
    let mut i = 0; while pat_query_module(ci.cell_loc, i) == PDC_OK as i32 { i += 1; } i
}

#[cfg(target_pointer_width = "64")]
unsafe fn sprockets_memconfig() {
    let mut ra = core::mem::zeroed::<pdc_memory_table_raddr>();
    let mut tab = [core::mem::zeroed::<pdc_memory_table>(); MAX_PHYSMEM_RANGES];
    if pdc_mem_mem_table(&mut ra, tab.as_mut_ptr(), MAX_PHYSMEM_RANGES) != PDC_OK as _ { pagezero_memconfig(); return; }
    npmem_ranges = 0;
    for e in tab.iter().take(ra.entries_returned as usize) { set_pmem_entry(pmem_ranges.add(npmem_ranges), e.paddr, e.pages); npmem_ranges += 1; }
}

#[cfg(target_pointer_width = "32")]
unsafe fn pat_inventory() -> usize { 0 }
#[cfg(target_pointer_width = "32")]
unsafe fn pat_memconfig() {}
#[cfg(target_pointer_width = "32")]
unsafe fn sprockets_memconfig() { pagezero_memconfig(); }

#[cfg(not(feature = "pa20"))]
unsafe fn legacy_create_device(r_addr: *mut pdc_memory_map, module_path: *mut pdc_module_path) -> *mut parisc_device {
    if pdc_mem_map_hpa(r_addr, module_path) != PDC_OK as _ { return core::ptr::null_mut(); }
    let dev = alloc_pa_dev((*r_addr).hpa, &(*module_path).path); if dev.is_null() { return dev; } register_parisc_device(dev); dev
}

#[cfg(not(feature = "pa20"))]
unsafe fn snake_inventory() {
    for mod_ in 0..16 { let mut mp = core::mem::zeroed::<pdc_module_path>(); let mut ra = core::mem::zeroed::<pdc_memory_map>(); memset(mp.path.bc.as_mut_ptr() as _, 0xff, 6); mp.path.mod_ = mod_; let dev = legacy_create_device(&mut ra, &mut mp); if dev.is_null() || (*dev).id.hw_type != HPHW_BA { continue; } memset(mp.path.bc.as_mut_ptr() as _, 0xff, 4); mp.path.bc[4] = mod_; for func in 0..16 { mp.path.bc[5] = 0; mp.path.mod_ = func; legacy_create_device(&mut ra, &mut mp); } }
}
#[cfg(feature = "pa20")]
unsafe fn snake_inventory() {}

unsafe fn add_system_map_addresses(dev: *mut parisc_device, num_addrs: i32, module_instance: i32) {
    (*dev).addr = kmalloc_objs((*dev).addr, num_addrs as usize); if (*dev).addr.is_null() { return; }
    for i in 1..=num_addrs { let mut ar = core::mem::zeroed::<pdc_system_map_addr_info>(); if pdc_system_map_find_addrs(&mut ar, module_instance, i) == PDC_OK as _ { *(*dev).addr.add((*dev).num_addrs as usize) = ar.mod_addr; (*dev).num_addrs += 1; } }
}

unsafe fn system_map_inventory() {
    for i in 0..256 { let mut mr = core::mem::zeroed::<pdc_system_map_mod_info>(); let mut mp = core::mem::zeroed::<pdc_module_path>(); let s = pdc_system_map_find_mods(&mut mr, &mut mp, i); if s == PDC_BAD_PROC as _ || s == PDC_NE_MOD as _ { break; } if s != PDC_OK as _ { continue; } let dev = alloc_pa_dev(mr.mod_addr, &mp.path); if dev.is_null() { continue; } register_parisc_device(dev); if mr.add_addrs != 0 { add_system_map_addresses(dev, mr.add_addrs, i); } } walk_central_bus();
}

pub unsafe fn do_memory_inventory() { match pdc_type { PDC_TYPE_PAT => pat_memconfig(), PDC_TYPE_SYSTEM_MAP => sprockets_memconfig(), PDC_TYPE_SNAKE => { pagezero_memconfig(); return; }, _ => panic!("Unknown PDC type!\n") } if npmem_ranges == 0 || (*pmem_ranges).start_pfn != 0 { pagezero_memconfig(); } }

pub unsafe fn do_device_inventory() { printk(KERN_INFO as _, b"Searching for devices...\n\0".as_ptr() as _); init_parisc_bus(); match pdc_type { PDC_TYPE_PAT => { pat_inventory(); }, PDC_TYPE_SYSTEM_MAP => system_map_inventory(), PDC_TYPE_SNAKE => snake_inventory(), _ => panic!("Unknown PDC type!\n") } printk(KERN_INFO as _, b"Found devices:\n\0".as_ptr() as _); print_parisc_devices(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
