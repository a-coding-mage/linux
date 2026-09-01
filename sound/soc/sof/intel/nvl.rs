// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright(c) 2025 Intel Corporation

/*
 * Hardware interface for audio DSP on NovaLake.
 */

// C dependencies:
// sound/hda_register.h
// sound/hda-mlink.h
// sound/sof/ipc4/header.h
// ../ipc4-priv.h
// ../ops.h
// hda.h
// hda-ipc.h
// ../sof-audio.h
// mtl.h
// lnl.h
// ptl.h
// nvl.h

extern "C" {
    fn sof_ptl_set_ops(
        sdev: *mut snd_sof_dev,
        dsp_ops: *mut snd_sof_dsp_ops,
    ) -> ::core::ffi::c_int;

    fn hda_sdw_check_lcount_ext();
    fn lnl_dsp_check_sdw_irq();
    fn lnl_sdw_check_wakeen_irq();
    fn hda_sdw_process_wakeen_common();
    fn mtl_dsp_check_ipc_irq();
    fn mtl_dsp_cl_init();
    fn mtl_power_down_dsp();
    fn lnl_dsp_disable_interrupts();
}

#[repr(C)]
pub struct snd_sof_dev {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub cores_num: ::core::ffi::c_int,
    pub init_core_mask: ::core::ffi::c_uint,
    pub host_managed_cores_mask: ::core::ffi::c_uint,
    pub ipc_req: ::core::ffi::c_uint,
    pub ipc_req_mask: ::core::ffi::c_uint,
    pub ipc_ack: ::core::ffi::c_uint,
    pub ipc_ack_mask: ::core::ffi::c_uint,
    pub ipc_ctl: ::core::ffi::c_uint,
    pub rom_status_reg: ::core::ffi::c_uint,
    pub rom_init_timeout: ::core::ffi::c_int,
    pub ssp_count: ::core::ffi::c_int,
    pub d0i3_offset: ::core::ffi::c_uint,
    pub read_sdw_lcount: unsafe extern "C" fn(),
    pub check_sdw_irq: unsafe extern "C" fn(),
    pub check_sdw_wakeen_irq: unsafe extern "C" fn(),
    pub sdw_process_wakeen: unsafe extern "C" fn(),
    pub check_ipc_irq: unsafe extern "C" fn(),
    pub cl_init: unsafe extern "C" fn(),
    pub power_down_dsp: unsafe extern "C" fn(),
    pub disable_interrupts: unsafe extern "C" fn(),
    pub hw_ip_version: ::core::ffi::c_int,
    pub platform: *const ::core::ffi::c_char,
}

unsafe impl Sync for sof_intel_dsp_desc {}

const fn BIT(nr: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    1u32 << nr
}

#[no_mangle]
pub unsafe extern "C" fn sof_nvl_set_ops(
    sdev: *mut snd_sof_dev,
    dsp_ops: *mut snd_sof_dsp_ops,
) -> ::core::ffi::c_int {
    /* Use PTL ops for NVL */
    unsafe { sof_ptl_set_ops(sdev, dsp_ops) }
}

// EXPORT_SYMBOL_NS(sof_nvl_set_ops, "SND_SOC_SOF_INTEL_NVL");

#[no_mangle]
pub static nvl_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    cores_num: 4,
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
    read_sdw_lcount: hda_sdw_check_lcount_ext,
    check_sdw_irq: lnl_dsp_check_sdw_irq,
    check_sdw_wakeen_irq: lnl_sdw_check_wakeen_irq,
    sdw_process_wakeen: hda_sdw_process_wakeen_common,
    check_ipc_irq: mtl_dsp_check_ipc_irq,
    cl_init: mtl_dsp_cl_init,
    power_down_dsp: mtl_power_down_dsp,
    disable_interrupts: lnl_dsp_disable_interrupts,
    hw_ip_version: SOF_INTEL_ACE_4_0,
    platform: b"nvl\0".as_ptr() as *const ::core::ffi::c_char,
};

#[no_mangle]
pub static nvl_s_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    cores_num: 2,
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
    read_sdw_lcount: hda_sdw_check_lcount_ext,
    check_sdw_irq: lnl_dsp_check_sdw_irq,
    check_sdw_wakeen_irq: lnl_sdw_check_wakeen_irq,
    sdw_process_wakeen: hda_sdw_process_wakeen_common,
    check_ipc_irq: mtl_dsp_check_ipc_irq,
    cl_init: mtl_dsp_cl_init,
    power_down_dsp: mtl_power_down_dsp,
    disable_interrupts: lnl_dsp_disable_interrupts,
    hw_ip_version: SOF_INTEL_ACE_4_0,
    platform: b"nvl\0".as_ptr() as *const ::core::ffi::c_char,
};

// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_MTL");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_LNL");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_PTL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
