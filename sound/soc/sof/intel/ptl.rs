// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright(c) 2025 Intel Corporation

/*
 * Hardware interface for audio DSP on PantherLake.
 */

// C includes translated as external dependencies:
// <sound/hda_register.h>, <sound/hda-mlink.h>, <sound/sof/ipc4/header.h>,
// "../ipc4-priv.h", "../ops.h", "hda.h", "hda-ipc.h", "../sof-audio.h",
// "mtl.h", "lnl.h", "ptl.h"

type bool_ = bool;
type u32_ = u32;

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
}

#[repr(C)]
pub struct hda_bus_wrapper {
    pub core: hdac_bus,
}

#[repr(C)]
pub struct sof_intel_hda_mic_privacy {
    pub work: work_struct,
    pub active: bool_,
}

#[repr(C)]
pub struct sof_intel_hda_dev {
    pub hbus: hda_bus_wrapper,
    pub mic_privacy: sof_intel_hda_mic_privacy,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut sof_intel_hda_dev,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub pdata: *mut snd_sof_pdata,
    pub private: *mut sof_ipc4_fw_data,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc4_intel_mic_privacy_cap {
    pub capabilities_length: u32_,
    pub capabilities: [u32_; 0],
}

#[repr(C)]
pub struct sof_ipc4_fw_data {
    pub intel_configure_mic_privacy:
        Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut sof_ipc4_intel_mic_privacy_cap)>,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub cores_num: i32,
    pub init_core_mask: u32_,
    pub host_managed_cores_mask: u32_,
    pub ipc_req: u32_,
    pub ipc_req_mask: u32_,
    pub ipc_ack: u32_,
    pub ipc_ack_mask: u32_,
    pub ipc_ctl: u32_,
    pub rom_status_reg: u32_,
    pub rom_init_timeout: i32,
    pub ssp_count: i32,
    pub d0i3_offset: u32_,
    pub read_sdw_lcount: Option<unsafe extern "C" fn()>,
    pub check_sdw_irq: Option<unsafe extern "C" fn()>,
    pub check_sdw_wakeen_irq: Option<unsafe extern "C" fn()>,
    pub sdw_process_wakeen: Option<unsafe extern "C" fn()>,
    pub check_ipc_irq: Option<unsafe extern "C" fn()>,
    pub check_mic_privacy_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev, bool_, i32) -> bool_>,
    pub process_mic_privacy: Option<unsafe extern "C" fn(*mut snd_sof_dev, bool_, i32)>,
    pub cl_init: Option<unsafe extern "C" fn()>,
    pub power_down_dsp: Option<unsafe extern "C" fn()>,
    pub disable_interrupts: Option<unsafe extern "C" fn()>,
    pub hw_ip_version: u32_,
    pub platform: *const core::ffi::c_char,
}

unsafe extern "C" {
    static AZX_REG_ML_LEPTR_ID_SDW: i32;
    static PTL_MICPVCP_DDZE_ENABLED: u32_;
    static PTL_MICPVCP_DDZE_FORCED: u32_;
    static MTL_DSP_REG_HFIPCXIDR: u32_;
    static MTL_DSP_REG_HFIPCXIDR_BUSY: u32_;
    static MTL_DSP_REG_HFIPCXIDA: u32_;
    static MTL_DSP_REG_HFIPCXIDA_DONE: u32_;
    static MTL_DSP_REG_HFIPCXCTL: u32_;
    static LNL_DSP_REG_HFDSC: u32_;
    static MTL_SSP_COUNT: i32;
    static MTL_HDA_VS_D0I3C: u32_;
    static SOF_INTEL_ACE_3_0: u32_;

    static hda_sdw_check_lcount_ext: unsafe extern "C" fn();
    static lnl_dsp_check_sdw_irq: unsafe extern "C" fn();
    static lnl_sdw_check_wakeen_irq: unsafe extern "C" fn();
    static hda_sdw_process_wakeen_common: unsafe extern "C" fn();
    static mtl_dsp_check_ipc_irq: unsafe extern "C" fn();
    static mtl_dsp_cl_init: unsafe extern "C" fn();
    static mtl_power_down_dsp: unsafe extern "C" fn();
    static lnl_dsp_disable_interrupts: unsafe extern "C" fn();

    fn hdac_bus_eml_is_mic_privacy_changed(bus: *mut hdac_bus, alt: bool_, elid: i32) -> bool_;
    fn hdac_bus_eml_get_mic_privacy_state(bus: *mut hdac_bus, alt: i32, elid: i32) -> bool_;
    fn hdac_bus_eml_set_mic_privacy_mask(bus: *mut hdac_bus, alt: bool_, elid: i32, mask: u32_);
    fn sof_to_bus(sdev: *mut snd_sof_dev) -> *mut hdac_bus;
    fn dev_get_drvdata(dev: *mut device) -> *mut snd_sof_dev;
    fn sof_ipc4_mic_privacy_state_change(sdev: *mut snd_sof_dev, state: bool_);
    fn schedule_work(work: *mut work_struct);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn sof_lnl_set_ops(sdev: *mut snd_sof_dev, dsp_ops: *mut snd_sof_dsp_ops) -> i32;
    fn PTL_MICPVCP_GET_SDW_MASK(micpvcp: u32_) -> u32_;
}

const fn BIT(n: u32) -> u32_ {
    1u32 << n
}

unsafe extern "C" fn sof_ptl_check_mic_privacy_irq(
    sdev: *mut snd_sof_dev,
    alt: bool_,
    elid: i32,
) -> bool_ {
    if !alt || elid != AZX_REG_ML_LEPTR_ID_SDW {
        return false;
    }

    hdac_bus_eml_is_mic_privacy_changed(sof_to_bus(sdev), alt, elid)
}

unsafe extern "C" fn sof_ptl_mic_privacy_work(work: *mut work_struct) {
    let hdev: *mut sof_intel_hda_dev = container_of_mic_privacy_work(work);
    let bus: *mut hdac_bus = &mut (*hdev).hbus.core;
    let sdev: *mut snd_sof_dev = dev_get_drvdata((*bus).dev);
    let state: bool_;

    /*
     * The microphone privacy state is only available via Soundwire shim
     * in PTL
     * The work is only scheduled on change.
     */
    state = hdac_bus_eml_get_mic_privacy_state(bus, 1, AZX_REG_ML_LEPTR_ID_SDW);
    sof_ipc4_mic_privacy_state_change(sdev, state);
}

unsafe fn container_of_mic_privacy_work(work: *mut work_struct) -> *mut sof_intel_hda_dev {
    let uninit = core::mem::MaybeUninit::<sof_intel_hda_dev>::uninit();
    let base = uninit.as_ptr();
    let offset = (&raw const (*base).mic_privacy.work as usize).wrapping_sub(base as usize);
    (work as *mut u8).wrapping_sub(offset) as *mut sof_intel_hda_dev
}

unsafe extern "C" fn sof_ptl_process_mic_privacy(
    sdev: *mut snd_sof_dev,
    alt: bool_,
    elid: i32,
) {
    let hdev: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata;

    if !alt || elid != AZX_REG_ML_LEPTR_ID_SDW {
        return;
    }

    /*
     * Schedule the work to read the microphone privacy state and send IPC
     * message about the new state to the firmware
     */
    schedule_work(&mut (*hdev).mic_privacy.work);
}

unsafe extern "C" fn sof_ptl_set_mic_privacy(
    sdev: *mut snd_sof_dev,
    caps: *mut sof_ipc4_intel_mic_privacy_cap,
) {
    let hdev: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata;
    let micpvcp: u32_;

    if caps.is_null() || (*caps).capabilities_length == 0 {
        return;
    }

    micpvcp = (*caps).capabilities.as_ptr().read();

    /* No need to set the mic privacy if it is not enabled or forced */
    if (micpvcp & PTL_MICPVCP_DDZE_ENABLED) == 0 || (micpvcp & PTL_MICPVCP_DDZE_FORCED) != 0 {
        return;
    }

    hdac_bus_eml_set_mic_privacy_mask(
        sof_to_bus(sdev),
        true,
        AZX_REG_ML_LEPTR_ID_SDW,
        PTL_MICPVCP_GET_SDW_MASK(micpvcp),
    );

    INIT_WORK(&mut (*hdev).mic_privacy.work, sof_ptl_mic_privacy_work);
    (*hdev).mic_privacy.active = true;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_ptl_set_ops(
    sdev: *mut snd_sof_dev,
    dsp_ops: *mut snd_sof_dsp_ops,
) -> i32 {
    let ipc4_data: *mut sof_ipc4_fw_data;
    let ret: i32;

    ret = sof_lnl_set_ops(sdev, dsp_ops);
    if ret != 0 {
        return ret;
    }

    ipc4_data = (*sdev).private;
    (*ipc4_data).intel_configure_mic_privacy = Some(sof_ptl_set_mic_privacy);

    0
}
// EXPORT_SYMBOL_NS(sof_ptl_set_ops, "SND_SOC_SOF_INTEL_PTL");

#[unsafe(no_mangle)]
pub static ptl_chip_info: sof_intel_dsp_desc = unsafe {
    sof_intel_dsp_desc {
        cores_num: 5,
        init_core_mask: BIT(0),
        host_managed_cores_mask: BIT(0),
        ipc_req: MTL_DSP_REG_HFIPCXIDR,
        ipc_req_mask: MTL_DSP_REG_HFIPCXIDR_BUSY,
        ipc_ack: MTL_DSP_REG_HFIPCXIDA,
        ipc_ack_mask: MTL_DSP_REG_HFIPCXIDA_DONE,
        ipc_ctl: MTL_DSP_REG_HFIPCXCTL,
        rom_status_reg: LNL_DSP_REG_HFDSC,
        rom_init_timeout: 300,
        ssp_count: MTL_SSP_COUNT,
        d0i3_offset: MTL_HDA_VS_D0I3C,
        read_sdw_lcount: Some(hda_sdw_check_lcount_ext),
        check_sdw_irq: Some(lnl_dsp_check_sdw_irq),
        check_sdw_wakeen_irq: Some(lnl_sdw_check_wakeen_irq),
        sdw_process_wakeen: Some(hda_sdw_process_wakeen_common),
        check_ipc_irq: Some(mtl_dsp_check_ipc_irq),
        check_mic_privacy_irq: Some(sof_ptl_check_mic_privacy_irq),
        process_mic_privacy: Some(sof_ptl_process_mic_privacy),
        cl_init: Some(mtl_dsp_cl_init),
        power_down_dsp: Some(mtl_power_down_dsp),
        disable_interrupts: Some(lnl_dsp_disable_interrupts),
        hw_ip_version: SOF_INTEL_ACE_3_0,
        platform: c"ptl".as_ptr(),
    }
};

#[unsafe(no_mangle)]
pub static wcl_chip_info: sof_intel_dsp_desc = unsafe {
    sof_intel_dsp_desc {
        cores_num: 3,
        init_core_mask: BIT(0),
        host_managed_cores_mask: BIT(0),
        ipc_req: MTL_DSP_REG_HFIPCXIDR,
        ipc_req_mask: MTL_DSP_REG_HFIPCXIDR_BUSY,
        ipc_ack: MTL_DSP_REG_HFIPCXIDA,
        ipc_ack_mask: MTL_DSP_REG_HFIPCXIDA_DONE,
        ipc_ctl: MTL_DSP_REG_HFIPCXCTL,
        rom_status_reg: LNL_DSP_REG_HFDSC,
        rom_init_timeout: 300,
        ssp_count: MTL_SSP_COUNT,
        d0i3_offset: MTL_HDA_VS_D0I3C,
        read_sdw_lcount: Some(hda_sdw_check_lcount_ext),
        check_sdw_irq: Some(lnl_dsp_check_sdw_irq),
        check_sdw_wakeen_irq: Some(lnl_sdw_check_wakeen_irq),
        sdw_process_wakeen: Some(hda_sdw_process_wakeen_common),
        check_ipc_irq: Some(mtl_dsp_check_ipc_irq),
        check_mic_privacy_irq: None,
        process_mic_privacy: None,
        cl_init: Some(mtl_dsp_cl_init),
        power_down_dsp: Some(mtl_power_down_dsp),
        disable_interrupts: Some(lnl_dsp_disable_interrupts),
        hw_ip_version: SOF_INTEL_ACE_3_0,
        platform: c"wcl".as_ptr(),
    }
};

// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_MTL");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_LNL");
// MODULE_IMPORT_NS("SND_SOC_SOF_HDA_MLINK");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
