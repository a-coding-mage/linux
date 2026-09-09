// SPDX-License-Identifier: GPL-2.0
// Counter interface. C header dependencies are supplied externally.

use core::ffi::c_void;

pub enum counter_device {}
pub enum counter_count {}
pub enum counter_synapse {}
pub enum counter_signal {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum counter_comp_type {
    COUNTER_COMP_U8,
    COUNTER_COMP_U64,
    COUNTER_COMP_BOOL,
    COUNTER_COMP_SIGNAL_LEVEL,
    COUNTER_COMP_FUNCTION,
    COUNTER_COMP_SYNAPSE_ACTION,
    COUNTER_COMP_ENUM,
    COUNTER_COMP_COUNT_DIRECTION,
    COUNTER_COMP_COUNT_MODE,
    COUNTER_COMP_SIGNAL_POLARITY,
    COUNTER_COMP_ARRAY,
}

// External types supplied by the corresponding kernel/UAPI headers.
pub type u8 = u8;
pub type u32 = u32;
pub type u64 = u64;
pub type size_t = usize;
pub enum counter_synapse_action {}
pub enum counter_function {}
pub enum counter_signal_level {}
pub enum counter_watch {}
pub enum device {}
pub enum cdev {}
pub enum list_head {}
pub enum spinlock_t {}
pub enum mutex {}
pub enum wait_queue_head_t {}
pub enum counter_event {}

#[repr(C)]
pub union counter_comp_read {
    pub action_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, *mut counter_synapse, *mut counter_synapse_action) -> i32>,
    pub device_u8_read: Option<unsafe extern "C" fn(*mut counter_device, *mut u8) -> i32>,
    pub count_u8_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, *mut u8) -> i32>,
    pub signal_u8_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_signal, *mut u8) -> i32>,
    pub device_u32_read: Option<unsafe extern "C" fn(*mut counter_device, *mut u32) -> i32>,
    pub count_u32_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, *mut u32) -> i32>,
    pub signal_u32_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_signal, *mut u32) -> i32>,
    pub device_u64_read: Option<unsafe extern "C" fn(*mut counter_device, *mut u64) -> i32>,
    pub count_u64_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, *mut u64) -> i32>,
    pub signal_u64_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_signal, *mut u64) -> i32>,
    pub signal_array_u32_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_signal, usize, *mut u32) -> i32>,
    pub device_array_u64_read: Option<unsafe extern "C" fn(*mut counter_device, usize, *mut u64) -> i32>,
    pub count_array_u64_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, usize, *mut u64) -> i32>,
    pub signal_array_u64_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_signal, usize, *mut u64) -> i32>,
}

#[repr(C)]
pub union counter_comp_write {
    pub action_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, *mut counter_synapse, counter_synapse_action) -> i32>,
    pub device_u8_write: Option<unsafe extern "C" fn(*mut counter_device, u8) -> i32>,
    pub count_u8_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, u8) -> i32>,
    pub signal_u8_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_signal, u8) -> i32>,
    pub device_u32_write: Option<unsafe extern "C" fn(*mut counter_device, u32) -> i32>,
    pub count_u32_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, u32) -> i32>,
    pub signal_u32_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_signal, u32) -> i32>,
    pub device_u64_write: Option<unsafe extern "C" fn(*mut counter_device, u64) -> i32>,
    pub count_u64_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, u64) -> i32>,
    pub signal_u64_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_signal, u64) -> i32>,
    pub signal_array_u32_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_signal, usize, u32) -> i32>,
    pub device_array_u64_write: Option<unsafe extern "C" fn(*mut counter_device, usize, u64) -> i32>,
    pub count_array_u64_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, usize, u64) -> i32>,
    pub signal_array_u64_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_signal, usize, u64) -> i32>,
}

#[repr(C)]
pub struct counter_comp { pub type_: counter_comp_type, pub name: *const i8, pub priv_: *mut c_void, pub read: counter_comp_read, pub write: counter_comp_write }

#[repr(C)]
pub struct counter_signal { pub id: i32, pub name: *const i8, pub ext: *mut counter_comp, pub num_ext: usize }
#[repr(C)]
pub struct counter_synapse { pub actions_list: *const counter_synapse_action, pub num_actions: usize, pub signal: *mut counter_signal }
#[repr(C)]
pub struct counter_count { pub id: i32, pub name: *const i8, pub functions_list: *const counter_function, pub num_functions: usize, pub synapses: *mut counter_synapse, pub num_synapses: usize, pub ext: *mut counter_comp, pub num_ext: usize }
#[repr(C)]
pub struct counter_event_node { pub l: list_head, pub event: u8, pub channel: u8, pub comp_list: list_head }

#[repr(C)]
pub struct counter_ops {
    pub signal_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_signal, *mut counter_signal_level) -> i32>,
    pub count_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, *mut u64) -> i32>,
    pub count_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, u64) -> i32>,
    pub function_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, *mut counter_function) -> i32>,
    pub function_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, counter_function) -> i32>,
    pub action_read: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, *mut counter_synapse, *mut counter_synapse_action) -> i32>,
    pub action_write: Option<unsafe extern "C" fn(*mut counter_device, *mut counter_count, *mut counter_synapse, counter_synapse_action) -> i32>,
    pub events_configure: Option<unsafe extern "C" fn(*mut counter_device) -> i32>,
    pub watch_validate: Option<unsafe extern "C" fn(*mut counter_device, *const counter_watch) -> i32>,
}

#[repr(C)]
pub struct counter_available { pub enums: *const u32, pub num_items: usize }
#[repr(C)]
pub struct counter_array { pub type_: counter_comp_type, pub avail: *const counter_available, pub length: usize }

#[repr(C)]
pub struct counter_device {
    pub name: *const i8,
    pub parent: *mut device,
    pub ops: *const counter_ops,
    pub signals: *mut counter_signal,
    pub num_signals: usize,
    pub counts: *mut counter_count,
    pub num_counts: usize,
    pub ext: *mut counter_comp,
    pub num_ext: usize,
    pub dev: device,
    pub chrdev: cdev,
    pub events_list: list_head,
    pub events_list_lock: spinlock_t,
    pub next_events_list: list_head,
    pub n_events_list_lock: mutex,
    pub events: *mut counter_event,
    pub events_wait: wait_queue_head_t,
    pub events_in_lock: spinlock_t,
    pub events_out_lock: mutex,
    pub ops_exist_lock: mutex,
}

#[macro_export] macro_rules! COUNTER_COMP_DEVICE_U8 { ($name:expr, $read:expr, $write:expr) => { counter_comp { type_: counter_comp_type::COUNTER_COMP_U8, name: $name as *const i8, priv_: core::ptr::null_mut(), read: counter_comp_read { device_u8_read: Some($read) }, write: counter_comp_write { device_u8_write: Some($write) } } }; }
#[macro_export] macro_rules! COUNTER_COMP_COUNT_U8 { ($name:expr, $read:expr, $write:expr) => { COUNTER_COMP_DEVICE_U8!($name, $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_SIGNAL_U8 { ($name:expr, $read:expr, $write:expr) => { COUNTER_COMP_DEVICE_U8!($name, $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_DEVICE_U64 { ($name:expr, $read:expr, $write:expr) => { counter_comp { type_: counter_comp_type::COUNTER_COMP_U64, name: $name as *const i8, priv_: core::ptr::null_mut(), read: counter_comp_read { device_u64_read: Some($read) }, write: counter_comp_write { device_u64_write: Some($write) } } }; }
#[macro_export] macro_rules! COUNTER_COMP_COUNT_U64 { ($name:expr, $read:expr, $write:expr) => { COUNTER_COMP_DEVICE_U64!($name, $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_SIGNAL_U64 { ($name:expr, $read:expr, $write:expr) => { COUNTER_COMP_DEVICE_U64!($name, $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_DEVICE_BOOL { ($name:expr, $read:expr, $write:expr) => { COUNTER_COMP_DEVICE_U8!($name, $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_COUNT_BOOL { ($name:expr, $read:expr, $write:expr) => { COUNTER_COMP_COUNT_U8!($name, $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_SIGNAL_BOOL { ($name:expr, $read:expr, $write:expr) => { COUNTER_COMP_SIGNAL_U8!($name, $read, $write) }; }

#[macro_export] macro_rules! COUNTER_COMP_CAPTURE { ($read:expr, $write:expr) => { COUNTER_COMP_COUNT_U64!("capture", $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_CEILING { ($read:expr, $write:expr) => { COUNTER_COMP_COUNT_U64!("ceiling", $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_COMPARE { ($read:expr, $write:expr) => { COUNTER_COMP_COUNT_U64!("compare", $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_ENABLE { ($read:expr, $write:expr) => { COUNTER_COMP_COUNT_BOOL!("enable", $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_FLOOR { ($read:expr, $write:expr) => { COUNTER_COMP_COUNT_U64!("floor", $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_FREQUENCY { ($read:expr) => { COUNTER_COMP_SIGNAL_U64!("frequency", $read, None) }; }
#[macro_export] macro_rules! COUNTER_COMP_PRESET { ($read:expr, $write:expr) => { COUNTER_COMP_COUNT_U64!("preset", $read, $write) }; }
#[macro_export] macro_rules! COUNTER_COMP_PRESET_ENABLE { ($read:expr, $write:expr) => { COUNTER_COMP_COUNT_BOOL!("preset_enable", $read, $write) }; }

#[macro_export] macro_rules! DEFINE_COUNTER_AVAILABLE { ($name:ident, $enums:expr) => { pub static mut $name: counter_available = counter_available { enums: $enums.as_ptr(), num_items: $enums.len() }; }; }
#[macro_export] macro_rules! DEFINE_COUNTER_ENUM { ($name:ident, $strs:expr) => { pub static mut $name: counter_available = counter_available { enums: $strs.as_ptr() as *const u32, num_items: $strs.len() }; }; }
#[macro_export] macro_rules! DEFINE_COUNTER_ARRAY_U64 { ($name:ident, $length:expr) => { pub static mut $name: counter_array = counter_array { type_: counter_comp_type::COUNTER_COMP_U64, avail: core::ptr::null(), length: $length }; }; }
#[macro_export] macro_rules! DEFINE_COUNTER_ARRAY_CAPTURE { ($name:ident, $length:expr) => { DEFINE_COUNTER_ARRAY_U64!($name, $length); }; }

// C declarations retained as external interfaces.
extern "C" {
    pub fn counter_priv(counter: *const counter_device) -> *mut c_void;
    pub fn counter_alloc(sizeof_priv: usize) -> *mut counter_device;
    pub fn counter_put(counter: *mut counter_device);
    pub fn counter_add(counter: *mut counter_device) -> i32;
    pub fn counter_unregister(counter: *mut counter_device);
    pub fn devm_counter_alloc(dev: *mut device, sizeof_priv: usize) -> *mut counter_device;
    pub fn devm_counter_add(dev: *mut device, counter: *mut counter_device) -> i32;
    pub fn counter_push_event(counter: *mut counter_device, event: u8, channel: u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
