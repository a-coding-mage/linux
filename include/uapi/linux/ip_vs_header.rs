/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* IP Virtual Server data structure and functionality definitions */

pub const IP_VS_VERSION_CODE: u32 = 0x010201;

/* Virtual Service Flags */
pub const IP_VS_SVC_F_PERSISTENT: u32 = 0x0001;
pub const IP_VS_SVC_F_HASHED: u32 = 0x0002;
pub const IP_VS_SVC_F_ONEPACKET: u32 = 0x0004;
pub const IP_VS_SVC_F_SCHED1: u32 = 0x0008;
pub const IP_VS_SVC_F_SCHED2: u32 = 0x0010;
pub const IP_VS_SVC_F_SCHED3: u32 = 0x0020;
pub const IP_VS_SVC_F_SCHED_SH_FALLBACK: u32 = IP_VS_SVC_F_SCHED1;
pub const IP_VS_SVC_F_SCHED_SH_PORT: u32 = IP_VS_SVC_F_SCHED2;

/* IPVS sync daemon states */
pub const IP_VS_STATE_NONE: u32 = 0x0000;
pub const IP_VS_STATE_MASTER: u32 = 0x0001;
pub const IP_VS_STATE_BACKUP: u32 = 0x0002;

/* IPVS socket options */
pub const IP_VS_BASE_CTL: u32 = 64 + 1024 + 64;
pub const IP_VS_SO_SET_NONE: u32 = IP_VS_BASE_CTL;
pub const IP_VS_SO_SET_INSERT: u32 = IP_VS_BASE_CTL + 1;
pub const IP_VS_SO_SET_ADD: u32 = IP_VS_BASE_CTL + 2;
pub const IP_VS_SO_SET_EDIT: u32 = IP_VS_BASE_CTL + 3;
pub const IP_VS_SO_SET_DEL: u32 = IP_VS_BASE_CTL + 4;
pub const IP_VS_SO_SET_FLUSH: u32 = IP_VS_BASE_CTL + 5;
pub const IP_VS_SO_SET_LIST: u32 = IP_VS_BASE_CTL + 6;
pub const IP_VS_SO_SET_ADDDEST: u32 = IP_VS_BASE_CTL + 7;
pub const IP_VS_SO_SET_DELDEST: u32 = IP_VS_BASE_CTL + 8;
pub const IP_VS_SO_SET_EDITDEST: u32 = IP_VS_BASE_CTL + 9;
pub const IP_VS_SO_SET_TIMEOUT: u32 = IP_VS_BASE_CTL + 10;
pub const IP_VS_SO_SET_STARTDAEMON: u32 = IP_VS_BASE_CTL + 11;
pub const IP_VS_SO_SET_STOPDAEMON: u32 = IP_VS_BASE_CTL + 12;
pub const IP_VS_SO_SET_RESTORE: u32 = IP_VS_BASE_CTL + 13;
pub const IP_VS_SO_SET_SAVE: u32 = IP_VS_BASE_CTL + 14;
pub const IP_VS_SO_SET_ZERO: u32 = IP_VS_BASE_CTL + 15;
pub const IP_VS_SO_SET_MAX: u32 = IP_VS_SO_SET_ZERO;
pub const IP_VS_SO_GET_VERSION: u32 = IP_VS_BASE_CTL;
pub const IP_VS_SO_GET_INFO: u32 = IP_VS_BASE_CTL + 1;
pub const IP_VS_SO_GET_SERVICES: u32 = IP_VS_BASE_CTL + 2;
pub const IP_VS_SO_GET_SERVICE: u32 = IP_VS_BASE_CTL + 3;
pub const IP_VS_SO_GET_DESTS: u32 = IP_VS_BASE_CTL + 4;
pub const IP_VS_SO_GET_DEST: u32 = IP_VS_BASE_CTL + 5;
pub const IP_VS_SO_GET_TIMEOUT: u32 = IP_VS_BASE_CTL + 6;
pub const IP_VS_SO_GET_DAEMON: u32 = IP_VS_BASE_CTL + 7;
pub const IP_VS_SO_GET_MAX: u32 = IP_VS_SO_GET_DAEMON;

/* IPVS Connection Flags */
pub const IP_VS_CONN_F_FWD_MASK: u32 = 0x0007;
pub const IP_VS_CONN_F_MASQ: u32 = 0x0000;
pub const IP_VS_CONN_F_LOCALNODE: u32 = 0x0001;
pub const IP_VS_CONN_F_TUNNEL: u32 = 0x0002;
pub const IP_VS_CONN_F_DROUTE: u32 = 0x0003;
pub const IP_VS_CONN_F_BYPASS: u32 = 0x0004;
pub const IP_VS_CONN_F_SYNC: u32 = 0x0020;
pub const IP_VS_CONN_F_HASHED: u32 = 0x0040;
pub const IP_VS_CONN_F_NOOUTPUT: u32 = 0x0080;
pub const IP_VS_CONN_F_INACTIVE: u32 = 0x0100;
pub const IP_VS_CONN_F_OUT_SEQ: u32 = 0x0200;
pub const IP_VS_CONN_F_IN_SEQ: u32 = 0x0400;
pub const IP_VS_CONN_F_SEQ_MASK: u32 = 0x0600;
pub const IP_VS_CONN_F_NO_CPORT: u32 = 0x0800;
pub const IP_VS_CONN_F_TEMPLATE: u32 = 0x1000;
pub const IP_VS_CONN_F_ONE_PACKET: u32 = 0x2000;
pub const IP_VS_CONN_F_BACKUP_MASK: u32 = IP_VS_CONN_F_FWD_MASK | IP_VS_CONN_F_NOOUTPUT | IP_VS_CONN_F_INACTIVE | IP_VS_CONN_F_SEQ_MASK | IP_VS_CONN_F_NO_CPORT | IP_VS_CONN_F_TEMPLATE;
pub const IP_VS_CONN_F_BACKUP_UPD_MASK: u32 = IP_VS_CONN_F_INACTIVE | IP_VS_CONN_F_SEQ_MASK;
pub const IP_VS_CONN_F_NFCT: u32 = 1 << 16;
pub const IP_VS_CONN_F_DEST_MASK: u32 = IP_VS_CONN_F_FWD_MASK | IP_VS_CONN_F_ONE_PACKET | IP_VS_CONN_F_NFCT | 0;

pub const IP_VS_SCHEDNAME_MAXLEN: usize = 16;
pub const IP_VS_PENAME_MAXLEN: usize = 16;
pub const IP_VS_IFNAME_MAXLEN: usize = 16;
pub const IP_VS_PEDATA_MAXLEN: usize = 255;

#[repr(u32)]
pub enum IpVsConnFTunnelType { IP_VS_CONN_F_TUNNEL_TYPE_IPIP = 0, IP_VS_CONN_F_TUNNEL_TYPE_GUE, IP_VS_CONN_F_TUNNEL_TYPE_GRE, IP_VS_CONN_F_TUNNEL_TYPE_MAX }
pub const IP_VS_TUNNEL_ENCAP_FLAG_NOCSUM: u32 = 0;
pub const IP_VS_TUNNEL_ENCAP_FLAG_CSUM: u32 = 1 << 0;
pub const IP_VS_TUNNEL_ENCAP_FLAG_REMCSUM: u32 = 1 << 1;

#[repr(C)]
pub struct ip_vs_service_user { pub protocol: u16, pub addr: u32, pub port: u16, pub fwmark: u32, pub sched_name: [i8; IP_VS_SCHEDNAME_MAXLEN], pub flags: u32, pub timeout: u32, pub netmask: u32 }
#[repr(C)]
pub struct ip_vs_dest_user { pub addr: u32, pub port: u16, pub conn_flags: u32, pub weight: i32, pub u_threshold: u32, pub l_threshold: u32 }
#[repr(C)]
pub struct ip_vs_stats_user { pub conns: u32, pub inpkts: u32, pub outpkts: u32, pub inbytes: u64, pub outbytes: u64, pub cps: u32, pub inpps: u32, pub outpps: u32, pub inbps: u32, pub outbps: u32 }
#[repr(C)]
pub struct ip_vs_getinfo { pub version: u32, pub size: u32, pub num_services: u32 }
#[repr(C)]
pub struct ip_vs_service_entry { pub protocol: u16, pub addr: u32, pub port: u16, pub fwmark: u32, pub sched_name: [i8; IP_VS_SCHEDNAME_MAXLEN], pub flags: u32, pub timeout: u32, pub netmask: u32, pub num_dests: u32, pub stats: ip_vs_stats_user }
#[repr(C)]
pub struct ip_vs_dest_entry { pub addr: u32, pub port: u16, pub conn_flags: u32, pub weight: i32, pub u_threshold: u32, pub l_threshold: u32, pub activeconns: u32, pub inactconns: u32, pub persistconns: u32, pub stats: ip_vs_stats_user }
#[repr(C)]
pub struct ip_vs_get_dests { pub protocol: u16, pub addr: u32, pub port: u16, pub fwmark: u32, pub num_dests: u32, pub entrytable: [ip_vs_dest_entry; 0] }
#[repr(C)]
pub struct ip_vs_get_services { pub num_services: u32, pub entrytable: [ip_vs_service_entry; 0] }
#[repr(C)]
pub struct ip_vs_timeout_user { pub tcp_timeout: i32, pub tcp_fin_timeout: i32, pub udp_timeout: i32 }
#[repr(C)]
pub struct ip_vs_daemon_user { pub state: i32, pub mcast_ifn: [i8; IP_VS_IFNAME_MAXLEN], pub syncid: i32 }

pub const IPVS_GENL_NAME: &str = "IPVS";
pub const IPVS_GENL_VERSION: u32 = 0x1;
#[repr(C)] pub struct ip_vs_flags { pub flags: u32, pub mask: u32 }

#[repr(u32)]
pub enum IpvsCmd { IPVS_CMD_UNSPEC = 0, IPVS_CMD_NEW_SERVICE, IPVS_CMD_SET_SERVICE, IPVS_CMD_DEL_SERVICE, IPVS_CMD_GET_SERVICE, IPVS_CMD_NEW_DEST, IPVS_CMD_SET_DEST, IPVS_CMD_DEL_DEST, IPVS_CMD_GET_DEST, IPVS_CMD_NEW_DAEMON, IPVS_CMD_DEL_DAEMON, IPVS_CMD_GET_DAEMON, IPVS_CMD_SET_CONFIG, IPVS_CMD_GET_CONFIG, IPVS_CMD_SET_INFO, IPVS_CMD_GET_INFO, IPVS_CMD_ZERO, IPVS_CMD_FLUSH, __IPVS_CMD_MAX }
pub const IPVS_CMD_MAX: u32 = IpvsCmd::__IPVS_CMD_MAX as u32 - 1;
#[repr(u32)] pub enum IpvsCmdAttr { IPVS_CMD_ATTR_UNSPEC = 0, IPVS_CMD_ATTR_SERVICE, IPVS_CMD_ATTR_DEST, IPVS_CMD_ATTR_DAEMON, IPVS_CMD_ATTR_TIMEOUT_TCP, IPVS_CMD_ATTR_TIMEOUT_TCP_FIN, IPVS_CMD_ATTR_TIMEOUT_UDP, __IPVS_CMD_ATTR_MAX }
pub const IPVS_CMD_ATTR_MAX: u32 = IpvsCmdAttr::__IPVS_CMD_ATTR_MAX as u32 - 1;
#[repr(u32)] pub enum IpvsSvcAttr { IPVS_SVC_ATTR_UNSPEC = 0, IPVS_SVC_ATTR_AF, IPVS_SVC_ATTR_PROTOCOL, IPVS_SVC_ATTR_ADDR, IPVS_SVC_ATTR_PORT, IPVS_SVC_ATTR_FWMARK, IPVS_SVC_ATTR_SCHED_NAME, IPVS_SVC_ATTR_FLAGS, IPVS_SVC_ATTR_TIMEOUT, IPVS_SVC_ATTR_NETMASK, IPVS_SVC_ATTR_STATS, IPVS_SVC_ATTR_PE_NAME, IPVS_SVC_ATTR_STATS64, __IPVS_SVC_ATTR_MAX }
pub const IPVS_SVC_ATTR_MAX: u32 = IpvsSvcAttr::__IPVS_SVC_ATTR_MAX as u32 - 1;
#[repr(u32)] pub enum IpvsDestAttr { IPVS_DEST_ATTR_UNSPEC = 0, IPVS_DEST_ATTR_ADDR, IPVS_DEST_ATTR_PORT, IPVS_DEST_ATTR_FWD_METHOD, IPVS_DEST_ATTR_WEIGHT, IPVS_DEST_ATTR_U_THRESH, IPVS_DEST_ATTR_L_THRESH, IPVS_DEST_ATTR_ACTIVE_CONNS, IPVS_DEST_ATTR_INACT_CONNS, IPVS_DEST_ATTR_PERSIST_CONNS, IPVS_DEST_ATTR_STATS, IPVS_DEST_ATTR_ADDR_FAMILY, IPVS_DEST_ATTR_STATS64, IPVS_DEST_ATTR_TUN_TYPE, IPVS_DEST_ATTR_TUN_PORT, IPVS_DEST_ATTR_TUN_FLAGS, __IPVS_DEST_ATTR_MAX }
pub const IPVS_DEST_ATTR_MAX: u32 = IpvsDestAttr::__IPVS_DEST_ATTR_MAX as u32 - 1;
#[repr(u32)] pub enum IpvsDaemonAttr { IPVS_DAEMON_ATTR_UNSPEC = 0, IPVS_DAEMON_ATTR_STATE, IPVS_DAEMON_ATTR_MCAST_IFN, IPVS_DAEMON_ATTR_SYNC_ID, IPVS_DAEMON_ATTR_SYNC_MAXLEN, IPVS_DAEMON_ATTR_MCAST_GROUP, IPVS_DAEMON_ATTR_MCAST_GROUP6, IPVS_DAEMON_ATTR_MCAST_PORT, IPVS_DAEMON_ATTR_MCAST_TTL, __IPVS_DAEMON_ATTR_MAX }
pub const IPVS_DAEMON_ATTR_MAX: u32 = IpvsDaemonAttr::__IPVS_DAEMON_ATTR_MAX as u32 - 1;
#[repr(u32)] pub enum IpvsStatsAttr { IPVS_STATS_ATTR_UNSPEC = 0, IPVS_STATS_ATTR_CONNS, IPVS_STATS_ATTR_INPKTS, IPVS_STATS_ATTR_OUTPKTS, IPVS_STATS_ATTR_INBYTES, IPVS_STATS_ATTR_OUTBYTES, IPVS_STATS_ATTR_CPS, IPVS_STATS_ATTR_INPPS, IPVS_STATS_ATTR_OUTPPS, IPVS_STATS_ATTR_INBPS, IPVS_STATS_ATTR_OUTBPS, IPVS_STATS_ATTR_PAD, __IPVS_STATS_ATTR_MAX }
pub const IPVS_STATS_ATTR_MAX: u32 = IpvsStatsAttr::__IPVS_STATS_ATTR_MAX as u32 - 1;
#[repr(u32)] pub enum IpvsInfoAttr { IPVS_INFO_ATTR_UNSPEC = 0, IPVS_INFO_ATTR_VERSION, IPVS_INFO_ATTR_CONN_TAB_SIZE, __IPVS_INFO_ATTR_MAX }
pub const IPVS_INFO_ATTR_MAX: u32 = IpvsInfoAttr::__IPVS_INFO_ATTR_MAX as u32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
