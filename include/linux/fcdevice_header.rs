/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  NET  is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Definitions for the Fibre Channel handlers.
 *
 * Version:	@(#)fcdevice.h	1.0.0	09/26/98
 *
 * Authors:	Vineet Abraham <vma@iol.unh.edu>
 *
 *		Relocated to include/linux where it belongs by Alan Cox
 *							<gw4pts@gw4pts.ampr.org>
 *
 *	WARNING: This move may well be temporary. This file will get merged with others RSN.
 */

// The C header <linux/if_fc.h> supplies related Fibre Channel definitions.

// In the original header, this declaration is enabled only when __KERNEL__ is
// defined. The surrounding build configuration supplies `net_device`.
extern "C" {
    pub fn alloc_fcdev(sizeof_priv: ::core::ffi::c_int) -> *mut net_device;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
