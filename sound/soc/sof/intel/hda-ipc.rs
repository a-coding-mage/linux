// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//          Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//          Rander Wang <rander.wang@intel.com>
//          Keyon Jie <yang.jie@linux.intel.com>
//

/*
 * Hardware interface for generic Intel audio DSP HDA IP
 */

// Dependencies from:
// <sound/hda_register.h>
// <sound/sof/ipc4/header.h>
// <trace/events/sof_intel.h>
// "../ops.h"
// "hda.h"
// "telemetry.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type size_t = usize;
type bool_t = bool;
type irqreturn_t = c_int;

const IRQ_HANDLED: irqreturn_t = 1;
const ESTRPIPE: c_int = 86;
const EINVAL: c_int = 22;

extern "C" {
    static system_dfl_wq: *mut c_void;
    static KERN_DEBUG: *mut c_char;
    static KERN_ERR: *mut c_char;

    fn snd_sof_dsp_update_bits_forced(
        sdev: *mut snd_sof_dev,
        bar: u32,
        offset: u32,
        mask: u32,
        value: u32,
    );
    fn snd_sof_dsp_update_bits(
        sdev: *mut snd_sof_dev,
        bar: u32,
        offset: u32,
        mask: u32,
        value: u32,
    );
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: u32, offset: u32, value: u32);
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u32;
    fn snd_sof_dsp_read8(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u8;
    fn sof_mailbox_write(sdev: *mut snd_sof_dev, offset: size_t, data: *mut c_void, size: size_t);
    fn sof_mailbox_read(sdev: *mut snd_sof_dev, offset: size_t, data: *mut c_void, size: size_t);
    fn snd_sof_ipc_get_reply(sdev: *mut snd_sof_dev);
    fn snd_sof_ipc_reply(sdev: *mut snd_sof_dev, msg: u32);
    fn snd_sof_ipc_msgs_rx(sdev: *mut snd_sof_dev);
    fn snd_sof_dsp_panic(sdev: *mut snd_sof_dev, offset: u32, non_recoverable: bool_t);
    fn mod_delayed_work(wq: *mut c_void, work: *mut c_void, delay: u64) -> bool_t;
    fn msecs_to_jiffies(msecs: u32) -> u64;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn wake_up(waitq: *mut c_void);
    fn hda_dsp_get_state(sdev: *mut snd_sof_dev, level: *mut c_char);
    fn sof_ipc4_intel_dump_telemetry_state(sdev: *mut snd_sof_dev, flags: u32);
    fn hda_dsp_dump_ext_rom_status(sdev: *mut snd_sof_dev, level: *mut c_char, flags: u32);
    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_intel_dsp_desc;

    fn trace_sof_intel_ipc_firmware_response(sdev: *mut snd_sof_dev, msg: u32, msg_ext: u32);
    fn trace_sof_intel_ipc_firmware_initiated(sdev: *mut snd_sof_dev, msg: u32, msg_ext: u32);
    fn trace_sof_intel_hda_irq_ipc_check(sdev: *mut snd_sof_dev, irq_status: u32);

    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg_ratelimited(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct snd_sof_dev {
    pub host_box: snd_sof_mailbox,
    pub dsp_box: snd_sof_mailbox,
    pub stream_box: snd_sof_mailbox,
    pub msg: *mut snd_sof_ipc_msg,
    pub ipc: *mut snd_sof_ipc,
    pub pdata: *mut snd_sof_pdata,
    pub dev: *mut c_void,
    pub fw_state: u32,
    pub ipc_lock: c_void,
    pub dspless_mode_selected: bool_t,
}

#[repr(C)]
pub struct snd_sof_mailbox {
    pub offset: size_t,
    pub size: size_t,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
    pub msg_data: *mut c_void,
    pub msg_size: size_t,
    pub reply_data: *mut c_void,
    pub reply_error: c_int,
}

#[repr(C)]
pub struct snd_sof_ipc {
    pub msg: snd_sof_ipc_msg_data,
}

#[repr(C)]
pub struct snd_sof_ipc_msg_data {
    pub reply_data: *mut c_void,
    pub rx_data: *mut c_void,
}

#[repr(C)]
pub struct sof_intel_hda_dev {
    pub d0i3_work: c_void,
    pub delayed_ipc_tx_msg: *mut snd_sof_ipc_msg,
    pub boot_iteration: u32,
    pub code_loading: c_int,
    pub waitq: c_void,
    pub desc: *const sof_intel_dsp_desc,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub check_ipc_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> bool_t>,
    pub ipc_req: u32,
    pub ipc_req_mask: u32,
}

#[repr(C)]
pub struct sof_ipc4_msg {
    pub primary: u32,
    pub extension: u32,
    pub data_ptr: *mut c_void,
    pub data_size: size_t,
}

#[repr(C)]
pub struct sof_ipc_reply {
    pub hdr: sof_ipc_cmd_hdr,
    pub error: c_int,
}

#[repr(C)]
pub struct sof_ipc_cmd_hdr {
    pub size: u32,
    pub cmd: u32,
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct hdac_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_intel_hda_stream {
    pub hext_stream: sof_intel_hda_hext_stream,
    pub sof_intel_stream: sof_intel_stream,
}

#[repr(C)]
pub struct sof_intel_hda_hext_stream {
    pub hstream: hdac_stream,
}

#[repr(C)]
pub struct sof_intel_stream {
    pub posn_offset: size_t,
}

#[repr(C)]
pub struct sof_ipc_stream_posn {
    _private: [u8; 0],
}

const HDA_DSP_BAR: u32 = 0;
const HDA_DSP_HDA_BAR: u32 = 0;
const HDA_DSP_PP_BAR: u32 = 0;
const HDA_DSP_REG_HIPCT: u32 = 0;
const HDA_DSP_REG_HIPCCTL: u32 = 0;
const HDA_DSP_REG_HIPCIE: u32 = 0;
const HDA_DSP_REG_HIPCI: u32 = 0;
const HDA_DSP_REG_HIPCTE: u32 = 0;
const HDA_DSP_REG_ADSPIS: u32 = 0;
const SOF_HDA_INTSTS: u32 = 0;
const SOF_HDA_INTCTL: u32 = 0;
const SOF_HDA_REG_PP_PPSTS: u32 = 0;
const AZX_REG_RIRBSTS: u32 = 0;
const HDA_DSP_REG_HIPCT_BUSY: u32 = 0;
const HDA_DSP_REG_HIPCCTL_BUSY: u32 = 0;
const HDA_DSP_REG_HIPCIE_DONE: u32 = 0;
const HDA_DSP_REG_HIPCCTL_DONE: u32 = 0;
const HDA_DSP_REG_HIPCI_BUSY: u32 = 0;
const HDA_DSP_REG_HIPCT_MSG_MASK: u32 = 0;
const HDA_DSP_REG_HIPCTE_MSG_MASK: u32 = 0;
const HDA_DSP_REG_HIPCI_MSG_MASK: u32 = 0;
const HDA_DSP_REG_HIPCIE_MSG_MASK: u32 = 0;
const HDA_DSP_ADSPIS_IPC: u32 = 0;
const HDA_DSP_ADSPIS_CL_DMA: u32 = 0;
const HDA_DSP_MBOX_UPLINK_OFFSET: c_int = 0;
const SOF_HDA_D0I3_WORK_DELAY_MS: u32 = 0;
const SOF_IPC4_MODULE_MSG: u32 = 0;
const SOF_IPC4_MOD_SET_DX: u32 = 0;
const SOF_IPC4_MOD_SET_D0IX: u32 = 0;
const SOF_IPC4_MSG_DIR_MASK: u32 = 0;
const SOF_IPC_GLB_PM_MSG: u32 = 0;
const SOF_IPC_PM_CTX_SAVE: u32 = 0;
const SOF_IPC_PM_GATE: u32 = 0;
const SOF_IPC_GLB_REPLY: u32 = 0;
const SOF_FW_BOOT_COMPLETE: u32 = 0;
const SOF_FW_BOOT_IN_PROGRESS: u32 = 0;
const SOF_IPC_PANIC_MAGIC_MASK: u32 = 0;
const SOF_IPC_PANIC_MAGIC: u32 = 0;
const HDA_FW_BOOT_ATTEMPTS: u32 = 0;
const SOF_DBG_DUMP_OPTIONAL: u32 = 0;
const SOF_DBG_DUMP_REGS: u32 = 0;

extern "C" {
    fn SOF_IPC4_MSG_IS_MODULE_MSG(primary: u32) -> u32;
    fn SOF_IPC4_MSG_TYPE_GET(primary: u32) -> u32;
    fn HDA_DSP_PANIC_OFFSET(msg_ext: u32) -> u32;
    fn SRAM_WINDOW_OFFSET(id: u32) -> c_int;
}

unsafe fn container_of_hstream(hstream: *mut hdac_stream) -> *mut sof_intel_hda_stream {
    hstream.cast::<sof_intel_hda_stream>()
}

unsafe fn hda_dsp_ipc_host_done(sdev: *mut snd_sof_dev) {
    /*
     * tell DSP cmd is done - clear busy
     * interrupt and send reply msg to dsp
     */
    snd_sof_dsp_update_bits_forced(
        sdev,
        HDA_DSP_BAR,
        HDA_DSP_REG_HIPCT,
        HDA_DSP_REG_HIPCT_BUSY,
        HDA_DSP_REG_HIPCT_BUSY,
    );

    /* unmask BUSY interrupt */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_BAR,
        HDA_DSP_REG_HIPCCTL,
        HDA_DSP_REG_HIPCCTL_BUSY,
        HDA_DSP_REG_HIPCCTL_BUSY,
    );
}

unsafe fn hda_dsp_ipc_dsp_done(sdev: *mut snd_sof_dev) {
    /*
     * set DONE bit - tell DSP we have received the reply msg
     * from DSP, and processed it, don't send more reply to host
     */
    snd_sof_dsp_update_bits_forced(
        sdev,
        HDA_DSP_BAR,
        HDA_DSP_REG_HIPCIE,
        HDA_DSP_REG_HIPCIE_DONE,
        HDA_DSP_REG_HIPCIE_DONE,
    );

    /* unmask Done interrupt */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_BAR,
        HDA_DSP_REG_HIPCCTL,
        HDA_DSP_REG_HIPCCTL_DONE,
        HDA_DSP_REG_HIPCCTL_DONE,
    );
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ipc_send_msg(
    sdev: *mut snd_sof_dev,
    msg: *mut snd_sof_ipc_msg,
) -> c_int {
    /* send IPC message to DSP */
    sof_mailbox_write(
        sdev,
        (*sdev).host_box.offset,
        (*msg).msg_data,
        (*msg).msg_size,
    );
    snd_sof_dsp_write(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCI, HDA_DSP_REG_HIPCI_BUSY);

    0
}

unsafe fn hda_dsp_ipc4_pm_msg(primary: u32) -> bool_t {
    /* pm setting is only supported by module msg */
    if SOF_IPC4_MSG_IS_MODULE_MSG(primary) != SOF_IPC4_MODULE_MSG {
        return false;
    }

    if SOF_IPC4_MSG_TYPE_GET(primary) == SOF_IPC4_MOD_SET_DX
        || SOF_IPC4_MSG_TYPE_GET(primary) == SOF_IPC4_MOD_SET_D0IX
    {
        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ipc4_schedule_d0i3_work(
    hdev: *mut sof_intel_hda_dev,
    msg: *mut snd_sof_ipc_msg,
) {
    let msg_data = (*msg).msg_data as *mut sof_ipc4_msg;

    /* Schedule a delayed work for d0i3 entry after sending non-pm ipc msg */
    if hda_dsp_ipc4_pm_msg((*msg_data).primary) {
        return;
    }

    mod_delayed_work(
        system_dfl_wq,
        ptr::addr_of_mut!((*hdev).d0i3_work),
        msecs_to_jiffies(SOF_HDA_D0I3_WORK_DELAY_MS),
    );
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ipc4_send_msg(
    sdev: *mut snd_sof_dev,
    msg: *mut snd_sof_ipc_msg,
) -> c_int {
    let hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let msg_data = (*msg).msg_data as *mut sof_ipc4_msg;

    if hda_ipc4_tx_is_busy(sdev) {
        (*hdev).delayed_ipc_tx_msg = msg;
        return 0;
    }

    (*hdev).delayed_ipc_tx_msg = ptr::null_mut();

    /* send the message via mailbox */
    if (*msg_data).data_size != 0 {
        sof_mailbox_write(
            sdev,
            (*sdev).host_box.offset,
            (*msg_data).data_ptr,
            (*msg_data).data_size,
        );
    }

    snd_sof_dsp_write(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCIE, (*msg_data).extension);
    snd_sof_dsp_write(
        sdev,
        HDA_DSP_BAR,
        HDA_DSP_REG_HIPCI,
        (*msg_data).primary | HDA_DSP_REG_HIPCI_BUSY,
    );

    hda_dsp_ipc4_schedule_d0i3_work(hdev, msg);

    0
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ipc_get_reply(sdev: *mut snd_sof_dev) {
    let msg = (*sdev).msg;
    let mut reply: sof_ipc_reply = core::mem::zeroed();
    let hdr: *mut sof_ipc_cmd_hdr;

    /*
     * Sometimes, there is unexpected reply ipc arriving. The reply
     * ipc belongs to none of the ipcs sent from driver.
     * In this case, the driver must ignore the ipc.
     */
    if msg.is_null() {
        dev_warn((*sdev).dev, c"unexpected ipc interrupt raised!\n".as_ptr());
        return;
    }

    hdr = (*msg).msg_data as *mut sof_ipc_cmd_hdr;
    if (*hdr).cmd == (SOF_IPC_GLB_PM_MSG | SOF_IPC_PM_CTX_SAVE)
        || (*hdr).cmd == (SOF_IPC_GLB_PM_MSG | SOF_IPC_PM_GATE)
    {
        /*
         * memory windows are powered off before sending IPC reply,
         * so we can't read the mailbox for CTX_SAVE and PM_GATE
         * replies.
         */
        reply.error = 0;
        reply.hdr.cmd = SOF_IPC_GLB_REPLY;
        reply.hdr.size = size_of::<sof_ipc_reply>() as u32;
        memcpy(
            (*msg).reply_data,
            ptr::addr_of!(reply).cast::<c_void>(),
            size_of::<sof_ipc_reply>(),
        );

        (*msg).reply_error = 0;
    } else {
        snd_sof_ipc_get_reply(sdev);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ipc4_irq_thread(
    _irq: c_int,
    context: *mut c_void,
) -> irqreturn_t {
    let mut notification_data: sof_ipc4_msg = core::mem::zeroed();
    let sdev = context as *mut snd_sof_dev;
    let mut ack_received = false;
    let mut ipc_irq = false;
    let hipcie: u32;
    let hipct: u32;

    hipcie = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCIE);
    hipct = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCT);

    if (hipcie & HDA_DSP_REG_HIPCIE_DONE) != 0 {
        /* DSP received the message */
        snd_sof_dsp_update_bits(
            sdev,
            HDA_DSP_BAR,
            HDA_DSP_REG_HIPCCTL,
            HDA_DSP_REG_HIPCCTL_DONE,
            0,
        );
        hda_dsp_ipc_dsp_done(sdev);

        ipc_irq = true;
        ack_received = true;
    }

    if (hipct & HDA_DSP_REG_HIPCT_BUSY) != 0 {
        /* Message from DSP (reply or notification) */
        let hipcte = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCTE);
        let primary = hipct & HDA_DSP_REG_HIPCT_MSG_MASK;
        let extension = hipcte & HDA_DSP_REG_HIPCTE_MSG_MASK;

        /* mask BUSY interrupt */
        snd_sof_dsp_update_bits(
            sdev,
            HDA_DSP_BAR,
            HDA_DSP_REG_HIPCCTL,
            HDA_DSP_REG_HIPCCTL_BUSY,
            0,
        );

        if (primary & SOF_IPC4_MSG_DIR_MASK) != 0 {
            /* Reply received */
            if (*sdev).fw_state == SOF_FW_BOOT_COMPLETE {
                let data = (*(*sdev).ipc).msg.reply_data as *mut sof_ipc4_msg;

                (*data).primary = primary;
                (*data).extension = extension;

                /* guard(spinlock_irq)(&sdev->ipc_lock); */
                snd_sof_ipc_get_reply(sdev);
                hda_dsp_ipc_host_done(sdev);
                snd_sof_ipc_reply(sdev, (*data).primary);
            } else {
                dev_dbg_ratelimited(
                    (*sdev).dev,
                    c"IPC reply before FW_READY: %#x|%#x\n".as_ptr(),
                    primary,
                    extension,
                );
            }
        } else {
            /* Notification received */

            notification_data.primary = primary;
            notification_data.extension = extension;
            (*(*sdev).ipc).msg.rx_data = ptr::addr_of_mut!(notification_data).cast::<c_void>();
            snd_sof_ipc_msgs_rx(sdev);
            (*(*sdev).ipc).msg.rx_data = ptr::null_mut();

            /* Let DSP know that we have finished processing the message */
            hda_dsp_ipc_host_done(sdev);
        }

        ipc_irq = true;
    }

    if !ipc_irq {
        /* This interrupt is not shared so no need to return IRQ_NONE. */
        dev_dbg_ratelimited((*sdev).dev, c"nothing to do in IPC IRQ thread\n".as_ptr());
    }

    if ack_received {
        let hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

        if !(*hdev).delayed_ipc_tx_msg.is_null() {
            hda_dsp_ipc4_send_msg(sdev, (*hdev).delayed_ipc_tx_msg);
        }
    }

    IRQ_HANDLED
}

/* IPC handler thread */
#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ipc_irq_thread(
    _irq: c_int,
    context: *mut c_void,
) -> irqreturn_t {
    let sdev = context as *mut snd_sof_dev;
    let hipci: u32;
    let hipcie: u32;
    let hipct: u32;
    let hipcte: u32;
    let mut msg: u32;
    let mut msg_ext: u32;
    let mut ipc_irq = false;

    /* read IPC status */
    hipcie = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCIE);
    hipct = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCT);
    hipci = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCI);
    hipcte = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCTE);

    /* is this a reply message from the DSP */
    if (hipcie & HDA_DSP_REG_HIPCIE_DONE) != 0 {
        msg = hipci & HDA_DSP_REG_HIPCI_MSG_MASK;
        msg_ext = hipcie & HDA_DSP_REG_HIPCIE_MSG_MASK;

        trace_sof_intel_ipc_firmware_response(sdev, msg, msg_ext);

        /* mask Done interrupt */
        snd_sof_dsp_update_bits(
            sdev,
            HDA_DSP_BAR,
            HDA_DSP_REG_HIPCCTL,
            HDA_DSP_REG_HIPCCTL_DONE,
            0,
        );

        /*
         * Make sure the interrupt thread cannot be preempted between
         * waking up the sender and re-enabling the interrupt. Also
         * protect against a theoretical race with sof_ipc_tx_message():
         * if the DSP is fast enough to receive an IPC message, reply to
         * it, and the host interrupt processing calls this function on
         * a different core from the one, where the sending is taking
         * place, the message might not yet be marked as expecting a
         * reply.
         */
        if (*sdev).fw_state == SOF_FW_BOOT_COMPLETE {
            /* handle immediate reply from DSP core */
            /* guard(spinlock_irq)(&sdev->ipc_lock); */
            hda_dsp_ipc_get_reply(sdev);
            snd_sof_ipc_reply(sdev, msg);
            /* set the done bit */
            hda_dsp_ipc_dsp_done(sdev);
        } else {
            dev_dbg_ratelimited(
                (*sdev).dev,
                c"IPC reply before FW_READY: %#x\n".as_ptr(),
                msg,
            );
        }

        ipc_irq = true;
    }

    /* is this a new message from DSP */
    if (hipct & HDA_DSP_REG_HIPCT_BUSY) != 0 {
        msg = hipct & HDA_DSP_REG_HIPCT_MSG_MASK;
        msg_ext = hipcte & HDA_DSP_REG_HIPCTE_MSG_MASK;

        trace_sof_intel_ipc_firmware_initiated(sdev, msg, msg_ext);

        /* mask BUSY interrupt */
        snd_sof_dsp_update_bits(
            sdev,
            HDA_DSP_BAR,
            HDA_DSP_REG_HIPCCTL,
            HDA_DSP_REG_HIPCCTL_BUSY,
            0,
        );

        /* handle messages from DSP */
        if (hipct & SOF_IPC_PANIC_MAGIC_MASK) == SOF_IPC_PANIC_MAGIC {
            let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
            let mut non_recoverable = true;

            /*
             * This is a PANIC message!
             *
             * If it is arriving during firmware boot and it is not
             * the last boot attempt then change the non_recoverable
             * to false as the DSP might be able to boot in the next
             * iteration(s)
             */
            if (*sdev).fw_state == SOF_FW_BOOT_IN_PROGRESS
                && (*hda).boot_iteration < HDA_FW_BOOT_ATTEMPTS
            {
                non_recoverable = false;
            }

            snd_sof_dsp_panic(sdev, HDA_DSP_PANIC_OFFSET(msg_ext), non_recoverable);
        } else {
            /* normal message - process normally */
            snd_sof_ipc_msgs_rx(sdev);
        }

        hda_dsp_ipc_host_done(sdev);

        ipc_irq = true;
    }

    if !ipc_irq {
        /*
         * This interrupt is not shared so no need to return IRQ_NONE.
         */
        dev_dbg_ratelimited((*sdev).dev, c"nothing to do in IPC IRQ thread\n".as_ptr());
    }

    IRQ_HANDLED
}

/* Check if an IPC IRQ occurred */
#[no_mangle]
pub unsafe extern "C" fn hda_dsp_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool_t {
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let mut ret = false;
    let irq_status: u32;

    if (*sdev).dspless_mode_selected {
        return false;
    }

    /* store status */
    irq_status = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPIS);
    trace_sof_intel_hda_irq_ipc_check(sdev, irq_status);

    /* invalid message ? */
    if irq_status == 0xffffffff {
        return ret;
    }

    /* IPC message ? */
    if (irq_status & HDA_DSP_ADSPIS_IPC) != 0 {
        ret = true;
    }

    /* CLDMA message ? */
    if (irq_status & HDA_DSP_ADSPIS_CL_DMA) != 0 {
        (*hda).code_loading = 0;
        wake_up(ptr::addr_of_mut!((*hda).waitq));
        ret = false;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ipc_get_mailbox_offset(_sdev: *mut snd_sof_dev) -> c_int {
    HDA_DSP_MBOX_UPLINK_OFFSET
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ipc_get_window_offset(
    _sdev: *mut snd_sof_dev,
    id: u32,
) -> c_int {
    SRAM_WINDOW_OFFSET(id)
}

#[no_mangle]
pub unsafe extern "C" fn hda_ipc_msg_data(
    sdev: *mut snd_sof_dev,
    sps: *mut snd_sof_pcm_stream,
    p: *mut c_void,
    sz: size_t,
) -> c_int {
    if sps.is_null() || (*sdev).stream_box.size == 0 {
        sof_mailbox_read(sdev, (*sdev).dsp_box.offset, p, sz);
    } else {
        let substream = (*sps).substream;
        let hstream = (*(*substream).runtime).private_data as *mut hdac_stream;
        let hda_stream: *mut sof_intel_hda_stream;

        hda_stream = container_of_hstream(hstream);

        /* The stream might already be closed */
        if hstream.is_null() {
            return -ESTRPIPE;
        }

        sof_mailbox_read(sdev, (*hda_stream).sof_intel_stream.posn_offset, p, sz);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn hda_set_stream_data_offset(
    sdev: *mut snd_sof_dev,
    sps: *mut snd_sof_pcm_stream,
    posn_offset: size_t,
) -> c_int {
    let substream = (*sps).substream;
    let hstream = (*(*substream).runtime).private_data as *mut hdac_stream;
    let hda_stream: *mut sof_intel_hda_stream;

    hda_stream = container_of_hstream(hstream);

    /* check for unaligned offset or overflow */
    if posn_offset > (*sdev).stream_box.size
        || posn_offset % size_of::<sof_ipc_stream_posn>() != 0
    {
        return -EINVAL;
    }

    (*hda_stream).sof_intel_stream.posn_offset =
        (*sdev).stream_box.offset.wrapping_add(posn_offset);

    dev_dbg(
        (*sdev).dev,
        c"pcm: stream dir %d, posn mailbox offset is %zu".as_ptr(),
        (*substream).stream,
        (*hda_stream).sof_intel_stream.posn_offset,
    );

    0
}

#[no_mangle]
pub unsafe extern "C" fn hda_ipc4_dsp_dump(sdev: *mut snd_sof_dev, flags: u32) {
    let level = if (flags & SOF_DBG_DUMP_OPTIONAL) != 0 {
        KERN_DEBUG
    } else {
        KERN_ERR
    };

    /* print ROM/FW status */
    hda_dsp_get_state(sdev, level);

    if (flags & SOF_DBG_DUMP_REGS) != 0 {
        sof_ipc4_intel_dump_telemetry_state(sdev, flags);
    } else {
        hda_dsp_dump_ext_rom_status(sdev, level, flags);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hda_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool_t {
    let chip: *const sof_intel_dsp_desc;

    chip = get_chip_info((*sdev).pdata);
    if !chip.is_null() {
        if let Some(check_ipc_irq) = (*chip).check_ipc_irq {
            return check_ipc_irq(sdev);
        }
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn hda_ipc_irq_dump(sdev: *mut snd_sof_dev) {
    let adspis: u32;
    let intsts: u32;
    let intctl: u32;
    let ppsts: u32;
    let rirbsts: u8;

    /* read key IRQ stats and config registers */
    adspis = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPIS);
    intsts = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, SOF_HDA_INTSTS);
    intctl = snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, SOF_HDA_INTCTL);
    ppsts = snd_sof_dsp_read(sdev, HDA_DSP_PP_BAR, SOF_HDA_REG_PP_PPSTS);
    rirbsts = snd_sof_dsp_read8(sdev, HDA_DSP_HDA_BAR, AZX_REG_RIRBSTS);

    dev_err(
        (*sdev).dev,
        c"hda irq intsts 0x%8.8x intlctl 0x%8.8x rirb %2.2x\n".as_ptr(),
        intsts,
        intctl,
        rirbsts as c_int,
    );
    dev_err(
        (*sdev).dev,
        c"dsp irq ppsts 0x%8.8x adspis 0x%8.8x\n".as_ptr(),
        ppsts,
        adspis,
    );
}

#[no_mangle]
pub unsafe extern "C" fn hda_ipc_dump(sdev: *mut snd_sof_dev) {
    let hipcie: u32;
    let hipct: u32;
    let hipcctl: u32;

    hda_ipc_irq_dump(sdev);

    /* read IPC status */
    hipcie = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCIE);
    hipct = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCT);
    hipcctl = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCCTL);

    /* dump the IPC regs */
    /* TODO: parse the raw msg */
    dev_err(
        (*sdev).dev,
        c"host status 0x%8.8x dsp status 0x%8.8x mask 0x%8.8x\n".as_ptr(),
        hipcie,
        hipct,
        hipcctl,
    );
}

#[no_mangle]
pub unsafe extern "C" fn hda_ipc4_dump(sdev: *mut snd_sof_dev) {
    let hipci: u32;
    let hipcie: u32;
    let hipct: u32;
    let hipcte: u32;
    let hipcctl: u32;

    hda_ipc_irq_dump(sdev);

    hipci = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCI);
    hipcie = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCIE);
    hipct = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCT);
    hipcte = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCTE);
    hipcctl = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_HIPCCTL);

    /* dump the IPC regs */
    /* TODO: parse the raw msg */
    dev_err(
        (*sdev).dev,
        c"Host IPC initiator: %#x|%#x, target: %#x|%#x, ctl: %#x\n".as_ptr(),
        hipci,
        hipcie,
        hipct,
        hipcte,
        hipcctl,
    );
}

#[no_mangle]
pub unsafe extern "C" fn hda_ipc4_tx_is_busy(sdev: *mut snd_sof_dev) -> bool_t {
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip = (*hda).desc;
    let val: u32;

    val = snd_sof_dsp_read(sdev, HDA_DSP_BAR, (*chip).ipc_req);

    (val & (*chip).ipc_req_mask) != 0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
