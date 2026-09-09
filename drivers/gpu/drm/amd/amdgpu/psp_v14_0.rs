/* Translated from psp_v14_0.c. External kernel and driver symbols are supplied by other units. */

pub const USBC_PD_POLLING_LIMIT_S: i32 = 240;
pub const GFX_CMD_USB_PD_USE_LFB: u32 = 0x480;
pub const MBOX_READY_MASK: u32 = 0x80000000;
pub const MBOX_STATUS_MASK: u32 = 0x0000ffff;
pub const MBOX_COMMAND_MASK: u32 = 0x00ff0000;
pub const MBOX_READY_FLAG: u32 = 0x80000000;
pub const C2PMSG_CMD_SPI_UPDATE_ROM_IMAGE_ADDR_LO: i32 = 0x2;
pub const C2PMSG_CMD_SPI_UPDATE_ROM_IMAGE_ADDR_HI: i32 = 0x3;
pub const C2PMSG_CMD_SPI_UPDATE_FLASH_IMAGE: i32 = 0x4;
pub const MEM_TRAIN_SEND_MSG_TIMEOUT_US: i32 = 3000000;

unsafe fn psp_v14_0_init_microcode(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev;
    let mut ucode_prefix = [0i8; 30];
    let mut err = 0;
    amdgpu_ucode_ip_version_decode(adev, MP0_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());
    match amdgpu_ip_version(adev, MP0_HWIP, 0) {
        x if x == IP_VERSION(14, 0, 2) || x == IP_VERSION(14, 0, 3) => {
            err = psp_init_sos_microcode(psp, ucode_prefix.as_mut_ptr()); if err != 0 { return err; }
            err = psp_init_ta_microcode(psp, ucode_prefix.as_mut_ptr()); if err != 0 { return err; }
        }
        x if x == IP_VERSION(14, 0, 5) => {
            err = psp_init_toc_microcode(psp, ucode_prefix.as_mut_ptr()); if err != 0 { return err; }
            err = psp_init_ta_microcode(psp, ucode_prefix.as_mut_ptr()); if err != 0 { return err; }
        }
        _ => { dev_warn((*adev).dev, "Unsupported MP0 version 0x%08x\n", amdgpu_ip_version(adev, MP0_HWIP, 0)); return -EINVAL; }
    }
    0
}

unsafe fn psp_v14_0_is_sos_alive(psp: *mut psp_context) -> bool {
    RREG32_SOC15((*psp).adev, MP0, 0, regMPASP_SMN_C2PMSG_81) != 0
}

unsafe fn psp_v14_0_wait_for_bootloader(psp: *mut psp_context) -> i32 {
    let mut ret = 0;
    for _ in 0..10 { ret = psp_wait_for(psp, SOC15_REG_OFFSET(MP0, 0, regMPASP_SMN_C2PMSG_35), 0x80000000, 0x80000000, PSP_WAITREG_NOVERBOSE); if ret == 0 { return 0; } }
    ret
}

unsafe fn psp_v14_0_bootloader_load_component(psp: *mut psp_context, bin_desc: *mut psp_bin_desc, bl_cmd: psp_bootloader_cmd) -> i32 {
    if psp_v14_0_is_sos_alive(psp) { return 0; }
    let mut ret = psp_v14_0_wait_for_bootloader(psp); if ret != 0 { return ret; }
    ret = psp_copy_fw(psp, (*bin_desc).start_addr, (*bin_desc).size_bytes); if ret != 0 { return ret; }
    WREG32_SOC15((*psp).adev, MP0, 0, regMPASP_SMN_C2PMSG_36, ((*psp).fw_pri_mc_addr >> 20) as u32);
    WREG32_SOC15((*psp).adev, MP0, 0, regMPASP_SMN_C2PMSG_35, bl_cmd as u32);
    psp_v14_0_wait_for_bootloader(psp)
}

unsafe fn psp_v14_0_bootloader_load_kdb(p: *mut psp_context)->i32 { psp_v14_0_bootloader_load_component(p,&mut (*p).kdb,PSP_BL__LOAD_KEY_DATABASE) }
unsafe fn psp_v14_0_bootloader_load_spl(p: *mut psp_context)->i32 { psp_v14_0_bootloader_load_component(p,&mut (*p).spl,PSP_BL__LOAD_TOS_SPL_TABLE) }
unsafe fn psp_v14_0_bootloader_load_sysdrv(p: *mut psp_context)->i32 { psp_v14_0_bootloader_load_component(p,&mut (*p).sys,PSP_BL__LOAD_SYSDRV) }
unsafe fn psp_v14_0_bootloader_load_soc_drv(p: *mut psp_context)->i32 { psp_v14_0_bootloader_load_component(p,&mut (*p).soc_drv,PSP_BL__LOAD_SOCDRV) }
unsafe fn psp_v14_0_bootloader_load_intf_drv(p: *mut psp_context)->i32 { psp_v14_0_bootloader_load_component(p,&mut (*p).intf_drv,PSP_BL__LOAD_INTFDRV) }
unsafe fn psp_v14_0_bootloader_load_dbg_drv(p: *mut psp_context)->i32 { psp_v14_0_bootloader_load_component(p,&mut (*p).dbg_drv,PSP_BL__LOAD_HADDRV) }
unsafe fn psp_v14_0_bootloader_load_ras_drv(p: *mut psp_context)->i32 { psp_v14_0_bootloader_load_component(p,&mut (*p).ras_drv,PSP_BL__LOAD_RASDRV) }
unsafe fn psp_v14_0_bootloader_load_ipkeymgr_drv(p: *mut psp_context)->i32 { psp_v14_0_bootloader_load_component(p,&mut (*p).ipkeymgr_drv,PSP_BL__LOAD_IPKEYMGRDRV) }

unsafe fn psp_v14_0_bootloader_load_sos(psp:*mut psp_context)->i32 {
    if psp_v14_0_is_sos_alive(psp) { return 0; }
    let mut ret=psp_v14_0_wait_for_bootloader(psp); if ret!=0{return ret;}
    ret=psp_copy_fw(psp,(*psp).sos.start_addr,(*psp).sos.size_bytes); if ret!=0{return ret;}
    WREG32_SOC15((*psp).adev,MP0,0,regMPASP_SMN_C2PMSG_36,((*psp).fw_pri_mc_addr>>20) as u32);
    WREG32_SOC15((*psp).adev,MP0,0,regMPASP_SMN_C2PMSG_35,PSP_BL__LOAD_SOSDRV as u32); mdelay(20);
    psp_wait_for(psp,SOC15_REG_OFFSET(MP0,0,regMPASP_SMN_C2PMSG_81),RREG32_SOC15((*psp).adev,MP0,0,regMPASP_SMN_C2PMSG_81),0,PSP_WAITREG_CHANGED)
}

unsafe fn psp_v14_0_ring_stop(psp:*mut psp_context, _ring_type:psp_ring_type)->i32 { let a=(*psp).adev; let r=if amdgpu_sriov_vf(a){regMPASP_SMN_C2PMSG_101}else{regMPASP_SMN_C2PMSG_64}; let c=if amdgpu_sriov_vf(a){GFX_CTRL_CMD_ID_DESTROY_GPCOM_RING}else{GFX_CTRL_CMD_ID_DESTROY_RINGS}; WREG32_SOC15(a,MP0,0,r,c); mdelay(20); psp_wait_for(psp,SOC15_REG_OFFSET(MP0,0,r),MBOX_TOS_RESP_FLAG,MBOX_TOS_RESP_MASK,0) }

unsafe fn psp_v14_0_ring_create(psp:*mut psp_context, ring_type:psp_ring_type)->i32 { let a=(*psp).adev; let r=&mut (*psp).km_ring; if amdgpu_sriov_vf(a){let mut x=psp_v14_0_ring_stop(psp,ring_type);if x!=0{return x;} WREG32_SOC15(a,MP0,0,regMPASP_SMN_C2PMSG_102,lower_32_bits(r.ring_mem_mc_addr));WREG32_SOC15(a,MP0,0,regMPASP_SMN_C2PMSG_103,upper_32_bits(r.ring_mem_mc_addr));WREG32_SOC15(a,MP0,0,regMPASP_SMN_C2PMSG_101,GFX_CTRL_CMD_ID_INIT_GPCOM_RING);mdelay(20);x=psp_wait_for(psp,SOC15_REG_OFFSET(MP0,0,regMPASP_SMN_C2PMSG_101),MBOX_TOS_RESP_FLAG,MBOX_TOS_RESP_MASK,0);x}else{let mut x=psp_wait_for(psp,SOC15_REG_OFFSET(MP0,0,regMPASP_SMN_C2PMSG_64),MBOX_TOS_READY_FLAG,MBOX_TOS_READY_MASK,0);if x!=0{return x;}WREG32_SOC15(a,MP0,0,regMPASP_SMN_C2PMSG_69,lower_32_bits(r.ring_mem_mc_addr));WREG32_SOC15(a,MP0,0,regMPASP_SMN_C2PMSG_70,upper_32_bits(r.ring_mem_mc_addr));WREG32_SOC15(a,MP0,0,regMPASP_SMN_C2PMSG_71,r.ring_size);WREG32_SOC15(a,MP0,0,regMPASP_SMN_C2PMSG_64,(ring_type as u32)<<16);mdelay(20);x=psp_wait_for(psp,SOC15_REG_OFFSET(MP0,0,regMPASP_SMN_C2PMSG_64),MBOX_TOS_RESP_FLAG,MBOX_TOS_RESP_MASK,0);x} }

unsafe fn psp_v14_0_ring_destroy(psp:*mut psp_context, t:psp_ring_type)->i32 {let ret=psp_v14_0_ring_stop(psp,t);amdgpu_bo_free_kernel(&mut (*psp).adev.firmware.rbuf,&mut (*psp).km_ring.ring_mem_mc_addr,&mut (*psp).km_ring.ring_mem as *mut _ as *mut *mut core::ffi::c_void);ret}
unsafe fn psp_v14_0_ring_get_wptr(p:*mut psp_context)->u32 {if amdgpu_sriov_vf((*p).adev){RREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_102)}else{RREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_67)}}
unsafe fn psp_v14_0_ring_set_wptr(p:*mut psp_context,v:u32){if amdgpu_sriov_vf((*p).adev){WREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_102,v);WREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_101,GFX_CTRL_CMD_ID_CONSUME_CMD)}else{WREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_67,v)}}

unsafe fn psp_v14_0_memory_training_send_msg(p:*mut psp_context,msg:i32)->i32 {let a=(*p).adev;WREG32_SOC15(a,MP0,0,regMPASP_SMN_C2PMSG_36,((*p).mem_train_ctx.c2p_train_data_offset>>20) as u32);WREG32_SOC15(a,MP0,0,regMPASP_SMN_C2PMSG_35,msg as u32);let max=MEM_TRAIN_SEND_MSG_TIMEOUT_US/(*a).usec_timeout;let mut i=0;let mut ret=-ETIME;while i<max{ret=psp_wait_for(p,SOC15_REG_OFFSET(MP0,0,regMPASP_SMN_C2PMSG_35),0x80000000,0x80000000,PSP_WAITREG_NOVERBOSE);if ret==0{break;}i+=1;}if i<max{0}else{ret}}

unsafe fn psp_v14_0_memory_training(p:*mut psp_context,ops:u32)->i32 {let c=&mut (*p).mem_train_ctx;if c.init==PSP_MEM_TRAIN_NOT_SUPPORT{return 0;}if c.init!=PSP_MEM_TRAIN_INIT_SUCCESS{return -EINVAL;}if psp_v14_0_is_sos_alive(p){return 0;}let mut h=[0u32;4];amdgpu_device_vram_access((*p).adev,c.p2c_train_data_offset,h.as_mut_ptr(),core::mem::size_of_val(&h),false);let mut o=ops;if o&PSP_MEM_TRAIN_SEND_SHORT_MSG!=0{o|=PSP_MEM_TRAIN_RESTORE;}let cache=c.sys_cache as *mut u32;if o&PSP_MEM_TRAIN_RESTORE!=0&&*cache!=MEM_TRAIN_SYSTEM_SIGNATURE{o|=PSP_MEM_TRAIN_SAVE;}if h[0]==MEM_TRAIN_SYSTEM_SIGNATURE&&(*cache!=MEM_TRAIN_SYSTEM_SIGNATURE||*cache.add(3)!=h[3]){o|=PSP_MEM_TRAIN_SAVE;}if o&PSP_MEM_TRAIN_SAVE!=0&&h[0]!=MEM_TRAIN_SYSTEM_SIGNATURE{o|=PSP_MEM_TRAIN_SEND_LONG_MSG;}if o&PSP_MEM_TRAIN_SEND_LONG_MSG!=0{o&=!PSP_MEM_TRAIN_SEND_SHORT_MSG;o|=PSP_MEM_TRAIN_SAVE;}if o&PSP_MEM_TRAIN_SEND_LONG_MSG!=0{let sz=BIST_MEM_TRAINING_ENCROACHED_SIZE;if (*(*p).adev).gmc.visible_vram_size<sz||(*(*p).adev).mman.aper_base_kaddr.is_null(){return -EINVAL;}let b=vmalloc(sz);if b.is_null(){return -ENOMEM;}let mut idx=0;if drm_dev_enter(adev_to_drm((*p).adev),&mut idx){memcpy_fromio(b,(*(*p).adev).mman.aper_base_kaddr,sz);let r=psp_v14_0_memory_training_send_msg(p,PSP_BL__DRAM_LONG_TRAIN);if r!=0{vfree(b);drm_dev_exit(idx);return r;}memcpy_toio((*(*p).adev).mman.aper_base_kaddr,b,sz);amdgpu_device_flush_hdp((*p).adev,core::ptr::null_mut());vfree(b);drm_dev_exit(idx);}else{vfree(b);return -ENODEV;}}if o&PSP_MEM_TRAIN_SAVE!=0{amdgpu_device_vram_access((*p).adev,c.p2c_train_data_offset,c.sys_cache,c.train_data_size,false);}if o&PSP_MEM_TRAIN_RESTORE!=0{amdgpu_device_vram_access((*p).adev,c.c2p_train_data_offset,c.sys_cache,c.train_data_size,true);}if o&PSP_MEM_TRAIN_SEND_SHORT_MSG!=0{let r=psp_v14_0_memory_training_send_msg(p,if amdgpu_force_long_training>0{PSP_BL__DRAM_LONG_TRAIN}else{PSP_BL__DRAM_SHORT_TRAIN});if r!=0{return r;}}c.training_cnt+=1;0}

unsafe fn psp_v14_0_load_usbc_pd_fw(p:*mut psp_context,a:u64)->i32{WREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_36,(a>>20)as u32);let r=psp_wait_for(p,SOC15_REG_OFFSET(MP0,0,regMPASP_SMN_C2PMSG_35),0x80000000,0x80000000,0);if r!=0{return r;}WREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_35,GFX_CMD_USB_PD_USE_LFB<<16);for _ in 0..USBC_PD_POLLING_LIMIT_S{msleep(1000);let s=RREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_35);if s&0x80000000!=0{return if s&0xffff!=0{-EIO}else{0};}}-ETIME}
unsafe fn psp_v14_0_read_usbc_pd_fw(p:*mut psp_context,v:*mut u32)->i32{WREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_35,C2PMSG_CMD_GFX_USB_PD_FW_VER);let r=psp_wait_for(p,SOC15_REG_OFFSET(MP0,0,regMPASP_SMN_C2PMSG_35),0x80000000,0x80000000,0);if r==0{*v=RREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_36);}r}
unsafe fn psp_v14_0_exec_spi_cmd(p:*mut psp_context,cmd:i32)->i32{WREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_115,(cmd as u32)<<16);WREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_73,1);let r=psp_wait_for(p,SOC15_REG_OFFSET(MP0,0,regMPASP_SMN_C2PMSG_115),MBOX_READY_FLAG,MBOX_READY_MASK,0);if r!=0{return r;}if RREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_115)&0xffff!=0{-EIO}else{0}}
unsafe fn psp_v14_0_update_spirom(p:*mut psp_context,a:u64)->i32{let mut r=psp_wait_for(p,SOC15_REG_OFFSET(MP0,0,regMPASP_SMN_C2PMSG_115),MBOX_READY_FLAG,MBOX_READY_MASK,0);if r!=0{return r;}WREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_116,lower_32_bits(a));r=psp_v14_0_exec_spi_cmd(p,C2PMSG_CMD_SPI_UPDATE_ROM_IMAGE_ADDR_LO);if r!=0{return r;}WREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_116,upper_32_bits(a));r=psp_v14_0_exec_spi_cmd(p,C2PMSG_CMD_SPI_UPDATE_ROM_IMAGE_ADDR_HI);if r!=0{return r;}(*p).vbflash_done=true;psp_v14_0_exec_spi_cmd(p,C2PMSG_CMD_SPI_UPDATE_FLASH_IMAGE)}
unsafe fn psp_v14_0_vbflash_status(p:*mut psp_context)->i32{RREG32_SOC15((*p).adev,MP0,0,regMPASP_SMN_C2PMSG_115) as i32}

pub unsafe fn psp_v14_0_set_psp_funcs(psp:*mut psp_context){(*psp).funcs=&psp_v14_0_funcs;}
static psp_v14_0_funcs: psp_funcs = psp_funcs { init_microcode:Some(psp_v14_0_init_microcode), bootloader_load_kdb:Some(psp_v14_0_bootloader_load_kdb), bootloader_load_spl:Some(psp_v14_0_bootloader_load_spl), bootloader_load_sysdrv:Some(psp_v14_0_bootloader_load_sysdrv), bootloader_load_soc_drv:Some(psp_v14_0_bootloader_load_soc_drv), bootloader_load_intf_drv:Some(psp_v14_0_bootloader_load_intf_drv), bootloader_load_dbg_drv:Some(psp_v14_0_bootloader_load_dbg_drv), bootloader_load_ras_drv:Some(psp_v14_0_bootloader_load_ras_drv), bootloader_load_ipkeymgr_drv:Some(psp_v14_0_bootloader_load_ipkeymgr_drv), bootloader_load_sos:Some(psp_v14_0_bootloader_load_sos), ring_create:Some(psp_v14_0_ring_create), ring_stop:Some(psp_v14_0_ring_stop), ring_destroy:Some(psp_v14_0_ring_destroy), ring_get_wptr:Some(psp_v14_0_ring_get_wptr), ring_set_wptr:Some(psp_v14_0_ring_set_wptr), mem_training:Some(psp_v14_0_memory_training), load_usbc_pd_fw:Some(psp_v14_0_load_usbc_pd_fw), read_usbc_pd_fw:Some(psp_v14_0_read_usbc_pd_fw), update_spirom:Some(psp_v14_0_update_spirom), vbflash_stat:Some(psp_v14_0_vbflash_status) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
