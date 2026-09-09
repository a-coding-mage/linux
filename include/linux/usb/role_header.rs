// SPDX-License-Identifier: GPL-2.0

// Dependency declarations supplied by other translated files.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_role_switch {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_role {
    USB_ROLE_NONE,
    USB_ROLE_HOST,
    USB_ROLE_DEVICE,
}

pub type usb_role_switch_set_t = unsafe extern "C" fn(
    sw: *mut usb_role_switch,
    role: usb_role,
) -> core::ffi::c_int;
pub type usb_role_switch_get_t = unsafe extern "C" fn(
    sw: *mut usb_role_switch,
) -> usb_role;

#[repr(C)]
pub struct usb_role_switch_desc {
    pub fwnode: *mut fwnode_handle,
    pub usb2_port: *mut device,
    pub usb3_port: *mut device,
    pub udc: *mut device,
    pub set: Option<usb_role_switch_set_t>,
    pub get: Option<usb_role_switch_get_t>,
    pub allow_userspace_control: bool,
    pub driver_data: *mut core::ffi::c_void,
    pub name: *const core::ffi::c_char,
}

// When CONFIG_USB_ROLE_SWITCH is enabled, these are provided by the USB role-switch implementation.
#[cfg(feature = "CONFIG_USB_ROLE_SWITCH")]
extern "C" {
    pub fn usb_role_switch_set_role(
        sw: *mut usb_role_switch,
        role: usb_role,
    ) -> core::ffi::c_int;
    pub fn usb_role_switch_get_role(sw: *mut usb_role_switch) -> usb_role;
    pub fn usb_role_switch_get(dev: *mut device) -> *mut usb_role_switch;
    pub fn fwnode_usb_role_switch_get(node: *mut fwnode_handle) -> *mut usb_role_switch;
    pub fn usb_role_switch_put(sw: *mut usb_role_switch);
    pub fn usb_role_switch_find_by_fwnode(
        fwnode: *const fwnode_handle,
    ) -> *mut usb_role_switch;
    pub fn usb_role_switch_register(
        parent: *mut device,
        desc: *const usb_role_switch_desc,
    ) -> *mut usb_role_switch;
    pub fn usb_role_switch_unregister(sw: *mut usb_role_switch);
    pub fn usb_role_switch_set_drvdata(sw: *mut usb_role_switch, data: *mut core::ffi::c_void);
    pub fn usb_role_switch_get_drvdata(sw: *mut usb_role_switch) -> *mut core::ffi::c_void;
    pub fn usb_role_string(role: usb_role) -> *const core::ffi::c_char;
}

// Fallbacks corresponding to the !IS_ENABLED(CONFIG_USB_ROLE_SWITCH) branch.
#[cfg(not(feature = "CONFIG_USB_ROLE_SWITCH"))]
pub unsafe fn usb_role_switch_set_role(
    _sw: *mut usb_role_switch,
    _role: usb_role,
) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_USB_ROLE_SWITCH"))]
pub unsafe fn usb_role_switch_get_role(_sw: *mut usb_role_switch) -> usb_role {
    usb_role::USB_ROLE_NONE
}

#[cfg(not(feature = "CONFIG_USB_ROLE_SWITCH"))]
pub unsafe fn usb_role_switch_get(_dev: *mut device) -> *mut usb_role_switch {
    crate::ERR_PTR(-crate::ENODEV)
}

#[cfg(not(feature = "CONFIG_USB_ROLE_SWITCH"))]
pub unsafe fn fwnode_usb_role_switch_get(_node: *mut fwnode_handle) -> *mut usb_role_switch {
    crate::ERR_PTR(-crate::ENODEV)
}

#[cfg(not(feature = "CONFIG_USB_ROLE_SWITCH"))]
pub unsafe fn usb_role_switch_put(_sw: *mut usb_role_switch) {}

#[cfg(not(feature = "CONFIG_USB_ROLE_SWITCH"))]
pub unsafe fn usb_role_switch_find_by_fwnode(
    _fwnode: *const fwnode_handle,
) -> *mut usb_role_switch {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_USB_ROLE_SWITCH"))]
pub unsafe fn usb_role_switch_register(
    _parent: *mut device,
    _desc: *const usb_role_switch_desc,
) -> *mut usb_role_switch {
    crate::ERR_PTR(-crate::ENODEV)
}

#[cfg(not(feature = "CONFIG_USB_ROLE_SWITCH"))]
pub unsafe fn usb_role_switch_unregister(_sw: *mut usb_role_switch) {}

#[cfg(not(feature = "CONFIG_USB_ROLE_SWITCH"))]
pub unsafe fn usb_role_switch_set_drvdata(
    _sw: *mut usb_role_switch,
    _data: *mut core::ffi::c_void,
) {
}

#[cfg(not(feature = "CONFIG_USB_ROLE_SWITCH"))]
pub unsafe fn usb_role_switch_get_drvdata(
    _sw: *mut usb_role_switch,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_USB_ROLE_SWITCH"))]
pub unsafe fn usb_role_string(_role: usb_role) -> *const core::ffi::c_char {
    c"unknown".as_ptr()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
