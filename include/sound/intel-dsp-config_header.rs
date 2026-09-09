/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  intel-dsp-config.h - Intel DSP config
 *
 *  Copyright (c) 2019 Jaroslav Kysela <perex@perex.cz>
 */

// C forward declaration: struct pci_dev;
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub const SND_INTEL_DSP_DRIVER_ANY: i32 = 0;
pub const SND_INTEL_DSP_DRIVER_LEGACY: i32 = 1;
pub const SND_INTEL_DSP_DRIVER_SST: i32 = 2;
pub const SND_INTEL_DSP_DRIVER_SOF: i32 = 3;
pub const SND_INTEL_DSP_DRIVER_AVS: i32 = 4;
pub const SND_INTEL_DSP_DRIVER_LAST: i32 = SND_INTEL_DSP_DRIVER_AVS;

// The C condition is controlled by CONFIG_SND_INTEL_DSP_CONFIG at build time.
// When enabled, these are declaration-only external functions.
#[cfg(feature = "CONFIG_SND_INTEL_DSP_CONFIG")]
unsafe extern "C" {
    pub fn snd_intel_dsp_driver_probe(pci: *mut pci_dev) -> i32;
    pub fn snd_intel_acpi_dsp_driver_probe(
        dev: *mut device,
        acpi_hid: *const u8,
    ) -> i32;
}

// When CONFIG_SND_INTEL_DSP_CONFIG is disabled, the C header provides these
// static inline fallbacks. ACPI_ID_LEN is supplied by the surrounding build.
#[cfg(not(feature = "CONFIG_SND_INTEL_DSP_CONFIG"))]
#[inline]
pub unsafe fn snd_intel_dsp_driver_probe(_pci: *mut pci_dev) -> i32 {
    SND_INTEL_DSP_DRIVER_ANY
}

#[cfg(not(feature = "CONFIG_SND_INTEL_DSP_CONFIG"))]
#[inline]
pub unsafe fn snd_intel_acpi_dsp_driver_probe(
    _dev: *mut device,
    _acpi_hid: *const u8,
) -> i32 {
    SND_INTEL_DSP_DRIVER_ANY
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
