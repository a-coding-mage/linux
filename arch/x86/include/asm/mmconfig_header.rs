/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_PCI_MMCONFIG is a build-time configuration condition from the C header.
#[cfg(CONFIG_PCI_MMCONFIG)]
extern "C" {
    pub fn fam10h_check_enable_mmcfg();
    pub fn check_enable_amd_mmconf_dmi();
}

#[cfg(not(CONFIG_PCI_MMCONFIG))]
#[inline]
fn fam10h_check_enable_mmcfg() {}

#[cfg(not(CONFIG_PCI_MMCONFIG))]
#[inline]
fn check_enable_amd_mmconf_dmi() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
