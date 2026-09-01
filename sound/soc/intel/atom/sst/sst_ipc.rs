// SPDX-License-Identifier: GPL-2.0-only
/*
 *  sst_ipc.c - Intel SST Driver for audio engine
 *
 *  Copyright (C) 2008-14 Intel Corporation
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

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null_mut, read_unaligned};

type bool_ = bool;
type u32 = u32;
type u64 = u64;

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_uint = 0;
const KERN_DEBUG: *const c_char = b"\0".as_ptr() as *const c_char;
const DUMP_PREFIX_NONE: c_int = 0;
const SST_IPCX: c_uint = 0;
const SST_IMRX: c_uint = 0;
const SST_ISRX: c_uint = 0;
const SST_IPCD: c_uint = 0;
const SST_MAILBOX_SEND: usize = 0;
const IPC_ACK_SUCCESS: u32 = 0;
const FW_DWNL_ID: u32 = 0;
const SST_RESET: c_int = 0;
const STREAM_INIT: c_int = 0;
const SST_ASYNC_DRV_ID: u32 = 0;
const IPC_SST_PERIOD_ELAPSED_MRFLD: u32 = 0;
const IPC_IA_DRAIN_STREAM_MRFLD: u32 = 0;
const IPC_IA_FW_ASYNC_ERR_MRFLD: u32 = 0;
const IPC_IA_FW_INIT_CMPLT_MRFLD: u32 = 0;
const IPC_IA_BUF_UNDER_RUN_MRFLD: u32 = 0;

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
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sst_version {
    pub type_: u8,
    pub major: u8,
    pub minor: u8,
    pub build: u8,
}

#[repr(C)]
pub struct stream_info {
    pub status: c_int,
    pub period_elapsed: Option<unsafe extern "C" fn(*mut c_void)>,
    pub pcm_substream: *mut c_void,
    pub compr_cb: Option<unsafe extern "C" fn(*mut c_void)>,
    pub compr_cb_param: *mut c_void,
    pub drain_notify: Option<unsafe extern "C" fn(*mut c_void)>,
    pub drain_cb_param: *mut c_void,
}

#[repr(C)]
pub struct intel_sst_drv {
    pub dev: *mut device,
    pub block_lock: spinlock_t,
    pub block_list: list_head,
    pub wait_queue: wait_queue_head_t,
    pub ipc_spin_lock: spinlock_t,
    pub shim: *mut c_void,
    pub ipc_dispatch_list: list_head,
    pub mailbox: *mut u8,
    pub fw_version: sst_version,
    pub streams: *mut stream_info,
}

#[repr(C)]
pub struct sst_block {
    pub node: list_head,
    pub condition: bool_,
    pub on: bool_,
    pub msg_id: u32,
    pub drv_id: u32,
    pub ret_code: c_int,
    pub data: *mut c_void,
    pub size: u32,
}

#[repr(C)]
pub struct ipc_header_high_part {
    pub busy: u32,
    pub done: u32,
    pub large: u32,
    pub result: c_int,
    pub drv_id: u32,
    pub msg_id: u32,
}

#[repr(C)]
pub struct ipc_header_high {
    pub full: u32,
    pub part: ipc_header_high_part,
}

#[repr(C)]
pub struct ipc_header_mrfld_p {
    pub header_high: ipc_header_high,
    pub header_low_payload: u32,
}

#[repr(C)]
pub union ipc_header_mrfld {
    pub full: u64,
    pub p: core::mem::ManuallyDrop<ipc_header_mrfld_p>,
}

#[repr(C)]
pub struct interrupt_reg_mrfld_part {
    pub busy_interrupt: u32,
}

#[repr(C)]
pub union interrupt_reg_mrfld {
    pub full: u64,
    pub part: core::mem::ManuallyDrop<interrupt_reg_mrfld_part>,
}

#[repr(C)]
pub struct ipc_post {
    pub node: list_head,
    pub mrfld_header: ipc_header_mrfld,
    pub mailbox_data: *mut u8,
}

#[repr(C)]
pub struct fw_build_info {
    pub date: *const c_char,
    pub time: *const c_char,
}

#[repr(C)]
pub struct ipc_header_fw_init {
    pub result: c_int,
    pub fw_version: sst_version,
    pub build_info: fw_build_info,
}

#[repr(C)]
pub struct ipc_dsp_hdr {
    pub cmd_id: u32,
    pub pipe_id: u32,
}

extern "C" {
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kmemdup(src: *const c_void, len: usize, gfp: c_uint) -> *mut c_void;
    fn memcpy_toio(dst: *mut c_void, src: *const c_void, count: usize);
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn wake_up(wq: *mut wait_queue_head_t);
    fn sst_shim_read64(addr: *mut c_void, offset: c_uint) -> u64;
    fn sst_shim_write64(addr: *mut c_void, offset: c_uint, value: u64);
    fn cpu_relax();
    fn sst_set_fw_state_locked(ctx: *mut intel_sst_drv, state: c_int);
    fn get_stream_id_mrfld(ctx: *mut intel_sst_drv, pipe_id: u32) -> c_int;
    fn print_hex_dump(
        level: *const c_char,
        prefix_str: *const c_char,
        prefix_type: c_int,
        rowsize: c_int,
        groupsize: c_int,
        buf: *const c_void,
        len: usize,
        ascii: bool_,
    );
}

unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

unsafe fn container_of_ipc_post(ptr: *mut list_head) -> *mut ipc_post {
    (ptr as *mut u8).sub(core::mem::offset_of!(ipc_post, node)) as *mut ipc_post
}

unsafe fn container_of_sst_block(ptr: *mut list_head) -> *mut sst_block {
    (ptr as *mut u8).sub(core::mem::offset_of!(sst_block, node)) as *mut sst_block
}

pub unsafe extern "C" fn sst_create_block(
    ctx: *mut intel_sst_drv,
    msg_id: u32,
    drv_id: u32,
) -> *mut sst_block {
    let msg: *mut sst_block;

    dev_dbg((*ctx).dev, c"Enter\n".as_ptr());
    msg = kzalloc(size_of::<sst_block>(), GFP_KERNEL) as *mut sst_block;
    if msg.is_null() {
        return null_mut();
    }
    (*msg).condition = false;
    (*msg).on = true;
    (*msg).msg_id = msg_id;
    (*msg).drv_id = drv_id;
    spin_lock_bh(&mut (*ctx).block_lock);
    list_add_tail(&mut (*msg).node, &mut (*ctx).block_list);
    spin_unlock_bh(&mut (*ctx).block_lock);

    msg
}

/*
 * while handling the interrupts, we need to check for message status and
 * then if we are blocking for a message
 *
 * here we are unblocking the blocked ones, this is based on id we have
 * passed and search that for block threads.
 * We will not find block in two cases
 *  a) when its small message and block in not there, so silently ignore
 *  them
 *  b) when we are actually not able to find the block (bug perhaps)
 *
 *  Since we have bit of small messages we can spam kernel log with err
 *  print on above so need to keep as debug prints which should be enabled
 *  via dynamic debug while debugging IPC issues
 */
pub unsafe extern "C" fn sst_wake_up_block(
    ctx: *mut intel_sst_drv,
    result: c_int,
    drv_id: u32,
    ipc: u32,
    data: *mut c_void,
    size: u32,
) -> c_int {
    let mut pos: *mut list_head;

    dev_dbg((*ctx).dev, c"Enter\n".as_ptr());

    spin_lock_bh(&mut (*ctx).block_lock);
    pos = (*ctx).block_list.next;
    while pos != &mut (*ctx).block_list {
        let block = container_of_sst_block(pos);
        dev_dbg(
            (*ctx).dev,
            c"Block ipc %d, drv_id %d\n".as_ptr(),
            (*block).msg_id,
            (*block).drv_id,
        );
        if (*block).msg_id == ipc && (*block).drv_id == drv_id {
            dev_dbg((*ctx).dev, c"free up the block\n".as_ptr());
            (*block).ret_code = result;
            (*block).data = data;
            (*block).size = size;
            (*block).condition = true;
            spin_unlock_bh(&mut (*ctx).block_lock);
            wake_up(&mut (*ctx).wait_queue);
            return 0;
        }
        pos = (*pos).next;
    }
    spin_unlock_bh(&mut (*ctx).block_lock);
    dev_dbg(
        (*ctx).dev,
        c"Block not found or a response received for a short msg for ipc %d, drv_id %d\n".as_ptr(),
        ipc,
        drv_id,
    );
    -EINVAL
}

pub unsafe extern "C" fn sst_free_block(ctx: *mut intel_sst_drv, freed: *mut sst_block) -> c_int {
    dev_dbg((*ctx).dev, c"Enter\n".as_ptr());
    spin_lock_bh(&mut (*ctx).block_lock);
    let mut pos = (*ctx).block_list.next;
    while pos != &mut (*ctx).block_list {
        let next = (*pos).next;
        let block = container_of_sst_block(pos);
        if block == freed {
            pr_debug(c"pvt_id freed --> %d\n".as_ptr(), (*freed).drv_id);
            /* toggle the index position of pvt_id */
            list_del(&mut (*freed).node);
            spin_unlock_bh(&mut (*ctx).block_lock);
            kfree((*freed).data);
            (*freed).data = null_mut();
            kfree(freed as *mut c_void);
            return 0;
        }
        pos = next;
    }
    spin_unlock_bh(&mut (*ctx).block_lock);
    dev_err((*ctx).dev, c"block is already freed!!!\n".as_ptr());
    -EINVAL
}

pub unsafe extern "C" fn sst_post_message_mrfld(
    sst_drv_ctx: *mut intel_sst_drv,
    ipc_msg: *mut ipc_post,
    sync: bool_,
) -> c_int {
    let mut msg = ipc_msg;
    let mut header = ipc_header_mrfld { full: 0 };
    let mut loop_count: c_uint = 0;
    let mut retval: c_int = 0;
    let mut irq_flags: c_ulong = 0;

    dev_dbg((*sst_drv_ctx).dev, c"Enter: sync: %d\n".as_ptr(), sync as c_int);
    spin_lock_irqsave(&mut (*sst_drv_ctx).ipc_spin_lock, &mut irq_flags);
    header.full = sst_shim_read64((*sst_drv_ctx).shim, SST_IPCX);
    if sync {
        while header.p.header_high.part.busy != 0 {
            if loop_count > 25 {
                dev_err(
                    (*sst_drv_ctx).dev,
                    c"sst: Busy wait failed, can't send this msg\n".as_ptr(),
                );
                retval = -EBUSY;
                goto_out(sst_drv_ctx, irq_flags, msg, retval);
                return retval;
            }
            cpu_relax();
            loop_count += 1;
            header.full = sst_shim_read64((*sst_drv_ctx).shim, SST_IPCX);
        }
    } else {
        if list_empty(&(*sst_drv_ctx).ipc_dispatch_list) {
            /* queue is empty, nothing to send */
            spin_unlock_irqrestore(&mut (*sst_drv_ctx).ipc_spin_lock, irq_flags);
            dev_dbg((*sst_drv_ctx).dev, c"Empty msg queue... NO Action\n".as_ptr());
            return 0;
        }

        if header.p.header_high.part.busy != 0 {
            spin_unlock_irqrestore(&mut (*sst_drv_ctx).ipc_spin_lock, irq_flags);
            dev_dbg((*sst_drv_ctx).dev, c"Busy not free... post later\n".as_ptr());
            return 0;
        }

        /* copy msg from list */
        msg = container_of_ipc_post((*sst_drv_ctx).ipc_dispatch_list.next);
        list_del(&mut (*msg).node);
    }
    dev_dbg(
        (*sst_drv_ctx).dev,
        c"sst: Post message: header = %x\n".as_ptr(),
        (*msg).mrfld_header.p.header_high.full,
    );
    dev_dbg(
        (*sst_drv_ctx).dev,
        c"sst: size = 0x%x\n".as_ptr(),
        (*msg).mrfld_header.p.header_low_payload,
    );

    if (*msg).mrfld_header.p.header_high.part.large != 0 {
        memcpy_toio(
            (*sst_drv_ctx).mailbox.add(SST_MAILBOX_SEND) as *mut c_void,
            (*msg).mailbox_data as *const c_void,
            (*msg).mrfld_header.p.header_low_payload as usize,
        );
    }

    sst_shim_write64((*sst_drv_ctx).shim, SST_IPCX, (*msg).mrfld_header.full);

    spin_unlock_irqrestore(&mut (*sst_drv_ctx).ipc_spin_lock, irq_flags);
    kfree((*msg).mailbox_data as *mut c_void);
    kfree(msg as *mut c_void);
    retval
}

unsafe fn goto_out(
    sst_drv_ctx: *mut intel_sst_drv,
    irq_flags: c_ulong,
    msg: *mut ipc_post,
    retval: c_int,
) {
    spin_unlock_irqrestore(&mut (*sst_drv_ctx).ipc_spin_lock, irq_flags);
    kfree((*msg).mailbox_data as *mut c_void);
    kfree(msg as *mut c_void);
    let _ = retval;
}

pub unsafe extern "C" fn intel_sst_clear_intr_mrfld(sst_drv_ctx: *mut intel_sst_drv) {
    let mut isr = interrupt_reg_mrfld { full: 0 };
    let mut imr = interrupt_reg_mrfld { full: 0 };
    let mut clear_ipc = ipc_header_mrfld { full: 0 };
    let mut irq_flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*sst_drv_ctx).ipc_spin_lock, &mut irq_flags);
    imr.full = sst_shim_read64((*sst_drv_ctx).shim, SST_IMRX);
    isr.full = sst_shim_read64((*sst_drv_ctx).shim, SST_ISRX);

    /* write 1 to clear*/
    isr.part.busy_interrupt = 1;
    sst_shim_write64((*sst_drv_ctx).shim, SST_ISRX, isr.full);

    /* Set IA done bit */
    clear_ipc.full = sst_shim_read64((*sst_drv_ctx).shim, SST_IPCD);

    clear_ipc.p.header_high.part.busy = 0;
    clear_ipc.p.header_high.part.done = 1;
    clear_ipc.p.header_low_payload = IPC_ACK_SUCCESS;
    sst_shim_write64((*sst_drv_ctx).shim, SST_IPCD, clear_ipc.full);
    /* un mask busy interrupt */
    imr.part.busy_interrupt = 0;
    sst_shim_write64((*sst_drv_ctx).shim, SST_IMRX, imr.full);
    spin_unlock_irqrestore(&mut (*sst_drv_ctx).ipc_spin_lock, irq_flags);
}

/*
 * process_fw_init - process the FW init msg
 *
 * @msg: IPC message mailbox data from FW
 *
 * This function processes the FW init msg from FW
 * marks FW state and prints debug info of loaded FW
 */
unsafe extern "C" fn process_fw_init(sst_drv_ctx: *mut intel_sst_drv, msg: *mut c_void) {
    let init = msg as *mut ipc_header_fw_init;
    let mut retval: c_int = 0;

    dev_dbg((*sst_drv_ctx).dev, c"*** FW Init msg came***\n".as_ptr());
    if (*init).result != 0 {
        sst_set_fw_state_locked(sst_drv_ctx, SST_RESET);
        dev_err(
            (*sst_drv_ctx).dev,
            c"FW Init failed, Error %x\n".as_ptr(),
            (*init).result,
        );
        retval = (*init).result;
        sst_wake_up_block(sst_drv_ctx, retval, FW_DWNL_ID, 0, null_mut(), 0);
        return;
    }
    if memcmp(
        &(*sst_drv_ctx).fw_version as *const _ as *const c_void,
        &(*init).fw_version as *const _ as *const c_void,
        size_of_val(&(*init).fw_version),
    ) != 0
    {
        dev_info(
            (*sst_drv_ctx).dev,
            c"FW Version %02x.%02x.%02x.%02x\n".as_ptr(),
            (*init).fw_version.type_,
            (*init).fw_version.major,
            (*init).fw_version.minor,
            (*init).fw_version.build,
        );
    }
    dev_dbg(
        (*sst_drv_ctx).dev,
        c"Build date %s Time %s\n".as_ptr(),
        (*init).build_info.date,
        (*init).build_info.time,
    );

    /* Save FW version */
    (*sst_drv_ctx).fw_version.type_ = (*init).fw_version.type_;
    (*sst_drv_ctx).fw_version.major = (*init).fw_version.major;
    (*sst_drv_ctx).fw_version.minor = (*init).fw_version.minor;
    (*sst_drv_ctx).fw_version.build = (*init).fw_version.build;

    sst_wake_up_block(sst_drv_ctx, retval, FW_DWNL_ID, 0, null_mut(), 0);
}

unsafe fn size_of_val<T>(_: *const T) -> usize {
    size_of::<T>()
}

unsafe extern "C" fn process_fw_async_msg(sst_drv_ctx: *mut intel_sst_drv, msg: *mut ipc_post) {
    let msg_id: u32;
    let str_id: c_int;
    let data_size: u32;
    let mut i: u32;
    let data_offset: *mut c_void;
    let stream: *mut stream_info;
    let msg_low: u32;
    let pipe_id: u32;

    msg_low = (*msg).mrfld_header.p.header_low_payload;
    msg_id = read_unaligned((*msg).mailbox_data as *const ipc_dsp_hdr).cmd_id;
    data_offset = (*msg).mailbox_data.add(size_of::<ipc_dsp_hdr>()) as *mut c_void;
    data_size = msg_low.wrapping_sub(size_of::<ipc_dsp_hdr>() as u32);

    match msg_id {
        IPC_SST_PERIOD_ELAPSED_MRFLD => {
            pipe_id = read_unaligned((*msg).mailbox_data as *const ipc_dsp_hdr).pipe_id;
            str_id = get_stream_id_mrfld(sst_drv_ctx, pipe_id);
            if str_id > 0 {
                dev_dbg(
                    (*sst_drv_ctx).dev,
                    c"Period elapsed rcvd for pipe id 0x%x\n".as_ptr(),
                    pipe_id,
                );
                stream = (*sst_drv_ctx).streams.add(str_id as usize);
                /* If stream is dropped, skip processing this message*/
                if (*stream).status == STREAM_INIT {
                    return;
                }
                if let Some(period_elapsed) = (*stream).period_elapsed {
                    period_elapsed((*stream).pcm_substream);
                }
                if let Some(compr_cb) = (*stream).compr_cb {
                    compr_cb((*stream).compr_cb_param);
                }
            }
        }

        IPC_IA_DRAIN_STREAM_MRFLD => {
            pipe_id = read_unaligned((*msg).mailbox_data as *const ipc_dsp_hdr).pipe_id;
            str_id = get_stream_id_mrfld(sst_drv_ctx, pipe_id);
            if str_id > 0 {
                stream = (*sst_drv_ctx).streams.add(str_id as usize);
                if let Some(drain_notify) = (*stream).drain_notify {
                    drain_notify((*stream).drain_cb_param);
                }
            }
        }

        IPC_IA_FW_ASYNC_ERR_MRFLD => {
            dev_err((*sst_drv_ctx).dev, c"FW sent async error msg:\n".as_ptr());
            i = 0;
            while i < data_size / 4 {
                print_hex_dump(
                    KERN_DEBUG,
                    null_mut(),
                    DUMP_PREFIX_NONE,
                    16,
                    4,
                    data_offset,
                    data_size as usize,
                    false,
                );
                i += 1;
            }
        }

        IPC_IA_FW_INIT_CMPLT_MRFLD => {
            process_fw_init(sst_drv_ctx, data_offset);
        }

        IPC_IA_BUF_UNDER_RUN_MRFLD => {
            pipe_id = read_unaligned((*msg).mailbox_data as *const ipc_dsp_hdr).pipe_id;
            str_id = get_stream_id_mrfld(sst_drv_ctx, pipe_id);
            if str_id > 0 {
                dev_err(
                    (*sst_drv_ctx).dev,
                    c"Buffer under-run for pipe:%#x str_id:%d\n".as_ptr(),
                    pipe_id,
                    str_id,
                );
            }
        }

        _ => {
            dev_err(
                (*sst_drv_ctx).dev,
                c"Unrecognized async msg from FW msg_id %#x\n".as_ptr(),
                msg_id,
            );
        }
    }
}

pub unsafe extern "C" fn sst_process_reply_mrfld(
    sst_drv_ctx: *mut intel_sst_drv,
    msg: *mut ipc_post,
) {
    let drv_id: c_uint;
    let data: *mut c_void;
    let msg_high: ipc_header_high;
    let msg_low: u32;
    let dsp_hdr: *mut ipc_dsp_hdr;

    msg_high = read_unaligned(&(*msg).mrfld_header.p.header_high);
    msg_low = (*msg).mrfld_header.p.header_low_payload;

    dev_dbg(
        (*sst_drv_ctx).dev,
        c"IPC process message header %x payload %x\n".as_ptr(),
        (*msg).mrfld_header.p.header_high.full,
        (*msg).mrfld_header.p.header_low_payload,
    );

    drv_id = msg_high.part.drv_id;

    /* Check for async messages first */
    if drv_id == SST_ASYNC_DRV_ID {
        /*FW sent async large message*/
        process_fw_async_msg(sst_drv_ctx, msg);
        return;
    }

    /* FW sent short error response for an IPC */
    if msg_high.part.result != 0 && msg_high.part.large == 0 {
        /* 32-bit FW error code in msg_low */
        dev_err(
            (*sst_drv_ctx).dev,
            c"FW sent error response 0x%x".as_ptr(),
            msg_low,
        );
        sst_wake_up_block(
            sst_drv_ctx,
            msg_high.part.result,
            msg_high.part.drv_id,
            msg_high.part.msg_id,
            null_mut(),
            0,
        );
        return;
    }

    /*
     * Process all valid responses
     * if it is a large message, the payload contains the size to
     * copy from mailbox
     **/
    if msg_high.part.large != 0 {
        data = kmemdup((*msg).mailbox_data as *const c_void, msg_low as usize, GFP_KERNEL);
        if data.is_null() {
            return;
        }
        /* Copy command id so that we can use to put sst to reset */
        dsp_hdr = data as *mut ipc_dsp_hdr;
        dev_dbg((*sst_drv_ctx).dev, c"cmd_id %d\n".as_ptr(), (*dsp_hdr).cmd_id);
        if sst_wake_up_block(
            sst_drv_ctx,
            msg_high.part.result,
            msg_high.part.drv_id,
            msg_high.part.msg_id,
            data,
            msg_low,
        ) != 0
        {
            kfree(data);
        }
    } else {
        sst_wake_up_block(
            sst_drv_ctx,
            msg_high.part.result,
            msg_high.part.drv_id,
            msg_high.part.msg_id,
            null_mut(),
            0,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
