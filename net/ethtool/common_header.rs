/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the kernel ethtool/netdevice translation.

pub const ETHTOOL_DEV_FEATURE_WORDS: usize =
    ((NETDEV_FEATURE_COUNT as usize) + 32 - 1) / 32;

/* Compose link mode index from speed, type and duplex. */
// C macro: ETHTOOL_LINK_MODE(speed, type, duplex)

pub const __SOF_TIMESTAMPING_CNT: usize = (const_ilog2(SOF_TIMESTAMPING_LAST) + 1) as usize;
pub const __HWTSTAMP_FLAG_CNT: usize = (const_ilog2(HWTSTAMP_FLAG_LAST) + 1) as usize;

#[repr(C)]
pub struct genl_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hwtstamp_provider_desc {
    _private: [u8; 0],
}

extern "C" {
    pub static netdev_features_strings: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; NETDEV_FEATURE_COUNT as usize];
    pub static rss_hash_func_strings: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; ETH_RSS_HASH_FUNCS_COUNT as usize];
    pub static tunable_strings: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; __ETHTOOL_TUNABLE_COUNT as usize];
    pub static phy_tunable_strings: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; __ETHTOOL_PHY_TUNABLE_COUNT as usize];
    pub static link_mode_names: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; 0];
    pub static netif_msg_class_names: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; 0];
    pub static wol_mode_names: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; 0];
    pub static sof_timestamping_names: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; 0];
    pub static ts_tx_type_names: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; 0];
    pub static ts_rx_filter_names: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; 0];
    pub static ts_flags_names: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; 0];
    pub static udp_tunnel_type_names: [[::std::os::raw::c_char; ETH_GSTRING_LEN as usize]; 0];

    pub fn __ethtool_get_link(dev: *mut net_device) -> ::std::os::raw::c_int;
    pub fn convert_legacy_settings_to_link_ksettings(
        link_ksettings: *mut ethtool_link_ksettings,
        legacy_settings: *const ethtool_cmd,
    ) -> bool;
    pub fn ethtool_check_max_channel(dev: *mut net_device, channels: ethtool_channels, info: *mut genl_info) -> ::std::os::raw::c_int;
    pub fn ethtool_rxfh_ctx_alloc(ops: *const ethtool_ops, indir_size: u32, key_size: u32) -> *mut ethtool_rxfh_context;
    pub fn ethtool_check_rss_ctx_busy(dev: *mut net_device, rss_context: u32) -> ::std::os::raw::c_int;
    pub fn ethtool_rxfh_config_is_sym(rxfh: u64) -> ::std::os::raw::c_int;
    pub fn ethtool_ringparam_get_cfg(dev: *mut net_device, param: *mut ethtool_ringparam, kparam: *mut kernel_ethtool_ringparam, extack: *mut netlink_ext_ack);
    pub fn ethtool_get_rx_ring_count(dev: *mut net_device) -> ::std::os::raw::c_int;
    pub fn __ethtool_get_ts_info(dev: *mut net_device, info: *mut kernel_ethtool_ts_info) -> ::std::os::raw::c_int;
    pub fn ethtool_get_ts_info_by_phc(dev: *mut net_device, info: *mut kernel_ethtool_ts_info, hwprov_desc: *mut hwtstamp_provider_desc) -> ::std::os::raw::c_int;
    pub fn ethtool_net_get_ts_info_by_phc(dev: *mut net_device, info: *mut kernel_ethtool_ts_info, hwprov_desc: *mut hwtstamp_provider_desc) -> ::std::os::raw::c_int;
    pub fn ethtool_phy_get_ts_info_by_phc(dev: *mut net_device, info: *mut kernel_ethtool_ts_info, hwprov_desc: *mut hwtstamp_provider_desc) -> *mut phy_device;
    pub fn net_support_hwtstamp_qualifier(dev: *mut net_device, qualifier: hwtstamp_provider_qualifier) -> bool;
    pub static ethtool_phy_ops: *const ethtool_phy_ops;
    pub static ethtool_pse_ops: *const ethtool_pse_ops;
    pub fn ethtool_get_module_info_call(dev: *mut net_device, modinfo: *mut ethtool_modinfo) -> ::std::os::raw::c_int;
    pub fn ethtool_get_module_eeprom_call(dev: *mut net_device, ee: *mut ethtool_eeprom, data: *mut u8) -> ::std::os::raw::c_int;
    pub fn __ethtool_dev_mm_supported(dev: *mut net_device) -> bool;
}

pub unsafe fn ethtool_nl_msg_needs_rtnl(dev: *const net_device, cmd: u8) -> bool {
    let ops = (*dev).ethtool_ops;
    match cmd {
        ETHTOOL_MSG_LINKINFO_GET | ETHTOOL_MSG_LINKINFO_SET |
        ETHTOOL_MSG_LINKMODES_GET | ETHTOOL_MSG_LINKMODES_SET => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_LINKSETTINGS != 0,
        ETHTOOL_MSG_PRIVFLAGS_SET => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_SPFLAGS != 0,
        ETHTOOL_MSG_RINGS_SET => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_SRINGPARAM != 0,
        ETHTOOL_MSG_CHANNELS_SET => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_SCHANNELS != 0,
        ETHTOOL_MSG_COALESCE_SET => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_SCOALESCE != 0,
        ETHTOOL_MSG_PAUSE_GET => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_GPAUSEPARAM != 0,
        ETHTOOL_MSG_PAUSE_SET => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_SPAUSEPARAM != 0,
        ETHTOOL_MSG_RSS_SET => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_RSS != 0,
        ETHTOOL_MSG_LINKSTATE_GET => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_GLINK != 0,
        ETHTOOL_MSG_TSCONFIG_GET | ETHTOOL_MSG_TSCONFIG_SET => true,
        _ => false,
    }
}

pub unsafe fn ethtool_ioctl_needs_rtnl(dev: *const net_device, ethcmd: u32) -> bool {
    let ops = (*dev).ethtool_ops;
    match ethcmd {
        ETHTOOL_GLINKSETTINGS | ETHTOOL_GSET | ETHTOOL_SLINKSETTINGS | ETHTOOL_SSET => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_LINKSETTINGS != 0,
        ETHTOOL_SPFLAGS => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_SPFLAGS != 0,
        ETHTOOL_SRINGPARAM => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_SRINGPARAM != 0,
        ETHTOOL_SCHANNELS => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_SCHANNELS != 0,
        ETHTOOL_SCOALESCE => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_SCOALESCE != 0,
        ETHTOOL_GPAUSEPARAM => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_GPAUSEPARAM != 0,
        ETHTOOL_SPAUSEPARAM => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_SPAUSEPARAM != 0,
        ETHTOOL_SRSSH | ETHTOOL_SRXFH | ETHTOOL_SRXFHINDIR => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_RSS != 0,
        ETHTOOL_GLINK => (*ops).op_needs_rtnl & ETHTOOL_OP_NEEDS_RTNL_GLINK != 0,
        _ => false,
    }
}

// CONFIG_ETHTOOL_NETLINK controls whether this notifier has an implementation.
extern "C" { pub fn ethtool_rss_notify(dev: *mut net_device, r#type: u32, rss_context: u32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
