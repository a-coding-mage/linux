/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/phy/phy.h. Kernel includes are external dependencies. */

use core::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum phy_mode {
    PHY_MODE_INVALID,
    PHY_MODE_USB_HOST,
    PHY_MODE_USB_HOST_LS,
    PHY_MODE_USB_HOST_FS,
    PHY_MODE_USB_HOST_HS,
    PHY_MODE_USB_HOST_SS,
    PHY_MODE_USB_DEVICE,
    PHY_MODE_USB_DEVICE_LS,
    PHY_MODE_USB_DEVICE_FS,
    PHY_MODE_USB_DEVICE_HS,
    PHY_MODE_USB_DEVICE_SS,
    PHY_MODE_USB_OTG,
    PHY_MODE_UFS_HS_A,
    PHY_MODE_UFS_HS_B,
    PHY_MODE_PCIE,
    PHY_MODE_ETHERNET,
    PHY_MODE_MIPI_DPHY,
    PHY_MODE_SATA,
    PHY_MODE_LVDS,
    PHY_MODE_DP,
    PHY_MODE_HDMI,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum phy_media { PHY_MEDIA_DEFAULT, PHY_MEDIA_SR, PHY_MEDIA_DAC }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum phy_ufs_state { PHY_UFS_HIBERN8_ENTER, PHY_UFS_HIBERN8_EXIT }

#[repr(C)]
pub union phy_notify { pub ufs_state: phy_ufs_state }

/* Definitions supplied by the included phy-* headers. */
#[repr(C)] pub struct phy_configure_opts_mipi_dphy { _private: [u8; 0] }
#[repr(C)] pub struct phy_configure_opts_dp { _private: [u8; 0] }
#[repr(C)] pub struct phy_configure_opts_lvds { _private: [u8; 0] }
#[repr(C)] pub struct phy_configure_opts_hdmi { _private: [u8; 0] }
#[repr(C)]
pub union phy_configure_opts {
    pub mipi_dphy: phy_configure_opts_mipi_dphy,
    pub dp: phy_configure_opts_dp,
    pub lvds: phy_configure_opts_lvds,
    pub hdmi: phy_configure_opts_hdmi,
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct of_phandle_args { _private: [u8; 0] }

pub type PhyFn = unsafe extern "C" fn(*mut phy) -> i32;
pub type SetModeFn = unsafe extern "C" fn(*mut phy, phy_mode, i32) -> i32;
pub type SetMediaFn = unsafe extern "C" fn(*mut phy, phy_media) -> i32;
pub type ConfigureFn = unsafe extern "C" fn(*mut phy, *mut phy_configure_opts) -> i32;
pub type ValidateFn = unsafe extern "C" fn(*mut phy, phy_mode, i32, *mut phy_configure_opts) -> i32;
pub type NotifyFn = unsafe extern "C" fn(*mut phy, phy_notify) -> i32;

#[repr(C)]
pub struct phy_ops {
    pub init: Option<PhyFn>, pub exit: Option<PhyFn>,
    pub power_on: Option<PhyFn>, pub power_off: Option<PhyFn>,
    pub set_mode: Option<SetModeFn>, pub set_media: Option<SetMediaFn>,
    pub set_speed: Option<unsafe extern "C" fn(*mut phy, i32) -> i32>,
    pub configure: Option<ConfigureFn>, pub validate: Option<ValidateFn>,
    pub reset: Option<PhyFn>, pub calibrate: Option<PhyFn>,
    pub connect: Option<unsafe extern "C" fn(*mut phy, i32) -> i32>,
    pub disconnect: Option<unsafe extern "C" fn(*mut phy, i32) -> i32>,
    pub notify_phystate: Option<NotifyFn>,
    pub release: Option<unsafe extern "C" fn(*mut phy)>,
    pub owner: *mut module,
}

#[repr(C)] pub struct phy_attrs { pub bus_width: u32, pub max_link_rate: u32, pub mode: phy_mode }
#[repr(C)] pub struct phy {
    pub dev: device, pub id: i32, pub ops: *const phy_ops, pub mutex: mutex,
    pub lockdep_key: lock_class_key, pub init_count: i32, pub power_count: i32,
    pub attrs: phy_attrs, pub pwr: *mut regulator, pub debugfs: *mut dentry,
}
#[repr(C)] pub struct phy_provider {
    pub dev: *mut device, pub children: *mut device_node, pub owner: *mut module,
    pub list: list_head,
    pub of_xlate: Option<unsafe extern "C" fn(*mut device, *const of_phandle_args) -> *mut phy>,
}
#[repr(C)] pub struct phy_lookup { pub node: list_head, pub dev_id: *const i8, pub con_id: *const i8, pub phy: *mut phy }

/* Registration macros are represented as direct calls to their underlying declarations. */
extern "C" {
    pub fn __of_phy_provider_register(dev: *mut device, children: *mut device_node, owner: *mut module,
        xlate: Option<unsafe extern "C" fn(*mut device, *const of_phandle_args) -> *mut phy>) -> *mut phy_provider;
    pub fn __devm_of_phy_provider_register(dev: *mut device, children: *mut device_node, owner: *mut module,
        xlate: Option<unsafe extern "C" fn(*mut device, *const of_phandle_args) -> *mut phy_provider>);
}

#[inline] pub unsafe fn phy_set_drvdata(_phy: *mut phy, _data: *mut c_void) { }
#[inline] pub unsafe fn phy_get_drvdata(_phy: *mut phy) -> *mut c_void { core::ptr::null_mut() }

/* CONFIG_GENERIC_PHY is a build-time condition; both API variants are preserved below. */
#[cfg(generic_phy)]
extern "C" {
    pub fn phy_pm_runtime_get(*mut phy) -> i32; pub fn phy_pm_runtime_get_sync(*mut phy) -> i32;
    pub fn phy_pm_runtime_put(*mut phy); pub fn phy_pm_runtime_put_sync(*mut phy) -> i32;
    pub fn phy_init(*mut phy) -> i32; pub fn phy_exit(*mut phy) -> i32;
    pub fn phy_power_on(*mut phy) -> i32; pub fn phy_power_off(*mut phy) -> i32;
    pub fn phy_set_mode_ext(*mut phy, phy_mode, i32) -> i32;
    pub fn phy_set_media(*mut phy, phy_media) -> i32; pub fn phy_set_speed(*mut phy, i32) -> i32;
    pub fn phy_configure(*mut phy, *mut phy_configure_opts) -> i32;
    pub fn phy_validate(*mut phy, phy_mode, i32, *mut phy_configure_opts) -> i32;
    pub fn phy_reset(*mut phy) -> i32; pub fn phy_calibrate(*mut phy) -> i32;
    pub fn phy_notify_connect(*mut phy, i32) -> i32; pub fn phy_notify_disconnect(*mut phy, i32) -> i32;
    pub fn phy_notify_state(*mut phy, phy_notify) -> i32;
    pub fn phy_get(*mut device, *const i8) -> *mut phy; pub fn devm_phy_get(*mut device, *const i8) -> *mut phy;
    pub fn devm_phy_optional_get(*mut device, *const i8) -> *mut phy;
    pub fn devm_of_phy_get(*mut device, *mut device_node, *const i8) -> *mut phy;
    pub fn devm_of_phy_optional_get(*mut device, *mut device_node, *const i8) -> *mut phy;
    pub fn devm_of_phy_get_by_index(*mut device, *mut device_node, i32) -> *mut phy;
    pub fn of_phy_put(*mut phy); pub fn phy_put(*mut device, *mut phy); pub fn devm_phy_put(*mut device, *mut phy);
    pub fn of_phy_get(*mut device_node, *const i8) -> *mut phy;
    pub fn of_phy_simple_xlate(*mut device, *const of_phandle_args) -> *mut phy;
    pub fn phy_create(*mut device, *mut device_node, *const phy_ops) -> *mut phy;
    pub fn devm_phy_create(*mut device, *mut device_node, *const phy_ops) -> *mut phy;
    pub fn phy_destroy(*mut phy); pub fn devm_phy_destroy(*mut device, *mut phy);
    pub fn of_phy_provider_unregister(*mut phy_provider); pub fn devm_of_phy_provider_unregister(*mut device, *mut phy_provider);
    pub fn phy_create_lookup(*mut phy, *const i8, *const i8) -> i32;
    pub fn phy_remove_lookup(*mut phy, *const i8, *const i8);
}

#[inline] pub unsafe fn phy_get_mode(phy: *mut phy) -> phy_mode { (*phy).attrs.mode }
#[inline] pub unsafe fn phy_get_bus_width(phy: *mut phy) -> i32 { (*phy).attrs.bus_width as i32 }
#[inline] pub unsafe fn phy_set_bus_width(phy: *mut phy, bus_width: i32) { (*phy).attrs.bus_width = bus_width as u32; }

#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_pm_runtime_get(phy: *mut phy) -> i32 { if phy.is_null() { 0 } else { -38 } }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_pm_runtime_get_sync(phy: *mut phy) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_pm_runtime_put(_phy: *mut phy) {}
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_pm_runtime_put_sync(phy: *mut phy) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_init(phy: *mut phy) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_exit(phy: *mut phy) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_power_on(phy: *mut phy) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_power_off(phy: *mut phy) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_set_mode_ext(phy: *mut phy, _mode: phy_mode, _submode: i32) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_set_media(phy: *mut phy, _media: phy_media) -> i32 { if phy.is_null() { 0 } else { -19 } }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_set_speed(phy: *mut phy, _speed: i32) -> i32 { phy_set_media(phy, phy_media::PHY_MEDIA_DEFAULT) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_get_mode(_phy: *mut phy) -> phy_mode { phy_mode::PHY_MODE_INVALID }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_reset(phy: *mut phy) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_calibrate(phy: *mut phy) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_notify_connect(phy: *mut phy, _index: i32) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_notify_disconnect(phy: *mut phy, _index: i32) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_notify_state(phy: *mut phy, _state: phy_notify) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_configure(phy: *mut phy, _opts: *mut phy_configure_opts) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_validate(phy: *mut phy, _mode: phy_mode, _submode: i32, _opts: *mut phy_configure_opts) -> i32 { phy_pm_runtime_get(phy) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_get_bus_width(_phy: *mut phy) -> i32 { -38 }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_set_bus_width(_phy: *mut phy, _bus_width: i32) {}

#[inline] pub unsafe fn phy_set_mode(phy: *mut phy, mode: phy_mode) -> i32 { phy_set_mode_ext(phy, mode, 0) }

#[cfg(not(generic_phy))]
#[inline] pub unsafe fn of_phy_provider_unregister(_p: *mut phy_provider) {}
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn devm_of_phy_provider_unregister(_dev: *mut device, _p: *mut phy_provider) {}
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_create_lookup(_phy: *mut phy, _con_id: *const i8, _dev_id: *const i8) -> i32 { 0 }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_remove_lookup(_phy: *mut phy, _con_id: *const i8, _dev_id: *const i8) {}

#[inline]
pub unsafe fn to_phy(dev: *mut device) -> *mut phy {
    (dev as *mut u8).sub(core::mem::offset_of!(phy, dev)) as *mut phy
}

#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_get(_dev: *mut device, _string: *const i8) -> *mut phy { (-38isize) as *mut phy }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn devm_phy_get(dev: *mut device, string: *const i8) -> *mut phy { phy_get(dev, string) }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn devm_phy_optional_get(_dev: *mut device, _string: *const i8) -> *mut phy { core::ptr::null_mut() }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn devm_of_phy_get(_dev: *mut device, _np: *mut device_node, _id: *const i8) -> *mut phy { (-38isize) as *mut phy }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn devm_of_phy_optional_get(_dev: *mut device, _np: *mut device_node, _id: *const i8) -> *mut phy { core::ptr::null_mut() }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn devm_of_phy_get_by_index(_dev: *mut device, _np: *mut device_node, _index: i32) -> *mut phy { (-38isize) as *mut phy }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn of_phy_put(_phy: *mut phy) {}
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_put(_dev: *mut device, _phy: *mut phy) {}
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn devm_phy_put(_dev: *mut device, _phy: *mut phy) {}
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn of_phy_get(_np: *mut device_node, _id: *const i8) -> *mut phy { (-38isize) as *mut phy }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn of_phy_simple_xlate(_dev: *mut device, _args: *const of_phandle_args) -> *mut phy { (-38isize) as *mut phy }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_create(_dev: *mut device, _node: *mut device_node, _ops: *const phy_ops) -> *mut phy { (-38isize) as *mut phy }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn devm_phy_create(_dev: *mut device, _node: *mut device_node, _ops: *const phy_ops) -> *mut phy { (-38isize) as *mut phy }
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn phy_destroy(_phy: *mut phy) {}
#[cfg(not(generic_phy))]
#[inline] pub unsafe fn devm_phy_destroy(_dev: *mut device, _phy: *mut phy) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
