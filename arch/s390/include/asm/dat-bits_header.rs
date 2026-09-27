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
    pub addr: kernel::ffi::c_ulong,
    pub parts: vaddress_parts,
    pub parts01: vaddress_parts01,
}

#[repr(C)]
pub struct vaddress_parts {
    pub rfx: kernel::ffi::c_ulong,
    pub rsx: kernel::ffi::c_ulong,
    pub rtx: kernel::ffi::c_ulong,
    pub sx: kernel::ffi::c_ulong,
    pub px: kernel::ffi::c_ulong,
    pub bx: kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct vaddress_parts01 {
    pub rfx01: kernel::ffi::c_ulong,
    pub rsx01: kernel::ffi::c_ulong,
    pub rtx01: kernel::ffi::c_ulong,
    pub sx01: kernel::ffi::c_ulong,
}

#[repr(C)]
pub union asce {
    pub val: kernel::ffi::c_ulong,
    pub bits: asce_bits,
}

#[repr(C)]
pub struct asce_bits {
    pub rsto: kernel::ffi::c_ulong,
    pub g: kernel::ffi::c_ulong,
    pub p: kernel::ffi::c_ulong,
    pub s: kernel::ffi::c_ulong,
    pub x: kernel::ffi::c_ulong,
    pub r: kernel::ffi::c_ulong,
    pub dt: kernel::ffi::c_ulong,
    pub tl: kernel::ffi::c_ulong,
}

pub const ASCE_TYPE_SEGMENT: kernel::ffi::c_int = 0;
pub const ASCE_TYPE_REGION3: kernel::ffi::c_int = 1;
pub const ASCE_TYPE_REGION2: kernel::ffi::c_int = 2;
pub const ASCE_TYPE_REGION1: kernel::ffi::c_int = 3;

#[repr(C)]
pub union region1_table_entry {
    pub val: kernel::ffi::c_ulong,
    pub bits: region1_table_entry_bits,
}

#[repr(C)]
pub struct region1_table_entry_bits {
    pub rto: kernel::ffi::c_ulong,
    pub p: kernel::ffi::c_ulong,
    pub tf: kernel::ffi::c_ulong,
    pub i: kernel::ffi::c_ulong,
    pub tt: kernel::ffi::c_ulong,
    pub tl: kernel::ffi::c_ulong,
}

#[repr(C)]
pub union region2_table_entry {
    pub val: kernel::ffi::c_ulong,
    pub bits: region2_table_entry_bits,
}

#[repr(C)]
pub struct region2_table_entry_bits {
    pub rto: kernel::ffi::c_ulong,
    pub p: kernel::ffi::c_ulong,
    pub tf: kernel::ffi::c_ulong,
    pub i: kernel::ffi::c_ulong,
    pub tt: kernel::ffi::c_ulong,
    pub tl: kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct region3_table_entry_fc0 {
    pub sto: kernel::ffi::c_ulong,
    pub fc: kernel::ffi::c_ulong,
    pub p: kernel::ffi::c_ulong,
    pub tf: kernel::ffi::c_ulong,
    pub i: kernel::ffi::c_ulong,
    pub cr: kernel::ffi::c_ulong,
    pub tt: kernel::ffi::c_ulong,
    pub tl: kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct region3_table_entry_fc1 {
    pub rfaa: kernel::ffi::c_ulong,
    pub av: kernel::ffi::c_ulong,
    pub acc: kernel::ffi::c_ulong,
    pub f: kernel::ffi::c_ulong,
    pub fc: kernel::ffi::c_ulong,
    pub p: kernel::ffi::c_ulong,
    pub iep: kernel::ffi::c_ulong,
    pub i: kernel::ffi::c_ulong,
    pub cr: kernel::ffi::c_ulong,
    pub tt: kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct region3_table_entry_bits {
    pub fc: kernel::ffi::c_ulong,
    pub p: kernel::ffi::c_ulong,
    pub i: kernel::ffi::c_ulong,
    pub cr: kernel::ffi::c_ulong,
    pub tt: kernel::ffi::c_ulong,
}

#[repr(C)]
pub union region3_table_entry {
    pub val: kernel::ffi::c_ulong,
    pub fc0: region3_table_entry_fc0,
    pub fc1: region3_table_entry_fc1,
    pub bits: region3_table_entry_bits,
}

#[repr(C)]
pub struct segment_table_entry_fc0 {
    pub pto: kernel::ffi::c_ulong,
    pub fc: kernel::ffi::c_ulong,
    pub p: kernel::ffi::c_ulong,
    pub i: kernel::ffi::c_ulong,
    pub cs: kernel::ffi::c_ulong,
    pub tt: kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct segment_table_entry_fc1 {
    pub sfaa: kernel::ffi::c_ulong,
    pub av: kernel::ffi::c_ulong,
    pub acc: kernel::ffi::c_ulong,
    pub f: kernel::ffi::c_ulong,
    pub fc: kernel::ffi::c_ulong,
    pub p: kernel::ffi::c_ulong,
    pub iep: kernel::ffi::c_ulong,
    pub i: kernel::ffi::c_ulong,
    pub cs: kernel::ffi::c_ulong,
    pub tt: kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct segment_table_entry_bits {
    pub fc: kernel::ffi::c_ulong,
    pub p: kernel::ffi::c_ulong,
    pub i: kernel::ffi::c_ulong,
    pub cs: kernel::ffi::c_ulong,
    pub tt: kernel::ffi::c_ulong,
}

#[repr(C)]
pub union segment_table_entry {
    pub val: kernel::ffi::c_ulong,
    pub fc0: segment_table_entry_fc0,
    pub fc1: segment_table_entry_fc1,
    pub bits: segment_table_entry_bits,
}

#[repr(C)]
pub union page_table_entry {
    pub val: kernel::ffi::c_ulong,
    pub bits: page_table_entry_bits,
}

#[repr(C)]
pub struct page_table_entry_bits {
    pub pfra: kernel::ffi::c_ulong,
    pub z: kernel::ffi::c_ulong,
    pub i: kernel::ffi::c_ulong,
    pub p: kernel::ffi::c_ulong,
    pub iep: kernel::ffi::c_ulong,
}

pub const TABLE_TYPE_SEGMENT: kernel::ffi::c_int = 0;
pub const TABLE_TYPE_REGION3: kernel::ffi::c_int = 1;
pub const TABLE_TYPE_REGION2: kernel::ffi::c_int = 2;
pub const TABLE_TYPE_REGION1: kernel::ffi::c_int = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
