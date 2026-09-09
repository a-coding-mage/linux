/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_ACPI selects whether the external ACPI reboot implementation is used.
#[cfg(feature = "CONFIG_ACPI")]
unsafe extern "C" {
    pub fn acpi_reboot();
}

#[cfg(not(feature = "CONFIG_ACPI"))]
pub unsafe fn acpi_reboot() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
