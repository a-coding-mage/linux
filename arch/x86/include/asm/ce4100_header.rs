/* SPDX-License-Identifier: GPL-2.0 */

// External declaration corresponding to: int ce4100_pci_init(void);
unsafe extern "C" {
    pub fn ce4100_pci_init() -> core::ffi::c_int;
}

// Build-time condition corresponding to CONFIG_SERIAL_8250.
#[cfg(feature = "CONFIG_SERIAL_8250")]
unsafe extern "C" {
    // The C declaration carries the __init annotation.
    pub fn sdv_serial_fixup();
}

#[cfg(not(feature = "CONFIG_SERIAL_8250"))]
#[inline]
fn sdv_serial_fixup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
