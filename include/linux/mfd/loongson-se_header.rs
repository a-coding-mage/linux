/* SPDX-License-Identifier: GPL-2.0+ */
/* Copyright (C) 2025 Loongson Technology Corporation Limited */

// C header guard: __MFD_LOONGSON_SE_H__

pub const LOONGSON_ENGINE_CMD_TIMEOUT_US: u32 = 10000;
pub const SE_SEND_CMD_REG: u32 = 0x0;
pub const SE_SEND_CMD_REG_LEN: u32 = 0x8;
// Controller command ID
pub const SE_CMD_START: u32 = 0x0;
pub const SE_CMD_SET_DMA: u32 = 0x3;
pub const SE_CMD_SET_ENGINE_CMDBUF: u32 = 0x4;

pub const SE_S2LINT_STAT: u32 = 0x88;
pub const SE_S2LINT_EN: u32 = 0x8c;
pub const SE_S2LINT_CL: u32 = 0x94;
pub const SE_L2SINT_STAT: u32 = 0x98;
pub const SE_L2SINT_SET: u32 = 0xa0;

pub const SE_INT_ALL: u32 = 0xffff_ffff;
pub const SE_INT_CONTROLLER: u32 = 1u32 << 0;

pub const SE_ENGINE_MAX: u32 = 16;
pub const SE_ENGINE_RNG: u32 = 1;
pub const SE_CMD_RNG: u32 = 0x100;

pub const SE_ENGINE_TPM: u32 = 5;
pub const SE_CMD_TPM: u32 = 0x500;

pub const SE_ENGINE_CMD_SIZE: u32 = 32;

#[repr(C)]
pub struct loongson_se_engine {
    pub se: *mut loongson_se,
    pub id: core::ffi::c_int,

    /* Command buffer */
    pub command: *mut core::ffi::c_void,
    pub command_ret: *mut core::ffi::c_void,

    pub data_buffer: *mut core::ffi::c_void,
    pub buffer_size: u32,
    /* Data buffer offset to DMA base */
    pub buffer_off: u32,

    pub completion: completion,
}

// External types supplied by other translation units.
#[repr(C)]
pub struct loongson_se {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

extern "C" {
    pub fn loongson_se_init_engine(dev: *mut device, id: core::ffi::c_int)
        -> *mut loongson_se_engine;
    pub fn loongson_se_send_engine_cmd(engine: *mut loongson_se_engine) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
