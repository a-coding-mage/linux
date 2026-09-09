/*
 * fixmap.h: compile-time virtual memory allocation
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998 Ingo Molnar
 *
 * Support of BIGMEM added by Gerhard Wichert, Siemens AG, July 1999
 * x86_32 and x86_64 integration by Gustavo F. Padovan, February 2009
 */

// Dependency: asm/kmap_size.h

/* Exposed to assembly code for setting up initial page tables. */
#[cfg(not(feature = "CONFIG_DEBUG_KMAP_LOCAL_FORCE_MAP"))]
pub const FIXMAP_PMD_NUM: usize = 2;
#[cfg(feature = "CONFIG_DEBUG_KMAP_LOCAL_FORCE_MAP")]
pub const FIXMAP_PMD_NUM: usize = KM_PMDS + 2;
#[cfg(feature = "CONFIG_DEBUG_KMAP_LOCAL_FORCE_MAP")]
pub const KM_PMDS: usize = KM_MAX_IDX * ((CONFIG_NR_CPUS + 511) / 512);

/* fixmap starts downwards from the 507th entry in level2_fixmap_pgt */
pub const FIXMAP_PMD_TOP: usize = 507;

/*
 * Here we define all the compile-time 'special' virtual addresses. The point
 * is to have a constant address at compile time, but to set the physical
 * address only in the boot process.
 *
 * Build-time configuration branches from the C header are represented with
 * Rust cfg feature tests.
 */
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FixedAddresses {
    #[cfg(feature = "CONFIG_X86_32")]
    FIX_HOLE,
    #[cfg(all(not(feature = "CONFIG_X86_32"), feature = "CONFIG_X86_VSYSCALL_EMULATION"))]
    VSYSCALL_PAGE = ((FIXADDR_TOP - VSYSCALL_ADDR) >> PAGE_SHIFT) as u32,

    FIX_DBGP_BASE,
    FIX_EARLYCON_MEM_BASE,
    #[cfg(feature = "CONFIG_PROVIDE_OHCI1394_DMA_INIT")]
    FIX_OHCI1394_BASE,
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
    FIX_APIC_BASE, /* local (CPU) APIC) -- required for SMP or not */
    #[cfg(feature = "CONFIG_X86_IO_APIC")]
    FIX_IO_APIC_BASE_0,
    #[cfg(feature = "CONFIG_X86_IO_APIC")]
    FIX_IO_APIC_BASE_END = FIX_IO_APIC_BASE_0 as u32 + MAX_IO_APICS as u32 - 1,
    #[cfg(feature = "CONFIG_KMAP_LOCAL")]
    FIX_KMAP_BEGIN, /* reserved pte's for temporary kernel mappings */
    #[cfg(feature = "CONFIG_KMAP_LOCAL")]
    FIX_KMAP_END = FIX_KMAP_BEGIN as u32 + (KM_MAX_IDX * NR_CPUS) as u32 - 1,
    #[cfg(all(feature = "CONFIG_KMAP_LOCAL", feature = "CONFIG_PCI_MMCONFIG"))]
    FIX_PCIE_MCFG,
    #[cfg(feature = "CONFIG_PARAVIRT_XXL")]
    FIX_PARAVIRT_BOOTMAP,
    #[cfg(feature = "CONFIG_ACPI_APEI_GHES")]
    FIX_APEI_GHES_IRQ,
    #[cfg(feature = "CONFIG_ACPI_APEI_GHES")]
    FIX_APEI_GHES_NMI,

    __end_of_permanent_fixed_addresses,

    /* 512 temporary boot-time mappings, used by early_ioremap(). */
    FIX_BTMAP_END = (((__end_of_permanent_fixed_addresses as u32
        ^ (__end_of_permanent_fixed_addresses as u32 + TOTAL_FIX_BTMAPS - 1))
        & (!PTRS_PER_PTE as u32)) != 0)
        as u32,
    FIX_BTMAP_BEGIN = FIX_BTMAP_END as u32 + TOTAL_FIX_BTMAPS - 1,
    #[cfg(feature = "CONFIG_X86_32")]
    FIX_WP_TEST,
    #[cfg(feature = "CONFIG_INTEL_TXT")]
    FIX_TBOOT_BASE,
    __end_of_fixed_addresses,
}

pub const NR_FIX_BTMAPS: u32 = 64;
pub const FIX_BTMAPS_SLOTS: u32 = 8;
pub const TOTAL_FIX_BTMAPS: u32 = NR_FIX_BTMAPS * FIX_BTMAPS_SLOTS;

extern "C" {
    pub static mut __FIXADDR_TOP: c_ulong;
    pub fn reserve_top_address(reserve: c_ulong);
    pub fn __native_set_fixmap(idx: FixedAddresses, pte: pte_t);
    pub fn native_set_fixmap(idx: c_uint, phys: phys_addr_t, flags: pgprot_t);
    pub fn early_memremap_encrypted(phys_addr: resource_size_t, size: c_ulong) -> *mut c_void;
    pub fn early_memremap_encrypted_wp(phys_addr: resource_size_t, size: c_ulong) -> *mut c_void;
    pub fn early_memremap_decrypted(phys_addr: resource_size_t, size: c_ulong) -> *mut c_void;
    pub fn early_memremap_decrypted_wp(phys_addr: resource_size_t, size: c_ulong) -> *mut c_void;
    pub fn __early_set_fixmap(idx: FixedAddresses, phys: phys_addr_t, flags: pgprot_t);
    pub static mut fixmaps_set: c_int;
    pub static mut pkmap_page_table: *mut pte_t;
}

pub const FIXADDR_SIZE: usize = (__end_of_permanent_fixed_addresses as usize) << PAGE_SHIFT;
pub const FIXADDR_START: usize = FIXADDR_TOP - FIXADDR_SIZE;
pub const FIXADDR_TOT_SIZE: usize = (__end_of_fixed_addresses as usize) << PAGE_SHIFT;
pub const FIXADDR_TOT_START: usize = FIXADDR_TOP - FIXADDR_TOT_SIZE;

#[cfg(not(feature = "CONFIG_PARAVIRT_XXL"))]
#[inline]
pub unsafe fn __set_fixmap(idx: FixedAddresses, phys: phys_addr_t, flags: pgprot_t) {
    native_set_fixmap(idx as c_uint, phys, flags);
}

pub const FIXMAP_PAGE_NOCACHE: pgprot_t = PAGE_KERNEL_IO_NOCACHE;

#[cfg(not(feature = "CONFIG_PARAVIRT_XXL"))]
#[inline]
pub unsafe fn __late_set_fixmap(idx: FixedAddresses, phys: phys_addr_t, flags: pgprot_t) {
    __set_fixmap(idx, phys, flags);
}

#[inline]
pub unsafe fn __late_clear_fixmap(idx: FixedAddresses) {
    __set_fixmap(idx, 0, __pgprot(0));
}

// Dependency: asm-generic/fixmap.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
