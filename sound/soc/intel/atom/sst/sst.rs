// SPDX-License-Identifier: GPL-2.0-only
/*
 *  sst.c - Intel SST Driver for audio engine
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

// C include dependencies:
// linux/module.h, linux/fs.h, linux/interrupt.h, linux/io.h,
// linux/firmware.h, linux/pci.h, linux/pm_runtime.h, linux/pm_qos.h,
// linux/async.h, linux/acpi.h, linux/sysfs.h, sound/core.h, sound/soc.h,
// asm/platform_sst_audio.h, ../sst-mfld-platform.h, sst.h
// MODULE_AUTHOR("Vinod Koul <vinod.koul@intel.com>");
// MODULE_AUTHOR("Harsha Priya <priya.harsha@intel.com>");
// MODULE_DESCRIPTION("Intel (R) SST(R) Audio Engine Driver");
// MODULE_LICENSE("GPL v2");

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type ssize_t = isize;
type u32 = u32;
type u64 = u64;
type irqreturn_t = c_uint;

const NULL: *mut c_void = ptr::null_mut();

extern "C" {
    static acpi_disabled: c_int;
    static THIS_MODULE: *mut module;
    static dev_attr_firmware_version: device_attribute;

    static mut mrfld_ops: intel_sst_ops;
    static mut sst_fw_version_attrs: [*const attribute; 2];
    static mut sst_fw_version_attr_group: attribute_group;

    fn sst_shim_read64(shim: *mut c_void, offset: c_uint) -> u64;
    fn sst_shim_write64(shim: *mut c_void, offset: c_uint, value: u64);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool_;
    fn sst_create_ipc_msg(msg: *mut *mut ipc_post, large: c_uint) -> c_int;
    fn memcpy_fromio(dst: *mut c_void, src: *const c_void, count: usize);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
    fn kfree(ptr: *const c_void);
    fn kvfree(ptr: *const c_void);
    fn sst_prepare_and_post_msg(
        sst: *mut intel_sst_drv,
        task_id: c_uint,
        ipc_msg: c_uint,
        cmd_id: c_uint,
        pipe_id: c_uint,
        len: c_uint,
        data: *mut c_void,
        block: *mut c_void,
        large: bool_,
        fill_dsp: bool_,
        sync: bool_,
        response: bool_,
    ) -> c_int;
    fn intel_sst_clear_intr_mrfld(drv: *mut intel_sst_drv);
    fn sst_start_mrfld(drv: *mut intel_sst_drv);
    fn intel_sst_reset_dsp_mrfld(drv: *mut intel_sst_drv);
    fn sst_post_message_mrfld(drv: *mut intel_sst_drv, msg: *mut ipc_post, sync: bool_) -> c_int;
    fn sst_process_reply_mrfld(drv: *mut intel_sst_drv, msg: *mut ipc_post);
    fn sst_alloc_stream_mrfld(drv: *mut intel_sst_drv, str_id: c_uint) -> c_int;
    fn sst_post_download_mrfld(drv: *mut intel_sst_drv) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn INIT_WORK(work: *mut work_struct, func: extern "C" fn(*mut work_struct));
    fn init_waitqueue_head(wq: *mut wait_queue_head_t);
    fn create_singlethread_workqueue(name: *const c_char) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn mutex_init(mutex: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn memcpy(dst: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, count: usize) -> *mut c_void;
    fn sst_set_fw_state_locked(ctx: *mut intel_sst_drv, state: c_uint);
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_uint,
        handler: Option<extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        thread_fn: Option<extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_ulong,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn cpu_latency_qos_add_request(req: *mut pm_qos_request, value: c_int);
    fn request_firmware_nowait(
        module: *mut module,
        uevent: bool_,
        name: *const c_char,
        device: *mut device,
        gfp: c_uint,
        context: *mut c_void,
        cont: extern "C" fn(*const firmware, *mut c_void),
    ) -> c_int;
    fn sst_firmware_load_cb(fw: *const firmware, context: *mut c_void);
    fn sysfs_create_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int;
    fn sysfs_remove_group(kobj: *mut kobject, grp: *const attribute_group);
    fn sst_register(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn sst_unregister(dev: *mut device);
    fn cpu_latency_qos_remove_request(req: *mut pm_qos_request);
    fn sst_memcpy_free_resources(ctx: *mut intel_sst_drv);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn synchronize_irq(irq: c_uint);
    fn flush_workqueue(wq: *mut workqueue_struct);
    fn sst_free_stream(ctx: *mut intel_sst_drv, str_id: c_int);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kvzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn memcpy32_fromio(dst: *mut c_void, src: *const c_void, count: usize);
    fn memcpy32_toio(dst: *mut c_void, src: *const c_void, count: usize);
    fn sst_create_block(ctx: *mut intel_sst_drv, pvt_id: c_uint, msg_id: c_uint) -> *mut sst_block;
    fn sst_wait_timeout(ctx: *mut intel_sst_drv, block: *mut sst_block) -> c_int;
    fn sst_realloc_stream(ctx: *mut intel_sst_drv, str_id: c_int);
    fn sst_free_block(ctx: *mut intel_sst_drv, block: *mut sst_block);
}

#[repr(C)]
struct module;
#[repr(C)]
struct firmware;
#[repr(C)]
struct workqueue_struct;
#[repr(C)]
struct sst_block;
#[repr(C)]
struct pm_qos_request;
#[repr(C)]
struct kobject;
#[repr(C)]
struct mutex;
#[repr(C)]
struct spinlock_t;
#[repr(C)]
struct wait_queue_head_t;

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    kobj: kobject,
}

#[repr(C)]
struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
struct device_attribute {
    attr: attribute,
}

#[repr(C)]
struct attribute_group {
    attrs: *mut *mut attribute,
}

#[repr(C)]
struct intel_sst_ops {
    interrupt: Option<extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
    irq_thread: Option<extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
    clear_interrupt: Option<extern "C" fn(*mut intel_sst_drv)>,
    start: Option<extern "C" fn(*mut intel_sst_drv)>,
    reset: Option<extern "C" fn(*mut intel_sst_drv)>,
    post_message: Option<extern "C" fn(*mut intel_sst_drv, *mut ipc_post, bool_) -> c_int>,
    process_reply: Option<extern "C" fn(*mut intel_sst_drv, *mut ipc_post)>,
    save_dsp_context: Option<extern "C" fn(*mut intel_sst_drv) -> c_int>,
    alloc_stream: Option<extern "C" fn(*mut intel_sst_drv, c_uint) -> c_int>,
    post_download: Option<extern "C" fn(*mut intel_sst_drv) -> c_int>,
    process_message: Option<extern "C" fn(*mut ipc_post)>,
}

#[repr(C)]
struct ipc_post {
    node: list_head,
    mailbox_data: *mut c_void,
    mrfld_header: ipc_header_mrfld,
    is_process_reply: bool_,
    is_large: bool_,
}

#[repr(C)]
struct fw_version {
    type_: c_uint,
    major: c_uint,
    minor: c_uint,
    build: c_uint,
}

#[repr(C)]
struct sst_info {
    max_streams: c_int,
}

#[repr(C)]
struct sst_platform_data {
    probe_data: *const c_void,
    ipc_info: *mut ipc_info,
    streams_lost_on_suspend: bool_,
}

#[repr(C)]
struct ipc_info {
    mbox_recv_off: c_uint,
    ipc_offset: c_uint,
}

#[repr(C)]
struct ipc_reg {
    ipcx: c_uint,
    ipcd: c_uint,
}

#[repr(C)]
struct stream_info {
    pipe_id: c_uint,
    lock: mutex,
    status: c_int,
    prev: c_int,
    resume_status: c_int,
    resume_prev: c_int,
}

#[repr(C)]
struct sst_fw_sg_list {
    src: *mut c_void,
    dst: *mut c_void,
    list_len: c_uint,
}

#[repr(C)]
struct sst_fw_save {
    iram: *mut c_void,
    dram: *mut c_void,
    sram: *mut c_void,
    ddr: *mut c_void,
}

#[repr(C)]
struct intel_sst_drv {
    shim: *mut c_void,
    ipc_reg: ipc_reg,
    ipc_spin_lock: spinlock_t,
    post_msg_wq: *mut workqueue_struct,
    ipc_post_msg_wq: work_struct,
    rx_msg_lock: spinlock_t,
    rx_list: list_head,
    ops: *mut intel_sst_ops,
    dev: *mut device,
    dev_id: c_uint,
    tstamp: c_uint,
    memcpy_list: list_head,
    ipc_dispatch_list: list_head,
    block_list: list_head,
    wait_queue: wait_queue_head_t,
    sst_lock: mutex,
    block_lock: spinlock_t,
    pdata: *mut sst_platform_data,
    info: sst_info,
    pvt_id: c_uint,
    stream_cnt: c_uint,
    fw_in_mem: *mut c_void,
    use_dma: c_uint,
    use_lli: c_uint,
    mailbox_recv_offset: c_uint,
    mailbox: *mut c_void,
    irq_num: c_uint,
    qos: *mut pm_qos_request,
    firmware_name: *const c_char,
    fw_version: fw_version,
    fw_sg_list: sst_fw_sg_list,
    sst_state: c_uint,
    streams: [stream_info; 1],
    fw_save: *mut sst_fw_save,
    iram: *mut c_void,
    iram_base: usize,
    iram_end: usize,
    dram: *mut c_void,
    dram_base: usize,
    dram_end: usize,
    ddr: *mut c_void,
    ddr_base: usize,
    ddr_end: usize,
}

#[repr(C)]
struct interrupt_reg_mrfld_part {
    done_interrupt: c_uint,
    busy_interrupt: c_uint,
}

#[repr(C)]
union interrupt_reg_mrfld {
    full: u64,
    part: interrupt_reg_mrfld_part,
}

#[repr(C)]
struct sst_imr_reg_mrfld_part {
    busy_interrupt: c_uint,
}

#[repr(C)]
union sst_imr_reg_mrfld {
    full: u64,
    part: sst_imr_reg_mrfld_part,
}

#[repr(C)]
struct ipc_header_mrfld_high_part {
    done: c_uint,
    large: c_uint,
    msg_id: c_uint,
}

#[repr(C)]
struct ipc_header_mrfld_high {
    part: ipc_header_mrfld_high_part,
}

#[repr(C)]
struct ipc_header_mrfld_p {
    header_high: ipc_header_mrfld_high,
    header_low_payload: c_uint,
}

#[repr(C)]
union ipc_header_mrfld {
    full: u64,
    p: ipc_header_mrfld_p,
}

#[repr(C)]
struct dev_pm_ops {
    suspend: Option<extern "C" fn(*mut device) -> c_int>,
    resume: Option<extern "C" fn(*mut device) -> c_int>,
    runtime_suspend: Option<extern "C" fn(*mut device) -> c_int>,
}

const PROCESS_MSG: u32 = 0;
const SST_MAILBOX_SIZE: c_uint = 0;
const SST_ISRX: c_uint = 0;
const SST_IMRX: c_uint = 0;
const SST_IPCX: c_uint = 0;
const SST_IPCD: c_uint = 0;
const SST_DRV_NAME: *const c_char = b"sst\0".as_ptr() as *const c_char;
const SST_TASK_ID_MEDIA: c_uint = 0;
const IPC_CMD: c_uint = 0;
const IPC_PREP_D3: c_uint = 0;
const PIPE_RSVD: c_uint = 0;
const PCI_DEVICE_ID_INTEL_SST_TNG: c_uint = 0;
const PCI_DEVICE_ID_INTEL_SST_BYT: c_uint = 0;
const PCI_DEVICE_ID_INTEL_SST_BSW: c_uint = 0;
const SST_TIME_STAMP_MRFLD: c_uint = 0;
const SST_RESET: c_uint = 0;
const SST_SHUTDOWN: c_uint = 0;
const SST_FW_LOADING: c_uint = 0;
const SST_FW_RUNNING: c_uint = 0;
const SST_SUSPEND_DELAY: c_int = 0;
const STREAM_RUNNING: c_int = 0;
const STREAM_UN_INIT: c_int = 0;
const FW_DWNL_ID: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const PM_QOS_DEFAULT_VALUE: c_int = 0;
const IRQ_HANDLED: irqreturn_t = 0;
const IRQ_WAKE_THREAD: irqreturn_t = 0;
const EIO: c_int = 5;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;

#[inline]
unsafe fn sst_is_process_reply(msg_id: u32) -> bool_ {
    if (msg_id & PROCESS_MSG) != 0 { true } else { false }
}

#[inline]
unsafe fn sst_validate_mailbox_size(size: c_uint) -> bool_ {
    if size <= SST_MAILBOX_SIZE { true } else { false }
}

extern "C" fn intel_sst_interrupt_mrfld(irq: c_int, context: *mut c_void) -> irqreturn_t {
    unsafe {
        let mut isr: interrupt_reg_mrfld = interrupt_reg_mrfld { full: 0 };
        let mut header: ipc_header_mrfld = ipc_header_mrfld { full: 0 };
        let mut imr: sst_imr_reg_mrfld = sst_imr_reg_mrfld { full: 0 };
        let mut msg: *mut ipc_post = ptr::null_mut();
        let size: c_uint;
        let drv: *mut intel_sst_drv = context as *mut intel_sst_drv;
        let mut retval: irqreturn_t = IRQ_HANDLED;

        /* Interrupt arrived, check src */
        isr.full = sst_shim_read64((*drv).shim, SST_ISRX);

        if isr.part.done_interrupt != 0 {
            /* Clear done bit */
            spin_lock(&mut (*drv).ipc_spin_lock);
            header.full = sst_shim_read64((*drv).shim, (*drv).ipc_reg.ipcx);
            header.p.header_high.part.done = 0;
            sst_shim_write64((*drv).shim, (*drv).ipc_reg.ipcx, header.full);

            /* write 1 to clear status register */
            isr.part.done_interrupt = 1;
            sst_shim_write64((*drv).shim, SST_ISRX, isr.full);
            spin_unlock(&mut (*drv).ipc_spin_lock);

            /* we can send more messages to DSP so trigger work */
            queue_work((*drv).post_msg_wq, &mut (*drv).ipc_post_msg_wq);
            retval = IRQ_HANDLED;
        }

        if isr.part.busy_interrupt != 0 {
            /* message from dsp so copy that */
            spin_lock(&mut (*drv).ipc_spin_lock);
            imr.full = sst_shim_read64((*drv).shim, SST_IMRX);
            imr.part.busy_interrupt = 1;
            sst_shim_write64((*drv).shim, SST_IMRX, imr.full);
            spin_unlock(&mut (*drv).ipc_spin_lock);
            header.full = sst_shim_read64((*drv).shim, (*drv).ipc_reg.ipcd);

            if sst_create_ipc_msg(&mut msg, header.p.header_high.part.large) != 0 {
                ((*(*drv).ops).clear_interrupt.unwrap())(drv);
                return IRQ_HANDLED;
            }

            if header.p.header_high.part.large != 0 {
                size = header.p.header_low_payload;
                if sst_validate_mailbox_size(size) {
                    memcpy_fromio(
                        (*msg).mailbox_data,
                        ((*drv).mailbox as *mut u8).add((*drv).mailbox_recv_offset as usize)
                            as *const c_void,
                        size as usize,
                    );
                } else {
                    dev_err(
                        (*drv).dev,
                        b"Mailbox not copied, payload size is: %u\n\0".as_ptr() as *const c_char,
                        size,
                    );
                    header.p.header_low_payload = 0;
                }
            }

            (*msg).mrfld_header = header;
            (*msg).is_process_reply =
                sst_is_process_reply(header.p.header_high.part.msg_id);
            spin_lock(&mut (*drv).rx_msg_lock);
            list_add_tail(&mut (*msg).node, &mut (*drv).rx_list);
            spin_unlock(&mut (*drv).rx_msg_lock);
            ((*(*drv).ops).clear_interrupt.unwrap())(drv);
            retval = IRQ_WAKE_THREAD;
        }
        retval
    }
}

extern "C" fn intel_sst_irq_thread_mrfld(irq: c_int, context: *mut c_void) -> irqreturn_t {
    unsafe {
        let drv: *mut intel_sst_drv = context as *mut intel_sst_drv;
        let mut __msg: *mut ipc_post;
        let mut msg: *mut ipc_post;
        let mut irq_flags: c_ulong = 0;

        spin_lock_irqsave(&mut (*drv).rx_msg_lock, irq_flags);
        if list_empty(&(*drv).rx_list) != 0 {
            spin_unlock_irqrestore(&mut (*drv).rx_msg_lock, irq_flags);
            return IRQ_HANDLED;
        }

        msg = (*drv).rx_list.next as *mut ipc_post;
        while &mut (*msg).node as *mut list_head != &mut (*drv).rx_list as *mut list_head {
            __msg = (*msg).node.next as *mut ipc_post;
            list_del(&mut (*msg).node);
            spin_unlock_irqrestore(&mut (*drv).rx_msg_lock, irq_flags);
            if (*msg).is_process_reply {
                ((*(*drv).ops).process_message.unwrap())(msg);
            } else {
                ((*(*drv).ops).process_reply.unwrap())(drv, msg);
            }

            if (*msg).is_large {
                kfree((*msg).mailbox_data);
            }
            kfree(msg as *const c_void);
            spin_lock_irqsave(&mut (*drv).rx_msg_lock, irq_flags);
            msg = __msg;
        }
        spin_unlock_irqrestore(&mut (*drv).rx_msg_lock, irq_flags);
        IRQ_HANDLED
    }
}

extern "C" fn sst_save_dsp_context_v2(sst: *mut intel_sst_drv) -> c_int {
    unsafe {
        let mut ret: c_int = 0;

        ret = sst_prepare_and_post_msg(
            sst,
            SST_TASK_ID_MEDIA,
            IPC_CMD,
            IPC_PREP_D3,
            PIPE_RSVD,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            true,
            true,
            false,
            true,
        );

        if ret < 0 {
            dev_err((*sst).dev, b"not suspending FW!!, Err: %d\n\0".as_ptr() as *const c_char, ret);
            return -EIO;
        }

        0
    }
}

static mut mrfld_ops_init: intel_sst_ops = intel_sst_ops {
    interrupt: Some(intel_sst_interrupt_mrfld),
    irq_thread: Some(intel_sst_irq_thread_mrfld),
    clear_interrupt: Some(intel_sst_clear_intr_mrfld),
    start: Some(sst_start_mrfld),
    reset: Some(intel_sst_reset_dsp_mrfld),
    post_message: Some(sst_post_message_mrfld),
    process_reply: Some(sst_process_reply_mrfld),
    save_dsp_context: Some(sst_save_dsp_context_v2),
    alloc_stream: Some(sst_alloc_stream_mrfld),
    post_download: Some(sst_post_download_mrfld),
    process_message: None,
};

#[no_mangle]
pub extern "C" fn sst_driver_ops(sst: *mut intel_sst_drv) -> c_int {
    unsafe {
        match (*sst).dev_id {
            PCI_DEVICE_ID_INTEL_SST_TNG
            | PCI_DEVICE_ID_INTEL_SST_BYT
            | PCI_DEVICE_ID_INTEL_SST_BSW => {
                (*sst).tstamp = SST_TIME_STAMP_MRFLD;
                (*sst).ops = &mut mrfld_ops_init;
                0
            }
            _ => {
                dev_err(
                    (*sst).dev,
                    b"SST Driver capabilities missing for dev_id: %x\0".as_ptr() as *const c_char,
                    (*sst).dev_id,
                );
                -EINVAL
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn sst_process_pending_msg(work: *mut work_struct) {
    unsafe {
        let ctx: *mut intel_sst_drv = (work as *mut u8).sub(0) as *mut intel_sst_drv;

        ((*(*ctx).ops).post_message.unwrap())(ctx, ptr::null_mut(), false);
    }
}

unsafe fn sst_workqueue_init(ctx: *mut intel_sst_drv) -> c_int {
    INIT_LIST_HEAD(&mut (*ctx).memcpy_list);
    INIT_LIST_HEAD(&mut (*ctx).rx_list);
    INIT_LIST_HEAD(&mut (*ctx).ipc_dispatch_list);
    INIT_LIST_HEAD(&mut (*ctx).block_list);
    INIT_WORK(&mut (*ctx).ipc_post_msg_wq, sst_process_pending_msg);
    init_waitqueue_head(&mut (*ctx).wait_queue);

    (*ctx).post_msg_wq = create_singlethread_workqueue(b"sst_post_msg_wq\0".as_ptr() as *const c_char);
    if (*ctx).post_msg_wq.is_null() {
        return -EBUSY;
    }
    0
}

unsafe fn sst_init_locks(ctx: *mut intel_sst_drv) {
    mutex_init(&mut (*ctx).sst_lock);
    spin_lock_init(&mut (*ctx).rx_msg_lock);
    spin_lock_init(&mut (*ctx).ipc_spin_lock);
    spin_lock_init(&mut (*ctx).block_lock);
}

/*
 * Driver handles PCI IDs in ACPI - sst_acpi_probe() - and we are using only
 * device ID part. If real ACPI ID appears, the kstrtouint() returns error, so
 * we are fine with using unsigned short as dev_id type.
 */
#[no_mangle]
pub extern "C" fn sst_alloc_drv_context(
    ctx: *mut *mut intel_sst_drv,
    dev: *mut device,
    dev_id: u16,
) -> c_int {
    unsafe {
        *ctx = devm_kzalloc(dev, size_of::<intel_sst_drv>(), GFP_KERNEL) as *mut intel_sst_drv;
        if (*ctx).is_null() {
            return -ENOMEM;
        }

        (**ctx).dev = dev;
        (**ctx).dev_id = dev_id as c_uint;

        0
    }
}
// EXPORT_SYMBOL_GPL(sst_alloc_drv_context);

extern "C" fn firmware_version_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    unsafe {
        let ctx: *mut intel_sst_drv = dev_get_drvdata(dev) as *mut intel_sst_drv;

        if (*ctx).fw_version.type_ == 0
            && (*ctx).fw_version.major == 0
            && (*ctx).fw_version.minor == 0
            && (*ctx).fw_version.build == 0
        {
            sysfs_emit(buf, b"FW not yet loaded\n\0".as_ptr() as *const c_char)
        } else {
            sysfs_emit(
                buf,
                b"v%02x.%02x.%02x.%02x\n\0".as_ptr() as *const c_char,
                (*ctx).fw_version.type_,
                (*ctx).fw_version.major,
                (*ctx).fw_version.minor,
                (*ctx).fw_version.build,
            )
        }
    }
}

// DEVICE_ATTR_RO(firmware_version);

static mut sst_fw_version_attrs_init: [*const attribute; 2] = unsafe {
    [
        &dev_attr_firmware_version.attr as *const attribute,
        ptr::null(),
    ]
};

static mut sst_fw_version_attr_group_init: attribute_group = attribute_group {
    attrs: unsafe { sst_fw_version_attrs_init.as_mut_ptr() as *mut *mut attribute },
};

#[no_mangle]
pub extern "C" fn sst_context_init(ctx: *mut intel_sst_drv) -> c_int {
    unsafe {
        let mut ret: c_int = 0;
        let mut i: c_int;

        if (*ctx).pdata.is_null() {
            return -EINVAL;
        }

        if (*(*ctx).pdata).probe_data.is_null() {
            return -EINVAL;
        }

        memcpy(
            &mut (*ctx).info as *mut sst_info as *mut c_void,
            (*(*ctx).pdata).probe_data,
            size_of::<sst_info>(),
        );

        ret = sst_driver_ops(ctx);
        if ret != 0 {
            return -EINVAL;
        }

        sst_init_locks(ctx);
        sst_set_fw_state_locked(ctx, SST_RESET);

        /* pvt_id 0 reserved for async messages */
        (*ctx).pvt_id = 1;
        (*ctx).stream_cnt = 0;
        (*ctx).fw_in_mem = ptr::null_mut();
        /* we use memcpy, so set to 0 */
        (*ctx).use_dma = 0;
        (*ctx).use_lli = 0;

        if sst_workqueue_init(ctx) != 0 {
            return -EINVAL;
        }

        (*ctx).mailbox_recv_offset = (*(*(*ctx).pdata).ipc_info).mbox_recv_off;
        (*ctx).ipc_reg.ipcx = SST_IPCX + (*(*(*ctx).pdata).ipc_info).ipc_offset;
        (*ctx).ipc_reg.ipcd = SST_IPCD + (*(*(*ctx).pdata).ipc_info).ipc_offset;

        dev_info(
            (*ctx).dev,
            b"Got drv data max stream %d\n\0".as_ptr() as *const c_char,
            (*ctx).info.max_streams,
        );

        i = 1;
        while i <= (*ctx).info.max_streams {
            let stream: *mut stream_info = (*ctx).streams.as_mut_ptr().add(i as usize);

            memset(stream as *mut c_void, 0, size_of::<stream_info>());
            (*stream).pipe_id = PIPE_RSVD;
            mutex_init(&mut (*stream).lock);
            i += 1;
        }

        /* Register the ISR */
        ret = devm_request_threaded_irq(
            (*ctx).dev,
            (*ctx).irq_num,
            (*(*ctx).ops).interrupt,
            (*(*ctx).ops).irq_thread,
            0,
            SST_DRV_NAME,
            ctx as *mut c_void,
        );
        if ret != 0 {
            goto_do_free_mem(ctx, ret)
        } else {
            dev_dbg((*ctx).dev, b"Registered IRQ %#x\n\0".as_ptr() as *const c_char, (*ctx).irq_num);

            /* default intr are unmasked so set this as masked */
            sst_shim_write64((*ctx).shim, SST_IMRX, 0xFFFF0038);

            (*ctx).qos =
                devm_kzalloc((*ctx).dev, size_of::<pm_qos_request>(), GFP_KERNEL) as *mut pm_qos_request;
            if (*ctx).qos.is_null() {
                ret = -ENOMEM;
                goto_do_free_mem(ctx, ret)
            } else {
                cpu_latency_qos_add_request((*ctx).qos, PM_QOS_DEFAULT_VALUE);

                dev_dbg((*ctx).dev, b"Requesting FW %s now...\n\0".as_ptr() as *const c_char, (*ctx).firmware_name);
                ret = request_firmware_nowait(
                    THIS_MODULE,
                    true,
                    (*ctx).firmware_name,
                    (*ctx).dev,
                    GFP_KERNEL,
                    ctx as *mut c_void,
                    sst_firmware_load_cb,
                );
                if ret != 0 {
                    dev_err((*ctx).dev, b"Firmware download failed:%d\n\0".as_ptr() as *const c_char, ret);
                    goto_do_free_mem(ctx, ret)
                } else {
                    ret = sysfs_create_group(&mut (*(*ctx).dev).kobj, &sst_fw_version_attr_group_init);
                    if ret != 0 {
                        dev_err((*ctx).dev, b"Unable to create sysfs\n\0".as_ptr() as *const c_char);
                        sysfs_remove_group(&mut (*(*ctx).dev).kobj, &sst_fw_version_attr_group_init);
                        destroy_workqueue((*ctx).post_msg_wq);
                        ret
                    } else {
                        sst_register((*ctx).dev);
                        0
                    }
                }
            }
        }
    }
}

unsafe fn goto_do_free_mem(ctx: *mut intel_sst_drv, ret: c_int) -> c_int {
    destroy_workqueue((*ctx).post_msg_wq);
    ret
}
// EXPORT_SYMBOL_GPL(sst_context_init);

#[no_mangle]
pub extern "C" fn sst_context_cleanup(ctx: *mut intel_sst_drv) {
    unsafe {
        pm_runtime_get_noresume((*ctx).dev);
        pm_runtime_disable((*ctx).dev);
        sst_unregister((*ctx).dev);
        sst_set_fw_state_locked(ctx, SST_SHUTDOWN);
        sysfs_remove_group(&mut (*(*ctx).dev).kobj, &sst_fw_version_attr_group_init);
        destroy_workqueue((*ctx).post_msg_wq);
        cpu_latency_qos_remove_request((*ctx).qos);
        kfree((*ctx).fw_sg_list.src);
        kfree((*ctx).fw_sg_list.dst);
        (*ctx).fw_sg_list.list_len = 0;
        kfree((*ctx).fw_in_mem);
        (*ctx).fw_in_mem = ptr::null_mut();
        sst_memcpy_free_resources(ctx);
    }
}
// EXPORT_SYMBOL_GPL(sst_context_cleanup);

#[no_mangle]
pub extern "C" fn sst_configure_runtime_pm(ctx: *mut intel_sst_drv) {
    unsafe {
        pm_runtime_set_autosuspend_delay((*ctx).dev, SST_SUSPEND_DELAY);
        pm_runtime_use_autosuspend((*ctx).dev);
        /*
         * For acpi devices, the actual physical device state is
         * initially active. So change the state to active before
         * enabling the pm
         */

        if acpi_disabled == 0 {
            pm_runtime_set_active((*ctx).dev);
        }

        pm_runtime_enable((*ctx).dev);

        if acpi_disabled != 0 {
            pm_runtime_set_active((*ctx).dev);
        } else {
            pm_runtime_put_noidle((*ctx).dev);
        }
    }
}
// EXPORT_SYMBOL_GPL(sst_configure_runtime_pm);

extern "C" fn intel_sst_runtime_suspend(dev: *mut device) -> c_int {
    unsafe {
        let mut ret: c_int = 0;
        let ctx: *mut intel_sst_drv = dev_get_drvdata(dev) as *mut intel_sst_drv;

        if (*ctx).sst_state == SST_RESET {
            dev_dbg(dev, b"LPE is already in RESET state, No action\n\0".as_ptr() as *const c_char);
            return 0;
        }
        /* save fw context */
        if ((*(*ctx).ops).save_dsp_context.unwrap())(ctx) != 0 {
            return -EBUSY;
        }

        /* Move the SST state to Reset */
        sst_set_fw_state_locked(ctx, SST_RESET);

        synchronize_irq((*ctx).irq_num);
        flush_workqueue((*ctx).post_msg_wq);

        ((*(*ctx).ops).reset.unwrap())(ctx);

        ret
    }
}

extern "C" fn intel_sst_suspend(dev: *mut device) -> c_int {
    unsafe {
        let ctx: *mut intel_sst_drv = dev_get_drvdata(dev) as *mut intel_sst_drv;
        let mut fw_save: *mut sst_fw_save;
        let mut i: c_int;
        let mut ret: c_int;

        /* check first if we are already in SW reset */
        if (*ctx).sst_state == SST_RESET {
            return 0;
        }

        /*
         * check if any stream is active and running
         * they should already by suspend by soc_suspend
         */
        i = 1;
        while i <= (*ctx).info.max_streams {
            let stream: *mut stream_info = (*ctx).streams.as_mut_ptr().add(i as usize);

            if (*stream).status == STREAM_RUNNING {
                dev_err(
                    dev,
                    b"stream %d is running, can't suspend, abort\n\0".as_ptr() as *const c_char,
                    i,
                );
                return -EBUSY;
            }

            if (*(*ctx).pdata).streams_lost_on_suspend {
                (*stream).resume_status = (*stream).status;
                (*stream).resume_prev = (*stream).prev;
                if (*stream).status != STREAM_UN_INIT {
                    sst_free_stream(ctx, i);
                }
            }
            i += 1;
        }
        synchronize_irq((*ctx).irq_num);
        flush_workqueue((*ctx).post_msg_wq);

        /* Move the SST state to Reset */
        sst_set_fw_state_locked(ctx, SST_RESET);

        /* tell DSP we are suspending */
        if ((*(*ctx).ops).save_dsp_context.unwrap())(ctx) != 0 {
            return -EBUSY;
        }

        /* save the memories */
        fw_save = kzalloc(size_of::<sst_fw_save>(), GFP_KERNEL) as *mut sst_fw_save;
        if fw_save.is_null() {
            return -ENOMEM;
        }
        (*fw_save).iram = kvzalloc((*ctx).iram_end - (*ctx).iram_base, GFP_KERNEL);
        if (*fw_save).iram.is_null() {
            ret = -ENOMEM;
            kfree(fw_save as *const c_void);
            return ret;
        }
        (*fw_save).dram = kvzalloc((*ctx).dram_end - (*ctx).dram_base, GFP_KERNEL);
        if (*fw_save).dram.is_null() {
            ret = -ENOMEM;
            kvfree((*fw_save).iram);
            kfree(fw_save as *const c_void);
            return ret;
        }
        (*fw_save).sram = kvzalloc(SST_MAILBOX_SIZE as usize, GFP_KERNEL);
        if (*fw_save).sram.is_null() {
            ret = -ENOMEM;
            kvfree((*fw_save).dram);
            kvfree((*fw_save).iram);
            kfree(fw_save as *const c_void);
            return ret;
        }

        (*fw_save).ddr = kvzalloc((*ctx).ddr_end - (*ctx).ddr_base, GFP_KERNEL);
        if (*fw_save).ddr.is_null() {
            ret = -ENOMEM;
            kvfree((*fw_save).sram);
            kvfree((*fw_save).dram);
            kvfree((*fw_save).iram);
            kfree(fw_save as *const c_void);
            return ret;
        }

        memcpy32_fromio((*fw_save).iram, (*ctx).iram, (*ctx).iram_end - (*ctx).iram_base);
        memcpy32_fromio((*fw_save).dram, (*ctx).dram, (*ctx).dram_end - (*ctx).dram_base);
        memcpy32_fromio((*fw_save).sram, (*ctx).mailbox, SST_MAILBOX_SIZE as usize);
        memcpy32_fromio((*fw_save).ddr, (*ctx).ddr, (*ctx).ddr_end - (*ctx).ddr_base);

        (*ctx).fw_save = fw_save;
        ((*(*ctx).ops).reset.unwrap())(ctx);
        0
    }
}

extern "C" fn intel_sst_resume(dev: *mut device) -> c_int {
    unsafe {
        let ctx: *mut intel_sst_drv = dev_get_drvdata(dev) as *mut intel_sst_drv;
        let fw_save: *mut sst_fw_save = (*ctx).fw_save;
        let mut block: *mut sst_block;
        let mut i: c_int;
        let mut ret: c_int = 0;

        if fw_save.is_null() {
            return 0;
        }

        sst_set_fw_state_locked(ctx, SST_FW_LOADING);

        /* we have to restore the memory saved */
        ((*(*ctx).ops).reset.unwrap())(ctx);

        (*ctx).fw_save = ptr::null_mut();

        memcpy32_toio((*ctx).iram, (*fw_save).iram, (*ctx).iram_end - (*ctx).iram_base);
        memcpy32_toio((*ctx).dram, (*fw_save).dram, (*ctx).dram_end - (*ctx).dram_base);
        memcpy32_toio((*ctx).mailbox, (*fw_save).sram, SST_MAILBOX_SIZE as usize);
        memcpy32_toio((*ctx).ddr, (*fw_save).ddr, (*ctx).ddr_end - (*ctx).ddr_base);

        kvfree((*fw_save).sram);
        kvfree((*fw_save).dram);
        kvfree((*fw_save).iram);
        kvfree((*fw_save).ddr);
        kfree(fw_save as *const c_void);

        block = sst_create_block(ctx, 0, FW_DWNL_ID);
        if block.is_null() {
            return -ENOMEM;
        }

        /* start and wait for ack */
        ((*(*ctx).ops).start.unwrap())(ctx);
        ret = sst_wait_timeout(ctx, block);
        if ret != 0 {
            dev_err((*ctx).dev, b"fw download failed %d\n\0".as_ptr() as *const c_char, ret);
            /* FW download failed due to timeout */
            ret = -EBUSY;
        } else {
            sst_set_fw_state_locked(ctx, SST_FW_RUNNING);
        }

        if (*(*ctx).pdata).streams_lost_on_suspend {
            i = 1;
            while i <= (*ctx).info.max_streams {
                let stream: *mut stream_info = (*ctx).streams.as_mut_ptr().add(i as usize);

                if (*stream).resume_status != STREAM_UN_INIT {
                    dev_dbg(
                        (*ctx).dev,
                        b"Re-allocing stream %d status %d prev %d\n\0".as_ptr() as *const c_char,
                        i,
                        (*stream).resume_status,
                        (*stream).resume_prev,
                    );
                    sst_realloc_stream(ctx, i);
                    (*stream).status = (*stream).resume_status;
                    (*stream).prev = (*stream).resume_prev;
                }
                i += 1;
            }
        }

        sst_free_block(ctx, block);
        ret
    }
}

#[no_mangle]
pub static intel_sst_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(intel_sst_suspend),
    resume: Some(intel_sst_resume),
    runtime_suspend: Some(intel_sst_runtime_suspend),
};
// EXPORT_SYMBOL_GPL(intel_sst_pm);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
