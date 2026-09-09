/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Red Hat, Inc.
 * Copyright (C) 2012 Jeremy Kerr <jeremy.kerr@canonical.com>
 */

/* linux/efi.h is supplied by the surrounding translation. */

#[repr(C)]
pub struct efivarfs_mount_opts {
    pub uid: kuid_t,
    pub gid: kgid_t,
}

#[repr(C)]
pub struct efivarfs_fs_info {
    pub mount_opts: efivarfs_mount_opts,
    pub sb: *mut super_block,
    pub nb: notifier_block,
}

#[repr(C)]
pub struct efi_variable {
    pub VariableName: [efi_char16_t; EFI_VAR_NAME_LEN / core::mem::size_of::<efi_char16_t>()],
    pub VendorGuid: efi_guid_t,
}

#[repr(C)]
pub struct efivar_entry {
    pub var: efi_variable,
    pub vfs_inode: inode,
    pub open_count: ::core::ffi::c_ulong,
    pub removed: bool,
}

#[inline]
pub unsafe fn efivar_entry(inode: *mut inode) -> *mut efivar_entry {
    (inode as *mut u8).sub(core::mem::offset_of!(efivar_entry, vfs_inode)) as *mut efivar_entry
}

extern "C" {
    pub fn efivar_init(
        func: Option<unsafe extern "C" fn(*mut efi_char16_t, efi_guid_t, ::core::ffi::c_ulong, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        data: *mut ::core::ffi::c_void,
        duplicate_check: bool,
    ) -> ::core::ffi::c_int;

    pub fn efivar_entry_delete(entry: *mut efivar_entry) -> ::core::ffi::c_int;

    pub fn efivar_entry_size(entry: *mut efivar_entry, size: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn __efivar_entry_get(
        entry: *mut efivar_entry,
        attributes: *mut u32,
        size: *mut ::core::ffi::c_ulong,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn efivar_entry_get(
        entry: *mut efivar_entry,
        attributes: *mut u32,
        size: *mut ::core::ffi::c_ulong,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn efivar_entry_set_get_size(
        entry: *mut efivar_entry,
        attributes: u32,
        size: *mut ::core::ffi::c_ulong,
        data: *mut ::core::ffi::c_void,
        set: *mut bool,
    ) -> ::core::ffi::c_int;

    pub fn efivar_validate(
        vendor: efi_guid_t,
        var_name: *mut efi_char16_t,
        data: *mut u8,
        data_size: ::core::ffi::c_ulong,
    ) -> bool;
    pub fn efivar_variable_is_removable(
        vendor: efi_guid_t,
        name: *const ::core::ffi::c_char,
        len: usize,
    ) -> bool;
    pub fn efivar_get_utf8name(name16: *const efi_char16_t, vendor: *mut efi_guid_t) -> *mut ::core::ffi::c_char;
    pub fn efivarfs_variable_is_present(
        variable_name: *mut efi_char16_t,
        vendor: *mut efi_guid_t,
        data: *mut ::core::ffi::c_void,
    ) -> bool;

    pub static efivarfs_file_operations: file_operations;
    pub static efivarfs_dir_inode_operations: inode_operations;
    pub fn efivarfs_get_inode(
        sb: *mut super_block,
        dir: *const inode,
        mode: ::core::ffi::c_int,
        dev: dev_t,
        is_removable: bool,
    ) -> *mut inode;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
