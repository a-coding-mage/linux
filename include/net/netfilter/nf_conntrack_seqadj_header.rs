/* SPDX-License-Identifier: GPL-2.0 */
// Translated from nf_conntrack_seqadj.h.
// Dependency: <net/netfilter/nf_conntrack_extend.h>

/// Sequence number adjustment information.
///
/// `correction_pos`: position of the last TCP sequence number modification.
/// `offset_before`: sequence number offset before last modification.
/// `offset_after`: sequence number offset after last modification.
#[repr(C)]
pub struct nf_ct_seqadj {
    pub correction_pos: u32,
    pub offset_before: i32,
    pub offset_after: i32,
}

#[repr(C)]
pub struct nf_conn_seqadj {
    pub seq: [nf_ct_seqadj; IP_CT_DIR_MAX as usize],
}

pub unsafe fn nfct_seqadj(ct: *const nf_conn) -> *mut nf_conn_seqadj {
    nf_ct_ext_find(ct, NF_CT_EXT_SEQADJ)
}

pub unsafe fn nfct_seqadj_ext_add(ct: *mut nf_conn) -> *mut nf_conn_seqadj {
    nf_ct_ext_add(ct, NF_CT_EXT_SEQADJ, GFP_ATOMIC)
}

pub unsafe extern "C" fn nf_ct_seqadj_init(
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    off: i32,
) -> i32;

pub unsafe extern "C" fn nf_ct_seqadj_set(
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    seq: __be32,
    off: i32,
) -> i32;

pub unsafe extern "C" fn nf_ct_tcp_seqadj_set(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    off: i32,
);

pub unsafe extern "C" fn nf_ct_seq_adjust(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    protoff: u32,
) -> i32;

pub unsafe extern "C" fn nf_ct_seq_offset(
    ct: *const nf_conn,
    dir: ip_conntrack_dir,
    seq: u32,
) -> i32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
