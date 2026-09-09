// SPDX-License-Identifier: GPL-2.0-only
/*
 * Author: Sudeep Holla <sudeep.holla@arm.com>
 * Copyright 2022 Arm Limited
 */

// Translated dependencies supplied by the surrounding kernel bindings:
// linux/kernel.h, linux/acpi.h, linux/completion.h, linux/idr.h, linux/io.h

static mut ffh_ctx: acpi_ffh_info = unsafe { core::mem::zeroed() };

#[no_mangle]
pub unsafe extern "C" fn acpi_ffh_address_space_arch_setup(
    _handler_ctxt: *mut core::ffi::c_void,
    _region_ctxt: *mut *mut core::ffi::c_void,
) -> i32 {
    -EOPNOTSUPP
}

#[no_mangle]
pub unsafe extern "C" fn acpi_ffh_address_space_arch_handler(
    _value: *mut acpi_integer,
    _region_context: *mut core::ffi::c_void,
) -> i32 {
    -EOPNOTSUPP
}

unsafe extern "C" fn acpi_ffh_address_space_setup(
    _region_handle: acpi_handle,
    _function: u32,
    handler_context: *mut core::ffi::c_void,
    region_context: *mut *mut core::ffi::c_void,
) -> acpi_status {
    acpi_ffh_address_space_arch_setup(handler_context, region_context)
}

unsafe extern "C" fn acpi_ffh_address_space_handler(
    _function: u32,
    _addr: acpi_physical_address,
    _bits: u32,
    value: *mut acpi_integer,
    _handler_context: *mut core::ffi::c_void,
    region_context: *mut core::ffi::c_void,
) -> acpi_status {
    acpi_ffh_address_space_arch_handler(value, region_context)
}

pub unsafe extern "C" fn acpi_init_ffh() {
    let status: acpi_status;

    status = acpi_install_address_space_handler(
        ACPI_ROOT_OBJECT,
        ACPI_ADR_SPACE_FIXED_HARDWARE,
        Some(acpi_ffh_address_space_handler),
        Some(acpi_ffh_address_space_setup),
        core::ptr::addr_of_mut!(ffh_ctx).cast(),
    );
    if ACPI_FAILURE(status) {
        pr_alert!("OperationRegion handler could not be installed\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
