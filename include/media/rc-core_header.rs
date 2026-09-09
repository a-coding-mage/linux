/* SPDX-License-Identifier: GPL-2.0-only */
/* Remote Controller core header -- translated from rc-core.h. */

// External kernel/media types referenced by this header are supplied by dependencies.
use core::ffi::c_void;

#[repr(C)]
pub enum rc_driver_type { RC_DRIVER_SCANCODE = 0, RC_DRIVER_IR_RAW, RC_DRIVER_IR_RAW_TX }

#[repr(C)]
pub struct rc_scancode_filter { pub data: u32, pub mask: u32 }

#[repr(C)]
pub enum rc_filter_type { RC_FILTER_NORMAL = 0, RC_FILTER_WAKEUP, RC_FILTER_MAX }

extern "C" {
    pub type list_head;
    pub type rc_map;
    pub type lirc_scancode;
    pub type wait_queue_head_t;
    pub type device;
    pub type attribute_group;
    pub type input_id;
    pub type mutex;
    pub type ir_raw_event_ctrl;
    pub type input_dev;
    pub type spinlock_t;
    pub type timer_list;
    pub type cdev;
    pub type ktime_t;
    pub type rc_proto;
}

#[repr(C)]
pub struct lirc_fh {
    pub list: *mut list_head,
    pub rc: *mut rc_dev,
    pub rawir: *mut c_void,
    pub scancodes: *mut c_void,
    pub wait_poll: *mut wait_queue_head_t,
    pub carrier_low: u32,
    pub send_mode: u8,
    pub rec_mode: u8,
}

#[repr(C)]
pub struct rc_dev {
    pub dev: device,
    pub registered: bool,
    pub idle: bool,
    pub encode_wakeup: bool,
    pub minor: u32,
    pub sysfs_groups: [*const attribute_group; 5],
    pub device_name: *const i8,
    pub input_phys: *const i8,
    pub input_id: input_id,
    pub driver_name: *const i8,
    pub map_name: *const i8,
    pub rc_map: rc_map,
    pub lock: mutex,
    pub raw: *mut ir_raw_event_ctrl,
    pub input_dev: *mut input_dev,
    pub driver_type: rc_driver_type,
    pub users: u32,
    pub allowed_protocols: u64,
    pub enabled_protocols: u64,
    pub allowed_wakeup_protocols: u64,
    pub wakeup_protocol: rc_proto,
    pub scancode_filter: rc_scancode_filter,
    pub scancode_wakeup_filter: rc_scancode_filter,
    pub scancode_mask: u32,
    pub priv_: *mut c_void,
    pub keylock: spinlock_t,
    pub keypressed: bool,
    pub last_toggle: u8,
    pub last_keycode: u32,
    pub last_protocol: rc_proto,
    pub last_scancode: u64,
    pub keyup_jiffies: usize,
    pub timer_keyup: timer_list,
    pub timer_repeat: timer_list,
    pub timeout: u32,
    pub min_timeout: u32,
    pub max_timeout: u32,
    pub rx_resolution: u32,
    #[cfg(feature = "CONFIG_LIRC")]
    pub lirc_dev: device,
    #[cfg(feature = "CONFIG_LIRC")]
    pub lirc_cdev: cdev,
    #[cfg(feature = "CONFIG_LIRC")]
    pub gap_start: ktime_t,
    #[cfg(feature = "CONFIG_LIRC")]
    pub lirc_fh_lock: spinlock_t,
    #[cfg(feature = "CONFIG_LIRC")]
    pub lirc_fh: list_head,
    pub change_protocol: Option<unsafe extern "C" fn(*mut rc_dev, *mut u64) -> i32>,
    pub open: Option<unsafe extern "C" fn(*mut rc_dev) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut rc_dev)>,
    pub s_tx_mask: Option<unsafe extern "C" fn(*mut rc_dev, u32) -> i32>,
    pub s_tx_carrier: Option<unsafe extern "C" fn(*mut rc_dev, u32) -> i32>,
    pub s_tx_duty_cycle: Option<unsafe extern "C" fn(*mut rc_dev, u32) -> i32>,
    pub s_rx_carrier_range: Option<unsafe extern "C" fn(*mut rc_dev, u32, u32) -> i32>,
    pub tx_ir: Option<unsafe extern "C" fn(*mut rc_dev, *mut u32, u32) -> i32>,
    pub s_idle: Option<unsafe extern "C" fn(*mut rc_dev, bool)>,
    pub s_wideband_receiver: Option<unsafe extern "C" fn(*mut rc_dev, i32) -> i32>,
    pub s_carrier_report: Option<unsafe extern "C" fn(*mut rc_dev, i32) -> i32>,
    pub s_filter: Option<unsafe extern "C" fn(*mut rc_dev, *mut rc_scancode_filter) -> i32>,
    pub s_wakeup_filter: Option<unsafe extern "C" fn(*mut rc_dev, *mut rc_scancode_filter) -> i32>,
    pub s_timeout: Option<unsafe extern "C" fn(*mut rc_dev, u32) -> i32>,
}

#[inline] pub unsafe fn to_rc_dev(d: *mut device) -> *mut rc_dev { d as *mut rc_dev }

extern "C" {
    pub fn rc_allocate_device(t: rc_driver_type) -> *mut rc_dev;
    pub fn devm_rc_allocate_device(dev: *mut device, t: rc_driver_type) -> *mut rc_dev;
    pub fn rc_free_device(dev: *mut rc_dev);
    pub fn rc_register_device(dev: *mut rc_dev) -> i32;
    pub fn devm_rc_register_device(parent: *mut device, dev: *mut rc_dev) -> i32;
    pub fn rc_unregister_device(dev: *mut rc_dev);
    pub fn rc_repeat(dev: *mut rc_dev);
    pub fn rc_keydown(dev: *mut rc_dev, protocol: rc_proto, scancode: u64, toggle: u8);
    pub fn rc_keydown_notimeout(dev: *mut rc_dev, protocol: rc_proto, scancode: u64, toggle: u8);
    pub fn rc_keyup(dev: *mut rc_dev);
    pub fn rc_g_keycode_from_table(dev: *mut rc_dev, scancode: u64) -> u32;
}

#[repr(C)]
pub union ir_raw_event_data { pub duration: u32, pub carrier: u32 }
#[repr(C)]
pub struct ir_raw_event { pub data: ir_raw_event_data, pub duty_cycle: u8, pub pulse: bool, pub overflow: bool, pub timeout: bool, pub carrier_report: bool }

pub const fn us_to_ns(usec: u32) -> u32 { usec.wrapping_mul(1000) }
pub const fn ms_to_us(msec: u32) -> u32 { msec.wrapping_mul(1000) }
pub const IR_MAX_DURATION: u32 = ms_to_us(1000);
pub const IR_DEFAULT_TIMEOUT: u32 = ms_to_us(125);
pub const IR_MAX_TIMEOUT: u32 = 0; // LIRC_VALUE_MASK is supplied externally.

extern "C" {
    pub fn ir_raw_event_handle(dev: *mut rc_dev);
    pub fn ir_raw_event_store(dev: *mut rc_dev, ev: *mut ir_raw_event) -> i32;
    pub fn ir_raw_event_store_edge(dev: *mut rc_dev, pulse: bool) -> i32;
    pub fn ir_raw_event_store_with_filter(dev: *mut rc_dev, ev: *mut ir_raw_event) -> i32;
    pub fn ir_raw_event_store_with_timeout(dev: *mut rc_dev, ev: *mut ir_raw_event) -> i32;
    pub fn ir_raw_event_set_idle(dev: *mut rc_dev, idle: bool);
    pub fn ir_raw_encode_scancode(protocol: rc_proto, scancode: u32, events: *mut ir_raw_event, max: u32) -> i32;
    pub fn ir_raw_encode_carrier(protocol: rc_proto) -> i32;
}

#[inline]
pub unsafe fn ir_raw_event_overflow(dev: *mut rc_dev) {
    let mut ev = ir_raw_event { data: ir_raw_event_data { duration: 0 }, duty_cycle: 0, pulse: false, overflow: true, timeout: false, carrier_report: false };
    ir_raw_event_store(dev, &mut ev);
    (*dev).idle = true;
    ir_raw_event_handle(dev);
}

#[inline]
pub fn ir_extract_bits(mut data: u32, mut mask: u32) -> u32 {
    let mut vbit = 1u32; let mut value = 0u32;
    loop { if mask & 1 != 0 { if data & 1 != 0 { value |= vbit; } vbit <<= 1; } data >>= 1; mask >>= 1; if mask == 0 { break; } }
    value
}

#[inline]
pub unsafe fn ir_nec_bytes_to_scancode(address: u8, not_address: u8, command: u8, not_command: u8, protocol: *mut rc_proto) -> u32 {
    if (command ^ not_command) != 0xff { *protocol = core::mem::transmute(0u32); ((not_address as u32) << 24) | ((address as u32) << 16) | ((not_command as u32) << 8) | command as u32 }
    else if (address ^ not_address) != 0xff { *protocol = core::mem::transmute(0u32); ((address as u32) << 16) | ((not_address as u32) << 8) | command as u32 }
    else { *protocol = core::mem::transmute(0u32); ((address as u32) << 8) | command as u32 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
