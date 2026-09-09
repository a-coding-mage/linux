/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET  An implementation of the TCP/IP protocol suite for the LINUX
 *       operating system.  INET is implemented using the BSD Socket
 *       interface as the means of communication with the user level.
 *
 *       Definitions for the FDDI handlers.
 *
 * Version:  @(#)fddidevice.h  1.0.0  08/12/96
 *
 * Author: Lawrence V. Stefani, <stefani@lkg.dec.com>
 *
 *       fddidevice.h is based on previous trdevice.h work by
 *           Ross Biro
 *           Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *           Alan Cox, <gw4pts@gw4pts.ampr.org>
 */

// C dependency: <linux/if_fddi.h>

// The following declarations are present only when compiled for the kernel
// (__KERNEL__). The referenced types are supplied by the surrounding crate.
#[cfg(feature = "__KERNEL__")]
extern "C" {
    pub fn fddi_type_trans(
        skb: *mut crate::sk_buff,
        dev: *mut crate::net_device,
    ) -> u16;
    pub fn alloc_fddidev(sizeof_priv: ::core::ffi::c_int) -> *mut crate::net_device;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
