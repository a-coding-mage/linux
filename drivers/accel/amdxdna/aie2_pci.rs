// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of aie2_pci.c. Kernel and driver symbols are
 * supplied by the surrounding crate. */

const DEFAULT_TIME_QUANTUM: u32 = 30000;
static mut AIE2_MAX_COL: i32 = XRS_MAX_COL;
static NPU_FW: [&str; 2] = ["npu_7.sbin", "npu.sbin"];

#[repr(C)]
struct MgmtMboxChannInfo { x2i_tail:u32, x2i_head:u32, x2i_buf:u32, x2i_buf_sz:u32,
    i2x_tail:u32, i2x_head:u32, i2x_buf:u32, i2x_buf_sz:u32, magic:u32, msi_id:u32,
    prot_major:u32, prot_minor:u32, rsvd:[u32;4] }

unsafe fn aie2_get_mgmt_chann_info(ndev: *mut amdxdna_dev_hdl) -> i32 {
    let mut info = core::mem::zeroed::<MgmtMboxChannInfo>();
    let mut addr=0u32; let mut ret = readx_poll_timeout(readl, SRAM_GET_ADDR(ndev, FW_ALIVE_OFF), &mut addr, addr, AIE_INTERVAL, AIE_TIMEOUT);
    if ret != 0 || addr == 0 { return -ETIME; }
    let off=AIE2_SRAM_OFF(ndev,addr); let p=&mut info as *mut _ as *mut u32;
    for i in 0..(core::mem::size_of::<MgmtMboxChannInfo>()/4) { *p.add(i)=readl((*ndev).sram_base.add(off as usize + i*4)); }
    if info.magic != MGMT_MBOX_MAGIC { XDNA_ERR((*ndev).aie.xdna, "Invalid mbox magic 0x%x", info.magic); ret=-EINVAL; }
    else { let i2x=&mut (*ndev).aie.mgmt_i2x; let x2i=&mut (*ndev).aie.mgmt_x2i;
        i2x.mb_head_ptr_reg=AIE2_MBOX_OFF(ndev,info.i2x_head); i2x.mb_tail_ptr_reg=AIE2_MBOX_OFF(ndev,info.i2x_tail); i2x.rb_start_addr=AIE2_SRAM_OFF(ndev,info.i2x_buf); i2x.rb_size=info.i2x_buf_sz;
        x2i.mb_head_ptr_reg=AIE2_MBOX_OFF(ndev,info.x2i_head); x2i.mb_tail_ptr_reg=AIE2_MBOX_OFF(ndev,info.x2i_tail); x2i.rb_start_addr=AIE2_SRAM_OFF(ndev,info.x2i_buf); x2i.rb_size=info.x2i_buf_sz;
        (*ndev).aie.mgmt_chan_idx=info.msi_id; (*ndev).aie.mgmt_prot_major=info.prot_major; (*ndev).aie.mgmt_prot_minor=info.prot_minor;
        ret=aie_check_protocol(&mut (*ndev).aie,info.prot_major,info.prot_minor); }
    aie_dump_mgmt_chann_debug(&mut (*ndev).aie); writel(0,SRAM_GET_ADDR(ndev,FW_ALIVE_OFF)); ret
}

pub unsafe fn aie2_runtime_cfg(ndev:*mut amdxdna_dev_hdl, category:rt_config_category, val:*mut u32)->i32 {
    let mut cfg=(*(*ndev).priv_).rt_config; while (*cfg).type_ != 0 { if (*cfg).category==category && ((*cfg).feature_mask==0 || !bitmap_subset(&(*cfg).feature_mask,&(*ndev).aie.feature_mask,AIE2_FEATURE_MAX)) { let v=if !val.is_null(){*val}else{(*cfg).value}; let r=aie2_set_runtime_cfg(ndev,(*cfg).type_,v); if r!=0{return r;} } cfg=cfg.add(1); } 0
}
unsafe fn aie2_xdna_reset(n:*mut amdxdna_dev_hdl)->i32 { let mut r=aie2_suspend_fw(n); if r!=0{return r} ; aie2_resume_fw(n) }
unsafe fn aie2_mgmt_fw_init(n:*mut amdxdna_dev_hdl)->i32 { let mut r=aie2_runtime_cfg(n,AIE2_RT_CFG_INIT,core::ptr::null_mut()); if r!=0{return r}; r=aie2_assign_mgmt_pasid(n,0); if r!=0{return r}; r=aie2_update_prop_time_quota(n,DEFAULT_TIME_QUANTUM); if r!=0{return r}; aie2_xdna_reset(n) }
unsafe fn aie2_mgmt_fw_query(n:*mut amdxdna_dev_hdl)->i32 { let mut r=aie2_query_firmware_version(n,&mut (*(*n).aie.xdna).fw_ver); if r!=0{return r}; r=aie2_query_aie_version(n,&mut (*n).version); if r!=0{return r}; r=aie2_query_aie_metadata(n,&mut (*n).aie.metadata); if r==0 {(*n).total_col=min(AIE2_MAX_COL as u32,(*n).aie.metadata.cols);} r }
unsafe fn aie2_mgmt_fw_fini(n:*mut amdxdna_dev_hdl) { if aie2_suspend_fw(n)!=0 { XDNA_ERR((*n).aie.xdna,"Suspend_fw failed"); } }

unsafe fn aie2_xrs_load(arg:*mut c_void, action:*mut xrs_action_load)->i32 { let h=arg as *mut amdxdna_hwctx; let x=(*(*h).client).xdna; (*h).start_col=(*action).part.start_col; (*h).num_unused_col=(*action).part.ncols-(*h).num_col; (*h).num_col=(*action).part.ncols; aie2_create_context((*x).dev_handle,h) }
unsafe fn aie2_xrs_unload(arg:*mut c_void)->i32 { let h=arg as *mut amdxdna_hwctx; aie2_destroy_context((*(*h).client).xdna.dev_handle,h) }
unsafe fn aie2_xrs_set_dft_dpm_level(d:*mut drm_device,l:u32)->i32 { let x=to_xdna_dev(d); let n=(*x).dev_handle; (*n).dft_dpm_level=l; if (*n).pw_mode!=POWER_MODE_DEFAULT || (*n).dpm_level==l {0}else{aie2_pm_set_dpm(n,l)} }
static mut AIE2_XRS_ACTIONS:xrs_action_ops=xrs_action_ops{load:Some(aie2_xrs_load),unload:Some(aie2_xrs_unload),set_dft_dpm_level:Some(aie2_xrs_set_dft_dpm_level)};

// The remaining operations retain the C driver's dispatch and cleanup order.
unsafe fn aie2_hw_stop(x:*mut amdxdna_dev){let n=(*x).dev_handle; if (*n).dev_status<=AIE2_DEV_INIT{return}; aie2_runtime_cfg(n,AIE2_RT_CFG_CLK_GATING,core::ptr::null_mut()); aie2_mgmt_fw_fini(n); aie_destroy_chann(&mut (*n).aie,&mut (*n).aie.mgmt_chann); (*n).mbox=core::ptr::null_mut(); aie_psp_stop((*n).aie.psp_hdl); (*n).priv_.hw_ops.set_dpm(n,0); aie_smu_fini((*n).aie.smu_hdl); aie2_error_async_events_free(n); (*n).dev_status=AIE2_DEV_INIT;}
unsafe fn aie2_hw_start(x:*mut amdxdna_dev)->i32 { let n=(*x).dev_handle; if (*n).dev_status>=AIE2_DEV_START{return 0}; let mut r=aie_smu_init((*n).aie.smu_hdl); if r!=0{return r}; r=aie_psp_start((*n).aie.psp_hdl); if r!=0{return r}; r=aie2_get_mgmt_chann_info(n); if r!=0{return r}; r=aie2_mgmt_fw_init(n); if r!=0{return r}; r=aie2_pm_init(n); if r!=0{return r}; r=aie2_mgmt_fw_query(n); if r!=0{return r}; r=aie2_error_async_events_alloc(n); if r==0 {(*n).dev_status=AIE2_DEV_START;} r }
unsafe fn aie2_hw_suspend(x:*mut amdxdna_dev)->i32 { list_for_each_entry((*x).client_list,aie2_hwctx_suspend); aie2_hw_stop(x); 0 }
unsafe fn aie2_hw_resume(x:*mut amdxdna_dev)->i32 { let r=aie2_hw_start(x); if r!=0{return r}; list_for_each_entry((*x).client_list,aie2_hwctx_resume); 0 }

pub static mut aie2_ops:amdxdna_dev_ops=amdxdna_dev_ops{init:Some(aie2_init),fini:Some(aie2_fini),resume:Some(aie2_hw_resume),suspend:Some(aie2_hw_suspend),get_aie_info:Some(aie2_get_info),set_aie_state:Some(aie2_set_state),hwctx_init:Some(aie2_hwctx_init),hwctx_fini:Some(aie2_hwctx_fini),hwctx_config:Some(aie2_hwctx_config),hwctx_sync_debug_bo:Some(aie2_hwctx_sync_debug_bo),cmd_submit:Some(aie2_cmd_submit),hmm_invalidate:Some(aie2_hmm_invalidate),get_array:Some(aie2_get_array),get_dev_revision:Some(aie2_get_dev_rev),hwctx_heap_expand:Some(aie2_hwctx_heap_expand)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
