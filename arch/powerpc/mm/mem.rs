// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  PowerPC version
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
 *  and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *    Copyright (C) 1996 Paul Mackerras
 *  PPC44x/36-bit changes by Matt Porter (mporter@mvista.com)
 *
 *  Derived from "arch/i386/mm/init.c"
 *    Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 */

// Dependencies are supplied by the surrounding kernel translation.

pub static mut memory_limit: u64 = 0;

pub unsafe fn __phys_mem_access_prot(
    pfn: c_ulong,
    size: c_ulong,
    mut vma_prot: pgprot_t,
) -> pgprot_t {
    if ppc_md.phys_mem_access_prot.is_some() {
        return (ppc_md.phys_mem_access_prot.unwrap())(pfn, size, vma_prot);
    }

    if !page_is_ram(pfn) {
        vma_prot = pgprot_noncached(vma_prot);
    }

    vma_prot
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
static mut linear_mapping_mutex: DEFINE_MUTEX!(()) = DEFINE_MUTEX!();

#[cfg(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_NUMA"))]
pub unsafe fn memory_add_physaddr_to_nid(start: u64) -> c_int {
    hot_add_scn_to_nid(start)
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub unsafe fn create_section_mapping(
    _start: c_ulong,
    _end: c_ulong,
    _nid: c_int,
    _prot: pgprot_t,
) -> c_int {
    -ENODEV
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub unsafe fn remove_section_mapping(_start: c_ulong, _end: c_ulong) -> c_int {
    -ENODEV
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub unsafe fn arch_create_linear_mapping(
    nid: c_int,
    mut start: u64,
    size: u64,
    params: *mut mhp_params,
) -> c_int {
    start = __va(start) as c_ulong as u64;
    mutex_lock(&raw mut linear_mapping_mutex);
    let rc = create_section_mapping(start as c_ulong, (start + size) as c_ulong, nid, (*params).pgprot);
    mutex_unlock(&raw mut linear_mapping_mutex);
    if rc != 0 {
        pr_warn!("Unable to create linear mapping for 0x{:x}..0x{:x}: {}\n", start, start + size, rc);
        return -EFAULT;
    }
    0
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub unsafe fn arch_remove_linear_mapping(mut start: u64, size: u64) {
    start = __va(start) as c_ulong as u64;
    mutex_lock(&raw mut linear_mapping_mutex);
    let ret = remove_section_mapping(start as c_ulong, (start + size) as c_ulong);
    mutex_unlock(&raw mut linear_mapping_mutex);
    if ret != 0 {
        pr_warn!("Unable to remove linear mapping for 0x{:x}..0x{:x}: {}\n", start, start + size, ret);
    }
    vm_unmap_aliases();
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
unsafe fn update_end_of_memory_vars(start: u64, size: u64) {
    let end_pfn = PFN_UP(start + size);
    if end_pfn > max_pfn {
        max_pfn = end_pfn;
        max_low_pfn = end_pfn;
        high_memory = (__va(max_pfn * PAGE_SIZE - 1) as *mut c_void).add(1);
    }
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub unsafe fn add_pages(nid: c_int, start_pfn: c_ulong, nr_pages: c_ulong, params: *mut mhp_params) -> c_int {
    let ret = __add_pages(nid, start_pfn, nr_pages, params);
    if ret != 0 { return ret; }
    update_end_of_memory_vars(start_pfn << PAGE_SHIFT, nr_pages << PAGE_SHIFT);
    ret
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub unsafe fn arch_add_memory(nid: c_int, start: u64, size: u64, params: *mut mhp_params) -> c_int {
    let start_pfn = start >> PAGE_SHIFT;
    let nr_pages = size >> PAGE_SHIFT;
    let rc = arch_create_linear_mapping(nid, start, size, params);
    if rc != 0 { return rc; }
    let rc = add_pages(nid, start_pfn, nr_pages, params);
    if rc != 0 { arch_remove_linear_mapping(start, size); }
    rc
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub unsafe fn arch_remove_memory(start: u64, size: u64, altmap: *mut vmem_altmap, pgmap: *mut dev_pagemap) {
    __remove_pages(start >> PAGE_SHIFT, size >> PAGE_SHIFT, altmap, pgmap);
    arch_remove_linear_mapping(start, size);
}

#[cfg(not(feature = "CONFIG_NUMA"))]
pub unsafe fn mem_topology_setup() {
    max_low_pfn = max_pfn = memblock_end_of_DRAM() >> PAGE_SHIFT;
    min_low_pfn = MEMORY_START >> PAGE_SHIFT;
    #[cfg(feature = "CONFIG_HIGHMEM")]
    { max_low_pfn = lowmem_end_addr >> PAGE_SHIFT; }
    memblock_set_node(0, PHYS_ADDR_MAX, &raw mut memblock.memory, 0);
}

#[cfg(not(feature = "CONFIG_NUMA"))]
unsafe fn mark_nonram_nosave() -> c_int {
    let (mut spfn, mut epfn, mut prev) = (0, 0, 0);
    for_each_mem_pfn_range!(i, MAX_NUMNODES, &mut spfn, &mut epfn, core::ptr::null_mut(), {
        if prev != 0 && prev < spfn { register_nosave_region(prev, spfn); }
        prev = epfn;
    });
    0
}

#[cfg(feature = "CONFIG_NUMA")]
unsafe fn mark_nonram_nosave() -> c_int { 0 }

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) {
    #[cfg(feature = "CONFIG_ZONE_DMA")]
    { *max_zone_pfns.add(ZONE_DMA) = min((zone_dma_limit >> PAGE_SHIFT) + 1, max_low_pfn); }
    *max_zone_pfns.add(ZONE_NORMAL) = max_low_pfn;
    #[cfg(feature = "CONFIG_HIGHMEM")]
    { *max_zone_pfns.add(ZONE_HIGHMEM) = max_pfn; }
}

pub unsafe fn paging_init() {
    let total_ram: u64 = memblock_phys_mem_size();
    let top_of_ram: phys_addr_t = memblock_end_of_DRAM();
    let zone_dma_bits: c_int;
    #[cfg(feature = "CONFIG_HIGHMEM")]
    {
        let mut v = __fix_to_virt(FIX_KMAP_END);
        let end = __fix_to_virt(FIX_KMAP_BEGIN);
        while v < end { map_kernel_page(v, 0, __pgprot(0)); v += PAGE_SIZE; }
        map_kernel_page(PKMAP_BASE, 0, __pgprot(0));
        pkmap_page_table = virt_to_kpte(PKMAP_BASE);
    }
    printk!(KERN_DEBUG "Top of RAM: 0x{:x}, Total RAM: 0x{:x}\n", top_of_ram, total_ram);
    printk!(KERN_DEBUG "Memory hole size: {}MB\n", ((top_of_ram - total_ram) >> 20) as c_long);
    if cfg!(feature = "CONFIG_PPC32") { zone_dma_bits = 30; } else { zone_dma_bits = 31; }
    zone_dma_limit = DMA_BIT_MASK(zone_dma_bits);
    mark_nonram_nosave();
}

pub unsafe fn arch_mm_preinit() {
    fadump_cma_init(); kdump_cma_reserve(); kvm_cma_reserve();
    BUILD_BUG_ON!(MMU_PAGE_COUNT > 16);
    #[cfg(feature = "CONFIG_SWIOTLB")]
    { memblock_set_bottom_up(true); swiotlb_init(ppc_swiotlb_enable, ppc_swiotlb_flags); }
    kasan_late_init();
    #[cfg(all(feature = "CONFIG_PPC_E500", not(feature = "CONFIG_SMP")))]
    { per_cpu!(next_tlbcam_idx, smp_processor_id()) = (mfspr(SPRN_TLB1CFG) & TLBnCFG_N_ENTRY) - 1; }
}

pub unsafe fn free_initmem() {
    ppc_md.progress = ppc_printk_progress;
    mark_initmem_nx(); free_initmem_default(POISON_FREE_INITMEM); ftrace_free_init_tramp();
}

unsafe fn add_system_ram_resources() -> c_int {
    let (mut start, mut end) = (0, 0);
    for_each_mem_range!(i, &mut start, &mut end, {
        let res = kzalloc_obj!(resource);
        WARN_ON!(res.is_null());
        if !res.is_null() { (*res).name = "System RAM"; (*res).start = start; (*res).end = end - 1; (*res).flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY; WARN_ON!(insert_resource(&raw mut iomem_resource, res) < 0); }
    });
    0
}

#[cfg(feature = "CONFIG_STRICT_DEVMEM")]
pub unsafe fn devmem_is_allowed(pfn: c_ulong) -> c_int {
    if page_is_rtas_user_buf(pfn) { return 1; }
    if iomem_is_exclusive(PFN_PHYS(pfn)) { return 0; }
    if !page_is_ram(pfn) { return 1; }
    0
}

#[cfg(feature = "CONFIG_EXECMEM")]
static mut execmem_info: execmem_info = execmem_info_zeroed!();

#[cfg(all(feature = "CONFIG_EXECMEM", any(feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_BOOK3S_603")))]
unsafe fn prealloc_execmem_pgtable() {
    let mut va = ALIGN_DOWN(MODULES_VADDR, PGDIR_SIZE);
    while va < MODULES_END { pte_alloc_kernel(pmd_off_k(va), va); va += PGDIR_SIZE; }
}

#[cfg(all(feature = "CONFIG_EXECMEM", not(any(feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_BOOK3S_603"))))]
unsafe fn prealloc_execmem_pgtable() {}

#[cfg(feature = "CONFIG_EXECMEM")]
pub unsafe fn execmem_arch_setup() -> *mut execmem_info {
    let kprobes_prot = if strict_module_rwx_enabled() { PAGE_KERNEL_ROX } else { PAGE_KERNEL_EXEC };
    let prot = if strict_module_rwx_enabled() { PAGE_KERNEL } else { PAGE_KERNEL_EXEC };
    let (mut fallback_start, mut fallback_end, start, end);
    #[cfg(feature = "MODULES_VADDR")]
    { let limit = _etext as c_ulong - SZ_32M; if MODULES_VADDR < PAGE_OFFSET && MODULES_END > limit { start = limit; fallback_start = MODULES_VADDR; fallback_end = MODULES_END; } else { start = MODULES_VADDR; fallback_start = 0; fallback_end = 0; } end = MODULES_END; }
    #[cfg(not(feature = "MODULES_VADDR"))]
    { start = VMALLOC_START; end = VMALLOC_END; fallback_start = 0; fallback_end = 0; }
    prealloc_execmem_pgtable();
    execmem_info = execmem_info { ranges: [
        execmem_range { start, end, pgprot: prot, alignment: 1, fallback_start, fallback_end },
        execmem_range { start: VMALLOC_START, end: VMALLOC_END, pgprot: kprobes_prot, alignment: 1, fallback_start: 0, fallback_end: 0 },
        execmem_range { start: VMALLOC_START, end: VMALLOC_END, pgprot: PAGE_KERNEL, alignment: 1, fallback_start: 0, fallback_end: 0 },
    ] };
    &raw mut execmem_info
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
