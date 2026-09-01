// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright(c) 2020 Intel Corporation
//
// Author: Fred Oh <fred.oh@linux.intel.com>
//

/*
 * Hardware interface for audio DSP on IceLake.
 */

// C dependencies translated as external Rust dependencies:
// linux/array_size.h, linux/bits.h, linux/dev_printk.h, linux/errno.h,
// linux/slab.h, linux/string.h, linux/types.h, ../ipc4-priv.h, ../ops.h,
// hda.h, hda-ipc.h, ../sof-audio.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const ICL_DSP_HPRO_CORE_ID: c_uint = 3;

const fn bit(n: c_uint) -> c_uint {
    1u32 << n
}

const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    let all = !0u32;
    (all << l) & (all >> (31 - h))
}

unsafe extern "C" {
    static mut sof_hda_common_ops: snd_sof_dsp_ops;

    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;

    fn snd_sof_dsp_update_bits_unlocked(
        sdev: *mut snd_sof_dev,
        bar: c_uint,
        offset: c_uint,
        mask: c_uint,
        value: c_uint,
    );
    fn snd_sof_dsp_stall(sdev: *mut snd_sof_dev, core_mask: c_uint);

    fn hda_sdw_startup(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_sdw_int_enable(sdev: *mut snd_sof_dev, enable: bool);
    fn hda_dsp_enable_core(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int;
    fn hda_dsp_ctrl_clock_power_gating(sdev: *mut snd_sof_dev, enable: bool) -> c_int;
    fn sof_debug_check_flag(flag: c_uint) -> bool;

    fn hda_dsp_shutdown(sdev: *mut snd_sof_dev);
    fn cnl_ipc_irq_thread(irq: c_int, context: *mut c_void) -> c_int;
    fn cnl_ipc_send_msg(sdev: *mut snd_sof_dev, msg: *mut c_void) -> c_int;
    fn cnl_ipc_dump(sdev: *mut snd_sof_dev);
    fn hda_dsp_set_power_state_ipc3(sdev: *mut snd_sof_dev, state: *mut c_void) -> c_int;

    fn cnl_ipc4_irq_thread(irq: c_int, context: *mut c_void) -> c_int;
    fn cnl_ipc4_send_msg(sdev: *mut snd_sof_dev, msg: *mut c_void) -> c_int;
    fn cnl_ipc4_dump(sdev: *mut snd_sof_dev);
    fn hda_dsp_set_power_state_ipc4(sdev: *mut snd_sof_dev, state: *mut c_void) -> c_int;
    fn hda_dsp_ipc4_load_library(sdev: *mut snd_sof_dev, dma_id: c_uint, lib_id: c_uint) -> c_int;

    fn hda_dsp_cl_boot_firmware_iccmax(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_dsp_core_get(sdev: *mut snd_sof_dev, core: c_int) -> c_int;
    fn hda_set_dai_drv_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops);

    fn hda_sdw_check_lcount_common(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_common_enable_sdw_irq(sdev: *mut snd_sof_dev, enable: bool);
    fn hda_common_check_sdw_irq(sdev: *mut snd_sof_dev) -> bool;
    fn hda_sdw_check_wakeen_irq_common(sdev: *mut snd_sof_dev) -> bool;
    fn hda_sdw_process_wakeen_common(sdev: *mut snd_sof_dev);
    fn hda_dsp_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool;
    fn cl_dsp_init(sdev: *mut snd_sof_dev, chip: *const sof_intel_dsp_desc) -> c_int;
    fn hda_power_down_dsp(sdev: *mut snd_sof_dev);
    fn hda_dsp_disable_interrupts(sdev: *mut snd_sof_dev);
}

#[repr(C)]
pub struct snd_sof_debugfs_map {
    pub name: *const c_char,
    pub bar: c_uint,
    pub offset: c_uint,
    pub size: c_uint,
    pub access: c_uint,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub pdata: *mut snd_sof_pdata,
    pub dev: *mut c_void,
    pub first_boot: bool,
    pub fw_ready: sof_ipc_info,
    pub enabled_cores_mask: c_uint,
    pub dsp_core_ref_count: [c_uint; 32],
    pub private: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut sof_intel_hda_dev,
    pub ipc_type: c_uint,
}

#[repr(C)]
pub struct sof_intel_hda_dev {
    pub desc: *const sof_intel_dsp_desc,
    pub clk_config_lpro: bool,
    pub imrboot_supported: bool,
}

#[repr(C)]
pub struct sof_ipc_info {
    pub flags: c_uint,
}

#[repr(C)]
pub struct sof_ipc4_fw_data {
    pub manifest_fw_hdr_offset: c_uint,
    pub mtrace_type: c_uint,
    pub load_library: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_uint, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub irq_thread: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    pub send_msg: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void) -> c_int>,
    pub ipc_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub set_power_state: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void) -> c_int>,
    pub debug_map: *const snd_sof_debugfs_map,
    pub debug_map_count: usize,
    pub post_fw_run: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub run: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub stall: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_uint) -> c_int>,
    pub core_get: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_int) -> c_int>,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub cores_num: c_uint,
    pub init_core_mask: c_uint,
    pub host_managed_cores_mask: c_uint,
    pub ipc_req: c_uint,
    pub ipc_req_mask: c_uint,
    pub ipc_ack: c_uint,
    pub ipc_ack_mask: c_uint,
    pub ipc_ctl: c_uint,
    pub rom_status_reg: c_uint,
    pub rom_init_timeout: c_uint,
    pub ssp_count: c_uint,
    pub ssp_base_offset: c_uint,
    pub sdw_shim_base: c_uint,
    pub sdw_alh_base: c_uint,
    pub d0i3_offset: c_uint,
    pub read_sdw_lcount: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub enable_sdw_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev, bool)>,
    pub check_sdw_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> bool>,
    pub check_sdw_wakeen_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> bool>,
    pub sdw_process_wakeen: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub check_ipc_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> bool>,
    pub cl_init: Option<unsafe extern "C" fn(*mut snd_sof_dev, *const sof_intel_dsp_desc) -> c_int>,
    pub power_down_dsp: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub disable_interrupts: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub hw_ip_version: c_uint,
    pub platform: *const c_char,
}

const HDA_DSP_HDA_BAR: c_uint = 0;
const HDA_DSP_PP_BAR: c_uint = 0;
const HDA_DSP_BAR: c_uint = 0;
const SOF_DEBUGFS_ACCESS_ALWAYS: c_uint = 0;
const HDA_DSP_REG_ADSPCS: c_uint = 0;
const SOF_DBG_IGNORE_D3_PERSISTENT: c_uint = 0;
const SOF_IPC_INFO_D3_PERSISTENT: c_uint = 0;
const SOF_IPC_TYPE_3: c_uint = 3;
const SOF_IPC_TYPE_4: c_uint = 4;
const SOF_MAN4_FW_HDR_OFFSET: c_uint = 0;
const SOF_IPC4_MTRACE_INTEL_CAVS_2: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const CNL_DSP_REG_HIPCIDR: c_uint = 0;
const CNL_DSP_REG_HIPCIDR_BUSY: c_uint = 0;
const CNL_DSP_REG_HIPCIDA: c_uint = 0;
const CNL_DSP_REG_HIPCIDA_DONE: c_uint = 0;
const CNL_DSP_REG_HIPCCTL: c_uint = 0;
const HDA_DSP_SRAM_REG_ROM_STATUS: c_uint = 0;
const ICL_SSP_COUNT: c_uint = 0;
const CNL_SSP_BASE_OFFSET: c_uint = 0;
const SDW_SHIM_BASE: c_uint = 0;
const SDW_ALH_BASE: c_uint = 0;
const SOF_HDA_VS_D0I3C: c_uint = 0;
const SOF_INTEL_CAVS_2_0: c_uint = 0;

const fn HDA_DSP_ADSPCS_CSTALL_MASK(core_mask: c_uint) -> c_uint {
    core_mask
}

static ICL_DSP_DEBUGFS_HDA: &[u8] = b"hda\0";
static ICL_DSP_DEBUGFS_PP: &[u8] = b"pp\0";
static ICL_DSP_DEBUGFS_DSP: &[u8] = b"dsp\0";
static ICL_PLATFORM: &[u8] = b"icl\0";

static icl_dsp_debugfs: [snd_sof_debugfs_map; 3] = [
    snd_sof_debugfs_map {
        name: ICL_DSP_DEBUGFS_HDA.as_ptr() as *const c_char,
        bar: HDA_DSP_HDA_BAR,
        offset: 0,
        size: 0x4000,
        access: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: ICL_DSP_DEBUGFS_PP.as_ptr() as *const c_char,
        bar: HDA_DSP_PP_BAR,
        offset: 0,
        size: 0x1000,
        access: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: ICL_DSP_DEBUGFS_DSP.as_ptr() as *const c_char,
        bar: HDA_DSP_BAR,
        offset: 0,
        size: 0x10000,
        access: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
];

unsafe extern "C" fn icl_dsp_core_stall(sdev: *mut snd_sof_dev, mut core_mask: c_uint) -> c_int {
    let hda = (*(*sdev).pdata).hw_pdata;
    let chip = (*hda).desc;

    /* make sure core_mask in host managed cores */
    core_mask &= (*chip).host_managed_cores_mask;
    if core_mask == 0 {
        dev_err(
            (*sdev).dev,
            c"error: core_mask is not in host managed cores\n".as_ptr(),
        );
        return -EINVAL;
    }

    /* stall core */
    snd_sof_dsp_update_bits_unlocked(
        sdev,
        HDA_DSP_BAR,
        HDA_DSP_REG_ADSPCS,
        HDA_DSP_ADSPCS_CSTALL_MASK(core_mask),
        HDA_DSP_ADSPCS_CSTALL_MASK(core_mask),
    );

    0
}

/*
 * post fw run operation for ICL.
 * Core 3 will be powered up and in stall when HPRO is enabled
 */
unsafe extern "C" fn icl_dsp_post_fw_run(sdev: *mut snd_sof_dev) -> c_int {
    let hda = (*(*sdev).pdata).hw_pdata;
    let mut ret: c_int;

    if (*sdev).first_boot {
        let hdev = (*(*sdev).pdata).hw_pdata;

        ret = hda_sdw_startup(sdev);
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c"error: could not startup SoundWire links\n".as_ptr(),
            );
            return ret;
        }

        /* Check if IMR boot is usable */
        if !sof_debug_check_flag(SOF_DBG_IGNORE_D3_PERSISTENT)
            && ((*sdev).fw_ready.flags & SOF_IPC_INFO_D3_PERSISTENT) != 0
        {
            (*hdev).imrboot_supported = true;
        }
    }

    hda_sdw_int_enable(sdev, true);

    /*
     * The recommended HW programming sequence for ICL is to
     * power up core 3 and keep it in stall if HPRO is enabled.
     */
    if !(*hda).clk_config_lpro {
        ret = hda_dsp_enable_core(sdev, bit(ICL_DSP_HPRO_CORE_ID));
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c"error: dsp core power up failed on core %d\n".as_ptr(),
                ICL_DSP_HPRO_CORE_ID,
            );
            return ret;
        }

        (*sdev).enabled_cores_mask |= bit(ICL_DSP_HPRO_CORE_ID);
        (*sdev).dsp_core_ref_count[ICL_DSP_HPRO_CORE_ID as usize] += 1;

        snd_sof_dsp_stall(sdev, bit(ICL_DSP_HPRO_CORE_ID));
    }

    /* re-enable clock gating and power gating */
    hda_dsp_ctrl_clock_power_gating(sdev, true)
}

/* Icelake ops */
#[unsafe(no_mangle)]
pub static mut sof_icl_ops: snd_sof_dsp_ops = unsafe { core::mem::zeroed() };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_icl_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    /* common defaults */
    memcpy(
        &raw mut sof_icl_ops as *mut c_void,
        &raw const sof_hda_common_ops as *const c_void,
        core::mem::size_of::<snd_sof_dsp_ops>(),
    );

    /* probe/remove/shutdown */
    sof_icl_ops.shutdown = Some(hda_dsp_shutdown);

    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_3 {
        /* doorbell */
        sof_icl_ops.irq_thread = Some(cnl_ipc_irq_thread);

        /* ipc */
        sof_icl_ops.send_msg = Some(cnl_ipc_send_msg);

        /* debug */
        sof_icl_ops.ipc_dump = Some(cnl_ipc_dump);

        sof_icl_ops.set_power_state = Some(hda_dsp_set_power_state_ipc3);
    }

    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_4 {
        let ipc4_data: *mut sof_ipc4_fw_data;

        (*sdev).private = kzalloc(core::mem::size_of::<sof_ipc4_fw_data>(), GFP_KERNEL);
        if (*sdev).private.is_null() {
            return -ENOMEM;
        }

        ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
        (*ipc4_data).manifest_fw_hdr_offset = SOF_MAN4_FW_HDR_OFFSET;

        (*ipc4_data).mtrace_type = SOF_IPC4_MTRACE_INTEL_CAVS_2;

        /* External library loading support */
        (*ipc4_data).load_library = Some(hda_dsp_ipc4_load_library);

        /* doorbell */
        sof_icl_ops.irq_thread = Some(cnl_ipc4_irq_thread);

        /* ipc */
        sof_icl_ops.send_msg = Some(cnl_ipc4_send_msg);

        /* debug */
        sof_icl_ops.ipc_dump = Some(cnl_ipc4_dump);

        sof_icl_ops.set_power_state = Some(hda_dsp_set_power_state_ipc4);
    }

    /* debug */
    sof_icl_ops.debug_map = icl_dsp_debugfs.as_ptr();
    sof_icl_ops.debug_map_count = icl_dsp_debugfs.len();

    /* pre/post fw run */
    sof_icl_ops.post_fw_run = Some(icl_dsp_post_fw_run);

    /* firmware run */
    sof_icl_ops.run = Some(hda_dsp_cl_boot_firmware_iccmax);
    sof_icl_ops.stall = Some(icl_dsp_core_stall);

    /* dsp core get/put */
    sof_icl_ops.core_get = Some(hda_dsp_core_get);

    /* set DAI driver ops */
    hda_set_dai_drv_ops(sdev, &raw mut sof_icl_ops);

    0
}

#[unsafe(no_mangle)]
pub static icl_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    /* Icelake */
    cores_num: 4,
    init_core_mask: 1,
    host_managed_cores_mask: genmask(3, 0),
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
    platform: ICL_PLATFORM.as_ptr() as *const c_char,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
