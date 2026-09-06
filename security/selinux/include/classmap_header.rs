/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct security_class_mapping {
    pub name: Option<&'static [u8]>,
    pub perms: &'static [Option<&'static [u8]>],
}

macro_rules! s {
    ($name:literal) => {
        Some(concat!($name, "\0").as_bytes())
    };
}

macro_rules! common_file_sock_perms {
    () => {
        s!("ioctl"), s!("read"), s!("write"), s!("create"), s!("getattr"), s!("setattr"),
        s!("lock"), s!("relabelfrom"), s!("relabelto"), s!("append"), s!("map")
    };
}

macro_rules! common_file_perms {
    () => {
        common_file_sock_perms!(), s!("unlink"), s!("link"), s!("rename"), s!("execute"),
        s!("quotaon"), s!("mounton"), s!("audit_access"), s!("open"), s!("execmod"),
        s!("watch"), s!("watch_mount"), s!("watch_sb"), s!("watch_with_perm"),
        s!("watch_reads"), s!("watch_mountns")
    };
}

macro_rules! common_sock_perms {
    () => {
        common_file_sock_perms!(), s!("bind"), s!("connect"), s!("listen"), s!("accept"),
        s!("getopt"), s!("setopt"), s!("shutdown"), s!("recvfrom"), s!("sendto"),
        s!("name_bind")
    };
}

macro_rules! common_ipc_perms {
    () => {
        s!("create"), s!("destroy"), s!("getattr"), s!("setattr"), s!("read"), s!("write"),
        s!("associate"), s!("unix_read"), s!("unix_write")
    };
}

macro_rules! common_cap_perms {
    () => {
        s!("chown"), s!("dac_override"), s!("dac_read_search"), s!("fowner"), s!("fsetid"),
        s!("kill"), s!("setgid"), s!("setuid"), s!("setpcap"), s!("linux_immutable"),
        s!("net_bind_service"), s!("net_broadcast"), s!("net_admin"), s!("net_raw"),
        s!("ipc_lock"), s!("ipc_owner"), s!("sys_module"), s!("sys_rawio"),
        s!("sys_chroot"), s!("sys_ptrace"), s!("sys_pacct"), s!("sys_admin"),
        s!("sys_boot"), s!("sys_nice"), s!("sys_resource"), s!("sys_time"),
        s!("sys_tty_config"), s!("mknod"), s!("lease"), s!("audit_write"),
        s!("audit_control"), s!("setfcap")
    };
}

macro_rules! common_cap2_perms {
    () => {
        s!("mac_override"), s!("mac_admin"), s!("syslog"), s!("wake_alarm"),
        s!("block_suspend"), s!("audit_read"), s!("perfmon"), s!("bpf"),
        s!("checkpoint_restore")
    };
}

/*
 * In the C header this check is enabled only under __KERNEL__:
 * include <linux/capability.h>
 * if CAP_LAST_CAP > CAP_CHECKPOINT_RESTORE, fail the build and update
 * COMMON_CAP2_PERMS.
 */

/*
 * Note: The name for any socket class should be suffixed by "socket",
 *	 and doesn't contain more than one substr of "socket".
 */
pub static secclass_map: [security_class_mapping; 108] = [
    security_class_mapping { name: s!("security"), perms: &[s!("compute_av"), s!("compute_create"), s!("compute_member"), s!("check_context"), s!("load_policy"), s!("compute_relabel"), s!("compute_user"), s!("setenforce"), s!("setbool"), s!("setsecparam"), s!("setcheckreqprot"), s!("read_policy"), s!("validate_trans"), None] },
    security_class_mapping { name: s!("process"), perms: &[s!("fork"), s!("transition"), s!("sigchld"), s!("sigkill"), s!("sigstop"), s!("signull"), s!("signal"), s!("ptrace"), s!("getsched"), s!("setsched"), s!("getsession"), s!("getpgid"), s!("setpgid"), s!("getcap"), s!("setcap"), s!("share"), s!("getattr"), s!("setexec"), s!("setfscreate"), s!("noatsecure"), s!("siginh"), s!("setrlimit"), s!("rlimitinh"), s!("dyntransition"), s!("setcurrent"), s!("execmem"), s!("execstack"), s!("execheap"), s!("setkeycreate"), s!("setsockcreate"), s!("getrlimit"), None] },
    security_class_mapping { name: s!("process2"), perms: &[s!("nnp_transition"), s!("nosuid_transition"), None] },
    security_class_mapping { name: s!("system"), perms: &[s!("ipc_info"), s!("syslog_read"), s!("syslog_mod"), s!("syslog_console"), s!("module_request"), s!("module_load"), s!("firmware_load"), s!("kexec_image_load"), s!("kexec_initramfs_load"), s!("policy_load"), s!("x509_certificate_load"), None] },
    security_class_mapping { name: s!("capability"), perms: &[common_cap_perms!(), None] },
    security_class_mapping { name: s!("filesystem"), perms: &[s!("mount"), s!("remount"), s!("unmount"), s!("getattr"), s!("relabelfrom"), s!("relabelto"), s!("associate"), s!("quotamod"), s!("quotaget"), s!("watch"), None] },
    security_class_mapping { name: s!("file"), perms: &[common_file_perms!(), s!("execute_no_trans"), s!("entrypoint"), None] },
    security_class_mapping { name: s!("dir"), perms: &[common_file_perms!(), s!("add_name"), s!("remove_name"), s!("reparent"), s!("search"), s!("rmdir"), None] },
    security_class_mapping { name: s!("fd"), perms: &[s!("use"), None] },
    security_class_mapping { name: s!("lnk_file"), perms: &[common_file_perms!(), None] },
    security_class_mapping { name: s!("chr_file"), perms: &[common_file_perms!(), None] },
    security_class_mapping { name: s!("blk_file"), perms: &[common_file_perms!(), None] },
    security_class_mapping { name: s!("sock_file"), perms: &[common_file_perms!(), None] },
    security_class_mapping { name: s!("fifo_file"), perms: &[common_file_perms!(), None] },
    security_class_mapping { name: s!("socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("tcp_socket"), perms: &[common_sock_perms!(), s!("node_bind"), s!("name_connect"), None] },
    security_class_mapping { name: s!("udp_socket"), perms: &[common_sock_perms!(), s!("node_bind"), None] },
    security_class_mapping { name: s!("rawip_socket"), perms: &[common_sock_perms!(), s!("node_bind"), None] },
    security_class_mapping { name: s!("node"), perms: &[s!("recvfrom"), s!("sendto"), None] },
    security_class_mapping { name: s!("netif"), perms: &[s!("ingress"), s!("egress"), None] },
    security_class_mapping { name: s!("netlink_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("packet_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("key_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("unix_stream_socket"), perms: &[common_sock_perms!(), s!("connectto"), None] },
    security_class_mapping { name: s!("unix_dgram_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("sem"), perms: &[common_ipc_perms!(), None] },
    security_class_mapping { name: s!("msg"), perms: &[s!("send"), s!("receive"), None] },
    security_class_mapping { name: s!("msgq"), perms: &[common_ipc_perms!(), s!("enqueue"), None] },
    security_class_mapping { name: s!("shm"), perms: &[common_ipc_perms!(), s!("lock"), None] },
    security_class_mapping { name: s!("ipc"), perms: &[common_ipc_perms!(), None] },
    security_class_mapping { name: s!("netlink_route_socket"), perms: &[common_sock_perms!(), s!("nlmsg_read"), s!("nlmsg_write"), s!("nlmsg"), None] },
    security_class_mapping { name: s!("netlink_tcpdiag_socket"), perms: &[common_sock_perms!(), s!("nlmsg_read"), s!("nlmsg_write"), s!("nlmsg"), None] },
    security_class_mapping { name: s!("netlink_nflog_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("netlink_xfrm_socket"), perms: &[common_sock_perms!(), s!("nlmsg_read"), s!("nlmsg_write"), s!("nlmsg"), None] },
    security_class_mapping { name: s!("netlink_selinux_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("netlink_iscsi_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("netlink_audit_socket"), perms: &[common_sock_perms!(), s!("nlmsg_read"), s!("nlmsg_write"), s!("nlmsg_relay"), s!("nlmsg_readpriv"), s!("nlmsg_tty_audit"), s!("nlmsg"), None] },
    security_class_mapping { name: s!("netlink_fib_lookup_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("netlink_connector_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("netlink_netfilter_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("netlink_dnrt_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("association"), perms: &[s!("sendto"), s!("recvfrom"), s!("setcontext"), s!("polmatch"), None] },
    security_class_mapping { name: s!("netlink_kobject_uevent_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("netlink_generic_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("netlink_scsitransport_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("netlink_rdma_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("netlink_crypto_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("appletalk_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("packet"), perms: &[s!("send"), s!("recv"), s!("relabelto"), s!("forward_in"), s!("forward_out"), None] },
    security_class_mapping { name: s!("key"), perms: &[s!("view"), s!("read"), s!("write"), s!("search"), s!("link"), s!("setattr"), s!("create"), None] },
    security_class_mapping { name: s!("memprotect"), perms: &[s!("mmap_zero"), None] },
    security_class_mapping { name: s!("peer"), perms: &[s!("recv"), None] },
    security_class_mapping { name: s!("capability2"), perms: &[common_cap2_perms!(), None] },
    security_class_mapping { name: s!("kernel_service"), perms: &[s!("use_as_override"), s!("create_files_as"), None] },
    security_class_mapping { name: s!("tun_socket"), perms: &[common_sock_perms!(), s!("attach_queue"), None] },
    security_class_mapping { name: s!("binder"), perms: &[s!("impersonate"), s!("call"), s!("set_context_mgr"), s!("transfer"), None] },
    security_class_mapping { name: s!("cap_userns"), perms: &[common_cap_perms!(), None] },
    security_class_mapping { name: s!("cap2_userns"), perms: &[common_cap2_perms!(), None] },
    security_class_mapping { name: s!("sctp_socket"), perms: &[common_sock_perms!(), s!("node_bind"), s!("name_connect"), s!("association"), None] },
    security_class_mapping { name: s!("icmp_socket"), perms: &[common_sock_perms!(), s!("node_bind"), None] },
    security_class_mapping { name: s!("ax25_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("ipx_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("netrom_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("atmpvc_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("x25_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("rose_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("decnet_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("atmsvc_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("rds_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("irda_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("pppox_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("llc_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("can_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("tipc_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("bluetooth_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("iucv_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("rxrpc_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("isdn_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("phonet_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("ieee802154_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("caif_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("alg_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("nfc_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("vsock_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("kcm_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("qipcrtr_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("smc_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("infiniband_pkey"), perms: &[s!("access"), None] },
    security_class_mapping { name: s!("infiniband_endport"), perms: &[s!("manage_subnet"), None] },
    security_class_mapping { name: s!("bpf"), perms: &[s!("map_create"), s!("map_read"), s!("map_write"), s!("prog_load"), s!("prog_run"), s!("map_create_as"), s!("prog_load_as"), None] },
    security_class_mapping { name: s!("xdp_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("mctp_socket"), perms: &[common_sock_perms!(), None] },
    security_class_mapping { name: s!("perf_event"), perms: &[s!("open"), s!("cpu"), s!("kernel"), s!("tracepoint"), s!("read"), s!("write"), None] },
    security_class_mapping { name: s!("anon_inode"), perms: &[common_file_perms!(), None] },
    security_class_mapping { name: s!("io_uring"), perms: &[s!("override_creds"), s!("sqpoll"), s!("cmd"), s!("allowed"), None] },
    security_class_mapping { name: s!("user_namespace"), perms: &[s!("create"), None] },
    security_class_mapping { name: s!("memfd_file"), perms: &[common_file_perms!(), s!("execute_no_trans"), s!("entrypoint"), None] },
    /* last one */ security_class_mapping { name: None, perms: &[] },
];

/*
 * In the C header this check is enabled only under __KERNEL__:
 * include <linux/socket.h>
 * if PF_MAX > 46, fail the build and update secclass_map.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
