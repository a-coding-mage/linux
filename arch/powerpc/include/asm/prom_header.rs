/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from the C header; contents are kernel-only in the original. */

/*
 * Definitions for talking to the Open Firmware PROM on
 * Power Macintosh computers.
 *
 * Copyright (C) 1996-2005 Paul Mackerras.
 *
 * Updates for PPC64 by Peter Bergner & David Engebretsen, IBM Corp.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

/* Supplied by the surrounding kernel bindings. */
#[allow(non_camel_case_types)]
pub type __be32 = u32;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct property {
    _private: [u8; 0],
}

/* Minimum RMA in bytes for CAS negotiation */
pub const MIN_RMA: u64 = 768u64 * (1u64 << 20);

pub const OF_DT_BEGIN_NODE: u32 = 0x1; /* Start of node, full name */
pub const OF_DT_END_NODE: u32 = 0x2; /* End node */
pub const OF_DT_PROP: u32 = 0x3; /* Property: name off, size, content */
pub const OF_DT_NOP: u32 = 0x4; /* nop */
pub const OF_DT_END: u32 = 0x9;
pub const OF_DT_VERSION: u32 = 0x10;

#[repr(C)]
pub struct boot_param_header {
    pub magic: __be32, /* magic word OF_DT_HEADER */
    pub totalsize: __be32, /* total size of DT block */
    pub off_dt_struct: __be32, /* offset to structure */
    pub off_dt_strings: __be32, /* offset to strings */
    pub off_mem_rsvmap: __be32, /* offset to memory reserve map */
    pub version: __be32, /* format version */
    pub last_comp_version: __be32, /* last compatible version */
    /* version 2 fields below */
    pub boot_cpuid_phys: __be32, /* Physical CPU id we're booting on */
    /* version 3 fields below */
    pub dt_strings_size: __be32, /* size of the DT strings block */
    /* version 17 fields below */
    pub dt_struct_size: __be32, /* size of the DT structure block */
}

extern "C" {
    pub fn of_parse_dma_window(
        dn: *mut device_node,
        dma_window: *const __be32,
        busno: *mut c_ulong,
        phys: *mut c_ulong,
        size: *mut c_ulong,
    );

    pub fn of_instantiate_rtc();

    pub fn of_get_ibm_chip_id(np: *mut device_node) -> c_int;

    pub fn of_read_drc_info_cell(
        prop: *mut *mut property,
        curval: *mut *const __be32,
        data: *mut of_drc_info,
    ) -> c_int;
}

#[repr(C)]
pub struct of_drc_info {
    pub drc_type: *mut c_char,
    pub drc_name_prefix: *mut c_char,
    pub drc_index_start: u32,
    pub drc_name_suffix_start: u32,
    pub num_sequential_elems: u32,
    pub sequential_inc: u32,
    pub drc_power_domain: u32,
    pub last_drc_index: u32,
}

extern "C" {
    pub static mut boot_cpu_node_count: c_uint;
}

/* New method - extensible architecture description vector. */

/* Option vector bits - generic bits in byte 1 */
pub const OV_IGNORE: u32 = 0x80; /* ignore this vector */
pub const OV_CESSATION_POLICY: u32 = 0x40; /* halt if unsupported option present */

/* Option vector 1: processor architectures supported */
pub const OV1_PPC_2_00: u32 = 0x80;
pub const OV1_PPC_2_01: u32 = 0x40;
pub const OV1_PPC_2_02: u32 = 0x20;
pub const OV1_PPC_2_03: u32 = 0x10;
pub const OV1_PPC_2_04: u32 = 0x08;
pub const OV1_PPC_2_05: u32 = 0x04;
pub const OV1_PPC_2_06: u32 = 0x02;
pub const OV1_PPC_2_07: u32 = 0x01;
pub const OV1_PPC_3_00: u32 = 0x80;
pub const OV1_PPC_3_1: u32 = 0x40;
pub const OV1_PPC_3_2: u32 = 0x20;

/* Option vector 2: Open Firmware options supported */
pub const OV2_REAL_MODE: u32 = 0x20;
/* Option vector 3: processor options supported */
pub const OV3_FP: u32 = 0x80;
pub const OV3_VMX: u32 = 0x40;
pub const OV3_DFP: u32 = 0x20;
/* Option vector 4: IBM PAPR implementation */
pub const OV4_MIN_ENT_CAP: u32 = 0x01;

/* Option vector 5: PAPR/OF options supported. */
#[inline]
pub const fn OV5_FEAT(x: u32) -> u32 { x & 0xff }
#[inline]
pub const fn OV5_INDX(x: u32) -> u32 { x >> 8 }

pub const OV5_LPAR: u32 = 0x0280;
pub const OV5_SPLPAR: u32 = 0x0240;
pub const OV5_DRCONF_MEMORY: u32 = 0x0220;
pub const OV5_LARGE_PAGES: u32 = 0x0210;
pub const OV5_DONATE_DEDICATE_CPU: u32 = 0x0202;
pub const OV5_MSI: u32 = 0x0201;
pub const OV5_CMO: u32 = 0x0480;
pub const OV5_XCMO: u32 = 0x0440;
pub const OV5_FORM1_AFFINITY: u32 = 0x0580;
pub const OV5_PRRN: u32 = 0x0540;
pub const OV5_FORM2_AFFINITY: u32 = 0x0520;
pub const OV5_HP_EVT: u32 = 0x0604;
pub const OV5_RESIZE_HPT: u32 = 0x0601;
pub const OV5_PFO_HW_RNG: u32 = 0x1180;
pub const OV5_PFO_HW_842: u32 = 0x1140;
pub const OV5_PFO_HW_ENCR: u32 = 0x1120;
pub const OV5_SUB_PROCESSORS: u32 = 0x1501;
pub const OV5_DRMEM_V2: u32 = 0x1680;
pub const OV5_XIVE_SUPPORT: u32 = 0x17C0;
pub const OV5_XIVE_LEGACY: u32 = 0x1700;
pub const OV5_XIVE_EXPLOIT: u32 = 0x1740;
pub const OV5_XIVE_EITHER: u32 = 0x1780;
pub const OV5_MMU_SUPPORT: u32 = 0x18C0;
pub const OV5_MMU_HASH: u32 = 0x1800;
pub const OV5_MMU_RADIX: u32 = 0x1840;
pub const OV5_MMU_EITHER: u32 = 0x1880;
pub const OV5_MMU_DYNAMIC: u32 = 0x18C0;
pub const OV5_NMMU: u32 = 0x1820;
pub const OV5_HASH_SEG_TBL: u32 = 0x1980;
pub const OV5_HASH_GTSE: u32 = 0x1940;
pub const OV5_RADIX_GTSE: u32 = 0x1A40;
pub const OV5_DRC_INFO: u32 = 0x1640;

/* Option vector 6: IBM PAPR hints */
pub const OV6_LINUX: u32 = 0x02;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
