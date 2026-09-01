// SPDX-License-Identifier: GPL-2.0-only
/*
 * motu-transaction.c - a part of driver for MOTU FireWire series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// C dependency intent: translated from `#include "motu.h"`.

use core::ffi::c_void;

type u32 = u32;
type __be32 = u32;
type size_t = usize;

const SND_MOTU_ADDR_BASE: u64 = 0xfffff0000000u64;
const ASYNC_ADDR_HI: u32 = 0x0b04;
const ASYNC_ADDR_LO: u32 = 0x0b08;

const EINVAL: i32 = 22;

// External constants supplied by the FireWire/kernel dependencies.
extern "C" {
    static TCODE_READ_QUADLET_REQUEST: i32;
    static TCODE_READ_BLOCK_REQUEST: i32;
    static TCODE_WRITE_QUADLET_REQUEST: i32;
    static TCODE_WRITE_BLOCK_REQUEST: i32;
    static RCODE_COMPLETE: i32;
    static RCODE_ADDRESS_ERROR: i32;
}

// External types and fields supplied by `motu.h` and FireWire/kernel headers.
#[repr(C)]
pub struct snd_motu {
    pub unit: *mut fw_unit,
    pub async_handler: fw_address_handler,
    pub lock: spinlock_t,
    pub msg: u32,
    pub hwdep_wait: wait_queue_head_t,
}

#[repr(C)]
pub struct fw_address_handler {
    pub offset: u64,
    pub length: size_t,
    pub address_callback: Option<
        unsafe extern "C" fn(
            *mut fw_card,
            *mut fw_request,
            i32,
            i32,
            i32,
            i32,
            u64,
            *mut c_void,
            size_t,
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
pub struct fw_device {
    pub card: *mut fw_card,
}

#[repr(C)]
pub struct fw_card {
    pub node_id: i32,
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

extern "C" {
    fn snd_fw_transaction(
        unit: *mut fw_unit,
        tcode: i32,
        offset: u64,
        data: *mut __be32,
        size: size_t,
        flags: i32,
    ) -> i32;

    fn fw_send_response(card: *mut fw_card, request: *mut fw_request, rcode: i32);
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn fw_core_add_address_handler(
        handler: *mut fw_address_handler,
        region: *const fw_address_region,
    ) -> i32;
    fn fw_core_remove_address_handler(handler: *mut fw_address_handler);
    fn wake_up(wait: *mut wait_queue_head_t);

    fn spin_lock_irqsave(lock: *mut spinlock_t) -> ::core::ffi::c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: ::core::ffi::c_ulong);
}

#[inline]
fn be32_to_cpu(value: __be32) -> u32 {
    u32::from_be(value)
}

#[inline]
fn cpu_to_be32(value: u32) -> __be32 {
    value.to_be()
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_transaction_read(
    motu: *mut snd_motu,
    offset: u32,
    reg: *mut __be32,
    size: size_t,
) -> i32 {
    let tcode: i32;

    if size % core::mem::size_of::<__be32>() > 0 || size <= 0 {
        return -EINVAL;
    }
    if size == core::mem::size_of::<__be32>() {
        tcode = TCODE_READ_QUADLET_REQUEST;
    } else {
        tcode = TCODE_READ_BLOCK_REQUEST;
    }

    snd_fw_transaction(
        (*motu).unit,
        tcode,
        SND_MOTU_ADDR_BASE + offset as u64,
        reg,
        size,
        0,
    )
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_transaction_write(
    motu: *mut snd_motu,
    offset: u32,
    reg: *mut __be32,
    size: size_t,
) -> i32 {
    let tcode: i32;

    if size % core::mem::size_of::<__be32>() > 0 || size <= 0 {
        return -EINVAL;
    }
    if size == core::mem::size_of::<__be32>() {
        tcode = TCODE_WRITE_QUADLET_REQUEST;
    } else {
        tcode = TCODE_WRITE_BLOCK_REQUEST;
    }

    snd_fw_transaction(
        (*motu).unit,
        tcode,
        SND_MOTU_ADDR_BASE + offset as u64,
        reg,
        size,
        0,
    )
}

unsafe extern "C" fn handle_message(
    card: *mut fw_card,
    request: *mut fw_request,
    tcode: i32,
    _destination: i32,
    _source: i32,
    _generation: i32,
    offset: u64,
    data: *mut c_void,
    length: size_t,
    callback_data: *mut c_void,
) {
    let motu = callback_data as *mut snd_motu;
    let buf = data as *mut __be32;

    if tcode != TCODE_WRITE_QUADLET_REQUEST {
        fw_send_response(card, request, RCODE_COMPLETE);
        return;
    }

    if offset != (*motu).async_handler.offset || length != 4 {
        fw_send_response(card, request, RCODE_ADDRESS_ERROR);
        return;
    }

    {
        let flags = spin_lock_irqsave(&mut (*motu).lock);
        (*motu).msg = be32_to_cpu(*buf);
        spin_unlock_irqrestore(&mut (*motu).lock, flags);
    }

    fw_send_response(card, request, RCODE_COMPLETE);

    wake_up(&mut (*motu).hwdep_wait);
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_transaction_reregister(motu: *mut snd_motu) -> i32 {
    let device = fw_parent_device((*motu).unit);
    let mut data: __be32;
    let mut err: i32;

    if (*motu).async_handler.callback_data.is_null() {
        return -EINVAL;
    }

    /* Register messaging address. Block transaction is not allowed. */
    data = cpu_to_be32((((*(*device).card).node_id as u32) << 16) | ((*motu).async_handler.offset >> 32) as u32);
    err = snd_motu_transaction_write(
        motu,
        ASYNC_ADDR_HI,
        &mut data,
        core::mem::size_of_val(&data),
    );
    if err < 0 {
        return err;
    }

    data = cpu_to_be32((*motu).async_handler.offset as u32);
    snd_motu_transaction_write(
        motu,
        ASYNC_ADDR_LO,
        &mut data,
        core::mem::size_of_val(&data),
    )
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_transaction_register(motu: *mut snd_motu) -> i32 {
    static RESP_REGISTER_REGION: fw_address_region = fw_address_region {
        start: 0xffffe0000000u64,
        end: 0xffffe000ffffu64,
    };
    let mut err: i32;

    /* Perhaps, 4 byte messages are transferred. */
    (*motu).async_handler.length = 4;
    (*motu).async_handler.address_callback = Some(handle_message);
    (*motu).async_handler.callback_data = motu as *mut c_void;

    err = fw_core_add_address_handler(&mut (*motu).async_handler, &RESP_REGISTER_REGION);
    if err < 0 {
        return err;
    }

    err = snd_motu_transaction_reregister(motu);
    if err < 0 {
        fw_core_remove_address_handler(&mut (*motu).async_handler);
        (*motu).async_handler.address_callback = None;
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_transaction_unregister(motu: *mut snd_motu) {
    let mut data: __be32;

    if (*motu).async_handler.address_callback.is_some() {
        fw_core_remove_address_handler(&mut (*motu).async_handler);
    }
    (*motu).async_handler.address_callback = None;

    /* Unregister the address. */
    data = cpu_to_be32(0x00000000);
    snd_motu_transaction_write(
        motu,
        ASYNC_ADDR_HI,
        &mut data,
        core::mem::size_of_val(&data),
    );
    snd_motu_transaction_write(
        motu,
        ASYNC_ADDR_LO,
        &mut data,
        core::mem::size_of_val(&data),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
