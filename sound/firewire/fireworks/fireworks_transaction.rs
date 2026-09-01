// SPDX-License-Identifier: GPL-2.0-only
/*
 * fireworks_transaction.c - a part of driver for Fireworks based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

/*
 * Fireworks have its own transaction. The transaction can be delivered by AV/C
 * Vendor Specific command frame or usual asynchronous transaction. At least,
 * Windows driver and firmware version 5.5 or later don't use AV/C command.
 *
 * Transaction substance:
 *  At first, 6 data exist. Following to the data, parameters for each command
 *  exist. All of the parameters are 32 bit aligned to big endian.
 *   data[0]:	Length of transaction substance
 *   data[1]:	Transaction version
 *   data[2]:	Sequence number. This is incremented by the device
 *   data[3]:	Transaction category
 *   data[4]:	Transaction command
 *   data[5]:	Return value in response.
 *   data[6-]:	Parameters
 *
 * Transaction address:
 *  command:	0xecc000000000
 *  response:	0xecc080000000 (default)
 *
 * I note that the address for response can be changed by command. But this
 * module uses the default address.
 */
// Dependencies supplied by the original C include: "./fireworks.h".

use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

const MEMORY_SPACE_EFW_COMMAND: c_ulonglong = 0xecc000000000u64;
const MEMORY_SPACE_EFW_RESPONSE: c_ulonglong = 0xecc080000000u64;

const ERROR_RETRIES: c_uint = 3;
const ERROR_DELAY_MS: c_uint = 5;
const EFC_TIMEOUT_MS: c_uint = 125;

type SizeT = usize;
type U32 = u32;

const STATE_PENDING: transaction_queue_state = 0;
const STATE_BUS_RESET: transaction_queue_state = 1;
const STATE_COMPLETE: transaction_queue_state = 2;

type transaction_queue_state = c_uint;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
    pub device: device,
}

#[repr(C)]
pub struct fw_device {
    pub card: *mut fw_card,
    pub generation: c_int,
    pub node_id: c_int,
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_efw_transaction {
    pub length: U32,
    pub version: U32,
    pub seqnum: U32,
    pub category: U32,
    pub command: U32,
    pub status: U32,
}

#[repr(C)]
pub struct snd_efw {
    pub lock: spinlock_t,
    pub unit: *mut fw_unit,
    pub resp_buf: *mut u8,
    pub push_ptr: *mut u8,
    pub pull_ptr: *mut u8,
    pub hwdep_wait: wait_queue_head_t,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_address_handler {
    pub length: SizeT,
    pub address_callback: Option<
        unsafe extern "C" fn(
            *mut fw_card,
            *mut fw_request,
            c_int,
            c_int,
            c_int,
            c_int,
            c_ulonglong,
            *mut c_void,
            SizeT,
            *mut c_void,
        ),
    >,
}

#[repr(C)]
pub struct fw_address_region {
    pub start: c_ulonglong,
    pub end: c_ulonglong,
}

#[repr(C)]
struct transaction_queue {
    list: list_head,
    unit: *mut fw_unit,
    buf: *mut c_void,
    size: c_uint,
    seqnum: U32,
    state: transaction_queue_state,
    wait: wait_queue_head_t,
}

extern "C" {
    static mut instances_lock: spinlock_t;
    static mut transaction_queues_lock: spinlock_t;
    static mut transaction_queues: list_head;
    static mut snd_efw_resp_buf_size: c_uint;
    static mut snd_efw_resp_buf_debug: bool;

    static SNDRV_CARDS: c_uint;
    static SND_EFW_TRANSACTION_USER_SEQNUM_MAX: U32;
    static SND_EFW_RESPONSE_MAXIMUM_BYTES: SizeT;

    static TCODE_WRITE_BLOCK_REQUEST: c_int;
    static RCODE_COMPLETE: c_int;
    static RCODE_CONFLICT_ERROR: c_int;
    static RCODE_TYPE_ERROR: c_int;
    static RCODE_DATA_ERROR: c_int;
    static RCODE_ADDRESS_ERROR: c_int;
    static EIO: c_int;

    fn snd_fw_transaction(
        unit: *mut fw_unit,
        tcode: c_int,
        offset: c_ulonglong,
        data: *mut c_void,
        size: c_uint,
        flags: c_int,
    ) -> c_int;
    fn init_waitqueue_head(wait: *mut wait_queue_head_t);
    fn list_add_tail(entry: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
    fn wait_event_timeout(
        wait: *mut wait_queue_head_t,
        condition: c_int,
        timeout: c_ulonglong,
    ) -> c_ulonglong;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulonglong;
    fn msleep(msecs: c_uint);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn smp_rmb();
    fn memcpy(dst: *mut c_void, src: *const c_void, n: SizeT) -> *mut c_void;
    fn wake_up(wait: *mut wait_queue_head_t);
    fn fw_send_response(card: *mut fw_card, request: *mut fw_request, rcode: c_int);
    fn fw_core_add_address_handler(
        handler: *mut fw_address_handler,
        region: *const fw_address_region,
    ) -> c_int;
    fn fw_core_remove_address_handler(handler: *mut fw_address_handler);
    fn WARN_ON(condition: c_int);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulonglong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulonglong);
}

static mut instances: *mut *mut snd_efw = ptr::null_mut();

#[inline]
unsafe fn be32_to_cpu(value: U32) -> U32 {
    U32::from_be(value)
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_transaction_cmd(
    unit: *mut fw_unit,
    cmd: *const c_void,
    size: c_uint,
) -> c_int {
    snd_fw_transaction(
        unit,
        TCODE_WRITE_BLOCK_REQUEST,
        MEMORY_SPACE_EFW_COMMAND,
        cmd as *mut c_void,
        size,
        0,
    )
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_transaction_run(
    unit: *mut fw_unit,
    cmd: *const c_void,
    cmd_size: c_uint,
    resp: *mut c_void,
    resp_size: c_uint,
) -> c_int {
    let mut t: transaction_queue = core::mem::zeroed();
    let mut tries: c_uint;
    let mut ret: c_int;

    t.unit = unit;
    t.buf = resp;
    t.size = resp_size;
    t.seqnum = be32_to_cpu((*(cmd as *const snd_efw_transaction)).seqnum).wrapping_add(1);
    t.state = STATE_PENDING;
    init_waitqueue_head(&mut t.wait);

    spin_lock_irq(&mut transaction_queues_lock);
    list_add_tail(&mut t.list, &mut transaction_queues);
    spin_unlock_irq(&mut transaction_queues_lock);

    tries = 0;
    loop {
        ret = snd_efw_transaction_cmd(t.unit, cmd as *mut c_void, cmd_size);
        if ret < 0 {
            break;
        }

        wait_event_timeout(
            &mut t.wait,
            (t.state != STATE_PENDING) as c_int,
            msecs_to_jiffies(EFC_TIMEOUT_MS),
        );

        if t.state == STATE_COMPLETE {
            ret = t.size as c_int;
            break;
        } else if t.state == STATE_BUS_RESET {
            msleep(ERROR_DELAY_MS);
        } else {
            tries = tries.wrapping_add(1);
            if tries >= ERROR_RETRIES {
                dev_err(
                    &mut (*t.unit).device,
                    b"EFW transaction timed out\n\0".as_ptr(),
                );
                ret = -EIO;
                break;
            }
        }
    }

    spin_lock_irq(&mut transaction_queues_lock);
    list_del(&mut t.list);
    spin_unlock_irq(&mut transaction_queues_lock);

    ret
}

unsafe fn copy_resp_to_buf(
    efw: *mut snd_efw,
    mut data: *mut c_void,
    mut length: SizeT,
    rcode: *mut c_int,
) {
    let mut capacity: SizeT;
    let mut till_end: SizeT;
    let t: *mut snd_efw_transaction;

    t = data as *mut snd_efw_transaction;
    length = core::cmp::min(
        be32_to_cpu((*t).length) as SizeT * size_of::<U32>(),
        length,
    );

    spin_lock(&mut (*efw).lock);

    if (*efw).push_ptr < (*efw).pull_ptr {
        capacity = (*efw).pull_ptr.offset_from((*efw).push_ptr) as c_uint as SizeT;
    } else {
        capacity = (snd_efw_resp_buf_size
            - (*efw).push_ptr.offset_from((*efw).pull_ptr) as c_uint) as SizeT;
    }

    /* confirm enough space for this response */
    if capacity < length {
        *rcode = RCODE_CONFLICT_ERROR;
        spin_unlock(&mut (*efw).lock);
        return;
    }

    /* copy to ring buffer */
    while length > 0 {
        till_end = (snd_efw_resp_buf_size
            - (*efw).push_ptr.offset_from((*efw).resp_buf) as c_uint) as SizeT;
        till_end = core::cmp::min(length, till_end);

        memcpy((*efw).push_ptr as *mut c_void, data, till_end);

        (*efw).push_ptr = (*efw).push_ptr.add(till_end);
        if (*efw).push_ptr >= (*efw).resp_buf.add(snd_efw_resp_buf_size as SizeT) {
            (*efw).push_ptr = (*efw).push_ptr.sub(snd_efw_resp_buf_size as SizeT);
        }

        length -= till_end;
        data = (data as *mut u8).add(till_end) as *mut c_void;
    }

    /* for hwdep */
    wake_up(&mut (*efw).hwdep_wait);

    *rcode = RCODE_COMPLETE;
    spin_unlock(&mut (*efw).lock);
}

unsafe fn handle_resp_for_user(
    card: *mut fw_card,
    generation: c_int,
    source: c_int,
    data: *mut c_void,
    length: SizeT,
    rcode: *mut c_int,
) {
    let mut device: *mut fw_device;
    let mut efw: *mut snd_efw;
    let mut i: c_uint;

    spin_lock_irq(&mut instances_lock);

    i = 0;
    while i < SNDRV_CARDS {
        efw = *instances.add(i as SizeT);
        if efw.is_null() {
            i = i.wrapping_add(1);
            continue;
        }
        device = fw_parent_device((*efw).unit);
        if ((*device).card != card) || ((*device).generation != generation) {
            i = i.wrapping_add(1);
            continue;
        }
        smp_rmb(); /* node id vs. generation */
        if (*device).node_id != source {
            i = i.wrapping_add(1);
            continue;
        }

        break;
    }
    if i == SNDRV_CARDS {
        spin_unlock_irq(&mut instances_lock);
        return;
    }

    copy_resp_to_buf(efw, data, length, rcode);
    spin_unlock_irq(&mut instances_lock);
}

unsafe fn handle_resp_for_kernel(
    card: *mut fw_card,
    generation: c_int,
    source: c_int,
    data: *mut c_void,
    length: SizeT,
    rcode: *mut c_int,
    seqnum: U32,
) {
    let mut device: *mut fw_device;
    let mut t: *mut transaction_queue;
    let flags: c_ulonglong;

    flags = spin_lock_irqsave(&mut transaction_queues_lock);
    t = transaction_queues.next as *mut transaction_queue;
    while &mut (*t).list as *mut list_head != &mut transaction_queues {
        device = fw_parent_device((*t).unit);
        if ((*device).card != card) || ((*device).generation != generation) {
            t = (*t).list.next as *mut transaction_queue;
            continue;
        }
        smp_rmb(); /* node_id vs. generation */
        if (*device).node_id != source {
            t = (*t).list.next as *mut transaction_queue;
            continue;
        }

        if ((*t).state == STATE_PENDING) && ((*t).seqnum == seqnum) {
            (*t).state = STATE_COMPLETE;
            (*t).size = core::cmp::min(length as c_uint, (*t).size);
            memcpy((*t).buf, data, (*t).size as SizeT);
            wake_up(&mut (*t).wait);
            *rcode = RCODE_COMPLETE;
        }
        t = (*t).list.next as *mut transaction_queue;
    }
    spin_unlock_irqrestore(&mut transaction_queues_lock, flags);
}

unsafe extern "C" fn efw_response(
    card: *mut fw_card,
    request: *mut fw_request,
    _tcode: c_int,
    _destination: c_int,
    source: c_int,
    generation: c_int,
    offset: c_ulonglong,
    data: *mut c_void,
    length: SizeT,
    _callback_data: *mut c_void,
) {
    let mut rcode: c_int;
    let mut dummy: c_int = 0;
    let seqnum: U32;

    rcode = RCODE_TYPE_ERROR;
    if length < size_of::<snd_efw_transaction>() {
        rcode = RCODE_DATA_ERROR;
        goto_end(card, request, rcode);
        return;
    } else if offset != MEMORY_SPACE_EFW_RESPONSE {
        rcode = RCODE_ADDRESS_ERROR;
        goto_end(card, request, rcode);
        return;
    }

    seqnum = be32_to_cpu((*(data as *mut snd_efw_transaction)).seqnum);
    if seqnum > SND_EFW_TRANSACTION_USER_SEQNUM_MAX.wrapping_add(1) {
        handle_resp_for_kernel(card, generation, source, data, length, &mut rcode, seqnum);
        if snd_efw_resp_buf_debug {
            handle_resp_for_user(card, generation, source, data, length, &mut dummy);
        }
    } else {
        handle_resp_for_user(card, generation, source, data, length, &mut rcode);
    }

    goto_end(card, request, rcode);
}

#[inline]
unsafe fn goto_end(card: *mut fw_card, request: *mut fw_request, rcode: c_int) {
    fw_send_response(card, request, rcode);
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_transaction_add_instance(efw: *mut snd_efw) {
    let mut i: c_uint;

    spin_lock_irq(&mut instances_lock);

    i = 0;
    while i < SNDRV_CARDS {
        if !(*instances.add(i as SizeT)).is_null() {
            i = i.wrapping_add(1);
            continue;
        }
        *instances.add(i as SizeT) = efw;
        break;
    }

    spin_unlock_irq(&mut instances_lock);
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_transaction_remove_instance(efw: *mut snd_efw) {
    let mut i: c_uint;

    spin_lock_irq(&mut instances_lock);

    i = 0;
    while i < SNDRV_CARDS {
        if *instances.add(i as SizeT) != efw {
            i = i.wrapping_add(1);
            continue;
        }
        *instances.add(i as SizeT) = ptr::null_mut();
        i = i.wrapping_add(1);
    }

    spin_unlock_irq(&mut instances_lock);
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_transaction_bus_reset(unit: *mut fw_unit) {
    let mut t: *mut transaction_queue;

    spin_lock_irq(&mut transaction_queues_lock);
    t = transaction_queues.next as *mut transaction_queue;
    while &mut (*t).list as *mut list_head != &mut transaction_queues {
        if ((*t).unit == unit) && ((*t).state == STATE_PENDING) {
            (*t).state = STATE_BUS_RESET;
            wake_up(&mut (*t).wait);
        }
        t = (*t).list.next as *mut transaction_queue;
    }
    spin_unlock_irq(&mut transaction_queues_lock);
}

static mut resp_register_handler: fw_address_handler = fw_address_handler {
    length: 0,
    address_callback: Some(efw_response),
};

#[no_mangle]
pub unsafe extern "C" fn snd_efw_transaction_register() -> c_int {
    let resp_register_region: fw_address_region = fw_address_region {
        start: MEMORY_SPACE_EFW_RESPONSE,
        end: MEMORY_SPACE_EFW_RESPONSE + SND_EFW_RESPONSE_MAXIMUM_BYTES as c_ulonglong,
    };

    resp_register_handler.length = SND_EFW_RESPONSE_MAXIMUM_BYTES;
    fw_core_add_address_handler(&mut resp_register_handler, &resp_register_region)
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_transaction_unregister() {
    WARN_ON((list_empty(&transaction_queues) == 0) as c_int);
    fw_core_remove_address_handler(&mut resp_register_handler);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
