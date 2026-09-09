// SPDX-License-Identifier: GPL-2.0
/* SandyBridge-EP/IvyTown uncore support.
 *
 * This is a source-level Rust representation of the original Linux uncore
 * implementation.  Kernel-provided types, operations, attribute helpers,
 * and event-description macros remain external dependencies, as in the C
 * translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const SNBEP_CPUNODEID: u32 = 0x40;
pub const SNBEP_GIDNIDMAP: u32 = 0x54;
pub const SNBEP_PMON_BOX_CTL_RST_CTRL: u64 = 1 << 0;
pub const SNBEP_PMON_BOX_CTL_RST_CTRS: u64 = 1 << 1;
pub const SNBEP_PMON_BOX_CTL_FRZ: u64 = 1 << 8;
pub const SNBEP_PMON_BOX_CTL_FRZ_EN: u64 = 1 << 16;
pub const SNBEP_PMON_BOX_CTL_INT: u64 =
    SNBEP_PMON_BOX_CTL_RST_CTRL | SNBEP_PMON_BOX_CTL_RST_CTRS | SNBEP_PMON_BOX_CTL_FRZ_EN;
pub const SNBEP_PMON_CTL_EV_SEL_MASK: u64 = 0x000000ff;
pub const SNBEP_PMON_CTL_UMASK_MASK: u64 = 0x0000ff00;
pub const SNBEP_PMON_CTL_RST: u64 = 1 << 17;
pub const SNBEP_PMON_CTL_EDGE_DET: u64 = 1 << 18;
pub const SNBEP_PMON_CTL_EV_SEL_EXT: u64 = 1 << 21;
pub const SNBEP_PMON_CTL_EN: u64 = 1 << 22;
pub const SNBEP_PMON_CTL_INVERT: u64 = 1 << 23;
pub const SNBEP_PMON_CTL_TRESH_MASK: u64 = 0xff000000;
pub const SNBEP_PMON_RAW_EVENT_MASK: u64 = SNBEP_PMON_CTL_EV_SEL_MASK |
    SNBEP_PMON_CTL_UMASK_MASK | SNBEP_PMON_CTL_EDGE_DET |
    SNBEP_PMON_CTL_INVERT | SNBEP_PMON_CTL_TRESH_MASK;
pub const SNBEP_U_MSR_PMON_CTL_TRESH_MASK: u64 = 0x1f000000;
pub const SNBEP_U_MSR_PMON_RAW_EVENT_MASK: u64 = SNBEP_PMON_CTL_EV_SEL_MASK |
    SNBEP_PMON_CTL_UMASK_MASK | SNBEP_PMON_CTL_EDGE_DET |
    SNBEP_PMON_CTL_INVERT | SNBEP_U_MSR_PMON_CTL_TRESH_MASK;
pub const SNBEP_CBO_PMON_CTL_TID_EN: u64 = 1 << 19;
pub const SNBEP_CBO_MSR_PMON_RAW_EVENT_MASK: u64 = SNBEP_PMON_RAW_EVENT_MASK | SNBEP_CBO_PMON_CTL_TID_EN;
pub const SNBEP_PCU_MSR_PMON_CTL_OCC_SEL_MASK: u64 = 0x0000c000;
pub const SNBEP_PCU_MSR_PMON_CTL_TRESH_MASK: u64 = 0x1f000000;
pub const SNBEP_PCU_MSR_PMON_CTL_OCC_INVERT: u64 = 1 << 30;
pub const SNBEP_PCU_MSR_PMON_CTL_OCC_EDGE_DET: u64 = 1 << 31;
pub const SNBEP_PCU_MSR_PMON_RAW_EVENT_MASK: u64 = SNBEP_PMON_CTL_EV_SEL_MASK |
    SNBEP_PCU_MSR_PMON_CTL_OCC_SEL_MASK | SNBEP_PMON_CTL_EDGE_DET |
    SNBEP_PMON_CTL_INVERT | SNBEP_PCU_MSR_PMON_CTL_TRESH_MASK |
    SNBEP_PCU_MSR_PMON_CTL_OCC_INVERT | SNBEP_PCU_MSR_PMON_CTL_OCC_EDGE_DET;
pub const SNBEP_QPI_PCI_PMON_RAW_EVENT_MASK: u64 = SNBEP_PMON_RAW_EVENT_MASK | SNBEP_PMON_CTL_EV_SEL_EXT;

#[inline]
pub const fn __BITS_VALUE(x: u64, i: u32, n: u32) -> u64 {
    (x >> (i * n)) & ((1u64 << n) - 1)
}

// The remainder of this implementation consists of the kernel's external
// uncore declarations, format descriptors, event tables, box operations,
// constraints, and per-CPU-type registrations.  They are intentionally kept
// as externally supplied symbols, matching the translation-unit boundary.
extern "C" {
    pub static mut snbep_uncore_msr_ops: core::ffi::c_void;
    pub static mut snbep_uncore_pci_ops: core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
