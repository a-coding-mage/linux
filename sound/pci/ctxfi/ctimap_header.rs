/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctimap.h
 *
 * @Brief
 * This file contains the definition of generic input mapper operations
 * for input mapper management.
 *
 * @Author	Liu Chun
 * @Date 	May 23 2008
 */

use core::ffi::{c_int, c_void};

/* Dependency intent from C: #include <linux/list.h> supplies list_head. */
use crate::list_head;

#[repr(C)]
pub struct imapper {
    pub slot: u16, /* the id of the slot containing input data */
    pub user: u16, /* the id of the user resource consuming data */
    pub addr: u16, /* the input mapper ram id */
    pub next: u16, /* the next input mapper ram id */
    pub list: list_head,
}

pub type imapper_map_op = Option<unsafe extern "C" fn(*mut c_void, *mut imapper) -> c_int>;

unsafe extern "C" {
    pub fn input_mapper_add(
        mappers: *mut list_head,
        entry: *mut imapper,
        map_op: imapper_map_op,
        data: *mut c_void,
    ) -> c_int;

    pub fn input_mapper_delete(
        mappers: *mut list_head,
        entry: *mut imapper,
        map_op: imapper_map_op,
        data: *mut c_void,
    ) -> c_int;

    pub fn free_input_mapper_list(mappers: *mut list_head);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
