// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2021 Intel Corporation */

// External dependencies supplied by the surrounding driver translation.

const ADF_DH895XCC_VF_MSK: u32 = 0xFFFF_FFFF;

/* Worker thread to service arbiter mappings */
static THRD_TO_ARB_MAP: [u32; ADF_DH895XCC_MAX_ACCELENGINES as usize] = [
    0x12222AAA, 0x11666666, 0x12222AAA, 0x11666666,
    0x12222AAA, 0x11222222, 0x12222AAA, 0x11222222,
    0x12222AAA, 0x11222222, 0x12222AAA, 0x11222222,
];

static mut dh895xcc_class: adf_hw_device_class = adf_hw_device_class {
    name: ADF_DH895XCC_DEVICE_NAME,
    type_: DEV_DH895XCC,
    instances: 0,
};

unsafe fn get_accel_mask(self_: *mut adf_hw_device_data) -> u32 {
    let fuses = (*self_).fuses[ADF_FUSECTL0 as usize];
    ((!fuses) >> ADF_DH895XCC_ACCELERATORS_REG_OFFSET)
        & ADF_DH895XCC_ACCELERATORS_MASK
}

unsafe fn get_ae_mask(self_: *mut adf_hw_device_data) -> u32 {
    let fuses = (*self_).fuses[ADF_FUSECTL0 as usize];
    (!fuses) & ADF_DH895XCC_ACCELENGINES_MASK
}

unsafe fn get_misc_bar_id(_self_: *mut adf_hw_device_data) -> u32 {
    ADF_DH895XCC_PMISC_BAR
}

unsafe fn get_ts_clock(self_: *mut adf_hw_device_data) -> u32 {
    /* Timestamp update interval is 16 AE clock ticks for dh895xcc. */
    (*self_).clock_frequency / 16
}

unsafe fn get_etr_bar_id(_self_: *mut adf_hw_device_data) -> u32 {
    ADF_DH895XCC_ETR_BAR
}

unsafe fn get_sram_bar_id(_self_: *mut adf_hw_device_data) -> u32 {
    ADF_DH895XCC_SRAM_BAR
}

unsafe fn get_accel_cap(accel_dev: *mut adf_accel_dev) -> u32 {
    let pdev = (*accel_dev).accel_pci_dev.pci_dev;
    let mut capabilities = ICP_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC
        | ICP_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC
        | ICP_ACCEL_CAPABILITIES_AUTHENTICATION
        | ICP_ACCEL_CAPABILITIES_CIPHER
        | ICP_ACCEL_CAPABILITIES_COMPRESSION;
    let mut legfuses: u32 = 0;

    pci_read_config_dword(pdev, ADF_DEVICE_LEGFUSE_OFFSET, &mut legfuses);

    /* A set bit in legfuses means the feature is OFF in this SKU */
    if legfuses & ICP_ACCEL_MASK_CIPHER_SLICE != 0 {
        capabilities &= !ICP_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC;
        capabilities &= !ICP_ACCEL_CAPABILITIES_CIPHER;
    }
    if legfuses & ICP_ACCEL_MASK_PKE_SLICE != 0 {
        capabilities &= !ICP_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC;
    }
    if legfuses & ICP_ACCEL_MASK_AUTH_SLICE != 0 {
        capabilities &= !ICP_ACCEL_CAPABILITIES_AUTHENTICATION;
        capabilities &= !ICP_ACCEL_CAPABILITIES_CIPHER;
    }
    if legfuses & ICP_ACCEL_MASK_COMPRESS_SLICE != 0 {
        capabilities &= !ICP_ACCEL_CAPABILITIES_COMPRESSION;
    }
    capabilities
}

unsafe fn get_sku(self_: *mut adf_hw_device_data) -> dev_sku_info {
    let sku = (((*self_).fuses[ADF_FUSECTL0 as usize]
        & ADF_DH895XCC_FUSECTL_SKU_MASK) >> ADF_DH895XCC_FUSECTL_SKU_SHIFT) as i32;
    match sku {
        ADF_DH895XCC_FUSECTL_SKU_1 => DEV_SKU_1,
        ADF_DH895XCC_FUSECTL_SKU_2 => DEV_SKU_2,
        ADF_DH895XCC_FUSECTL_SKU_3 => DEV_SKU_3,
        ADF_DH895XCC_FUSECTL_SKU_4 => DEV_SKU_4,
        _ => DEV_SKU_UNKNOWN,
    }
}

unsafe fn adf_get_arbiter_mapping(_accel_dev: *mut adf_accel_dev) -> *const u32 {
    THRD_TO_ARB_MAP.as_ptr()
}

unsafe fn enable_vf2pf_interrupts(pmisc_addr: *mut core::ffi::c_void, vf_mask: u32) {
    if vf_mask & 0xFFFF != 0 {
        let val = ADF_CSR_RD(pmisc_addr, ADF_GEN2_ERRMSK3)
            & !ADF_DH895XCC_ERR_MSK_VF2PF_L(vf_mask);
        ADF_CSR_WR(pmisc_addr, ADF_GEN2_ERRMSK3, val);
    }
    if vf_mask >> 16 != 0 {
        let val = ADF_CSR_RD(pmisc_addr, ADF_GEN2_ERRMSK5)
            & !ADF_DH895XCC_ERR_MSK_VF2PF_U(vf_mask);
        ADF_CSR_WR(pmisc_addr, ADF_GEN2_ERRMSK5, val);
    }
}

unsafe fn disable_all_vf2pf_interrupts(pmisc_addr: *mut core::ffi::c_void) {
    let val = ADF_CSR_RD(pmisc_addr, ADF_GEN2_ERRMSK3)
        | ADF_DH895XCC_ERR_MSK_VF2PF_L(ADF_DH895XCC_VF_MSK);
    ADF_CSR_WR(pmisc_addr, ADF_GEN2_ERRMSK3, val);
    let val = ADF_CSR_RD(pmisc_addr, ADF_GEN2_ERRMSK5)
        | ADF_DH895XCC_ERR_MSK_VF2PF_U(ADF_DH895XCC_VF_MSK);
    ADF_CSR_WR(pmisc_addr, ADF_GEN2_ERRMSK5, val);
}

unsafe fn disable_pending_vf2pf_interrupts(pmisc_addr: *mut core::ffi::c_void) -> u32 {
    let errsou3 = ADF_CSR_RD(pmisc_addr, ADF_GEN2_ERRSOU3);
    let errsou5 = ADF_CSR_RD(pmisc_addr, ADF_GEN2_ERRSOU5);
    let sources = ADF_DH895XCC_ERR_REG_VF2PF_L(errsou3)
        | ADF_DH895XCC_ERR_REG_VF2PF_U(errsou5);
    if sources == 0 { return 0; }
    let errmsk3 = ADF_CSR_RD(pmisc_addr, ADF_GEN2_ERRMSK3);
    let errmsk5 = ADF_CSR_RD(pmisc_addr, ADF_GEN2_ERRMSK5);
    let disabled = ADF_DH895XCC_ERR_REG_VF2PF_L(errmsk3)
        | ADF_DH895XCC_ERR_REG_VF2PF_U(errmsk5);
    let pending = sources & !disabled;
    if pending == 0 { return 0; }
    let mut errmsk3 = errmsk3 | ADF_DH895XCC_ERR_MSK_VF2PF_L(ADF_DH895XCC_VF_MSK);
    let mut errmsk5 = errmsk5 | ADF_DH895XCC_ERR_MSK_VF2PF_U(ADF_DH895XCC_VF_MSK);
    ADF_CSR_WR(pmisc_addr, ADF_GEN2_ERRMSK3, errmsk3);
    ADF_CSR_WR(pmisc_addr, ADF_GEN2_ERRMSK5, errmsk5);
    errmsk3 &= !ADF_DH895XCC_ERR_MSK_VF2PF_L(ADF_DH895XCC_VF_MSK);
    errmsk5 &= !ADF_DH895XCC_ERR_MSK_VF2PF_U(ADF_DH895XCC_VF_MSK);
    errmsk3 |= ADF_DH895XCC_ERR_MSK_VF2PF_L(sources | disabled);
    errmsk5 |= ADF_DH895XCC_ERR_MSK_VF2PF_U(sources | disabled);
    ADF_CSR_WR(pmisc_addr, ADF_GEN2_ERRMSK3, errmsk3);
    ADF_CSR_WR(pmisc_addr, ADF_GEN2_ERRMSK5, errmsk5);
    pending
}

unsafe fn configure_iov_threads(accel_dev: *mut adf_accel_dev, enable: bool) {
    adf_gen2_cfg_iov_thds(accel_dev, enable,
        ADF_DH895XCC_AE2FUNC_MAP_GRP_A_NUM_REGS,
        ADF_DH895XCC_AE2FUNC_MAP_GRP_B_NUM_REGS);
}

pub unsafe fn adf_init_hw_data_dh895xcc(hw_data: *mut adf_hw_device_data) {
    (*hw_data).dev_class = &mut dh895xcc_class;
    (*hw_data).instance_id = dh895xcc_class.instances;
    dh895xcc_class.instances += 1;
    (*hw_data).num_banks = ADF_DH895XCC_ETR_MAX_BANKS;
    (*hw_data).num_rings_per_bank = ADF_ETR_MAX_RINGS_PER_BANK;
    (*hw_data).num_accel = ADF_DH895XCC_MAX_ACCELERATORS;
    (*hw_data).num_logical_accel = 1;
    (*hw_data).num_engines = ADF_DH895XCC_MAX_ACCELENGINES;
    (*hw_data).tx_rx_gap = ADF_GEN2_RX_RINGS_OFFSET;
    (*hw_data).tx_rings_mask = ADF_GEN2_TX_RINGS_MASK;
    (*hw_data).ring_to_svc_map = ADF_GEN2_DEFAULT_RING_TO_SRV_MAP;
    (*hw_data).alloc_irq = adf_isr_resource_alloc;
    (*hw_data).free_irq = adf_isr_resource_free;
    (*hw_data).enable_error_correction = adf_gen2_enable_error_correction;
    (*hw_data).get_accel_mask = get_accel_mask;
    (*hw_data).get_ae_mask = get_ae_mask;
    (*hw_data).get_accel_cap = get_accel_cap;
    (*hw_data).get_num_accels = adf_gen2_get_num_accels;
    (*hw_data).get_num_aes = adf_gen2_get_num_aes;
    (*hw_data).get_etr_bar_id = get_etr_bar_id;
    (*hw_data).get_misc_bar_id = get_misc_bar_id;
    (*hw_data).get_admin_info = adf_gen2_get_admin_info;
    (*hw_data).get_arb_info = adf_gen2_get_arb_info;
    (*hw_data).get_sram_bar_id = get_sram_bar_id;
    (*hw_data).get_sku = get_sku;
    (*hw_data).fw_name = ADF_DH895XCC_FW;
    (*hw_data).fw_mmp_name = ADF_DH895XCC_MMP;
    (*hw_data).init_admin_comms = adf_init_admin_comms;
    (*hw_data).exit_admin_comms = adf_exit_admin_comms;
    (*hw_data).configure_iov_threads = configure_iov_threads;
    (*hw_data).send_admin_init = adf_send_admin_init;
    (*hw_data).init_arb = adf_init_arb;
    (*hw_data).exit_arb = adf_exit_arb;
    (*hw_data).get_arb_mapping = adf_get_arbiter_mapping;
    (*hw_data).enable_ints = adf_gen2_enable_ints;
    (*hw_data).reset_device = adf_reset_sbr;
    (*hw_data).disable_iov = adf_disable_sriov;
    (*hw_data).dev_config = adf_gen2_dev_config;
    (*hw_data).clock_frequency = ADF_DH895X_AE_FREQ;
    (*hw_data).get_hb_clock = get_ts_clock;
    (*hw_data).num_hb_ctrs = ADF_NUM_HB_CNT_PER_AE;
    (*hw_data).check_hb_ctrs = adf_heartbeat_check_ctrs;
    adf_gen2_init_pf_pfvf_ops(&mut (*hw_data).pfvf_ops);
    (*hw_data).pfvf_ops.enable_vf2pf_interrupts = enable_vf2pf_interrupts;
    (*hw_data).pfvf_ops.disable_all_vf2pf_interrupts = disable_all_vf2pf_interrupts;
    (*hw_data).pfvf_ops.disable_pending_vf2pf_interrupts = disable_pending_vf2pf_interrupts;
    adf_gen2_init_hw_csr_ops(&mut (*hw_data).csr_ops);
    adf_gen2_init_dc_ops(&mut (*hw_data).dc_ops);
}

pub unsafe fn adf_clean_hw_data_dh895xcc(hw_data: *mut adf_hw_device_data) {
    (*(*hw_data).dev_class).instances -= 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
