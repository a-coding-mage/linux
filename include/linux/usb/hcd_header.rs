// SPDX-License-Identifier: GPL-2.0+
// Translation of linux/usb/hcd.h. External kernel types and symbols are supplied elsewhere.

pub const MAX_TOPO_LEVEL: usize = 6;
pub const USB_PID_EXT: u8 = 0xf0;
pub const USB_PID_OUT: u8 = 0xe1;
pub const USB_PID_ACK: u8 = 0xd2;
pub const USB_PID_DATA0: u8 = 0xc3;
pub const USB_PID_PING: u8 = 0xb4;
pub const USB_PID_SOF: u8 = 0xa5;
pub const USB_PID_NYET: u8 = 0x96;
pub const USB_PID_DATA2: u8 = 0x87;
pub const USB_PID_SPLIT: u8 = 0x78;
pub const USB_PID_IN: u8 = 0x69;
pub const USB_PID_NAK: u8 = 0x5a;
pub const USB_PID_DATA1: u8 = 0x4b;
pub const USB_PID_PREAMBLE: u8 = 0x3c;
pub const USB_PID_ERR: u8 = 0x3c;
pub const USB_PID_SETUP: u8 = 0x2d;
pub const USB_PID_STALL: u8 = 0x1e;
pub const USB_PID_MDATA: u8 = 0x0f;

#[repr(C)] pub struct giveback_urb_bh { pub running: bool, pub high_prio: bool, pub lock: spinlock_t, pub head: list_head, pub bh: work_struct, pub completing_ep: *mut usb_host_endpoint }
#[repr(C)] pub struct usb_hcd {
    pub self_: usb_bus, pub kref: kref, pub product_desc: *const c_char, pub speed: c_int,
    pub irq_descr: [c_char; 24], pub rh_timer: timer_list, pub status_urb: *mut urb,
    #[cfg(feature = "CONFIG_PM")] pub wakeup_work: work_struct,
    pub died_work: work_struct, pub driver: *const hc_driver, pub usb_phy: *mut usb_phy,
    pub phy_roothub: *mut usb_phy_roothub, pub flags: c_ulong, pub dev_policy: usb_dev_authorize_policy,
    pub rh_registered: u32, pub rh_pollable: u32, pub msix_enabled: u32, pub msi_enabled: u32,
    pub skip_phy_initialization: u32, pub uses_new_polling: u32, pub has_tt: u32,
    pub amd_resume_bug: u32, pub can_do_streams: u32, pub tpl_support: u32, pub cant_recv_wakeups: u32,
    pub irq: c_uint, pub regs: *mut c_void, pub rsrc_start: resource_size_t, pub rsrc_len: resource_size_t,
    pub power_budget: c_uint, pub high_prio_bh: giveback_urb_bh, pub low_prio_bh: giveback_urb_bh,
    pub address0_mutex: *mut mutex, pub bandwidth_mutex: *mut mutex, pub shared_hcd: *mut usb_hcd,
    pub primary_hcd: *mut usb_hcd, pub pool: [*mut dma_pool; 4], pub state: c_int,
    pub localmem_pool: *mut gen_pool,
    pub hcd_priv: [c_ulong; 0],
}
#[repr(C)] pub enum usb_dev_authorize_policy { USB_DEVICE_AUTHORIZE_NONE=0, USB_DEVICE_AUTHORIZE_ALL=1, USB_DEVICE_AUTHORIZE_INTERNAL=2 }

pub const HCD_FLAG_HW_ACCESSIBLE: u32=0; pub const HCD_FLAG_POLL_RH:u32=2; pub const HCD_FLAG_POLL_PENDING:u32=3;
pub const HCD_FLAG_WAKEUP_PENDING:u32=4; pub const HCD_FLAG_RH_RUNNING:u32=5; pub const HCD_FLAG_DEAD:u32=6;
pub const HCD_FLAG_INTF_AUTHORIZED:u32=7; pub const HCD_FLAG_DEFER_RH_REGISTER:u32=8;
pub const HCD_MEMORY:c_int=0x0001; pub const HCD_DMA:c_int=0x0002; pub const HCD_SHARED:c_int=0x0004;
pub const HCD_USB11:c_int=0x0010; pub const HCD_USB2:c_int=0x0020; pub const HCD_USB3:c_int=0x0040;
pub const HCD_USB31:c_int=0x0050; pub const HCD_USB32:c_int=0x0060; pub const HCD_MASK:c_int=0x0070; pub const HCD_BH:c_int=0x0100;
pub const EHSET_TEST_SINGLE_STEP_SET_FEATURE: u16=0x06;

#[inline] pub unsafe fn HCD_HW_ACCESSIBLE(h: *const usb_hcd)->c_ulong { (*h).flags & (1 << HCD_FLAG_HW_ACCESSIBLE) }
#[inline] pub unsafe fn HCD_POLL_RH(h:*const usb_hcd)->c_ulong { (*h).flags & (1 << HCD_FLAG_POLL_RH) }
#[inline] pub unsafe fn HCD_POLL_PENDING(h:*const usb_hcd)->c_ulong { (*h).flags & (1 << HCD_FLAG_POLL_PENDING) }
#[inline] pub unsafe fn HCD_WAKEUP_PENDING(h:*const usb_hcd)->c_ulong { (*h).flags & (1 << HCD_FLAG_WAKEUP_PENDING) }
#[inline] pub unsafe fn HCD_RH_RUNNING(h:*const usb_hcd)->c_ulong { (*h).flags & (1 << HCD_FLAG_RH_RUNNING) }
#[inline] pub unsafe fn HCD_DEAD(h:*const usb_hcd)->c_ulong { (*h).flags & (1 << HCD_FLAG_DEAD) }
#[inline] pub unsafe fn HCD_DEFER_RH_REGISTER(h:*const usb_hcd)->c_ulong { (*h).flags & (1 << HCD_FLAG_DEFER_RH_REGISTER) }
#[inline] pub unsafe fn HCD_INTF_AUTHORIZED(h:*const usb_hcd)->c_ulong { (*h).flags & (1 << HCD_FLAG_INTF_AUTHORIZED) }

#[repr(C)] pub struct hc_driver {
 pub description:*const c_char, pub product_desc:*const c_char, pub hcd_priv_size:usize,
 pub irq:Option<unsafe extern "C" fn(*mut usb_hcd)->irqreturn_t>, pub flags:c_int,
 pub reset:Option<unsafe extern "C" fn(*mut usb_hcd)->c_int>, pub start:Option<unsafe extern "C" fn(*mut usb_hcd)->c_int>,
 pub pci_suspend:Option<unsafe extern "C" fn(*mut usb_hcd,bool)->c_int>, pub pci_resume:Option<unsafe extern "C" fn(*mut usb_hcd,pm_message_t)->c_int>,
 pub pci_poweroff_late:Option<unsafe extern "C" fn(*mut usb_hcd,bool)->c_int>, pub stop:Option<unsafe extern "C" fn(*mut usb_hcd)>, pub shutdown:Option<unsafe extern "C" fn(*mut usb_hcd)>,
 pub get_frame_number:Option<unsafe extern "C" fn(*mut usb_hcd)->c_int>, pub urb_enqueue:Option<unsafe extern "C" fn(*mut usb_hcd,*mut urb,gfp_t)->c_int>, pub urb_dequeue:Option<unsafe extern "C" fn(*mut usb_hcd,*mut urb,c_int)->c_int>,
 pub map_urb_for_dma:Option<unsafe extern "C" fn(*mut usb_hcd,*mut urb,gfp_t)->c_int>, pub unmap_urb_for_dma:Option<unsafe extern "C" fn(*mut usb_hcd,*mut urb)>,
 pub endpoint_disable:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_host_endpoint)>, pub endpoint_reset:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_host_endpoint)>,
 pub hub_status_data:Option<unsafe extern "C" fn(*mut usb_hcd,*mut c_char)->c_int>, pub hub_control:Option<unsafe extern "C" fn(*mut usb_hcd,u16,u16,u16,*mut c_char,u16)->c_int>,
 pub bus_suspend:Option<unsafe extern "C" fn(*mut usb_hcd)->c_int>, pub bus_resume:Option<unsafe extern "C" fn(*mut usb_hcd)->c_int>, pub start_port_reset:Option<unsafe extern "C" fn(*mut usb_hcd,c_uint)->c_int>, pub get_resuming_ports:Option<unsafe extern "C" fn(*mut usb_hcd)->c_ulong>,
 pub relinquish_port:Option<unsafe extern "C" fn(*mut usb_hcd,c_int)>, pub port_handed_over:Option<unsafe extern "C" fn(*mut usb_hcd,c_int)->c_int>, pub clear_tt_buffer_complete:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_host_endpoint)>,
 pub alloc_dev:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device)->c_int>, pub free_dev:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device)>,
 pub alloc_streams:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device,*mut *mut usb_host_endpoint,c_uint,c_uint,gfp_t)->c_int>, pub free_streams:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device,*mut *mut usb_host_endpoint,c_uint,gfp_t)->c_int>,
 pub add_endpoint:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device,*mut usb_host_endpoint)->c_int>, pub drop_endpoint:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device,*mut usb_host_endpoint)->c_int>, pub check_bandwidth:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device)->c_int>, pub reset_bandwidth:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device)>,
 pub address_device:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device,c_uint)->c_int>, pub enable_device:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device)->c_int>, pub update_hub_device:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device,*mut usb_tt,gfp_t)->c_int>, pub reset_device:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device)->c_int>, pub update_device:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device)->c_int>, pub set_usb2_hw_lpm:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device,c_int)->c_int>,
 pub enable_usb3_lpm_timeout:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device,usb3_link_state)->c_int>, pub disable_usb3_lpm_timeout:Option<unsafe extern "C" fn(*mut usb_hcd,*mut usb_device,usb3_link_state)->c_int>, pub find_raw_port_number:Option<unsafe extern "C" fn(*mut usb_hcd,c_int)->c_int>, pub port_power:Option<unsafe extern "C" fn(*mut usb_hcd,c_int,bool)->c_int>, pub submit_single_step_set_feature:Option<unsafe extern "C" fn(*mut usb_hcd,*mut urb,c_int)->c_int>,
}

#[inline] pub unsafe fn hcd_to_bus(h:*mut usb_hcd)->*mut usb_bus { &mut (*h).self_ }
#[inline] pub unsafe fn bus_to_hcd(b:*mut usb_bus)->*mut usb_hcd { b as *mut usb_hcd }
#[inline] pub unsafe fn hcd_giveback_urb_in_bh(h:*mut usb_hcd)->c_int { (*h).driver.as_ref().unwrap().flags & HCD_BH }
#[inline] pub unsafe fn hcd_periodic_completion_in_progress(h:*mut usb_hcd,e:*mut usb_host_endpoint)->bool { (*h).high_prio_bh.completing_ep==e }
#[inline] pub unsafe fn hcd_uses_dma(h:*mut usb_hcd)->bool { (*h).driver.as_ref().unwrap().flags & HCD_DMA != 0 }

#[repr(C)] pub struct usb_tt { pub hub:*mut usb_device, pub multi:c_int, pub think_time:c_uint, pub hcpriv:*mut c_void, pub lock:spinlock_t, pub clear_list:list_head, pub clear_work:work_struct }
#[repr(C)] pub struct usb_tt_clear { pub clear_list:list_head, pub tt:c_uint, pub devinfo:u16, pub hcd:*mut usb_hcd, pub ep:*mut usb_host_endpoint }
pub const FRAME_TIME_USECS:c_ulong=1000; pub const BW_HOST_DELAY:c_ulong=1000; pub const BW_HUB_LS_SETUP:c_ulong=333; pub const FRAME_TIME_BITS:c_ulong=12000;
#[inline] pub const fn BitTime(bytecount:c_ulong)->c_ulong { 7*8*bytecount/6 }
#[inline] pub const fn NS_TO_US(ns:c_ulong)->c_ulong { (ns+999)/1000 }
#[inline] pub const fn HS_NSECS(bytes:c_ulong)->c_ulong { ((55*8*2083)+(2083*(3+BitTime(bytes))))/1000+5 }
#[inline] pub const fn HS_NSECS_ISO(bytes:c_ulong)->c_ulong { ((38*8*2083)+(2083*(3+BitTime(bytes))))/1000+5 }
#[inline] pub const fn HS_USECS(bytes:c_ulong)->c_ulong { NS_TO_US(HS_NSECS(bytes)) }
#[inline] pub const fn HS_USECS_ISO(bytes:c_ulong)->c_ulong { NS_TO_US(HS_NSECS_ISO(bytes)) }

pub type c_char=i8; pub type c_int=i32; pub type c_uint=u32; pub type c_ulong=usize; pub type c_void=core::ffi::c_void;
// External kernel declarations and configuration-dependent APIs.
extern "C" {
 pub static mut usb_bus_idr: idr; pub static mut usb_bus_idr_lock: mutex; pub static mut usb_kill_urb_queue: wait_queue_head_t;
 pub fn usb_hcd_link_urb_to_ep(*mut usb_hcd,*mut urb)->c_int; pub fn usb_hcd_check_unlink_urb(*mut usb_hcd,*mut urb,c_int)->c_int; pub fn usb_hcd_unlink_urb_from_ep(*mut usb_hcd,*mut urb);
 pub fn usb_hcd_submit_urb(*mut urb,gfp_t)->c_int; pub fn usb_hcd_unlink_urb(*mut urb,c_int)->c_int; pub fn usb_hcd_giveback_urb(*mut usb_hcd,*mut urb,c_int);
 pub fn usb_hcd_map_urb_for_dma(*mut usb_hcd,*mut urb,gfp_t)->c_int; pub fn usb_hcd_unmap_urb_setup_for_dma(*mut usb_hcd,*mut urb); pub fn usb_hcd_unmap_urb_for_dma(*mut usb_hcd,*mut urb);
 pub fn usb_hcd_flush_endpoint(*mut usb_device,*mut usb_host_endpoint); pub fn usb_hcd_disable_endpoint(*mut usb_device,*mut usb_host_endpoint); pub fn usb_hcd_reset_endpoint(*mut usb_device,*mut usb_host_endpoint); pub fn usb_hcd_synchronize_unlinks(*mut usb_device);
 pub fn usb_hcd_get_frame_number(*mut usb_device)->c_int; pub fn usb_hc_died(*mut usb_hcd); pub fn usb_hcd_poll_rh_status(*mut usb_hcd); pub fn usb_wakeup_notification(*mut usb_device,c_uint);
 pub fn usb_hcd_start_port_resume(*mut usb_bus,c_int); pub fn usb_hcd_end_port_resume(*mut usb_bus,c_int); pub fn usb_hub_clear_tt_buffer(*mut urb)->c_int; pub fn usb_ep0_reinit(*mut usb_device);
 pub fn usb_create_hcd(*const hc_driver,*mut device,*const c_char)->*mut usb_hcd;
 pub fn usb_create_shared_hcd(*const hc_driver,*mut device,*const c_char,*mut usb_hcd)->*mut usb_hcd;
 pub fn usb_get_hcd(*mut usb_hcd)->*mut usb_hcd; pub fn usb_put_hcd(*mut usb_hcd); pub fn usb_hcd_is_primary_hcd(*mut usb_hcd)->c_int;
 pub fn usb_add_hcd(*mut usb_hcd,c_uint,c_ulong)->c_int; pub fn usb_remove_hcd(*mut usb_hcd); pub fn usb_hcd_find_raw_port_number(*mut usb_hcd,c_int)->c_int;
 pub fn usb_init_pool_max(); pub fn hcd_buffer_create(*mut usb_hcd)->c_int; pub fn hcd_buffer_destroy(*mut usb_hcd);
 pub fn usb_hcd_irq(c_int,*mut c_void)->irqreturn_t; pub fn usb_alloc_dev(*mut usb_device,*mut usb_bus,c_uint)->*mut usb_device; pub fn usb_new_device(*mut usb_device)->c_int; pub fn usb_disconnect(*mut *mut usb_device);
 pub fn usb_get_configuration(*mut usb_device)->c_int; pub fn usb_destroy_configuration(*mut usb_device); pub fn usb_set_device_state(*mut usb_device,usb_device_state);
}

pub const __ACTIVE:c_int=0x01; pub const __SUSPEND:c_int=0x04; pub const __TRANSIENT:c_int=0x80;
pub const HC_STATE_HALT:c_int=0; pub const HC_STATE_RUNNING:c_int=__ACTIVE; pub const HC_STATE_QUIESCING:c_int=__SUSPEND|__TRANSIENT|__ACTIVE; pub const HC_STATE_RESUMING:c_int=__SUSPEND|__TRANSIENT; pub const HC_STATE_SUSPENDED:c_int=__SUSPEND;
#[inline] pub const fn HC_IS_RUNNING(state:c_int)->c_int { state & __ACTIVE }
#[inline] pub const fn HC_IS_SUSPENDED(state:c_int)->c_int { state & __SUSPEND }
#[inline] pub const fn usb_endpoint_out(ep_dir:u32)->bool { ep_dir & USB_DIR_IN == 0 }
pub const HCD_BUFFER_POOLS:usize=4;
pub type gfp_t=usize; pub type resource_size_t=usize; pub type irqreturn_t=c_int; pub type pm_message_t=c_int; pub type dma_addr_t=u64; pub type phys_addr_t=u64;
pub enum usb3_link_state {} pub enum usb_device_state {}
pub enum usb_bus {} pub enum urb {} pub enum usb_host_endpoint {} pub enum usb_phy {} pub enum usb_phy_roothub {} pub enum kref {} pub enum spinlock_t {} pub enum list_head {} pub enum work_struct {} pub enum timer_list {} pub enum mutex {} pub enum dma_pool {} pub enum gen_pool {} pub enum device {} pub enum idr {} pub enum wait_queue_head_t {} pub enum usb_device {} pub enum usb_host_config {} pub enum usb_host_interface {} pub enum usb_tt_type {}
pub const USB_DIR_IN:u32=0x80;
#[cfg(feature="CONFIG_USB_HCD_TEST_MODE")] extern "C" { pub fn ehset_single_step_set_feature(*mut usb_hcd,c_int)->c_int; }
#[cfg(not(feature="CONFIG_USB_HCD_TEST_MODE"))] #[inline] pub unsafe fn ehset_single_step_set_feature(_: *mut usb_hcd,_:c_int)->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
