/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the translated netfilter definitions.

// Forward declarations from the surrounding kernel interfaces.
pub struct proc_dir_entry;
pub struct nf_logger;
pub struct nf_queue_handler;
pub struct ctl_table_header;
pub struct nf_hook_entries;

#[repr(C)]
pub struct netns_nf {
    #[cfg(defined_config_proc_fs)]
    pub proc_netfilter: *mut proc_dir_entry,

    pub nf_loggers: [*const nf_logger; NFPROTO_NUMPROTO],

    #[cfg(defined_config_sysctl)]
    pub nf_log_dir_header: *mut ctl_table_header,

    #[cfg(all(defined_config_sysctl, defined_config_lwtnel))]
    pub nf_lwtnl_dir_header: *mut ctl_table_header,

    pub hooks_ipv4: [*mut nf_hook_entries; NF_INET_NUMHOOKS],
    pub hooks_ipv6: [*mut nf_hook_entries; NF_INET_NUMHOOKS],

    #[cfg(defined_config_netfilter_family_arp)]
    pub hooks_arp: [*mut nf_hook_entries; NF_ARP_NUMHOOKS],

    #[cfg(defined_config_netfilter_family_bridge)]
    pub hooks_bridge: [*mut nf_hook_entries; NF_INET_NUMHOOKS],

    #[cfg(is_enabled_config_nf_defrag_ipv4)]
    pub defrag_ipv4_users: ::core::ffi::c_uint,

    #[cfg(is_enabled_config_nf_defrag_ipv6)]
    pub defrag_ipv6_users: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
