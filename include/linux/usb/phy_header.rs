/* SPDX-License-Identifier: GPL-2.0 */
/*
 * USB PHY defines
 *
 * These APIs may be used between USB controllers.  USB device drivers
 * (for either host or peripheral roles) don't use these calls; they
 * continue to use just usb_device and usb_gadget.
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_phy_interface {
    USBPHY_INTERFACE_MODE_UNKNOWN,
    USBPHY_INTERFACE_MODE_UTMI,
    USBPHY_INTERFACE_MODE_UTMIW,
    USBPHY_INTERFACE_MODE_ULPI,
    USBPHY_INTERFACE_MODE_SERIAL,
    USBPHY_INTERFACE_MODE_HSIC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_phy_events {
    USB_EVENT_NONE,
    USB_EVENT_VBUS,
    USB_EVENT_ID,
    USB_EVENT_CHARGER,
    USB_EVENT_ENUMERATED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_phy_type {
    USB_PHY_TYPE_UNDEFINED,
    USB_PHY_TYPE_USB2,
    USB_PHY_TYPE_USB3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_otg_state {
    OTG_STATE_UNDEFINED = 0,
    OTG_STATE_B_IDLE,
    OTG_STATE_B_SRP_INIT,
    OTG_STATE_B_PERIPHERAL,
    OTG_STATE_B_WAIT_ACON,
    OTG_STATE_B_HOST,
    OTG_STATE_A_IDLE,
    OTG_STATE_A_WAIT_VRISE,
    OTG_STATE_A_WAIT_BCON,
    OTG_STATE_A_HOST,
    OTG_STATE_A_SUSPEND,
    OTG_STATE_A_PERIPHERAL,
    OTG_STATE_A_WAIT_VFALL,
    OTG_STATE_A_VBUS_ERR,
}

#[repr(C)]
pub struct usb_phy_io_ops {
    pub read: Option<unsafe extern "C" fn(x: *mut usb_phy, reg: u32) -> i32>,
    pub write: Option<unsafe extern "C" fn(x: *mut usb_phy, val: u32, reg: u32) -> i32>,
}

#[repr(C)]
pub struct usb_charger_current {
    pub sdp_min: core::ffi::c_uint,
    pub sdp_max: core::ffi::c_uint,
    pub dcp_min: core::ffi::c_uint,
    pub dcp_max: core::ffi::c_uint,
    pub cdp_min: core::ffi::c_uint,
    pub cdp_max: core::ffi::c_uint,
    pub aca_min: core::ffi::c_uint,
    pub aca_max: core::ffi::c_uint,
}

#[repr(C)]
pub struct usb_phy {
    pub dev: *mut device,
    pub label: *const core::ffi::c_char,
    pub flags: core::ffi::c_uint,
    pub type_: usb_phy_type,
    pub last_event: usb_phy_events,
    pub otg: *mut usb_otg,
    pub io_dev: *mut device,
    pub io_ops: *mut usb_phy_io_ops,
    pub io_priv: *mut core::ffi::c_void,
    pub edev: *mut extcon_dev,
    pub id_edev: *mut extcon_dev,
    pub vbus_nb: notifier_block,
    pub id_nb: notifier_block,
    pub type_nb: notifier_block,
    pub chg_type: usb_charger_type,
    pub chg_state: usb_charger_state,
    pub chg_cur: usb_charger_current,
    pub chg_work: work_struct,
    pub notifier: atomic_notifier_head,
    pub port_status: u16,
    pub port_change: u16,
    pub head: list_head,
    pub init: Option<unsafe extern "C" fn(x: *mut usb_phy) -> i32>,
    pub shutdown: Option<unsafe extern "C" fn(x: *mut usb_phy)>,
    pub set_vbus: Option<unsafe extern "C" fn(x: *mut usb_phy, on: i32) -> i32>,
    pub set_power: Option<unsafe extern "C" fn(x: *mut usb_phy, ma: core::ffi::c_uint) -> i32>,
    pub set_suspend: Option<unsafe extern "C" fn(x: *mut usb_phy, suspend: i32) -> i32>,
    pub set_wakeup: Option<unsafe extern "C" fn(x: *mut usb_phy, enabled: bool) -> i32>,
    pub notify_connect: Option<unsafe extern "C" fn(x: *mut usb_phy, speed: usb_device_speed) -> i32>,
    pub notify_disconnect: Option<unsafe extern "C" fn(x: *mut usb_phy, speed: usb_device_speed) -> i32>,
    pub charger_detect: Option<unsafe extern "C" fn(x: *mut usb_phy) -> usb_charger_type>,
}

pub enum device {}
pub enum usb_otg {}
pub enum extcon_dev {}
pub enum device_node {}
pub enum work_struct {}
pub enum atomic_notifier_head {}
pub enum notifier_block {}
pub enum list_head {}
pub enum usb_charger_type {}
pub enum usb_charger_state {}
pub enum usb_device_speed {}

extern "C" {
    pub fn usb_add_phy(x: *mut usb_phy, type_: usb_phy_type) -> i32;
    pub fn usb_add_phy_dev(x: *mut usb_phy) -> i32;
    pub fn usb_remove_phy(x: *mut usb_phy);
}

pub unsafe fn usb_phy_io_read(x: *mut usb_phy, reg: u32) -> i32 {
    if !x.is_null() && !(*x).io_ops.is_null() {
        if let Some(read) = (*(*x).io_ops).read { return read(x, reg); }
    }
    -22
}

pub unsafe fn usb_phy_io_write(x: *mut usb_phy, val: u32, reg: u32) -> i32 {
    if !x.is_null() && !(*x).io_ops.is_null() {
        if let Some(write) = (*(*x).io_ops).write { return write(x, val, reg); }
    }
    -22
}

pub unsafe fn usb_phy_init(x: *mut usb_phy) -> i32 {
    if !x.is_null() { if let Some(init) = (*x).init { return init(x); } }
    0
}

pub unsafe fn usb_phy_shutdown(x: *mut usb_phy) {
    if !x.is_null() { if let Some(shutdown) = (*x).shutdown { shutdown(x); } }
}

pub unsafe fn usb_phy_vbus_on(x: *mut usb_phy) -> i32 {
    if x.is_null() { return 0; }
    if let Some(set_vbus) = (*x).set_vbus { return set_vbus(x, 1); }
    0
}

pub unsafe fn usb_phy_vbus_off(x: *mut usb_phy) -> i32 {
    if x.is_null() { return 0; }
    if let Some(set_vbus) = (*x).set_vbus { return set_vbus(x, 0); }
    0
}

/* CONFIG_USB_PHY is a build-time kernel condition; both branches are retained as intent. */
extern "C" {
    pub fn usb_get_phy(type_: usb_phy_type) -> *mut usb_phy;
    pub fn devm_usb_get_phy(dev: *mut device, type_: usb_phy_type) -> *mut usb_phy;
    pub fn devm_usb_get_phy_by_phandle(dev: *mut device, phandle: *const core::ffi::c_char, index: u8) -> *mut usb_phy;
    pub fn devm_usb_get_phy_by_node(dev: *mut device, node: *mut device_node, nb: *mut notifier_block) -> *mut usb_phy;
    pub fn usb_put_phy(x: *mut usb_phy);
    pub fn usb_phy_set_event(x: *mut usb_phy, event: core::ffi::c_ulong);
    pub fn usb_phy_set_charger_current(x: *mut usb_phy, ma: core::ffi::c_uint);
    pub fn usb_phy_get_charger_current(x: *mut usb_phy, min: *mut core::ffi::c_uint, max: *mut core::ffi::c_uint);
    pub fn usb_phy_set_charger_state(x: *mut usb_phy, state: usb_charger_state);
}

pub unsafe fn usb_phy_set_power(x: *mut usb_phy, ma: core::ffi::c_uint) -> i32 {
    if x.is_null() { return 0; }
    usb_phy_set_charger_current(x, ma);
    if let Some(set_power) = (*x).set_power { return set_power(x, ma); }
    0
}

pub unsafe fn usb_phy_set_suspend(x: *mut usb_phy, suspend: i32) -> i32 {
    if !x.is_null() { if let Some(f) = (*x).set_suspend { return f(x, suspend); } }
    0
}

pub unsafe fn usb_phy_set_wakeup(x: *mut usb_phy, enabled: bool) -> i32 {
    if !x.is_null() { if let Some(f) = (*x).set_wakeup { return f(x, enabled); } }
    0
}

pub unsafe fn usb_phy_notify_connect(x: *mut usb_phy, speed: usb_device_speed) -> i32 {
    if !x.is_null() { if let Some(f) = (*x).notify_connect { return f(x, speed); } }
    0
}

pub unsafe fn usb_phy_notify_disconnect(x: *mut usb_phy, speed: usb_device_speed) -> i32 {
    if !x.is_null() { if let Some(f) = (*x).notify_disconnect { return f(x, speed); } }
    0
}

extern "C" {
    pub fn atomic_notifier_chain_register(head: *mut atomic_notifier_head, nb: *mut notifier_block) -> i32;
    pub fn atomic_notifier_chain_unregister(head: *mut atomic_notifier_head, nb: *mut notifier_block);
}

pub unsafe fn usb_register_notifier(x: *mut usb_phy, nb: *mut notifier_block) -> i32 {
    atomic_notifier_chain_register(&mut (*x).notifier, nb)
}

pub unsafe fn usb_unregister_notifier(x: *mut usb_phy, nb: *mut notifier_block) {
    atomic_notifier_chain_unregister(&mut (*x).notifier, nb);
}

pub unsafe fn usb_phy_type_string(type_: usb_phy_type) -> &'static [u8] {
    match type_ {
        usb_phy_type::USB_PHY_TYPE_USB2 => b"USB2 PHY\0",
        usb_phy_type::USB_PHY_TYPE_USB3 => b"USB3 PHY\0",
        _ => b"UNKNOWN PHY TYPE\0",
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
