/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/mdio.h.  Names supplied by other headers remain external. */

use core::ffi::c_void;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct mii_bus { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct reset_control { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct ethtool_link_ksettings { _private: [u8; 0] }
#[repr(C)] pub struct mii_ioctl_data { _private: [u8; 0] }
#[repr(C)] pub struct phy_device { _private: [u8; 0] }

#[repr(C)]
pub enum mdio_mutex_lock_class { MDIO_MUTEX_NORMAL, MDIO_MUTEX_MUX, MDIO_MUTEX_NESTED }

#[repr(C)]
pub struct mdio_device {
    pub dev: device,
    pub bus: *mut mii_bus,
    pub bus_match: Option<unsafe extern "C" fn(*mut device, *const device_driver) -> i32>,
    pub device_free: Option<unsafe extern "C" fn(*mut mdio_device)>,
    pub device_remove: Option<unsafe extern "C" fn(*mut mdio_device)>,
    pub addr: i32,
    pub flags: i32,
    pub reset_state: i32,
    pub reset_gpio: *mut gpio_desc,
    pub reset_ctrl: *mut reset_control,
    pub reset_assert_delay: u32,
    pub reset_deassert_delay: u32,
}

#[repr(C)] pub struct mdio_driver_common { pub driver: device_driver, pub flags: i32 }
pub const MDIO_DEVICE_FLAG_PHY: i32 = 1;

#[repr(C)]
pub struct mdio_driver {
    pub mdiodrv: mdio_driver_common,
    pub probe: Option<unsafe extern "C" fn(*mut mdio_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut mdio_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut mdio_device)>,
}

#[repr(C)]
pub struct mdio_if_info {
    pub prtad: i32,
    pub mmds: u32,
    pub mode_support: u32,
    pub dev: *mut net_device,
    pub mdio_read: Option<unsafe extern "C" fn(*mut net_device, i32, i32, u16) -> i32>,
    pub mdio_write: Option<unsafe extern "C" fn(*mut net_device, i32, i32, u16, u16) -> i32>,
}

pub const MDIO_PRTAD_NONE: i32 = -1;
pub const MDIO_DEVAD_NONE: i32 = -1;
pub const MDIO_SUPPORTS_C22: u32 = 1;
pub const MDIO_SUPPORTS_C45: u32 = 2;
pub const MDIO_EMULATE_C22: u32 = 4;

pub const MDIO_DEVICE_CONTAINER_OFFSET: usize = 0;

extern "C" {
    pub fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    pub fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    pub fn get_device(dev: *mut device);
    pub fn mdio_device_free(mdiodev: *mut mdio_device);
    pub fn mdio_device_create(bus: *mut mii_bus, addr: i32) -> *mut mdio_device;
    pub fn mdio_device_register(mdiodev: *mut mdio_device) -> i32;
    pub fn mdio_device_remove(mdiodev: *mut mdio_device);
    pub fn mdio_device_reset(mdiodev: *mut mdio_device, value: i32);
    pub fn mdio_driver_register(drv: *mut mdio_driver) -> i32;
    pub fn mdio_driver_unregister(drv: *mut mdio_driver);
    pub fn mdio45_probe(mdio: *mut mdio_if_info, prtad: i32) -> i32;
    pub fn mdio_set_flag(mdio: *const mdio_if_info, prtad: i32, devad: i32, addr: u16, mask: i32, sense: bool) -> i32;
    pub fn mdio45_links_ok(mdio: *const mdio_if_info, mmds: u32) -> i32;
    pub fn mdio45_nway_restart(mdio: *const mdio_if_info) -> i32;
    pub fn mdio45_ethtool_ksettings_get_npage(mdio: *const mdio_if_info, cmd: *mut ethtool_link_ksettings, npage_adv: u32, npage_lpa: u32);
    pub fn mdio_mii_ioctl(mdio: *const mdio_if_info, mii_data: *mut mii_ioctl_data, cmd: i32) -> i32;
    pub fn __mdiobus_read(bus: *mut mii_bus, addr: i32, regnum: u32) -> i32;
    pub fn __mdiobus_write(bus: *mut mii_bus, addr: i32, regnum: u32, val: u16) -> i32;
    pub fn __mdiobus_modify(bus: *mut mii_bus, addr: i32, regnum: u32, mask: u16, set: u16) -> i32;
    pub fn __mdiobus_modify_changed(bus: *mut mii_bus, addr: i32, regnum: u32, mask: u16, set: u16) -> i32;
    pub fn mdiobus_read(bus: *mut mii_bus, addr: i32, regnum: u32) -> i32;
    pub fn mdiobus_read_nested(bus: *mut mii_bus, addr: i32, regnum: u32) -> i32;
    pub fn mdiobus_write(bus: *mut mii_bus, addr: i32, regnum: u32, val: u16) -> i32;
    pub fn mdiobus_write_nested(bus: *mut mii_bus, addr: i32, regnum: u32, val: u16) -> i32;
    pub fn mdiobus_modify(bus: *mut mii_bus, addr: i32, regnum: u32, mask: u16, set: u16) -> i32;
    pub fn mdiobus_modify_changed(bus: *mut mii_bus, addr: i32, regnum: u32, mask: u16, set: u16) -> i32;
    pub fn __mdiobus_c45_read(bus: *mut mii_bus, addr: i32, devad: i32, regnum: u32) -> i32;
    pub fn mdiobus_c45_read(bus: *mut mii_bus, addr: i32, devad: i32, regnum: u32) -> i32;
    pub fn mdiobus_c45_read_nested(bus: *mut mii_bus, addr: i32, devad: i32, regnum: u32) -> i32;
    pub fn __mdiobus_c45_write(bus: *mut mii_bus, addr: i32, devad: i32, regnum: u32, val: u16) -> i32;
    pub fn mdiobus_c45_write(bus: *mut mii_bus, addr: i32, devad: i32, regnum: u32, val: u16) -> i32;
    pub fn mdiobus_c45_write_nested(bus: *mut mii_bus, addr: i32, devad: i32, regnum: u32, val: u16) -> i32;
    pub fn mdiobus_c45_modify(bus: *mut mii_bus, addr: i32, devad: i32, regnum: u32, mask: u16, set: u16) -> i32;
    pub fn mdiobus_c45_modify_changed(bus: *mut mii_bus, addr: i32, devad: i32, regnum: u32, mask: u16, set: u16) -> i32;
    pub fn mdiobus_is_registered_device(bus: *mut mii_bus, addr: i32) -> bool;
    pub fn mdiobus_get_phy(bus: *mut mii_bus, addr: i32) -> *mut phy_device;
    pub fn linkmode_test_bit(bit: u32, bitmap: *mut usize) -> bool;
    pub fn linkmode_mod_bit(bit: u32, bitmap: *mut usize, value: bool);
}

#[inline] pub unsafe fn to_mdio_device(dev: *mut device) -> *mut mdio_device { dev as *mut mdio_device }
#[inline] pub unsafe fn to_mdio_common_driver(drv: *mut device_driver) -> *mut mdio_driver_common { drv as *mut mdio_driver_common }
#[inline] pub unsafe fn to_mdio_driver(drv: *mut device_driver) -> *mut mdio_driver { drv as *mut mdio_driver }
#[inline] pub unsafe fn mdiodev_set_drvdata(mdio: *mut mdio_device, data: *mut c_void) { dev_set_drvdata(&mut (*mdio).dev, data) }
#[inline] pub unsafe fn mdiodev_get_drvdata(mdio: *mut mdio_device) -> *mut c_void { dev_get_drvdata(&mut (*mdio).dev) }
#[inline] pub unsafe fn mdiodev_has_reset(mdio: *mut mdio_device) -> bool { !(*mdio).reset_gpio.is_null() || !(*mdio).reset_ctrl.is_null() }
#[inline] pub unsafe fn mdio_device_get(mdio: *mut mdio_device) { get_device(&mut (*mdio).dev) }
#[inline] pub unsafe fn mdio_device_put(mdio: *mut mdio_device) { mdio_device_free(mdio) }

// Constants below are supplied by uapi/linux/mdio.h and ethtool headers.
extern "C" {
    pub static MDIO_PHY_ID_C45: i32; pub static MDIO_PHY_ID_C45_MASK: i32;
    pub static MDIO_PHY_ID_PRTAD: i32; pub static MDIO_PHY_ID_DEVAD: i32;
    pub static MDIO_EEE_100TX: u32; pub static MDIO_EEE_1000T: u32; pub static MDIO_EEE_10GT: u32;
    pub static MDIO_EEE_1000KX: u32; pub static MDIO_EEE_10GKX4: u32; pub static MDIO_EEE_10GKR: u32;
    pub static MDIO_EEE_2_5GT: u32; pub static MDIO_EEE_5GT: u32;
    pub static MDIO_AN_10GBT_CTRL_ADV2_5G: u32; pub static MDIO_AN_10GBT_CTRL_ADV5G: u32; pub static MDIO_AN_10GBT_CTRL_ADV10G: u32;
    pub static MDIO_AN_10GBT_STAT_LP2_5G: u32; pub static MDIO_AN_10GBT_STAT_LP5G: u32; pub static MDIO_AN_10GBT_STAT_LP10G: u32;
    pub static MDIO_AN_T1_ADV_L_PAUSE_CAP: u32; pub static MDIO_AN_T1_ADV_L_PAUSE_ASYM: u32;
    pub static MDIO_AN_T1_ADV_M_B10L: u32; pub static MDIO_AN_T1_ADV_M_100BT1: u32; pub static MDIO_AN_T1_ADV_M_1000BT1: u32;
}

#[inline] pub fn mdio_phy_id_is_c45(phy_id: i32) -> bool { (phy_id & MDIO_PHY_ID_C45) != 0 && (phy_id & !MDIO_PHY_ID_C45_MASK) == 0 }
#[inline] pub fn mdio_phy_id_prtad(phy_id: i32) -> u16 { ((phy_id & MDIO_PHY_ID_PRTAD) >> 5) as u16 }
#[inline] pub fn mdio_phy_id_devad(phy_id: i32) -> u16 { (phy_id & MDIO_PHY_ID_DEVAD) as u16 }

#[inline] pub unsafe fn mdio45_ethtool_ksettings_get(m: *const mdio_if_info, c: *mut ethtool_link_ksettings) { mdio45_ethtool_ksettings_get_npage(m, c, 0, 0) }

#[inline] pub unsafe fn __mdiodev_read(d: *mut mdio_device, r: u32) -> i32 { __mdiobus_read((*d).bus, (*d).addr, r) }
#[inline] pub unsafe fn __mdiodev_write(d: *mut mdio_device, r: u32, v: u16) -> i32 { __mdiobus_write((*d).bus, (*d).addr, r, v) }
#[inline] pub unsafe fn __mdiodev_modify(d: *mut mdio_device, r: u32, m: u16, s: u16) -> i32 { __mdiobus_modify((*d).bus, (*d).addr, r, m, s) }
#[inline] pub unsafe fn __mdiodev_modify_changed(d: *mut mdio_device, r: u32, m: u16, s: u16) -> i32 { __mdiobus_modify_changed((*d).bus, (*d).addr, r, m, s) }
#[inline] pub unsafe fn mdiodev_read(d: *mut mdio_device, r: u32) -> i32 { mdiobus_read((*d).bus, (*d).addr, r) }
#[inline] pub unsafe fn mdiodev_write(d: *mut mdio_device, r: u32, v: u16) -> i32 { mdiobus_write((*d).bus, (*d).addr, r, v) }
#[inline] pub unsafe fn mdiodev_modify(d: *mut mdio_device, r: u32, m: u16, s: u16) -> i32 { mdiobus_modify((*d).bus, (*d).addr, r, m, s) }
#[inline] pub unsafe fn mdiodev_modify_changed(d: *mut mdio_device, r: u32, m: u16, s: u16) -> i32 { mdiobus_modify_changed((*d).bus, (*d).addr, r, m, s) }
#[inline] pub unsafe fn __mdiodev_c45_read(d: *mut mdio_device, a: i32, r: u16) -> i32 { __mdiobus_c45_read((*d).bus, (*d).addr, a, r as u32) }
#[inline] pub unsafe fn __mdiodev_c45_write(d: *mut mdio_device, a: u32, r: u16, v: u16) -> i32 { __mdiobus_c45_write((*d).bus, (*d).addr, a as i32, r as u32, v) }
#[inline] pub unsafe fn mdiodev_c45_modify(d: *mut mdio_device, a: i32, r: u32, m: u16, s: u16) -> i32 { mdiobus_c45_modify((*d).bus, (*d).addr, a, r, m, s) }
#[inline] pub unsafe fn mdiodev_c45_modify_changed(d: *mut mdio_device, a: i32, r: u32, m: u16, s: u16) -> i32 { mdiobus_c45_modify_changed((*d).bus, (*d).addr, a, r, m, s) }
#[inline] pub unsafe fn mdiodev_c45_read(d: *mut mdio_device, a: i32, r: u16) -> i32 { mdiobus_c45_read((*d).bus, (*d).addr, a, r as u32) }
#[inline] pub unsafe fn mdiodev_c45_write(d: *mut mdio_device, a: u32, r: u16, v: u16) -> i32 { mdiobus_c45_write((*d).bus, (*d).addr, a as i32, r as u32, v) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
