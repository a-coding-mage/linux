/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Rust translation of dc_dmub_srv.c.  Types, constants, macros, and external
 * functions supplied by the surrounding display-core implementation remain
 * external dependencies.
 */

const GPINT_RETRY_NUM: u32 = 20;
const MAX_WAIT_US: u32 = 100000;

extern "C" {
    fn dc_dmub_srv_log_diagnostic_data(srv: *mut dc_dmub_srv);
    fn dm_helpers_dmu_timeout(ctx: *mut dc_context);
    fn dmub_srv_wait_for_pending(srv: *mut dmub_srv, timeout: u32) -> dmub_status;
    fn dmub_srv_clear_inbox0_ack(srv: *mut dmub_srv) -> dmub_status;
    fn dmub_srv_wait_for_inbox0_ack(srv: *mut dmub_srv, timeout: u32) -> dmub_status;
    fn dmub_srv_send_inbox0_cmd(srv: *mut dmub_srv, data: dmub_inbox0_data_register) -> dmub_status;
    fn dmub_srv_wait_for_idle(srv: *mut dmub_srv, timeout: u32) -> dmub_status;
    fn dmub_srv_reg_cmd_execute(srv: *mut dmub_srv, cmd: *mut dmub_rb_cmd) -> dmub_status;
    fn dmub_srv_fb_cmd_queue(srv: *mut dmub_srv, cmd: *mut dmub_rb_cmd) -> dmub_status;
    fn dmub_srv_fb_cmd_execute(srv: *mut dmub_srv) -> dmub_status;
    fn dmub_srv_wait_for_inbox_free(srv: *mut dmub_srv, timeout: u32, count: u32) -> dmub_status;
    fn dmub_srv_update_inbox_status(srv: *mut dmub_srv) -> dmub_status;
    fn dmub_srv_cmd_get_response(srv: *mut dmub_srv, cmd: *mut dmub_rb_cmd);
    fn dmub_srv_get_fw_boot_status(srv: *mut dmub_srv, status: *mut dmub_fw_boot_status) -> dmub_status;
    fn dmub_srv_get_outbox0_msg(srv: *mut dmub_srv, entry: *mut dmcub_trace_buf_entry) -> bool;
    fn dmub_srv_get_diagnostic_data(srv: *mut dmub_srv) -> bool;
    fn dmub_srv_send_gpint_command(srv: *mut dmub_srv, code: dmub_gpint_command, param: u16, wait: u32) -> dmub_status;
    fn dmub_srv_get_gpint_response(srv: *mut dmub_srv, response: *mut u32);
    fn dmub_srv_is_hw_pwr_up(srv: *mut dmub_srv) -> bool;
    fn dmub_srv_wait_for_hw_pwr_up(srv: *mut dmub_srv, timeout: u32) -> dmub_status;
    fn dmub_srv_set_power_state(srv: *mut dmub_srv, state: dmub_power_state);
    fn dmub_srv_sync_inboxes(srv: *mut dmub_srv);
    fn dmub_srv_flush_buffer_mem(srv: *mut dmub_srv, fb: *mut dmub_fb);
    fn dmub_srv_subvp_save_surf_addr(srv: *mut dmub_srv, addr: *const dc_plane_address, index: u8);
    fn dmub_srv_get_preos_info(srv: *mut dmub_srv) -> bool;
    fn dc_wake_and_execute_dmub_cmd_list(ctx: *const dc_context, count: u32, cmd: *mut dmub_rb_cmd, wait: dm_dmub_wait_type) -> bool;
    fn dm_execute_dmub_cmd(ctx: *const dc_context, cmd: *mut dmub_rb_cmd, wait: dm_dmub_wait_type) -> bool;
    fn dm_execute_dmub_cmd_list(ctx: *const dc_context, count: u32, cmd: *mut dmub_rb_cmd, wait: dm_dmub_wait_type) -> bool;
    fn dc_wake_and_execute_gpint(ctx: *const dc_context, code: dmub_gpint_command, param: u16, response: *mut u32, wait: dm_dmub_wait_type) -> bool;
    fn dm_get_timestamp(ctx: *mut dc_context) -> u64;
    fn dmub_rb_num_free(rb: *mut dmub_ring_buffer) -> u32;
    fn dc_state_get_stream_status(state: *mut dc_state, stream: *mut dc_stream_state) -> *mut dc_stream_status;
    fn dc_state_get_paired_subvp_stream(state: *mut dc_state, stream: *mut dc_stream_state) -> *mut dc_stream_state;
    fn dc_state_get_pipe_subvp_type(state: *mut dc_state, pipe: *mut pipe_ctx) -> mall_stream_type;
    fn resource_is_pipe_type(pipe: *mut pipe_ctx, ty: pipe_type) -> bool;
    fn resource_can_pipe_disable_cursor(pipe: *mut pipe_ctx) -> bool;
    fn resource_get_otg_master_for_stream(res: *mut resource_context, stream: *const dc_stream_state) -> *const pipe_ctx;
    fn dc_get_edp_link_panel_inst(dc: *mut dc, link: *mut dc_link, panel: *mut u32) -> bool;
    fn dc_plane_get_pipe_mask(state: *mut dc_state, plane: *mut dc_plane_state) -> u32;
    fn reduce_fraction(a: u32, b: u32, n: *mut u32, d: *mut u32);
    fn udelay(us: u32);
}

#[repr(C)] pub struct dc_dmub_srv { pub dmub: *mut dmub_srv, pub ctx: *mut dc_context, pub cursor_offload_enabled: bool, pub idle_allowed: bool, pub idle_exit_counter: i32, pub needs_idle_wake: bool, pub driver_signals: dmub_shared_state_ips_driver_signals }
#[repr(C)] pub struct dc_context { pub dc: *mut dc, pub dmub_srv: *mut dc_dmub_srv }
#[repr(C)] pub struct dc { pub ctx: *mut dc_context, pub current_state: *mut dc_state, pub res_pool: *mut resource_pool, pub config: dc_config, pub debug: dc_debug, pub caps: dc_caps, pub work_arounds: dc_work_arounds, pub clk_mgr: *mut clk_mgr }
#[repr(C)] pub struct dmub_srv { pub inbox_type: u32, pub debug: dmub_debug, pub feature_caps: dmub_feature_caps, pub meta_info: dmub_meta_info, pub cursor_offload_enabled: bool, pub shared_state: *mut dmub_shared_state, pub cursor_offload_fb: dmub_fb, pub ib_mem_gart: dmub_fb, pub scratch_mem_fb: dmub_fb, pub lsdma_rb_fb: dmub_fb }
extern "C" { /* surrounding translation supplies the complete repr(C) layouts */ }

#[inline] pub unsafe fn dc_dmub_srv_construct(s: *mut dc_dmub_srv, dc: *mut dc, dmub: *mut dmub_srv) { (*s).dmub = dmub; (*s).ctx = (*dc).ctx; }
unsafe fn dc_dmub_srv_handle_failure(s: *mut dc_dmub_srv) { dc_dmub_srv_log_diagnostic_data(s); if (*(*s).ctx).dc.as_ref().unwrap().debug.enable_dmu_recovery { dm_helpers_dmu_timeout((*s).ctx); } }

pub unsafe fn dc_dmub_srv_create(dc: *mut dc, dmub: *mut dmub_srv) -> *mut dc_dmub_srv { let s = libc::calloc(1, core::mem::size_of::<dc_dmub_srv>()) as *mut dc_dmub_srv; if s.is_null() { return core::ptr::null_mut(); } dc_dmub_srv_construct(s, dc, dmub); s }
pub unsafe fn dc_dmub_srv_destroy(s: *mut *mut dc_dmub_srv) { if !s.is_null() && !(*s).is_null() { libc::free(*s as *mut _); *s = core::ptr::null_mut(); } }

pub unsafe fn dc_dmub_srv_wait_for_pending(s: *mut dc_dmub_srv) -> bool { if s.is_null() || (*s).dmub.is_null() { return false; } let mut st; loop { st=dmub_srv_wait_for_pending((*s).dmub,MAX_WAIT_US); if !(*(*s).ctx).dc.as_ref().unwrap().debug.disable_timeout || st==DMUB_STATUS_OK { break; } } if st!=DMUB_STATUS_OK { dc_dmub_srv_handle_failure(s); } st==DMUB_STATUS_OK }
pub unsafe fn dc_dmub_srv_clear_inbox0_ack(s: *mut dc_dmub_srv) { let st=dmub_srv_clear_inbox0_ack((*s).dmub); if st!=DMUB_STATUS_OK { dc_dmub_srv_handle_failure(s); } }
pub unsafe fn dc_dmub_srv_wait_for_inbox0_ack(s: *mut dc_dmub_srv) { let st=dmub_srv_wait_for_inbox0_ack((*s).dmub,MAX_WAIT_US); if st!=DMUB_STATUS_OK { dc_dmub_srv_handle_failure(s); } }
pub unsafe fn dc_dmub_srv_send_inbox0_cmd(s: *mut dc_dmub_srv, data: dmub_inbox0_data_register) { if dmub_srv_send_inbox0_cmd((*s).dmub,data)!=DMUB_STATUS_OK { dc_dmub_srv_handle_failure(s); } }

pub unsafe fn dc_dmub_srv_cmd_list_queue_execute(s:*mut dc_dmub_srv,count:u32,list:*mut dmub_rb_cmd)->bool { if s.is_null()||(*s).dmub.is_null(){return false;} for i in 0..count { let mut st; loop {st=dmub_srv_wait_for_idle((*s).dmub,MAX_WAIT_US);if !(*(*s).ctx).dc.as_ref().unwrap().debug.disable_timeout||st==DMUB_STATUS_OK{break;}} if st==DMUB_STATUS_OK {st=dmub_srv_reg_cmd_execute((*s).dmub,list.add(i as usize));} if st!=DMUB_STATUS_OK{return false;} } dmub_srv_update_inbox_status((*s).dmub)==DMUB_STATUS_OK }
pub unsafe fn dc_dmub_srv_wait_for_idle(s:*mut dc_dmub_srv, wait:dm_dmub_wait_type, cmd:*mut dmub_rb_cmd)->bool { if s.is_null()||(*s).dmub.is_null(){return false;} if wait!=DM_DMUB_WAIT_TYPE_NO_WAIT {let st=dmub_srv_wait_for_idle((*s).dmub,MAX_WAIT_US);if st!=DMUB_STATUS_OK{dc_dmub_srv_handle_failure(s);return false;}if wait==DM_DMUB_WAIT_TYPE_WAIT_WITH_REPLY&&!cmd.is_null(){dmub_srv_cmd_get_response((*s).dmub,cmd);}} true }
pub unsafe fn dc_wake_and_execute_dmub_cmd(ctx:*const dc_context,cmd:*mut dmub_rb_cmd,wait:dm_dmub_wait_type)->bool{dc_wake_and_execute_dmub_cmd_list(ctx,1,cmd,wait)}
pub unsafe fn dc_dmub_srv_cmd_run(s:*mut dc_dmub_srv,cmd:*mut dmub_rb_cmd,wait:dm_dmub_wait_type)->bool{dc_wake_and_execute_dmub_cmd((*s).ctx,cmd,wait)}
pub unsafe fn dc_dmub_srv_get_diagnostic_data(s:*mut dc_dmub_srv)->bool{!s.is_null()&&!(*s).dmub.is_null()&&dmub_srv_get_diagnostic_data((*s).dmub)}
pub unsafe fn dc_dmub_srv_get_outbox0_msg(dc:*const dc,e:*mut dmcub_trace_buf_entry)->bool{dmub_srv_get_outbox0_msg((*(*dc).ctx).dmub_srv.as_ref().unwrap().dmub,e)}
pub unsafe fn dc_dmub_srv_is_cursor_offload_enabled(dc:*const dc)->bool{!(*(*dc).ctx).dmub_srv.is_null()&&(*(*(*dc).ctx).dmub_srv).cursor_offload_enabled}
pub unsafe fn dc_dmub_srv_subvp_save_surf_addr(s:*const dc_dmub_srv,a:*const dc_plane_address,i:u8){dmub_srv_subvp_save_surf_addr((*s).dmub,a,i)}
pub unsafe fn dc_dmub_srv_set_power_state(s:*mut dc_dmub_srv,state:dc_acpi_cm_power_state){if !s.is_null(){dmub_srv_set_power_state((*s).dmub,if state==DC_ACPI_CM_POWER_STATE_D0{DMUB_POWER_STATE_D0}else{DMUB_POWER_STATE_D3});}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
