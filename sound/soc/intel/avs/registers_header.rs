/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2021-2022 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

// C includes translated as dependency intent:
// linux/io-64-nonatomic-lo-hi.h
// linux/iopoll.h
// linux/sizes.h

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

pub const AZX_PCIREG_PGCTL: u32 = 0x44;
pub const AZX_PCIREG_CGCTL: u32 = 0x48;
pub const AZX_PGCTL_LSRMD_MASK: u32 = BIT(4);
pub const AZX_CGCTL_MISCBDCGE_MASK: u32 = BIT(6);
pub const AZX_VS_EM2_L1SEN: u32 = BIT(13);
pub const AZX_VS_EM2_DUM: u32 = BIT(23);

/* Intel HD Audio General DSP Registers */
pub const AVS_ADSP_GEN_BASE: u32 = 0x0;
pub const AVS_ADSP_REG_ADSPCS: u32 = AVS_ADSP_GEN_BASE + 0x04;
pub const AVS_ADSP_REG_ADSPIC: u32 = AVS_ADSP_GEN_BASE + 0x08;
pub const AVS_ADSP_REG_ADSPIS: u32 = AVS_ADSP_GEN_BASE + 0x0C;

pub const AVS_ADSP_ADSPIC_IPC: u32 = BIT(0);
pub const AVS_ADSP_ADSPIC_CLDMA: u32 = BIT(1);
pub const AVS_ADSP_ADSPIS_IPC: u32 = BIT(0);
pub const AVS_ADSP_ADSPIS_CLDMA: u32 = BIT(1);

pub const fn AVS_ADSPCS_CRST_MASK(cm: u32) -> u32 {
    cm
}

pub const fn AVS_ADSPCS_CSTALL_MASK(cm: u32) -> u32 {
    cm << 8
}

pub const fn AVS_ADSPCS_SPA_MASK(cm: u32) -> u32 {
    cm << 16
}

pub const fn AVS_ADSPCS_CPA_MASK(cm: u32) -> u32 {
    cm << 24
}

pub const AVS_ADSPCS_INTERVAL_US: u32 = 500;
pub const AVS_ADSPCS_TIMEOUT_US: u32 = 10000;
pub const AVS_MAIN_CORE_MASK: u32 = BIT(0);

pub const AVS_ADSP_HIPCCTL_BUSY: u32 = BIT(0);
pub const AVS_ADSP_HIPCCTL_DONE: u32 = BIT(1);

/* SKL Intel HD Audio Inter-Processor Communication Registers */
pub const SKL_ADSP_IPC_BASE: u32 = 0x40;
pub const SKL_ADSP_REG_HIPCT: u32 = SKL_ADSP_IPC_BASE + 0x00;
pub const SKL_ADSP_REG_HIPCTE: u32 = SKL_ADSP_IPC_BASE + 0x04;
pub const SKL_ADSP_REG_HIPCI: u32 = SKL_ADSP_IPC_BASE + 0x08;
pub const SKL_ADSP_REG_HIPCIE: u32 = SKL_ADSP_IPC_BASE + 0x0C;
pub const SKL_ADSP_REG_HIPCCTL: u32 = SKL_ADSP_IPC_BASE + 0x10;

pub const SKL_ADSP_HIPCI_BUSY: u32 = BIT(31);
pub const SKL_ADSP_HIPCIE_DONE: u32 = BIT(30);
pub const SKL_ADSP_HIPCT_BUSY: u32 = BIT(31);

/* CNL Intel HD Audio Inter-Processor Communication Registers */
pub const CNL_ADSP_IPC_BASE: u32 = 0xC0;
pub const CNL_ADSP_REG_HIPCTDR: u32 = CNL_ADSP_IPC_BASE + 0x00;
pub const CNL_ADSP_REG_HIPCTDA: u32 = CNL_ADSP_IPC_BASE + 0x04;
pub const CNL_ADSP_REG_HIPCTDD: u32 = CNL_ADSP_IPC_BASE + 0x08;
pub const CNL_ADSP_REG_HIPCIDR: u32 = CNL_ADSP_IPC_BASE + 0x10;
pub const CNL_ADSP_REG_HIPCIDA: u32 = CNL_ADSP_IPC_BASE + 0x14;
pub const CNL_ADSP_REG_HIPCIDD: u32 = CNL_ADSP_IPC_BASE + 0x18;
pub const CNL_ADSP_REG_HIPCCTL: u32 = CNL_ADSP_IPC_BASE + 0x28;

pub const CNL_ADSP_HIPCTDR_BUSY: u32 = BIT(31);
pub const CNL_ADSP_HIPCTDA_DONE: u32 = BIT(31);
pub const CNL_ADSP_HIPCIDR_BUSY: u32 = BIT(31);
pub const CNL_ADSP_HIPCIDA_DONE: u32 = BIT(31);

/* MTL Intel HOST Inter-Processor Communication Registers */
pub const MTL_HfIPC_BASE: u32 = 0x73000;
pub const MTL_REG_HfIPCxTDR: u32 = MTL_HfIPC_BASE + 0x200;
pub const MTL_REG_HfIPCxTDA: u32 = MTL_HfIPC_BASE + 0x204;
pub const MTL_REG_HfIPCxIDR: u32 = MTL_HfIPC_BASE + 0x210;
pub const MTL_REG_HfIPCxIDA: u32 = MTL_HfIPC_BASE + 0x214;
pub const MTL_REG_HfIPCxCTL: u32 = MTL_HfIPC_BASE + 0x228;
pub const MTL_REG_HfIPCxTDD: u32 = MTL_HfIPC_BASE + 0x300;
pub const MTL_REG_HfIPCxIDD: u32 = MTL_HfIPC_BASE + 0x380;

pub const MTL_HfIPCxTDR_BUSY: u32 = BIT(31);
pub const MTL_HfIPCxTDA_BUSY: u32 = BIT(31);
pub const MTL_HfIPCxIDR_BUSY: u32 = BIT(31);
pub const MTL_HfIPCxIDA_DONE: u32 = BIT(31);

pub const MTL_HfFLV_BASE: u32 = 0x162000;

pub const fn MTL_REG_HfFLGP(x: u32, y: u32) -> u32 {
    MTL_HfFLV_BASE + 0x1200 + x * 0x20 + y * 0x08
}

pub const fn LNL_REG_HfDFR(x: u32) -> u32 {
    0x160200 + x * 0x8
}

pub const MTL_DWICTL_BASE: u32 = 0x1800;
pub const MTL_DWICTL_REG_INTENL: u32 = MTL_DWICTL_BASE + 0x0;
pub const MTL_DWICTL_REG_FINALSTATUSL: u32 = MTL_DWICTL_BASE + 0x30;

pub const MTL_HfPMCCU_BASE: u32 = 0x1D00;
pub const MTL_REG_HfCLKCTL: u32 = MTL_HfPMCCU_BASE + 0x10;
pub const MTL_REG_HfPWRCTL: u32 = MTL_HfPMCCU_BASE + 0x18;
pub const MTL_REG_HfPWRSTS: u32 = MTL_HfPMCCU_BASE + 0x1C;
pub const MTL_REG_HfPWRCTL2: u32 = MTL_HfPMCCU_BASE + 0x20;
pub const MTL_REG_HfPWRSTS2: u32 = MTL_HfPMCCU_BASE + 0x24;
pub const MTL_HfPWRCTL_WPDSPHPxPG: u32 = BIT(0);
pub const MTL_HfPWRSTS_DSPHPxPGS: u32 = BIT(0);
pub const MTL_HfPWRCTL2_WPDSPHPxPG: u32 = BIT(0);
pub const MTL_HfPWRSTS2_DSPHPxPGS: u32 = BIT(0);

/* Intel HD Audio SRAM windows base addresses */
pub const SKL_ADSP_SRAM_BASE_OFFSET: u32 = 0x8000;
pub const SKL_ADSP_SRAM_WINDOW_SIZE: u32 = 0x2000;
pub const APL_ADSP_SRAM_BASE_OFFSET: u32 = 0x80000;
pub const APL_ADSP_SRAM_WINDOW_SIZE: u32 = 0x20000;
pub const MTL_ADSP_SRAM_BASE_OFFSET: u32 = 0x180000;
pub const MTL_ADSP_SRAM_WINDOW_SIZE: u32 = 0x8000;

/* Constants used when accessing SRAM, space shared with firmware */
pub unsafe fn AVS_FW_REG_BASE(adev: *mut avs_dev) -> u32 {
    unsafe { (*(*(*adev).spec).hipc).sts_offset }
}

pub unsafe fn AVS_FW_REG_STATUS(adev: *mut avs_dev) -> u32 {
    unsafe { AVS_FW_REG_BASE(adev) + 0x0 }
}

pub unsafe fn AVS_FW_REG_ERROR(adev: *mut avs_dev) -> u32 {
    unsafe { AVS_FW_REG_BASE(adev) + 0x4 }
}

pub const AVS_WINDOW_CHUNK_SIZE: u32 = SZ_4K;
pub const AVS_FW_REGS_SIZE: u32 = AVS_WINDOW_CHUNK_SIZE;
pub const AVS_FW_REGS_WINDOW: u32 = 0;
/* DSP -> HOST communication window */
pub const AVS_UPLINK_WINDOW: u32 = AVS_FW_REGS_WINDOW;
/* HOST -> DSP communication window */
pub const AVS_DOWNLINK_WINDOW: u32 = 1;
pub const AVS_DEBUG_WINDOW: u32 = 2;

/* registry I/O helpers */
pub unsafe fn avs_sram_offset(adev: *mut avs_dev, window_idx: u32) -> u32 {
    unsafe {
        (*(*(*adev).spec).sram)
            .base_offset
            .wrapping_add((*(*(*adev).spec).sram).window_size.wrapping_mul(window_idx))
    }
}

pub unsafe fn avs_sram_addr(adev: *mut avs_dev, window_idx: u32) -> *mut core::ffi::c_void {
    unsafe { (*adev).dsp_ba.byte_add(avs_sram_offset(adev, window_idx) as usize) }
}

pub unsafe fn avs_uplink_addr(adev: *mut avs_dev) -> *mut core::ffi::c_void {
    unsafe { avs_sram_addr(adev, AVS_UPLINK_WINDOW).byte_add(AVS_FW_REGS_SIZE as usize) }
}

pub unsafe fn avs_downlink_addr(adev: *mut avs_dev) -> *mut core::ffi::c_void {
    unsafe { avs_sram_addr(adev, AVS_DOWNLINK_WINDOW) }
}

pub unsafe fn snd_hdac_adsp_writeb(adev: *mut avs_dev, reg: u32, value: u8) {
    unsafe {
        snd_hdac_reg_writeb(
            &mut (*adev).base.core,
            (*adev).dsp_ba.byte_add(reg as usize),
            value,
        )
    }
}

pub unsafe fn snd_hdac_adsp_readb(adev: *mut avs_dev, reg: u32) -> u8 {
    unsafe { snd_hdac_reg_readb(&mut (*adev).base.core, (*adev).dsp_ba.byte_add(reg as usize)) }
}

pub unsafe fn snd_hdac_adsp_writew(adev: *mut avs_dev, reg: u32, value: u16) {
    unsafe {
        snd_hdac_reg_writew(
            &mut (*adev).base.core,
            (*adev).dsp_ba.byte_add(reg as usize),
            value,
        )
    }
}

pub unsafe fn snd_hdac_adsp_readw(adev: *mut avs_dev, reg: u32) -> u16 {
    unsafe { snd_hdac_reg_readw(&mut (*adev).base.core, (*adev).dsp_ba.byte_add(reg as usize)) }
}

pub unsafe fn snd_hdac_adsp_writel(adev: *mut avs_dev, reg: u32, value: u32) {
    unsafe {
        snd_hdac_reg_writel(
            &mut (*adev).base.core,
            (*adev).dsp_ba.byte_add(reg as usize),
            value,
        )
    }
}

pub unsafe fn snd_hdac_adsp_readl(adev: *mut avs_dev, reg: u32) -> u32 {
    unsafe { snd_hdac_reg_readl(&mut (*adev).base.core, (*adev).dsp_ba.byte_add(reg as usize)) }
}

pub unsafe fn snd_hdac_adsp_writeq(adev: *mut avs_dev, reg: u32, value: u64) {
    unsafe {
        snd_hdac_reg_writeq(
            &mut (*adev).base.core,
            (*adev).dsp_ba.byte_add(reg as usize),
            value,
        )
    }
}

pub unsafe fn snd_hdac_adsp_readq(adev: *mut avs_dev, reg: u32) -> u64 {
    unsafe { snd_hdac_reg_readq(&mut (*adev).base.core, (*adev).dsp_ba.byte_add(reg as usize)) }
}

pub unsafe fn snd_hdac_adsp_updateb(adev: *mut avs_dev, reg: u32, mask: u8, val: u8) {
    unsafe { snd_hdac_adsp_writeb(adev, reg, (snd_hdac_adsp_readb(adev, reg) & !mask) | val) }
}

pub unsafe fn snd_hdac_adsp_updatew(adev: *mut avs_dev, reg: u32, mask: u16, val: u16) {
    unsafe { snd_hdac_adsp_writew(adev, reg, (snd_hdac_adsp_readw(adev, reg) & !mask) | val) }
}

pub unsafe fn snd_hdac_adsp_updatel(adev: *mut avs_dev, reg: u32, mask: u32, val: u32) {
    unsafe { snd_hdac_adsp_writel(adev, reg, (snd_hdac_adsp_readl(adev, reg) & !mask) | val) }
}

pub unsafe fn snd_hdac_adsp_updateq(adev: *mut avs_dev, reg: u32, mask: u64, val: u64) {
    unsafe { snd_hdac_adsp_writeq(adev, reg, (snd_hdac_adsp_readq(adev, reg) & !mask) | val) }
}

// C polling macros preserved as dependency intent. These rely on C macro
// substitution of `val` and `cond` and map to the Linux iopoll helpers:
// snd_hdac_adsp_readb_poll(adev, reg, val, cond, delay_us, timeout_us)
//     readb_poll_timeout((adev)->dsp_ba + (reg), val, cond, delay_us, timeout_us)
// snd_hdac_adsp_readw_poll(adev, reg, val, cond, delay_us, timeout_us)
//     readw_poll_timeout((adev)->dsp_ba + (reg), val, cond, delay_us, timeout_us)
// snd_hdac_adsp_readl_poll(adev, reg, val, cond, delay_us, timeout_us)
//     readl_poll_timeout((adev)->dsp_ba + (reg), val, cond, delay_us, timeout_us)
// snd_hdac_adsp_readq_poll(adev, reg, val, cond, delay_us, timeout_us)
//     readq_poll_timeout((adev)->dsp_ba + (reg), val, cond, delay_us, timeout_us)

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
