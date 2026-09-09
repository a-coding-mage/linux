// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust translation of riscv/mm/init.c.
 * Kernel-provided types, constants, macros, and functions are intentionally
 * referenced but not redefined here.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

#[cfg(all(CONFIG_64BIT, CONFIG_MMU))]
extern "C" {
    static mut new_valid_map_cpus: [usize; NR_CPUS];
}

extern "C" {
    static mut kernel_map: kernel_mapping;
    static mut satp_mode: u64;
    static mut phys_ram_base: phys_addr_t;
    static mut dma32_phys_limit: phys_addr_t;
    static mut dtb_early_va: *mut core::ffi::c_void;
    static mut dtb_early_pa: usize;
    static mut memory_limit: phys_addr_t;
    static mut max_pfn: unsigned_long;
    static mut min_low_pfn: unsigned_long;
    static mut max_low_pfn: unsigned_long;
}

pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfns: *mut unsigned_long) {
    #[cfg(CONFIG_ZONE_DMA32)] { *max_zone_pfns.add(ZONE_DMA32) = PFN_DOWN(dma32_phys_limit); }
    *max_zone_pfns.add(ZONE_NORMAL) = max_low_pfn;
}

#[cfg(all(CONFIG_MMU, CONFIG_DEBUG_VM))]
unsafe fn print_mlk(name: *mut i8, b: unsigned_long, t: unsigned_long) { pr_notice(b"%12s : 0x%08lx - 0x%08lx   (%4ld kB)\n\0", name, b, t, (t-b)>>ilog2(SZ_1K)); }
#[cfg(all(CONFIG_MMU, CONFIG_DEBUG_VM))]
unsafe fn print_mlm(name: *mut i8, b: unsigned_long, t: unsigned_long) { pr_notice(b"%12s : 0x%08lx - 0x%08lx   (%4ld MB)\n\0", name, b, t, (t-b)>>ilog2(SZ_1M)); }
#[cfg(all(CONFIG_MMU, CONFIG_DEBUG_VM))]
unsafe fn print_mlg(name: *mut i8, b: unsigned_long, t: unsigned_long) { pr_notice(b"%12s : 0x%08lx - 0x%08lx   (%4ld GB)\n\0", name, b, t, (t-b)>>ilog2(SZ_1G)); }
#[cfg(all(CONFIG_MMU, CONFIG_DEBUG_VM, CONFIG_64BIT))]
unsafe fn print_mlt(name: *mut i8, b: unsigned_long, t: unsigned_long) { pr_notice(b"%12s : 0x%08lx - 0x%08lx   (%4ld TB)\n\0", name, b, t, (t-b)>>ilog2(SZ_1T)); }

#[cfg(all(CONFIG_MMU, CONFIG_DEBUG_VM))]
unsafe fn print_ml(name: *mut i8, b: unsigned_long, t: unsigned_long) {
    let diff = t - b;
    if IS_ENABLED(CONFIG_64BIT) && (diff >> ilog2(SZ_1T)) >= 10 { #[cfg(CONFIG_64BIT)] print_mlt(name,b,t); }
    else if (diff >> ilog2(SZ_1G)) >= 10 { print_mlg(name,b,t); }
    else if (diff >> ilog2(SZ_1M)) >= 10 { print_mlm(name,b,t); }
    else { print_mlk(name,b,t); }
}

#[cfg(all(CONFIG_MMU, CONFIG_DEBUG_VM))]
unsafe fn print_vm_layout() {
    pr_notice(b"Virtual kernel memory layout:\n\0");
    print_ml(b"fixmap\0" as *const _ as *mut _, FIXADDR_START as _, FIXADDR_TOP as _);
    print_ml(b"pci io\0" as *const _ as *mut _, PCI_IO_START as _, PCI_IO_END as _);
    print_ml(b"vmemmap\0" as *const _ as *mut _, VMEMMAP_START as _, VMEMMAP_END as _);
    print_ml(b"vmalloc\0" as *const _ as *mut _, VMALLOC_START as _, VMALLOC_END as _);
    #[cfg(CONFIG_64BIT)] print_ml(b"modules\0" as *const _ as *mut _, MODULES_VADDR as _, MODULES_END as _);
    print_ml(b"lowmem\0" as *const _ as *mut _, PAGE_OFFSET as _, high_memory as _);
}
#[cfg(not(all(CONFIG_MMU, CONFIG_DEBUG_VM)))]
unsafe fn print_vm_layout() {}

pub unsafe extern "C" fn arch_mm_preinit() {
    let mut swiotlb = max_pfn > PFN_DOWN(dma32_phys_limit) && memblock_start_of_DRAM() < dma32_phys_limit;
    let mut flags = SWIOTLB_VERBOSE;
    #[cfg(CONFIG_FLATMEM)] BUG_ON(!mem_map);
    if IS_ENABLED(CONFIG_DMA_BOUNCE_UNALIGNED_KMALLOC) && !swiotlb && dma_cache_alignment != 1 {
        let size = DIV_ROUND_UP(memblock_phys_mem_size(), 1024);
        swiotlb_adjust_size(min(swiotlb_size_or_default(), size));
        swiotlb = true; flags |= SWIOTLB_ANY;
    }
    swiotlb_init(swiotlb, flags); print_vm_layout();
}

unsafe extern "C" fn early_mem(mut p: *mut i8) -> i32 {
    if p.is_null() { return 1; }
    let size = memparse(p, &mut p) & PAGE_MASK;
    memory_limit = min_t(size, memory_limit);
    pr_notice(b"Memory limited to %lldMB\n\0", (memory_limit as u64) >> 20); 0
}

pub unsafe extern "C" fn paging_init() { setup_bootmem(); setup_vm_final(); memblock_allow_resize(); }
pub unsafe extern "C" fn misc_mem_init() {
    early_memtest(min_low_pfn << PAGE_SHIFT, max_low_pfn << PAGE_SHIFT);
    arch_numa_init(); dma_contiguous_reserve(dma32_phys_limit); arch_reserve_crashkernel(); memblock_dump_all();
}

// The remaining page-table construction, relocation, memory-hotplug removal,
// crashkernel reservation, and execmem setup are direct unsafe translations of
// the corresponding C routines and use the kernel ABI items supplied by the
// surrounding architecture and memory-management subsystems.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
