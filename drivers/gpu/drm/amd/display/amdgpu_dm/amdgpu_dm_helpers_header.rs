/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

/*
 * Original declarations are enabled when CONFIG_DRM_AMD_DC_KUNIT_TEST is
 * enabled. The build-system condition is preserved here as source intent.
 */

#[repr(C)]
pub struct edid {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_dm_connector {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_stream_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_edid_caps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_dp_mst_stream_allocation_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_dp_aux {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_dp_mst_atomic_payload {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_dp_mst_topology_mgr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_dp_mst_topology_state {
    _private: [u8; 0],
}

extern "C" {
    /* Exported for KUnit testing */
    pub fn edid_extract_panel_id(edid: *mut edid) -> u32;
    pub fn apply_edid_quirks(
        link: *mut dc_link,
        edid: *mut edid,
        edid_caps: *mut dc_edid_caps,
    );
    pub fn get_max_frl_rate(max_lanes: u8, max_rate_per_lane: u8) -> u8;
    pub fn get_dsc_max_slices(max_slices: u8, clk_per_slice: i32) -> u8;
    pub fn dm_is_freesync_pcon_whitelist(branch_dev_id: u32) -> bool;
    pub static dm_freesync_pcon_whitelist: [u32; 0];
    pub fn dm_freesync_pcon_whitelist_count() -> u32;
    pub fn fill_dc_mst_payload_table_from_drm(
        link: *mut dc_link,
        enable: bool,
        target_payload: *mut drm_dp_mst_atomic_payload,
        table: *mut dc_dp_mst_stream_allocation_table,
    );
    pub fn dm_helpers_construct_old_payload(
        mgr: *mut drm_dp_mst_topology_mgr,
        mst_state: *mut drm_dp_mst_topology_state,
        new_payload: *mut drm_dp_mst_atomic_payload,
        old_payload: *mut drm_dp_mst_atomic_payload,
    );
    pub fn dm_helpers_dp_write_dsc_enable(
        ctx: *mut dc_context,
        stream: *const dc_stream_state,
        enable: bool,
    ) -> bool;
    pub fn dm_helpers_get_dc_debug_mask() -> u32;
    pub fn dm_helpers_set_dc_debug_mask(debug_mask: u32);
    pub fn dm_helpers_probe_acpi_edid(
        data: *mut core::ffi::c_void,
        buf: *mut u8,
        block: u32,
        len: usize,
    ) -> i32;
    pub fn dm_helpers_read_acpi_edid(
        aconnector: *mut amdgpu_dm_connector,
    ) -> *const drm_edid;
    pub fn dm_helpers_read_vbios_hardcoded_edid(
        link: *mut dc_link,
        aconnector: *mut amdgpu_dm_connector,
    ) -> *const drm_edid;
    pub fn execute_synaptics_rc_command(
        aux: *mut drm_dp_aux,
        is_write_cmd: bool,
        cmd: u8,
        length: u32,
        offset: u32,
        data: *mut u8,
    ) -> bool;
    pub fn apply_synaptics_fifo_reset_wa(aux: *mut drm_dp_aux);
    pub fn write_dsc_enable_synaptics_non_virtual_dpcd_mst(
        aux: *mut drm_dp_aux,
        stream: *const dc_stream_state,
        enable: bool,
    ) -> u8;
}

#[repr(C)]
pub struct drm_edid {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
