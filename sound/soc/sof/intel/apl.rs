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
 * Hardware interface for audio DSP on Apollolake and GeminiLake
 */

// Dependencies in the original C source:
// <sound/sof/ext_manifest4.h>
// "../ipc4-priv.h"
// "../sof-priv.h"
// "hda.h"
// "../sof-audio.h"

static apl_dsp_debugfs: [snd_sof_debugfs_map; 3] = [
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

/* apollolake ops */
#[no_mangle]
pub static mut sof_apl_ops: snd_sof_dsp_ops = unsafe { core::mem::zeroed() };

#[no_mangle]
pub unsafe extern "C" fn sof_apl_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    let mut ipc4_data: *mut sof_ipc4_fw_data;

    /* common defaults */
    memcpy(
        &mut sof_apl_ops as *mut snd_sof_dsp_ops as *mut c_void,
        &sof_hda_common_ops as *const snd_sof_dsp_ops as *const c_void,
        core::mem::size_of::<snd_sof_dsp_ops>(),
    );

    /* probe/remove/shutdown */
    sof_apl_ops.shutdown = Some(hda_dsp_shutdown);

    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_3 {
        /* doorbell */
        sof_apl_ops.irq_thread = Some(hda_dsp_ipc_irq_thread);

        /* ipc */
        sof_apl_ops.send_msg = Some(hda_dsp_ipc_send_msg);

        /* debug */
        sof_apl_ops.ipc_dump = Some(hda_ipc_dump);

        sof_apl_ops.set_power_state = Some(hda_dsp_set_power_state_ipc3);
    }

    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_4 {
        (*sdev).private = kzalloc_obj(core::mem::size_of::<sof_ipc4_fw_data>()) as *mut c_void;
        if (*sdev).private.is_null() {
            return -ENOMEM;
        }

        ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
        (*ipc4_data).manifest_fw_hdr_offset = SOF_MAN4_FW_HDR_OFFSET;

        (*ipc4_data).mtrace_type = SOF_IPC4_MTRACE_INTEL_CAVS_1_5;

        /* External library loading support */
        (*ipc4_data).load_library = Some(hda_dsp_ipc4_load_library);

        /* doorbell */
        sof_apl_ops.irq_thread = Some(hda_dsp_ipc4_irq_thread);

        /* ipc */
        sof_apl_ops.send_msg = Some(hda_dsp_ipc4_send_msg);

        /* debug */
        sof_apl_ops.ipc_dump = Some(hda_ipc4_dump);

        sof_apl_ops.set_power_state = Some(hda_dsp_set_power_state_ipc4);
    }

    /* set DAI driver ops */
    hda_set_dai_drv_ops(sdev, &mut sof_apl_ops);

    /* debug */
    sof_apl_ops.debug_map = apl_dsp_debugfs.as_ptr();
    sof_apl_ops.debug_map_count = apl_dsp_debugfs.len();

    /* firmware run */
    sof_apl_ops.run = Some(hda_dsp_cl_boot_firmware);

    /* pre/post fw run */
    sof_apl_ops.post_fw_run = Some(hda_dsp_post_fw_run);

    /* dsp core get/put */
    sof_apl_ops.core_get = Some(hda_dsp_core_get);

    return 0;
}

#[no_mangle]
pub static apl_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    /* Apollolake */
    cores_num: 2,
    init_core_mask: 1,
    host_managed_cores_mask: GENMASK(1, 0),
    ipc_req: HDA_DSP_REG_HIPCI,
    ipc_req_mask: HDA_DSP_REG_HIPCI_BUSY,
    ipc_ack: HDA_DSP_REG_HIPCIE,
    ipc_ack_mask: HDA_DSP_REG_HIPCIE_DONE,
    ipc_ctl: HDA_DSP_REG_HIPCCTL,
    rom_status_reg: HDA_DSP_SRAM_REG_ROM_STATUS,
    rom_init_timeout: 150,
    ssp_count: APL_SSP_COUNT,
    ssp_base_offset: APL_SSP_BASE_OFFSET,
    d0i3_offset: SOF_HDA_VS_D0I3C,
    quirks: SOF_INTEL_PROCEN_FMT_QUIRK,
    check_ipc_irq: Some(hda_dsp_check_ipc_irq),
    cl_init: Some(cl_dsp_init),
    power_down_dsp: Some(hda_power_down_dsp),
    disable_interrupts: Some(hda_dsp_disable_interrupts),
    hw_ip_version: SOF_INTEL_CAVS_1_5_PLUS,
    platform: b"apl\0".as_ptr() as *const c_char,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
