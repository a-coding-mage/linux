/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependencies: linux/types.h and icp_qat_hw.h are supplied externally.

#[macro_export]
macro_rules! QAT_FIELD_SET {
    ($flags:expr, $val:expr, $bitpos:expr, $mask:expr) => {
        $flags = (($flags & !(($mask) << ($bitpos))) | (($val & ($mask)) << ($bitpos)))
    };
}
#[macro_export]
macro_rules! QAT_FIELD_GET {
    ($flags:expr, $bitpos:expr, $mask:expr) => { (($flags >> ($bitpos)) & ($mask)) };
}

pub const ICP_QAT_FW_REQ_DEFAULT_SZ: u32 = 128;
pub const ICP_QAT_FW_RESP_DEFAULT_SZ: u32 = 32;
pub const ICP_QAT_FW_COMN_ONE_BYTE_SHIFT: u32 = 8;
pub const ICP_QAT_FW_COMN_SINGLE_BYTE_MASK: u32 = 0xFF;
pub const ICP_QAT_FW_NUM_LONGWORDS_1: usize = 1;
pub const ICP_QAT_FW_NUM_LONGWORDS_2: usize = 2;
pub const ICP_QAT_FW_NUM_LONGWORDS_3: usize = 3;
pub const ICP_QAT_FW_NUM_LONGWORDS_4: usize = 4;
pub const ICP_QAT_FW_NUM_LONGWORDS_5: usize = 5;
pub const ICP_QAT_FW_NUM_LONGWORDS_6: usize = 6;
pub const ICP_QAT_FW_NUM_LONGWORDS_7: usize = 7;
pub const ICP_QAT_FW_NUM_LONGWORDS_10: usize = 10;
pub const ICP_QAT_FW_NUM_LONGWORDS_13: usize = 13;
pub const ICP_QAT_FW_NULL_REQ_SERV_ID: u32 = 1;

pub const ICP_QAT_FW_COMN_RESP_SERV_NULL: i32 = 0;
pub const ICP_QAT_FW_COMN_RESP_SERV_CPM_FW: i32 = 1;
pub const ICP_QAT_FW_COMN_RESP_SERV_DELIMITER: i32 = 2;
pub const ICP_QAT_FW_COMN_REQ_NULL: i32 = 0;
pub const ICP_QAT_FW_COMN_REQ_CPM_FW_PKE: i32 = 3;
pub const ICP_QAT_FW_COMN_REQ_CPM_FW_LA: i32 = 4;
pub const ICP_QAT_FW_COMN_REQ_CPM_FW_DMA: i32 = 7;
pub const ICP_QAT_FW_COMN_REQ_CPM_FW_COMP: i32 = 9;
pub const ICP_QAT_FW_COMN_REQ_DELIMITER: i32 = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub union icp_qat_fw_comn_req_hdr_cd_pars_u {
    pub s: icp_qat_fw_comn_req_hdr_cd_pars_s,
    pub s1: icp_qat_fw_comn_req_hdr_cd_pars_s1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comn_req_hdr_cd_pars_s {
    pub content_desc_addr: u64,
    pub content_desc_resrvd1: u16,
    pub content_desc_params_sz: u8,
    pub content_desc_hdr_resrvd2: u8,
    pub content_desc_resrvd3: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_comn_req_hdr_cd_pars_s1 { pub serv_specif_fields: [u32; 4] }
#[repr(C)]
pub struct icp_qat_fw_comn_req_hdr_cd_pars { pub u: icp_qat_fw_comn_req_hdr_cd_pars_u }

#[repr(C)]
pub struct icp_qat_fw_comn_req_mid { pub opaque_data: u64, pub src_data_addr: u64, pub dest_data_addr: u64, pub src_length: u32, pub dst_length: u32 }
#[repr(C)]
pub struct icp_qat_fw_comn_req_cd_ctrl { pub content_desc_ctrl_lw: [u32; ICP_QAT_FW_NUM_LONGWORDS_5] }
#[repr(C)]
pub struct icp_qat_fw_comn_req_hdr { pub resrvd1: u8, pub service_cmd_id: u8, pub service_type: u8, pub hdr_flags: u8, pub serv_specif_flags: u16, pub comn_req_flags: u16 }
#[repr(C)]
pub struct icp_qat_fw_comn_req_rqpars { pub serv_specif_rqpars_lw: [u32; ICP_QAT_FW_NUM_LONGWORDS_13] }
#[repr(C)]
pub struct icp_qat_fw_comn_req { pub comn_hdr: icp_qat_fw_comn_req_hdr, pub cd_pars: icp_qat_fw_comn_req_hdr_cd_pars, pub comn_mid: icp_qat_fw_comn_req_mid, pub serv_specif_rqpars: icp_qat_fw_comn_req_rqpars, pub cd_ctrl: icp_qat_fw_comn_req_cd_ctrl }
#[repr(C)]
pub struct icp_qat_fw_comn_error { pub xlat_err_code: u8, pub cmp_err_code: u8 }
#[repr(C)]
pub struct icp_qat_fw_comn_resp_hdr { pub resrvd1: u8, pub service_id: u8, pub response_type: u8, pub hdr_flags: u8, pub comn_error: icp_qat_fw_comn_error, pub comn_status: u8, pub cmd_id: u8 }
#[repr(C)]
pub struct icp_qat_fw_comn_resp { pub comn_hdr: icp_qat_fw_comn_resp_hdr, pub opaque_data: u64, pub resrvd: [u32; ICP_QAT_FW_NUM_LONGWORDS_4] }

pub const ICP_QAT_FW_COMN_REQ_FLAG_SET: u32 = 1; pub const ICP_QAT_FW_COMN_REQ_FLAG_CLR: u32 = 0;
pub const ICP_QAT_FW_COMN_VALID_FLAG_BITPOS: u32 = 7; pub const ICP_QAT_FW_COMN_VALID_FLAG_MASK: u32 = 0x1;
pub const ICP_QAT_FW_COMN_HDR_RESRVD_FLD_MASK: u32 = 0x7F; pub const ICP_QAT_FW_COMN_CNV_FLAG_BITPOS: u32 = 6; pub const ICP_QAT_FW_COMN_CNV_FLAG_MASK: u32 = 0x1;
pub const ICP_QAT_FW_COMN_CNVNR_FLAG_BITPOS: u32 = 5; pub const ICP_QAT_FW_COMN_CNVNR_FLAG_MASK: u32 = 0x1;
pub const ICP_QAT_FW_COMN_ST_BLK_FLAG_BITPOS: u32 = 4; pub const ICP_QAT_FW_COMN_ST_BLK_FLAG_MASK: u32 = 0x1;

macro_rules! field_get { ($n:ident, $p:expr, $m:expr) => { #[allow(non_snake_case)] pub fn $n(v: u32) -> u32 { (v >> $p) & $m } }; }
field_get!(ICP_QAT_FW_COMN_HDR_CNVNR_FLAG_GET, ICP_QAT_FW_COMN_CNVNR_FLAG_BITPOS, ICP_QAT_FW_COMN_CNVNR_FLAG_MASK);
field_get!(ICP_QAT_FW_COMN_HDR_CNV_FLAG_GET, ICP_QAT_FW_COMN_CNV_FLAG_BITPOS, ICP_QAT_FW_COMN_CNV_FLAG_MASK);
field_get!(ICP_QAT_FW_COMN_HDR_ST_BLK_FLAG_GET, ICP_QAT_FW_COMN_ST_BLK_FLAG_BITPOS, ICP_QAT_FW_COMN_ST_BLK_FLAG_MASK);
field_get!(ICP_QAT_FW_COMN_VALID_FLAG_GET, ICP_QAT_FW_COMN_VALID_FLAG_BITPOS, ICP_QAT_FW_COMN_VALID_FLAG_MASK);
pub fn ICP_QAT_FW_COMN_HDR_RESRVD_FLD_GET(v: u32) -> u32 { v & ICP_QAT_FW_COMN_HDR_RESRVD_FLD_MASK }
pub fn ICP_QAT_FW_COMN_HDR_FLAGS_BUILD(valid: u32) -> u32 { (valid & ICP_QAT_FW_COMN_VALID_FLAG_MASK) << ICP_QAT_FW_COMN_VALID_FLAG_BITPOS }

pub const QAT_COMN_PTR_TYPE_BITPOS: u32 = 0; pub const QAT_COMN_PTR_TYPE_MASK: u32 = 0x1;
pub const QAT_COMN_CD_FLD_TYPE_BITPOS: u32 = 1; pub const QAT_COMN_CD_FLD_TYPE_MASK: u32 = 0x1;
pub const QAT_COMN_PTR_TYPE_FLAT: u32 = 0; pub const QAT_COMN_PTR_TYPE_SGL: u32 = 1;
pub const QAT_COMN_CD_FLD_TYPE_64BIT_ADR: u32 = 0; pub const QAT_COMN_CD_FLD_TYPE_16BYTE_DATA: u32 = 1;
pub fn ICP_QAT_FW_COMN_FLAGS_BUILD(cdt: u32, ptr: u32) -> u32 { ((cdt & 1) << 1) | (ptr & 1) }
pub fn ICP_QAT_FW_COMN_PTR_TYPE_GET(v: u32) -> u32 { v & 1 }
pub fn ICP_QAT_FW_COMN_CD_FLD_TYPE_GET(v: u32) -> u32 { (v >> 1) & 1 }

pub const ICP_QAT_FW_COMN_NEXT_ID_BITPOS: u32 = 4; pub const ICP_QAT_FW_COMN_NEXT_ID_MASK: u32 = 0xF0;
pub const ICP_QAT_FW_COMN_CURR_ID_BITPOS: u32 = 0; pub const ICP_QAT_FW_COMN_CURR_ID_MASK: u32 = 0x0F;
pub fn ICP_QAT_FW_COMN_NEXT_ID_GET(v: u8) -> u8 { (v & 0xF0) >> 4 }
pub fn ICP_QAT_FW_COMN_CURR_ID_GET(v: u8) -> u8 { v & 0x0F }

pub const QAT_COMN_RESP_CRYPTO_STATUS_BITPOS: u32 = 7; pub const QAT_COMN_RESP_CRYPTO_STATUS_MASK: u32 = 1;
pub const QAT_COMN_RESP_PKE_STATUS_BITPOS: u32 = 6; pub const QAT_COMN_RESP_PKE_STATUS_MASK: u32 = 1;
pub const QAT_COMN_RESP_CMP_STATUS_BITPOS: u32 = 5; pub const QAT_COMN_RESP_CMP_STATUS_MASK: u32 = 1;
pub const QAT_COMN_RESP_XLAT_STATUS_BITPOS: u32 = 4; pub const QAT_COMN_RESP_XLAT_STATUS_MASK: u32 = 1;
pub const QAT_COMN_RESP_CMP_END_OF_LAST_BLK_BITPOS: u32 = 3; pub const QAT_COMN_RESP_CMP_END_OF_LAST_BLK_MASK: u32 = 1;
pub fn ICP_QAT_FW_COMN_RESP_STATUS_BUILD(crypto: u32, comp: u32, xlat: u32, eolb: u32) -> u32 { ((crypto & 1) << 7) | ((comp & 1) << 5) | ((xlat & 1) << 4) | ((eolb & 1) << 3) }
pub fn ICP_QAT_FW_COMN_RESP_CRYPTO_STAT_GET(v: u32) -> u32 { (v >> 7) & 1 }
pub fn ICP_QAT_FW_COMN_RESP_CMP_STAT_GET(v: u32) -> u32 { (v >> 5) & 1 }
pub fn ICP_QAT_FW_COMN_RESP_XLAT_STAT_GET(v: u32) -> u32 { (v >> 4) & 1 }
pub fn ICP_QAT_FW_COMN_RESP_CMP_END_OF_LAST_BLK_FLAG_GET(v: u32) -> u32 { (v >> 3) & 1 }

#[macro_export]
macro_rules! ICP_QAT_FW_COMN_OV_SRV_TYPE_GET { ($h:expr) => { $h.service_type }; }
#[macro_export]
macro_rules! ICP_QAT_FW_COMN_OV_SRV_TYPE_SET { ($h:expr, $v:expr) => { $h.service_type = $v }; }
#[macro_export]
macro_rules! ICP_QAT_FW_COMN_OV_SRV_CMD_ID_GET { ($h:expr) => { $h.service_cmd_id }; }
#[macro_export]
macro_rules! ICP_QAT_FW_COMN_OV_SRV_CMD_ID_SET { ($h:expr, $v:expr) => { $h.service_cmd_id = $v }; }
#[macro_export]
macro_rules! ICP_QAT_FW_COMN_HDR_CNVNR_FLAG_SET { ($h:expr, $v:expr) => { $h.hdr_flags = (($h.hdr_flags & !(1 << 5)) | (($v & 1) << 5)) }; }
#[macro_export]
macro_rules! ICP_QAT_FW_COMN_HDR_CNV_FLAG_SET { ($h:expr, $v:expr) => { $h.hdr_flags = (($h.hdr_flags & !(1 << 6)) | (($v & 1) << 6)) }; }
#[macro_export]
macro_rules! ICP_QAT_FW_COMN_VALID_FLAG_SET { ($h:expr, $v:expr) => { $h.hdr_flags = (($h.hdr_flags & !(1 << 7)) | (($v & 1) << 7)) }; }
#[macro_export]
macro_rules! ICP_QAT_FW_COMN_HDR_VALID_FLAG_SET { ($h:expr, $v:expr) => { $h.hdr_flags = (($h.hdr_flags & !(1 << 7)) | (($v & 1) << 7)) }; }
#[macro_export]
macro_rules! ICP_QAT_FW_COMN_PTR_TYPE_SET { ($f:expr, $v:expr) => { $f = (($f & !1) | ($v & 1)) }; }
#[macro_export]
macro_rules! ICP_QAT_FW_COMN_CD_FLD_TYPE_SET { ($f:expr, $v:expr) => { $f = (($f & !(1 << 1)) | (($v & 1) << 1)) }; }
#[macro_export]
macro_rules! ICP_QAT_FW_COMN_NEXT_ID_SET { ($p:expr, $v:expr) => { $p.next_curr_id = (($p.next_curr_id & 0x0F) | (($v << 4) & 0xF0)) }; }
#[macro_export]
macro_rules! ICP_QAT_FW_COMN_CURR_ID_SET { ($p:expr, $v:expr) => { $p.next_curr_id = (($p.next_curr_id & 0xF0) | ($v & 0x0F)) }; }

pub const ICP_QAT_FW_COMN_STATUS_FLAG_OK: i32 = 0; pub const ICP_QAT_FW_COMN_STATUS_FLAG_ERROR: i32 = 1;
pub const ICP_QAT_FW_COMN_STATUS_CMP_END_OF_LAST_BLK_FLAG_CLR: i32 = 0; pub const ICP_QAT_FW_COMN_STATUS_CMP_END_OF_LAST_BLK_FLAG_SET: i32 = 1;
pub const ERR_CODE_NO_ERROR: i32 = 0; pub const ERR_CODE_INVALID_BLOCK_TYPE: i32 = -1; pub const ERR_CODE_NO_MATCH_ONES_COMP: i32 = -2; pub const ERR_CODE_TOO_MANY_LEN_OR_DIS: i32 = -3; pub const ERR_CODE_INCOMPLETE_LEN: i32 = -4; pub const ERR_CODE_RPT_LEN_NO_FIRST_LEN: i32 = -5; pub const ERR_CODE_RPT_GT_SPEC_LEN: i32 = -6; pub const ERR_CODE_INV_LIT_LEN_CODE_LEN: i32 = -7; pub const ERR_CODE_INV_DIS_CODE_LEN: i32 = -8; pub const ERR_CODE_INV_LIT_LEN_DIS_IN_BLK: i32 = -9; pub const ERR_CODE_DIS_TOO_FAR_BACK: i32 = -10; pub const ERR_CODE_OVERFLOW_ERROR: i32 = -11; pub const ERR_CODE_SOFT_ERROR: i32 = -12; pub const ERR_CODE_FATAL_ERROR: i32 = -13; pub const ERR_CODE_SSM_ERROR: i32 = -14; pub const ERR_CODE_ENDPOINT_ERROR: i32 = -15;
pub const ICP_QAT_FW_SLICE_NULL: i32 = 0; pub const ICP_QAT_FW_SLICE_CIPHER: i32 = 1; pub const ICP_QAT_FW_SLICE_AUTH: i32 = 2; pub const ICP_QAT_FW_SLICE_DRAM_RD: i32 = 3; pub const ICP_QAT_FW_SLICE_DRAM_WR: i32 = 4; pub const ICP_QAT_FW_SLICE_COMP: i32 = 5; pub const ICP_QAT_FW_SLICE_XLAT: i32 = 6; pub const ICP_QAT_FW_SLICE_DELIMITER: i32 = 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
