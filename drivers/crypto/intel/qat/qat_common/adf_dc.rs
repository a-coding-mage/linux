// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2022 Intel Corporation */
// Dependencies supplied by the corresponding accelerator, data-center
// compression, and QAT firmware headers are intentionally external.

pub unsafe fn qat_comp_build_ctx(
    accel_dev: *mut adf_accel_dev,
    ctx: *mut core::ffi::c_void,
    algo: adf_dc_algo,
) -> i32 {
    let mut req_tmpl = ctx as *mut icp_qat_fw_comp_req;
    let comp_cd_ctrl = &mut (*req_tmpl).comp_cd_ctrl as *mut icp_qat_fw_comp_cd_hdr;
    let req_pars = &mut (*req_tmpl).comp_pars as *mut icp_qat_fw_comp_req_params;
    let header = &mut (*req_tmpl).comn_hdr as *mut icp_qat_fw_comn_req_hdr;
    let mut ret: i32;

    core::ptr::write_bytes(
        req_tmpl as *mut u8,
        0,
        core::mem::size_of::<icp_qat_fw_comp_req>(),
    );
    (*header).hdr_flags =
        ICP_QAT_FW_COMN_HDR_FLAGS_BUILD(ICP_QAT_FW_COMN_REQ_FLAG_SET);
    (*header).service_type = ICP_QAT_FW_COMN_REQ_CPM_FW_COMP;
    (*header).comn_req_flags = ICP_QAT_FW_COMN_FLAGS_BUILD(
        QAT_COMN_CD_FLD_TYPE_16BYTE_DATA,
        QAT_COMN_PTR_TYPE_SGL,
    );
    (*header).serv_specif_flags = ICP_QAT_FW_COMP_FLAGS_BUILD(
        ICP_QAT_FW_COMP_STATELESS_SESSION,
        ICP_QAT_FW_COMP_NOT_AUTO_SELECT_BEST,
        ICP_QAT_FW_COMP_NOT_ENH_AUTO_SELECT_BEST,
        ICP_QAT_FW_COMP_NOT_DISABLE_TYPE0_ENH_AUTO_SELECT_BEST,
        ICP_QAT_FW_COMP_ENABLE_SECURE_RAM_USED_AS_INTMD_BUF,
    );

    /* Build HW config block for compression */
    ret = (*GET_DC_OPS(accel_dev)).build_comp_block(ctx, algo);
    if ret != 0 {
        dev_err(&GET_DEV(accel_dev), "Failed to build compression block\n");
        return ret;
    }

    (*req_pars).crc.legacy.initial_adler = COMP_CPR_INITIAL_ADLER;
    (*req_pars).crc.legacy.initial_crc32 = COMP_CPR_INITIAL_CRC;
    (*req_pars).req_par_flags = ICP_QAT_FW_COMP_REQ_PARAM_FLAGS_BUILD(
        ICP_QAT_FW_COMP_SOP,
        ICP_QAT_FW_COMP_EOP,
        ICP_QAT_FW_COMP_BFINAL,
        ICP_QAT_FW_COMP_CNV,
        ICP_QAT_FW_COMP_CNV_RECOVERY,
        ICP_QAT_FW_COMP_NO_CNV_DFX,
        ICP_QAT_FW_COMP_CRC_MODE_LEGACY,
        ICP_QAT_FW_COMP_NO_XXHASH_ACC,
        ICP_QAT_FW_COMP_CNV_ERROR_NONE,
        ICP_QAT_FW_COMP_NO_APPEND_CRC,
        ICP_QAT_FW_COMP_NO_DROP_DATA,
        ICP_QAT_FW_COMP_NO_PARTIAL_DECOMPRESS,
    );
    ICP_QAT_FW_COMN_NEXT_ID_SET(comp_cd_ctrl, ICP_QAT_FW_SLICE_DRAM_WR);
    ICP_QAT_FW_COMN_CURR_ID_SET(comp_cd_ctrl, ICP_QAT_FW_SLICE_COMP);

    /* Fill second half of the template for decompression */
    core::ptr::copy_nonoverlapping(
        req_tmpl,
        req_tmpl.add(1),
        1,
    );
    req_tmpl = req_tmpl.add(1);

    /* Build HW config block for decompression */
    ret = (*GET_DC_OPS(accel_dev)).build_decomp_block(req_tmpl as *mut core::ffi::c_void, algo);
    if ret != 0 {
        dev_err(&GET_DEV(accel_dev), "Failed to build decompression block\n");
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
