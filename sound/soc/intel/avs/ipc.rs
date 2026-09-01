// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

// C dependencies:
// <linux/cleanup.h>
// <linux/io-64-nonatomic-lo-hi.h>
// <linux/slab.h>
// <sound/hdaudio_ext.h>
// "avs.h"
// "debug.h"
// "messages.h"
// "registers.h"
// "trace.h"

pub const AVS_IPC_TIMEOUT_MS: i32 = 300;
pub const AVS_D0IX_DELAY_MS: i32 = 300;

unsafe fn avs_dsp_set_d0ix(adev: *mut avs_dev, enable: bool) -> i32 {
    let ipc: *mut avs_ipc = (*adev).ipc;
    let ret: i32;

    /* Is transition required? */
    if (*ipc).in_d0ix == enable {
        return 0;
    }

    ret = avs_dsp_op!(adev, set_d0ix, enable);
    if ret != 0 {
        /* Prevent further d0ix attempts on conscious IPC failure. */
        if ret == -AVS_EIPC {
            atomic_inc(&mut (*ipc).d0ix_disable_depth);
        }

        (*ipc).in_d0ix = false;
        return ret;
    }

    (*ipc).in_d0ix = enable;
    0
}

unsafe fn avs_dsp_schedule_d0ix(adev: *mut avs_dev, _tx: *mut avs_ipc_msg) {
    if atomic_read(&mut (*(*adev).ipc).d0ix_disable_depth) != 0 {
        return;
    }

    mod_delayed_work(
        system_power_efficient_wq,
        &mut (*(*adev).ipc).d0ix_work,
        msecs_to_jiffies(AVS_D0IX_DELAY_MS),
    );
}

unsafe fn avs_dsp_d0ix_work(work: *mut work_struct) {
    let ipc: *mut avs_ipc = container_of!(work, avs_ipc, d0ix_work.work);

    avs_dsp_set_d0ix(to_avs_dev((*ipc).dev), true);
}

unsafe fn avs_dsp_wake_d0i0(adev: *mut avs_dev, _tx: *mut avs_ipc_msg) -> i32 {
    let ipc: *mut avs_ipc = (*adev).ipc;

    if atomic_read(&mut (*ipc).d0ix_disable_depth) == 0 {
        cancel_delayed_work_sync(&mut (*ipc).d0ix_work);
        return avs_dsp_set_d0ix(adev, false);
    }

    0
}

pub unsafe fn avs_dsp_disable_d0ix(adev: *mut avs_dev) -> i32 {
    let ipc: *mut avs_ipc = (*adev).ipc;

    /* Prevent PG only on the first disable. */
    if atomic_inc_return(&mut (*ipc).d0ix_disable_depth) == 1 {
        cancel_delayed_work_sync(&mut (*ipc).d0ix_work);
        return avs_dsp_set_d0ix(adev, false);
    }

    0
}

pub unsafe fn avs_dsp_enable_d0ix(adev: *mut avs_dev) -> i32 {
    let ipc: *mut avs_ipc = (*adev).ipc;

    if atomic_dec_and_test(&mut (*ipc).d0ix_disable_depth) {
        queue_delayed_work(
            system_power_efficient_wq,
            &mut (*ipc).d0ix_work,
            msecs_to_jiffies(AVS_D0IX_DELAY_MS),
        );
    }
    0
}

unsafe fn avs_dsp_recovery(adev: *mut avs_dev) {
    let mut acomp: *mut avs_soc_component;
    let core_mask: u32;
    let ret: i32;

    mutex_lock(&mut (*adev).comp_list_mutex);
    /* disconnect all running streams */
    list_for_each_entry!(acomp, &mut (*adev).comp_list, node, {
        let mut rtd: *mut snd_soc_pcm_runtime;
        let card: *mut snd_soc_card;

        card = (*(*acomp).base).card;
        if card.is_null() {
            continue;
        }

        for_each_card_rtds!(card, rtd, {
            let pcm: *mut snd_pcm;
            let mut dir: i32;

            pcm = (*rtd).pcm;
            if pcm.is_null() || (*(*rtd).dai_link).no_pcm {
                continue;
            }

            for_each_pcm_streams!(dir, {
                let substream: *mut snd_pcm_substream;

                substream = (*pcm).streams[dir as usize].substream;
                if substream.is_null() || (*substream).runtime.is_null() {
                    continue;
                }

                /* No need for _irq() as we are in nonatomic context. */
                snd_pcm_stream_lock(substream);
                snd_pcm_stop(substream, SNDRV_PCM_STATE_DISCONNECTED);
                snd_pcm_stream_unlock(substream);
            });
        });
    });
    mutex_unlock(&mut (*adev).comp_list_mutex);

    /* forcibly shutdown all cores */
    core_mask = GENMASK((*adev).hw_cfg.dsp_cores - 1, 0);
    avs_dsp_core_disable(adev, core_mask);

    /* attempt dsp reboot */
    ret = avs_dsp_boot_firmware(adev, true);
    if ret < 0 {
        dev_err((*adev).dev, c_str!("dsp reboot failed: %d\n"), ret);
    }

    pm_runtime_enable((*adev).dev);
    pm_request_autosuspend((*adev).dev);

    atomic_set(&mut (*(*adev).ipc).recovering, 0);
}

unsafe fn avs_dsp_recovery_work(work: *mut work_struct) {
    let ipc: *mut avs_ipc = container_of!(work, avs_ipc, recovery_work);

    avs_dsp_recovery(to_avs_dev((*ipc).dev));
}

unsafe fn avs_dsp_exception_caught(adev: *mut avs_dev, msg: *mut avs_notify_msg) {
    let ipc: *mut avs_ipc = (*adev).ipc;

    /* Account for the double-exception case. */
    (*ipc).ready = false;

    if !atomic_add_unless(&mut (*ipc).recovering, 1, 1) {
        dev_err((*adev).dev, c_str!("dsp recovery is already in progress\n"));
        return;
    }

    dev_crit((*adev).dev, c_str!("communication severed, rebooting dsp..\n"));

    /* Avoid deadlock as the exception may be the response to SET_D0IX. */
    if current_work() != &mut (*ipc).d0ix_work.work as *mut work_struct {
        cancel_delayed_work_sync(&mut (*ipc).d0ix_work);
    }
    (*ipc).in_d0ix = false;
    /* Re-enabled on recovery completion. */
    pm_runtime_disable((*adev).dev);

    /* Process received notification. */
    avs_dsp_op!(adev, coredump, msg);

    schedule_work(&mut (*ipc).recovery_work);
}

unsafe fn avs_dsp_receive_rx(adev: *mut avs_dev, header: u64) {
    let ipc: *mut avs_ipc = (*adev).ipc;
    let msg: avs_reply_msg = AVS_MSG(header);
    let sts: u32;
    let lec: u32;

    sts = snd_hdac_adsp_readl(adev, AVS_FW_REG_STATUS(adev));
    lec = snd_hdac_adsp_readl(adev, AVS_FW_REG_ERROR(adev));
    trace_avs_ipc_reply_msg(header, sts, lec);

    (*ipc).rx.header = header;
    /* Abort copying payload if request processing was unsuccessful. */
    if msg.status == 0 {
        /* update size in case of LARGE_CONFIG_GET */
        if msg.msg_target == AVS_MOD_MSG && msg.global_msg_type == AVS_MOD_LARGE_CONFIG_GET {
            (*ipc).rx.size = min_t::<u32>(
                AVS_MAILBOX_SIZE,
                msg.ext.large_config.data_off_size,
            );
        }

        memcpy_fromio((*ipc).rx.data, avs_uplink_addr(adev), (*ipc).rx.size);
        trace_avs_msg_payload((*ipc).rx.data, (*ipc).rx.size);
    }
}

unsafe fn avs_dsp_process_notification(adev: *mut avs_dev, header: u64) {
    let mut mod_data: avs_notify_mod_data = core::mem::zeroed();
    let msg: avs_notify_msg = AVS_MSG(header);
    let mut data_size: usize = 0;
    let mut data: *mut core::ffi::c_void = core::ptr::null_mut();
    let sts: u32;
    let lec: u32;

    sts = snd_hdac_adsp_readl(adev, AVS_FW_REG_STATUS(adev));
    lec = snd_hdac_adsp_readl(adev, AVS_FW_REG_ERROR(adev));
    trace_avs_ipc_notify_msg(header, sts, lec);

    /* Ignore spurious notifications until handshake is established. */
    if !(*(*adev).ipc).ready && msg.notify_msg_type != AVS_NOTIFY_FW_READY {
        dev_dbg(
            (*adev).dev,
            c_str!("FW not ready, skip notification: 0x%08x\n"),
            msg.primary,
        );
        return;
    }

    /* Calculate notification payload size. */
    match msg.notify_msg_type {
        AVS_NOTIFY_FW_READY => {}

        AVS_NOTIFY_PHRASE_DETECTED => {
            data_size = core::mem::size_of::<avs_notify_voice_data>();
        }

        AVS_NOTIFY_RESOURCE_EVENT => {
            data_size = core::mem::size_of::<avs_notify_res_data>();
        }

        AVS_NOTIFY_LOG_BUFFER_STATUS | AVS_NOTIFY_EXCEPTION_CAUGHT => {}

        AVS_NOTIFY_MODULE_EVENT => {
            /* To know the total payload size, header needs to be read first. */
            memcpy_fromio(
                &mut mod_data as *mut _ as *mut core::ffi::c_void,
                avs_uplink_addr(adev),
                core::mem::size_of_val(&mod_data),
            );
            data_size = core::mem::size_of_val(&mod_data) + mod_data.data_size as usize;
        }

        _ => {
            dev_info(
                (*adev).dev,
                c_str!("unknown notification: 0x%08x\n"),
                msg.primary,
            );
        }
    }

    if data_size != 0 {
        data = kmalloc(data_size, GFP_KERNEL);
        if data.is_null() {
            return;
        }

        memcpy_fromio(data, avs_uplink_addr(adev), data_size);
        trace_avs_msg_payload(data, data_size);
    }

    /* Perform notification-specific operations. */
    match msg.notify_msg_type {
        AVS_NOTIFY_FW_READY => {
            dev_dbg((*adev).dev, c_str!("FW READY 0x%08x\n"), msg.primary);
            (*(*adev).ipc).ready = true;
            complete(&mut (*adev).fw_ready);
        }

        AVS_NOTIFY_LOG_BUFFER_STATUS => {
            avs_log_buffer_status_locked(adev, &msg);
        }

        AVS_NOTIFY_EXCEPTION_CAUGHT => {
            avs_dsp_exception_caught(adev, &msg as *const _ as *mut avs_notify_msg);
        }

        _ => {}
    }

    kfree(data);
}

pub unsafe fn avs_dsp_process_response(adev: *mut avs_dev, header: u64) {
    let ipc: *mut avs_ipc = (*adev).ipc;

    /*
     * Response may either be solicited - a reply for a request that has
     * been sent beforehand - or unsolicited (notification).
     */
    if avs_msg_is_reply(header) {
        /* Response processing is invoked from IRQ thread. */
        spin_lock_irq(&mut (*ipc).rx_lock);
        avs_dsp_receive_rx(adev, header);
        (*ipc).rx_completed = true;
        spin_unlock_irq(&mut (*ipc).rx_lock);
    } else {
        avs_dsp_process_notification(adev, header);
    }

    complete(&mut (*ipc).busy_completion);
}

unsafe fn avs_ipc_is_busy(ipc: *mut avs_ipc) -> bool {
    let adev: *mut avs_dev = to_avs_dev((*ipc).dev);
    let spec: *const avs_spec = (*adev).spec;
    let hipc_rsp: u32;

    hipc_rsp = snd_hdac_adsp_readl(adev, (*(*spec).hipc).rsp_offset);
    (hipc_rsp & (*(*spec).hipc).rsp_busy_mask) != 0
}

unsafe fn avs_ipc_wait_busy_completion(ipc: *mut avs_ipc, timeout: i32) -> i32 {
    let mut repeats_left: u32 = 128; /* to avoid infinite looping */
    let mut ret: i32;

    loop {
        ret = wait_for_completion_timeout(
            &mut (*ipc).busy_completion,
            msecs_to_jiffies(timeout),
        );

        /* DSP could be unresponsive at this point. */
        if !(*ipc).ready {
            return -EPERM;
        }

        if ret == 0 {
            if !avs_ipc_is_busy(ipc) {
                return -ETIMEDOUT;
            }
            /*
             * Firmware did its job, either notification or reply
             * has been received - now wait until it's processed.
             */
            wait_for_completion_killable(&mut (*ipc).busy_completion);
        }

        /* Ongoing notification's bottom-half may cause early wakeup */
        spin_lock(&mut (*ipc).rx_lock);
        if !(*ipc).rx_completed {
            if repeats_left != 0 {
                /* Reply delayed due to notification. */
                repeats_left -= 1;
                reinit_completion(&mut (*ipc).busy_completion);
                spin_unlock(&mut (*ipc).rx_lock);
                continue;
            }

            spin_unlock(&mut (*ipc).rx_lock);
            return -ETIMEDOUT;
        }

        spin_unlock(&mut (*ipc).rx_lock);
        return 0;
    }
}

unsafe fn avs_ipc_msg_init(ipc: *mut avs_ipc, reply: *mut avs_ipc_msg) {
    lockdep_assert_held(&mut (*ipc).rx_lock);

    (*ipc).rx.header = 0;
    (*ipc).rx.size = if !reply.is_null() { (*reply).size } else { 0 };
    (*ipc).rx_completed = false;

    reinit_completion(&mut (*ipc).done_completion);
    reinit_completion(&mut (*ipc).busy_completion);
}

unsafe fn avs_dsp_send_tx(adev: *mut avs_dev, tx: *mut avs_ipc_msg, read_fwregs: bool) {
    let spec: *const avs_spec = (*adev).spec;
    let mut sts: u32 = UINT_MAX;
    let mut lec: u32 = UINT_MAX;

    (*tx).header |= (*(*spec).hipc).req_busy_mask as u64;
    if read_fwregs {
        sts = snd_hdac_adsp_readl(adev, AVS_FW_REG_STATUS(adev));
        lec = snd_hdac_adsp_readl(adev, AVS_FW_REG_ERROR(adev));
    }

    trace_avs_request(tx, sts, lec);

    if (*tx).size != 0 {
        memcpy_toio(avs_downlink_addr(adev), (*tx).data, (*tx).size);
    }
    snd_hdac_adsp_writel(adev, (*(*spec).hipc).req_ext_offset, ((*tx).header >> 32) as u32);
    snd_hdac_adsp_writel(adev, (*(*spec).hipc).req_offset, ((*tx).header & UINT_MAX as u64) as u32);
}

unsafe fn avs_dsp_do_send_msg(
    adev: *mut avs_dev,
    request: *mut avs_ipc_msg,
    reply: *mut avs_ipc_msg,
    timeout: i32,
    name: *const core::ffi::c_char,
) -> i32 {
    let ipc: *mut avs_ipc = (*adev).ipc;
    let mut ret: i32;

    if !(*ipc).ready {
        return -EPERM;
    }

    mutex_lock(&mut (*ipc).msg_mutex);

    spin_lock(&mut (*ipc).rx_lock);
    avs_ipc_msg_init(ipc, reply);
    avs_dsp_send_tx(adev, request, true);
    spin_unlock(&mut (*ipc).rx_lock);

    ret = avs_ipc_wait_busy_completion(ipc, timeout);
    if ret != 0 {
        if ret == -ETIMEDOUT {
            let mut msg: avs_notify_msg = AVS_NOTIFICATION!(EXCEPTION_CAUGHT);

            /* Same treatment as on exception, just stack_dump=0. */
            avs_dsp_exception_caught(adev, &mut msg);
        }
        mutex_unlock(&mut (*ipc).msg_mutex);
        return ret;
    }

    ret = (*ipc).rx.rsp.status;
    /*
     * If IPC channel is blocked e.g.: due to ongoing recovery,
     * -EPERM error code is expected and thus it's not an actual error.
     *
     * Unsupported IPCs are of no harm either.
     */
    if ret == -EPERM || ret == AVS_IPC_NOT_SUPPORTED {
        dev_dbg(
            (*adev).dev,
            c_str!("%s (0x%08x 0x%08x) failed: %d\n"),
            name,
            (*request).glb.primary,
            (*request).glb.ext.val,
            ret,
        );
    } else if ret != 0 {
        dev_err(
            (*adev).dev,
            c_str!("%s (0x%08x 0x%08x) failed: %d\n"),
            name,
            (*request).glb.primary,
            (*request).glb.ext.val,
            ret,
        );
    }

    if !reply.is_null() {
        (*reply).header = (*ipc).rx.header;
        (*reply).size = (*ipc).rx.size;
        if !(*reply).data.is_null() && (*ipc).rx.size != 0 {
            memcpy((*reply).data, (*ipc).rx.data, (*reply).size);
        }
    }

    mutex_unlock(&mut (*ipc).msg_mutex);
    ret
}

unsafe fn avs_dsp_send_msg_sequence(
    adev: *mut avs_dev,
    request: *mut avs_ipc_msg,
    reply: *mut avs_ipc_msg,
    timeout: i32,
    wake_d0i0: bool,
    schedule_d0ix: bool,
    name: *const core::ffi::c_char,
) -> i32 {
    let mut ret: i32;

    trace_avs_d0ix(c_str!("wake"), wake_d0i0, (*request).header);
    if wake_d0i0 {
        ret = avs_dsp_wake_d0i0(adev, request);
        if ret != 0 {
            return ret;
        }
    }

    ret = avs_dsp_do_send_msg(adev, request, reply, timeout, name);
    if ret != 0 {
        return ret;
    }

    trace_avs_d0ix(c_str!("schedule"), schedule_d0ix, (*request).header);
    if schedule_d0ix {
        avs_dsp_schedule_d0ix(adev, request);
    }

    0
}

pub unsafe fn avs_dsp_send_msg_timeout(
    adev: *mut avs_dev,
    request: *mut avs_ipc_msg,
    reply: *mut avs_ipc_msg,
    timeout: i32,
    name: *const core::ffi::c_char,
) -> i32 {
    let wake_d0i0: bool = avs_dsp_op!(adev, d0ix_toggle, request, true);
    let schedule_d0ix: bool = avs_dsp_op!(adev, d0ix_toggle, request, false);

    avs_dsp_send_msg_sequence(
        adev,
        request,
        reply,
        timeout,
        wake_d0i0,
        schedule_d0ix,
        name,
    )
}

pub unsafe fn avs_dsp_send_msg(
    adev: *mut avs_dev,
    request: *mut avs_ipc_msg,
    reply: *mut avs_ipc_msg,
    name: *const core::ffi::c_char,
) -> i32 {
    avs_dsp_send_msg_timeout(adev, request, reply, (*(*adev).ipc).default_timeout_ms, name)
}

pub unsafe fn avs_dsp_send_pm_msg_timeout(
    adev: *mut avs_dev,
    request: *mut avs_ipc_msg,
    reply: *mut avs_ipc_msg,
    timeout: i32,
    wake_d0i0: bool,
    name: *const core::ffi::c_char,
) -> i32 {
    avs_dsp_send_msg_sequence(adev, request, reply, timeout, wake_d0i0, false, name)
}

pub unsafe fn avs_dsp_send_pm_msg(
    adev: *mut avs_dev,
    request: *mut avs_ipc_msg,
    reply: *mut avs_ipc_msg,
    wake_d0i0: bool,
    name: *const core::ffi::c_char,
) -> i32 {
    avs_dsp_send_pm_msg_timeout(
        adev,
        request,
        reply,
        (*(*adev).ipc).default_timeout_ms,
        wake_d0i0,
        name,
    )
}

unsafe fn avs_dsp_do_send_rom_msg(
    adev: *mut avs_dev,
    request: *mut avs_ipc_msg,
    timeout: i32,
    name: *const core::ffi::c_char,
) -> i32 {
    let ipc: *mut avs_ipc = (*adev).ipc;
    let mut ret: i32;

    mutex_lock(&mut (*ipc).msg_mutex);

    spin_lock(&mut (*ipc).rx_lock);
    avs_ipc_msg_init(ipc, core::ptr::null_mut());
    /*
     * with hw still stalled, memory windows may not be
     * configured properly so avoid accessing SRAM
     */
    avs_dsp_send_tx(adev, request, false);
    spin_unlock(&mut (*ipc).rx_lock);

    /* ROM messages must be sent before main core is unstalled */
    ret = avs_dsp_op!(adev, stall, AVS_MAIN_CORE_MASK, false);
    if ret == 0 {
        ret = wait_for_completion_timeout(
            &mut (*ipc).done_completion,
            msecs_to_jiffies(timeout),
        );
        ret = if ret != 0 { 0 } else { -ETIMEDOUT };
    }
    if ret != 0 {
        dev_err(
            (*adev).dev,
            c_str!("%s (0x%08x 0x%08x) failed: %d\n"),
            name,
            (*request).glb.primary,
            (*request).glb.ext.val,
            ret,
        );
    }

    mutex_unlock(&mut (*ipc).msg_mutex);
    ret
}

pub unsafe fn avs_dsp_send_rom_msg_timeout(
    adev: *mut avs_dev,
    request: *mut avs_ipc_msg,
    timeout: i32,
    name: *const core::ffi::c_char,
) -> i32 {
    avs_dsp_do_send_rom_msg(adev, request, timeout, name)
}

pub unsafe fn avs_dsp_send_rom_msg(
    adev: *mut avs_dev,
    request: *mut avs_ipc_msg,
    name: *const core::ffi::c_char,
) -> i32 {
    avs_dsp_send_rom_msg_timeout(adev, request, (*(*adev).ipc).default_timeout_ms, name)
}

pub unsafe fn avs_dsp_interrupt_control(adev: *mut avs_dev, enable: bool) {
    let spec: *const avs_spec = (*adev).spec;
    let mut value: u32;
    let mask: u32;

    /*
     * No particular bit setting order. All of these are required
     * to have a functional SW <-> FW communication.
     */
    value = if enable { AVS_ADSP_ADSPIC_IPC } else { 0 };
    snd_hdac_adsp_updatel(
        adev,
        AVS_ADSP_REG_ADSPIC,
        AVS_ADSP_ADSPIC_IPC,
        value,
    );

    mask = AVS_ADSP_HIPCCTL_DONE | AVS_ADSP_HIPCCTL_BUSY;
    value = if enable { mask } else { 0 };
    snd_hdac_adsp_updatel(adev, (*(*spec).hipc).ctl_offset, mask, value);
}

pub unsafe fn avs_ipc_init(ipc: *mut avs_ipc, dev: *mut device) -> i32 {
    (*ipc).rx.data = devm_kzalloc(dev, AVS_MAILBOX_SIZE as usize, GFP_KERNEL);
    if (*ipc).rx.data.is_null() {
        return -ENOMEM;
    }

    (*ipc).dev = dev;
    (*ipc).ready = false;
    (*ipc).default_timeout_ms = AVS_IPC_TIMEOUT_MS;
    INIT_WORK(&mut (*ipc).recovery_work, avs_dsp_recovery_work);
    INIT_DELAYED_WORK(&mut (*ipc).d0ix_work, avs_dsp_d0ix_work);
    init_completion(&mut (*ipc).done_completion);
    init_completion(&mut (*ipc).busy_completion);
    spin_lock_init(&mut (*ipc).rx_lock);
    mutex_init(&mut (*ipc).msg_mutex);

    0
}

pub unsafe fn avs_ipc_block(ipc: *mut avs_ipc) {
    (*ipc).ready = false;
    cancel_work_sync(&mut (*ipc).recovery_work);
    cancel_delayed_work_sync(&mut (*ipc).d0ix_work);
    (*ipc).in_d0ix = false;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
