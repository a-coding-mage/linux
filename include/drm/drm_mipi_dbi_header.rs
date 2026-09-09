/* SPDX-License-Identifier: GPL-2.0-or-later */
/* MIPI Display Bus Interface (DBI) LCD controller support */

// C dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct mipi_dbi {
    pub cmdlock: mutex,
    pub command: Option<unsafe extern "C" fn(*mut mipi_dbi, *mut u8, *mut u8, usize) -> i32>,
    pub read_commands: *const u8,
    pub swap_bytes: bool,
    pub reset: *mut gpio_desc,
    pub spi: *mut spi_device,
    pub write_memory_bpw: u32,
    pub dc: *mut gpio_desc,
    pub tx_buf9: *mut core::ffi::c_void,
    pub tx_buf9_len: usize,
}

#[repr(C)]
pub struct mipi_dbi_dev {
    pub drm: drm_device,
    pub mode: drm_display_mode,
    pub pixel_format: u32,
    pub tx_buf: *mut u16,
    pub rotation: u32,
    pub left_offset: u32,
    pub top_offset: u32,
    pub backlight: *mut backlight_device,
    pub regulator: *mut regulator,
    pub io_regulator: *mut regulator,
    pub dbi: mipi_dbi,
    pub driver_private: *mut core::ffi::c_void,
}

pub unsafe fn drm_to_mipi_dbi_dev(drm: *mut drm_device) -> *mut mipi_dbi_dev {
    // Equivalent to container_of(drm, struct mipi_dbi_dev, drm).
    (drm as *mut u8).sub(core::mem::offset_of!(mipi_dbi_dev, drm)) as *mut mipi_dbi_dev
}

extern "C" {
    pub fn mipi_dbi_spi_init(spi: *mut spi_device, dbi: *mut mipi_dbi, dc: *mut gpio_desc) -> i32;
    pub fn drm_mipi_dbi_dev_init(dbidev: *mut mipi_dbi_dev, mode: *const drm_display_mode,
                                 format: u32, rotation: u32, tx_buf_size: usize) -> i32;
    pub fn mipi_dbi_hw_reset(dbi: *mut mipi_dbi);
    pub fn mipi_dbi_display_is_on(dbi: *mut mipi_dbi) -> bool;
    pub fn mipi_dbi_poweron_reset(dbidev: *mut mipi_dbi_dev) -> i32;
    pub fn mipi_dbi_poweron_conditional_reset(dbidev: *mut mipi_dbi_dev) -> i32;
    pub fn mipi_dbi_spi_cmd_max_speed(spi: *mut spi_device, len: usize) -> u32;
    pub fn mipi_dbi_spi_transfer(spi: *mut spi_device, speed_hz: u32, bpw: u8,
                                 buf: *const core::ffi::c_void, len: usize) -> i32;
    pub fn mipi_dbi_command_read(dbi: *mut mipi_dbi, cmd: u8, val: *mut u8) -> i32;
    pub fn mipi_dbi_command_buf(dbi: *mut mipi_dbi, cmd: u8, data: *mut u8, len: usize) -> i32;
    pub fn mipi_dbi_command_stackbuf(dbi: *mut mipi_dbi, cmd: u8, data: *const u8, len: usize) -> i32;
    pub fn mipi_dbi_buf_copy(dst: *mut core::ffi::c_void, src: *mut iosys_map,
                             fb: *mut drm_framebuffer, clip: *mut drm_rect, swap: bool,
                             fmtcnv_state: *mut drm_format_conv_state) -> i32;
    pub fn drm_mipi_dbi_plane_helper_atomic_check(plane: *mut drm_plane, state: *mut drm_atomic_commit) -> i32;
    pub fn drm_mipi_dbi_plane_helper_atomic_update(plane: *mut drm_plane, state: *mut drm_atomic_commit);
    pub fn drm_mipi_dbi_crtc_helper_mode_valid(crtc: *mut drm_crtc, mode: *const drm_display_mode) -> drm_mode_status;
    pub fn drm_mipi_dbi_crtc_helper_atomic_check(crtc: *mut drm_crtc, state: *mut drm_atomic_commit) -> i32;
    pub fn drm_mipi_dbi_crtc_helper_atomic_disable(crtc: *mut drm_crtc, state: *mut drm_atomic_commit);
    pub fn drm_mipi_dbi_connector_helper_get_modes(connector: *mut drm_connector) -> i32;
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub fn mipi_dbi_debugfs_init(minor: *mut drm_minor);
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn mipi_dbi_debugfs_init(_minor: *mut drm_minor) {}

// The following C macros expand to initializer fragments and retain their names for users
// of the translated header.
#[macro_export]
macro_rules! mipi_dbi_command {
    ($dbi:expr, $cmd:expr $(, $seq:expr)* $(,)?) => {{
        let d: [u8; <[()]>::len(&[$( { let _ = &$seq; () }),*])] = [$($seq),*];
        mipi_dbi_command_stackbuf($dbi, $cmd, d.as_ptr(), d.len())
    }};
}

// DRM_MIPI_DBI_PLANE_FORMATS: DRM_FORMAT_RGB565, DRM_FORMAT_XRGB8888
// DRM_MIPI_DBI_PLANE_FORMAT_MODIFIERS: DRM_FORMAT_MOD_LINEAR, DRM_FORMAT_MOD_INVALID
// DRM_MIPI_DBI_PLANE_FUNCS, DRM_MIPI_DBI_PLANE_HELPER_FUNCS,
// DRM_MIPI_DBI_CRTC_FUNCS, DRM_MIPI_DBI_CRTC_HELPER_FUNCS,
// DRM_MIPI_DBI_CONNECTOR_FUNCS, DRM_MIPI_DBI_CONNECTOR_HELPER_FUNCS,
// DRM_MIPI_DBI_MODE_CONFIG_FUNCS, DRM_MIPI_DBI_MODE_CONFIG_HELPER_FUNCS:
// C initializer fragments preserved as comments because their containing types are external.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
