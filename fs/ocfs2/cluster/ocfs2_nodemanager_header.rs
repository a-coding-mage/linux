/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * ocfs2_nodemanager.h
 *
 * Header describing the interface between userspace and the kernel
 * for the ocfs2_nodemanager module.
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// C header guard: _OCFS2_NODEMANAGER_H

pub const O2NM_API_VERSION: u32 = 5;

pub const O2NM_MAX_NODES: u32 = 255;
pub const O2NM_INVALID_NODE_NUM: u32 = 255;

/* host name, group name, cluster name all 64 bytes */
pub const O2NM_MAX_NAME_LEN: u32 = 64; // __NEW_UTS_LEN

/*
 * Maximum number of global heartbeat regions allowed.
 * **CAUTION**  Changing this number will break dlm compatibility.
 */
pub const O2NM_MAX_REGIONS: u32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
