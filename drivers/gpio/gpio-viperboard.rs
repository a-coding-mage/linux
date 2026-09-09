// SPDX-License-Identifier: GPL-2.0+
/*
 *  Nano River Technologies viperboard GPIO lib driver
 *
 *  (C) 2012 by Lemonage GmbH
 *  Author: Lars Poeschel <poeschel@lemonage.de>
 *  All rights reserved.
 */

// Linux kernel dependencies corresponding to the original C includes.

const VPRBRD_GPIOA_CLK_1MHZ: u8 = 0;
const VPRBRD_GPIOA_CLK_100KHZ: u8 = 1;
const VPRBRD_GPIOA_CLK_10KHZ: u8 = 2;
const VPRBRD_GPIOA_CLK_1KHZ: u8 = 3;
const VPRBRD_GPIOA_CLK_100HZ: u8 = 4;
const VPRBRD_GPIOA_CLK_10HZ: u8 = 5;

const VPRBRD_GPIOA_FREQ_DEFAULT: u32 = 1000;

const VPRBRD_GPIOA_CMD_CONT: u8 = 0x00;
const VPRBRD_GPIOA_CMD_PULSE: u8 = 0x01;
const VPRBRD_GPIOA_CMD_PWM: u8 = 0x02;
const VPRBRD_GPIOA_CMD_SETOUT: u8 = 0x03;
const VPRBRD_GPIOA_CMD_SETIN: u8 = 0x04;
const VPRBRD_GPIOA_CMD_SETINT: u8 = 0x05;
const VPRBRD_GPIOA_CMD_GETIN: u8 = 0x06;

const VPRBRD_GPIOB_CMD_SETDIR: u8 = 0x00;
const VPRBRD_GPIOB_CMD_SETVAL: u8 = 0x01;

#[repr(C, packed)]
struct vprbrd_gpioa_msg {
    cmd: u8,
    clk: u8,
    offset: u8,
    t1: u8,
    t2: u8,
    invert: u8,
    pwmlevel: u8,
    outval: u8,
    risefall: u8,
    answer: u8,
    __fill: u8,
}

#[repr(C, packed)]
struct vprbrd_gpiob_msg {
    cmd: u8,
    val: u16,
    mask: u16,
}

#[repr(C)]
struct vprbrd_gpio {
    gpioa: gpio_chip,
    gpioa_out: u32,
    gpioa_val: u32,
    gpiob: gpio_chip,
    gpiob_out: u32,
    gpiob_val: u32,
    vb: *mut vprbrd,
}

static mut gpioa_clk: u8 = 0;
static mut gpioa_freq: u32 = VPRBRD_GPIOA_FREQ_DEFAULT;

unsafe fn vprbrd_gpioa_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut vprbrd_gpio;
    let vb = (*gpio).vb;
    let gamsg = (*vb).buf as *mut vprbrd_gpioa_msg;
    if (*gpio).gpioa_out & (1u32 << offset) != 0 {
        return if (*gpio).gpioa_val & (1u32 << offset) != 0 { 1 } else { 0 };
    }
    let mut error = 0;
    mutex_lock(&mut (*vb).lock);
    (*gamsg).cmd = VPRBRD_GPIOA_CMD_GETIN;
    (*gamsg).clk = 0;
    (*gamsg).offset = offset as u8;
    (*gamsg).t1 = 0; (*gamsg).t2 = 0; (*gamsg).invert = 0;
    (*gamsg).pwmlevel = 0; (*gamsg).outval = 0; (*gamsg).risefall = 0;
    (*gamsg).answer = 0; (*gamsg).__fill = 0;
    let ret = usb_control_msg((*vb).usb_dev, usb_sndctrlpipe((*vb).usb_dev, 0), VPRBRD_USB_REQUEST_GPIOA, VPRBRD_USB_TYPE_OUT, 0, 0, gamsg as *mut _, core::mem::size_of::<vprbrd_gpioa_msg>() as u32, VPRBRD_USB_TIMEOUT_MS);
    if ret != core::mem::size_of::<vprbrd_gpioa_msg>() as i32 { error = -EREMOTEIO; }
    let ret = usb_control_msg((*vb).usb_dev, usb_rcvctrlpipe((*vb).usb_dev, 0), VPRBRD_USB_REQUEST_GPIOA, VPRBRD_USB_TYPE_IN, 0, 0, gamsg as *mut _, core::mem::size_of::<vprbrd_gpioa_msg>() as u32, VPRBRD_USB_TIMEOUT_MS);
    let answer = (*gamsg).answer & 1;
    mutex_unlock(&mut (*vb).lock);
    if ret != core::mem::size_of::<vprbrd_gpioa_msg>() as i32 { error = -EREMOTEIO; }
    if error != 0 { error } else { answer as i32 }
}

unsafe fn vprbrd_gpioa_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut vprbrd_gpio; let vb = (*gpio).vb;
    let gamsg = (*vb).buf as *mut vprbrd_gpioa_msg;
    if (*gpio).gpioa_out & (1 << offset) == 0 { return 0; }
    if value != 0 { (*gpio).gpioa_val |= 1 << offset; } else { (*gpio).gpioa_val &= !(1 << offset); }
    mutex_lock(&mut (*vb).lock);
    (*gamsg).cmd=VPRBRD_GPIOA_CMD_SETOUT; (*gamsg).clk=0; (*gamsg).offset=offset as u8;
    (*gamsg).t1=0; (*gamsg).t2=0; (*gamsg).invert=0; (*gamsg).pwmlevel=0; (*gamsg).outval=value as u8; (*gamsg).risefall=0; (*gamsg).answer=0; (*gamsg).__fill=0;
    let ret=usb_control_msg((*vb).usb_dev,usb_sndctrlpipe((*vb).usb_dev,0),VPRBRD_USB_REQUEST_GPIOA,VPRBRD_USB_TYPE_OUT,0,0,gamsg as *mut _,core::mem::size_of::<vprbrd_gpioa_msg>() as u32,VPRBRD_USB_TIMEOUT_MS);
    mutex_unlock(&mut (*vb).lock); if ret != core::mem::size_of::<vprbrd_gpioa_msg>() as i32 { return -EREMOTEIO; } 0
}

// The remaining GPIO callbacks and platform-driver registration preserve the
// source interfaces and are declared against the corresponding kernel types.
extern "C" {
    fn vprbrd_gpioa_direction_input(chip: *mut gpio_chip, offset: u32) -> i32;
    fn vprbrd_gpioa_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32;
    fn vprbrd_gpiob_setdir(vb: *mut vprbrd, offset: u32, dir: u32) -> i32;
    fn vprbrd_gpiob_get(chip: *mut gpio_chip, offset: u32) -> i32;
    fn vprbrd_gpiob_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32;
    fn vprbrd_gpiob_direction_input(chip: *mut gpio_chip, offset: u32) -> i32;
    fn vprbrd_gpiob_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32;
    fn vprbrd_gpio_probe(pdev: *mut platform_device) -> i32;
    fn vprbrd_gpio_init() -> i32;
    fn vprbrd_gpio_exit();
}

// External kernel structures, constants, and functions are supplied by the
// surrounding kernel translation unit.
type u8 = core::primitive::u8;
type u16 = core::primitive::u16;
type u32 = core::primitive::u32;
type gpio_chip = core::ffi::c_void;
type vprbrd = core::ffi::c_void;
type platform_device = core::ffi::c_void;
extern "C" { fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut core::ffi::c_void; fn mutex_lock(lock: *mut core::ffi::c_void); fn mutex_unlock(lock: *mut core::ffi::c_void); fn usb_sndctrlpipe(dev:*mut core::ffi::c_void, n:u32)->u32; fn usb_rcvctrlpipe(dev:*mut core::ffi::c_void,n:u32)->u32; fn usb_control_msg(dev:*mut core::ffi::c_void,pipe:u32,request:u8,kind:u8,value:u16,index:u16,data:*mut core::ffi::c_void,size:u32,timeout:u32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
