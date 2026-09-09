/* SPDX-License-Identifier: GPL-2.0-or-later */
/************************************************************
 * EFI GUID Partition Table
 * Per Intel EFI Specification v1.02
 * http://developer.intel.com/technology/efi/efi.htm
 *
 * By Matt Domsch <Matt_Domsch@dell.com>  Fri Sep 22 22:15:56 CDT 2000
 *   Copyright 2000,2001 Dell Inc.
 ************************************************************/

// Dependencies supplied by the surrounding kernel translation.
pub use crate::{efi_guid_t, __le16, __le32, __le64};

pub const MSDOS_MBR_SIGNATURE: u16 = 0xaa55;
pub const EFI_PMBR_OSTYPE_EFI: u8 = 0xEF;
pub const EFI_PMBR_OSTYPE_EFI_GPT: u8 = 0xEE;

pub const GPT_MBR_PROTECTIVE: i32 = 1;
pub const GPT_MBR_HYBRID: i32 = 2;

pub const GPT_HEADER_SIGNATURE: u64 = 0x5452415020494645;
pub const GPT_HEADER_REVISION_V1: u32 = 0x00010000;
pub const GPT_PRIMARY_PARTITION_TABLE_LBA: i32 = 1;

// EFI_GUID(...) is the GUID constructor supplied by the EFI dependency.
pub const PARTITION_SYSTEM_GUID: efi_guid_t = EFI_GUID!(
    0xC12A7328, 0xF81F, 0x11d2, 0xBA, 0x4B, 0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B
);
pub const LEGACY_MBR_PARTITION_GUID: efi_guid_t = EFI_GUID!(
    0x024DEE41, 0x33E7, 0x11d3, 0x9D, 0x69, 0x00, 0x08, 0xC7, 0x81, 0xF3, 0x9F
);
pub const PARTITION_MSFT_RESERVED_GUID: efi_guid_t = EFI_GUID!(
    0xE3C9E316, 0x0B5C, 0x4DB8, 0x81, 0x7D, 0xF9, 0x2D, 0xF0, 0x02, 0x15, 0xAE
);
pub const PARTITION_BASIC_DATA_GUID: efi_guid_t = EFI_GUID!(
    0xEBD0A0A2, 0xB9E5, 0x4433, 0x87, 0xC0, 0x68, 0xB6, 0xB7, 0x26, 0x99, 0xC7
);
pub const PARTITION_LINUX_RAID_GUID: efi_guid_t = EFI_GUID!(
    0xa19d880f, 0x05fc, 0x4d3b, 0xa0, 0x06, 0x74, 0x3f, 0x0f, 0x84, 0x91, 0x1e
);
pub const PARTITION_LINUX_SWAP_GUID: efi_guid_t = EFI_GUID!(
    0x0657fd6d, 0xa4ab, 0x43c4, 0x84, 0xe5, 0x09, 0x33, 0xc8, 0x4b, 0x4f, 0x4f
);
pub const PARTITION_LINUX_LVM_GUID: efi_guid_t = EFI_GUID!(
    0xe6d6d379, 0xf507, 0x44c2, 0xa2, 0x3c, 0x23, 0x8f, 0x2a, 0x3d, 0xf9, 0x28
);

#[repr(C, packed)]
pub struct gpt_header {
    pub signature: __le64,
    pub revision: __le32,
    pub header_size: __le32,
    pub header_crc32: __le32,
    pub reserved1: __le32,
    pub my_lba: __le64,
    pub alternate_lba: __le64,
    pub first_usable_lba: __le64,
    pub last_usable_lba: __le64,
    pub disk_guid: efi_guid_t,
    pub partition_entry_lba: __le64,
    pub num_partition_entries: __le32,
    pub sizeof_partition_entry: __le32,
    pub partition_entry_array_crc32: __le32,
    // The rest of the logical block is reserved by UEFI and must be zero.
}

#[repr(C, packed)]
pub struct gpt_entry_attributes {
    // C bitfields: required_to_function:1, reserved:47, type_guid_specific:16.
    pub bits: u64,
}

#[repr(C, packed)]
pub struct gpt_entry {
    pub partition_type_guid: efi_guid_t,
    pub unique_partition_guid: efi_guid_t,
    pub starting_lba: __le64,
    pub ending_lba: __le64,
    pub attributes: gpt_entry_attributes,
    pub partition_name: [__le16; 72 / core::mem::size_of::<__le16>()],
}

#[repr(C, packed)]
pub struct gpt_mbr_record {
    pub boot_indicator: u8, // unused by EFI, set to 0x80 for bootable
    pub start_head: u8,     // unused by EFI, pt start in CHS
    pub start_sector: u8,   // unused by EFI, pt start in CHS
    pub start_track: u8,
    pub os_type: u8,        // EFI and legacy non-EFI OS types
    pub end_head: u8,       // unused by EFI, pt end in CHS
    pub end_sector: u8,     // unused by EFI, pt end in CHS
    pub end_track: u8,      // unused by EFI, pt end in CHS
    pub starting_lba: __le32, // used by EFI - start addr of the on disk pt
    pub size_in_lba: __le32,  // used by EFI - size of pt in LBA
}

#[repr(C, packed)]
pub struct legacy_mbr {
    pub boot_code: [u8; 440],
    pub unique_mbr_signature: __le32,
    pub unknown: __le16,
    pub partition_record: [gpt_mbr_record; 4],
    pub signature: __le16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
