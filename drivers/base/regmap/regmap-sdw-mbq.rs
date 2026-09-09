// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2020 Intel Corporation.

// Linux/SoundWire dependencies are supplied by the surrounding kernel bindings.

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct sdw_slave { _private: [u8; 0] }
#[repr(C)]
pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct regmap_config {
    pub val_bits: u32,
    pub reg_bits: u32,
    pub pad_bits: u32,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool>,
}
#[repr(C)]
pub struct regmap_sdw_mbq_cfg {
    pub mbq_size: Option<unsafe extern "C" fn(*mut device, u32) -> i32>,
    pub deferrable: Option<unsafe extern "C" fn(*mut device, u32) -> bool>,
    pub retry_us: u32,
    pub timeout_us: u32,
}
#[repr(C)]
struct regmap_bus {
    reg_read: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, *mut u32) -> i32>,
    reg_write: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, u32) -> i32>,
    reg_format_endian_default: u32,
    val_format_endian_default: u32,
}

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const ENODATA: i32 = 61;
const ENOTSUPP: i32 = 524;
const BITS_PER_BYTE: i32 = 8;
const REGMAP_ENDIAN_LITTLE: u32 = 0;

extern "C" {
    fn sdw_read_no_pm(slave: *mut sdw_slave, reg: u32) -> i32;
    fn sdw_write_no_pm(slave: *mut sdw_slave, reg: u32, val: u32) -> i32;
    fn SDW_SDCA_CTL(func: u32, channel: u32, entity: u32, control: u32) -> u32;
    fn SDW_SDCA_CTL_FUNC(reg: u32) -> u32;
    fn SDW_SDCA_MBQ_CTL(reg: u32) -> u32;
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_warn(dev: *mut device, fmt: *const u8, ...);
    fn fsleep(usecs: u32);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn __regmap_init(dev: *mut device, bus: *const regmap_bus, context: *mut core::ffi::c_void,
                     config: *const regmap_config, lock_key: *mut lock_class_key,
                     lock_name: *const u8) -> *mut regmap;
    fn __devm_regmap_init(dev: *mut device, bus: *const regmap_bus, context: *mut core::ffi::c_void,
                          config: *const regmap_config, lock_key: *mut lock_class_key,
                          lock_name: *const u8) -> *mut regmap;
}

#[repr(C)]
struct regmap_mbq_context {
    dev: *mut device,
    sdw: *mut sdw_slave,
    readable_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool>,
    cfg: regmap_sdw_mbq_cfg,
    val_size: i32,
}

unsafe fn regmap_sdw_mbq_size(ctx: *mut regmap_mbq_context, reg: u32) -> i32 {
    let mut size = (*ctx).val_size;
    if let Some(f) = (*ctx).cfg.mbq_size {
        size = f((*ctx).dev, reg);
        if size == 0 || size > (*ctx).val_size { return -EINVAL; }
    }
    size
}

unsafe fn regmap_sdw_mbq_deferrable(ctx: *mut regmap_mbq_context, reg: u32) -> bool {
    if let Some(f) = (*ctx).cfg.deferrable { return f((*ctx).dev, reg); }
    false
}

unsafe fn regmap_sdw_mbq_poll_busy(slave: *mut sdw_slave, mut reg: u32,
                                   ctx: *mut regmap_mbq_context) -> i32 {
    let dev = (*ctx).dev;
    let mut val: i32 = 0;
    dev_dbg(dev, b"Deferring transaction for 0x%x\n\0".as_ptr(), reg);
    reg = SDW_SDCA_CTL(SDW_SDCA_CTL_FUNC(reg), 0, 0, 0);
    if (*ctx).readable_reg.map_or(true, |f| f(dev, reg)) {
        loop {
            val = sdw_read_no_pm(slave, reg);
            if val < 0 || (val & 1) == 0 { break; }
            fsleep((*ctx).cfg.retry_us);
        }
        if val < 0 { return val; }
        if val & 1 != 0 { dev_err(dev, b"Function busy timed out 0x%x: %d\n\0".as_ptr(), reg, val); return -1; }
    } else { fsleep((*ctx).cfg.timeout_us); }
    0
}

unsafe fn regmap_sdw_mbq_write_impl(slave: *mut sdw_slave, reg: u32, val: u32, mut mbq_size: i32) -> i32 {
    let mut shift = mbq_size * BITS_PER_BYTE;
    while { mbq_size -= 1; mbq_size > 0 } {
        shift -= BITS_PER_BYTE;
        let ret = sdw_write_no_pm(slave, SDW_SDCA_MBQ_CTL(reg), (val >> shift) & 0xff);
        if ret < 0 { return ret; }
    }
    sdw_write_no_pm(slave, reg, val & 0xff)
}

unsafe extern "C" fn regmap_sdw_mbq_write(context: *mut core::ffi::c_void, reg: u32, val: u32) -> i32 {
    let ctx = context as *mut regmap_mbq_context;
    let size = regmap_sdw_mbq_size(ctx, reg); if size < 0 { return size; }
    let ret = regmap_sdw_mbq_write_impl((*ctx).sdw, reg, val, size);
    if ret == -ENODATA { let r = regmap_sdw_mbq_poll_busy((*ctx).sdw, reg, ctx); if r != 0 { return r; } return regmap_sdw_mbq_write_impl((*ctx).sdw, reg, val, size); }
    ret
}

unsafe fn regmap_sdw_mbq_read_impl(slave: *mut sdw_slave, reg: u32, val: *mut u32, mut mbq_size: i32) -> i32 {
    let mut shift = BITS_PER_BYTE;
    let mut read = sdw_read_no_pm(slave, reg); if read < 0 { return read; } *val = read as u32;
    while { mbq_size -= 1; mbq_size > 0 } { read = sdw_read_no_pm(slave, SDW_SDCA_MBQ_CTL(reg)); if read < 0 { return read; } *val |= (read << shift) as u32; shift += BITS_PER_BYTE; }
    0
}

unsafe extern "C" fn regmap_sdw_mbq_read(context: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32 {
    let ctx = context as *mut regmap_mbq_context;
    let size = regmap_sdw_mbq_size(ctx, reg); if size < 0 { return size; }
    let ret = regmap_sdw_mbq_read_impl((*ctx).sdw, reg, val, size);
    if ret == -ENODATA { let r = regmap_sdw_mbq_poll_busy((*ctx).sdw, reg, ctx); if r != 0 { return r; } return regmap_sdw_mbq_read_impl((*ctx).sdw, reg, val, size); }
    ret
}

static REGMAP_SDW_MBQ: regmap_bus = regmap_bus { reg_read: Some(regmap_sdw_mbq_read), reg_write: Some(regmap_sdw_mbq_write), reg_format_endian_default: REGMAP_ENDIAN_LITTLE, val_format_endian_default: REGMAP_ENDIAN_LITTLE };

unsafe fn regmap_sdw_mbq_config_check(config: *const regmap_config) -> i32 {
    if (*config).val_bits > (core::mem::size_of::<u32>() as u32 * BITS_PER_BYTE as u32) || (*config).reg_bits != 32 || (*config).pad_bits != 0 { return -ENOTSUPP; }
    0
}

unsafe fn regmap_sdw_mbq_gen_context(dev: *mut device, sdw: *mut sdw_slave, config: *const regmap_config, mbq_config: *const regmap_sdw_mbq_cfg) -> *mut regmap_mbq_context {
    let ctx = devm_kzalloc(dev, core::mem::size_of::<regmap_mbq_context>(), 0) as *mut regmap_mbq_context;
    if ctx.is_null() { return (-ENOMEM) as *mut regmap_mbq_context; }
    (*ctx).dev = dev; (*ctx).sdw = sdw; (*ctx).cfg = if !mbq_config.is_null() { *mbq_config } else { core::mem::zeroed() }; (*ctx).val_size = (*config).val_bits as i32 / BITS_PER_BYTE; (*ctx).readable_reg = (*config).readable_reg; ctx
}

#[no_mangle]
pub unsafe extern "C" fn __regmap_init_sdw_mbq(dev: *mut device, sdw: *mut sdw_slave, config: *const regmap_config, mbq_config: *const regmap_sdw_mbq_cfg, lock_key: *mut lock_class_key, lock_name: *const u8) -> *mut regmap {
    let ret = regmap_sdw_mbq_config_check(config); if ret != 0 { return ret as *mut regmap; }
    let ctx = regmap_sdw_mbq_gen_context(dev, sdw, config, mbq_config); if (ctx as isize) < 0 { return ctx as *mut regmap; }
    __regmap_init(dev, &REGMAP_SDW_MBQ, ctx as *mut _, config, lock_key, lock_name)
}

#[no_mangle]
pub unsafe extern "C" fn __devm_regmap_init_sdw_mbq(dev: *mut device, sdw: *mut sdw_slave, config: *const regmap_config, mbq_config: *const regmap_sdw_mbq_cfg, lock_key: *mut lock_class_key, lock_name: *const u8) -> *mut regmap {
    let ret = regmap_sdw_mbq_config_check(config); if ret != 0 { return ret as *mut regmap; }
    let ctx = regmap_sdw_mbq_gen_context(dev, sdw, config, mbq_config); if (ctx as isize) < 0 { return ctx as *mut regmap; }
    __devm_regmap_init(dev, &REGMAP_SDW_MBQ, ctx as *mut _, config, lock_key, lock_name)
}

// MODULE_DESCRIPTION("regmap SoundWire MBQ Module");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
