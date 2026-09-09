/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * These are the public elements of the Linux kernel X.25 implementation.
 *
 * 	History
 *	mar/20/00	Daniela Squassoni Disabling/enabling of facilities
 *					  negotiation.
 *	apr/02/05	Shaun Pereira Selective sub address matching with
 *					 call user data
 */

/* C headers: linux/types.h and linux/socket.h */
use core::ffi::{c_char, c_ulong};
use core::mem::size_of;

pub const SIOCX25GSUBSCRIP: _ = SIOCPROTOPRIVATE + 0;
pub const SIOCX25SSUBSCRIP: _ = SIOCPROTOPRIVATE + 1;
pub const SIOCX25GFACILITIES: _ = SIOCPROTOPRIVATE + 2;
pub const SIOCX25SFACILITIES: _ = SIOCPROTOPRIVATE + 3;
pub const SIOCX25GCALLUSERDATA: _ = SIOCPROTOPRIVATE + 4;
pub const SIOCX25SCALLUSERDATA: _ = SIOCPROTOPRIVATE + 5;
pub const SIOCX25GCAUSEDIAG: _ = SIOCPROTOPRIVATE + 6;
pub const SIOCX25SCUDMATCHLEN: _ = SIOCPROTOPRIVATE + 7;
pub const SIOCX25CALLACCPTAPPRV: _ = SIOCPROTOPRIVATE + 8;
pub const SIOCX25SENDCALLACCPT: _ = SIOCPROTOPRIVATE + 9;
pub const SIOCX25GDTEFACILITIES: _ = SIOCPROTOPRIVATE + 10;
pub const SIOCX25SDTEFACILITIES: _ = SIOCPROTOPRIVATE + 11;
pub const SIOCX25SCAUSEDIAG: _ = SIOCPROTOPRIVATE + 12;

/* Values for {get,set}sockopt. */
pub const X25_QBITINCL: u32 = 1;

/* X.25 Packet Size values. */
pub const X25_PS16: u32 = 4;
pub const X25_PS32: u32 = 5;
pub const X25_PS64: u32 = 6;
pub const X25_PS128: u32 = 7;
pub const X25_PS256: u32 = 8;
pub const X25_PS512: u32 = 9;
pub const X25_PS1024: u32 = 10;
pub const X25_PS2048: u32 = 11;
pub const X25_PS4096: u32 = 12;

#[repr(C)]
pub struct x25_address {
    pub x25_addr: [c_char; 16],
}

#[repr(C)]
pub struct sockaddr_x25 {
    pub sx25_family: __kernel_sa_family_t,
    pub sx25_addr: x25_address,
}

#[repr(C)]
pub struct x25_subscrip_struct {
    pub device: [c_char; 200 - size_of::<c_ulong>()],
    pub global_facil_mask: c_ulong,
    pub extended: u32,
}

pub const X25_MASK_REVERSE: u32 = 0x01;
pub const X25_MASK_THROUGHPUT: u32 = 0x02;
pub const X25_MASK_PACKET_SIZE: u32 = 0x04;
pub const X25_MASK_WINDOW_SIZE: u32 = 0x08;
pub const X25_MASK_CALLING_AE: u32 = 0x10;
pub const X25_MASK_CALLED_AE: u32 = 0x20;

#[repr(C)]
pub struct x25_route_struct {
    pub address: x25_address,
    pub sigdigits: u32,
    pub device: [c_char; 200],
}

#[repr(C)]
pub struct x25_facilities {
    pub winsize_in: u32,
    pub winsize_out: u32,
    pub pacsize_in: u32,
    pub pacsize_out: u32,
    pub throughput: u32,
    pub reverse: u32,
}

#[repr(C)]
pub struct x25_dte_facilities {
    pub delay_cumul: __u16,
    pub delay_target: __u16,
    pub delay_max: __u16,
    pub min_throughput: __u8,
    pub expedited: __u8,
    pub calling_len: __u8,
    pub called_len: __u8,
    pub calling_ae: [__u8; 20],
    pub called_ae: [__u8; 20],
}

#[repr(C)]
pub struct x25_calluserdata {
    pub cudlength: u32,
    pub cuddata: [u8; 128],
}

#[repr(C)]
pub struct x25_causediag {
    pub cause: u8,
    pub diagnostic: u8,
}

#[repr(C)]
pub struct x25_subaddr {
    pub cudmatchlength: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
