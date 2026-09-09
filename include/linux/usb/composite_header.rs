// SPDX-License-Identifier: GPL-2.0+
// composite.h -- framework for usb gadgets which are composite devices
//
// C headers and build-time includes are intentionally not reproduced here;
// their supplied types and constants are external dependencies.

pub const USB_GADGET_DELAYED_STATUS: u16 = 0x7fff;
pub const USB_COMP_EP0_BUFSIZ: usize = 4096;
pub const USB_COMP_EP0_OS_DESC_BUFSIZ: usize = 4096;

#[inline]
pub const fn usb_ms_to_hs_interval(x: u32) -> u32 {
    // C: ilog2((x * 1000 / 125)) + 1
    1 + (x.saturating_mul(1000) / 125).ilog2()
}

pub const MAX_CONFIG_INTERFACES: usize = 32;
pub const OS_STRING_QW_SIGN_LEN: usize = 14;
pub const OS_STRING_IDX: u8 = 0xee;

#[repr(C)]
pub struct usb_os_desc_ext_prop {
    pub entry: list_head,
    pub type_: u8,
    pub name_len: i32,
    pub name: *mut i8,
    pub data_len: i32,
    pub data: *mut i8,
    pub item: config_item,
}

#[repr(C)]
pub struct usb_os_desc {
    pub ext_compat_id: *mut i8,
    pub ext_prop: list_head,
    pub ext_prop_len: i32,
    pub ext_prop_count: i32,
    pub opts_mutex: *mut mutex,
    pub group: config_group,
    pub owner: *mut module,
}

#[repr(C)]
pub struct usb_os_desc_table {
    pub if_id: i32,
    pub os_desc: *mut usb_os_desc,
}

#[repr(C)]
pub struct usb_function {
    pub name: *const i8,
    pub strings: *mut *mut usb_gadget_strings,
    pub fs_descriptors: *mut *mut usb_descriptor_header,
    pub hs_descriptors: *mut *mut usb_descriptor_header,
    pub ss_descriptors: *mut *mut usb_descriptor_header,
    pub ssp_descriptors: *mut *mut usb_descriptor_header,
    pub config: *mut usb_configuration,
    pub os_desc_table: *mut usb_os_desc_table,
    pub os_desc_n: u32,
    pub bind: Option<unsafe extern "C" fn(*mut usb_configuration, *mut usb_function) -> i32>,
    pub unbind: Option<unsafe extern "C" fn(*mut usb_configuration, *mut usb_function)>,
    pub free_func: Option<unsafe extern "C" fn(*mut usb_function)>,
    pub mod_: *mut module,
    pub set_alt: Option<unsafe extern "C" fn(*mut usb_function, u32, u32) -> i32>,
    pub get_alt: Option<unsafe extern "C" fn(*mut usb_function, u32) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut usb_function)>,
    pub setup: Option<unsafe extern "C" fn(*mut usb_function, *const usb_ctrlrequest) -> i32>,
    pub req_match: Option<unsafe extern "C" fn(*mut usb_function, *const usb_ctrlrequest, bool) -> bool>,
    pub suspend: Option<unsafe extern "C" fn(*mut usb_function)>,
    pub resume: Option<unsafe extern "C" fn(*mut usb_function)>,
    pub get_status: Option<unsafe extern "C" fn(*mut usb_function) -> i32>,
    pub func_suspend: Option<unsafe extern "C" fn(*mut usb_function, u8) -> i32>,
    pub func_suspended: bool,
    pub func_wakeup_armed: bool,
    pub list: list_head,
    pub endpoints: [u8; 4],
    pub fi: *mut usb_function_instance,
    pub bind_deactivated: u32,
}

extern "C" {
    pub fn usb_add_function(c: *mut usb_configuration, f: *mut usb_function) -> i32;
    pub fn usb_function_deactivate(f: *mut usb_function) -> i32;
    pub fn usb_function_activate(f: *mut usb_function) -> i32;
    pub fn usb_interface_id(c: *mut usb_configuration, f: *mut usb_function) -> i32;
    pub fn config_ep_by_speed_and_alt(g: *mut usb_gadget, f: *mut usb_function, ep: *mut usb_ep, alt: u8) -> i32;
    pub fn config_ep_by_speed(g: *mut usb_gadget, f: *mut usb_function, ep: *mut usb_ep) -> i32;
    pub fn usb_func_wakeup(func: *mut usb_function) -> i32;
}

#[repr(C)]
pub struct usb_configuration {
    pub label: *const i8,
    pub strings: *mut *mut usb_gadget_strings,
    pub descriptors: *const *const usb_descriptor_header,
    pub unbind: Option<unsafe extern "C" fn(*mut usb_configuration)>,
    pub setup: Option<unsafe extern "C" fn(*mut usb_configuration, *const usb_ctrlrequest) -> i32>,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub MaxPower: u16,
    pub cdev: *mut usb_composite_dev,
    pub list: list_head,
    pub functions: list_head,
    pub next_interface_id: u8,
    pub superspeed: u32,
    pub highspeed: u32,
    pub fullspeed: u32,
    pub superspeed_plus: u32,
    pub interface: [*mut usb_function; MAX_CONFIG_INTERFACES],
}

extern "C" {
    pub fn usb_add_config(cdev: *mut usb_composite_dev, config: *mut usb_configuration, bind: Option<unsafe extern "C" fn(*mut usb_configuration) -> i32>) -> i32;
}

pub const USB_GADGET_MANUFACTURER_IDX: u32 = 0;
pub const USB_GADGET_PRODUCT_IDX: u32 = 1;
pub const USB_GADGET_SERIAL_IDX: u32 = 2;
pub const USB_GADGET_FIRST_AVAIL_IDX: u32 = 3;

#[repr(C)]
pub struct usb_composite_driver {
    pub name: *const i8,
    pub dev: *const usb_device_descriptor,
    pub strings: *mut *mut usb_gadget_strings,
    pub max_speed: usb_device_speed,
    pub needs_serial: u32,
    pub bind: Option<unsafe extern "C" fn(*mut usb_composite_dev) -> i32>,
    pub unbind: Option<unsafe extern "C" fn(*mut usb_composite_dev) -> i32>,
    pub disconnect: Option<unsafe extern "C" fn(*mut usb_composite_dev)>,
    pub suspend: Option<unsafe extern "C" fn(*mut usb_composite_dev)>,
    pub resume: Option<unsafe extern "C" fn(*mut usb_composite_dev)>,
    pub gadget_driver: usb_gadget_driver,
}

extern "C" {
    pub fn usb_composite_probe(driver: *mut usb_composite_driver) -> i32;
    pub fn usb_composite_unregister(driver: *mut usb_composite_driver);
    pub fn usb_composite_setup_continue(cdev: *mut usb_composite_dev);
    pub fn composite_dev_prepare(composite: *mut usb_composite_driver, cdev: *mut usb_composite_dev) -> i32;
    pub fn composite_os_desc_req_prepare(cdev: *mut usb_composite_dev, ep0: *mut usb_ep) -> i32;
    pub fn composite_dev_cleanup(cdev: *mut usb_composite_dev);
    pub fn check_remote_wakeup_config(g: *mut usb_gadget, c: *mut usb_configuration);
}

#[inline]
pub unsafe fn to_cdriver(gdrv: *mut usb_gadget_driver) -> *mut usb_composite_driver {
    // container_of(gdrv, struct usb_composite_driver, gadget_driver)
    (gdrv as *mut u8).sub(core::mem::offset_of!(usb_composite_driver, gadget_driver)) as *mut usb_composite_driver
}

#[repr(C)]
pub struct usb_composite_dev {
    pub gadget: *mut usb_gadget,
    pub req: *mut usb_request,
    pub os_desc_req: *mut usb_request,
    pub config: *mut usb_configuration,
    pub qw_sign: [u8; OS_STRING_QW_SIGN_LEN],
    pub b_vendor_code: u8,
    pub os_desc_config: *mut usb_configuration,
    pub use_os_string: u32,
    pub bcd_webusb_version: u16,
    pub b_webusb_vendor_code: u8,
    pub landing_page: [i8; WEBUSB_URL_RAW_MAX_LENGTH],
    pub use_webusb: u32,
    pub suspended: u32,
    pub desc: usb_device_descriptor,
    pub configs: list_head,
    pub gstrings: list_head,
    pub driver: *mut usb_composite_driver,
    pub next_string_id: u8,
    pub def_manufacturer: *mut i8,
    pub usb_strings: *mut usb_string,
    pub deactivations: u32,
    pub delayed_status: i32,
    pub lock: spinlock_t,
    pub setup_pending: u32,
    pub os_desc_pending: u32,
}

#[repr(C)]
pub struct usb_composite_overwrite {
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub serial_number: *mut i8,
    pub manufacturer: *mut i8,
    pub product: *mut i8,
}

extern "C" {
    pub fn usb_string_id(c: *mut usb_composite_dev) -> i32;
    pub fn usb_string_ids_tab(c: *mut usb_composite_dev, str_: *mut usb_string) -> i32;
    pub fn usb_gstrings_attach(cdev: *mut usb_composite_dev, sp: *mut *mut usb_gadget_strings, n_strings: u32) -> *mut usb_string;
    pub fn usb_string_ids_n(c: *mut usb_composite_dev, n: u32) -> i32;
    pub fn composite_disconnect(gadget: *mut usb_gadget);
    pub fn composite_reset(gadget: *mut usb_gadget);
    pub fn composite_setup(gadget: *mut usb_gadget, ctrl: *const usb_ctrlrequest) -> i32;
    pub fn composite_suspend(gadget: *mut usb_gadget);
    pub fn composite_resume(gadget: *mut usb_gadget);
    pub fn usb_composite_overwrite_options(cdev: *mut usb_composite_dev, covr: *mut usb_composite_overwrite);
}

#[inline]
pub fn get_default_bcdDevice() -> u16 {
    ((bin2bcd(LINUX_VERSION_MAJOR) as u16) << 8) | bin2bcd(LINUX_VERSION_PATCHLEVEL) as u16
}

#[repr(C)]
pub struct usb_function_driver {
    pub name: *const i8,
    pub mod_: *mut module,
    pub list: list_head,
    pub alloc_inst: Option<unsafe extern "C" fn() -> *mut usb_function_instance>,
    pub alloc_func: Option<unsafe extern "C" fn(*mut usb_function_instance) -> *mut usb_function>,
}

#[repr(C)]
pub struct usb_function_instance {
    pub group: config_group,
    pub cfs_list: list_head,
    pub fd: *mut usb_function_driver,
    pub set_inst_name: Option<unsafe extern "C" fn(*mut usb_function_instance, *const i8) -> i32>,
    pub free_func_inst: Option<unsafe extern "C" fn(*mut usb_function_instance)>,
}

extern "C" {
    pub fn usb_function_unregister(f: *mut usb_function_driver);
    pub fn usb_function_register(newf: *mut usb_function_driver) -> i32;
    pub fn usb_put_function_instance(fi: *mut usb_function_instance);
    pub fn usb_put_function(f: *mut usb_function);
    pub fn usb_get_function_instance(name: *const i8) -> *mut usb_function_instance;
    pub fn usb_get_function(fi: *mut usb_function_instance) -> *mut usb_function;
    pub fn usb_get_config(cdev: *mut usb_composite_dev, val: i32) -> *mut usb_configuration;
    pub fn usb_add_config_only(cdev: *mut usb_composite_dev, config: *mut usb_configuration) -> i32;
    pub fn usb_remove_function(c: *mut usb_configuration, f: *mut usb_function);
}

// The C registration, module parameter, and device logging macros are build-system
// constructs; their original names and intent are retained here for consumers.
// DECLARE_USB_FUNCTION, DECLARE_USB_FUNCTION_INIT, USB_GADGET_COMPOSITE_OPTIONS,
// module_usb_composite_driver, DBG, VDBG, ERROR, WARNING, and INFO.

// External types/constants supplied by the included Linux headers.
extern "C" {
    fn bin2bcd(value: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
