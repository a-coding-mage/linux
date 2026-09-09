/* SPDX-License-Identifier: GPL-2.0 */

// External types and functions are supplied by the surrounding translation.
unsafe extern "C" {
    fn acpi_enable_wakeup_devices(sleep_state: u8);
    fn acpi_disable_wakeup_devices(sleep_state: u8);
    fn acpi_check_wakeup_handlers() -> bool;

    static mut acpi_wakeup_device_list: list_head;
    static mut acpi_device_lock: mutex;

    fn acpi_resume_power_resources();

    fn acpi_set_firmware_waking_vector(
        wakeup_address: acpi_physical_address,
        vector_width: u32,
    ) -> acpi_status;

    fn acpi_s2idle_begin() -> i32;
    fn acpi_s2idle_prepare() -> i32;
    fn acpi_s2idle_wake() -> bool;
    fn acpi_s2idle_restore();
    fn acpi_s2idle_end();

    fn acpi_s2idle_setup();
}

#[inline]
unsafe fn acpi_set_waking_vector(wakeup_address: u32) -> acpi_status {
    unsafe {
        acpi_set_firmware_waking_vector(
            wakeup_address as acpi_physical_address,
            0,
        )
    }
}

// CONFIG_ACPI_SLEEP controls whether this value is externally supplied.
#[cfg(CONFIG_ACPI_SLEEP)]
unsafe extern "C" {
    static mut acpi_sleep_default_s3: bool;
}

#[cfg(not(CONFIG_ACPI_SLEEP))]
const acpi_sleep_default_s3: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
