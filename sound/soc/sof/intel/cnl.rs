// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//	    Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//	    Rander Wang <rander.wang@intel.com>
//          Keyon Jie <yang.jie@linux.intel.com>
//

/*
 * Hardware interface for audio DSP on Cannonlake.
 *
 * C include dependencies translated as external Rust dependencies:
 * <sound/sof/ext_manifest4.h>, <sound/sof/ipc4/header.h>,
 * <trace/events/sof_intel.h>, "../ipc4-priv.h", "../ops.h", "hda.h",
 * "hda-ipc.h", "../sof-audio.h".
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type bool_ = bool;
type irqreturn_t = c_int;

#[repr(C)]
pub struct snd_sof_debugfs_map {
    pub name: *const c_char,
    pub bar: u32,
    pub offset: u32,
    pub size: u32,
    pub access: u32,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut c_void,
    pub pdata: *mut snd_sof_pdata,
    pub ipc: *mut snd_sof_ipc,
    pub ipc_lock: c_void,
    pub fw_state: u32,
    pub host_box: sof_host_box,
    pub private: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut c_void,
    pub ipc_type: u32,
}

#[repr(C)]
pub struct snd_sof_ipc {
    pub msg: snd_sof_ipc_message_data,
}

#[repr(C)]
pub struct snd_sof_ipc_message_data {
    pub reply_data: *mut c_void,
    pub rx_data: *mut sof_ipc4_msg,
}

#[repr(C)]
pub struct sof_host_box {
    pub offset: u32,
}

#[repr(C)]
pub struct sof_ipc4_msg {
    pub primary: u32,
    pub extension: u32,
    pub data_size: u32,
    pub data_ptr: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
    pub msg_data: *mut c_void,
    pub msg_size: u32,
}

#[repr(C)]
pub struct sof_ipc_cmd_hdr {
    pub cmd: u32,
}

#[repr(C)]
pub struct sof_ipc_pm_gate {
    pub hdr: sof_ipc_cmd_hdr,
    pub flags: u32,
}

#[repr(C)]
pub struct sof_intel_hda_dev {
    pub delayed_ipc_tx_msg: *mut snd_sof_ipc_msg,
    pub boot_iteration: u32,
    pub d0i3_work: c_void,
}

#[repr(C)]
pub struct sof_ipc4_fw_data {
    pub manifest_fw_hdr_offset: u32,
    pub mtrace_type: u32,
    pub load_library: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub shutdown: Option<unsafe extern "C" fn()>,
    pub irq_thread: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
    pub send_msg: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_ipc_msg) -> c_int>,
    pub ipc_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub set_power_state: Option<unsafe extern "C" fn()>,
    pub debug_map: *const snd_sof_debugfs_map,
    pub debug_map_count: usize,
    pub post_fw_run: Option<unsafe extern "C" fn()>,
    pub run: Option<unsafe extern "C" fn()>,
    pub core_get: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub cores_num: u32,
    pub init_core_mask: u32,
    pub host_managed_cores_mask: u32,
    pub ipc_req: u32,
    pub ipc_req_mask: u32,
    pub ipc_ack: u32,
    pub ipc_ack_mask: u32,
    pub ipc_ctl: u32,
    pub rom_status_reg: u32,
    pub rom_init_timeout: u32,
    pub ssp_count: u32,
    pub ssp_base_offset: u32,
    pub sdw_shim_base: u32,
    pub sdw_alh_base: u32,
    pub d0i3_offset: u32,
    pub read_sdw_lcount: Option<unsafe extern "C" fn()>,
    pub enable_sdw_irq: Option<unsafe extern "C" fn()>,
    pub check_sdw_irq: Option<unsafe extern "C" fn()>,
    pub check_sdw_wakeen_irq: Option<unsafe extern "C" fn()>,
    pub sdw_process_wakeen: Option<unsafe extern "C" fn()>,
    pub check_ipc_irq: Option<unsafe extern "C" fn()>,
    pub cl_init: Option<unsafe extern "C" fn()>,
    pub power_down_dsp: Option<unsafe extern "C" fn()>,
    pub disable_interrupts: Option<unsafe extern "C" fn()>,
    pub hw_ip_version: u32,
    pub platform: *const c_char,
}

extern "C" {
    static sof_hda_common_ops: snd_sof_dsp_ops;
    static mut system_dfl_wq: *mut c_void;

    static hda_dsp_shutdown: unsafe extern "C" fn();
    static hda_dsp_set_power_state_ipc3: unsafe extern "C" fn();
    static hda_dsp_set_power_state_ipc4: unsafe extern "C" fn();
    static hda_dsp_post_fw_run: unsafe extern "C" fn();
    static hda_dsp_cl_boot_firmware: unsafe extern "C" fn();
    static hda_dsp_core_get: unsafe extern "C" fn();
    static hda_dsp_ipc4_load_library: unsafe extern "C" fn();
    static hda_sdw_check_lcount_common: unsafe extern "C" fn();
    static hda_common_enable_sdw_irq: unsafe extern "C" fn();
    static hda_common_check_sdw_irq: unsafe extern "C" fn();
    static hda_sdw_check_wakeen_irq_common: unsafe extern "C" fn();
    static hda_sdw_process_wakeen_common: unsafe extern "C" fn();
    static hda_dsp_check_ipc_irq: unsafe extern "C" fn();
    static cl_dsp_init: unsafe extern "C" fn();
    static hda_power_down_dsp: unsafe extern "C" fn();
    static hda_dsp_disable_interrupts: unsafe extern "C" fn();

    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u32;
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: u32, offset: u32, value: u32);
    fn snd_sof_dsp_update_bits(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u32, value: u32);
    fn snd_sof_dsp_update_bits_forced(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u32, value: u32);
    fn snd_sof_ipc_get_reply(sdev: *mut snd_sof_dev);
    fn hda_dsp_ipc_get_reply(sdev: *mut snd_sof_dev);
    fn snd_sof_ipc_reply(sdev: *mut snd_sof_dev, msg: u32);
    fn snd_sof_ipc_msgs_rx(sdev: *mut snd_sof_dev);
    fn dev_dbg_ratelimited(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn hda_ipc4_tx_is_busy(sdev: *mut snd_sof_dev) -> bool_;
    fn sof_mailbox_write(sdev: *mut snd_sof_dev, offset: u32, src: *mut c_void, size: u32);
    fn hda_dsp_ipc4_schedule_d0i3_work(hdev: *mut sof_intel_hda_dev, msg: *mut snd_sof_ipc_msg);
    fn trace_sof_intel_ipc_firmware_response(sdev: *mut snd_sof_dev, msg: u32, msg_ext: u32);
    fn trace_sof_intel_ipc_firmware_initiated(sdev: *mut snd_sof_dev, msg: u32, msg_ext: u32);
    fn snd_sof_dsp_panic(sdev: *mut snd_sof_dev, offset: u32, non_recoverable: bool_);
    fn hda_ipc_irq_dump(sdev: *mut snd_sof_dev);
    fn mod_delayed_work(wq: *mut c_void, dwork: *mut c_void, delay: u64);
    fn msecs_to_jiffies(msecs: u32) -> u64;
    fn hda_set_dai_drv_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
}

const IRQ_HANDLED: irqreturn_t = 1;
const ENOMEM: c_int = 12;
const GFP_KERNEL: u32 = 0;

const fn GENMASK(h: u32, l: u32) -> u32 {
    if h == 31 {
        u32::MAX << l
    } else {
        ((1u32 << (h + 1)) - 1) & !((1u32 << l) - 1)
    }
}

static cnl_dsp_debugfs: [snd_sof_debugfs_map; 3] = [
    snd_sof_debugfs_map {
        name: b"hda\0".as_ptr() as *const c_char,
        bar: HDA_DSP_HDA_BAR,
        offset: 0,
        size: 0x4000,
        access: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: b"pp\0".as_ptr() as *const c_char,
        bar: HDA_DSP_PP_BAR,
        offset: 0,
        size: 0x1000,
        access: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: b"dsp\0".as_ptr() as *const c_char,
        bar: HDA_DSP_BAR,
        offset: 0,
        size: 0x10000,
        access: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
];

unsafe fn cnl_ipc_host_done(sdev: *mut snd_sof_dev);
unsafe fn cnl_ipc_dsp_done(sdev: *mut snd_sof_dev);

#[no_mangle]
pub unsafe extern "C" fn cnl_ipc4_irq_thread(_irq: c_int, context: *mut c_void) -> irqreturn_t {
    let mut notification_data = sof_ipc4_msg {
        primary: 0,
        extension: 0,
        data_size: 0,
        data_ptr: ptr::null_mut(),
    };
    let sdev = context as *mut snd_sof_dev;
    let mut ack_received = false;
    let mut ipc_irq = false;

    let hipcida = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCIDA);
    let hipctdr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCTDR);
    if hipcida & CNL_DSP_REG_HIPCIDA_DONE != 0 {
        /* DSP received the message */
        snd_sof_dsp_update_bits(
            sdev,
            HDA_DSP_BAR,
            CNL_DSP_REG_HIPCCTL,
            CNL_DSP_REG_HIPCCTL_DONE,
            0,
        );
        cnl_ipc_dsp_done(sdev);

        ipc_irq = true;
        ack_received = true;
    }

    if hipctdr & CNL_DSP_REG_HIPCTDR_BUSY != 0 {
        /* Message from DSP (reply or notification) */
        let hipctdd = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCTDD);
        let primary = hipctdr & CNL_DSP_REG_HIPCTDR_MSG_MASK;
        let extension = hipctdd & CNL_DSP_REG_HIPCTDD_MSG_MASK;

        if primary & SOF_IPC4_MSG_DIR_MASK != 0 {
            /* Reply received */
            if (*sdev).fw_state == SOF_FW_BOOT_COMPLETE {
                let data = (*(*sdev).ipc).msg.reply_data as *mut sof_ipc4_msg;

                (*data).primary = primary;
                (*data).extension = extension;

                /* C guard(spinlock_irq)(&sdev->ipc_lock) scopes the IPC lock here. */
                snd_sof_ipc_get_reply(sdev);
                cnl_ipc_host_done(sdev);
                snd_sof_ipc_reply(sdev, (*data).primary);
            } else {
                dev_dbg_ratelimited(
                    (*sdev).dev,
                    b"IPC reply before FW_READY: %#x|%#x\n\0".as_ptr() as *const c_char,
                    primary,
                    extension,
                );
            }
        } else {
            /* Notification received */
            notification_data.primary = primary;
            notification_data.extension = extension;

            (*(*sdev).ipc).msg.rx_data = &mut notification_data;
            snd_sof_ipc_msgs_rx(sdev);
            (*(*sdev).ipc).msg.rx_data = ptr::null_mut();

            /* Let DSP know that we have finished processing the message */
            cnl_ipc_host_done(sdev);
        }

        ipc_irq = true;
    }

    if !ipc_irq {
        /* This interrupt is not shared so no need to return IRQ_NONE. */
        dev_dbg_ratelimited(
            (*sdev).dev,
            b"nothing to do in IPC IRQ thread\n\0".as_ptr() as *const c_char,
        );
    }

    if ack_received {
        let hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

        if !(*hdev).delayed_ipc_tx_msg.is_null() {
            cnl_ipc4_send_msg(sdev, (*hdev).delayed_ipc_tx_msg);
        }
    }

    IRQ_HANDLED
}
/* EXPORT_SYMBOL_NS(cnl_ipc4_irq_thread, "SND_SOC_SOF_INTEL_CNL"); */

#[no_mangle]
pub unsafe extern "C" fn cnl_ipc_irq_thread(_irq: c_int, context: *mut c_void) -> irqreturn_t {
    let sdev = context as *mut snd_sof_dev;
    let mut ipc_irq = false;

    let hipcida = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCIDA);
    let hipctdr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCTDR);
    let hipctdd = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCTDD);
    let hipci = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCIDR);

    /* reply message from DSP */
    if hipcida & CNL_DSP_REG_HIPCIDA_DONE != 0 {
        let msg_ext = hipci & CNL_DSP_REG_HIPCIDR_MSG_MASK;
        let msg = hipcida & CNL_DSP_REG_HIPCIDA_MSG_MASK;

        trace_sof_intel_ipc_firmware_response(sdev, msg, msg_ext);

        /* mask Done interrupt */
        snd_sof_dsp_update_bits(
            sdev,
            HDA_DSP_BAR,
            CNL_DSP_REG_HIPCCTL,
            CNL_DSP_REG_HIPCCTL_DONE,
            0,
        );

        if (*sdev).fw_state == SOF_FW_BOOT_COMPLETE {
            /* handle immediate reply from DSP core */
            /* C guard(spinlock_irq)(&sdev->ipc_lock) scopes the IPC lock here. */
            hda_dsp_ipc_get_reply(sdev);
            snd_sof_ipc_reply(sdev, msg);
            cnl_ipc_dsp_done(sdev);
        } else {
            dev_dbg_ratelimited(
                (*sdev).dev,
                b"IPC reply before FW_READY: %#x\n\0".as_ptr() as *const c_char,
                msg,
            );
        }

        ipc_irq = true;
    }

    /* new message from DSP */
    if hipctdr & CNL_DSP_REG_HIPCTDR_BUSY != 0 {
        let msg = hipctdr & CNL_DSP_REG_HIPCTDR_MSG_MASK;
        let msg_ext = hipctdd & CNL_DSP_REG_HIPCTDD_MSG_MASK;

        trace_sof_intel_ipc_firmware_initiated(sdev, msg, msg_ext);

        /* handle messages from DSP */
        if (hipctdr & SOF_IPC_PANIC_MAGIC_MASK) == SOF_IPC_PANIC_MAGIC {
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
            snd_sof_ipc_msgs_rx(sdev);
        }

        cnl_ipc_host_done(sdev);

        ipc_irq = true;
    }

    if !ipc_irq {
        /*
         * This interrupt is not shared so no need to return IRQ_NONE.
         */
        dev_dbg_ratelimited(
            (*sdev).dev,
            b"nothing to do in IPC IRQ thread\n\0".as_ptr() as *const c_char,
        );
    }

    IRQ_HANDLED
}
/* EXPORT_SYMBOL_NS(cnl_ipc_irq_thread, "SND_SOC_SOF_INTEL_CNL"); */

unsafe fn cnl_ipc_host_done(sdev: *mut snd_sof_dev) {
    /*
     * clear busy interrupt to tell dsp controller this
     * interrupt has been accepted, not trigger it again
     */
    snd_sof_dsp_update_bits_forced(
        sdev,
        HDA_DSP_BAR,
        CNL_DSP_REG_HIPCTDR,
        CNL_DSP_REG_HIPCTDR_BUSY,
        CNL_DSP_REG_HIPCTDR_BUSY,
    );
    /*
     * set done bit to ack dsp the msg has been
     * processed and send reply msg to dsp
     */
    snd_sof_dsp_update_bits_forced(
        sdev,
        HDA_DSP_BAR,
        CNL_DSP_REG_HIPCTDA,
        CNL_DSP_REG_HIPCTDA_DONE,
        CNL_DSP_REG_HIPCTDA_DONE,
    );
}

unsafe fn cnl_ipc_dsp_done(sdev: *mut snd_sof_dev) {
    /*
     * set DONE bit - tell DSP we have received the reply msg
     * from DSP, and processed it, don't send more reply to host
     */
    snd_sof_dsp_update_bits_forced(
        sdev,
        HDA_DSP_BAR,
        CNL_DSP_REG_HIPCIDA,
        CNL_DSP_REG_HIPCIDA_DONE,
        CNL_DSP_REG_HIPCIDA_DONE,
    );

    /* unmask Done interrupt */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_BAR,
        CNL_DSP_REG_HIPCCTL,
        CNL_DSP_REG_HIPCCTL_DONE,
        CNL_DSP_REG_HIPCCTL_DONE,
    );
}

unsafe fn cnl_compact_ipc_compress(msg: *mut snd_sof_ipc_msg, dr: *mut u32, dd: *mut u32) -> bool_ {
    let pm_gate = (*msg).msg_data as *mut sof_ipc_pm_gate;

    if (*pm_gate).hdr.cmd == (SOF_IPC_GLB_PM_MSG | SOF_IPC_PM_GATE) {
        /* send the compact message via the primary register */
        *dr = HDA_IPC_MSG_COMPACT | HDA_IPC_PM_GATE;

        /* send payload via the extended data register */
        *dd = (*pm_gate).flags;

        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn cnl_ipc4_send_msg(
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

    snd_sof_dsp_write(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCIDD, (*msg_data).extension);
    snd_sof_dsp_write(
        sdev,
        HDA_DSP_BAR,
        CNL_DSP_REG_HIPCIDR,
        (*msg_data).primary | CNL_DSP_REG_HIPCIDR_BUSY,
    );

    hda_dsp_ipc4_schedule_d0i3_work(hdev, msg);

    0
}
/* EXPORT_SYMBOL_NS(cnl_ipc4_send_msg, "SND_SOC_SOF_INTEL_CNL"); */

#[no_mangle]
pub unsafe extern "C" fn cnl_ipc_send_msg(
    sdev: *mut snd_sof_dev,
    msg: *mut snd_sof_ipc_msg,
) -> c_int {
    let hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let mut dr: u32 = 0;
    let mut dd: u32 = 0;

    /*
     * Currently the only compact IPC supported is the PM_GATE
     * IPC which is used for transitioning the DSP between the
     * D0I0 and D0I3 states. And these are sent only during the
     * set_power_state() op. Therefore, there will never be a case
     * that a compact IPC results in the DSP exiting D0I3 without
     * the host and FW being in sync.
     */
    if cnl_compact_ipc_compress(msg, &mut dr, &mut dd) {
        /* send the message via IPC registers */
        snd_sof_dsp_write(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCIDD, dd);
        snd_sof_dsp_write(
            sdev,
            HDA_DSP_BAR,
            CNL_DSP_REG_HIPCIDR,
            CNL_DSP_REG_HIPCIDR_BUSY | dr,
        );
        return 0;
    }

    /* send the message via mailbox */
    sof_mailbox_write(sdev, (*sdev).host_box.offset, (*msg).msg_data, (*msg).msg_size);
    snd_sof_dsp_write(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCIDR, CNL_DSP_REG_HIPCIDR_BUSY);

    let hdr = (*msg).msg_data as *mut sof_ipc_cmd_hdr;

    /*
     * Use mod_delayed_work() to schedule the delayed work
     * to avoid scheduling multiple workqueue items when
     * IPCs are sent at a high-rate. mod_delayed_work()
     * modifies the timer if the work is pending.
     * Also, a new delayed work should not be queued after the
     * CTX_SAVE IPC, which is sent before the DSP enters D3.
     */
    if (*hdr).cmd != (SOF_IPC_GLB_PM_MSG | SOF_IPC_PM_CTX_SAVE) {
        mod_delayed_work(
            system_dfl_wq,
            &mut (*hdev).d0i3_work,
            msecs_to_jiffies(SOF_HDA_D0I3_WORK_DELAY_MS),
        );
    }

    0
}
/* EXPORT_SYMBOL_NS(cnl_ipc_send_msg, "SND_SOC_SOF_INTEL_CNL"); */

#[no_mangle]
pub unsafe extern "C" fn cnl_ipc_dump(sdev: *mut snd_sof_dev) {
    hda_ipc_irq_dump(sdev);

    /* read IPC status */
    let hipcida = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCIDA);
    let hipcctl = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCCTL);
    let hipctdr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCTDR);

    /* dump the IPC regs */
    /* TODO: parse the raw msg */
    dev_err(
        (*sdev).dev,
        b"error: host status 0x%8.8x dsp status 0x%8.8x mask 0x%8.8x\n\0".as_ptr()
            as *const c_char,
        hipcida,
        hipctdr,
        hipcctl,
    );
}
/* EXPORT_SYMBOL_NS(cnl_ipc_dump, "SND_SOC_SOF_INTEL_CNL"); */

#[no_mangle]
pub unsafe extern "C" fn cnl_ipc4_dump(sdev: *mut snd_sof_dev) {
    hda_ipc_irq_dump(sdev);

    let hipcidr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCIDR);
    let hipcidd = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCIDD);
    let hipcida = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCIDA);
    let hipctdr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCTDR);
    let hipctdd = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCTDD);
    let hipctda = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCTDA);
    let hipcctl = snd_sof_dsp_read(sdev, HDA_DSP_BAR, CNL_DSP_REG_HIPCCTL);

    /* dump the IPC regs */
    /* TODO: parse the raw msg */
    dev_err(
        (*sdev).dev,
        b"Host IPC initiator: %#x|%#x|%#x, target: %#x|%#x|%#x, ctl: %#x\n\0".as_ptr()
            as *const c_char,
        hipcidr,
        hipcidd,
        hipcida,
        hipctdr,
        hipctdd,
        hipctda,
        hipcctl,
    );
}
/* EXPORT_SYMBOL_NS(cnl_ipc4_dump, "SND_SOC_SOF_INTEL_CNL"); */

/* cannonlake ops */
#[no_mangle]
pub static mut sof_cnl_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    shutdown: None,
    irq_thread: None,
    send_msg: None,
    ipc_dump: None,
    set_power_state: None,
    debug_map: ptr::null(),
    debug_map_count: 0,
    post_fw_run: None,
    run: None,
    core_get: None,
};
/* EXPORT_SYMBOL_NS(sof_cnl_ops, "SND_SOC_SOF_INTEL_CNL"); */

#[no_mangle]
pub unsafe extern "C" fn sof_cnl_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    /* common defaults */
    ptr::copy_nonoverlapping(&sof_hda_common_ops, &mut sof_cnl_ops, 1);

    /* probe/remove/shutdown */
    sof_cnl_ops.shutdown = Some(hda_dsp_shutdown);

    /* ipc */
    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_3 {
        /* doorbell */
        sof_cnl_ops.irq_thread = Some(cnl_ipc_irq_thread);

        /* ipc */
        sof_cnl_ops.send_msg = Some(cnl_ipc_send_msg);

        /* debug */
        sof_cnl_ops.ipc_dump = Some(cnl_ipc_dump);

        sof_cnl_ops.set_power_state = Some(hda_dsp_set_power_state_ipc3);
    }

    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_4 {
        let ipc4_data: *mut sof_ipc4_fw_data;

        (*sdev).private = kzalloc(size_of::<sof_ipc4_fw_data>(), GFP_KERNEL);
        if (*sdev).private.is_null() {
            return -ENOMEM;
        }

        ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
        (*ipc4_data).manifest_fw_hdr_offset = SOF_MAN4_FW_HDR_OFFSET;

        (*ipc4_data).mtrace_type = SOF_IPC4_MTRACE_INTEL_CAVS_1_8;

        /* External library loading support */
        (*ipc4_data).load_library = Some(hda_dsp_ipc4_load_library);

        /* doorbell */
        sof_cnl_ops.irq_thread = Some(cnl_ipc4_irq_thread);

        /* ipc */
        sof_cnl_ops.send_msg = Some(cnl_ipc4_send_msg);

        /* debug */
        sof_cnl_ops.ipc_dump = Some(cnl_ipc4_dump);

        sof_cnl_ops.set_power_state = Some(hda_dsp_set_power_state_ipc4);
    }

    /* set DAI driver ops */
    hda_set_dai_drv_ops(sdev, &mut sof_cnl_ops);

    /* debug */
    sof_cnl_ops.debug_map = cnl_dsp_debugfs.as_ptr();
    sof_cnl_ops.debug_map_count = cnl_dsp_debugfs.len();

    /* pre/post fw run */
    sof_cnl_ops.post_fw_run = Some(hda_dsp_post_fw_run);

    /* firmware run */
    sof_cnl_ops.run = Some(hda_dsp_cl_boot_firmware);

    /* dsp core get/put */
    sof_cnl_ops.core_get = Some(hda_dsp_core_get);

    0
}
/* EXPORT_SYMBOL_NS(sof_cnl_ops_init, "SND_SOC_SOF_INTEL_CNL"); */

#[no_mangle]
pub static cnl_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    /* Cannonlake */
    cores_num: 4,
    init_core_mask: 1,
    host_managed_cores_mask: GENMASK(3, 0),
    ipc_req: CNL_DSP_REG_HIPCIDR,
    ipc_req_mask: CNL_DSP_REG_HIPCIDR_BUSY,
    ipc_ack: CNL_DSP_REG_HIPCIDA,
    ipc_ack_mask: CNL_DSP_REG_HIPCIDA_DONE,
    ipc_ctl: CNL_DSP_REG_HIPCCTL,
    rom_status_reg: HDA_DSP_SRAM_REG_ROM_STATUS,
    rom_init_timeout: 300,
    ssp_count: CNL_SSP_COUNT,
    ssp_base_offset: CNL_SSP_BASE_OFFSET,
    sdw_shim_base: SDW_SHIM_BASE,
    sdw_alh_base: SDW_ALH_BASE,
    d0i3_offset: SOF_HDA_VS_D0I3C,
    read_sdw_lcount: Some(hda_sdw_check_lcount_common),
    enable_sdw_irq: Some(hda_common_enable_sdw_irq),
    check_sdw_irq: Some(hda_common_check_sdw_irq),
    check_sdw_wakeen_irq: Some(hda_sdw_check_wakeen_irq_common),
    sdw_process_wakeen: Some(hda_sdw_process_wakeen_common),
    check_ipc_irq: Some(hda_dsp_check_ipc_irq),
    cl_init: Some(cl_dsp_init),
    power_down_dsp: Some(hda_power_down_dsp),
    disable_interrupts: Some(hda_dsp_disable_interrupts),
    hw_ip_version: SOF_INTEL_CAVS_1_8,
    platform: b"cnl\0".as_ptr() as *const c_char,
};

/*
 * JasperLake is technically derived from IceLake, and should be in
 * described in icl.c. However since JasperLake was designed with
 * two cores, it cannot support the IceLake-specific power-up sequences
 * which rely on core3. To simplify, JasperLake uses the CannonLake ops and
 * is described in cnl.c
 */
#[no_mangle]
pub static jsl_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    /* Jasperlake */
    cores_num: 2,
    init_core_mask: 1,
    host_managed_cores_mask: GENMASK(1, 0),
    ipc_req: CNL_DSP_REG_HIPCIDR,
    ipc_req_mask: CNL_DSP_REG_HIPCIDR_BUSY,
    ipc_ack: CNL_DSP_REG_HIPCIDA,
    ipc_ack_mask: CNL_DSP_REG_HIPCIDA_DONE,
    ipc_ctl: CNL_DSP_REG_HIPCCTL,
    rom_status_reg: HDA_DSP_SRAM_REG_ROM_STATUS,
    rom_init_timeout: 300,
    ssp_count: ICL_SSP_COUNT,
    ssp_base_offset: CNL_SSP_BASE_OFFSET,
    sdw_shim_base: SDW_SHIM_BASE,
    sdw_alh_base: SDW_ALH_BASE,
    d0i3_offset: SOF_HDA_VS_D0I3C,
    read_sdw_lcount: Some(hda_sdw_check_lcount_common),
    enable_sdw_irq: Some(hda_common_enable_sdw_irq),
    check_sdw_irq: Some(hda_common_check_sdw_irq),
    check_sdw_wakeen_irq: Some(hda_sdw_check_wakeen_irq_common),
    sdw_process_wakeen: Some(hda_sdw_process_wakeen_common),
    check_ipc_irq: Some(hda_dsp_check_ipc_irq),
    cl_init: Some(cl_dsp_init),
    power_down_dsp: Some(hda_power_down_dsp),
    disable_interrupts: Some(hda_dsp_disable_interrupts),
    hw_ip_version: SOF_INTEL_CAVS_2_0,
    platform: b"jsl\0".as_ptr() as *const c_char,
};
/* EXPORT_SYMBOL_NS(jsl_chip_info, "SND_SOC_SOF_INTEL_CNL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
