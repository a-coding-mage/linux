// SPDX-License-Identifier: GPL-2.0
//! Native compressed-boot EFI declarations and firmware table layouts.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, missing_docs, unreachable_pub, unused_imports)]

// Use the existing boot-parameter identity in every EFI caller and owner.
pub use crate::bindings::boot_params;

// efi.h defines efi_guid_t as guid_t with u32 alignment. A bindgen type alias
// cannot carry that extra alignment, so blocklist just that typedef and retain
// its exact byte representation here. Generated native table fields and C ABI
// declarations all refer to this same type.
#[repr(C, align(4))]
#[derive(Copy, Clone, Default)]
pub struct efi_guid_t {
    pub b: [u8; 16],
}

include!(concat!(
    env!("LUPOS_BOOT_OBJ"),
    "/boot_efi_bindings_generated.rs"
));

// Exact EFI_GUID expansions from compressed/efi.h, in firmware byte order.
pub const ACPI_TABLE_GUID: efi_guid_t = efi_guid_t {
    b: [
        0x30, 0x2d, 0x9d, 0xeb, 0x88, 0x2d, 0xd3, 0x11, 0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1,
        0x4d,
    ],
};
pub const ACPI_20_TABLE_GUID: efi_guid_t = efi_guid_t {
    b: [
        0x71, 0xe8, 0x68, 0x88, 0xf1, 0xe4, 0xd3, 0x11, 0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88,
        0x81,
    ],
};

// Check the replacement typedef and dependent native layouts under each build's
// C ABI, including i386's u64 alignment. No generated layout-test executable.
const _: () = {
    use core::mem::{align_of, offset_of, size_of};
    assert!(size_of::<efi_guid_t>() == LUPOS_BOOT_EFI_GUID_SIZE as usize);
    assert!(align_of::<efi_guid_t>() == LUPOS_BOOT_EFI_GUID_ALIGN as usize);
    assert!(size_of::<[u8; 16]>() == LUPOS_BOOT_EFI_GUID_BYTES as usize);
    assert!(offset_of!(efi_guid_t, b) == LUPOS_BOOT_EFI_GUID_B_OFFSET as usize);
    assert!(size_of::<efi_config_table_64_t>() == LUPOS_BOOT_EFI_CONFIG64_SIZE as usize);
    assert!(align_of::<efi_config_table_64_t>() == LUPOS_BOOT_EFI_CONFIG64_ALIGN as usize);
    assert!(
        offset_of!(efi_config_table_64_t, table) == LUPOS_BOOT_EFI_CONFIG64_TABLE_OFFSET as usize
    );
    assert!(size_of::<efi_config_table_32_t>() == LUPOS_BOOT_EFI_CONFIG32_SIZE as usize);
    assert!(align_of::<efi_config_table_32_t>() == LUPOS_BOOT_EFI_CONFIG32_ALIGN as usize);
    assert!(
        offset_of!(efi_config_table_32_t, table) == LUPOS_BOOT_EFI_CONFIG32_TABLE_OFFSET as usize
    );
    assert!(size_of::<efi_system_table_64_t>() == LUPOS_BOOT_EFI_SYSTEM64_SIZE as usize);
    assert!(
        offset_of!(efi_system_table_64_t, nr_tables) == LUPOS_BOOT_EFI_SYSTEM64_NR_OFFSET as usize
    );
    assert!(
        offset_of!(efi_system_table_64_t, tables) == LUPOS_BOOT_EFI_SYSTEM64_TABLES_OFFSET as usize
    );
    assert!(size_of::<efi_system_table_32_t>() == LUPOS_BOOT_EFI_SYSTEM32_SIZE as usize);
    assert!(
        offset_of!(efi_system_table_32_t, nr_tables) == LUPOS_BOOT_EFI_SYSTEM32_NR_OFFSET as usize
    );
    assert!(
        offset_of!(efi_system_table_32_t, tables) == LUPOS_BOOT_EFI_SYSTEM32_TABLES_OFFSET as usize
    );
};
