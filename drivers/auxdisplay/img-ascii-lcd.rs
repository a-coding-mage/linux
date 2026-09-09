// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct linedisp {
    pub buf: *mut u8,
    pub num_chars: c_uint,
    pub dev: device,
}
#[repr(C)] pub struct device { pub kobj: kobject, pub parent: *mut device, pub of_node: *mut device_node }
#[repr(C)] pub struct kobject;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct platform_driver { pub driver: driver, pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut platform_device)> }
#[repr(C)] pub struct driver { pub name: *const c_char, pub of_match_table: *const of_device_id }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }
#[repr(C)] pub struct linedisp_ops { pub update: Option<unsafe extern "C" fn(*mut linedisp)> }

extern "C" {
    fn linedisp_register(ld: *mut linedisp, dev: *mut device, num_chars: c_uint, ops: *const linedisp_ops) -> c_int;
    fn linedisp_unregister(ld: *mut linedisp);
    fn device_get_match_data(dev: *mut device) -> *const img_ascii_lcd_config;
    fn syscon_node_to_regmap(node: *mut device_node) -> *mut regmap;
    fn of_property_read_u32(node: *mut device_node, name: *const c_char, value: *mut u32) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> c_int;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut c_uint) -> c_int;
    fn compat_only_sysfs_link_entry_to_kobj(kobj: *mut kobject, target: *mut kobject, name: *const c_char, attr: *const c_char) -> c_int;
    fn sysfs_remove_link(kobj: *mut kobject, name: *const c_char);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut img_ascii_lcd_ctx);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut img_ascii_lcd_ctx;
}

#[repr(C)]
pub struct img_ascii_lcd_config {
    pub num_chars: c_uint,
    pub external_regmap: bool,
    pub ops: linedisp_ops,
}

#[repr(C)]
pub union img_ascii_lcd_ctx_base { pub base: *mut c_void, pub regmap: *mut regmap }
#[repr(C)]
pub struct img_ascii_lcd_ctx {
    pub linedisp: linedisp,
    pub base_or_regmap: img_ascii_lcd_ctx_base,
    pub offset: u32,
}

unsafe extern "C" fn boston_update(linedisp: *mut linedisp) {
    let ctx = (linedisp as *mut u8).sub(core::mem::offset_of!(img_ascii_lcd_ctx, linedisp)) as *mut img_ascii_lcd_ctx;
    #[cfg(target_pointer_width = "64")]
    { let val = *( (*linedisp).buf as *const u64 ); core::ptr::write_volatile((*ctx).base_or_regmap.base as *mut u64, val); }
    #[cfg(target_pointer_width = "32")]
    { let val = *( (*linedisp).buf as *const u32 ); core::ptr::write_volatile((*ctx).base_or_regmap.base as *mut u32, val); let val = *((*linedisp).buf.add(4) as *const u32); core::ptr::write_volatile(((*ctx).base_or_regmap.base as *mut u8).add(4) as *mut u32, val); }
}

static BOSTON_CONFIG: img_ascii_lcd_config = img_ascii_lcd_config { num_chars: 8, external_regmap: false, ops: linedisp_ops { update: Some(boston_update) } };

unsafe extern "C" fn malta_update(linedisp: *mut linedisp) {
    let ctx = (linedisp as *mut u8).sub(core::mem::offset_of!(img_ascii_lcd_ctx, linedisp)) as *mut img_ascii_lcd_ctx;
    let mut err = 0;
    for i in 0..(*linedisp).num_chars { err = regmap_write((*ctx).base_or_regmap.regmap, (*ctx).offset + i * 8, *(*linedisp).buf.add(i as usize) as u32); if err != 0 { break; } }
    if err != 0 { /* pr_err_ratelimited("Failed to update LCD display: %d\n", err); */ }
}
static MALTA_CONFIG: img_ascii_lcd_config = img_ascii_lcd_config { num_chars: 8, external_regmap: true, ops: linedisp_ops { update: Some(malta_update) } };

const SEAD3_REG_LCD_CTRL: u32 = 0x00;
const SEAD3_REG_LCD_CTRL_SETDRAM: u32 = 1 << 7;
const SEAD3_REG_LCD_DATA: u32 = 0x08;
const SEAD3_REG_CPLD_STATUS: u32 = 0x10;
const SEAD3_REG_CPLD_STATUS_BUSY: u32 = 1;
const SEAD3_REG_CPLD_DATA: u32 = 0x18;
const SEAD3_REG_CPLD_DATA_BUSY: u32 = 1 << 7;

unsafe fn sead3_wait_sm_idle(ctx: *mut img_ascii_lcd_ctx) -> c_int { let mut status = 0; loop { let err = regmap_read((*ctx).base_or_regmap.regmap, (*ctx).offset + SEAD3_REG_CPLD_STATUS, &mut status); if err != 0 { return err; } if status & SEAD3_REG_CPLD_STATUS_BUSY == 0 { return 0; } } }
unsafe fn sead3_wait_lcd_idle(ctx: *mut img_ascii_lcd_ctx) -> c_int { let mut cpld_data = 0; let mut err = sead3_wait_sm_idle(ctx); if err != 0 { return err; } loop { err = regmap_read((*ctx).base_or_regmap.regmap, (*ctx).offset + SEAD3_REG_LCD_CTRL, &mut cpld_data); if err != 0 { return err; } err = sead3_wait_sm_idle(ctx); if err != 0 { return err; } err = regmap_read((*ctx).base_or_regmap.regmap, (*ctx).offset + SEAD3_REG_CPLD_DATA, &mut cpld_data); if err != 0 { return err; } if cpld_data & SEAD3_REG_CPLD_DATA_BUSY == 0 { return 0; } } }
unsafe extern "C" fn sead3_update(linedisp: *mut linedisp) { let ctx = (linedisp as *mut u8).sub(core::mem::offset_of!(img_ascii_lcd_ctx, linedisp)) as *mut img_ascii_lcd_ctx; let mut err = 0; for i in 0..(*linedisp).num_chars { err = sead3_wait_lcd_idle(ctx); if err != 0 { break; } err = regmap_write((*ctx).base_or_regmap.regmap, (*ctx).offset + SEAD3_REG_LCD_CTRL, SEAD3_REG_LCD_CTRL_SETDRAM | i); if err != 0 { break; } err = sead3_wait_lcd_idle(ctx); if err != 0 { break; } err = regmap_write((*ctx).base_or_regmap.regmap, (*ctx).offset + SEAD3_REG_LCD_DATA, *(*linedisp).buf.add(i as usize) as u32); if err != 0 { break; } } if err != 0 { /* pr_err_ratelimited("Failed to update LCD display: %d\n", err); */ } }
static SEAD3_CONFIG: img_ascii_lcd_config = img_ascii_lcd_config { num_chars: 16, external_regmap: true, ops: linedisp_ops { update: Some(sead3_update) } };

static IMG_ASCII_LCD_MATCHES: [of_device_id; 4] = [
    of_device_id { compatible: b"img,boston-lcd\0".as_ptr() as *const c_char, data: &BOSTON_CONFIG as *const _ as *const c_void },
    of_device_id { compatible: b"mti,malta-lcd\0".as_ptr() as *const c_char, data: &MALTA_CONFIG as *const _ as *const c_void },
    of_device_id { compatible: b"mti,sead3-lcd\0".as_ptr() as *const c_char, data: &SEAD3_CONFIG as *const _ as *const c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe extern "C" fn img_ascii_lcd_probe(pdev: *mut platform_device) -> c_int { let dev = &mut (*pdev).dev; let cfg = device_get_match_data(dev); let ctx = devm_kzalloc(dev, core::mem::size_of::<img_ascii_lcd_ctx>(), 0) as *mut img_ascii_lcd_ctx; if ctx.is_null() { return -12; } if (*cfg).external_regmap { (*ctx).base_or_regmap.regmap = syscon_node_to_regmap((*dev).parent.as_ref().unwrap().of_node); if (*ctx).base_or_regmap.regmap.is_null() { return -1; } if of_property_read_u32(dev.of_node, b"offset\0".as_ptr() as *const c_char, &mut (*ctx).offset) != 0 { return -22; } } else { (*ctx).base_or_regmap.base = devm_platform_ioremap_resource(pdev, 0); if (*ctx).base_or_regmap.base.is_null() { return -1; } } let mut err = linedisp_register(&mut (*ctx).linedisp, dev, (*cfg).num_chars, &(*cfg).ops); if err != 0 { return err; } err = compat_only_sysfs_link_entry_to_kobj(&mut dev.kobj, &mut (*ctx).linedisp.dev.kobj, b"message\0".as_ptr() as *const c_char, core::ptr::null()); if err != 0 { linedisp_unregister(&mut (*ctx).linedisp); return err; } platform_set_drvdata(pdev, ctx); 0 }
unsafe extern "C" fn img_ascii_lcd_remove(pdev: *mut platform_device) { let ctx = platform_get_drvdata(pdev); sysfs_remove_link(&mut (*pdev).dev.kobj, b"message\0".as_ptr() as *const c_char); linedisp_unregister(&mut (*ctx).linedisp); }

static mut IMG_ASCII_LCD_DRIVER: platform_driver = platform_driver { driver: driver { name: b"img-ascii-lcd\0".as_ptr() as *const c_char, of_match_table: IMG_ASCII_LCD_MATCHES.as_ptr() }, probe: Some(img_ascii_lcd_probe), remove: Some(img_ascii_lcd_remove) };
// module_platform_driver(IMG_ASCII_LCD_DRIVER);
// MODULE_DESCRIPTION("Imagination Technologies ASCII LCD Display");
// MODULE_AUTHOR("Paul Burton <paul.burton@mips.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("LINEDISP");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
