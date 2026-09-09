/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux BCMA subsystem: <linux/bcma/bcma.h>

#[repr(C)]
pub struct bcma_soc {
    pub bus: bcma_bus,
    pub dev: *mut device,
}

// The C __init annotation has no direct Rust equivalent; these are external
// kernel entry points with the same ABI and signatures.
unsafe extern "C" {
    pub fn bcma_host_soc_register(soc: *mut bcma_soc) -> core::ffi::c_int;
    pub fn bcma_host_soc_init(soc: *mut bcma_soc) -> core::ffi::c_int;

    pub fn bcma_bus_register(bus: *mut bcma_bus) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
