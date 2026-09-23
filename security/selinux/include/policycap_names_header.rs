// SPDX-License-Identifier: GPL-2.0
//! Policy capability names, indexed in the order defined by policycap.h.

/// Capabilities understood by this kernel, in policy capability number order.
pub(crate) const SELINUX_POLICYCAP_NAMES: &[&str] = &[
    "network_peer_controls",
    "open_perms",
    "extended_socket_class",
    "always_check_network",
    "cgroup_seclabel",
    "nnp_nosuid_transition",
    "genfs_seclabel_symlinks",
    "ioctl_skip_cloexec",
    "userspace_initial_context",
    "netlink_xperm",
    "netif_wildcard",
    "genfs_seclabel_wildcard",
    "functionfs_seclabel",
    "memfd_class",
    "bpf_token_perms",
];
