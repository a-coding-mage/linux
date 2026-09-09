// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependency equivalent of C's `#include "dio.h"`.

// #define TO_DCN10_DIO(dio_base) \
// 	container_of(dio_base, struct dcn10_dio, base)
//
// The C container_of operation is represented as a raw-pointer cast. The
// caller must provide a pointer to the embedded `base` field.
#[inline]
pub unsafe fn to_dcn10_dio(dio_base: *mut dio) -> *mut dcn10_dio {
    dio_base as *mut dcn10_dio
}

// #define DIO_REG_LIST_DCN10()\
// 	SR(DIO_MEM_PWR_CTRL)
//
// `SR` is supplied by the including/build environment.
#[macro_export]
macro_rules! DIO_REG_LIST_DCN10 {
    () => {
        SR!(DIO_MEM_PWR_CTRL)
    };
}

#[repr(C)]
pub struct dcn_dio_registers {
    pub DIO_MEM_PWR_CTRL: u32,
}

#[repr(C)]
pub struct dcn_dio_shift {
    pub I2C_LIGHT_SLEEP_FORCE: u8,
}

#[repr(C)]
pub struct dcn_dio_mask {
    pub I2C_LIGHT_SLEEP_FORCE: u32,
}

#[repr(C)]
pub struct dcn10_dio {
    pub base: dio,
    pub regs: *const dcn_dio_registers,
    pub shifts: *const dcn_dio_shift,
    pub masks: *const dcn_dio_mask,
}

extern "C" {
    pub fn dcn10_dio_construct(
        dio10: *mut dcn10_dio,
        ctx: *mut dc_context,
        regs: *const dcn_dio_registers,
        shifts: *const dcn_dio_shift,
        masks: *const dcn_dio_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
