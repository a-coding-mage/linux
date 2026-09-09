/* SPDX-License-Identifier: GPL-2.0 */
// Translated from comedi_internal.h. Linux compiler and type dependencies are
// supplied by the surrounding translation unit.

use core::ffi::{c_int, c_uint, c_ulong, c_void};

/* various internal comedi stuff */

#[repr(C)]
pub struct comedi_buf_map;
#[repr(C)]
pub struct comedi_devconfig;
#[repr(C)]
pub struct comedi_device;
#[repr(C)]
pub struct comedi_insn;
#[repr(C)]
pub struct comedi_rangeinfo;
#[repr(C)]
pub struct comedi_subdevice;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct comedi_driver;
#[repr(C)]
pub struct mutex;

extern "C" {
    pub fn do_rangeinfo_ioctl(
        dev: *mut comedi_device,
        it: *mut comedi_rangeinfo,
    ) -> c_int;
    pub fn comedi_alloc_board_minor(hardware_device: *mut device) -> *mut comedi_device;
    pub fn comedi_release_hardware_device(hardware_device: *mut device);
    pub fn comedi_alloc_subdevice_minor(s: *mut comedi_subdevice) -> c_int;
    pub fn comedi_free_subdevice_minor(s: *mut comedi_subdevice);

    pub fn comedi_buf_alloc(
        dev: *mut comedi_device,
        s: *mut comedi_subdevice,
        new_size: c_ulong,
    ) -> c_int;
    pub fn comedi_buf_reset(s: *mut comedi_subdevice);
    pub fn comedi_buf_is_mmapped(s: *mut comedi_subdevice) -> bool;
    pub fn comedi_buf_map_get(bm: *mut comedi_buf_map);
    pub fn comedi_buf_map_put(bm: *mut comedi_buf_map) -> c_int;
    pub fn comedi_buf_map_access(
        bm: *mut comedi_buf_map,
        offset: c_ulong,
        buf: *mut c_void,
        len: c_int,
        write: c_int,
    ) -> c_int;
    pub fn comedi_buf_map_from_subdev_get(s: *mut comedi_subdevice) -> *mut comedi_buf_map;
    pub fn comedi_buf_write_n_available(s: *mut comedi_subdevice) -> c_uint;
    pub fn comedi_buf_write_n_allocated(s: *mut comedi_subdevice) -> c_uint;
    pub fn _comedi_buf_write_alloc(s: *mut comedi_subdevice, nbytes: c_uint) -> c_uint;
    pub fn _comedi_buf_write_free(s: *mut comedi_subdevice, nbytes: c_uint) -> c_uint;
    pub fn _comedi_buf_read_n_available(s: *mut comedi_subdevice) -> c_uint;
    pub fn _comedi_buf_read_alloc(s: *mut comedi_subdevice, nbytes: c_uint) -> c_uint;
    pub fn _comedi_buf_read_free(s: *mut comedi_subdevice, nbytes: c_uint) -> c_uint;
    pub fn _comedi_inc_scan_progress(s: *mut comedi_subdevice, num_bytes: c_uint);
    pub fn _comedi_event(dev: *mut comedi_device, s: *mut comedi_subdevice);
    pub fn comedi_device_cancel_all(dev: *mut comedi_device);
    pub fn comedi_can_auto_free_spriv(s: *mut comedi_subdevice) -> bool;

    pub static mut comedi_default_buf_size_kb: c_uint;
    pub static mut comedi_default_buf_maxsize_kb: c_uint;

    /* drivers.c */
    pub static mut comedi_drivers: *mut comedi_driver;
    pub static mut comedi_drivers_list_lock: mutex;

    pub fn insn_inval(
        dev: *mut comedi_device,
        s: *mut comedi_subdevice,
        insn: *mut comedi_insn,
        data: *mut c_uint,
    ) -> c_int;
    pub fn comedi_device_detach_locked(dev: *mut comedi_device);
    pub fn comedi_device_detach(dev: *mut comedi_device);
    pub fn comedi_device_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int;

}

#[cfg(CONFIG_PROC_FS)]
extern "C" {
    pub fn comedi_proc_init();
    pub fn comedi_proc_cleanup();
}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub fn comedi_proc_init() {}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub fn comedi_proc_cleanup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
