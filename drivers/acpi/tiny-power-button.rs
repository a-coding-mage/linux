// SPDX-License-Identifier: GPL-2.0-or-later
//
// Dependencies supplied by the Linux kernel and ACPI headers are referenced
// here but are not implemented in this translation unit.

use core::ffi::c_void;

extern "C" {
    fn kill_cad_pid(signal: i32, priv_: i32);
    fn acpi_os_execute(
        typ: u32,
        function: unsafe extern "C" fn(*mut c_void),
        context: *mut c_void,
    ) -> u32;
    fn acpi_install_fixed_event_handler(
        event: u32,
        handler: unsafe extern "C" fn(*mut c_void) -> u32,
        context: *mut c_void,
    ) -> u32;
    fn acpi_install_notify_handler(
        handle: *mut c_void,
        typ: u32,
        handler: unsafe extern "C" fn(*mut c_void, u32, *mut c_void),
        context: *mut c_void,
    ) -> u32;
    fn acpi_remove_fixed_event_handler(event: u32, handler: unsafe extern "C" fn(*mut c_void) -> u32);
    fn acpi_remove_notify_handler(
        handle: *mut c_void,
        typ: u32,
        handler: unsafe extern "C" fn(*mut c_void, u32, *mut c_void),
    );
    fn acpi_os_wait_events_complete();
}

const CONFIG_ACPI_TINY_POWER_BUTTON_SIGNAL: i32 = 0;
const ACPI_BUTTON_HID_POWER: &str = "PNP0C0C";
const ACPI_BUTTON_HID_POWERF: &str = "ACPI_FPB";
const ACPI_FIXED_HARDWARE_EVENT: u32 = 0;
const OSL_NOTIFY_HANDLER: u32 = 0;
const ACPI_INTERRUPT_HANDLED: u32 = 0;
const ACPI_EVENT_POWER_BUTTON: u32 = 0;
const ACPI_DEVICE_NOTIFY: u32 = 0;
const ACPI_BUS_TYPE_POWER_BUTTON: u32 = 0;
const ENODEV: i32 = 19;

#[repr(C)]
pub struct acpi_device_id {
    pub id: *const u8,
    pub driver_data: usize,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct acpi_device {
    pub device_type: u32,
    pub handle: *mut c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const u8,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

// The kernel's ACPI companion lookup supplies this relationship.
extern "C" {
    fn ACPI_COMPANION(dev: *mut device) -> *mut acpi_device;
}

#[no_mangle]
pub static mut power_signal: i32 = CONFIG_ACPI_TINY_POWER_BUTTON_SIGNAL;

#[no_mangle]
pub unsafe extern "C" fn acpi_tiny_power_button_notify(
    _handle: *mut c_void,
    _event: u32,
    _data: *mut c_void,
) {
    kill_cad_pid(power_signal, 1);
}

#[no_mangle]
pub unsafe extern "C" fn acpi_tiny_power_button_notify_run(_not_used: *mut c_void) {
    acpi_tiny_power_button_notify(core::ptr::null_mut(), ACPI_FIXED_HARDWARE_EVENT, core::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn acpi_tiny_power_button_event(_not_used: *mut c_void) -> u32 {
    acpi_os_execute(
        OSL_NOTIFY_HANDLER,
        acpi_tiny_power_button_notify_run,
        core::ptr::null_mut(),
    );
    ACPI_INTERRUPT_HANDLED
}

#[no_mangle]
pub unsafe extern "C" fn acpi_tiny_power_button_probe(pdev: *mut platform_device) -> i32 {
    let device = ACPI_COMPANION(&mut (*pdev).dev);
    if device.is_null() {
        return -ENODEV;
    }

    let status;
    if (*device).device_type == ACPI_BUS_TYPE_POWER_BUTTON {
        status = acpi_install_fixed_event_handler(
            ACPI_EVENT_POWER_BUTTON,
            acpi_tiny_power_button_event,
            core::ptr::null_mut(),
        );
    } else {
        status = acpi_install_notify_handler(
            (*device).handle,
            ACPI_DEVICE_NOTIFY,
            acpi_tiny_power_button_notify,
            core::ptr::null_mut(),
        );
    }
    if status != 0 {
        return -ENODEV;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn acpi_tiny_power_button_remove(pdev: *mut platform_device) {
    let device = ACPI_COMPANION(&mut (*pdev).dev);

    if (*device).device_type == ACPI_BUS_TYPE_POWER_BUTTON {
        acpi_remove_fixed_event_handler(ACPI_EVENT_POWER_BUTTON, acpi_tiny_power_button_event);
    } else {
        acpi_remove_notify_handler((*device).handle, ACPI_DEVICE_NOTIFY, acpi_tiny_power_button_notify);
    }
    acpi_os_wait_events_complete();
}

#[no_mangle]
pub static mut acpi_tiny_power_button_driver: platform_driver = platform_driver {
    probe: Some(acpi_tiny_power_button_probe),
    remove: Some(acpi_tiny_power_button_remove),
    driver: device_driver {
        name: b"acpi-tiny-power-button\0".as_ptr(),
        acpi_match_table: core::ptr::null(),
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
