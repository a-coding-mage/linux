/* SPDX-License-Identifier: GPL-2.0
 * Marvell OcteonTX CPT driver
 *
 * Copyright (C) 2019 Marvell International Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const OTX_CPT_MAX_MBOX_DATA_STR_SIZE: usize = 64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum otx_cptpf_type {
    OTX_CPT_AE = 2,
    OTX_CPT_SE = 3,
    BAD_OTX_CPTPF_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum otx_cptvf_type {
    OTX_CPT_AE_TYPES = 1,
    OTX_CPT_SE_TYPES = 2,
    BAD_OTX_CPTVF_TYPE,
}

/* VF-PF message opcodes */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum otx_cpt_mbox_opcode {
    OTX_CPT_MSG_VF_UP = 1,
    OTX_CPT_MSG_VF_DOWN,
    OTX_CPT_MSG_READY,
    OTX_CPT_MSG_QLEN,
    OTX_CPT_MSG_QBIND_GRP,
    OTX_CPT_MSG_VQ_PRIORITY,
    OTX_CPT_MSG_PF_TYPE,
    OTX_CPT_MSG_ACK,
    OTX_CPT_MSG_NACK,
}

/* OcteonTX CPT mailbox structure */
#[repr(C)]
pub struct otx_cpt_mbox {
    pub msg: u64, /* Message type MBOX[0] */
    pub data: u64, /* Data         MBOX[1] */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
