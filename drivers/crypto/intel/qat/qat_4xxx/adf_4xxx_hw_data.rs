// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2020 - 2021 Intel Corporation */

// Dependencies supplied by the surrounding kernel/QAT sources are intentionally
// referenced here but not redefined.

const ADF_AE_GROUP_0: u32 = GENMASK(3, 0);
const ADF_AE_GROUP_1: u32 = GENMASK(7, 4);
const ADF_AE_GROUP_2: u32 = BIT(8);
const ENA_THD_MASK_ASYM: u32 = GENMASK(1, 0);
const ENA_THD_MASK_ASYM_401XX: u32 = GENMASK(5, 0);
const ENA_THD_MASK_SYM: u32 = GENMASK(6, 0);
const ENA_THD_MASK_DC: u32 = GENMASK(1, 0);

static mut adf_4xxx_fw_objs: [*const c_char; 4] = [
    ADF_4XXX_SYM_OBJ, ADF_4XXX_ASYM_OBJ, ADF_4XXX_DC_OBJ, ADF_4XXX_ADMIN_OBJ,
];
static mut adf_402xx_fw_objs: [*const c_char; 4] = [
    ADF_402XX_SYM_OBJ, ADF_402XX_ASYM_OBJ, ADF_402XX_DC_OBJ, ADF_402XX_ADMIN_OBJ,
];

static adf_fw_cy_config: [struct_adf_fw_config; 3] = [
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_SYM_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_ASYM_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_dc_config: [struct_adf_fw_config; 3] = [
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_DC_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_DC_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_sym_config: [struct_adf_fw_config; 3] = [
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_SYM_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_SYM_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_asym_config: [struct_adf_fw_config; 3] = [
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_ASYM_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_ASYM_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_asym_dc_config: [struct_adf_fw_config; 3] = [
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_ASYM_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_DC_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_sym_dc_config: [struct_adf_fw_config; 3] = [
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_SYM_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_DC_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ADMIN_OBJ },
];
static adf_fw_dcc_config: [struct_adf_fw_config; 3] = [
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_DC_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_SYM_OBJ },
    struct_adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ADMIN_OBJ },
];

static mut adf_4xxx_class: struct_adf_hw_device_class = struct_adf_hw_device_class {
    name: ADF_4XXX_DEVICE_NAME, type_: DEV_4XXX, instances: 0,
};

unsafe fn get_ae_mask(self_: *mut struct_adf_hw_device_data) -> u32 {
    let fuses = (*self_).fuses[ADF_FUSECTL4 as usize];
    let mut mask = ADF_4XXX_ACCELENGINES_MASK;
    if test_bit(0, &fuses) { mask &= !ADF_AE_GROUP_0; }
    if test_bit(4, &fuses) { mask &= !ADF_AE_GROUP_1; }
    if test_bit(8, &fuses) { mask &= !ADF_AE_GROUP_2; }
    mask
}

unsafe fn get_accel_cap(accel_dev: *mut struct_adf_accel_dev) -> u32 {
    let pdev = (*(*accel_dev).accel_pci_dev).pci_dev;
    let mut capabilities_sym = ICP_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC | ICP_ACCEL_CAPABILITIES_CIPHER | ICP_ACCEL_CAPABILITIES_AUTHENTICATION | ICP_ACCEL_CAPABILITIES_SHA3 | ICP_ACCEL_CAPABILITIES_SHA3_EXT | ICP_ACCEL_CAPABILITIES_HKDF | ICP_ACCEL_CAPABILITIES_CHACHA_POLY | ICP_ACCEL_CAPABILITIES_AESGCM_SPC | ICP_ACCEL_CAPABILITIES_SM3 | ICP_ACCEL_CAPABILITIES_SM4 | ICP_ACCEL_CAPABILITIES_AES_V2;
    let mut capabilities_asym = ICP_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC | ICP_ACCEL_CAPABILITIES_CIPHER | ICP_ACCEL_CAPABILITIES_SM2 | ICP_ACCEL_CAPABILITIES_ECEDMONT;
    let mut capabilities_dc = ICP_ACCEL_CAPABILITIES_COMPRESSION | ICP_ACCEL_CAPABILITIES_LZ4_COMPRESSION | ICP_ACCEL_CAPABILITIES_LZ4S_COMPRESSION | ICP_ACCEL_CAPABILITIES_CNV_INTEGRITY64;
    let mut fusectl1 = 0u32;
    pci_read_config_dword(pdev, ADF_GEN4_FUSECTL1_OFFSET, &mut fusectl1);
    if fusectl1 & ICP_ACCEL_GEN4_MASK_CIPHER_SLICE != 0 { capabilities_sym &= !(ICP_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC | ICP_ACCEL_CAPABILITIES_HKDF | ICP_ACCEL_CAPABILITIES_CIPHER); }
    if fusectl1 & ICP_ACCEL_GEN4_MASK_UCS_SLICE != 0 { capabilities_sym &= !(ICP_ACCEL_CAPABILITIES_CHACHA_POLY | ICP_ACCEL_CAPABILITIES_AESGCM_SPC | ICP_ACCEL_CAPABILITIES_AES_V2 | ICP_ACCEL_CAPABILITIES_CIPHER); }
    if fusectl1 & ICP_ACCEL_GEN4_MASK_AUTH_SLICE != 0 { capabilities_sym &= !(ICP_ACCEL_CAPABILITIES_AUTHENTICATION | ICP_ACCEL_CAPABILITIES_SHA3 | ICP_ACCEL_CAPABILITIES_SHA3_EXT | ICP_ACCEL_CAPABILITIES_CIPHER); }
    if fusectl1 & ICP_ACCEL_GEN4_MASK_SMX_SLICE != 0 { capabilities_sym &= !(ICP_ACCEL_CAPABILITIES_SM3 | ICP_ACCEL_CAPABILITIES_SM4); }
    if fusectl1 & ICP_ACCEL_GEN4_MASK_PKE_SLICE != 0 { capabilities_asym &= !(ICP_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC | ICP_ACCEL_CAPABILITIES_SM2 | ICP_ACCEL_CAPABILITIES_ECEDMONT); }
    if fusectl1 & ICP_ACCEL_GEN4_MASK_COMPRESS_SLICE != 0 { capabilities_dc &= !(ICP_ACCEL_CAPABILITIES_COMPRESSION | ICP_ACCEL_CAPABILITIES_LZ4_COMPRESSION | ICP_ACCEL_CAPABILITIES_LZ4S_COMPRESSION | ICP_ACCEL_CAPABILITIES_CNV_INTEGRITY64); }
    match adf_get_service_enabled(accel_dev) {
        SVC_SYM_ASYM => capabilities_sym | capabilities_asym,
        SVC_DC => capabilities_dc,
        SVC_DCC => (capabilities_dc | capabilities_sym) & !ICP_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC,
        SVC_SYM => capabilities_sym, SVC_ASYM => capabilities_asym,
        SVC_ASYM_DC => capabilities_asym | capabilities_dc,
        SVC_SYM_DC => capabilities_sym | capabilities_dc, _ => 0,
    }
}

unsafe fn adf_get_arbiter_mapping(accel_dev: *mut struct_adf_accel_dev) -> *const u32 {
    if adf_gen4_init_thd2arb_map(accel_dev) != 0 { dev_warn(&GET_DEV(accel_dev), "Failed to generate thread to arbiter mapping"); }
    GET_HW_DATA(accel_dev).thd_to_arb_map.as_ptr()
}

unsafe fn adf_init_rl_data(rl_data: *mut struct_adf_rl_hw_data) {
    (*rl_data).pciout_tb_offset = ADF_GEN4_RL_TOKEN_PCIEOUT_BUCKET_OFFSET;
    (*rl_data).pciin_tb_offset = ADF_GEN4_RL_TOKEN_PCIEIN_BUCKET_OFFSET;
    (*rl_data).r2l_offset = ADF_GEN4_RL_R2L_OFFSET;
    (*rl_data).l2c_offset = ADF_GEN4_RL_L2C_OFFSET;
    (*rl_data).c2s_offset = ADF_GEN4_RL_C2S_OFFSET;
    (*rl_data).pcie_scale_div = ADF_4XXX_RL_PCIE_SCALE_FACTOR_DIV;
    (*rl_data).pcie_scale_mul = ADF_4XXX_RL_PCIE_SCALE_FACTOR_MUL;
    (*rl_data).dcpr_correction = ADF_4XXX_RL_DCPR_CORRECTION;
    (*rl_data).max_tp[SVC_ASYM as usize] = ADF_4XXX_RL_MAX_TP_ASYM;
    (*rl_data).max_tp[SVC_SYM as usize] = ADF_4XXX_RL_MAX_TP_SYM;
    (*rl_data).max_tp[SVC_DC as usize] = ADF_4XXX_RL_MAX_TP_DC;
    (*rl_data).scan_interval = ADF_4XXX_RL_SCANS_PER_SEC;
    (*rl_data).scale_ref = ADF_4XXX_RL_SLICE_REF;
    adf_gen4_init_num_svc_aes(rl_data);
}

unsafe fn uof_get_num_objs(_: *mut struct_adf_accel_dev) -> u32 { adf_fw_cy_config.len() as u32 }
unsafe fn get_fw_config(accel_dev: *mut struct_adf_accel_dev) -> *const struct_adf_fw_config {
    match adf_get_service_enabled(accel_dev) { SVC_SYM_ASYM => adf_fw_cy_config.as_ptr(), SVC_DC => adf_fw_dc_config.as_ptr(), SVC_DCC => adf_fw_dcc_config.as_ptr(), SVC_SYM => adf_fw_sym_config.as_ptr(), SVC_ASYM => adf_fw_asym_config.as_ptr(), SVC_ASYM_DC => adf_fw_asym_dc_config.as_ptr(), SVC_SYM_DC => adf_fw_sym_dc_config.as_ptr(), _ => core::ptr::null() }
}
unsafe fn get_rp_group(accel_dev: *mut struct_adf_accel_dev, ae_mask: u32) -> i32 { match ae_mask { ADF_AE_GROUP_0 => RP_GROUP_0, ADF_AE_GROUP_1 => RP_GROUP_1, _ => { dev_dbg(&GET_DEV(accel_dev), "ae_mask not recognized"); -EINVAL } } }
unsafe fn get_ena_thd_mask(accel_dev: *mut struct_adf_accel_dev, obj_num: u32) -> u32 { if obj_num >= uof_get_num_objs(accel_dev) { return ADF_GEN4_ENA_THD_MASK_ERROR; } let p = get_fw_config(accel_dev); if p.is_null() { return ADF_GEN4_ENA_THD_MASK_ERROR; } match (*p.add(obj_num as usize)).obj { ADF_FW_ASYM_OBJ => ENA_THD_MASK_ASYM, ADF_FW_SYM_OBJ => ENA_THD_MASK_SYM, ADF_FW_DC_OBJ => ENA_THD_MASK_DC, _ => ADF_GEN4_ENA_THD_MASK_ERROR } }
unsafe fn get_ena_thd_mask_401xx(accel_dev: *mut struct_adf_accel_dev, obj_num: u32) -> u32 { if obj_num >= uof_get_num_objs(accel_dev) { return ADF_GEN4_ENA_THD_MASK_ERROR; } let p = get_fw_config(accel_dev); if p.is_null() { return ADF_GEN4_ENA_THD_MASK_ERROR; } match (*p.add(obj_num as usize)).obj { ADF_FW_ASYM_OBJ => ENA_THD_MASK_ASYM_401XX, ADF_FW_SYM_OBJ => ENA_THD_MASK_SYM, ADF_FW_DC_OBJ => ENA_THD_MASK_DC, _ => ADF_GEN4_ENA_THD_MASK_ERROR } }
unsafe fn uof_get_name(accel_dev: *mut struct_adf_accel_dev, obj_num: u32, fw_objs: *const *const c_char, num_objs: i32) -> *const c_char { let p = get_fw_config(accel_dev); if p.is_null() { return core::ptr::null(); } let id = (*p.add(obj_num as usize)).obj as i32; if id < 0 || id >= num_objs { core::ptr::null() } else { *fw_objs.add(id as usize) } }
unsafe fn uof_get_name_4xxx(a: *mut struct_adf_accel_dev, n: u32) -> *const c_char { uof_get_name(a, n, adf_4xxx_fw_objs.as_ptr(), 4) }
unsafe fn uof_get_name_402xx(a: *mut struct_adf_accel_dev, n: u32) -> *const c_char { uof_get_name(a, n, adf_402xx_fw_objs.as_ptr(), 4) }
unsafe fn uof_get_obj_type(a: *mut struct_adf_accel_dev, n: u32) -> i32 { if n >= uof_get_num_objs(a) { return -EINVAL; } let p = get_fw_config(a); if p.is_null() { -EINVAL } else { (*p.add(n as usize)).obj } }
unsafe fn uof_get_ae_mask(a: *mut struct_adf_accel_dev, n: u32) -> u32 { let p = get_fw_config(a); if p.is_null() { 0 } else { (*p.add(n as usize)).ae_mask } }
unsafe fn adf_gen4_set_err_mask(m: *mut struct_adf_dev_err_mask) { (*m).cppagentcmdpar_mask = ADF_4XXX_HICPPAGENTCMDPARERRLOG_MASK; (*m).parerr_ath_cph_mask = ADF_4XXX_PARITYERRORMASK_ATH_CPH_MASK; (*m).parerr_cpr_xlt_mask = ADF_4XXX_PARITYERRORMASK_CPR_XLT_MASK; (*m).parerr_dcpr_ucs_mask = ADF_4XXX_PARITYERRORMASK_DCPR_UCS_MASK; (*m).parerr_pke_mask = ADF_4XXX_PARITYERRORMASK_PKE_MASK; (*m).ssmfeatren_mask = ADF_4XXX_SSMFEATREN_MASK; }

pub unsafe fn adf_init_hw_data_4xxx(hw_data: *mut struct_adf_hw_device_data, dev_id: u32) {
    (*hw_data).dev_class = &mut adf_4xxx_class; (*hw_data).instance_id = adf_4xxx_class.instances; adf_4xxx_class.instances += 1;
    (*hw_data).num_banks = ADF_GEN4_ETR_MAX_BANKS; (*hw_data).num_banks_per_vf = ADF_GEN4_NUM_BANKS_PER_VF; (*hw_data).num_rings_per_bank = ADF_GEN4_NUM_RINGS_PER_BANK; (*hw_data).num_accel = ADF_GEN4_MAX_ACCELERATORS; (*hw_data).num_engines = ADF_4XXX_MAX_ACCELENGINES; (*hw_data).num_logical_accel = 1; (*hw_data).tx_rx_gap = ADF_GEN4_RX_RINGS_OFFSET; (*hw_data).tx_rings_mask = ADF_GEN4_TX_RINGS_MASK; (*hw_data).ring_to_svc_map = ADF_GEN4_DEFAULT_RING_TO_SRV_MAP;
    (*hw_data).alloc_irq = adf_isr_resource_alloc; (*hw_data).free_irq = adf_isr_resource_free; (*hw_data).enable_error_correction = adf_gen4_enable_error_correction; (*hw_data).get_accel_mask = adf_gen4_get_accel_mask; (*hw_data).get_ae_mask = get_ae_mask; (*hw_data).get_num_accels = adf_gen4_get_num_accels; (*hw_data).get_num_aes = adf_gen4_get_num_aes; (*hw_data).get_sram_bar_id = adf_gen4_get_sram_bar_id; (*hw_data).get_etr_bar_id = adf_gen4_get_etr_bar_id; (*hw_data).get_misc_bar_id = adf_gen4_get_misc_bar_id; (*hw_data).get_arb_info = adf_gen4_get_arb_info; (*hw_data).get_admin_info = adf_gen4_get_admin_info; (*hw_data).get_accel_cap = get_accel_cap; (*hw_data).get_sku = adf_gen4_get_sku; (*hw_data).init_admin_comms = adf_init_admin_comms; (*hw_data).exit_admin_comms = adf_exit_admin_comms; (*hw_data).send_admin_init = adf_send_admin_init; (*hw_data).init_arb = adf_init_arb; (*hw_data).exit_arb = adf_exit_arb; (*hw_data).get_arb_mapping = adf_get_arbiter_mapping; (*hw_data).enable_ints = adf_gen4_enable_ints; (*hw_data).init_device = adf_gen4_init_device; (*hw_data).reset_device = adf_reset_flr; (*hw_data).admin_ae_mask = ADF_4XXX_ADMIN_AE_MASK; (*hw_data).num_rps = ADF_GEN4_MAX_RPS;
    match dev_id { PCI_DEVICE_ID_INTEL_QAT_402XX => { (*hw_data).fw_name = ADF_402XX_FW; (*hw_data).fw_mmp_name = ADF_402XX_MMP; (*hw_data).uof_get_name = uof_get_name_402xx; (*hw_data).get_ena_thd_mask = get_ena_thd_mask; }, PCI_DEVICE_ID_INTEL_QAT_401XX => { (*hw_data).fw_name = ADF_4XXX_FW; (*hw_data).fw_mmp_name = ADF_4XXX_MMP; (*hw_data).uof_get_name = uof_get_name_4xxx; (*hw_data).get_ena_thd_mask = get_ena_thd_mask_401xx; }, _ => { (*hw_data).fw_name = ADF_4XXX_FW; (*hw_data).fw_mmp_name = ADF_4XXX_MMP; (*hw_data).uof_get_name = uof_get_name_4xxx; (*hw_data).get_ena_thd_mask = get_ena_thd_mask; } }
    (*hw_data).uof_get_num_objs = uof_get_num_objs; (*hw_data).uof_get_obj_type = uof_get_obj_type; (*hw_data).uof_get_ae_mask = uof_get_ae_mask; (*hw_data).get_rp_group = get_rp_group; (*hw_data).set_msix_rttable = adf_gen4_set_msix_default_rttable; (*hw_data).set_ssm_wdtimer = adf_gen4_set_ssm_wdtimer; (*hw_data).get_ring_to_svc_map = adf_gen4_get_ring_to_svc_map; (*hw_data).disable_iov = adf_disable_sriov; (*hw_data).ring_pair_reset = adf_gen4_ring_pair_reset; (*hw_data).bank_state_save = adf_bank_state_save; (*hw_data).bank_state_restore = adf_bank_state_restore; (*hw_data).enable_pm = adf_gen4_enable_pm; (*hw_data).handle_pm_interrupt = adf_gen4_handle_pm_interrupt; (*hw_data).dev_config = adf_gen4_dev_config; (*hw_data).start_timer = adf_timer_start; (*hw_data).stop_timer = adf_timer_stop; (*hw_data).get_hb_clock = adf_gen4_get_heartbeat_clock; (*hw_data).num_hb_ctrs = ADF_NUM_HB_CNT_PER_AE; (*hw_data).clock_frequency = ADF_4XXX_AE_FREQ; (*hw_data).services_supported = adf_gen4_services_supported; (*hw_data).get_svc_slice_cnt = adf_gen4_get_svc_slice_cnt; (*hw_data).accel_capabilities_ext_mask = ADF_ACCEL_CAPABILITIES_EXT_ZSTD_LZ4S;
    adf_gen4_set_err_mask(&mut (*hw_data).dev_err_mask); adf_gen4_init_hw_csr_ops(&mut (*hw_data).csr_ops); adf_gen4_init_pf_pfvf_ops(&mut (*hw_data).pfvf_ops); adf_gen4_init_dc_ops(&mut (*hw_data).dc_ops); adf_gen4_init_ras_ops(&mut (*hw_data).ras_ops); adf_gen4_init_tl_data(&mut (*hw_data).tl_data); adf_gen4_init_vf_mig_ops(&mut (*hw_data).vfmig_ops); adf_init_rl_data(&mut (*hw_data).rl_data);
}

pub unsafe fn adf_clean_hw_data_4xxx(hw_data: *mut struct_adf_hw_device_data) { (*(*hw_data).dev_class).instances -= 1; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
