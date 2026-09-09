/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 *  User API methods for ACPI-WMI mapping driver
 *
 *  Copyright (C) 2017 Dell, Inc.
 */

/* Dependency intent: this header uses Linux ioctl encoding and fixed-width
 * integer types supplied by the surrounding Linux UAPI bindings.
 */

/* WMI bus will filter all WMI vendor driver requests through this IOC */
pub const WMI_IOC: u32 = b'W' as u32;

/* All ioctl requests through WMI should declare their size followed by
 * relevant data objects
 */
#[repr(C)]
pub struct wmi_ioctl_buffer {
    pub length: u64,
    pub data: [u8; 0],
}

/* This structure may be modified by the firmware when we enter
 * system management mode through SMM, hence the volatiles
 */
#[repr(C, packed)]
pub struct calling_interface_buffer {
    pub cmd_class: u16,
    pub cmd_select: u16,
    pub input: [u32; 4],
    pub output: [u32; 4],
}

#[repr(C, packed)]
pub struct dell_wmi_extensions {
    pub argattrib: u32,
    pub blength: u32,
    pub data: [u8; 0],
}

#[repr(C, packed)]
pub struct dell_wmi_smbios_buffer {
    pub length: u64,
    pub std: calling_interface_buffer,
    pub ext: dell_wmi_extensions,
}

/* Whitelisted smbios class/select commands */
pub const CLASS_TOKEN_READ: u32 = 0;
pub const CLASS_TOKEN_WRITE: u32 = 1;
pub const SELECT_TOKEN_STD: u32 = 0;
pub const SELECT_TOKEN_BAT: u32 = 1;
pub const SELECT_TOKEN_AC: u32 = 2;
pub const CLASS_FLASH_INTERFACE: u32 = 7;
pub const SELECT_FLASH_INTERFACE: u32 = 3;
pub const CLASS_ADMIN_PROP: u32 = 10;
pub const SELECT_ADMIN_PROP: u32 = 3;
pub const CLASS_INFO: u32 = 17;
pub const SELECT_RFKILL: u32 = 11;
pub const SELECT_APP_REGISTRATION: u32 = 3;
pub const SELECT_DOCK: u32 = 22;

/* whitelisted tokens */
pub const CAPSULE_EN_TOKEN: u32 = 0x0461;
pub const CAPSULE_DIS_TOKEN: u32 = 0x0462;
pub const WSMT_EN_TOKEN: u32 = 0x04EC;
pub const WSMT_DIS_TOKEN: u32 = 0x04ED;

/* Dell SMBIOS calling IOCTL command used by dell-smbios-wmi.
 * `_IOWR` is supplied by the surrounding Linux UAPI ioctl bindings.
 */
pub const DELL_WMI_SMBIOS_CMD: _ = _IOWR!(WMI_IOC, 0, dell_wmi_smbios_buffer);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
