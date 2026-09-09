// SPDX-License-Identifier: GPL-2.0-only
// Copyright 2022 Google LLC
// Author: Ard Biesheuvel <ardb@google.com>

// Dependencies supplied by the EFI stub environment are intentionally left external.

use core::ffi::c_void;

pub type EfiSmbiosProtocolT = EfiSmbiosProtocol;

#[repr(C)]
pub struct EfiSmbiosRecord {
    pub type_: u8,
    pub length: u8,
    pub handle: u16,
}

#[repr(C)]
pub union EfiSmbiosProtocol {
    pub protocol: EfiSmbiosProtocolMethods,
    pub mixed_mode: EfiSmbiosProtocolMixedMode,
}

#[repr(C)]
pub struct EfiSmbiosProtocolMethods {
    pub add: Option<unsafe extern "efiapi" fn(
        *mut EfiSmbiosProtocolT,
        efi_handle_t,
        *mut u16,
        *mut EfiSmbiosRecord,
    ) -> efi_status_t>,
    pub update_string: Option<unsafe extern "efiapi" fn(
        *mut EfiSmbiosProtocolT,
        *mut u16,
        *mut c_ulong,
        *mut u8,
    ) -> efi_status_t>,
    pub remove: Option<unsafe extern "efiapi" fn(*mut EfiSmbiosProtocolT, u16) -> efi_status_t>,
    pub get_next: Option<unsafe extern "efiapi" fn(
        *mut EfiSmbiosProtocolT,
        *mut u16,
        *mut u8,
        *mut *mut EfiSmbiosRecord,
        efi_handle_t,
    ) -> efi_status_t>,
    pub major_version: u8,
    pub minor_version: u8,
}

#[repr(C)]
pub struct EfiSmbiosProtocolMixedMode {
    pub add: u32,
    pub update_string: u32,
    pub remove: u32,
    pub get_next: u32,
    pub major_version: u8,
    pub minor_version: u8,
}

pub unsafe fn efi_get_smbios_record(type_: u8) -> *const EfiSmbiosRecord {
    let mut record: *mut EfiSmbiosRecord = core::ptr::null_mut();
    let mut smbios: *mut EfiSmbiosProtocolT = core::ptr::null_mut();
    let mut handle: u16 = 0xfffe;

    let status = {
        let status = efi_bs_call!(locate_protocol, &EFI_SMBIOS_PROTOCOL_GUID, core::ptr::null_mut(),
                                  &mut smbios as *mut _ as *mut *mut c_void);
        if status != EFI_SUCCESS {
            status
        } else {
            efi_call_proto!(smbios, get_next, &mut handle, &type_, &mut record,
                            core::ptr::null_mut())
        }
    };
    if status != EFI_SUCCESS {
        return core::ptr::null();
    }
    record as *const EfiSmbiosRecord
}

pub unsafe fn __efi_get_smbios_string(
    record: *const EfiSmbiosRecord,
    offset: *const u8,
) -> *const u8 {
    if record.is_null() {
        return core::ptr::null();
    }

    let mut strtable = (record as *const u8).add((*record).length as usize);
    let mut i = 1;
    while i < *offset {
        let len = strlen(strtable);

        if len == 0 {
            return core::ptr::null();
        }
        strtable = strtable.add(len + 1);
        i += 1;
    }
    strtable
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
