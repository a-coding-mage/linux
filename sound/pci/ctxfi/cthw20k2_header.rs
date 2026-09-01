// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	cthw20k2.h
 *
 * @Brief
 * This file contains the definition of hardware access methord.
 *
 * @Author	Liu Chun
 * @Date 	May 13 2008
 */

// C dependency: #include "cthardware.h"

unsafe extern "C" {
    pub fn create_20k2_hw_obj(rhw: *mut *mut hw) -> ::core::ffi::c_int;
    pub fn destroy_20k2_hw_obj(hw: *mut hw) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
