/*
 * Public header for the MPC52xx processor BestComm driver
 *
 * Copyright (C) 2006      Sylvain Munaut <tnt@246tNt.com>
 * Copyright (C) 2005      Varma Electronics Oy,
 *                         ( by Andrey Volkov <avolkov@varma-el.com> )
 * Copyright (C) 2003-2004 MontaVista, Software, Inc.
 *                         ( by Dale Farnsworth <dfarnsworth@mvista.com> )
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

#[repr(C)]
pub struct bcom_bd {
    pub status: u32,
    pub data: [u32; 0],
}

#[repr(C)]
pub struct bcom_task {
    pub tasknum: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
    pub irq: ::core::ffi::c_int,
    pub bd: *mut bcom_bd,
    pub bd_pa: phys_addr_t,
    pub cookie: *mut *mut ::core::ffi::c_void,
    pub index: u16,
    pub outdex: u16,
    pub num_bd: ::core::ffi::c_uint,
    pub bd_size: ::core::ffi::c_uint,
    pub priv_: *mut ::core::ffi::c_void,
}

pub const BCOM_FLAGS_NONE: ::core::ffi::c_ulong = 0x00000000;
pub const BCOM_FLAGS_ENABLE_TASK: ::core::ffi::c_ulong = 1 << 0;

extern "C" {
    pub fn bcom_enable(tsk: *mut bcom_task);
    pub fn bcom_disable(tsk: *mut bcom_task);
    pub fn mb();
}

#[inline]
pub unsafe fn bcom_get_task_irq(tsk: *mut bcom_task) -> ::core::ffi::c_int {
    (*tsk).irq
}

pub const BCOM_BD_READY: ::core::ffi::c_ulong = 0x40000000;

#[inline]
pub unsafe fn _bcom_next_index(tsk: *mut bcom_task) -> ::core::ffi::c_int {
    if ((*tsk).index as ::core::ffi::c_uint + 1) == (*tsk).num_bd {
        0
    } else {
        (*tsk).index as ::core::ffi::c_int + 1
    }
}

#[inline]
pub unsafe fn _bcom_next_outdex(tsk: *mut bcom_task) -> ::core::ffi::c_int {
    if ((*tsk).outdex as ::core::ffi::c_uint + 1) == (*tsk).num_bd {
        0
    } else {
        (*tsk).outdex as ::core::ffi::c_int + 1
    }
}

#[inline]
pub unsafe fn bcom_queue_empty(tsk: *mut bcom_task) -> ::core::ffi::c_int {
    ((*tsk).index == (*tsk).outdex) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn bcom_queue_full(tsk: *mut bcom_task) -> ::core::ffi::c_int {
    ((*tsk).outdex as ::core::ffi::c_int == _bcom_next_index(tsk)) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn bcom_get_bd(tsk: *mut bcom_task, index: ::core::ffi::c_uint) -> *mut bcom_bd {
    ((*tsk).bd as *mut u8).add((index * (*tsk).bd_size) as usize) as *mut bcom_bd
}

#[inline]
pub unsafe fn bcom_buffer_done(tsk: *mut bcom_task) -> ::core::ffi::c_int {
    if bcom_queue_empty(tsk) != 0 {
        return 0;
    }
    let bd = bcom_get_bd(tsk, (*tsk).outdex as ::core::ffi::c_uint);
    ((!((*bd).status & BCOM_BD_READY as u32) != 0)) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn bcom_prepare_next_buffer(tsk: *mut bcom_task) -> *mut bcom_bd {
    let bd = bcom_get_bd(tsk, (*tsk).index as ::core::ffi::c_uint);
    (*bd).status = 0;
    bd
}

#[inline]
pub unsafe fn bcom_submit_next_buffer(tsk: *mut bcom_task, cookie: *mut ::core::ffi::c_void) {
    let bd = bcom_get_bd(tsk, (*tsk).index as ::core::ffi::c_uint);
    *(*tsk).cookie.add((*tsk).index as usize) = cookie;
    mb();
    (*bd).status |= BCOM_BD_READY as u32;
    (*tsk).index = _bcom_next_index(tsk) as u16;
    if ((*tsk).flags as ::core::ffi::c_ulong & BCOM_FLAGS_ENABLE_TASK) != 0 {
        bcom_enable(tsk);
    }
}

#[inline]
pub unsafe fn bcom_retrieve_buffer(
    tsk: *mut bcom_task,
    p_status: *mut u32,
    p_bd: *mut *mut bcom_bd,
) -> *mut ::core::ffi::c_void {
    let cookie = *(*tsk).cookie.add((*tsk).outdex as usize);
    let bd = bcom_get_bd(tsk, (*tsk).outdex as ::core::ffi::c_uint);
    if !p_status.is_null() {
        *p_status = (*bd).status;
    }
    if !p_bd.is_null() {
        *p_bd = bd;
    }
    (*tsk).outdex = _bcom_next_outdex(tsk) as u16;
    cookie
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
