/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2018-2019, Intel Corporation. */

// Dependencies supplied by the corresponding Linux headers:
// list_head, device, and firmware.

pub const PLDM_DEVICE_UPDATE_CONTINUE_AFTER_FAIL: u32 = 1u32 << 0;

pub const PLDM_STRING_TYPE_UNKNOWN: u8 = 0;
pub const PLDM_STRING_TYPE_ASCII: u8 = 1;
pub const PLDM_STRING_TYPE_UTF8: u8 = 2;
pub const PLDM_STRING_TYPE_UTF16: u8 = 3;
pub const PLDM_STRING_TYPE_UTF16LE: u8 = 4;
pub const PLDM_STRING_TYPE_UTF16BE: u8 = 5;

#[repr(C)]
pub struct pldmfw_record {
    pub entry: list_head,

    /* List of descriptor TLVs */
    pub descs: list_head,

    /* Component Set version string*/
    pub version_string: *const u8,
    pub version_type: u8,
    pub version_len: u8,

    /* Package Data length */
    pub package_data_len: u16,

    /* Bitfield of Device Update Flags */
    pub device_update_flags: u32,

    /* Package Data block */
    pub package_data: *const u8,

    /* Bitmap of components applicable to this record */
    pub component_bitmap: *mut usize,
    pub component_bitmap_len: u16,
}

/* Standard descriptor TLV identifiers */
pub const PLDM_DESC_ID_PCI_VENDOR_ID: u16 = 0x0000;
pub const PLDM_DESC_ID_IANA_ENTERPRISE_ID: u16 = 0x0001;
pub const PLDM_DESC_ID_UUID: u16 = 0x0002;
pub const PLDM_DESC_ID_PNP_VENDOR_ID: u16 = 0x0003;
pub const PLDM_DESC_ID_ACPI_VENDOR_ID: u16 = 0x0004;
pub const PLDM_DESC_ID_PCI_DEVICE_ID: u16 = 0x0100;
pub const PLDM_DESC_ID_PCI_SUBVENDOR_ID: u16 = 0x0101;
pub const PLDM_DESC_ID_PCI_SUBDEV_ID: u16 = 0x0102;
pub const PLDM_DESC_ID_PCI_REVISION_ID: u16 = 0x0103;
pub const PLDM_DESC_ID_PNP_PRODUCT_ID: u16 = 0x0104;
pub const PLDM_DESC_ID_ACPI_PRODUCT_ID: u16 = 0x0105;
pub const PLDM_DESC_ID_VENDOR_DEFINED: u16 = 0xFFFF;

#[repr(C)]
pub struct pldmfw_desc_tlv {
    pub entry: list_head,
    pub data: *const u8,
    pub type_: u16,
    pub size: u16,
}

pub const PLDM_CLASSIFICATION_UNKNOWN: u16 = 0x0000;
pub const PLDM_CLASSIFICATION_OTHER: u16 = 0x0001;
pub const PLDM_CLASSIFICATION_DRIVER: u16 = 0x0002;
pub const PLDM_CLASSIFICATION_CONFIG_SW: u16 = 0x0003;
pub const PLDM_CLASSIFICATION_APP_SW: u16 = 0x0004;
pub const PLDM_CLASSIFICATION_INSTRUMENTATION: u16 = 0x0005;
pub const PLDM_CLASSIFICATION_BIOS: u16 = 0x0006;
pub const PLDM_CLASSIFICATION_DIAGNOSTIC_SW: u16 = 0x0007;
pub const PLDM_CLASSIFICATION_OS: u16 = 0x0008;
pub const PLDM_CLASSIFICATION_MIDDLEWARE: u16 = 0x0009;
pub const PLDM_CLASSIFICATION_FIRMWARE: u16 = 0x000A;
pub const PLDM_CLASSIFICATION_CODE: u16 = 0x000B;
pub const PLDM_CLASSIFICATION_SERVICE_PACK: u16 = 0x000C;
pub const PLDM_CLASSIFICATION_SOFTWARE_BUNDLE: u16 = 0x000D;

pub const PLDM_ACTIVATION_METHOD_AUTO: u16 = 1u16 << 0;
pub const PLDM_ACTIVATION_METHOD_SELF_CONTAINED: u16 = 1u16 << 1;
pub const PLDM_ACTIVATION_METHOD_MEDIUM_SPECIFIC: u16 = 1u16 << 2;
pub const PLDM_ACTIVATION_METHOD_REBOOT: u16 = 1u16 << 3;
pub const PLDM_ACTIVATION_METHOD_DC_CYCLE: u16 = 1u16 << 4;
pub const PLDM_ACTIVATION_METHOD_AC_CYCLE: u16 = 1u16 << 5;

pub const PLDMFW_COMPONENT_OPTION_FORCE_UPDATE: u16 = 1u16 << 0;
pub const PLDMFW_COMPONENT_OPTION_USE_COMPARISON_STAMP: u16 = 1u16 << 1;

#[repr(C)]
pub struct pldmfw_component {
    pub entry: list_head,
    /* component identifier */
    pub classification: u16,
    pub identifier: u16,
    pub options: u16,
    pub activation_method: u16,
    pub comparison_stamp: u32,
    pub component_size: u32,
    pub component_data: *const u8,
    /* Component version string */
    pub version_string: *const u8,
    pub version_type: u8,
    pub version_len: u8,
    /* component index */
    pub index: u8,
}

/* Transfer flag used for sending components to the firmware */
pub const PLDM_TRANSFER_FLAG_START: u8 = 1u8 << 0;
pub const PLDM_TRANSFER_FLAG_MIDDLE: u8 = 1u8 << 1;
pub const PLDM_TRANSFER_FLAG_END: u8 = 1u8 << 2;

#[repr(C)]
pub struct pldmfw_ops {
    pub match_record: Option<unsafe extern "C" fn(context: *mut pldmfw, record: *mut pldmfw_record) -> bool>,
    pub send_package_data: Option<unsafe extern "C" fn(context: *mut pldmfw, data: *const u8, length: u16) -> i32>,
    pub send_component_table: Option<unsafe extern "C" fn(context: *mut pldmfw, component: *mut pldmfw_component, transfer_flag: u8) -> i32>,
    pub flash_component: Option<unsafe extern "C" fn(context: *mut pldmfw, component: *mut pldmfw_component) -> i32>,
    pub finalize_update: Option<unsafe extern "C" fn(context: *mut pldmfw) -> i32>,
}

#[repr(C)]
pub enum pldmfw_update_mode {
    PLDMFW_UPDATE_MODE_FULL,
    PLDMFW_UPDATE_MODE_SINGLE_COMPONENT,
}

#[repr(C)]
pub struct pldmfw {
    pub ops: *const pldmfw_ops,
    pub dev: *mut device,
    pub component_identifier: u16,
    pub mode: pldmfw_update_mode,
}

// Main entry point to the PLDM firmware update engine. Device drivers should
// embed this in a private structure and use container_of to obtain a pointer
// to their own data, used to implement the device specific operations.

unsafe extern "C" {
    pub fn pldmfw_op_pci_match_record(context: *mut pldmfw, record: *mut pldmfw_record) -> bool;
    pub fn pldmfw_flash_image(context: *mut pldmfw, fw: *const firmware) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
