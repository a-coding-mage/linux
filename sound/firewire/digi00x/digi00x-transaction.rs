// SPDX-License-Identifier: GPL-2.0-only
/*
 * digi00x-transaction.c - a part of driver for Digidesign Digi 002/003 family
 *
 * Copyright (c) 2014-2015 Takashi Sakamoto
 */

// C dependencies:
// #include <sound/asound.h>
// #include "digi00x.h"

use core::ffi::{c_int, c_void};

type SizeT = usize;
type Be32 = u32;

const RCODE_COMPLETE: c_int = 0;
const TCODE_WRITE_BLOCK_REQUEST: c_int = 0;
const DG00X_ADDR_BASE: u64 = 0;
const DG00X_OFFSET_MESSAGE_ADDR: u64 = 0;

#[repr(C)]
pub struct snd_dg00x {
    pub lock: spinlock_t,
    pub msg: u32,
    pub hwdep_wait: wait_queue_head_t,
    pub unit: *mut fw_unit,
    pub async_handler: fw_address_handler,
}

#[repr(C)]
pub struct fw_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_device {
    pub card: *mut fw_card_public,
}

#[repr(C)]
pub struct fw_card_public {
    pub node_id: c_int,
}

#[repr(C)]
pub struct fw_address_handler {
    pub offset: u64,
    pub length: SizeT,
    pub address_callback: Option<
        unsafe extern "C" fn(
            *mut fw_card,
            *mut fw_request,
            c_int,
            c_int,
            c_int,
            c_int,
            u64,
            *mut c_void,
            SizeT,
            *mut c_void,
        ),
    >,
    pub callback_data: *mut c_void,
}

#[repr(C)]
pub struct fw_address_region {
    pub start: u64,
    pub end: u64,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn be32_to_cpu(value: Be32) -> u32;
    fn cpu_to_be32(value: u32) -> Be32;
    fn fw_send_response(card: *mut fw_card, request: *mut fw_request, rcode: c_int);
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn snd_fw_transaction(
        unit: *mut fw_unit,
        tcode: c_int,
        offset: u64,
        data: *const c_void,
        length: SizeT,
        flags: c_int,
    ) -> c_int;
    fn fw_core_remove_address_handler(handler: *mut fw_address_handler);
    fn fw_core_add_address_handler(
        handler: *mut fw_address_handler,
        region: *const fw_address_region,
    ) -> c_int;
    fn wake_up(wait: *mut wait_queue_head_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> usize;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
}

unsafe fn handle_unknown_message(dg00x: *mut snd_dg00x, _offset: u64, buf: *mut Be32) {
    let flags = unsafe { spin_lock_irqsave(core::ptr::addr_of_mut!((*dg00x).lock)) };
    unsafe {
        (*dg00x).msg = be32_to_cpu(*buf);
    }
    unsafe {
        spin_unlock_irqrestore(core::ptr::addr_of_mut!((*dg00x).lock), flags);
    }

    unsafe {
        wake_up(core::ptr::addr_of_mut!((*dg00x).hwdep_wait));
    }
}

unsafe extern "C" fn handle_message(
    card: *mut fw_card,
    request: *mut fw_request,
    _tcode: c_int,
    _destination: c_int,
    _source: c_int,
    _generation: c_int,
    offset: u64,
    data: *mut c_void,
    _length: SizeT,
    callback_data: *mut c_void,
) {
    let dg00x = callback_data as *mut snd_dg00x;
    let buf = data as *mut Be32;

    unsafe {
        fw_send_response(card, request, RCODE_COMPLETE);
    }

    if unsafe { offset == (*dg00x).async_handler.offset } {
        unsafe {
            handle_unknown_message(dg00x, offset, buf);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dg00x_transaction_reregister(dg00x: *mut snd_dg00x) -> c_int {
    let device = unsafe { fw_parent_device((*dg00x).unit) };
    let mut data: [Be32; 2] = [0; 2];

    /* Unknown. 4bytes. */
    data[0] = unsafe {
        cpu_to_be32(
            (((*(*device).card).node_id as u32) << 16)
                | (((*dg00x).async_handler.offset >> 32) as u32),
        )
    };
    data[1] = unsafe { cpu_to_be32((*dg00x).async_handler.offset as u32) };
    unsafe {
        snd_fw_transaction(
            (*dg00x).unit,
            TCODE_WRITE_BLOCK_REQUEST,
            DG00X_ADDR_BASE + DG00X_OFFSET_MESSAGE_ADDR,
            data.as_ptr() as *const c_void,
            core::mem::size_of_val(&data),
            0,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dg00x_transaction_unregister(dg00x: *mut snd_dg00x) {
    if unsafe { (*dg00x).async_handler.callback_data.is_null() } {
        return;
    }

    unsafe {
        fw_core_remove_address_handler(core::ptr::addr_of_mut!((*dg00x).async_handler));
    }

    unsafe {
        (*dg00x).async_handler.callback_data = core::ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dg00x_transaction_register(dg00x: *mut snd_dg00x) -> c_int {
    static RESP_REGISTER_REGION: fw_address_region = fw_address_region {
        start: 0xffffe0000000_u64,
        end: 0xffffe000ffff_u64,
    };
    let mut err: c_int;

    unsafe {
        (*dg00x).async_handler.length = 4;
        (*dg00x).async_handler.address_callback = Some(handle_message);
        (*dg00x).async_handler.callback_data = dg00x as *mut c_void;
    }

    err = unsafe {
        fw_core_add_address_handler(
            core::ptr::addr_of_mut!((*dg00x).async_handler),
            core::ptr::addr_of!(RESP_REGISTER_REGION),
        )
    };
    if err < 0 {
        return err;
    }

    err = unsafe { snd_dg00x_transaction_reregister(dg00x) };
    if err < 0 {
        unsafe {
            snd_dg00x_transaction_unregister(dg00x);
        }
    }

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
