/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright(c) 2007 Intel Corporation. All rights reserved.
 *
 * Maintained at www.Open-FCoE.org
 */

/*
 * Fibre Channel Services - Name Service (dNS)
 * From T11.org FC-GS-2 Rev 5.3 November 1998.
 */

/* Common-transport sub-type for Name Server. */
pub const FC_NS_SUBTYPE: u32 = 2; /* fs_ct_hdr.ct_fs_subtype */

/*
 * Name server Requests.
 * Note: this is an incomplete list, some unused requests are omitted.
 */
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum fc_ns_req {
    FC_NS_GA_NXT = 0x0100, /* get all next */
    FC_NS_GI_A = 0x0101, /* get identifiers - scope */
    FC_NS_GPN_ID = 0x0112, /* get port name by ID */
    FC_NS_GNN_ID = 0x0113, /* get node name by ID */
    FC_NS_GSPN_ID = 0x0118, /* get symbolic port name */
    FC_NS_GID_PN = 0x0121, /* get ID for port name */
    FC_NS_GID_NN = 0x0131, /* get IDs for node name */
    FC_NS_GID_FT = 0x0171, /* get IDs by FC4 type */
    FC_NS_GPN_FT = 0x0172, /* get port names by FC4 type */
    FC_NS_GID_PT = 0x01a1, /* get IDs by port type */
    FC_NS_RPN_ID = 0x0212, /* reg port name for ID */
    FC_NS_RNN_ID = 0x0213, /* reg node name for ID */
    FC_NS_RFT_ID = 0x0217, /* reg FC4 type for ID */
    FC_NS_RSPN_ID = 0x0218, /* reg symbolic port name */
    FC_NS_RFF_ID = 0x021f, /* reg FC4 Features for ID */
    FC_NS_RSNN_NN = 0x0239, /* reg symbolic node name */
}

/* Port type values. */
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum fc_ns_pt {
    FC_NS_UNID_PORT = 0x00, /* unidentified */
    FC_NS_N_PORT = 0x01, /* N port */
    FC_NS_NL_PORT = 0x02, /* NL port */
    FC_NS_FNL_PORT = 0x03, /* F/NL port */
    FC_NS_NX_PORT = 0x7f, /* Nx port */
    FC_NS_F_PORT = 0x81, /* F port */
    FC_NS_FL_PORT = 0x82, /* FL port */
    FC_NS_E_PORT = 0x84, /* E port */
    FC_NS_B_PORT = 0x85, /* B port */
}

/* Port type object. */
#[repr(C)]
pub struct fc_ns_pt_obj {
    pub pt_type: u8,
}

/* Port ID object */
#[repr(C)]
pub struct fc_ns_fid {
    pub fp_flags: u8, /* flags for responses only */
    pub fp_fid: [u8; 3],
}

/* fp_flags in port ID object, for responses only. */
pub const FC_NS_FID_LAST: u8 = 0x80; /* last object */

/* FC4-types object. */
pub const FC_NS_TYPES: usize = 256; /* number of possible FC-4 types */
pub const FC_NS_BPW: usize = 32; /* bits per word in bitmap */

#[repr(C)]
pub struct fc_ns_fts {
    pub ff_type_map: [u32; FC_NS_TYPES / FC_NS_BPW], /* bitmap of FC-4 types */
}

/* FC4-features object. */
#[repr(C)]
pub struct fc_ns_ff {
    pub fd_feat: [u32; FC_NS_TYPES * 4 / FC_NS_BPW], /* 4-bits per FC-type */
}

/* GID_PT request. */
#[repr(C)]
pub struct fc_ns_gid_pt {
    pub fn_pt_type: u8,
    pub fn_domain_id_scope: u8,
    pub fn_area_id_scope: u8,
    pub fn_resvd: u8,
}

/* GID_FT or GPN_FT request. */
#[repr(C)]
pub struct fc_ns_gid_ft {
    pub fn_resvd: u8,
    pub fn_domain_id_scope: u8,
    pub fn_area_id_scope: u8,
    pub fn_fc4_type: u8,
}

/* GPN_FT response. */
#[repr(C)]
pub struct fc_gpn_ft_resp {
    pub fp_flags: u8, /* see fp_flags definitions above */
    pub fp_fid: [u8; 3], /* port ID */
    pub fp_resvd: u32,
    pub fp_wwpn: u64, /* port name */
}

/* GID_PN request */
#[repr(C)]
pub struct fc_ns_gid_pn {
    pub fn_wwpn: u64, /* port name */
}

/* GID_PN response or GSPN_ID request */
#[repr(C)]
pub struct fc_gid_pn_resp {
    pub fp_resvd: u8,
    pub fp_fid: [u8; 3], /* port ID */
}

/* GSPN_ID response */
#[repr(C)]
pub struct fc_gspn_resp {
    pub fp_name_len: u8,
    pub fp_name: [i8; 0],
}

/* RFT_ID request - register FC-4 types for ID. */
#[repr(C)]
pub struct fc_ns_rft_id {
    pub fr_fid: fc_ns_fid, /* port ID object */
    pub fr_fts: fc_ns_fts, /* FC-4 types object */
}

/* RPN_ID request - register port name for ID.
 * RNN_ID request - register node name for ID.
 */
#[repr(C, packed)]
pub struct fc_ns_rn_id {
    pub fr_fid: fc_ns_fid, /* port ID object */
    pub fr_wwn: u64, /* node name or port name */
}

/* RSNN_NN request - register symbolic node name */
#[repr(C, packed)]
pub struct fc_ns_rsnn {
    pub fr_wwn: u64, /* node name */
    pub fr_name_len: u8,
    pub fr_name: [i8; 0],
}

/* RSPN_ID request - register symbolic port name */
#[repr(C, packed)]
pub struct fc_ns_rspn {
    pub fr_fid: fc_ns_fid, /* port ID object */
    pub fr_name_len: u8,
    pub fr_name: [i8; 0],
}

/* RFF_ID request - register FC-4 Features for ID. */
#[repr(C, packed)]
pub struct fc_ns_rff_id {
    pub fr_fid: fc_ns_fid, /* port ID object */
    pub fr_resvd: [u8; 2],
    pub fr_feat: u8, /* FC-4 Feature bits */
    pub fr_type: u8, /* FC-4 type */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
