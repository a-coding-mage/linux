/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel translation units.

pub const H323_RTP_CHANNEL_MAX: usize = 4; // Audio, video, FAX and other

/* This structure exists only once per master */
#[repr(C)]
pub union nf_ct_h323_master__bindgen_ty_1 {
	/* RAS connection timeout */
	pub timeout: u32,

	/* Next TPKT length (for separate TPKT header and data) */
	pub tpkt_len: [u16; IP_CT_DIR_MAX as usize],
}

#[repr(C)]
pub struct nf_ct_h323_master {
	/* Original and NATed Q.931 or H.245 signal ports */
	pub sig_port: [u16; IP_CT_DIR_MAX as usize],

	/* Original and NATed RTP ports */
	pub rtp_port: [[u16; IP_CT_DIR_MAX as usize]; H323_RTP_CHANNEL_MAX],

	pub __bindgen_anon_1: nf_ct_h323_master__bindgen_ty_1,
}

extern "C" {
	pub fn get_h225_addr(
		ct: *mut nf_conn,
		data: *mut u8,
		taddr: *mut TransportAddress,
		addr: *mut nf_inet_addr,
		port: *mut u16,
	) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct nfct_h323_nat_hooks {
	pub set_h245_addr: Option<unsafe extern "C" fn(
		skb: *mut sk_buff,
		protoff: u32,
		data: *mut *mut u8,
		dataoff: ::core::ffi::c_int,
		taddr: *mut H245_TransportAddress,
		addr: *mut nf_inet_addr,
		port: u16,
	) -> ::core::ffi::c_int>,
	pub set_h225_addr: Option<unsafe extern "C" fn(
		skb: *mut sk_buff,
		protoff: u32,
		data: *mut *mut u8,
		dataoff: ::core::ffi::c_int,
		taddr: *mut TransportAddress,
		addr: *mut nf_inet_addr,
		port: u16,
	) -> ::core::ffi::c_int>,
	pub set_sig_addr: Option<unsafe extern "C" fn(
		skb: *mut sk_buff,
		ct: *mut nf_conn,
		ctinfo: ip_conntrack_info,
		protoff: u32,
		data: *mut *mut u8,
		taddr: *mut TransportAddress,
		count: ::core::ffi::c_int,
	) -> ::core::ffi::c_int>,
	pub set_ras_addr: Option<unsafe extern "C" fn(
		skb: *mut sk_buff,
		ct: *mut nf_conn,
		ctinfo: ip_conntrack_info,
		protoff: u32,
		data: *mut *mut u8,
		taddr: *mut TransportAddress,
		count: ::core::ffi::c_int,
	) -> ::core::ffi::c_int>,
	pub nat_rtp_rtcp: Option<unsafe extern "C" fn(
		skb: *mut sk_buff,
		ct: *mut nf_conn,
		ctinfo: ip_conntrack_info,
		protoff: u32,
		data: *mut *mut u8,
		dataoff: ::core::ffi::c_int,
		taddr: *mut H245_TransportAddress,
		port: u16,
		rtp_port: u16,
		rtp_exp: *mut nf_conntrack_expect,
		rtcp_exp: *mut nf_conntrack_expect,
	) -> ::core::ffi::c_int>,
	pub nat_t120: Option<unsafe extern "C" fn(
		skb: *mut sk_buff,
		ct: *mut nf_conn,
		ctinfo: ip_conntrack_info,
		protoff: u32,
		data: *mut *mut u8,
		dataoff: ::core::ffi::c_int,
		taddr: *mut H245_TransportAddress,
		port: u16,
		exp: *mut nf_conntrack_expect,
	) -> ::core::ffi::c_int>,
	pub nat_h245: Option<unsafe extern "C" fn(
		skb: *mut sk_buff,
		ct: *mut nf_conn,
		ctinfo: ip_conntrack_info,
		protoff: u32,
		data: *mut *mut u8,
		dataoff: ::core::ffi::c_int,
		taddr: *mut TransportAddress,
		port: u16,
		exp: *mut nf_conntrack_expect,
	) -> ::core::ffi::c_int>,
	pub nat_callforwarding: Option<unsafe extern "C" fn(
		skb: *mut sk_buff,
		ct: *mut nf_conn,
		ctinfo: ip_conntrack_info,
		protoff: u32,
		data: *mut *mut u8,
		dataoff: ::core::ffi::c_int,
		taddr: *mut TransportAddress,
		port: u16,
		exp: *mut nf_conntrack_expect,
	) -> ::core::ffi::c_int>,
	pub nat_q931: Option<unsafe extern "C" fn(
		skb: *mut sk_buff,
		ct: *mut nf_conn,
		ctinfo: ip_conntrack_info,
		protoff: u32,
		data: *mut *mut u8,
		taddr: *mut TransportAddress,
		idx: ::core::ffi::c_int,
		port: u16,
		exp: *mut nf_conntrack_expect,
	) -> ::core::ffi::c_int>,
}

extern "C" {
	pub static mut nfct_h323_nat_hook: *const nfct_h323_nat_hooks;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
