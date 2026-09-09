// SPDX-License-Identifier: GPL-2.0
// HD-Audio helpers to sync with DRM driver

// Dependency declarations supplied by the surrounding translation unit are
// intentionally not reproduced here.

/* virtual idx for controller */
pub const HDA_CODEC_IDX_CONTROLLER: u32 = HDA_MAX_CODECS;

// CONFIG_SND_HDA_COMPONENT is a build-time configuration condition.  The
// enabled declarations are preserved below; select the disabled inline
// fallbacks when that configuration is absent.
#[cfg(feature = "CONFIG_SND_HDA_COMPONENT")]
extern "C" {
    pub fn snd_hdac_set_codec_wakeup(bus: *mut hdac_bus, enable: bool) -> i32;
    pub fn snd_hdac_display_power(bus: *mut hdac_bus, idx: u32, enable: bool);
    pub fn snd_hdac_sync_audio_rate(
        codec: *mut hdac_device,
        nid: hda_nid_t,
        dev_id: i32,
        rate: i32,
    ) -> i32;
    pub fn snd_hdac_acomp_get_eld(
        codec: *mut hdac_device,
        nid: hda_nid_t,
        dev_id: i32,
        audio_enabled: *mut bool,
        buffer: *mut std::ffi::c_char,
        max_bytes: i32,
    ) -> i32;
    pub fn snd_hdac_acomp_init(
        bus: *mut hdac_bus,
        aops: *const drm_audio_component_audio_ops,
        match_master: Option<unsafe extern "C" fn(*mut device, i32, *mut std::ffi::c_void) -> i32>,
        extra_size: usize,
    ) -> i32;
    pub fn snd_hdac_acomp_exit(bus: *mut hdac_bus) -> i32;
    pub fn snd_hdac_acomp_register_notifier(
        bus: *mut hdac_bus,
        ops: *const drm_audio_component_audio_ops,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_SND_HDA_COMPONENT"))]
pub unsafe fn snd_hdac_set_codec_wakeup(_bus: *mut hdac_bus, _enable: bool) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SND_HDA_COMPONENT"))]
pub unsafe fn snd_hdac_display_power(_bus: *mut hdac_bus, _idx: u32, _enable: bool) {}

#[cfg(not(feature = "CONFIG_SND_HDA_COMPONENT"))]
pub unsafe fn snd_hdac_sync_audio_rate(
    _codec: *mut hdac_device,
    _nid: hda_nid_t,
    _dev_id: i32,
    _rate: i32,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SND_HDA_COMPONENT"))]
pub unsafe fn snd_hdac_acomp_get_eld(
    _codec: *mut hdac_device,
    _nid: hda_nid_t,
    _dev_id: i32,
    _audio_enabled: *mut bool,
    _buffer: *mut std::ffi::c_char,
    _max_bytes: i32,
) -> i32 {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_SND_HDA_COMPONENT"))]
pub unsafe fn snd_hdac_acomp_init(
    _bus: *mut hdac_bus,
    _aops: *const drm_audio_component_audio_ops,
    _match_master: Option<unsafe extern "C" fn(*mut device, i32, *mut std::ffi::c_void) -> i32>,
    _extra_size: usize,
) -> i32 {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_SND_HDA_COMPONENT"))]
pub unsafe fn snd_hdac_acomp_exit(_bus: *mut hdac_bus) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SND_HDA_COMPONENT"))]
pub unsafe fn snd_hdac_acomp_register_notifier(
    _bus: *mut hdac_bus,
    _ops: *const drm_audio_component_audio_ops,
) -> i32 {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
