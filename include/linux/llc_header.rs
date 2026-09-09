/* SPDX-License-Identifier: GPL-2.0 */
/*
 * IEEE 802.2 User Interface SAPs for Linux, data structures and indicators.
 *
 * Copyright (c) 2001 by Jay Schulist <jschlst@samba.org>
 */

// Dependency equivalent of: #include <uapi/linux/llc.h>

pub const LLC_SAP_DYN_START: u32 = 0xC0;
pub const LLC_SAP_DYN_STOP: u32 = 0xDE;
pub const LLC_SAP_DYN_TRIES: u32 = 4;

// Equivalent of:
// #define llc_ui_skb_cb(__skb) ((struct sockaddr_llc *)&((__skb)->cb[0]))
#[macro_export]
macro_rules! llc_ui_skb_cb {
    ($skb:expr) => {{
        unsafe { &mut (*($skb)).cb[0] as *mut _ as *mut sockaddr_llc }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
