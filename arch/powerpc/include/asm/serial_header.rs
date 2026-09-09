/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Serial ports are not listed here, because they are discovered
 * through the device tree.
 */

/*
 * Provides BASE_BAUD, used as fallback if not found in device tree.
 * Dependency: <asm-generic/serial.h>
 */

/* Build-time condition corresponding to CONFIG_PPC_UDBG_16550. */
#[cfg(CONFIG_PPC_UDBG_16550)]
unsafe extern "C" {
    pub fn find_legacy_serial_ports();
}

/* When CONFIG_PPC_UDBG_16550 is not enabled, the C macro is a no-op. */
#[cfg(not(CONFIG_PPC_UDBG_16550))]
#[inline]
pub fn find_legacy_serial_ports() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
