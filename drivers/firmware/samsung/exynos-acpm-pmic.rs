// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2020 Samsung Electronics Co., Ltd.
 * Copyright 2020 Google LLC.
 * Copyright 2024 Linaro Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_int;

const ACPM_PMIC_CHANNEL: u32 = 0x0000_f000;
const ACPM_PMIC_TYPE: u32 = 0x0000_0f00;
const ACPM_PMIC_REG: u32 = 0x0000_00ff;

const ACPM_PMIC_RETURN: u32 = 0xff00_0000;
const ACPM_PMIC_MASK: u32 = 0x00ff_0000;
const ACPM_PMIC_VALUE: u32 = 0x0000_ff00;
const ACPM_PMIC_FUNC: u32 = 0x0000_00ff;

const ACPM_PMIC_BULK_SHIFT: u32 = 8;
const ACPM_PMIC_BULK_MASK: u32 = 0xff;
const ACPM_PMIC_BULK_MAX_COUNT: u8 = 8;

#[repr(C)]
pub struct acpm_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpm_xfer {
    pub rxd: [u32; 4],
    _private: [u8; 0],
}

#[repr(i32)]
enum exynos_acpm_pmic_func {
    ACPM_PMIC_READ,
    ACPM_PMIC_WRITE,
    ACPM_PMIC_UPDATE,
    ACPM_PMIC_BULK_READ,
    ACPM_PMIC_BULK_WRITE,
}

static ACPM_PMIC_LINUX_ERRMAP: [c_int; 3] = [0, -13, -13];

unsafe extern "C" {
    fn acpm_set_xfer(xfer: *mut acpm_xfer, cmd: *mut u32, count: usize,
                     acpm_chan_id: u32, wait: bool);
    fn acpm_do_xfer(handle: *mut acpm_handle, xfer: *mut acpm_xfer) -> c_int;
    fn ktime_get() -> i64;
    fn ktime_to_ms(ktime: i64) -> u32;
}

#[inline]
fn field_prep(mask: u32, value: u32) -> u32 {
    (value << mask.trailing_zeros()) & mask
}

#[inline]
fn field_get(mask: u32, value: u32) -> u32 {
    (value & mask) >> mask.trailing_zeros()
}

fn acpm_pmic_to_linux_err(err: u32) -> c_int {
    if err < ACPM_PMIC_LINUX_ERRMAP.len() as u32 {
        return ACPM_PMIC_LINUX_ERRMAP[err as usize];
    }
    -5
}

#[inline]
fn acpm_pmic_set_bulk(data: u32, i: u32) -> u32 {
    (data & ACPM_PMIC_BULK_MASK) << (ACPM_PMIC_BULK_SHIFT * i)
}

#[inline]
fn acpm_pmic_get_bulk(data: u32, i: u32) -> u32 {
    (data >> (ACPM_PMIC_BULK_SHIFT * i)) & ACPM_PMIC_BULK_MASK
}

fn acpm_pmic_init_read_cmd(cmd: *mut u32, type_: u8, reg: u8, chan: u8) {
    unsafe {
        *cmd.add(0) = field_prep(ACPM_PMIC_TYPE, type_ as u32)
            | field_prep(ACPM_PMIC_REG, reg as u32)
            | field_prep(ACPM_PMIC_CHANNEL, chan as u32);
        *cmd.add(1) = field_prep(ACPM_PMIC_FUNC, ACPM_PMIC_READ as u32);
        *cmd.add(3) = ktime_to_ms(ktime_get());
    }
}

pub unsafe fn acpm_pmic_read_reg(handle: *mut acpm_handle, acpm_chan_id: u32,
                                 type_: u8, reg: u8, chan: u8, buf: *mut u8) -> c_int {
    let mut xfer = core::mem::MaybeUninit::<acpm_xfer>::uninit();
    let mut cmd = [0u32; 4];
    acpm_pmic_init_read_cmd(cmd.as_mut_ptr(), type_, reg, chan);
    acpm_set_xfer(xfer.as_mut_ptr(), cmd.as_mut_ptr(), cmd.len(), acpm_chan_id, true);
    let mut xfer = xfer.assume_init();
    let ret = acpm_do_xfer(handle, &mut xfer);
    if ret != 0 { return ret; }
    *buf = field_get(ACPM_PMIC_VALUE, xfer.rxd[1]) as u8;
    acpm_pmic_to_linux_err(field_get(ACPM_PMIC_RETURN, xfer.rxd[1]))
}

fn acpm_pmic_init_bulk_read_cmd(cmd: *mut u32, type_: u8, reg: u8, chan: u8, count: u8) {
    unsafe {
        *cmd.add(0) = field_prep(ACPM_PMIC_TYPE, type_ as u32) | field_prep(ACPM_PMIC_REG, reg as u32) | field_prep(ACPM_PMIC_CHANNEL, chan as u32);
        *cmd.add(1) = field_prep(ACPM_PMIC_FUNC, ACPM_PMIC_BULK_READ as u32) | field_prep(ACPM_PMIC_VALUE, count as u32);
    }
}

pub unsafe fn acpm_pmic_bulk_read(handle: *mut acpm_handle, acpm_chan_id: u32, type_: u8, reg: u8, chan: u8, count: u8, buf: *mut u8) -> c_int {
    if count > ACPM_PMIC_BULK_MAX_COUNT { return -22; }
    let mut xfer = core::mem::MaybeUninit::<acpm_xfer>::uninit(); let mut cmd = [0u32; 4];
    acpm_pmic_init_bulk_read_cmd(cmd.as_mut_ptr(), type_, reg, chan, count);
    acpm_set_xfer(xfer.as_mut_ptr(), cmd.as_mut_ptr(), cmd.len(), acpm_chan_id, true); let mut xfer = xfer.assume_init();
    let ret = acpm_do_xfer(handle, &mut xfer); if ret != 0 { return ret; }
    let ret = acpm_pmic_to_linux_err(field_get(ACPM_PMIC_RETURN, xfer.rxd[1])); if ret != 0 { return ret; }
    for i in 0..count as u32 { *buf.add(i as usize) = acpm_pmic_get_bulk(xfer.rxd[if i < 4 { 2 } else { 3 }], if i < 4 { i } else { i - 4 }) as u8; } 0
}

fn acpm_pmic_init_write_cmd(cmd: *mut u32, type_: u8, reg: u8, chan: u8, value: u8) { unsafe { *cmd.add(0) = field_prep(ACPM_PMIC_TYPE,type_ as u32)|field_prep(ACPM_PMIC_REG,reg as u32)|field_prep(ACPM_PMIC_CHANNEL,chan as u32); *cmd.add(1)=field_prep(ACPM_PMIC_FUNC,ACPM_PMIC_WRITE as u32)|field_prep(ACPM_PMIC_VALUE,value as u32); *cmd.add(3)=ktime_to_ms(ktime_get()); } }
pub unsafe fn acpm_pmic_write_reg(handle:*mut acpm_handle, id:u32, t:u8, r:u8, c:u8, v:u8)->c_int { let mut x=core::mem::MaybeUninit::<acpm_xfer>::uninit();let mut q=[0u32;4];acpm_pmic_init_write_cmd(q.as_mut_ptr(),t,r,c,v);acpm_set_xfer(x.as_mut_ptr(),q.as_mut_ptr(),4,id,true);let mut x=x.assume_init();let z=acpm_do_xfer(handle,&mut x);if z!=0{z}else{acpm_pmic_to_linux_err(field_get(ACPM_PMIC_RETURN,x.rxd[1]))} }

// Bulk-write and update declarations retain the source interfaces and behavior.
pub unsafe fn acpm_pmic_bulk_write(handle:*mut acpm_handle,id:u32,t:u8,r:u8,c:u8,count:u8,buf:*const u8)->c_int { if count>8{return -22;} let mut x=core::mem::MaybeUninit::<acpm_xfer>::uninit();let mut q=[0u32;4];q[0]=field_prep(ACPM_PMIC_TYPE,t as u32)|field_prep(ACPM_PMIC_REG,r as u32)|field_prep(ACPM_PMIC_CHANNEL,c as u32);q[1]=field_prep(ACPM_PMIC_FUNC,ACPM_PMIC_BULK_WRITE as u32)|field_prep(ACPM_PMIC_VALUE,count as u32);for i in 0..count as u32{q[if i<4{2}else{3}]|=acpm_pmic_set_bulk(*buf.add(i as usize),if i<4{i}else{i-4});}acpm_set_xfer(x.as_mut_ptr(),q.as_mut_ptr(),4,id,true);let mut x=x.assume_init();let z=acpm_do_xfer(handle,&mut x);if z!=0{z}else{acpm_pmic_to_linux_err(field_get(ACPM_PMIC_RETURN,x.rxd[1]))} }
pub unsafe fn acpm_pmic_update_reg(handle:*mut acpm_handle,id:u32,t:u8,r:u8,c:u8,v:u8,m:u8)->c_int { let mut x=core::mem::MaybeUninit::<acpm_xfer>::uninit();let mut q=[0u32;4];q[0]=field_prep(ACPM_PMIC_TYPE,t as u32)|field_prep(ACPM_PMIC_REG,r as u32)|field_prep(ACPM_PMIC_CHANNEL,c as u32);q[1]=field_prep(ACPM_PMIC_FUNC,ACPM_PMIC_UPDATE as u32)|field_prep(ACPM_PMIC_VALUE,v as u32)|field_prep(ACPM_PMIC_MASK,m as u32);q[3]=ktime_to_ms(ktime_get());acpm_set_xfer(x.as_mut_ptr(),q.as_mut_ptr(),4,id,true);let mut x=x.assume_init();let z=acpm_do_xfer(handle,&mut x);if z!=0{z}else{acpm_pmic_to_linux_err(field_get(ACPM_PMIC_RETURN,x.rxd[1]))} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
