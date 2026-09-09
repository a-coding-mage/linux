/* SPDX-License-Identifier: MIT */
/* Rust source-level translation of amdgpu_dm.h. External kernel/DC types are
 * intentionally referenced as declarations supplied by other translation units. */

pub const AMDGPU_DM_MAX_CRTC: usize = 6;
pub const AMDGPU_DM_MAX_NUM_EDP: usize = 2;
pub const AMDGPU_DMUB_NOTIFICATION_MAX: usize = 8;
pub const AMDGPU_HDR_MULT_DEFAULT: i64 = 0x100000000;
pub const AMDGPU_DM_MAX_HDMI_HPD_DEBOUNCE_MS: u32 = 5000;
pub const MAX_LUMINANCE_DATA_POINTS: usize = 99;

#[repr(C)] pub enum amd_vsdb_panel_type { AMD_VSDB_PANEL_TYPE_DEFAULT = 0, AMD_VSDB_PANEL_TYPE_MINILED, AMD_VSDB_PANEL_TYPE_OLED, AMD_VSDB_PANEL_TYPE_RESERVED }
#[repr(C)] pub enum dsc_clock_force_state { DSC_CLK_FORCE_DEFAULT = 0, DSC_CLK_FORCE_ENABLE, DSC_CLK_FORCE_DISABLE }
#[repr(C)] pub struct dsc_preferred_settings { pub dsc_force_enable: dsc_clock_force_state, pub dsc_num_slices_v: u32, pub dsc_num_slices_h: u32, pub dsc_bits_per_pixel: u32, pub dsc_force_disable_passthrough: bool }
#[repr(C)] pub enum mst_progress_status { MST_STATUS_DEFAULT = 0, MST_PROBE = 1 << 0, MST_REMOTE_EDID = 1 << 1, MST_ALLOCATE_NEW_PAYLOAD = 1 << 2, MST_CLEAR_ALLOCATED_PAYLOAD = 1 << 3 }

/* Forward declarations for dependencies supplied by the kernel/DC translation. */
macro_rules! opaque { ($($n:ident),* $(,)?) => { $(pub struct $n;)* }; }
opaque!(amdgpu_device, amdgpu_crtc, drm_device, dc, amdgpu_bo, dmub_srv, dc_plane_state, dmub_notification, dmub_cmd_fused_request, dc_stream_state, dc_crtc_timing_adjust, dpcd_sink_ext_caps, firmware, cgs_device, drm_private_obj, drm_audio_component, work_struct, workqueue_struct, backlight_device, dc_link, mod_freesync, mod_power, hdcp_workqueue, drm_atomic_commit, dc_state, gpu_info_soc_bounding_box_v1_0, secure_display_context, amdgpu_encoder, completion, delayed_work, drm_connector, cec_notifier, drm_edid, amdgpu_hpd, dc_sink, drm_dp_mst_topology_mgr, amdgpu_dm_dp_aux, drm_dp_mst_port, drm_dp_aux, amdgpu_i2c_adapter, psr_caps, drm_display_mode, dc_crtc_timing, drm_property_blob, drm_plane_state, drm_crtc_state, mod_freesync_config, dc_info_packet, drm_private_state, drm_connector_state, dc_edid_caps, dc_context, aux_payload, set_config_cmd_payload, drm_hdmi_info, pci_dev, __drm_planes_state, amdgpu_ip_block, drm_plane, list_head, mutex, spinlock_t, drm_writeback_connector, amdgpu_ip_block_version, dmub_srv_fb_info, dmub_rb_cmd);
#[repr(C)] pub union hpd_irq_data { pub raw: [u8; 64] }
#[repr(C)] pub union dp_downstream_port_present { pub raw: u32 }
#[repr(C)] pub union dwnstream_portxcaps { pub raw: [u8; 32] }
#[repr(C)] pub struct atomic64_t { pub value: i64 }
#[repr(C)] pub struct fused_io_sync { pub replied: completion, pub reply_data: [core::ffi::c_char; 0x40] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

#[repr(C)] pub struct common_irq_params { pub adev: *mut amdgpu_device, pub irq_src: i32, pub previous_timestamp: atomic64_t }
#[repr(C)] pub struct dm_compressor_info { pub cpu_addr: *mut core::ffi::c_void, pub bo_ptr: *mut amdgpu_bo, pub gpu_addr: u64 }
#[repr(C)] pub struct dm_boot_time_crc_info { pub cpu_addr: *mut core::ffi::c_void, pub bo_ptr: *mut amdgpu_bo, pub gpu_addr: u64, pub size: u32 }
pub type dmub_notify_interrupt_callback_t = Option<unsafe extern "C" fn(*mut amdgpu_device, *mut dmub_notification)>;
#[repr(C)] pub struct dmub_hpd_work { pub handle_hpd_work: work_struct, pub dmub_notify: *mut dmub_notification, pub adev: *mut amdgpu_device }
#[repr(C)] pub struct vblank_control_work { pub work: work_struct, pub dm: *mut amdgpu_display_manager, pub acrtc: *mut amdgpu_crtc, pub stream: *mut dc_stream_state, pub enable: bool }
#[repr(C)] pub struct idle_workqueue { pub work: work_struct, pub dm: *mut amdgpu_display_manager, pub enable: bool, pub running: bool }
#[repr(C)] pub struct vupdate_offload_work { pub work: work_struct, pub adev: *mut amdgpu_device, pub stream: *mut dc_stream_state, pub adjust: *mut dc_crtc_timing_adjust }
#[repr(C, packed)] pub struct amdgpu_dm_luminance_data { pub luminance: u8, pub input_signal: u8 }
#[repr(C)] pub struct amdgpu_dm_backlight_caps { pub ext_caps: *mut dpcd_sink_ext_caps, pub aux_min_input_signal: u32, pub aux_max_input_signal: u32, pub min_input_signal: i32, pub max_input_signal: i32, pub caps_valid: bool, pub aux_support: bool, pub brightness_mask: u32, pub ac_level: u8, pub dc_level: u8, pub data_points: u8, pub luminance_data: [amdgpu_dm_luminance_data; MAX_LUMINANCE_DATA_POINTS] }
#[repr(C)] pub struct dal_allocation { pub list: list_head, pub bo: *mut amdgpu_bo, pub cpu_ptr: *mut core::ffi::c_void, pub gpu_addr: u64 }
#[repr(C)] pub struct hpd_rx_irq_offload_work_queue { pub wq: *mut workqueue_struct, pub offload_lock: spinlock_t, pub is_handling_link_loss: bool, pub is_handling_mst_msg_rdy_event: bool, pub aconnector: *mut amdgpu_dm_connector }
#[repr(C)] pub struct hpd_rx_irq_offload_work { pub work: work_struct, pub data: hpd_irq_data, pub offload_wq: *mut hpd_rx_irq_offload_work_queue, pub adev: *mut amdgpu_device }

#[repr(C)] pub struct amdgpu_display_manager {
    pub dc: *mut dc, pub dmub_srv: *mut dmub_srv, pub dmub_notify: *mut dmub_notification,
    pub dmub_callback: [dmub_notify_interrupt_callback_t; AMDGPU_DMUB_NOTIFICATION_MAX],
    pub dmub_thread_offload: [bool; AMDGPU_DMUB_NOTIFICATION_MAX], pub dmub_fb_info: *mut dmub_srv_fb_info,
    pub dmub_fw: *const firmware, pub dmub_bo: *mut amdgpu_bo, pub dmub_bo_gpu_addr: u64,
    pub dmub_bo_cpu_addr: *mut core::ffi::c_void, pub dmcub_fw_version: u32, pub fw_inst_size: u32,
    pub cgs_device: *mut cgs_device, pub adev: *mut amdgpu_device, pub ddev: *mut drm_device,
    pub display_indexes_num: u16, pub atomic_obj: drm_private_obj, pub dc_lock: mutex, pub dmub_lock: spinlock_t,
    pub audio_lock: mutex, pub audio_component: *mut drm_audio_component, pub audio_registered: bool,
    pub irq_handler_list_low_tab: [list_head; 1], pub irq_handler_list_high_tab: [list_head; 1],
    pub pflip_params: [common_irq_params; 1], pub vblank_params: [common_irq_params; 1],
    pub vline0_params: [common_irq_params; 1], pub vupdate_params: [common_irq_params; 1],
    pub dmub_trace_params: [common_irq_params; 1], pub dmub_outbox_params: [common_irq_params; 1],
    pub irq_handler_list_table_lock: spinlock_t, pub irq_wq: *mut workqueue_struct, pub vmin_vmax_wq: *mut workqueue_struct,
    pub backlight_dev: [*mut backlight_device; AMDGPU_DM_MAX_NUM_EDP], pub backlight_link: [*const dc_link; AMDGPU_DM_MAX_NUM_EDP],
    pub num_of_edps: u8, pub backlight_caps: [amdgpu_dm_backlight_caps; AMDGPU_DM_MAX_NUM_EDP],
    pub freesync_module: *mut mod_freesync, pub power_module: *mut mod_power, pub hdcp_workqueue: *mut hdcp_workqueue,
    pub vblank_control_workqueue: *mut workqueue_struct, pub idle_workqueue: *mut idle_workqueue,
    pub cached_state: *mut drm_atomic_commit, pub cached_dc_state: *mut dc_state, pub compressor: dm_compressor_info,
    pub fw_dmcu: *const firmware, pub dmcu_fw_version: u32, pub soc_bounding_box: *const gpu_info_soc_bounding_box_v1_0,
    pub active_vblank_irq_count: u32, pub hpd_rx_offload_wq: *mut hpd_rx_irq_offload_work_queue,
    pub mst_encoders: [amdgpu_encoder; AMDGPU_DM_MAX_CRTC], pub force_timing_sync: bool, pub disable_hpd_irq: bool,
    pub dmcub_trace_event_en: bool, pub da_list: list_head, pub dmub_aux_transfer_done: completion,
    pub delayed_hpd_wq: *mut workqueue_struct, pub brightness: [u32; AMDGPU_DM_MAX_NUM_EDP],
    pub actual_brightness: [u32; AMDGPU_DM_MAX_NUM_EDP], pub aux_hpd_discon_quirk: bool, pub edp0_on_dp1_quirk: bool,
    pub dpia_aux_lock: mutex, pub bb_from_dmub: *mut core::ffi::c_void, pub i2c_devres_group: *mut core::ffi::c_void,
    pub oem_i2c: *mut amdgpu_i2c_adapter, pub fused_io: [fused_io_sync; 8], pub hdmi_frl_status_polling_wq: *mut workqueue_struct,
    pub hdmi_frl_status_polling_work: delayed_work, pub hdmi_frl_status_polling_delay_ms: u32,
    pub boot_time_crc_info: dm_boot_time_crc_info,
}

#[repr(C)] pub struct amdgpu_hdmi_vsdb_info { pub amd_vsdb_version: u32, pub freesync_supported: bool, pub min_refresh_rate_hz: u32, pub max_refresh_rate_hz: u32, pub freesync_mccs_vcp_code: u32, pub replay_mode: bool }
#[repr(C)] pub struct amdgpu_dm_connector { pub base: drm_connector, pub connector_id: u32, pub bl_idx: i32, pub notifier: *mut cec_notifier, pub drm_edid: *const drm_edid, pub hpd: amdgpu_hpd, pub num_modes: i32, pub dc_sink: *mut dc_sink, pub dc_link: *mut dc_link, pub dc_em_sink: *mut dc_sink, pub mst_mgr: drm_dp_mst_topology_mgr, pub dm_dp_aux: amdgpu_dm_dp_aux, pub mst_output_port: *mut drm_dp_mst_port, pub mst_root: *mut amdgpu_dm_connector, pub dsc_aux: *mut drm_dp_aux, pub mst_local_bw: u32, pub vc_full_pbn: u16, pub handle_mst_msg_ready: mutex, pub branch_ieee_oui: u32, pub i2c: *mut amdgpu_i2c_adapter, pub min_vfreq: i32, pub max_vfreq: i32, pub audio_inst: i32, pub hpd_lock: mutex, pub fake_enable: bool, pub force_yuv_pixel_format: u8, pub dsc_settings: dsc_preferred_settings, pub psr_caps: psr_caps, pub mst_downstream_port_present: dp_downstream_port_present, pub freesync_vid_base: drm_display_mode, pub sr_skip_count: i32, pub disallow_edp_enter_psr: bool, pub disallow_edp_enter_replay: bool, pub mst_downstream_port_caps: dwnstream_portxcaps, pub mst_status: u8, pub timing_changed: bool, pub timing_requested: *mut dc_crtc_timing, pub pack_sdp_v1_3: bool, pub as_type: i32, pub vsdb_info: amdgpu_hdmi_vsdb_info, pub hdmi_hpd_debounce_delay_ms: u32, pub hdmi_hpd_debounce_work: delayed_work, pub hdmi_prev_sink: *mut dc_sink, pub hdmi_comp_auto: bool }
pub unsafe fn amdgpu_dm_set_mst_status(status: *mut u8, flags: u8, set: bool) { if set { *status |= flags; } else { *status &= !flags; } }
#[repr(C)] pub struct amdgpu_dm_wb_connector { pub base: drm_writeback_connector, pub link: *mut dc_link }
#[repr(C)] pub enum amdgpu_transfer_function { AMDGPU_TRANSFER_FUNCTION_DEFAULT, AMDGPU_TRANSFER_FUNCTION_SRGB_EOTF, AMDGPU_TRANSFER_FUNCTION_BT709_INV_OETF, AMDGPU_TRANSFER_FUNCTION_PQ_EOTF, AMDGPU_TRANSFER_FUNCTION_IDENTITY, AMDGPU_TRANSFER_FUNCTION_GAMMA22_EOTF, AMDGPU_TRANSFER_FUNCTION_GAMMA24_EOTF, AMDGPU_TRANSFER_FUNCTION_GAMMA26_EOTF, AMDGPU_TRANSFER_FUNCTION_SRGB_INV_EOTF, AMDGPU_TRANSFER_FUNCTION_BT709_OETF, AMDGPU_TRANSFER_FUNCTION_PQ_INV_EOTF, AMDGPU_TRANSFER_FUNCTION_GAMMA22_INV_EOTF, AMDGPU_TRANSFER_FUNCTION_GAMMA24_INV_EOTF, AMDGPU_TRANSFER_FUNCTION_GAMMA26_INV_EOTF, AMDGPU_TRANSFER_FUNCTION_COUNT }
#[repr(C)] pub struct dm_plane_state { pub base: drm_plane_state, pub dc_state: *mut dc_plane_state, pub degamma_lut: *mut drm_property_blob, pub degamma_tf: amdgpu_transfer_function, pub hdr_mult: u64, pub ctm: *mut drm_property_blob, pub shaper_lut: *mut drm_property_blob, pub shaper_tf: amdgpu_transfer_function, pub lut3d: *mut drm_property_blob, pub blend_lut: *mut drm_property_blob, pub blend_tf: amdgpu_transfer_function }
#[repr(C)] pub enum amdgpu_dm_cursor_mode { DM_CURSOR_NATIVE_MODE = 0, DM_CURSOR_OVERLAY_MODE }
#[repr(C)] pub struct dm_crtc_state { pub base: drm_crtc_state, pub stream: *mut dc_stream_state, pub cm_has_degamma: bool, pub cm_is_degamma_srgb: bool, pub mpo_requested: bool, pub update_type: i32, pub active_planes: i32, pub crc_skip_count: i32, pub freesync_vrr_info_changed: bool, pub mode_changed_independent_from_dsc: bool, pub dsc_force_changed: bool, pub vrr_supported: bool, pub freesync_config: mod_freesync_config, pub vrr_infopacket: dc_info_packet, pub abm_level: i32, pub regamma_tf: amdgpu_transfer_function, pub cursor_mode: amdgpu_dm_cursor_mode }
#[repr(C)] pub struct dm_atomic_state { pub base: drm_private_state, pub context: *mut dc_state }
#[repr(C)] pub struct dm_connector_state { pub base: drm_connector_state, pub scaling: i32, pub underscan_vborder: u8, pub underscan_hborder: u8, pub underscan_enable: bool, pub freesync_capable: bool, pub update_hdcp: bool, pub abm_sysfs_forbidden: bool, pub abm_level: u8, pub vcpi_slots: i32, pub pbn: u64 }

pub const MAX_COLOR_3DLUT_SIZE: usize = 17;
pub const MAX_COLOR_3DLUT_BITDEPTH: usize = 12;
pub const MAX_COLOR_LUT_ENTRIES: usize = 4096;
pub const MAX_COLOR_LEGACY_LUT_ENTRIES: usize = 256;
pub const AMDGPU_DM_MAX_CRTC_U32: u32 = 6;

/* Function declarations from the original header. */
unsafe extern "C" { pub fn amdgpu_dm_trigger_timing_sync(dev: *mut drm_device); pub fn amdgpu_dm_verify_lut3d_size(adev: *mut amdgpu_device, plane_state: *mut drm_plane_state) -> i32; pub fn amdgpu_dm_init_color_mod(); pub fn amdgpu_dm_create_color_properties(adev: *mut amdgpu_device) -> i32; pub fn amdgpu_dm_verify_lut_sizes(crtc_state: *const drm_crtc_state) -> i32; pub fn amdgpu_dm_update_crtc_color_mgmt(crtc: *mut dm_crtc_state) -> i32; pub fn amdgpu_dm_check_crtc_color_mgmt(crtc: *mut dm_crtc_state, check_only: bool) -> i32; pub fn amdgpu_dm_update_plane_color_mgmt(crtc: *mut dm_crtc_state, plane_state: *mut drm_plane_state, dc_plane_state: *mut dc_plane_state) -> i32; pub fn amdgpu_dm_is_headless(adev: *mut amdgpu_device) -> bool; pub fn amdgpu_dm_crtc_complete_writeback(acrtc: *mut amdgpu_crtc) -> bool; }

pub const DAL_IRQ_SOURCES_NUMBER: usize = 1; // Dependency-provided value in the original build.
pub const MAX_COLOR_3DLUT_ENTRIES: usize = 17 * 17 * 17;
unsafe extern "C" {
    pub fn populate_hdmi_info_from_connector(enable_frl: bool, info: *mut drm_hdmi_info, edid_caps: *mut dc_edid_caps);
    pub fn amdgpu_dm_process_dmub_aux_transfer_sync(ctx: *mut dc_context, link_index: u32, payload: *mut aux_payload, operation_result: *mut i32) -> i32;
    pub fn amdgpu_dm_execute_fused_io(dev: *mut amdgpu_device, link: *mut dc_link, commands: *mut dmub_rb_cmd, count: u8, timeout_us: u32) -> bool;
    pub fn amdgpu_dm_process_dmub_set_config_sync(ctx: *mut dc_context, link_index: u32, payload: *mut set_config_cmd_payload, operation_result: *mut i32) -> i32;
    pub fn dm_atomic_get_state(state: *mut drm_atomic_commit, dm_state: *mut *mut dm_atomic_state) -> i32;
    pub fn idle_create_workqueue(adev: *mut amdgpu_device) -> *mut idle_workqueue;
    pub fn dm_allocate_gpu_mem(adev: *mut amdgpu_device, ty: i32, size: usize, addr: *mut i64) -> *mut core::ffi::c_void;
    pub fn dm_free_gpu_mem(adev: *mut amdgpu_device, ty: i32, addr: *mut core::ffi::c_void);
    pub fn retrieve_dmi_info(dm: *mut amdgpu_display_manager);
    pub fn dm_should_disable_stutter(pdev: *mut pci_dev) -> bool;
    pub fn amdgpu_dm_emulated_link_detect(link: *mut dc_link);
    pub fn amdgpu_dm_apply_delay_after_dpcd_poweroff(adev: *mut amdgpu_device, sink: *mut dc_sink);
    pub fn amdgpu_dm_get_next_zpos(state: *mut drm_atomic_commit, prev: *mut __drm_planes_state) -> *mut __drm_planes_state;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
