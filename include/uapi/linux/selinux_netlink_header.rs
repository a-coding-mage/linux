/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Netlink event notifications for SELinux.
 *
 * Author: James Morris <jmorris@redhat.com>
 *
 * Copyright (C) 2004 Red Hat, Inc., James Morris <jmorris@redhat.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2,
 * as published by the Free Software Foundation.
 */

/* Dependency: linux/types.h supplies the signed and unsigned 32-bit types. */

/* Message types. */
pub const SELNL_MSG_BASE: i32 = 0x10;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SelNlMsg {
    SELNL_MSG_SETENFORCE = SELNL_MSG_BASE,
    SELNL_MSG_POLICYLOAD,
    SELNL_MSG_MAX,
}

/* Multicast groups - backwards compatiblility for userspace.
 * These constants are available only when compiling outside the kernel.
 */
pub const SELNL_GRP_NONE: u32 = 0x00000000;
pub const SELNL_GRP_AVC: u32 = 0x00000001; /* AVC notifications */
pub const SELNL_GRP_ALL: u32 = 0xffffffff;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum selinux_nlgroups {
    SELNLGRP_NONE = 0,
    SELNLGRP_AVC,
    __SELNLGRP_MAX,
}

/* C preprocessor aliases: SELNLGRP_NONE and SELNLGRP_AVC map to the enum
 * members above. */
pub const SELNLGRP_MAX: i32 = selinux_nlgroups::__SELNLGRP_MAX as i32 - 1;

/* Message structures */
#[repr(C)]
pub struct selnl_msg_setenforce {
    pub val: i32,
}

#[repr(C)]
pub struct selnl_msg_policyload {
    pub seqno: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
