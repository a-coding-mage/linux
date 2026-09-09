/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from llc_pdu.h. */

pub const LLC_PDU_LEN_I: usize = 4;
pub const LLC_PDU_LEN_S: usize = 4;
pub const LLC_PDU_LEN_U: usize = 3;
pub const LLC_PDU_LEN_U_XID: usize = LLC_PDU_LEN_U + core::mem::size_of::<llc_xid_info>();
pub const LLC_GLOBAL_SAP: u8 = 0xFF;
pub const LLC_NULL_SAP: u8 = 0x00;
pub const LLC_MGMT_INDIV: u8 = 0x02;
pub const LLC_MGMT_GRP: u8 = 0x03;
pub const LLC_RDE_SAP: u8 = 0xA6;
pub const LLC_ISO_RESERVED_SAP: u8 = 0x02;
pub const LLC_SAP_GROUP_DSAP: u8 = 0x01;
pub const LLC_SAP_RESP_SSAP: u8 = 0x01;
pub const LLC_PDU_GROUP_DSAP_MASK: u8 = 0x01;
pub const LLC_PDU_CMD_RSP_MASK: u8 = 0x01;
pub const LLC_PDU_CMD: u8 = 0;
pub const LLC_PDU_RSP: u8 = 1;
pub const LLC_PDU_TYPE_I_MASK: u8 = 0x01;
pub const LLC_PDU_TYPE_S_MASK: u8 = 0x03;
pub const LLC_PDU_TYPE_U_MASK: u8 = 0x03;
pub const LLC_PDU_TYPE_MASK: u8 = 0x03;
pub const LLC_PDU_TYPE_I: u8 = 0;
pub const LLC_PDU_TYPE_S: u8 = 1;
pub const LLC_PDU_TYPE_U: u8 = 3;
pub const LLC_PDU_TYPE_U_XID: u8 = 4;
pub const LLC_U_PF_BIT_MASK: u8 = 0x10;
pub const LLC_U_PDU_CMD_MASK: u8 = 0xEC;
pub const LLC_1_PDU_CMD_UI: u8 = 0x00;
pub const LLC_1_PDU_CMD_XID: u8 = 0xAC;
pub const LLC_1_PDU_CMD_TEST: u8 = 0xE0;
pub const LLC_2_PDU_CMD_SABME: u8 = 0x6C;
pub const LLC_2_PDU_CMD_DISC: u8 = 0x40;
pub const LLC_2_PDU_RSP_UA: u8 = 0x60;
pub const LLC_2_PDU_RSP_DM: u8 = 0x0C;
pub const LLC_2_PDU_RSP_FRMR: u8 = 0x84;
pub const LLC_XID_FMT_ID: u8 = 0x81;
pub const LLC_XID_CLASS_ZEROS_MASK: u8 = 0xE0;
pub const LLC_XID_CLASS_MASK: u8 = 0x1F;
pub const LLC_XID_NULL_CLASS_1: u8 = 0x01;
pub const LLC_XID_NULL_CLASS_2: u8 = 0x03;
pub const LLC_XID_NULL_CLASS_3: u8 = 0x05;
pub const LLC_XID_NULL_CLASS_4: u8 = 0x07;
pub const LLC_XID_NNULL_TYPE_1: u8 = 0x01;
pub const LLC_XID_NNULL_TYPE_2: u8 = 0x02;
pub const LLC_XID_NNULL_TYPE_3: u8 = 0x04;
pub const LLC_XID_NNULL_TYPE_1_2: u8 = 0x03;
pub const LLC_XID_NNULL_TYPE_1_3: u8 = 0x05;
pub const LLC_XID_NNULL_TYPE_2_3: u8 = 0x06;
pub const LLC_XID_NNULL_ALL: u8 = 0x07;
pub const LLC_XID_RW_MASK: u8 = 0xFE;
pub const LLC_XID_MIN_RW: u8 = 0x02;
pub const LLC_2_SEQ_NBR_MODULO: u8 = 128;
pub const LLC_I_PF_BIT_MASK: u8 = 0x01;
pub const LLC_S_PDU_CMD_MASK: u8 = 0x0C;
pub const LLC_2_PDU_CMD_RR: u8 = 0x00;
pub const LLC_2_PDU_RSP_RR: u8 = 0x00;
pub const LLC_2_PDU_CMD_REJ: u8 = 0x08;
pub const LLC_2_PDU_RSP_REJ: u8 = 0x08;
pub const LLC_2_PDU_CMD_RNR: u8 = 0x04;
pub const LLC_2_PDU_RSP_RNR: u8 = 0x04;
pub const LLC_S_PF_BIT_MASK: u8 = 0x01;
pub const FRMR_INFO_LENGTH: usize = 5;

#[repr(C, packed)]
pub struct llc_pdu_sn { pub dsap: u8, pub ssap: u8, pub ctrl_1: u8, pub ctrl_2: u8 }
#[repr(C, packed)]
pub struct llc_pdu_un { pub dsap: u8, pub ssap: u8, pub ctrl_1: u8 }
#[repr(C, packed)]
pub struct llc_xid_info { pub fmt_id: u8, pub r#type: u8, pub rw: u8 }
#[repr(C, packed)]
pub struct llc_frmr_info { pub rej_pdu_ctrl: u16, pub curr_ssv: u8, pub curr_rsv: u8, pub ind_bits: u8 }

#[inline] pub unsafe fn LLC_PDU_IS_GROUP_DSAP(pdu: *const llc_pdu_un) -> u8 { ((!((*pdu).dsap & LLC_PDU_GROUP_DSAP_MASK != 0)) as u8) }
#[inline] pub unsafe fn LLC_PDU_IS_INDIV_DSAP(pdu: *const llc_pdu_un) -> u8 { (((*pdu).dsap & LLC_PDU_GROUP_DSAP_MASK != 0) as u8) }
#[inline] pub unsafe fn LLC_PDU_IS_CMD(pdu: *const llc_pdu_un) -> u8 { ((!((*pdu).ssap & LLC_PDU_RSP != 0)) as u8) }
#[inline] pub unsafe fn LLC_PDU_IS_RSP(pdu: *const llc_pdu_un) -> u8 { (((*pdu).ssap & LLC_PDU_RSP != 0) as u8) }
#[inline] pub unsafe fn LLC_PDU_TYPE_IS_I(pdu: *const llc_pdu_sn) -> u8 { ((!((*pdu).ctrl_1 & LLC_PDU_TYPE_I_MASK != 0)) as u8) }
#[inline] pub unsafe fn LLC_PDU_TYPE_IS_U(pdu: *const llc_pdu_un) -> u8 { (((*pdu).ctrl_1 & LLC_PDU_TYPE_U_MASK) == LLC_PDU_TYPE_U) as u8 }
#[inline] pub unsafe fn LLC_PDU_TYPE_IS_S(pdu: *const llc_pdu_sn) -> u8 { (((*pdu).ctrl_1 & LLC_PDU_TYPE_S_MASK) == LLC_PDU_TYPE_S) as u8 }
#[inline] pub unsafe fn LLC_U_PF_IS_1(pdu: *const llc_pdu_un) -> u8 { (((*pdu).ctrl_1 & LLC_U_PF_BIT_MASK != 0) as u8) }
#[inline] pub unsafe fn LLC_U_PF_IS_0(pdu: *const llc_pdu_un) -> u8 { ((!((*pdu).ctrl_1 & LLC_U_PF_BIT_MASK != 0)) as u8) }
#[inline] pub unsafe fn LLC_U_PDU_CMD(pdu: *const llc_pdu_un) -> u8 { (*pdu).ctrl_1 & LLC_U_PDU_CMD_MASK }
#[inline] pub unsafe fn LLC_U_PDU_RSP(pdu: *const llc_pdu_un) -> u8 { (*pdu).ctrl_1 & LLC_U_PDU_CMD_MASK }
#[inline] pub unsafe fn LLC_I_GET_NS(pdu: *const llc_pdu_sn) -> u8 { ((*pdu).ctrl_1 & 0xFE) >> 1 }
#[inline] pub unsafe fn LLC_I_GET_NR(pdu: *const llc_pdu_sn) -> u8 { ((*pdu).ctrl_2 & 0xFE) >> 1 }
#[inline] pub unsafe fn PDU_SUPV_GET_Nr(pdu: *const llc_pdu_sn) -> u8 { ((*pdu).ctrl_2 & 0xFE) >> 1 }
#[inline] pub fn PDU_GET_NEXT_Vr(sn: u8) -> u8 { (sn.wrapping_add(1)) & !LLC_2_SEQ_NBR_MODULO }

/* External kernel types and helpers are supplied by dependent translations. */
extern "C" {
    pub fn llc_pdu_set_cmd_rsp(skb: *mut sk_buff, r#type: u8);
    pub fn llc_pdu_set_pf_bit(skb: *mut sk_buff, bit_value: u8);
    pub fn llc_pdu_decode_pf_bit(skb: *mut sk_buff, pf_bit: *mut u8);
    pub fn llc_pdu_init_as_disc_cmd(skb: *mut sk_buff, p_bit: u8);
    pub fn llc_pdu_init_as_i_cmd(skb: *mut sk_buff, p_bit: u8, ns: u8, nr: u8);
    pub fn llc_pdu_init_as_rej_cmd(skb: *mut sk_buff, p_bit: u8, nr: u8);
    pub fn llc_pdu_init_as_rnr_cmd(skb: *mut sk_buff, p_bit: u8, nr: u8);
    pub fn llc_pdu_init_as_rr_cmd(skb: *mut sk_buff, p_bit: u8, nr: u8);
    pub fn llc_pdu_init_as_sabme_cmd(skb: *mut sk_buff, p_bit: u8);
    pub fn llc_pdu_init_as_dm_rsp(skb: *mut sk_buff, f_bit: u8);
    pub fn llc_pdu_init_as_frmr_rsp(skb: *mut sk_buff, prev_pdu: *mut llc_pdu_sn, f_bit: u8, vs: u8, vr: u8, vzyxw: u8);
    pub fn llc_pdu_init_as_rr_rsp(skb: *mut sk_buff, f_bit: u8, nr: u8);
    pub fn llc_pdu_init_as_rej_rsp(skb: *mut sk_buff, f_bit: u8, nr: u8);
    pub fn llc_pdu_init_as_rnr_rsp(skb: *mut sk_buff, f_bit: u8, nr: u8);
    pub fn llc_pdu_init_as_ua_rsp(skb: *mut sk_buff, f_bit: u8);
}

extern "C" { type sk_buff; }

extern "C" {
    fn skb_network_header(skb: *mut sk_buff) -> *mut core::ffi::c_void;
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut core::ffi::c_void;
    fn skb_reset_network_header(skb: *mut sk_buff);
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
}

#[inline] pub unsafe fn llc_pdu_sn_hdr(skb: *mut sk_buff) -> *mut llc_pdu_sn { skb_network_header(skb) as *mut llc_pdu_sn }
#[inline] pub unsafe fn llc_pdu_un_hdr(skb: *mut sk_buff) -> *mut llc_pdu_un { skb_network_header(skb) as *mut llc_pdu_un }
#[inline] pub unsafe fn llc_pdu_header_init(skb: *mut sk_buff, r#type: u8, ssap: u8, dsap: u8, cr: u8) {
    let hlen: usize = match r#type { LLC_PDU_TYPE_U => 3, LLC_PDU_TYPE_U_XID => 6, _ => 4 };
    skb_push(skb, hlen); skb_reset_network_header(skb);
    let pdu = llc_pdu_un_hdr(skb); (*pdu).dsap = dsap; (*pdu).ssap = ssap; (*pdu).ssap |= cr;
}
#[inline] pub unsafe fn llc_pdu_decode_ssap(skb: *mut sk_buff, ssap: *mut u8) { *ssap = (*llc_pdu_un_hdr(skb)).ssap & 0xFE; }
#[inline] pub unsafe fn llc_pdu_decode_dsap(skb: *mut sk_buff, dsap: *mut u8) { *dsap = (*llc_pdu_un_hdr(skb)).dsap & 0xFE; }
#[inline] pub unsafe fn llc_pdu_init_as_ui_cmd(skb: *mut sk_buff) { let p=llc_pdu_un_hdr(skb); (*p).ctrl_1=LLC_PDU_TYPE_U|LLC_1_PDU_CMD_UI; }
#[inline] pub unsafe fn llc_pdu_init_as_test_cmd(skb: *mut sk_buff) { let p=llc_pdu_un_hdr(skb); (*p).ctrl_1=LLC_PDU_TYPE_U|LLC_1_PDU_CMD_TEST|LLC_U_PF_BIT_MASK; }
#[inline] pub unsafe fn FRMR_INFO_SET_REJ_CNTRL(info: *mut llc_frmr_info, rej_ctrl: *const u8) { (*info).rej_pdu_ctrl = if (*rej_ctrl & LLC_PDU_TYPE_U) != LLC_PDU_TYPE_U { *(rej_ctrl as *const u16) } else { *rej_ctrl as u16 }; }
#[inline] pub unsafe fn FRMR_INFO_SET_Vs(info: *mut llc_frmr_info, vs: u8) { (*info).curr_ssv = vs << 1; }
#[inline] pub unsafe fn FRMR_INFO_SET_Vr(info: *mut llc_frmr_info, vr: u8) { (*info).curr_rsv = vr << 1; }
#[inline] pub unsafe fn FRMR_INFO_SET_C_R_BIT(info: *mut llc_frmr_info, cr: u8) { (*info).curr_rsv |= cr & 1; }
#[inline] pub unsafe fn FRMR_INFO_SET_INVALID_PDU_CTRL_IND(info: *mut llc_frmr_info, ind: u8) { (*info).ind_bits = ((*info).ind_bits & 0xFE) | (ind & 1); }
#[inline] pub unsafe fn FRMR_INFO_SET_INVALID_PDU_INFO_IND(info: *mut llc_frmr_info, ind: u8) { (*info).ind_bits = ((*info).ind_bits & 0xFD) | (ind & 2); }
#[inline] pub unsafe fn FRMR_INFO_SET_PDU_INFO_2LONG_IND(info: *mut llc_frmr_info, ind: u8) { (*info).ind_bits = ((*info).ind_bits & 0xFB) | (ind & 4); }
#[inline] pub unsafe fn FRMR_INFO_SET_PDU_INVALID_Nr_IND(info: *mut llc_frmr_info, ind: u8) { (*info).ind_bits = ((*info).ind_bits & 0xF7) | (ind & 8); }
#[inline] pub unsafe fn FRMR_INFO_SET_PDU_INVALID_Ns_IND(info: *mut llc_frmr_info, ind: u8) { (*info).ind_bits = ((*info).ind_bits & 0xEF) | (ind & 0x10); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
