/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ocfs2_lockingver.h
 *
 * Defines OCFS2 Locking version values.
 *
 * Copyright (C) 2008 Oracle.  All rights reserved.
 */

/*
 * The protocol version for ocfs2 cluster locking.  See dlmglue.c for
 * more details.
 *
 * 1.0 - Initial locking version from ocfs2 1.4.
 */
pub const OCFS2_LOCKING_PROTOCOL_MAJOR: u32 = 1;
pub const OCFS2_LOCKING_PROTOCOL_MINOR: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
