/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/usb.h.  Names supplied by included kernel headers
 * remain external dependencies, as in the original header. */

pub const USB_MAJOR: i32 = 180;
pub const USB_DEVICE_MAJOR: i32 = 189;

pub const USB_MAXENDPOINTS: usize = 30;
pub const USB_MAXINTERFACES: usize = 32;
pub const USB_MAXIADS: usize = USB_MAXINTERFACES / 2;
pub const USB_RESUME_TIMEOUT: i32 = 40;

#[repr(C)] pub struct usb_device;
#[repr(C)] pub struct usb_driver;
#[repr(C)] pub struct ep_device;
#[repr(C)] pub struct usb_dev_state;
#[repr(C)] pub struct usb_tt;

#[repr(C)] pub struct usb_host_endpoint {
    pub desc: usb_endpoint_descriptor,
    pub ss_ep_comp: usb_ss_ep_comp_descriptor,
    pub ssp_isoc_ep_comp: usb_ssp_isoc_ep_comp_descriptor,
    pub eusb2_isoc_ep_comp: usb_eusb2_isoc_ep_comp_descriptor,
    pub urb_list: list_head,
    pub hcpriv: *mut core::ffi::c_void,
    pub ep_dev: *mut ep_device,
    pub extra: *mut u8,
    pub extralen: i32,
    pub enabled: i32,
    pub streams: i32,
}

#[repr(C)] pub struct usb_host_interface {
    pub desc: usb_interface_descriptor,
    pub extralen: i32,
    pub extra: *mut u8,
    pub endpoint: *mut usb_host_endpoint,
    pub string: *mut i8,
}

#[repr(C)] pub enum usb_interface_condition { USB_INTERFACE_UNBOUND = 0, USB_INTERFACE_BINDING, USB_INTERFACE_BOUND, USB_INTERFACE_UNBINDING }
#[repr(C)] pub enum usb_wireless_status { USB_WIRELESS_STATUS_NA = 0, USB_WIRELESS_STATUS_DISCONNECTED, USB_WIRELESS_STATUS_CONNECTED }
#[repr(C)] pub enum usb_link_tunnel_mode { USB_LINK_UNKNOWN = 0, USB_LINK_NATIVE, USB_LINK_TUNNELED }
#[repr(C)] pub enum usb_port_connect_type { USB_PORT_CONNECT_TYPE_UNKNOWN = 0, USB_PORT_CONNECT_TYPE_HOT_PLUG, USB_PORT_CONNECT_TYPE_HARD_WIRED, USB_PORT_NOT_USED }

#[repr(C)] pub struct usb_interface {
    pub altsetting: *mut usb_host_interface,
    pub cur_altsetting: *mut usb_host_interface,
    pub num_altsetting: u32,
    pub intf_assoc: *mut usb_interface_assoc_descriptor,
    pub minor: i32,
    pub condition: usb_interface_condition,
    pub sysfs_files_created: u32,
    pub ep_devs_created: u32,
    pub unregistering: u32,
    pub needs_remote_wakeup: u32,
    pub needs_altsetting0: u32,
    pub needs_binding: u32,
    pub resetting_device: u32,
    pub authorized: u32,
    pub wireless_status: usb_wireless_status,
    pub wireless_status_work: work_struct,
    pub dev: device,
    pub usb_dev: *mut device,
    pub reset_ws: work_struct,
}

#[repr(C)] pub struct usb_interface_cache { pub num_altsetting: u32, pub ref_: kref, pub altsetting: [usb_host_interface; 0] }
#[repr(C)] pub struct usb_host_config {
    pub desc: usb_config_descriptor,
    pub string: *mut i8,
    pub intf_assoc: [*mut usb_interface_assoc_descriptor; USB_MAXIADS],
    pub interface: [*mut usb_interface; USB_MAXINTERFACES],
    pub intf_cache: [*mut usb_interface_cache; USB_MAXINTERFACES],
    pub extra: *mut u8,
    pub extralen: i32,
}
#[repr(C)] pub struct usb_host_bos { pub desc: *mut usb_bos_descriptor, pub ext_cap: *mut usb_ext_cap_descriptor, pub ss_cap: *mut usb_ss_cap_descriptor, pub ssp_cap: *mut usb_ssp_cap_descriptor, pub ss_id: *mut usb_ss_container_id_descriptor, pub ptm_cap: *mut usb_ptm_cap_descriptor }

#[repr(C)] pub struct usb_bus {
    pub controller: *mut device, pub sysdev: *mut device, pub busnum: i32, pub bus_name: *const i8,
    pub uses_pio_for_control: u8, pub otg_port: u8, pub is_b_host: u32, pub b_hnp_enable: u32,
    pub no_stop_on_short: u32, pub no_sg_constraint: u32, pub sg_tablesize: u32, pub devnum_next: i32,
    pub devnum_next_mutex: mutex, pub devmap: [u64; 2], pub root_hub: *mut usb_device,
    pub hs_companion: *mut usb_bus, pub bandwidth_allocated: i32, pub bandwidth_int_reqs: i32,
    pub bandwidth_isoc_reqs: i32, pub resuming_ports: u32,
}

#[repr(C)] pub struct usb2_lpm_parameters { pub besl: u32, pub timeout: i32 }
#[repr(C)] pub struct usb3_lpm_parameters { pub mel: u32, pub pel: u32, pub sel: u32, pub timeout: i32 }

#[repr(C)] pub struct usb_device {
    pub devnum: i32, pub devpath: [i8; 16], pub route: u32, pub state: usb_device_state, pub speed: usb_device_speed,
    pub rx_lanes: u32, pub tx_lanes: u32, pub ssp_rate: usb_ssp_rate, pub tt: *mut usb_tt, pub ttport: i32,
    pub toggle: [u32; 2], pub parent: *mut usb_device, pub bus: *mut usb_bus, pub ep0: usb_host_endpoint,
    pub dev: device, pub descriptor: usb_device_descriptor, pub bos: *mut usb_host_bos, pub config: *mut usb_host_config,
    pub actconfig: *mut usb_host_config, pub ep_in: [*mut usb_host_endpoint; 16], pub ep_out: [*mut usb_host_endpoint; 16],
    pub rawdescriptors: *mut *mut i8, pub bus_mA: u16, pub portnum: u8, pub level: u8, pub devaddr: u8,
    pub can_submit: u32, pub persist_enabled: u32, pub reset_in_progress: u32, pub have_langid: u32, pub authorized: u32,
    pub authenticated: u32, pub lpm_capable: u32, pub lpm_devinit_allow: u32, pub usb2_hw_lpm_capable: u32,
    pub usb2_hw_lpm_besl_capable: u32, pub usb2_hw_lpm_enabled: u32, pub usb2_hw_lpm_allowed: u32,
    pub usb3_lpm_u1_enabled: u32, pub usb3_lpm_u2_enabled: u32, pub string_langid: i32, pub product: *mut i8,
    pub manufacturer: *mut i8, pub serial: *mut i8, pub filelist: list_head, pub maxchild: i32, pub quirks: u32,
    pub urbnum: atomic_t, pub active_duration: usize, pub connect_time: usize, pub do_remote_wakeup: u32,
    pub reset_resume: u32, pub port_is_suspended: u32, pub offload_pm_locked: u32, pub offload_usage: i32,
    pub offload_lock: spinlock_t, pub tunnel_mode: usb_link_tunnel_mode, pub usb4_link: *mut device_link, pub slot_id: i32,
    pub l1_params: usb2_lpm_parameters, pub u1_params: usb3_lpm_parameters, pub u2_params: usb3_lpm_parameters,
    pub lpm_disable_count: u32, pub hub_delay: u16, pub use_generic_driver: u32,
}

pub const USB_PORT_QUIRK_OLD_SCHEME: u32 = 1 << 0;
pub const USB_PORT_QUIRK_FAST_ENUM: u32 = 1 << 1;
pub const URB_SHORT_NOT_OK: u32 = 0x0001; pub const URB_ISO_ASAP: u32 = 0x0002; pub const URB_NO_TRANSFER_DMA_MAP: u32 = 0x0004;
pub const URB_ZERO_PACKET: u32 = 0x0040; pub const URB_NO_INTERRUPT: u32 = 0x0080; pub const URB_FREE_BUFFER: u32 = 0x0100;
pub const URB_DIR_IN: u32 = 0x0200; pub const URB_DIR_OUT: u32 = 0; pub const URB_DIR_MASK: u32 = URB_DIR_IN;
pub const USB_MAX_SYNCHRONOUS_TIMEOUT: u32 = 60000;

#[repr(C)] pub struct usb_iso_packet_descriptor { pub offset: u32, pub length: u32, pub actual_length: u32, pub status: i32 }
#[repr(C)] pub struct usb_anchor { pub urb_list: list_head, pub wait: wait_queue_head_t, pub lock: spinlock_t, pub suspend_wakeups: atomic_t, pub poisoned: u32 }
#[repr(C)] pub struct urb {
    pub kref: kref, pub unlinked: i32, pub hcpriv: *mut core::ffi::c_void, pub use_count: atomic_t, pub reject: atomic_t,
    pub urb_list: list_head, pub anchor_list: list_head, pub anchor: *mut usb_anchor, pub dev: *mut usb_device,
    pub ep: *mut usb_host_endpoint, pub pipe: u32, pub stream_id: u32, pub status: i32, pub transfer_flags: u32,
    pub transfer_buffer: *mut core::ffi::c_void, pub transfer_dma: dma_addr_t, pub sg: *mut scatterlist, pub sgt: *mut sg_table,
    pub num_mapped_sgs: i32, pub num_sgs: i32, pub transfer_buffer_length: u32, pub actual_length: u32,
    pub setup_packet: *mut u8, pub setup_dma: dma_addr_t, pub start_frame: i32, pub number_of_packets: i32,
    pub interval: i32, pub error_count: i32, pub context: *mut core::ffi::c_void, pub complete: Option<unsafe extern "C" fn(*mut urb)>,
    pub iso_frame_desc: [usb_iso_packet_descriptor; 0],
}

pub const PIPE_ISOCHRONOUS: u32 = 0; pub const PIPE_INTERRUPT: u32 = 1; pub const PIPE_CONTROL: u32 = 2; pub const PIPE_BULK: u32 = 3;
pub const USB_DEVICE_ADD: u32 = 1; pub const USB_DEVICE_REMOVE: u32 = 2; pub const USB_BUS_ADD: u32 = 3; pub const USB_BUS_REMOVE: u32 = 4;
pub const USB_CTRL_GET_TIMEOUT: u32 = 5000; pub const USB_CTRL_SET_TIMEOUT: u32 = 5000;

pub unsafe fn usb_fill_control_urb(urb: *mut urb, dev: *mut usb_device, pipe: u32, setup_packet: *mut u8, transfer_buffer: *mut core::ffi::c_void, buffer_length: i32, complete_fn: Option<unsafe extern "C" fn(*mut urb)>, context: *mut core::ffi::c_void) { (*urb).dev=dev; (*urb).pipe=pipe; (*urb).setup_packet=setup_packet; (*urb).transfer_buffer=transfer_buffer; (*urb).transfer_buffer_length=buffer_length as u32; (*urb).complete=complete_fn; (*urb).context=context; }
pub unsafe fn usb_fill_bulk_urb(urb: *mut urb, dev: *mut usb_device, pipe: u32, transfer_buffer: *mut core::ffi::c_void, buffer_length: i32, complete_fn: Option<unsafe extern "C" fn(*mut urb)>, context: *mut core::ffi::c_void) { usb_fill_control_urb(urb,dev,pipe,core::ptr::null_mut(),transfer_buffer,buffer_length,complete_fn,context); }
pub unsafe fn usb_urb_dir_in(urb: *mut urb) -> i32 { (((*urb).transfer_flags & URB_DIR_MASK) == URB_DIR_IN) as i32 }
pub unsafe fn usb_urb_dir_out(urb: *mut urb) -> i32 { (((*urb).transfer_flags & URB_DIR_MASK) == URB_DIR_OUT) as i32 }

extern "C" {
    pub fn usb_get_dev(dev: *mut usb_device) -> *mut usb_device;
    pub fn usb_put_dev(dev: *mut usb_device);
    pub fn usb_alloc_urb(iso_packets: i32, mem_flags: gfp_t) -> *mut urb;
    pub fn usb_free_urb(urb: *mut urb);
    pub fn usb_submit_urb(urb: *mut urb, mem_flags: gfp_t) -> i32;
    pub fn usb_unlink_urb(urb: *mut urb) -> i32;
    pub fn usb_kill_urb(urb: *mut urb);
    pub fn usb_control_msg(dev: *mut usb_device, pipe: u32, request: u8, requesttype: u8, value: u16, index: u16, data: *mut core::ffi::c_void, size: u16, timeout: i32) -> i32;
    pub fn usb_reset_device(dev: *mut usb_device) -> i32;
    pub fn usb_set_interface(dev: *mut usb_device, ifnum: i32, alternate: i32) -> i32;
}

/* External Linux kernel declarations referenced by this header. */
extern "C" {
    pub type usb_endpoint_descriptor; pub type usb_ss_ep_comp_descriptor; pub type usb_ssp_isoc_ep_comp_descriptor; pub type usb_eusb2_isoc_ep_comp_descriptor;
    pub type usb_interface_descriptor; pub type usb_interface_assoc_descriptor; pub type usb_config_descriptor; pub type usb_bos_descriptor;
    pub type usb_ext_cap_descriptor; pub type usb_ss_cap_descriptor; pub type usb_ssp_cap_descriptor; pub type usb_ss_container_id_descriptor; pub type usb_ptm_cap_descriptor;
    pub type list_head; pub type kref; pub type device; pub type work_struct; pub type mutex; pub type atomic_t; pub type spinlock_t; pub type device_link;
    pub type wait_queue_head_t; pub type scatterlist; pub type sg_table; pub type dma_addr_t; pub type gfp_t; pub type usb_device_state; pub type usb_device_speed; pub type usb_ssp_rate;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
