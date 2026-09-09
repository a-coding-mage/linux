// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */
// C dependencies are supplied by the surrounding translation unit.

const ADF_AE_GROUP_0: u32 = GENMASK(3, 0);
const ADF_AE_GROUP_1: u32 = GENMASK(7, 4);
const ADF_AE_GROUP_2: u32 = GENMASK(11, 8);
const ADF_AE_GROUP_3: u32 = GENMASK(15, 12);
const ADF_AE_GROUP_4: u32 = BIT(16);
const ENA_THD_MASK_ASYM: u32 = GENMASK(1, 0);
const ENA_THD_MASK_SYM: u32 = GENMASK(3, 0);
const ENA_THD_MASK_DC: u32 = GENMASK(1, 0);

static mut adf_420xx_fw_objs: [*const core::ffi::c_char; 4] = [
    ADF_420XX_SYM_OBJ, ADF_420XX_ASYM_OBJ, ADF_420XX_DC_OBJ, ADF_420XX_ADMIN_OBJ,
];
static adf_fw_cy_config: [adf_fw_config; 5] = [
    adf_fw_config { ae_mask: ADF_AE_GROUP_3, obj: ADF_FW_SYM_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ASYM_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_SYM_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_ASYM_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_4, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_dc_config: [adf_fw_config; 3] = [
    adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_DC_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_DC_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_4, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_sym_config: [adf_fw_config; 5] = [
    adf_fw_config { ae_mask: ADF_AE_GROUP_3, obj: ADF_FW_SYM_OBJ }, adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_SYM_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_SYM_OBJ }, adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_SYM_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_4, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_asym_config: [adf_fw_config; 5] = [
    adf_fw_config { ae_mask: ADF_AE_GROUP_3, obj: ADF_FW_ASYM_OBJ }, adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ASYM_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_ASYM_OBJ }, adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_ASYM_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_4, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_asym_dc_config: [adf_fw_config; 5] = [
    adf_fw_config { ae_mask: ADF_AE_GROUP_3, obj: ADF_FW_ASYM_OBJ }, adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ASYM_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_ASYM_OBJ }, adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_DC_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_4, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_sym_dc_config: [adf_fw_config; 4] = [
    adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_SYM_OBJ }, adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_SYM_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_DC_OBJ }, adf_fw_config { ae_mask: ADF_AE_GROUP_4, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_dcc_config: [adf_fw_config; 3] = [
    adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_DC_OBJ }, adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_SYM_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_4, obj: ADF_FW_ADMIN_OBJ },
];

static mut adf_420xx_class: adf_hw_device_class = adf_hw_device_class { name: ADF_420XX_DEVICE_NAME, r#type: DEV_420XX, instances: 0 };

unsafe fn get_ae_mask(self_: *mut adf_hw_device_data) -> u32 {
    let fuses = (*self_).fuses[ADF_FUSECTL4 as usize]; let mut mask = ADF_420XX_ACCELENGINES_MASK;
    if test_bit(0, &fuses) != 0 { mask &= !ADF_AE_GROUP_0; } if test_bit(4, &fuses) != 0 { mask &= !ADF_AE_GROUP_1; }
    if test_bit(8, &fuses) != 0 { mask &= !ADF_AE_GROUP_2; } if test_bit(12, &fuses) != 0 { mask &= !ADF_AE_GROUP_3; }
    if test_bit(16, &fuses) != 0 { mask &= !ADF_AE_GROUP_4; } mask
}

unsafe fn get_fw_config(a: *mut adf_accel_dev) -> *const adf_fw_config { match adf_get_service_enabled(a) {
    SVC_SYM_ASYM => adf_fw_cy_config.as_ptr(), SVC_DC => adf_fw_dc_config.as_ptr(), SVC_DCC => adf_fw_dcc_config.as_ptr(),
    SVC_SYM => adf_fw_sym_config.as_ptr(), SVC_ASYM => adf_fw_asym_config.as_ptr(), SVC_ASYM_DC => adf_fw_asym_dc_config.as_ptr(),
    SVC_SYM_DC => adf_fw_sym_dc_config.as_ptr(), _ => core::ptr::null(),
} }
unsafe fn uof_get_num_objs(a: *mut adf_accel_dev) -> u32 { match adf_get_service_enabled(a) {
    SVC_SYM_ASYM => 5, SVC_DC => 3, SVC_DCC => 3, SVC_SYM => 5, SVC_ASYM => 5, SVC_ASYM_DC => 5, SVC_SYM_DC => 4, _ => 0,
} }
unsafe fn update_ae_mask(a: *mut adf_accel_dev) { let h = GET_HW_DATA(a); let f = get_fw_config(a); let n = uof_get_num_objs(a); let mut m = ADF_420XX_ADMIN_AE_MASK; for i in 0..n { m |= (*f.add(i as usize)).ae_mask; } (*h).ae_mask = get_ae_mask(h) & m; }

unsafe fn get_accel_cap(a: *mut adf_accel_dev) -> u32 {
    update_ae_mask(a); let p = (*(*a).accel_pci_dev.pci_dev); let mut fusectl1 = 0u32; pci_read_config_dword(&p, ADF_GEN4_FUSECTL1_OFFSET, &mut fusectl1);
    let mut sym = ICP_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC|ICP_ACCEL_CAPABILITIES_CIPHER|ICP_ACCEL_CAPABILITIES_AUTHENTICATION|ICP_ACCEL_CAPABILITIES_SHA3|ICP_ACCEL_CAPABILITIES_SHA3_EXT|ICP_ACCEL_CAPABILITIES_HKDF|ICP_ACCEL_CAPABILITIES_CHACHA_POLY|ICP_ACCEL_CAPABILITIES_AESGCM_SPC|ICP_ACCEL_CAPABILITIES_SM3|ICP_ACCEL_CAPABILITIES_SM4|ICP_ACCEL_CAPABILITIES_AES_V2|ICP_ACCEL_CAPABILITIES_ZUC|ICP_ACCEL_CAPABILITIES_WIRELESS_CRYPTO_EXT|ICP_ACCEL_CAPABILITIES_EXT_ALGCHAIN;
    if fusectl1 & ICP_ACCEL_GEN4_MASK_CIPHER_SLICE != 0 { sym &= !(ICP_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC|ICP_ACCEL_CAPABILITIES_HKDF|ICP_ACCEL_CAPABILITIES_CIPHER); }
    if fusectl1 & ICP_ACCEL_GEN4_MASK_UCS_SLICE != 0 { sym &= !(ICP_ACCEL_CAPABILITIES_CHACHA_POLY|ICP_ACCEL_CAPABILITIES_AESGCM_SPC|ICP_ACCEL_CAPABILITIES_AES_V2|ICP_ACCEL_CAPABILITIES_CIPHER); }
    if fusectl1 & ICP_ACCEL_GEN4_MASK_AUTH_SLICE != 0 { sym &= !(ICP_ACCEL_CAPABILITIES_AUTHENTICATION|ICP_ACCEL_CAPABILITIES_SHA3|ICP_ACCEL_CAPABILITIES_SHA3_EXT|ICP_ACCEL_CAPABILITIES_CIPHER); }
    if fusectl1 & ICP_ACCEL_GEN4_MASK_SMX_SLICE != 0 { sym &= !(ICP_ACCEL_CAPABILITIES_SM3|ICP_ACCEL_CAPABILITIES_SM4); }
    if fusectl1 & ICP_ACCEL_GEN4_MASK_WCP_WAT_SLICE != 0 { sym &= !(ICP_ACCEL_CAPABILITIES_ZUC|ICP_ACCEL_CAPABILITIES_WIRELESS_CRYPTO_EXT); }
    if fusectl1 & ICP_ACCEL_GEN4_MASK_EIA3_SLICE != 0 { sym &= !ICP_ACCEL_CAPABILITIES_ZUC; }
    let mut asym = ICP_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC|ICP_ACCEL_CAPABILITIES_SM2|ICP_ACCEL_CAPABILITIES_ECEDMONT;
    if fusectl1 & ICP_ACCEL_GEN4_MASK_PKE_SLICE != 0 { asym &= !(ICP_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC|ICP_ACCEL_CAPABILITIES_SM2|ICP_ACCEL_CAPABILITIES_ECEDMONT); }
    let mut dc = ICP_ACCEL_CAPABILITIES_COMPRESSION|ICP_ACCEL_CAPABILITIES_LZ4_COMPRESSION|ICP_ACCEL_CAPABILITIES_LZ4S_COMPRESSION|ICP_ACCEL_CAPABILITIES_CNV_INTEGRITY64;
    if fusectl1 & ICP_ACCEL_GEN4_MASK_COMPRESS_SLICE != 0 { dc &= !(ICP_ACCEL_CAPABILITIES_COMPRESSION|ICP_ACCEL_CAPABILITIES_LZ4_COMPRESSION|ICP_ACCEL_CAPABILITIES_LZ4S_COMPRESSION|ICP_ACCEL_CAPABILITIES_CNV_INTEGRITY64); }
    match adf_get_service_enabled(a) { SVC_SYM_ASYM=>sym|asym, SVC_DC=>dc, SVC_DCC=>(dc|sym)&!ICP_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC, SVC_SYM=>sym, SVC_ASYM=>asym, SVC_ASYM_DC=>asym|dc, SVC_SYM_DC=>sym|dc, _=>0 }
}

unsafe fn adf_get_arbiter_mapping(a: *mut adf_accel_dev) -> *const u32 { if adf_gen4_init_thd2arb_map(a) != 0 { dev_warn(&GET_DEV(a), "Failed to generate thread to arbiter mapping"); } GET_HW_DATA(a).as_ref().unwrap().thd_to_arb_map.as_ptr() }
unsafe fn adf_init_rl_data(r: *mut adf_rl_hw_data) { (*r).pciout_tb_offset=ADF_GEN4_RL_TOKEN_PCIEOUT_BUCKET_OFFSET; (*r).pciin_tb_offset=ADF_GEN4_RL_TOKEN_PCIEIN_BUCKET_OFFSET; (*r).r2l_offset=ADF_GEN4_RL_R2L_OFFSET; (*r).l2c_offset=ADF_GEN4_RL_L2C_OFFSET; (*r).c2s_offset=ADF_GEN4_RL_C2S_OFFSET; (*r).pcie_scale_div=ADF_420XX_RL_PCIE_SCALE_FACTOR_DIV; (*r).pcie_scale_mul=ADF_420XX_RL_PCIE_SCALE_FACTOR_MUL; (*r).dcpr_correction=ADF_420XX_RL_DCPR_CORRECTION; (*r).max_tp[SVC_ASYM as usize]=ADF_420XX_RL_MAX_TP_ASYM; (*r).max_tp[SVC_SYM as usize]=ADF_420XX_RL_MAX_TP_SYM; (*r).max_tp[SVC_DC as usize]=ADF_420XX_RL_MAX_TP_DC; (*r).scan_interval=ADF_420XX_RL_SCANS_PER_SEC; (*r).scale_ref=ADF_420XX_RL_SLICE_REF; adf_gen4_init_num_svc_aes(r); }
unsafe fn get_rp_group(a: *mut adf_accel_dev, m: u32) -> i32 { match m { ADF_AE_GROUP_0=>RP_GROUP_0, ADF_AE_GROUP_1|ADF_AE_GROUP_3=>RP_GROUP_1, ADF_AE_GROUP_2=>if get_fw_config(a)==adf_fw_cy_config.as_ptr(){RP_GROUP_0}else{RP_GROUP_1}, _=>{-1} } }
unsafe fn get_ena_thd_mask(a: *mut adf_accel_dev, n: u32) -> u32 { if n>=uof_get_num_objs(a){return ADF_GEN4_ENA_THD_MASK_ERROR} let f=get_fw_config(a); if f.is_null(){return ADF_GEN4_ENA_THD_MASK_ERROR} match (*f.add(n as usize)).obj { ADF_FW_ASYM_OBJ=>ENA_THD_MASK_ASYM, ADF_FW_SYM_OBJ=>ENA_THD_MASK_SYM, ADF_FW_DC_OBJ=>ENA_THD_MASK_DC, _=>ADF_GEN4_ENA_THD_MASK_ERROR } }
unsafe fn uof_get_name(a: *mut adf_accel_dev,n:u32,objs:*const *const core::ffi::c_char,num:i32)->*const core::ffi::c_char { let f=get_fw_config(a); if f.is_null(){return core::ptr::null()} let id=(*f.add(n as usize)).obj as i32; if id<0||id>=num {core::ptr::null()} else {*objs.add(id as usize)} }
unsafe fn uof_get_name_420xx(a:*mut adf_accel_dev,n:u32)->*const core::ffi::c_char {uof_get_name(a,n,adf_420xx_fw_objs.as_ptr(),4)}
unsafe fn uof_get_obj_type(a:*mut adf_accel_dev,n:u32)->i32 {if n>=uof_get_num_objs(a){return -1} let f=get_fw_config(a);if f.is_null(){-1}else{(*f.add(n as usize)).obj}}
unsafe fn uof_get_ae_mask(a:*mut adf_accel_dev,n:u32)->u32 {let f=get_fw_config(a);if f.is_null(){0}else{(*f.add(n as usize)).ae_mask}}
unsafe fn adf_gen4_set_err_mask(m:*mut adf_dev_err_mask){(*m).cppagentcmdpar_mask=ADF_420XX_HICPPAGENTCMDPARERRLOG_MASK;(*m).parerr_ath_cph_mask=ADF_420XX_PARITYERRORMASK_ATH_CPH_MASK;(*m).parerr_cpr_xlt_mask=ADF_420XX_PARITYERRORMASK_CPR_XLT_MASK;(*m).parerr_dcpr_ucs_mask=ADF_420XX_PARITYERRORMASK_DCPR_UCS_MASK;(*m).parerr_pke_mask=ADF_420XX_PARITYERRORMASK_PKE_MASK;(*m).parerr_wat_wcp_mask=ADF_420XX_PARITYERRORMASK_WAT_WCP_MASK;(*m).ssmfeatren_mask=ADF_420XX_SSMFEATREN_MASK;}

pub unsafe fn adf_init_hw_data_420xx(h:*mut adf_hw_device_data,_dev_id:u32){
(*h).dev_class=&mut adf_420xx_class;(*h).instance_id=adf_420xx_class.instances;adf_420xx_class.instances+=1;
(*h).num_banks=ADF_GEN4_ETR_MAX_BANKS;(*h).num_banks_per_vf=ADF_GEN4_NUM_BANKS_PER_VF;(*h).num_rings_per_bank=ADF_GEN4_NUM_RINGS_PER_BANK;(*h).num_accel=ADF_GEN4_MAX_ACCELERATORS;(*h).num_engines=ADF_420XX_MAX_ACCELENGINES;(*h).num_logical_accel=1;(*h).tx_rx_gap=ADF_GEN4_RX_RINGS_OFFSET;(*h).tx_rings_mask=ADF_GEN4_TX_RINGS_MASK;(*h).ring_to_svc_map=ADF_GEN4_DEFAULT_RING_TO_SRV_MAP;
(*h).alloc_irq=Some(adf_isr_resource_alloc);(*h).free_irq=Some(adf_isr_resource_free);(*h).enable_error_correction=Some(adf_gen4_enable_error_correction);(*h).get_accel_mask=Some(adf_gen4_get_accel_mask);(*h).get_ae_mask=Some(get_ae_mask);(*h).get_num_accels=Some(adf_gen4_get_num_accels);(*h).get_num_aes=Some(adf_gen4_get_num_aes);(*h).get_sram_bar_id=Some(adf_gen4_get_sram_bar_id);(*h).get_etr_bar_id=Some(adf_gen4_get_etr_bar_id);(*h).get_misc_bar_id=Some(adf_gen4_get_misc_bar_id);(*h).get_arb_info=Some(adf_gen4_get_arb_info);(*h).get_admin_info=Some(adf_gen4_get_admin_info);(*h).get_accel_cap=Some(get_accel_cap);(*h).get_sku=Some(adf_gen4_get_sku);(*h).init_admin_comms=Some(adf_init_admin_comms);(*h).exit_admin_comms=Some(adf_exit_admin_comms);(*h).send_admin_init=Some(adf_send_admin_init);(*h).init_arb=Some(adf_init_arb);(*h).exit_arb=Some(adf_exit_arb);(*h).get_arb_mapping=Some(adf_get_arbiter_mapping);(*h).enable_ints=Some(adf_gen4_enable_ints);(*h).init_device=Some(adf_gen4_init_device);(*h).reset_device=Some(adf_reset_flr);
(*h).admin_ae_mask=ADF_420XX_ADMIN_AE_MASK;(*h).num_rps=ADF_GEN4_MAX_RPS;(*h).fw_name=ADF_420XX_FW;(*h).fw_mmp_name=ADF_420XX_MMP;(*h).uof_get_name=Some(uof_get_name_420xx);(*h).uof_get_num_objs=Some(uof_get_num_objs);(*h).uof_get_obj_type=Some(uof_get_obj_type);(*h).uof_get_ae_mask=Some(uof_get_ae_mask);(*h).get_rp_group=Some(get_rp_group);(*h).get_ena_thd_mask=Some(get_ena_thd_mask);(*h).set_msix_rttable=Some(adf_gen4_set_msix_default_rttable);(*h).set_ssm_wdtimer=Some(adf_gen4_set_ssm_wdtimer);(*h).get_ring_to_svc_map=Some(adf_gen4_get_ring_to_svc_map);(*h).disable_iov=Some(adf_disable_sriov);(*h).ring_pair_reset=Some(adf_gen4_ring_pair_reset);(*h).bank_state_save=Some(adf_bank_state_save);(*h).bank_state_restore=Some(adf_bank_state_restore);(*h).enable_pm=Some(adf_gen4_enable_pm);(*h).handle_pm_interrupt=Some(adf_gen4_handle_pm_interrupt);(*h).dev_config=Some(adf_gen4_dev_config);(*h).start_timer=Some(adf_timer_start);(*h).stop_timer=Some(adf_timer_stop);(*h).get_hb_clock=Some(adf_gen4_get_heartbeat_clock);(*h).num_hb_ctrs=ADF_NUM_HB_CNT_PER_AE;(*h).clock_frequency=ADF_420XX_AE_FREQ;(*h).services_supported=Some(adf_gen4_services_supported);(*h).get_svc_slice_cnt=Some(adf_gen4_get_svc_slice_cnt);(*h).accel_capabilities_ext_mask=ADF_ACCEL_CAPABILITIES_EXT_ZSTD_LZ4S;
adf_gen4_set_err_mask(&mut (*h).dev_err_mask);adf_gen4_init_hw_csr_ops(&mut (*h).csr_ops);adf_gen4_init_pf_pfvf_ops(&mut (*h).pfvf_ops);adf_gen4_init_dc_ops(&mut (*h).dc_ops);adf_gen4_init_ras_ops(&mut (*h).ras_ops);adf_gen4_init_tl_data(&mut (*h).tl_data);adf_gen4_init_vf_mig_ops(&mut (*h).vfmig_ops);adf_init_rl_data(&mut (*h).rl_data);}
pub unsafe fn adf_clean_hw_data_420xx(h:*mut adf_hw_device_data){(*(*h).dev_class).instances-=1;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
