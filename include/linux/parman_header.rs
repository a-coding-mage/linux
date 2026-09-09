/*
 * include/linux/parman.h - Manager for linear priority array areas
 * Copyright (c) 2017 Mellanox Technologies. All rights reserved.
 * Copyright (c) 2017 Jiri Pirko <jiri@mellanox.com>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the names of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed under the terms of the
 * GNU General Public License ("GPL") version 2 as published by the Free
 * Software Foundation.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

use std::os::raw::{c_int, c_ulong, c_void};

// Supplied by the Linux list header dependency.
use crate::list_head;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum parman_algo_type {
    PARMAN_ALGO_TYPE_LSORT,
}

#[repr(C)]
pub struct parman_item {
    pub list: list_head,
    pub index: c_ulong,
}

#[repr(C)]
pub struct parman_prio {
    pub list: list_head,
    pub item_list: list_head,
    pub priority: c_ulong,
}

#[repr(C)]
pub struct parman_ops {
    pub base_count: c_ulong,
    pub resize_step: c_ulong,
    pub resize: Option<unsafe extern "C" fn(priv_: *mut c_void, new_count: c_ulong) -> c_int>,
    pub move_: Option<unsafe extern "C" fn(
        priv_: *mut c_void,
        from_index: c_ulong,
        to_index: c_ulong,
        count: c_ulong,
    )>,
    pub algo: parman_algo_type,
}

#[repr(C)]
pub struct parman {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn parman_create(ops: *const parman_ops, priv_: *mut c_void) -> *mut parman;
    pub fn parman_destroy(parman: *mut parman);
    pub fn parman_prio_init(
        parman: *mut parman,
        prio: *mut parman_prio,
        priority: c_ulong,
    );
    pub fn parman_prio_fini(prio: *mut parman_prio);
    pub fn parman_item_add(
        parman: *mut parman,
        prio: *mut parman_prio,
        item: *mut parman_item,
    ) -> c_int;
    pub fn parman_item_remove(
        parman: *mut parman,
        prio: *mut parman_prio,
        item: *mut parman_item,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
