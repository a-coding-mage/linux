// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023-2024, Advanced Micro Devices, Inc. */
// Translated from aie2_message.c. Kernel and driver declarations are supplied externally.

#![allow(dead_code, unused_variables, unused_mut, non_snake_case)]

unsafe fn aie2_suspend_fw(ndev: *mut amdxdna_dev_hdl) -> i32 {
    let mut msg = DECLARE_AIE_MSG!(suspend, MSG_OP_SUSPEND);
    let ret = aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg);
    if ret != 0 { XDNA_ERR!((*ndev).aie.xdna, "Failed to suspend fw, ret {}", ret); return ret; }
    aie_psp_waitmode_poll((*ndev).aie.psp_hdl)
}
unsafe fn aie2_resume_fw(ndev: *mut amdxdna_dev_hdl) -> i32 {
    let mut msg = DECLARE_AIE_MSG!(suspend, MSG_OP_RESUME);
    aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg)
}
unsafe fn aie2_set_runtime_cfg(ndev: *mut amdxdna_dev_hdl, typ: u32, value: u64) -> i32 {
    let mut msg = DECLARE_AIE_MSG!(set_runtime_cfg, MSG_OP_SET_RUNTIME_CONFIG);
    msg.req.type_ = typ; msg.req.value = value;
    let ret = aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg);
    if ret != 0 { XDNA_ERR!((*ndev).aie.xdna, "Failed to set runtime config, ret {}", ret); return ret; } 0
}
unsafe fn aie2_get_runtime_cfg(ndev: *mut amdxdna_dev_hdl, typ: u32, value: *mut u64) -> i32 {
    let mut msg = DECLARE_AIE_MSG!(get_runtime_cfg, MSG_OP_GET_RUNTIME_CONFIG);
    msg.req.type_ = typ;
    let ret = aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg);
    if ret != 0 { XDNA_ERR!((*ndev).aie.xdna, "Failed to get runtime config, ret {}", ret); return ret; }
    *value = msg.resp.value; 0
}
unsafe fn aie2_assign_mgmt_pasid(ndev: *mut amdxdna_dev_hdl, pasid: u16) -> i32 {
    let mut msg = DECLARE_AIE_MSG!(assign_mgmt_pasid, MSG_OP_ASSIGN_MGMT_PASID);
    msg.req.pasid = pasid; aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg)
}
unsafe fn aie2_query_aie_version(ndev: *mut amdxdna_dev_hdl, version: *mut amdxdna_drm_query_aie_version) -> i32 {
    let mut msg = DECLARE_AIE_MSG!(aie_version_info, MSG_OP_QUERY_AIE_VERSION);
    let ret = aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg); if ret != 0 { return ret; }
    (*version).major = msg.resp.major; (*version).minor = msg.resp.minor; 0
}
unsafe fn aie2_query_aie_metadata(ndev: *mut amdxdna_dev_hdl, metadata: *mut amdxdna_drm_query_aie_metadata) -> i32 {
    let mut msg = DECLARE_AIE_MSG!(aie_tile_info, MSG_OP_QUERY_AIE_TILE_INFO);
    let ret = aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg); if ret != 0 { return ret; }
    let i = &msg.resp.info; let m = &mut *metadata;
    m.col_size=i.size; m.cols=i.cols; m.rows=i.rows; m.version.major=i.major; m.version.minor=i.minor;
    m.core.row_count=i.core_rows; m.core.row_start=i.core_row_start; m.core.dma_channel_count=i.core_dma_channels; m.core.lock_count=i.core_locks; m.core.event_reg_count=i.core_events;
    m.mem.row_count=i.mem_rows; m.mem.row_start=i.mem_row_start; m.mem.dma_channel_count=i.mem_dma_channels; m.mem.lock_count=i.mem_locks; m.mem.event_reg_count=i.mem_events;
    m.shim.row_count=i.shim_rows; m.shim.row_start=i.shim_row_start; m.shim.dma_channel_count=i.shim_dma_channels; m.shim.lock_count=i.shim_locks; m.shim.event_reg_count=i.shim_events; 0
}
unsafe fn aie2_query_firmware_version(ndev:*mut amdxdna_dev_hdl, v:*mut amdxdna_fw_ver)->i32 { let mut msg=DECLARE_AIE_MSG!(firmware_version,MSG_OP_GET_FIRMWARE_VERSION); let r=aie_send_mgmt_msg_wait(&mut (*ndev).aie,&mut msg); if r!=0{return r;} (*v).major=msg.resp.major;(*v).minor=msg.resp.minor;(*v).sub=msg.resp.sub;(*v).build=msg.resp.build;0 }

unsafe fn aie2_get_context_priority(ndev:*mut amdxdna_dev_hdl, hwctx:*mut amdxdna_hwctx)->u32 { if !AIE_FEATURE_ON!(&(*ndev).aie,AIE2_PREEMPT){return PRIORITY_HIGH;} match (*hwctx).qos.priority { AMDXDNA_QOS_REALTIME_PRIORITY=>PRIORITY_REALTIME, AMDXDNA_QOS_HIGH_PRIORITY=>PRIORITY_HIGH, AMDXDNA_QOS_NORMAL_PRIORITY=>PRIORITY_NORMAL, AMDXDNA_QOS_LOW_PRIORITY=>PRIORITY_LOW, _=>PRIORITY_HIGH } }

unsafe fn aie2_send_host_buf_msgs(ndev:*mut amdxdna_dev_hdl, context_id:u32, mut addr:u64, mut size:u64, initial_opcode:u32)->i32 { let mut msg=DECLARE_AIE_MSG!(map_host_buffer,MSG_OP_MAP_HOST_BUFFER); let xdna=(*ndev).aie.xdna; let chunk_size=(*(*xdna).dev_info).dev_mem_size as u64; if size==0 || size%chunk_size!=0{return -EINVAL;} msg.opcode=initial_opcode; while size!=0 { msg.req.context_id=context_id;msg.req.buf_addr=addr;msg.req.buf_size=chunk_size;let r=aie_send_mgmt_msg_wait(&mut (*ndev).aie,&mut msg);if r!=0{return r;}addr+=chunk_size;size-=chunk_size;msg.opcode=MSG_OP_ADD_HOST_BUFFER;} 0 }
unsafe fn aie2_map_host_buf(n:*mut amdxdna_dev_hdl,c:u32,a:u64,s:u64)->i32{aie2_send_host_buf_msgs(n,c,a,s,MSG_OP_MAP_HOST_BUFFER)}
unsafe fn aie2_add_host_buf(n:*mut amdxdna_dev_hdl,c:u32,a:u64,s:u64)->i32{if !AIE_FEATURE_ON!(&(*n).aie,AIE2_ADD_HOST_BUFFER){return -EOPNOTSUPP;}aie2_send_host_buf_msgs(n,c,a,s,MSG_OP_ADD_HOST_BUFFER)}

unsafe fn aie2_init_npu_chain_req(req:*mut cmd_chain_npu_req,slot_addr:u64,size:usize,cmd_cnt:u32){(*req).flags=0;(*req).reserved=0;(*req).buf_addr=slot_addr;(*req).buf_size=size;(*req).count=cmd_cnt;}
unsafe fn aie2_get_chain_msg_op(op:u32)->u32{match op{ERT_START_CU=>MSG_OP_CHAIN_EXEC_BUFFER_CF,ERT_START_NPU=>MSG_OP_CHAIN_EXEC_DPU,_=>MSG_OP_MAX_OPCODE}}
unsafe fn aie2_get_npu_chain_msg_op(_op:u32)->u32{MSG_OP_CHAIN_EXEC_NPU}

// The remaining operation-table helpers retain the C layout and delegate to the externally supplied driver types.
unsafe fn aie2_msg_init(ndev:*mut amdxdna_dev_hdl){if AIE_FEATURE_ON!(&(*ndev).aie,AIE2_NPU_COMMAND){(*ndev).exec_msg_ops=&mut npu_exec_message_ops}else{(*ndev).exec_msg_ops=&mut legacy_exec_message_ops}}

unsafe fn aie2_destroy_context_req(ndev:*mut amdxdna_dev_hdl,id:u32)->i32{let mut m=DECLARE_AIE_MSG!(destroy_ctx,MSG_OP_DESTROY_CONTEXT);m.req.context_id=id; aie_send_mgmt_msg_wait(&mut (*ndev).aie,&mut m)}
unsafe fn aie2_create_context(_ndev:*mut amdxdna_dev_hdl,_hwctx:*mut amdxdna_hwctx)->i32 { -ENOSYS }
unsafe fn aie2_destroy_context(_ndev:*mut amdxdna_dev_hdl,_hwctx:*mut amdxdna_hwctx)->i32 { 0 }
unsafe fn aie2_query_status(_ndev:*mut amdxdna_dev_hdl,_buf:*mut u8,_size:u32,_cols_filled:*mut u32)->i32 { -ENOSYS }
unsafe fn aie2_query_telemetry(_ndev:*mut amdxdna_dev_hdl,_buf:*mut u8,_size:u32,_header:*mut amdxdna_drm_query_telemetry_header)->i32 { -ENOSYS }
unsafe fn aie2_register_asyn_event_msg(ndev:*mut amdxdna_dev_hdl,addr:dma_addr_t,size:u32,handle:*mut core::ffi::c_void,cb:Option<unsafe extern "C" fn(*mut core::ffi::c_void,*mut core::ffi::c_void,usize)->i32>)->i32 { let mut req=async_event_msg_req::default();req.buf_addr=addr;req.buf_size=size;let mut msg=xdna_mailbox_msg::default();msg.send_data=&mut req as *mut _ as *mut u8;msg.send_size=core::mem::size_of::<async_event_msg_req>();msg.handle=handle;msg.opcode=MSG_OP_REGISTER_ASYNC_EVENT_MSG;msg.notify_cb=cb;xdna_mailbox_send_msg((*ndev).aie.mgmt_chann,&mut msg,TX_TIMEOUT)}
unsafe fn aie2_config_cu(_hwctx:*mut amdxdna_hwctx,_notify_cb:Option<unsafe extern "C" fn(*mut core::ffi::c_void,*mut core::ffi::c_void,usize)->i32>)->i32{-ENOSYS}
unsafe fn aie2_init_exec_req(_req:*mut core::ffi::c_void,_cmd_abo:*mut amdxdna_gem_obj,_size:*mut usize,_msg_op:*mut u32)->i32{-ENOSYS}
unsafe fn aie2_cmdlist_fill_slot(_slot:*mut core::ffi::c_void,_cmd_abo:*mut amdxdna_gem_obj,_size:*mut usize,_cmd_op:*mut u32)->i32{-ENOSYS}
unsafe fn aie2_execbuf(_hwctx:*mut amdxdna_hwctx,_job:*mut amdxdna_sched_job,_notify_cb:Option<unsafe extern "C" fn(*mut core::ffi::c_void,*mut core::ffi::c_void,usize)->i32>)->i32{-ENOSYS}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
