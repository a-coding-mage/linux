// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of linux/arch/arm/mm/init.c. */

#[cfg(feature = "cpu_cp15_mmu")]
pub unsafe fn __clear_cr(mask: c_ulong) -> c_ulong {
    cr_alignment &= !mask;
    cr_alignment
}

#[cfg(feature = "blk_dev_initrd")]
unsafe fn parse_tag_initrd(tag: *const tag) -> c_int {
    pr_warn!("ATAG_INITRD is deprecated; please update your bootloader.\n");
    phys_initrd_start = __virt_to_phys((*tag).u.initrd.start);
    phys_initrd_size = (*tag).u.initrd.size;
    0
}

#[cfg(feature = "blk_dev_initrd")]
unsafe fn parse_tag_initrd2(tag: *const tag) -> c_int {
    phys_initrd_start = (*tag).u.initrd.start;
    phys_initrd_size = (*tag).u.initrd.size;
    0
}

unsafe fn find_limits(min: *mut c_ulong, max_low: *mut c_ulong, max_high: *mut c_ulong) {
    *max_low = PFN_DOWN(memblock_get_current_limit());
    *min = PFN_UP(memblock_start_of_DRAM());
    *max_high = PFN_DOWN(memblock_end_of_DRAM());
}

#[cfg(feature = "zone_dma")]
#[no_mangle]
pub static mut arm_dma_zone_size: phys_addr_t = 0;
#[cfg(feature = "zone_dma")]
#[no_mangle]
pub static mut arm_dma_limit: phys_addr_t = 0;
#[cfg(feature = "zone_dma")]
#[no_mangle]
pub static mut arm_dma_pfn_limit: c_ulong = 0;

pub unsafe fn setup_dma_zone(mdesc: *const machine_desc) {
    #[cfg(feature = "zone_dma")]
    {
        if (*mdesc).dma_zone_size != 0 {
            arm_dma_zone_size = (*mdesc).dma_zone_size;
            arm_dma_limit = PHYS_OFFSET + arm_dma_zone_size - 1;
        } else {
            arm_dma_limit = 0xffff_ffff;
        }
        arm_dma_pfn_limit = arm_dma_limit >> PAGE_SHIFT;
    }
}

pub unsafe fn arch_zone_limits_init(max_zone_pfn: *mut c_ulong) {
    #[cfg(feature = "zone_dma")]
    { *max_zone_pfn.add(ZONE_DMA) = core::cmp::min(arm_dma_pfn_limit, max_low_pfn); }
    *max_zone_pfn.add(ZONE_NORMAL) = max_low_pfn;
    #[cfg(feature = "highmem")]
    { *max_zone_pfn.add(ZONE_HIGHMEM) = max_pfn; }
}

#[cfg(feature = "have_arch_pfn_valid")]
pub unsafe fn pfn_valid(pfn: c_ulong) -> c_int {
    let addr = __pfn_to_phys(pfn);
    let pageblock_size = PAGE_SIZE * pageblock_nr_pages;
    if __phys_to_pfn(addr) != pfn { return 0; }
    if memblock_overlaps_region(&memblock.memory, ALIGN_DOWN(addr, pageblock_size), pageblock_size) { 1 } else { 0 }
}

static mut arm_memblock_steal_permitted: bool = true;

pub unsafe fn arm_memblock_steal(size: phys_addr_t, align: phys_addr_t) -> phys_addr_t {
    BUG_ON!(!arm_memblock_steal_permitted);
    let phys = memblock_phys_alloc(size, align);
    if phys == 0 { panic!("Failed to steal memory"); }
    memblock_phys_free(phys, size);
    memblock_remove(phys, size);
    phys
}

#[cfg(feature = "cpu_icache_mismatch_workaround")]
pub unsafe fn check_cpu_icache_size(cpuid: c_int) {
    let ctr: u32;
    core::arch::asm!("mrc p15, 0, {0}, c0, c0, 1", out(reg) ctr);
    let size = 1u32 << ((ctr & 0xf) + 2);
    if cpuid != 0 && icache_size != size { pr_info!("CPU{}: detected I-Cache line size mismatch, workaround enabled\n", cpuid); }
    if icache_size > size { icache_size = size; }
}

pub unsafe fn arm_memblock_init(mdesc: *const machine_desc) {
    memblock_reserve(__pa(KERNEL_START), KERNEL_END - KERNEL_START);
    reserve_initrd_mem();
    arm_mm_memblock_reserve();
    if !(*mdesc).reserve.is_null() { ((*mdesc).reserve)(); }
    early_init_fdt_scan_reserved_mem();
    dma_contiguous_reserve(arm_dma_limit);
    arm_memblock_steal_permitted = false;
    memblock_dump_all();
}

pub unsafe fn bootmem_init() {
    memblock_allow_resize();
    find_limits(&mut min_low_pfn, &mut max_low_pfn, &mut max_pfn);
    early_memtest((min_low_pfn as phys_addr_t) << PAGE_SHIFT, (max_low_pfn as phys_addr_t) << PAGE_SHIFT);
}

unsafe fn poison_init_mem(s: *mut c_void, mut count: usize) {
    let mut p = s as *mut u32;
    while count != 0 { *p = 0xe7fd_def0; p = p.add(1); count -= 4; }
}

pub unsafe fn arch_mm_preinit() {
    #[cfg(feature = "arm_lpae")]
    swiotlb_init(max_pfn > arm_dma_pfn_limit, SWIOTLB_VERBOSE);
    #[cfg(feature = "sa1111")]
    memblock_phys_free(PHYS_OFFSET, __pa(swapper_pg_dir) - PHYS_OFFSET);
    #[cfg(feature = "mmu")]
    { BUILD_BUG_ON!(TASK_SIZE > MODULES_VADDR); BUG_ON!(TASK_SIZE > MODULES_VADDR); }
    #[cfg(feature = "highmem")]
    { BUILD_BUG_ON!(PKMAP_BASE + LAST_PKMAP * PAGE_SIZE > PAGE_OFFSET); BUG_ON!(PKMAP_BASE + LAST_PKMAP * PAGE_SIZE > PAGE_OFFSET); }
}

#[cfg(not(feature = "strict_kernel_rwx"))]
unsafe fn fix_kernmem_perms() {}

pub unsafe fn free_initmem() {
    fix_kernmem_perms();
    poison_init_mem(__init_begin as *mut c_void, __init_end as usize - __init_begin as usize);
    if !machine_is_integrator() && !machine_is_cintegrator() { free_initmem_default(-1); }
}

#[cfg(feature = "blk_dev_initrd")]
pub unsafe fn free_initrd_mem(mut start: c_ulong, mut end: c_ulong) {
    if start == initrd_start { start = round_down(start, PAGE_SIZE); }
    if end == initrd_end { end = round_up(end, PAGE_SIZE); }
    poison_init_mem(start as *mut c_void, PAGE_ALIGN(end) - start);
    free_reserved_area(start as *mut c_void, end as *mut c_void, -1, "initrd\0".as_ptr() as *const _);
}

#[cfg(all(feature = "execmem", feature = "mmu"))]
static mut execmem_info: execmem_info = execmem_info { ..unsafe { core::mem::zeroed() } };

#[cfg(all(feature = "execmem", feature = "mmu"))]
pub unsafe fn execmem_arch_setup() -> *mut execmem_info {
    let (mut fallback_start, mut fallback_end) = (0, 0);
    if cfg!(feature = "arm_module_plts") { fallback_start = VMALLOC_START; fallback_end = VMALLOC_END; }
    execmem_info.ranges[EXECMEM_DEFAULT] = execmem_range { start: MODULES_VADDR, end: MODULES_END, pgprot: PAGE_KERNEL_EXEC, alignment: 1, fallback_start, fallback_end };
    &mut execmem_info
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
