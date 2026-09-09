/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2005 Oracle.  All rights reserved.
 */

// Declarations translated from O2CLUSTER_QUORUM_H.

unsafe extern "C" {
    pub fn o2quo_init();
    pub fn o2quo_exit();

    pub fn o2quo_hb_up(node: u8);
    pub fn o2quo_hb_down(node: u8);
    pub fn o2quo_hb_still_up(node: u8);
    pub fn o2quo_conn_up(node: u8);
    pub fn o2quo_conn_err(node: u8);
    pub fn o2quo_disk_timeout();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
