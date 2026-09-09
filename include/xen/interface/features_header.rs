/* SPDX-License-Identifier: MIT */
/*****************************************************************************
 * features.h
 *
 * Feature flags, reported by XENVER_get_features.
 *
 * Copyright (c) 2006, Keir Fraser <keir@xensource.com>
 */

/*
 * If set, the guest does not need to write-protect its pagetables, and can
 * update them via direct writes.
 */
pub const XENFEAT_writable_page_tables: u32 = 0;

/*
 * If set, the guest does not need to write-protect its segment descriptor
 * tables, and can update them via direct writes.
 */
pub const XENFEAT_writable_descriptor_tables: u32 = 1;

/*
 * If set, translation between the guest's 'pseudo-physical' address space
 * and the host's machine address space are handled by the hypervisor. In this
 * mode the guest does not need to perform phys-to/from-machine translations
 * when performing page table operations.
 */
pub const XENFEAT_auto_translated_physmap: u32 = 2;

/* If set, the guest is running in supervisor mode (e.g., x86 ring 0). */
pub const XENFEAT_supervisor_mode_kernel: u32 = 3;

/*
 * If set, the guest does not need to allocate x86 PAE page directories
 * below 4GB. This flag is usually implied by auto_translated_physmap.
 */
pub const XENFEAT_pae_pgdir_above_4gb: u32 = 4;

/* x86: Does this Xen host support the MMU_PT_UPDATE_PRESERVE_AD hypercall? */
pub const XENFEAT_mmu_pt_update_preserve_ad: u32 = 5;

/* x86: Does this Xen host support the MMU_{CLEAR,COPY}_PAGE hypercall? */
pub const XENFEAT_highmem_assist: u32 = 6;

/*
 * If set, GNTTABOP_map_grant_ref honors flags to be placed into guest kernel
 * available pte bits.
 */
pub const XENFEAT_gnttab_map_avail_bits: u32 = 7;

/* x86: Does this Xen host support the HVM callback vector type? */
pub const XENFEAT_hvm_callback_vector: u32 = 8;

/* x86: pvclock algorithm is safe to use on HVM */
pub const XENFEAT_hvm_safe_pvclock: u32 = 9;

/* x86: pirq can be used by HVM guests */
pub const XENFEAT_hvm_pirqs: u32 = 10;

/* operation as Dom0 is supported */
pub const XENFEAT_dom0: u32 = 11;

/* Xen also maps grant references at pfn = mfn.
 * This feature flag is deprecated and should not be used.
 *
 * #define XENFEAT_grant_map_identity 12
 */

/* Guest can use XENMEMF_vnode to specify virtual node for memory op. */
pub const XENFEAT_memory_op_vnode_supported: u32 = 13;

/* arm: Hypervisor supports ARM SMC calling convention. */
pub const XENFEAT_ARM_SMCCC_supported: u32 = 14;

/*
 * x86/PVH: If set, ACPI RSDP can be placed at any address. Otherwise RSDP
 * must be located in lower 1MB, as required by ACPI Specification for IA-PC
 * systems.
 * This feature flag is only consulted if XEN_ELFNOTE_GUEST_OS contains
 * the "linux" string.
 */
pub const XENFEAT_linux_rsdp_unrestricted: u32 = 15;

/*
 * A direct-mapped (or 1:1 mapped) domain is a domain for which its
 * local pages have gfn == mfn. If a domain is direct-mapped,
 * XENFEAT_direct_mapped is set; otherwise XENFEAT_not_direct_mapped
 * is set.
 *
 * If neither flag is set (e.g. older Xen releases) the assumptions are:
 * - not auto_translated domains (x86 only) are always direct-mapped
 * - on x86, auto_translated domains are not direct-mapped
 * - on ARM, Dom0 is direct-mapped, DomUs are not
 */
pub const XENFEAT_not_direct_mapped: u32 = 16;
pub const XENFEAT_direct_mapped: u32 = 17;

pub const XENFEAT_NR_SUBMAPS: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
