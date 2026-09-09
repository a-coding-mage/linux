/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_void};

macro_rules! DMI_MATCH {
    ($a:expr, $b:expr) => {
        dmi_strmatch {
            slot: $a,
            substr: $b,
            exact_match: 0,
        }
    };
}

macro_rules! DMI_EXACT_MATCH {
    ($a:expr, $b:expr) => {
        dmi_strmatch {
            slot: $a,
            substr: $b,
            exact_match: 1,
        }
    };
}

/* dmi */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmi_field {
    DMI_NONE,
    DMI_BIOS_VENDOR,
    DMI_BIOS_VERSION,
    DMI_BIOS_DATE,
    DMI_BIOS_RELEASE,
    DMI_EC_FIRMWARE_RELEASE,
    DMI_SYS_VENDOR,
    DMI_PRODUCT_NAME,
    DMI_PRODUCT_VERSION,
    DMI_PRODUCT_SERIAL,
    DMI_PRODUCT_UUID,
    DMI_PRODUCT_SKU,
    DMI_PRODUCT_FAMILY,
    DMI_BOARD_VENDOR,
    DMI_BOARD_NAME,
    DMI_BOARD_VERSION,
    DMI_BOARD_SERIAL,
    DMI_BOARD_ASSET_TAG,
    DMI_CHASSIS_VENDOR,
    DMI_CHASSIS_TYPE,
    DMI_CHASSIS_VERSION,
    DMI_CHASSIS_SERIAL,
    DMI_CHASSIS_ASSET_TAG,
    DMI_STRING_MAX,
    DMI_OEM_STRING, /* special case - will not be in dmi_ident */
}

#[repr(C)]
pub struct dmi_strmatch {
    /* C declares slot:7 and exact_match:1 in one unsigned-char bitfield. */
    pub slot: u8,
    pub exact_match: u8,
    pub substr: [c_char; 79],
}

#[repr(C)]
pub struct dmi_system_id {
    pub callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> c_int>,
    pub ident: *const c_char,
    pub matches: [dmi_strmatch; 4],
    pub driver_data: *mut c_void,
}

/*
 * struct dmi_device_id appears during expansion of
 * "MODULE_DEVICE_TABLE(dmi, x)". Compiler doesn't look inside it
 * but this is enough for gcc 3.4.6 to error out:
 *	error: storage size of '__mod_dmi_device_table' isn't known
 */
pub type dmi_device_id = dmi_system_id;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
