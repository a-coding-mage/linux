/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ieee802154 6lowpan in net namespaces
 */

// Dependency supplied by net/inet_frag.h.

#[repr(C)]
pub struct netns_sysctl_lowpan {
    #[cfg(CONFIG_SYSCTL)]
    pub frags_hdr: *mut ctl_table_header,
}

#[repr(C)]
pub struct netns_ieee802154_lowpan {
    pub sysctl: netns_sysctl_lowpan,
    pub fqdir: *mut fqdir,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
