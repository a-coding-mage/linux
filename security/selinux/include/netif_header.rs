/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Network interface table.
 *
 * Network interfaces (devices) do not have a security field, so we
 * maintain a table associating each interface with a SID.
 *
 * Author: James Morris <jmorris@redhat.com>
 *
 * Copyright (C) 2003 Red Hat, Inc., James Morris <jmorris@redhat.com>
 * Copyright (C) 2007 Hewlett-Packard Development Company, L.P.
 *                    Paul Moore <paul@paul-moore.com>
 */

/* Dependency from C header: <net/net_namespace.h> provides `struct net`. */

use core::ffi::c_int;

unsafe extern "C" {
    pub fn sel_netif_flush();

    pub fn sel_netif_sid(ns: *mut net, ifindex: c_int, sid: *mut u32) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
