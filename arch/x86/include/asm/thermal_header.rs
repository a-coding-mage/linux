/* SPDX-License-Identifier: GPL-2.0 */

// The CONFIG_X86_THERMAL_VECTOR build condition is preserved using a Rust
// feature of the same name.
#[cfg(feature = "CONFIG_X86_THERMAL_VECTOR")]
extern "C" {
    pub fn therm_lvt_init();
    pub fn intel_init_thermal(c: *mut cpuinfo_x86);
    pub fn x86_thermal_enabled() -> bool;
    pub fn intel_thermal_interrupt();
}

#[cfg(not(feature = "CONFIG_X86_THERMAL_VECTOR"))]
pub extern "C" fn therm_lvt_init() {}

#[cfg(not(feature = "CONFIG_X86_THERMAL_VECTOR"))]
pub extern "C" fn intel_init_thermal(_c: *mut cpuinfo_x86) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
