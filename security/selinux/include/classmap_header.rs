// SPDX-License-Identifier: GPL-2.0
//! SELinux security classes and permissions, in their policy ABI order.

/// One security class and its ordered permission names.
pub(crate) struct SecurityClassMapping {
    /// The policy language name of this class.
    pub(crate) name: &'static str,
    /// Permission bit names, starting with the least significant bit.
    pub(crate) permissions: &'static [&'static str],
}

const COMMON_FILE_SOCK_PERMS: &[&str] = &[
    "ioctl",
    "read",
    "write",
    "create",
    "getattr",
    "setattr",
    "lock",
    "relabelfrom",
    "relabelto",
    "append",
    "map",
];

const fn append<const N: usize>(
    first: &[&'static str],
    second: &[&'static str],
) -> [&'static str; N] {
    assert!(N == first.len() + second.len());
    assert!(N <= u32::BITS as usize);
    let mut result = [""; N];
    let mut index = 0;
    while index < first.len() {
        result[index] = first[index];
        index += 1;
    }
    while index < N {
        result[index] = second[index - first.len()];
        index += 1;
    }
    result
}

const COMMON_FILE_PERMS: &[&str] = &append::<26>(
    COMMON_FILE_SOCK_PERMS,
    &[
        "unlink",
        "link",
        "rename",
        "execute",
        "quotaon",
        "mounton",
        "audit_access",
        "open",
        "execmod",
        "watch",
        "watch_mount",
        "watch_sb",
        "watch_with_perm",
        "watch_reads",
        "watch_mountns",
    ],
);
const COMMON_SOCK_PERMS: &[&str] = &append::<21>(
    COMMON_FILE_SOCK_PERMS,
    &[
        "bind",
        "connect",
        "listen",
        "accept",
        "getopt",
        "setopt",
        "shutdown",
        "recvfrom",
        "sendto",
        "name_bind",
    ],
);
const COMMON_IPC_PERMS: &[&str] = &[
    "create",
    "destroy",
    "getattr",
    "setattr",
    "read",
    "write",
    "associate",
    "unix_read",
    "unix_write",
];
const COMMON_CAP_PERMS: &[&str] = &[
    "chown",
    "dac_override",
    "dac_read_search",
    "fowner",
    "fsetid",
    "kill",
    "setgid",
    "setuid",
    "setpcap",
    "linux_immutable",
    "net_bind_service",
    "net_broadcast",
    "net_admin",
    "net_raw",
    "ipc_lock",
    "ipc_owner",
    "sys_module",
    "sys_rawio",
    "sys_chroot",
    "sys_ptrace",
    "sys_pacct",
    "sys_admin",
    "sys_boot",
    "sys_nice",
    "sys_resource",
    "sys_time",
    "sys_tty_config",
    "mknod",
    "lease",
    "audit_write",
    "audit_control",
    "setfcap",
];
const COMMON_CAP2_PERMS: &[&str] = &[
    "mac_override",
    "mac_admin",
    "syslog",
    "wake_alarm",
    "block_suspend",
    "audit_read",
    "perfmon",
    "bpf",
    "checkpoint_restore",
];

macro_rules! class {
    ($name:literal, [$($permission:literal),* $(,)?]) => {
        SecurityClassMapping { name: $name, permissions: &[$($permission),*] }
    };
    ($name:literal, $common:ident $(, $permission:literal)* $(,)?) => {{
        const EXTRA: &[&str] = &[$($permission),*];
        SecurityClassMapping {
            name: $name,
            permissions: &append::<{ $common.len() + EXTRA.len() }>($common, EXTRA),
        }
    }};
}

/// Classes in security class number order (the first class has number one).
/// Socket class names contain exactly one `socket` suffix.
pub(crate) const SECCLASS_MAP: &[SecurityClassMapping] = &[
    class!(
        "security",
        [
            "compute_av",
            "compute_create",
            "compute_member",
            "check_context",
            "load_policy",
            "compute_relabel",
            "compute_user",
            "setenforce",
            "setbool",
            "setsecparam",
            "setcheckreqprot",
            "read_policy",
            "validate_trans"
        ]
    ),
    class!(
        "process",
        [
            "fork",
            "transition",
            "sigchld",
            "sigkill",
            "sigstop",
            "signull",
            "signal",
            "ptrace",
            "getsched",
            "setsched",
            "getsession",
            "getpgid",
            "setpgid",
            "getcap",
            "setcap",
            "share",
            "getattr",
            "setexec",
            "setfscreate",
            "noatsecure",
            "siginh",
            "setrlimit",
            "rlimitinh",
            "dyntransition",
            "setcurrent",
            "execmem",
            "execstack",
            "execheap",
            "setkeycreate",
            "setsockcreate",
            "getrlimit"
        ]
    ),
    class!("process2", ["nnp_transition", "nosuid_transition"]),
    class!(
        "system",
        [
            "ipc_info",
            "syslog_read",
            "syslog_mod",
            "syslog_console",
            "module_request",
            "module_load",
            "firmware_load",
            "kexec_image_load",
            "kexec_initramfs_load",
            "policy_load",
            "x509_certificate_load"
        ]
    ),
    class!("capability", COMMON_CAP_PERMS),
    class!(
        "filesystem",
        [
            "mount",
            "remount",
            "unmount",
            "getattr",
            "relabelfrom",
            "relabelto",
            "associate",
            "quotamod",
            "quotaget",
            "watch"
        ]
    ),
    class!("file", COMMON_FILE_PERMS, "execute_no_trans", "entrypoint"),
    class!(
        "dir",
        COMMON_FILE_PERMS,
        "add_name",
        "remove_name",
        "reparent",
        "search",
        "rmdir"
    ),
    class!("fd", ["use"]),
    class!("lnk_file", COMMON_FILE_PERMS),
    class!("chr_file", COMMON_FILE_PERMS),
    class!("blk_file", COMMON_FILE_PERMS),
    class!("sock_file", COMMON_FILE_PERMS),
    class!("fifo_file", COMMON_FILE_PERMS),
    class!("socket", COMMON_SOCK_PERMS),
    class!("tcp_socket", COMMON_SOCK_PERMS, "node_bind", "name_connect"),
    class!("udp_socket", COMMON_SOCK_PERMS, "node_bind"),
    class!("rawip_socket", COMMON_SOCK_PERMS, "node_bind"),
    class!("node", ["recvfrom", "sendto"]),
    class!("netif", ["ingress", "egress"]),
    class!("netlink_socket", COMMON_SOCK_PERMS),
    class!("packet_socket", COMMON_SOCK_PERMS),
    class!("key_socket", COMMON_SOCK_PERMS),
    class!("unix_stream_socket", COMMON_SOCK_PERMS, "connectto"),
    class!("unix_dgram_socket", COMMON_SOCK_PERMS),
    class!("sem", COMMON_IPC_PERMS),
    class!("msg", ["send", "receive"]),
    class!("msgq", COMMON_IPC_PERMS, "enqueue"),
    class!("shm", COMMON_IPC_PERMS, "lock"),
    class!("ipc", COMMON_IPC_PERMS),
    class!(
        "netlink_route_socket",
        COMMON_SOCK_PERMS,
        "nlmsg_read",
        "nlmsg_write",
        "nlmsg"
    ),
    class!(
        "netlink_tcpdiag_socket",
        COMMON_SOCK_PERMS,
        "nlmsg_read",
        "nlmsg_write",
        "nlmsg"
    ),
    class!("netlink_nflog_socket", COMMON_SOCK_PERMS),
    class!(
        "netlink_xfrm_socket",
        COMMON_SOCK_PERMS,
        "nlmsg_read",
        "nlmsg_write",
        "nlmsg"
    ),
    class!("netlink_selinux_socket", COMMON_SOCK_PERMS),
    class!("netlink_iscsi_socket", COMMON_SOCK_PERMS),
    class!(
        "netlink_audit_socket",
        COMMON_SOCK_PERMS,
        "nlmsg_read",
        "nlmsg_write",
        "nlmsg_relay",
        "nlmsg_readpriv",
        "nlmsg_tty_audit",
        "nlmsg"
    ),
    class!("netlink_fib_lookup_socket", COMMON_SOCK_PERMS),
    class!("netlink_connector_socket", COMMON_SOCK_PERMS),
    class!("netlink_netfilter_socket", COMMON_SOCK_PERMS),
    class!("netlink_dnrt_socket", COMMON_SOCK_PERMS),
    class!(
        "association",
        ["sendto", "recvfrom", "setcontext", "polmatch"]
    ),
    class!("netlink_kobject_uevent_socket", COMMON_SOCK_PERMS),
    class!("netlink_generic_socket", COMMON_SOCK_PERMS),
    class!("netlink_scsitransport_socket", COMMON_SOCK_PERMS),
    class!("netlink_rdma_socket", COMMON_SOCK_PERMS),
    class!("netlink_crypto_socket", COMMON_SOCK_PERMS),
    class!("appletalk_socket", COMMON_SOCK_PERMS),
    class!(
        "packet",
        ["send", "recv", "relabelto", "forward_in", "forward_out"]
    ),
    class!(
        "key",
        ["view", "read", "write", "search", "link", "setattr", "create"]
    ),
    class!("memprotect", ["mmap_zero"]),
    class!("peer", ["recv"]),
    class!("capability2", COMMON_CAP2_PERMS),
    class!("kernel_service", ["use_as_override", "create_files_as"]),
    class!("tun_socket", COMMON_SOCK_PERMS, "attach_queue"),
    class!(
        "binder",
        ["impersonate", "call", "set_context_mgr", "transfer"]
    ),
    class!("cap_userns", COMMON_CAP_PERMS),
    class!("cap2_userns", COMMON_CAP2_PERMS),
    class!(
        "sctp_socket",
        COMMON_SOCK_PERMS,
        "node_bind",
        "name_connect",
        "association"
    ),
    class!("icmp_socket", COMMON_SOCK_PERMS, "node_bind"),
    class!("ax25_socket", COMMON_SOCK_PERMS),
    class!("ipx_socket", COMMON_SOCK_PERMS),
    class!("netrom_socket", COMMON_SOCK_PERMS),
    class!("atmpvc_socket", COMMON_SOCK_PERMS),
    class!("x25_socket", COMMON_SOCK_PERMS),
    class!("rose_socket", COMMON_SOCK_PERMS),
    class!("decnet_socket", COMMON_SOCK_PERMS),
    class!("atmsvc_socket", COMMON_SOCK_PERMS),
    class!("rds_socket", COMMON_SOCK_PERMS),
    class!("irda_socket", COMMON_SOCK_PERMS),
    class!("pppox_socket", COMMON_SOCK_PERMS),
    class!("llc_socket", COMMON_SOCK_PERMS),
    class!("can_socket", COMMON_SOCK_PERMS),
    class!("tipc_socket", COMMON_SOCK_PERMS),
    class!("bluetooth_socket", COMMON_SOCK_PERMS),
    class!("iucv_socket", COMMON_SOCK_PERMS),
    class!("rxrpc_socket", COMMON_SOCK_PERMS),
    class!("isdn_socket", COMMON_SOCK_PERMS),
    class!("phonet_socket", COMMON_SOCK_PERMS),
    class!("ieee802154_socket", COMMON_SOCK_PERMS),
    class!("caif_socket", COMMON_SOCK_PERMS),
    class!("alg_socket", COMMON_SOCK_PERMS),
    class!("nfc_socket", COMMON_SOCK_PERMS),
    class!("vsock_socket", COMMON_SOCK_PERMS),
    class!("kcm_socket", COMMON_SOCK_PERMS),
    class!("qipcrtr_socket", COMMON_SOCK_PERMS),
    class!("smc_socket", COMMON_SOCK_PERMS),
    class!("infiniband_pkey", ["access"]),
    class!("infiniband_endport", ["manage_subnet"]),
    class!(
        "bpf",
        [
            "map_create",
            "map_read",
            "map_write",
            "prog_load",
            "prog_run",
            "map_create_as",
            "prog_load_as"
        ]
    ),
    class!("xdp_socket", COMMON_SOCK_PERMS),
    class!("mctp_socket", COMMON_SOCK_PERMS),
    class!(
        "perf_event",
        ["open", "cpu", "kernel", "tracepoint", "read", "write"]
    ),
    class!("anon_inode", COMMON_FILE_PERMS),
    class!("io_uring", ["override_creds", "sqpoll", "cmd", "allowed"]),
    class!("user_namespace", ["create"]),
    class!(
        "memfd_file",
        COMMON_FILE_PERMS,
        "execute_no_trans",
        "entrypoint"
    ),
];
