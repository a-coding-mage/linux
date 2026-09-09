/*
 * Copyright 2015 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency supplied by the corresponding shared header: uint16_t, uint32_t,
// uint64_t, bool, and related C definitions.
use core::ffi::{c_uint, c_ulong, c_void};

/** enum cgs_ind_reg - Indirect register spaces */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum cgs_ind_reg {
    CGS_IND_REG__PCIE,
    CGS_IND_REG__SMC,
    CGS_IND_REG__UVD_CTX,
    CGS_IND_REG__DIDT,
    CGS_IND_REG_GC_CAC,
    CGS_IND_REG_SE_CAC,
    CGS_IND_REG__AUDIO_ENDPT,
}

/* enum cgs_ucode_id - Firmware types for different IPs */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum cgs_ucode_id {
    CGS_UCODE_ID_SMU = 0,
    CGS_UCODE_ID_SMU_SK,
    CGS_UCODE_ID_SDMA0,
    CGS_UCODE_ID_SDMA1,
    CGS_UCODE_ID_CP_CE,
    CGS_UCODE_ID_CP_PFP,
    CGS_UCODE_ID_CP_ME,
    CGS_UCODE_ID_CP_MEC,
    CGS_UCODE_ID_CP_MEC_JT1,
    CGS_UCODE_ID_CP_MEC_JT2,
    CGS_UCODE_ID_GMCON_RENG,
    CGS_UCODE_ID_RLC_G,
    CGS_UCODE_ID_STORAGE,
    CGS_UCODE_ID_MAXIMUM,
}

/** struct cgs_firmware_info - Firmware information */
#[repr(C)]
pub struct cgs_firmware_info {
    pub version: u16,
    pub fw_version: u16,
    pub feature_version: u16,
    pub image_size: u32,
    pub mc_addr: u64,
    /* only for smc firmware */
    pub ucode_start_address: u32,
    pub kptr: *mut c_void,
    pub is_kicker: bool,
}

pub type cgs_handle_t = c_ulong;

pub type cgs_read_register_t = unsafe extern "C" fn(*mut cgs_device, c_uint) -> u32;
pub type cgs_write_register_t = unsafe extern "C" fn(*mut cgs_device, c_uint, u32);
pub type cgs_read_ind_register_t = unsafe extern "C" fn(*mut cgs_device, cgs_ind_reg, c_uint) -> u32;
pub type cgs_write_ind_register_t = unsafe extern "C" fn(*mut cgs_device, cgs_ind_reg, c_uint, u32);

#[macro_export]
macro_rules! CGS_REG_FIELD_SHIFT { ($reg:ident, $field:ident) => { $reg::__##$field##__SHIFT }; }
#[macro_export]
macro_rules! CGS_REG_FIELD_MASK { ($reg:ident, $field:ident) => { $reg::__##$field##_MASK }; }

#[macro_export]
macro_rules! CGS_REG_SET_FIELD {
    ($orig_val:expr, $reg:ident, $field:ident, $field_val:expr) => {
        (($orig_val & !CGS_REG_FIELD_MASK!($reg, $field)) |
            (CGS_REG_FIELD_MASK!($reg, $field) & ($field_val << CGS_REG_FIELD_SHIFT!($reg, $field))))
    };
}

#[macro_export]
macro_rules! CGS_REG_GET_FIELD {
    ($value:expr, $reg:ident, $field:ident) => {
        (($value & CGS_REG_FIELD_MASK!($reg, $field)) >> CGS_REG_FIELD_SHIFT!($reg, $field))
    };
}

pub type cgs_get_firmware_info = unsafe extern "C" fn(*mut cgs_device, cgs_ucode_id, *mut cgs_firmware_info) -> i32;

#[repr(C)]
pub struct cgs_ops {
    /* MMIO access */
    pub read_register: cgs_read_register_t,
    pub write_register: cgs_write_register_t,
    pub read_ind_register: cgs_read_ind_register_t,
    pub write_ind_register: cgs_write_ind_register_t,
    /* Firmware Info */
    pub get_firmware_info: cgs_get_firmware_info,
}

pub struct cgs_os_ops; /* To be define in OS-specific CGS header */

#[repr(C)]
pub struct cgs_device {
    pub ops: *const cgs_ops,
    /* to be embedded at the start of driver private structure */
}

/* Convenience macros that make CGS indirect function calls look like
 * normal function calls */
#[macro_export]
macro_rules! CGS_CALL {
    ($func:ident, $dev:expr $(, $arg:expr)*) => {
        unsafe { ((*(*($dev as *mut $crate::cgs_device)).ops).$func)($dev $(, $arg)*) }
    };
}

#[macro_export]
macro_rules! CGS_OS_CALL {
    ($func:ident, $dev:expr $(, $arg:expr)*) => { compile_error!("cgs_device OS operations are supplied by the OS-specific CGS header") };
}

#[macro_export]
macro_rules! cgs_read_register { ($dev:expr, $offset:expr) => { CGS_CALL!(read_register, $dev, $offset) }; }
#[macro_export]
macro_rules! cgs_write_register { ($dev:expr, $offset:expr, $value:expr) => { CGS_CALL!(write_register, $dev, $offset, $value) }; }
#[macro_export]
macro_rules! cgs_read_ind_register { ($dev:expr, $space:expr, $index:expr) => { CGS_CALL!(read_ind_register, $dev, $space, $index) }; }
#[macro_export]
macro_rules! cgs_write_ind_register { ($dev:expr, $space:expr, $index:expr, $value:expr) => { CGS_CALL!(write_ind_register, $dev, $space, $index, $value) }; }
#[macro_export]
macro_rules! cgs_get_firmware_info { ($dev:expr, $type:expr, $info:expr) => { CGS_CALL!(get_firmware_info, $dev, $type, $info) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
