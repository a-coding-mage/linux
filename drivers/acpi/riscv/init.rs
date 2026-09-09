// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023-2024, Ventana Micro Systems Inc
 *\tAuthor: Sunil V L <sunilvl@ventanamicro.com>
 */

// Declarations supplied by the included Linux ACPI and local headers.
unsafe extern "C" {
    fn riscv_acpi_init_gsi_mapping();
    fn riscv_acpi_rimt_init();
}

// `__init` is a kernel section annotation in C; preserve the function's
// externally visible entry point here.
pub unsafe extern "C" fn acpi_arch_init() {
    riscv_acpi_init_gsi_mapping();
    // Build-time CONFIG_ACPI_RIMT condition from IS_ENABLED(CONFIG_ACPI_RIMT).
    if cfg!(feature = "CONFIG_ACPI_RIMT") {
        riscv_acpi_rimt_init();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
