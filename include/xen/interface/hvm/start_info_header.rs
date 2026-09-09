/* SPDX-License-Identifier: MIT */
/*
 * Copyright (c) 2016, Citrix Systems, Inc.
 */

/*
 * Start of day structure passed to PVH guests and to HVM guests in %ebx.
 *
 * NOTE: nothing will be loaded at physical address 0, so a 0 value in any
 * of the address fields should be treated as not present.
 *
 *  0 +----------------+
 *    | magic          | Contains the magic value XEN_HVM_START_MAGIC_VALUE
 *    |                | ("xEn3" with the 0x80 bit of the "E" set).
 *  4 +----------------+
 *    | version        | Version of this structure. Current version is 1. New
 *    |                | versions are guaranteed to be backwards-compatible.
 *  8 +----------------+
 *    | flags          | SIF_xxx flags.
 * 12 +----------------+
 *    | nr_modules     | Number of modules passed to the kernel.
 * 16 +----------------+
 *    | modlist_paddr  | Physical address of an array of modules
 *    |                | (layout of the structure below).
 * 24 +----------------+
 *    | cmdline_paddr  | Physical address of the command line,
 *    |                | a zero-terminated ASCII string.
 * 32 +----------------+
 *    | rsdp_paddr     | Physical address of the RSDP ACPI data structure.
 * 40 +----------------+
 *    | memmap_paddr   | Physical address of the (optional) memory map. Only
 *    |                | present in version 1 and newer of the structure.
 * 48 +----------------+
 *    | memmap_entries | Number of entries in the memory map table. Zero
 *    |                | if there is no memory map being provided. Only
 *    |                | present in version 1 and newer of the structure.
 * 52 +----------------+
 *    | reserved       | Version 1 and newer only.
 * 56 +----------------+
 */
pub const XEN_HVM_START_MAGIC_VALUE: u32 = 0x336e_c578;

/*
 * The values used in the type field of the memory map table entries are
 * defined below and match the Address Range Types as defined in the "System
 * Address Map Interfaces" section of the ACPI Specification. Please refer to
 * section 15 in version 6.2 of the ACPI spec: http://uefi.org/specifications
 */
pub const XEN_HVM_MEMMAP_TYPE_RAM: u32 = 1;
pub const XEN_HVM_MEMMAP_TYPE_RESERVED: u32 = 2;
pub const XEN_HVM_MEMMAP_TYPE_ACPI: u32 = 3;
pub const XEN_HVM_MEMMAP_TYPE_NVS: u32 = 4;
pub const XEN_HVM_MEMMAP_TYPE_UNUSABLE: u32 = 5;
pub const XEN_HVM_MEMMAP_TYPE_DISABLED: u32 = 6;
pub const XEN_HVM_MEMMAP_TYPE_PMEM: u32 = 7;

/*
 * C representation of the x86/HVM start info layout.
 *
 * The canonical definition of this layout is above, this is just a way to
 * represent the layout described there using C types.
 */
#[repr(C)]
pub struct hvm_start_info {
    pub magic: u32,             /* Contains the magic value 0x336ec578       */
                                /* ("xEn3" with the 0x80 bit of the "E" set).*/
    pub version: u32,           /* Version of this structure.                */
    pub flags: u32,             /* SIF_xxx flags.                            */
    pub nr_modules: u32,        /* Number of modules passed to the kernel.   */
    pub modlist_paddr: u64,     /* Physical address of an array of           */
                                /* hvm_modlist_entry.                        */
    pub cmdline_paddr: u64,     /* Physical address of the command line.     */
    pub rsdp_paddr: u64,        /* Physical address of the RSDP ACPI data    */
                                /* structure.                                */
    /* All following fields only present in version 1 and newer */
    pub memmap_paddr: u64,      /* Physical address of an array of           */
                                /* hvm_memmap_table_entry.                   */
    pub memmap_entries: u32,    /* Number of entries in the memmap table.    */
                                /* Value will be zero if there is no memory  */
                                /* map being provided.                       */
    pub reserved: u32,          /* Must be zero.                             */
}

#[repr(C)]
pub struct hvm_modlist_entry {
    pub paddr: u64,             /* Physical address of the module.           */
    pub size: u64,              /* Size of the module in bytes.              */
    pub cmdline_paddr: u64,     /* Physical address of the command line.     */
    pub reserved: u64,
}

#[repr(C)]
pub struct hvm_memmap_table_entry {
    pub addr: u64,              /* Base address of the memory region         */
    pub size: u64,              /* Size of the memory region in bytes        */
    pub r#type: u32,            /* Mapping type                              */
    pub reserved: u32,          /* Must be zero for Version 1.               */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
