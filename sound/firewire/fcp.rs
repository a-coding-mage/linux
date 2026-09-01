// SPDX-License-Identifier: GPL-2.0-only
/*
 * Function Control Protocol (IEC 61883-1) helper functions
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// Translated from linux/device.h, linux/firewire.h,
// linux/firewire-constants.h, linux/list.h, linux/module.h, linux/slab.h,
// linux/sched.h, linux/spinlock.h, linux/wait.h, linux/delay.h, fcp.h,
// lib.h, and amdtp-stream.h dependencies.

use core::ffi::c_void;

type u8 = u8;
type size_t = usize;

const CTS_AVC: u8 = 0x00;

const ERROR_RETRIES: i32 = 3;
const ERROR_DELAY_MS: u32 = 5;
const FCP_TIMEOUT_MS: u32 = 125;

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const EIO: i32 = 5;
const ENOSYS: i32 = 38;
const EAGAIN: i32 = 11;
const GFP_KERNEL: u32 = 0;

const AVC_PLUG_INFO_BUF_BYTES: usize = 4;
const CIP_SFC_COUNT: u32 = 16;
const TCODE_WRITE_QUADLET_REQUEST: i32 = 0;
const TCODE_WRITE_BLOCK_REQUEST: i32 = 1;
const CSR_REGISTER_BASE: u64 = 0xffff_f000_0000;
const CSR_FCP_COMMAND: u64 = 0x0b00;
const CSR_FCP_RESPONSE: u64 = 0x0d00;
const CSR_FCP_END: u64 = 0x0f00;

const fn BIT(nr: u32) -> u32 {
    1_u32 << nr
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
pub struct fw_device {
    pub card: *mut fw_card,
    pub generation: i32,
    pub node_id: i32,
}

#[repr(C)]
pub struct fw_unit {
    pub device: device,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_address_region {
    pub start: u64,
    pub end: u64,
}

type fw_address_callback_t = Option<
    unsafe extern "C" fn(
        card: *mut fw_card,
        request: *mut fw_request,
        tcode: i32,
        destination: i32,
        source: i32,
        generation: i32,
        offset: u64,
        data: *mut c_void,
        length: size_t,
        callback_data: *mut c_void,
    ),
>;

#[repr(C)]
pub struct fw_address_handler {
    pub length: u64,
    pub address_callback: fw_address_callback_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avc_general_plug_dir {
    AVC_GENERAL_PLUG_DIR_IN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum fcp_state {
    STATE_PENDING,
    STATE_BUS_RESET,
    STATE_COMPLETE,
    STATE_DEFERRED,
}

#[repr(C)]
struct fcp_transaction {
    list: list_head,
    unit: *mut fw_unit,
    response_buffer: *mut c_void,
    response_size: u32,
    response_match_bytes: u32,
    state: fcp_state,
    wait: wait_queue_head_t,
    deferrable: bool,
}

extern "C" {
    static amdtp_rate_table: [u32; CIP_SFC_COUNT as usize];

    fn kzalloc(size: size_t, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn init_waitqueue_head(wq_head: *mut wait_queue_head_t);
    fn wait_event_timeout(
        wq_head: wait_queue_head_t,
        condition: bool,
        timeout: u64,
    ) -> i64;
    fn msecs_to_jiffies(m: u32) -> u64;
    fn msleep(msecs: u32);
    fn snd_fw_transaction(
        unit: *mut fw_unit,
        tcode: i32,
        offset: u64,
        buffer: *mut c_void,
        length: u32,
        flags: u32,
    ) -> i32;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn smp_rmb();
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn fw_core_add_address_handler(
        handler: *mut fw_address_handler,
        region: *const fw_address_region,
    ) -> i32;
    fn fw_core_remove_address_handler(handler: *mut fw_address_handler);
    fn WARN_ON(condition: bool) -> i32;

    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> u64;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: u64);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn wake_up(wq_head: *mut wait_queue_head_t);
}

static mut transactions_lock: spinlock_t = spinlock_t { _private: [] };
static mut transactions: list_head = list_head {
    next: unsafe { &mut transactions as *mut list_head },
    prev: unsafe { &mut transactions as *mut list_head },
};

unsafe fn list_for_each_entry_fcp_transaction<F>(head: *mut list_head, mut f: F)
where
    F: FnMut(*mut fcp_transaction),
{
    let mut pos = (*head).next;
    while pos != head {
        let t = pos as *mut fcp_transaction;
        pos = (*pos).next;
        f(t);
    }
}

#[no_mangle]
pub unsafe extern "C" fn avc_general_set_sig_fmt(
    unit: *mut fw_unit,
    rate: u32,
    dir: avc_general_plug_dir,
    pid: u16,
) -> i32 {
    let mut sfc: u32;
    let buf: *mut u8;
    let mut flag: bool;
    let mut err: i32;

    flag = false;
    sfc = 0;
    while sfc < CIP_SFC_COUNT {
        if amdtp_rate_table[sfc as usize] == rate {
            flag = true;
            break;
        }
        sfc += 1;
    }
    if !flag {
        return -EINVAL;
    }

    buf = kzalloc(8, GFP_KERNEL) as *mut u8;
    if buf.is_null() {
        return -ENOMEM;
    }

    *buf.add(0) = 0x00; /* AV/C CONTROL */
    *buf.add(1) = 0xff; /* UNIT */
    if dir == avc_general_plug_dir::AVC_GENERAL_PLUG_DIR_IN {
        *buf.add(2) = 0x19; /* INPUT PLUG SIGNAL FORMAT */
    } else {
        *buf.add(2) = 0x18; /* OUTPUT PLUG SIGNAL FORMAT */
    }
    *buf.add(3) = (0xff & pid) as u8; /* plug id */
    *buf.add(4) = 0x90; /* EOH_1, Form_1, FMT. AM824 */
    *buf.add(5) = (0x07 & sfc) as u8; /* FDF-hi. AM824, frequency */
    *buf.add(6) = 0xff; /* FDF-mid. AM824, SYT hi (not used)*/
    *buf.add(7) = 0xff; /* FDF-low. AM824, SYT lo (not used) */

    /* do transaction and check buf[1-5] are the same against command */
    err = fcp_avc_transaction(
        unit,
        buf as *const c_void,
        8,
        buf as *mut c_void,
        8,
        BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5),
    );
    if err < 0 {
    } else if err < 8 {
        err = -EIO;
    } else if *buf.add(0) == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENOSYS;
    } else if *buf.add(0) == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    }
    if err < 0 {
        kfree(buf as *mut c_void);
        return err;
    }

    err = 0;
    kfree(buf as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn avc_general_get_sig_fmt(
    unit: *mut fw_unit,
    rate: *mut u32,
    dir: avc_general_plug_dir,
    pid: u16,
) -> i32 {
    let mut sfc: u32;
    let buf: *mut u8;
    let mut err: i32;

    buf = kzalloc(8, GFP_KERNEL) as *mut u8;
    if buf.is_null() {
        return -ENOMEM;
    }

    *buf.add(0) = 0x01; /* AV/C STATUS */
    *buf.add(1) = 0xff; /* Unit */
    if dir == avc_general_plug_dir::AVC_GENERAL_PLUG_DIR_IN {
        *buf.add(2) = 0x19; /* INPUT PLUG SIGNAL FORMAT */
    } else {
        *buf.add(2) = 0x18; /* OUTPUT PLUG SIGNAL FORMAT */
    }
    *buf.add(3) = (0xff & pid) as u8; /* plug id */
    *buf.add(4) = 0x90; /* EOH_1, Form_1, FMT. AM824 */
    *buf.add(5) = 0xff; /* FDF-hi. AM824, frequency */
    *buf.add(6) = 0xff; /* FDF-mid. AM824, SYT hi (not used) */
    *buf.add(7) = 0xff; /* FDF-low. AM824, SYT lo (not used) */

    /* do transaction and check buf[1-4] are the same against command */
    err = fcp_avc_transaction(
        unit,
        buf as *const c_void,
        8,
        buf as *mut c_void,
        8,
        BIT(1) | BIT(2) | BIT(3) | BIT(4),
    );
    if err < 0 {
    } else if err < 8 {
        err = -EIO;
    } else if *buf.add(0) == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENOSYS;
    } else if *buf.add(0) == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    } else if *buf.add(0) == 0x0b {
        /* IN TRANSITION */
        err = -EAGAIN;
    }
    if err < 0 {
        kfree(buf as *mut c_void);
        return err;
    }

    /* check sfc field and pick up rate */
    sfc = (0x07 & *buf.add(5)) as u32;
    if sfc >= CIP_SFC_COUNT {
        err = -EAGAIN; /* also in transition */
        kfree(buf as *mut c_void);
        return err;
    }

    *rate = amdtp_rate_table[sfc as usize];
    err = 0;
    kfree(buf as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn avc_general_get_plug_info(
    unit: *mut fw_unit,
    subunit_type: u32,
    subunit_id: u32,
    subfunction: u32,
    info: *mut u8,
) -> i32 {
    let buf: *mut u8;
    let mut err: i32;

    /* extended subunit in spec.4.2 is not supported */
    if subunit_type == 0x1E || subunit_id == 5 {
        return -EINVAL;
    }

    buf = kzalloc(8, GFP_KERNEL) as *mut u8;
    if buf.is_null() {
        return -ENOMEM;
    }

    *buf.add(0) = 0x01; /* AV/C STATUS */
    /* UNIT or Subunit, Functionblock */
    *buf.add(1) = (((subunit_type & 0x1f) << 3) | (subunit_id & 0x7)) as u8;
    *buf.add(2) = 0x02; /* PLUG INFO */
    *buf.add(3) = (0xff & subfunction) as u8;

    err = fcp_avc_transaction(
        unit,
        buf as *const c_void,
        8,
        buf as *mut c_void,
        8,
        BIT(1) | BIT(2),
    );
    if err < 0 {
    } else if err < 8 {
        err = -EIO;
    } else if *buf.add(0) == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENOSYS;
    } else if *buf.add(0) == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    } else if *buf.add(0) == 0x0b {
        /* IN TRANSITION */
        err = -EAGAIN;
    }
    if err < 0 {
        kfree(buf as *mut c_void);
        return err;
    }

    *info.add(0) = *buf.add(4);
    *info.add(1) = *buf.add(5);
    *info.add(2) = *buf.add(6);
    *info.add(3) = *buf.add(7);

    err = 0;
    kfree(buf as *mut c_void);
    err
}

/**
 * fcp_avc_transaction - send an AV/C command and wait for its response
 * @unit: a unit on the target device
 * @command: a buffer containing the command frame; must be DMA-able
 * @command_size: the size of @command
 * @response: a buffer for the response frame
 * @response_size: the maximum size of @response
 * @response_match_bytes: a bitmap specifying the bytes used to detect the
 *                        correct response frame
 *
 * This function sends a FCP command frame to the target and waits for the
 * corresponding response frame to be returned.
 *
 * Because it is possible for multiple FCP transactions to be active at the
 * same time, the correct response frame is detected by the value of certain
 * bytes.  These bytes must be set in @response before calling this function,
 * and the corresponding bits must be set in @response_match_bytes.
 *
 * @command and @response can point to the same buffer.
 *
 * Returns the actual size of the response frame, or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn fcp_avc_transaction(
    unit: *mut fw_unit,
    command: *const c_void,
    command_size: u32,
    response: *mut c_void,
    response_size: u32,
    response_match_bytes: u32,
) -> i32 {
    let mut t: fcp_transaction = core::mem::zeroed();
    let mut tcode: i32;
    let mut ret: i32;
    let mut tries: i32 = 0;

    t.unit = unit;
    t.response_buffer = response;
    t.response_size = response_size;
    t.response_match_bytes = response_match_bytes;
    t.state = fcp_state::STATE_PENDING;
    init_waitqueue_head(&mut t.wait);
    t.deferrable = *(command as *const u8) == 0x00 || *(command as *const u8) == 0x03;

    spin_lock_irq(&mut transactions_lock);
    list_add_tail(&mut t.list, &mut transactions);
    spin_unlock_irq(&mut transactions_lock);

    loop {
        tcode = if command_size == 4 {
            TCODE_WRITE_QUADLET_REQUEST
        } else {
            TCODE_WRITE_BLOCK_REQUEST
        };
        ret = snd_fw_transaction(
            t.unit,
            tcode,
            CSR_REGISTER_BASE + CSR_FCP_COMMAND,
            command as *mut c_void,
            command_size,
            0,
        );
        if ret < 0 {
            break;
        }

        loop {
            wait_event_timeout(
                t.wait,
                t.state != fcp_state::STATE_PENDING,
                msecs_to_jiffies(FCP_TIMEOUT_MS),
            );

            if t.state == fcp_state::STATE_DEFERRED {
                /*
                 * 'AV/C General Specification' define no time limit
                 * on command completion once an INTERIM response has
                 * been sent. but we promise to finish this function
                 * for a caller. Here we use FCP_TIMEOUT_MS for next
                 * interval. This is not in the specification.
                 */
                t.state = fcp_state::STATE_PENDING;
                continue;
            } else if t.state == fcp_state::STATE_COMPLETE {
                ret = t.response_size as i32;
                break;
            } else if t.state == fcp_state::STATE_BUS_RESET {
                msleep(ERROR_DELAY_MS);
            } else {
                tries += 1;
                if tries >= ERROR_RETRIES {
                    dev_err(
                        &mut (*t.unit).device,
                        b"FCP command timed out\n\0".as_ptr(),
                    );
                    ret = -EIO;
                    break;
                }
            }
            break;
        }
        if ret < 0 || t.state == fcp_state::STATE_COMPLETE || tries >= ERROR_RETRIES {
            break;
        }
    }

    spin_lock_irq(&mut transactions_lock);
    list_del(&mut t.list);
    spin_unlock_irq(&mut transactions_lock);

    ret
}

/**
 * fcp_bus_reset - inform the target handler about a bus reset
 * @unit: the unit that might be used by fcp_avc_transaction()
 *
 * This function must be called from the driver's .update handler to inform
 * the FCP transaction handler that a bus reset has happened.  Any pending FCP
 * transactions are retried.
 */
#[no_mangle]
pub unsafe extern "C" fn fcp_bus_reset(unit: *mut fw_unit) {
    spin_lock_irq(&mut transactions_lock);
    list_for_each_entry_fcp_transaction(&mut transactions, |t| {
        if (*t).unit == unit
            && ((*t).state == fcp_state::STATE_PENDING
                || (*t).state == fcp_state::STATE_DEFERRED)
        {
            (*t).state = fcp_state::STATE_BUS_RESET;
            wake_up(&mut (*t).wait);
        }
    });
    spin_unlock_irq(&mut transactions_lock);
}

/* checks whether the response matches the masked bytes in response_buffer */
unsafe fn is_matching_response(
    transaction: *mut fcp_transaction,
    response: *const c_void,
    mut length: size_t,
) -> bool {
    let p1: *const u8;
    let p2: *const u8;
    let mut mask: u32;
    let mut i: u32;

    p1 = response as *const u8;
    p2 = (*transaction).response_buffer as *const u8;
    mask = (*transaction).response_match_bytes;

    i = 0;
    loop {
        if (mask & 1) != 0 && *p1.add(i as usize) != *p2.add(i as usize) {
            return false;
        }
        mask >>= 1;
        if mask == 0 {
            return true;
        }
        length -= 1;
        if length == 0 {
            return false;
        }
        i += 1;
    }
}

unsafe extern "C" fn fcp_response(
    card: *mut fw_card,
    request: *mut fw_request,
    tcode: i32,
    destination: i32,
    source: i32,
    generation: i32,
    offset: u64,
    data: *mut c_void,
    length: size_t,
    callback_data: *mut c_void,
) {
    let _ = request;
    let _ = tcode;
    let _ = destination;
    let _ = offset;
    let _ = callback_data;

    if length < 1 || (*(data as *const u8) & 0xf0) != CTS_AVC {
        return;
    }

    let flags = spin_lock_irqsave(&mut transactions_lock);
    list_for_each_entry_fcp_transaction(&mut transactions, |t| {
        let device: *mut fw_device = fw_parent_device((*t).unit);
        if (*device).card != card || (*device).generation != generation {
            return;
        }
        smp_rmb(); /* node_id vs. generation */
        if (*device).node_id != source {
            return;
        }

        if (*t).state == fcp_state::STATE_PENDING && is_matching_response(t, data, length) {
            if (*t).deferrable && *(data as *const u8) == 0x0f {
                (*t).state = fcp_state::STATE_DEFERRED;
            } else {
                (*t).state = fcp_state::STATE_COMPLETE;
                (*t).response_size = core::cmp::min(length as u32, (*t).response_size);
                memcpy((*t).response_buffer, data as *const c_void, (*t).response_size as size_t);
            }
            wake_up(&mut (*t).wait);
        }
    });
    spin_unlock_irqrestore(&mut transactions_lock, flags);
}

static mut response_register_handler: fw_address_handler = fw_address_handler {
    length: 0x200,
    address_callback: Some(fcp_response),
};

unsafe extern "C" fn fcp_module_init() -> i32 {
    static response_register_region: fw_address_region = fw_address_region {
        start: CSR_REGISTER_BASE + CSR_FCP_RESPONSE,
        end: CSR_REGISTER_BASE + CSR_FCP_END,
    };

    fw_core_add_address_handler(&mut response_register_handler, &response_register_region);

    0
}

unsafe extern "C" fn fcp_module_exit() {
    WARN_ON(!list_empty(&transactions));
    fw_core_remove_address_handler(&mut response_register_handler);
}

// module_init(fcp_module_init);
// module_exit(fcp_module_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
