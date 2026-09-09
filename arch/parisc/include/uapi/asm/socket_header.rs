/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent from the C header:
// #include <linux/posix_types.h>
// #include <asm/sockios.h>

/* For setsockopt(2) */
pub const SOL_SOCKET: u32 = 0xffff;

pub const SO_DEBUG: u32 = 0x0001;
pub const SO_REUSEADDR: u32 = 0x0004;
pub const SO_KEEPALIVE: u32 = 0x0008;
pub const SO_DONTROUTE: u32 = 0x0010;
pub const SO_BROADCAST: u32 = 0x0020;
pub const SO_LINGER: u32 = 0x0080;
pub const SO_OOBINLINE: u32 = 0x0100;
pub const SO_REUSEPORT: u32 = 0x0200;
pub const SO_SNDBUF: u32 = 0x1001;
pub const SO_RCVBUF: u32 = 0x1002;
pub const SO_SNDBUFFORCE: u32 = 0x100a;
pub const SO_RCVBUFFORCE: u32 = 0x100b;
pub const SO_SNDLOWAT: u32 = 0x1003;
pub const SO_RCVLOWAT: u32 = 0x1004;
pub const SO_SNDTIMEO_OLD: u32 = 0x1005;
pub const SO_RCVTIMEO_OLD: u32 = 0x1006;
pub const SO_ERROR: u32 = 0x1007;
pub const SO_TYPE: u32 = 0x1008;
pub const SO_PROTOCOL: u32 = 0x1028;
pub const SO_DOMAIN: u32 = 0x1029;
pub const SO_PEERNAME: u32 = 0x2000;

pub const SO_NO_CHECK: u32 = 0x400b;
pub const SO_PRIORITY: u32 = 0x400c;
pub const SO_BSDCOMPAT: u32 = 0x400e;
pub const SO_PASSCRED: u32 = 0x4010;
pub const SO_PEERCRED: u32 = 0x4011;

/* Security levels - as per NRL IPv6 - don't actually do anything */
pub const SO_SECURITY_AUTHENTICATION: u32 = 0x4016;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: u32 = 0x4017;
pub const SO_SECURITY_ENCRYPTION_NETWORK: u32 = 0x4018;
pub const SO_BINDTODEVICE: u32 = 0x4019;

/* Socket filtering */
pub const SO_ATTACH_FILTER: u32 = 0x401a;
pub const SO_DETACH_FILTER: u32 = 0x401b;
pub const SO_GET_FILTER: u32 = SO_ATTACH_FILTER;
pub const SO_ACCEPTCONN: u32 = 0x401c;
pub const SO_PEERSEC: u32 = 0x401d;
pub const SO_PASSSEC: u32 = 0x401e;
pub const SO_MARK: u32 = 0x401f;
pub const SO_RXQ_OVFL: u32 = 0x4021;
pub const SO_WIFI_STATUS: u32 = 0x4022;
pub const SCM_WIFI_STATUS: u32 = SO_WIFI_STATUS;
pub const SO_PEEK_OFF: u32 = 0x4023;

/* Instruct lower device to use last 4-bytes of skb data as FCS */
pub const SO_NOFCS: u32 = 0x4024;
pub const SO_LOCK_FILTER: u32 = 0x4025;
pub const SO_SELECT_ERR_QUEUE: u32 = 0x4026;
pub const SO_BUSY_POLL: u32 = 0x4027;
pub const SO_MAX_PACING_RATE: u32 = 0x4028;
pub const SO_BPF_EXTENSIONS: u32 = 0x4029;
pub const SO_INCOMING_CPU: u32 = 0x402A;
pub const SO_ATTACH_BPF: u32 = 0x402B;
pub const SO_DETACH_BPF: u32 = SO_DETACH_FILTER;
pub const SO_ATTACH_REUSEPORT_CBPF: u32 = 0x402C;
pub const SO_ATTACH_REUSEPORT_EBPF: u32 = 0x402D;
pub const SO_CNX_ADVICE: u32 = 0x402E;
pub const SCM_TIMESTAMPING_OPT_STATS: u32 = 0x402F;
pub const SO_MEMINFO: u32 = 0x4030;
pub const SO_INCOMING_NAPI_ID: u32 = 0x4031;
pub const SO_COOKIE: u32 = 0x4032;
pub const SCM_TIMESTAMPING_PKTINFO: u32 = 0x4033;
pub const SO_PEERGROUPS: u32 = 0x4034;
pub const SO_ZEROCOPY: u32 = 0x4035;
pub const SO_TXTIME: u32 = 0x4036;
pub const SCM_TXTIME: u32 = SO_TXTIME;
pub const SO_BINDTOIFINDEX: u32 = 0x4037;
pub const SO_TIMESTAMP_OLD: u32 = 0x4012;
pub const SO_TIMESTAMPNS_OLD: u32 = 0x4013;
pub const SO_TIMESTAMPING_OLD: u32 = 0x4020;
pub const SO_TIMESTAMP_NEW: u32 = 0x4038;
pub const SO_TIMESTAMPNS_NEW: u32 = 0x4039;
pub const SO_TIMESTAMPING_NEW: u32 = 0x403A;
pub const SO_RCVTIMEO_NEW: u32 = 0x4040;
pub const SO_SNDTIMEO_NEW: u32 = 0x4041;
pub const SO_DETACH_REUSEPORT_BPF: u32 = 0x4042;
pub const SO_PREFER_BUSY_POLL: u32 = 0x4043;
pub const SO_BUSY_POLL_BUDGET: u32 = 0x4044;
pub const SO_NETNS_COOKIE: u32 = 0x4045;
pub const SO_BUF_LOCK: u32 = 0x4046;
pub const SO_RESERVE_MEM: u32 = 0x4047;
pub const SO_TXREHASH: u32 = 0x4048;
pub const SO_RCVMARK: u32 = 0x4049;
pub const SO_PASSPIDFD: u32 = 0x404A;
pub const SO_PEERPIDFD: u32 = 0x404B;
pub const SCM_TS_OPT_ID: u32 = 0x404C;
pub const SO_RCVPRIORITY: u32 = 0x404D;
pub const SO_DEVMEM_LINEAR: u32 = 0x404E;
pub const SCM_DEVMEM_LINEAR: u32 = SO_DEVMEM_LINEAR;
pub const SO_DEVMEM_DMABUF: u32 = 0x404F;
pub const SCM_DEVMEM_DMABUF: u32 = SO_DEVMEM_DMABUF;
pub const SO_DEVMEM_DONTNEED: u32 = 0x4050;
pub const SO_PASSRIGHTS: u32 = 0x4051;
pub const SO_INQ: u32 = 0x4052;
pub const SCM_INQ: u32 = SO_INQ;
pub const SO_RIGHTS_NOTRUNC: u32 = 0x4053;

// The C header's __KERNEL__ exclusion is preserved conceptually here.
// On 64-bit targets these aliases select the old ABI values.  The non-64-bit
// C branch depends on the externally supplied time_t and __kernel_long_t types.
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

#[cfg(not(target_pointer_width = "64"))]
pub const SO_TIMESTAMP: u32 = SO_TIMESTAMP_NEW;
#[cfg(not(target_pointer_width = "64"))]
pub const SO_TIMESTAMPNS: u32 = SO_TIMESTAMPNS_NEW;
#[cfg(not(target_pointer_width = "64"))]
pub const SO_TIMESTAMPING: u32 = SO_TIMESTAMPING_NEW;
#[cfg(not(target_pointer_width = "64"))]
pub const SO_RCVTIMEO: u32 = SO_RCVTIMEO_NEW;
#[cfg(not(target_pointer_width = "64"))]
pub const SO_SNDTIMEO: u32 = SO_SNDTIMEO_NEW;

pub const SCM_TIMESTAMP: u32 = SO_TIMESTAMP;
pub const SCM_TIMESTAMPNS: u32 = SO_TIMESTAMPNS;
pub const SCM_TIMESTAMPING: u32 = SO_TIMESTAMPING;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
