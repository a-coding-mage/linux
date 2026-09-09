/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding kernel translation.

pub const SIP_PORT: u32 = 5060;
pub const SIP_TIMEOUT: u32 = 3600;

#[repr(C)]
pub struct nf_ct_sip_master {
    pub register_cseq: u32,
    pub invite_cseq: u32,
    pub forced_dport: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sip_expectation_classes {
    SIP_EXPECT_SIGNALLING,
    SIP_EXPECT_AUDIO,
    SIP_EXPECT_VIDEO,
    SIP_EXPECT_IMAGE,
    __SIP_EXPECT_MAX,
}

pub const SIP_EXPECT_MAX: sip_expectation_classes = sip_expectation_classes::__SIP_EXPECT_MAX;

#[repr(C)]
pub struct sdp_media_type {
    pub name: *const core::ffi::c_char,
    pub len: u32,
    pub class: sip_expectation_classes,
}

#[macro_export]
macro_rules! SDP_MEDIA_TYPE {
    ($name:expr, $class:expr) => {
        sdp_media_type { name: $name, len: core::mem::size_of_val(&$name) as u32 - 1, class: $class }
    };
}

#[repr(C)]
pub struct sip_handler {
    pub method: *const core::ffi::c_char,
    pub len: u32,
    pub request: Option<unsafe extern "C" fn(*mut sk_buff, u32, u32, *mut *const core::ffi::c_char, *mut u32, u32) -> i32>,
    pub response: Option<unsafe extern "C" fn(*mut sk_buff, u32, u32, *mut *const core::ffi::c_char, *mut u32, u32, u32) -> i32>,
}

#[macro_export]
macro_rules! SIP_HANDLER {
    ($method:expr, $request:expr, $response:expr) => {
        sip_handler { method: $method, len: core::mem::size_of_val(&$method) as u32 - 1, request: $request, response: $response }
    };
}

#[repr(C)]
pub struct sip_header {
    pub name: *const core::ffi::c_char,
    pub cname: *const core::ffi::c_char,
    pub search: *const core::ffi::c_char,
    pub len: u32,
    pub clen: u32,
    pub slen: u32,
    pub match_len: Option<unsafe extern "C" fn(*const nf_conn, *const core::ffi::c_char, *const core::ffi::c_char, *mut i32) -> i32>,
}

#[macro_export]
macro_rules! __SIP_HDR {
    ($name:expr, $cname:expr, $search:expr, $match:expr) => {
        sip_header { name: $name, len: core::mem::size_of_val(&$name) as u32 - 1, cname: $cname, clen: if !$cname.is_null() { core::mem::size_of_val(&$cname) as u32 - 1 } else { 0 }, search: $search, slen: if !$search.is_null() { core::mem::size_of_val(&$search) as u32 - 1 } else { 0 }, match_len: $match }
    };
}

#[macro_export]
macro_rules! SIP_HDR { ($name:expr, $cname:expr, $search:expr, $match:expr) => { __SIP_HDR!($name, $cname, $search, $match) }; }
#[macro_export]
macro_rules! SDP_HDR { ($name:expr, $search:expr, $match:expr) => { __SIP_HDR!($name, core::ptr::null(), $search, $match) }; }

#[repr(C)]
pub enum sip_header_types { SIP_HDR_CSEQ, SIP_HDR_FROM, SIP_HDR_TO, SIP_HDR_CONTACT, SIP_HDR_VIA_UDP, SIP_HDR_VIA_TCP, SIP_HDR_EXPIRES, SIP_HDR_CONTENT_LENGTH, SIP_HDR_CALL_ID }

#[repr(C)]
pub enum sdp_header_types { SDP_HDR_UNSPEC, SDP_HDR_VERSION, SDP_HDR_OWNER, SDP_HDR_CONNECTION, SDP_HDR_MEDIA }

#[repr(C)]
pub struct nf_nat_sip_hooks {
    pub msg: Option<unsafe extern "C" fn(*mut sk_buff, u32, u32, *mut *const core::ffi::c_char, *mut u32) -> u32>,
    pub seq_adjust: Option<unsafe extern "C" fn(*mut sk_buff, u32, s32)>,
    pub expect: Option<unsafe extern "C" fn(*mut sk_buff, u32, u32, *mut *const core::ffi::c_char, *mut u32, *mut nf_conntrack_expect, u32, u32) -> u32>,
    pub sdp_addr: Option<unsafe extern "C" fn(*mut sk_buff, u32, u32, *mut *const core::ffi::c_char, *mut u32, u32, sdp_header_types, sdp_header_types, *const nf_inet_addr) -> u32>,
    pub sdp_port: Option<unsafe extern "C" fn(*mut sk_buff, u32, u32, *mut *const core::ffi::c_char, *mut u32, u32, u32, u_int16_t) -> u32>,
    pub sdp_session: Option<unsafe extern "C" fn(*mut sk_buff, u32, u32, *mut *const core::ffi::c_char, *mut u32, u32, *const nf_inet_addr) -> u32>,
    pub sdp_media: Option<unsafe extern "C" fn(*mut sk_buff, u32, u32, *mut *const core::ffi::c_char, *mut u32, *mut nf_conntrack_expect, *mut nf_conntrack_expect, u32, u32, *mut nf_inet_addr) -> u32>,
}

extern "C" {
    pub static nf_nat_sip_hooks: *const nf_nat_sip_hooks;
    pub fn ct_sip_parse_request(ct: *const nf_conn, dptr: *const core::ffi::c_char, datalen: u32, matchoff: *mut u32, matchlen: *mut u32, addr: *mut nf_inet_addr, port: *mut __be16) -> i32;
    pub fn ct_sip_get_header(ct: *const nf_conn, dptr: *const core::ffi::c_char, dataoff: u32, datalen: u32, ty: sip_header_types, matchoff: *mut u32, matchlen: *mut u32) -> i32;
    pub fn ct_sip_parse_header_uri(ct: *const nf_conn, dptr: *const core::ffi::c_char, dataoff: *mut u32, datalen: u32, ty: sip_header_types, in_header: *mut i32, matchoff: *mut u32, matchlen: *mut u32, addr: *mut nf_inet_addr, port: *mut __be16) -> i32;
    pub fn ct_sip_parse_address_param(ct: *const nf_conn, dptr: *const core::ffi::c_char, dataoff: u32, datalen: u32, name: *const core::ffi::c_char, matchoff: *mut u32, matchlen: *mut u32, addr: *mut nf_inet_addr, delim: bool) -> i32;
    pub fn ct_sip_parse_numerical_param(ct: *const nf_conn, dptr: *const core::ffi::c_char, off: u32, datalen: u32, name: *const core::ffi::c_char, matchoff: *mut u32, matchen: *mut u32, val: *mut u32) -> i32;
    pub fn ct_sip_get_sdp_header(ct: *const nf_conn, dptr: *const core::ffi::c_char, dataoff: u32, datalen: u32, ty: sdp_header_types, term: sdp_header_types, matchoff: *mut u32, matchlen: *mut u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
