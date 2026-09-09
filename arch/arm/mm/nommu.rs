// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/nommu.c
 *
 * ARM uCLinux supporting functions.
 */

// Declarations supplied by the Linux kernel and ARM architecture headers.

pub static mut vectors_base: ::core::ffi::c_ulong = 0;

#[cfg(feature = "CONFIG_ARM_MPU")]
pub static mut mpu_rgn_info: mpu_rgn_info = unsafe { core::mem::zeroed() };

#[cfg(all(feature = "CONFIG_CPU_CP15", feature = "CONFIG_CPU_HIGH_VECTOR"))]
pub unsafe fn setup_vectors_base() -> ::core::ffi::c_ulong {
    let reg = get_cr();
    set_cr(reg | CR_V);
    0xffff0000
}

#[cfg(all(feature = "CONFIG_CPU_CP15", not(feature = "CONFIG_CPU_HIGH_VECTOR")))]
#[inline]
unsafe fn set_vbar(val: ::core::ffi::c_ulong) {
    // Write exception base address to VBAR.
    core::arch::asm!("mcr p15, 0, {0}, c12, c0, 0", in(reg) val, options(nostack));
}

#[cfg(all(feature = "CONFIG_CPU_CP15", not(feature = "CONFIG_CPU_HIGH_VECTOR")))]
#[inline]
unsafe fn security_extensions_enabled() -> bool {
    // Check CPUID Identification Scheme before ID_PFR1 read.
    if (read_cpuid_id() & 0x000f0000) == 0x000f0000 {
        return cpuid_feature_extract(CPUID_EXT_PFR1, 4) != 0
            || cpuid_feature_extract(CPUID_EXT_PFR1, 20) != 0;
    }
    false
}

#[cfg(all(feature = "CONFIG_CPU_CP15", not(feature = "CONFIG_CPU_HIGH_VECTOR")))]
pub unsafe fn setup_vectors_base() -> ::core::ffi::c_ulong {
    let mut base: ::core::ffi::c_ulong = 0;
    let reg = get_cr();
    set_cr(reg & !CR_V);
    if security_extensions_enabled() {
        if cfg!(feature = "CONFIG_REMAP_VECTORS_TO_RAM") {
            base = CONFIG_DRAM_BASE;
        }
        set_vbar(base);
    } else if cfg!(feature = "CONFIG_REMAP_VECTORS_TO_RAM") {
        if CONFIG_DRAM_BASE != 0 {
            pr_err("Security extensions not enabled, vectors cannot be remapped to RAM, vectors base will be 0x00000000\n");
        }
    }
    base
}

pub unsafe fn arm_mm_memblock_reserve() {
    #[cfg(not(feature = "CONFIG_CPU_V7M"))]
    {
        vectors_base = if cfg!(feature = "CONFIG_CPU_CP15") {
            setup_vectors_base()
        } else {
            0
        };
        // Register the exception vector page.
        // Some architectures use DRAM as the exception vector to trap;
        // alloc_page breaks with error, although it is not NULL, but "0."
        memblock_reserve(vectors_base, 2 * PAGE_SIZE);
    }
    #[cfg(feature = "CONFIG_CPU_V7M")]
    {
        // There is no dedicated vector page on V7-M. So nothing needs to be
        // reserved here.
    }
    // Always ensure address 0 is never used as a legitimate address.
    memblock_reserve(0, 1);
}

unsafe fn adjust_lowmem_bounds_mpu() {
    let pmsa = read_cpuid_ext(CPUID_EXT_MMFR0) & MMFR0_PMSA;
    match pmsa {
        MMFR0_PMSAv7 => pmsav7_adjust_lowmem_bounds(),
        MMFR0_PMSAv8 => pmsav8_adjust_lowmem_bounds(),
        _ => {}
    }
}

unsafe fn mpu_setup() {
    let pmsa = read_cpuid_ext(CPUID_EXT_MMFR0) & MMFR0_PMSA;
    match pmsa {
        MMFR0_PMSAv7 => pmsav7_setup(),
        MMFR0_PMSAv8 => pmsav8_setup(),
        _ => {}
    }
}

pub unsafe fn adjust_lowmem_bounds() {
    adjust_lowmem_bounds_mpu();
    let end = memblock_end_of_DRAM();
    high_memory = (__va(end - 1) as usize + 1) as _;
    memblock_set_current_limit(end);
}

// paging_init() sets up the page tables, initialises the zone memory maps,
// and sets up the zero page, bad page and bad page tables.
pub unsafe fn paging_init(_mdesc: *const machine_desc) {
    early_trap_init(vectors_base as *mut core::ffi::c_void);
    mpu_setup();
    bootmem_init();
}

// We don't need to do anything here for nommu machines.
pub unsafe fn setup_mm_for_reboot() {}

pub unsafe fn flush_dcache_folio(folio: *mut folio) {
    __cpuc_flush_dcache_area(folio_address(folio), folio_size(folio));
}

pub unsafe fn flush_dcache_page(page: *mut page) {
    __cpuc_flush_dcache_area(page_address(page), PAGE_SIZE);
}

pub unsafe fn copy_to_user_page(
    vma: *mut vm_area_struct,
    _page: *mut page,
    uaddr: ::core::ffi::c_ulong,
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: usize,
) {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
    if (*vma).vm_flags & VM_EXEC != 0 {
        __cpuc_coherent_user_range(uaddr, uaddr + len as _);
    }
}

pub unsafe fn __arm_ioremap_pfn(
    pfn: ::core::ffi::c_ulong,
    offset: ::core::ffi::c_ulong,
    _size: usize,
    _mtype: u32,
) -> *mut core::ffi::c_void {
    if pfn >= (0x100000000u64 >> PAGE_SHIFT) as _ {
        return core::ptr::null_mut();
    }
    (offset + (pfn << PAGE_SHIFT)) as *mut core::ffi::c_void
}

pub unsafe fn __arm_ioremap_caller(
    phys_addr: phys_addr_t,
    _size: usize,
    _mtype: u32,
    _caller: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    phys_addr as *mut core::ffi::c_void
}

pub static mut arch_ioremap_caller: Option<unsafe fn(phys_addr_t, usize, u32, *mut core::ffi::c_void) -> *mut core::ffi::c_void> = None;

pub unsafe fn ioremap(res_cookie: resource_size_t, size: usize) -> *mut core::ffi::c_void {
    __arm_ioremap_caller(res_cookie, size, MT_DEVICE, core::ptr::null_mut())
}

pub unsafe fn ioremap_cache(res_cookie: resource_size_t, size: usize) -> *mut core::ffi::c_void {
    __arm_ioremap_caller(res_cookie, size, MT_DEVICE_CACHED, core::ptr::null_mut())
}

pub unsafe fn ioremap_wc(res_cookie: resource_size_t, size: usize) -> *mut core::ffi::c_void {
    __arm_ioremap_caller(res_cookie, size, MT_DEVICE_WC, core::ptr::null_mut())
}

#[cfg(feature = "CONFIG_PCI")]
pub unsafe fn pci_remap_cfgspace(res_cookie: resource_size_t, size: usize) -> *mut core::ffi::c_void {
    (arch_ioremap_caller.unwrap())(res_cookie, size, MT_UNCACHED, core::ptr::null_mut())
}

pub unsafe fn arch_memremap_wb(
    phys_addr: phys_addr_t,
    _size: usize,
    _flags: ::core::ffi::c_ulong,
) -> *mut core::ffi::c_void {
    phys_addr as *mut core::ffi::c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
