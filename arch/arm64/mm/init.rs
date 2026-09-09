// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/mm/init.c
 *
 * Copyright (C) 1995-2005 Russell King
 * Copyright (C) 2012 ARM Ltd.
 */

// Linux kernel dependencies supplied by the surrounding translation.

pub static mut memstart_addr: i64 = -1;

pub static mut arm64_dma_phys_limit: phys_addr_t = 0;

// CONFIG_ARM64_4K_PAGES => PUD_SHIFT; CONFIG_ARM64_16K_PAGES => CONT_PMD_SHIFT;
// otherwise PMD_SHIFT.
#[cfg(CONFIG_ARM64_4K_PAGES)]
const ARM64_MEMSTART_SHIFT: usize = PUD_SHIFT;
#[cfg(CONFIG_ARM64_16K_PAGES)]
const ARM64_MEMSTART_SHIFT: usize = CONT_PMD_SHIFT;
#[cfg(not(any(CONFIG_ARM64_4K_PAGES, CONFIG_ARM64_16K_PAGES)))]
const ARM64_MEMSTART_SHIFT: usize = PMD_SHIFT;

// This preserves the source's build-time alignment condition.
#[cfg(any())]
const ARM64_MEMSTART_ALIGN: u64 = 1u64 << SECTION_SIZE_BITS;
#[cfg(not(any()))]
const ARM64_MEMSTART_ALIGN: u64 = 1u64 << ARM64_MEMSTART_SHIFT;

unsafe fn arch_reserve_crashkernel() {
    let mut crash_base: u64 = 0;
    let mut crash_size: u64 = 0;
    let mut cma_size: u64 = 0;
    let mut low_size: u64 = 0;
    let mut high = false;
    let ret: i32;

    if !IS_ENABLED(CONFIG_CRASH_RESERVE) { return; }
    ret = parse_crashkernel(boot_command_line, memblock_phys_mem_size(),
                            &mut crash_size, &mut crash_base,
                            &mut low_size, &mut cma_size, &mut high);
    if ret != 0 { return; }
    reserve_crashkernel_generic(crash_size, crash_base, low_size, high);
    reserve_crashkernel_cma(cma_size);
}

unsafe fn max_zone_phys(zone_limit: phys_addr_t) -> phys_addr_t {
    min(zone_limit, memblock_end_of_DRAM() - 1) + 1
}

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut unsigned_long) {
    let dma32_phys_limit = max_zone_phys(DMA_BIT_MASK(32));
    #[cfg(CONFIG_ZONE_DMA)]
    *max_zone_pfns.add(ZONE_DMA) = PFN_DOWN(max_zone_phys(zone_dma_limit));
    #[cfg(CONFIG_ZONE_DMA32)]
    *max_zone_pfns.add(ZONE_DMA32) = PFN_DOWN(dma32_phys_limit);
    *max_zone_pfns.add(ZONE_NORMAL) = max_pfn;
}

unsafe fn dma_limits_init() {
    let dma32_phys_limit = max_zone_phys(DMA_BIT_MASK(32));
    #[cfg(CONFIG_ZONE_DMA)]
    {
        let acpi_zone_dma_limit = acpi_iort_dma_get_max_cpu_address();
        let dt_zone_dma_limit = of_dma_get_max_cpu_address(core::ptr::null_mut());
        zone_dma_limit = min(dt_zone_dma_limit, acpi_zone_dma_limit);
        if memblock_start_of_DRAM() < U32_MAX { zone_dma_limit = min(zone_dma_limit, U32_MAX); }
        arm64_dma_phys_limit = max_zone_phys(zone_dma_limit);
    }
    #[cfg(CONFIG_ZONE_DMA32)]
    if arm64_dma_phys_limit == 0 { arm64_dma_phys_limit = dma32_phys_limit; }
    if arm64_dma_phys_limit == 0 { arm64_dma_phys_limit = PHYS_MASK + 1; }
}

pub unsafe fn pfn_is_map_memory(pfn: unsigned_long) -> i32 {
    let addr = PFN_PHYS(pfn);
    if PHYS_PFN(addr) != pfn { return 0; }
    memblock_is_map_memory(addr)
}

static mut memory_limit: phys_addr_t = PHYS_ADDR_MAX;

unsafe fn early_mem(mut p: *mut i8) -> i32 {
    if p.is_null() { return 1; }
    memory_limit = memparse(p, &mut p) & PAGE_MASK;
    pr_notice!("Memory limited to %lldMB\n", memory_limit >> 20);
    0
}

unsafe fn arm64_memblock_init() {
    let mut linear_region_size = PAGE_END - _PAGE_OFFSET(vabits_actual);
    if IS_ENABLED(CONFIG_KVM) && vabits_actual == 52 && is_hyp_mode_available() && !is_kernel_in_hyp_mode() {
        pr_info!("Capping linear region to 51 bits for KVM in nVHE mode on LVA capable hardware.\n");
        linear_region_size = min_t(u64, linear_region_size, BIT(51));
    }
    memblock_remove(1u64 << PHYS_MASK_SHIFT, ULLONG_MAX);
    memstart_addr = round_down(memblock_start_of_DRAM(), ARM64_MEMSTART_ALIGN);
    if memblock_end_of_DRAM() - memstart_addr > linear_region_size {
        pr_warn!("Memory doesn't fit in the linear mapping, VA_BITS too small\n");
    }
    memblock_remove(max_t(u64, memstart_addr + linear_region_size, __pa_symbol(_end)), ULLONG_MAX);
    if memstart_addr + linear_region_size < memblock_end_of_DRAM() {
        memstart_addr = round_up(memblock_end_of_DRAM() - linear_region_size, ARM64_MEMSTART_ALIGN);
        memblock_remove(0, memstart_addr);
    }
    if IS_ENABLED(CONFIG_ARM64_VA_BITS_52) && vabits_actual != 52 {
        memstart_addr -= _PAGE_OFFSET(vabits_actual) - _PAGE_OFFSET(52);
    }
    if memory_limit != PHYS_ADDR_MAX {
        memblock_mem_limit_remove_map(memory_limit);
        memblock_add(__pa_symbol(_text), (_end as u64) - (_text as u64));
    }
    if IS_ENABLED(CONFIG_BLK_DEV_INITRD) && phys_initrd_size != 0 {
        let base = phys_initrd_start & PAGE_MASK;
        let size = PAGE_ALIGN(phys_initrd_start + phys_initrd_size) - base;
        if WARN!(base < memblock_start_of_DRAM() || base + size > memblock_start_of_DRAM() + linear_region_size,
                 "initrd not fully accessible via the linear mapping -- please check your bootloader ...\n") {
            phys_initrd_size = 0;
        } else {
            memblock_add(base, size); memblock_clear_nomap(base, size); memblock_reserve(base, size);
        }
    }
    memblock_reserve(__pa_symbol(_text), (_end as u64) - (_text as u64));
    if IS_ENABLED(CONFIG_BLK_DEV_INITRD) && phys_initrd_size != 0 {
        initrd_start = __phys_to_virt(phys_initrd_start);
        initrd_end = initrd_start + phys_initrd_size;
    }
    early_init_fdt_scan_reserved_mem();
}

pub unsafe fn bootmem_init() {
    let min = PFN_UP(memblock_start_of_DRAM());
    let max = PFN_DOWN(memblock_end_of_DRAM());
    early_memtest(min << PAGE_SHIFT, max << PAGE_SHIFT);
    max_pfn = max_low_pfn = max; min_low_pfn = min;
    arch_numa_init(); kvm_hyp_reserve(); dma_limits_init();
    dma_contiguous_reserve(arm64_dma_phys_limit);
    arch_reserve_crashkernel();
    memblock_dump_all();
}

pub unsafe fn arch_setup_zero_pages() { __zero_page = phys_to_page(__pa_symbol(empty_zero_page)); }

pub unsafe fn arch_mm_preinit() {
    let flags = SWIOTLB_VERBOSE;
    if max_pfn <= PFN_DOWN(arm64_dma_phys_limit) {
        let size = DIV_ROUND_UP(memblock_phys_mem_size(), 1024);
        swiotlb_adjust_size(min(swiotlb_size_or_default(), size));
    }
    swiotlb_init(true, flags);
    // BUILD_BUG_ON checks preserve CONFIG_COMPAT and page-table-level invariants.
    if PAGE_SIZE >= 16384 && get_num_physpages() <= 128 {
        sysctl_overcommit_memory = OVERCOMMIT_ALWAYS;
    }
}

pub static mut page_alloc_available: bool = false;
pub unsafe fn mem_init() { page_alloc_available = true; swiotlb_update_mem_attributes(); }

pub unsafe fn free_initmem() {
    let lm_init_begin = lm_alias(__init_begin);
    let lm_init_end = lm_alias(__init_end);
    WARN_ON!(!IS_ALIGNED(lm_init_begin as unsigned_long, PAGE_SIZE));
    WARN_ON!(!IS_ALIGNED(lm_init_end as unsigned_long, PAGE_SIZE));
    free_reserved_area(lm_init_begin, lm_init_end, POISON_FREE_INITMEM, "unused kernel");
    // Unmap the __init region but leave the VM area in place.
    vunmap_range(__init_begin as u64, __init_end as u64);
}

pub unsafe fn dump_mem_limit() {
    if memory_limit != PHYS_ADDR_MAX { pr_emerg!("Memory Limit: %llu MB\n", memory_limit >> 20); }
    else { pr_emerg!("Memory Limit: none\n"); }
}

pub unsafe fn cc_platform_has(attr: enum_cc_attr) -> bool {
    match attr {
        CC_ATTR_MEM_ENCRYPT | CC_ATTR_GUEST_MEM_ENCRYPT => is_realm_world() || is_protected_kvm_guest(),
        _ => false,
    }
}

#[cfg(CONFIG_EXECMEM)]
static mut module_direct_base: u64 = 0;
#[cfg(CONFIG_EXECMEM)]
static mut module_plt_base: u64 = 0;

#[cfg(CONFIG_EXECMEM)]
unsafe fn random_bounding_box(size: u64, start: u64, end: u64) -> u64 {
    if end - start >= size { return 0; }
    let max_pgoff = (size - (end - start)) / PAGE_SIZE;
    let pgoff = get_random_u32_inclusive(0, max_pgoff);
    start - pgoff * PAGE_SIZE
}

#[cfg(CONFIG_EXECMEM)]
unsafe fn module_init_limits() -> i32 {
    let kernel_end = _end as u64; let kernel_start = _text as u64; let kernel_size = kernel_end - kernel_start;
    if !kaslr_enabled() {
        if kernel_size < SZ_128M { module_direct_base = kernel_end - SZ_128M; }
        if kernel_size < SZ_2G { module_plt_base = kernel_end - SZ_2G; }
    } else {
        let mut min = kernel_start; let mut max = kernel_end;
        if IS_ENABLED(CONFIG_RANDOMIZE_MODULE_REGION_FULL) { pr_info!("2G module region forced by RANDOMIZE_MODULE_REGION_FULL\n"); }
        else { module_direct_base = random_bounding_box(SZ_128M, min, max); if module_direct_base != 0 { min = module_direct_base; max = module_direct_base + SZ_128M; } }
        module_plt_base = random_bounding_box(SZ_2G, min, max);
    }
    pr_info!("%llu pages in range for non-PLT usage", if module_direct_base != 0 { (SZ_128M - kernel_size) / PAGE_SIZE } else { 0 });
    pr_info!("%llu pages in range for PLT usage", if module_plt_base != 0 { (SZ_2G - kernel_size) / PAGE_SIZE } else { 0 });
    0
}

#[cfg(CONFIG_EXECMEM)]
static mut execmem_info: execmem_info = unsafe { core::mem::zeroed() };

#[cfg(CONFIG_EXECMEM)]
pub unsafe fn execmem_arch_setup() -> *mut execmem_info {
    let mut fallback_start = 0; let mut fallback_end = 0; let mut start = 0; let mut end = 0;
    module_init_limits();
    if module_direct_base != 0 { start = module_direct_base; end = module_direct_base + SZ_128M; if module_plt_base != 0 { fallback_start = module_plt_base; fallback_end = module_plt_base + SZ_2G; } }
    else if module_plt_base != 0 { start = module_plt_base; end = module_plt_base + SZ_2G; }
    execmem_info = execmem_info { ranges: [
        execmem_range { start, end, pgprot: PAGE_KERNEL, alignment: 1, fallback_start, fallback_end },
        execmem_range { start: VMALLOC_START, end: VMALLOC_END, pgprot: PAGE_KERNEL_ROX, alignment: 1, fallback_start: 0, fallback_end: 0 },
        execmem_range { start: VMALLOC_START, end: VMALLOC_END, pgprot: PAGE_KERNEL, alignment: 1, fallback_start: 0, fallback_end: 0 },
    ]};
    &mut execmem_info
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
