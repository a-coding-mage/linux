/*
 * Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

const SCL: bool = false;
const SDA: bool = true;

#[repr(C)]
pub struct dc_context;
#[repr(C)]
pub struct resource_pool;
#[repr(C)]
pub struct gpio;

#[repr(C)]
pub struct ddc {
    pub pin_data: *mut gpio,
    pub pin_clock: *mut gpio,
}

#[repr(C)]
pub struct dce_i2c_sw {
    pub ctx: *mut dc_context,
    pub ddc: *mut ddc,
    pub speed: u32,
    pub clock_delay: u32,
}

#[repr(C)]
pub struct i2c_payload {
    pub address: u16,
    pub write: bool,
    pub length: u32,
    pub data: *mut u8,
}

#[repr(C)]
pub struct i2c_command {
    pub speed: u32,
    pub number_of_payloads: u8,
    pub payloads: *mut i2c_payload,
}

#[repr(C)]
pub struct i2c_request_transaction_data {
    pub action: u32,
    pub address: u8,
    pub length: u32,
    pub data: *mut u8,
    pub status: u32,
}

extern "C" {
    fn dal_gpio_get_value(pin: *mut gpio, value: *mut u32);
    fn dal_gpio_set_value(pin: *mut gpio, value: u32);
    fn dal_ddc_close(ddc: *mut ddc);
    fn dal_ddc_open(ddc: *mut ddc, mode: u32, config: u32) -> u32;
    fn udelay(usecs: u32);
    fn ASSERT(condition: bool);
}

const I2C_SW_TIMEOUT_DELAY: u32 = 1000;
const I2C_SW_RETRIES: u32 = 3;
const DCE_I2C_DEFAULT_I2C_SW_SPEED: u32 = 50;
const GPIO_MODE_FAST_OUTPUT: u32 = 0;
const GPIO_DDC_CONFIG_TYPE_MODE_I2C: u32 = 0;
const GPIO_RESULT_OK: u32 = 0;
const DCE_I2C_TRANSACTION_ACTION_I2C_WRITE: u32 = 0;
const DCE_I2C_TRANSACTION_ACTION_I2C_WRITE_MOT: u32 = 1;
const DCE_I2C_TRANSACTION_ACTION_I2C_READ: u32 = 2;
const DCE_I2C_TRANSACTION_ACTION_I2C_READ_MOT: u32 = 3;
const I2C_CHANNEL_OPERATION_ENGINE_BUSY: u32 = 1;
const I2C_CHANNEL_OPERATION_SUCCEEDED: u32 = 0;
const I2C_CHANNEL_OPERATION_FAILED: u32 = 2;

pub unsafe fn dce_i2c_sw_construct(dce_i2c_sw: *mut dce_i2c_sw, ctx: *mut dc_context) {
    (*dce_i2c_sw).ctx = ctx;
}

unsafe fn read_bit_from_ddc(ddc: *mut ddc, data_nor_clock: bool) -> bool {
    let mut value: u32 = 0;
    if data_nor_clock { dal_gpio_get_value((*ddc).pin_data, &mut value); }
    else { dal_gpio_get_value((*ddc).pin_clock, &mut value); }
    value != 0
}

unsafe fn write_bit_to_ddc(ddc: *mut ddc, data_nor_clock: bool, bit: bool) {
    let value = if bit { 1 } else { 0 };
    if data_nor_clock { dal_gpio_set_value((*ddc).pin_data, value); }
    else { dal_gpio_set_value((*ddc).pin_clock, value); }
}

unsafe fn release_engine_dce_sw(_pool: *mut resource_pool, dce_i2c_sw: *mut dce_i2c_sw) {
    dal_ddc_close((*dce_i2c_sw).ddc);
    (*dce_i2c_sw).ddc = core::ptr::null_mut();
}

unsafe fn wait_for_scl_high_sw(ctx: *mut dc_context, ddc: *mut ddc, delay: u32) -> bool {
    let _ = ctx;
    let mut retry = 0;
    let max = I2C_SW_TIMEOUT_DELAY / delay;
    udelay(delay);
    loop {
        if read_bit_from_ddc(ddc, SCL) { return true; }
        udelay(delay);
        retry += 1;
        if retry > max { break; }
    }
    false
}

unsafe fn write_byte_sw(ctx: *mut dc_context, ddc: *mut ddc, delay: u32, byte: u8) -> bool {
    let mut shift: i32 = 7;
    loop {
        udelay(delay);
        write_bit_to_ddc(ddc, SDA, ((byte >> shift) & 1) != 0);
        udelay(delay);
        write_bit_to_ddc(ddc, SCL, true);
        if !wait_for_scl_high_sw(ctx, ddc, delay) { return false; }
        write_bit_to_ddc(ddc, SCL, false);
        shift -= 1;
        if shift < 0 { break; }
    }
    udelay(delay); write_bit_to_ddc(ddc, SDA, true); udelay(delay);
    write_bit_to_ddc(ddc, SCL, true);
    if !wait_for_scl_high_sw(ctx, ddc, delay) { return false; }
    let ack = !read_bit_from_ddc(ddc, SDA);
    udelay(delay << 1); write_bit_to_ddc(ddc, SCL, false); udelay(delay << 1);
    ack
}

unsafe fn read_byte_sw(ctx: *mut dc_context, ddc: *mut ddc, delay: u32, byte: *mut u8, more: bool) -> bool {
    let mut shift: i32 = 7;
    let mut data = 0u8;
    loop {
        write_bit_to_ddc(ddc, SCL, true);
        if !wait_for_scl_high_sw(ctx, ddc, delay) { return false; }
        if read_bit_from_ddc(ddc, SDA) { data |= 1u8 << shift; }
        write_bit_to_ddc(ddc, SCL, false); udelay(delay << 1);
        shift -= 1; if shift < 0 { break; }
    }
    *byte = data; udelay(delay); write_bit_to_ddc(ddc, SDA, !more); udelay(delay);
    write_bit_to_ddc(ddc, SCL, true);
    if !wait_for_scl_high_sw(ctx, ddc, delay) { return false; }
    write_bit_to_ddc(ddc, SCL, false); udelay(delay); write_bit_to_ddc(ddc, SDA, true); udelay(delay);
    true
}

unsafe fn stop_sync_sw(ctx: *mut dc_context, ddc: *mut ddc, delay: u32) -> bool {
    let mut retry = 0;
    write_bit_to_ddc(ddc, SCL, false); udelay(delay); write_bit_to_ddc(ddc, SDA, false); udelay(delay);
    write_bit_to_ddc(ddc, SCL, true); if !wait_for_scl_high_sw(ctx, ddc, delay) { return false; }
    write_bit_to_ddc(ddc, SDA, true);
    loop { udelay(delay); if read_bit_from_ddc(ddc, SDA) { return true; } retry += 1; if retry > 2 { break; } }
    false
}

unsafe fn i2c_write_sw(ctx: *mut dc_context, ddc: *mut ddc, delay: u32, address: u8, length: u32, data: *const u8) -> bool {
    if !write_byte_sw(ctx, ddc, delay, address) { return false; }
    let mut i = 0; while i < length { if !write_byte_sw(ctx, ddc, delay, *data.add(i as usize)) { return false; } i += 1; }
    true
}

unsafe fn i2c_read_sw(ctx: *mut dc_context, ddc: *mut ddc, delay: u32, address: u8, length: u32, data: *mut u8) -> bool {
    if !write_byte_sw(ctx, ddc, delay, address) { return false; }
    let mut i = 0; while i < length { if !read_byte_sw(ctx, ddc, delay, data.add(i as usize), i < length - 1) { return false; } i += 1; }
    true
}

unsafe fn start_sync_sw(ctx: *mut dc_context, ddc: *mut ddc, delay: u32) -> bool {
    let mut retry = 0; write_bit_to_ddc(ddc, SCL, true); udelay(delay);
    loop {
        write_bit_to_ddc(ddc, SDA, true);
        if !read_bit_from_ddc(ddc, SDA) { retry += 1; if retry > I2C_SW_RETRIES { break; } continue; }
        udelay(delay); write_bit_to_ddc(ddc, SCL, true);
        if !wait_for_scl_high_sw(ctx, ddc, delay) { break; }
        write_bit_to_ddc(ddc, SDA, false); udelay(delay); write_bit_to_ddc(ddc, SCL, false); udelay(delay); return true;
    }
    false
}

unsafe fn dce_i2c_sw_engine_set_speed(engine: *mut dce_i2c_sw, speed: u32) {
    ASSERT(speed != 0); (*engine).speed = if speed != 0 { speed } else { DCE_I2C_DEFAULT_I2C_SW_SPEED };
    (*engine).clock_delay = 1000 / (*engine).speed; if (*engine).clock_delay < 12 { (*engine).clock_delay = 12; }
}

unsafe fn dce_i2c_sw_engine_acquire_engine(engine: *mut dce_i2c_sw, ddc: *mut ddc) -> bool {
    if dal_ddc_open(ddc, GPIO_MODE_FAST_OUTPUT, GPIO_DDC_CONFIG_TYPE_MODE_I2C) != GPIO_RESULT_OK { return false; }
    (*engine).ddc = ddc; true
}

pub unsafe fn dce_i2c_engine_acquire_sw(engine: *mut dce_i2c_sw, ddc: *mut ddc) -> bool {
    let mut counter = 0; loop { let result = dce_i2c_sw_engine_acquire_engine(engine, ddc); if result { return true; } udelay(10); counter += 1; if counter >= 2 { return false; } }
}

unsafe fn dce_i2c_sw_engine_submit_channel_request(engine: *mut dce_i2c_sw, req: *mut i2c_request_transaction_data) {
    let ddc = (*engine).ddc; let delay = (*engine).clock_delay >> 2; let mut result = start_sync_sw((*engine).ctx, ddc, delay);
    if result { result = match (*req).action {
        DCE_I2C_TRANSACTION_ACTION_I2C_WRITE | DCE_I2C_TRANSACTION_ACTION_I2C_WRITE_MOT => i2c_write_sw((*engine).ctx, ddc, delay, (*req).address, (*req).length, (*req).data as *const u8),
        DCE_I2C_TRANSACTION_ACTION_I2C_READ | DCE_I2C_TRANSACTION_ACTION_I2C_READ_MOT => i2c_read_sw((*engine).ctx, ddc, delay, (*req).address, (*req).length, (*req).data),
        _ => false,
    }; }
    if !result || (*req).action == DCE_I2C_TRANSACTION_ACTION_I2C_WRITE || (*req).action == DCE_I2C_TRANSACTION_ACTION_I2C_READ { if !stop_sync_sw((*engine).ctx, ddc, delay) { result = false; } }
    (*req).status = if result { I2C_CHANNEL_OPERATION_SUCCEEDED } else { I2C_CHANNEL_OPERATION_FAILED };
}

unsafe fn dce_i2c_sw_engine_submit_payload(engine: *mut dce_i2c_sw, payload: *mut i2c_payload, middle: bool) -> bool {
    let mut request = i2c_request_transaction_data { action: if (*payload).write { if middle { DCE_I2C_TRANSACTION_ACTION_I2C_WRITE_MOT } else { DCE_I2C_TRANSACTION_ACTION_I2C_WRITE } } else { if middle { DCE_I2C_TRANSACTION_ACTION_I2C_READ_MOT } else { DCE_I2C_TRANSACTION_ACTION_I2C_READ } }, address: (((*payload).address << 1) | if (*payload).write { 0 } else { 1 }) as u8, length: (*payload).length, data: (*payload).data, status: 0 };
    dce_i2c_sw_engine_submit_channel_request(engine, &mut request);
    request.status != I2C_CHANNEL_OPERATION_ENGINE_BUSY && request.status != I2C_CHANNEL_OPERATION_FAILED
}

pub unsafe fn dce_i2c_submit_command_sw(pool: *mut resource_pool, _ddc: *mut ddc, cmd: *mut i2c_command, engine: *mut dce_i2c_sw) -> bool {
    dce_i2c_sw_engine_set_speed(engine, (*cmd).speed); let mut result = true; let mut index = 0u8;
    while index < (*cmd).number_of_payloads { let mot = index != (*cmd).number_of_payloads - 1; let payload = (*cmd).payloads.add(index as usize); if !dce_i2c_sw_engine_submit_payload(engine, payload, mot) { result = false; break; } index += 1; }
    release_engine_dce_sw(pool, engine); result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
