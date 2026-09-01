// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright(c) 2020 Intel Corporation
//
// Authors: Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//

/*
 * Hardware interface for audio DSP on Tigerlake.
 */

// C dependencies:
// <sound/sof/ext_manifest4.h>
// "../ipc4-priv.h"
// "../ops.h"
// "hda.h"
// "hda-ipc.h"
// "../sof-audio.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem;

const fn BIT(n: c_int) -> u32 {
    1u32 << (n as u32)
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

unsafe extern "C" {
    static sof_hda_common_ops: snd_sof_dsp_ops;

    fn hda_dsp_enable_core(sdev: *mut snd_sof_dev, core_mask: u32) -> c_int;
    fn hda_dsp_core_reset_power_down(sdev: *mut snd_sof_dev, core_mask: u32) -> c_int;
    fn hda_dsp_shutdown_dma_flush(sdev: *mut snd_sof_dev);
    fn cnl_ipc_irq_thread(irq: c_int, context: *mut c_void) -> c_int;
    fn cnl_ipc_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
    fn cnl_ipc_dump(sdev: *mut snd_sof_dev);
    fn hda_dsp_set_power_state_ipc3(sdev: *mut snd_sof_dev, target_state: *const sof_dsp_power_state) -> c_int;
    fn cnl_ipc4_irq_thread(irq: c_int, context: *mut c_void) -> c_int;
    fn cnl_ipc4_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
    fn cnl_ipc4_dump(sdev: *mut snd_sof_dev);
    fn hda_ipc4_dsp_dump(sdev: *mut snd_sof_dev, flags: u32);
    fn hda_dsp_ipc4_load_library(sdev: *mut snd_sof_dev, dma_id: c_int, lib_id: u32) -> c_int;
    fn hda_dsp_set_power_state_ipc4(sdev: *mut snd_sof_dev, target_state: *const sof_dsp_power_state) -> c_int;
    fn hda_set_dai_drv_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops);
    fn hda_dsp_post_fw_run(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_dsp_cl_boot_firmware_iccmax(sdev: *mut snd_sof_dev) -> c_int;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn hda_sdw_check_lcount_common(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_common_enable_sdw_irq(sdev: *mut snd_sof_dev, enable: bool);
    fn hda_common_check_sdw_irq(sdev: *mut snd_sof_dev) -> bool;
    fn hda_sdw_check_wakeen_irq_common(sdev: *mut snd_sof_dev) -> bool;
    fn hda_sdw_process_wakeen_common(sdev: *mut snd_sof_dev);
    fn hda_dsp_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool;
    fn cl_dsp_init(sdev: *mut snd_sof_dev, stream_tag: c_int, imr_boot: bool) -> c_int;
    fn hda_power_down_dsp(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_dsp_disable_interrupts(sdev: *mut snd_sof_dev);
}

fn kzalloc_obj<T>() -> *mut T {
    unsafe { kzalloc(mem::size_of::<T>(), GFP_KERNEL) as *mut T }
}

static tgl_dsp_debugfs: [snd_sof_debugfs_map; 3] = [
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

static tgl_ipc4_dsp_debugfs: [snd_sof_debugfs_map; 4] = [
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
    snd_sof_debugfs_map {
        name: b"fw_regs\0".as_ptr() as *const c_char,
        bar: HDA_DSP_BAR,
        offset: SRAM_WINDOW_OFFSET(0),
        size: 0x1000,
        access: SOF_DEBUGFS_ACCESS_D0_ONLY,
    },
];

unsafe extern "C" fn tgl_dsp_core_get(sdev: *mut snd_sof_dev, core: c_int) -> c_int {
    let pm_ops: *const sof_ipc_pm_ops = (*(*(*sdev).ipc).ops).pm;

    /* power up primary core if not already powered up and return */
    if core == SOF_DSP_PRIMARY_CORE {
        return hda_dsp_enable_core(sdev, BIT(core));
    }

    if let Some(set_core_state) = (*pm_ops).set_core_state {
        return set_core_state(sdev, core, true);
    }

    0
}

unsafe extern "C" fn tgl_dsp_core_put(sdev: *mut snd_sof_dev, core: c_int) -> c_int {
    let pm_ops: *const sof_ipc_pm_ops = (*(*(*sdev).ipc).ops).pm;
    let ret: c_int;

    if let Some(set_core_state) = (*pm_ops).set_core_state {
        ret = set_core_state(sdev, core, false);
        if ret < 0 {
            return ret;
        }
    }

    /* power down primary core and return */
    if core == SOF_DSP_PRIMARY_CORE {
        return hda_dsp_core_reset_power_down(sdev, BIT(core));
    }

    0
}

/* Tigerlake ops */
#[no_mangle]
pub static mut sof_tgl_ops: snd_sof_dsp_ops = unsafe { mem::zeroed() };

#[no_mangle]
pub unsafe extern "C" fn sof_tgl_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    /* common defaults */
    core::ptr::copy_nonoverlapping(
        &sof_hda_common_ops as *const snd_sof_dsp_ops,
        &mut sof_tgl_ops as *mut snd_sof_dsp_ops,
        1,
    );

    /* probe/remove/shutdown */
    sof_tgl_ops.shutdown = Some(hda_dsp_shutdown_dma_flush);

    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_3 {
        /* doorbell */
        sof_tgl_ops.irq_thread = Some(cnl_ipc_irq_thread);

        /* ipc */
        sof_tgl_ops.send_msg = Some(cnl_ipc_send_msg);

        /* debug */
        sof_tgl_ops.ipc_dump = Some(cnl_ipc_dump);
        sof_tgl_ops.debug_map = tgl_dsp_debugfs.as_ptr();
        sof_tgl_ops.debug_map_count = ARRAY_SIZE(&tgl_dsp_debugfs);

        sof_tgl_ops.set_power_state = Some(hda_dsp_set_power_state_ipc3);
    }

    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_4 {
        let ipc4_data: *mut sof_ipc4_fw_data;

        (*sdev).private = kzalloc_obj::<sof_ipc4_fw_data>() as *mut c_void;
        if (*sdev).private.is_null() {
            return -ENOMEM;
        }

        ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
        (*ipc4_data).manifest_fw_hdr_offset = SOF_MAN4_FW_HDR_OFFSET;

        (*ipc4_data).mtrace_type = SOF_IPC4_MTRACE_INTEL_CAVS_2;

        (*ipc4_data).fw_context_save = true;

        /* External library loading support */
        (*ipc4_data).load_library = Some(hda_dsp_ipc4_load_library);

        /* doorbell */
        sof_tgl_ops.irq_thread = Some(cnl_ipc4_irq_thread);

        /* ipc */
        sof_tgl_ops.send_msg = Some(cnl_ipc4_send_msg);

        /* debug */
        sof_tgl_ops.ipc_dump = Some(cnl_ipc4_dump);
        sof_tgl_ops.dbg_dump = Some(hda_ipc4_dsp_dump);
        sof_tgl_ops.debug_map = tgl_ipc4_dsp_debugfs.as_ptr();
        sof_tgl_ops.debug_map_count = ARRAY_SIZE(&tgl_ipc4_dsp_debugfs);

        sof_tgl_ops.set_power_state = Some(hda_dsp_set_power_state_ipc4);
    }

    /* set DAI driver ops */
    hda_set_dai_drv_ops(sdev, &mut sof_tgl_ops as *mut snd_sof_dsp_ops);

    /* pre/post fw run */
    sof_tgl_ops.post_fw_run = Some(hda_dsp_post_fw_run);

    /* firmware run */
    sof_tgl_ops.run = Some(hda_dsp_cl_boot_firmware_iccmax);

    /* dsp core get/put */
    sof_tgl_ops.core_get = Some(tgl_dsp_core_get);
    sof_tgl_ops.core_put = Some(tgl_dsp_core_put);

    0
}

#[no_mangle]
pub static tgl_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    /* Tigerlake , Alderlake */
    cores_num: 4,
    init_core_mask: 1,
    host_managed_cores_mask: BIT(0),
    ipc_req: CNL_DSP_REG_HIPCIDR,
    ipc_req_mask: CNL_DSP_REG_HIPCIDR_BUSY,
    ipc_ack: CNL_DSP_REG_HIPCIDA,
    ipc_ack_mask: CNL_DSP_REG_HIPCIDA_DONE,
    ipc_ctl: CNL_DSP_REG_HIPCCTL,
    rom_status_reg: HDA_DSP_SRAM_REG_ROM_STATUS,
    rom_init_timeout: 300,
    ssp_count: TGL_SSP_COUNT,
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
    hw_ip_version: SOF_INTEL_CAVS_2_5,
    platform: b"tgl\0".as_ptr() as *const c_char,
};

#[no_mangle]
pub static tglh_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    /* Tigerlake-H */
    cores_num: 2,
    init_core_mask: 1,
    host_managed_cores_mask: BIT(0),
    ipc_req: CNL_DSP_REG_HIPCIDR,
    ipc_req_mask: CNL_DSP_REG_HIPCIDR_BUSY,
    ipc_ack: CNL_DSP_REG_HIPCIDA,
    ipc_ack_mask: CNL_DSP_REG_HIPCIDA_DONE,
    ipc_ctl: CNL_DSP_REG_HIPCCTL,
    rom_status_reg: HDA_DSP_SRAM_REG_ROM_STATUS,
    rom_init_timeout: 300,
    ssp_count: TGL_SSP_COUNT,
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
    hw_ip_version: SOF_INTEL_CAVS_2_5,
    platform: b"tgl\0".as_ptr() as *const c_char,
};

#[no_mangle]
pub static ehl_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    /* Elkhartlake */
    cores_num: 4,
    init_core_mask: 1,
    host_managed_cores_mask: BIT(0),
    ipc_req: CNL_DSP_REG_HIPCIDR,
    ipc_req_mask: CNL_DSP_REG_HIPCIDR_BUSY,
    ipc_ack: CNL_DSP_REG_HIPCIDA,
    ipc_ack_mask: CNL_DSP_REG_HIPCIDA_DONE,
    ipc_ctl: CNL_DSP_REG_HIPCCTL,
    rom_status_reg: HDA_DSP_SRAM_REG_ROM_STATUS,
    rom_init_timeout: 300,
    ssp_count: TGL_SSP_COUNT,
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
    hw_ip_version: SOF_INTEL_CAVS_2_5,
    platform: b"ehl\0".as_ptr() as *const c_char,
};

#[no_mangle]
pub static adls_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    /* Alderlake-S */
    cores_num: 2,
    init_core_mask: BIT(0),
    host_managed_cores_mask: BIT(0),
    ipc_req: CNL_DSP_REG_HIPCIDR,
    ipc_req_mask: CNL_DSP_REG_HIPCIDR_BUSY,
    ipc_ack: CNL_DSP_REG_HIPCIDA,
    ipc_ack_mask: CNL_DSP_REG_HIPCIDA_DONE,
    ipc_ctl: CNL_DSP_REG_HIPCCTL,
    rom_status_reg: HDA_DSP_SRAM_REG_ROM_STATUS,
    rom_init_timeout: 300,
    ssp_count: TGL_SSP_COUNT,
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
    hw_ip_version: SOF_INTEL_CAVS_2_5,
    platform: b"adl\0".as_ptr() as *const c_char,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
