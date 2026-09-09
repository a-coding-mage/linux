/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright(c) 2007 Intel Corporation. All rights reserved.
 *
 * Maintained at www.Open-FCoE.org
 */

/*
 * Fibre Channel Services - Common Transport.
 * From T11.org FC-GS-2 Rev 5.3 November 1998.
 */

#[repr(C)]
pub struct fc_ct_hdr {
    pub ct_rev: __u8,             /* revision */
    pub ct_in_id: [__u8; 3],       /* N_Port ID of original requestor */
    pub ct_fs_type: __u8,          /* type of fibre channel service */
    pub ct_fs_subtype: __u8,       /* subtype */
    pub ct_options: __u8,
    pub _ct_resvd1: __u8,
    pub ct_cmd: __be16,            /* command / response code */
    pub ct_mr_size: __be16,        /* maximum / residual size */
    pub _ct_resvd2: __u8,
    pub ct_reason: __u8,            /* reject reason */
    pub ct_explan: __u8,            /* reason code explanation */
    pub ct_vendor: __u8,            /* vendor unique data */
}

pub const FC_CT_HDR_LEN: usize = 16; /* expected sizeof (struct fc_ct_hdr) */

#[repr(u8)]
pub enum fc_ct_rev {
    FC_CT_REV = 1, /* common transport revision */
}

/*
 * ct_fs_type values.
 */
#[repr(u8)]
pub enum fc_ct_fs_type {
    FC_FST_ALIAS = 0xf8, /* alias service */
    FC_FST_MGMT = 0xfa,  /* management service */
    FC_FST_TIME = 0xfb,  /* time service */
    FC_FST_DIR = 0xfc,   /* directory service */
}

/*
 * ct_cmd: Command / response codes
 */
#[repr(u16)]
pub enum fc_ct_cmd {
    FC_FS_RJT = 0x8001, /* reject */
    FC_FS_ACC = 0x8002, /* accept */
}

/*
 * FS_RJT reason codes.
 */
#[repr(u8)]
pub enum fc_ct_reason {
    FC_FS_RJT_CMD = 0x01,   /* invalid command code */
    FC_FS_RJT_VER = 0x02,   /* invalid version level */
    FC_FS_RJT_LOG = 0x03,   /* logical error */
    FC_FS_RJT_IUSIZ = 0x04, /* invalid IU size */
    FC_FS_RJT_BSY = 0x05,   /* logical busy */
    FC_FS_RJT_PROTO = 0x07, /* protocol error */
    FC_FS_RJT_UNABL = 0x09, /* unable to perform command request */
    FC_FS_RJT_UNSUP = 0x0b, /* command not supported */
}

/*
 * FS_RJT reason code explanations.
 */
#[repr(u8)]
pub enum fc_ct_explan {
    FC_FS_EXP_NONE = 0x00, /* no additional explanation */
    FC_FS_EXP_PID = 0x01,  /* port ID not registered */
    FC_FS_EXP_PNAM = 0x02, /* port name not registered */
    FC_FS_EXP_NNAM = 0x03, /* node name not registered */
    FC_FS_EXP_COS = 0x04,  /* class of service not registered */
    FC_FS_EXP_FTNR = 0x07, /* FC-4 types not registered */
    /* definitions not complete */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
