/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: the C header includes <linux/types.h> for fixed-width
// integer and big-endian integer types.  Rust fixed-width integers are used
// here; their values retain the C fields' byte-order intent.

/* From document "24x7 Event and Group Catalog Formats Proposal" v0.15 */

#[repr(C, packed)]
pub struct hv_24x7_catalog_page_0 {
    pub magic: u32,
    pub length: u32, /* In 4096 byte pages */
    pub version: u64, /* XXX: arbitrary? what's the meaning/useage/purpose? */
    pub build_time_stamp: [u8; 16], /* "YYYYMMDDHHMMSS\0\0" */
    pub reserved2: [u8; 32],
    pub schema_data_offs: u16, /* in 4096 byte pages */
    pub schema_data_len: u16,  /* in 4096 byte pages */
    pub schema_entry_count: u16,
    pub reserved3: [u8; 2],
    pub event_data_offs: u16,
    pub event_data_len: u16,
    pub event_entry_count: u16,
    pub reserved4: [u8; 2],
    pub group_data_offs: u16, /* in 4096 byte pages */
    pub group_data_len: u16,  /* in 4096 byte pages */
    pub group_entry_count: u16,
    pub reserved5: [u8; 2],
    pub formula_data_offs: u16, /* in 4096 byte pages */
    pub formula_data_len: u16,  /* in 4096 byte pages */
    pub formula_entry_count: u16,
    pub reserved6: [u8; 2],
}

pub const HV_24X7_CATALOG_MAGIC: u32 = 0x3234_7837; /* "24x7" in ASCII */

#[repr(C, packed)]
pub struct hv_24x7_event_data {
    pub length: u16, /* in bytes, must be a multiple of 16 */
    pub reserved1: [u8; 2],
    pub domain: u8, /* Chip = 1, Core = 2 */
    pub reserved2: [u8; 1],
    pub event_group_record_offs: u16, /* in bytes, must be 8 byte aligned */
    pub event_group_record_len: u16, /* in bytes */

    /* in bytes, offset from event_group_record */
    pub event_counter_offs: u16,

    /* verified_state, unverified_state, caveat_state, broken_state, ... */
    pub flags: u32,

    pub primary_group_ix: u16,
    pub group_count: u16,
    pub event_name_len: u16,
    pub remainder: [u8; 0],
    /* pub event_name: [u8; event_name_len - 2]; */
    /* pub event_description_len: u16; */
    /* pub event_desc: [u8; event_description_len - 2]; */
    /* pub detailed_desc_len: u16; */
    /* pub detailed_desc: [u8; detailed_desc_len - 2]; */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
