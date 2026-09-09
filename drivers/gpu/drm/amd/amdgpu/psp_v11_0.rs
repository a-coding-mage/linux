/* Translated from psp_v11_0.c. External kernel symbols are supplied by dependencies. */

/* address block */
const SMN_MP1_FIRMWARE_FLAGS: u32 = 0x3010024;
const MM_RLC_GPM_UCODE_ADDR_NV10: u32 = 0x5b61;
const MM_RLC_GPM_UCODE_DATA_NV10: u32 = 0x5b62;
const MM_SDMA0_UCODE_ADDR_NV10: u32 = 0x5880;
const MM_SDMA0_UCODE_DATA_NV10: u32 = 0x5881;
const MEM_TRAIN_SEND_MSG_TIMEOUT_US: i32 = 3000000;
const USBC_PD_POLLING_LIMIT_S: i32 = 240;
const GFX_CMD_USB_PD_USE_LFB: u32 = 0x480;

unsafe fn psp_v11_0_init_microcode(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev;
    let mut ucode_prefix = [0i8; 30];
    let mut err = 0;
    DRM_DEBUG!("\n");
    amdgpu_ucode_ip_version_decode(adev, MP0_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());
    match amdgpu_ip_version(adev, MP0_HWIP, 0) {
        IP_VERSION!(11, 0, 2) | IP_VERSION!(11, 0, 4) => {
            err = psp_init_sos_microcode(psp, ucode_prefix.as_mut_ptr()); if err != 0 { return err; }
            err = psp_init_asd_microcode(psp, ucode_prefix.as_mut_ptr()); if err != 0 { return err; }
            err = psp_init_ta_microcode(psp, ucode_prefix.as_mut_ptr()); (*adev).psp.securedisplay_context.context.bin_desc.size_bytes = 0;
        }
        IP_VERSION!(11, 0, 0) | IP_VERSION!(11, 0, 5) | IP_VERSION!(11, 0, 9) => {
            err = psp_init_sos_microcode(psp, ucode_prefix.as_mut_ptr()); if err != 0 { return err; }
            err = psp_init_asd_microcode(psp, ucode_prefix.as_mut_ptr()); if err != 0 { return err; }
            err = psp_init_ta_microcode(psp, ucode_prefix.as_mut_ptr()); (*adev).psp.securedisplay_context.context.bin_desc.size_bytes = 0;
        }
        IP_VERSION!(11, 0, 7) | IP_VERSION!(11, 0, 11) | IP_VERSION!(11, 0, 12) | IP_VERSION!(11, 0, 13) => {
            err = psp_init_sos_microcode(psp, ucode_prefix.as_mut_ptr()); if err != 0 { return err; }
            err = psp_init_ta_microcode(psp, ucode_prefix.as_mut_ptr());
        }
        IP_VERSION!(11, 5, 0) | IP_VERSION!(11, 5, 2) => {
            err = psp_init_asd_microcode(psp, ucode_prefix.as_mut_ptr()); if err != 0 { return err; }
            err = psp_init_toc_microcode(psp, ucode_prefix.as_mut_ptr());
        }
        _ => { dev_warn!((*adev).dev, "Unsupported MP0 version 0x%08x\n", amdgpu_ip_version(adev, MP0_HWIP, 0)); return -EINVAL; }
    }
    err
}

unsafe fn psp_v11_wait_for_tos_unload(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev;
    let mut sol_reg1; let mut sol_reg2;
    for _retry_loop in 0..20 {
        sol_reg1 = RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_81); usleep_range(1000, 2000);
        sol_reg2 = RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_81);
        if sol_reg1 == sol_reg2 { return 0; }
    }
    dev_err!((*adev).dev, "TOS unload failed, C2PMSG_33: %x C2PMSG_81: %x", RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_33), RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_81));
    -ETIME
}

unsafe fn psp_v11_0_wait_for_bootloader(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev;
    if ((*adev).in_s4 || (*adev).in_s3) && ((*adev).flags & AMD_IS_APU) == 0 && amdgpu_in_reset(adev) { return psp_v11_wait_for_tos_unload(psp); }
    let mut ret = 0;
    for _ in 0..20 {
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_35), 0x80000000, 0x8000FFFF, PSP_WAITREG_NOVERBOSE);
        if ret == 0 { return 0; }
    }
    ret
}

unsafe fn psp_v11_0_is_sos_alive(psp: *mut psp_context) -> bool {
    RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_81) != 0
}

unsafe fn psp_v11_0_bootloader_load_component(psp: *mut psp_context, bin_desc: *mut psp_bin_desc, bl_cmd: psp_bootloader_cmd) -> i32 {
    if psp_v11_0_is_sos_alive(psp) { return 0; }
    let mut ret = psp_v11_0_wait_for_bootloader(psp); if ret != 0 { return ret; }
    ret = psp_copy_fw(psp, (*bin_desc).start_addr, (*bin_desc).size_bytes); if ret != 0 { return ret; }
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_36, ((*psp).fw_pri_mc_addr >> 20) as u32);
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_35, bl_cmd as u32);
    psp_v11_0_wait_for_bootloader(psp)
}
unsafe fn psp_v11_0_bootloader_load_kdb(psp: *mut psp_context) -> i32 { psp_v11_0_bootloader_load_component(psp, &mut (*psp).kdb, PSP_BL__LOAD_KEY_DATABASE) }
unsafe fn psp_v11_0_bootloader_load_spl(psp: *mut psp_context) -> i32 { psp_v11_0_bootloader_load_component(psp, &mut (*psp).spl, PSP_BL__LOAD_TOS_SPL_TABLE) }
unsafe fn psp_v11_0_bootloader_load_sysdrv(psp: *mut psp_context) -> i32 { psp_v11_0_bootloader_load_component(psp, &mut (*psp).sys, PSP_BL__LOAD_SYSDRV) }

unsafe fn psp_v11_0_bootloader_load_sos(psp: *mut psp_context) -> i32 {
    if psp_v11_0_is_sos_alive(psp) { return 0; }
    let mut ret = psp_v11_0_wait_for_bootloader(psp); if ret != 0 { return ret; }
    ret = psp_copy_fw(psp, (*psp).sos.start_addr, (*psp).sos.size_bytes); if ret != 0 { return ret; }
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_36, ((*psp).fw_pri_mc_addr >> 20) as u32);
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_35, PSP_BL__LOAD_SOSDRV);
    mdelay(20);
    psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_81), RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_81), 0, PSP_WAITREG_CHANGED)
}

unsafe fn psp_v11_0_ring_stop(psp: *mut psp_context, _ring_type: psp_ring_type) -> i32 {
    let adev = (*psp).adev;
    if amdgpu_sriov_vf(adev) { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_DESTROY_GPCOM_RING); } else { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, GFX_CTRL_CMD_ID_DESTROY_RINGS); }
    mdelay(20);
    if amdgpu_sriov_vf(adev) { psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_101), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0) } else { psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0) }
}

unsafe fn psp_v11_0_ring_create(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let adev = (*psp).adev; let ring = &mut (*psp).km_ring; let mut ret;
    if amdgpu_sriov_vf(adev) {
        ring.ring_wptr = 0; ret = psp_v11_0_ring_stop(psp, ring_type); if ret != 0 { DRM_ERROR!("psp_v11_0_ring_stop_sriov failed!\n"); return ret; }
        WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_102, lower_32_bits(ring.ring_mem_mc_addr)); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_103, upper_32_bits(ring.ring_mem_mc_addr));
        WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_INIT_GPCOM_RING); mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_101), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0);
    } else {
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), MBOX_TOS_READY_FLAG, MBOX_TOS_READY_MASK, 0); if ret != 0 { DRM_ERROR!("Failed to wait for sOS ready for ring creation\n"); return ret; }
        WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_69, lower_32_bits(ring.ring_mem_mc_addr)); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_70, upper_32_bits(ring.ring_mem_mc_addr)); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_71, ring.ring_size);
        WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, (ring_type as u32) << 16); mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0);
    } ret
}

unsafe fn psp_v11_0_ring_destroy(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let adev = (*psp).adev; let ring = &mut (*psp).km_ring; let ret = psp_v11_0_ring_stop(psp, ring_type); if ret != 0 { DRM_ERROR!("Fail to stop psp ring\n"); }
    amdgpu_bo_free_kernel(&mut (*adev).firmware.rbuf, &mut ring.ring_mem_mc_addr, &mut ring.ring_mem as *mut _ as *mut *mut core::ffi::c_void); ret
}

unsafe fn psp_v11_0_mode1_reset(psp: *mut psp_context) -> i32 { let adev = (*psp).adev; let offset = SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64); let ret = psp_wait_for(psp, offset, MBOX_TOS_READY_FLAG, MBOX_TOS_READY_MASK, 0); if ret != 0 { drm_info!(adev_to_drm(adev), "psp is not working correctly before mode1 reset!\n"); return -EINVAL; } WREG32!(offset, GFX_CTRL_CMD_ID_MODE1_RST); msleep(500); 0 }

unsafe fn psp_v11_0_memory_training_send_msg(psp: *mut psp_context, msg: i32) -> i32 {
    let adev = (*psp).adev; let data_32 = ((*psp).mem_train_ctx.c2p_train_data_offset >> 20) as u32; WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_36, data_32); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_35, msg);
    let max_wait = MEM_TRAIN_SEND_MSG_TIMEOUT_US / (*adev).usec_timeout; let mut i = 0; let mut ret = 0;
    while i < max_wait { ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_35), 0x80000000, 0x80000000, PSP_WAITREG_NOVERBOSE); if ret == 0 { break; } i += 1; }
    if i < max_wait { ret = 0; } else { ret = -ETIME; }
    DRM_DEBUG!("training %s %s, cost %d @ %d ms\n", if msg == PSP_BL__DRAM_SHORT_TRAIN { "short" } else { "long" }, if ret == 0 { "succeed" } else { "failed" }, i, (*adev).usec_timeout / 1000); ret
}

unsafe fn psp_v11_0_memory_training(psp: *mut psp_context, mut ops: u32) -> i32 {
    let ctx = &mut (*psp).mem_train_ctx; let pcache = ctx.sys_cache as *mut u32; let adev = (*psp).adev; let mut p2c_header = [0u32; 4]; let mut sz; let mut buf; let mut ret; let mut idx = 0;
    if ctx.init == PSP_MEM_TRAIN_NOT_SUPPORT { DRM_DEBUG!("Memory training is not supported.\n"); return 0; } else if ctx.init != PSP_MEM_TRAIN_INIT_SUCCESS { DRM_ERROR!("Memory training initialization failure.\n"); return -EINVAL; }
    if psp_v11_0_is_sos_alive(psp) { DRM_DEBUG!("SOS is alive, skip memory training.\n"); return 0; }
    amdgpu_device_vram_access(adev, ctx.p2c_train_data_offset, p2c_header.as_mut_ptr(), core::mem::size_of_val(&p2c_header), false);
    if (ops & PSP_MEM_TRAIN_SEND_SHORT_MSG) != 0 { ops |= PSP_MEM_TRAIN_RESTORE; }
    if (ops & PSP_MEM_TRAIN_RESTORE) != 0 && *pcache != MEM_TRAIN_SYSTEM_SIGNATURE { ops |= PSP_MEM_TRAIN_SAVE; }
    if p2c_header[0] == MEM_TRAIN_SYSTEM_SIGNATURE && !(*pcache == MEM_TRAIN_SYSTEM_SIGNATURE && *pcache.add(3) == p2c_header[3]) { ops |= PSP_MEM_TRAIN_SAVE; }
    if (ops & PSP_MEM_TRAIN_SAVE) != 0 && p2c_header[0] != MEM_TRAIN_SYSTEM_SIGNATURE { ops |= PSP_MEM_TRAIN_SEND_LONG_MSG; }
    if (ops & PSP_MEM_TRAIN_SEND_LONG_MSG) != 0 { ops &= !PSP_MEM_TRAIN_SEND_SHORT_MSG; ops |= PSP_MEM_TRAIN_SAVE; }
    if (ops & PSP_MEM_TRAIN_SEND_LONG_MSG) != 0 {
        sz = BIST_MEM_TRAINING_ENCROACHED_SIZE; if (*adev).gmc.visible_vram_size < sz || (*adev).mman.aper_base_kaddr.is_null() { DRM_ERROR!("visible_vram_size or aper_base_kaddr is not initialized.\n"); return -EINVAL; }
        buf = vmalloc(sz); if buf.is_null() { DRM_ERROR!("failed to allocate system memory.\n"); return -ENOMEM; }
        if drm_dev_enter(adev_to_drm(adev), &mut idx) { memcpy_fromio(buf, (*adev).mman.aper_base_kaddr, sz); ret = psp_v11_0_memory_training_send_msg(psp, PSP_BL__DRAM_LONG_TRAIN); if ret != 0 { vfree(buf); drm_dev_exit(idx); return ret; } memcpy_toio((*adev).mman.aper_base_kaddr, buf, sz); amdgpu_device_flush_hdp(adev, core::ptr::null_mut()); vfree(buf); drm_dev_exit(idx); } else { vfree(buf); return -ENODEV; }
    }
    if (ops & PSP_MEM_TRAIN_SAVE) != 0 { amdgpu_device_vram_access(adev, ctx.p2c_train_data_offset, ctx.sys_cache, ctx.train_data_size, false); }
    if (ops & PSP_MEM_TRAIN_RESTORE) != 0 { amdgpu_device_vram_access(adev, ctx.c2p_train_data_offset, ctx.sys_cache, ctx.train_data_size, true); }
    if (ops & PSP_MEM_TRAIN_SEND_SHORT_MSG) != 0 { ret = psp_v11_0_memory_training_send_msg(psp, if amdgpu_force_long_training > 0 { PSP_BL__DRAM_LONG_TRAIN } else { PSP_BL__DRAM_SHORT_TRAIN }); if ret != 0 { DRM_ERROR!("send training msg failed.\n"); return ret; } }
    ctx.training_cnt += 1; 0
}

unsafe fn psp_v11_0_ring_get_wptr(psp: *mut psp_context) -> u32 { if amdgpu_sriov_vf((*psp).adev) { (*psp).km_ring.ring_wptr } else { RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_67) } }
unsafe fn psp_v11_0_ring_set_wptr(psp: *mut psp_context, value: u32) { if amdgpu_sriov_vf((*psp).adev) { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_102, value); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_CONSUME_CMD); (*psp).km_ring.ring_wptr = value; } else { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_67, value); } }

unsafe fn psp_v11_0_load_usbc_pd_fw(psp: *mut psp_context, fw_pri_mc_addr: u64) -> i32 {
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_36, (fw_pri_mc_addr >> 20) as u32); let mut ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_35), 0x80000000, 0x80000000, 0); if ret != 0 { return ret; }
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_35, GFX_CMD_USB_PD_USE_LFB << 16); let mut i = 0; let mut reg_status;
    loop { msleep(1000); reg_status = RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_35); if reg_status & 0x80000000 != 0 { break; } i += 1; if i >= USBC_PD_POLLING_LIMIT_S { return -ETIME; } }
    if reg_status & 0xFFFF != 0 { DRM_ERROR!("Address load failed - MP0_SMN_C2PMSG_35.Bits [15:0] = 0x%04x\n", reg_status & 0xFFFF); return -EIO; } ret = 0; ret
}
unsafe fn psp_v11_0_read_usbc_pd_fw(psp: *mut psp_context, fw_ver: *mut u32) -> i32 { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_35, C2PMSG_CMD_GFX_USB_PD_FW_VER); let ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_35), 0x80000000, 0x80000000, 0); if ret == 0 { *fw_ver = RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_36); } ret }

static psp_v11_0_funcs: psp_funcs = psp_funcs { init_microcode: Some(psp_v11_0_init_microcode), bootloader_load_kdb: Some(psp_v11_0_bootloader_load_kdb), bootloader_load_spl: Some(psp_v11_0_bootloader_load_spl), bootloader_load_sysdrv: Some(psp_v11_0_bootloader_load_sysdrv), bootloader_load_sos: Some(psp_v11_0_bootloader_load_sos), ring_create: Some(psp_v11_0_ring_create), ring_stop: Some(psp_v11_0_ring_stop), ring_destroy: Some(psp_v11_0_ring_destroy), mode1_reset: Some(psp_v11_0_mode1_reset), mem_training: Some(psp_v11_0_memory_training), ring_get_wptr: Some(psp_v11_0_ring_get_wptr), ring_set_wptr: Some(psp_v11_0_ring_set_wptr), load_usbc_pd_fw: Some(psp_v11_0_load_usbc_pd_fw), read_usbc_pd_fw: Some(psp_v11_0_read_usbc_pd_fw), wait_for_bootloader: Some(psp_v11_0_wait_for_bootloader) };
pub unsafe fn psp_v11_0_set_psp_funcs(psp: *mut psp_context) { (*psp).funcs = &psp_v11_0_funcs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
