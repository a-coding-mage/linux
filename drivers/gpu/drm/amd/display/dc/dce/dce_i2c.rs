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
 *
 */

use core::ffi::c_void;

#[repr(C)]
pub struct resource_pool { _private: [u8; 0] }
#[repr(C)]
pub struct ddc_service { pub ctx: *mut dc_context }
#[repr(C)]
pub struct ddc { pub ctx: *mut dc_context }
#[repr(C)]
pub struct i2c_command { _private: [u8; 0] }
#[repr(C)]
pub struct dce_i2c_hw { _private: [u8; 0] }
#[repr(C)]
pub struct dce_i2c_sw { pub ctx: *mut dc_context }

#[repr(C)]
pub struct dc_context { pub dc: *mut dc, pub dc_bios: *mut dc_bios }
#[repr(C)]
pub struct dc { pub ctx: *mut dc_context }
#[repr(C)]
pub struct dc_bios {
    pub fw_info: firmware_info,
    pub funcs: *mut dc_bios_funcs,
}
#[repr(C)]
pub struct firmware_info {
    pub oem_i2c_present: bool,
    pub oem_i2c_obj_id: u32,
}
#[repr(C)]
pub struct dc_bios_funcs {
    pub get_i2c_info: Option<unsafe extern "C" fn(*mut dc_bios, graphics_object_id, *mut graphics_object_i2c_info) -> bp_result>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphics_object_id {
    pub id: u32,
    pub enum_id: u32,
    pub type_: u32,
}
#[repr(C)]
pub struct graphics_object_i2c_info { pub i2c_slave_address: usize }
#[repr(C)]
pub struct dc_bios_holder { pub dc_bios: *mut dc_bios }

pub type bp_result = i32;
pub const BP_RESULT_OK: bp_result = 0;
pub const OBJECT_TYPE_GENERIC: u32 = 0;

extern "C" {
    fn acquire_i2c_hw_engine(pool: *mut resource_pool, ddc: *mut ddc) -> *mut dce_i2c_hw;
    fn dce_i2c_submit_command_hw(
        pool: *mut resource_pool,
        ddc: *mut ddc,
        cmd: *mut i2c_command,
        hw: *mut dce_i2c_hw,
    ) -> bool;
    fn dce_i2c_engine_acquire_sw(sw: *mut dce_i2c_sw, ddc: *mut ddc) -> bool;
    fn dce_i2c_submit_command_sw(
        pool: *mut resource_pool,
        ddc: *mut ddc,
        cmd: *mut i2c_command,
        sw: *mut dce_i2c_sw,
    ) -> bool;
    fn BREAK_TO_DEBUGGER();
}

pub unsafe extern "C" fn dce_i2c_oem_device_present(
    pool: *mut resource_pool,
    ddc: *mut ddc_service,
    slave_address: usize,
) -> bool {
    let _ = pool;
    let dc = (*(*ddc).ctx).dc;
    let dcb = (*(*dc).ctx).dc_bios;
    let mut id = graphics_object_id { id: 0, enum_id: 0, type_: 0 };
    let mut i2c_info = core::mem::MaybeUninit::<graphics_object_i2c_info>::uninit();

    if !(*dcb).fw_info.oem_i2c_present {
        return false;
    }

    id.id = (*dcb).fw_info.oem_i2c_obj_id;
    id.enum_id = 0;
    id.type_ = OBJECT_TYPE_GENERIC;
    let get_i2c_info = (*(*dcb).funcs).get_i2c_info.unwrap();
    if get_i2c_info(dcb, id, i2c_info.as_mut_ptr()) != BP_RESULT_OK {
        return false;
    }

    if (*i2c_info.as_ptr()).i2c_slave_address != slave_address {
        return false;
    }

    true
}

pub unsafe extern "C" fn dce_i2c_submit_command(
    pool: *mut resource_pool,
    ddc: *mut ddc,
    cmd: *mut i2c_command,
) -> bool {
    let dce_i2c_hw: *mut dce_i2c_hw;
    let mut dce_i2c_sw = dce_i2c_sw { ctx: core::ptr::null_mut() };

    if ddc.is_null() {
        BREAK_TO_DEBUGGER();
        return false;
    }

    if cmd.is_null() {
        BREAK_TO_DEBUGGER();
        return false;
    }

    dce_i2c_hw = acquire_i2c_hw_engine(pool, ddc);

    if !dce_i2c_hw.is_null() {
        return dce_i2c_submit_command_hw(pool, ddc, cmd, dce_i2c_hw);
    }

    dce_i2c_sw.ctx = (*ddc).ctx;
    if dce_i2c_engine_acquire_sw(&mut dce_i2c_sw, ddc) {
        return dce_i2c_submit_command_sw(pool, ddc, cmd, &mut dce_i2c_sw);
    }

    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
