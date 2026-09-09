/* SPDX-License-Identifier: MIT */

// Translated from drm_hdmi_state_helper.h.

#[repr(C)]
pub struct drm_atomic_commit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_connector {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_connector_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_display_mode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdmi_audio_infoframe {
    _private: [u8; 0],
}

pub enum drm_connector_status {}
pub enum drm_mode_status {}

extern "C" {
    pub fn __drm_atomic_helper_connector_hdmi_state_init(
        connector: *mut drm_connector,
        new_conn_state: *mut drm_connector_state,
    );

    pub fn drm_atomic_helper_connector_hdmi_check(
        connector: *mut drm_connector,
        state: *mut drm_atomic_commit,
    ) -> i32;

    pub fn drm_atomic_helper_connector_hdmi_update_audio_infoframe(
        connector: *mut drm_connector,
        frame: *mut hdmi_audio_infoframe,
    ) -> i32;

    pub fn drm_atomic_helper_connector_hdmi_clear_audio_infoframe(
        connector: *mut drm_connector,
    ) -> i32;

    pub fn drm_atomic_helper_connector_hdmi_update_infoframes(
        connector: *mut drm_connector,
        state: *mut drm_atomic_commit,
    ) -> i32;

    pub fn drm_atomic_helper_connector_hdmi_hotplug(
        connector: *mut drm_connector,
        status: drm_connector_status,
    );

    pub fn drm_atomic_helper_connector_hdmi_force(connector: *mut drm_connector);

    pub fn drm_hdmi_connector_mode_valid(
        connector: *mut drm_connector,
        mode: *const drm_display_mode,
    ) -> drm_mode_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
