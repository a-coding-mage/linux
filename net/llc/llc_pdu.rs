// SPDX-License-Identifier: GPL-2.0
/*
 * llc_pdu.c - access to PDU internals
 *
 * Copyright (c) 1997 by Procom Technology, Inc.
 *		 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

extern "C" {
    fn llc_pdu_un_hdr(skb: *mut sk_buff) -> *mut llc_pdu_un;
    fn llc_pdu_sn_hdr(skb: *mut sk_buff) -> *mut llc_pdu_sn;
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut core::ffi::c_uchar;
}

unsafe fn llc_pdu_decode_pdu_type(skb: *mut sk_buff, ty: *mut u8);
unsafe fn llc_pdu_get_pf_bit(pdu: *mut llc_pdu_sn) -> u8;

pub unsafe fn llc_pdu_set_cmd_rsp(skb: *mut sk_buff, pdu_type: u8) {
    (*llc_pdu_un_hdr(skb)).ssap |= pdu_type;
}

/**
 * llc_pdu_set_pf_bit - sets poll/final bit in LLC header
 * @skb: Frame to set bit in
 * @bit_value: poll/final bit (0 or 1).
 *
 * This function sets poll/final bit in LLC header (based on type of PDU).
 * in I or S pdus, p/f bit is right bit of fourth byte in header. in U
 * pdus p/f bit is fifth bit of third byte.
 */
pub unsafe fn llc_pdu_set_pf_bit(skb: *mut sk_buff, bit_value: u8) {
    let mut pdu_type: u8 = 0;
    let pdu: *mut llc_pdu_sn;

    llc_pdu_decode_pdu_type(skb, &mut pdu_type);
    pdu = llc_pdu_sn_hdr(skb);

    match pdu_type {
        LLC_PDU_TYPE_I | LLC_PDU_TYPE_S => {
            (*pdu).ctrl_2 = ((*pdu).ctrl_2 & 0xFE) | bit_value;
        }
        LLC_PDU_TYPE_U => {
            (*pdu).ctrl_1 |= ((*pdu).ctrl_1 & 0xEF) | (bit_value << 4);
        }
        _ => {}
    }
}

/**
 * llc_pdu_decode_pf_bit - extracs poll/final bit from LLC header
 * @skb: input skb that p/f bit must be extracted from it
 * @pf_bit: poll/final bit (0 or 1)
 *
 * This function extracts poll/final bit from LLC header (based on type of
 * PDU). In I or S pdus, p/f bit is right bit of fourth byte in header. In
 * U pdus p/f bit is fifth bit of third byte.
 */
pub unsafe fn llc_pdu_decode_pf_bit(skb: *mut sk_buff, pf_bit: *mut u8) {
    let mut pdu_type: u8 = 0;
    let pdu: *mut llc_pdu_sn;

    llc_pdu_decode_pdu_type(skb, &mut pdu_type);
    pdu = llc_pdu_sn_hdr(skb);

    match pdu_type {
        LLC_PDU_TYPE_I | LLC_PDU_TYPE_S => *pf_bit = (*pdu).ctrl_2 & LLC_S_PF_BIT_MASK,
        LLC_PDU_TYPE_U => *pf_bit = ((*pdu).ctrl_1 & LLC_U_PF_BIT_MASK) >> 4,
        _ => {}
    }
}

pub unsafe fn llc_pdu_init_as_disc_cmd(skb: *mut sk_buff, p_bit: u8) {
    let pdu = llc_pdu_un_hdr(skb);
    (*pdu).ctrl_1 = LLC_PDU_TYPE_U;
    (*pdu).ctrl_1 |= LLC_2_PDU_CMD_DISC;
    (*pdu).ctrl_1 |= ((p_bit & 1) << 4) & LLC_U_PF_BIT_MASK;
}

pub unsafe fn llc_pdu_init_as_i_cmd(skb: *mut sk_buff, p_bit: u8, ns: u8, nr: u8) {
    let pdu = llc_pdu_sn_hdr(skb);
    (*pdu).ctrl_1 = LLC_PDU_TYPE_I;
    (*pdu).ctrl_2 = 0;
    (*pdu).ctrl_2 |= p_bit & LLC_I_PF_BIT_MASK; /* p/f bit */
    (*pdu).ctrl_1 |= (ns << 1) & 0xFE; /* set N(S) in bits 2..8 */
    (*pdu).ctrl_2 |= (nr << 1) & 0xFE; /* set N(R) in bits 10..16 */
}

pub unsafe fn llc_pdu_init_as_rej_cmd(skb: *mut sk_buff, p_bit: u8, nr: u8) {
    let pdu = llc_pdu_sn_hdr(skb);
    (*pdu).ctrl_1 = LLC_PDU_TYPE_S;
    (*pdu).ctrl_1 |= LLC_2_PDU_CMD_REJ;
    (*pdu).ctrl_2 = 0;
    (*pdu).ctrl_2 |= p_bit & LLC_S_PF_BIT_MASK;
    (*pdu).ctrl_1 &= 0x0F; /* setting bits 5..8 to zero(reserved) */
    (*pdu).ctrl_2 |= (nr << 1) & 0xFE; /* set N(R) in bits 10..16 */
}

pub unsafe fn llc_pdu_init_as_rnr_cmd(skb: *mut sk_buff, p_bit: u8, nr: u8) {
    let pdu = llc_pdu_sn_hdr(skb);
    (*pdu).ctrl_1 = LLC_PDU_TYPE_S;
    (*pdu).ctrl_1 |= LLC_2_PDU_CMD_RNR;
    (*pdu).ctrl_2 = 0;
    (*pdu).ctrl_2 |= p_bit & LLC_S_PF_BIT_MASK;
    (*pdu).ctrl_1 &= 0x0F;
    (*pdu).ctrl_2 |= (nr << 1) & 0xFE;
}

pub unsafe fn llc_pdu_init_as_rr_cmd(skb: *mut sk_buff, p_bit: u8, nr: u8) {
    let pdu = llc_pdu_sn_hdr(skb);
    (*pdu).ctrl_1 = LLC_PDU_TYPE_S;
    (*pdu).ctrl_1 |= LLC_2_PDU_CMD_RR;
    (*pdu).ctrl_2 = p_bit & LLC_S_PF_BIT_MASK;
    (*pdu).ctrl_1 &= 0x0F;
    (*pdu).ctrl_2 |= (nr << 1) & 0xFE;
}

pub unsafe fn llc_pdu_init_as_sabme_cmd(skb: *mut sk_buff, p_bit: u8) {
    let pdu = llc_pdu_un_hdr(skb);
    (*pdu).ctrl_1 = LLC_PDU_TYPE_U;
    (*pdu).ctrl_1 |= LLC_2_PDU_CMD_SABME;
    (*pdu).ctrl_1 |= ((p_bit & 1) << 4) & LLC_U_PF_BIT_MASK;
}

pub unsafe fn llc_pdu_init_as_dm_rsp(skb: *mut sk_buff, f_bit: u8) {
    let pdu = llc_pdu_un_hdr(skb);
    (*pdu).ctrl_1 = LLC_PDU_TYPE_U;
    (*pdu).ctrl_1 |= LLC_2_PDU_RSP_DM;
    (*pdu).ctrl_1 |= ((f_bit & 1) << 4) & LLC_U_PF_BIT_MASK;
}

pub unsafe fn llc_pdu_init_as_frmr_rsp(
    skb: *mut sk_buff,
    prev_pdu: *mut llc_pdu_sn,
    f_bit: u8,
    vs: u8,
    vr: u8,
    vzyxw: u8,
) {
    let frmr_info: *mut llc_frmr_info;
    let mut prev_pf: u8 = 0;
    let ctrl: *mut u8;
    let pdu = llc_pdu_sn_hdr(skb);

    (*pdu).ctrl_1 = LLC_PDU_TYPE_U;
    (*pdu).ctrl_1 |= LLC_2_PDU_RSP_FRMR;
    (*pdu).ctrl_1 |= ((f_bit & 1) << 4) & LLC_U_PF_BIT_MASK;

    frmr_info = &mut (*pdu).ctrl_2 as *mut _ as *mut llc_frmr_info;
    ctrl = &mut (*prev_pdu).ctrl_1 as *mut _;
    FRMR_INFO_SET_REJ_CNTRL(frmr_info, ctrl);
    FRMR_INFO_SET_Vs(frmr_info, vs);
    FRMR_INFO_SET_Vr(frmr_info, vr);
    prev_pf = llc_pdu_get_pf_bit(prev_pdu);
    FRMR_INFO_SET_C_R_BIT(frmr_info, prev_pf);
    FRMR_INFO_SET_INVALID_PDU_CTRL_IND(frmr_info, vzyxw);
    FRMR_INFO_SET_INVALID_PDU_INFO_IND(frmr_info, vzyxw);
    FRMR_INFO_SET_PDU_INFO_2LONG_IND(frmr_info, vzyxw);
    FRMR_INFO_SET_PDU_INVALID_Nr_IND(frmr_info, vzyxw);
    FRMR_INFO_SET_PDU_INVALID_Ns_IND(frmr_info, vzyxw);
    skb_put(skb, core::mem::size_of::<llc_frmr_info>());
}

pub unsafe fn llc_pdu_init_as_rr_rsp(skb: *mut sk_buff, f_bit: u8, nr: u8) {
    let pdu = llc_pdu_sn_hdr(skb);
    (*pdu).ctrl_1 = LLC_PDU_TYPE_S;
    (*pdu).ctrl_1 |= LLC_2_PDU_RSP_RR;
    (*pdu).ctrl_2 = 0;
    (*pdu).ctrl_2 |= f_bit & LLC_S_PF_BIT_MASK;
    (*pdu).ctrl_1 &= 0x0F;
    (*pdu).ctrl_2 |= (nr << 1) & 0xFE;
}

pub unsafe fn llc_pdu_init_as_rej_rsp(skb: *mut sk_buff, f_bit: u8, nr: u8) {
    let pdu = llc_pdu_sn_hdr(skb);
    (*pdu).ctrl_1 = LLC_PDU_TYPE_S;
    (*pdu).ctrl_1 |= LLC_2_PDU_RSP_REJ;
    (*pdu).ctrl_2 = 0;
    (*pdu).ctrl_2 |= f_bit & LLC_S_PF_BIT_MASK;
    (*pdu).ctrl_1 &= 0x0F;
    (*pdu).ctrl_2 |= (nr << 1) & 0xFE;
}

pub unsafe fn llc_pdu_init_as_rnr_rsp(skb: *mut sk_buff, f_bit: u8, nr: u8) {
    let pdu = llc_pdu_sn_hdr(skb);
    (*pdu).ctrl_1 = LLC_PDU_TYPE_S;
    (*pdu).ctrl_1 |= LLC_2_PDU_RSP_RNR;
    (*pdu).ctrl_2 = 0;
    (*pdu).ctrl_2 |= f_bit & LLC_S_PF_BIT_MASK;
    (*pdu).ctrl_1 &= 0x0F;
    (*pdu).ctrl_2 |= (nr << 1) & 0xFE;
}

pub unsafe fn llc_pdu_init_as_ua_rsp(skb: *mut sk_buff, f_bit: u8) {
    let pdu = llc_pdu_un_hdr(skb);
    (*pdu).ctrl_1 = LLC_PDU_TYPE_U;
    (*pdu).ctrl_1 |= LLC_2_PDU_RSP_UA;
    (*pdu).ctrl_1 |= ((f_bit & 1) << 4) & LLC_U_PF_BIT_MASK;
}

unsafe fn llc_pdu_decode_pdu_type(skb: *mut sk_buff, ty: *mut u8) {
    let pdu = llc_pdu_un_hdr(skb);
    if (*pdu).ctrl_1 & 1 != 0 {
        if (*pdu).ctrl_1 & LLC_PDU_TYPE_U == LLC_PDU_TYPE_U {
            *ty = LLC_PDU_TYPE_U;
        } else {
            *ty = LLC_PDU_TYPE_S;
        }
    } else {
        *ty = LLC_PDU_TYPE_I;
    }
}

unsafe fn llc_pdu_get_pf_bit(pdu: *mut llc_pdu_sn) -> u8 {
    let pdu_type: u8;
    let mut pf_bit: u8 = 0;

    if (*pdu).ctrl_1 & 1 != 0 {
        if (*pdu).ctrl_1 & LLC_PDU_TYPE_U == LLC_PDU_TYPE_U {
            pdu_type = LLC_PDU_TYPE_U;
        } else {
            pdu_type = LLC_PDU_TYPE_S;
        }
    } else {
        pdu_type = LLC_PDU_TYPE_I;
    }
    match pdu_type {
        LLC_PDU_TYPE_I | LLC_PDU_TYPE_S => pf_bit = (*pdu).ctrl_2 & LLC_S_PF_BIT_MASK,
        LLC_PDU_TYPE_U => pf_bit = ((*pdu).ctrl_1 & LLC_U_PF_BIT_MASK) >> 4,
        _ => {}
    }
    pf_bit
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
