/* SPDX-License-Identifier: GPL-2.0 */
/*
 * DAT table and related structures
 *
 * Copyright IBM Corp. 2024
 *
 */

/* C bitfields are represented by their containing machine word; field masks
 * and shifts remain part of the target ABI and are intentionally not changed. */

#[repr(C)]
pub union vaddress {
    pub addr: libc::c_ulong,
    pub parts: vaddress_parts,
    pub parts01: vaddress_parts01,
}

#[repr(C)]
pub struct vaddress_parts {
    pub rfx: libc::c_ulong,
    pub rsx: libc::c_ulong,
    pub rtx: libc::c_ulong,
    pub sx: libc::c_ulong,
    pub px: libc::c_ulong,
    pub bx: libc::c_ulong,
}

#[repr(C)]
pub struct vaddress_parts01 {
    pub rfx01: libc::c_ulong,
    pub rsx01: libc::c_ulong,
    pub rtx01: libc::c_ulong,
    pub sx01: libc::c_ulong,
}

#[repr(C)]
pub union asce {
    pub val: libc::c_ulong,
    pub bits: asce_bits,
}

#[repr(C)]
pub struct asce_bits {
    pub rsto: libc::c_ulong,
    pub g: libc::c_ulong,
    pub p: libc::c_ulong,
    pub s: libc::c_ulong,
    pub x: libc::c_ulong,
    pub r: libc::c_ulong,
    pub dt: libc::c_ulong,
    pub tl: libc::c_ulong,
}

pub const ASCE_TYPE_SEGMENT: libc::c_int = 0;
pub const ASCE_TYPE_REGION3: libc::c_int = 1;
pub const ASCE_TYPE_REGION2: libc::c_int = 2;
pub const ASCE_TYPE_REGION1: libc::c_int = 3;

#[repr(C)]
pub union region1_table_entry {
    pub val: libc::c_ulong,
    pub bits: region1_table_entry_bits,
}

#[repr(C)]
pub struct region1_table_entry_bits {
    pub rto: libc::c_ulong,
    pub p: libc::c_ulong,
    pub tf: libc::c_ulong,
    pub i: libc::c_ulong,
    pub tt: libc::c_ulong,
    pub tl: libc::c_ulong,
}

#[repr(C)]
pub union region2_table_entry {
    pub val: libc::c_ulong,
    pub bits: region2_table_entry_bits,
}

#[repr(C)]
pub struct region2_table_entry_bits {
    pub rto: libc::c_ulong,
    pub p: libc::c_ulong,
    pub tf: libc::c_ulong,
    pub i: libc::c_ulong,
    pub tt: libc::c_ulong,
    pub tl: libc::c_ulong,
}

#[repr(C)]
pub struct region3_table_entry_fc0 {
    pub sto: libc::c_ulong,
    pub fc: libc::c_ulong,
    pub p: libc::c_ulong,
    pub tf: libc::c_ulong,
    pub i: libc::c_ulong,
    pub cr: libc::c_ulong,
    pub tt: libc::c_ulong,
    pub tl: libc::c_ulong,
}

#[repr(C)]
pub struct region3_table_entry_fc1 {
    pub rfaa: libc::c_ulong,
    pub av: libc::c_ulong,
    pub acc: libc::c_ulong,
    pub f: libc::c_ulong,
    pub fc: libc::c_ulong,
    pub p: libc::c_ulong,
    pub iep: libc::c_ulong,
    pub i: libc::c_ulong,
    pub cr: libc::c_ulong,
    pub tt: libc::c_ulong,
}

#[repr(C)]
pub struct region3_table_entry_bits {
    pub fc: libc::c_ulong,
    pub p: libc::c_ulong,
    pub i: libc::c_ulong,
    pub cr: libc::c_ulong,
    pub tt: libc::c_ulong,
}

#[repr(C)]
pub union region3_table_entry {
    pub val: libc::c_ulong,
    pub fc0: region3_table_entry_fc0,
    pub fc1: region3_table_entry_fc1,
    pub bits: region3_table_entry_bits,
}

#[repr(C)]
pub struct segment_table_entry_fc0 {
    pub pto: libc::c_ulong,
    pub fc: libc::c_ulong,
    pub p: libc::c_ulong,
    pub i: libc::c_ulong,
    pub cs: libc::c_ulong,
    pub tt: libc::c_ulong,
}

#[repr(C)]
pub struct segment_table_entry_fc1 {
    pub sfaa: libc::c_ulong,
    pub av: libc::c_ulong,
    pub acc: libc::c_ulong,
    pub f: libc::c_ulong,
    pub fc: libc::c_ulong,
    pub p: libc::c_ulong,
    pub iep: libc::c_ulong,
    pub i: libc::c_ulong,
    pub cs: libc::c_ulong,
    pub tt: libc::c_ulong,
}

#[repr(C)]
pub struct segment_table_entry_bits {
    pub fc: libc::c_ulong,
    pub p: libc::c_ulong,
    pub i: libc::c_ulong,
    pub cs: libc::c_ulong,
    pub tt: libc::c_ulong,
}

#[repr(C)]
pub union segment_table_entry {
    pub val: libc::c_ulong,
    pub fc0: segment_table_entry_fc0,
    pub fc1: segment_table_entry_fc1,
    pub bits: segment_table_entry_bits,
}

#[repr(C)]
pub union page_table_entry {
    pub val: libc::c_ulong,
    pub bits: page_table_entry_bits,
}

#[repr(C)]
pub struct page_table_entry_bits {
    pub pfra: libc::c_ulong,
    pub z: libc::c_ulong,
    pub i: libc::c_ulong,
    pub p: libc::c_ulong,
    pub iep: libc::c_ulong,
}

pub const TABLE_TYPE_SEGMENT: libc::c_int = 0;
pub const TABLE_TYPE_REGION3: libc::c_int = 1;
pub const TABLE_TYPE_REGION2: libc::c_int = 2;
pub const TABLE_TYPE_REGION1: libc::c_int = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
