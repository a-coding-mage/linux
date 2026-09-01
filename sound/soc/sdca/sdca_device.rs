// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2024 Intel Corporation

/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 */

use core::ffi::{c_char, c_int, c_void};

const ACPI_SIG_SWFT: *const c_char = b"SWFT\0".as_ptr() as *const c_char;
const DMI_SYS_VENDOR: c_int = 3;
const DMI_PRODUCT_SKU: c_int = 8;

#[repr(C)]
pub struct device {
    pub fwnode: *mut fwnode_handle,
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_table_header {
    _private: [u8; 0],
}

pub type acpi_status = u32;

#[repr(C)]
pub struct sdw_slave_id {
    pub mfg_id: u16,
    pub part_id: u16,
}

#[repr(C)]
pub struct sdca_function {
    pub type_: u32,
}

#[repr(C)]
pub struct sdca_data {
    pub interface_revision: u32,
    pub swft: *mut c_void,
    pub num_functions: c_int,
    pub function: *mut sdca_function,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub id: sdw_slave_id,
    pub sdca_data: sdca_data,
}

pub type sdca_quirk = c_int;

extern "C" {
    static SDCA_FUNCTION_TYPE_SMART_MIC: u32;
    static SDCA_QUIRKS_RT712_VB: sdca_quirk;
    static SDCA_QUIRKS_SKIP_FUNC_TYPE_PATCHING: sdca_quirk;

    fn fwnode_property_read_u32(
        fwnode: *mut fwnode_handle,
        propname: *const c_char,
        val: *mut u32,
    ) -> c_int;
    fn acpi_put_table(table: *mut acpi_table_header);
    fn acpi_get_table(
        signature: *const c_char,
        instance: u32,
        out_table: *mut *mut acpi_table_header,
    ) -> acpi_status;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn dmi_get_system_info(field: c_int) -> *const c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

#[inline]
fn ACPI_FAILURE(status: acpi_status) -> bool {
    (status as i32) < 0
}

#[no_mangle]
pub unsafe extern "C" fn sdca_lookup_interface_revision(slave: *mut sdw_slave) {
    let fwnode: *mut fwnode_handle = (*slave).dev.fwnode;

    /*
     * if this property is not present, then the sdca_interface_revision will
     * remain zero, which will be considered as 'not defined' or 'invalid'.
     */
    fwnode_property_read_u32(
        fwnode,
        b"mipi-sdw-sdca-interface-revision\0".as_ptr() as *const c_char,
        &mut (*slave).sdca_data.interface_revision,
    );
}
// EXPORT_SYMBOL_NS(sdca_lookup_interface_revision, "SND_SOC_SDCA");

unsafe extern "C" fn devm_acpi_table_put(ptr: *mut c_void) {
    acpi_put_table(ptr as *mut acpi_table_header);
}

#[no_mangle]
pub unsafe extern "C" fn sdca_lookup_swft(slave: *mut sdw_slave) {
    let status: acpi_status;

    status = acpi_get_table(
        ACPI_SIG_SWFT,
        0,
        &mut (*slave).sdca_data.swft as *mut *mut c_void as *mut *mut acpi_table_header,
    );
    if ACPI_FAILURE(status) {
        dev_info(&mut (*slave).dev, b"SWFT not available\n\0".as_ptr() as *const c_char);
    } else {
        devm_add_action_or_reset(
            &mut (*slave).dev,
            devm_acpi_table_put,
            (*slave).sdca_data.swft,
        );
    }
}
// EXPORT_SYMBOL_NS(sdca_lookup_swft, "SND_SOC_SDCA");

unsafe fn sdca_device_quirk_rt712_vb(slave: *mut sdw_slave) -> bool {
    let id: *mut sdw_slave_id = &mut (*slave).id;
    let mut i: c_int;

    /*
     * The RT712_VA relies on the v06r04 draft, and the
     * RT712_VB on a more recent v08r01 draft.
     */
    if (*slave).sdca_data.interface_revision < 0x0801 {
        return false;
    }

    if (*id).mfg_id != 0x025d {
        return false;
    }

    if (*id).part_id != 0x712
        && (*id).part_id != 0x713
        && (*id).part_id != 0x716
        && (*id).part_id != 0x717
    {
        return false;
    }

    i = 0;
    while i < (*slave).sdca_data.num_functions {
        if (*(*slave).sdca_data.function.offset(i as isize)).type_ == SDCA_FUNCTION_TYPE_SMART_MIC {
            return true;
        }
        i += 1;
    }

    false
}

unsafe fn sdca_device_quirk_skip_func_type_patching(slave: *mut sdw_slave) -> bool {
    let vendor: *const c_char;
    let sku: *const c_char;

    vendor = dmi_get_system_info(DMI_SYS_VENDOR);
    sku = dmi_get_system_info(DMI_PRODUCT_SKU);

    if !vendor.is_null()
        && !sku.is_null()
        && strcmp(vendor, b"Dell Inc.\0".as_ptr() as *const c_char) == 0
        && (strcmp(sku, b"0C62\0".as_ptr() as *const c_char) == 0
            || strcmp(sku, b"0C63\0".as_ptr() as *const c_char) == 0
            || strcmp(sku, b"0C6B\0".as_ptr() as *const c_char) == 0)
        && (*slave).sdca_data.interface_revision == 0x061c
        && (*slave).id.mfg_id == 0x01fa
        && (*slave).id.part_id == 0x4243
    {
        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn sdca_device_quirk_match(
    slave: *mut sdw_slave,
    quirk: sdca_quirk,
) -> bool {
    if quirk == SDCA_QUIRKS_RT712_VB {
        return sdca_device_quirk_rt712_vb(slave);
    }
    if quirk == SDCA_QUIRKS_SKIP_FUNC_TYPE_PATCHING {
        return sdca_device_quirk_skip_func_type_patching(slave);
    }

    false
}
// EXPORT_SYMBOL_NS(sdca_device_quirk_match, "SND_SOC_SDCA");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
