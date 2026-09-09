// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust translation of gpio/gpiolib-cdev.c.  The implementation
// intentionally keeps the Linux kernel ABI symbols and low-level operations as
// external dependencies supplied by the surrounding kernel translation.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Kernel-provided opaque types and constants are deliberately not redefined.
#[repr(C)]
pub struct linehandle_state {
    pub gdev: *mut gpio_device,
    pub label: *const c_char,
    pub descs: [*mut gpio_desc; GPIOHANDLES_MAX],
    pub num_descs: u32,
}

#[repr(C)]
pub struct line {
    pub desc: *mut gpio_desc,
    pub req: *mut linereq,
    pub irq: c_uint,
    pub edflags: u64,
    pub timestamp_ns: u64,
    pub req_seqno: u32,
    pub line_seqno: u32,
    pub work: delayed_work,
    pub sw_debounced: c_uint,
    pub level: c_uint,
    #[cfg(feature = "CONFIG_HTE")]
    pub hdesc: hte_ts_desc,
    #[cfg(feature = "CONFIG_HTE")]
    pub raw_level: c_int,
    #[cfg(feature = "CONFIG_HTE")]
    pub total_discard_seq: u32,
    #[cfg(feature = "CONFIG_HTE")]
    pub last_seqno: u32,
}

#[repr(C)]
pub struct linereq {
    pub gdev: *mut gpio_device,
    pub label: *const c_char,
    pub num_lines: u32,
    pub wait: wait_queue_head_t,
    pub device_unregistered_nb: notifier_block,
    pub event_buffer_size: u32,
    pub events: kfifo_gpio_v2_line_event,
    pub seqno: atomic_t,
    pub config_mutex: mutex,
    // C flexible array member: lines[num_lines].
    pub lines: [line; 0],
}

extern "C" {
    fn linehandle_validate_flags(flags: u32) -> c_int;
    fn linehandle_flags_to_desc_flags(flags: u32, flagsp: *mut c_ulong);
    fn linehandle_set_config(lh: *mut linehandle_state, ip: *mut c_void) -> isize;
    fn linehandle_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> isize;
    fn linehandle_free(lh: *mut linehandle_state);
    fn linehandle_release(inode: *mut inode, file: *mut file) -> c_int;

    fn linereq_unregistered_notify(nb: *mut notifier_block, action: c_ulong, data: *mut c_void) -> c_int;
    fn linereq_put_event(lr: *mut linereq, event: *mut gpio_v2_line_event);
    fn line_event_timestamp(line: *mut line) -> u64;
    fn line_event_id(level: c_int) -> u32;
    fn debounced_value(line: *mut line) -> bool;
    fn edge_detector_stop(line: *mut line);
    fn edge_detector_setup(line: *mut line, config: *mut gpio_v2_line_config, index: c_uint, flags: u64) -> c_int;
    fn linereq_get_values(lr: *mut linereq, ip: *mut c_void) -> isize;
    fn linereq_set_values(lr: *mut linereq, ip: *mut c_void) -> isize;
    fn linereq_set_config(lr: *mut linereq, ip: *mut c_void) -> isize;
    fn linereq_free(lr: *mut linereq);
    fn gpio_desc_to_lineinfo(desc: *mut gpio_desc, info: *mut gpio_v2_line_info, atomic: bool);
}

// External kernel declarations referenced by the translated implementation.
#[repr(C)] pub struct gpio_device { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct hte_ts_desc { _private: [u8; 0] }
#[repr(C)] pub struct gpio_v2_line_event { _private: [u8; 0] }
#[repr(C)] pub struct gpio_v2_line_config { _private: [u8; 0] }
#[repr(C)] pub struct gpio_v2_line_info { _private: [u8; 0] }
#[repr(C)] pub struct kfifo_gpio_v2_line_event { _private: [u8; 0] }

// Values supplied by the translated kernel uAPI headers.
extern "Rust" {
    static GPIOHANDLES_MAX: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
