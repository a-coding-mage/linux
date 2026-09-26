/* SPDX-License-Identifier: GPL-2.0 */

// The CONFIG_SOC_AR5312 build condition is represented as a Cargo feature.
#[cfg(CONFIG_SOC_AR5312)]
extern "C" {
    pub fn ar5312_arch_init_irq();
    pub fn ar5312_init_devices();
    pub fn ar5312_plat_time_init();
    pub fn ar5312_plat_mem_setup();
    pub fn ar5312_arch_init();
}

#[cfg(not(CONFIG_SOC_AR5312))]
#[inline]
pub fn ar5312_arch_init_irq() {}

#[cfg(not(CONFIG_SOC_AR5312))]
#[inline]
pub fn ar5312_init_devices() {}

#[cfg(not(CONFIG_SOC_AR5312))]
#[inline]
pub fn ar5312_plat_time_init() {}

#[cfg(not(CONFIG_SOC_AR5312))]
#[inline]
pub fn ar5312_plat_mem_setup() {}

#[cfg(not(CONFIG_SOC_AR5312))]
#[inline]
pub fn ar5312_arch_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
