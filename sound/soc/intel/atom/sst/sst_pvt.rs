// SPDX-License-Identifier: GPL-2.0-only
/*
 *  sst_pvt.c - Intel SST Driver for audio engine
 *
 *  Copyright (C) 2008-14	Intel Corp
 *  Authors:	Vinod Koul <vinod.koul@intel.com>
 *		Harsha Priya <priya.harsha@intel.com>
 *		Dharageswari R <dharageswari.r@intel.com>
 *		KP Jeeja <jeeja.kp@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null_mut, read_volatile, write_volatile};

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type size_t = usize;

const GFP_ATOMIC: c_int = 0;
const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const SST_BLOCK_TIMEOUT: c_int = 0;
const SST_RESET: c_int = 0;
const STREAM_UN_INIT: c_int = 0;
const SST_MAILBOX_SIZE: size_t = 0;
const MRFLD_FW_VIRTUAL_BASE: u32 = 0;
const SST_MAX_BLOCKS: c_int = 15;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
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

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct sst_info {
    pub max_streams: c_int,
}

#[repr(C)]
pub struct stream_info {
    pub status: c_int,
    pub prev: c_int,
    pub lock: mutex,
    pub cumm_bytes: u64,
    pub pipe_id: u32,
}

#[repr(C)]
pub struct sst_block {
    pub condition: c_int,
    pub msg_id: u32,
    pub drv_id: u32,
    pub ret_code: c_int,
    pub on: bool_,
    pub data: *mut c_void,
    pub size: size_t,
}

#[repr(C)]
pub struct ipc_header_high_part {
    pub msg_id: c_int,
    pub task_id: c_int,
    pub large: c_int,
    pub drv_id: c_int,
    pub done: c_int,
    pub busy: c_int,
    pub res_rqd: c_int,
}

#[repr(C)]
pub struct ipc_header_high {
    pub full: u32,
    pub part: ipc_header_high_part,
}

#[repr(C)]
pub struct ipc_header_parts {
    pub header_high: ipc_header_high,
    pub header_low_payload: size_t,
}

#[repr(C)]
pub union ipc_header_mrfld {
    pub full: u64,
    pub p: core::mem::ManuallyDrop<ipc_header_parts>,
}

#[repr(C)]
pub struct ipc_dsp_hdr {
    pub cmd_id: c_int,
    pub mod_index_id: c_int,
    pub pipe_id: c_int,
    pub length: c_int,
    pub mod_id: c_int,
}

#[repr(C)]
pub struct ipc_post {
    pub node: list_head,
    pub mailbox_data: *mut c_void,
    pub is_large: bool_,
    pub mrfld_header: ipc_header_mrfld,
}

#[repr(C)]
pub struct sst_ops {
    pub post_message: unsafe extern "C" fn(*mut intel_sst_drv, *mut ipc_post, bool_),
}

#[repr(C)]
pub struct intel_sst_drv {
    pub sst_lock: mutex,
    pub sst_state: c_int,
    pub dev: *mut device,
    pub wait_queue: wait_queue_head_t,
    pub pvt_id: c_ulong,
    pub block_lock: spinlock_t,
    pub info: sst_info,
    pub streams: *mut stream_info,
    pub ipc_spin_lock: spinlock_t,
    pub ipc_dispatch_list: list_head,
    pub ops: *mut sst_ops,
}

unsafe extern "C" {
    fn memcpy_fromio(to: *mut c_void, from: *const c_void, count: size_t);
    fn memcpy_toio(to: *mut c_void, from: *const c_void, count: size_t);
    fn memcpy(to: *mut c_void, from: *const c_void, count: size_t) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kmemdup(src: *const c_void, len: size_t, gfp: c_int) -> *mut c_void;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn wait_event_timeout(queue: wait_queue_head_t, condition: c_int, timeout: c_ulong) -> c_long;
    fn msecs_to_jiffies(msecs: c_int) -> c_ulong;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn ffz(word: c_ulong) -> c_int;
    fn change_bit(nr: c_int, addr: *mut c_ulong);
    fn test_and_clear_bit(nr: c_int, addr: *mut c_ulong) -> c_int;
    fn sst_create_block(
        sst_drv_ctx: *mut intel_sst_drv,
        msg_id: u32,
        drv_id: u32,
    ) -> *mut sst_block;
    fn sst_free_block(sst_drv_ctx: *mut intel_sst_drv, block: *mut sst_block);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
}

type c_long = isize;

#[no_mangle]
pub unsafe extern "C" fn sst_shim_write(addr: *mut c_void, offset: c_int, value: c_int) -> c_int {
    write_volatile((addr as *mut u8).offset(offset as isize) as *mut u32, value as u32);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sst_shim_read(addr: *mut c_void, offset: c_int) -> u32 {
    read_volatile((addr as *mut u8).offset(offset as isize) as *const u32)
}

#[no_mangle]
pub unsafe extern "C" fn sst_reg_read64(addr: *mut c_void, offset: c_int) -> u64 {
    let mut val: u64 = 0;

    memcpy_fromio(
        &mut val as *mut u64 as *mut c_void,
        (addr as *mut u8).offset(offset as isize) as *const c_void,
        size_of::<u64>(),
    );

    val
}

#[no_mangle]
pub unsafe extern "C" fn sst_shim_write64(addr: *mut c_void, offset: c_int, value: u64) -> c_int {
    memcpy_toio(
        (addr as *mut u8).offset(offset as isize) as *mut c_void,
        &value as *const u64 as *const c_void,
        size_of::<u64>(),
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn sst_shim_read64(addr: *mut c_void, offset: c_int) -> u64 {
    let mut val: u64 = 0;

    memcpy_fromio(
        &mut val as *mut u64 as *mut c_void,
        (addr as *mut u8).offset(offset as isize) as *const c_void,
        size_of::<u64>(),
    );
    val
}

#[no_mangle]
pub unsafe extern "C" fn sst_set_fw_state_locked(
    sst_drv_ctx: *mut intel_sst_drv,
    sst_state: c_int,
) {
    mutex_lock(&mut (*sst_drv_ctx).sst_lock);
    (*sst_drv_ctx).sst_state = sst_state;
    mutex_unlock(&mut (*sst_drv_ctx).sst_lock);
}

/*
 * sst_wait_timeout - wait on event for timeout
 *
 * @sst_drv_ctx: Driver context
 * @block: Driver block to wait on
 *
 * This function waits with a timeout value (and is not interruptible) on a
 * given block event
 */
#[no_mangle]
pub unsafe extern "C" fn sst_wait_timeout(
    sst_drv_ctx: *mut intel_sst_drv,
    block: *mut sst_block,
) -> c_int {
    let mut retval: c_int = 0;

    /*
     * NOTE:
     * Observed that FW processes the alloc msg and replies even
     * before the alloc thread has finished execution
     */
    dev_dbg(
        (*sst_drv_ctx).dev,
        b"waiting for condition %x ipc %d drv_id %d\n\0".as_ptr(),
        (*block).condition,
        (*block).msg_id,
        (*block).drv_id,
    );
    if wait_event_timeout(
        (*sst_drv_ctx).wait_queue,
        (*block).condition,
        msecs_to_jiffies(SST_BLOCK_TIMEOUT),
    ) != 0
    {
        /* event wake */
        dev_dbg(
            (*sst_drv_ctx).dev,
            b"Event wake %x\n\0".as_ptr(),
            (*block).condition,
        );
        dev_dbg(
            (*sst_drv_ctx).dev,
            b"message ret: %d\n\0".as_ptr(),
            (*block).ret_code,
        );
        retval = -(*block).ret_code;
    } else {
        (*block).on = false;
        dev_err(
            (*sst_drv_ctx).dev,
            b"Wait timed-out condition:%#x, msg_id:%#x fw_state %#x\n\0".as_ptr(),
            (*block).condition,
            (*block).msg_id,
            (*sst_drv_ctx).sst_state,
        );
        (*sst_drv_ctx).sst_state = SST_RESET;

        retval = -EBUSY;
    }
    retval
}

/*
 * sst_create_ipc_msg - create a IPC message
 *
 * @arg: ipc message
 * @large: large or short message
 *
 * this function allocates structures to send a large or short
 * message to the firmware
 */
#[no_mangle]
pub unsafe extern "C" fn sst_create_ipc_msg(arg: *mut *mut ipc_post, large: bool_) -> c_int {
    let msg: *mut ipc_post;

    msg = kzalloc(size_of::<ipc_post>(), GFP_ATOMIC) as *mut ipc_post;
    if msg.is_null() {
        return -ENOMEM;
    }
    if large {
        (*msg).mailbox_data = kzalloc(SST_MAILBOX_SIZE, GFP_ATOMIC);
        if (*msg).mailbox_data.is_null() {
            kfree(msg as *mut c_void);
            return -ENOMEM;
        }
    } else {
        (*msg).mailbox_data = null_mut();
    }
    (*msg).is_large = large;
    *arg = msg;
    0
}

/*
 * sst_create_block_and_ipc_msg - Creates IPC message and sst block
 * @arg: passed to sst_create_ipc_message API
 * @large: large or short message
 * @sst_drv_ctx: sst driver context
 * @block: return block allocated
 * @msg_id: IPC
 * @drv_id: stream id or private id
 */
#[no_mangle]
pub unsafe extern "C" fn sst_create_block_and_ipc_msg(
    arg: *mut *mut ipc_post,
    large: bool_,
    sst_drv_ctx: *mut intel_sst_drv,
    block: *mut *mut sst_block,
    msg_id: u32,
    drv_id: u32,
) -> c_int {
    let retval: c_int;

    retval = sst_create_ipc_msg(arg, large);
    if retval != 0 {
        return retval;
    }
    *block = sst_create_block(sst_drv_ctx, msg_id, drv_id);
    if (*block).is_null() {
        kfree(*arg as *mut c_void);
        return -ENOMEM;
    }
    0
}

/*
 * sst_clean_stream - clean the stream context
 *
 * @stream: stream structure
 *
 * this function resets the stream contexts
 * should be called in free
 */
#[no_mangle]
pub unsafe extern "C" fn sst_clean_stream(stream: *mut stream_info) {
    (*stream).status = STREAM_UN_INIT;
    (*stream).prev = STREAM_UN_INIT;
    mutex_lock(&mut (*stream).lock);
    (*stream).cumm_bytes = 0;
    mutex_unlock(&mut (*stream).lock);
}

#[no_mangle]
pub unsafe extern "C" fn sst_prepare_and_post_msg(
    sst: *mut intel_sst_drv,
    task_id: c_int,
    ipc_msg: c_int,
    cmd_id: c_int,
    pipe_id: c_int,
    mbox_data_len: size_t,
    mbox_data: *const c_void,
    data: *mut *mut c_void,
    large: bool_,
    fill_dsp: bool_,
    sync: bool_,
    response: bool_,
) -> c_int {
    let mut block: *mut sst_block = null_mut();
    let mut msg: *mut ipc_post = null_mut();
    let mut dsp_hdr: ipc_dsp_hdr = core::mem::zeroed();
    let mut ret: c_int = 0;
    let pvt_id: c_int;

    pvt_id = sst_assign_pvt_id(sst);
    if pvt_id < 0 {
        return pvt_id;
    }

    if response {
        ret = sst_create_block_and_ipc_msg(
            &mut msg,
            large,
            sst,
            &mut block,
            ipc_msg as u32,
            pvt_id as u32,
        );
    } else {
        ret = sst_create_ipc_msg(&mut msg, large);
    }

    if ret < 0 {
        test_and_clear_bit(pvt_id, &mut (*sst).pvt_id);
        return -ENOMEM;
    }

    dev_dbg(
        (*sst).dev,
        b"pvt_id = %d, pipe id = %d, task = %d ipc_msg: %d\n\0".as_ptr(),
        pvt_id,
        pipe_id,
        task_id,
        ipc_msg,
    );
    sst_fill_header_mrfld(&mut (*msg).mrfld_header, ipc_msg, task_id, large as c_int, pvt_id);
    (*msg).mrfld_header.p.header_low_payload = size_of::<ipc_dsp_hdr>() + mbox_data_len;
    (*msg).mrfld_header.p.header_high.part.res_rqd = (!sync) as c_int;
    dev_dbg(
        (*sst).dev,
        b"header:%x\n\0".as_ptr(),
        (*msg).mrfld_header.p.header_high.full,
    );
    dev_dbg(
        (*sst).dev,
        b"response rqd: %x\0".as_ptr(),
        (*msg).mrfld_header.p.header_high.part.res_rqd,
    );
    dev_dbg(
        (*sst).dev,
        b"msg->mrfld_header.p.header_low_payload:%d\0".as_ptr(),
        (*msg).mrfld_header.p.header_low_payload,
    );
    if fill_dsp {
        sst_fill_header_dsp(&mut dsp_hdr, cmd_id, pipe_id, mbox_data_len as c_int);
        memcpy(
            (*msg).mailbox_data,
            &dsp_hdr as *const ipc_dsp_hdr as *const c_void,
            size_of::<ipc_dsp_hdr>(),
        );
        if mbox_data_len != 0 {
            memcpy(
                ((*msg).mailbox_data as *mut u8).add(size_of::<ipc_dsp_hdr>()) as *mut c_void,
                mbox_data,
                mbox_data_len,
            );
        }
    }

    if sync {
        ((*(*sst).ops).post_message)(sst, msg, true);
    } else {
        sst_add_to_dispatch_list_and_post(sst, msg);
    }

    if response {
        ret = sst_wait_timeout(sst, block);
        if ret < 0 {
            sst_free_block(sst, block);
            test_and_clear_bit(pvt_id, &mut (*sst).pvt_id);
            return ret;
        }

        if !data.is_null() && !(*block).data.is_null() {
            *data = kmemdup((*block).data, (*block).size, GFP_KERNEL);
            if (*data).is_null() {
                ret = -ENOMEM;
                sst_free_block(sst, block);
                test_and_clear_bit(pvt_id, &mut (*sst).pvt_id);
                return ret;
            }
        }
    }

    if response {
        sst_free_block(sst, block);
    }
    test_and_clear_bit(pvt_id, &mut (*sst).pvt_id);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn sst_pm_runtime_put(sst_drv: *mut intel_sst_drv) -> c_int {
    let ret: c_int;

    ret = pm_runtime_put_autosuspend((*sst_drv).dev);
    if ret < 0 {
        return ret;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sst_fill_header_mrfld(
    header: *mut ipc_header_mrfld,
    msg: c_int,
    task_id: c_int,
    large: c_int,
    drv_id: c_int,
) {
    (*header).full = 0;
    (*header).p.header_high.part.msg_id = msg;
    (*header).p.header_high.part.task_id = task_id;
    (*header).p.header_high.part.large = large;
    (*header).p.header_high.part.drv_id = drv_id;
    (*header).p.header_high.part.done = 0;
    (*header).p.header_high.part.busy = 1;
    (*header).p.header_high.part.res_rqd = 1;
}

#[no_mangle]
pub unsafe extern "C" fn sst_fill_header_dsp(
    dsp: *mut ipc_dsp_hdr,
    msg: c_int,
    pipe_id: c_int,
    len: c_int,
) {
    (*dsp).cmd_id = msg;
    (*dsp).mod_index_id = 0xff;
    (*dsp).pipe_id = pipe_id;
    (*dsp).length = len;
    (*dsp).mod_id = 0;
}

/*
 * sst_assign_pvt_id - assign a pvt id for stream
 *
 * @sst_drv_ctx : driver context
 *
 * this function assigns a private id for calls that dont have stream
 * context yet, should be called with lock held
 * uses bits for the id, and finds first free bits and assigns that
 */
#[no_mangle]
pub unsafe extern "C" fn sst_assign_pvt_id(drv: *mut intel_sst_drv) -> c_int {
    let local: c_int;

    spin_lock(&mut (*drv).block_lock);
    /* find first zero index from lsb */
    local = ffz((*drv).pvt_id);
    dev_dbg((*drv).dev, b"pvt_id assigned --> %d\n\0".as_ptr(), local);
    if local >= SST_MAX_BLOCKS {
        dev_err((*drv).dev, b"PVT _ID error: no free id blocks \0".as_ptr());
        spin_unlock(&mut (*drv).block_lock);
        return -EINVAL;
    }
    /* toggle the index */
    change_bit(local, &mut (*drv).pvt_id);

    spin_unlock(&mut (*drv).block_lock);
    local
}

#[no_mangle]
pub unsafe extern "C" fn sst_validate_strid(
    sst_drv_ctx: *mut intel_sst_drv,
    str_id: c_int,
) -> c_int {
    if str_id <= 0 || str_id > (*sst_drv_ctx).info.max_streams {
        dev_err(
            (*sst_drv_ctx).dev,
            b"SST ERR: invalid stream id : %d, max %d\n\0".as_ptr(),
            str_id,
            (*sst_drv_ctx).info.max_streams,
        );
        return -EINVAL;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn get_stream_info(
    sst_drv_ctx: *mut intel_sst_drv,
    str_id: c_int,
) -> *mut stream_info {
    if sst_validate_strid(sst_drv_ctx, str_id) != 0 {
        return null_mut();
    }
    (*sst_drv_ctx).streams.offset(str_id as isize)
}

#[no_mangle]
pub unsafe extern "C" fn get_stream_id_mrfld(
    sst_drv_ctx: *mut intel_sst_drv,
    pipe_id: u32,
) -> c_int {
    let mut i: c_int;

    i = 1;
    while i <= (*sst_drv_ctx).info.max_streams {
        if pipe_id == (*(*sst_drv_ctx).streams.offset(i as isize)).pipe_id {
            return i;
        }
        i += 1;
    }

    dev_dbg((*sst_drv_ctx).dev, b"no such pipe_id(%u)\0".as_ptr(), pipe_id);
    -1
}

#[no_mangle]
pub unsafe extern "C" fn relocate_imr_addr_mrfld(mut base_addr: u32) -> u32 {
    /* Get the difference from 512MB aligned base addr */
    /* relocate the base */
    base_addr = MRFLD_FW_VIRTUAL_BASE.wrapping_add(base_addr % (512 * 1024 * 1024));
    base_addr
}
/* EXPORT_SYMBOL_GPL(relocate_imr_addr_mrfld); */

#[no_mangle]
pub unsafe extern "C" fn sst_add_to_dispatch_list_and_post(
    sst: *mut intel_sst_drv,
    msg: *mut ipc_post,
) {
    let irq_flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*sst).ipc_spin_lock, irq_flags);
    list_add_tail(&mut (*msg).node, &mut (*sst).ipc_dispatch_list);
    spin_unlock_irqrestore(&mut (*sst).ipc_spin_lock, irq_flags);
    ((*(*sst).ops).post_message)(sst, null_mut(), false);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
