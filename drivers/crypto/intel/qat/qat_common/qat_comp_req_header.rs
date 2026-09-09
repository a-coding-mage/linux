/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2022 Intel Corporation */

// Dependency supplied by icp_qat_fw_comp.h is intentionally external to this translation.

pub const QAT_COMP_REQ_SIZE: usize = core::mem::size_of::<icp_qat_fw_comp_req>();
pub const QAT_COMP_CTX_SIZE: usize = QAT_COMP_REQ_SIZE * 2;

#[inline]
pub unsafe fn qat_comp_create_req(
    ctx: *mut core::ffi::c_void,
    req: *mut core::ffi::c_void,
    src: u64,
    slen: u32,
    dst: u64,
    dlen: u32,
    opaque: u64,
) {
    let fw_tmpl = ctx as *mut icp_qat_fw_comp_req;
    let fw_req = req as *mut icp_qat_fw_comp_req;
    let req_pars = &mut (*fw_req).comp_pars;

    core::ptr::copy_nonoverlapping(fw_tmpl, fw_req, 1);
    (*fw_req).comn_mid.src_data_addr = src;
    (*fw_req).comn_mid.src_length = slen;
    (*fw_req).comn_mid.dest_data_addr = dst;
    (*fw_req).comn_mid.dst_length = dlen;
    (*fw_req).comn_mid.opaque_data = opaque;
    req_pars.comp_len = slen;
    req_pars.out_buffer_sz = dlen;
    (*fw_req).u3.asb_threshold.asb_value = (*fw_req).u3.asb_threshold.asb_value
        .wrapping_mul(slen >> 4);
}

#[inline]
pub unsafe fn qat_comp_create_compression_req(
    ctx: *mut core::ffi::c_void,
    req: *mut core::ffi::c_void,
    src: u64,
    slen: u32,
    dst: u64,
    dlen: u32,
    opaque: u64,
) {
    qat_comp_create_req(ctx, req, src, slen, dst, dlen, opaque);
}

#[inline]
pub unsafe fn qat_comp_create_decompression_req(
    ctx: *mut core::ffi::c_void,
    req: *mut core::ffi::c_void,
    src: u64,
    slen: u32,
    dst: u64,
    dlen: u32,
    opaque: u64,
) {
    let fw_tmpl = (ctx as *mut icp_qat_fw_comp_req).add(1);
    qat_comp_create_req(fw_tmpl.cast(), req, src, slen, dst, dlen, opaque);
}

#[inline]
pub unsafe fn qat_comp_get_consumed_ctr(resp: *mut core::ffi::c_void) -> u32 {
    let qat_resp = resp as *mut icp_qat_fw_comp_resp;
    (*qat_resp).comp_resp_pars.input_byte_counter
}

#[inline]
pub unsafe fn qat_comp_get_produced_ctr(resp: *mut core::ffi::c_void) -> u32 {
    let qat_resp = resp as *mut icp_qat_fw_comp_resp;
    (*qat_resp).comp_resp_pars.output_byte_counter
}

#[inline]
pub unsafe fn qat_comp_get_produced_adler32(resp: *mut core::ffi::c_void) -> u32 {
    let qat_resp = resp as *mut icp_qat_fw_comp_resp;
    (*qat_resp).comp_resp_pars.crc.legacy.curr_adler_32
}

#[inline]
pub unsafe fn qat_comp_get_opaque(resp: *mut core::ffi::c_void) -> u64 {
    let qat_resp = resp as *mut icp_qat_fw_comp_resp;
    (*qat_resp).opaque_data
}

#[inline]
pub unsafe fn qat_comp_get_cmp_err(resp: *mut core::ffi::c_void) -> i8 {
    let qat_resp = resp as *mut icp_qat_fw_comp_resp;
    (*qat_resp).comn_resp.comn_error.cmp_err_code
}

#[inline]
pub unsafe fn qat_comp_get_xlt_err(resp: *mut core::ffi::c_void) -> i8 {
    let qat_resp = resp as *mut icp_qat_fw_comp_resp;
    (*qat_resp).comn_resp.comn_error.xlat_err_code
}

#[inline]
pub unsafe fn qat_comp_get_cmp_status(resp: *mut core::ffi::c_void) -> i8 {
    let qat_resp = resp as *mut icp_qat_fw_comp_resp;
    let stat_filed: u8 = (*qat_resp).comn_resp.comn_status;
    ICP_QAT_FW_COMN_RESP_CMP_STAT_GET(stat_filed)
}

#[inline]
pub unsafe fn qat_comp_get_xlt_status(resp: *mut core::ffi::c_void) -> i8 {
    let qat_resp = resp as *mut icp_qat_fw_comp_resp;
    let stat_filed: u8 = (*qat_resp).comn_resp.comn_status;
    ICP_QAT_FW_COMN_RESP_XLAT_STAT_GET(stat_filed)
}

#[inline]
pub unsafe fn qat_comp_get_cmp_cnv_flag(resp: *mut core::ffi::c_void) -> u8 {
    let qat_resp = resp as *mut icp_qat_fw_comp_resp;
    let flags: u8 = (*qat_resp).comn_resp.hdr_flags;
    ICP_QAT_FW_COMN_HDR_CNV_FLAG_GET(flags)
}

#[inline]
pub unsafe fn qat_comp_get_cmp_uncomp_flag(resp: *mut core::ffi::c_void) -> u8 {
    let qat_resp = resp as *mut icp_qat_fw_comp_resp;
    let flags: u8 = (*qat_resp).comn_resp.hdr_flags;
    ICP_QAT_FW_COMN_HDR_ST_BLK_FLAG_GET(flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
