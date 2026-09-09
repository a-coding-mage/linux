// SPDX-License-Identifier: GPL-2.0
//
// Register map access API - I2C support
//
// Copyright 2011 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>

// The declarations below are supplied by the corresponding kernel headers.
use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;

type SizeT = usize;
type U8 = u8;
type U16 = u16;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { pub dev: device, pub adapter: *mut i2c_adapter, pub addr: U16 }
#[repr(C)] pub struct i2c_adapter { pub quirks: *const i2c_adapter_quirks }
#[repr(C)] pub struct i2c_adapter_quirks { pub max_read_len: u16, pub max_write_len: u16 }
#[repr(C)] pub struct i2c_msg { pub addr: U16, pub flags: U16, pub len: u16, pub buf: *mut U8 }
#[repr(C)] pub struct regmap_config { pub val_bits: u32, pub reg_bits: u32, pub pad_bits: u32 }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct regmap_bus {
    pub write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, SizeT) -> c_int>,
    pub gather_write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, SizeT, *const c_void, SizeT) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut c_void, *const c_void, SizeT, *mut c_void, SizeT) -> c_int>,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, u32, u32) -> c_int>,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, u32, *mut u32) -> c_int>,
    pub reg_format_endian_default: u32,
    pub val_format_endian_default: u32,
    pub max_raw_read: u16,
    pub max_raw_write: u16,
    pub free_on_exit: bool,
}

extern "C" {
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn i2c_smbus_read_byte_data(i2c: *mut i2c_client, reg: u32) -> c_int;
    fn i2c_smbus_write_byte_data(i2c: *mut i2c_client, reg: u32, val: u32) -> c_int;
    fn i2c_smbus_read_word_data(i2c: *mut i2c_client, reg: u32) -> c_int;
    fn i2c_smbus_write_word_data(i2c: *mut i2c_client, reg: u32, val: u32) -> c_int;
    fn i2c_smbus_read_word_swapped(i2c: *mut i2c_client, reg: u32) -> c_int;
    fn i2c_smbus_write_word_swapped(i2c: *mut i2c_client, reg: u32, val: u32) -> c_int;
    fn i2c_master_send(i2c: *mut i2c_client, data: *const c_void, count: SizeT) -> c_int;
    fn i2c_check_functionality(adapter: *mut i2c_adapter, functionality: u32) -> bool;
    fn i2c_transfer(adapter: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int;
    fn i2c_smbus_write_i2c_block_data(i2c: *mut i2c_client, command: U8, count: SizeT, values: *const U8) -> c_int;
    fn i2c_smbus_read_i2c_block_data(i2c: *mut i2c_client, command: U8, count: SizeT, values: *mut c_void) -> c_int;
    fn i2c_smbus_read_byte(i2c: *mut i2c_client) -> c_int;
    fn regmap_get_val_endian(dev: *mut device, reg: *const c_void, config: *const regmap_config) -> u32;
    fn kmemdup(src: *const c_void, size: SizeT, flags: u32) -> *mut regmap_bus;
    fn __regmap_init(dev: *mut device, bus: *const regmap_bus, context: *mut device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const c_char) -> *mut regmap;
    fn __devm_regmap_init(dev: *mut device, bus: *const regmap_bus, context: *mut device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const c_char) -> *mut regmap;
}

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;
const I2C_FUNC_I2C: u32 = 1;
const I2C_FUNC_NOSTART: u32 = 1 << 2;
const I2C_FUNC_SMBUS_I2C_BLOCK: u32 = 1 << 20;
const I2C_FUNC_SMBUS_BYTE_DATA: u32 = 1 << 3;
const I2C_FUNC_SMBUS_WORD_DATA: u32 = 1 << 4;
const I2C_M_NOSTART: U16 = 0x4000;
const I2C_M_RD: U16 = 1;
const I2C_SMBUS_BLOCK_MAX: u16 = 32;
const REGMAP_ENDIAN_LITTLE: u32 = 1;
const REGMAP_ENDIAN_BIG: u32 = 2;
const GFP_KERNEL: u32 = 0;

#[inline] unsafe fn err_ptr(e: c_int) -> *mut regmap { e as isize as *mut regmap }
#[inline] unsafe fn err_cast<T>(p: *const regmap_bus) -> *mut regmap { p as *mut regmap }

unsafe extern "C" fn regmap_smbus_byte_reg_read(context: *mut c_void, reg: u32, val: *mut u32) -> c_int {
    let i2c = to_i2c_client(context as *mut device); if reg > 0xff { return -EINVAL; }
    let ret = i2c_smbus_read_byte_data(i2c, reg); if ret < 0 { return ret; } *val = ret as u32; 0
}
unsafe extern "C" fn regmap_smbus_byte_reg_write(context: *mut c_void, reg: u32, val: u32) -> c_int {
    let i2c = to_i2c_client(context as *mut device); if val > 0xff || reg > 0xff { return -EINVAL; } i2c_smbus_write_byte_data(i2c, reg, val)
}
static mut REGMAP_SMBUS_BYTE: regmap_bus = regmap_bus { write: None, gather_write: None, read: None, reg_write: Some(regmap_smbus_byte_reg_write), reg_read: Some(regmap_smbus_byte_reg_read), reg_format_endian_default: 0, val_format_endian_default: 0, max_raw_read: 0, max_raw_write: 0, free_on_exit: false };

unsafe extern "C" fn regmap_smbus_word_reg_read(context: *mut c_void, reg: u32, val: *mut u32) -> c_int { let i2c = to_i2c_client(context as *mut device); if reg > 0xff { return -EINVAL; } let ret = i2c_smbus_read_word_data(i2c, reg); if ret < 0 { return ret; } *val = ret as u32; 0 }
unsafe extern "C" fn regmap_smbus_word_reg_write(context: *mut c_void, reg: u32, val: u32) -> c_int { let i2c = to_i2c_client(context as *mut device); if val > 0xffff || reg > 0xff { return -EINVAL; } i2c_smbus_write_word_data(i2c, reg, val) }
unsafe extern "C" fn regmap_smbus_word_read_swapped(context: *mut c_void, reg: u32, val: *mut u32) -> c_int { let i2c = to_i2c_client(context as *mut device); if reg > 0xff { return -EINVAL; } let ret = i2c_smbus_read_word_swapped(i2c, reg); if ret < 0 { return ret; } *val = ret as u32; 0 }
unsafe extern "C" fn regmap_smbus_word_write_swapped(context: *mut c_void, reg: u32, val: u32) -> c_int { let i2c = to_i2c_client(context as *mut device); if val > 0xffff || reg > 0xff { return -EINVAL; } i2c_smbus_write_word_swapped(i2c, reg, val) }
static mut REGMAP_SMBUS_WORD: regmap_bus = regmap_bus { write: None, gather_write: None, read: None, reg_write: Some(regmap_smbus_word_reg_write), reg_read: Some(regmap_smbus_word_reg_read), reg_format_endian_default: 0, val_format_endian_default: 0, max_raw_read: 0, max_raw_write: 0, free_on_exit: false };
static mut REGMAP_SMBUS_WORD_SWAPPED: regmap_bus = regmap_bus { write: None, gather_write: None, read: None, reg_write: Some(regmap_smbus_word_write_swapped), reg_read: Some(regmap_smbus_word_read_swapped), reg_format_endian_default: 0, val_format_endian_default: 0, max_raw_read: 0, max_raw_write: 0, free_on_exit: false };

unsafe extern "C" fn regmap_i2c_write(context: *mut c_void, data: *const c_void, count: SizeT) -> c_int { let i2c = to_i2c_client(context as *mut device); let ret = i2c_master_send(i2c, data, count); if ret == count as c_int { 0 } else if ret < 0 { ret } else { -EIO } }
unsafe extern "C" fn regmap_i2c_gather_write(context: *mut c_void, reg: *const c_void, reg_size: SizeT, val: *const c_void, val_size: SizeT) -> c_int { let i2c = to_i2c_client(context as *mut device); if !i2c_check_functionality((*(*i2c).adapter), I2C_FUNC_NOSTART) { return -ENOTSUPP; } let mut xfer = [i2c_msg { addr: (*i2c).addr, flags: 0, len: reg_size as u16, buf: reg as *mut U8 }, i2c_msg { addr: (*i2c).addr, flags: I2C_M_NOSTART, len: val_size as u16, buf: val as *mut U8 }]; let ret = i2c_transfer((*i2c).adapter, xfer.as_mut_ptr(), 2); if ret == 2 { 0 } else if ret < 0 { ret } else { -EIO } }
unsafe extern "C" fn regmap_i2c_read(context: *mut c_void, reg: *const c_void, reg_size: SizeT, val: *mut c_void, val_size: SizeT) -> c_int { let i2c = to_i2c_client(context as *mut device); let mut xfer = [i2c_msg { addr: (*i2c).addr, flags: 0, len: reg_size as u16, buf: reg as *mut U8 }, i2c_msg { addr: (*i2c).addr, flags: I2C_M_RD, len: val_size as u16, buf: val as *mut U8 }]; let ret = i2c_transfer((*i2c).adapter, xfer.as_mut_ptr(), 2); if ret == 2 { 0 } else if ret < 0 { ret } else { -EIO } }
static mut REGMAP_I2C: regmap_bus = regmap_bus { write: Some(regmap_i2c_write), gather_write: Some(regmap_i2c_gather_write), read: Some(regmap_i2c_read), reg_write: None, reg_read: None, reg_format_endian_default: REGMAP_ENDIAN_BIG, val_format_endian_default: REGMAP_ENDIAN_BIG, max_raw_read: 0, max_raw_write: 0, free_on_exit: false };

unsafe extern "C" fn regmap_i2c_smbus_i2c_write(context: *mut c_void, data: *const c_void, mut count: SizeT) -> c_int { let i2c = to_i2c_client(context as *mut device); if count < 1 { return -EINVAL; } count -= 1; i2c_smbus_write_i2c_block_data(i2c, *(data as *const U8), count, (data as *const U8).add(1)) }
unsafe extern "C" fn regmap_i2c_smbus_i2c_read(context: *mut c_void, reg: *const c_void, reg_size: SizeT, val: *mut c_void, val_size: SizeT) -> c_int { let i2c = to_i2c_client(context as *mut device); if reg_size != 1 || val_size < 1 { return -EINVAL; } let ret = i2c_smbus_read_i2c_block_data(i2c, *(reg as *const U8), val_size, val); if ret == val_size as c_int { 0 } else if ret < 0 { ret } else { -EIO } }
static mut REGMAP_I2C_SMBUS_I2C_BLOCK: regmap_bus = regmap_bus { write: Some(regmap_i2c_smbus_i2c_write), gather_write: None, read: Some(regmap_i2c_smbus_i2c_read), reg_write: None, reg_read: None, reg_format_endian_default: 0, val_format_endian_default: 0, max_raw_read: I2C_SMBUS_BLOCK_MAX - 1, max_raw_write: I2C_SMBUS_BLOCK_MAX - 1, free_on_exit: false };

unsafe extern "C" fn regmap_i2c_smbus_i2c_write_reg16(context: *mut c_void, data: *const c_void, mut count: SizeT) -> c_int { let i2c = to_i2c_client(context as *mut device); if count < 2 { return -EINVAL; } count -= 1; i2c_smbus_write_i2c_block_data(i2c, *(data as *const U8), count, (data as *const U8).add(1)) }
unsafe extern "C" fn regmap_i2c_smbus_i2c_read_reg16(context: *mut c_void, reg: *const c_void, reg_size: SizeT, val: *mut c_void, val_size: SizeT) -> c_int { let i2c = to_i2c_client(context as *mut device); if reg_size != 2 { return -EINVAL; } let r = *(reg as *const U16); let mut ret = i2c_smbus_write_byte_data(i2c, (r & 0xff) as u32, (r >> 8) as u32); if ret < 0 { return ret; } let mut count = 0; let mut p = val as *mut U8; while count < val_size { ret = i2c_smbus_read_byte(i2c); if ret < 0 { break; } *p = ret as U8; p = p.add(1); count += 1; } if count == val_size { 0 } else if ret < 0 { ret } else { -EIO } }
static mut REGMAP_I2C_SMBUS_I2C_BLOCK_REG16: regmap_bus = regmap_bus { write: Some(regmap_i2c_smbus_i2c_write_reg16), gather_write: None, read: Some(regmap_i2c_smbus_i2c_read_reg16), reg_write: None, reg_read: None, reg_format_endian_default: 0, val_format_endian_default: 0, max_raw_read: I2C_SMBUS_BLOCK_MAX - 2, max_raw_write: I2C_SMBUS_BLOCK_MAX - 2, free_on_exit: false };

/* SMBus byte/word reg16 support for adapters lacking I2C and I2C-block support. */
unsafe extern "C" fn regmap_smbus_word_write_reg16(context: *mut c_void, data: *const c_void, count: SizeT) -> c_int { let i2c = to_i2c_client(context as *mut device); if count != 3 { return -EINVAL; } let p = data as *const U8; i2c_smbus_write_word_data(i2c, *p as u32, ((*p.add(2) as U16) << 8 | *p.add(1) as U16) as u32) }
static mut REGMAP_SMBUS_BYTE_WORD_REG16: regmap_bus = regmap_bus { write: Some(regmap_smbus_word_write_reg16), gather_write: None, read: Some(regmap_i2c_smbus_i2c_read_reg16), reg_write: None, reg_read: None, reg_format_endian_default: 0, val_format_endian_default: 0, max_raw_read: I2C_SMBUS_BLOCK_MAX - 2, max_raw_write: 1, free_on_exit: false };

unsafe fn regmap_get_i2c_bus(i2c: *mut i2c_client, config: *const regmap_config) -> *const regmap_bus {
    let mut bus: *const regmap_bus = core::ptr::null(); let mut max_read = 0u16; let mut max_write = 0u16; let a = (*i2c).adapter;
    if i2c_check_functionality(a, I2C_FUNC_I2C) { bus = &REGMAP_I2C; }
    else if (*config).val_bits == 8 && (*config).reg_bits == 8 && i2c_check_functionality(a, I2C_FUNC_SMBUS_I2C_BLOCK) { bus = &REGMAP_I2C_SMBUS_I2C_BLOCK; }
    else if (*config).val_bits == 8 && (*config).reg_bits == 16 && i2c_check_functionality(a, I2C_FUNC_SMBUS_I2C_BLOCK) { bus = &REGMAP_I2C_SMBUS_I2C_BLOCK_REG16; }
    else if (*config).val_bits == 8 && (*config).reg_bits == 16 && i2c_check_functionality(a, I2C_FUNC_SMBUS_BYTE_DATA | I2C_FUNC_SMBUS_WORD_DATA) { bus = &REGMAP_SMBUS_BYTE_WORD_REG16; }
    else if (*config).val_bits == 16 && (*config).reg_bits == 8 && i2c_check_functionality(a, I2C_FUNC_SMBUS_WORD_DATA) { let e = regmap_get_val_endian(&mut (*i2c).dev, core::ptr::null(), config); if e == REGMAP_ENDIAN_LITTLE { bus = &REGMAP_SMBUS_WORD; } else if e == REGMAP_ENDIAN_BIG { bus = &REGMAP_SMBUS_WORD_SWAPPED; } }
    else if (*config).val_bits == 8 && (*config).reg_bits == 8 && i2c_check_functionality(a, I2C_FUNC_SMBUS_BYTE_DATA) { bus = &REGMAP_SMBUS_BYTE; }
    if bus.is_null() { return err_ptr(-ENOTSUPP) as *const regmap_bus; }
    let q = (*a).quirks; if !q.is_null() { if (*q).max_read_len != 0 && ((*bus).max_raw_read == 0 || (*bus).max_raw_read > (*q).max_read_len) { max_read = (*q).max_read_len; } if (*q).max_write_len != 0 && ((*bus).max_raw_write == 0 || (*bus).max_raw_write > (*q).max_write_len) { max_write = (*q).max_write_len - ((*config).reg_bits + (*config).pad_bits) as u16 / 8; } if max_read != 0 || max_write != 0 { let r = kmemdup(bus as *const c_void, size_of::<regmap_bus>(), GFP_KERNEL); if r.is_null() { return err_ptr(-ENOMEM) as *const regmap_bus; } (*r).free_on_exit = true; (*r).max_raw_read = max_read; (*r).max_raw_write = max_write; bus = r; } } bus
}

#[no_mangle] pub unsafe extern "C" fn __regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const c_char) -> *mut regmap { let bus = regmap_get_i2c_bus(i2c, config); if (bus as *const regmap as isize) < 0 { return bus as *mut regmap; } __regmap_init(&mut (*i2c).dev, bus, &mut (*i2c).dev, config, lock_key, lock_name) }
#[no_mangle] pub unsafe extern "C" fn __devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const c_char) -> *mut regmap { let bus = regmap_get_i2c_bus(i2c, config); if (bus as *const regmap as isize) < 0 { return bus as *mut regmap; } __devm_regmap_init(&mut (*i2c).dev, bus, &mut (*i2c).dev, config, lock_key, lock_name) }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
