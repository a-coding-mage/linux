/* SPDX-License-Identifier: GPL-2.0 */
/* SCTP tracking. */

/* Corresponds to: #include <uapi/linux/netfilter/nf_conntrack_sctp.h> */

#[repr(C)]
pub struct ip_ct_sctp {
    pub state: sctp_conntrack,

    pub vtag: [u32; IP_CT_DIR_MAX],
    pub init: [u8; IP_CT_DIR_MAX],
    pub last_dir: u8,
    pub flags: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
