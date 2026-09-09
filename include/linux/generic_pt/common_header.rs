/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES
 */

/*
 * Generic Radix Page Table
 *
 * Generic Radix Page Table is a set of functions and helpers to efficiently
 * parse radix style page tables typically seen in HW implementations. The
 * interface is built to deliver similar code generation as the mm's pte/pmd/etc
 * system by fully inlining the exact code required to handle each table level.
 *
 * Like the mm subsystem each format contributes its parsing implementation
 * under common names and the common code implements the required algorithms.
 *
 * The system is divided into three logical levels:
 *
 *  - The page table format and its manipulation functions
 *  - Generic helpers to give a consistent API regardless of underlying format
 *  - An algorithm implementation (e.g. IOMMU/DRM/KVM/MM)
 *
 * Multiple implementations are supported. The intention is to have the generic
 * format code be re-usable for whatever specialized implementation is required.
 * The generic code is solely about the format of the radix tree; it does not
 * include memory allocation or higher level decisions that are left for the
 * implementation.
 *
 * The generic framework supports a superset of functions across many HW
 * implementations:
 *
 *  - Entries comprised of contiguous blocks of IO PTEs for larger page sizes
 *  - Multi-level tables, up to 6 levels. Runtime selected top level
 *  - Runtime variable table level size (ARM's concatenated tables)
 *  - Expandable top level allowing dynamic sizing of table levels
 *  - Optional leaf entries at any level
 *  - 32-bit/64-bit virtual and output addresses, using every address bit
 *  - Dirty tracking
 *  - Sign extended addressing
 */

/// struct pt_common - struct for all page table implementations
#[repr(C)]
pub struct pt_common {
    /// Encodes the table top pointer and the top level in a single value.
    /// Must use READ_ONCE/WRITE_ONCE to access it. The lower bits of the
    /// aligned table pointer are used for the level.
    pub top_of_table: uintptr_t,
    /// Maximum number of bits the OA can contain. Upper bits must be zero.
    /// This may be less than what the page table format supports, but must
    /// not be more.
    pub max_oasz_lg2: u8,
    /// Maximum number of bits the VA can contain. Upper bits are 0 or 1
    /// depending on pt_full_va_prefix(). This may be less than what the page
    /// table format supports, but must not be more. When PT_FEAT_DYNAMIC_TOP
    /// is set this reflects the maximum VA capability.
    pub max_vasz_lg2: u8,
    /// Bitmap of `enum pt_features`
    pub features: core::ffi::c_uint,
}

/// Encoding parameters for top_of_table.
pub const PT_TOP_LEVEL_BITS: core::ffi::c_int = 3;
pub const PT_TOP_LEVEL_MASK: core::ffi::c_uint = (1u32 << PT_TOP_LEVEL_BITS) - 1;

/// Features turned on in the table. Each symbol is a bit position.
#[repr(u32)]
pub enum pt_features {
    /// Cache flush page table memory before assuming the HW can read it.
    /// Otherwise a SMP release is sufficient for HW to read it.
    PT_FEAT_DMA_INCOHERENT,
    /// The table can span the full VA range from 0 to PT_VADDR_MAX.
    PT_FEAT_FULL_VA,
    /// The table's top level can be increased dynamically during map. This
    /// requires HW support for atomically setting both the table top pointer
    /// and the starting table level.
    PT_FEAT_DYNAMIC_TOP,
    /// The top most bit of the valid VA range sign extends up to the full
    /// pt_vaddr_t. This divides the page table into three VA ranges:
    ///
    ///   0 -> 2^N - 1             Lower
    ///   2^N -> (MAX - 2^N - 1)    Non-Canonical
    ///   MAX - 2^N -> MAX          Upper
    ///
    /// In this mode pt_common::max_vasz_lg2 includes the sign bit and the
    /// upper bits that don't fall within the translation are just validated.
    /// If not set there is no sign extension and valid VA goes from 0 to
    /// 2^N - 1.
    PT_FEAT_SIGN_EXTEND,
    /// IOTLB maintenance is done by flushing IOVA ranges which will clean
    /// out any walk cache or any IOPTE fully contained by the range. The
    /// optimization objective is to minimize the number of flushes even if
    /// ranges include IOVA gaps that do not need to be flushed.
    PT_FEAT_FLUSH_RANGE,
    /// Like PT_FEAT_FLUSH_RANGE except that the optimization objective is to
    /// only flush IOVA that has been changed. This mode is suitable for cases
    /// like hypervisor shadowing where flushing unchanged ranges may cause the
    /// hypervisor to reparse significant amount of page table.
    PT_FEAT_FLUSH_RANGE_NO_GAPS,
    /// Fill in the struct iommu_iotlb_gather pt sub structure with information
    /// about which levels were changed.
    PT_FEAT_DETAILED_GATHER,
    /// private:
    PT_FEAT_FMT_START,
}

#[repr(C)]
pub struct pt_amdv1 {
    pub common: pt_common,
}

/// The memory backing the tables is encrypted. Use __sme_set() to adjust the
/// page table pointers in the tree. This only works with CONFIG_AMD_MEM_ENCRYPT.
pub const PT_FEAT_AMDV1_ENCRYPT_TABLES: u32 = PT_FEAT_FMT_START as u32;
/// The PTEs are set to prevent cache incoherent traffic, such as PCI no snoop.
/// This is set either at creation time or before the first map operation.
pub const PT_FEAT_AMDV1_FORCE_COHERENCE: u32 = PT_FEAT_AMDV1_ENCRYPT_TABLES + 1;

#[repr(C)]
pub struct pt_vtdss {
    pub common: pt_common,
}

/// The PTEs are set to prevent cache incoherent traffic, such as PCI no snoop.
/// This is set either at creation time or before the first map operation.
pub const PT_FEAT_VTDSS_FORCE_COHERENCE: u32 = PT_FEAT_FMT_START as u32;
/// Prevent creating read-only PTEs. Used to work around HW errata ERRATA_772415_SPR17.
pub const PT_FEAT_VTDSS_FORCE_WRITEABLE: u32 = PT_FEAT_VTDSS_FORCE_COHERENCE + 1;

#[repr(C)]
pub struct pt_riscv_32 {
    pub common: pt_common,
}

#[repr(C)]
pub struct pt_riscv_64 {
    pub common: pt_common,
}

/// Support the 64k contiguous page size following the Svnapot extension.
pub const PT_FEAT_RISCV_SVNAPOT_64K: u32 = PT_FEAT_FMT_START as u32;
/// Support Svpbmt extension: encode page-based memory type (PBMT) in PTEs.
pub const PT_FEAT_RISCV_SVPBMT: u32 = PT_FEAT_RISCV_SVNAPOT_64K + 1;

#[repr(C)]
pub struct pt_x86_64 {
    pub common: pt_common,
}

/// The memory backing the tables is encrypted. Use __sme_set() to adjust the
/// page table pointers in the tree. This only works with CONFIG_AMD_MEM_ENCRYPT.
pub const PT_FEAT_X86_64_AMD_ENCRYPT_TABLES: u32 = PT_FEAT_FMT_START as u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
