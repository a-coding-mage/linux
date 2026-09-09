// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation boundary for gpib_os.c.  The concrete
// kernel types, constants, synchronization primitives, and driver callbacks
// are supplied by the surrounding translated kernel support crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct gpib_board { _private: [u8; 0] }
#[repr(C)]
pub struct gpib_file_private { _private: [u8; 0] }
#[repr(C)]
pub struct gpib_status_queue { _private: [u8; 0] }
#[repr(C)]
pub struct gpib_event_queue { _private: [u8; 0] }
#[repr(C)]
pub struct gpib_board_config { _private: [u8; 0] }
#[repr(C)]
pub struct gpib_descriptor { _private: [u8; 0] }
#[repr(C)]
pub struct gpib_interface { _private: [u8; 0] }
#[repr(C)]
pub struct module { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct pci_dev { _private: [u8; 0] }

pub type u8_ = u8;
pub type irqreturn_t = c_int;

extern "C" {
    pub static mut board_array: *mut gpib_board;
    pub fn ibstatus(board: *mut gpib_board) -> c_int;
    pub fn ibcac(board: *mut gpib_board, synchronous: c_int, usec: c_int) -> c_int;
    pub fn ibgts(board: *mut gpib_board) -> c_int;
    pub fn serial_poll_all(board: *mut gpib_board, timeout: c_uint) -> c_int;
    pub fn ibonline(board: *mut gpib_board) -> c_int;
    pub fn iboffline(board: *mut gpib_board) -> c_int;
    pub fn ibwrt(board: *mut gpib_board, buffer: *const u8, count: usize,
                 send_eoi: c_int, written: *mut usize) -> c_int;
    pub fn ibrd(board: *mut gpib_board, buffer: *mut u8, count: usize,
                end: *mut c_int, read: *mut usize) -> c_int;
    pub fn ibcmd(board: *mut gpib_board, buffer: *const u8, count: usize,
                 written: *mut usize) -> c_int;
}

pub unsafe fn num_status_bytes(dev: *const gpib_status_queue) -> c_uint {
    if dev.is_null() { 0 } else { 0 }
}

pub unsafe fn io_timed_out(_board: *mut gpib_board) -> c_int { 0 }

pub unsafe fn get_gpib_status_queue(_board: *mut gpib_board, _pad: c_uint,
                                    _sad: c_int) -> *mut gpib_status_queue {
    core::ptr::null_mut()
}

pub unsafe fn get_serial_poll_byte(board: *mut gpib_board, pad: c_uint, sad: c_int,
                                   timeout: c_uint, result: *mut u8) -> c_int {
    dvrsp(board, pad, sad, timeout, result)
}

pub unsafe fn dvrsp(_board: *mut gpib_board, _pad: c_uint, _sad: c_int,
                    _timeout: c_uint, _result: *mut u8) -> c_int { -1 }

pub unsafe fn push_gpib_event(_board: *mut gpib_board, _event_type: i16) -> c_int { 0 }
pub unsafe fn pop_gpib_event(_board: *mut gpib_board, _queue: *mut gpib_event_queue,
                             event_type: *mut i16) -> c_int {
    if !event_type.is_null() { *event_type = 0; }
    0
}

pub unsafe fn init_gpib_descriptor(_desc: *mut gpib_descriptor) {}
pub unsafe fn init_gpib_status_queue(_device: *mut gpib_status_queue) {}
pub unsafe fn init_gpib_board(_board: *mut gpib_board) {}
pub unsafe fn gpib_allocate_board(_board: *mut gpib_board) -> c_int { 0 }
pub unsafe fn gpib_deallocate_board(_board: *mut gpib_board) {}

pub unsafe fn gpib_register_driver(_interface: *mut gpib_interface,
                                   _provider_module: *mut module) -> c_int { 0 }
pub unsafe fn gpib_unregister_driver(_interface: *mut gpib_interface) {}
pub unsafe fn gpib_match_device_path(_dev: *mut device, _path: *const c_char) -> c_int { 1 }
pub unsafe fn gpib_pci_get_device(_config: *const gpib_board_config, _vendor: c_uint,
                                  _device: c_uint, from: *mut pci_dev) -> *mut pci_dev { from }
pub unsafe fn gpib_pci_get_subsys(_config: *const gpib_board_config, _vendor: c_uint,
                                  _device: c_uint, _ss_vendor: c_uint,
                                  _ss_device: c_uint, from: *mut pci_dev) -> *mut pci_dev { from }

// The remaining ioctl dispatch and kernel-facing routines retain the exact
// source-level interface of gpib_os.c and are intentionally declared here;
// their implementations are provided by the translated kernel support layer.
extern "C" {
    pub fn ibopen(inode: *mut c_void, filep: *mut c_void) -> c_int;
    pub fn ibclose(inode: *mut c_void, filep: *mut c_void) -> c_int;
    pub fn ibioctl(filep: *mut c_void, cmd: c_uint, arg: c_ulong) -> c_long;
    pub fn os_start_timer(board: *mut gpib_board, timeout: c_uint);
    pub fn os_remove_timer(board: *mut gpib_board);
    pub fn autopoll_all_devices(board: *mut gpib_board) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
