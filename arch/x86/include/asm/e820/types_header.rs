/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the translated uapi/asm/bootparam.h header.

/*
 * These are the E820 types known to the kernel:
 */
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum E820Type {
    E820_TYPE_RAM = 1,
    E820_TYPE_RESERVED = 2,
    E820_TYPE_ACPI = 3,
    E820_TYPE_NVS = 4,
    E820_TYPE_UNUSABLE = 5,
    E820_TYPE_PMEM = 7,

    /*
     * This is a non-standardized way to represent ADR or
     * NVDIMM regions that persist over a reboot.
     *
     * The kernel will ignore their special capabilities
     * unless the CONFIG_X86_PMEM_LEGACY=y option is set.
     *
     * ( Note that older platforms also used 6 for the same
     *   type of memory, but newer versions switched to 12 as
     *   6 was assigned differently. Some time they will learn... )
     */
    E820_TYPE_PRAM = 12,

    /*
     * Special-purpose memory is indicated to the system via the
     * EFI_MEMORY_SP attribute. Define an e820 translation of this
     * memory type for the purpose of reserving this range and
     * marking it with the IORES_DESC_SOFT_RESERVED designation.
     */
    E820_TYPE_SOFT_RESERVED = 0xefffffff,
}

/*
 * A single E820 map entry, describing a memory range of [addr...addr+size-1],
 * of 'type' memory type:
 *
 * (We pack it because there can be thousands of them on large systems.)
 */
#[repr(C, packed)]
pub struct E820Entry {
    pub addr: u64,
    pub size: u64,
    pub type_: E820Type,
}

/*
 * The legacy E820 BIOS limits us to 128 (E820_MAX_ENTRIES_ZEROPAGE) nodes
 * due to the constrained space in the zeropage.
 *
 * On large systems we can easily have thousands of nodes with RAM,
 * which cannot be fit into so few entries - so we have a mechanism
 * to extend the e820 table size at build-time, via the E820_MAX_ENTRIES
 * define below.
 *
 * ( Those extra entries are enumerated via the EFI memory map, not
 *   via the legacy zeropage mechanism. )
 *
 * Size our internal memory map tables to have room for these additional
 * entries, based on a heuristic calculation: up to three entries per
 * NUMA node, plus E820_MAX_ENTRIES_ZEROPAGE for some extra space.
 *
 * This allows for bootstrap/firmware quirks such as possible duplicate
 * E820 entries that might need room in the same arrays, prior to the
 * call to e820__update_table() to remove duplicates.  The allowance
 * of three memory map entries per node is "enough" entries for
 * the initial hardware platform motivating this mechanism to make
 * use of additional EFI map entries.  Future platforms may want
 * to allow more than three entries per node or otherwise refine
 * this size.
 */

// Dependency supplied by the translated linux/numa.h header.
pub const E820_MAX_ENTRIES: usize = (E820_MAX_ENTRIES_ZEROPAGE + 3 * MAX_NUMNODES) as usize;

/*
 * The whole array of E820 entries:
 */
#[repr(C)]
pub struct E820Table {
    pub nr_entries: u32,
    pub entries: [E820Entry; E820_MAX_ENTRIES],
}

/*
 * Various well-known legacy memory ranges in physical memory:
 */
pub const ISA_START_ADDRESS: u32 = 0x000a0000;
pub const ISA_END_ADDRESS: u32 = 0x00100000;

pub const BIOS_BEGIN: u32 = 0x000a0000;
pub const BIOS_END: u32 = 0x00100000;

pub const HIGH_MEMORY: u32 = 0x00100000;

pub const BIOS_ROM_BASE: u32 = 0xffe00000;
pub const BIOS_ROM_END: u32 = 0xffffffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
