/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mpls in net namespaces
 */

// Translated from the C header. The Linux type and synchronization
// definitions are supplied by other dependencies.

pub struct netns_mpls {
    pub ip_ttl_propagate: ::core::ffi::c_int,
    pub default_ttl: ::core::ffi::c_int,
    pub platform_labels: usize,
    pub platform_label: *mut *mut mpls_route,
    pub platform_mutex: mutex,
    pub platform_label_seq: seqcount_mutex_t,
    pub ctl: *mut ctl_table_header,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
