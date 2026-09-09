// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the corresponding ACPI and architecture headers.

unsafe extern "C" {
    fn acpi_agdi_init();
    fn acpi_apmt_init();
    fn acpi_iort_init();
    fn acpi_amba_init();
}

pub unsafe fn acpi_arch_init() {
    // Equivalent to IS_ENABLED(CONFIG_ACPI_AGDI).
    if cfg!(feature = "CONFIG_ACPI_AGDI") {
        unsafe { acpi_agdi_init() };
    }
    // Equivalent to IS_ENABLED(CONFIG_ACPI_APMT).
    if cfg!(feature = "CONFIG_ACPI_APMT") {
        unsafe { acpi_apmt_init() };
    }
    // Equivalent to IS_ENABLED(CONFIG_ACPI_IORT).
    if cfg!(feature = "CONFIG_ACPI_IORT") {
        unsafe { acpi_iort_init() };
    }
    // Equivalent to IS_ENABLED(CONFIG_ARM_AMBA).
    if cfg!(feature = "CONFIG_ARM_AMBA") {
        unsafe { acpi_amba_init() };
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
