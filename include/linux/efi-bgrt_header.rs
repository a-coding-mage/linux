/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/acpi.h supplies `acpi_table_header` and `acpi_table_bgrt`.

#[cfg(CONFIG_ACPI_BGRT)]
extern "C" {
    pub fn efi_bgrt_init(table: *mut acpi_table_header);
    pub fn acpi_parse_bgrt(table: *mut acpi_table_header) -> core::ffi::c_int;

    /* The BGRT data itself; only valid if bgrt_image != NULL. */
    pub static mut bgrt_image_size: usize;
    pub static mut bgrt_tab: acpi_table_bgrt;
}

#[cfg(not(CONFIG_ACPI_BGRT))]
pub unsafe fn efi_bgrt_init(_table: *mut acpi_table_header) {}

#[cfg(not(CONFIG_ACPI_BGRT))]
pub unsafe fn acpi_parse_bgrt(_table: *mut acpi_table_header) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
