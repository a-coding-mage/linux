// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2024 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

// C dependencies:
// <sound/hdaudio_ext.h>
// "avs.h"
// "debug.h"
// "messages.h"
// "registers.h"

unsafe fn avs_cnl_ipc_interrupt(adev: *mut avs_dev) {
    let spec: *const avs_spec = unsafe { (*adev).spec };
    let mut hipctda: u32;

    unsafe {
        snd_hdac_adsp_updatel(
            adev,
            (*(*spec).hipc).ctl_offset,
            AVS_ADSP_HIPCCTL_DONE | AVS_ADSP_HIPCCTL_BUSY,
            0,
        );
    }

    let hipc_ack: u32 = unsafe { snd_hdac_adsp_readl(adev, (*(*spec).hipc).ack_offset) };
    let hipc_rsp: u32 = unsafe { snd_hdac_adsp_readl(adev, (*(*spec).hipc).rsp_offset) };

    /* DSP acked host's request. */
    if (hipc_ack & unsafe { (*(*spec).hipc).ack_done_mask }) != 0 {
        unsafe {
            complete(&mut (*(*adev).ipc).done_completion);
        }

        /* Tell DSP it has our attention. */
        unsafe {
            snd_hdac_adsp_updatel(
                adev,
                (*(*spec).hipc).ack_offset,
                (*(*spec).hipc).ack_done_mask,
                (*(*spec).hipc).ack_done_mask,
            );
        }
    }

    /* DSP sent new response to process. */
    if (hipc_rsp & unsafe { (*(*spec).hipc).rsp_busy_mask }) != 0 {
        let mut msg: avs_reply_msg = unsafe { core::mem::zeroed() };

        unsafe {
            msg.primary = snd_hdac_adsp_readl(adev, CNL_ADSP_REG_HIPCTDR);
            msg.ext.val = snd_hdac_adsp_readl(adev, CNL_ADSP_REG_HIPCTDD);

            avs_dsp_process_response(adev, msg.val);
        }

        /* Tell DSP we accepted its message. */
        unsafe {
            snd_hdac_adsp_updatel(
                adev,
                CNL_ADSP_REG_HIPCTDR,
                CNL_ADSP_HIPCTDR_BUSY,
                CNL_ADSP_HIPCTDR_BUSY,
            );
        }
        /* Ack this response. */
        unsafe {
            snd_hdac_adsp_updatel(
                adev,
                CNL_ADSP_REG_HIPCTDA,
                CNL_ADSP_HIPCTDA_DONE,
                CNL_ADSP_HIPCTDA_DONE,
            );
        }
        /* HW might have been clock gated, give some time for change to propagate. */
        unsafe {
            snd_hdac_adsp_readl_poll(
                adev,
                CNL_ADSP_REG_HIPCTDA,
                &mut hipctda,
                |hipctda| (hipctda & CNL_ADSP_HIPCTDA_DONE) == 0,
                10,
                1000,
            );
        }
    }

    unsafe {
        snd_hdac_adsp_updatel(
            adev,
            (*(*spec).hipc).ctl_offset,
            AVS_ADSP_HIPCCTL_DONE | AVS_ADSP_HIPCCTL_BUSY,
            AVS_ADSP_HIPCCTL_DONE | AVS_ADSP_HIPCCTL_BUSY,
        );
    }
}

pub unsafe extern "C" fn avs_cnl_dsp_interrupt(adev: *mut avs_dev) -> irqreturn_t {
    let adspis: u32 = unsafe { snd_hdac_adsp_readl(adev, AVS_ADSP_REG_ADSPIS) };
    let mut ret: irqreturn_t = IRQ_NONE;

    if adspis == UINT_MAX {
        return ret;
    }

    if (adspis & AVS_ADSP_ADSPIS_IPC) != 0 {
        unsafe {
            avs_cnl_ipc_interrupt(adev);
        }
        ret = IRQ_HANDLED;
    }

    ret
}

pub static avs_cnl_dsp_ops: avs_dsp_ops = avs_dsp_ops {
    power: Some(avs_dsp_core_power),
    reset: Some(avs_dsp_core_reset),
    stall: Some(avs_dsp_core_stall),
    dsp_interrupt: Some(avs_cnl_dsp_interrupt),
    int_control: Some(avs_dsp_interrupt_control),
    load_basefw: Some(avs_hda_load_basefw),
    load_lib: Some(avs_hda_load_library),
    transfer_mods: Some(avs_hda_transfer_modules),
    log_buffer_offset: Some(avs_skl_log_buffer_offset),
    log_buffer_status: Some(avs_apl_log_buffer_status),
    coredump: Some(avs_apl_coredump),
    d0ix_toggle: Some(avs_apl_d0ix_toggle),
    set_d0ix: Some(avs_apl_set_d0ix),
    // C macro initializer: AVS_SET_ENABLE_LOGS_OP(apl)
    enable_logs: Some(avs_apl_enable_logs),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
