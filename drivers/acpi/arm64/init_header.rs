/* SPDX-License-Identifier: GPL-2.0-only */
// Dependency: declarations use the Linux `__init` annotation from <linux/init.h>.

extern "C" {
    pub fn acpi_agdi_init();
    pub fn acpi_apmt_init();
    pub fn acpi_iort_init();
    pub fn acpi_amba_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
