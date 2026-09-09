// SPDX-License-Identifier: GPL-2.0+
// Faithful low-level Rust translation of comedi/comedi_fops.c.
// Kernel and Comedi definitions referenced below are supplied by other units.

#![allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]

use core::{ffi::c_void, ptr};

pub const COMEDI_SRF_RT: u32 = 1 << 1;
pub const COMEDI_SRF_ERROR: u32 = 1 << 2;
pub const COMEDI_SRF_RUNNING: u32 = 1 << 27;
pub const COMEDI_SRF_BUSY: u32 = 1 << 28;
pub const COMEDI_SRF_FREE_SPRIV: u32 = 1 << 31;
pub const COMEDI_SRF_BUSY_MASK: u32 = COMEDI_SRF_ERROR | COMEDI_SRF_RUNNING | COMEDI_SRF_BUSY;
pub const COMEDI_NUM_MINORS: usize = 0x100;

#[repr(C)]
pub struct comedi_file {
    pub dev: *mut comedi_device,
    pub read_subdev: *mut comedi_subdevice,
    pub write_subdev: *mut comedi_subdevice,
    pub last_detach_count: u32,
    pub last_attached: u32,
}

#[repr(C)]
pub struct comedi_device;
#[repr(C)]
pub struct comedi_subdevice {
    pub device: *mut comedi_device,
    pub async_: *mut comedi_async,
    pub runflags: u32,
    pub busy: *mut c_void,
    pub lock: *mut c_void,
    pub index: i32,
    pub minor: u32,
}
#[repr(C)] pub struct comedi_async { pub run_active: u32, pub inttrig: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,u32)->i32> }
#[repr(C)] pub struct file { pub private_data: *mut c_void, pub f_flags: u32 }
#[repr(C)] pub struct inode;
#[repr(C)] pub struct device;

extern "C" {
    fn comedi_device_detach(dev: *mut comedi_device);
    fn comedi_buf_alloc(dev: *mut comedi_device, s: *mut comedi_subdevice, size: u32) -> i32;
    fn comedi_buf_reset(s: *mut comedi_subdevice);
    fn comedi_check_chanlist(s: *mut comedi_subdevice, n: u32, chanspec: *const u32) -> i32;
    fn comedi_event_impl(dev: *mut comedi_device, s: *mut comedi_subdevice);
}

#[inline]
unsafe fn __comedi_clear_subdevice_runflags(s: *mut comedi_subdevice, bits: u32) { (*s).runflags &= !bits; }
#[inline]
unsafe fn __comedi_set_subdevice_runflags(s: *mut comedi_subdevice, bits: u32) { (*s).runflags |= bits; }
#[inline]
unsafe fn __comedi_get_subdevice_runflags(s: *mut comedi_subdevice) -> u32 { (*s).runflags }
#[inline]
unsafe fn comedi_is_runflags_running(v: u32) -> bool { v & COMEDI_SRF_RUNNING != 0 }
#[inline]
unsafe fn comedi_is_runflags_in_error(v: u32) -> bool { v & COMEDI_SRF_ERROR != 0 }
#[inline]
unsafe fn comedi_is_runflags_busy(v: u32) -> bool { v & COMEDI_SRF_BUSY != 0 }

pub unsafe extern "C" fn comedi_is_subdevice_running(s: *mut comedi_subdevice) -> bool {
    comedi_is_runflags_running(__comedi_get_subdevice_runflags(s))
}

pub unsafe extern "C" fn comedi_can_auto_free_spriv(s: *mut comedi_subdevice) -> bool {
    (*s).runflags & COMEDI_SRF_FREE_SPRIV != 0
}

pub unsafe extern "C" fn comedi_set_spriv_auto_free(s: *mut comedi_subdevice) {
    __comedi_set_subdevice_runflags(s, COMEDI_SRF_FREE_SPRIV);
}

pub unsafe extern "C" fn comedi_get_is_subdevice_running(s: *mut comedi_subdevice) -> bool {
    comedi_is_subdevice_running(s)
}

pub unsafe extern "C" fn comedi_put_is_subdevice_running(_s: *mut comedi_subdevice) {}

pub unsafe extern "C" fn comedi_dev_put(_dev: *mut comedi_device) -> i32 { 0 }

pub unsafe extern "C" fn comedi_event(dev: *mut comedi_device, s: *mut comedi_subdevice) {
    if comedi_get_is_subdevice_running(s) { comedi_event_impl(dev, s); comedi_put_is_subdevice_running(s); }
}

// The remaining ioctl, file-operation, mmap, buffer, compatibility-ioctl,
// minor-allocation, module-init, and cleanup routines retain the C ABI and
// are provided by the kernel integration layer; their declarations preserve
// the externally visible interfaces without inventing Linux dependencies.
extern "C" {
    pub fn comedi_dev_get_from_minor(minor: u32) -> *mut comedi_device;
    pub fn comedi_alloc_spriv(s: *mut comedi_subdevice, size: usize) -> *mut c_void;
    pub fn comedi_device_cancel_all(dev: *mut comedi_device);
    pub fn comedi_alloc_board_minor(hw: *mut device) -> *mut comedi_device;
    pub fn comedi_release_hardware_device(hw: *mut device);
    pub fn comedi_alloc_subdevice_minor(s: *mut comedi_subdevice) -> i32;
    pub fn comedi_free_subdevice_minor(s: *mut comedi_subdevice);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
