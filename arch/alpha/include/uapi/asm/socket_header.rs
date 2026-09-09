/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* For setsockopt(2) */
/*
 * Note: we only bother about making the SOL_SOCKET options
 * same as OSF/1, as that's all that "normal" programs are
 * likely to set.  We don't necessarily want to be binary
 * compatible with _everything_.
 */
pub const SOL_SOCKET: u32 = 0xffff;

pub const SO_DEBUG: u32 = 0x0001;
pub const SO_REUSEADDR: u32 = 0x0004;
pub const SO_KEEPALIVE: u32 = 0x0008;
pub const SO_DONTROUTE: u32 = 0x0010;
pub const SO_BROADCAST: u32 = 0x0020;
pub const SO_LINGER: u32 = 0x0080;
pub const SO_OOBINLINE: u32 = 0x0100;
pub const SO_REUSEPORT: u32 = 0x0200;

pub const SO_TYPE: u32 = 0x1008;
pub const SO_ERROR: u32 = 0x1007;
pub const SO_SNDBUF: u32 = 0x1001;
pub const SO_RCVBUF: u32 = 0x1002;
pub const SO_SNDBUFFORCE: u32 = 0x100a;
pub const SO_RCVBUFFORCE: u32 = 0x100b;
pub const SO_RCVLOWAT: u32 = 0x1010;
pub const SO_SNDLOWAT: u32 = 0x1011;
pub const SO_RCVTIMEO_OLD: u32 = 0x1012;
pub const SO_SNDTIMEO_OLD: u32 = 0x1013;
pub const SO_ACCEPTCONN: u32 = 0x1014;
pub const SO_PROTOCOL: u32 = 0x1028;
pub const SO_DOMAIN: u32 = 0x1029;

/* linux-specific, might as well be the same as on i386 */
pub const SO_NO_CHECK: u32 = 11;
pub const SO_PRIORITY: u32 = 12;
pub const SO_BSDCOMPAT: u32 = 14;
pub const SO_PASSCRED: u32 = 17;
pub const SO_PEERCRED: u32 = 18;
pub const SO_BINDTODEVICE: u32 = 25;

/* Socket filtering */
pub const SO_ATTACH_FILTER: u32 = 26;
pub const SO_DETACH_FILTER: u32 = 27;
pub const SO_GET_FILTER: u32 = SO_ATTACH_FILTER;
pub const SO_PEERNAME: u32 = 28;
pub const SO_TIMESTAMP_OLD: u32 = 29;
pub const SO_PEERSEC: u32 = 30;
pub const SO_PASSSEC: u32 = 34;

/* Security levels - as per NRL IPv6 - don't actually do anything */
pub const SO_SECURITY_AUTHENTICATION: u32 = 19;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: u32 = 20;
pub const SO_SECURITY_ENCRYPTION_NETWORK: u32 = 21;
pub const SO_TIMESTAMPNS_OLD: u32 = 35;
pub const SO_MARK: u32 = 36;
pub const SO_TIMESTAMPING_OLD: u32 = 37;
pub const SO_RXQ_OVFL: u32 = 40;
pub const SO_WIFI_STATUS: u32 = 41;
pub const SCM_WIFI_STATUS: u32 = SO_WIFI_STATUS;
pub const SO_PEEK_OFF: u32 = 42;

/* Instruct lower device to use last 4-bytes of skb data as FCS */
pub const SO_NOFCS: u32 = 43;
pub const SO_LOCK_FILTER: u32 = 44;
pub const SO_SELECT_ERR_QUEUE: u32 = 45;
pub const SO_BUSY_POLL: u32 = 46;
pub const SO_MAX_PACING_RATE: u32 = 47;
pub const SO_BPF_EXTENSIONS: u32 = 48;
pub const SO_INCOMING_CPU: u32 = 49;
pub const SO_ATTACH_BPF: u32 = 50;
pub const SO_DETACH_BPF: u32 = SO_DETACH_FILTER;
pub const SO_ATTACH_REUSEPORT_CBPF: u32 = 51;
pub const SO_ATTACH_REUSEPORT_EBPF: u32 = 52;
pub const SO_CNX_ADVICE: u32 = 53;
pub const SCM_TIMESTAMPING_OPT_STATS: u32 = 54;
pub const SO_MEMINFO: u32 = 55;
pub const SO_INCOMING_NAPI_ID: u32 = 56;
pub const SO_COOKIE: u32 = 57;
pub const SCM_TIMESTAMPING_PKTINFO: u32 = 58;
pub const SO_PEERGROUPS: u32 = 59;
pub const SO_ZEROCOPY: u32 = 60;
pub const SO_TXTIME: u32 = 61;
pub const SCM_TXTIME: u32 = SO_TXTIME;
pub const SO_BINDTOIFINDEX: u32 = 62;
pub const SO_TIMESTAMP_NEW: u32 = 63;
pub const SO_TIMESTAMPNS_NEW: u32 = 64;
pub const SO_TIMESTAMPING_NEW: u32 = 65;
pub const SO_RCVTIMEO_NEW: u32 = 66;
pub const SO_SNDTIMEO_NEW: u32 = 67;
pub const SO_DETACH_REUSEPORT_BPF: u32 = 68;
pub const SO_PREFER_BUSY_POLL: u32 = 69;
pub const SO_BUSY_POLL_BUDGET: u32 = 70;
pub const SO_NETNS_COOKIE: u32 = 71;
pub const SO_BUF_LOCK: u32 = 72;
pub const SO_RESERVE_MEM: u32 = 73;
pub const SO_TXREHASH: u32 = 74;
pub const SO_RCVMARK: u32 = 75;
pub const SO_PASSPIDFD: u32 = 76;
pub const SO_PEERPIDFD: u32 = 77;
pub const SO_DEVMEM_LINEAR: u32 = 78;
pub const SCM_DEVMEM_LINEAR: u32 = SO_DEVMEM_LINEAR;
pub const SO_DEVMEM_DMABUF: u32 = 79;
pub const SCM_DEVMEM_DMABUF: u32 = SO_DEVMEM_DMABUF;
pub const SO_DEVMEM_DONTNEED: u32 = 80;
pub const SCM_TS_OPT_ID: u32 = 81;
pub const SO_RCVPRIORITY: u32 = 82;
pub const SO_PASSRIGHTS: u32 = 83;
pub const SO_INQ: u32 = 84;
pub const SCM_INQ: u32 = SO_INQ;
pub const SO_RIGHTS_NOTRUNC: u32 = 85;

/* The C header excludes these aliases for kernel builds. */
#[cfg(not(feature = "kernel"))]
pub const SO_TIMESTAMP: u32 = if cfg!(target_pointer_width = "64") {
    SO_TIMESTAMP_OLD
} else {
    // On non-64-bit targets C selects OLD or NEW by comparing time_t and __kernel_long_t sizes.
    SO_TIMESTAMP_NEW
};
#[cfg(not(feature = "kernel"))]
pub const SO_TIMESTAMPNS: u32 = if cfg!(target_pointer_width = "64") {
    SO_TIMESTAMPNS_OLD
} else {
    SO_TIMESTAMPNS_NEW
};
#[cfg(not(feature = "kernel"))]
pub const SO_TIMESTAMPING: u32 = if cfg!(target_pointer_width = "64") {
    SO_TIMESTAMPING_OLD
} else {
    SO_TIMESTAMPING_NEW
};
#[cfg(not(feature = "kernel"))]
pub const SO_RCVTIMEO: u32 = if cfg!(target_pointer_width = "64") {
    SO_RCVTIMEO_OLD
} else {
    SO_RCVTIMEO_NEW
};
#[cfg(not(feature = "kernel"))]
pub const SO_SNDTIMEO: u32 = if cfg!(target_pointer_width = "64") {
    SO_SNDTIMEO_OLD
} else {
    SO_SNDTIMEO_NEW
};

#[cfg(not(feature = "kernel"))]
pub const SCM_TIMESTAMP: u32 = SO_TIMESTAMP;
#[cfg(not(feature = "kernel"))]
pub const SCM_TIMESTAMPNS: u32 = SO_TIMESTAMPNS;
#[cfg(not(feature = "kernel"))]
pub const SCM_TIMESTAMPING: u32 = SO_TIMESTAMPING;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
