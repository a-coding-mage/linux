/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C header dependencies: <linux/posix_types.h>, <asm/sockios.h>

/* For setsockopt(2) */
pub const SOL_SOCKET: u32 = 0xffff;

pub const SO_DEBUG: u32 = 0x0001;
pub const SO_PASSCRED: u32 = 0x0002;
pub const SO_REUSEADDR: u32 = 0x0004;
pub const SO_KEEPALIVE: u32 = 0x0008;
pub const SO_DONTROUTE: u32 = 0x0010;
pub const SO_BROADCAST: u32 = 0x0020;
pub const SO_PEERCRED: u32 = 0x0040;
pub const SO_LINGER: u32 = 0x0080;
pub const SO_OOBINLINE: u32 = 0x0100;
pub const SO_REUSEPORT: u32 = 0x0200;
pub const SO_BSDCOMPAT: u32 = 0x0400;
pub const SO_RCVLOWAT: u32 = 0x0800;
pub const SO_SNDLOWAT: u32 = 0x1000;
pub const SO_RCVTIMEO_OLD: u32 = 0x2000;
pub const SO_SNDTIMEO_OLD: u32 = 0x4000;
pub const SO_ACCEPTCONN: u32 = 0x8000;

pub const SO_SNDBUF: u32 = 0x1001;
pub const SO_RCVBUF: u32 = 0x1002;
pub const SO_SNDBUFFORCE: u32 = 0x100a;
pub const SO_RCVBUFFORCE: u32 = 0x100b;
pub const SO_ERROR: u32 = 0x1007;
pub const SO_TYPE: u32 = 0x1008;
pub const SO_PROTOCOL: u32 = 0x1028;
pub const SO_DOMAIN: u32 = 0x1029;

/* Linux specific, keep the same. */
pub const SO_NO_CHECK: u32 = 0x000b;
pub const SO_PRIORITY: u32 = 0x000c;
pub const SO_BINDTODEVICE: u32 = 0x000d;
pub const SO_ATTACH_FILTER: u32 = 0x001a;
pub const SO_DETACH_FILTER: u32 = 0x001b;
pub const SO_GET_FILTER: u32 = SO_ATTACH_FILTER;
pub const SO_PEERNAME: u32 = 0x001c;
pub const SO_PEERSEC: u32 = 0x001e;
pub const SO_PASSSEC: u32 = 0x001f;
pub const SO_MARK: u32 = 0x0022;
pub const SO_RXQ_OVFL: u32 = 0x0024;
pub const SO_WIFI_STATUS: u32 = 0x0025;
pub const SCM_WIFI_STATUS: u32 = SO_WIFI_STATUS;
pub const SO_PEEK_OFF: u32 = 0x0026;

/* Instruct lower device to use last 4-bytes of skb data as FCS */
pub const SO_NOFCS: u32 = 0x0027;
pub const SO_LOCK_FILTER: u32 = 0x0028;
pub const SO_SELECT_ERR_QUEUE: u32 = 0x0029;
pub const SO_BUSY_POLL: u32 = 0x0030;
pub const SO_MAX_PACING_RATE: u32 = 0x0031;
pub const SO_BPF_EXTENSIONS: u32 = 0x0032;
pub const SO_INCOMING_CPU: u32 = 0x0033;
pub const SO_ATTACH_BPF: u32 = 0x0034;
pub const SO_DETACH_BPF: u32 = SO_DETACH_FILTER;
pub const SO_ATTACH_REUSEPORT_CBPF: u32 = 0x0035;
pub const SO_ATTACH_REUSEPORT_EBPF: u32 = 0x0036;
pub const SO_CNX_ADVICE: u32 = 0x0037;
pub const SCM_TIMESTAMPING_OPT_STATS: u32 = 0x0038;
pub const SO_MEMINFO: u32 = 0x0039;
pub const SO_INCOMING_NAPI_ID: u32 = 0x003a;
pub const SO_COOKIE: u32 = 0x003b;
pub const SCM_TIMESTAMPING_PKTINFO: u32 = 0x003c;
pub const SO_PEERGROUPS: u32 = 0x003d;
pub const SO_ZEROCOPY: u32 = 0x003e;
pub const SO_TXTIME: u32 = 0x003f;
pub const SCM_TXTIME: u32 = SO_TXTIME;
pub const SO_BINDTOIFINDEX: u32 = 0x0041;

/* Security levels - as per NRL IPv6 - don't actually do anything */
pub const SO_SECURITY_AUTHENTICATION: u32 = 0x5001;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: u32 = 0x5002;
pub const SO_SECURITY_ENCRYPTION_NETWORK: u32 = 0x5004;

pub const SO_TIMESTAMP_OLD: u32 = 0x001d;
pub const SO_TIMESTAMPNS_OLD: u32 = 0x0021;
pub const SO_TIMESTAMPING_OLD: u32 = 0x0023;
pub const SO_TIMESTAMP_NEW: u32 = 0x0046;
pub const SO_TIMESTAMPNS_NEW: u32 = 0x0042;
pub const SO_TIMESTAMPING_NEW: u32 = 0x0043;
pub const SO_RCVTIMEO_NEW: u32 = 0x0044;
pub const SO_SNDTIMEO_NEW: u32 = 0x0045;
pub const SO_DETACH_REUSEPORT_BPF: u32 = 0x0047;
pub const SO_PREFER_BUSY_POLL: u32 = 0x0048;
pub const SO_BUSY_POLL_BUDGET: u32 = 0x0049;
pub const SO_NETNS_COOKIE: u32 = 0x0050;
pub const SO_BUF_LOCK: u32 = 0x0051;
pub const SO_RESERVE_MEM: u32 = 0x0052;
pub const SO_TXREHASH: u32 = 0x0053;
pub const SO_RCVMARK: u32 = 0x0054;
pub const SO_PASSPIDFD: u32 = 0x0055;
pub const SO_PEERPIDFD: u32 = 0x0056;
pub const SO_DEVMEM_LINEAR: u32 = 0x0057;
pub const SCM_DEVMEM_LINEAR: u32 = SO_DEVMEM_LINEAR;
pub const SO_DEVMEM_DMABUF: u32 = 0x0058;
pub const SCM_DEVMEM_DMABUF: u32 = SO_DEVMEM_DMABUF;
pub const SO_DEVMEM_DONTNEED: u32 = 0x0059;
pub const SCM_TS_OPT_ID: u32 = 0x005a;
pub const SO_RCVPRIORITY: u32 = 0x005b;
pub const SO_PASSRIGHTS: u32 = 0x005c;
pub const SO_INQ: u32 = 0x005d;
pub const SCM_INQ: u32 = SO_INQ;
pub const SO_RIGHTS_NOTRUNC: u32 = 0x005e;

/* The following are user-space-only definitions from !__KERNEL__. */
#[cfg(target_pointer_width = "64")]
mod user_space_64 {
    pub const SO_TIMESTAMP: u32 = super::SO_TIMESTAMP_OLD;
    pub const SO_TIMESTAMPNS: u32 = super::SO_TIMESTAMPNS_OLD;
    pub const SO_TIMESTAMPING: u32 = super::SO_TIMESTAMPING_OLD;
    pub const SO_RCVTIMEO: u32 = super::SO_RCVTIMEO_OLD;
    pub const SO_SNDTIMEO: u32 = super::SO_SNDTIMEO_OLD;
}

// On 32-bit targets the C header selects OLD or NEW according to time_t and __kernel_long_t.
#[cfg(target_pointer_width = "32")]
mod user_space_32 {
    pub const SO_TIMESTAMP: u32 = super::SO_TIMESTAMP_NEW;
    pub const SO_TIMESTAMPNS: u32 = super::SO_TIMESTAMPNS_NEW;
    pub const SO_TIMESTAMPING: u32 = super::SO_TIMESTAMPING_NEW;
    pub const SO_RCVTIMEO: u32 = super::SO_RCVTIMEO_NEW;
    pub const SO_SNDTIMEO: u32 = super::SO_SNDTIMEO_NEW;
}

#[cfg(target_pointer_width = "64")]
pub const SO_TIMESTAMP: u32 = SO_TIMESTAMP_OLD;
#[cfg(target_pointer_width = "64")]
pub const SO_TIMESTAMPNS: u32 = SO_TIMESTAMPNS_OLD;
#[cfg(target_pointer_width = "64")]
pub const SO_TIMESTAMPING: u32 = SO_TIMESTAMPING_OLD;
#[cfg(target_pointer_width = "64")]
pub const SO_RCVTIMEO: u32 = SO_RCVTIMEO_OLD;
#[cfg(target_pointer_width = "64")]
pub const SO_SNDTIMEO: u32 = SO_SNDTIMEO_OLD;

// On 32-bit targets, these correspond to the C sizeof(time_t) conditional.
#[cfg(target_pointer_width = "32")]
pub const SO_TIMESTAMP: u32 = SO_TIMESTAMP_NEW;
#[cfg(target_pointer_width = "32")]
pub const SO_TIMESTAMPNS: u32 = SO_TIMESTAMPNS_NEW;
#[cfg(target_pointer_width = "32")]
pub const SO_TIMESTAMPING: u32 = SO_TIMESTAMPING_NEW;
#[cfg(target_pointer_width = "32")]
pub const SO_RCVTIMEO: u32 = SO_RCVTIMEO_NEW;
#[cfg(target_pointer_width = "32")]
pub const SO_SNDTIMEO: u32 = SO_SNDTIMEO_NEW;

pub const SCM_TIMESTAMP: u32 = SO_TIMESTAMP;
pub const SCM_TIMESTAMPNS: u32 = SO_TIMESTAMPNS;
pub const SCM_TIMESTAMPING: u32 = SO_TIMESTAMPING;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
