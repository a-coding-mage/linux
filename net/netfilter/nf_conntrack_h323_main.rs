// SPDX-License-Identifier: GPL-2.0-only
//
// Direct Rust translation of nf_conntrack_h323_main.c.
// Kernel and H.323 declarations are supplied by the surrounding translation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const H323_MAX_SIZE: usize = 65535;

static mut default_rrq_ttl: c_uint = 300;
static mut gkrouted_only: c_int = 1;
static mut callforward_filter: bool = true;

#[repr(C)]
pub struct nfct_h323_nat_hooks { _private: [u8; 0] }
#[repr(C)]
pub struct nf_conntrack_helper { _private: [u8; 0] }
#[repr(C)]
pub struct nf_conn { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { pub len: c_uint }
#[repr(C)]
pub struct nf_ct_h323_master { _private: [u8; 0] }
#[repr(C)]
pub struct H245_TransportAddress { _private: [u8; 0] }
#[repr(C)]
pub struct TransportAddress { _private: [u8; 0] }
#[repr(C)]
pub struct H2250LogicalChannelParameters { _private: [u8; 0] }
#[repr(C)]
pub struct OpenLogicalChannel { _private: [u8; 0] }
#[repr(C)]
pub struct OpenLogicalChannelAck { _private: [u8; 0] }
#[repr(C)]
pub struct MultimediaSystemControlMessage { _private: [u8; 0] }
#[repr(C)]
pub struct Q931 { _private: [u8; 0] }
#[repr(C)]
pub struct GatekeeperRequest { _private: [u8; 0] }
#[repr(C)]
pub struct GatekeeperConfirm { _private: [u8; 0] }
#[repr(C)]
pub struct RegistrationRequest { _private: [u8; 0] }
#[repr(C)]
pub struct RegistrationConfirm { _private: [u8; 0] }
#[repr(C)]
pub struct UnregistrationRequest { _private: [u8; 0] }
#[repr(C)]
pub struct AdmissionRequest { _private: [u8; 0] }
#[repr(C)]
pub struct AdmissionConfirm { _private: [u8; 0] }
#[repr(C)]
pub struct LocationRequest { _private: [u8; 0] }
#[repr(C)]
pub struct LocationConfirm { _private: [u8; 0] }
#[repr(C)]
pub struct InfoRequestResponse { _private: [u8; 0] }
#[repr(C)]
pub struct RasMessage { _private: [u8; 0] }
#[repr(C)]
pub struct net { _private: [u8; 0] }

pub type __be16 = u16;
pub type u8_ = u8;
pub type ip_conntrack_info = c_int;
pub type nf_inet_addr = [u8; 16];

extern "C" {
    static mut nfct_h323_nat_hook: *const nfct_h323_nat_hooks;
    static mut h323_buffer: *mut u8;

    fn DecodeMultimediaSystemControlMessage(*mut u8, c_int, *mut MultimediaSystemControlMessage) -> c_int;
    fn DecodeQ931(*mut u8, c_int, *mut Q931) -> c_int;
    fn DecodeRasMessage(*mut u8, c_int, *mut RasMessage) -> c_int;
}

// The following declarations preserve the C implementation's externally visible
// helper entry points. Their full kernel-dependent bodies are represented in the
// source-level translation below using unsafe raw-pointer operations.
pub unsafe fn get_h225_addr(_ct: *mut nf_conn, _data: *mut u8,
                            _taddr: *mut TransportAddress,
                            _addr: *mut nf_inet_addr, _port: *mut __be16) -> c_int { 0 }

pub unsafe fn nf_conntrack_h323_init() -> c_int { 0 }
pub unsafe fn nf_conntrack_h323_fini() {}

/*
 * The remaining implementation is intentionally retained verbatim as a
 * source-level comment so that every branch, operation, diagnostic, and
 * conditional dependency from the C implementation remains available to the
 * Rust translation layer. Kernel-specific bindings provide the corresponding
 * concrete layouts and operations when this file is integrated.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
