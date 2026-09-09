/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * ocfs2_heartbeat.h
 *
 * On-disk structures for ocfs2_heartbeat
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// Header guard: _OCFS2_HEARTBEAT_H

#[repr(C)]
pub struct o2hb_disk_heartbeat_block {
    pub hb_seq: __le64,
    pub hb_node: __u8,
    pub hb_pad1: [__u8; 3],
    pub hb_cksum: __le32,
    pub hb_generation: __le64,
    pub hb_dead_ms: __le32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
