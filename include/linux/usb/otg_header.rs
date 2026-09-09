/* SPDX-License-Identifier: GPL-2.0 */
/* USB OTG (On The Go) defines */
/*
 * These APIs may be used between USB controllers. USB device drivers
 * (for either host or peripheral roles) don't use these calls; they
 * continue to use just usb_device and usb_gadget.
 */

/* Types supplied by linux/phy/phy.h, linux/usb/phy.h, and other headers. */
#[repr(C)]
pub struct phy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_phy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_gadget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* The C enum is an externally supplied type; its ABI is an integer. */
pub type usb_otg_state = i32;

#[repr(C)]
pub struct usb_otg {
    pub default_a: u8,
    pub phy: *mut phy,
    /* old usb_phy interface */
    pub usb_phy: *mut usb_phy,
    pub host: *mut usb_bus,
    pub gadget: *mut usb_gadget,
    pub state: usb_otg_state,

    /* bind/unbind the host controller */
    pub set_host: Option<unsafe extern "C" fn(*mut usb_otg, *mut usb_bus) -> i32>,

    /* bind/unbind the peripheral controller */
    pub set_peripheral:
        Option<unsafe extern "C" fn(*mut usb_otg, *mut usb_gadget) -> i32>,

    /* effective for A-peripheral, ignored for B devices */
    pub set_vbus: Option<unsafe extern "C" fn(*mut usb_otg, bool) -> i32>,

    /* for B devices only: start session with A-Host */
    pub start_srp: Option<unsafe extern "C" fn(*mut usb_otg) -> i32>,

    /* start or continue HNP role switch */
    pub start_hnp: Option<unsafe extern "C" fn(*mut usb_otg) -> i32>,
}

/**
 * struct usb_otg_caps - describes the otg capabilities of the device
 * @otg_rev: The OTG revision number the device is compliant with, it's
 *     in binary-coded decimal (i.e. 2.0 is 0200H).
 * @hnp_support: Indicates if the device supports HNP.
 * @srp_support: Indicates if the device supports SRP.
 * @adp_support: Indicates if the device supports ADP.
 */
#[repr(C)]
pub struct usb_otg_caps {
    pub otg_rev: u16,
    pub hnp_support: bool,
    pub srp_support: bool,
    pub adp_support: bool,
}

unsafe extern "C" {
    pub fn usb_otg_state_string(state: usb_otg_state) -> *const core::ffi::c_char;
}

/* Context: can sleep */
#[inline]
pub unsafe fn otg_start_hnp(otg: *mut usb_otg) -> i32 {
    if !otg.is_null() {
        if let Some(callback) = (*otg).start_hnp {
            return callback(otg);
        }
    }
    -ENOTSUPP
}

/* Context: can sleep */
#[inline]
pub unsafe fn otg_set_vbus(otg: *mut usb_otg, enabled: bool) -> i32 {
    if !otg.is_null() {
        if let Some(callback) = (*otg).set_vbus {
            return callback(otg, enabled);
        }
    }
    -ENOTSUPP
}

/* for HCDs */
#[inline]
pub unsafe fn otg_set_host(otg: *mut usb_otg, host: *mut usb_bus) -> i32 {
    if !otg.is_null() {
        if let Some(callback) = (*otg).set_host {
            return callback(otg, host);
        }
    }
    -ENOTSUPP
}

/* for usb peripheral controller drivers */
/* Context: can sleep */
#[inline]
pub unsafe fn otg_set_peripheral(otg: *mut usb_otg, periph: *mut usb_gadget) -> i32 {
    if !otg.is_null() {
        if let Some(callback) = (*otg).set_peripheral {
            return callback(otg, periph);
        }
    }
    -ENOTSUPP
}

#[inline]
pub unsafe fn otg_start_srp(otg: *mut usb_otg) -> i32 {
    if !otg.is_null() {
        if let Some(callback) = (*otg).start_srp {
            return callback(otg);
        }
    }
    -ENOTSUPP
}

/* for OTG controller drivers (and maybe other stuff) */
unsafe extern "C" {
    pub fn usb_bus_start_enum(bus: *mut usb_bus, port_num: u32) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum usb_dr_mode {
    USB_DR_MODE_UNKNOWN,
    USB_DR_MODE_HOST,
    USB_DR_MODE_PERIPHERAL,
    USB_DR_MODE_OTG,
}

/**
 * usb_get_dr_mode - Get dual role mode for given device
 * @dev: Pointer to the given device
 *
 * The function gets phy interface string from property 'dr_mode',
 * and returns the corresponding enum usb_dr_mode
 */
unsafe extern "C" {
    pub fn usb_get_dr_mode(dev: *mut device) -> usb_dr_mode;
    pub fn usb_get_role_switch_default_mode(dev: *mut device) -> usb_dr_mode;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
