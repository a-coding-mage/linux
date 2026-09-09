/*
 * fixmap.h: compile-time virtual memory allocation
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998 Ingo Molnar
 * Copyright (C) 2013 Mark Salter <msalter@redhat.com>
 *
 * Adapted from arch/x86 version.
 */

/* C header dependencies are supplied by the surrounding translation unit. */

/*
 * Here we define all the compile-time 'special' virtual addresses. The point
 * is to have a constant address at compile time, but to set the physical
 * address only in the boot process.
 *
 * Each enum increment in these 'compile-time allocated' memory buffers is
 * page-sized. Use set_fixmap(idx, phys) to associate physical memory with a
 * fixmap index.
 */
#[repr(usize)]
pub enum FixedAddresses {
    FixHole,

    /* Reserve a virtual window for the FDT, one page bigger than its maximum. */
    FixFdtEnd,
    FixFdt = FixFdtEnd as usize + div_round_up(MAX_FDT_SIZE, PAGE_SIZE) + 1,

    FixEarlyconMemBase,
    FixTextPoke0,

    /* CONFIG_KVM: one slot per CPU, mapping the guest's VNCR page at EL2. */
    #[cfg(feature = "CONFIG_KVM")]
    FixVncrEnd,
    #[cfg(feature = "CONFIG_KVM")]
    FixVncr = FixVncrEnd as usize + NR_CPUS,

    /* CONFIG_ACPI_APEI_GHES: used for GHES mapping from assorted contexts. */
    #[cfg(feature = "CONFIG_ACPI_APEI_GHES")]
    FixApeiGhesIrq,
    #[cfg(feature = "CONFIG_ACPI_APEI_GHES")]
    FixApeiGhesSea,
    #[cfg(all(feature = "CONFIG_ACPI_APEI_GHES", feature = "CONFIG_ARM_SDE_INTERFACE"))]
    FixApeiGhesSdeiNormal,
    #[cfg(all(feature = "CONFIG_ACPI_APEI_GHES", feature = "CONFIG_ARM_SDE_INTERFACE"))]
    FixApeiGhesSdeiCritical,

    /* CONFIG_UNMAP_KERNEL_AT_EL0 / CONFIG_RELOCATABLE. */
    #[cfg(all(feature = "CONFIG_UNMAP_KERNEL_AT_EL0", feature = "CONFIG_RELOCATABLE"))]
    FixEntryTrampText4,
    #[cfg(feature = "CONFIG_UNMAP_KERNEL_AT_EL0")]
    FixEntryTrampText3,
    #[cfg(feature = "CONFIG_UNMAP_KERNEL_AT_EL0")]
    FixEntryTrampText2,
    #[cfg(feature = "CONFIG_UNMAP_KERNEL_AT_EL0")]
    FixEntryTrampText1,

    EndOfPermanentFixedAddresses,

    FixBtmpEnd = EndOfPermanentFixedAddresses as usize,
    FixBtmpBegin = FixBtmpEnd as usize + TOTAL_FIX_BTMAPS - 1,

    FixPte,
    FixPmd,
    FixPud,
    FixP4d,
    FixPgd,

    EndOfFixedAddresses,
}

pub const NR_FIX_BTMAPS: usize = (SZ_256K / PAGE_SIZE) + 1;
pub const FIX_BTMAPS_SLOTS: usize = 7;
pub const TOTAL_FIX_BTMAPS: usize = NR_FIX_BTMAPS * FIX_BTMAPS_SLOTS;

pub const FIXADDR_SIZE: usize = (FixedAddresses::EndOfPermanentFixedAddresses as usize) << PAGE_SHIFT;
pub const FIXADDR_START: usize = FIXADDR_TOP - FIXADDR_SIZE;
pub const FIXADDR_TOT_SIZE: usize = (FixedAddresses::EndOfFixedAddresses as usize) << PAGE_SHIFT;
pub const FIXADDR_TOT_START: usize = FIXADDR_TOP - FIXADDR_TOT_SIZE;

pub const FIXMAP_PAGE_IO: pgprot_t = __pgprot(PROT_DEVICE_nGnRE);

pub const TRAMP_VALIAS: usize = __fix_to_virt(FixedAddresses::FixEntryTrampText1 as usize);

extern "C" {
    pub fn early_fixmap_init();
    pub fn __set_fixmap(idx: FixedAddresses, phys: phys_addr_t, prot: pgprot_t);
}

/* __early_set_fixmap is the C macro alias for __set_fixmap. */
pub use __set_fixmap as __early_set_fixmap;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
