/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This file define a set of standard wireless extensions
 *
 * Version :\t22\t16.3.07
 *
 * Authors :\tJean Tourrilhes - HPL - <jt@hpl.hp.com>
 * Copyright (c) 1997-2007 Jean Tourrilhes, All Rights Reserved.
 */

/* Translated from <uapi/linux/wireless.h>. */

/* The following declarations are present only when CONFIG_COMPAT is enabled. */
#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct compat_iw_point {
    pub pointer: compat_caddr_t,
    pub length: u16,
    pub flags: u16,
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub union __compat_iw_event {
    pub pointer: compat_caddr_t,

    /* we need ptr_bytes to make memcpy() run-time destination
     * buffer bounds checking happy, nothing special
     */
    pub ptr_bytes: [u8; 0],
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct __compat_iw_event_layout {
    pub len: u16, /* Real length of this stuff */
    pub cmd: u16, /* Wireless IOCTL */
    pub data: __compat_iw_event,
}

#[cfg(CONFIG_COMPAT)]
pub const IW_EV_COMPAT_LCP_LEN: usize = core::mem::offset_of!(__compat_iw_event_layout, data);

#[cfg(CONFIG_COMPAT)]
pub const IW_EV_COMPAT_POINT_OFF: usize = core::mem::offset_of!(compat_iw_point, length);

/* Size of the various events for compat */
#[cfg(CONFIG_COMPAT)]
pub const IW_EV_COMPAT_CHAR_LEN: usize = IW_EV_COMPAT_LCP_LEN + IFNAMSIZ;

#[cfg(CONFIG_COMPAT)]
pub const IW_EV_COMPAT_UINT_LEN: usize = IW_EV_COMPAT_LCP_LEN + core::mem::size_of::<u32>();

#[cfg(CONFIG_COMPAT)]
pub const IW_EV_COMPAT_FREQ_LEN: usize = IW_EV_COMPAT_LCP_LEN + core::mem::size_of::<iw_freq>();

#[cfg(CONFIG_COMPAT)]
pub const IW_EV_COMPAT_PARAM_LEN: usize = IW_EV_COMPAT_LCP_LEN + core::mem::size_of::<iw_param>();

#[cfg(CONFIG_COMPAT)]
pub const IW_EV_COMPAT_ADDR_LEN: usize = IW_EV_COMPAT_LCP_LEN + core::mem::size_of::<sockaddr>();

#[cfg(CONFIG_COMPAT)]
pub const IW_EV_COMPAT_QUAL_LEN: usize = IW_EV_COMPAT_LCP_LEN + core::mem::size_of::<iw_quality>();

#[cfg(CONFIG_COMPAT)]
pub const IW_EV_COMPAT_POINT_LEN: usize = IW_EV_COMPAT_LCP_LEN
    + core::mem::size_of::<compat_iw_point>()
    - IW_EV_COMPAT_POINT_OFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
