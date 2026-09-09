/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const E820MAP: usize = 0x2d0; // our map
pub const E820MAX: usize = 128; // number of entries in E820MAP

/*
 * Legacy E820 BIOS limits us to 128 (E820MAX) nodes due to the
 * constrained space in the zeropage.  If we have more nodes than
 * that, and if we've booted off EFI firmware, then the EFI tables
 * passed us from the EFI firmware can list more nodes.  Size our
 * internal memory map tables to have room for these additional
 * nodes, based on up to three entries per node for which the
 * kernel was built: MAX_NUMNODES == (1 << CONFIG_NODES_SHIFT),
 * plus E820MAX, allowing space for the possible duplicate E820
 * entries that might need room in the same arrays, prior to the
 * call to sanitize_e820_map() to remove duplicates.  The allowance
 * of three memory map entries per node is "enough" entries for
 * the initial hardware platform motivating this mechanism to make
 * use of additional EFI map entries.  Future platforms may want
 * to allow more than three entries per node or otherwise refine
 * this size.
 */

// The C definition is guarded by #ifndef __KERNEL__.
#[cfg(not(feature = "kernel"))]
pub const E820_X_MAX: usize = E820MAX;

pub const E820NR: usize = 0x1e8; // # entries in E820MAP

pub const E820_RAM: u32 = 1;
pub const E820_RESERVED: u32 = 2;
pub const E820_ACPI: u32 = 3;
pub const E820_NVS: u32 = 4;
pub const E820_UNUSABLE: u32 = 5;
pub const E820_PMEM: u32 = 7;

/*
 * This is a non-standardized way to represent ADR or NVDIMM regions that
 * persist over a reboot.  The kernel will ignore their special capabilities
 * unless the CONFIG_X86_PMEM_LEGACY option is set.
 *
 * ( Note that older platforms also used 6 for the same type of memory,
 *   but newer versions switched to 12 as 6 was assigned differently.  Some
 *   time they will learn... )
 */
pub const E820_PRAM: u32 = 12;

/*
 * reserved RAM used by kernel itself
 * if CONFIG_INTEL_TXT is enabled, memory of this type will be
 * included in the S3 integrity calculation and so should not include
 * any memory that BIOS might alter over the S3 transition
 */
pub const E820_RESERVED_KERN: u32 = 128;

#[repr(C, packed)]
pub struct e820entry {
    pub addr: u64, // start of memory segment
    pub size: u64, // size of memory segment
    pub type_: u32, // type of memory segment
}

#[repr(C)]
pub struct e820map {
    pub nr_map: u32,
    pub map: [e820entry; E820_X_MAX],
}

pub const ISA_START_ADDRESS: usize = 0xa0000;
pub const ISA_END_ADDRESS: usize = 0x100000;

pub const BIOS_BEGIN: usize = 0x000a0000;
pub const BIOS_END: usize = 0x00100000;

pub const BIOS_ROM_BASE: u32 = 0xffe00000;
pub const BIOS_ROM_END: u32 = 0xffffffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
