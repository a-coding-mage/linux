// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021, 2023 Advanced Micro Devices, Inc.
//
// Authors: Balakishore Pati <Balakishore.pati@amd.com>
//	    Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>

/* ACP-specific SOF IPC code */

// C dependencies: <linux/module.h>, "../ops.h", "acp.h", "acp-dsp-offset.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

pub type u32 = u32;
pub type size_t = usize;
pub type bool_ = bool;
pub type irqreturn_t = c_uint;

pub const IRQ_HANDLED: irqreturn_t = 1;

extern "C" {
    pub static ACP_DSP_BAR: c_uint;
    pub static ACP7B_PCI_ID: c_uint;
    pub static ACP7F_PCI_ID: c_uint;
    pub static ACP7X_DSP_SW_INTR_TRIG_OFFSET: c_uint;
    pub static DSP_SW_INTR_TRIG_OFFSET: c_uint;
    pub static ACP_SCRATCH_REG_0: c_uint;
    pub static ACP_HW_SEM_RETRY_COUNT: c_uint;
    pub static ACP_DSP_MSG_SET: c_int;
    pub static ACP_DSP_ACK_SET: c_int;
    pub static SOF_FW_BOOT_COMPLETE: c_int;
    pub static SOF_IPC_PANIC_MAGIC_MASK: u32;
    pub static SOF_IPC_PANIC_MAGIC: u32;
    pub static SOF_IPC_GLB_PM_MSG: u32;
    pub static SOF_IPC_PM_CTX_SAVE: u32;
    pub static SOF_IPC_PM_GATE: u32;
    pub static SOF_IPC_GLB_REPLY: u32;
    pub static SOF_IPC_GLB_PROBE: u32;
    pub static PROBE_STATUS_BIT: u32;
    pub static EINVAL: c_int;
    pub static ESTRPIPE: c_int;

    pub fn memcpy_to_scratch(
        sdev: *mut snd_sof_dev,
        offset: u32,
        message: *mut c_void,
        bytes: size_t,
    );
    pub fn memcpy_from_scratch(
        sdev: *mut snd_sof_dev,
        offset: u32,
        message: *mut c_void,
        bytes: size_t,
    );
    pub fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_amd_acp_desc;
    pub fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: c_uint, offset: c_uint) -> u32;
    pub fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: c_uint, offset: c_uint, value: u32);
    pub fn snd_sof_dsp_panic(sdev: *mut snd_sof_dev, offset: c_uint, non_recoverable: bool_);
    pub fn snd_sof_ipc_msgs_rx(sdev: *mut snd_sof_dev);
    pub fn snd_sof_ipc_reply(sdev: *mut snd_sof_dev, error: c_int);
    pub fn snd_compr_fragment_elapsed(cstream: *mut c_void);
    pub fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    pub fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    pub fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    pub fn dev_dbg_ratelimited(dev: *mut c_void, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct snd_sof_dev {
    pub pdata: *mut snd_sof_pdata,
    pub debug_box: snd_sof_mailbox,
    pub host_box: snd_sof_mailbox,
    pub dsp_box: snd_sof_mailbox,
    pub stream_box: snd_sof_mailbox,
    pub msg: *mut snd_sof_ipc_msg,
    pub dev: *mut c_void,
    pub first_boot: bool_,
    pub fw_state: c_int,
    pub ipc_lock: spinlock_t,
    pub dsp_oops_offset: c_uint,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut acp_dev_data,
}

#[repr(C)]
pub struct snd_sof_mailbox {
    pub offset: c_uint,
    pub size: size_t,
}

#[repr(C)]
pub struct acp_dev_data {
    pub dev: *mut snd_sof_dev,
    pub pci_rev: c_uint,
    pub probe_stream: *mut acp_dsp_stream,
}

#[repr(C)]
pub struct sof_amd_acp_desc {
    pub dsp_intr_base: c_uint,
    pub hw_semaphore_offset: c_uint,
    pub probe_reg_offset: c_uint,
    pub sram_pte_offset: c_int,
}

#[repr(C)]
pub struct scratch_ipc_conf {
    pub sof_host_msg_write: u32,
    pub sof_dsp_msg_write: u32,
    pub sof_dsp_ack_write: u32,
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
    pub msg_data: *mut c_void,
    pub msg_size: size_t,
    pub reply_data: *mut c_void,
    pub reply_size: size_t,
    pub reply_error: c_int,
}

#[repr(C)]
pub struct sof_ipc_cmd_hdr {
    pub size: u32,
    pub cmd: u32,
}

#[repr(C)]
pub struct sof_ipc_reply {
    pub hdr: sof_ipc_cmd_hdr,
    pub error: c_int,
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
pub struct acp_dsp_stream {
    pub posn_offset: size_t,
    pub cstream_posn: u32,
    pub cstream: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc_stream_posn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

const fn offset_of_scratch_ipc_conf_sof_host_msg_write() -> c_uint {
    0
}

const fn offset_of_scratch_ipc_conf_sof_dsp_msg_write() -> c_uint {
    size_of::<u32>() as c_uint
}

const fn offset_of_scratch_ipc_conf_sof_dsp_ack_write() -> c_uint {
    (size_of::<u32>() * 2) as c_uint
}

#[no_mangle]
pub unsafe extern "C" fn acp_mailbox_write(
    sdev: *mut snd_sof_dev,
    offset: u32,
    message: *mut c_void,
    bytes: size_t,
) {
    memcpy_to_scratch(sdev, offset, message, bytes);
}
// EXPORT_SYMBOL_NS(acp_mailbox_write, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn acp_mailbox_read(
    sdev: *mut snd_sof_dev,
    offset: u32,
    message: *mut c_void,
    bytes: size_t,
) {
    memcpy_from_scratch(sdev, offset, message, bytes);
}
// EXPORT_SYMBOL_NS(acp_mailbox_read, "SND_SOC_SOF_AMD_COMMON");

unsafe fn acpbus_trigger_host_to_dsp_swintr(adata: *mut acp_dev_data) {
    let sdev: *mut snd_sof_dev = (*adata).dev;
    let desc: *const sof_amd_acp_desc = get_chip_info((*sdev).pdata);
    let mut swintr_trigger: u32;
    let swintr_trigger_reg_offset: c_uint;

    if (*adata).pci_rev == ACP7B_PCI_ID || (*adata).pci_rev == ACP7F_PCI_ID {
        swintr_trigger_reg_offset = ACP7X_DSP_SW_INTR_TRIG_OFFSET;
    } else {
        swintr_trigger_reg_offset = DSP_SW_INTR_TRIG_OFFSET;
    }

    swintr_trigger = snd_sof_dsp_read(
        sdev,
        ACP_DSP_BAR,
        (*desc).dsp_intr_base.wrapping_add(swintr_trigger_reg_offset),
    );
    swintr_trigger |= 0x01;
    snd_sof_dsp_write(
        sdev,
        ACP_DSP_BAR,
        (*desc).dsp_intr_base.wrapping_add(swintr_trigger_reg_offset),
        swintr_trigger,
    );
}

unsafe fn acp_ipc_host_msg_set(sdev: *mut snd_sof_dev) {
    let host_msg: c_uint = (*sdev)
        .debug_box
        .offset
        .wrapping_add(offset_of_scratch_ipc_conf_sof_host_msg_write());

    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0.wrapping_add(host_msg), 1);
}

unsafe fn acp_dsp_ipc_host_done(sdev: *mut snd_sof_dev) {
    let dsp_msg: c_uint = (*sdev)
        .debug_box
        .offset
        .wrapping_add(offset_of_scratch_ipc_conf_sof_dsp_msg_write());

    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0.wrapping_add(dsp_msg), 0);
}

unsafe fn acp_dsp_ipc_dsp_done(sdev: *mut snd_sof_dev) {
    let dsp_ack: c_uint = (*sdev)
        .debug_box
        .offset
        .wrapping_add(offset_of_scratch_ipc_conf_sof_dsp_ack_write());

    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0.wrapping_add(dsp_ack), 0);
}

#[no_mangle]
pub unsafe extern "C" fn acp_sof_ipc_send_msg(
    sdev: *mut snd_sof_dev,
    msg: *mut snd_sof_ipc_msg,
) -> c_int {
    let adata: *mut acp_dev_data = (*(*sdev).pdata).hw_pdata;
    let desc: *const sof_amd_acp_desc = get_chip_info((*sdev).pdata);
    let offset: c_uint = (*sdev).host_box.offset;
    let mut count: c_uint = ACP_HW_SEM_RETRY_COUNT;

    while snd_sof_dsp_read(sdev, ACP_DSP_BAR, (*desc).hw_semaphore_offset) != 0 {
        /* Wait until acquired HW Semaphore Lock or timeout*/
        count = count.wrapping_sub(1);
        if count == 0 {
            dev_err(
                (*sdev).dev,
                b"%s: Failed to acquire HW lock\n\0".as_ptr() as *const c_char,
                b"acp_sof_ipc_send_msg\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }
    }

    acp_mailbox_write(sdev, offset, (*msg).msg_data, (*msg).msg_size);
    acp_ipc_host_msg_set(sdev);

    /* Trigger host to dsp interrupt for the msg */
    acpbus_trigger_host_to_dsp_swintr(adata);

    /* Unlock or Release HW Semaphore */
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).hw_semaphore_offset, 0x0);

    0
}
// EXPORT_SYMBOL_NS(acp_sof_ipc_send_msg, "SND_SOC_SOF_AMD_COMMON");

unsafe fn acp_dsp_ipc_get_reply(sdev: *mut snd_sof_dev) {
    let msg: *mut snd_sof_ipc_msg = (*sdev).msg;
    let mut reply: sof_ipc_reply = core::mem::zeroed();
    let mut hdr: *mut sof_ipc_cmd_hdr;
    let offset: c_uint = (*sdev).host_box.offset;
    let mut ret: c_int = 0;

    /*
     * Sometimes, there is unexpected reply ipc arriving. The reply
     * ipc belongs to none of the ipcs sent from driver.
     * In this case, the driver must ignore the ipc.
     */
    if msg.is_null() {
        dev_warn(
            (*sdev).dev,
            b"unexpected ipc interrupt raised!\n\0".as_ptr() as *const c_char,
        );
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
        ptr::copy_nonoverlapping(
            &reply as *const sof_ipc_reply as *const u8,
            (*msg).reply_data as *mut u8,
            size_of::<sof_ipc_reply>(),
        );
    } else {
        /* get IPC reply from DSP in the mailbox */
        acp_mailbox_read(
            sdev,
            offset,
            &mut reply as *mut sof_ipc_reply as *mut c_void,
            size_of::<sof_ipc_reply>(),
        );
        if reply.error < 0 {
            ptr::copy_nonoverlapping(
                &reply as *const sof_ipc_reply as *const u8,
                (*msg).reply_data as *mut u8,
                size_of::<sof_ipc_reply>(),
            );
            ret = reply.error;
        } else {
            /*
             * To support an IPC tx_message with a
             * reply_size set to zero.
             */
            if (*msg).reply_size != 0 {
                /* reply correct size ? */
                if reply.hdr.size as size_t != (*msg).reply_size
                    && (reply.hdr.cmd & SOF_IPC_GLB_PROBE) == 0
                {
                    dev_err(
                        (*sdev).dev,
                        b"reply expected %zu got %u bytes\n\0".as_ptr() as *const c_char,
                        (*msg).reply_size,
                        reply.hdr.size,
                    );
                    ret = -EINVAL;
                }
                /* read the message */
                if (*msg).reply_size > 0 {
                    acp_mailbox_read(sdev, offset, (*msg).reply_data, (*msg).reply_size);
                }
            }
        }
    }
    (*msg).reply_error = ret;
}

#[no_mangle]
pub unsafe extern "C" fn acp_sof_ipc_irq_thread(
    _irq: c_int,
    context: *mut c_void,
) -> irqreturn_t {
    let sdev: *mut snd_sof_dev = context as *mut snd_sof_dev;
    let desc: *const sof_amd_acp_desc = get_chip_info((*sdev).pdata);
    let adata: *mut acp_dev_data = (*(*sdev).pdata).hw_pdata;
    let dsp_msg_write: c_uint = (*sdev)
        .debug_box
        .offset
        .wrapping_add(offset_of_scratch_ipc_conf_sof_dsp_msg_write());
    let dsp_ack_write: c_uint = (*sdev)
        .debug_box
        .offset
        .wrapping_add(offset_of_scratch_ipc_conf_sof_dsp_ack_write());
    let mut ipc_irq: bool_ = false;
    let dsp_msg: c_int;
    let dsp_ack: c_int;
    let mut status: c_uint;

    if (*sdev).first_boot && (*sdev).fw_state != SOF_FW_BOOT_COMPLETE {
        status = 0;
        acp_mailbox_read(
            sdev,
            (*sdev).dsp_box.offset,
            &mut status as *mut c_uint as *mut c_void,
            size_of::<c_uint>(),
        );

        if (status & SOF_IPC_PANIC_MAGIC_MASK) == SOF_IPC_PANIC_MAGIC {
            snd_sof_dsp_panic(
                sdev,
                (*sdev).dsp_box.offset.wrapping_add(size_of::<c_uint>() as c_uint),
                true,
            );
            status = 0;
            acp_mailbox_write(
                sdev,
                (*sdev).dsp_box.offset,
                &mut status as *mut c_uint as *mut c_void,
                size_of_val_u32(&status),
            );
            return IRQ_HANDLED;
        }
        snd_sof_ipc_msgs_rx(sdev);
        acp_dsp_ipc_host_done(sdev);
        return IRQ_HANDLED;
    }

    dsp_msg = snd_sof_dsp_read(
        sdev,
        ACP_DSP_BAR,
        ACP_SCRATCH_REG_0.wrapping_add(dsp_msg_write),
    ) as c_int;
    if dsp_msg == ACP_DSP_MSG_SET {
        snd_sof_ipc_msgs_rx(sdev);
        acp_dsp_ipc_host_done(sdev);
        ipc_irq = true;
    }

    dsp_ack = snd_sof_dsp_read(
        sdev,
        ACP_DSP_BAR,
        ACP_SCRATCH_REG_0.wrapping_add(dsp_ack_write),
    ) as c_int;
    if dsp_ack == ACP_DSP_ACK_SET {
        if (*sdev).fw_state == SOF_FW_BOOT_COMPLETE {
            // guard(spinlock_irq)(&sdev->ipc_lock);

            /* handle immediate reply from DSP core */
            acp_dsp_ipc_get_reply(sdev);
            snd_sof_ipc_reply(sdev, 0);
            /* set the done bit */
            acp_dsp_ipc_dsp_done(sdev);
        } else {
            dev_dbg_ratelimited(
                (*sdev).dev,
                b"IPC reply before FW_BOOT_COMPLETE: %#x\n\0".as_ptr() as *const c_char,
                dsp_ack,
            );
        }

        ipc_irq = true;
    }

    status = 0;
    acp_mailbox_read(
        sdev,
        (*sdev).debug_box.offset,
        &mut status as *mut c_uint as *mut c_void,
        size_of::<u32>(),
    );
    if (status & SOF_IPC_PANIC_MAGIC_MASK) == SOF_IPC_PANIC_MAGIC {
        snd_sof_dsp_panic(sdev, (*sdev).dsp_oops_offset, true);
        status = 0;
        acp_mailbox_write(
            sdev,
            (*sdev).debug_box.offset,
            &mut status as *mut c_uint as *mut c_void,
            size_of_val_u32(&status),
        );
        return IRQ_HANDLED;
    }

    if (*desc).probe_reg_offset != 0 {
        let val: u32;
        let posn: u32;

        /* Probe register consists of two parts
         * (0-30) bit has cumulative position value
         * 31 bit is a synchronization flag between DSP and CPU
         * for the position update
         */
        val = snd_sof_dsp_read(sdev, ACP_DSP_BAR, (*desc).probe_reg_offset);
        if (val & PROBE_STATUS_BIT) != 0 {
            posn = val & !PROBE_STATUS_BIT;
            if !(*adata).probe_stream.is_null() {
                /* Probe related posn value is of 31 bits limited to 2GB
                 * once wrapped DSP won't send posn interrupt.
                 */
                (*(*adata).probe_stream).cstream_posn = posn;
                snd_compr_fragment_elapsed((*(*adata).probe_stream).cstream);
                snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).probe_reg_offset, posn);
                ipc_irq = true;
            }
        }
    }

    if !ipc_irq {
        dev_dbg_ratelimited(
            (*sdev).dev,
            b"nothing to do in IPC IRQ thread\n\0".as_ptr() as *const c_char,
        );
    }

    IRQ_HANDLED
}
// EXPORT_SYMBOL_NS(acp_sof_ipc_irq_thread, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn acp_sof_ipc_msg_data(
    sdev: *mut snd_sof_dev,
    sps: *mut snd_sof_pcm_stream,
    p: *mut c_void,
    sz: size_t,
) -> c_int {
    let offset: c_uint = (*sdev).dsp_box.offset;

    if sps.is_null() || (*sdev).stream_box.size == 0 {
        acp_mailbox_read(sdev, offset, p, sz);
    } else {
        let substream: *mut snd_pcm_substream = (*sps).substream;
        let stream: *mut acp_dsp_stream;

        if substream.is_null() || (*substream).runtime.is_null() {
            return -ESTRPIPE;
        }

        stream = (*(*substream).runtime).private_data as *mut acp_dsp_stream;

        if stream.is_null() {
            return -ESTRPIPE;
        }

        acp_mailbox_read(sdev, (*stream).posn_offset as c_uint, p, sz);
    }

    0
}
// EXPORT_SYMBOL_NS(acp_sof_ipc_msg_data, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn acp_set_stream_data_offset(
    sdev: *mut snd_sof_dev,
    sps: *mut snd_sof_pcm_stream,
    posn_offset: size_t,
) -> c_int {
    let substream: *mut snd_pcm_substream = (*sps).substream;
    let stream: *mut acp_dsp_stream =
        (*(*substream).runtime).private_data as *mut acp_dsp_stream;

    /* check for unaligned offset or overflow */
    if posn_offset > (*sdev).stream_box.size
        || posn_offset % size_of::<sof_ipc_stream_posn>() != 0
    {
        return -EINVAL;
    }

    (*stream).posn_offset = (*sdev).stream_box.offset as size_t + posn_offset;

    dev_dbg(
        (*sdev).dev,
        b"pcm: stream dir %d, posn mailbox offset is %zu\0".as_ptr() as *const c_char,
        (*substream).stream,
        (*stream).posn_offset,
    );

    0
}
// EXPORT_SYMBOL_NS(acp_set_stream_data_offset, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn acp_sof_ipc_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int {
    let desc: *const sof_amd_acp_desc = get_chip_info((*sdev).pdata);

    (*desc).sram_pte_offset
}
// EXPORT_SYMBOL_NS(acp_sof_ipc_get_mailbox_offset, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn acp_sof_ipc_get_window_offset(
    _sdev: *mut snd_sof_dev,
    _id: u32,
) -> c_int {
    0
}
// EXPORT_SYMBOL_NS(acp_sof_ipc_get_window_offset, "SND_SOC_SOF_AMD_COMMON");

const fn size_of_val_u32(_: &u32) -> usize {
    size_of::<u32>()
}

// MODULE_DESCRIPTION("AMD ACP sof-ipc driver");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
