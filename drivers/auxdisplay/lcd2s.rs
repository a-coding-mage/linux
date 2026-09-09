// SPDX-License-Identifier: GPL-2.0
/*
 * Console driver for LCD2S 4x20 character displays connected through i2c.
 * The display also has a SPI interface, but the driver does not support
 * this yet.
 *
 * This is a driver allowing you to use a LCD2S 4x20 from Modtronix
 * engineering as auxdisplay character device.
 *
 * (C) 2019 by Lemonage Software GmbH
 * Author: Lars Pöschel <poeschel@lemonage.de>
 * All rights reserved.
 */

const LCD2S_CMD_CUR_MOVES_FWD: u8 = 0x09;
const LCD2S_CMD_CUR_BLINK_OFF: u8 = 0x10;
const LCD2S_CMD_CUR_UL_OFF: u8 = 0x11;
const LCD2S_CMD_DISPLAY_OFF: u8 = 0x12;
const LCD2S_CMD_CUR_BLINK_ON: u8 = 0x18;
const LCD2S_CMD_CUR_UL_ON: u8 = 0x19;
const LCD2S_CMD_DISPLAY_ON: u8 = 0x1a;
const LCD2S_CMD_BACKLIGHT_OFF: u8 = 0x20;
const LCD2S_CMD_BACKLIGHT_ON: u8 = 0x28;
const LCD2S_CMD_WRITE: u8 = 0x80;
const LCD2S_CMD_MOV_CUR_RIGHT: u8 = 0x83;
const LCD2S_CMD_MOV_CUR_LEFT: u8 = 0x84;
const LCD2S_CMD_SHIFT_RIGHT: u8 = 0x85;
const LCD2S_CMD_SHIFT_LEFT: u8 = 0x86;
const LCD2S_CMD_SHIFT_UP: u8 = 0x87;
const LCD2S_CMD_SHIFT_DOWN: u8 = 0x88;
const LCD2S_CMD_CUR_ADDR: u8 = 0x89;
const LCD2S_CMD_CUR_POS: u8 = 0x8a;
const LCD2S_CMD_CUR_RESET: u8 = 0x8b;
const LCD2S_CMD_CLEAR: u8 = 0x8c;
const LCD2S_CMD_DEF_CUSTOM_CHAR: u8 = 0x92;
const LCD2S_CMD_READ_STATUS: u8 = 0xd0;
const LCD2S_CHARACTER_SIZE: usize = 8;
const LCD2S_STATUS_BUF_MASK: i32 = 0x7f;

#[repr(C)]
struct lcd2s_data {
    i2c: *mut i2c_client,
    charlcd: *mut charlcd,
}

unsafe fn lcd2s_wait_buf_free(client: *const i2c_client, count: i32) -> i32 {
    let mut status = i2c_smbus_read_byte_data(client, LCD2S_CMD_READ_STATUS);
    if status < 0 { return status; }
    while (status & LCD2S_STATUS_BUF_MASK) < count {
        mdelay(1);
        status = i2c_smbus_read_byte_data(client, LCD2S_CMD_READ_STATUS);
        if status < 0 { return status; }
    }
    0
}

unsafe fn lcd2s_i2c_master_send(client: *const i2c_client, buf: *const u8, count: i32) -> i32 {
    let status = lcd2s_wait_buf_free(client, count);
    if status < 0 { return status; }
    i2c_master_send(client, buf, count)
}

unsafe fn lcd2s_i2c_smbus_write_byte(client: *const i2c_client, value: u8) -> i32 {
    let status = lcd2s_wait_buf_free(client, 1);
    if status < 0 { return status; }
    i2c_smbus_write_byte(client, value)
}

unsafe fn lcd2s_print(lcd: *mut charlcd, c: i32) -> i32 {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    let buf = [LCD2S_CMD_WRITE, c as u8];
    let ret = lcd2s_i2c_master_send((*lcd2s).i2c, buf.as_ptr(), 2);
    if ret < 0 { return ret; }
    if ret != 2 { return -EIO; }
    0
}

unsafe fn lcd2s_gotoxy(lcd: *mut charlcd, x: u32, y: u32) -> i32 {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    let buf = [LCD2S_CMD_CUR_POS, (y + 1) as u8, (x + 1) as u8];
    let ret = lcd2s_i2c_master_send((*lcd2s).i2c, buf.as_ptr(), 3);
    if ret < 0 { return ret; }
    if ret != 3 { return -EIO; }
    0
}

unsafe fn lcd2s_home(lcd: *mut charlcd) -> i32 {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, LCD2S_CMD_CUR_RESET);
    0
}

unsafe fn lcd2s_init_display(lcd: *mut charlcd) -> i32 {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, LCD2S_CMD_DISPLAY_ON);
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, LCD2S_CMD_BACKLIGHT_OFF);
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, LCD2S_CMD_CUR_MOVES_FWD);
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, LCD2S_CMD_CUR_BLINK_OFF);
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, LCD2S_CMD_CUR_UL_OFF);
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, LCD2S_CMD_CLEAR);
    0
}

unsafe fn lcd2s_shift_cursor(lcd: *mut charlcd, dir: charlcd_shift_dir) -> i32 {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    let cmd = if dir == CHARLCD_SHIFT_LEFT { LCD2S_CMD_MOV_CUR_LEFT } else { LCD2S_CMD_MOV_CUR_RIGHT };
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, cmd);
    0
}

unsafe fn lcd2s_shift_display(lcd: *mut charlcd, dir: charlcd_shift_dir) -> i32 {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    let cmd = if dir == CHARLCD_SHIFT_LEFT { LCD2S_CMD_SHIFT_LEFT } else { LCD2S_CMD_SHIFT_RIGHT };
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, cmd);
    0
}

unsafe fn lcd2s_backlight(lcd: *mut charlcd, on: charlcd_onoff) {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, if on != 0 { LCD2S_CMD_BACKLIGHT_ON } else { LCD2S_CMD_BACKLIGHT_OFF });
}

unsafe fn lcd2s_display(lcd: *mut charlcd, on: charlcd_onoff) -> i32 {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, if on != 0 { LCD2S_CMD_DISPLAY_ON } else { LCD2S_CMD_DISPLAY_OFF });
    0
}

unsafe fn lcd2s_cursor(lcd: *mut charlcd, on: charlcd_onoff) -> i32 {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, if on != 0 { LCD2S_CMD_CUR_UL_ON } else { LCD2S_CMD_CUR_UL_OFF });
    0
}

unsafe fn lcd2s_blink(lcd: *mut charlcd, on: charlcd_onoff) -> i32 {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, if on != 0 { LCD2S_CMD_CUR_BLINK_ON } else { LCD2S_CMD_CUR_BLINK_OFF });
    0
}

unsafe fn lcd2s_fontsize(_lcd: *mut charlcd, _size: charlcd_fontsize) -> i32 { 0 }
unsafe fn lcd2s_lines(_lcd: *mut charlcd, _lines: charlcd_lines) -> i32 { 0 }

/* Generator: LGcxxxxx...xx; <c> is between '0' and '7', and <xx...xx>
 * is a sequence of 16 hex digits representing 8 bytes. */
unsafe fn lcd2s_redefine_char(lcd: *mut charlcd, mut esc: *mut u8) -> i32 {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    if strchr(esc, b';') == 0 { return 0; }
    esc = esc.add(1);
    let mut buf = [0u8; LCD2S_CHARACTER_SIZE + 2];
    buf[0] = LCD2S_CMD_DEF_CUSTOM_CHAR;
    buf[1] = *esc; esc = esc.add(1); buf[1] = buf[1].wrapping_sub(b'0');
    if buf[1] > 7 { return 1; }
    let mut i = 2usize; let mut shift = 0i32; let mut value = 0u8;
    while *esc != 0 && i < LCD2S_CHARACTER_SIZE + 2 {
        shift ^= 4;
        let half = hex_to_bin(*esc); esc = esc.add(1);
        if half < 0 { continue; }
        value |= (half as u8) << shift;
        if shift == 0 { buf[i] = value; i += 1; value = 0; }
    }
    lcd2s_i2c_master_send((*lcd2s).i2c, buf.as_ptr(), buf.len() as i32);
    1
}

unsafe fn lcd2s_clear_display(lcd: *mut charlcd) -> i32 {
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    lcd2s_i2c_smbus_write_byte((*lcd2s).i2c, LCD2S_CMD_CLEAR);
    0
}

#[repr(C)]
struct lcd2s_ops_type {
    print: Option<unsafe fn(*mut charlcd, i32) -> i32>,
    backlight: Option<unsafe fn(*mut charlcd, charlcd_onoff)>,
    gotoxy: Option<unsafe fn(*mut charlcd, u32, u32) -> i32>,
    home: Option<unsafe fn(*mut charlcd) -> i32>,
    clear_display: Option<unsafe fn(*mut charlcd) -> i32>,
    init_display: Option<unsafe fn(*mut charlcd) -> i32>,
    shift_cursor: Option<unsafe fn(*mut charlcd, charlcd_shift_dir) -> i32>,
    shift_display: Option<unsafe fn(*mut charlcd, charlcd_shift_dir) -> i32>,
    display: Option<unsafe fn(*mut charlcd, charlcd_onoff) -> i32>,
    cursor: Option<unsafe fn(*mut charlcd, charlcd_onoff) -> i32>,
    blink: Option<unsafe fn(*mut charlcd, charlcd_onoff) -> i32>,
    fontsize: Option<unsafe fn(*mut charlcd, charlcd_fontsize) -> i32>,
    lines: Option<unsafe fn(*mut charlcd, charlcd_lines) -> i32>,
    redefine_char: Option<unsafe fn(*mut charlcd, *mut u8) -> i32>,
}

static lcd2s_ops: lcd2s_ops_type = lcd2s_ops_type {
    print: Some(lcd2s_print), backlight: Some(lcd2s_backlight), gotoxy: Some(lcd2s_gotoxy),
    home: Some(lcd2s_home), clear_display: Some(lcd2s_clear_display), init_display: Some(lcd2s_init_display),
    shift_cursor: Some(lcd2s_shift_cursor), shift_display: Some(lcd2s_shift_display), display: Some(lcd2s_display),
    cursor: Some(lcd2s_cursor), blink: Some(lcd2s_blink), fontsize: Some(lcd2s_fontsize), lines: Some(lcd2s_lines),
    redefine_char: Some(lcd2s_redefine_char),
};

// External kernel and charlcd declarations are supplied by the surrounding translation unit.
extern "C" {
    fn i2c_smbus_read_byte_data(client: *const i2c_client, command: u8) -> i32;
    fn i2c_master_send(client: *const i2c_client, buf: *const u8, count: i32) -> i32;
    fn i2c_smbus_write_byte(client: *const i2c_client, value: u8) -> i32;
    fn mdelay(ms: u32);
    fn strchr(s: *mut u8, c: u8) -> *mut u8;
    fn hex_to_bin(c: u8) -> i32;
}

// The following driver registration items retain the C driver's external interface.
unsafe fn lcd2s_i2c_probe(i2c: *mut i2c_client) -> i32 {
    if !i2c_check_functionality((*i2c).adapter,
            I2C_FUNC_SMBUS_WRITE_BYTE_DATA | I2C_FUNC_SMBUS_WRITE_BLOCK_DATA) { return -EIO; }
    let err = lcd2s_i2c_smbus_write_byte(i2c, LCD2S_CMD_DISPLAY_OFF);
    if err < 0 { return err; }
    let lcd = charlcd_alloc(core::mem::size_of::<lcd2s_data>());
    if lcd.is_null() { return -ENOMEM; }
    (*lcd).ops = &lcd2s_ops;
    let lcd2s = (*lcd).drvdata as *mut lcd2s_data;
    (*lcd2s).i2c = i2c;
    (*lcd2s).charlcd = lcd;
    let mut err = device_property_read_u32(&(*i2c).dev, "display-height-chars", &mut (*lcd).height);
    if err != 0 { charlcd_free((*lcd2s).charlcd); return err; }
    err = device_property_read_u32(&(*i2c).dev, "display-width-chars", &mut (*lcd).width);
    if err != 0 { charlcd_free((*lcd2s).charlcd); return err; }
    err = charlcd_register((*lcd2s).charlcd);
    if err != 0 { charlcd_free((*lcd2s).charlcd); return err; }
    i2c_set_clientdata(i2c, lcd2s);
    0
}

unsafe fn lcd2s_i2c_remove(i2c: *mut i2c_client) {
    let lcd2s = i2c_get_clientdata(i2c) as *mut lcd2s_data;
    charlcd_unregister((*lcd2s).charlcd);
    charlcd_free((*lcd2s).charlcd);
}

// Equivalent declarations for types, constants, and helpers supplied by Linux and charlcd headers.
#[allow(non_camel_case_types)] type s32 = i32;
#[allow(non_camel_case_types)] type u8 = core::ffi::c_uchar;
#[repr(C)] struct i2c_client { adapter: *mut i2c_adapter, dev: device }
#[repr(C)] struct i2c_adapter;
#[repr(C)] struct device;
#[repr(C)] struct charlcd { drvdata: *mut core::ffi::c_void, ops: *const charlcd_ops, height: u32, width: u32 }
#[repr(C)] struct charlcd_ops;
type charlcd_shift_dir = i32;
type charlcd_onoff = i32;
type charlcd_fontsize = i32;
type charlcd_lines = i32;
const CHARLCD_SHIFT_LEFT: charlcd_shift_dir = 0;
const I2C_FUNC_SMBUS_WRITE_BYTE_DATA: u32 = 1 << 0;
const I2C_FUNC_SMBUS_WRITE_BLOCK_DATA: u32 = 1 << 1;
const EIO: i32 = 5;
const ENOMEM: i32 = 12;

extern "C" {
    fn i2c_check_functionality(adapter: *mut i2c_adapter, functionality: u32) -> bool;
    fn charlcd_alloc(size: usize) -> *mut charlcd;
    fn charlcd_free(lcd: *mut charlcd);
    fn charlcd_register(lcd: *mut charlcd) -> i32;
    fn charlcd_unregister(lcd: *mut charlcd);
    fn device_property_read_u32(dev: *const device, name: *const u8, value: *mut u32) -> i32;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut core::ffi::c_void);
    fn i2c_get_clientdata(i2c: *mut i2c_client) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
