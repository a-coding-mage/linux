/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (c) 2021 MediaTek Corporation. All rights reserved.
 *
 *  Header file for the mt8195 DSP register definition
 */

#[repr(C)]
pub struct mtk_adsp_chip_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const DSP_REG_BASE: u32 = 0x10803000;
pub const SCP_CFGREG_BASE: u32 = 0x10724000;
pub const DSP_SYSAO_BASE: u32 = 0x1080C000;

/*****************************************************************************
 *                  R E G I S T E R       TABLE
 *****************************************************************************/
pub const DSP_JTAGMUX: u32 = 0x0000;
pub const DSP_ALTRESETVEC: u32 = 0x0004;
pub const DSP_PDEBUGDATA: u32 = 0x0008;
pub const DSP_PDEBUGBUS0: u32 = 0x000c;
pub const PDEBUG_ENABLE: u32 = BIT(0);
pub const DSP_PDEBUGBUS1: u32 = 0x0010;
pub const DSP_PDEBUGINST: u32 = 0x0014;
pub const DSP_PDEBUGLS0STAT: u32 = 0x0018;
pub const DSP_PDEBUGLS1STAT: u32 = 0x001c;
pub const DSP_PDEBUGPC: u32 = 0x0020;
pub const DSP_RESET_SW: u32 = 0x0024; /*reset sw*/
pub const ADSP_BRESET_SW: u32 = BIT(0);
pub const ADSP_DRESET_SW: u32 = BIT(1);
pub const ADSP_RUNSTALL: u32 = BIT(3);
pub const STATVECTOR_SEL: u32 = BIT(4);
pub const ADSP_PWAIT: u32 = BIT(16);
pub const DSP_PFAULTBUS: u32 = 0x0028;
pub const DSP_PFAULTINFO: u32 = 0x002c;
pub const DSP_GPR00: u32 = 0x0030;
pub const DSP_GPR01: u32 = 0x0034;
pub const DSP_GPR02: u32 = 0x0038;
pub const DSP_GPR03: u32 = 0x003c;
pub const DSP_GPR04: u32 = 0x0040;
pub const DSP_GPR05: u32 = 0x0044;
pub const DSP_GPR06: u32 = 0x0048;
pub const DSP_GPR07: u32 = 0x004c;
pub const DSP_GPR08: u32 = 0x0050;
pub const DSP_GPR09: u32 = 0x0054;
pub const DSP_GPR0A: u32 = 0x0058;
pub const DSP_GPR0B: u32 = 0x005c;
pub const DSP_GPR0C: u32 = 0x0060;
pub const DSP_GPR0D: u32 = 0x0064;
pub const DSP_GPR0E: u32 = 0x0068;
pub const DSP_GPR0F: u32 = 0x006c;
pub const DSP_GPR10: u32 = 0x0070;
pub const DSP_GPR11: u32 = 0x0074;
pub const DSP_GPR12: u32 = 0x0078;
pub const DSP_GPR13: u32 = 0x007c;
pub const DSP_GPR14: u32 = 0x0080;
pub const DSP_GPR15: u32 = 0x0084;
pub const DSP_GPR16: u32 = 0x0088;
pub const DSP_GPR17: u32 = 0x008c;
pub const DSP_GPR18: u32 = 0x0090;
pub const DSP_GPR19: u32 = 0x0094;
pub const DSP_GPR1A: u32 = 0x0098;
pub const DSP_GPR1B: u32 = 0x009c;
pub const DSP_GPR1C: u32 = 0x00a0;
pub const DSP_GPR1D: u32 = 0x00a4;
pub const DSP_GPR1E: u32 = 0x00a8;
pub const DSP_GPR1F: u32 = 0x00ac;
pub const DSP_TCM_OFFSET: u32 = 0x00b0; /* not used */
pub const DSP_DDR_OFFSET: u32 = 0x00b4; /* not used */
pub const DSP_INTFDSP: u32 = 0x00d0;
pub const DSP_INTFDSP_CLR: u32 = 0x00d4;
pub const DSP_SRAM_PD_SW1: u32 = 0x00d8;
pub const DSP_SRAM_PD_SW2: u32 = 0x00dc;
pub const DSP_OCD: u32 = 0x00e0;
pub const DSP_RG_DSP_IRQ_POL: u32 = 0x00f0; /* not used */
pub const DSP_DSP_IRQ_EN: u32 = 0x00f4; /* not used */
pub const DSP_DSP_IRQ_LEVEL: u32 = 0x00f8; /* not used */
pub const DSP_DSP_IRQ_STATUS: u32 = 0x00fc; /* not used */
pub const DSP_RG_INT2CIRQ: u32 = 0x0114;
pub const DSP_RG_INT_POL_CTL0: u32 = 0x0120;
pub const DSP_RG_INT_EN_CTL0: u32 = 0x0130;
pub const DSP_RG_INT_LV_CTL0: u32 = 0x0140;
pub const DSP_RG_INT_STATUS0: u32 = 0x0150;
pub const DSP_PDEBUGSTATUS0: u32 = 0x0200;
pub const DSP_PDEBUGSTATUS1: u32 = 0x0204;
pub const DSP_PDEBUGSTATUS2: u32 = 0x0208;
pub const DSP_PDEBUGSTATUS3: u32 = 0x020c;
pub const DSP_PDEBUGSTATUS4: u32 = 0x0210;
pub const DSP_PDEBUGSTATUS5: u32 = 0x0214;
pub const DSP_PDEBUGSTATUS6: u32 = 0x0218;
pub const DSP_PDEBUGSTATUS7: u32 = 0x021c;
pub const DSP_DSP2PSRAM_PRIORITY: u32 = 0x0220; /* not used */
pub const DSP_AUDIO_DSP2SPM_INT: u32 = 0x0224;
pub const DSP_AUDIO_DSP2SPM_INT_ACK: u32 = 0x0228;
pub const DSP_AUDIO_DSP_DEBUG_SEL: u32 = 0x022C;
pub const DSP_AUDIO_DSP_EMI_BASE_ADDR: u32 = 0x02E0; /* not used */
pub const DSP_AUDIO_DSP_SHARED_IRAM: u32 = 0x02E4;
pub const DSP_AUDIO_DSP_CKCTRL_P2P_CK_CON: u32 = 0x02F0;
pub const DSP_RG_SEMAPHORE00: u32 = 0x0300;
pub const DSP_RG_SEMAPHORE01: u32 = 0x0304;
pub const DSP_RG_SEMAPHORE02: u32 = 0x0308;
pub const DSP_RG_SEMAPHORE03: u32 = 0x030C;
pub const DSP_RG_SEMAPHORE04: u32 = 0x0310;
pub const DSP_RG_SEMAPHORE05: u32 = 0x0314;
pub const DSP_RG_SEMAPHORE06: u32 = 0x0318;
pub const DSP_RG_SEMAPHORE07: u32 = 0x031C;
pub const DSP_RESERVED_0: u32 = 0x03F0;
pub const DSP_RESERVED_1: u32 = 0x03F4;

/* dsp wdt */
pub const DSP_WDT_MODE: u32 = 0x0400;

/* dsp mbox */
pub const DSP_MBOX_IN_CMD: u32 = 0x00;
pub const DSP_MBOX_IN_CMD_CLR: u32 = 0x04;
pub const DSP_MBOX_OUT_CMD: u32 = 0x1c;
pub const DSP_MBOX_OUT_CMD_CLR: u32 = 0x20;
pub const DSP_MBOX_IN_MSG0: u32 = 0x08;
pub const DSP_MBOX_IN_MSG1: u32 = 0x0C;
pub const DSP_MBOX_OUT_MSG0: u32 = 0x24;
pub const DSP_MBOX_OUT_MSG1: u32 = 0x28;

/*dsp sys ao*/
pub const ADSP_SRAM_POOL_CON: u32 = DSP_SYSAO_BASE + 0x30;
pub const DSP_SRAM_POOL_PD_MASK: u32 = 0xf;
pub const DSP_EMI_MAP_ADDR: u32 = DSP_SYSAO_BASE + 0x81c;

/* DSP memories */
pub const MBOX_OFFSET: u32 = 0x800000; /* DRAM */
pub const MBOX_SIZE: u32 = 0x1000; /* consistent with which in memory.h of sof fw */
pub const DSP_DRAM_SIZE: u32 = 0x1000000; /* 16M */

pub const DSP_REG_BAR: u32 = 4;
pub const DSP_MBOX0_BAR: u32 = 5;
pub const DSP_MBOX1_BAR: u32 = 6;
pub const DSP_MBOX2_BAR: u32 = 7;

pub const SIZE_SHARED_DRAM_DL: u32 = 0x40000; /*Shared buffer for Downlink*/
pub const SIZE_SHARED_DRAM_UL: u32 = 0x40000; /*Shared buffer for Uplink*/

pub const TOTAL_SIZE_SHARED_DRAM_FROM_TAIL: u32 = SIZE_SHARED_DRAM_DL + SIZE_SHARED_DRAM_UL;

pub const SRAM_PHYS_BASE_FROM_DSP_VIEW: u32 = 0x40000000; /* MT8195 DSP view */
pub const DRAM_PHYS_BASE_FROM_DSP_VIEW: u32 = 0x60000000; /* MT8195 DSP view */

/*remap dram between AP and DSP view, 4KB aligned*/
pub const DRAM_REMAP_SHIFT: u32 = 12;
pub const DRAM_REMAP_MASK: u32 = BIT(DRAM_REMAP_SHIFT) - 1;

/* suspend dsp idle check interval and timeout */
pub const SUSPEND_DSP_IDLE_TIMEOUT_US: u32 = 1000000; /* timeout to wait dsp idle, 1 sec */
pub const SUSPEND_DSP_IDLE_POLL_INTERVAL_US: u32 = 500; /* 0.5 msec */

unsafe extern "C" {
    pub fn sof_hifixdsp_boot_sequence(sdev: *mut snd_sof_dev, boot_addr: u32);
    pub fn sof_hifixdsp_shutdown(sdev: *mut snd_sof_dev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
