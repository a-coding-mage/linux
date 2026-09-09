/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2018-2019, Intel Corporation. */

// The following data structures define the layout of a PLDM firmware binary.
// Multi-byte fields are little-endian and may be unaligned.

/* UUID for PLDM firmware packages: f018878c-cb7d-4943-9800-a02f059aca02 */
static PLDM_FIRMWARE_HEADER_ID: uuid_t =
    UUID_INIT(0xf018878c, 0xcb7d, 0x4943,
              0x98, 0x00, 0xa0, 0x2f, 0x05, 0x9a, 0xca, 0x02);

/* Revision number of the PLDM header format this code supports */
const PACKAGE_HEADER_FORMAT_REVISION: u8 = 0x01;

/* timestamp104 structure defined in PLDM Base specification */
const PLDM_TIMESTAMP_SIZE: usize = 13;

#[repr(C, packed(1))]
pub struct __pldm_timestamp {
    pub b: [u8; PLDM_TIMESTAMP_SIZE],
}

/* Package Header Information */
#[repr(C, packed(1))]
pub struct __pldm_header {
    pub id: uuid_t,
    pub revision: u8,
    pub size: __le16,
    pub release_date: __pldm_timestamp,
    pub component_bitmap_len: __le16,
    pub version_type: u8,
    pub version_len: u8,
    pub version_string: [u8; 0],
}

/* Firmware Device ID Record */
#[repr(C, packed(1))]
pub struct __pldmfw_record_info {
    pub record_len: __le16,
    pub descriptor_count: u8,
    pub device_update_flags: __le32,
    pub version_type: u8,
    pub version_len: u8,
    pub package_data_len: __le16,
    pub variable_record_data: [u8; 0],
}

/* Firmware Descriptor Definition */
#[repr(C, packed(1))]
pub struct __pldmfw_desc_tlv {
    pub type_: __le16,
    pub size: __le16,
    pub data: [u8; 0],
}

/* Firmware Device Identification Area */
#[repr(C, packed(1))]
pub struct __pldmfw_record_area {
    pub record_count: u8,
    pub records: [u8; 0],
}

/* Individual Component Image Information */
#[repr(C, packed(1))]
pub struct __pldmfw_component_info {
    pub classification: __le16,
    pub identifier: __le16,
    pub comparison_stamp: __le32,
    pub options: __le16,
    pub activation_method: __le16,
    pub location_offset: __le32,
    pub size: __le32,
    pub version_type: u8,
    pub version_len: u8,
    pub version_string: [u8; 0],
}

/* Component Image Information Area */
#[repr(C, packed(1))]
pub struct __pldmfw_component_area {
    pub component_image_count: __le16,
    pub components: [u8; 0],
}

#[inline]
pub unsafe fn pldm_first_desc_tlv(start: *const u8) -> *const __pldmfw_desc_tlv {
    start as *const __pldmfw_desc_tlv
}

#[inline]
pub unsafe fn pldm_next_desc_tlv(desc: *const __pldmfw_desc_tlv) -> *const __pldmfw_desc_tlv {
    let size = get_unaligned_le16(core::ptr::addr_of!((*desc).size));
    (*desc).data.as_ptr().add(size as usize) as *const __pldmfw_desc_tlv
}

#[inline]
pub unsafe fn pldm_first_record(start: *const u8) -> *const __pldmfw_record_info {
    start as *const __pldmfw_record_info
}

#[inline]
pub unsafe fn pldm_next_record(record: *const __pldmfw_record_info) -> *const __pldmfw_record_info {
    let len = get_unaligned_le16(core::ptr::addr_of!((*record).record_len));
    (record as *const u8).add(len as usize) as *const __pldmfw_record_info
}

#[inline]
pub unsafe fn pldm_first_component(start: *const u8) -> *const __pldmfw_component_info {
    start as *const __pldmfw_component_info
}

#[inline]
pub unsafe fn pldm_next_component(
    component: *const __pldmfw_component_info,
) -> *const __pldmfw_component_info {
    (*component).version_string.as_ptr().add((*component).version_len as usize)
        as *const __pldmfw_component_info
}

// The C for-each macros are represented by the corresponding first/next
// functions above; callers should preserve the original index/count loop.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
