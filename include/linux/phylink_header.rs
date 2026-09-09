// Translation of linux/phylink.h. External kernel types and symbols are
// intentionally referenced but not defined here.

#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct ethtool_cmd { _private: [u8; 0] }
#[repr(C)]
pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)]
pub struct net_device { _private: [u8; 0] }
#[repr(C)]
pub struct phylink { _private: [u8; 0] }
#[repr(C)]
pub struct phy_device { _private: [u8; 0] }
#[repr(C)]
pub struct mdio_device { _private: [u8; 0] }
#[repr(C)]
pub struct ethtool_wolinfo { _private: [u8; 0] }
#[repr(C)]
pub struct ethtool_link_ksettings { _private: [u8; 0] }
#[repr(C)]
pub struct ethtool_pauseparam { _private: [u8; 0] }
#[repr(C)]
pub struct ethtool_keee { _private: [u8; 0] }
#[repr(C)]
pub struct ifreq { _private: [u8; 0] }

pub type phy_interface_t = u32;
pub type u8_ = u8;
pub type u16_ = u16;
pub type u32_ = u32;

pub const MLO_PAUSE_NONE: u32 = 0;
pub const MLO_PAUSE_RX: u32 = 1 << 0;
pub const MLO_PAUSE_TX: u32 = 1 << 1;
pub const MLO_PAUSE_TXRX_MASK: u32 = MLO_PAUSE_TX | MLO_PAUSE_RX;
pub const MLO_PAUSE_AN: u32 = 1 << 2;
pub const MLO_AN_PHY: u32 = 0;
pub const MLO_AN_FIXED: u32 = 1;
pub const MLO_AN_INBAND: u32 = 2;
pub const PHYLINK_PCS_NEG_NONE: u32 = 0;
pub const PHYLINK_PCS_NEG_ENABLED: u32 = 1 << 4;
pub const PHYLINK_PCS_NEG_OUTBAND: u32 = 1 << 5;
pub const PHYLINK_PCS_NEG_INBAND: u32 = 1 << 6;
pub const PHYLINK_PCS_NEG_INBAND_DISABLED: u32 = PHYLINK_PCS_NEG_INBAND;
pub const PHYLINK_PCS_NEG_INBAND_ENABLED: u32 = PHYLINK_PCS_NEG_INBAND | PHYLINK_PCS_NEG_ENABLED;
pub const MAC_SYM_PAUSE: u32 = 1 << 0;
pub const MAC_ASYM_PAUSE: u32 = 1 << 1;
pub const MAC_10HD: u32 = 1 << 2;
pub const MAC_10FD: u32 = 1 << 3;
pub const MAC_10: u32 = MAC_10HD | MAC_10FD;
pub const MAC_100HD: u32 = 1 << 4;
pub const MAC_100FD: u32 = 1 << 5;
pub const MAC_100: u32 = MAC_100HD | MAC_100FD;
pub const MAC_1000HD: u32 = 1 << 6;
pub const MAC_1000FD: u32 = 1 << 7;
pub const MAC_1000: u32 = MAC_1000HD | MAC_1000FD;
pub const MAC_2500FD: u32 = 1 << 8;
pub const MAC_5000FD: u32 = 1 << 9;
pub const MAC_10000FD: u32 = 1 << 10;
pub const MAC_20000FD: u32 = 1 << 11;
pub const MAC_25000FD: u32 = 1 << 12;
pub const MAC_40000FD: u32 = 1 << 13;
pub const MAC_50000FD: u32 = 1 << 14;
pub const MAC_56000FD: u32 = 1 << 15;
pub const MAC_80000FD: u32 = 1 << 16;
pub const MAC_100000FD: u32 = 1 << 17;
pub const MAC_200000FD: u32 = 1 << 18;
pub const MAC_400000FD: u32 = 1 << 19;

#[inline]
pub fn phylink_autoneg_inband(mode: u32) -> bool { mode == MLO_AN_INBAND }

#[repr(C)]
pub struct phylink_link_state {
    pub advertising: [u64; 1],
    pub lp_advertising: [u64; 1],
    pub interface: phy_interface_t,
    pub speed: i32,
    pub duplex: i32,
    pub pause: i32,
    pub rate_matching: i32,
    pub link: u32,
    pub an_complete: u32,
}

#[repr(C)]
pub struct phylink_config {
    pub dev: *mut core::ffi::c_void,
    pub type_: phylink_op_type,
    pub poll_fixed_state: bool,
    pub mac_managed_pm: bool,
    pub mac_requires_rxc: bool,
    pub default_an_inband: bool,
    pub eee_rx_clk_stop_enable: bool,
    pub get_fixed_state: Option<unsafe extern "C" fn(*mut phylink_config, *mut phylink_link_state)>,
    pub supported_interfaces: [u64; 1],
    pub lpi_interfaces: [u64; 1],
    pub mac_capabilities: usize,
    pub lpi_capabilities: usize,
    pub lpi_timer_default: u32,
    pub eee_enabled_default: bool,
    pub wol_phy_legacy: bool,
    pub wol_phy_speed_ctrl: bool,
    pub wol_mac_support: u32,
}

#[repr(C)]
pub enum phylink_op_type { PHYLINK_NETDEV = 0, PHYLINK_DEV }

#[repr(C)]
pub struct phylink_mac_ops {
    pub mac_get_caps: Option<unsafe extern "C" fn(*mut phylink_config, phy_interface_t) -> usize>,
    pub mac_select_pcs: Option<unsafe extern "C" fn(*mut phylink_config, phy_interface_t) -> *mut phylink_pcs>,
    pub mac_prepare: Option<unsafe extern "C" fn(*mut phylink_config, u32, phy_interface_t) -> i32>,
    pub mac_config: Option<unsafe extern "C" fn(*mut phylink_config, u32, *const phylink_link_state)>,
    pub mac_finish: Option<unsafe extern "C" fn(*mut phylink_config, u32, phy_interface_t) -> i32>,
    pub mac_link_down: Option<unsafe extern "C" fn(*mut phylink_config, u32, phy_interface_t)>,
    pub mac_link_up: Option<unsafe extern "C" fn(*mut phylink_config, *mut phy_device, u32, phy_interface_t, i32, i32, bool, bool)>,
    pub mac_disable_tx_lpi: Option<unsafe extern "C" fn(*mut phylink_config)>,
    pub mac_enable_tx_lpi: Option<unsafe extern "C" fn(*mut phylink_config, u32, bool) -> i32>,
    pub mac_wol_set: Option<unsafe extern "C" fn(*mut phylink_config, u32, *const u8) -> i32>,
}

#[repr(C)]
pub struct phylink_pcs { pub supported_interfaces: [u64; 1], pub ops: *const phylink_pcs_ops, pub phylink: *mut phylink, pub poll: bool, pub rxc_always_on: bool }

#[repr(C)]
pub struct phylink_pcs_ops {
    pub pcs_validate: Option<unsafe extern "C" fn(*mut phylink_pcs, *mut usize, *const phylink_link_state) -> i32>,
    pub pcs_inband_caps: Option<unsafe extern "C" fn(*mut phylink_pcs, phy_interface_t) -> u32>,
    pub pcs_enable: Option<unsafe extern "C" fn(*mut phylink_pcs) -> i32>,
    pub pcs_disable: Option<unsafe extern "C" fn(*mut phylink_pcs)>,
    pub pcs_pre_config: Option<unsafe extern "C" fn(*mut phylink_pcs, phy_interface_t)>,
    pub pcs_post_config: Option<unsafe extern "C" fn(*mut phylink_pcs, phy_interface_t) -> i32>,
    pub pcs_get_state: Option<unsafe extern "C" fn(*mut phylink_pcs, u32, *mut phylink_link_state)>,
    pub pcs_config: Option<unsafe extern "C" fn(*mut phylink_pcs, u32, phy_interface_t, *const usize, bool) -> i32>,
    pub pcs_an_restart: Option<unsafe extern "C" fn(*mut phylink_pcs)>,
    pub pcs_link_up: Option<unsafe extern "C" fn(*mut phylink_pcs, u32, phy_interface_t, i32, i32)>,
    pub pcs_disable_eee: Option<unsafe extern "C" fn(*mut phylink_pcs)>,
    pub pcs_enable_eee: Option<unsafe extern "C" fn(*mut phylink_pcs)>,
    pub pcs_pre_init: Option<unsafe extern "C" fn(*mut phylink_pcs) -> i32>,
}

extern "C" {
    pub fn phylink_limit_mac_speed(config: *mut phylink_config, max_speed: u32);
    pub fn phylink_create(config: *mut phylink_config, fwnode: *const fwnode_handle, interface: phy_interface_t, mac_ops: *const phylink_mac_ops) -> *mut phylink;
    pub fn phylink_destroy(pl: *mut phylink);
    pub fn phylink_expects_phy(pl: *mut phylink) -> bool;
    pub fn phylink_connect_phy(pl: *mut phylink, phy: *mut phy_device) -> i32;
    pub fn phylink_of_phy_connect(pl: *mut phylink, node: *mut device_node, flags: u32) -> i32;
    pub fn phylink_fwnode_phy_connect(pl: *mut phylink, fwnode: *const fwnode_handle, flags: u32) -> i32;
    pub fn phylink_disconnect_phy(pl: *mut phylink);
    pub fn phylink_set_fixed_link(pl: *mut phylink, state: *const phylink_link_state) -> i32;
    pub fn phylink_mac_change(pl: *mut phylink, up: bool);
    pub fn phylink_pcs_change(pcs: *mut phylink_pcs, up: bool);
    pub fn phylink_pcs_pre_init(pl: *mut phylink, pcs: *mut phylink_pcs) -> i32;
    pub fn phylink_start(pl: *mut phylink); pub fn phylink_stop(pl: *mut phylink);
    pub fn phylink_rx_clk_stop_block(pl: *mut phylink); pub fn phylink_rx_clk_stop_unblock(pl: *mut phylink);
    pub fn phylink_suspend(pl: *mut phylink, mac_wol: bool); pub fn phylink_prepare_resume(pl: *mut phylink); pub fn phylink_resume(pl: *mut phylink);
    pub fn phylink_ethtool_get_wol(pl: *mut phylink, wol: *mut ethtool_wolinfo);
    pub fn phylink_ethtool_set_wol(pl: *mut phylink, wol: *mut ethtool_wolinfo) -> i32;
    pub fn phylink_ethtool_ksettings_get(pl: *mut phylink, ks: *mut ethtool_link_ksettings) -> i32;
    pub fn phylink_ethtool_ksettings_set(pl: *mut phylink, ks: *const ethtool_link_ksettings) -> i32;
    pub fn phylink_ethtool_nway_reset(pl: *mut phylink) -> i32;
    pub fn phylink_ethtool_get_pauseparam(pl: *mut phylink, p: *mut ethtool_pauseparam);
    pub fn phylink_ethtool_set_pauseparam(pl: *mut phylink, p: *mut ethtool_pauseparam) -> i32;
    pub fn phylink_get_eee_err(pl: *mut phylink) -> i32;
    pub fn phylink_ethtool_get_eee(link: *mut phylink, eee: *mut ethtool_keee) -> i32;
    pub fn phylink_ethtool_set_eee(link: *mut phylink, eee: *mut ethtool_keee) -> i32;
    pub fn phylink_mii_ioctl(pl: *mut phylink, ifr: *mut ifreq, cmd: i32) -> i32;
    pub fn phylink_speed_down(pl: *mut phylink, sync: bool) -> i32; pub fn phylink_speed_up(pl: *mut phylink) -> i32;
    pub fn phylink_set_port_modes(bits: *mut usize);
    pub fn phylink_mii_c22_pcs_decode_state(state: *mut phylink_link_state, neg_mode: u32, bmsr: u16, lpa: u16);
    pub fn phylink_mii_c22_pcs_get_state(pcs: *mut mdio_device, neg_mode: u32, state: *mut phylink_link_state);
    pub fn phylink_mii_c22_pcs_encode_advertisement(interface: phy_interface_t, advertising: *const usize) -> i32;
    pub fn phylink_mii_c22_pcs_config(pcs: *mut mdio_device, interface: phy_interface_t, advertising: *const usize, neg_mode: u32) -> i32;
    pub fn phylink_mii_c22_pcs_an_restart(pcs: *mut mdio_device);
    pub fn phylink_resolve_c73(state: *mut phylink_link_state);
    pub fn phylink_mii_c45_pcs_get_state(pcs: *mut mdio_device, state: *mut phylink_link_state);
    pub fn phylink_decode_usxgmii_word(state: *mut phylink_link_state, lpa: u16);
    pub fn phylink_replay_link_begin(pl: *mut phylink); pub fn phylink_replay_link_end(pl: *mut phylink);
}

#[inline]
pub fn phylink_get_link_timer_ns(interface: phy_interface_t) -> i32 {
    match interface {
        PHY_INTERFACE_MODE_SGMII | PHY_INTERFACE_MODE_PSGMII | PHY_INTERFACE_MODE_QSGMII | PHY_INTERFACE_MODE_USXGMII | PHY_INTERFACE_MODE_10G_QXGMII => 1_600_000,
        PHY_INTERFACE_MODE_1000BASEX | PHY_INTERFACE_MODE_2500BASEX => 10_000_000,
        _ => -22,
    }
}

// External constants from linux/phy.h.
pub const PHY_INTERFACE_MODE_SGMII: phy_interface_t = 0;
pub const PHY_INTERFACE_MODE_PSGMII: phy_interface_t = 1;
pub const PHY_INTERFACE_MODE_QSGMII: phy_interface_t = 2;
pub const PHY_INTERFACE_MODE_USXGMII: phy_interface_t = 3;
pub const PHY_INTERFACE_MODE_10G_QXGMII: phy_interface_t = 4;
pub const PHY_INTERFACE_MODE_1000BASEX: phy_interface_t = 5;
pub const PHY_INTERFACE_MODE_2500BASEX: phy_interface_t = 6;

#[inline]
pub unsafe fn phylink_mac_implements_lpi(ops: *const phylink_mac_ops) -> bool {
    !ops.is_null() && (*ops).mac_disable_tx_lpi.is_some() && (*ops).mac_enable_tx_lpi.is_some()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
