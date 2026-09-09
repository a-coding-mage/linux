/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * These are the public elements of the Linux kernel AX.25 code. A similar
 * file netrom.h exists for the NET/ROM protocol.
 */

use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_uchar};

// Supplied by linux/socket.h in the original header.
extern "C" {
    pub static SIOCPROTOPRIVATE: c_ulong;
}

pub const AX25_MTU: c_int = 256;
pub const AX25_MAX_DIGIS: usize = 8;

pub const AX25_WINDOW: c_int = 1;
pub const AX25_T1: c_int = 2;
pub const AX25_N2: c_int = 3;
pub const AX25_T3: c_int = 4;
pub const AX25_T2: c_int = 5;
pub const AX25_BACKOFF: c_int = 6;
pub const AX25_EXTSEQ: c_int = 7;
pub const AX25_PIDINCL: c_int = 8;
pub const AX25_IDLE: c_int = 9;
pub const AX25_PACLEN: c_int = 10;
pub const AX25_IAMDIGI: c_int = 12;

pub const AX25_KILL: c_int = 99;

pub const SIOCAX25GETUID: c_ulong = SIOCPROTOPRIVATE + 0;
pub const SIOCAX25ADDUID: c_ulong = SIOCPROTOPRIVATE + 1;
pub const SIOCAX25DELUID: c_ulong = SIOCPROTOPRIVATE + 2;
pub const SIOCAX25NOUID: c_ulong = SIOCPROTOPRIVATE + 3;
pub const SIOCAX25OPTRT: c_ulong = SIOCPROTOPRIVATE + 7;
pub const SIOCAX25CTLCON: c_ulong = SIOCPROTOPRIVATE + 8;
pub const SIOCAX25GETINFOOLD: c_ulong = SIOCPROTOPRIVATE + 9;
pub const SIOCAX25ADDFWD: c_ulong = SIOCPROTOPRIVATE + 10;
pub const SIOCAX25DELFWD: c_ulong = SIOCPROTOPRIVATE + 11;
pub const SIOCAX25DEVCTL: c_ulong = SIOCPROTOPRIVATE + 12;
pub const SIOCAX25GETINFO: c_ulong = SIOCPROTOPRIVATE + 13;

pub const AX25_SET_RT_IPMODE: c_int = 2;

pub const AX25_NOUID_DEFAULT: c_int = 0;
pub const AX25_NOUID_BLOCK: c_int = 1;

#[repr(C)]
pub struct ax25_address {
    pub ax25_call: [c_char; 7], // 6 call + SSID (shifted ascii!)
}

#[repr(C)]
pub struct sockaddr_ax25 {
    pub sax25_family: __kernel_sa_family_t,
    pub sax25_call: ax25_address,
    pub sax25_ndigis: c_int,
    // Digipeater ax25_address sets follow
}

// #define sax25_uid sax25_ndigis

#[repr(C)]
pub struct full_sockaddr_ax25 {
    pub fsa_ax25: sockaddr_ax25,
    pub fsa_digipeater: [ax25_address; AX25_MAX_DIGIS],
}

#[repr(C)]
pub struct ax25_routes_struct {
    pub port_addr: ax25_address,
    pub dest_addr: ax25_address,
    pub digi_count: c_uchar,
    pub digi_addr: [ax25_address; AX25_MAX_DIGIS],
}

#[repr(C)]
pub struct ax25_route_opt_struct {
    pub port_addr: ax25_address,
    pub dest_addr: ax25_address,
    pub cmd: c_int,
    pub arg: c_int,
}

#[repr(C)]
pub struct ax25_ctl_struct {
    pub port_addr: ax25_address,
    pub source_addr: ax25_address,
    pub dest_addr: ax25_address,
    pub cmd: c_uint,
    pub arg: c_ulong,
    pub digi_count: c_uchar,
    pub digi_addr: [ax25_address; AX25_MAX_DIGIS],
}

/* this will go away. Please do not export to user land */
#[repr(C)]
pub struct ax25_info_struct_deprecated {
    pub n2: c_uint, pub n2count: c_uint,
    pub t1: c_uint, pub t1timer: c_uint,
    pub t2: c_uint, pub t2timer: c_uint,
    pub t3: c_uint, pub t3timer: c_uint,
    pub idle: c_uint, pub idletimer: c_uint,
    pub state: c_uint,
    pub rcv_q: c_uint, pub snd_q: c_uint,
}

#[repr(C)]
pub struct ax25_info_struct {
    pub n2: c_uint, pub n2count: c_uint,
    pub t1: c_uint, pub t1timer: c_uint,
    pub t2: c_uint, pub t2timer: c_uint,
    pub t3: c_uint, pub t3timer: c_uint,
    pub idle: c_uint, pub idletimer: c_uint,
    pub state: c_uint,
    pub rcv_q: c_uint, pub snd_q: c_uint,
    pub vs: c_uint, pub vr: c_uint, pub va: c_uint, pub vs_max: c_uint,
    pub paclen: c_uint,
    pub window: c_uint,
}

#[repr(C)]
pub struct ax25_fwd_struct {
    pub port_from: ax25_address,
    pub port_to: ax25_address,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
