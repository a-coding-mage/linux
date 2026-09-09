/* SPDX-License-Identifier: MIT */
/* Copyright 2019-2026 Advanced Micro Devices, Inc. */
/* Translated from dmub_srv.h. External types are supplied by dependent headers. */

pub const DMUB_PC_SNAPSHOT_COUNT: usize = 10;
pub const DMUB_TRACE_BUFFER_SIZE: u32 = 64 * 1024;
pub const PSP_HEADER_BYTES_256: u32 = 0x100;
pub const PSP_FOOTER_BYTES_256: u32 = 0x100;

pub enum dmub_srv_common_regs {}
pub enum dmub_srv_dcn31_regs {}
pub enum dmub_srv_dcn32_regs {}
pub enum dmub_srv_dcn35_regs {}
pub enum dmub_srv_dcn401_regs {}
pub enum dmub_srv_dcn42_regs {}
pub enum dmub_srv_dcn60_regs {}
pub enum dmcub_trace_buf_entry {}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_window_memory_type { DMUB_WINDOW_MEMORY_TYPE_FB = 0, DMUB_WINDOW_MEMORY_TYPE_GART }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_status { DMUB_STATUS_OK = 0, DMUB_STATUS_NO_CTX, DMUB_STATUS_QUEUE_FULL, DMUB_STATUS_TIMEOUT, DMUB_STATUS_INVALID, DMUB_STATUS_HW_FAILURE, DMUB_STATUS_POWER_STATE_D3 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_asic { DMUB_ASIC_NONE = 0, DMUB_ASIC_DCN20, DMUB_ASIC_DCN21, DMUB_ASIC_DCN30, DMUB_ASIC_DCN301, DMUB_ASIC_DCN302, DMUB_ASIC_DCN303, DMUB_ASIC_DCN31, DMUB_ASIC_DCN31B, DMUB_ASIC_DCN314, DMUB_ASIC_DCN315, DMUB_ASIC_DCN316, DMUB_ASIC_DCN32, DMUB_ASIC_DCN321, DMUB_ASIC_DCN35, DMUB_ASIC_DCN351, DMUB_ASIC_DCN36, DMUB_ASIC_DCN401, DMUB_ASIC_DCN42, DMUB_ASIC_DCN42B, DMUB_ASIC_DCN60, DMUB_ASIC_MAX }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_window_id { DMUB_WINDOW_0_INST_CONST = 0, DMUB_WINDOW_1_STACK, DMUB_WINDOW_2_BSS_DATA, DMUB_WINDOW_3_VBIOS, DMUB_WINDOW_4_MAILBOX, DMUB_WINDOW_5_TRACEBUFF, DMUB_WINDOW_6_FW_STATE, DMUB_WINDOW_7_SCRATCH_MEM, DMUB_WINDOW_IB_MEM, DMUB_WINDOW_SHARED_STATE, DMUB_WINDOW_LSDMA_BUFFER, DMUB_WINDOW_CURSOR_OFFLOAD, DMUB_WINDOW_TOTAL }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_notification_type { DMUB_NOTIFICATION_NO_DATA = 0, DMUB_NOTIFICATION_AUX_REPLY, DMUB_NOTIFICATION_HPD, DMUB_NOTIFICATION_HPD_IRQ, DMUB_NOTIFICATION_SET_CONFIG_REPLY, DMUB_NOTIFICATION_DPIA_NOTIFICATION, DMUB_NOTIFICATION_HPD_SENSE_NOTIFY, DMUB_NOTIFICATION_FUSED_IO, DMUB_NOTIFICATION_MAX }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpia_notify_bw_alloc_status { DPIA_BW_REQ_FAILED = 0, DPIA_BW_REQ_SUCCESS, DPIA_EST_BW_CHANGED, DPIA_BW_ALLOC_CAPS_CHANGED }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_memory_access_type { DMUB_MEMORY_ACCESS_DEFAULT = 0, DMUB_MEMORY_ACCESS_CPU = 0, DMUB_MEMORY_ACCESS_DMA }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_srv_power_state_type { DMUB_POWER_STATE_UNDEFINED = 0, DMUB_POWER_STATE_D0 = 1, DMUB_POWER_STATE_D3 = 8 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_inbox_cmd_interface_type { DMUB_CMD_INTERFACE_DEFAULT = 0, DMUB_CMD_INTERFACE_FB = 1, DMUB_CMD_INTERFACE_REG = 2 }

#[repr(C)] pub struct dmub_region { pub base: u32, pub top: u32 }
#[repr(C)] pub struct dmub_window { pub offset: dmub_addr, pub region: dmub_region }
#[repr(C)] pub struct dmub_fb { pub cpu_addr: *mut core::ffi::c_void, pub gpu_addr: u64, pub size: u32 }
#[repr(C)] pub struct dmub_srv_region_params { pub inst_const_size: u32, pub bss_data_size: u32, pub vbios_size: u32, pub fw_inst_const: *const u8, pub fw_bss_data: *const u8, pub window_memory_type: *const dmub_window_memory_type, pub fw_info: *const dmub_fw_meta_info }
#[repr(C)] pub struct dmub_srv_fw_meta_info_params { pub inst_const_size: u32, pub bss_data_size: u32, pub fw_inst_const: *const u8, pub fw_bss_data: *const u8, pub custom_psp_footer_size: u32 }
#[repr(C)] pub struct dmub_srv_region_info { pub fb_size: u32, pub gart_size: u32, pub num_regions: u8, pub regions: [dmub_region; DMUB_WINDOW_TOTAL as usize], pub verified_psp_footer_size: u32 }
#[repr(C)] pub struct dmub_srv_memory_params { pub region_info: *const dmub_srv_region_info, pub cpu_fb_addr: *mut core::ffi::c_void, pub cpu_gart_addr: *mut core::ffi::c_void, pub gpu_fb_addr: u64, pub gpu_gart_addr: u64, pub window_memory_type: *const dmub_window_memory_type }
#[repr(C)] pub struct dmub_srv_fb_info { pub num_fb: u8, pub fb: [dmub_fb; DMUB_WINDOW_TOTAL as usize] }
#[repr(C)] pub struct dmub_soc_fb_info { pub fb_base: u64, pub fb_offset: u64, pub alt_channel_region_size: [u32; 2], pub alt_channel_region_base: [u64; 2] }

#[repr(C)] pub struct dmub_srv_hw_params { pub fb_info: *mut dmub_srv_fb_info, pub soc_fb_info: dmub_soc_fb_info, pub psp_version: u32, pub load_inst_const: bool, pub skip_panel_power_sequence: bool, pub disable_z10: bool, pub power_optimization: bool, pub dpia_supported: bool, pub disable_dpia: bool, pub usb4_cm_version: bool, pub fw_in_system_memory: bool, pub dpia_hpd_int_enable_supported: bool, pub disable_clock_gate: bool, pub disallow_dispclk_dppclk_ds: bool, pub ips_sequential_ono: bool, pub mem_access_type: dmub_memory_access_type, pub disable_ips: dmub_ips_disable_type, pub disallow_phy_access: bool, pub disable_sldo_opt: bool, pub enable_non_transparent_setconfig: bool, pub lower_hbr3_phy_ssc: bool, pub override_hbr3_pll_vco: bool, pub disable_dpia_bw_allocation: bool }
#[repr(C)] pub struct dmub_timeout_info { pub timeout_occured: bool, pub timeout_cmd: dmub_rb_cmd, pub timestamp: u64 }
#[repr(C)] pub struct dmub_diagnostic_data { pub dmcub_version: u32, pub scratch: [u32; 17], pub pc: [u32; DMUB_PC_SNAPSHOT_COUNT], pub undefined_address_fault_addr: u32, pub inst_fetch_fault_addr: u32, pub data_write_fault_addr: u32, pub inbox1_rptr: u32, pub inbox1_wptr: u32, pub inbox1_size: u32, pub inbox0_rptr: u32, pub inbox0_wptr: u32, pub inbox0_size: u32, pub outbox1_rptr: u32, pub outbox1_wptr: u32, pub outbox1_size: u32, pub gpint_datain0: u32, pub timeout_info: dmub_timeout_info, pub is_dmcub_enabled: u8, pub is_dmcub_soft_reset: u8, pub is_dmcub_secure_reset: u8, pub is_traceport_en: u8, pub is_cw0_enabled: u8, pub is_cw6_enabled: u8, pub is_pwait: u8 }
#[repr(C)] pub struct dmub_preos_info { pub fb_base: u64, pub fb_offset: u64, pub trace_buffer_phy_addr: u64, pub trace_buffer_size: u32, pub fw_version: u32, pub boot_status: u32, pub boot_options: u32 }
#[repr(C)] pub struct dmub_srv_inbox { pub num_submitted: u64, pub num_reported: u64, pub status: dmub_srv_inbox_status }
#[repr(C)] pub union dmub_srv_inbox_status { pub rb: dmub_rb, pub register_status: dmub_srv_register_status }
#[repr(C)] pub struct dmub_srv_register_status { pub is_pending: bool, pub is_multi_pending: bool }

pub type dmub_reg_read_fn = unsafe extern "C" fn(*mut core::ffi::c_void, u32) -> u32;
pub type dmub_reg_write_fn = unsafe extern "C" fn(*mut core::ffi::c_void, u32, u32);
#[repr(C)] pub struct dmub_srv_base_funcs { pub reg_read: Option<dmub_reg_read_fn>, pub reg_write: Option<dmub_reg_write_fn> }

pub type dmub_void_srv_fn = unsafe extern "C" fn(*mut dmub_srv);
pub type dmub_bool_srv_fn = unsafe extern "C" fn(*mut dmub_srv) -> bool;
#[repr(C)] pub struct dmub_srv_hw_funcs {
 pub init: Option<dmub_void_srv_fn>, pub reset: Option<dmub_void_srv_fn>, pub reset_release: Option<dmub_void_srv_fn>,
 pub backdoor_load: Option<unsafe extern "C" fn(*mut dmub_srv,*const dmub_window,*const dmub_window)>,
 pub backdoor_load_zfb_mode: Option<unsafe extern "C" fn(*mut dmub_srv,*const dmub_window,*const dmub_window)>,
 pub setup_windows: Option<unsafe extern "C" fn(*mut dmub_srv,*const dmub_window,*const dmub_window,*const dmub_window,*const dmub_window,*const dmub_window,*const dmub_window)>,
 pub setup_mailbox: Option<unsafe extern "C" fn(*mut dmub_srv,*const dmub_region)>, pub get_inbox1_wptr: Option<unsafe extern "C" fn(*mut dmub_srv)->u32>, pub get_inbox1_rptr: Option<unsafe extern "C" fn(*mut dmub_srv)->u32>, pub set_inbox1_wptr: Option<unsafe extern "C" fn(*mut dmub_srv,u32)>, pub setup_out_mailbox: Option<unsafe extern "C" fn(*mut dmub_srv,*const dmub_region)>, pub get_outbox1_wptr: Option<unsafe extern "C" fn(*mut dmub_srv)->u32>, pub set_outbox1_rptr: Option<unsafe extern "C" fn(*mut dmub_srv,u32)>, pub setup_outbox0: Option<unsafe extern "C" fn(*mut dmub_srv,*const dmub_region)>, pub get_outbox0_wptr: Option<unsafe extern "C" fn(*mut dmub_srv)->u32>, pub set_outbox0_rptr: Option<unsafe extern "C" fn(*mut dmub_srv,u32)>, pub emul_get_inbox1_rptr: Option<unsafe extern "C" fn(*mut dmub_srv)->u32>, pub emul_get_inbox1_wptr: Option<unsafe extern "C" fn(*mut dmub_srv)->u32>, pub emul_set_inbox1_wptr: Option<unsafe extern "C" fn(*mut dmub_srv,u32)>, pub is_supported: Option<dmub_bool_srv_fn>, pub is_psrsu_supported: Option<dmub_bool_srv_fn>, pub is_hw_init: Option<dmub_bool_srv_fn>, pub is_hw_powered_up: Option<dmub_bool_srv_fn>,
}

#[repr(C)] pub struct dmub_srv_create_params { pub funcs: dmub_srv_base_funcs, pub hw_funcs: *mut dmub_srv_hw_funcs, pub user_ctx: *mut core::ffi::c_void, pub asic: dmub_asic, pub fw_version: u32, pub is_virtual: bool, pub inbox_type: dmub_inbox_cmd_interface_type }
#[repr(C)] pub struct dmub_srv { pub asic: dmub_asic, pub user_ctx: *mut core::ffi::c_void, pub fw_version: u32, pub is_virtual: bool, pub no_ext_reg_access: bool, pub scratch_mem_fb: dmub_fb, pub ib_mem_gart: dmub_fb, pub cursor_offload_fb: dmub_fb, pub fb_info: *const dmub_srv_fb_info, pub shared_state: *mut dmub_shared_state_feature_block, pub cursor_offload_v1: *mut dmub_cursor_offload_v1, pub fw_state: *const dmub_fw_state, pub regs: *const dmub_srv_common_regs, pub regs_dcn31: *const dmub_srv_dcn31_regs, pub regs_dcn32: *mut dmub_srv_dcn32_regs, pub regs_dcn35: *mut dmub_srv_dcn35_regs, pub regs_dcn401: *const dmub_srv_dcn401_regs, pub regs_dcn42: *mut dmub_srv_dcn42_regs, pub regs_dcn60: *const dmub_srv_dcn60_regs, pub funcs: dmub_srv_base_funcs, pub hw_funcs: dmub_srv_hw_funcs, pub inbox1: dmub_srv_inbox, pub inbox1_last_wptr: u32, pub reg_inbox0: dmub_srv_inbox, pub outbox1_rb: dmub_rb, pub outbox0_rb: dmub_rb, pub sw_init: bool, pub hw_init: bool, pub dpia_supported: bool, pub soc_fb_info: dmub_soc_fb_info, pub psp_version: u32, pub meta_info: dmub_fw_meta_info, pub feature_caps: dmub_feature_caps, pub visual_confirm_color: dmub_visual_confirm_color, pub inbox_type: dmub_inbox_cmd_interface_type, pub power_state: dmub_srv_power_state_type, pub debug: dmub_diagnostic_data, pub lsdma_rb_fb: dmub_fb, pub preos_info: dmub_preos_info }
#[repr(C)] pub struct dmub_notification { pub type_: dmub_notification_type, pub link_index: u8, pub result: u8, pub instance: u8, pub pending_notification: bool, pub data: dmub_notification_data }
#[repr(C)] pub union dmub_notification_data { pub aux_reply: aux_reply_data, pub hpd_status: dp_hpd_status, pub sc_status: set_config_status, pub hpd_sense_notify: dmub_rb_cmd_hpd_sense_notify_data, pub fused_request: dmub_cmd_fused_request }

#[inline] pub const fn DMUB_FW_VERSION(major: u32, minor: u32, revision: u32) -> u32 { (((major & 0xFF) << 24) | ((minor & 0xFF) << 16) | ((revision & 0xFF) << 8)) }

extern "C" {
 pub fn dmub_srv_create(dmub: *mut dmub_srv, params: *const dmub_srv_create_params) -> dmub_status;
 pub fn dmub_srv_destroy(dmub: *mut dmub_srv);
 pub fn dmub_srv_calc_region_info(dmub: *mut dmub_srv, params: *const dmub_srv_region_params, out: *mut dmub_srv_region_info) -> dmub_status;
 pub fn dmub_srv_calc_mem_info(dmub: *mut dmub_srv, params: *const dmub_srv_memory_params, out: *mut dmub_srv_fb_info) -> dmub_status;
 pub fn dmub_srv_has_hw_support(dmub: *mut dmub_srv, supported: *mut bool) -> dmub_status;
 pub fn dmub_srv_is_hw_init(dmub: *mut dmub_srv, initialized: *mut bool) -> dmub_status;
 pub fn dmub_srv_hw_init(dmub: *mut dmub_srv, params: *const dmub_srv_hw_params) -> dmub_status;
 pub fn dmub_srv_hw_reset(dmub: *mut dmub_srv) -> dmub_status;
 pub fn dmub_srv_fb_cmd_queue(dmub: *mut dmub_srv, cmd: *const dmub_rb_cmd) -> dmub_status;
 pub fn dmub_srv_fb_cmd_execute(dmub: *mut dmub_srv) -> dmub_status;
 pub fn dmub_srv_wait_for_hw_pwr_up(dmub: *mut dmub_srv, timeout_us: u32) -> dmub_status;
 pub fn dmub_srv_is_hw_pwr_up(dmub: *mut dmub_srv) -> bool;
 pub fn dmub_srv_wait_for_auto_load(dmub: *mut dmub_srv, timeout_us: u32) -> dmub_status;
 pub fn dmub_srv_wait_for_phy_init(dmub: *mut dmub_srv, timeout_us: u32) -> dmub_status;
 pub fn dmub_srv_wait_for_pending(dmub: *mut dmub_srv, timeout_us: u32) -> dmub_status;
 pub fn dmub_srv_wait_for_idle(dmub: *mut dmub_srv, timeout_us: u32) -> dmub_status;
 pub fn dmub_srv_send_gpint_command(dmub: *mut dmub_srv, command_code: dmub_gpint_command, param: u16, timeout_us: u32) -> dmub_status;
 pub fn dmub_srv_get_gpint_response(dmub: *mut dmub_srv, response: *mut u32) -> dmub_status;
 pub fn dmub_srv_get_gpint_dataout(dmub: *mut dmub_srv, dataout: *mut u32) -> dmub_status;
 pub fn dmub_srv_flush_buffer_mem(dmub: *mut dmub_srv, fb: *const dmub_fb);
 pub fn dmub_srv_get_fw_boot_status(dmub: *mut dmub_srv, status: *mut dmub_fw_boot_status) -> dmub_status;
 pub fn dmub_srv_get_fw_boot_option(dmub: *mut dmub_srv, option: *mut dmub_fw_boot_options) -> dmub_status;
 pub fn dmub_srv_set_skip_panel_power_sequence(dmub: *mut dmub_srv, skip: bool) -> dmub_status;
 pub fn dmub_srv_get_outbox0_msg(dmub: *mut dmub_srv, entry: *mut dmcub_trace_buf_entry) -> bool;
 pub fn dmub_srv_get_diagnostic_data(dmub: *mut dmub_srv) -> bool;
 pub fn dmub_srv_should_detect(dmub: *mut dmub_srv) -> bool;
 pub fn dmub_srv_send_inbox0_cmd(dmub: *mut dmub_srv, data: dmub_inbox0_data_register) -> dmub_status;
 pub fn dmub_srv_wait_for_inbox0_ack(dmub: *mut dmub_srv, timeout_us: u32) -> dmub_status;
 pub fn dmub_srv_clear_inbox0_ack(dmub: *mut dmub_srv) -> dmub_status;
 pub fn dmub_srv_subvp_save_surf_addr(dmub: *mut dmub_srv, addr: *const dc_plane_address, subvp_index: u8);
 pub fn dmub_srv_set_power_state(dmub: *mut dmub_srv, power_state: dmub_srv_power_state_type);
 pub fn dmub_srv_reg_cmd_execute(dmub: *mut dmub_srv, cmd: *mut dmub_rb_cmd) -> dmub_status;
 pub fn dmub_srv_cmd_get_response(dmub: *mut dmub_srv, cmd_rsp: *mut dmub_rb_cmd);
 pub fn dmub_srv_sync_inboxes(dmub: *mut dmub_srv) -> dmub_status;
 pub fn dmub_srv_wait_for_inbox_free(dmub: *mut dmub_srv, timeout_us: u32, num_free_required: u32) -> dmub_status;
 pub fn dmub_srv_update_inbox_status(dmub: *mut dmub_srv) -> dmub_status;
 pub fn dmub_srv_get_preos_info(dmub: *mut dmub_srv) -> bool;
 pub fn dmub_srv_get_fw_meta_info_from_raw_fw(params: *mut dmub_srv_fw_meta_info_params, fw_info_out: *mut dmub_fw_meta_info) -> dmub_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
