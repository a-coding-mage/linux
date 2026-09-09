/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations corresponding to the Linux list and DMI headers
// are supplied by the surrounding translation unit.

pub const EFI_EMBEDDED_FW_PREFIX_LEN: usize = 8;

/*
 * This struct is private to the efi-embedded fw implementation.
 * They are in this header for use by lib/test_firmware.c only!
 */
#[repr(C)]
pub struct efi_embedded_fw {
    pub list: list_head,
    pub name: *const std::ffi::c_char,
    pub data: *const u8,
    pub length: usize,
}

/**
 * struct efi_embedded_fw_desc - This struct is used by the EFI embedded-fw
 *                               code to search for embedded firmwares.
 *
 * @name:   Name to register the firmware with if found
 * @prefix: First 8 bytes of the firmware
 * @length: Length of the firmware in bytes including prefix
 * @sha256: SHA256 of the firmware
 */
#[repr(C)]
pub struct efi_embedded_fw_desc {
    pub name: *const std::ffi::c_char,
    pub prefix: [u8; EFI_EMBEDDED_FW_PREFIX_LEN],
    pub length: u32,
    pub sha256: [u8; 32],
}

unsafe extern "C" {
    pub static touchscreen_dmi_table: [dmi_system_id; 0];

    pub fn efi_get_embedded_fw(
        name: *const std::ffi::c_char,
        dat: *mut *const u8,
        sz: *mut usize,
    ) -> std::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
