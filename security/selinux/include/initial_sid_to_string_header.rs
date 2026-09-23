// SPDX-License-Identifier: GPL-2.0
//! Initial security identifiers. Empty slots retain their assigned SID numbers.

/// Initial SID names indexed by SID, including unused entries and slot zero.
pub(crate) const INITIAL_SID_TO_STRING: [Option<&str>; 28] = [
    None, // zero placeholder, not used
    Some("kernel"),
    Some("security"),
    Some("unlabeled"),
    None, // fs
    Some("file"),
    None, // file_labels
    Some("init"),
    Some("any_socket"),
    Some("port"),
    Some("netif"),
    Some("netmsg"),
    Some("node"),
    None, // igmp_packet
    None, // icmp_socket
    None, // tcp_socket
    None, // sysctl_modprobe
    None, // sysctl
    None, // sysctl_fs
    None, // sysctl_kernel
    None, // sysctl_net
    None, // sysctl_net_unix
    None, // sysctl_vm
    None, // sysctl_dev
    None, // kmod
    None, // policy
    None, // scmp_packet
    Some("devnull"),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
