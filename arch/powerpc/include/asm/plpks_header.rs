/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022 IBM Corporation
 * Author: Nayna Jain <nayna@linux.ibm.com>
 *
 * Platform keystore for pseries LPAR(PLPKS).
 */

/* CONFIG_PSERIES_PLPKS is a build-time configuration condition. */

#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_OSSECBOOTAUDIT: u32 = 1u32 << (31 - 1); // OS secure boot must be audit/enforce
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_OSSECBOOTENFORCE: u32 = 1u32 << (31 - 2); // OS secure boot must be enforce
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_PWSET: u32 = 1u32 << (31 - 3); // No access without password set
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_WORLDREADABLE: u32 = 1u32 << (31 - 4); // Readable without authentication
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_IMMUTABLE: u32 = 1u32 << (31 - 5); // Once written, object cannot be removed
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_TRANSIENT: u32 = 1u32 << (31 - 6); // Object does not persist through reboot
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_SIGNEDUPDATE: u32 = 1u32 << (31 - 7); // Object can only be modified by signed updates
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_WRAPPINGKEY: u32 = 1u32 << (31 - 8); // Object contains a wrapping key
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_HVPROVISIONED: u32 = 1u32 << (31 - 28); // Hypervisor has provisioned this object

#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_ALG_RSA2048: u64 = 1u64 << (63 - 0);
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_ALG_RSA4096: u64 = 1u64 << (63 - 1);

#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_VAR_LINUX: u8 = 0x02;
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_VAR_COMMON: u8 = 0x04;

#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_FW_OWNER: u8 = 0x1;
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_BOOTLOADER_OWNER: u8 = 0x2;
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_OS_OWNER: u8 = 0x3;

#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_LABEL_VERSION: u16 = 0;
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_MAX_LABEL_ATTR_SIZE: u16 = 16;
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_MAX_NAME_SIZE: u16 = 239;
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_MAX_DATA_SIZE: u16 = 4000;

#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_MAX_TIMEOUT: u64 = 5 * USEC_PER_SEC;
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub const PLPKS_FLUSH_SLEEP: u32 = 10000; // usec

#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
#[repr(C)]
pub struct plpks_var {
    pub component: *mut core::ffi::c_char,
    pub name: *mut u8,
    pub data: *mut u8,
    pub policy: u32,
    pub namelen: u16,
    pub datalen: u16,
    pub os: u8,
}

#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
#[repr(C)]
pub struct plpks_var_name {
    pub name: *mut u8,
    pub namelen: u16,
}

#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
#[repr(C)]
pub struct plpks_var_name_list {
    pub varcount: u32,
    pub varlist: [plpks_var_name; 0],
}

#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
extern "C" {
    pub fn plpks_signed_update_var(var: *mut plpks_var, flags: u64) -> i32;
    pub fn plpks_write_var(var: plpks_var) -> i32;
    pub fn plpks_remove_var(component: *mut core::ffi::c_char, varos: u8, vname: plpks_var_name) -> i32;
    pub fn plpks_read_os_var(var: *mut plpks_var) -> i32;
    pub fn plpks_read_fw_var(var: *mut plpks_var) -> i32;
    pub fn plpks_read_bootloader_var(var: *mut plpks_var) -> i32;
    pub fn plpks_is_available() -> bool;
    pub fn plpks_get_version() -> u8;
    pub fn plpks_get_objoverhead() -> u16;
    pub fn plpks_get_maxpwsize() -> u16;
    pub fn plpks_get_maxobjectsize() -> u16;
    pub fn plpks_get_maxobjectlabelsize() -> u16;
    pub fn plpks_get_totalsize() -> u32;
    pub fn plpks_get_usedspace() -> u32;
    pub fn plpks_get_supportedpolicies() -> u32;
    pub fn plpks_get_maxlargeobjectsize() -> u32;
    pub fn plpks_get_signedupdatealgorithms() -> u64;
    pub fn plpks_get_wrappingfeatures() -> u64;
    pub fn plpks_get_passwordlen() -> u16;
    pub fn plpks_early_init_devtree();
    pub fn plpks_populate_fdt(fdt: *mut core::ffi::c_void) -> i32;
    pub fn plpks_config_create_softlink(from: *mut kobject) -> i32;
    pub fn plpks_wrapping_is_supported() -> bool;
    pub fn plpks_gen_wrapping_key() -> i32;
    pub fn plpks_wrap_object(input_buf: *mut *mut u8, input_len: u32, wrap_flags: u16, output_buf: *mut *mut u8, output_len: *mut u32) -> i32;
    pub fn plpks_unwrap_object(input_buf: *mut *mut u8, input_len: u32, output_buf: *mut *mut u8, output_len: *mut u32) -> i32;
}

#[cfg(not(feature = "CONFIG_PSERIES_PLPKS"))]
pub fn plpks_is_available() -> bool { false }
#[cfg(not(feature = "CONFIG_PSERIES_PLPKS"))]
pub fn plpks_get_passwordlen() -> u16 { panic!("BUILD_BUG") }
#[cfg(not(feature = "CONFIG_PSERIES_PLPKS"))]
pub fn plpks_early_init_devtree() {}
#[cfg(not(feature = "CONFIG_PSERIES_PLPKS"))]
pub fn plpks_populate_fdt(_fdt: *mut core::ffi::c_void) -> i32 { panic!("BUILD_BUG") }
#[cfg(not(feature = "CONFIG_PSERIES_PLPKS"))]
pub fn plpks_config_create_softlink(_from: *mut kobject) -> i32 { 0 }

/* External dependencies supplied by the including kernel translation. */
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
pub enum kobject {}
#[cfg(feature = "CONFIG_PSERIES_PLPKS")]
extern "C" {
    static USEC_PER_SEC: u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
