// SPDX-License-Identifier: MIT
// Copyright © 2014 Intel Corporation

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct drm_audio_component {
    /// DRM device, used as parameter for ops
    pub dev: *mut device,
    /// Ops implemented by DRM driver, called by hda driver
    pub ops: *const drm_audio_component_ops,
    /// Ops implemented by hda driver, called by DRM driver
    pub audio_ops: *const drm_audio_component_audio_ops,
    /// completion held during component master binding
    pub master_bind_complete: completion,
}

#[repr(C)]
pub struct drm_audio_component_ops {
    /// drm module to pin down
    pub owner: *mut module,
    /// Get the POWER_DOMAIN_AUDIO power well.
    ///
    /// Request the power well to be turned on.
    ///
    /// Returns a wakeref cookie to be passed back to the corresponding
    /// call to `put_power`.
    pub get_power: Option<unsafe extern "C" fn(*mut device) -> libc::c_ulong>,
    /// Allow the POWER_DOMAIN_AUDIO power well to be turned off.
    pub put_power: Option<unsafe extern "C" fn(*mut device, libc::c_ulong)>,
    /// Enable/disable codec wake signal.
    pub codec_wake_override: Option<unsafe extern "C" fn(*mut device, bool)>,
    /// Get the Core Display Clock in kHz.
    pub get_cdclk_freq: Option<unsafe extern "C" fn(*mut device) -> libc::c_int>,
    /// Set n/cts based on the sample rate.
    ///
    /// Called from audio driver. After audio driver sets the
    /// sample rate, it will call this function to set n/cts.
    pub sync_audio_rate:
        Option<unsafe extern "C" fn(*mut device, libc::c_int, libc::c_int, libc::c_int) -> libc::c_int>,
    /// Fill the audio state and ELD bytes for the given port.
    ///
    /// Called from audio driver to get the HDMI/DP audio state of the given
    /// digital port, and also fetch ELD bytes to the given pointer.
    ///
    /// It returns the byte size of the original ELD (not the actually
    /// copied size), zero for an invalid ELD, or a negative error code.
    ///
    /// Note that the returned size may be over `max_bytes`. Then it
    /// implies that only a part of ELD has been copied to the buffer.
    pub get_eld: Option<unsafe extern "C" fn(
        *mut device,
        libc::c_int,
        libc::c_int,
        *mut bool,
        *mut libc::c_uchar,
        libc::c_int,
    ) -> libc::c_int>,
}

#[repr(C)]
pub struct drm_audio_component_audio_ops {
    /// Pointer to be used in call to pin_eld_notify.
    pub audio_ptr: *mut libc::c_void,
    /// Notify the HDA driver that pin sense and/or ELD information has changed.
    ///
    /// Called when the DRM driver has set up audio pipeline or has just
    /// begun to tear it down. This allows the HDA driver to update its
    /// status accordingly (even when the HDA controller is in power save
    /// mode).
    pub pin_eld_notify: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, libc::c_int)>,
    /// Check and convert from pin node to port number.
    ///
    /// Called by HDA driver to check and convert from the pin widget node
    /// number to a port number in the graphics side.
    pub pin2port: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int>,
    /// (Optional) component master bind callback.
    ///
    /// Called at binding master component, for HDA codec-specific
    /// handling of dynamic binding.
    pub master_bind:
        Option<unsafe extern "C" fn(*mut device, *mut drm_audio_component) -> libc::c_int>,
    /// (Optional) component master unbind callback.
    ///
    /// Called at unbinding master component, for HDA codec-specific
    /// handling of dynamic unbinding.
    pub master_unbind: Option<unsafe extern "C" fn(*mut device, *mut drm_audio_component)>,
}

// Forward declarations and types supplied by included kernel headers:
// `device`, `module`, and `completion`.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
