// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2020 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub const CATPT_IPC_TIMEOUT_MS: c_int = 300;

pub type u8 = u8;
pub type u32 = u32;
pub type u64 = u64;
pub type irqreturn_t = c_uint;
pub type c_uint = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_fw_ready {
    pub outbox_size: u32,
}

#[repr(C)]
pub struct catpt_reply {
    pub status: c_int,
}

#[repr(C)]
pub struct catpt_ipc_msg {
    pub header: u32,
    pub data: *mut u8,
    pub size: usize,
    pub rsp: catpt_reply,
}

#[repr(C)]
pub struct catpt_ipc {
    pub dev: *mut device,
    pub ready: bool,
    pub default_timeout: c_int,
    pub done_completion: completion,
    pub busy_completion: completion,
    pub lock: spinlock_t,
    pub mutex: mutex,
    pub rx: catpt_ipc_msg,
    pub config: catpt_fw_ready,
}

#[repr(C)]
pub struct catpt_dev {
    pub ipc: catpt_ipc,
    pub dev: *mut device,
    pub stream_mutex: mutex,
    pub fw_ready: completion,
    pub lpe_ba: *mut u8,
}

#[repr(C)]
pub struct catpt_stream_runtime {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_notify_msg {
    pub fw_ready: bool,
    pub mailbox_address: u32,
    pub global_msg_type: c_int,
    pub stream_msg_type: c_int,
    pub stream_hw_id: c_int,
    pub notify_reason: c_int,
}

#[repr(C)]
pub struct catpt_notify_position {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_notify_glitch {
    pub type_: c_int,
    pub presentation_pos: u64,
    pub write_pos: u32,
}

unsafe extern "C" {
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;
    static EPERM: c_int;
    static EINVAL: c_int;
    static ETIMEDOUT: c_int;

    static CATPT_IPCC_BUSY: u32;
    static CATPT_REPLY_PENDING: c_int;
    static CATPT_REPLY_SUCCESS: c_int;
    static CATPT_NOTIFY_POSITION_CHANGED: c_int;
    static CATPT_NOTIFY_GLITCH_OCCURRED: c_int;
    static CATPT_GLB_REQUEST_CORE_DUMP: c_int;
    static CATPT_GLB_STREAM_MESSAGE: c_int;
    static CATPT_STRM_NOTIFICATION: c_int;
    static COREDUMP: c_int;
    static CATPT_COREDUMP_REQUEST: u32;
    static CATPT_COREDUMP_RELEASE: u32;
    static IPCD: c_int;
    static CATPT_IPCD_BUSY: u32;
    static CATPT_IPCD_DONE: u32;
    static IMC: c_int;
    static CATPT_IMC_IPCDB: u32;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static ISC: c_int;
    static IPCC: c_int;
    static CATPT_ISC_IPCCD: u32;
    static CATPT_IMC_IPCCD: u32;
    static CATPT_IPCC_DONE: u32;
    static CATPT_ISC_IPCDB: u32;
    static IRQ_WAKE_THREAD: irqreturn_t;

    fn init_completion(x: *mut completion);
    fn reinit_completion(x: *mut completion);
    fn complete(x: *mut completion);
    fn wait_for_completion_timeout(x: *mut completion, timeout: c_ulong) -> c_ulong;
    fn msecs_to_jiffies(m: c_int) -> c_ulong;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
    fn memcpy_toio(dst: *mut c_void, src: *const c_void, size: usize);
    fn memcpy_fromio(dst: *mut c_void, src: *const c_void, size: usize);
    fn catpt_outbox_addr(cdev: *mut catpt_dev) -> *mut c_void;
    fn catpt_inbox_addr(cdev: *mut catpt_dev) -> *mut c_void;
    fn catpt_writel_shim(cdev: *mut catpt_dev, reg: c_int, val: u32);
    fn catpt_readl_shim(cdev: *mut catpt_dev, reg: c_int) -> u32;
    fn catpt_updatel_shim(cdev: *mut catpt_dev, reg: c_int, mask: u32, val: u32);
    fn catpt_readl_dram(cdev: *mut catpt_dev, reg: c_int) -> u32;
    fn catpt_writel_dram(cdev: *mut catpt_dev, reg: c_int, val: u32);
    fn catpt_coredump(cdev: *mut catpt_dev);
    fn catpt_stream_find(cdev: *mut catpt_dev, stream_hw_id: c_int) -> *mut catpt_stream_runtime;
    fn catpt_stream_update_position(
        cdev: *mut catpt_dev,
        stream: *mut catpt_stream_runtime,
        pos: *mut catpt_notify_position,
    );
    fn CATPT_MSG(header: u32) -> catpt_notify_msg;
    fn trace_catpt_ipc_request(header: u32);
    fn trace_catpt_ipc_payload(data: *const u8, size: usize);
    fn trace_catpt_ipc_notify(header: u32);
    fn trace_catpt_irq(isc: u32);
    fn trace_catpt_ipc_reply(header: u32);
    fn dev_crit(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn catpt_ipc_init(ipc: *mut catpt_ipc, dev: *mut device) {
    unsafe {
        (*ipc).dev = dev;
        (*ipc).ready = false;
        (*ipc).default_timeout = CATPT_IPC_TIMEOUT_MS;
        init_completion(&mut (*ipc).done_completion);
        init_completion(&mut (*ipc).busy_completion);
        spin_lock_init(&mut (*ipc).lock);
        mutex_init(&mut (*ipc).mutex);
    }
}

unsafe extern "C" fn catpt_ipc_arm(ipc: *mut catpt_ipc, config: *mut catpt_fw_ready) -> c_int {
    unsafe {
        /*
         * Both tx and rx are put into and received from outbox. Inbox is
         * only used for notifications where payload size is known upfront,
         * thus no separate buffer is allocated for it.
         */
        (*ipc).rx.data = devm_kzalloc((*ipc).dev, (*config).outbox_size as usize, GFP_KERNEL) as *mut u8;
        if (*ipc).rx.data.is_null() {
            return -ENOMEM;
        }

        memcpy(
            &mut (*ipc).config as *mut _ as *mut c_void,
            config as *const c_void,
            core::mem::size_of::<catpt_fw_ready>(),
        );
        (*ipc).ready = true;

        0
    }
}

unsafe extern "C" fn catpt_ipc_msg_init(ipc: *mut catpt_ipc, reply: *mut catpt_ipc_msg) {
    unsafe {
        /* lockdep_assert_held(&ipc->lock); */

        (*ipc).rx.header = 0;
        (*ipc).rx.size = if !reply.is_null() { (*reply).size } else { 0 };
        reinit_completion(&mut (*ipc).done_completion);
        reinit_completion(&mut (*ipc).busy_completion);
    }
}

unsafe extern "C" fn catpt_dsp_send_tx(cdev: *mut catpt_dev, tx: *const catpt_ipc_msg) {
    unsafe {
        let header: u32 = (*tx).header | CATPT_IPCC_BUSY;

        trace_catpt_ipc_request(header);
        trace_catpt_ipc_payload((*tx).data, (*tx).size);

        memcpy_toio(catpt_outbox_addr(cdev), (*tx).data as *const c_void, (*tx).size);
        catpt_writel_shim(cdev, IPCC, header);
    }
}

unsafe extern "C" fn catpt_wait_msg_completion(cdev: *mut catpt_dev, timeout: c_int) -> c_int {
    unsafe {
        let ipc: *mut catpt_ipc = &mut (*cdev).ipc;
        let mut ret: c_ulong;

        ret = wait_for_completion_timeout(
            &mut (*ipc).done_completion,
            msecs_to_jiffies(timeout),
        );
        if ret == 0 {
            return -ETIMEDOUT;
        }
        if (*ipc).rx.rsp.status != CATPT_REPLY_PENDING {
            return 0;
        }

        /* wait for delayed reply */
        ret = wait_for_completion_timeout(
            &mut (*ipc).busy_completion,
            msecs_to_jiffies(timeout),
        );
        if ret != 0 { 0 } else { -ETIMEDOUT }
    }
}

unsafe extern "C" fn catpt_dsp_do_send_msg(
    cdev: *mut catpt_dev,
    request: catpt_ipc_msg,
    reply: *mut catpt_ipc_msg,
    timeout: c_int,
    name: *const c_char,
) -> c_int {
    unsafe {
        let ipc: *mut catpt_ipc = &mut (*cdev).ipc;
        let flags: c_ulong = 0;
        let mut ret: c_int;

        if !(*ipc).ready {
            return -EPERM;
        }
        if request.size > (*ipc).config.outbox_size as usize
            || (!reply.is_null() && (*reply).size > (*ipc).config.outbox_size as usize)
        {
            return -EINVAL;
        }

        spin_lock_irqsave(&mut (*ipc).lock, flags);
        catpt_ipc_msg_init(ipc, reply);
        catpt_dsp_send_tx(cdev, &request);
        spin_unlock_irqrestore(&mut (*ipc).lock, flags);

        ret = catpt_wait_msg_completion(cdev, timeout);
        if ret != 0 {
            dev_crit(
                (*cdev).dev,
                c"communication severed: %d, rebooting dsp..\n".as_ptr(),
                ret,
            );
            (*ipc).ready = false;
            /* TODO: attempt recovery */
            return ret;
        }

        ret = (*ipc).rx.rsp.status;
        if ret != 0 {
            dev_err(
                (*cdev).dev,
                c"%s (0x%08x) failed: %d\n".as_ptr(),
                name,
                request.header,
                ret,
            );
        }
        if !reply.is_null() {
            (*reply).header = (*ipc).rx.header;

            if ret == 0 && !(*reply).data.is_null() {
                memcpy((*reply).data as *mut c_void, (*ipc).rx.data as *const c_void, (*reply).size);
            }
        }

        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn catpt_dsp_send_msg_timeout(
    cdev: *mut catpt_dev,
    request: catpt_ipc_msg,
    reply: *mut catpt_ipc_msg,
    timeout: c_int,
    name: *const c_char,
) -> c_int {
    unsafe {
        mutex_lock(&mut (*cdev).ipc.mutex);
        let ret = catpt_dsp_do_send_msg(cdev, request, reply, timeout, name);
        mutex_unlock(&mut (*cdev).ipc.mutex);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn catpt_dsp_send_msg(
    cdev: *mut catpt_dev,
    request: catpt_ipc_msg,
    reply: *mut catpt_ipc_msg,
    name: *const c_char,
) -> c_int {
    unsafe {
        catpt_dsp_send_msg_timeout(cdev, request, reply, (*cdev).ipc.default_timeout, name)
    }
}

unsafe extern "C" fn catpt_dsp_notify_stream(cdev: *mut catpt_dev, msg: catpt_notify_msg) {
    unsafe {
        let stream: *mut catpt_stream_runtime;
        let mut pos: catpt_notify_position = core::mem::zeroed();
        let mut glitch: catpt_notify_glitch = core::mem::zeroed();

        mutex_lock(&mut (*cdev).stream_mutex);

        stream = catpt_stream_find(cdev, msg.stream_hw_id);
        if stream.is_null() {
            dev_warn(
                (*cdev).dev,
                c"notify %d for non-existent stream %d\n".as_ptr(),
                msg.notify_reason,
                msg.stream_hw_id,
            );
            mutex_unlock(&mut (*cdev).stream_mutex);
            return;
        }

        if msg.notify_reason == CATPT_NOTIFY_POSITION_CHANGED {
            memcpy_fromio(
                &mut pos as *mut _ as *mut c_void,
                catpt_inbox_addr(cdev),
                core::mem::size_of::<catpt_notify_position>(),
            );
            trace_catpt_ipc_payload(
                &pos as *const _ as *const u8,
                core::mem::size_of::<catpt_notify_position>(),
            );

            catpt_stream_update_position(cdev, stream, &mut pos);
        } else if msg.notify_reason == CATPT_NOTIFY_GLITCH_OCCURRED {
            memcpy_fromio(
                &mut glitch as *mut _ as *mut c_void,
                catpt_inbox_addr(cdev),
                core::mem::size_of::<catpt_notify_glitch>(),
            );
            trace_catpt_ipc_payload(
                &glitch as *const _ as *const u8,
                core::mem::size_of::<catpt_notify_glitch>(),
            );

            dev_warn(
                (*cdev).dev,
                c"glitch %d at pos: 0x%08llx, wp: 0x%08x\n".as_ptr(),
                glitch.type_,
                glitch.presentation_pos,
                glitch.write_pos,
            );
        } else {
            dev_warn(
                (*cdev).dev,
                c"unknown notification: %d received\n".as_ptr(),
                msg.notify_reason,
            );
        }

        mutex_unlock(&mut (*cdev).stream_mutex);
    }
}

unsafe extern "C" fn catpt_dsp_copy_rx(cdev: *mut catpt_dev, header: u32) {
    unsafe {
        let ipc: *mut catpt_ipc = &mut (*cdev).ipc;

        (*ipc).rx.header = header;
        if (*ipc).rx.rsp.status != CATPT_REPLY_SUCCESS {
            return;
        }

        memcpy_fromio((*ipc).rx.data as *mut c_void, catpt_outbox_addr(cdev), (*ipc).rx.size);
        trace_catpt_ipc_payload((*ipc).rx.data, (*ipc).rx.size);
    }
}

unsafe extern "C" fn catpt_dsp_process_response(cdev: *mut catpt_dev, header: u32) {
    unsafe {
        let msg: catpt_notify_msg = CATPT_MSG(header);
        let ipc: *mut catpt_ipc = &mut (*cdev).ipc;

        if msg.fw_ready {
            let mut config: catpt_fw_ready = core::mem::zeroed();
            /* to fit 32b header original address is shifted right by 3 */
            let off: u32 = msg.mailbox_address << 3;

            memcpy_fromio(
                &mut config as *mut _ as *mut c_void,
                (*cdev).lpe_ba.add(off as usize) as *const c_void,
                core::mem::size_of::<catpt_fw_ready>(),
            );
            trace_catpt_ipc_payload(
                &config as *const _ as *const u8,
                core::mem::size_of::<catpt_fw_ready>(),
            );

            dev_dbg((*cdev).dev, c"FW READY 0x%08x\n".as_ptr(), header);
            catpt_ipc_arm(ipc, &mut config);
            complete(&mut (*cdev).fw_ready);
            return;
        }

        if msg.global_msg_type == CATPT_GLB_REQUEST_CORE_DUMP {
            dev_err((*cdev).dev, c"ADSP device coredump received\n".as_ptr());
            (*ipc).ready = false;
            catpt_coredump(cdev);

            if catpt_readl_dram(cdev, COREDUMP) == CATPT_COREDUMP_REQUEST {
                dev_dbg(
                    (*cdev).dev,
                    c"releasing firmware from the coredump state\n".as_ptr(),
                );
                catpt_writel_dram(cdev, COREDUMP, CATPT_COREDUMP_RELEASE);
            }

            complete(&mut (*cdev).fw_ready);
            /* TODO: attempt recovery */
        } else if msg.global_msg_type == CATPT_GLB_STREAM_MESSAGE {
            if msg.stream_msg_type == CATPT_STRM_NOTIFICATION {
                catpt_dsp_notify_stream(cdev, msg);
            } else {
                catpt_dsp_copy_rx(cdev, header);
                /* signal completion of delayed reply */
                complete(&mut (*ipc).busy_completion);
            }
        } else {
            dev_warn(
                (*cdev).dev,
                c"unknown response: %d received\n".as_ptr(),
                msg.global_msg_type,
            );
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn catpt_dsp_irq_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    unsafe {
        let cdev: *mut catpt_dev = dev_id as *mut catpt_dev;
        let ipcd: u32;

        ipcd = catpt_readl_shim(cdev, IPCD);
        trace_catpt_ipc_notify(ipcd);

        /* ensure there is delayed reply or notification to process */
        if (ipcd & CATPT_IPCD_BUSY) == 0 {
            return IRQ_NONE;
        }

        catpt_dsp_process_response(cdev, ipcd);

        /* tell DSP processing is completed */
        catpt_updatel_shim(
            cdev,
            IPCD,
            CATPT_IPCD_BUSY | CATPT_IPCD_DONE,
            CATPT_IPCD_DONE,
        );
        /* unmask dsp BUSY interrupt */
        catpt_updatel_shim(cdev, IMC, CATPT_IMC_IPCDB, 0);

        IRQ_HANDLED
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn catpt_dsp_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    unsafe {
        let cdev: *mut catpt_dev = dev_id as *mut catpt_dev;
        let mut ret: irqreturn_t = IRQ_NONE;
        let isc: u32;
        let ipcc: u32;

        isc = catpt_readl_shim(cdev, ISC);
        trace_catpt_irq(isc);

        /* immediate reply */
        if (isc & CATPT_ISC_IPCCD) != 0 {
            /* mask host DONE interrupt */
            catpt_updatel_shim(cdev, IMC, CATPT_IMC_IPCCD, CATPT_IMC_IPCCD);

            ipcc = catpt_readl_shim(cdev, IPCC);
            trace_catpt_ipc_reply(ipcc);
            catpt_dsp_copy_rx(cdev, ipcc);
            complete(&mut (*cdev).ipc.done_completion);

            /* tell DSP processing is completed */
            catpt_updatel_shim(cdev, IPCC, CATPT_IPCC_DONE, 0);
            /* unmask host DONE interrupt */
            catpt_updatel_shim(cdev, IMC, CATPT_IMC_IPCCD, 0);
            ret = IRQ_HANDLED;
        }

        /* delayed reply or notification */
        if (isc & CATPT_ISC_IPCDB) != 0 {
            /* mask dsp BUSY interrupt */
            catpt_updatel_shim(cdev, IMC, CATPT_IMC_IPCDB, CATPT_IMC_IPCDB);
            ret = IRQ_WAKE_THREAD;
        }

        ret
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
