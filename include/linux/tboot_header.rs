/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tboot.h: shared data structure with tboot and kernel and functions
 *          used by kernel for runtime support of Intel(R) Trusted
 *          Execution Technology
 *
 * Copyright (c) 2006-2009, Intel Corporation
 */

/* These must have the values from 0-5 in this order. */
pub const TB_SHUTDOWN_REBOOT: i32 = 0;
pub const TB_SHUTDOWN_S5: i32 = 1;
pub const TB_SHUTDOWN_S4: i32 = 2;
pub const TB_SHUTDOWN_S3: i32 = 3;
pub const TB_SHUTDOWN_HALT: i32 = 4;
pub const TB_SHUTDOWN_WFS: i32 = 5;

/* The following declarations are present when CONFIG_INTEL_TXT is enabled. */
#[cfg(CONFIG_INTEL_TXT)]
pub const TB_KEY_SIZE: usize = 64; /* 512 bits */

#[cfg(CONFIG_INTEL_TXT)]
pub const MAX_TB_MAC_REGIONS: usize = 32;

#[cfg(CONFIG_INTEL_TXT)]
#[repr(C, packed)]
pub struct tboot_mac_region {
    pub start: u64, /* must be 64 byte-aligned */
    pub size: u32,  /* must be 64 byte-granular */
}

/* GAS - Generic Address Structure (ACPI 2.0+). */
#[cfg(CONFIG_INTEL_TXT)]
#[repr(C, packed)]
pub struct tboot_acpi_generic_address {
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_width: u8,
    pub address: u64,
}

/*
 * Combines Sx info from FADT and FACS tables per ACPI 2.0+ spec
 * (https://uefi.org/specifications)
 */
#[cfg(CONFIG_INTEL_TXT)]
#[repr(C, packed)]
pub struct tboot_acpi_sleep_info {
    pub pm1a_cnt_blk: tboot_acpi_generic_address,
    pub pm1b_cnt_blk: tboot_acpi_generic_address,
    pub pm1a_evt_blk: tboot_acpi_generic_address,
    pub pm1b_evt_blk: tboot_acpi_generic_address,
    pub pm1a_cnt_val: u16,
    pub pm1b_cnt_val: u16,
    pub wakeup_vector: u64,
    pub vector_width: u32,
    pub kernel_s3_resume_vector: u64,
}

/* Shared memory page used for communication between tboot and kernel. */
#[cfg(CONFIG_INTEL_TXT)]
#[repr(C, packed)]
pub struct tboot {
    /* version 3+ fields: */
    pub uuid: [u8; 16], /* TBOOT_UUID */
    pub version: u32,  /* version number: 5 is current */
    pub log_addr: u32, /* physical addr of tb_log_t log */

    /* physical addr of entry point for tboot shutdown and
     * type of shutdown (TB_SHUTDOWN_*) being requested */
    pub shutdown_entry: u32,
    pub shutdown_type: u32,

    /* kernel-specified ACPI info for Sx shutdown */
    pub acpi_sinfo: tboot_acpi_sleep_info,

    /* tboot location in memory (physical) */
    pub tboot_base: u32,
    pub tboot_size: u32,

    /* memory regions (phys addrs) for tboot to MAC on S3 */
    pub num_mac_regions: u8,
    pub mac_regions: [tboot_mac_region; MAX_TB_MAC_REGIONS],

    /* version 4+ fields: */
    /* symmetric key for use by kernel; will be encrypted on S3 */
    pub s3_key: [u8; TB_KEY_SIZE],

    /* version 5+ fields: */
    /* used to 4byte-align num_in_wfs */
    pub reserved_align: [u8; 3],
    /* number of processors in wait-for-SIPI */
    pub num_in_wfs: u32,
}

/*
 * UUID for tboot data struct to facilitate matching.
 * Defined as {663C8DFF-E8B3-4b82-AABF-19EA4D057A08} by tboot, which is
 * represented as {} in the char array used here.
 */
pub const TBOOT_UUID: [u8; 16] = [
    0xff, 0x8d, 0x3c, 0x66, 0xb3, 0xe8, 0x82, 0x4b,
    0xbf, 0xaa, 0x19, 0xea, 0x4d, 0x05, 0x7a, 0x08,
];

#[cfg(CONFIG_INTEL_TXT)]
extern "C" {
    pub fn tboot_enabled() -> bool;
    pub fn tboot_probe();
    pub fn tboot_shutdown(shutdown_type: u32);
    pub fn tboot_get_dmar_table(
        dmar_tbl: *mut crate::acpi_table_header,
    ) -> *mut crate::acpi_table_header;
}

/* CONFIG_INTEL_TXT-disabled equivalents of the C macros. */
#[cfg(not(CONFIG_INTEL_TXT))]
#[inline]
pub const fn tboot_enabled() -> bool { false }

#[cfg(not(CONFIG_INTEL_TXT))]
#[inline]
pub fn tboot_probe() {}

#[cfg(not(CONFIG_INTEL_TXT))]
#[inline]
pub fn tboot_shutdown(_shutdown_type: u32) {}

#[cfg(not(CONFIG_INTEL_TXT))]
#[inline]
pub fn tboot_get_dmar_table<T>(dmar_tbl: T) -> T { dmar_tbl }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
