/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of phy.h.  Kernel dependencies are supplied externally. */

use core::ffi::{c_char, c_int, c_void};

pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type s8 = i8;
pub type irqreturn_t = c_int;

extern "C" {
    pub static mut phy_basic_features: [usize; 0];
    pub static mut phy_basic_t1_features: [usize; 0];
    pub static mut phy_basic_t1s_p2mp_features: [usize; 0];
    pub static mut phy_gbit_features: [usize; 0];
    pub static mut phy_gbit_fibre_features: [usize; 0];
    pub static mut phy_10gbit_features: [usize; 0];
    pub static mut phy_eee_cap1_features: [usize; 0];
    pub static mut phy_eee_cap2_features: [usize; 0];
}

pub const PHY_POLL: c_int = -1;
pub const PHY_MAC_INTERRUPT: c_int = -2;
pub const PHY_IS_INTERNAL: u32 = 0x00000001;
pub const PHY_RST_AFTER_CLK_EN: u32 = 0x00000002;
pub const PHY_POLL_CABLE_TEST: u32 = 0x00000004;
pub const PHY_ALWAYS_CALL_SUSPEND: u32 = 0x00000008;
pub const MDIO_DEVICE_IS_PHY: u32 = 0x80000000;

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_interface_t {
    PHY_INTERFACE_MODE_NA, PHY_INTERFACE_MODE_INTERNAL, PHY_INTERFACE_MODE_MII,
    PHY_INTERFACE_MODE_GMII, PHY_INTERFACE_MODE_SGMII, PHY_INTERFACE_MODE_TBI,
    PHY_INTERFACE_MODE_REVMII, PHY_INTERFACE_MODE_RMII, PHY_INTERFACE_MODE_REVRMII,
    PHY_INTERFACE_MODE_RGMII, PHY_INTERFACE_MODE_RGMII_ID, PHY_INTERFACE_MODE_RGMII_RXID,
    PHY_INTERFACE_MODE_RGMII_TXID, PHY_INTERFACE_MODE_RTBI, PHY_INTERFACE_MODE_SMII,
    PHY_INTERFACE_MODE_XGMII, PHY_INTERFACE_MODE_XLGMII, PHY_INTERFACE_MODE_MOCA,
    PHY_INTERFACE_MODE_PSGMII, PHY_INTERFACE_MODE_QSGMII, PHY_INTERFACE_MODE_TRGMII,
    PHY_INTERFACE_MODE_100BASEX, PHY_INTERFACE_MODE_1000BASEX, PHY_INTERFACE_MODE_2500BASEX,
    PHY_INTERFACE_MODE_5GBASER, PHY_INTERFACE_MODE_RXAUI, PHY_INTERFACE_MODE_XAUI,
    PHY_INTERFACE_MODE_10GBASER, PHY_INTERFACE_MODE_25GBASER, PHY_INTERFACE_MODE_USXGMII,
    PHY_INTERFACE_MODE_10GKR, PHY_INTERFACE_MODE_QUSGMII, PHY_INTERFACE_MODE_1000BASEKX,
    PHY_INTERFACE_MODE_10G_QXGMII, PHY_INTERFACE_MODE_50GBASER, PHY_INTERFACE_MODE_LAUI,
    PHY_INTERFACE_MODE_100GBASEP, PHY_INTERFACE_MODE_MIILITE, PHY_INTERFACE_MODE_MAX,
}

pub const PHY_MAX_ADDR: usize = 32;
pub const MII_BUS_ID_SIZE: usize = 61;
pub const MDIO_MMD_NUM: usize = 32;
pub const PHY_PAIR_ALL: s8 = -1;
pub const PHY_MSE_CAP_CHANNEL_A: u32 = 1 << 0;
pub const PHY_MSE_CAP_CHANNEL_B: u32 = 1 << 1;
pub const PHY_MSE_CAP_CHANNEL_C: u32 = 1 << 2;
pub const PHY_MSE_CAP_CHANNEL_D: u32 = 1 << 3;
pub const PHY_MSE_CAP_WORST_CHANNEL: u32 = 1 << 4;
pub const PHY_MSE_CAP_LINK: u32 = 1 << 5;
pub const PHY_MSE_CAP_AVG: u32 = 1 << 6;
pub const PHY_MSE_CAP_PEAK: u32 = 1 << 7;
pub const PHY_MSE_CAP_WORST_PEAK: u32 = 1 << 8;

#[repr(C)] pub struct mdio_bus_stats { pub transfers:u64, pub errors:u64, pub writes:u64, pub reads:u64, pub syncp:c_void }
#[repr(C)] pub struct phy_c45_device_ids { pub devices_in_package:u32, pub mmds_present:u32, pub device_ids:[u32;MDIO_MMD_NUM] }
#[repr(C)] pub struct phy_oatc14_sqi_capability { pub updated:bool, pub sqi_max:c_int, pub sqiplus_bits:u8 }
#[repr(C)] pub struct phy_tdr_config { pub first:u32, pub last:u32, pub step:u32, pub pair:s8 }
#[repr(C)] pub struct phy_plca_cfg { pub version:c_int, pub enabled:c_int, pub node_id:c_int, pub node_cnt:c_int, pub to_tmr:c_int, pub burst_cnt:c_int, pub burst_tmr:c_int }
#[repr(C)] pub struct phy_plca_status { pub pst:bool }
#[repr(C)] pub struct phy_mse_capability { pub max_average_mse:u64, pub max_peak_mse:u64, pub refresh_rate_ps:u64, pub num_symbols:u64, pub supported_caps:u32 }
#[repr(C)] pub struct phy_mse_snapshot { pub average_mse:u64, pub peak_mse:u64, pub worst_peak_mse:u64 }

#[repr(C)] #[derive(Copy,Clone,PartialEq,Eq)] pub enum phy_state { PHY_DOWN=0, PHY_READY, PHY_HALTED, PHY_ERROR, PHY_UP, PHY_RUNNING, PHY_NOLINK, PHY_CABLETEST }
#[repr(C)] pub enum link_inband_signalling { LINK_INBAND_DISABLE=1, LINK_INBAND_ENABLE=2, LINK_INBAND_BYPASS=4 }
#[repr(C)] pub enum phy_mse_channel { PHY_MSE_CHANNEL_A, PHY_MSE_CHANNEL_B, PHY_MSE_CHANNEL_C, PHY_MSE_CHANNEL_D, PHY_MSE_CHANNEL_WORST, PHY_MSE_CHANNEL_LINK }
#[repr(C)] pub enum phy_led_modes { PHY_LED_ACTIVE_HIGH=0, PHY_LED_ACTIVE_LOW=1, PHY_LED_INACTIVE_HIGH_IMPEDANCE=2, __PHY_LED_MODES_NUM }

/* External kernel structures are intentionally left as declarations. */
pub enum device {} pub enum module {} pub enum mdio_device {} pub enum mdio_driver_common {}
pub enum phy_driver {} pub enum phy_package_shared {} pub enum phy_port {} pub enum sfp_bus {}
pub enum phylink {} pub enum net_device {} pub enum mii_timestamper {} pub enum pse_control {}
pub enum sk_buff {} pub enum nlattr {} pub enum eee_config {} pub enum led_classdev {}
pub enum phy_led_trigger {} pub enum ethtool_wolinfo {} pub enum ethtool_modinfo {}
pub enum ethtool_eeprom {} pub enum ethtool_eth_phy_stats {} pub enum ethtool_phy_stats {}
pub enum ethtool_link_ext_stats {} pub enum ethtool_stats {} pub enum ethtool_tunable {}
pub enum kernel_hwtstamp_config {} pub enum netlink_ext_ack {}

#[repr(C)] pub struct phy_device {
    pub mdio: mdio_device, pub drv:*const phy_driver, pub devlink:*mut c_void,
    pub phyindex:u32, pub phy_id:u32, pub c45_ids:phy_c45_device_ids,
    pub is_c45:u32, pub is_internal:u32, pub is_pseudo_fixed_link:u32, pub is_gigabit_capable:u32,
    pub has_fixups:u32, pub suspended:u32, pub suspended_by_mdio_bus:u32, pub sysfs_links:u32,
    pub loopback_enabled:u32, pub downshifted_rate:u32, pub is_on_sfp_module:u32, pub mac_managed_pm:u32,
    pub wol_enabled:u32, pub is_genphy_driven:u32, pub autoneg:u32, pub link:u32,
    pub autoneg_complete:u32, pub pause:bool, pub asym_pause:bool, pub interrupts:u32,
    pub irq_suspended:u32, pub irq_rerun:u32, pub default_timestamp:u32, pub rate_matching:c_int,
    pub state:phy_state, pub dev_flags:u32, pub interface:phy_interface_t, pub possible_interfaces:[usize;1],
    pub speed:c_int, pub duplex:c_int, pub port:c_int, pub master_slave_get:u8, pub master_slave_set:u8, pub master_slave_state:u8,
    pub supported:[usize;1], pub advertising:[usize;1], pub lp_advertising:[usize;1], pub adv_old:[usize;1],
    pub supported_eee:[usize;1], pub advertising_eee:[usize;1], pub eee_disabled_modes:[usize;1],
    pub enable_tx_lpi:bool, pub eee_active:bool, pub autonomous_eee_disabled:bool, pub eee_cfg:eee_config,
    pub host_interfaces:[usize;1], pub leds:c_void, pub irq:c_int, pub priv_:*mut c_void, pub skb:*mut sk_buff,
    pub ehdr:*mut c_void, pub nest:*mut nlattr, pub state_queue:c_void, pub lock:c_void,
    pub sfp_bus_attached:bool, pub sfp_bus:*mut sfp_bus, pub phylink:*mut phylink, pub attached_dev:*mut net_device,
    pub mii_ts:*mut mii_timestamper, pub psec:*mut pse_control, pub ports:c_void, pub n_ports:c_int, pub max_n_ports:c_int,
    pub mdix:u8, pub mdix_ctrl:u8, pub pma_extable:c_int, pub link_down_events:u32,
    pub phy_link_change:Option<unsafe extern "C" fn(*mut phy_device,bool)>,
    pub adjust_link:Option<unsafe extern "C" fn(*mut net_device)>, pub oatc14_sqi_capability:phy_oatc14_sqi_capability,
}

pub const PHY_F_NO_IRQ:u32=0x80000000; pub const PHY_F_RXC_ALWAYS_ON:u32=0x40000000; pub const PHY_F_KEEP_PREAMBLE_BEFORE_SFD:u32=0x20000000;
pub const LINK_INBAND_DISABLE:u32=1; pub const LINK_INBAND_ENABLE:u32=2; pub const LINK_INBAND_BYPASS:u32=4;
pub const PHY_INTERRUPT_DISABLED:bool=false; pub const PHY_INTERRUPT_ENABLED:bool=true;

pub const PHY_ID_MATCH_EXTACT_MASK:u32=0xffff_ffff; pub const PHY_ID_MATCH_MODEL_MASK:u32=0xffff_fff0; pub const PHY_ID_MATCH_VENDOR_MASK:u32=0xffff_fc00;
pub const fn phy_id_compare(id1:u32,id2:u32,mask:u32)->bool { ((id1^id2)&mask)==0 }
pub const fn phy_id_compare_vendor(id:u32,vendor_mask:u32)->bool { phy_id_compare(id,vendor_mask,PHY_ID_MATCH_VENDOR_MASK) }
pub const fn phy_id_compare_model(id:u32,model_mask:u32)->bool { phy_id_compare(id,model_mask,PHY_ID_MATCH_MODEL_MASK) }
pub const fn phy_is_started(phydev:&phy_device)->bool { (phydev.state as u32)>=(phy_state::PHY_UP as u32) }

extern "C" {
    pub fn phy_speed_to_str(speed:c_int)->*const c_char; pub fn phy_duplex_to_str(duplex:u32)->*const c_char; pub fn phy_rate_matching_to_str(rate_matching:c_int)->*const c_char;
    pub fn phy_interface_num_ports(interface:phy_interface_t)->c_int; pub fn phy_may_wakeup(phydev:*mut phy_device)->bool;
    pub fn phy_resolve_aneg_pause(phydev:*mut phy_device); pub fn phy_resolve_aneg_linkmode(phydev:*mut phy_device);
    pub fn phy_read_mmd(phydev:*mut phy_device,devad:c_int,regnum:u32)->c_int; pub fn phy_write_mmd(phydev:*mut phy_device,devad:c_int,regnum:u32,val:u16)->c_int;
    pub fn __phy_read_mmd(phydev:*mut phy_device,devad:c_int,regnum:u32)->c_int; pub fn __phy_write_mmd(phydev:*mut phy_device,devad:c_int,regnum:u32,val:u16)->c_int;
    pub fn phy_modify_changed(phydev:*mut phy_device,regnum:u32,mask:u16,set:u16)->c_int; pub fn __phy_modify_changed(phydev:*mut phy_device,regnum:u32,mask:u16,set:u16)->c_int;
    pub fn phy_modify(phydev:*mut phy_device,regnum:u32,mask:u16,set:u16)->c_int; pub fn __phy_modify(phydev:*mut phy_device,regnum:u32,mask:u16,set:u16)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
