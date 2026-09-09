// SPDX-License-Identifier: GPL-2.0
//
// Register map access API - FSI support
//
// Copyright 2022 IBM Corp
//
// Author: Eddie James <eajames@linux.ibm.com>

// Dependencies supplied by the surrounding kernel/Rust environment.

unsafe fn regmap_fsi32_reg_read(context: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32 {
    let mut v: u32 = 0;
    let ret = unsafe {
        fsi_slave_read(
            context,
            reg,
            &mut v as *mut u32 as *mut core::ffi::c_void,
            core::mem::size_of::<u32>(),
        )
    };
    if ret != 0 {
        return ret;
    }
    unsafe { *val = v; }
    0
}

unsafe fn regmap_fsi32_reg_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    let mut v = val;
    unsafe {
        fsi_slave_write(
            context,
            reg,
            &mut v as *mut u32 as *mut core::ffi::c_void,
            core::mem::size_of::<u32>(),
        )
    }
}

static REGMAP_FSI32: RegmapBus = RegmapBus {
    reg_write: Some(regmap_fsi32_reg_write),
    reg_read: Some(regmap_fsi32_reg_read),
};

unsafe fn regmap_fsi32le_reg_read(context: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32 {
    let mut v: u32 = 0;
    let ret = unsafe {
        fsi_slave_read(
            context,
            reg,
            &mut v as *mut u32 as *mut core::ffi::c_void,
            core::mem::size_of::<u32>(),
        )
    };
    if ret != 0 {
        return ret;
    }
    unsafe { *val = u32::from_be(v); }
    0
}

unsafe fn regmap_fsi32le_reg_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    let mut v = val.to_be();
    unsafe {
        fsi_slave_write(
            context,
            reg,
            &mut v as *mut u32 as *mut core::ffi::c_void,
            core::mem::size_of::<u32>(),
        )
    }
}

static REGMAP_FSI32LE: RegmapBus = RegmapBus {
    reg_write: Some(regmap_fsi32le_reg_write),
    reg_read: Some(regmap_fsi32le_reg_read),
};

unsafe fn regmap_fsi16_reg_read(context: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32 {
    let mut v: u16 = 0;
    let ret = unsafe {
        fsi_slave_read(
            context,
            reg,
            &mut v as *mut u16 as *mut core::ffi::c_void,
            core::mem::size_of::<u16>(),
        )
    };
    if ret != 0 { return ret; }
    unsafe { *val = v as u32; }
    0
}

unsafe fn regmap_fsi16_reg_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    if val > 0xffff { return -22; }
    let mut v = val as u16;
    unsafe { fsi_slave_write(context, reg, &mut v as *mut u16 as *mut core::ffi::c_void, core::mem::size_of::<u16>()) }
}

static REGMAP_FSI16: RegmapBus = RegmapBus { reg_write: Some(regmap_fsi16_reg_write), reg_read: Some(regmap_fsi16_reg_read) };

unsafe fn regmap_fsi16le_reg_read(context: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32 {
    let mut v: u16 = 0;
    let ret = unsafe { fsi_slave_read(context, reg, &mut v as *mut u16 as *mut core::ffi::c_void, core::mem::size_of::<u16>()) };
    if ret != 0 { return ret; }
    unsafe { *val = u16::from_be(v) as u32; }
    0
}

unsafe fn regmap_fsi16le_reg_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    if val > 0xffff { return -22; }
    let mut v = (val as u16).to_be();
    unsafe { fsi_slave_write(context, reg, &mut v as *mut u16 as *mut core::ffi::c_void, core::mem::size_of::<u16>()) }
}

static REGMAP_FSI16LE: RegmapBus = RegmapBus { reg_write: Some(regmap_fsi16le_reg_write), reg_read: Some(regmap_fsi16le_reg_read) };

unsafe fn regmap_fsi8_reg_read(context: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32 {
    let mut v: u8 = 0;
    let ret = unsafe { fsi_slave_read(context, reg, &mut v as *mut u8 as *mut core::ffi::c_void, core::mem::size_of::<u8>()) };
    if ret != 0 { return ret; }
    unsafe { *val = v as u32; }
    0
}

unsafe fn regmap_fsi8_reg_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    if val > 0xff { return -22; }
    let mut v = val as u8;
    unsafe { fsi_slave_write(context, reg, &mut v as *mut u8 as *mut core::ffi::c_void, core::mem::size_of::<u8>()) }
}

static REGMAP_FSI8: RegmapBus = RegmapBus { reg_write: Some(regmap_fsi8_reg_write), reg_read: Some(regmap_fsi8_reg_read) };

unsafe fn regmap_get_fsi_bus(fsi_dev: *mut FsiDevice, config: *const RegmapConfig) -> *const RegmapBus {
    let mut bus: *const RegmapBus = core::ptr::null();
    let reg_bits = unsafe { (*config).reg_bits };
    let val_bits = unsafe { (*config).val_bits };
    if reg_bits == 8 || reg_bits == 16 || reg_bits == 32 {
        match val_bits {
            8 => bus = &REGMAP_FSI8,
            16 => match unsafe { regmap_get_val_endian(&mut (*fsi_dev).dev, core::ptr::null(), config) } {
                RegmapEndian::Little => bus = &REGMAP_FSI16LE,
                RegmapEndian::Native => bus = &REGMAP_FSI16LE,
                RegmapEndian::Default | RegmapEndian::Big => bus = &REGMAP_FSI16,
                _ => {}
            },
            32 => match unsafe { regmap_get_val_endian(&mut (*fsi_dev).dev, core::ptr::null(), config) } {
                RegmapEndian::Little => bus = &REGMAP_FSI32LE,
                RegmapEndian::Native => bus = &REGMAP_FSI32LE,
                RegmapEndian::Default | RegmapEndian::Big => bus = &REGMAP_FSI32,
                _ => {}
            },
            _ => {}
        }
    }
    if bus.is_null() { err_ptr(-95) } else { bus }
}

pub unsafe fn __regmap_init_fsi(fsi_dev: *mut FsiDevice, config: *const RegmapConfig, lock_key: *mut LockClassKey, lock_name: *const core::ffi::c_char) -> *mut Regmap {
    let bus = unsafe { regmap_get_fsi_bus(fsi_dev, config) };
    if is_err(bus) { return err_cast(bus); }
    unsafe { __regmap_init(&mut (*fsi_dev).dev, bus, (*fsi_dev).slave, config, lock_key, lock_name) }
}

pub unsafe fn __devm_regmap_init_fsi(fsi_dev: *mut FsiDevice, config: *const RegmapConfig, lock_key: *mut LockClassKey, lock_name: *const core::ffi::c_char) -> *mut Regmap {
    let bus = unsafe { regmap_get_fsi_bus(fsi_dev, config) };
    if is_err(bus) { return err_cast(bus); }
    unsafe { __devm_regmap_init(&mut (*fsi_dev).dev, bus, (*fsi_dev).slave, config, lock_key, lock_name) }
}

// EXPORT_SYMBOL_GPL(__regmap_init_fsi)
// EXPORT_SYMBOL_GPL(__devm_regmap_init_fsi)
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
