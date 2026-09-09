// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerPC version; direct translation of init_64.c.
 * C headers and configuration-dependent declarations are supplied elsewhere.
 */

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
unsafe fn vmemmap_subsection_start(vmemmap_addr: c_ulong) -> *mut page {
    let offset = vmemmap_addr.wrapping_sub(vmemmap as c_ulong);
    let start_pfn = (offset / core::mem::size_of::<page>() as c_ulong) & PAGE_SUBSECTION_MASK;
    pfn_to_page(start_pfn)
}

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
#[no_mangle]
pub unsafe extern "C" fn vmemmap_populated(vmemmap_addr: c_ulong, vmemmap_map_size: c_int) -> c_int {
    let vmemmap_end = vmemmap_addr.wrapping_add(vmemmap_map_size as c_ulong);
    let mut start = vmemmap_subsection_start(vmemmap_addr);
    while (start as c_ulong) < vmemmap_end {
        if pfn_valid(page_to_pfn(start)) { return 1; }
        start = start.add(PAGES_PER_SUBSECTION as usize);
    }
    0
}

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
#[no_mangle]
pub static mut vmemmap_list: *mut vmemmap_backing = core::ptr::null_mut();
#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
static mut next: *mut vmemmap_backing = core::ptr::null_mut();
#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
static mut num_left: c_int = 0;
#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
static mut num_freed: c_int = 0;

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
unsafe fn vmemmap_list_alloc(node: c_int) -> *mut vmemmap_backing {
    if num_freed != 0 {
        num_freed -= 1;
        let ret = next;
        next = (*next).list;
        return ret;
    }
    if num_left == 0 {
        next = vmemmap_alloc_block(PAGE_SIZE, node);
        if next.is_null() { WARN_ON(1); return core::ptr::null_mut(); }
        num_left = (PAGE_SIZE / core::mem::size_of::<vmemmap_backing>()) as c_int;
    }
    num_left -= 1;
    let ret = next;
    next = next.add(1);
    ret
}

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
unsafe fn vmemmap_list_populate(phys: c_ulong, start: c_ulong, node: c_int) -> c_int {
    let vmem_back = vmemmap_list_alloc(node);
    if vmem_back.is_null() { pr_debug!("vmemap list allocation failed\n"); return -ENOMEM; }
    (*vmem_back).phys = phys;
    (*vmem_back).virt_addr = start;
    (*vmem_back).list = vmemmap_list;
    vmemmap_list = vmem_back;
    0
}

#[no_mangle]
pub unsafe extern "C" fn altmap_cross_boundary(altmap: *mut vmem_altmap, start: c_ulong, page_size: c_ulong) -> bool {
    let nr_pfn = page_size / core::mem::size_of::<page>() as c_ulong;
    let start_pfn = page_to_pfn(start as *mut page);
    if start_pfn.wrapping_add(nr_pfn).wrapping_sub(1) > (*altmap).end_pfn || start_pfn < (*altmap).base_pfn { return true; }
    false
}

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
unsafe fn __vmemmap_populate(mut start: c_ulong, end: c_ulong, node: c_int, altmap: *mut vmem_altmap) -> c_int {
    let page_size = 1u64 << mmu_psize_defs[mmu_vmemmap_psize as usize].shift;
    start = ALIGN_DOWN(start, page_size);
    pr_debug!("vmemmap_populate %lx..%lx, node %d\n", start, end, node);
    while start < end {
        let mut p: *mut core::ffi::c_void = core::ptr::null_mut();
        let altmap_alloc;
        if !vmemmap_populated(start, page_size as c_int) { 
            if !altmap.is_null() && !altmap_cross_boundary(altmap, start, page_size) { p = vmemmap_alloc_block_buf(page_size, node, altmap); if p.is_null() { pr_debug!("altmap block allocation failed, falling back to system memory"); } }
            if p.is_null() { p = vmemmap_alloc_block_buf(page_size, node, core::ptr::null_mut()); altmap_alloc = false; } else { altmap_alloc = true; }
            if p.is_null() { return -ENOMEM; }
            if vmemmap_list_populate(__pa(p), start, node) != 0 {
                let nr_pfns = page_size >> PAGE_SHIFT;
                let page_order = get_order(page_size);
                if altmap_alloc { vmem_altmap_free(altmap, nr_pfns); } else { free_pages(p as c_ulong, page_order); }
                return -ENOMEM;
            }
            pr_debug!("      * %016lx..%016lx allocated at %p\n", start, start + page_size, p);
            let rc = vmemmap_create_mapping(start, page_size, __pa(p));
            if rc < 0 { pr_warn!("%s: Unable to create vmemmap mapping: %d\n", "__vmemmap_populate", rc); return -EFAULT; }
        }
        start += page_size;
    }
    0
}

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
#[no_mangle]
pub unsafe extern "C" fn vmemmap_populate(start: c_ulong, end: c_ulong, node: c_int, altmap: *mut vmem_altmap) -> c_int {
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    { if radix_enabled() { return radix__vmemmap_populate(start, end, node, altmap); } }
    __vmemmap_populate(start, end, node, altmap)
}

// CONFIG_MEMORY_HOTPLUG code is preserved as a conditional block in the source translation.
#[cfg(all(CONFIG_SPARSEMEM_VMEMMAP, CONFIG_MEMORY_HOTPLUG))]
unsafe fn vmemmap_list_free(start: c_ulong) -> c_ulong {
    let mut prev = vmemmap_list; let mut cur = vmemmap_list;
    while !cur.is_null() { if (*cur).virt_addr == start { break; } prev = cur; cur = (*cur).list; }
    if cur.is_null() { return 0; }
    if cur == vmemmap_list { vmemmap_list = (*cur).list; } else { (*prev).list = (*cur).list; }
    (*cur).list = next; next = cur; num_freed += 1; (*cur).phys
}

#[cfg(all(CONFIG_SPARSEMEM_VMEMMAP, CONFIG_MEMORY_HOTPLUG))]
unsafe fn __vmemmap_free(mut start: c_ulong, end: c_ulong, altmap: *mut vmem_altmap) {
    let page_size = 1u64 << mmu_psize_defs[mmu_vmemmap_psize as usize].shift;
    let page_order = get_order(page_size); start = ALIGN_DOWN(start, page_size);
    let (mut alt_start, mut alt_end) = (!0u64, !0u64);
    if !altmap.is_null() { alt_start = (*altmap).base_pfn; alt_end = alt_start + (*altmap).reserve + (*altmap).free; }
    while start < end {
        if vmemmap_populated(start, page_size as c_int) { start += page_size; continue; }
        let addr = vmemmap_list_free(start); if addr == 0 { start += page_size; continue; }
        let page = pfn_to_page(addr >> PAGE_SHIFT); let mut nr_pages = 1u64 << page_order; let base_pfn = PHYS_PFN(addr);
        if base_pfn >= alt_start && base_pfn < alt_end { vmem_altmap_free(altmap, nr_pages); }
        else if PageReserved(page) { while nr_pages != 0 { free_reserved_page(page); nr_pages -= 1; } }
        else { free_pages(__va(addr) as c_ulong, page_order); }
        vmemmap_remove_mapping(start, page_size); start += page_size;
    }
}

#[cfg(all(CONFIG_SPARSEMEM_VMEMMAP, CONFIG_MEMORY_HOTPLUG))]
#[no_mangle]
pub unsafe extern "C" fn vmemmap_free(start: c_ulong, end: c_ulong, altmap: *mut vmem_altmap) {
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    { if radix_enabled() { radix__vmemmap_free(start, end, altmap); return; } }
    __vmemmap_free(start, end, altmap)
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
#[no_mangle] pub static mut mmu_lpid_bits: c_uint = 0;
#[cfg(CONFIG_PPC_BOOK3S_64)]
#[no_mangle] pub static mut mmu_pid_bits: c_uint = 0;
#[cfg(CONFIG_PPC_BOOK3S_64)]
static mut disable_radix: bool = !IS_ENABLED(CONFIG_PPC_RADIX_MMU_DEFAULT);

#[cfg(CONFIG_PPC_BOOK3S_64)]
unsafe fn parse_disable_radix(p: *mut c_char) -> c_int {
    let mut val = true;
    if !p.is_null() { if kstrtobool(p, &mut val) != 0 { return -EINVAL; } }
    disable_radix = val; 0
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
unsafe fn early_check_vec5() {
    let root = of_get_flat_dt_root(); let chosen = of_get_flat_dt_subnode_by_name(root, "chosen");
    if chosen == -FDT_ERR_NOTFOUND as u64 { (*cur_cpu_spec).mmu_features &= !MMU_FTR_TYPE_RADIX; return; }
    let mut size = 0; let vec5 = of_get_flat_dt_prop(chosen, "ibm,architecture-vec-5", &mut size);
    if vec5.is_null() || size <= OV5_INDX(OV5_MMU_SUPPORT) { (*cur_cpu_spec).mmu_features &= !MMU_FTR_TYPE_RADIX; return; }
    let supported = *vec5.add(OV5_INDX(OV5_MMU_SUPPORT) as usize) & OV5_FEAT(OV5_MMU_SUPPORT);
    if supported == OV5_FEAT(OV5_MMU_RADIX) { if !early_radix_enabled() { pr_warn!("WARNING: Ignoring cmdline option disable_radix\n"); } if (*vec5.add(OV5_INDX(OV5_RADIX_GTSE) as usize) & OV5_FEAT(OV5_RADIX_GTSE)) == 0 { (*cur_cpu_spec).mmu_features &= !MMU_FTR_GTSE; } else { (*cur_cpu_spec).mmu_features |= MMU_FTR_GTSE; } (*cur_cpu_spec).mmu_features |= MMU_FTR_TYPE_RADIX; }
    else if supported == OV5_FEAT(OV5_MMU_HASH) { (*cur_cpu_spec).mmu_features &= !MMU_FTR_TYPE_RADIX; (*cur_cpu_spec).mmu_features &= !MMU_FTR_GTSE; }
}

// Remaining device-tree probing and MMU initialization retain the C interfaces and ordering.
#[cfg(CONFIG_PPC_BOOK3S_64)]
#[no_mangle] pub static mut memory_block_size: c_ulong = 1u64 << 30;

#[cfg(CONFIG_PPC_BOOK3S_64)]
unsafe extern "C" fn dt_scan_mmu_pid_width(node: c_ulong, _uname: *const c_char, _depth: c_int, _data: *mut c_void) -> c_int {
    let mut size = 0; let typ = of_get_flat_dt_prop(node, "device_type", core::ptr::null_mut());
    if typ.is_null() || strcmp(typ, "cpu") != 0 { return 0; }
    let mut prop = of_get_flat_dt_prop(node, "ibm,mmu-lpid-bits", &mut size);
    if !prop.is_null() && size == 4 { mmu_lpid_bits = be32_to_cpup(prop); }
    prop = of_get_flat_dt_prop(node, "ibm,mmu-pid-bits", &mut size);
    if !prop.is_null() && size == 4 { mmu_pid_bits = be32_to_cpup(prop); }
    if mmu_pid_bits == 0 && mmu_lpid_bits == 0 { 0 } else { 1 }
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
unsafe extern "C" fn probe_memory_block_size(node: c_ulong, uname: *const c_char, depth: c_int, data: *mut c_void) -> c_int {
    if depth != 1 { return 0; }
    let block_size = data as *mut c_ulong; let mut len = 0;
    if strcmp(uname, "ibm,dynamic-reconfiguration-memory") == 0 {
        let prop = of_get_flat_dt_prop(node, "ibm,lmb-size", &mut len);
        if prop.is_null() || len < dt_root_size_cells * core::mem::size_of::<u32>() as c_int { *block_size = DEFAULT_MEMORY_BLOCK_SIZE; }
        else { *block_size = of_read_number(prop, dt_root_size_cells); }
        return 1;
    }
    let typ = of_get_flat_dt_prop(node, "device_type", core::ptr::null_mut());
    if typ.is_null() || strcmp(typ, "memory") != 0 { return 0; }
    let mut reg = of_get_flat_dt_prop(node, "linux,usable-memory", &mut len);
    if reg.is_null() { reg = of_get_flat_dt_prop(node, "reg", &mut len); }
    if reg.is_null() { return 0; }
    let end = reg.add((len as usize) / core::mem::size_of::<u32>());
    while end.offset_from(reg) >= (dt_root_addr_cells + dt_root_size_cells) as isize {
        dt_mem_next_cell(dt_root_addr_cells, &mut reg);
        let size = dt_mem_next_cell(dt_root_size_cells, &mut reg);
        if size != 0 { update_memory_block_size(block_size, size); continue; }
        let compatible = of_get_flat_dt_prop(node, "compatible", core::ptr::null_mut());
        if !compatible.is_null() && strcmp(compatible, "ibm,coherent-device-memory") == 0 {
            if *block_size > SZ_256M { *block_size = SZ_256M; }
            return 0;
        }
    }
    0
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
unsafe fn update_memory_block_size(block_size: *mut c_ulong, mem_size: c_ulong) {
    let min_size = DEFAULT_MEMORY_BLOCK_SIZE;
    while *block_size > min_size { if mem_size & *block_size == 0 { break; } *block_size >>= 2; }
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
#[no_mangle]
pub unsafe extern "C" fn mmu_early_init_devtree() {
    let hvmode = (mfmsr() & MSR_HV) != 0;
    if disable_radix { if IS_ENABLED(CONFIG_PPC_64S_HASH_MMU) { (*cur_cpu_spec).mmu_features &= !MMU_FTR_TYPE_RADIX; } else { pr_warn!("WARNING: Ignoring cmdline option disable_radix\n"); } }
    of_scan_flat_dt(dt_scan_mmu_pid_width, core::ptr::null_mut());
    if hvmode && mmu_lpid_bits == 0 { mmu_lpid_bits = if early_cpu_has_feature(CPU_FTR_ARCH_32) { 16 } else if early_cpu_has_feature(CPU_FTR_ARCH_207S) { 12 } else { 10 }; }
    if mmu_pid_bits == 0 && early_cpu_has_feature(CPU_FTR_ARCH_300) { mmu_pid_bits = 20; }
    if !hvmode { early_check_vec5(); }
    of_scan_flat_dt(probe_memory_block_size, &mut memory_block_size as *mut _ as *mut c_void);
    if early_radix_enabled() { radix__early_init_devtree(); ppc64_rma_size = ULONG_MAX; memblock_set_current_limit(MEMBLOCK_ALLOC_ANYWHERE); } else { hash__early_init_devtree(); }
    if IS_ENABLED(CONFIG_HUGETLB_PAGE_SIZE_VARIABLE) { hugetlbpage_init_defaultsize(); }
    if (*cur_cpu_spec).mmu_features & MMU_FTR_HPTE_TABLE == 0 && (*cur_cpu_spec).mmu_features & MMU_FTR_TYPE_RADIX == 0 { panic!("kernel does not support any MMU type offered by platform"); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
