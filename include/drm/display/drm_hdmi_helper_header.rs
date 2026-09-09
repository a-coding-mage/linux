/* SPDX-License-Identifier: MIT */

/* Dependency supplied by <linux/hdmi.h>. */
#[repr(C)]
pub struct hdmi_avi_infoframe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdmi_drm_infoframe {
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

/* Opaque representation of enum drm_output_color_format. */
pub type drm_output_color_format = i32;

extern "C" {
    pub fn drm_hdmi_avi_infoframe_colorimetry(
        frame: *mut hdmi_avi_infoframe,
        conn_state: *const drm_connector_state,
    );

    pub fn drm_hdmi_avi_infoframe_bars(
        frame: *mut hdmi_avi_infoframe,
        conn_state: *const drm_connector_state,
    );

    pub fn drm_hdmi_infoframe_set_hdr_metadata(
        frame: *mut hdmi_drm_infoframe,
        conn_state: *const drm_connector_state,
    ) -> i32;

    pub fn drm_hdmi_avi_infoframe_content_type(
        frame: *mut hdmi_avi_infoframe,
        conn_state: *const drm_connector_state,
    );

    pub fn drm_hdmi_compute_mode_clock(
        mode: *const drm_display_mode,
        bpc: u32,
        fmt: drm_output_color_format,
    ) -> u64;

    pub fn drm_hdmi_acr_get_n_cts(
        tmds_char_rate: u64,
        sample_rate: u32,
        out_n: *mut u32,
        out_cts: *mut u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
