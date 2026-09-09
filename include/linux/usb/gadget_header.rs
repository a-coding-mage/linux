// SPDX-License-Identifier: GPL-2.0
// Translation of <linux/usb/gadget.h>; C includes and preprocessor guards are omitted.

pub const UDC_TRACE_STR_MAX: usize = 512;

pub const USB_EP_CAPS_TYPE_CONTROL: u32 = 0x01;
pub const USB_EP_CAPS_TYPE_ISO: u32 = 0x02;
pub const USB_EP_CAPS_TYPE_BULK: u32 = 0x04;
pub const USB_EP_CAPS_TYPE_INT: u32 = 0x08;
pub const USB_EP_CAPS_TYPE_ALL: u32 = USB_EP_CAPS_TYPE_ISO | USB_EP_CAPS_TYPE_BULK | USB_EP_CAPS_TYPE_INT;
pub const USB_EP_CAPS_DIR_IN: u32 = 0x01;
pub const USB_EP_CAPS_DIR_OUT: u32 = 0x02;
pub const USB_EP_CAPS_DIR_ALL: u32 = USB_EP_CAPS_DIR_IN | USB_EP_CAPS_DIR_OUT;

#[repr(C)]
pub struct usb_ep_caps {
    pub type_control: u32, pub type_iso: u32, pub type_bulk: u32, pub type_int: u32,
    pub dir_in: u32, pub dir_out: u32,
}

#[repr(C)]
pub struct usb_request {
    pub ep: *mut usb_ep, pub buf: *mut core::ffi::c_void, pub length: u32, pub dma: dma_addr_t,
    pub sg: *mut scatterlist, pub num_sgs: u32, pub num_mapped_sgs: u32,
    // C bitfields: stream_id:16, is_last/no_interrupt/zero/short_not_ok/dma_mapped/sg_was_mapped:1.
    pub stream_id: u16, pub is_last: bool, pub no_interrupt: bool, pub zero: bool,
    pub short_not_ok: bool, pub dma_mapped: bool, pub sg_was_mapped: bool,
    pub complete: Option<unsafe extern "C" fn(*mut usb_ep, *mut usb_request)>,
    pub context: *mut core::ffi::c_void, pub list: list_head, pub frame_number: u32,
    pub status: i32, pub actual: u32,
}

#[repr(C)]
pub struct usb_ep_ops {
    pub enable: Option<unsafe extern "C" fn(*mut usb_ep, *const usb_endpoint_descriptor) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut usb_ep) -> i32>,
    pub dispose: Option<unsafe extern "C" fn(*mut usb_ep)>,
    pub alloc_request: Option<unsafe extern "C" fn(*mut usb_ep, gfp_t) -> *mut usb_request>,
    pub free_request: Option<unsafe extern "C" fn(*mut usb_ep, *mut usb_request)>,
    pub queue: Option<unsafe extern "C" fn(*mut usb_ep, *mut usb_request, gfp_t) -> i32>,
    pub dequeue: Option<unsafe extern "C" fn(*mut usb_ep, *mut usb_request) -> i32>,
    pub set_halt: Option<unsafe extern "C" fn(*mut usb_ep, i32) -> i32>,
    pub set_wedge: Option<unsafe extern "C" fn(*mut usb_ep) -> i32>,
    pub fifo_status: Option<unsafe extern "C" fn(*mut usb_ep) -> i32>,
    pub fifo_flush: Option<unsafe extern "C" fn(*mut usb_ep)>,
}

#[repr(C)]
pub struct usb_ep {
    pub driver_data: *mut core::ffi::c_void, pub name: *const i8, pub ops: *const usb_ep_ops,
    pub desc: *const usb_endpoint_descriptor, pub comp_desc: *const usb_ss_ep_comp_descriptor,
    pub ep_list: list_head, pub caps: usb_ep_caps, pub claimed: bool, pub enabled: bool,
    // C bitfields: mult:2, maxburst:5.
    pub mult: u8, pub maxburst: u8, pub address: u8, pub maxpacket: u16,
    pub maxpacket_limit: u16, pub max_streams: u16,
}

#[repr(C)]
pub struct usb_dcd_config_params {
    pub bU1devExitLat: __u8, pub bU2DevExitLat: __le16, pub besl_baseline: __u8, pub besl_deep: __u8,
}
pub const USB_DEFAULT_U1_DEV_EXIT_LAT: u8 = 0x01;
pub const USB_DEFAULT_U2_DEV_EXIT_LAT: u16 = 0x1F4;
pub const USB_DEFAULT_BESL_UNSPECIFIED: u8 = 0xFF;

#[repr(C)]
pub struct usb_gadget_ops {
    pub get_frame: Option<unsafe extern "C" fn(*mut usb_gadget) -> i32>,
    pub wakeup: Option<unsafe extern "C" fn(*mut usb_gadget) -> i32>,
    pub func_wakeup: Option<unsafe extern "C" fn(*mut usb_gadget, i32) -> i32>,
    pub set_remote_wakeup: Option<unsafe extern "C" fn(*mut usb_gadget, i32) -> i32>,
    pub set_selfpowered: Option<unsafe extern "C" fn(*mut usb_gadget, i32) -> i32>,
    pub vbus_session: Option<unsafe extern "C" fn(*mut usb_gadget, i32) -> i32>,
    pub vbus_draw: Option<unsafe extern "C" fn(*mut usb_gadget, u32) -> i32>,
    pub pullup: Option<unsafe extern "C" fn(*mut usb_gadget, i32) -> i32>,
    pub ioctl: Option<unsafe extern "C" fn(*mut usb_gadget, u32, c_ulong) -> i32>,
    pub get_config_params: Option<unsafe extern "C" fn(*mut usb_gadget, *mut usb_dcd_config_params)>,
    pub udc_start: Option<unsafe extern "C" fn(*mut usb_gadget, *mut usb_gadget_driver) -> i32>,
    pub udc_stop: Option<unsafe extern "C" fn(*mut usb_gadget) -> i32>,
    pub udc_set_speed: Option<unsafe extern "C" fn(*mut usb_gadget, usb_device_speed)>,
    pub udc_set_ssp_rate: Option<unsafe extern "C" fn(*mut usb_gadget, usb_ssp_rate)>,
    pub udc_async_callbacks: Option<unsafe extern "C" fn(*mut usb_gadget, bool)>,
    pub match_ep: Option<unsafe extern "C" fn(*mut usb_gadget, *mut usb_endpoint_descriptor, *mut usb_ss_ep_comp_descriptor) -> *mut usb_ep>,
    pub check_config: Option<unsafe extern "C" fn(*mut usb_gadget) -> i32>,
}

#[repr(C)]
pub struct usb_gadget {
    pub work: work_struct, pub udc: *mut usb_udc, pub ops: *const usb_gadget_ops, pub ep0: *mut usb_ep,
    pub ep_list: list_head, pub speed: usb_device_speed, pub max_speed: usb_device_speed,
    pub ssp_rate: usb_ssp_rate, pub max_ssp_rate: usb_ssp_rate, pub state: usb_device_state,
    pub state_lock: spinlock_t, pub teardown: bool, pub name: *const i8, pub dev: device,
    pub isoch_delay: u32, pub out_epnum: u32, pub in_epnum: u32, pub mA: u32, pub otg_caps: *mut usb_otg_caps,
    // C bitfields, each one bit.
    pub sg_supported: bool, pub is_otg: bool, pub is_a_peripheral: bool, pub b_hnp_enable: bool,
    pub a_hnp_support: bool, pub a_alt_hnp_support: bool, pub hnp_polling_support: bool,
    pub host_request_flag: bool, pub quirk_ep_out_aligned_size: bool, pub quirk_altset_not_supp: bool,
    pub quirk_stall_not_supp: bool, pub quirk_zlp_not_supp: bool, pub quirk_avoids_skb_reserve: bool,
    pub is_selfpowered: bool, pub deactivated: bool, pub connected: bool, pub lpm_capable: bool,
    pub wakeup_capable: bool, pub wakeup_armed: bool, pub irq: i32, pub id_number: i32,
}

#[repr(C)]
pub struct usb_gadget_driver {
    pub function: *mut i8, pub max_speed: usb_device_speed,
    pub bind: Option<unsafe extern "C" fn(*mut usb_gadget, *mut usb_gadget_driver) -> i32>,
    pub unbind: Option<unsafe extern "C" fn(*mut usb_gadget)>,
    pub setup: Option<unsafe extern "C" fn(*mut usb_gadget, *const usb_ctrlrequest) -> i32>,
    pub disconnect: Option<unsafe extern "C" fn(*mut usb_gadget)>, pub suspend: Option<unsafe extern "C" fn(*mut usb_gadget)>,
    pub resume: Option<unsafe extern "C" fn(*mut usb_gadget)>, pub reset: Option<unsafe extern "C" fn(*mut usb_gadget)>,
    pub driver: device_driver, pub udc_name: *mut i8, pub match_existing_only: bool, pub is_bound: bool,
}

#[repr(C)] pub struct usb_string { pub id: u8, pub s: *const i8 }
#[repr(C)] pub struct usb_gadget_strings { pub language: u16, pub strings: *mut usb_string }
#[repr(C)] pub struct usb_gadget_string_container { pub list: list_head, pub stash: [*mut u8; 0] }
#[repr(C)] pub struct gadget_string { pub item: config_item, pub list: list_head, pub string: [i8; USB_MAX_STRING_LEN], pub usb_string: usb_string }

pub const USB_MAX_STRING_LEN: usize = 256;

pub unsafe extern "C" fn usb_ep_align(ep: *mut usb_ep, len: usize) -> usize {
    let max_packet_size = usb_endpoint_maxp((*ep).desc) as usize;
    (len + max_packet_size - 1) / max_packet_size * max_packet_size
}
pub unsafe extern "C" fn usb_ep_align_maybe(g: *mut usb_gadget, ep: *mut usb_ep, len: usize) -> usize {
    if (*g).quirk_ep_out_aligned_size { usb_ep_align(ep, len) } else { len }
}
pub unsafe extern "C" fn gadget_is_altset_supported(g: *mut usb_gadget) -> i32 { (!(*g).quirk_altset_not_supp) as i32 }
pub unsafe extern "C" fn gadget_is_stall_supported(g: *mut usb_gadget) -> i32 { (!(*g).quirk_stall_not_supp) as i32 }
pub unsafe extern "C" fn gadget_is_zlp_supported(g: *mut usb_gadget) -> i32 { (!(*g).quirk_zlp_not_supp) as i32 }
pub unsafe extern "C" fn gadget_avoids_skb_reserve(g: *mut usb_gadget) -> i32 { (*g).quirk_avoids_skb_reserve as i32 }
pub unsafe extern "C" fn gadget_is_dualspeed(g: *mut usb_gadget) -> i32 { ((*g).max_speed as i32 >= USB_SPEED_HIGH as i32) as i32 }
pub unsafe extern "C" fn gadget_is_superspeed(g: *mut usb_gadget) -> i32 { ((*g).max_speed as i32 >= USB_SPEED_SUPER as i32) as i32 }
pub unsafe extern "C" fn gadget_is_superspeed_plus(g: *mut usb_gadget) -> i32 { ((*g).max_speed as i32 >= USB_SPEED_SUPER_PLUS as i32) as i32 }
pub unsafe extern "C" fn gadget_is_otg(g: *mut usb_gadget) -> i32 { (*g).is_otg as i32 }

extern "C" {
    pub fn usb_ep_set_maxpacket_limit(ep: *mut usb_ep, maxpacket_limit: u32);
    pub fn usb_ep_enable(ep: *mut usb_ep) -> i32; pub fn usb_ep_disable(ep: *mut usb_ep) -> i32;
    pub fn usb_ep_alloc_request(ep: *mut usb_ep, gfp_flags: gfp_t) -> *mut usb_request;
    pub fn usb_ep_free_request(ep: *mut usb_ep, req: *mut usb_request); pub fn usb_ep_queue(ep: *mut usb_ep, req: *mut usb_request, gfp_flags: gfp_t) -> i32;
    pub fn usb_ep_dequeue(ep: *mut usb_ep, req: *mut usb_request) -> i32; pub fn usb_ep_set_halt(ep: *mut usb_ep) -> i32;
    pub fn usb_ep_clear_halt(ep: *mut usb_ep) -> i32; pub fn usb_ep_set_wedge(ep: *mut usb_ep) -> i32;
    pub fn usb_ep_fifo_status(ep: *mut usb_ep) -> i32; pub fn usb_ep_fifo_flush(ep: *mut usb_ep);
    pub fn usb_gadget_frame_number(gadget: *mut usb_gadget) -> i32; pub fn usb_gadget_wakeup(gadget: *mut usb_gadget) -> i32;
    pub fn usb_gadget_set_remote_wakeup(gadget: *mut usb_gadget, set: i32) -> i32; pub fn usb_gadget_set_selfpowered(gadget: *mut usb_gadget) -> i32;
    pub fn usb_gadget_clear_selfpowered(gadget: *mut usb_gadget) -> i32; pub fn usb_gadget_vbus_connect(gadget: *mut usb_gadget) -> i32;
    pub fn usb_gadget_vbus_draw(gadget: *mut usb_gadget, mA: u32) -> i32; pub fn usb_gadget_vbus_disconnect(gadget: *mut usb_gadget) -> i32;
    pub fn usb_gadget_connect(gadget: *mut usb_gadget) -> i32; pub fn usb_gadget_disconnect(gadget: *mut usb_gadget) -> i32;
    pub fn usb_gadget_deactivate(gadget: *mut usb_gadget) -> i32; pub fn usb_gadget_activate(gadget: *mut usb_gadget) -> i32;
    pub fn usb_gadget_check_config(gadget: *mut usb_gadget) -> i32;
    pub fn usb_gadget_register_driver_owner(driver: *mut usb_gadget_driver, owner: *mut module, mod_name: *const i8) -> i32;
    pub fn usb_gadget_unregister_driver(driver: *mut usb_gadget_driver) -> i32;
    pub fn usb_initialize_gadget(parent: *mut device, gadget: *mut usb_gadget, release: Option<unsafe extern "C" fn(*mut device)>);
    pub fn usb_add_gadget(gadget: *mut usb_gadget) -> i32; pub fn usb_del_gadget(gadget: *mut usb_gadget);
    pub fn usb_add_gadget_udc_release(parent: *mut device, gadget: *mut usb_gadget, release: Option<unsafe extern "C" fn(*mut device)>) -> i32;
    pub fn usb_add_gadget_udc(parent: *mut device, gadget: *mut usb_gadget) -> i32; pub fn usb_del_gadget_udc(gadget: *mut usb_gadget);
    pub fn usb_get_gadget_udc_name() -> *mut i8;
    pub fn usb_gadget_get_string(table: *const usb_gadget_strings, id: i32, buf: *mut u8) -> i32;
    pub fn usb_validate_langid(langid: u16) -> bool;
    pub fn usb_descriptor_fillbuf(buf: *mut core::ffi::c_void, len: u32, src: *const *const usb_descriptor_header) -> i32;
    pub fn usb_copy_descriptors(src: *mut *mut usb_descriptor_header) -> *mut *mut usb_descriptor_header;
    pub fn usb_assign_descriptors(f: *mut usb_function, fs: *mut *mut usb_descriptor_header, hs: *mut *mut usb_descriptor_header, ss: *mut *mut usb_descriptor_header, ssp: *mut *mut usb_descriptor_header) -> i32;
    pub fn usb_free_all_descriptors(f: *mut usb_function);
    pub fn usb_free_descriptors(v: *mut *mut usb_descriptor_header);
    pub fn usb_otg_descriptor_alloc(gadget: *mut usb_gadget) -> *mut usb_descriptor_header;
    pub fn usb_otg_descriptor_init(gadget: *mut usb_gadget, otg_desc: *mut usb_descriptor_header) -> i32;
    pub fn usb_gadget_ep_match_desc(gadget: *mut usb_gadget, ep: *mut usb_ep, desc: *mut usb_endpoint_descriptor, ep_comp: *mut usb_ss_ep_comp_descriptor) -> i32;
    pub fn usb_ep_autoconfig(gadget: *mut usb_gadget, desc: *mut usb_endpoint_descriptor) -> *mut usb_ep;
    pub fn usb_ep_autoconfig_ss(gadget: *mut usb_gadget, desc: *mut usb_endpoint_descriptor, comp: *mut usb_ss_ep_comp_descriptor) -> *mut usb_ep;
    pub fn usb_ep_autoconfig_release(ep: *mut usb_ep); pub fn usb_ep_autoconfig_reset(gadget: *mut usb_gadget);
    pub fn usb_gadget_map_request_by_dev(dev: *mut device, req: *mut usb_request, is_in: i32) -> i32;
    pub fn usb_gadget_map_request(gadget: *mut usb_gadget, req: *mut usb_request, is_in: i32) -> i32;
    pub fn usb_gadget_unmap_request_by_dev(dev: *mut device, req: *mut usb_request, is_in: i32);
    pub fn usb_gadget_unmap_request(gadget: *mut usb_gadget, req: *mut usb_request, is_in: i32);
    pub fn usb_gadget_set_state(gadget: *mut usb_gadget, state: usb_device_state);
    pub fn usb_gadget_udc_reset(gadget: *mut usb_gadget, driver: *mut usb_gadget_driver);
    pub fn usb_gadget_giveback_request(ep: *mut usb_ep, req: *mut usb_request);
    pub fn gadget_find_ep_by_name(g: *mut usb_gadget, name: *const i8) -> *mut usb_ep;
    pub fn usb_udc_vbus_handler(gadget: *mut usb_gadget, status: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
