// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2026 Intel Corporation
 */

// Dependencies are supplied by the surrounding driver translation unit.

pub unsafe fn ivpu_jsm_msg_type_to_str(type_: vpu_ipc_msg_type) -> *const c_char {
    macro_rules! case_to_str { ($x:ident) => { if type_ == $x { return concat!(stringify!($x), "\0").as_ptr() as *const c_char; } }; }
    case_to_str!(VPU_JSM_MSG_UNKNOWN);
    case_to_str!(VPU_JSM_MSG_ENGINE_RESET);
    case_to_str!(VPU_JSM_MSG_ENGINE_PREEMPT);
    case_to_str!(VPU_JSM_MSG_REGISTER_DB);
    case_to_str!(VPU_JSM_MSG_UNREGISTER_DB);
    case_to_str!(VPU_JSM_MSG_QUERY_ENGINE_HB);
    case_to_str!(VPU_JSM_MSG_GET_POWER_LEVEL_COUNT);
    case_to_str!(VPU_JSM_MSG_GET_POWER_LEVEL);
    case_to_str!(VPU_JSM_MSG_SET_POWER_LEVEL);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_OPEN);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_CLOSE);
    case_to_str!(VPU_JSM_MSG_TRACE_SET_CONFIG);
    case_to_str!(VPU_JSM_MSG_TRACE_GET_CONFIG);
    case_to_str!(VPU_JSM_MSG_TRACE_GET_CAPABILITY);
    case_to_str!(VPU_JSM_MSG_TRACE_GET_NAME);
    case_to_str!(VPU_JSM_MSG_SSID_RELEASE);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_START);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_STOP);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_UPDATE);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_INFO);
    case_to_str!(VPU_JSM_MSG_SET_PRIORITY_BAND_SETUP);
    case_to_str!(VPU_JSM_MSG_CREATE_CMD_QUEUE);
    case_to_str!(VPU_JSM_MSG_DESTROY_CMD_QUEUE);
    case_to_str!(VPU_JSM_MSG_SET_CONTEXT_SCHED_PROPERTIES);
    case_to_str!(VPU_JSM_MSG_HWS_REGISTER_DB);
    case_to_str!(VPU_JSM_MSG_HWS_RESUME_CMDQ);
    case_to_str!(VPU_JSM_MSG_HWS_SUSPEND_CMDQ);
    case_to_str!(VPU_JSM_MSG_HWS_RESUME_CMDQ_RSP);
    case_to_str!(VPU_JSM_MSG_HWS_SUSPEND_CMDQ_DONE);
    case_to_str!(VPU_JSM_MSG_HWS_SET_SCHEDULING_LOG);
    case_to_str!(VPU_JSM_MSG_HWS_SET_SCHEDULING_LOG_RSP);
    case_to_str!(VPU_JSM_MSG_HWS_SCHEDULING_LOG_NOTIFICATION);
    case_to_str!(VPU_JSM_MSG_HWS_ENGINE_RESUME);
    case_to_str!(VPU_JSM_MSG_HWS_RESUME_ENGINE_DONE);
    case_to_str!(VPU_JSM_MSG_STATE_DUMP);
    case_to_str!(VPU_JSM_MSG_STATE_DUMP_RSP);
    case_to_str!(VPU_JSM_MSG_BLOB_DEINIT_DEPRECATED);
    case_to_str!(VPU_JSM_MSG_DYNDBG_CONTROL);
    case_to_str!(VPU_JSM_MSG_JOB_DONE);
    case_to_str!(VPU_JSM_MSG_NATIVE_FENCE_SIGNALLED);
    case_to_str!(VPU_JSM_MSG_ENGINE_RESET_DONE);
    case_to_str!(VPU_JSM_MSG_ENGINE_PREEMPT_DONE);
    case_to_str!(VPU_JSM_MSG_REGISTER_DB_DONE);
    case_to_str!(VPU_JSM_MSG_UNREGISTER_DB_DONE);
    case_to_str!(VPU_JSM_MSG_QUERY_ENGINE_HB_DONE);
    case_to_str!(VPU_JSM_MSG_GET_POWER_LEVEL_COUNT_DONE);
    case_to_str!(VPU_JSM_MSG_GET_POWER_LEVEL_DONE);
    case_to_str!(VPU_JSM_MSG_SET_POWER_LEVEL_DONE);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_OPEN_DONE);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_CLOSE_DONE);
    case_to_str!(VPU_JSM_MSG_TRACE_SET_CONFIG_RSP);
    case_to_str!(VPU_JSM_MSG_TRACE_GET_CONFIG_RSP);
    case_to_str!(VPU_JSM_MSG_TRACE_GET_CAPABILITY_RSP);
    case_to_str!(VPU_JSM_MSG_TRACE_GET_NAME_RSP);
    case_to_str!(VPU_JSM_MSG_SSID_RELEASE_DONE);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_START_DONE);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_STOP_DONE);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_UPDATE_DONE);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_INFO_DONE);
    case_to_str!(VPU_JSM_MSG_METRIC_STREAMER_NOTIFICATION);
    case_to_str!(VPU_JSM_MSG_SET_PRIORITY_BAND_SETUP_RSP);
    case_to_str!(VPU_JSM_MSG_CREATE_CMD_QUEUE_RSP);
    case_to_str!(VPU_JSM_MSG_DESTROY_CMD_QUEUE_RSP);
    case_to_str!(VPU_JSM_MSG_SET_CONTEXT_SCHED_PROPERTIES_RSP);
    case_to_str!(VPU_JSM_MSG_BLOB_DEINIT_DONE);
    case_to_str!(VPU_JSM_MSG_DYNDBG_CONTROL_RSP);
    case_to_str!(VPU_JSM_MSG_PWR_D0I3_ENTER);
    case_to_str!(VPU_JSM_MSG_PWR_D0I3_ENTER_DONE);
    case_to_str!(VPU_JSM_MSG_DCT_ENABLE);
    case_to_str!(VPU_JSM_MSG_DCT_ENABLE_DONE);
    case_to_str!(VPU_JSM_MSG_DCT_DISABLE);
    case_to_str!(VPU_JSM_MSG_DCT_DISABLE_DONE);
    case_to_str!(VPU_JSM_MSG_FREQ_CONFIG);
    case_to_str!(VPU_JSM_MSG_FREQ_CONFIG_RSP);
    case_to_str!(VPU_JSM_MSG_RESERVED_111E);
    "Unknown JSM message type\0".as_ptr() as *const c_char
}

pub unsafe fn ivpu_jsm_register_db(vdev: *mut ivpu_device, ctx_id: u32, db_id: u32, jobq_base: u64, jobq_size: u32) -> i32 {
    let mut req: vpu_jsm_msg = core::mem::zeroed(); req.type_ = VPU_JSM_MSG_REGISTER_DB; let mut resp: vpu_jsm_msg = core::mem::zeroed();
    req.payload.register_db.db_idx = db_id; req.payload.register_db.jobq_base = jobq_base; req.payload.register_db.jobq_size = jobq_size; req.payload.register_db.host_ssid = ctx_id;
    let ret = ivpu_ipc_send_receive(vdev, &mut req, VPU_JSM_MSG_REGISTER_DB_DONE, &mut resp, VPU_IPC_CHAN_ASYNC_CMD, (*vdev).timeout.jsm);
    if ret != 0 { ivpu_err_ratelimited(vdev, "Failed to register doorbell %u: %d\n", db_id, ret); } ret
}

pub unsafe fn ivpu_jsm_unregister_db(vdev: *mut ivpu_device, db_id: u32) -> i32 {
    let mut req: vpu_jsm_msg = core::mem::zeroed(); req.type_ = VPU_JSM_MSG_UNREGISTER_DB; let mut resp: vpu_jsm_msg = core::mem::zeroed(); req.payload.unregister_db.db_idx = db_id;
    let ret = ivpu_ipc_send_receive(vdev, &mut req, VPU_JSM_MSG_UNREGISTER_DB_DONE, &mut resp, VPU_IPC_CHAN_ASYNC_CMD, (*vdev).timeout.jsm);
    if ret != 0 { ivpu_warn_ratelimited(vdev, "Failed to unregister doorbell %u: %d\n", db_id, ret); } ret
}

pub unsafe fn ivpu_jsm_get_heartbeat(vdev: *mut ivpu_device, engine: u32, heartbeat: *mut u64) -> i32 {
    if engine != VPU_ENGINE_COMPUTE { return -EINVAL; }
    let mut req: vpu_jsm_msg = core::mem::zeroed(); req.type_ = VPU_JSM_MSG_QUERY_ENGINE_HB; req.payload.query_engine_hb.engine_idx = engine; let mut resp: vpu_jsm_msg = core::mem::zeroed();
    let ret = ivpu_ipc_send_receive(vdev, &mut req, VPU_JSM_MSG_QUERY_ENGINE_HB_DONE, &mut resp, VPU_IPC_CHAN_ASYNC_CMD, (*vdev).timeout.jsm);
    if ret != 0 { ivpu_err_ratelimited(vdev, "Failed to get heartbeat from engine %d: %d\n", engine, ret); return ret; } *heartbeat = resp.payload.query_engine_hb_done.heartbeat; ret
}

pub unsafe fn ivpu_jsm_reset_engine(vdev: *mut ivpu_device, engine: u32, resp: *mut vpu_jsm_msg) -> i32 {
    if engine != VPU_ENGINE_COMPUTE { return -EINVAL; }
    let mut req: vpu_jsm_msg = core::mem::zeroed(); req.type_ = VPU_JSM_MSG_ENGINE_RESET; req.payload.engine_reset.engine_idx = engine;
    let ret = ivpu_ipc_send_receive(vdev, &mut req, VPU_JSM_MSG_ENGINE_RESET_DONE, resp, VPU_IPC_CHAN_ASYNC_CMD, (*vdev).timeout.jsm);
    if ret != 0 { ivpu_err_ratelimited(vdev, "Failed to reset engine %d: %d\n", engine, ret); ivpu_pm_trigger_recovery(vdev, "Engine reset failed"); return ret; } atomic_inc(&mut (*(*vdev).pm).engine_reset_counter); 0
}

pub unsafe fn ivpu_jsm_preempt_engine(vdev: *mut ivpu_device, engine: u32, preempt_id: u32) -> i32 {
    if engine != VPU_ENGINE_COMPUTE { return -EINVAL; }
    let mut req: vpu_jsm_msg = core::mem::zeroed(); req.type_ = VPU_JSM_MSG_ENGINE_PREEMPT; req.payload.engine_preempt.engine_idx = engine; req.payload.engine_preempt.preempt_id = preempt_id; let mut resp: vpu_jsm_msg = core::mem::zeroed();
    let ret = ivpu_ipc_send_receive(vdev, &mut req, VPU_JSM_MSG_ENGINE_PREEMPT_DONE, &mut resp, VPU_IPC_CHAN_ASYNC_CMD, (*vdev).timeout.jsm); if ret != 0 { ivpu_err_ratelimited(vdev, "Failed to preempt engine %d: %d\n", engine, ret); } ret
}

pub unsafe fn ivpu_jsm_dyndbg_control(vdev: *mut ivpu_device, command: *mut c_char, _size: usize) -> i32 { let mut req: vpu_jsm_msg = core::mem::zeroed(); req.type_ = VPU_JSM_MSG_DYNDBG_CONTROL; let mut resp: vpu_jsm_msg = core::mem::zeroed(); strscpy(req.payload.dyndbg_control.dyndbg_cmd.as_mut_ptr(), command, VPU_DYNDBG_CMD_MAX_LEN); let ret = ivpu_ipc_send_receive(vdev, &mut req, VPU_JSM_MSG_DYNDBG_CONTROL_RSP, &mut resp, VPU_IPC_CHAN_GEN_CMD, (*vdev).timeout.jsm); if ret != 0 { ivpu_warn_ratelimited(vdev, "Failed to send command \"%s\": ret %d\n", command, ret); } ret }

pub unsafe fn ivpu_jsm_trace_get_capability(vdev: *mut ivpu_device, destination: *mut u32, component: *mut u64) -> i32 { let mut req: vpu_jsm_msg = core::mem::zeroed(); req.type_ = VPU_JSM_MSG_TRACE_GET_CAPABILITY; let mut resp: vpu_jsm_msg = core::mem::zeroed(); let ret = ivpu_ipc_send_receive(vdev, &mut req, VPU_JSM_MSG_TRACE_GET_CAPABILITY_RSP, &mut resp, VPU_IPC_CHAN_ASYNC_CMD, (*vdev).timeout.jsm); if ret != 0 { ivpu_warn_ratelimited(vdev, "Failed to get trace capability: %d\n", ret); return ret; } *destination = resp.payload.trace_capability.trace_destination_mask; *component = resp.payload.trace_capability.trace_hw_component_mask; ret }
pub unsafe fn ivpu_jsm_trace_set_config(vdev: *mut ivpu_device, level: u32, destination: u32, component: u64) -> i32 { let mut req: vpu_jsm_msg = core::mem::zeroed(); req.type_ = VPU_JSM_MSG_TRACE_SET_CONFIG; req.payload.trace_config.trace_level = level; req.payload.trace_config.trace_destination_mask = destination; req.payload.trace_config.trace_hw_component_mask = component; let mut resp: vpu_jsm_msg = core::mem::zeroed(); let ret = ivpu_ipc_send_receive(vdev, &mut req, VPU_JSM_MSG_TRACE_SET_CONFIG_RSP, &mut resp, VPU_IPC_CHAN_ASYNC_CMD, (*vdev).timeout.jsm); if ret != 0 { ivpu_warn_ratelimited(vdev, "Failed to set config: %d\n", ret); } ret }
pub unsafe fn ivpu_jsm_context_release(vdev: *mut ivpu_device, host_ssid: u32) -> i32 { let mut req: vpu_jsm_msg = core::mem::zeroed(); req.type_ = VPU_JSM_MSG_SSID_RELEASE; req.payload.ssid_release.host_ssid = host_ssid; let mut resp: vpu_jsm_msg = core::mem::zeroed(); let ret = ivpu_ipc_send_receive(vdev, &mut req, VPU_JSM_MSG_SSID_RELEASE_DONE, &mut resp, VPU_IPC_CHAN_ASYNC_CMD, (*vdev).timeout.jsm); if ret != 0 { ivpu_warn_ratelimited(vdev, "Failed to release context: %d\n", ret); } ret }
pub unsafe fn ivpu_jsm_pwr_d0i3_enter(vdev: *mut ivpu_device) -> i32 { let mut req: vpu_jsm_msg = core::mem::zeroed(); req.type_ = VPU_JSM_MSG_PWR_D0I3_ENTER; req.payload.pwr_d0i3_enter.send_response = 1; let mut resp: vpu_jsm_msg = core::mem::zeroed(); let ret = ivpu_ipc_send_receive_internal(vdev, &mut req, VPU_JSM_MSG_PWR_D0I3_ENTER_DONE, &mut resp, VPU_IPC_CHAN_GEN_CMD, (*vdev).timeout.d0i3_entry_msg); if ret != 0 { return ret; } ivpu_hw_wait_for_idle(vdev) }

pub unsafe fn ivpu_jsm_hws_create_cmdq(vdev: *mut ivpu_device, ctx_id: u32, group: u32, id: u32, pid: u32, engine: u32, base: u64, size: u32) -> i32 { let mut req: vpu_jsm_msg = core::mem::zeroed(); req.type_ = VPU_JSM_MSG_CREATE_CMD_QUEUE; req.payload.hws_create_cmdq.host_ssid=ctx_id; req.payload.hws_create_cmdq.process_id=pid; req.payload.hws_create_cmdq.engine_idx=engine; req.payload.hws_create_cmdq.cmdq_group=group; req.payload.hws_create_cmdq.cmdq_id=id; req.payload.hws_create_cmdq.cmdq_base=base; req.payload.hws_create_cmdq.cmdq_size=size; let mut resp: vpu_jsm_msg=core::mem::zeroed(); let ret=ivpu_ipc_send_receive(vdev,&mut req,VPU_JSM_MSG_CREATE_CMD_QUEUE_RSP,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm); if ret!=0 {ivpu_warn_ratelimited(vdev,"Failed to create command queue: %d\n",ret);} ret }
pub unsafe fn ivpu_jsm_hws_destroy_cmdq(vdev:*mut ivpu_device,ctx_id:u32,id:u32)->i32 { let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_DESTROY_CMD_QUEUE;req.payload.hws_destroy_cmdq.host_ssid=ctx_id;req.payload.hws_destroy_cmdq.cmdq_id=id;let mut resp:vpu_jsm_msg=core::mem::zeroed();let ret=ivpu_ipc_send_receive(vdev,&mut req,VPU_JSM_MSG_DESTROY_CMD_QUEUE_RSP,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm);if ret!=0{ivpu_warn_ratelimited(vdev,"Failed to destroy command queue: %d\n",ret);}ret }
pub unsafe fn ivpu_jsm_hws_register_db(vdev:*mut ivpu_device,ctx:u32,qid:u32,db:u32,base:u64,size:u32)->i32 { let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_HWS_REGISTER_DB;req.payload.hws_register_db.db_id=db;req.payload.hws_register_db.host_ssid=ctx;req.payload.hws_register_db.cmdq_id=qid;req.payload.hws_register_db.cmdq_base=base;req.payload.hws_register_db.cmdq_size=size;let mut resp:vpu_jsm_msg=core::mem::zeroed();let ret=ivpu_ipc_send_receive(vdev,&mut req,VPU_JSM_MSG_REGISTER_DB_DONE,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm);if ret!=0{ivpu_err_ratelimited(vdev,"Failed to register doorbell %u: %d\n",db,ret);}ret }
pub unsafe fn ivpu_jsm_hws_resume_engine(vdev:*mut ivpu_device,engine:u32)->i32 {if engine!=VPU_ENGINE_COMPUTE{return -EINVAL;}let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_HWS_ENGINE_RESUME;req.payload.hws_resume_engine.engine_idx=engine;let mut resp:vpu_jsm_msg=core::mem::zeroed();let ret=ivpu_ipc_send_receive(vdev,&mut req,VPU_JSM_MSG_HWS_RESUME_ENGINE_DONE,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm);if ret!=0{ivpu_err_ratelimited(vdev,"Failed to resume engine %d: %d\n",engine,ret);ivpu_pm_trigger_recovery(vdev,"Engine resume failed");}ret}

pub unsafe fn ivpu_jsm_hws_set_context_sched_properties(vdev:*mut ivpu_device,ctx:u32,qid:u32,priority:u32)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_SET_CONTEXT_SCHED_PROPERTIES;let p=&mut req.payload.hws_set_context_sched_properties;p.host_ssid=ctx;p.cmdq_id=qid;p.priority_band=priority;p.realtime_priority_level=0;p.in_process_priority=0;p.context_quantum=20000;p.grace_period_same_priority=10000;p.grace_period_lower_priority=0;let mut resp:vpu_jsm_msg=core::mem::zeroed();let ret=ivpu_ipc_send_receive(vdev,&mut req,VPU_JSM_MSG_SET_CONTEXT_SCHED_PROPERTIES_RSP,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm);if ret!=0{ivpu_warn_ratelimited(vdev,"Failed to set context sched properties: %d\n",ret);}ret}
pub unsafe fn ivpu_jsm_hws_set_scheduling_log(vdev:*mut ivpu_device,engine:u32,ssid:u32,va:u64)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_HWS_SET_SCHEDULING_LOG;let p=&mut req.payload.hws_set_scheduling_log;p.engine_idx=engine;p.host_ssid=ssid;p.vpu_log_buffer_va=va;p.notify_index=0;let mut resp:vpu_jsm_msg=core::mem::zeroed();let ret=ivpu_ipc_send_receive(vdev,&mut req,VPU_JSM_MSG_HWS_SET_SCHEDULING_LOG_RSP,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm);if ret!=0{ivpu_warn_ratelimited(vdev,"Failed to set scheduling log: %d\n",ret);}ret}
pub unsafe fn ivpu_jsm_hws_setup_priority_bands(vdev:*mut ivpu_device)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_SET_PRIORITY_BAND_SETUP;let mut resp:vpu_jsm_msg=core::mem::zeroed();let hw=(*vdev).hw;let setup=&mut req.payload.hws_priority_band_setup;for band in VPU_JOB_SCHEDULING_PRIORITY_BAND_IDLE..VPU_JOB_SCHEDULING_PRIORITY_BAND_COUNT {setup.grace_period[band as usize]=(*hw).hws.grace_period[band as usize];setup.process_grace_period[band as usize]=(*hw).hws.process_grace_period[band as usize];setup.process_quantum[band as usize]=(*hw).hws.process_quantum[band as usize];}setup.normal_band_percentage=10;let ret=ivpu_ipc_send_receive_internal(vdev,&mut req,VPU_JSM_MSG_SET_PRIORITY_BAND_SETUP_RSP,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm);if ret!=0{ivpu_warn_ratelimited(vdev,"Failed to set priority bands: %d\n",ret);}ret}
pub unsafe fn ivpu_jsm_metric_streamer_start(vdev:*mut ivpu_device,mask:u64,rate:u64,addr:u64,size:u64)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_METRIC_STREAMER_START;let p=&mut req.payload.metric_streamer_start;p.metric_group_mask=mask;p.sampling_rate=rate;p.buffer_addr=addr;p.buffer_size=size;let mut resp:vpu_jsm_msg=core::mem::zeroed();let ret=ivpu_ipc_send_receive(vdev,&mut req,VPU_JSM_MSG_METRIC_STREAMER_START_DONE,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm);if ret!=0{ivpu_warn_ratelimited(vdev,"Failed to start metric streamer: ret %d\n",ret);}ret}
pub unsafe fn ivpu_jsm_metric_streamer_stop(vdev:*mut ivpu_device,mask:u64)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_METRIC_STREAMER_STOP;req.payload.metric_streamer_stop.metric_group_mask=mask;let mut resp:vpu_jsm_msg=core::mem::zeroed();let ret=ivpu_ipc_send_receive(vdev,&mut req,VPU_JSM_MSG_METRIC_STREAMER_STOP_DONE,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm);if ret!=0{ivpu_warn_ratelimited(vdev,"Failed to stop metric streamer: ret %d\n",ret);}ret}
pub unsafe fn ivpu_jsm_metric_streamer_update(vdev:*mut ivpu_device,mask:u64,addr:u64,size:u64,written:*mut u64)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_METRIC_STREAMER_UPDATE;let p=&mut req.payload.metric_streamer_update;p.metric_group_mask=mask;p.buffer_addr=addr;p.buffer_size=size;let mut resp:vpu_jsm_msg=core::mem::zeroed();let ret=ivpu_ipc_send_receive(vdev,&mut req,VPU_JSM_MSG_METRIC_STREAMER_UPDATE_DONE,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm);if ret!=0{return ret;}if size!=0&&resp.payload.metric_streamer_done.bytes_written>size{return -EOVERFLOW;}*written=resp.payload.metric_streamer_done.bytes_written;ret}
pub unsafe fn ivpu_jsm_metric_streamer_info(vdev:*mut ivpu_device,mask:u64,addr:u64,size:u64,sample:*mut u32,info:*mut u64)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_METRIC_STREAMER_INFO;let p=&mut req.payload.metric_streamer_start;p.metric_group_mask=mask;p.buffer_addr=addr;p.buffer_size=size;let mut resp:vpu_jsm_msg=core::mem::zeroed();let ret=ivpu_ipc_send_receive(vdev,&mut req,VPU_JSM_MSG_METRIC_STREAMER_INFO_DONE,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm);if ret!=0{return ret;}if resp.payload.metric_streamer_done.sample_size==0{return -EBADMSG;}if !sample.is_null(){*sample=resp.payload.metric_streamer_done.sample_size;}if !info.is_null(){*info=resp.payload.metric_streamer_done.bytes_written;}ret}
pub unsafe fn ivpu_jsm_dct_enable(vdev:*mut ivpu_device,active:u32,inactive:u32)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_DCT_ENABLE;req.payload.pwr_dct_control.dct_active_us=active;req.payload.pwr_dct_control.dct_inactive_us=inactive;let mut resp:vpu_jsm_msg=core::mem::zeroed();ivpu_ipc_send_receive_internal(vdev,&mut req,VPU_JSM_MSG_DCT_ENABLE_DONE,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm)}
pub unsafe fn ivpu_jsm_dct_disable(vdev:*mut ivpu_device)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_DCT_DISABLE;let mut resp:vpu_jsm_msg=core::mem::zeroed();ivpu_ipc_send_receive_internal(vdev,&mut req,VPU_JSM_MSG_DCT_DISABLE_DONE,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm)}
pub unsafe fn ivpu_jsm_state_dump(vdev:*mut ivpu_device)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_STATE_DUMP;let mut resp:vpu_jsm_msg=core::mem::zeroed();ivpu_ipc_send_receive_internal(vdev,&mut req,VPU_JSM_MSG_STATE_DUMP_RSP,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm)}
pub unsafe fn ivpu_jsm_state_dump_no_reply(vdev:*mut ivpu_device)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_STATE_DUMP;ivpu_ipc_send_and_wait(vdev,&mut req,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.state_dump_msg)}
pub unsafe fn ivpu_jsm_msg_freq_config(vdev:*mut ivpu_device,min_ratio:u16,pn_ratio:u16,max_ratio:u16)->i32 {let mut req:vpu_jsm_msg=core::mem::zeroed();req.type_=VPU_JSM_MSG_FREQ_CONFIG;req.payload.freq_config.min_freq_pll_ratio=min_ratio;req.payload.freq_config.pn_freq_pll_ratio=pn_ratio;req.payload.freq_config.max_freq_pll_ratio=max_ratio;let mut resp:vpu_jsm_msg=core::mem::zeroed();ivpu_ipc_send_receive_internal(vdev,&mut req,VPU_JSM_MSG_FREQ_CONFIG_RSP,&mut resp,VPU_IPC_CHAN_ASYNC_CMD,(*vdev).timeout.jsm)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
