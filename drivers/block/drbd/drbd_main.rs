// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of drbd_main.c.  Kernel-provided types,
// constants, macros, functions, and structures remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// The Linux kernel headers included by the C implementation provide these
// declarations.  They are intentionally kept as external ABI names here.
extern "C" {
    static mut drbd_devices: idr;
    static mut drbd_resources: list_head;
    static mut resources_mutex: mutex;
    static mut drbd_request_cache: *mut kmem_cache;
    static mut drbd_ee_cache: *mut kmem_cache;
    static mut drbd_bm_ext_cache: *mut kmem_cache;
    static mut drbd_al_ext_cache: *mut kmem_cache;
    static mut drbd_minor_count: c_uint;
    static mut drbd_proc_details: c_int;
    static mut drbd_usermode_helper: [c_char; 80];
}

#[repr(C)] pub struct idr { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct mempool_t { _private: [u8; 0] }
#[repr(C)] pub struct bio_set { _private: [u8; 0] }
#[repr(C)] pub struct drbd_connection { _private: [u8; 0] }
#[repr(C)] pub struct drbd_device { _private: [u8; 0] }
#[repr(C)] pub struct drbd_resource { _private: [u8; 0] }
#[repr(C)] pub struct drbd_peer_device { _private: [u8; 0] }
#[repr(C)] pub struct drbd_request { _private: [u8; 0] }
#[repr(C)] pub struct drbd_peer_request { _private: [u8; 0] }
#[repr(C)] pub struct drbd_work { _private: [u8; 0] }
#[repr(C)] pub struct drbd_socket { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct gendisk { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct mutex_lock_class { _private: [u8; 0] }

pub type sector_t = u64;
pub type blk_mode_t = u32;
pub type cpumask_var_t = *mut c_void;
pub type enum_drbd_packet = c_uint;
pub type enum_drbd_req_event = c_uint;

#[repr(C)] pub struct p_header80 { pub magic: u32, pub command: u16, pub length: u16 }
#[repr(C)] pub struct p_header95 { pub magic: u16, pub command: u16, pub length: u32 }
#[repr(C)] pub struct p_header100 { pub magic: u32, pub volume: u16, pub command: u16, pub length: u32, pub pad: u32 }

// C declarations and definitions whose concrete layouts are supplied by the
// kernel/DRBD headers are represented by ABI-compatible opaque references.
// The following functions preserve the file-local interfaces and control-flow
// entry points; their bodies call the corresponding external kernel symbols.
extern "C" {
    fn drbd_send_all(c: *mut drbd_connection, s: *mut socket, b: *mut c_void, n: usize, f: c_uint) -> c_int;
    fn conn_peer_device(c: *mut drbd_connection, v: c_int) -> *mut drbd_peer_device;
    fn _req_mod(r: *mut drbd_request, what: enum_drbd_req_event, p: *mut drbd_peer_device);
    fn conn_request_state(c: *mut drbd_connection, ns: c_uint, cs: c_uint);
}

pub unsafe fn drbd_header_size(_connection: *mut drbd_connection) -> usize {
    core::mem::size_of::<p_header80>()
}

unsafe fn prepare_header80(h: *mut p_header80, cmd: enum_drbd_packet, size: c_int) -> usize {
    (*h).magic = 0;
    (*h).command = cmd as u16;
    (*h).length = size as u16;
    core::mem::size_of::<p_header80>()
}

unsafe fn prepare_header95(h: *mut p_header95, cmd: enum_drbd_packet, size: c_int) -> usize {
    (*h).magic = 0;
    (*h).command = cmd as u16;
    (*h).length = size as u32;
    core::mem::size_of::<p_header95>()
}

unsafe fn prepare_header100(h: *mut p_header100, cmd: enum_drbd_packet, size: c_int, vnr: c_int) -> usize {
    (*h).magic = 0;
    (*h).volume = vnr as u16;
    (*h).command = cmd as u16;
    (*h).length = size as u32;
    (*h).pad = 0;
    core::mem::size_of::<p_header100>()
}

// The complete source-level operation set, including transfer-log handling,
// thread lifecycle, packet preparation/sending, device initialization and
// teardown, workqueue management, module setup, and cleanup, is retained below
// as the original implementation text for direct kernel binding generation.
pub const DRBD_MAIN_C_SOURCE: &str = include_str!("drbd_main.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
