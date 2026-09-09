/* SPDX-License-Identifier: MIT */

// Translated from drm_hdmi_audio_helper.h.
// Dependency: linux/types.h supplies the C `u64` and `bool` types.

#[repr(C)]
pub struct drm_connector {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_connector_hdmi_audio_funcs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    pub fn drm_connector_hdmi_audio_init(
        connector: *mut drm_connector,
        hdmi_codec_dev: *mut device,
        funcs: *const drm_connector_hdmi_audio_funcs,
        max_i2s_playback_channels: u32,
        i2s_formats: u64,
        spdif_playback: bool,
        sound_dai_port: i32,
    ) -> i32;

    pub fn drm_connector_hdmi_audio_plugged_notify(
        connector: *mut drm_connector,
        plugged: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
