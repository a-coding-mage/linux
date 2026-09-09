/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */
// Translated from icp_qat_fw_la.h.  Dependencies are supplied by the including crate.

#[repr(u32)]
pub enum icp_qat_fw_la_cmd_id {
    ICP_QAT_FW_LA_CMD_CIPHER = 0,
    ICP_QAT_FW_LA_CMD_AUTH = 1,
    ICP_QAT_FW_LA_CMD_CIPHER_HASH = 2,
    ICP_QAT_FW_LA_CMD_HASH_CIPHER = 3,
    ICP_QAT_FW_LA_CMD_TRNG_GET_RANDOM = 4,
    ICP_QAT_FW_LA_CMD_TRNG_TEST = 5,
    ICP_QAT_FW_LA_CMD_SSL3_KEY_DERIVE = 6,
    ICP_QAT_FW_LA_CMD_TLS_V1_1_KEY_DERIVE = 7,
    ICP_QAT_FW_LA_CMD_TLS_V1_2_KEY_DERIVE = 8,
    ICP_QAT_FW_LA_CMD_MGF1 = 9,
    ICP_QAT_FW_LA_CMD_AUTH_PRE_COMP = 10,
    ICP_QAT_FW_LA_CMD_CIPHER_PRE_COMP = 11,
    ICP_QAT_FW_LA_CMD_DELIMITER = 12,
}

pub const ICP_QAT_FW_LA_ICV_VER_STATUS_PASS: u32 = ICP_QAT_FW_COMN_STATUS_FLAG_OK;
pub const ICP_QAT_FW_LA_ICV_VER_STATUS_FAIL: u32 = ICP_QAT_FW_COMN_STATUS_FLAG_ERROR;
pub const ICP_QAT_FW_LA_TRNG_STATUS_PASS: u32 = ICP_QAT_FW_COMN_STATUS_FLAG_OK;
pub const ICP_QAT_FW_LA_TRNG_STATUS_FAIL: u32 = ICP_QAT_FW_COMN_STATUS_FLAG_ERROR;

#[repr(C)]
pub struct icp_qat_fw_la_bulk_req {
    pub comn_hdr: icp_qat_fw_comn_req_hdr,
    pub cd_pars: icp_qat_fw_comn_req_hdr_cd_pars,
    pub comn_mid: icp_qat_fw_comn_req_mid,
    pub serv_specif_rqpars: icp_qat_fw_comn_req_rqpars,
    pub cd_ctrl: icp_qat_fw_comn_req_cd_ctrl,
}

pub const ICP_QAT_FW_LA_USE_UCS_SLICE_TYPE: u32 = 1;
pub const QAT_LA_SLICE_TYPE_BITPOS: u32 = 14;
pub const QAT_LA_SLICE_TYPE_MASK: u32 = 0x3;
pub const ICP_QAT_FW_LA_GCM_IV_LEN_12_OCTETS: u32 = 1;
pub const ICP_QAT_FW_LA_GCM_IV_LEN_NOT_12_OCTETS: u32 = 0;
pub const QAT_FW_LA_ZUC_3G_PROTO_FLAG_BITPOS: u32 = 12;
pub const ICP_QAT_FW_LA_ZUC_3G_PROTO: u32 = 1;
pub const QAT_FW_LA_ZUC_3G_PROTO_FLAG_MASK: u32 = 0x1;
pub const QAT_LA_GCM_IV_LEN_FLAG_BITPOS: u32 = 11;
pub const QAT_LA_GCM_IV_LEN_FLAG_MASK: u32 = 0x1;
pub const ICP_QAT_FW_LA_DIGEST_IN_BUFFER: u32 = 1;
pub const ICP_QAT_FW_LA_NO_DIGEST_IN_BUFFER: u32 = 0;
pub const QAT_LA_DIGEST_IN_BUFFER_BITPOS: u32 = 10;
pub const QAT_LA_DIGEST_IN_BUFFER_MASK: u32 = 0x1;
pub const ICP_QAT_FW_LA_SNOW_3G_PROTO: u32 = 4;
pub const ICP_QAT_FW_LA_GCM_PROTO: u32 = 2;
pub const ICP_QAT_FW_LA_CCM_PROTO: u32 = 1;
pub const ICP_QAT_FW_LA_NO_PROTO: u32 = 0;
pub const QAT_LA_PROTO_BITPOS: u32 = 7;
pub const QAT_LA_PROTO_MASK: u32 = 0x7;
pub const ICP_QAT_FW_LA_CMP_AUTH_RES: u32 = 1;
pub const ICP_QAT_FW_LA_NO_CMP_AUTH_RES: u32 = 0;
pub const QAT_LA_CMP_AUTH_RES_BITPOS: u32 = 6;
pub const QAT_LA_CMP_AUTH_RES_MASK: u32 = 0x1;
pub const ICP_QAT_FW_LA_RET_AUTH_RES: u32 = 1;
pub const ICP_QAT_FW_LA_NO_RET_AUTH_RES: u32 = 0;
pub const QAT_LA_RET_AUTH_RES_BITPOS: u32 = 5;
pub const QAT_LA_RET_AUTH_RES_MASK: u32 = 0x1;
pub const ICP_QAT_FW_LA_UPDATE_STATE: u32 = 1;
pub const ICP_QAT_FW_LA_NO_UPDATE_STATE: u32 = 0;
pub const QAT_LA_UPDATE_STATE_BITPOS: u32 = 4;
pub const QAT_LA_UPDATE_STATE_MASK: u32 = 0x1;
pub const ICP_QAT_FW_CIPH_AUTH_CFG_OFFSET_IN_CD_SETUP: u32 = 0;
pub const ICP_QAT_FW_CIPH_AUTH_CFG_OFFSET_IN_SHRAM_CP: u32 = 1;
pub const QAT_LA_CIPH_AUTH_CFG_OFFSET_BITPOS: u32 = 3;
pub const QAT_LA_CIPH_AUTH_CFG_OFFSET_MASK: u32 = 0x1;
pub const ICP_QAT_FW_CIPH_IV_64BIT_PTR: u32 = 0;
pub const ICP_QAT_FW_CIPH_IV_16BYTE_DATA: u32 = 1;
pub const QAT_LA_CIPH_IV_FLD_BITPOS: u32 = 2;
pub const QAT_LA_CIPH_IV_FLD_MASK: u32 = 0x1;
pub const ICP_QAT_FW_LA_PARTIAL_NONE: u32 = 0;
pub const ICP_QAT_FW_LA_PARTIAL_START: u32 = 1;
pub const ICP_QAT_FW_LA_PARTIAL_MID: u32 = 3;
pub const ICP_QAT_FW_LA_PARTIAL_END: u32 = 2;
pub const QAT_LA_PARTIAL_BITPOS: u32 = 0;
pub const QAT_LA_PARTIAL_MASK: u32 = 0x3;

#[inline]
pub const fn ICP_QAT_FW_LA_FLAGS_BUILD(zuc_proto: u32, gcm_iv_len: u32, auth_rslt: u32, proto: u32, cmp_auth: u32, ret_auth: u32, update_state: u32, ciph_iv: u32, ciphcfg: u32, partial: u32) -> u32 {
    ((zuc_proto & QAT_FW_LA_ZUC_3G_PROTO_FLAG_MASK) << QAT_FW_LA_ZUC_3G_PROTO_FLAG_BITPOS)
        | ((gcm_iv_len & QAT_LA_GCM_IV_LEN_FLAG_MASK) << QAT_LA_GCM_IV_LEN_FLAG_BITPOS)
        | ((auth_rslt & QAT_LA_DIGEST_IN_BUFFER_MASK) << QAT_LA_DIGEST_IN_BUFFER_BITPOS)
        | ((proto & QAT_LA_PROTO_MASK) << QAT_LA_PROTO_BITPOS)
        | ((cmp_auth & QAT_LA_CMP_AUTH_RES_MASK) << QAT_LA_CMP_AUTH_RES_BITPOS)
        | ((ret_auth & QAT_LA_RET_AUTH_RES_MASK) << QAT_LA_RET_AUTH_RES_BITPOS)
        | ((update_state & QAT_LA_UPDATE_STATE_MASK) << QAT_LA_UPDATE_STATE_BITPOS)
        | ((ciph_iv & QAT_LA_CIPH_IV_FLD_MASK) << QAT_LA_CIPH_IV_FLD_BITPOS)
        | ((ciphcfg & QAT_LA_CIPH_AUTH_CFG_OFFSET_MASK) << QAT_LA_CIPH_AUTH_CFG_OFFSET_BITPOS)
        | ((partial & QAT_LA_PARTIAL_MASK) << QAT_LA_PARTIAL_BITPOS)
}

#[inline] pub const fn QAT_FIELD_GET(flags: u32, bitpos: u32, mask: u32) -> u32 { (flags >> bitpos) & mask }
#[inline] pub const fn QAT_FIELD_SET(flags: u32, val: u32, bitpos: u32, mask: u32) -> u32 { (flags & !(mask << bitpos)) | ((val & mask) << bitpos) }

macro_rules! field_get { ($name:ident, $pos:ident, $mask:ident) => { #[inline] pub const fn $name(flags: u32) -> u32 { QAT_FIELD_GET(flags, $pos, $mask) } }; }
macro_rules! field_set { ($name:ident, $pos:ident, $mask:ident) => { #[inline] pub const fn $name(flags: u32, val: u32) -> u32 { QAT_FIELD_SET(flags, val, $pos, $mask) } }; }
field_get!(ICP_QAT_FW_LA_CIPH_IV_FLD_FLAG_GET, QAT_LA_CIPH_IV_FLD_BITPOS, QAT_LA_CIPH_IV_FLD_MASK);
field_get!(ICP_QAT_FW_LA_CIPH_AUTH_CFG_OFFSET_FLAG_GET, QAT_LA_CIPH_AUTH_CFG_OFFSET_BITPOS, QAT_LA_CIPH_AUTH_CFG_OFFSET_MASK);
field_get!(ICP_QAT_FW_LA_ZUC_3G_PROTO_FLAG_GET, QAT_FW_LA_ZUC_3G_PROTO_FLAG_BITPOS, QAT_FW_LA_ZUC_3G_PROTO_FLAG_MASK);
field_get!(ICP_QAT_FW_LA_GCM_IV_LEN_FLAG_GET, QAT_LA_GCM_IV_LEN_FLAG_BITPOS, QAT_LA_GCM_IV_LEN_FLAG_MASK);
field_get!(ICP_QAT_FW_LA_PROTO_GET, QAT_LA_PROTO_BITPOS, QAT_LA_PROTO_MASK);
field_get!(ICP_QAT_FW_LA_CMP_AUTH_GET, QAT_LA_CMP_AUTH_RES_BITPOS, QAT_LA_CMP_AUTH_RES_MASK);
field_get!(ICP_QAT_FW_LA_RET_AUTH_GET, QAT_LA_RET_AUTH_RES_BITPOS, QAT_LA_RET_AUTH_RES_MASK);
field_get!(ICP_QAT_FW_LA_DIGEST_IN_BUFFER_GET, QAT_LA_DIGEST_IN_BUFFER_BITPOS, QAT_LA_DIGEST_IN_BUFFER_MASK);
field_get!(ICP_QAT_FW_LA_UPDATE_STATE_GET, QAT_LA_UPDATE_STATE_BITPOS, QAT_LA_UPDATE_STATE_MASK);
field_get!(ICP_QAT_FW_LA_PARTIAL_GET, QAT_LA_PARTIAL_BITPOS, QAT_LA_PARTIAL_MASK);
field_set!(ICP_QAT_FW_LA_CIPH_IV_FLD_FLAG_SET, QAT_LA_CIPH_IV_FLD_BITPOS, QAT_LA_CIPH_IV_FLD_MASK);
field_set!(ICP_QAT_FW_LA_CIPH_AUTH_CFG_OFFSET_FLAG_SET, QAT_LA_CIPH_AUTH_CFG_OFFSET_BITPOS, QAT_LA_CIPH_AUTH_CFG_OFFSET_MASK);
field_set!(ICP_QAT_FW_LA_ZUC_3G_PROTO_FLAG_SET, QAT_FW_LA_ZUC_3G_PROTO_FLAG_BITPOS, QAT_FW_LA_ZUC_3G_PROTO_FLAG_MASK);
field_set!(ICP_QAT_FW_LA_GCM_IV_LEN_FLAG_SET, QAT_LA_GCM_IV_LEN_FLAG_BITPOS, QAT_LA_GCM_IV_LEN_FLAG_MASK);
field_set!(ICP_QAT_FW_LA_PROTO_SET, QAT_LA_PROTO_BITPOS, QAT_LA_PROTO_MASK);
field_set!(ICP_QAT_FW_LA_CMP_AUTH_SET, QAT_LA_CMP_AUTH_RES_BITPOS, QAT_LA_CMP_AUTH_RES_MASK);
field_set!(ICP_QAT_FW_LA_RET_AUTH_SET, QAT_LA_RET_AUTH_RES_BITPOS, QAT_LA_RET_AUTH_RES_MASK);
field_set!(ICP_QAT_FW_LA_DIGEST_IN_BUFFER_SET, QAT_LA_DIGEST_IN_BUFFER_BITPOS, QAT_LA_DIGEST_IN_BUFFER_MASK);
field_set!(ICP_QAT_FW_LA_UPDATE_STATE_SET, QAT_LA_UPDATE_STATE_BITPOS, QAT_LA_UPDATE_STATE_MASK);
field_set!(ICP_QAT_FW_LA_PARTIAL_SET, QAT_LA_PARTIAL_BITPOS, QAT_LA_PARTIAL_MASK);
field_set!(ICP_QAT_FW_LA_SLICE_TYPE_SET, QAT_LA_SLICE_TYPE_BITPOS, QAT_LA_SLICE_TYPE_MASK);

#[repr(C)] pub struct icp_qat_fw_cipher_req_hdr_cd_pars { pub u: icp_qat_fw_cipher_req_hdr_cd_pars_u }
#[repr(C)] pub union icp_qat_fw_cipher_req_hdr_cd_pars_u { pub s: icp_qat_fw_cipher_req_hdr_cd_pars_s, pub s1: icp_qat_fw_cipher_req_hdr_cd_pars_s1 }
#[repr(C)] pub struct icp_qat_fw_cipher_req_hdr_cd_pars_s { pub content_desc_addr: u64, pub content_desc_resrvd1: u16, pub content_desc_params_sz: u8, pub content_desc_hdr_resrvd2: u8, pub content_desc_resrvd3: u32 }
#[repr(C)] pub struct icp_qat_fw_cipher_req_hdr_cd_pars_s1 { pub cipher_key_array: [u32; ICP_QAT_FW_NUM_LONGWORDS_4 as usize] }
#[repr(C)] pub struct icp_qat_fw_cipher_auth_req_hdr_cd_pars { pub u: icp_qat_fw_cipher_auth_req_hdr_cd_pars_u }
#[repr(C)] pub union icp_qat_fw_cipher_auth_req_hdr_cd_pars_u { pub s: icp_qat_fw_cipher_req_hdr_cd_pars_s, pub sl: icp_qat_fw_cipher_auth_req_hdr_cd_pars_sl }
#[repr(C)] pub struct icp_qat_fw_cipher_auth_req_hdr_cd_pars_sl { pub cipher_key_array: [u32; ICP_QAT_FW_NUM_LONGWORDS_4 as usize] }
#[repr(C)] pub struct icp_qat_fw_cipher_cd_ctrl_hdr { pub cipher_state_sz:u8, pub cipher_key_sz:u8, pub cipher_cfg_offset:u8, pub next_curr_id:u8, pub cipher_padding_sz:u8, pub resrvd1:u8, pub resrvd2:u16, pub resrvd3:[u32; ICP_QAT_FW_NUM_LONGWORDS_3 as usize] }
#[repr(C)] pub struct icp_qat_fw_auth_cd_ctrl_hdr { pub resrvd1:u32, pub resrvd2:u8, pub hash_flags:u8, pub hash_cfg_offset:u8, pub next_curr_id:u8, pub resrvd3:u8, pub outer_prefix_sz:u8, pub final_sz:u8, pub inner_res_sz:u8, pub resrvd4:u8, pub inner_state1_sz:u8, pub inner_state2_offset:u8, pub inner_state2_sz:u8, pub outer_config_offset:u8, pub outer_state1_sz:u8, pub outer_res_sz:u8, pub outer_prefix_offset:u8 }
#[repr(C)] pub struct icp_qat_fw_cipher_auth_cd_ctrl_hdr { pub cipher_state_sz:u8, pub cipher_key_sz:u8, pub cipher_cfg_offset:u8, pub next_curr_id_cipher:u8, pub cipher_padding_sz:u8, pub hash_flags:u8, pub hash_cfg_offset:u8, pub next_curr_id_auth:u8, pub resrvd1:u8, pub outer_prefix_sz:u8, pub final_sz:u8, pub inner_res_sz:u8, pub resrvd2:u8, pub inner_state1_sz:u8, pub inner_state2_offset:u8, pub inner_state2_sz:u8, pub outer_config_offset:u8, pub outer_state1_sz:u8, pub outer_res_sz:u8, pub outer_prefix_offset:u8 }

pub const ICP_QAT_FW_AUTH_HDR_FLAG_DO_NESTED:u32=1; pub const ICP_QAT_FW_AUTH_HDR_FLAG_NO_NESTED:u32=0; pub const ICP_QAT_FW_CCM_GCM_AAD_SZ_MAX:u32=240;
pub const ICP_QAT_FW_HASH_REQUEST_PARAMETERS_OFFSET: usize = core::mem::size_of::<icp_qat_fw_la_cipher_req_params>();
pub const ICP_QAT_FW_CIPHER_REQUEST_PARAMETERS_OFFSET: usize = 0;

#[repr(C)] pub struct icp_qat_fw_la_cipher_req_params { pub cipher_offset:u32, pub cipher_length:u32, pub u: icp_qat_fw_la_cipher_req_params_u }
#[repr(C)] pub union icp_qat_fw_la_cipher_req_params_u { pub cipher_IV_array:[u32; ICP_QAT_FW_NUM_LONGWORDS_4 as usize], pub s: icp_qat_fw_la_cipher_req_params_s }
#[repr(C)] pub struct icp_qat_fw_la_cipher_req_params_s { pub cipher_IV_ptr:u64, pub resrvd1:u64 }
#[repr(C, packed)] pub struct icp_qat_fw_la_auth_req_params { pub auth_off:u32, pub auth_len:u32, pub u1:icp_qat_fw_la_auth_req_params_u1, pub auth_res_addr:u64, pub u2:icp_qat_fw_la_auth_req_params_u2, pub resrvd1:u8, pub hash_state_sz:u8, pub auth_res_sz:u8 }
#[repr(C)] pub union icp_qat_fw_la_auth_req_params_u1 { pub auth_partial_st_prefix:u64, pub aad_adr:u64 }
#[repr(C)] pub union icp_qat_fw_la_auth_req_params_u2 { pub inner_prefix_sz:u8, pub aad_sz:u8 }
#[repr(C)] pub struct icp_qat_fw_la_auth_req_params_resrvd_flds { pub resrvd:[u32; ICP_QAT_FW_NUM_LONGWORDS_6 as usize], pub u2:icp_qat_fw_la_auth_req_params_u2, pub resrvd1:u8, pub resrvd2:u16 }
#[repr(C)] pub struct icp_qat_fw_la_resp { pub comn_resp:icp_qat_fw_comn_resp_hdr, pub opaque_data:u64, pub resrvd:[u32; ICP_QAT_FW_NUM_LONGWORDS_4 as usize] }

#[inline] pub const fn ICP_QAT_FW_CIPHER_NEXT_ID_GET(v:u8)->u8 { ((v & ICP_QAT_FW_COMN_NEXT_ID_MASK as u8) >> ICP_QAT_FW_COMN_NEXT_ID_BITPOS) }
#[inline] pub const fn ICP_QAT_FW_CIPHER_NEXT_ID_SET(v:u8,val:u8)->u8 { (v & ICP_QAT_FW_COMN_CURR_ID_MASK as u8) | ((val << ICP_QAT_FW_COMN_NEXT_ID_BITPOS) & ICP_QAT_FW_COMN_NEXT_ID_MASK as u8) }
#[inline] pub const fn ICP_QAT_FW_CIPHER_CURR_ID_GET(v:u8)->u8 { v & ICP_QAT_FW_COMN_CURR_ID_MASK as u8 }
#[inline] pub const fn ICP_QAT_FW_CIPHER_CURR_ID_SET(v:u8,val:u8)->u8 { (v & ICP_QAT_FW_COMN_NEXT_ID_MASK as u8) | (val & ICP_QAT_FW_COMN_CURR_ID_MASK as u8) }
#[inline] pub const fn ICP_QAT_FW_AUTH_NEXT_ID_GET(v:u8)->u8 { (v & ICP_QAT_FW_COMN_NEXT_ID_MASK as u8) >> ICP_QAT_FW_COMN_NEXT_ID_BITPOS }
#[inline] pub const fn ICP_QAT_FW_AUTH_NEXT_ID_SET(v:u8,val:u8)->u8 { (v & ICP_QAT_FW_COMN_CURR_ID_MASK as u8) | ((val << ICP_QAT_FW_COMN_NEXT_ID_BITPOS) & ICP_QAT_FW_COMN_NEXT_ID_MASK as u8) }
#[inline] pub const fn ICP_QAT_FW_AUTH_CURR_ID_GET(v:u8)->u8 { v & ICP_QAT_FW_COMN_CURR_ID_MASK as u8 }
#[inline] pub const fn ICP_QAT_FW_AUTH_CURR_ID_SET(v:u8,val:u8)->u8 { (v & ICP_QAT_FW_COMN_NEXT_ID_MASK as u8) | (val & ICP_QAT_FW_COMN_CURR_ID_MASK as u8) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
