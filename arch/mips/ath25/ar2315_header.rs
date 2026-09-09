/* SPDX-License-Identifier: GPL-2.0 */

// The C header conditionally exposes external functions when CONFIG_SOC_AR2315
// is enabled; otherwise it provides empty inline definitions.
#[cfg(feature = "CONFIG_SOC_AR2315")]
extern "C" {
    pub fn ar2315_arch_init_irq();
    pub fn ar2315_init_devices();
    pub fn ar2315_plat_time_init();
    pub fn ar2315_plat_mem_setup();
    pub fn ar2315_arch_init();
}

#[cfg(not(feature = "CONFIG_SOC_AR2315"))]
#[inline]
pub unsafe fn ar2315_arch_init_irq() {}

#[cfg(not(feature = "CONFIG_SOC_AR2315"))]
#[inline]
pub unsafe fn ar2315_init_devices() {}

#[cfg(not(feature = "CONFIG_SOC_AR2315"))]
#[inline]
pub unsafe fn ar2315_plat_time_init() {}

#[cfg(not(feature = "CONFIG_SOC_AR2315"))]
#[inline]
pub unsafe fn ar2315_plat_mem_setup() {}

#[cfg(not(feature = "CONFIG_SOC_AR2315"))]
#[inline]
pub unsafe fn ar2315_arch_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
