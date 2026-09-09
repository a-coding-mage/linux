/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright Gavin Shan, IBM Corporation 2016. */

#[repr(C)]
pub struct ncsi_channel_version {
    pub major: u8, pub minor: u8, pub update: u8,
    pub alpha1: std::ffi::c_char, pub alpha2: std::ffi::c_char,
    pub fw_name: [u8; 13], pub fw_version: u32, pub pci_ids: [u16; 4], pub mf_id: u32,
}
#[repr(C)] pub struct ncsi_channel_cap { pub index: u32, pub cap: u32 }
#[repr(C)] pub struct ncsi_channel_mode { pub index: u32, pub enable: u32, pub size: u32, pub data: [u32; 8] }
#[repr(C)] pub struct ncsi_channel_mac_filter { pub n_uc: u8, pub n_mc: u8, pub n_mixed: u8, pub bitmap: u64, pub addrs: *mut u8 }
#[repr(C)] pub struct ncsi_channel_vlan_filter { pub n_vids: u8, pub bitmap: u64, pub vids: *mut u16 }

#[repr(C)] pub struct ncsi_channel_stats {
    pub hnc_cnt: u64, pub hnc_rx_bytes: u64, pub hnc_tx_bytes: u64,
    pub hnc_rx_uc_pkts: u64, pub hnc_rx_mc_pkts: u64, pub hnc_rx_bc_pkts: u64,
    pub hnc_tx_uc_pkts: u64, pub hnc_tx_mc_pkts: u64, pub hnc_tx_bc_pkts: u64,
    pub hnc_fcs_err: u32, pub hnc_align_err: u32, pub hnc_false_carrier: u32,
    pub hnc_runt_pkts: u32, pub hnc_jabber_pkts: u32, pub hnc_rx_pause_xon: u32,
    pub hnc_rx_pause_xoff: u32, pub hnc_tx_pause_xon: u32, pub hnc_tx_pause_xoff: u32,
    pub hnc_tx_s_collision: u32, pub hnc_tx_m_collision: u32, pub hnc_l_collision: u32,
    pub hnc_e_collision: u32, pub hnc_rx_ctl_frames: u32, pub hnc_rx_64_frames: u32,
    pub hnc_rx_127_frames: u32, pub hnc_rx_255_frames: u32, pub hnc_rx_511_frames: u32,
    pub hnc_rx_1023_frames: u32, pub hnc_rx_1522_frames: u32, pub hnc_rx_9022_frames: u32,
    pub hnc_tx_64_frames: u32, pub hnc_tx_127_frames: u32, pub hnc_tx_255_frames: u32,
    pub hnc_tx_511_frames: u32, pub hnc_tx_1023_frames: u32, pub hnc_tx_1522_frames: u32,
    pub hnc_tx_9022_frames: u32, pub hnc_rx_valid_bytes: u64, pub hnc_rx_runt_pkts: u32,
    pub hnc_rx_jabber_pkts: u32, pub ncsi_rx_cmds: u32, pub ncsi_dropped_cmds: u32,
    pub ncsi_cmd_type_errs: u32, pub ncsi_cmd_csum_errs: u32, pub ncsi_rx_pkts: u32,
    pub ncsi_tx_pkts: u32, pub ncsi_tx_aen_pkts: u32, pub pt_tx_pkts: u32,
    pub pt_tx_dropped: u32, pub pt_tx_channel_err: u32, pub pt_tx_us_err: u32,
    pub pt_rx_pkts: u32, pub pt_rx_dropped: u32, pub pt_rx_channel_err: u32,
    pub pt_rx_us_err: u32, pub pt_rx_os_err: u32,
}

pub const NCSI_CAP_BASE: u32 = 0; pub const NCSI_CAP_GENERIC: u32 = 0;
pub const NCSI_CAP_BC: u32 = 1; pub const NCSI_CAP_MC: u32 = 2; pub const NCSI_CAP_BUFFER: u32 = 3;
pub const NCSI_CAP_AEN: u32 = 4; pub const NCSI_CAP_VLAN: u32 = 5; pub const NCSI_CAP_MAX: usize = 6;
pub const NCSI_CAP_GENERIC_HWA: u32 = 1; pub const NCSI_CAP_GENERIC_HDS: u32 = 2;
pub const NCSI_CAP_GENERIC_FC: u32 = 4; pub const NCSI_CAP_GENERIC_FC1: u32 = 8;
pub const NCSI_CAP_GENERIC_MC: u32 = 0x10; pub const NCSI_CAP_GENERIC_HWA_UNKNOWN: u32 = 0;
pub const NCSI_CAP_GENERIC_HWA_SUPPORT: u32 = 0x20; pub const NCSI_CAP_GENERIC_HWA_NOT_SUPPORT: u32 = 0x40;
pub const NCSI_CAP_GENERIC_HWA_RESERVED: u32 = 0x60; pub const NCSI_CAP_GENERIC_HWA_MASK: u32 = 0x60;
pub const NCSI_CAP_GENERIC_MASK: u32 = 0x7f; pub const NCSI_CAP_BC_ARP: u32 = 1;
pub const NCSI_CAP_BC_DHCPC: u32 = 2; pub const NCSI_CAP_BC_DHCPS: u32 = 4; pub const NCSI_CAP_BC_NETBIOS: u32 = 8;
pub const NCSI_CAP_BC_MASK: u32 = 0xf; pub const NCSI_CAP_MC_IPV6_NEIGHBOR: u32 = 1;
pub const NCSI_CAP_MC_IPV6_ROUTER: u32 = 2; pub const NCSI_CAP_MC_DHCPV6_RELAY: u32 = 4;
pub const NCSI_CAP_MC_DHCPV6_WELL_KNOWN: u32 = 8; pub const NCSI_CAP_MC_IPV6_MLD: u32 = 0x10;
pub const NCSI_CAP_MC_IPV6_NEIGHBOR_S: u32 = 0x20; pub const NCSI_CAP_MC_MASK: u32 = 0x3f;
pub const NCSI_CAP_AEN_LSC: u32 = 1; pub const NCSI_CAP_AEN_CR: u32 = 2; pub const NCSI_CAP_AEN_HDS: u32 = 4;
pub const NCSI_CAP_AEN_MASK: u32 = 7; pub const NCSI_CAP_VLAN_ONLY: u32 = 1;
pub const NCSI_CAP_VLAN_NO: u32 = 2; pub const NCSI_CAP_VLAN_ANY: u32 = 4; pub const NCSI_CAP_VLAN_MASK: u32 = 7;

pub const NCSI_MODE_BASE: u32 = 0; pub const NCSI_MODE_ENABLE: u32 = 0; pub const NCSI_MODE_TX_ENABLE: u32 = 1;
pub const NCSI_MODE_LINK: u32 = 2; pub const NCSI_MODE_VLAN: u32 = 3; pub const NCSI_MODE_BC: u32 = 4;
pub const NCSI_MODE_MC: u32 = 5; pub const NCSI_MODE_AEN: u32 = 6; pub const NCSI_MODE_FC: u32 = 7; pub const NCSI_MODE_MAX: usize = 8;
pub const MLX_MC_RBT_SUPPORT: u32 = 1; pub const MLX_MC_RBT_AVL: u32 = 8;

pub const NCSI_OEM_MFR_MLX_ID: u32 = 0x8119; pub const NCSI_OEM_MFR_BCM_ID: u32 = 0x113d; pub const NCSI_OEM_MFR_INTEL_ID: u32 = 0x157;
pub const NCSI_OEM_INTEL_CMD_GMA: u32 = 6; pub const NCSI_OEM_INTEL_CMD_KEEP_PHY: u32 = 0x20;
pub const NCSI_OEM_BCM_CMD_GMA: u32 = 1; pub const NCSI_OEM_MLX_CMD_GMA: u32 = 0;
pub const NCSI_OEM_MLX_CMD_GMA_PARAM: u32 = 0x1b; pub const NCSI_OEM_MLX_CMD_SMAF: u32 = 1; pub const NCSI_OEM_MLX_CMD_SMAF_PARAM: u32 = 7;
pub const NCSI_OEM_INTEL_CMD_GMA_LEN: usize = 5; pub const NCSI_OEM_INTEL_CMD_KEEP_PHY_LEN: usize = 7;
pub const NCSI_OEM_BCM_CMD_GMA_LEN: usize = 12; pub const NCSI_OEM_MLX_CMD_GMA_LEN: usize = 8; pub const NCSI_OEM_MLX_CMD_SMAF_LEN: usize = 60;
pub const MLX_SMAF_MAC_ADDR_OFFSET: usize = 8; pub const MLX_SMAF_MED_SUPPORT_OFFSET: usize = 14;
pub const BCM_MAC_ADDR_OFFSET: usize = 28; pub const MLX_MAC_ADDR_OFFSET: usize = 8; pub const INTEL_MAC_ADDR_OFFSET: usize = 1;

pub const NCSI_PACKAGE_SHIFT: u32 = 5; pub const NCSI_RESERVED_CHANNEL: u32 = 0x1f;
#[inline] pub const fn NCSI_PACKAGE_INDEX(c: u32) -> u32 { (c >> NCSI_PACKAGE_SHIFT) & 7 }
#[inline] pub const fn NCSI_CHANNEL_INDEX(c: u32) -> u32 { c & ((1 << NCSI_PACKAGE_SHIFT) - 1) }
#[inline] pub const fn NCSI_TO_CHANNEL(p: u32, c: u32) -> u32 { (p << NCSI_PACKAGE_SHIFT) | c }
pub const NCSI_MAX_PACKAGE: usize = 8; pub const NCSI_MAX_CHANNEL: usize = 32;

/* Kernel types and the remaining declarations are supplied by other headers. */
pub enum ncsi_dev {}
pub enum net_device {}
pub enum sk_buff {}
pub enum packet_type {}
pub enum genl_info {}
pub enum sockaddr_storage {}
pub enum spinlock_t {}
pub enum timer_list {}
pub enum list_head {}
pub enum work_struct {}
pub enum nlmsghdr {}
pub type __be16 = u16;

#[repr(C)] pub struct ncsi_channel { pub id: u8, pub state: i32, pub reconfigure_needed: bool, pub lock: spinlock_t, pub package: *mut ncsi_package, pub version: ncsi_channel_version, pub caps: [ncsi_channel_cap; NCSI_CAP_MAX], pub modes: [ncsi_channel_mode; NCSI_MODE_MAX], pub mac_filter: ncsi_channel_mac_filter, pub vlan_filter: ncsi_channel_vlan_filter, pub stats: ncsi_channel_stats, pub monitor: ncsi_channel_monitor, pub node: list_head, pub link: list_head }
#[repr(C)] pub struct ncsi_channel_monitor { pub timer: timer_list, pub enabled: bool, pub state: u32 }
#[repr(C)] pub struct ncsi_package { pub id: u8, pub uuid: [u8; 16], pub ndp: *mut ncsi_dev_priv, pub lock: spinlock_t, pub channel_num: u32, pub channels: list_head, pub node: list_head, pub multi_channel: bool, pub channel_whitelist: u32, pub preferred_channel: *mut ncsi_channel }
#[repr(C)] pub struct ncsi_request { pub id: u8, pub used: bool, pub flags: u32, pub ndp: *mut ncsi_dev_priv, pub cmd: *mut sk_buff, pub rsp: *mut sk_buff, pub timer: timer_list, pub enabled: bool, pub snd_seq: u32, pub snd_portid: u32, pub nlhdr: nlmsghdr }
#[repr(C)] pub struct vlan_vid { pub list: list_head, pub proto: __be16, pub vid: u16 }
#[repr(C)] pub struct ncsi_dev_priv { pub ndev: ncsi_dev, pub flags: u32, pub gma_flag: u32, pub pending_mac: sockaddr_storage, pub lock: spinlock_t, pub package_probe_id: u32, pub package_num: u32, pub channel_probe_id: u32, pub packages: list_head, pub hot_channel: *mut ncsi_channel, pub requests: [ncsi_request; 256], pub request_id: u32, pub pending_req_num: u32, pub active_package: *mut ncsi_package, pub active_channel: *mut ncsi_channel, pub channel_queue: list_head, pub work: work_struct, pub ptype: packet_type, pub node: list_head, pub vlan_vids: list_head, pub multi_package: bool, pub mlx_multi_host: bool, pub package_whitelist: u32, pub channel_count: u8 }
#[repr(C)] pub union ncsi_cmd_arg_data { pub bytes: [u8; 16], pub words: [u16; 8], pub dwords: [u32; 4] }
#[repr(C)] pub struct ncsi_cmd_arg { pub ndp: *mut ncsi_dev_priv, pub type_: u8, pub id: u8, pub package: u8, pub channel: u8, pub payload: u16, pub req_flags: u32, pub data_union: ncsi_cmd_arg_data, pub data: *mut u8, pub info: *mut genl_info }

pub const NCSI_CHANNEL_INACTIVE: i32 = 1; pub const NCSI_CHANNEL_ACTIVE: i32 = 2; pub const NCSI_CHANNEL_INVISIBLE: i32 = 3;
pub const NCSI_REQ_FLAG_EVENT_DRIVEN: u32 = 1; pub const NCSI_REQ_FLAG_NETLINK_DRIVEN: u32 = 2;
pub const NCSI_DEV_PROBED: u32 = 1; pub const NCSI_DEV_HWA: u32 = 2; pub const NCSI_DEV_RESHUFFLE: u32 = 4; pub const NCSI_DEV_RESET: u32 = 8;
pub const NCSI_REQ_START_IDX: u32 = 1; pub const NCSI_MAX_VLAN_VIDS: usize = 15;

pub const ncsi_dev_state_major: u32 = 0xff00; pub const ncsi_dev_state_minor: u32 = 0x00ff;
pub const ncsi_dev_state_probe_deselect: u32 = 0x0201; pub const ncsi_dev_state_probe_package: u32 = 0x0202;
pub const ncsi_dev_state_probe_channel: u32 = 0x0203; pub const ncsi_dev_state_probe_mlx_gma: u32 = 0x0204;
pub const ncsi_dev_state_probe_mlx_smaf: u32 = 0x0205; pub const ncsi_dev_state_probe_cis: u32 = 0x0206;
pub const ncsi_dev_state_probe_keep_phy: u32 = 0x0207; pub const ncsi_dev_state_probe_gvi: u32 = 0x0208;
pub const ncsi_dev_state_probe_gc: u32 = 0x0209; pub const ncsi_dev_state_probe_gls: u32 = 0x020a;
pub const ncsi_dev_state_probe_dp: u32 = 0x020b; pub const ncsi_dev_state_config_sp: u32 = 0x0301;
pub const ncsi_dev_state_config_cis: u32 = 0x0302; pub const ncsi_dev_state_config_oem_gma: u32 = 0x0303;
pub const ncsi_dev_state_config_apply_mac: u32 = 0x0304; pub const ncsi_dev_state_config_clear_vids: u32 = 0x0305;
pub const ncsi_dev_state_config_svf: u32 = 0x0306; pub const ncsi_dev_state_config_ev: u32 = 0x0307;
pub const ncsi_dev_state_config_sma: u32 = 0x0308; pub const ncsi_dev_state_config_ebf: u32 = 0x0309;
pub const ncsi_dev_state_config_dgmf: u32 = 0x030a; pub const ncsi_dev_state_config_ecnt: u32 = 0x030b;
pub const ncsi_dev_state_config_ec: u32 = 0x030c; pub const ncsi_dev_state_config_ae: u32 = 0x030d;
pub const ncsi_dev_state_config_gls: u32 = 0x030e; pub const ncsi_dev_state_config_done: u32 = 0x030f;
pub const ncsi_dev_state_suspend_select: u32 = 0x0401; pub const ncsi_dev_state_suspend_gls: u32 = 0x0402;
pub const ncsi_dev_state_suspend_dcnt: u32 = 0x0403; pub const ncsi_dev_state_suspend_dc: u32 = 0x0404;
pub const ncsi_dev_state_suspend_deselect: u32 = 0x0405; pub const ncsi_dev_state_suspend_done: u32 = 0x0406;

extern "C" { pub static mut ncsi_dev_list: list_head; pub static mut ncsi_dev_lock: spinlock_t; }
extern "C" { pub fn ncsi_reset_dev(nd: *mut ncsi_dev) -> i32; pub fn ncsi_start_channel_monitor(nc: *mut ncsi_channel); pub fn ncsi_stop_channel_monitor(nc: *mut ncsi_channel); pub fn ncsi_find_channel(np: *mut ncsi_package, id: u8) -> *mut ncsi_channel; pub fn ncsi_add_channel(np: *mut ncsi_package, id: u8) -> *mut ncsi_channel; pub fn ncsi_find_package(ndp: *mut ncsi_dev_priv, id: u8) -> *mut ncsi_package; pub fn ncsi_add_package(ndp: *mut ncsi_dev_priv, id: u8) -> *mut ncsi_package; pub fn ncsi_remove_package(np: *mut ncsi_package); pub fn ncsi_find_package_and_channel(ndp: *mut ncsi_dev_priv, id: u8, np: *mut *mut ncsi_package, nc: *mut *mut ncsi_channel); pub fn ncsi_alloc_request(ndp: *mut ncsi_dev_priv, req_flags: u32) -> *mut ncsi_request; pub fn ncsi_free_request(nr: *mut ncsi_request); pub fn ncsi_find_dev(dev: *mut net_device) -> *mut ncsi_dev; pub fn ncsi_process_next_channel(ndp: *mut ncsi_dev_priv) -> i32; pub fn ncsi_channel_has_link(channel: *mut ncsi_channel) -> bool; pub fn ncsi_channel_is_last(ndp: *mut ncsi_dev_priv, channel: *mut ncsi_channel) -> bool; pub fn ncsi_update_tx_channel(ndp: *mut ncsi_dev_priv, np: *mut ncsi_package, disable: *mut ncsi_channel, enable: *mut ncsi_channel) -> i32; pub fn ncsi_calculate_checksum(data: *mut u8, len: i32) -> u32; pub fn ncsi_xmit_cmd(nca: *mut ncsi_cmd_arg) -> i32; pub fn ncsi_rcv_rsp(skb: *mut sk_buff, dev: *mut net_device, pt: *mut packet_type, orig_dev: *mut net_device) -> i32; pub fn ncsi_aen_handler(ndp: *mut ncsi_dev_priv, skb: *mut sk_buff) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
