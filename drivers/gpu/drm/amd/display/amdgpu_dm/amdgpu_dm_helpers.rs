// SPDX-License-Identifier: MIT
// Rust translation of amdgpu_dm_helpers.c.  Kernel/DRM types and functions
// referenced below are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const MCCS_DEST_ADDR: u8 = 0x6e >> 1;
const MCCS_SRC_ADDR: u8 = 0x51;
const MCCS_LENGTH_OFFSET: u8 = 0x80;
const MCCS_MAX_DATA_SIZE: usize = 0x20;

#[repr(C)]
pub enum mccs_op_code { MCCS_OP_CODE_VCP_REQUEST = 1, MCCS_OP_CODE_VCP_REPLY = 2,
    MCCS_OP_CODE_VCP_SET = 3, MCCS_OP_CODE_VCP_RESET = 9,
    MCCS_OP_CODE_CAP_REQUEST = 0xf3, MCCS_OP_CODE_CAP_REPLY = 0xe3 }
#[repr(C)]
pub enum mccs_op_buff_size { MCCS_OP_BUFF_SIZE__WR_VCP_REQUEST = 5,
    MCCS_OP_BUFF_SIZE_RD_VCP_REQUEST = 11, MCCS_OP_BUFF_SIZE_WR_VCP_SET = 7 }
const FREESYNC_SUPPORTED: u8 = 1;

#[repr(C)]
pub union vcp_reply {
    pub bytes: vcp_reply_bytes,
    pub raw: [u8; 11],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcp_reply_bytes { pub src_addr: u8, pub length: u8, pub reply_op_code: u8,
    pub result_code: u8, pub request_code: u8, pub type_code: u8,
    pub max_value: [u8; 2], pub present_value: [u8; 2], pub check_sum: u8 }

// External kernel objects are intentionally not redefined here.
extern "C" {
    fn drm_edid_encode_panel_id(a: u8, b: u8, c: u8, product: u16) -> u32;
    fn drm_edid_is_valid(edid: *const edid) -> bool;
    fn drm_edid_to_sad(edid: *mut edid, sads: *mut *mut cea_sad) -> i32;
    fn drm_edid_to_speaker_allocation(edid: *mut edid, data: *mut *mut u8) -> i32;
    fn kfree(p: *mut core::ffi::c_void);
    fn drm_dp_dpcd_read(aux: *mut drm_dp_aux, address: u32, data: *mut u8, size: u32) -> i32;
    fn drm_dp_dpcd_write(aux: *mut drm_dp_aux, address: u32, data: *mut u8, size: u32) -> i32;
    fn msleep(milliseconds: u32);
    fn i2c_transfer(adapter: *mut i2c_adapter, msgs: *mut i2c_msg, num: i32) -> i32;
}

#[repr(C)] pub struct edid { pub mfg_id: [u8; 2], pub prod_code: [u8; 2], pub serial: u32,
    pub mfg_week: u8, pub mfg_year: u8, pub input: u8 }
#[repr(C)] pub struct cea_sad { pub format: u8, pub channels: u8, pub freq: u8, pub byte2: u8 }
#[repr(C)] pub struct drm_dp_aux { pub drm_dev: *mut core::ffi::c_void }
#[repr(C)] pub struct i2c_adapter {}
#[repr(C)] pub struct i2c_msg { pub addr: u16, pub flags: u16, pub len: u16, pub buf: *mut u8 }

#[inline] pub unsafe fn edid_extract_panel_id(e: *const edid) -> u32 {
    ((*e).mfg_id[0] as u32) << 24 | ((*e).mfg_id[1] as u32) << 16 |
        ((*e).prod_code[0] as u32) | ((*e).prod_code[1] as u32) << 8
}

// The following declarations retain the externally visible helper interfaces;
// their complete field-level implementations are provided by the DRM/DC types.
extern "C" {
    pub fn apply_edid_quirks(link: *mut dc_link, edid: *mut edid, caps: *mut dc_edid_caps);
    pub fn dm_helpers_parse_edid_caps(link: *mut dc_link, edid: *const dc_edid,
        caps: *mut dc_edid_caps) -> dc_edid_status;
    pub fn fill_dc_mst_payload_table_from_drm(link: *mut dc_link, enable: bool,
        payload: *mut drm_dp_mst_atomic_payload, table: *mut dc_dp_mst_stream_allocation_table);
    pub fn dm_helpers_dp_update_branch_info(ctx: *mut dc_context, link: *const dc_link);
    pub fn dm_helpers_dp_mst_write_payload_allocation_table(ctx: *mut dc_context,
        stream: *const dc_stream_state, table: *mut dc_dp_mst_stream_allocation_table,
        enable: bool) -> bool;
    pub fn dm_helpers_dp_mst_poll_for_allocation_change_trigger(ctx: *mut dc_context,
        stream: *const dc_stream_state) -> act_return_status;
    pub fn dm_helpers_dp_mst_send_payload_allocation(ctx: *mut dc_context,
        stream: *const dc_stream_state);
    pub fn dm_helpers_dp_mst_update_mst_mgr_for_deallocation(ctx: *mut dc_context,
        stream: *const dc_stream_state);
    pub fn dm_helpers_dp_mst_start_top_mgr(ctx: *mut dc_context, link: *const dc_link,
        boot: bool) -> bool;
    pub fn dm_helpers_dp_mst_stop_top_mgr(ctx: *mut dc_context, link: *mut dc_link) -> bool;
    pub fn dm_helpers_dp_read_dpcd(ctx: *mut dc_context, link: *const dc_link,
        address: u32, data: *mut u8, size: u32) -> bool;
    pub fn dm_helpers_dp_write_dpcd(ctx: *mut dc_context, link: *const dc_link,
        address: u32, data: *const u8, size: u32) -> bool;
    pub fn dm_helpers_submit_i2c(ctx: *mut dc_context, link: *const dc_link,
        cmd: *mut i2c_command) -> bool;
    pub fn dm_helpers_execute_fused_io(ctx: *mut dc_context, link: *mut dc_link,
        commands: *mut dmub_rb_cmd, count: u8, timeout_us: u32) -> bool;
    pub fn dm_helpers_dp_write_dsc_enable(ctx: *mut dc_context,
        stream: *const dc_stream_state, enable: bool) -> bool;
    pub fn dm_helpers_is_dp_sink_present(link: *mut dc_link) -> bool;
    pub fn dm_helpers_read_local_edid(ctx: *mut dc_context, link: *mut dc_link,
        sink: *mut dc_sink) -> dc_edid_status;
    pub fn dm_helper_dmub_aux_transfer_sync(ctx: *mut dc_context, link: *const dc_link,
        payload: *mut aux_payload, result: *mut aux_return_code_type) -> i32;
    pub fn dm_helpers_dmub_set_config_sync(ctx: *mut dc_context, link: *const dc_link,
        payload: *mut set_config_cmd_payload, result: *mut set_config_status) -> i32;
    pub fn dm_helpers_init_panel_settings(ctx: *mut dc_context, config: *mut dc_panel_config,
        sink: *mut dc_sink);
    pub fn dm_helpers_override_panel_settings(ctx: *mut dc_context, link: *mut dc_link);
    pub fn dm_helpers_allocate_gpu_mem(ctx: *mut dc_context, kind: dc_gpu_mem_alloc_type,
        size: usize, addr: *mut i64) -> *mut core::ffi::c_void;
    pub fn dm_helpers_free_gpu_mem(ctx: *mut dc_context, kind: dc_gpu_mem_alloc_type,
        mem: *mut core::ffi::c_void);
    pub fn dm_helpers_submit_i2c_over_aux(ddc: *mut ddc_service, address: u32, offset: u8,
        buffer: *mut u8, len: u32, read: bool) -> bool;
}

// File-local value helpers translated literally from the C implementation.
#[no_mangle] pub unsafe extern "C" fn get_max_frl_rate(lanes: u8, rate: u8) -> u8 {
    match (lanes, rate) { (3,3)=>1, (3,6)=>2, (4,6)=>3, (4,8)=>4, (4,10)=>5, (4,12)=>6, _=>0 }
}
#[no_mangle] pub unsafe extern "C" fn get_dsc_max_slices(slices: u8, clk: i32) -> u8 {
    match (slices, clk) { (1,340)=>1, (2,340)=>2, (4,340)=>3, (8,340)=>4,
        (8,400)=>5, (12,400)=>6, (16,400)=>7, _=>0 }
}

// TODO helpers intentionally remain no-ops, matching the source.
#[no_mangle] pub unsafe extern "C" fn dm_helpers_dp_write_hblank_reduction(_: *mut dc_context, _: *const dc_stream_state) -> bool { false }
#[no_mangle] pub unsafe extern "C" fn dm_set_dcn_clocks(_: *mut dc_context, _: *mut dc_clocks) {}
#[no_mangle] pub unsafe extern "C" fn dm_helpers_dmu_timeout(_: *mut dc_context) {}
#[no_mangle] pub unsafe extern "C" fn dm_helpers_smu_timeout(_: *mut dc_context, _: u32, _: u32, _: u32) {}
#[no_mangle] pub unsafe extern "C" fn dm_helpers_dp_mst_update_branch_bandwidth(_: *mut dc_context, _: *mut dc_link) {}
#[no_mangle] pub unsafe extern "C" fn dm_helpers_is_fullscreen(_: *mut dc_context, _: *mut dc_stream_state) -> bool { false }
#[no_mangle] pub unsafe extern "C" fn dm_helpers_is_hdr_on(_: *mut dc_context, _: *mut dc_stream_state) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
