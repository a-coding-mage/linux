/* SPDX-License-Identifier: GPL-2.0-only */
/**
 * file phonet.h
 *
 * Phonet sockets kernel interface
 *
 * Copyright (C) 2008 Nokia Corporation. All rights reserved.
 */

/* Dependency intent: declarations from <uapi/linux/phonet.h> are supplied externally. */

pub const SIOCPNGAUTOCONF: _ = SIOCDEVPRIVATE + 0;

#[repr(C)]
pub struct IfPhonetAutoconf {
    pub device: u8,
}

#[repr(C)]
pub union IfPhonetReqIfru {
    pub ifru_phonet_autoconf: IfPhonetAutoconf,
}

#[repr(C)]
pub struct IfPhonetReq {
    pub ifr_phonet_name: [core::ffi::c_char; 16],
    pub ifr_ifru: IfPhonetReqIfru,
}

/* C macro equivalent: ifr_phonet_autoconf expands to ifr_ifru.ifru_phonet_autoconf. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
