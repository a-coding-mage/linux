/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2005-2008 Red Hat, Inc.  All rights reserved.
 *
 * This copyrighted material is made available to anyone wishing to use,
 * modify, copy, or redistribute it subject to the terms and conditions
 * of the GNU General Public License v.2.
 */

// C dependency: <linux/types.h>

pub const DLM_PLOCK_MISC_NAME: &str = "dlm_plock";

pub const DLM_PLOCK_VERSION_MAJOR: u32 = 1;
pub const DLM_PLOCK_VERSION_MINOR: u32 = 2;
pub const DLM_PLOCK_VERSION_PATCH: u32 = 0;

pub const DLM_PLOCK_OP_LOCK: u32 = 1;
pub const DLM_PLOCK_OP_UNLOCK: u32 = 2;
pub const DLM_PLOCK_OP_GET: u32 = 3;
pub const DLM_PLOCK_OP_CANCEL: u32 = 4;

pub const DLM_PLOCK_FL_CLOSE: u32 = 1;

#[repr(C)]
pub struct dlm_plock_info {
    pub version: [u32; 3],
    pub optype: u8,
    pub ex: u8,
    pub wait: u8,
    pub flags: u8,
    pub pid: u32,
    pub nodeid: i32,
    pub rv: i32,
    pub fsid: u32,
    pub number: u64,
    pub start: u64,
    pub end: u64,
    pub owner: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
