// SPDX-License-Identifier: GPL-2.0-only
/*
 * ACPI support for CMOS RTC Address Space access
 *
 * Copyright (C) 2013, Intel Corporation
 * Authors: Lan Tianyu <tianyu.lan@intel.com>
 */

// Dependencies supplied by the surrounding kernel translation.

static ACPI_CMOS_RTC_IDS: &[AcpiDeviceId] = &[];

static ACPI_CMOS_RTC_IDS_TABLE: &[AcpiDeviceId] = &[
    AcpiDeviceId { id: "ACPI000E", driver_data: 1 }, // ACPI Time and Alarm Device (TAD)
];

static mut CMOS_RTC_PLATFORM_DEVICE_PRESENT: bool = false;

unsafe fn acpi_cmos_rtc_space_handler(
    function: u32,
    mut address: AcpiPhysicalAddress,
    bits: u32,
    value64: *mut u64,
    _handler_context: *mut core::ffi::c_void,
    _region_context: *mut core::ffi::c_void,
) -> AcpiStatus {
    let bytes = ((bits + 8 - 1) / 8) as usize;
    let value = value64 as *mut u8;

    if address > 0xff || value64.is_null() {
        return AE_BAD_PARAMETER;
    }

    // Equivalent of guard(spinlock_irq)(&rtc_lock).
    let _rtc_lock_guard = spinlock_irq_guard(&raw mut rtc_lock);

    if function == ACPI_WRITE {
        for i in 0..bytes {
            CMOS_WRITE(value.add(i).read(), address);
            address += 1;
        }

        return AE_OK;
    }

    if function == ACPI_READ {
        for i in 0..bytes {
            value.add(i).write(CMOS_READ(address));
            address += 1;
        }

        return AE_OK;
    }

    AE_BAD_PARAMETER
}

unsafe fn acpi_install_cmos_rtc_space_handler(handle: AcpiHandle) -> i32 {
    static mut CMOS_RTC_SPACE_HANDLER_PRESENT: bool = false;
    let status: AcpiStatus;

    if CMOS_RTC_SPACE_HANDLER_PRESENT {
        return 0;
    }

    status = acpi_install_address_space_handler(
        handle,
        ACPI_ADR_SPACE_CMOS,
        acpi_cmos_rtc_space_handler,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
    if ACPI_FAILURE(status) {
        pr_err!("Failed to install CMOS-RTC address space handler\n");
        return -ENODEV;
    }

    CMOS_RTC_SPACE_HANDLER_PRESENT = true;

    1
}

unsafe fn acpi_cmos_rtc_attach(
    adev: *mut AcpiDevice,
    id: *const AcpiDeviceId,
) -> i32 {
    let ret = acpi_install_cmos_rtc_space_handler((*adev).handle);
    if ret < 0 {
        return ret;
    }

    let platform_device = acpi_create_platform_device(adev, core::ptr::null_mut());
    if IS_ERR_OR_NULL(platform_device) {
        pr_err!(
            "Failed to create a platform device for %s\n",
            (*id).id.as_ptr() as *const i8
        );
        return 0;
    } else if (*id).driver_data == 0 {
        CMOS_RTC_PLATFORM_DEVICE_PRESENT = true;
    }
    1
}

static mut CMOS_RTC_HANDLER: AcpiScanHandler = AcpiScanHandler {
    ids: ACPI_CMOS_RTC_IDS_TABLE,
    attach: acpi_cmos_rtc_attach,
};

unsafe fn acpi_cmos_rtc_init() {
    acpi_scan_add_handler(&raw mut CMOS_RTC_HANDLER);
}

// External types, constants, globals, functions, and macros are provided by
// the corresponding kernel translations.
extern "C" {
    static mut rtc_lock: Spinlock;
    fn spinlock_irq_guard(lock: *mut Spinlock) -> SpinlockGuard;
    fn acpi_install_address_space_handler(
        handle: AcpiHandle,
        space_id: u32,
        handler: unsafe fn(u32, AcpiPhysicalAddress, u32, *mut u64, *mut core::ffi::c_void, *mut core::ffi::c_void) -> AcpiStatus,
        setup: *mut core::ffi::c_void,
        context: *mut core::ffi::c_void,
    ) -> AcpiStatus;
    fn acpi_create_platform_device(adev: *mut AcpiDevice, properties: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn acpi_scan_add_handler(handler: *mut AcpiScanHandler);
    fn CMOS_WRITE(value: u8, address: AcpiPhysicalAddress);
    fn CMOS_READ(address: AcpiPhysicalAddress) -> u8;
    fn ACPI_FAILURE(status: AcpiStatus) -> bool;
    fn IS_ERR_OR_NULL(ptr: *mut core::ffi::c_void) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
