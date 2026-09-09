/* SPDX-License-Identifier: GPL-2.0-or-later */
/* AFS Cache Manager definitions
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C header guard: AFS_CM_H

pub const AFS_CM_PORT: u32 = 7001; // AFS file server port
pub const CM_SERVICE: u32 = 1; // AFS File Service ID

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AFS_CM_Operations {
    CBCallBack = 204, // break callback promises
    CBInitCallBackState = 205, // initialise callback state
    CBProbe = 206, // probe client
    CBGetLock = 207, // get contents of CM lock table
    CBGetCE = 208, // get cache file description
    CBGetXStatsVersion = 209, // get version of extended statistics
    CBGetXStats = 210, // get contents of extended statistics data
    CBInitCallBackState3 = 213, // initialise callback state, version 3
    CBProbeUuid = 214, // check the client hasn't rebooted
    CBTellMeAboutYourself = 65538, // get client capabilities
}

pub const AFS_CAP_ERROR_TRANSLATION: u32 = 0x1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
