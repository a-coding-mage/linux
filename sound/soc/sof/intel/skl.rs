// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2022 Intel Corporation
//

/*
 * Hardware interface for audio DSP on Skylake and Kabylake.
 */

// C include dependencies translated as external Rust dependencies:
// linux/delay.h, linux/device.h, linux/dma-mapping.h, linux/firmware.h,
// linux/fs.h, linux/interrupt.h, linux/module.h, linux/slab.h, linux/pci.h,
// sound/hdaudio_ext.h, sound/pcm_params.h, sound/sof.h,
// sound/sof/ext_manifest4.h, ../sof-priv.h, ../ipc4-priv.h, ../ops.h,
// hda.h, ../sof-audio.h.

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

const SRAM_MEMORY_WINDOW_BASE: u32 = 0x8000;

const ENOMEM: c_int = 12;

const HDA_DSP_HDA_BAR: u32 = 0;
const HDA_DSP_PP_BAR: u32 = 0;
const HDA_DSP_BAR: u32 = 0;
const SOF_MAN4_FW_HDR_OFFSET_CAVS_1_5: u32 = 0;
const SOF_IPC4_MTRACE_INTEL_CAVS_1_5: u32 = 0;
const HDA_DSP_REG_HIPCI: u32 = 0;
const HDA_DSP_REG_HIPCI_BUSY: u32 = 0;
const HDA_DSP_REG_HIPCIE: u32 = 0;
const HDA_DSP_REG_HIPCIE_DONE: u32 = 0;
const HDA_DSP_REG_HIPCCTL: u32 = 0;
const HDA_DSP_SRAM_REG_ROM_STATUS_SKL: u32 = 0;
const SOF_INTEL_CAVS_1_5: u32 = 0;

const fn genmask(h: u32, l: u32) -> u32 {
    let high = if h == 31 {
        u32::MAX
    } else {
        (1u32 << (h + 1)) - 1
    };
    let low = if l == 0 { 0 } else { (1u32 << l) - 1 };
    high & !low
}

#[repr(C)]
pub struct snd_sof_debugfs_map {
    pub name: *const c_char,
    pub bar: u32,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc4_fw_data {
    pub manifest_fw_hdr_offset: u32,
    pub mtrace_type: u32,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub get_window_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
    pub get_mailbox_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub irq_thread: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    pub send_msg: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void) -> c_int>,
    pub debug_map: *const snd_sof_debugfs_map,
    pub debug_map_count: usize,
    pub ipc_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub run: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub post_fw_run: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub cores_num: u32,
    pub init_core_mask: u32,
    pub host_managed_cores_mask: u32,
    pub ipc_req: u32,
    pub ipc_req_mask: u32,
    pub ipc_ack: u32,
    pub ipc_ack_mask: u32,
    pub ipc_ctl: u32,
    pub rom_status_reg: u32,
    pub rom_init_timeout: u32,
    pub check_ipc_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> bool>,
    pub power_down_dsp: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub disable_interrupts: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub hw_ip_version: u32,
    pub platform: *const c_char,
}

unsafe extern "C" {
    static sof_hda_common_ops: snd_sof_dsp_ops;

    fn hda_dsp_shutdown(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_dsp_ipc4_irq_thread(irq: c_int, context: *mut c_void) -> c_int;
    fn hda_dsp_ipc4_send_msg(sdev: *mut snd_sof_dev, msg: *mut c_void) -> c_int;
    fn hda_set_dai_drv_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops);
    fn hda_ipc4_dump(sdev: *mut snd_sof_dev);
    fn hda_dsp_cl_boot_firmware_skl(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_dsp_post_fw_run(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_dsp_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool;
    fn hda_power_down_dsp(sdev: *mut snd_sof_dev) -> c_int;
    fn hda_dsp_disable_interrupts(sdev: *mut snd_sof_dev);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
}

static SKL_DSP_DEBUGFS_HDA: &[u8] = b"hda\0";
static SKL_DSP_DEBUGFS_PP: &[u8] = b"pp\0";
static SKL_DSP_DEBUGFS_DSP: &[u8] = b"dsp\0";
static SKL_PLATFORM: &[u8] = b"skl\0";

static skl_dsp_debugfs: [snd_sof_debugfs_map; 3] = [
    snd_sof_debugfs_map {
        name: SKL_DSP_DEBUGFS_HDA.as_ptr() as *const c_char,
        bar: HDA_DSP_HDA_BAR,
        offset: 0,
        size: 0x4000,
    },
    snd_sof_debugfs_map {
        name: SKL_DSP_DEBUGFS_PP.as_ptr() as *const c_char,
        bar: HDA_DSP_PP_BAR,
        offset: 0,
        size: 0x1000,
    },
    snd_sof_debugfs_map {
        name: SKL_DSP_DEBUGFS_DSP.as_ptr() as *const c_char,
        bar: HDA_DSP_BAR,
        offset: 0,
        size: 0x10000,
    },
];

unsafe extern "C" fn skl_dsp_ipc_get_window_offset(
    _sdev: *mut snd_sof_dev,
    id: u32,
) -> c_int {
    SRAM_MEMORY_WINDOW_BASE.wrapping_add(0x2000u32.wrapping_mul(id)) as c_int
}

unsafe extern "C" fn skl_dsp_ipc_get_mailbox_offset(_sdev: *mut snd_sof_dev) -> c_int {
    SRAM_MEMORY_WINDOW_BASE.wrapping_add(0x1000) as c_int
}

/* skylake ops */
#[unsafe(no_mangle)]
pub static mut sof_skl_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    shutdown: None,
    get_window_offset: None,
    get_mailbox_offset: None,
    irq_thread: None,
    send_msg: None,
    debug_map: ptr::null(),
    debug_map_count: 0,
    ipc_dump: None,
    run: None,
    post_fw_run: None,
};
// EXPORT_SYMBOL_NS(sof_skl_ops, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_skl_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    let mut ipc4_data: *mut sof_ipc4_fw_data;

    /* common defaults */
    ptr::copy_nonoverlapping(
        &sof_hda_common_ops as *const snd_sof_dsp_ops,
        &raw mut sof_skl_ops,
        1,
    );

    /* probe/remove/shutdown */
    sof_skl_ops.shutdown = Some(hda_dsp_shutdown);

    (*sdev).private = kzalloc(mem::size_of::<sof_ipc4_fw_data>(), 0);
    if (*sdev).private.is_null() {
        return -ENOMEM;
    }

    ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    (*ipc4_data).manifest_fw_hdr_offset = SOF_MAN4_FW_HDR_OFFSET_CAVS_1_5;

    (*ipc4_data).mtrace_type = SOF_IPC4_MTRACE_INTEL_CAVS_1_5;

    sof_skl_ops.get_window_offset = Some(skl_dsp_ipc_get_window_offset);
    sof_skl_ops.get_mailbox_offset = Some(skl_dsp_ipc_get_mailbox_offset);

    /* doorbell */
    sof_skl_ops.irq_thread = Some(hda_dsp_ipc4_irq_thread);

    /* ipc */
    sof_skl_ops.send_msg = Some(hda_dsp_ipc4_send_msg);

    /* set DAI driver ops */
    hda_set_dai_drv_ops(sdev, &raw mut sof_skl_ops);

    /* debug */
    sof_skl_ops.debug_map = skl_dsp_debugfs.as_ptr();
    sof_skl_ops.debug_map_count = skl_dsp_debugfs.len();
    sof_skl_ops.ipc_dump = Some(hda_ipc4_dump);

    /* firmware run */
    sof_skl_ops.run = Some(hda_dsp_cl_boot_firmware_skl);

    /* pre/post fw run */
    sof_skl_ops.post_fw_run = Some(hda_dsp_post_fw_run);

    return 0;
}
// EXPORT_SYMBOL_NS(sof_skl_ops_init, "SND_SOC_SOF_INTEL_HDA_COMMON");

#[unsafe(no_mangle)]
pub static skl_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    cores_num: 2,
    init_core_mask: 1,
    host_managed_cores_mask: genmask(1, 0),
    ipc_req: HDA_DSP_REG_HIPCI,
    ipc_req_mask: HDA_DSP_REG_HIPCI_BUSY,
    ipc_ack: HDA_DSP_REG_HIPCIE,
    ipc_ack_mask: HDA_DSP_REG_HIPCIE_DONE,
    ipc_ctl: HDA_DSP_REG_HIPCCTL,
    rom_status_reg: HDA_DSP_SRAM_REG_ROM_STATUS_SKL,
    rom_init_timeout: 300,
    check_ipc_irq: Some(hda_dsp_check_ipc_irq),
    power_down_dsp: Some(hda_power_down_dsp),
    disable_interrupts: Some(hda_dsp_disable_interrupts),
    hw_ip_version: SOF_INTEL_CAVS_1_5,
    platform: SKL_PLATFORM.as_ptr() as *const c_char,
};
// EXPORT_SYMBOL_NS(skl_chip_info, "SND_SOC_SOF_INTEL_HDA_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
