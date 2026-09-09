/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 *  Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

// Dependency equivalent of <linux/socket.h>; `sockaddr` is supplied externally.

/* struct used to pass info to network drivers */
#[repr(C)]
pub struct ltq_eth_data {
    pub mac: sockaddr,
    pub mii_mode: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
