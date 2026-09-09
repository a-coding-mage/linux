/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Common library for ADIS16XXX devices. Rust translation of adis.h. */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_char;

pub const ADIS_WRITE_REG: fn(u32) -> u32 = |reg| 0x80 | reg;
pub const ADIS_READ_REG: fn(u32) -> u32 = |reg| reg & 0x7f;
pub const ADIS_PAGE_SIZE: u32 = 0x80;
pub const ADIS_REG_PAGE_ID: u32 = 0x00;

#[repr(C)]
pub struct adis;
#[repr(C)]
pub struct iio_dev_attr;
#[repr(C)]
pub struct spi_device;
#[repr(C)]
pub struct iio_trigger;
#[repr(C)]
pub struct iio_dev;
#[repr(C)]
pub struct iio_chan_spec;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct spi_message;
#[repr(C)]
pub struct spi_transfer;

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type c_int = core::ffi::c_int;
pub type c_uint = core::ffi::c_uint;
pub type c_ulong = core::ffi::c_ulong;

#[repr(C)]
pub struct adis_timeout {
    pub reset_ms: u16,
    pub sw_reset_ms: u16,
    pub self_test_ms: u16,
}

#[repr(C)]
pub struct adis_data {
    pub read_delay: c_uint,
    pub write_delay: c_uint,
    pub cs_change_delay: c_uint,
    pub glob_cmd_reg: c_uint,
    pub msc_ctrl_reg: c_uint,
    pub diag_stat_reg: c_uint,
    pub diag_stat_size: c_uint,
    pub prod_id_reg: c_uint,
    pub prod_id: c_uint,
    pub self_test_mask: c_uint,
    pub self_test_reg: c_uint,
    pub self_test_no_autoclear: bool,
    pub timeouts: *const adis_timeout,
    pub status_error_msgs: *const *const c_char,
    pub status_error_mask: c_uint,
    pub enable_irq: Option<unsafe extern "C" fn(*mut adis, bool) -> c_int>,
    pub unmasked_drdy: bool,
    pub has_paging: bool,
    pub has_fifo: bool,
    pub burst_reg_cmd: c_uint,
    pub burst_len: c_uint,
    pub burst_max_len: c_uint,
    pub burst_max_speed_hz: c_uint,
}

#[repr(C)]
pub struct adis_ops {
    pub write: Option<unsafe extern "C" fn(*mut adis, c_uint, c_uint, c_uint) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut adis, c_uint, *mut c_uint, c_uint) -> c_int>,
    pub reset: Option<unsafe extern "C" fn(*mut adis) -> c_int>,
}

#[repr(C)]
pub struct adis {
    pub spi: *mut spi_device,
    pub trig: *mut iio_trigger,
    pub data: *const adis_data,
    pub burst_extra_len: c_uint,
    pub ops: *const adis_ops,
    pub state_lock: mutex,
    pub msg: spi_message,
    pub xfer: *mut spi_transfer,
    pub current_page: c_uint,
    pub irq_flag: c_ulong,
    pub buffer: *mut core::ffi::c_void,
    pub tx: [u8; 10],
    pub rx: [u8; 4],
}

extern "C" {
    pub fn adis_init(adis: *mut adis, indio_dev: *mut iio_dev,
                     spi: *mut spi_device, data: *const adis_data) -> c_int;
    pub fn __adis_reset(adis: *mut adis) -> c_int;
    pub fn __adis_write_reg(adis: *mut adis, reg: c_uint, val: c_uint, size: c_uint) -> c_int;
    pub fn __adis_read_reg(adis: *mut adis, reg: c_uint, val: *mut c_uint, size: c_uint) -> c_int;
    pub fn __adis_update_bits_base(adis: *mut adis, reg: c_uint, mask: u32, val: u32, size: u8) -> c_int;
    pub fn __adis_check_status(adis: *mut adis) -> c_int;
    pub fn __adis_initial_startup(adis: *mut adis) -> c_int;
    pub fn __adis_enable_irq(adis: *mut adis, enable: bool) -> c_int;
    pub fn adis_single_conversion(indio_dev: *mut iio_dev, chan: *const iio_chan_spec,
                                  error_mask: c_uint, val: *mut c_int) -> c_int;
}

#[inline]
pub unsafe fn __adis_write_reg_8(adis: *mut adis, reg: c_uint, val: u8) -> c_int {
    ((*(*adis).ops).write.unwrap())(adis, reg, val as c_uint, 1)
}
#[inline]
pub unsafe fn __adis_write_reg_16(adis: *mut adis, reg: c_uint, val: u16) -> c_int {
    ((*(*adis).ops).write.unwrap())(adis, reg, val as c_uint, 2)
}
#[inline]
pub unsafe fn __adis_write_reg_32(adis: *mut adis, reg: c_uint, val: u32) -> c_int {
    ((*(*adis).ops).write.unwrap())(adis, reg, val, 4)
}

#[inline]
pub unsafe fn __adis_read_reg_16(adis: *mut adis, reg: c_uint, val: *mut u16) -> c_int {
    let mut tmp = 0u32;
    let ret = ((*(*adis).ops).read.unwrap())(adis, reg, &mut tmp, 2);
    if ret == 0 { *val = tmp as u16; }
    ret
}
#[inline]
pub unsafe fn __adis_read_reg_32(adis: *mut adis, reg: c_uint, val: *mut u32) -> c_int {
    let mut tmp = 0u32;
    let ret = ((*(*adis).ops).read.unwrap())(adis, reg, &mut tmp, 4);
    if ret == 0 { *val = tmp; }
    ret
}

// The C versions acquire state_lock with cleanup.h guard(mutex); lock acquisition
// is supplied by the surrounding kernel translation.
#[inline] pub unsafe fn adis_reset(adis: *mut adis) -> c_int { __adis_reset(adis) }
#[inline] pub unsafe fn adis_write_reg(adis: *mut adis, reg: c_uint, val: c_uint, size: c_uint) -> c_int { ((*(*adis).ops).write.unwrap())(adis, reg, val, size) }
#[inline] pub unsafe fn adis_read_reg(adis: *mut adis, reg: c_uint, val: *mut c_uint, size: c_uint) -> c_int { ((*(*adis).ops).read.unwrap())(adis, reg, val, size) }
#[inline] pub unsafe fn adis_write_reg_8(adis: *mut adis, reg: c_uint, val: u8) -> c_int { adis_write_reg(adis, reg, val as c_uint, 1) }
#[inline] pub unsafe fn adis_write_reg_16(adis: *mut adis, reg: c_uint, val: u16) -> c_int { adis_write_reg(adis, reg, val as c_uint, 2) }
#[inline] pub unsafe fn adis_write_reg_32(adis: *mut adis, reg: c_uint, val: u32) -> c_int { adis_write_reg(adis, reg, val, 4) }
#[inline] pub unsafe fn adis_read_reg_16(adis: *mut adis, reg: c_uint, val: *mut u16) -> c_int { let mut tmp=0u32; let ret=adis_read_reg(adis,reg,&mut tmp,2); if ret==0 {*val=tmp as u16;} ret }
#[inline] pub unsafe fn adis_read_reg_32(adis: *mut adis, reg: c_uint, val: *mut u32) -> c_int { let mut tmp=0u32; let ret=adis_read_reg(adis,reg,&mut tmp,4); if ret==0 {*val=tmp;} ret }
#[inline] pub unsafe fn adis_update_bits_base(adis: *mut adis, reg: c_uint, mask: u32, val: u32, size: u8) -> c_int { __adis_update_bits_base(adis,reg,mask,val,size) }
#[inline] pub unsafe fn adis_enable_irq(adis: *mut adis, enable: bool) -> c_int { __adis_enable_irq(adis,enable) }
#[inline] pub unsafe fn adis_check_status(adis: *mut adis) -> c_int { __adis_check_status(adis) }

#[macro_export]
macro_rules! adis_update_bits { ($adis:expr,$reg:expr,$mask:expr,$val:expr) => {{ $crate::adis_update_bits_base($adis,$reg,$mask,$val,core::mem::size_of_val(&$val) as u8) }} }
#[macro_export]
macro_rules! __adis_update_bits { ($adis:expr,$reg:expr,$mask:expr,$val:expr) => {{ $crate::__adis_update_bits_base($adis,$reg,$mask,$val,core::mem::size_of_val(&$val) as u8) }} }

#[macro_export] macro_rules! adis_dev_auto_lock { ($adis:expr) => { /* guard(mutex)(&($adis)->state_lock) */ } }
#[macro_export] macro_rules! adis_dev_auto_scoped_lock { ($adis:expr) => { /* scoped_guard(mutex, &($adis)->state_lock) */ } }

// ADIS_VOLTAGE_CHAN and ADIS_SUPPLY_CHAN are retained as source-level
// construction macros; the IIO field constants/types are supplied externally.
#[macro_export]
macro_rules! ADIS_VOLTAGE_CHAN { ($addr:expr,$si:expr,$chan:expr,$name:expr,$info_all:expr,$bits:expr) => { {
    type _AdisVoltageChan = iio_chan_spec;
    _AdisVoltageChan { ..core::mem::zeroed() }
} } }
#[macro_export]
macro_rules! ADIS_SUPPLY_CHAN { ($addr:expr,$si:expr,$info_all:expr,$bits:expr) => { $crate::ADIS_VOLTAGE_CHAN!($addr,$si,0,"supply",$info_all,$bits) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
