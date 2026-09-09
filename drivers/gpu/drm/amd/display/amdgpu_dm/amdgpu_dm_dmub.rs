// SPDX-License-Identifier: MIT
/* Direct low-level translation of amdgpu_dm_dmub.c. */

// External kernel/DC declarations are supplied by the surrounding translation unit.

pub unsafe extern "C" fn dm_dmub_aux_setconfig_callback(adev: *mut amdgpu_device, notify: *const dmub_notification) {
    if !(*adev).dm.dmub_notify.is_null() { memcpy((*adev).dm.dmub_notify as *mut _, notify as *const _, core::mem::size_of::<dmub_notification>()); }
    if (*notify).type_ == DMUB_NOTIFICATION_AUX_REPLY { complete(&mut (*adev).dm.dmub_aux_transfer_done); }
}

pub unsafe extern "C" fn dm_dmub_aux_fused_io_callback(adev: *mut amdgpu_device, notify: *const dmub_notification) {
    if adev.is_null() || notify.is_null() { ASSERT(false); return; }
    let req = &(*notify).fused_request;
    let ddc_line = req.u.aux.ddc_line;
    if ddc_line as usize >= ARRAY_SIZE((*adev).dm.fused_io) { ASSERT(false); return; }
    let sync = &mut (*adev).dm.fused_io[ddc_line as usize];
    memcpy(sync.reply_data.as_mut_ptr() as *mut _, req as *const _ as *const _, core::mem::size_of_val(req));
    complete(&mut sync.replied);
}

pub unsafe extern "C" fn dm_register_dmub_notify_callback(adev: *mut amdgpu_device, type_: dmub_notification_type, callback: dmub_notify_interrupt_callback_t, offload: bool) -> bool {
    if callback.is_none() || type_ as usize >= ARRAY_SIZE((*adev).dm.dmub_thread_offload) { return false; }
    (*adev).dm.dmub_callback[type_ as usize] = callback;
    (*adev).dm.dmub_thread_offload[type_ as usize] = offload;
    true
}

pub unsafe extern "C" fn dm_dmub_hw_init(adev: *mut amdgpu_device) -> i32 {
    let dmub_srv = (*adev).dm.dmub_srv;
    let fb_info = (*adev).dm.dmub_fb_info;
    let dmub_fw = (*adev).dm.dmub_fw;
    let dc = (*adev).dm.dc;
    if dmub_srv.is_null() { return 0; }
    if fb_info.is_null() { drm_err(adev_to_drm(adev), "No framebuffer info for DMUB service.\n"); return -EINVAL; }
    if dmub_fw.is_null() { drm_err(adev_to_drm(adev), "No firmware provided for DMUB.\n"); return -EINVAL; }
    let ctx = (*dc).ctx;
    if let Some(f) = (*dmub_srv).hw_funcs.init_reg_offsets { f(dmub_srv, ctx); }
    let mut has_hw_support = false;
    let mut status = dmub_srv_has_hw_support(dmub_srv, &mut has_hw_support);
    if status != DMUB_STATUS_OK { drm_err(adev_to_drm(adev), "Error checking HW support for DMUB: %d\n", status); return -EINVAL; }
    if !has_hw_support { drm_info(adev_to_drm(adev), "DMUB unsupported on ASIC\n"); return 0; }
    status = dmub_srv_hw_reset(dmub_srv);
    if status != DMUB_STATUS_OK { drm_warn(adev_to_drm(adev), "Error resetting DMUB HW: %d\n", status); }
    let hdr = (*dmub_fw).data as *const dmcub_firmware_header_v1_0;
    let base = (*dmub_fw).data.add(le32_to_cpu((*hdr).header.ucode_array_offset_bytes) as usize);
    let fw_inst_const = base.add(PSP_HEADER_BYTES_256 as usize);
    let fw_bss_data = base.add(le32_to_cpu((*hdr).inst_const_bytes) as usize);
    let fw_inst_const_size = (*adev).dm.fw_inst_size;
    let fw_bss_data_size = le32_to_cpu((*hdr).bss_data_bytes);
    if (*adev).firmware.load_type != AMDGPU_FW_LOAD_PSP { memcpy((*fb_info).fb[DMUB_WINDOW_0_INST_CONST].cpu_addr, fw_inst_const as *const _, fw_inst_const_size as usize); }
    if fw_bss_data_size != 0 { memcpy((*fb_info).fb[DMUB_WINDOW_2_BSS_DATA].cpu_addr, fw_bss_data as *const _, fw_bss_data_size as usize); }
    memcpy((*fb_info).fb[DMUB_WINDOW_3_VBIOS].cpu_addr, (*adev).bios, (*adev).bios_size as usize);
    for w in [DMUB_WINDOW_4_MAILBOX, DMUB_WINDOW_5_TRACEBUFF, DMUB_WINDOW_6_FW_STATE, DMUB_WINDOW_SHARED_STATE] { memset((*fb_info).fb[w].cpu_addr, 0, (*fb_info).fb[w].size as usize); }
    let mut hw_params: dmub_srv_hw_params = core::mem::zeroed();
    if (*adev).firmware.load_type != AMDGPU_FW_LOAD_PSP { hw_params.load_inst_const = true; }
    if !(*dc).res_pool.dmcu.is_null() { hw_params.psp_version = (*(*dc).res_pool.dmcu).psp_version; }
    hw_params.fb_info = fb_info;
    if (*dc).caps.is_apu && (*dc).res_pool.usb4_dpia_count != 0 && !(*dc).debug.dpia_debug.bits.disable_dpia { hw_params.dpia_supported = true; hw_params.disable_dpia = (*dc).debug.dpia_debug.bits.disable_dpia; hw_params.enable_non_transparent_setconfig = (*dc).config.consolidated_dpia_dp_lt; hw_params.disable_dpia_bw_allocation = !(*dc).config.usb4_bw_alloc_support; }
    match amdgpu_ip_version(adev, DCE_HWIP, 0) { IP_VERSION(3,5,0)|IP_VERSION(3,5,1)|IP_VERSION(3,6,0)|IP_VERSION(4,2,0)|IP_VERSION(4,2,1) => { hw_params.ips_sequential_ono = (*adev).external_rev_id > 0x10; hw_params.lower_hbr3_phy_ssc = true; }, _ => {} }
    status = dmub_srv_hw_init(dmub_srv, &mut hw_params); if status != DMUB_STATUS_OK { drm_err(adev_to_drm(adev), "Error initializing DMUB HW: %d\n", status); return -EINVAL; }
    status = dmub_srv_wait_for_auto_load(dmub_srv, 100000); if status != DMUB_STATUS_OK { drm_warn(adev_to_drm(adev), "Wait for DMUB auto-load failed: %d\n", status); }
    let dmcu = (*dc).res_pool.dmcu; let abm = (*dc).res_pool.abm;
    if !dmcu.is_null() && !abm.is_null() { ((*dmcu).funcs).dmcu_init(dmcu); (*abm).dmcu_is_running = ((*dmcu).funcs).is_dmcu_initialized(dmcu); }
    if (*ctx).dmub_srv.is_null() { (*ctx).dmub_srv = dc_dmub_srv_create(dc, dmub_srv); }
    if (*ctx).dmub_srv.is_null() { drm_err(adev_to_drm(adev), "Couldn't allocate DC DMUB server!\n"); return -ENOMEM; }
    0
}

pub unsafe extern "C" fn dm_dmub_hw_resume(adev: *mut amdgpu_device) { if (*adev).dm.dmub_srv.is_null() { return; } let mut init=false; let mut s=dmub_srv_is_hw_init((*adev).dm.dmub_srv,&mut init); if s==DMUB_STATUS_OK && init { s=dmub_srv_wait_for_auto_load((*adev).dm.dmub_srv,100000); } else { let r=dm_dmub_hw_init(adev); if r!=0 { drm_err(adev_to_drm(adev),"DMUB interface failed to initialize: status=%d\n",r); } } }

pub unsafe extern "C" fn dm_get_default_ips_mode(adev: *mut amdgpu_device) -> dmub_ips_disable_type { match amdgpu_ip_version(adev,DCE_HWIP,0) { IP_VERSION(3,5,0)|IP_VERSION(3,6,0)|IP_VERSION(3,5,1)=>DMUB_IPS_RCG_IN_ACTIVE_IPS2_IN_OFF, v if v<IP_VERSION(3,5,0)=>DMUB_IPS_DISABLE_ALL, _=>DMUB_IPS_ENABLE } }

pub unsafe extern "C" fn amdgpu_dm_dmub_reg_read(ctx:*mut core::ffi::c_void,address:u32)->u32 { dm_read_reg((*((ctx as *mut amdgpu_device))).dm.dc.ctx,address) }
pub unsafe extern "C" fn amdgpu_dm_dmub_reg_write(ctx:*mut core::ffi::c_void,address:u32,value:u32) { dm_write_reg((*((ctx as *mut amdgpu_device))).dm.dc.ctx,address,value); }

pub unsafe extern "C" fn dm_execute_dmub_cmd(ctx:*const dc_context,cmd:*mut dmub_rb_cmd,wait_type:dm_dmub_wait_type)->bool { let adev=(*ctx).driver_context; let _guard=spinlock_irqsave_guard(&mut (*adev).dm.dmub_lock); dc_dmub_srv_cmd_run((*ctx).dmub_srv,cmd,wait_type) }
pub unsafe extern "C" fn dm_execute_dmub_cmd_list(ctx:*const dc_context,count:u32,cmd:*mut dmub_rb_cmd,wait_type:dm_dmub_wait_type)->bool { let adev=(*ctx).driver_context; let _guard=spinlock_irqsave_guard(&mut (*adev).dm.dmub_lock); dc_dmub_srv_cmd_run_list((*ctx).dmub_srv,count,cmd,wait_type) }

pub unsafe extern "C" fn dm_init_microcode(adev:*mut amdgpu_device)->i32 {
    let fw = match amdgpu_ip_version(adev,DCE_HWIP,0) {
        IP_VERSION(2,1,0)=>FIRMWARE_RENOIR_DMUB, IP_VERSION(3,0,0)=>FIRMWARE_NAVY_FLOUNDER_DMUB,
        IP_VERSION(3,0,1)=>FIRMWARE_VANGOGH_DMUB, IP_VERSION(3,0,2)=>FIRMWARE_DIMGREY_CAVEFISH_DMUB,
        IP_VERSION(3,0,3)=>FIRMWARE_BEIGE_GOBY_DMUB, IP_VERSION(3,1,2)|IP_VERSION(3,1,3)=>FIRMWARE_YELLOW_CARP_DMUB,
        IP_VERSION(3,1,4)=>FIRMWARE_DCN_314_DMUB, IP_VERSION(3,1,5)=>FIRMWARE_DCN_315_DMUB,
        IP_VERSION(3,1,6)=>FIRMWARE_DCN316_DMUB, IP_VERSION(3,2,0)=>FIRMWARE_DCN_V3_2_0_DMCUB,
        IP_VERSION(3,2,1)=>FIRMWARE_DCN_V3_2_1_DMCUB, IP_VERSION(3,5,0)=>FIRMWARE_DCN_35_DMUB,
        IP_VERSION(3,5,1)=>FIRMWARE_DCN_351_DMUB, IP_VERSION(3,6,0)=>FIRMWARE_DCN_36_DMUB,
        IP_VERSION(4,0,1)=>FIRMWARE_DCN_401_DMUB, IP_VERSION(4,2,0)=>FIRMWARE_DCN_42_DMUB,
        IP_VERSION(4,2,1)=>FIRMWARE_DCN_42B_DMUB, IP_VERSION(6,0,0)=>FIRMWARE_DCN_60_DMUB, _=>return 0,
    }; amdgpu_ucode_request(adev,&mut (*adev).dm.dmub_fw,AMDGPU_UCODE_REQUIRED,"%s",fw)
}

pub unsafe extern "C" fn dm_dmub_sw_init(adev:*mut amdgpu_device)->i32 { if (*adev).dm.dmub_fw.is_null(){return -EINVAL;} (*adev).dm.dmub_srv=kzalloc_obj(); if (*adev).dm.dmub_srv.is_null(){return -ENOMEM;} 0 }

pub unsafe extern "C" fn amdgpu_dm_process_dmub_aux_transfer_sync(ctx:*mut dc_context,_link_index:u32,payload:*mut aux_payload,operation_result:*mut aux_return_code_type)->i32 { let adev=(*ctx).driver_context; mutex_lock(&mut (*adev).dm.dpia_aux_lock); let ok=dc_process_dmub_aux_transfer_async((*ctx).dc,_link_index,payload); if !ok {*operation_result=AUX_RET_ERROR_ENGINE_ACQUIRE; mutex_unlock(&mut (*adev).dm.dpia_aux_lock);return -1;} if !wait_for_completion_timeout(&mut (*adev).dm.dmub_aux_transfer_done,10*HZ){*operation_result=AUX_RET_ERROR_TIMEOUT;mutex_unlock(&mut (*adev).dm.dpia_aux_lock);return -1;} let n=(*adev).dm.dmub_notify; *operation_result=(*n).result; mutex_unlock(&mut (*adev).dm.dpia_aux_lock); (*n).aux_reply.length as i32 }

pub unsafe extern "C" fn amdgpu_dm_process_dmub_set_config_sync(ctx:*mut dc_context,link_index:u32,payload:*mut set_config_cmd_payload,result:*mut set_config_status)->i32 { let adev=(*ctx).driver_context; mutex_lock(&mut (*adev).dm.dpia_aux_lock); let done=dc_process_dmub_set_config_async((*ctx).dc,link_index,payload,(*adev).dm.dmub_notify); let ok=done||wait_for_completion_timeout(&mut (*adev).dm.dmub_aux_transfer_done,10*HZ); if ok {*result=(*(*adev).dm.dmub_notify).sc_status;} else {*result=SET_CONFIG_UNKNOWN_ERROR;} if !done{reinit_completion(&mut (*adev).dm.dmub_aux_transfer_done);} mutex_unlock(&mut (*adev).dm.dpia_aux_lock); if ok{0}else{-1} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
