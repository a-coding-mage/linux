// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2020 Intel Corporation */

// Dependencies supplied by the surrounding translation unit/build.

pub unsafe fn adf_gen2_get_num_accels(self_: *mut adf_hw_device_data) -> u32 {
    if self_.is_null() || (*self_).accel_mask == 0 {
        return 0;
    }
    ((*self_).accel_mask as u16).count_ones()
}

pub unsafe fn adf_gen2_get_num_aes(self_: *mut adf_hw_device_data) -> u32 {
    if self_.is_null() || (*self_).ae_mask == 0 {
        return 0;
    }
    (*self_).ae_mask.count_ones()
}

pub unsafe fn adf_gen2_enable_error_correction(accel_dev: *mut adf_accel_dev) {
    let hw_data = (*accel_dev).hw_device;
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    let accel_mask = (*hw_data).accel_mask;
    let ae_mask = (*hw_data).ae_mask;

    /* Enable Accel Engine error detection & correction */
    for i in 0..(*hw_data).num_engines {
        if (ae_mask & (1usize << i)) == 0 { continue; }
        let mut val = ADF_CSR_RD(pmisc_addr, ADF_GEN2_AE_CTX_ENABLES(i));
        val |= ADF_GEN2_ENABLE_AE_ECC_ERR;
        ADF_CSR_WR(pmisc_addr, ADF_GEN2_AE_CTX_ENABLES(i), val);
        val = ADF_CSR_RD(pmisc_addr, ADF_GEN2_AE_MISC_CONTROL(i));
        val |= ADF_GEN2_ENABLE_AE_ECC_PARITY_CORR;
        ADF_CSR_WR(pmisc_addr, ADF_GEN2_AE_MISC_CONTROL(i), val);
    }

    /* Enable shared memory error detection & correction */
    for i in 0..(*hw_data).num_accel {
        if (accel_mask & (1usize << i)) == 0 { continue; }
        let mut val = ADF_CSR_RD(pmisc_addr, ADF_GEN2_UERRSSMSH(i));
        val |= ADF_GEN2_ERRSSMSH_EN;
        ADF_CSR_WR(pmisc_addr, ADF_GEN2_UERRSSMSH(i), val);
        val = ADF_CSR_RD(pmisc_addr, ADF_GEN2_CERRSSMSH(i));
        val |= ADF_GEN2_ERRSSMSH_EN;
        ADF_CSR_WR(pmisc_addr, ADF_GEN2_CERRSSMSH(i), val);
    }
}

pub unsafe fn adf_gen2_cfg_iov_thds(accel_dev: *mut adf_accel_dev, enable: bool, num_a_regs: i32, num_b_regs: i32) {
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    for i in 0..num_a_regs {
        let mut reg = READ_CSR_AE2FUNCTION_MAP_A(pmisc_addr, i);
        if enable { reg |= AE2FUNCTION_MAP_VALID; } else { reg &= !AE2FUNCTION_MAP_VALID; }
        WRITE_CSR_AE2FUNCTION_MAP_A(pmisc_addr, i, reg);
    }
    for i in 0..num_b_regs {
        let mut reg = READ_CSR_AE2FUNCTION_MAP_B(pmisc_addr, i);
        if enable { reg |= AE2FUNCTION_MAP_VALID; } else { reg &= !AE2FUNCTION_MAP_VALID; }
        WRITE_CSR_AE2FUNCTION_MAP_B(pmisc_addr, i, reg);
    }
}

pub unsafe fn adf_gen2_get_admin_info(admin_csrs_info: *mut admin_info) {
    (*admin_csrs_info).mailbox_offset = ADF_MAILBOX_BASE_OFFSET;
    (*admin_csrs_info).admin_msg_ur = ADF_ADMINMSGUR_OFFSET;
    (*admin_csrs_info).admin_msg_lr = ADF_ADMINMSGLR_OFFSET;
}

pub unsafe fn adf_gen2_get_arb_info(arb_info: *mut arb_info) {
    (*arb_info).arb_cfg = ADF_ARB_CONFIG;
    (*arb_info).arb_offset = ADF_ARB_OFFSET;
    (*arb_info).wt2sam_offset = ADF_ARB_WRK_2_SER_MAP_OFFSET;
}

pub unsafe fn adf_gen2_enable_ints(accel_dev: *mut adf_accel_dev) {
    let addr = adf_get_pmisc_base(accel_dev);
    let val = if (*accel_dev).pf.vf_info.is_null() { (1u64 << GET_MAX_BANKS(accel_dev)) - 1 } else { 0 };
    ADF_CSR_WR(addr, ADF_GEN2_SMIAPF0_MASK_OFFSET, val);
    ADF_CSR_WR(addr, ADF_GEN2_SMIAPF1_MASK_OFFSET, ADF_GEN2_SMIA1_MASK);
}

pub unsafe fn adf_gen2_get_accel_cap(accel_dev: *mut adf_accel_dev) -> u32 {
    let hw_data = (*accel_dev).hw_device;
    let pdev = (*accel_dev).accel_pci_dev.pci_dev;
    let fuses = (*hw_data).fuses[ADF_FUSECTL0];
    let straps = (*hw_data).straps;
    let mut legfuses = 0u32;
    let mut capabilities = ICP_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC |
        ICP_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC | ICP_ACCEL_CAPABILITIES_AUTHENTICATION |
        ICP_ACCEL_CAPABILITIES_CIPHER | ICP_ACCEL_CAPABILITIES_COMPRESSION;
    pci_read_config_dword(pdev, ADF_DEVICE_LEGFUSE_OFFSET, &mut legfuses);
    if legfuses & ICP_ACCEL_MASK_CIPHER_SLICE != 0 { capabilities &= !ICP_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC; capabilities &= !ICP_ACCEL_CAPABILITIES_CIPHER; }
    if legfuses & ICP_ACCEL_MASK_PKE_SLICE != 0 { capabilities &= !ICP_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC; }
    if legfuses & ICP_ACCEL_MASK_AUTH_SLICE != 0 { capabilities &= !ICP_ACCEL_CAPABILITIES_AUTHENTICATION; capabilities &= !ICP_ACCEL_CAPABILITIES_CIPHER; }
    if legfuses & ICP_ACCEL_MASK_COMPRESS_SLICE != 0 { capabilities &= !ICP_ACCEL_CAPABILITIES_COMPRESSION; }
    if (straps | fuses) & ADF_POWERGATE_PKE != 0 { capabilities &= !ICP_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC; }
    if (straps | fuses) & ADF_POWERGATE_DC != 0 { capabilities &= !ICP_ACCEL_CAPABILITIES_COMPRESSION; }
    capabilities
}

pub unsafe fn adf_gen2_set_ssm_wdtimer(accel_dev: *mut adf_accel_dev) {
    let hw_data = (*accel_dev).hw_device;
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    let accel_mask = (*hw_data).accel_mask;
    for i in 0..(*hw_data).num_accel {
        if accel_mask & (1usize << i) == 0 { continue; }
        ADF_CSR_WR(pmisc_addr, ADF_SSMWDT(i), ADF_SSM_WDT_DEFAULT_VALUE);
        ADF_CSR_WR(pmisc_addr, ADF_SSMWDTPKE(i), ADF_SSM_WDT_PKE_DEFAULT_VALUE);
    }
}

unsafe fn adf_gen2_build_comp_block(ctx: *mut core::ffi::c_void, algo: adf_dc_algo) -> i32 {
    let req_tmpl = ctx as *mut icp_qat_fw_comp_req;
    let cd_pars = &mut (*req_tmpl).cd_pars;
    let header = &mut (*req_tmpl).comn_hdr;
    match algo { QAT_DEFLATE => header.service_cmd_id = ICP_QAT_FW_COMP_CMD_STATIC, _ => return -22 }
    cd_pars.u.sl.comp_slice_cfg_word[0] = ICP_QAT_HW_COMPRESSION_CONFIG_BUILD(ICP_QAT_HW_COMPRESSION_DIR_COMPRESS, ICP_QAT_HW_COMPRESSION_DELAYED_MATCH_DISABLED, ICP_QAT_HW_COMPRESSION_ALGO_DEFLATE, ICP_QAT_HW_COMPRESSION_DEPTH_1, ICP_QAT_HW_COMPRESSION_FILE_TYPE_0);
    0
}

unsafe fn adf_gen2_build_decomp_block(ctx: *mut core::ffi::c_void, algo: adf_dc_algo) -> i32 {
    let req_tmpl = ctx as *mut icp_qat_fw_comp_req;
    let cd_pars = &mut (*req_tmpl).cd_pars;
    let header = &mut (*req_tmpl).comn_hdr;
    match algo { QAT_DEFLATE => header.service_cmd_id = ICP_QAT_FW_COMP_CMD_DECOMPRESS, _ => return -22 }
    cd_pars.u.sl.comp_slice_cfg_word[0] = ICP_QAT_HW_COMPRESSION_CONFIG_BUILD(ICP_QAT_HW_COMPRESSION_DIR_DECOMPRESS, ICP_QAT_HW_COMPRESSION_DELAYED_MATCH_DISABLED, ICP_QAT_HW_COMPRESSION_ALGO_DEFLATE, ICP_QAT_HW_COMPRESSION_DEPTH_1, ICP_QAT_HW_COMPRESSION_FILE_TYPE_0);
    0
}

pub unsafe fn adf_gen2_init_dc_ops(dc_ops: *mut adf_dc_ops) {
    (*dc_ops).build_comp_block = Some(adf_gen2_build_comp_block);
    (*dc_ops).build_decomp_block = Some(adf_gen2_build_decomp_block);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
