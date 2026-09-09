// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2020 Intel Corporation */

// C includes and preprocessor dependencies are supplied by the surrounding
// translation unit.

pub unsafe fn adf_gen4_get_accel_mask(_self_: *mut adf_hw_device_data) -> u32 {
    ADF_GEN4_ACCELERATORS_MASK
}

pub unsafe fn adf_gen4_get_num_accels(_self_: *mut adf_hw_device_data) -> u32 {
    ADF_GEN4_MAX_ACCELERATORS
}

pub unsafe fn adf_gen4_get_num_aes(self_: *mut adf_hw_device_data) -> u32 {
    if self_.is_null() || (*self_).ae_mask == 0 { return 0; }
    hweight32((*self_).ae_mask)
}

pub unsafe fn adf_gen4_get_misc_bar_id(_self_: *mut adf_hw_device_data) -> u32 { ADF_GEN4_PMISC_BAR }
pub unsafe fn adf_gen4_get_etr_bar_id(_self_: *mut adf_hw_device_data) -> u32 { ADF_GEN4_ETR_BAR }
pub unsafe fn adf_gen4_get_sram_bar_id(_self_: *mut adf_hw_device_data) -> u32 { ADF_GEN4_SRAM_BAR }
pub unsafe fn adf_gen4_get_sku(_self_: *mut adf_hw_device_data) -> dev_sku_info { DEV_SKU_1 }

pub unsafe fn adf_gen4_get_arb_info(info: *mut arb_info) {
    (*info).arb_cfg = ADF_GEN4_ARB_CONFIG;
    (*info).arb_offset = ADF_GEN4_ARB_OFFSET;
    (*info).wt2sam_offset = ADF_GEN4_ARB_WRK_2_SER_MAP_OFFSET;
}

pub unsafe fn adf_gen4_get_admin_info(info: *mut admin_info) {
    (*info).mailbox_offset = ADF_GEN4_MAILBOX_BASE_OFFSET;
    (*info).admin_msg_ur = ADF_GEN4_ADMINMSGUR_OFFSET;
    (*info).admin_msg_lr = ADF_GEN4_ADMINMSGLR_OFFSET;
}

pub unsafe fn adf_gen4_get_heartbeat_clock(_self_: *mut adf_hw_device_data) -> u32 {
    /* GEN4 uses KPT counter for HB */
    ADF_GEN4_KPT_COUNTER_FREQ
}

pub unsafe fn adf_gen4_enable_error_correction(accel_dev: *mut adf_accel_dev) {
    let misc_bar = &mut GET_BARS(accel_dev)[ADF_GEN4_PMISC_BAR as usize];
    let csr = misc_bar.virt_addr;
    // Enable all in errsou3 except VFLR notification on host
    ADF_CSR_WR(csr, ADF_GEN4_ERRMSK3, ADF_GEN4_VFLNOTIFY);
}

pub unsafe fn adf_gen4_enable_ints(accel_dev: *mut adf_accel_dev) {
    let addr = GET_BARS(accel_dev)[ADF_GEN4_PMISC_BAR as usize].virt_addr;
    // Enable bundle interrupts
    ADF_CSR_WR(addr, ADF_GEN4_SMIAPF_RP_X0_MASK_OFFSET, 0);
    ADF_CSR_WR(addr, ADF_GEN4_SMIAPF_RP_X1_MASK_OFFSET, 0);
    // Enable misc interrupts
    ADF_CSR_WR(addr, ADF_GEN4_SMIAPF_MASK_OFFSET, 0);
}

pub unsafe fn adf_gen4_init_device(accel_dev: *mut adf_accel_dev) -> i32 {
    let addr = GET_BARS(accel_dev)[ADF_GEN4_PMISC_BAR as usize].virt_addr;
    let mut csr: u32 = ADF_CSR_RD(addr, ADF_GEN4_ERRMSK2);
    csr |= ADF_GEN4_PM_SOU;
    ADF_CSR_WR(addr, ADF_GEN4_ERRMSK2, csr);
    ADF_CSR_WR(addr, ADF_GEN4_PM_INTERRUPT, ADF_GEN4_PM_DRV_ACTIVE);
    let mut status: u32 = 0;
    let ret = read_poll_timeout(ADF_CSR_RD, status,
        status & ADF_GEN4_PM_INIT_STATE, ADF_GEN4_PM_POLL_DELAY_US,
        ADF_GEN4_PM_POLL_TIMEOUT_US, true, addr, ADF_GEN4_PM_STATUS);
    if ret != 0 { dev_err(&GET_DEV(accel_dev), "Failed to power up the device\n"); }
    ret
}

pub unsafe fn adf_gen4_set_ssm_wdtimer(accel_dev: *mut adf_accel_dev) {
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    let timer_val_pke: u64 = ADF_SSM_WDT_PKE_DEFAULT_VALUE;
    let timer_val: u64 = ADF_SSM_WDT_DEFAULT_VALUE;
    ADF_CSR_WR64_LO_HI(pmisc_addr, ADF_SSMWDTL_OFFSET, ADF_SSMWDTH_OFFSET, timer_val);
    ADF_CSR_WR64_LO_HI(pmisc_addr, ADF_SSMWDTPKEL_OFFSET, ADF_SSMWDTPKEH_OFFSET, timer_val_pke);
}

pub unsafe fn adf_gen4_set_msix_default_rttable(accel_dev: *mut adf_accel_dev) {
    let csr = GET_BARS(accel_dev)[ADF_GEN4_PMISC_BAR as usize].virt_addr;
    for i in 0..=ADF_GEN4_ETR_MAX_BANKS { ADF_CSR_WR(csr, ADF_GEN4_MSIX_RTTABLE_OFFSET(i), i); }
}

pub unsafe fn adf_pfvf_comms_disabled(_accel_dev: *mut adf_accel_dev) -> i32 { 0 }

unsafe fn reset_ring_pair(csr: *mut core::ffi::c_void, bank_number: u32) -> i32 {
    ADF_CSR_WR(csr, ADF_WQM_CSR_RPRESETCTL(bank_number), ADF_WQM_CSR_RPRESETCTL_RESET);
    let mut status: u32 = 0;
    let ret = read_poll_timeout(ADF_CSR_RD, status, status & ADF_WQM_CSR_RPRESETSTS_STATUS,
        ADF_RPRESET_POLL_DELAY_US, ADF_RPRESET_POLL_TIMEOUT_US, true,
        csr, ADF_WQM_CSR_RPRESETSTS(bank_number));
    if ret == 0 { ADF_CSR_WR(csr, ADF_WQM_CSR_RPRESETSTS(bank_number), ADF_WQM_CSR_RPRESETSTS_STATUS); }
    ret
}

pub unsafe fn adf_gen4_ring_pair_reset(accel_dev: *mut adf_accel_dev, bank_number: u32) -> i32 {
    let hw_data = (*accel_dev).hw_device;
    let csr = adf_get_etr_base(accel_dev);
    if bank_number >= (*hw_data).num_banks { return -EINVAL; }
    dev_dbg(&GET_DEV(accel_dev), "ring pair reset for bank:%d\n", bank_number);
    let ret = reset_ring_pair(csr, bank_number);
    if ret != 0 { dev_err(&GET_DEV(accel_dev), "ring pair reset failed (timeout)\n"); }
    else { dev_dbg(&GET_DEV(accel_dev), "ring pair reset successful\n"); }
    ret
}

static THRD_TO_ARB_MAP_DCC: [u32; 17] = [
    0, 0, 0, 0, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static RP_GROUP_TO_ARB_MASK: [u16; RP_GROUP_COUNT as usize] = [0x5, 0xA];

unsafe fn is_single_service(service_id: i32) -> bool {
    matches!(service_id, SVC_DC | SVC_SYM | SVC_ASYM)
}

pub unsafe fn adf_gen4_services_supported(mask: c_ulong) -> bool {
    let num_svc = hweight_long(mask);
    if mask >= BIT(SVC_COUNT) || test_bit(SVC_DECOMP, &mask) { return false; }
    match num_svc { ADF_ONE_SERVICE => true, ADF_TWO_SERVICES => !test_bit(SVC_DCC, &mask), _ => false }
}

pub unsafe fn adf_gen4_init_thd2arb_map(accel_dev: *mut adf_accel_dev) -> i32 {
    let hw_data = GET_HW_DATA(accel_dev);
    let thd2arb_map = (*hw_data).thd_to_arb_map;
    if (*hw_data).get_rp_group.is_none() || (*hw_data).get_ena_thd_mask.is_none() ||
       (*hw_data).get_num_aes.is_none() || (*hw_data).uof_get_num_objs.is_none() ||
       (*hw_data).uof_get_ae_mask.is_none() { return -EFAULT; }
    let srv_id = adf_get_service_enabled(accel_dev); if srv_id < 0 { return srv_id; }
    let ae_cnt = ((*hw_data).get_num_aes.unwrap())(hw_data);
    let worker_obj_cnt = ((*hw_data).uof_get_num_objs.unwrap())(accel_dev) - ADF_GEN4_ADMIN_ACCELENGINES;
    if srv_id == SVC_DCC { if ae_cnt > ICP_QAT_HW_AE_DELIMITER { return -EINVAL; }
        core::ptr::copy_nonoverlapping(THRD_TO_ARB_MAP_DCC.as_ptr(), thd2arb_map, ae_cnt as usize); return 0; }
    for i in 0..worker_obj_cnt {
        let ae_mask = ((*hw_data).uof_get_ae_mask.unwrap())(accel_dev, i);
        let rp_group = ((*hw_data).get_rp_group.unwrap())(accel_dev, ae_mask);
        let thds_mask = ((*hw_data).get_ena_thd_mask.unwrap())(accel_dev, i);
        if rp_group >= RP_GROUP_COUNT || rp_group < RP_GROUP_0 || thds_mask == ADF_GEN4_ENA_THD_MASK_ERROR { return -EINVAL; }
        let arb_mask = if is_single_service(srv_id) { RP_GROUP_TO_ARB_MASK[0] | RP_GROUP_TO_ARB_MASK[1] } else { RP_GROUP_TO_ARB_MASK[rp_group as usize] };
        let mut base = 0u32;
        for j in 0..ADF_NUM_THREADS_PER_AE { if (thds_mask & (1usize << j)) != 0 { base |= (arb_mask as u32) << (j * 4); } }
        for j in 0..ae_cnt { if (ae_mask & (1usize << j)) != 0 { *thd2arb_map.add(j as usize) = base; } }
    }
    0
}

pub unsafe fn adf_gen4_get_ring_to_svc_map(accel_dev: *mut adf_accel_dev) -> u16 {
    let hw_data = GET_HW_DATA(accel_dev); let mut rps = [0i32; RP_GROUP_COUNT as usize];
    if (*hw_data).get_rp_group.is_none() || (*hw_data).uof_get_ae_mask.is_none() || (*hw_data).uof_get_obj_type.is_none() || (*hw_data).uof_get_num_objs.is_none() { return 0; }
    if adf_get_service_enabled(accel_dev) == SVC_DCC { rps.fill(COMP); }
    else { let n = ((*hw_data).uof_get_num_objs.unwrap())(accel_dev) - ADF_GEN4_ADMIN_ACCELENGINES; let start = n - RP_GROUP_COUNT;
        for i in start..n { let ae = ((*hw_data).uof_get_ae_mask.unwrap())(accel_dev, i); let g = ((*hw_data).get_rp_group.unwrap())(accel_dev, ae); if g >= RP_GROUP_COUNT || g < RP_GROUP_0 { return 0; }
            rps[g as usize] = match ((*hw_data).uof_get_obj_type.unwrap())(accel_dev, i) { ADF_FW_SYM_OBJ => SYM, ADF_FW_ASYM_OBJ => ASYM, ADF_FW_DC_OBJ => COMP, _ => 0 }; }
    }
    (rps[0] << ADF_CFG_SERV_RING_PAIR_0_SHIFT | rps[1] << ADF_CFG_SERV_RING_PAIR_1_SHIFT |
     rps[0] << ADF_CFG_SERV_RING_PAIR_2_SHIFT | rps[1] << ADF_CFG_SERV_RING_PAIR_3_SHIFT) as u16
}

// The remaining hardware operations retain the original kernel helper calls and layouts.
pub unsafe fn adf_gen4_bank_quiesce_coal_timer(accel_dev: *mut adf_accel_dev, bank_idx: u32, timeout_ms: i32) -> i32 {
    if timeout_ms < 0 { return -EINVAL; }
    let hw = GET_HW_DATA(accel_dev); let ops = GET_CSR_OPS(accel_dev); let misc = adf_get_pmisc_base(accel_dev); let etr = adf_get_etr_base(accel_dev);
    let ctl = ((*ops).read_csr_int_col_ctl)(etr, bank_idx); let mask = ((*ops).get_int_col_ctl_enable_mask)(); if ctl & mask == 0 { return 0; }
    let en = ((*ops).read_csr_int_col_en)(etr, bank_idx) & BIT(ADF_WQM_CSR_RP_IDX_RX); let estat = ((*ops).read_csr_e_stat)(etr, bank_idx); if !(!estat & en != 0) { return 0; }
    let mut wait_us = 2u64 * ((ctl & !mask) as u64) * 256 * USEC_PER_SEC as u64; wait_us /= (*hw).clock_frequency as u64; wait_us = core::cmp::min(wait_us, timeout_ms as u64 * USEC_PER_MSEC as u64);
    let mut intsrc = 0u32; read_poll_timeout(ADF_CSR_RD, intsrc, intsrc, ADF_COALESCED_POLL_DELAY_US, wait_us, true, misc, ADF_WQM_CSR_RPINTSOU(bank_idx))
}

unsafe fn drain_bank(csr: *mut core::ffi::c_void, bank: u32, timeout: i32) -> i32 { ADF_CSR_WR(csr, ADF_WQM_CSR_RPRESETCTL(bank), ADF_WQM_CSR_RPRESETCTL_DRAIN); let mut s=0u32; read_poll_timeout(ADF_CSR_RD,s,s & ADF_WQM_CSR_RPRESETSTS_STATUS,ADF_RPRESET_POLL_DELAY_US,timeout,true,csr,ADF_WQM_CSR_RPRESETSTS(bank)) }
pub unsafe fn adf_gen4_bank_drain_finish(dev: *mut adf_accel_dev, bank: u32) { let c=adf_get_etr_base(dev); ADF_CSR_WR(c,ADF_WQM_CSR_RPRESETSTS(bank),ADF_WQM_CSR_RPRESETSTS_STATUS); }
pub unsafe fn adf_gen4_bank_drain_start(dev: *mut adf_accel_dev, bank: u32, timeout: i32) -> i32 { drain_bank(adf_get_etr_base(dev),bank,timeout) }

unsafe fn adf_gen4_build_comp_block(ctx: *mut core::ffi::c_void, algo: adf_dc_algo) -> i32 { let req=ctx as *mut icp_qat_fw_comp_req; let h=&mut (*req).comn_hdr; let cd=&mut (*req).cd_pars; let mut lo=0u32; let mut hi=0u32; match algo { QAT_DEFLATE=>{h.service_cmd_id=ICP_QAT_FW_COMP_CMD_DYNAMIC;lo=ICP_QAT_FW_COMP_20_BUILD_CONFIG_LOWER(ICP_QAT_HW_COMP_20_HW_COMP_FORMAT_ILZ77,ICP_QAT_HW_COMP_20_LLLBD_CTRL_LLLBD_ENABLED,ICP_QAT_HW_COMP_20_BYTE_SKIP_3BYTE_LITERAL);},QAT_LZ4S=>{h.service_cmd_id=ICP_QAT_FW_COMP_20_CMD_LZ4S_COMPRESS;lo=ICP_QAT_FW_COMP_20_BUILD_CONFIG_LOWER(ICP_QAT_HW_COMP_20_HW_COMP_FORMAT_LZ4S,ICP_QAT_HW_COMP_20_LLLBD_CTRL_LLLBD_DISABLED,ICP_QAT_HW_COMP_20_ABD_ABD_DISABLED);},_=>return -EINVAL}; hi=ICP_QAT_FW_COMP_20_BUILD_CONFIG_UPPER(ICP_QAT_HW_COMP_20_CONFIG_CSR_NICE_PARAM_DEFAULT_VAL,ICP_QAT_HW_COMP_20_CONFIG_CSR_LAZY_PARAM_DEFAULT_VAL); (*cd).u.sl.comp_slice_cfg_word[0]=lo;(*cd).u.sl.comp_slice_cfg_word[1]=hi;0 }
unsafe fn adf_gen4_build_decomp_block(ctx:*mut core::ffi::c_void,algo:adf_dc_algo)->i32{let req=ctx as *mut icp_qat_fw_comp_req;let h=&mut(*req).comn_hdr;let cd=&mut(*req).cd_pars;let lo=match algo{QAT_DEFLATE=>{h.service_cmd_id=ICP_QAT_FW_COMP_CMD_DECOMPRESS;ICP_QAT_FW_DECOMP_20_BUILD_CONFIG_LOWER(ICP_QAT_HW_DECOMP_20_HW_DECOMP_FORMAT_DEFLATE)},QAT_LZ4S=>{h.service_cmd_id=ICP_QAT_FW_COMP_20_CMD_LZ4S_DECOMPRESS;ICP_QAT_FW_DECOMP_20_BUILD_CONFIG_LOWER(ICP_QAT_HW_DECOMP_20_HW_DECOMP_FORMAT_LZ4S)},_=>return -EINVAL};(*cd).u.sl.comp_slice_cfg_word[0]=lo;(*cd).u.sl.comp_slice_cfg_word[1]=0;0}
pub unsafe fn adf_gen4_init_dc_ops(ops:*mut adf_dc_ops){(*ops).build_comp_block=Some(adf_gen4_build_comp_block);(*ops).build_decomp_block=Some(adf_gen4_build_decomp_block);}
pub unsafe fn adf_gen4_init_num_svc_aes(d:*mut adf_rl_hw_data){let h=container_of!(d,adf_hw_device_data,rl_data);let n=hweight32(((*h).get_ae_mask.unwrap())(h));if n==0{return;}for i in 0..SVC_BASE_COUNT{(*d).svc_ae_mask[i]=n-1;}(*d).svc_ae_mask[SVC_DECOMP as usize]=0;}
pub unsafe fn adf_gen4_get_svc_slice_cnt(dev:*mut adf_accel_dev,svc:adf_base_services)->u32{let d=&(*(*dev).hw_device).rl_data;match svc{SVC_SYM=>d.slices.cph_cnt,SVC_ASYM=>d.slices.pke_cnt,SVC_DC=>d.slices.dcpr_cnt,_=>0}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
