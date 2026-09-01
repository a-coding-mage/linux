// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright(c) 2021-2025 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

use core::ffi::{c_char, c_int, c_void};

#[allow(non_camel_case_types)]
pub type u32 = core::ffi::c_uint;
#[allow(non_camel_case_types)]
pub type irqreturn_t = c_int;

const fn bit(n: u32) -> u32 {
    1u32 << n
}

const fn genmask(high: u32, low: u32) -> u32 {
    u32::MAX.wrapping_shl(low) & u32::MAX.wrapping_shr(31 - high)
}

const UINT_MAX: u32 = u32::MAX;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;

const MTL_HfDSSGBL_BASE: u32 = 0x1000;
const MTL_REG_HfDSSCS: u32 = MTL_HfDSSGBL_BASE + 0x0;
const MTL_HfDSSCS_SPA: u32 = bit(16);
const MTL_HfDSSCS_CPA: u32 = bit(24);

const MTL_DSPCS_BASE: u32 = 0x178D00;
const MTL_REG_DSPCCTL: u32 = MTL_DSPCS_BASE + 0x4;
const MTL_DSPCCTL_SPA: u32 = bit(0);
const MTL_DSPCCTL_CPA: u32 = bit(8);
const MTL_DSPCCTL_OSEL: u32 = genmask(25, 24);
const MTL_DSPCCTL_OSEL_HOST: u32 = bit(25);

const MTL_HfINT_BASE: u32 = 0x1100;
const MTL_REG_HfINTIPPTR: u32 = MTL_HfINT_BASE + 0x8;
const MTL_REG_HfHIPCIE: u32 = MTL_HfINT_BASE + 0x40;
const MTL_HfINTIPPTR_PTR: u32 = genmask(20, 0);
const MTL_HfHIPCIE_IE: u32 = bit(0);

const MTL_DWICTL_INTENL_IE: u32 = bit(0);
const MTL_DWICTL_FINALSTATUSL_IPC: u32 = bit(0); /* same as ADSPIS_IPC */

extern "C" {
    static AVS_MAIN_CORE_MASK: u32;
    static AVS_ADSPCS_INTERVAL_US: c_int;
    static AVS_ADSPCS_TIMEOUT_US: c_int;

    static MTL_REG_HfPWRCTL: u32;
    static MTL_HfPWRCTL_WPDSPHPxPG: u32;
    static MTL_REG_HfPWRSTS: u32;
    static MTL_HfPWRSTS_DSPHPxPGS: u32;
    static MTL_REG_HfIPCxTDR: u32;
    static MTL_REG_HfIPCxTDD: u32;
    static MTL_REG_HfIPCxTDA: u32;
    static MTL_REG_HfIPCxCTL: u32;
    static MTL_HfIPCxTDR_BUSY: u32;
    static MTL_HfIPCxTDA_BUSY: u32;
    static MTL_DWICTL_REG_FINALSTATUSL: u32;
    static MTL_DWICTL_REG_INTENL: u32;
    static AVS_ADSP_HIPCCTL_DONE: u32;
    static AVS_ADSP_HIPCCTL_BUSY: u32;

    fn snd_hdac_adsp_updatel(adev: *mut avs_dev, reg: u32, mask: u32, value: u32);
    fn snd_hdac_adsp_updatew(adev: *mut avs_dev, reg: u32, mask: u32, value: u32);
    fn snd_hdac_adsp_readl(adev: *mut avs_dev, reg: u32) -> u32;
    fn trace_avs_dsp_core_op(value: u32, core_mask: u32, op: *const c_char, flag: bool);
    fn complete(completion: *mut completion);
    fn avs_dsp_process_response(adev: *mut avs_dev, msg: u64);
}

#[repr(C)]
pub struct avs_dev {
    pub dev: *mut c_void,
    pub spec: *const avs_spec,
    pub ipc: *mut avs_ipc,
}

#[repr(C)]
pub struct avs_spec {
    pub hipc: *const avs_hipc,
}

#[repr(C)]
pub struct avs_hipc {
    pub ctl_offset: u32,
    pub ack_offset: u32,
    pub rsp_offset: u32,
    pub ack_done_mask: u32,
    pub rsp_busy_mask: u32,
}

#[repr(C)]
pub struct avs_ipc {
    pub done_completion: completion,
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_reply_msg_ext {
    pub val: u32,
}

#[repr(C)]
pub union avs_reply_msg {
    pub val: u64,
    pub primary: u32,
    pub ext: avs_reply_msg_ext,
}

unsafe fn avs_mtl_core_power_on(adev: *mut avs_dev) -> c_int {
    let mut reg: u32;
    let mut ret: c_int;

    /* Power up DSP domain. */
    snd_hdac_adsp_updatel(adev, MTL_REG_HfDSSCS, MTL_HfDSSCS_SPA, MTL_HfDSSCS_SPA);
    trace_avs_dsp_core_op(1, AVS_MAIN_CORE_MASK, b"power dsp\0".as_ptr() as *const c_char, true);

    ret = snd_hdac_adsp_readl_poll!(
        adev,
        MTL_REG_HfDSSCS,
        reg,
        (reg & MTL_HfDSSCS_CPA) == MTL_HfDSSCS_CPA,
        AVS_ADSPCS_INTERVAL_US,
        AVS_ADSPCS_TIMEOUT_US
    );
    if ret != 0 {
        dev_err!((*adev).dev, "power on domain dsp failed: %d\n", ret);
        return ret;
    }

    /* Prevent power gating of DSP domain. */
    snd_hdac_adsp_updatel(
        adev,
        MTL_REG_HfPWRCTL,
        MTL_HfPWRCTL_WPDSPHPxPG,
        MTL_HfPWRCTL_WPDSPHPxPG,
    );
    trace_avs_dsp_core_op(
        1,
        AVS_MAIN_CORE_MASK,
        b"prevent dsp PG\0".as_ptr() as *const c_char,
        true,
    );

    ret = snd_hdac_adsp_readl_poll!(
        adev,
        MTL_REG_HfPWRSTS,
        reg,
        (reg & MTL_HfPWRSTS_DSPHPxPGS) == MTL_HfPWRSTS_DSPHPxPGS,
        AVS_ADSPCS_INTERVAL_US,
        AVS_ADSPCS_TIMEOUT_US
    );

    /* Set ownership to HOST. */
    snd_hdac_adsp_updatel(adev, MTL_REG_DSPCCTL, MTL_DSPCCTL_OSEL, MTL_DSPCCTL_OSEL_HOST);
    ret
}

unsafe fn avs_mtl_core_power_off(adev: *mut avs_dev) -> c_int {
    let mut reg: u32;

    /* Allow power gating of DSP domain. No STS polling as HOST is only one of its users. */
    snd_hdac_adsp_updatel(adev, MTL_REG_HfPWRCTL, MTL_HfPWRCTL_WPDSPHPxPG, 0);
    trace_avs_dsp_core_op(0, AVS_MAIN_CORE_MASK, b"allow dsp pg\0".as_ptr() as *const c_char, false);

    /* Power down DSP domain. */
    snd_hdac_adsp_updatel(adev, MTL_REG_HfDSSCS, MTL_HfDSSCS_SPA, 0);
    trace_avs_dsp_core_op(0, AVS_MAIN_CORE_MASK, b"power dsp\0".as_ptr() as *const c_char, false);

    snd_hdac_adsp_readl_poll!(
        adev,
        MTL_REG_HfDSSCS,
        reg,
        (reg & MTL_HfDSSCS_CPA) == 0,
        AVS_ADSPCS_INTERVAL_US,
        AVS_ADSPCS_TIMEOUT_US
    )
}

#[no_mangle]
pub unsafe extern "C" fn avs_mtl_core_power(
    adev: *mut avs_dev,
    mut core_mask: u32,
    power: bool,
) -> c_int {
    core_mask &= AVS_MAIN_CORE_MASK;
    if core_mask == 0 {
        return 0;
    }

    if power {
        return avs_mtl_core_power_on(adev);
    }
    avs_mtl_core_power_off(adev)
}

#[no_mangle]
pub unsafe extern "C" fn avs_mtl_core_reset(
    _adev: *mut avs_dev,
    _core_mask: u32,
    _power: bool,
) -> c_int {
    /* No logical equivalent on ACE 1.x. */
    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_mtl_core_stall(
    adev: *mut avs_dev,
    mut core_mask: u32,
    stall: bool,
) -> c_int {
    let mut value: u32;
    let mut reg: u32;
    let ret: c_int;

    core_mask &= AVS_MAIN_CORE_MASK;
    if core_mask == 0 {
        return 0;
    }

    value = snd_hdac_adsp_readl(adev, MTL_REG_DSPCCTL);
    trace_avs_dsp_core_op(value, core_mask, b"stall\0".as_ptr() as *const c_char, stall);
    if value == UINT_MAX {
        return 0;
    }

    value = if stall { 0 } else { MTL_DSPCCTL_SPA };
    snd_hdac_adsp_updatel(adev, MTL_REG_DSPCCTL, MTL_DSPCCTL_SPA, value);

    value = if stall { 0 } else { MTL_DSPCCTL_CPA };
    ret = snd_hdac_adsp_readl_poll!(
        adev,
        MTL_REG_DSPCCTL,
        reg,
        (reg & MTL_DSPCCTL_CPA) == value,
        AVS_ADSPCS_INTERVAL_US,
        AVS_ADSPCS_TIMEOUT_US
    );
    if ret != 0 {
        dev_err!(
            (*adev).dev,
            "core_mask %d %sstall failed: %d\n",
            core_mask,
            if stall { b"\0".as_ptr() } else { b"un\0".as_ptr() },
            ret
        );
    }
    ret
}

unsafe fn avs_mtl_ipc_interrupt(adev: *mut avs_dev) {
    let spec: *const avs_spec = (*adev).spec;
    let hipc_ack: u32;
    let hipc_rsp: u32;

    snd_hdac_adsp_updatel(
        adev,
        (*(*spec).hipc).ctl_offset,
        AVS_ADSP_HIPCCTL_DONE | AVS_ADSP_HIPCCTL_BUSY,
        0,
    );

    hipc_ack = snd_hdac_adsp_readl(adev, (*(*spec).hipc).ack_offset);
    hipc_rsp = snd_hdac_adsp_readl(adev, (*(*spec).hipc).rsp_offset);

    /* DSP acked host's request. */
    if (hipc_ack & (*(*spec).hipc).ack_done_mask) != 0 {
        complete(&mut (*(*adev).ipc).done_completion);

        /* Tell DSP it has our attention. */
        snd_hdac_adsp_updatel(
            adev,
            (*(*spec).hipc).ack_offset,
            (*(*spec).hipc).ack_done_mask,
            (*(*spec).hipc).ack_done_mask,
        );
    }

    /* DSP sent new response to process. */
    if (hipc_rsp & (*(*spec).hipc).rsp_busy_mask) != 0 {
        let mut msg = avs_reply_msg { val: 0 };

        msg.primary = snd_hdac_adsp_readl(adev, MTL_REG_HfIPCxTDR);
        msg.ext.val = snd_hdac_adsp_readl(adev, MTL_REG_HfIPCxTDD);

        avs_dsp_process_response(adev, msg.val);

        /* Tell DSP we accepted its message. */
        snd_hdac_adsp_updatel(adev, MTL_REG_HfIPCxTDR, MTL_HfIPCxTDR_BUSY, MTL_HfIPCxTDR_BUSY);
        /* Ack this response. */
        snd_hdac_adsp_updatel(adev, MTL_REG_HfIPCxTDA, MTL_HfIPCxTDA_BUSY, 0);
    }

    snd_hdac_adsp_updatel(
        adev,
        (*(*spec).hipc).ctl_offset,
        AVS_ADSP_HIPCCTL_DONE | AVS_ADSP_HIPCCTL_BUSY,
        AVS_ADSP_HIPCCTL_DONE | AVS_ADSP_HIPCCTL_BUSY,
    );
}

#[no_mangle]
pub unsafe extern "C" fn avs_mtl_dsp_interrupt(adev: *mut avs_dev) -> irqreturn_t {
    let adspis: u32 = snd_hdac_adsp_readl(adev, MTL_DWICTL_REG_FINALSTATUSL);
    let mut ret: irqreturn_t = IRQ_NONE;

    if adspis == UINT_MAX {
        return ret;
    }

    if (adspis & MTL_DWICTL_FINALSTATUSL_IPC) != 0 {
        avs_mtl_ipc_interrupt(adev);
        ret = IRQ_HANDLED;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn avs_mtl_interrupt_control(adev: *mut avs_dev, enable: bool) {
    if enable {
        snd_hdac_adsp_updatel(
            adev,
            MTL_DWICTL_REG_INTENL,
            MTL_DWICTL_INTENL_IE,
            MTL_DWICTL_INTENL_IE,
        );
        snd_hdac_adsp_updatew(adev, MTL_REG_HfHIPCIE, MTL_HfHIPCIE_IE, MTL_HfHIPCIE_IE);
        snd_hdac_adsp_updatel(
            adev,
            MTL_REG_HfIPCxCTL,
            AVS_ADSP_HIPCCTL_DONE,
            AVS_ADSP_HIPCCTL_DONE,
        );
        snd_hdac_adsp_updatel(
            adev,
            MTL_REG_HfIPCxCTL,
            AVS_ADSP_HIPCCTL_BUSY,
            AVS_ADSP_HIPCCTL_BUSY,
        );
    } else {
        snd_hdac_adsp_updatel(adev, MTL_REG_HfIPCxCTL, AVS_ADSP_HIPCCTL_BUSY, 0);
        snd_hdac_adsp_updatel(adev, MTL_REG_HfIPCxCTL, AVS_ADSP_HIPCCTL_DONE, 0);
        snd_hdac_adsp_updatew(adev, MTL_REG_HfHIPCIE, MTL_HfHIPCIE_IE, 0);
        snd_hdac_adsp_updatel(adev, MTL_DWICTL_REG_INTENL, MTL_DWICTL_INTENL_IE, 0);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
