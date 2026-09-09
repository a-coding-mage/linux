/* SPDX-License-Identifier: GPL-2.0 */

pub const ACPI_BUTTON_HID_POWER: &str = "PNP0C0C";
pub const ACPI_BUTTON_HID_LID: &str = "PNP0C0D";
pub const ACPI_BUTTON_HID_SLEEP: &str = "PNP0C0E";

// Equivalent to IS_ENABLED(CONFIG_ACPI_BUTTON).
#[cfg(feature = "acpi_button")]
unsafe extern "C" {
    pub fn acpi_lid_open() -> i32;
}

#[cfg(not(feature = "acpi_button"))]
#[inline]
pub fn acpi_lid_open() -> i32 {
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
