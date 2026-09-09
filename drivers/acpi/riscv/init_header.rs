/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency intent: declarations originate from <linux/init.h>.

extern "C" {
    pub fn riscv_acpi_init_gsi_mapping();
    pub fn riscv_acpi_rimt_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
