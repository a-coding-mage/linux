/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
**  Copyright (C) 2004-2011 Red Hat, Inc.  All rights reserved.
**
**  This copyrighted material is made available to anyone wishing to use,
**  modify, copy, or redistribute it subject to the terms and conditions
**  of the GNU General Public License v.2.
**
*******************************************************************************
******************************************************************************/

/* Interface to Distributed Lock Manager (DLM) routines and structures. */
/* Lock levels and flags are supplied by linux/dlmconstants.h. */
/* Integer types are supplied by linux/types.h. */

use core::ffi::{c_char, c_void};

pub type dlm_lockspace_t = c_void;

/*
 * Lock status block
 *
 * Use this structure to specify the contents of the lock value block.  For a
 * conversion request, this structure is used to specify the lock ID of the
 * lock.  DLM writes the status of the lock request and the lock ID assigned
 * to the request in the lock status block.
 *
 * sb_lkid: the returned lock ID.  It is set on new (non-conversion) requests.
 * It is available when dlm_lock returns.
 *
 * sb_lvbptr: saves or returns the contents of the lock's LVB according to rules
 * shown for the DLM_LKF_VALBLK flag.
 *
 * sb_flags: DLM_SBF_DEMOTED is returned if in the process of promoting a lock,
 * it was first demoted to NL to avoid conversion deadlock.
 * DLM_SBF_VALNOTVALID is returned if the resource's LVB is marked invalid.
 *
 * sb_status: the returned status of the lock request set prior to AST
 * execution.  Possible return values:
 *
 * 0 if lock request was successful
 * -EAGAIN if request would block and is flagged DLM_LKF_NOQUEUE
 * -DLM_EUNLOCK if unlock request was successful
 * -DLM_ECANCEL if a cancel completed successfully
 * -EDEADLK if a deadlock was detected
 * -ETIMEDOUT if the lock request was canceled due to a timeout
 */

pub const DLM_SBF_DEMOTED: u8 = 0x01;
pub const DLM_SBF_VALNOTVALID: u8 = 0x02;
pub const DLM_SBF_ALTMODE: u8 = 0x04;

#[repr(C)]
pub struct dlm_lksb {
    pub sb_status: i32,
    pub sb_lkid: u32,
    pub sb_flags: c_char,
    pub sb_lvbptr: *mut c_char,
}

/* dlm_new_lockspace() flags */

/* DLM_LSFL_TIMEWARN is deprecated and reserved. DO NOT USE! */
pub const DLM_LSFL_TIMEWARN: u32 = 0x00000002;
pub const DLM_LSFL_NEWEXCL: u32 = 0x00000008;
/* currently reserved due in-kernel use */
pub const __DLM_LSFL_RESERVED0: u32 = 0x00000010;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
