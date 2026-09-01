// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2020-2022 Intel Corporation

use core::ffi::c_int;

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

/* DSP Registers */
pub const MTL_HFDSSCS: u32 = 0x1000;
pub const MTL_HFDSSCS_SPA_MASK: u32 = BIT(16);
pub const MTL_HFDSSCS_CPA_MASK: u32 = BIT(24);
pub const MTL_HFSNDWIE: u32 = 0x114C;
pub const MTL_HFPWRCTL: u32 = 0x1D18;
pub const PTL_HFPWRCTL2: u32 = 0x1D20;
pub const fn MTL_HfPWRCTL_WPIOXPG(x: u32) -> u32 {
    BIT(x + 8)
}
pub const MTL_HFPWRCTL_WPDSPHPXPG: u32 = BIT(0);
pub const MTL_HFPWRSTS: u32 = 0x1D1C;
pub const PTL_HFPWRSTS2: u32 = 0x1D24;
pub const MTL_HFPWRSTS_DSPHPXPGS_MASK: u32 = BIT(0);
pub const MTL_HFINTIPPTR: u32 = 0x1108;
pub const MTL_IRQ_INTEN_L_HOST_IPC_MASK: u32 = BIT(0);
pub const MTL_IRQ_INTEN_L_SOUNDWIRE_MASK: u32 = BIT(6);
pub const MTL_HFINTIPPTR_PTR_MASK: u32 = GENMASK(20, 0);

pub const MTL_HDA_VS_D0I3C: u32 = 0x1D4A;

pub const MTL_DSP2CXCAP_PRIMARY_CORE: u32 = 0x178D00;
pub const MTL_DSP2CXCTL_PRIMARY_CORE: u32 = 0x178D04;
pub const MTL_DSP2CXCTL_PRIMARY_CORE_SPA_MASK: u32 = BIT(0);
pub const MTL_DSP2CXCTL_PRIMARY_CORE_CPA_MASK: u32 = BIT(8);
pub const MTL_DSP2CXCTL_PRIMARY_CORE_OSEL: u32 = GENMASK(25, 24);
pub const MTL_DSP2CXCTL_PRIMARY_CORE_OSEL_SHIFT: u32 = 24;

/* IPC Registers */
pub const MTL_DSP_REG_HFIPCXTDR: u32 = 0x73200;
pub const MTL_DSP_REG_HFIPCXTDR_BUSY: u32 = BIT(31);
pub const MTL_DSP_REG_HFIPCXTDR_MSG_MASK: u32 = GENMASK(30, 0);
pub const MTL_DSP_REG_HFIPCXTDA: u32 = 0x73204;
pub const MTL_DSP_REG_HFIPCXTDA_BUSY: u32 = BIT(31);
pub const MTL_DSP_REG_HFIPCXIDR: u32 = 0x73210;
pub const MTL_DSP_REG_HFIPCXIDR_BUSY: u32 = BIT(31);
pub const MTL_DSP_REG_HFIPCXIDR_MSG_MASK: u32 = GENMASK(30, 0);
pub const MTL_DSP_REG_HFIPCXIDA: u32 = 0x73214;
pub const MTL_DSP_REG_HFIPCXIDA_DONE: u32 = BIT(31);
pub const MTL_DSP_REG_HFIPCXIDA_MSG_MASK: u32 = GENMASK(30, 0);
pub const MTL_DSP_REG_HFIPCXCTL: u32 = 0x73228;
pub const MTL_DSP_REG_HFIPCXCTL_BUSY: u32 = BIT(0);
pub const MTL_DSP_REG_HFIPCXCTL_DONE: u32 = BIT(1);
pub const MTL_DSP_REG_HFIPCXTDDY: u32 = 0x73300;
pub const MTL_DSP_REG_HFIPCXIDDY: u32 = 0x73380;
pub const MTL_DSP_REG_HfHIPCIE: u32 = 0x1140;
pub const MTL_DSP_REG_HfHIPCIE_IE_MASK: u32 = BIT(0);
pub const MTL_DSP_REG_HfSNDWIE: u32 = 0x114C;
pub const MTL_DSP_REG_HfSNDWIE_IE_MASK: u32 = GENMASK(3, 0);

pub const MTL_DSP_IRQSTS: u32 = 0x20;
pub const MTL_DSP_IRQSTS_IPC: u32 = BIT(0);
pub const MTL_DSP_IRQSTS_SDW: u32 = BIT(6);

pub const MTL_DSP_REG_POLL_INTERVAL_US: u32 = 10; /* 10 us */

/* Memory windows */
pub const fn MTL_SRAM_WINDOW_OFFSET(x: u32) -> u32 {
    0x180000 + 0x8000 * x
}

pub const MTL_DSP_MBOX_UPLINK_OFFSET: u32 = MTL_SRAM_WINDOW_OFFSET(0) + 0x1000;
pub const MTL_DSP_MBOX_UPLINK_SIZE: u32 = 0x1000;
pub const MTL_DSP_MBOX_DOWNLINK_OFFSET: u32 = MTL_SRAM_WINDOW_OFFSET(1);
pub const MTL_DSP_MBOX_DOWNLINK_SIZE: u32 = 0x1000;

/* FW registers */
pub const MTL_DSP_ROM_STS: u32 = MTL_SRAM_WINDOW_OFFSET(0); /* ROM status */
pub const MTL_DSP_ROM_ERROR: u32 = MTL_SRAM_WINDOW_OFFSET(0) + 0x4; /* ROM error code */

pub const MTL_DSP_REG_HFFLGPXQWY: u32 = 0x163200; /* DSP core0 status */
pub const MTL_DSP_REG_HFFLGPXQWY_ERROR: u32 = 0x163204; /* DSP core0 error */

/* FSR status codes */
pub const FSR_STATE_ROM_RESET_VECTOR_DONE: u32 = 0x8;
pub const FSR_STATE_ROM_PURGE_BOOT: u32 = 0x9;
pub const FSR_STATE_ROM_RESTORE_BOOT: u32 = 0xA;
pub const FSR_STATE_ROM_FW_ENTRY_POINT: u32 = 0xB;
pub const FSR_STATE_ROM_VALIDATE_PUB_KEY: u32 = 0xC;
pub const FSR_STATE_ROM_POWER_DOWN_HPSRAM: u32 = 0xD;
pub const FSR_STATE_ROM_POWER_DOWN_ULPSRAM: u32 = 0xE;
pub const FSR_STATE_ROM_POWER_UP_ULPSRAM_STACK: u32 = 0xF;
pub const FSR_STATE_ROM_POWER_UP_HPSRAM_DMA: u32 = 0x10;
pub const FSR_STATE_ROM_BEFORE_EP_POINTER_READ: u32 = 0x11;
pub const FSR_STATE_ROM_VALIDATE_MANIFEST: u32 = 0x12;
pub const FSR_STATE_ROM_VALIDATE_FW_MODULE: u32 = 0x13;
pub const FSR_STATE_ROM_PROTECT_IMR_REGION: u32 = 0x14;
pub const FSR_STATE_ROM_PUSH_MODEL_ROUTINE: u32 = 0x15;
pub const FSR_STATE_ROM_PULL_MODEL_ROUTINE: u32 = 0x16;
pub const FSR_STATE_ROM_VALIDATE_PKG_DIR: u32 = 0x17;
pub const FSR_STATE_ROM_VALIDATE_CPD: u32 = 0x18;
pub const FSR_STATE_ROM_VALIDATE_CSS_MAN_HEADER: u32 = 0x19;
pub const FSR_STATE_ROM_VALIDATE_BLOB_SVN: u32 = 0x1A;
pub const FSR_STATE_ROM_VERIFY_IFWI_PARTITION: u32 = 0x1B;
pub const FSR_STATE_ROM_REMOVE_ACCESS_CONTROL: u32 = 0x1C;
pub const FSR_STATE_ROM_AUTH_BYPASS: u32 = 0x1D;
pub const FSR_STATE_ROM_AUTH_ENABLED: u32 = 0x1E;
pub const FSR_STATE_ROM_INIT_DMA: u32 = 0x1F;
pub const FSR_STATE_ROM_PURGE_FW_ENTRY: u32 = 0x20;
pub const FSR_STATE_ROM_PURGE_FW_END: u32 = 0x21;
pub const FSR_STATE_ROM_CLEAN_UP_BSS_DONE: u32 = 0x22;
pub const FSR_STATE_ROM_IMR_RESTORE_ENTRY: u32 = 0x23;
pub const FSR_STATE_ROM_IMR_RESTORE_END: u32 = 0x24;
pub const FSR_STATE_ROM_FW_MANIFEST_IN_DMA_BUFF: u32 = 0x25;
pub const FSR_STATE_ROM_LOAD_CSE_MAN_TO_IMR: u32 = 0x26;
pub const FSR_STATE_ROM_LOAD_FW_MAN_TO_IMR: u32 = 0x27;
pub const FSR_STATE_ROM_LOAD_FW_CODE_TO_IMR: u32 = 0x28;
pub const FSR_STATE_ROM_FW_LOADING_DONE: u32 = 0x29;
pub const FSR_STATE_ROM_FW_CODE_LOADED: u32 = 0x2A;
pub const FSR_STATE_ROM_VERIFY_IMAGE_TYPE: u32 = 0x2B;
pub const FSR_STATE_ROM_AUTH_API_INIT: u32 = 0x2C;
pub const FSR_STATE_ROM_AUTH_API_PROC: u32 = 0x2D;
pub const FSR_STATE_ROM_AUTH_API_FIRST_BUSY: u32 = 0x2E;
pub const FSR_STATE_ROM_AUTH_API_FIRST_RESULT: u32 = 0x2F;
pub const FSR_STATE_ROM_AUTH_API_CLEANUP: u32 = 0x30;

pub const MTL_DSP_REG_HfIMRIS1: u32 = 0x162088;
pub const MTL_DSP_REG_HfIMRIS1_IU_MASK: u32 = BIT(0);

unsafe extern "C" {
    pub fn mtl_dsp_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool;

    pub fn mtl_enable_ipc_interrupts(sdev: *mut snd_sof_dev);
    pub fn mtl_disable_ipc_interrupts(sdev: *mut snd_sof_dev);

    pub fn mtl_enable_interrupts(sdev: *mut snd_sof_dev, enable: bool) -> c_int;

    pub fn mtl_power_down_dsp(sdev: *mut snd_sof_dev) -> c_int;
    pub fn mtl_dsp_cl_init(sdev: *mut snd_sof_dev, stream_tag: c_int, imr_boot: bool) -> c_int;

    pub fn sof_mtl_set_ops(sdev: *mut snd_sof_dev, dsp_ops: *mut snd_sof_dsp_ops) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
