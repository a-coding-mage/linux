// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	cthw20k1.h
 *
 * @Brief
 * This file contains the definition of hardware access methord.
 *
 * @Author	Liu Chun
 * @Date 	May 13 2008
 */

// Depends on declarations from "cthardware.h".

extern "C" {
    pub fn create_20k1_hw_obj(rhw: *mut *mut hw) -> ::std::os::raw::c_int;
    pub fn destroy_20k1_hw_obj(hw: *mut hw) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
