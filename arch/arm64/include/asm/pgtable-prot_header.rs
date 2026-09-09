/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2016 ARM Ltd. */

/* Translated from pgtable-prot.h. Included C headers provide the referenced symbols. */

pub const PTE_WRITE: pteval_t = PTE_DBM;
pub const PTE_SWP_EXCLUSIVE: pteval_t = (1 as pteval_t) << 2;
pub const PTE_DIRTY: pteval_t = (1 as pteval_t) << 55;
pub const PTE_SPECIAL: pteval_t = (1 as pteval_t) << 56;
pub const PTE_PRESENT_INVALID: pteval_t = PTE_NG;
pub const PTE_PRESENT_VALID_KERNEL: pteval_t = PTE_VALID | PTE_MAYBE_NG;

#[cfg(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP")]
pub const PTE_UFFD: pteval_t = (1 as pteval_t) << 58;
#[cfg(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP")]
pub const PTE_SWP_UFFD: pteval_t = (1 as pteval_t) << 3;
#[cfg(not(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP"))]
pub const PTE_UFFD: pteval_t = 0 as pteval_t;
#[cfg(not(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP"))]
pub const PTE_SWP_UFFD: pteval_t = 0 as pteval_t;

pub const _PROT_DEFAULT: pteval_t = PTE_TYPE_PAGE | PTE_AF | PTE_SHARED;
pub const PROT_DEFAULT: pteval_t = PTE_TYPE_PAGE | PTE_MAYBE_NG | PTE_MAYBE_SHARED | PTE_AF;
pub const PROT_SECT_DEFAULT: pteval_t = PMD_TYPE_SECT | PMD_MAYBE_NG | PMD_MAYBE_SHARED | PMD_SECT_AF;
pub const PROT_DEVICE_nGnRnE: pteval_t = PROT_DEFAULT | PTE_PXN | PTE_UXN | PTE_WRITE | PTE_ATTRINDX!(MT_DEVICE_nGnRnE);
pub const PROT_DEVICE_nGnRE: pteval_t = PROT_DEFAULT | PTE_PXN | PTE_UXN | PTE_WRITE | PTE_ATTRINDX!(MT_DEVICE_nGnRE);
pub const PROT_NORMAL_NC: pteval_t = PROT_DEFAULT | PTE_PXN | PTE_UXN | PTE_WRITE | PTE_ATTRINDX!(MT_NORMAL_NC);
pub const PROT_NORMAL: pteval_t = PROT_DEFAULT | PTE_PXN | PTE_UXN | PTE_WRITE | PTE_ATTRINDX!(MT_NORMAL);
pub const PROT_NORMAL_TAGGED: pteval_t = PROT_DEFAULT | PTE_PXN | PTE_UXN | PTE_WRITE | PTE_ATTRINDX!(MT_NORMAL_TAGGED);
pub const PROT_SECT_DEVICE_nGnRE: pteval_t = PROT_SECT_DEFAULT | PMD_SECT_PXN | PMD_SECT_UXN | PMD_ATTRINDX!(MT_DEVICE_nGnRE);
pub const PROT_SECT_NORMAL: pteval_t = PROT_SECT_DEFAULT | PMD_SECT_PXN | PMD_SECT_UXN | PTE_WRITE | PMD_ATTRINDX!(MT_NORMAL);
pub const PROT_SECT_NORMAL_EXEC: pteval_t = PROT_SECT_DEFAULT | PMD_SECT_UXN | PMD_ATTRINDX!(MT_NORMAL);
pub const _PAGE_DEFAULT: pteval_t = _PROT_DEFAULT | PTE_ATTRINDX!(MT_NORMAL);
pub const _PAGE_KERNEL: pteval_t = PROT_NORMAL | PTE_DIRTY;
pub const _PAGE_KERNEL_RO: pteval_t = (PROT_NORMAL & !PTE_WRITE) | PTE_RDONLY | PTE_DIRTY;
pub const _PAGE_KERNEL_ROX: pteval_t = (PROT_NORMAL & !(PTE_WRITE | PTE_PXN)) | PTE_RDONLY | PTE_DIRTY;
pub const _PAGE_KERNEL_EXEC: pteval_t = (PROT_NORMAL & !PTE_PXN) | PTE_DIRTY;
pub const _PAGE_KERNEL_EXEC_CONT: pteval_t = (PROT_NORMAL & !PTE_PXN) | PTE_CONT | PTE_DIRTY;
pub const _PAGE_SHARED: pteval_t = _PAGE_DEFAULT | PTE_USER | PTE_RDONLY | PTE_NG | PTE_PXN | PTE_UXN | PTE_WRITE;
pub const _PAGE_SHARED_EXEC: pteval_t = _PAGE_DEFAULT | PTE_USER | PTE_RDONLY | PTE_NG | PTE_PXN | PTE_WRITE;
pub const _PAGE_READONLY: pteval_t = _PAGE_DEFAULT | PTE_USER | PTE_RDONLY | PTE_NG | PTE_PXN | PTE_UXN;
pub const _PAGE_READONLY_EXEC: pteval_t = _PAGE_DEFAULT | PTE_USER | PTE_RDONLY | PTE_NG | PTE_PXN;
pub const _PAGE_EXECONLY: pteval_t = _PAGE_DEFAULT | PTE_RDONLY | PTE_NG | PTE_PXN;

extern "C" {
    pub static mut arm64_use_ng_mappings: bool;
    pub static mut prot_ns_shared: usize;
}

#[inline]
pub unsafe fn PROT_NS_SHARED() -> pteval_t { if is_realm_world() { prot_ns_shared as pteval_t } else { 0 } }
#[inline]
pub unsafe fn PTE_MAYBE_NG() -> pteval_t { if arm64_use_ng_mappings { PTE_NG } else { 0 } }
#[inline]
pub unsafe fn PMD_MAYBE_NG() -> pteval_t { if arm64_use_ng_mappings { PMD_SECT_NG } else { 0 } }

#[cfg(not(feature = "CONFIG_ARM64_LPA2"))]
pub const PHYS_MASK_SHIFT: usize = CONFIG_ARM64_PA_BITS;
#[cfg(not(feature = "CONFIG_ARM64_LPA2"))]
pub const PTE_MAYBE_SHARED: pteval_t = PTE_SHARED;
#[cfg(not(feature = "CONFIG_ARM64_LPA2"))]
pub const PMD_MAYBE_SHARED: pteval_t = PMD_SECT_S;
#[cfg(feature = "CONFIG_ARM64_LPA2")]
pub unsafe fn lpa2_is_enabled() -> bool { read_tcr() & TCR_EL1_DS != 0 }
#[cfg(feature = "CONFIG_ARM64_LPA2")]
pub unsafe fn PTE_MAYBE_SHARED() -> pteval_t { if lpa2_is_enabled() { 0 } else { PTE_SHARED } }
#[cfg(feature = "CONFIG_ARM64_LPA2")]
pub unsafe fn PMD_MAYBE_SHARED() -> pteval_t { if lpa2_is_enabled() { 0 } else { PMD_SECT_S } }
#[cfg(feature = "CONFIG_ARM64_LPA2")]
pub unsafe fn PHYS_MASK_SHIFT() -> usize { if lpa2_is_enabled() { CONFIG_ARM64_PA_BITS } else { 48 } }

pub const PHYS_MASK: usize = (1usize << PHYS_MASK_SHIFT) - 1;
pub unsafe fn PTE_MAYBE_GP() -> pteval_t { if system_supports_bti_kernel() { PTE_GP } else { 0 } }
pub const _PAGE_GCS: pteval_t = _PAGE_DEFAULT | PTE_NG | PTE_UXN | PTE_WRITE | PTE_USER;
pub const _PAGE_GCS_RO: pteval_t = _PAGE_DEFAULT | PTE_NG | PTE_UXN | PTE_USER;

pub unsafe fn pte_pi_index(pte: pteval_t) -> pteval_t {
    ((pte & BIT!(PTE_PI_IDX_3)) >> (PTE_PI_IDX_3 - 3)) |
    ((pte & BIT!(PTE_PI_IDX_2)) >> (PTE_PI_IDX_2 - 2)) |
    ((pte & BIT!(PTE_PI_IDX_1)) >> (PTE_PI_IDX_1 - 1)) |
    ((pte & BIT!(PTE_PI_IDX_0)) >> (PTE_PI_IDX_0 - 0))
}

pub const PIE_E0: pteval_t =
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_GCS), PIE_GCS) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_GCS_RO), PIE_R) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_EXECONLY), PIE_X_O) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_READONLY_EXEC), PIE_RX_O) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_SHARED_EXEC), PIE_RWX_O) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_READONLY), PIE_R_O) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_SHARED), PIE_RW_O);
pub const PIE_E1: pteval_t =
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_GCS), PIE_NONE_O) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_GCS_RO), PIE_NONE_O) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_EXECONLY), PIE_NONE_O) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_READONLY_EXEC), PIE_R) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_SHARED_EXEC), PIE_RW) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_READONLY), PIE_R) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_SHARED), PIE_RW) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_KERNEL_ROX), PIE_RX) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_KERNEL_EXEC), PIE_RWX) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_KERNEL_RO), PIE_R) |
    PIRx_ELx_PERM_PREP!(pte_pi_index(_PAGE_KERNEL), PIE_RW);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
