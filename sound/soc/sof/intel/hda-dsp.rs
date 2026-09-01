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

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type size_t = usize;
type bool_t = bool;

const EXCEPT_MAX_HDR_SIZE: u32 = 0x400;
const HDA_EXT_ROM_STATUS_SIZE: c_int = 8;

#[repr(C)]
pub struct hda_dsp_msg_code {
    pub code: u32,
    pub text: *const c_char,
}

static mut hda_enable_trace_D0I3_S0: bool = false;
/* CONFIG_SND_SOC_SOF_DEBUG: module_param_named(enable_trace_D0I3_S0, ...);
 * MODULE_PARM_DESC(enable_trace_D0I3_S0,
 * "SOF HDA enable trace when the DSP is in D0I3 in S0");
 */

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut snd_sof_pdata,
    pub dspless_mode_selected: u32,
    pub dsp_power_state: sof_dsp_power_state,
    pub fw_trace_is_supported: bool,
    pub system_suspend_target: u32,
    pub fw_state: u32,
    pub ipc_irq: c_int,
    pub dsp_core_ref_count: *mut c_int,
    pub ipc: *mut sof_ipc,
    pub dsp_oops_offset: u32,
    pub mmio_bar: u32,
    pub pcm_list: list_head,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut c_void,
    pub ipc_type: u32,
}

#[repr(C)]
pub struct sof_intel_hda_dev {
    pub desc: *const sof_intel_dsp_desc,
    pub skip_imr_boot: bool,
    pub l1_disabled: bool,
    pub d0i3_work: delayed_work,
    pub mic_privacy: hda_mic_privacy,
    pub hbus: hda_bus_wrapper,
    pub sdw: *mut sdw_intel_ctx,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub hw_ip_version: u32,
    pub host_managed_cores_mask: c_uint,
    pub ipc_ctl: u32,
    pub d0i3_offset: u32,
    pub cores_num: c_int,
    pub disable_interrupts: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub power_down_dsp: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub enable_sdw_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev, bool)>,
    pub read_sdw_lcount: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub sdw_process_wakeen: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub rom_status_reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_dsp_power_state {
    pub state: u32,
    pub substate: u32,
}

#[repr(C)]
pub struct sof_ipc_pm_ops {
    pub set_pm_gate: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
    pub set_core_state: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_int, bool) -> c_int>,
}

#[repr(C)]
pub struct sof_ipc_ops {
    pub pm: *const sof_ipc_pm_ops,
}

#[repr(C)]
pub struct sof_ipc {
    pub ops: *const sof_ipc_ops,
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
    pub codec_powered: c_uint,
    pub stream_list: list_head,
}

#[repr(C)]
pub struct hdac_stream {
    pub list: list_head,
    pub index: c_int,
}

#[repr(C)]
pub struct snd_sof_pcm {
    pub list: list_head,
    pub stream: [snd_sof_pcm_stream; 2],
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
    pub substream: *mut snd_pcm_substream,
    pub d0i3_compatible: bool,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_xtensa {
    pub arch_hdr: sof_ipc_dsp_oops_arch_hdr,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_arch_hdr {
    pub totalsize: u32,
}

#[repr(C)]
pub struct sof_ipc_panic_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub irq: c_uint,
}

#[repr(C)]
pub struct sdw_intel_ctx {
    pub shim_base: u32,
    pub count: u32,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct hda_mic_privacy {
    pub active: bool,
    pub work: work_struct,
}

#[repr(C)]
pub struct hda_bus_wrapper {
    pub core: hdac_bus,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    static KERN_DEBUG: *const c_char;
    static KERN_ERR: *const c_char;

    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_intel_dsp_desc;
    fn snd_sof_dsp_update_bits_unlocked(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u32, value: u32);
    fn snd_sof_dsp_update_bits(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u32, value: u32);
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u32;
    fn snd_sof_dsp_read8(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u8;
    fn snd_sof_dsp_update8(sdev: *mut snd_sof_dev, bar: u32, offset: u32, mask: u8, value: u8);
    fn snd_sof_pci_update_bits(sdev: *mut snd_sof_dev, offset: u32, mask: u32, value: u32);
    fn sof_ipc_get_ops_pm(sdev: *mut snd_sof_dev) -> *const sof_ipc_pm_ops;
    fn sof_to_bus(sdev: *mut snd_sof_dev) -> *mut hdac_bus;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn hda_codec_jack_wake_enable(sdev: *mut snd_sof_dev, enable: bool);
    fn hda_codec_jack_check(sdev: *mut snd_sof_dev);
    fn hda_codec_i915_display_power(sdev: *mut snd_sof_dev, enable: bool);
    fn hda_codec_resume_cmd_io(sdev: *mut snd_sof_dev);
    fn hda_codec_suspend_cmd_io(sdev: *mut snd_sof_dev);
    fn hda_bus_ml_suspend(bus: *mut hdac_bus) -> c_int;
    fn hda_bus_ml_resume(bus: *mut hdac_bus) -> c_int;
    fn hda_dsp_ctrl_ppcap_enable(sdev: *mut snd_sof_dev, enable: bool);
    fn hda_dsp_ctrl_ppcap_int_enable(sdev: *mut snd_sof_dev, enable: bool);
    fn hda_dsp_ctrl_stop_chip(sdev: *mut snd_sof_dev);
    fn hda_dsp_ctrl_link_reset(sdev: *mut snd_sof_dev, reset: bool) -> c_int;
    fn hda_dsp_ctrl_init_chip(sdev: *mut snd_sof_dev, full_reset: bool) -> c_int;
    fn synchronize_irq(irq: c_int);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn pci_restore_state(pci: *mut pci_dev);
    fn pci_save_state(pci: *mut pci_dev) -> c_int;
    fn disable_irq_wake(irq: c_uint) -> c_int;
    fn enable_irq_wake(irq: c_uint) -> c_int;
    fn snd_sof_suspend(dev: *mut device) -> c_int;
    fn snd_sof_dsp_set_power_state(sdev: *mut snd_sof_dev, target_state: *const sof_dsp_power_state) -> c_int;
    fn snd_sof_dsp_only_d0i3_compatible_stream_active(sdev: *mut snd_sof_dev) -> bool;
    fn hda_dsp_dais_suspend(sdev: *mut snd_sof_dev) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut snd_sof_dev;
    fn hdac_bus_eml_get_count(bus: *mut hdac_bus, alt: bool, id: u32) -> u32;
    fn sof_mailbox_read(sdev: *mut snd_sof_dev, offset: u32, dest: *mut c_void, bytes: size_t);
    fn sof_block_read(sdev: *mut snd_sof_dev, bar: u32, offset: u32, dest: *mut c_void, bytes: size_t);
    fn sof_print_oops_and_stack(sdev: *mut snd_sof_dev, level: *const c_char, status: u32, panic: u32, xoops: *mut sof_ipc_dsp_oops_xtensa, panic_info: *mut sof_ipc_panic_info, stack: *mut u32, stack_words: size_t);
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn dev_printk(level: *const c_char, dev: *mut device, fmt: *const c_char, ...);
    fn trace_sof_intel_D0I3C_updated(sdev: *mut snd_sof_dev, reg: u8);
}

extern "C" {
    fn snd_sof_dsp_read_poll_timeout_adspcs_eq(sdev: *mut snd_sof_dev, bar: u32, reg: u32, mask: u32, interval_us: u32, timeout_us: u32) -> c_int;
    fn snd_sof_dsp_read_poll_timeout_adspcs_clear(sdev: *mut snd_sof_dev, bar: u32, reg: u32, mask: u32, interval_us: u32, timeout_us: u32) -> c_int;
}

extern "C" {
    static HDA_DSP_BAR: u32;
    static HDA_DSP_HDA_BAR: u32;
    static HDA_DSP_REG_ADSPCS: u32;
    static HDA_DSP_REG_POLL_INTERVAL_US: u32;
    static HDA_DSP_RESET_TIMEOUT_US: u32;
    static HDA_DSP_PD_TIMEOUT: u32;
    static USEC_PER_MSEC: u32;
    static HDA_DSP_REG_HIPCCTL_DONE: u32;
    static HDA_DSP_REG_HIPCCTL_BUSY: u32;
    static HDA_DSP_REG_ADSPIC: u32;
    static HDA_DSP_ADSPIC_IPC: u32;
    static HDA_DSP_REG_POLL_RETRY_COUNT: c_int;
    static SOF_HDA_VS_D0I3C_CIP: u8;
    static SOF_HDA_VS_D0I3C_I3: u8;
    static HDA_PM_NO_DMA_TRACE: u32;
    static HDA_PM_PG_STREAMING: u32;
    static HDA_PM_PPG: u32;
    static SOF_SUSPEND_NONE: u32;
    static SOF_SUSPEND_S0IX: u32;
    static SOF_SUSPEND_S3: u32;
    static SOF_DSP_PM_D0: u32;
    static SOF_DSP_PM_D1: u32;
    static SOF_DSP_PM_D2: u32;
    static SOF_DSP_PM_D3: u32;
    static SOF_HDA_DSP_PM_D0I0: u32;
    static SOF_HDA_DSP_PM_D0I3: u32;
    static SOF_FW_CRASHED: u32;
    static SOF_FW_BOOT_FAILED: u32;
    static SOF_FW_BOOT_COMPLETE: u32;
    static SOF_DSP_PRIMARY_CORE: c_int;
    static PCI_PGCTL: u32;
    static PCI_PGCTL_LSRMD_MASK: u32;
    static PCI_TCSEL: u32;
    static HDA_VS_INTEL_EM2: u32;
    static HDA_VS_INTEL_EM2_L1SEN: u32;
    static SOF_HDA_SD_CTL_DMA_START: u32;
    static HDA_DSP_REG_ADSPIC2: u32;
    static HDA_DSP_REG_ADSPIC2_SNDW: u32;
    static SDW_SHIM_LCAP: u32;
    static SDW_SHIM_LCAP_LCOUNT_MASK: u32;
    static AZX_REG_ML_LEPTR_ID_SDW: u32;
    static SOF_IPC_TYPE_3: u32;
    static SOF_DBG_DUMP_OPTIONAL: u32;
    static SOF_DBG_DUMP_REGS: u32;
    static HDA_DSP_SRAM_REG_FW_STATUS: u32;
    static HDA_DSP_SRAM_REG_FW_TRACEP: u32;
    static HDA_DSP_STACK_DUMP_SIZE: usize;
}

unsafe fn BIT(n: u32) -> u32 {
    1u32 << n
}

extern "C" {
    fn HDA_DSP_ADSPCS_CRST_MASK(core_mask: c_uint) -> u32;
    fn HDA_DSP_ADSPCS_CSTALL_MASK(core_mask: c_uint) -> u32;
    fn HDA_DSP_ADSPCS_CPA_MASK(core_mask: c_uint) -> u32;
    fn HDA_DSP_ADSPCS_SPA_MASK(core_mask: c_uint) -> u32;
    fn SOF_STREAM_SD_OFFSET(s: *mut hdac_stream) -> c_int;
    fn FSR_TO_STATE_CODE(fsr: u32) -> u32;
    fn FSR_TO_WAIT_STATE_CODE(fsr: u32) -> u32;
    fn FSR_TO_MODULE_CODE(fsr: u32) -> u32;
}

extern "C" {
    static SOF_DAI_DSP_ACCESS: usize;
    static SOF_DAI_HOST_ACCESS: usize;
    static SOF_DAI_ACCESS_NUM: usize;
    static SOF_DAI_INTEL_SSP: u32;
    static SOF_DAI_INTEL_DMIC: u32;
    static SOF_DAI_INTEL_HDA: u32;
    static SOF_DAI_INTEL_ALH: u32;
    static SOF_INTEL_TANGIER: u32;
    static SOF_INTEL_BAYTRAIL: u32;
    static SOF_INTEL_BROADWELL: u32;
    static SOF_INTEL_CAVS_1_5: u32;
    static SOF_INTEL_CAVS_1_5_PLUS: u32;
    static SOF_INTEL_CAVS_1_8: u32;
    static SOF_INTEL_CAVS_2_0: u32;
    static SOF_INTEL_CAVS_2_5: u32;
    static SOF_INTEL_ACE_1_0: u32;
    static SOF_INTEL_ACE_2_0: u32;
    static SOF_INTEL_ACE_3_0: u32;
    static SOF_INTEL_ACE_4_0: u32;
}

extern "C" {
    static EIO: c_int;
    static EINVAL: c_int;
    static EBUSY: c_int;
    static ETIMEDOUT: c_int;
}

extern "C" {
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
}

extern "C" {
    static HDA_DSP_ROM_CSE_ERROR: u32;
    static HDA_DSP_ROM_CSE_WRONG_RESPONSE: u32;
    static HDA_DSP_ROM_IMR_TO_SMALL: u32;
    static HDA_DSP_ROM_BASE_FW_NOT_FOUND: u32;
    static HDA_DSP_ROM_CSE_VALIDATION_FAILED: u32;
    static HDA_DSP_ROM_IPC_FATAL_ERROR: u32;
    static HDA_DSP_ROM_L2_CACHE_ERROR: u32;
    static HDA_DSP_ROM_LOAD_OFFSET_TO_SMALL: u32;
    static HDA_DSP_ROM_API_PTR_INVALID: u32;
    static HDA_DSP_ROM_BASEFW_INCOMPAT: u32;
    static HDA_DSP_ROM_UNHANDLED_INTERRUPT: u32;
    static HDA_DSP_ROM_MEMORY_HOLE_ECC: u32;
    static HDA_DSP_ROM_KERNEL_EXCEPTION: u32;
    static HDA_DSP_ROM_USER_EXCEPTION: u32;
    static HDA_DSP_ROM_UNEXPECTED_RESET: u32;
    static HDA_DSP_ROM_NULL_FW_ENTRY: u32;
    static FSR_STATE_ROM_INIT: u32;
    static FSR_STATE_ROM_INIT_DONE: u32;
    static FSR_STATE_ROM_CSE_MANIFEST_LOADED: u32;
    static FSR_STATE_ROM_FW_MANIFEST_LOADED: u32;
    static FSR_STATE_ROM_FW_FW_LOADED: u32;
    static FSR_STATE_ROM_FW_ENTERED: u32;
    static FSR_STATE_ROM_VERIFY_FEATURE_MASK: u32;
    static FSR_STATE_ROM_GET_LOAD_OFFSET: u32;
    static FSR_STATE_ROM_FETCH_ROM_EXT: u32;
    static FSR_STATE_ROM_FETCH_ROM_EXT_DONE: u32;
    static FSR_STATE_ROM_CSE_IMR_REQUEST: u32;
    static FSR_STATE_ROM_CSE_IMR_GRANTED: u32;
    static FSR_STATE_ROM_CSE_VALIDATE_IMAGE_REQUEST: u32;
    static FSR_STATE_ROM_CSE_IMAGE_VALIDATED: u32;
    static FSR_STATE_ROM_CSE_IPC_IFACE_INIT: u32;
    static FSR_STATE_ROM_CSE_IPC_RESET_PHASE_1: u32;
    static FSR_STATE_ROM_CSE_IPC_OPERATIONAL_ENTRY: u32;
    static FSR_STATE_ROM_CSE_IPC_OPERATIONAL: u32;
    static FSR_STATE_ROM_CSE_IPC_DOWN: u32;
    static FSR_STATE_ROM_RESET_VECTOR_DONE: u32;
    static FSR_STATE_ROM_PURGE_BOOT: u32;
    static FSR_STATE_ROM_RESTORE_BOOT: u32;
    static FSR_STATE_ROM_FW_ENTRY_POINT: u32;
    static FSR_STATE_ROM_VALIDATE_PUB_KEY: u32;
    static FSR_STATE_ROM_POWER_DOWN_HPSRAM: u32;
    static FSR_STATE_ROM_POWER_DOWN_ULPSRAM: u32;
    static FSR_STATE_ROM_POWER_UP_ULPSRAM_STACK: u32;
    static FSR_STATE_ROM_POWER_UP_HPSRAM_DMA: u32;
    static FSR_STATE_ROM_BEFORE_EP_POINTER_READ: u32;
    static FSR_STATE_ROM_VALIDATE_MANIFEST: u32;
    static FSR_STATE_ROM_VALIDATE_FW_MODULE: u32;
    static FSR_STATE_ROM_PROTECT_IMR_REGION: u32;
    static FSR_STATE_ROM_PUSH_MODEL_ROUTINE: u32;
    static FSR_STATE_ROM_PULL_MODEL_ROUTINE: u32;
    static FSR_STATE_ROM_VALIDATE_PKG_DIR: u32;
    static FSR_STATE_ROM_VALIDATE_CPD: u32;
    static FSR_STATE_ROM_VALIDATE_CSS_MAN_HEADER: u32;
    static FSR_STATE_ROM_VALIDATE_BLOB_SVN: u32;
    static FSR_STATE_ROM_VERIFY_IFWI_PARTITION: u32;
    static FSR_STATE_ROM_REMOVE_ACCESS_CONTROL: u32;
    static FSR_STATE_ROM_AUTH_BYPASS: u32;
    static FSR_STATE_ROM_AUTH_ENABLED: u32;
    static FSR_STATE_ROM_INIT_DMA: u32;
    static FSR_STATE_ROM_PURGE_FW_ENTRY: u32;
    static FSR_STATE_ROM_PURGE_FW_END: u32;
    static FSR_STATE_ROM_CLEAN_UP_BSS_DONE: u32;
    static FSR_STATE_ROM_IMR_RESTORE_ENTRY: u32;
    static FSR_STATE_ROM_IMR_RESTORE_END: u32;
    static FSR_STATE_ROM_FW_MANIFEST_IN_DMA_BUFF: u32;
    static FSR_STATE_ROM_LOAD_CSE_MAN_TO_IMR: u32;
    static FSR_STATE_ROM_LOAD_FW_MAN_TO_IMR: u32;
    static FSR_STATE_ROM_LOAD_FW_CODE_TO_IMR: u32;
    static FSR_STATE_ROM_FW_LOADING_DONE: u32;
    static FSR_STATE_ROM_FW_CODE_LOADED: u32;
    static FSR_STATE_ROM_VERIFY_IMAGE_TYPE: u32;
    static FSR_STATE_ROM_AUTH_API_INIT: u32;
    static FSR_STATE_ROM_AUTH_API_PROC: u32;
    static FSR_STATE_ROM_AUTH_API_FIRST_BUSY: u32;
    static FSR_STATE_ROM_AUTH_API_FIRST_RESULT: u32;
    static FSR_STATE_ROM_AUTH_API_CLEANUP: u32;
    static FSR_STATE_BRINGUP_INIT: u32;
    static FSR_STATE_BRINGUP_INIT_DONE: u32;
    static FSR_STATE_BRINGUP_HPSRAM_LOAD: u32;
    static FSR_STATE_BRINGUP_UNPACK_START: u32;
    static FSR_STATE_BRINGUP_IMR_RESTORE: u32;
    static FSR_STATE_BRINGUP_FW_ENTERED: u32;
    static FSR_WAIT_FOR_IPC_BUSY: u32;
    static FSR_WAIT_FOR_IPC_DONE: u32;
    static FSR_WAIT_FOR_CACHE_INVALIDATION: u32;
    static FSR_WAIT_FOR_LP_SRAM_OFF: u32;
    static FSR_WAIT_FOR_DMA_BUFFER_FULL: u32;
    static FSR_WAIT_FOR_CSE_CSR: u32;
    static FSR_MOD_ROM: usize;
    static FSR_MOD_ROM_BYP: usize;
    static FSR_MOD_BASE_FW: usize;
    static FSR_MOD_LP_BOOT: usize;
    static FSR_MOD_BRNGUP: u32;
    static FSR_MOD_ROM_EXT: u32;
    static FSR_HALTED: u32;
}

unsafe fn hda_get_interfaces(sdev: *mut snd_sof_dev, interface_mask: *mut u32) {
    let chip: *const sof_intel_dsp_desc = get_chip_info((*sdev).pdata);

    match (*chip).hw_ip_version {
        x if x == SOF_INTEL_TANGIER || x == SOF_INTEL_BAYTRAIL || x == SOF_INTEL_BROADWELL => {
            *interface_mask.add(SOF_DAI_DSP_ACCESS) = BIT(SOF_DAI_INTEL_SSP);
        }
        x if x == SOF_INTEL_CAVS_1_5 || x == SOF_INTEL_CAVS_1_5_PLUS => {
            *interface_mask.add(SOF_DAI_DSP_ACCESS) =
                BIT(SOF_DAI_INTEL_SSP) | BIT(SOF_DAI_INTEL_DMIC) | BIT(SOF_DAI_INTEL_HDA);
            *interface_mask.add(SOF_DAI_HOST_ACCESS) = BIT(SOF_DAI_INTEL_HDA);
        }
        x if x == SOF_INTEL_CAVS_1_8 || x == SOF_INTEL_CAVS_2_0 || x == SOF_INTEL_CAVS_2_5 || x == SOF_INTEL_ACE_1_0 => {
            *interface_mask.add(SOF_DAI_DSP_ACCESS) =
                BIT(SOF_DAI_INTEL_SSP) | BIT(SOF_DAI_INTEL_DMIC) |
                BIT(SOF_DAI_INTEL_HDA) | BIT(SOF_DAI_INTEL_ALH);
            *interface_mask.add(SOF_DAI_HOST_ACCESS) = BIT(SOF_DAI_INTEL_HDA);
        }
        x if x == SOF_INTEL_ACE_2_0 || x == SOF_INTEL_ACE_3_0 || x == SOF_INTEL_ACE_4_0 => {
            *interface_mask.add(SOF_DAI_DSP_ACCESS) =
                BIT(SOF_DAI_INTEL_SSP) | BIT(SOF_DAI_INTEL_DMIC) |
                BIT(SOF_DAI_INTEL_HDA) | BIT(SOF_DAI_INTEL_ALH);
            /* all interfaces accessible without DSP */
            *interface_mask.add(SOF_DAI_HOST_ACCESS) = *interface_mask.add(SOF_DAI_DSP_ACCESS);
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn hda_get_interface_mask(sdev: *mut snd_sof_dev) -> u32 {
    let mut interface_mask: [u32; 2] = [0; 2];

    hda_get_interfaces(sdev, interface_mask.as_mut_ptr());

    interface_mask[(*sdev).dspless_mode_selected as usize]
}

#[no_mangle]
pub unsafe extern "C" fn hda_is_chain_dma_supported(sdev: *mut snd_sof_dev, dai_type: u32) -> bool {
    let mut interface_mask: [u32; 2] = [0; 2];
    let chip: *const sof_intel_dsp_desc;

    if (*sdev).dspless_mode_selected != 0 {
        return false;
    }

    hda_get_interfaces(sdev, interface_mask.as_mut_ptr());

    if (interface_mask[SOF_DAI_DSP_ACCESS] & BIT(dai_type)) == 0 {
        return false;
    }

    if dai_type == SOF_DAI_INTEL_HDA {
        return true;
    }

    match dai_type {
        x if x == SOF_DAI_INTEL_SSP || x == SOF_DAI_INTEL_DMIC || x == SOF_DAI_INTEL_ALH => {
            chip = get_chip_info((*sdev).pdata);
            if (*chip).hw_ip_version < SOF_INTEL_ACE_2_0 {
                return false;
            }
            true
        }
        _ => false,
    }
}

/*
 * DSP Core control.
 */

unsafe fn hda_dsp_core_reset_enter(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int {
    let mut adspcs: u32;
    let reset: u32;
    let mut ret: c_int;

    /* set reset bits for cores */
    reset = HDA_DSP_ADSPCS_CRST_MASK(core_mask);
    snd_sof_dsp_update_bits_unlocked(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS, reset, reset);

    /* poll with timeout to check if operation successful */
    ret = snd_sof_dsp_read_poll_timeout_adspcs_eq(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS, reset, HDA_DSP_REG_POLL_INTERVAL_US, HDA_DSP_RESET_TIMEOUT_US);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: %s: timeout on HDA_DSP_REG_ADSPCS read\n\0".as_ptr() as *const c_char, b"hda_dsp_core_reset_enter\0".as_ptr());
        return ret;
    }

    /* has core entered reset ? */
    adspcs = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS);
    if (adspcs & HDA_DSP_ADSPCS_CRST_MASK(core_mask)) != HDA_DSP_ADSPCS_CRST_MASK(core_mask) {
        dev_err((*sdev).dev, b"error: reset enter failed: core_mask %x adspcs 0x%x\n\0".as_ptr() as *const c_char, core_mask, adspcs);
        ret = -EIO;
    }

    ret
}

unsafe fn hda_dsp_core_reset_leave(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int {
    let crst: c_uint;
    let mut adspcs: u32;
    let mut ret: c_int;

    /* clear reset bits for cores */
    snd_sof_dsp_update_bits_unlocked(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS, HDA_DSP_ADSPCS_CRST_MASK(core_mask), 0);

    /* poll with timeout to check if operation successful */
    crst = HDA_DSP_ADSPCS_CRST_MASK(core_mask);
    ret = snd_sof_dsp_read_poll_timeout_adspcs_clear(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS, crst, HDA_DSP_REG_POLL_INTERVAL_US, HDA_DSP_RESET_TIMEOUT_US);

    if ret < 0 {
        dev_err((*sdev).dev, b"error: %s: timeout on HDA_DSP_REG_ADSPCS read\n\0".as_ptr() as *const c_char, b"hda_dsp_core_reset_leave\0".as_ptr());
        return ret;
    }

    /* has core left reset ? */
    adspcs = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS);
    if (adspcs & HDA_DSP_ADSPCS_CRST_MASK(core_mask)) != 0 {
        dev_err((*sdev).dev, b"error: reset leave failed: core_mask %x adspcs 0x%x\n\0".as_ptr() as *const c_char, core_mask, adspcs);
        ret = -EIO;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_core_stall_reset(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int {
    /* stall core */
    snd_sof_dsp_update_bits_unlocked(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS, HDA_DSP_ADSPCS_CSTALL_MASK(core_mask), HDA_DSP_ADSPCS_CSTALL_MASK(core_mask));

    /* set reset state */
    hda_dsp_core_reset_enter(sdev, core_mask)
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_core_is_enabled(sdev: *mut snd_sof_dev, core_mask: c_uint) -> bool {
    let val: c_int;
    let is_enable: bool;

    val = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS) as c_int;

    unsafe fn mask_is_equal(v: c_int, m: c_uint, field: unsafe extern "C" fn(c_uint) -> u32) -> bool {
        let _m: u32 = field(m);
        ((v as u32) & _m) == _m
    }

    is_enable = mask_is_equal(val, core_mask, HDA_DSP_ADSPCS_CPA_MASK) &&
        mask_is_equal(val, core_mask, HDA_DSP_ADSPCS_SPA_MASK) &&
        ((val as u32) & HDA_DSP_ADSPCS_CRST_MASK(core_mask)) == 0 &&
        ((val as u32) & HDA_DSP_ADSPCS_CSTALL_MASK(core_mask)) == 0;

    dev_dbg((*sdev).dev, b"DSP core(s) enabled? %d : core_mask %x\n\0".as_ptr() as *const c_char, is_enable as c_int, core_mask);

    is_enable
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_core_run(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int {
    let mut ret: c_int;

    /* leave reset state */
    ret = hda_dsp_core_reset_leave(sdev, core_mask);
    if ret < 0 {
        return ret;
    }

    /* run core */
    dev_dbg((*sdev).dev, b"unstall/run core: core_mask = %x\n\0".as_ptr() as *const c_char, core_mask);
    snd_sof_dsp_update_bits_unlocked(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS, HDA_DSP_ADSPCS_CSTALL_MASK(core_mask), 0);

    /* is core now running ? */
    if !hda_dsp_core_is_enabled(sdev, core_mask) {
        hda_dsp_core_stall_reset(sdev, core_mask);
        dev_err((*sdev).dev, b"error: DSP start core failed: core_mask %x\n\0".as_ptr() as *const c_char, core_mask);
        ret = -EIO;
    }

    ret
}

/*
 * Power Management.
 */

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_core_power_up(sdev: *mut snd_sof_dev, mut core_mask: c_uint) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip: *const sof_intel_dsp_desc = (*hda).desc;
    let cpa: c_uint;
    let mut adspcs: u32;
    let mut ret: c_int;

    /* restrict core_mask to host managed cores mask */
    core_mask &= (*chip).host_managed_cores_mask;
    /* return if core_mask is not valid */
    if core_mask == 0 {
        return 0;
    }

    /* update bits */
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS, HDA_DSP_ADSPCS_SPA_MASK(core_mask), HDA_DSP_ADSPCS_SPA_MASK(core_mask));

    /* poll with timeout to check if operation successful */
    cpa = HDA_DSP_ADSPCS_CPA_MASK(core_mask);
    ret = snd_sof_dsp_read_poll_timeout_adspcs_eq(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS, cpa, HDA_DSP_REG_POLL_INTERVAL_US, HDA_DSP_RESET_TIMEOUT_US);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: %s: timeout on HDA_DSP_REG_ADSPCS read\n\0".as_ptr() as *const c_char, b"hda_dsp_core_power_up\0".as_ptr());
        return ret;
    }

    /* did core power up ? */
    adspcs = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS);
    if (adspcs & HDA_DSP_ADSPCS_CPA_MASK(core_mask)) != HDA_DSP_ADSPCS_CPA_MASK(core_mask) {
        dev_err((*sdev).dev, b"error: power up core failed core_mask %xadspcs 0x%x\n\0".as_ptr() as *const c_char, core_mask, adspcs);
        ret = -EIO;
    }

    ret
}

unsafe fn hda_dsp_core_power_down(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int {
    let mut ret: c_int;

    /* update bits */
    snd_sof_dsp_update_bits_unlocked(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS, HDA_DSP_ADSPCS_SPA_MASK(core_mask), 0);

    ret = snd_sof_dsp_read_poll_timeout_adspcs_clear(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPCS, HDA_DSP_ADSPCS_CPA_MASK(core_mask), HDA_DSP_REG_POLL_INTERVAL_US, HDA_DSP_PD_TIMEOUT * USEC_PER_MSEC);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: %s: timeout on HDA_DSP_REG_ADSPCS read\n\0".as_ptr() as *const c_char, b"hda_dsp_core_power_down\0".as_ptr());
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_enable_core(sdev: *mut snd_sof_dev, mut core_mask: c_uint) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip: *const sof_intel_dsp_desc = (*hda).desc;
    let ret: c_int;

    /* restrict core_mask to host managed cores mask */
    core_mask &= (*chip).host_managed_cores_mask;

    /* return if core_mask is not valid or cores are already enabled */
    if core_mask == 0 || hda_dsp_core_is_enabled(sdev, core_mask) {
        return 0;
    }

    /* power up */
    ret = hda_dsp_core_power_up(sdev, core_mask);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: dsp core power up failed: core_mask %x\n\0".as_ptr() as *const c_char, core_mask);
        return ret;
    }

    hda_dsp_core_run(sdev, core_mask)
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_core_reset_power_down(sdev: *mut snd_sof_dev, mut core_mask: c_uint) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip: *const sof_intel_dsp_desc = (*hda).desc;
    let mut ret: c_int;

    /* restrict core_mask to host managed cores mask */
    core_mask &= (*chip).host_managed_cores_mask;

    /* return if core_mask is not valid */
    if core_mask == 0 {
        return 0;
    }

    /* place core in reset prior to power down */
    ret = hda_dsp_core_stall_reset(sdev, core_mask);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: dsp core reset failed: core_mask %x\n\0".as_ptr() as *const c_char, core_mask);
        return ret;
    }

    /* power down core */
    ret = hda_dsp_core_power_down(sdev, core_mask);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: dsp core power down fail mask %x: %d\n\0".as_ptr() as *const c_char, core_mask, ret);
        return ret;
    }

    /* make sure we are in OFF state */
    if hda_dsp_core_is_enabled(sdev, core_mask) {
        dev_err((*sdev).dev, b"error: dsp core disable fail mask %x: %d\n\0".as_ptr() as *const c_char, core_mask, ret);
        ret = -EIO;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ipc_int_enable(sdev: *mut snd_sof_dev) {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip: *const sof_intel_dsp_desc = (*hda).desc;

    if (*sdev).dspless_mode_selected != 0 {
        return;
    }

    /* enable IPC DONE and BUSY interrupts */
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, (*chip).ipc_ctl, HDA_DSP_REG_HIPCCTL_DONE | HDA_DSP_REG_HIPCCTL_BUSY, HDA_DSP_REG_HIPCCTL_DONE | HDA_DSP_REG_HIPCCTL_BUSY);

    /* enable IPC interrupt */
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPIC, HDA_DSP_ADSPIC_IPC, HDA_DSP_ADSPIC_IPC);
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ipc_int_disable(sdev: *mut snd_sof_dev) {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip: *const sof_intel_dsp_desc = (*hda).desc;

    if (*sdev).dspless_mode_selected != 0 {
        return;
    }

    /* disable IPC interrupt */
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPIC, HDA_DSP_ADSPIC_IPC, 0);

    /* disable IPC BUSY and DONE interrupt */
    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, (*chip).ipc_ctl, HDA_DSP_REG_HIPCCTL_BUSY | HDA_DSP_REG_HIPCCTL_DONE, 0);
}

unsafe fn hda_dsp_wait_d0i3c_done(sdev: *mut snd_sof_dev) -> c_int {
    let mut retry: c_int = HDA_DSP_REG_POLL_RETRY_COUNT;
    let pdata: *mut snd_sof_pdata = (*sdev).pdata;
    let chip: *const sof_intel_dsp_desc;

    chip = get_chip_info(pdata);
    while (snd_sof_dsp_read8(sdev, HDA_DSP_HDA_BAR, (*chip).d0i3_offset) & SOF_HDA_VS_D0I3C_CIP) != 0 {
        if retry == 0 {
            return -ETIMEDOUT;
        }
        retry -= 1;
        usleep_range(10, 15);
    }

    0
}

unsafe fn hda_dsp_send_pm_gate_ipc(sdev: *mut snd_sof_dev, flags: u32) -> c_int {
    let pm_ops: *const sof_ipc_pm_ops = sof_ipc_get_ops_pm(sdev);

    if !pm_ops.is_null() {
        if let Some(set_pm_gate) = (*pm_ops).set_pm_gate {
            return set_pm_gate(sdev, flags);
        }
    }

    0
}

unsafe fn hda_dsp_update_d0i3c_register(sdev: *mut snd_sof_dev, value: u8) -> c_int {
    let pdata: *mut snd_sof_pdata = (*sdev).pdata;
    let chip: *const sof_intel_dsp_desc;
    let mut ret: c_int;
    let reg: u8;

    chip = get_chip_info(pdata);

    /* Write to D0I3C after Command-In-Progress bit is cleared */
    ret = hda_dsp_wait_d0i3c_done(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, b"CIP timeout before D0I3C update!\n\0".as_ptr() as *const c_char);
        return ret;
    }

    /* Update D0I3C register */
    snd_sof_dsp_update8(sdev, HDA_DSP_HDA_BAR, (*chip).d0i3_offset, SOF_HDA_VS_D0I3C_I3, value);

    /*
     * The value written to the D0I3C::I3 bit may not be taken into account immediately.
     * A delay is recommended before checking if D0I3C::CIP is cleared
     */
    usleep_range(30, 40);

    /* Wait for cmd in progress to be cleared before exiting the function */
    ret = hda_dsp_wait_d0i3c_done(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, b"CIP timeout after D0I3C update!\n\0".as_ptr() as *const c_char);
        return ret;
    }

    reg = snd_sof_dsp_read8(sdev, HDA_DSP_HDA_BAR, (*chip).d0i3_offset);
    /* Confirm d0i3 state changed with paranoia check */
    if ((reg ^ value) & SOF_HDA_VS_D0I3C_I3) != 0 {
        dev_err((*sdev).dev, b"failed to update D0I3C!\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    trace_sof_intel_D0I3C_updated(sdev, reg);

    0
}

/*
 * d0i3 streaming is enabled if all the active streams can
 * work in d0i3 state and playback is enabled
 */
unsafe fn hda_dsp_d0i3_streaming_applicable(_sdev: *mut snd_sof_dev) -> bool {
    let playback_active: bool = false;

    /* TODO: C list_for_each_entry(spcm, &sdev->pcm_list, list) and
     * for_each_pcm_streams(dir) require external kernel list/PCM iteration
     * mappings. The original loop returns false on any active stream that is
     * not d0i3_compatible and sets playback_active for playback streams.
     */

    playback_active
}

unsafe fn hda_dsp_set_D0_state(sdev: *mut snd_sof_dev, target_state: *const sof_dsp_power_state) -> c_int {
    let mut flags: u32 = 0;
    let mut ret: c_int;
    let mut value: u8 = 0;

    /*
     * Sanity check for illegal state transitions
     * The only allowed transitions are:
     * 1. D3 -> D0I0
     * 2. D0I0 -> D0I3
     * 3. D0I3 -> D0I0
     */
    match (*sdev).dsp_power_state.state {
        x if x == SOF_DSP_PM_D0 => {
            /* Follow the sequence below for D0 substate transitions */
        }
        x if x == SOF_DSP_PM_D3 => {
            /* Follow regular flow for D3 -> D0 transition */
            return 0;
        }
        _ => {
            dev_err((*sdev).dev, b"error: transition from %d to %d not allowed\n\0".as_ptr() as *const c_char, (*sdev).dsp_power_state.state, (*target_state).state);
            return -EINVAL;
        }
    }

    /* Set flags and register value for D0 target substate */
    if (*target_state).substate == SOF_HDA_DSP_PM_D0I3 {
        value = SOF_HDA_VS_D0I3C_I3;

        /*
         * Trace DMA need to be disabled when the DSP enters
         * D0I3 for S0Ix suspend, but it can be kept enabled
         * when the DSP enters D0I3 while the system is in S0
         * for debug purpose.
         */
        if !(*sdev).fw_trace_is_supported ||
            !hda_enable_trace_D0I3_S0 ||
            (*sdev).system_suspend_target != SOF_SUSPEND_NONE {
            flags = HDA_PM_NO_DMA_TRACE;
        }

        if hda_dsp_d0i3_streaming_applicable(sdev) {
            flags |= HDA_PM_PG_STREAMING;
        }
    } else {
        /* prevent power gating in D0I0 */
        flags = HDA_PM_PPG;
    }

    /* update D0I3C register */
    ret = hda_dsp_update_d0i3c_register(sdev, value);
    if ret < 0 {
        return ret;
    }

    /*
     * Notify the DSP of the state change.
     * If this IPC fails, revert the D0I3C register update in order
     * to prevent partial state change.
     */
    ret = hda_dsp_send_pm_gate_ipc(sdev, flags);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: PM_GATE ipc error %d\n\0".as_ptr() as *const c_char, ret);
        /* revert */
        value = if value != 0 { 0 } else { SOF_HDA_VS_D0I3C_I3 };
        /*
         * This can fail but return the IPC error to signal that
         * the state change failed.
         */
        hda_dsp_update_d0i3c_register(sdev, value);
        return ret;
    }

    ret
}

/* helper to log DSP state */
unsafe fn hda_dsp_state_log(sdev: *mut snd_sof_dev) {
    match (*sdev).dsp_power_state.state {
        x if x == SOF_DSP_PM_D0 => {
            match (*sdev).dsp_power_state.substate {
                y if y == SOF_HDA_DSP_PM_D0I0 => dev_dbg((*sdev).dev, b"Current DSP power state: D0I0\n\0".as_ptr() as *const c_char),
                y if y == SOF_HDA_DSP_PM_D0I3 => dev_dbg((*sdev).dev, b"Current DSP power state: D0I3\n\0".as_ptr() as *const c_char),
                _ => dev_dbg((*sdev).dev, b"Unknown DSP D0 substate: %d\n\0".as_ptr() as *const c_char, (*sdev).dsp_power_state.substate),
            }
        }
        x if x == SOF_DSP_PM_D1 => dev_dbg((*sdev).dev, b"Current DSP power state: D1\n\0".as_ptr() as *const c_char),
        x if x == SOF_DSP_PM_D2 => dev_dbg((*sdev).dev, b"Current DSP power state: D2\n\0".as_ptr() as *const c_char),
        x if x == SOF_DSP_PM_D3 => dev_dbg((*sdev).dev, b"Current DSP power state: D3\n\0".as_ptr() as *const c_char),
        _ => dev_dbg((*sdev).dev, b"Unknown DSP power state: %d\n\0".as_ptr() as *const c_char, (*sdev).dsp_power_state.state),
    }
}

/*
 * All DSP power state transitions are initiated by the driver.
 * If the requested state change fails, the error is simply returned.
 * Further state transitions are attempted only when the set_power_save() op
 * is called again either because of a new IPC sent to the DSP or
 * during system suspend/resume.
 */
unsafe fn hda_dsp_set_power_state(sdev: *mut snd_sof_dev, target_state: *const sof_dsp_power_state) -> c_int {
    let mut ret: c_int = 0;

    match (*target_state).state {
        x if x == SOF_DSP_PM_D0 => ret = hda_dsp_set_D0_state(sdev, target_state),
        x if x == SOF_DSP_PM_D3 => {
            /* The only allowed transition is: D0I0 -> D3 */
            if (*sdev).dsp_power_state.state == SOF_DSP_PM_D0 &&
                (*sdev).dsp_power_state.substate == SOF_HDA_DSP_PM_D0I0 {
            } else {
                dev_err((*sdev).dev, b"error: transition from %d to %d not allowed\n\0".as_ptr() as *const c_char, (*sdev).dsp_power_state.state, (*target_state).state);
                return -EINVAL;
            }
        }
        _ => {
            dev_err((*sdev).dev, b"error: target state unsupported %d\n\0".as_ptr() as *const c_char, (*target_state).state);
            return -EINVAL;
        }
    }
    if ret < 0 {
        dev_err((*sdev).dev, b"failed to set requested target DSP state %d substate %d\n\0".as_ptr() as *const c_char, (*target_state).state, (*target_state).substate);
        return ret;
    }

    (*sdev).dsp_power_state = *target_state;
    hda_dsp_state_log(sdev);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_set_power_state_ipc3(sdev: *mut snd_sof_dev, target_state: *const sof_dsp_power_state) -> c_int {
    /*
     * When the DSP is already in D0I3 and the target state is D0I3,
     * it could be the case that the DSP is in D0I3 during S0
     * and the system is suspending to S0Ix. Therefore,
     * hda_dsp_set_D0_state() must be called to disable trace DMA
     * by sending the PM_GATE IPC to the FW.
     */
    if (*target_state).substate == SOF_HDA_DSP_PM_D0I3 &&
        (*sdev).system_suspend_target == SOF_SUSPEND_S0IX {
        return hda_dsp_set_power_state(sdev, target_state);
    }

    /*
     * For all other cases, return without doing anything if
     * the DSP is already in the target state.
     */
    if (*target_state).state == (*sdev).dsp_power_state.state &&
        (*target_state).substate == (*sdev).dsp_power_state.substate {
        return 0;
    }

    hda_dsp_set_power_state(sdev, target_state)
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_set_power_state_ipc4(sdev: *mut snd_sof_dev, target_state: *const sof_dsp_power_state) -> c_int {
    /* Return without doing anything if the DSP is already in the target state */
    if (*target_state).state == (*sdev).dsp_power_state.state &&
        (*target_state).substate == (*sdev).dsp_power_state.substate {
        return 0;
    }

    hda_dsp_set_power_state(sdev, target_state)
}

/*
 * Audio DSP states may transform as below:-
 *
 *                                         Opportunistic D0I3 in S0
 *     Runtime    +---------------------+  Delayed D0i3 work timeout
 *     suspend    |                     +--------------------+
 *   +------------+       D0I0(active)  |                    |
 *   |            |                     <---------------+    |
 *   |   +-------->                     |    New IPC    |    |
 *   |   |Runtime +--^--+---------^--+--+ (via mailbox) |    |
 *   |   |resume     |  |         |  |                  |    |
 *   |   |           |  |         |  |                  |    |
 *   |   |     System|  |         |  |                  |    |
 *   |   |     resume|  | S3/S0IX |  | S0IX             |    |
 *   |   |           |  | suspend |  |suspend           |    |
 *   |   |           |  |         |  |                  |    |
 *   |   |           |  |         |  |                  |    |
 * +-v---+-----------+--v-------+ |  |           +------+----v----+
 * |                            | |  +----------->                |
 * |       D3 (suspended)       | |              |      D0I3      |
 * |                            | +--------------+                |
 * |                            |  System resume |                |
 * +----------------------------+                 +----------------+
 *
 * S0IX suspend: The DSP is in D0I3 if any D0I3-compatible streams
 *               ignored the suspend trigger. Otherwise the DSP
 *               is in D3.
 */

unsafe fn hda_suspend(sdev: *mut snd_sof_dev, runtime_suspend: bool) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip: *const sof_intel_dsp_desc = (*hda).desc;
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let mut imr_lost: bool = false;
    let mut ret: c_int;
    let mut j: c_int;

    /*
     * The memory used for IMR boot loses its content in deeper than S3
     * state on CAVS platforms.
     * On ACE platforms due to the system architecture the IMR content is
     * lost at S3 state already, they are tailored for s2idle use.
     * We must not try IMR boot on next power up in these cases as it will
     * fail.
     */
    if (*sdev).system_suspend_target > SOF_SUSPEND_S3 ||
        ((*chip).hw_ip_version >= SOF_INTEL_ACE_1_0 && (*sdev).system_suspend_target == SOF_SUSPEND_S3) {
        imr_lost = true;
    }

    /*
     * In case of firmware crash or boot failure set the skip_imr_boot to true
     * as well in order to try to re-load the firmware to do a 'cold' boot.
     */
    if imr_lost || (*sdev).fw_state == SOF_FW_CRASHED || (*sdev).fw_state == SOF_FW_BOOT_FAILED {
        (*hda).skip_imr_boot = true;
    }

    ret = ((*chip).disable_interrupts.unwrap())(sdev);
    if ret < 0 {
        return ret;
    }

    /* make sure that no irq handler is pending before shutdown */
    synchronize_irq((*sdev).ipc_irq);

    hda_codec_jack_wake_enable(sdev, runtime_suspend);

    /* power down all hda links */
    hda_bus_ml_suspend(bus);

    if (*sdev).dspless_mode_selected != 0 {
        /* skip_dsp */
    } else {
        ret = ((*chip).power_down_dsp.unwrap())(sdev);
        if ret < 0 {
            dev_err((*sdev).dev, b"failed to power down DSP during suspend\n\0".as_ptr() as *const c_char);
            return ret;
        }

        /* reset ref counts for all cores */
        j = 0;
        while j < (*chip).cores_num {
            *(*sdev).dsp_core_ref_count.add(j as usize) = 0;
            j += 1;
        }

        /* disable ppcap interrupt */
        hda_dsp_ctrl_ppcap_enable(sdev, false);
        hda_dsp_ctrl_ppcap_int_enable(sdev, false);
    }

    /* disable hda bus irq and streams */
    hda_dsp_ctrl_stop_chip(sdev);

    /* disable LP retention mode */
    snd_sof_pci_update_bits(sdev, PCI_PGCTL, PCI_PGCTL_LSRMD_MASK, PCI_PGCTL_LSRMD_MASK);

    /* reset controller */
    ret = hda_dsp_ctrl_link_reset(sdev, true);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: failed to reset controller during suspend\n\0".as_ptr() as *const c_char);
        return ret;
    }

    /* display codec can powered off after link reset */
    hda_codec_i915_display_power(sdev, false);

    0
}

unsafe fn hda_resume(sdev: *mut snd_sof_dev, runtime_resume: bool) -> c_int {
    let mut ret: c_int;

    /* display codec must be powered before link reset */
    hda_codec_i915_display_power(sdev, true);

    /*
     * clear TCSEL to clear playback on some HD Audio
     * codecs. PCI TCSEL is defined in the Intel manuals.
     */
    snd_sof_pci_update_bits(sdev, PCI_TCSEL, 0x07, 0);

    /* reset and start hda controller */
    ret = hda_dsp_ctrl_init_chip(sdev, false);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: failed to start controller after resume\n\0".as_ptr() as *const c_char);
        /* cleanup */
    } else {
        /* check jack status */
        if runtime_resume {
            hda_codec_jack_wake_enable(sdev, false);
            if (*sdev).system_suspend_target == SOF_SUSPEND_NONE {
                hda_codec_jack_check(sdev);
            }
        }

        if (*sdev).dspless_mode_selected == 0 {
            /* enable ppcap interrupt */
            hda_dsp_ctrl_ppcap_enable(sdev, true);
            hda_dsp_ctrl_ppcap_int_enable(sdev, true);
        }
    }

    /* cleanup: display codec can powered off after controller init */
    hda_codec_i915_display_power(sdev, false);

    0
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_resume(sdev: *mut snd_sof_dev) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let pci: *mut pci_dev = to_pci_dev((*sdev).dev);
    let target_state = sof_dsp_power_state {
        state: SOF_DSP_PM_D0,
        substate: SOF_HDA_DSP_PM_D0I0,
    };
    let mut ret: c_int;

    /* resume from D0I3 */
    if (*sdev).dsp_power_state.state == SOF_DSP_PM_D0 {
        ret = hda_bus_ml_resume(bus);
        if ret < 0 {
            dev_err((*sdev).dev, b"error %d in %s: failed to power up links\0".as_ptr() as *const c_char, ret, b"hda_dsp_resume\0".as_ptr());
            return ret;
        }

        /* set up CORB/RIRB buffers if was on before suspend */
        hda_codec_resume_cmd_io(sdev);

        /* Set DSP power state */
        ret = snd_sof_dsp_set_power_state(sdev, &target_state);
        if ret < 0 {
            dev_err((*sdev).dev, b"error: setting dsp state %d substate %d\n\0".as_ptr() as *const c_char, target_state.state, target_state.substate);
            return ret;
        }

        /* restore L1SEN bit */
        if (*hda).l1_disabled {
            snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, HDA_VS_INTEL_EM2, HDA_VS_INTEL_EM2_L1SEN, 0);
        }

        /* restore and disable the system wakeup */
        pci_restore_state(pci);
        disable_irq_wake((*pci).irq);
        return 0;
    }

    /* init hda controller. DSP cores will be powered up during fw boot */
    ret = hda_resume(sdev, false);
    if ret < 0 {
        return ret;
    }

    snd_sof_dsp_set_power_state(sdev, &target_state)
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_runtime_resume(sdev: *mut snd_sof_dev) -> c_int {
    let target_state = sof_dsp_power_state {
        state: SOF_DSP_PM_D0,
        substate: 0,
    };
    let ret: c_int;

    /* init hda controller. DSP cores will be powered up during fw boot */
    ret = hda_resume(sdev, true);
    if ret < 0 {
        return ret;
    }

    snd_sof_dsp_set_power_state(sdev, &target_state)
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_runtime_idle(sdev: *mut snd_sof_dev) -> c_int {
    let hbus: *mut hdac_bus = sof_to_bus(sdev);

    if (*hbus).codec_powered != 0 {
        dev_dbg((*sdev).dev, b"some codecs still powered (%08X), not idle\n\0".as_ptr() as *const c_char, (*hbus).codec_powered as c_uint);
        return -EBUSY;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_runtime_suspend(sdev: *mut snd_sof_dev) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let target_state = sof_dsp_power_state {
        state: SOF_DSP_PM_D3,
        substate: 0,
    };
    let ret: c_int;

    if (*sdev).dspless_mode_selected == 0 {
        /* cancel any attempt for DSP D0I3 */
        cancel_delayed_work_sync(&mut (*hda).d0i3_work);

        /* Cancel the microphone privacy work if mic privacy is active */
        if (*hda).mic_privacy.active {
            cancel_work_sync(&mut (*hda).mic_privacy.work);
        }
    }

    /* stop hda controller and power dsp off */
    ret = hda_suspend(sdev, true);
    if ret < 0 {
        return ret;
    }

    snd_sof_dsp_set_power_state(sdev, &target_state)
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_suspend(sdev: *mut snd_sof_dev, target_state: u32) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let pci: *mut pci_dev = to_pci_dev((*sdev).dev);
    let target_dsp_state = sof_dsp_power_state {
        state: target_state,
        substate: if target_state == SOF_DSP_PM_D0 { SOF_HDA_DSP_PM_D0I3 } else { 0 },
    };
    let mut ret: c_int;

    if (*sdev).dspless_mode_selected == 0 {
        /* cancel any attempt for DSP D0I3 */
        cancel_delayed_work_sync(&mut (*hda).d0i3_work);

        /* Cancel the microphone privacy work if mic privacy is active */
        if (*hda).mic_privacy.active {
            cancel_work_sync(&mut (*hda).mic_privacy.work);
        }
    }

    if target_state == SOF_DSP_PM_D0 {
        /* Set DSP power state */
        ret = snd_sof_dsp_set_power_state(sdev, &target_dsp_state);
        if ret < 0 {
            dev_err((*sdev).dev, b"error: setting dsp state %d substate %d\n\0".as_ptr() as *const c_char, target_dsp_state.state, target_dsp_state.substate);
            return ret;
        }

        /* enable L1SEN to make sure the system can enter S0Ix */
        if (*hda).l1_disabled {
            snd_sof_dsp_update_bits(sdev, HDA_DSP_HDA_BAR, HDA_VS_INTEL_EM2, HDA_VS_INTEL_EM2_L1SEN, HDA_VS_INTEL_EM2_L1SEN);
        }

        /* stop the CORB/RIRB DMA if it is On */
        hda_codec_suspend_cmd_io(sdev);

        /* no link can be powered in s0ix state */
        ret = hda_bus_ml_suspend(bus);
        if ret < 0 {
            dev_err((*sdev).dev, b"error %d in %s: failed to power down links\0".as_ptr() as *const c_char, ret, b"hda_dsp_suspend\0".as_ptr());
            return ret;
        }

        /* enable the system waking up via IPC IRQ */
        enable_irq_wake((*pci).irq);
        pci_save_state(pci);
        return 0;
    }

    /* stop hda controller and power dsp off */
    ret = hda_suspend(sdev, false);
    if ret < 0 {
        dev_err((*bus).dev, b"error: suspending dsp\n\0".as_ptr() as *const c_char);
        return ret;
    }

    snd_sof_dsp_set_power_state(sdev, &target_dsp_state)
}

unsafe fn hda_dsp_check_for_dma_streams(_sdev: *mut snd_sof_dev) -> c_uint {
    let active_streams: c_uint = 0;

    /* TODO: C list_for_each_entry(s, &bus->stream_list, list) requires an
     * external kernel list iteration mapping. The original body reads
     * SOF_STREAM_SD_OFFSET(s) and ORs BIT(s->index) into active_streams when
     * SOF_HDA_SD_CTL_DMA_START is set.
     */

    active_streams
}

unsafe fn hda_dsp_s5_quirk(sdev: *mut snd_sof_dev) -> c_int {
    let ret: c_int;

    /*
     * Do not assume a certain timing between the prior
     * suspend flow, and running of this quirk function.
     * This is needed if the controller was just put
     * to reset before calling this function.
     */
    usleep_range(500, 1000);

    /*
     * Take controller out of reset to flush DMA
     * transactions.
     */
    ret = hda_dsp_ctrl_link_reset(sdev, false);
    if ret < 0 {
        return ret;
    }

    usleep_range(500, 1000);

    /* Restore state for shutdown, back to reset */
    hda_dsp_ctrl_link_reset(sdev, true)
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_shutdown_dma_flush(sdev: *mut snd_sof_dev) -> c_int {
    let active_streams: c_uint;
    let ret: c_int;
    let ret2: c_int;

    /* check if DMA cleanup has been successful */
    active_streams = hda_dsp_check_for_dma_streams(sdev);

    (*sdev).system_suspend_target = SOF_SUSPEND_S3;
    ret = snd_sof_suspend((*sdev).dev);

    if active_streams != 0 {
        dev_warn((*sdev).dev, b"There were active DSP streams (%#x) at shutdown, trying to recover\n\0".as_ptr() as *const c_char, active_streams);
        ret2 = hda_dsp_s5_quirk(sdev);
        if ret2 < 0 {
            dev_err((*sdev).dev, b"shutdown recovery failed (%d)\n\0".as_ptr() as *const c_char, ret2);
        }
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_shutdown(sdev: *mut snd_sof_dev) -> c_int {
    (*sdev).system_suspend_target = SOF_SUSPEND_S3;
    snd_sof_suspend((*sdev).dev)
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_set_hw_params_upon_resume(sdev: *mut snd_sof_dev) -> c_int {
    let ret: c_int;

    /* make sure all DAI resources are freed */
    ret = hda_dsp_dais_suspend(sdev);
    if ret < 0 {
        dev_warn((*sdev).dev, b"%s: failure in hda_dsp_dais_suspend\n\0".as_ptr() as *const c_char, b"hda_dsp_set_hw_params_upon_resume\0".as_ptr());
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_d0i3_work(work: *mut work_struct) {
    /* container_of(work, struct sof_intel_hda_dev, d0i3_work.work) */
    let hdev: *mut sof_intel_hda_dev = work as *mut sof_intel_hda_dev;
    let bus: *mut hdac_bus = &mut (*hdev).hbus.core;
    let sdev: *mut snd_sof_dev = dev_get_drvdata((*bus).dev);
    let target_state = sof_dsp_power_state {
        state: SOF_DSP_PM_D0,
        substate: SOF_HDA_DSP_PM_D0I3,
    };
    let ret: c_int;

    /* DSP can enter D0I3 iff only D0I3-compatible streams are active */
    if !snd_sof_dsp_only_d0i3_compatible_stream_active(sdev) {
        /* remain in D0I0 */
        return;
    }

    /* This can fail but error cannot be propagated */
    ret = snd_sof_dsp_set_power_state(sdev, &target_state);
    if ret < 0 {
        dev_err_ratelimited((*sdev).dev, b"error: failed to set DSP state %d substate %d\n\0".as_ptr() as *const c_char, target_state.state, target_state.substate);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_core_get(sdev: *mut snd_sof_dev, core: c_int) -> c_int {
    let pm_ops: *const sof_ipc_pm_ops = (*(*(*sdev).ipc).ops).pm;
    let mut ret: c_int;
    let ret1: c_int;

    /* power up core */
    ret = hda_dsp_enable_core(sdev, BIT(core as u32));
    if ret < 0 {
        dev_err((*sdev).dev, b"failed to power up core %d with err: %d\n\0".as_ptr() as *const c_char, core, ret);
        return ret;
    }

    /* No need to send IPC for primary core or if FW boot is not complete */
    if (*sdev).fw_state != SOF_FW_BOOT_COMPLETE || core == SOF_DSP_PRIMARY_CORE {
        return 0;
    }

    /* No need to continue the set_core_state ops is not available */
    if (*pm_ops).set_core_state.is_none() {
        return 0;
    }

    /* Now notify DSP for secondary cores */
    ret = ((*pm_ops).set_core_state.unwrap())(sdev, core, true);
    if ret < 0 {
        dev_err((*sdev).dev, b"failed to enable secondary core '%d' failed with %d\n\0".as_ptr() as *const c_char, core, ret);
        /* power_down */
        ret1 = hda_dsp_core_reset_power_down(sdev, BIT(core as u32));
        if ret1 < 0 {
            dev_err((*sdev).dev, b"failed to power down core: %d with err: %d\n\0".as_ptr() as *const c_char, core, ret1);
        }
        return ret;
    }

    ret
}

/* CONFIG_SND_SOC_SOF_INTEL_SOUNDWIRE */
#[no_mangle]
pub unsafe extern "C" fn hda_common_enable_sdw_irq(sdev: *mut snd_sof_dev, enable: bool) {
    let hdev: *mut sof_intel_hda_dev;

    hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

    if (*hdev).sdw.is_null() {
        return;
    }

    snd_sof_dsp_update_bits(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPIC2, HDA_DSP_REG_ADSPIC2_SNDW, if enable { HDA_DSP_REG_ADSPIC2_SNDW } else { 0 });
}

#[no_mangle]
pub unsafe extern "C" fn hda_sdw_int_enable(sdev: *mut snd_sof_dev, enable: bool) {
    let interface_mask: u32 = hda_get_interface_mask(sdev);
    let chip: *const sof_intel_dsp_desc;

    if (interface_mask & BIT(SOF_DAI_INTEL_ALH)) == 0 {
        return;
    }

    chip = get_chip_info((*sdev).pdata);
    if !chip.is_null() {
        if let Some(enable_sdw_irq) = (*chip).enable_sdw_irq {
            enable_sdw_irq(sdev, enable);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn hda_sdw_check_lcount_common(sdev: *mut snd_sof_dev) -> c_int {
    let hdev: *mut sof_intel_hda_dev;
    let ctx: *mut sdw_intel_ctx;
    let mut caps: u32;

    hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    ctx = (*hdev).sdw;

    caps = snd_sof_dsp_read(sdev, HDA_DSP_BAR, (*ctx).shim_base + SDW_SHIM_LCAP);
    caps &= SDW_SHIM_LCAP_LCOUNT_MASK;

    /* Check HW supported vs property value */
    if caps < (*ctx).count {
        dev_err((*sdev).dev, b"%s: BIOS master count %d is larger than hardware capabilities %d\n\0".as_ptr() as *const c_char, b"hda_sdw_check_lcount_common\0".as_ptr(), (*ctx).count, caps);
        return -EINVAL;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn hda_sdw_check_lcount_ext(sdev: *mut snd_sof_dev) -> c_int {
    let hdev: *mut sof_intel_hda_dev;
    let ctx: *mut sdw_intel_ctx;
    let bus: *mut hdac_bus;
    let slcount: u32;

    bus = sof_to_bus(sdev);

    hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    ctx = (*hdev).sdw;

    slcount = hdac_bus_eml_get_count(bus, true, AZX_REG_ML_LEPTR_ID_SDW);

    /* Check HW supported vs property value */
    if slcount < (*ctx).count {
        dev_err((*sdev).dev, b"%s: BIOS master count %d is larger than hardware capabilities %d\n\0".as_ptr() as *const c_char, b"hda_sdw_check_lcount_ext\0".as_ptr(), (*ctx).count, slcount);
        return -EINVAL;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn hda_sdw_check_lcount(sdev: *mut snd_sof_dev) -> c_int {
    let chip: *const sof_intel_dsp_desc;

    chip = get_chip_info((*sdev).pdata);
    if !chip.is_null() {
        if let Some(read_sdw_lcount) = (*chip).read_sdw_lcount {
            return read_sdw_lcount(sdev);
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn hda_sdw_process_wakeen(sdev: *mut snd_sof_dev) {
    let interface_mask: u32 = hda_get_interface_mask(sdev);
    let chip: *const sof_intel_dsp_desc;

    if (interface_mask & BIT(SOF_DAI_INTEL_ALH)) == 0 {
        return;
    }

    chip = get_chip_info((*sdev).pdata);
    if !chip.is_null() {
        if let Some(sdw_process_wakeen) = (*chip).sdw_process_wakeen {
            sdw_process_wakeen(sdev);
        }
    }
}
/* end CONFIG_SND_SOC_SOF_INTEL_SOUNDWIRE */

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_disable_interrupts(sdev: *mut snd_sof_dev) -> c_int {
    hda_sdw_int_enable(sdev, false);
    hda_dsp_ipc_int_disable(sdev);

    0
}

static hda_dsp_rom_fw_error_texts: [hda_dsp_msg_code; 16] = unsafe { [
    hda_dsp_msg_code { code: HDA_DSP_ROM_CSE_ERROR, text: b"error: cse error\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_CSE_WRONG_RESPONSE, text: b"error: cse wrong response\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_IMR_TO_SMALL, text: b"error: IMR too small\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_BASE_FW_NOT_FOUND, text: b"error: base fw not found\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_CSE_VALIDATION_FAILED, text: b"error: signature verification failed\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_IPC_FATAL_ERROR, text: b"error: ipc fatal error\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_L2_CACHE_ERROR, text: b"error: L2 cache error\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_LOAD_OFFSET_TO_SMALL, text: b"error: load offset too small\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_API_PTR_INVALID, text: b"error: API ptr invalid\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_BASEFW_INCOMPAT, text: b"error: base fw incompatible\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_UNHANDLED_INTERRUPT, text: b"error: unhandled interrupt\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_MEMORY_HOLE_ECC, text: b"error: ECC memory hole\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_KERNEL_EXCEPTION, text: b"error: kernel exception\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_USER_EXCEPTION, text: b"error: user exception\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_UNEXPECTED_RESET, text: b"error: unexpected reset\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: HDA_DSP_ROM_NULL_FW_ENTRY, text: b"error: null FW entry point\0".as_ptr() as *const c_char },
] };

static cavs_fsr_rom_state_names: [hda_dsp_msg_code; 19] = unsafe { [
    hda_dsp_msg_code { code: FSR_STATE_ROM_INIT, text: b"INIT\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_INIT_DONE, text: b"INIT_DONE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_CSE_MANIFEST_LOADED, text: b"CSE_MANIFEST_LOADED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FW_MANIFEST_LOADED, text: b"FW_MANIFEST_LOADED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FW_FW_LOADED, text: b"FW_FW_LOADED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FW_ENTERED, text: b"FW_ENTERED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_VERIFY_FEATURE_MASK, text: b"VERIFY_FEATURE_MASK\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_GET_LOAD_OFFSET, text: b"GET_LOAD_OFFSET\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FETCH_ROM_EXT, text: b"FETCH_ROM_EXT\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FETCH_ROM_EXT_DONE, text: b"FETCH_ROM_EXT_DONE\0".as_ptr() as *const c_char },
    /* CSE states */
    hda_dsp_msg_code { code: FSR_STATE_ROM_CSE_IMR_REQUEST, text: b"CSE_IMR_REQUEST\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_CSE_IMR_GRANTED, text: b"CSE_IMR_GRANTED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_CSE_VALIDATE_IMAGE_REQUEST, text: b"CSE_VALIDATE_IMAGE_REQUEST\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_CSE_IMAGE_VALIDATED, text: b"CSE_IMAGE_VALIDATED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_CSE_IPC_IFACE_INIT, text: b"CSE_IPC_IFACE_INIT\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_CSE_IPC_RESET_PHASE_1, text: b"CSE_IPC_RESET_PHASE_1\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_CSE_IPC_OPERATIONAL_ENTRY, text: b"CSE_IPC_OPERATIONAL_ENTRY\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_CSE_IPC_OPERATIONAL, text: b"CSE_IPC_OPERATIONAL\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_CSE_IPC_DOWN, text: b"CSE_IPC_DOWN\0".as_ptr() as *const c_char },
] };

static ace_fsr_rom_state_names: [hda_dsp_msg_code; 48] = unsafe { [
    hda_dsp_msg_code { code: FSR_STATE_ROM_INIT, text: b"INIT\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_INIT_DONE, text: b"INIT_DONE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_CSE_MANIFEST_LOADED, text: b"CSE_MANIFEST_LOADED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FW_MANIFEST_LOADED, text: b"FW_MANIFEST_LOADED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FW_FW_LOADED, text: b"FW_FW_LOADED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FW_ENTERED, text: b"FW_ENTERED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_VERIFY_FEATURE_MASK, text: b"VERIFY_FEATURE_MASK\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_GET_LOAD_OFFSET, text: b"GET_LOAD_OFFSET\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_RESET_VECTOR_DONE, text: b"RESET_VECTOR_DONE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_PURGE_BOOT, text: b"PURGE_BOOT\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_RESTORE_BOOT, text: b"RESTORE_BOOT\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FW_ENTRY_POINT, text: b"FW_ENTRY_POINT\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_VALIDATE_PUB_KEY, text: b"VALIDATE_PUB_KEY\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_POWER_DOWN_HPSRAM, text: b"POWER_DOWN_HPSRAM\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_POWER_DOWN_ULPSRAM, text: b"POWER_DOWN_ULPSRAM\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_POWER_UP_ULPSRAM_STACK, text: b"POWER_UP_ULPSRAM_STACK\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_POWER_UP_HPSRAM_DMA, text: b"POWER_UP_HPSRAM_DMA\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_BEFORE_EP_POINTER_READ, text: b"BEFORE_EP_POINTER_READ\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_VALIDATE_MANIFEST, text: b"VALIDATE_MANIFEST\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_VALIDATE_FW_MODULE, text: b"VALIDATE_FW_MODULE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_PROTECT_IMR_REGION, text: b"PROTECT_IMR_REGION\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_PUSH_MODEL_ROUTINE, text: b"PUSH_MODEL_ROUTINE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_PULL_MODEL_ROUTINE, text: b"PULL_MODEL_ROUTINE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_VALIDATE_PKG_DIR, text: b"VALIDATE_PKG_DIR\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_VALIDATE_CPD, text: b"VALIDATE_CPD\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_VALIDATE_CSS_MAN_HEADER, text: b"VALIDATE_CSS_MAN_HEADER\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_VALIDATE_BLOB_SVN, text: b"VALIDATE_BLOB_SVN\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_VERIFY_IFWI_PARTITION, text: b"VERIFY_IFWI_PARTITION\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_REMOVE_ACCESS_CONTROL, text: b"REMOVE_ACCESS_CONTROL\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_AUTH_BYPASS, text: b"AUTH_BYPASS\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_AUTH_ENABLED, text: b"AUTH_ENABLED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_INIT_DMA, text: b"INIT_DMA\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_PURGE_FW_ENTRY, text: b"PURGE_FW_ENTRY\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_PURGE_FW_END, text: b"PURGE_FW_END\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_CLEAN_UP_BSS_DONE, text: b"CLEAN_UP_BSS_DONE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_IMR_RESTORE_ENTRY, text: b"IMR_RESTORE_ENTRY\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_IMR_RESTORE_END, text: b"IMR_RESTORE_END\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FW_MANIFEST_IN_DMA_BUFF, text: b"FW_MANIFEST_IN_DMA_BUFF\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_LOAD_CSE_MAN_TO_IMR, text: b"LOAD_CSE_MAN_TO_IMR\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_LOAD_FW_MAN_TO_IMR, text: b"LOAD_FW_MAN_TO_IMR\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_LOAD_FW_CODE_TO_IMR, text: b"LOAD_FW_CODE_TO_IMR\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FW_LOADING_DONE, text: b"FW_LOADING_DONE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_FW_CODE_LOADED, text: b"FW_CODE_LOADED\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_VERIFY_IMAGE_TYPE, text: b"VERIFY_IMAGE_TYPE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_AUTH_API_INIT, text: b"AUTH_API_INIT\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_AUTH_API_PROC, text: b"AUTH_API_PROC\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_AUTH_API_FIRST_BUSY, text: b"AUTH_API_FIRST_BUSY\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_AUTH_API_FIRST_RESULT, text: b"AUTH_API_FIRST_RESULT\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_ROM_AUTH_API_CLEANUP, text: b"AUTH_API_CLEANUP\0".as_ptr() as *const c_char },
] };

static fsr_bringup_state_names: [hda_dsp_msg_code; 6] = unsafe { [
    hda_dsp_msg_code { code: FSR_STATE_BRINGUP_INIT, text: b"INIT\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_BRINGUP_INIT_DONE, text: b"INIT_DONE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_BRINGUP_HPSRAM_LOAD, text: b"HPSRAM_LOAD\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_BRINGUP_UNPACK_START, text: b"UNPACK_START\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_BRINGUP_IMR_RESTORE, text: b"IMR_RESTORE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_STATE_BRINGUP_FW_ENTERED, text: b"FW_ENTERED\0".as_ptr() as *const c_char },
] };

static fsr_wait_state_names: [hda_dsp_msg_code; 6] = unsafe { [
    hda_dsp_msg_code { code: FSR_WAIT_FOR_IPC_BUSY, text: b"IPC_BUSY\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_WAIT_FOR_IPC_DONE, text: b"IPC_DONE\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_WAIT_FOR_CACHE_INVALIDATION, text: b"CACHE_INVALIDATION\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_WAIT_FOR_LP_SRAM_OFF, text: b"LP_SRAM_OFF\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_WAIT_FOR_DMA_BUFFER_FULL, text: b"DMA_BUFFER_FULL\0".as_ptr() as *const c_char },
    hda_dsp_msg_code { code: FSR_WAIT_FOR_CSE_CSR, text: b"CSE_CSR\0".as_ptr() as *const c_char },
] };

static mut fsr_module_names: [*const c_char; 6] = [
    b"ROM\0".as_ptr() as *const c_char,
    b"ROM_BYP\0".as_ptr() as *const c_char,
    b"BASE_FW\0".as_ptr() as *const c_char,
    b"LP_BOOT\0".as_ptr() as *const c_char,
    b"BRNGUP\0".as_ptr() as *const c_char,
    b"ROM_EXT\0".as_ptr() as *const c_char,
];

unsafe fn hda_dsp_get_state_text(code: u32, msg_code: *const hda_dsp_msg_code, array_size: size_t) -> *const c_char {
    let mut i: c_int;

    i = 0;
    while (i as size_t) < array_size {
        if code == (*msg_code.add(i as usize)).code {
            return (*msg_code.add(i as usize)).text;
        }
        i += 1;
    }

    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_get_state(sdev: *mut snd_sof_dev, level: *const c_char) {
    let chip: *const sof_intel_dsp_desc = get_chip_info((*sdev).pdata);
    let mut state_text: *const c_char;
    let mut error_text: *const c_char;
    let module_text: *const c_char;
    let fsr: u32;
    let state: u32;
    let wait_state: u32;
    let module: u32;
    let error_code: u32;

    fsr = snd_sof_dsp_read(sdev, HDA_DSP_BAR, (*chip).rom_status_reg);
    state = FSR_TO_STATE_CODE(fsr);
    wait_state = FSR_TO_WAIT_STATE_CODE(fsr);
    module = FSR_TO_MODULE_CODE(fsr);

    if module > FSR_MOD_ROM_EXT {
        module_text = b"unknown\0".as_ptr() as *const c_char;
    } else {
        module_text = fsr_module_names[module as usize];
    }

    if module == FSR_MOD_BRNGUP {
        state_text = hda_dsp_get_state_text(state, fsr_bringup_state_names.as_ptr(), fsr_bringup_state_names.len());
    } else if (*chip).hw_ip_version < SOF_INTEL_ACE_1_0 {
        state_text = hda_dsp_get_state_text(state, cavs_fsr_rom_state_names.as_ptr(), cavs_fsr_rom_state_names.len());
    } else {
        state_text = hda_dsp_get_state_text(state, ace_fsr_rom_state_names.as_ptr(), ace_fsr_rom_state_names.len());
    }

    /* not for us, must be generic sof message */
    if state_text.is_null() {
        dev_printk(level, (*sdev).dev, b"%#010x: unknown ROM status value\n\0".as_ptr() as *const c_char, fsr);
        return;
    }

    if wait_state != 0 {
        let mut wait_state_text: *const c_char;

        wait_state_text = hda_dsp_get_state_text(wait_state, fsr_wait_state_names.as_ptr(), fsr_wait_state_names.len());
        if wait_state_text.is_null() {
            wait_state_text = b"unknown\0".as_ptr() as *const c_char;
        }

        dev_printk(level, (*sdev).dev, b"%#010x: module: %s, state: %s, waiting for: %s, %s\n\0".as_ptr() as *const c_char, fsr, module_text, state_text, wait_state_text, if (fsr & FSR_HALTED) != 0 { b"not running\0".as_ptr() } else { b"running\0".as_ptr() });
    } else {
        dev_printk(level, (*sdev).dev, b"%#010x: module: %s, state: %s, %s\n\0".as_ptr() as *const c_char, fsr, module_text, state_text, if (fsr & FSR_HALTED) != 0 { b"not running\0".as_ptr() } else { b"running\0".as_ptr() });
    }

    error_code = snd_sof_dsp_read(sdev, HDA_DSP_BAR, (*chip).rom_status_reg + 4);
    if error_code == 0 {
        return;
    }

    error_text = hda_dsp_get_state_text(error_code, hda_dsp_rom_fw_error_texts.as_ptr(), hda_dsp_rom_fw_error_texts.len());
    if error_text.is_null() {
        error_text = b"unknown\0".as_ptr() as *const c_char;
    }

    if state == FSR_STATE_ROM_FW_ENTERED {
        dev_printk(level, (*sdev).dev, b"status code: %#x (%s)\n\0".as_ptr() as *const c_char, error_code, error_text);
    } else {
        dev_printk(level, (*sdev).dev, b"error code: %#x (%s)\n\0".as_ptr() as *const c_char, error_code, error_text);
    }
}

unsafe fn hda_dsp_get_registers(sdev: *mut snd_sof_dev, xoops: *mut sof_ipc_dsp_oops_xtensa, panic_info: *mut sof_ipc_panic_info, stack: *mut u32, stack_words: size_t) {
    let mut offset: u32 = (*sdev).dsp_oops_offset;

    /* first read registers */
    sof_mailbox_read(sdev, offset, xoops as *mut c_void, size_of::<sof_ipc_dsp_oops_xtensa>());

    /* note: variable AR register array is not read */

    /* then get panic info */
    if (*xoops).arch_hdr.totalsize > EXCEPT_MAX_HDR_SIZE {
        dev_err((*sdev).dev, b"invalid header size 0x%x. FW oops is bogus\n\0".as_ptr() as *const c_char, (*xoops).arch_hdr.totalsize);
        return;
    }
    offset += (*xoops).arch_hdr.totalsize;
    sof_block_read(sdev, (*sdev).mmio_bar, offset, panic_info as *mut c_void, size_of::<sof_ipc_panic_info>());

    /* then get the stack */
    offset += size_of::<sof_ipc_panic_info>() as u32;
    sof_block_read(sdev, (*sdev).mmio_bar, offset, stack as *mut c_void, stack_words * size_of::<u32>());
}

/* dump the first 8 dwords representing the extended ROM status */
#[no_mangle]
pub unsafe extern "C" fn hda_dsp_dump_ext_rom_status(sdev: *mut snd_sof_dev, level: *const c_char, _flags: u32) {
    let chip: *const sof_intel_dsp_desc;
    let mut msg: [c_char; 128] = [0; 128];
    let mut len: c_int = 0;
    let value: u32;
    let mut i: c_int;

    chip = get_chip_info((*sdev).pdata);
    i = 0;
    while i < HDA_EXT_ROM_STATUS_SIZE {
        let value = snd_sof_dsp_read(sdev, HDA_DSP_BAR, (*chip).rom_status_reg + (i as u32) * 0x4);
        len += scnprintf(msg.as_mut_ptr().add(len as usize), msg.len() - len as usize, b" 0x%x\0".as_ptr() as *const c_char, value);
        i += 1;
    }

    dev_printk(level, (*sdev).dev, b"extended rom status: %s\0".as_ptr() as *const c_char, msg.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_dump(sdev: *mut snd_sof_dev, flags: u32) {
    let level: *const c_char = if (flags & SOF_DBG_DUMP_OPTIONAL) != 0 { KERN_DEBUG } else { KERN_ERR };
    let mut xoops: sof_ipc_dsp_oops_xtensa = core::mem::zeroed();
    let mut panic_info: sof_ipc_panic_info = core::mem::zeroed();
    let mut stack: [u32; 32] = [0; 32];

    /* print ROM/FW status */
    hda_dsp_get_state(sdev, level);

    /* The firmware register dump only available with IPC3 */
    if (flags & SOF_DBG_DUMP_REGS) != 0 && (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_3 {
        let status: u32 = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_SRAM_REG_FW_STATUS);
        let panic: u32 = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_SRAM_REG_FW_TRACEP);

        hda_dsp_get_registers(sdev, &mut xoops, &mut panic_info, stack.as_mut_ptr(), HDA_DSP_STACK_DUMP_SIZE);
        sof_print_oops_and_stack(sdev, level, status, panic, &mut xoops, &mut panic_info, stack.as_mut_ptr(), HDA_DSP_STACK_DUMP_SIZE);
    } else {
        hda_dsp_dump_ext_rom_status(sdev, level, flags);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
