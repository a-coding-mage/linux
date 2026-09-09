/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (c) 2015 6WIND S.A.
 * Author: Nicolas Dichtel <nicolas.dichtel@6wind.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms and conditions of the GNU General Public License,
 * version 2, as published by the Free Software Foundation.
 */

/* Attributes of RTM_NEWNSID/RTM_GETNSID messages */
#[repr(C)]
pub enum netns_attr {
    NETNSA_NONE = 0,
    NETNSA_NSID = 1,
    NETNSA_PID = 2,
    NETNSA_FD = 3,
    NETNSA_TARGET_NSID = 4,
    NETNSA_CURRENT_NSID = 5,
    __NETNSA_MAX = 6,
}

pub const NETNSA_NSID_NOT_ASSIGNED: i32 = -1;
pub const NETNSA_MAX: i32 = __NETNSA_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
