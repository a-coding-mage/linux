/* SPDX-License-Identifier: GPL-2.0 */

// Required declarations are supplied by the corresponding translated
// dependency headers.

pub const FTP_PORT: u32 = 21;

pub const NF_CT_FTP_SEQ_PICKUP: u32 = 1 << 0;

pub const NUM_SEQ_TO_REMEMBER: usize = 2;

/* This structure exists only once per master */
#[repr(C)]
pub struct nf_ct_ftp_master {
	/* Valid seq positions for cmd matching after newline */
	pub seq_aft_nl: [[u32; NUM_SEQ_TO_REMEMBER]; IP_CT_DIR_MAX as usize],
	/* 0 means seq_match_aft_nl not set */
	pub seq_aft_nl_num: [u16; IP_CT_DIR_MAX as usize],
	/* pickup sequence tracking, useful for conntrackd */
	pub flags: [u16; IP_CT_DIR_MAX as usize],
}

/* For NAT to hook in when we find a packet which describes what other
 * connection we should expect. */
pub type nf_nat_ftp_hook_fn = unsafe extern "C" fn(
	skb: *mut sk_buff,
	ct: *mut nf_conn,
	ctinfo: ip_conntrack_info,
	type_: nf_ct_ftp_type,
	protoff: u32,
	matchoff: u32,
	matchlen: u32,
	exp: *mut nf_conntrack_expect,
);

extern "C" {
	pub static mut nf_nat_ftp_hook: *mut nf_nat_ftp_hook_fn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
