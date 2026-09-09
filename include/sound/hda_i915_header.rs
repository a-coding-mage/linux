/* SPDX-License-Identifier: GPL-2.0 */
/*
 * HD-Audio helpers to sync with i915 driver
 */

// Dependency supplied by hda_component.h.
#[repr(C)]
pub struct hdac_bus {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SND_HDA_I915")]
unsafe extern "C" {
    pub fn snd_hdac_i915_set_bclk(bus: *mut hdac_bus);
    pub fn snd_hdac_i915_init(bus: *mut hdac_bus) -> i32;
}

#[cfg(not(feature = "CONFIG_SND_HDA_I915"))]
#[inline]
pub unsafe fn snd_hdac_i915_set_bclk(_bus: *mut hdac_bus) {}

#[cfg(not(feature = "CONFIG_SND_HDA_I915"))]
#[inline]
pub unsafe fn snd_hdac_i915_init(_bus: *mut hdac_bus) -> i32 {
    -19 // -ENODEV
}

unsafe extern "C" {
    fn snd_hdac_acomp_exit(bus: *mut hdac_bus) -> i32;
}

#[inline]
pub unsafe fn snd_hdac_i915_exit(bus: *mut hdac_bus) -> i32 {
    snd_hdac_acomp_exit(bus)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
