/* SPDX-License-Identifier: GPL-2.0 */
/* Analog Devices 1889 audio driver
 * Copyright (C) 2004, Kyle McMartin <kyle@parisc-linux.org>
 */

pub const AD_DS_WSMC: u32 = 0x00; /* wave/synthesis channel mixer control */
pub const AD_DS_WSMC_SYEN: u32 = 0x0004; /* synthesis channel enable */
pub const AD_DS_WSMC_SYRQ: u32 = 0x0030; /* synth. fifo request point */
pub const AD_DS_WSMC_WA16: u32 = 0x0100; /* wave channel 16bit select */
pub const AD_DS_WSMC_WAST: u32 = 0x0200; /* wave channel stereo select */
pub const AD_DS_WSMC_WAEN: u32 = 0x0400; /* wave channel enable */
pub const AD_DS_WSMC_WARQ: u32 = 0x3000; /* wave fifo request point */

pub const AD_DS_RAMC: u32 = 0x02; /* resampler/ADC channel mixer control */
pub const AD_DS_RAMC_AD16: u32 = 0x0001; /* ADC channel 16bit select */
pub const AD_DS_RAMC_ADST: u32 = 0x0002; /* ADC channel stereo select */
pub const AD_DS_RAMC_ADEN: u32 = 0x0004; /* ADC channel enable */
pub const AD_DS_RAMC_ACRQ: u32 = 0x0030; /* ADC fifo request point */
pub const AD_DS_RAMC_REEN: u32 = 0x0400; /* resampler channel enable */
pub const AD_DS_RAMC_RERQ: u32 = 0x3000; /* res. fifo request point */

pub const AD_DS_WADA: u32 = 0x04; /* wave channel mix attenuation */
pub const AD_DS_WADA_RWAM: u32 = 0x0080; /* right wave mute */
pub const AD_DS_WADA_RWAA: u32 = 0x001f; /* right wave attenuation */
pub const AD_DS_WADA_LWAM: u32 = 0x8000; /* left wave mute */
pub const AD_DS_WADA_LWAA: u32 = 0x3e00; /* left wave attenuation */

pub const AD_DS_SYDA: u32 = 0x06; /* synthesis channel mix attenuation */
pub const AD_DS_SYDA_RSYM: u32 = 0x0080; /* right synthesis mute */
pub const AD_DS_SYDA_RSYA: u32 = 0x001f; /* right synthesis attenuation */
pub const AD_DS_SYDA_LSYM: u32 = 0x8000; /* left synthesis mute */
pub const AD_DS_SYDA_LSYA: u32 = 0x3e00; /* left synthesis attenuation */

pub const AD_DS_WAS: u32 = 0x08; /* wave channel sample rate */
pub const AD_DS_WAS_WAS: u32 = 0xffff; /* sample rate mask */

pub const AD_DS_RES: u32 = 0x0a; /* resampler channel sample rate */
pub const AD_DS_RES_RES: u32 = 0xffff; /* sample rate mask */

pub const AD_DS_CCS: u32 = 0x0c; /* chip control/status */
pub const AD_DS_CCS_ADO: u32 = 0x0001; /* ADC channel overflow */
pub const AD_DS_CCS_REO: u32 = 0x0002; /* resampler channel overflow */
pub const AD_DS_CCS_SYU: u32 = 0x0004; /* synthesis channel underflow */
pub const AD_DS_CCS_WAU: u32 = 0x0008; /* wave channel underflow */
/* bits 4 -> 7, 9, 11 -> 14 reserved */
pub const AD_DS_CCS_XTD: u32 = 0x0100; /* xtd delay control (4096 clock cycles) */
pub const AD_DS_CCS_PDALL: u32 = 0x0400; /* power */
pub const AD_DS_CCS_CLKEN: u32 = 0x8000; /* clock */

pub const AD_DMA_RESBA: u32 = 0x40; /* RES base address */
pub const AD_DMA_RESCA: u32 = 0x44; /* RES current address */
pub const AD_DMA_RESBC: u32 = 0x48; /* RES base count */
pub const AD_DMA_RESCC: u32 = 0x4c; /* RES current count */

pub const AD_DMA_ADCBA: u32 = 0x50; /* ADC base address */
pub const AD_DMA_ADCCA: u32 = 0x54; /* ADC current address */
pub const AD_DMA_ADCBC: u32 = 0x58; /* ADC base count */
pub const AD_DMA_ADCCC: u32 = 0x5c; /* ADC current count */

pub const AD_DMA_SYNBA: u32 = 0x60; /* synth base address */
pub const AD_DMA_SYNCA: u32 = 0x64; /* synth current address */
pub const AD_DMA_SYNBC: u32 = 0x68; /* synth base count */
pub const AD_DMA_SYNCC: u32 = 0x6c; /* synth current count */

pub const AD_DMA_WAVBA: u32 = 0x70; /* wave base address */
pub const AD_DMA_WAVCA: u32 = 0x74; /* wave current address */
pub const AD_DMA_WAVBC: u32 = 0x78; /* wave base count */
pub const AD_DMA_WAVCC: u32 = 0x7c; /* wave current count */

pub const AD_DMA_RESIC: u32 = 0x80; /* RES dma interrupt current byte count */
pub const AD_DMA_RESIB: u32 = 0x84; /* RES dma interrupt base byte count */

pub const AD_DMA_ADCIC: u32 = 0x88; /* ADC dma interrupt current byte count */
pub const AD_DMA_ADCIB: u32 = 0x8c; /* ADC dma interrupt base byte count */

pub const AD_DMA_SYNIC: u32 = 0x90; /* synth dma interrupt current byte count */
pub const AD_DMA_SYNIB: u32 = 0x94; /* synth dma interrupt base byte count */

pub const AD_DMA_WAVIC: u32 = 0x98; /* wave dma interrupt current byte count */
pub const AD_DMA_WAVIB: u32 = 0x9c; /* wave dma interrupt base byte count */

pub const AD_DMA_ICC: u32 = 0xffffff; /* current byte count mask */
pub const AD_DMA_IBC: u32 = 0xffffff; /* base byte count mask */
/* bits 24 -> 31 reserved */

/* 4 bytes pad */
pub const AD_DMA_ADC: u32 = 0xa8; /* ADC      dma control and status */
pub const AD_DMA_SYNTH: u32 = 0xb0; /* Synth    dma control and status */
pub const AD_DMA_WAV: u32 = 0xb8; /* wave     dma control and status */
pub const AD_DMA_RES: u32 = 0xa0; /* Resample dma control and status */

pub const AD_DMA_SGDE: u32 = 0x0001; /* SGD mode enable */
pub const AD_DMA_LOOP: u32 = 0x0002; /* loop enable */
pub const AD_DMA_IM: u32 = 0x000c; /* interrupt mode mask */
pub const AD_DMA_IM_DIS: u32 = !AD_DMA_IM; /* disable */
pub const AD_DMA_IM_CNT: u32 = 0x0004; /* interrupt on count */
pub const AD_DMA_IM_SGD: u32 = 0x0008; /* interrupt on SGD flag */
pub const AD_DMA_IM_EOL: u32 = 0x000c; /* interrupt on End of Linked List */
pub const AD_DMA_SGDS: u32 = 0x0030; /* SGD status */
pub const AD_DMA_SFLG: u32 = 0x0040; /* SGD flag */
pub const AD_DMA_EOL: u32 = 0x0080; /* SGD end of list */
/* bits 8 -> 15 reserved */

pub const AD_DMA_DISR: u32 = 0xc0; /* dma interrupt status */
pub const AD_DMA_DISR_RESI: u32 = 0x000001; /* resampler channel interrupt */
pub const AD_DMA_DISR_ADCI: u32 = 0x000002; /* ADC channel interrupt */
pub const AD_DMA_DISR_SYNI: u32 = 0x000004; /* synthesis channel interrupt */
pub const AD_DMA_DISR_WAVI: u32 = 0x000008; /* wave channel interrupt */
/* bits 4, 5 reserved */
pub const AD_DMA_DISR_SEPS: u32 = 0x000040; /* serial eeprom status */
/* bits 7 -> 13 reserved */
pub const AD_DMA_DISR_PMAI: u32 = 0x004000; /* pci master abort interrupt */
pub const AD_DMA_DISR_PTAI: u32 = 0x008000; /* pci target abort interrupt */
pub const AD_DMA_DISR_PTAE: u32 = 0x010000; /* pci target abort interrupt enable */
pub const AD_DMA_DISR_PMAE: u32 = 0x020000; /* pci master abort interrupt enable */
/* bits 19 -> 31 reserved */

/* interrupt mask */
pub const AD_INTR_MASK: u32 =
    AD_DMA_DISR_RESI | AD_DMA_DISR_ADCI | AD_DMA_DISR_WAVI | AD_DMA_DISR_SYNI | AD_DMA_DISR_PMAI | AD_DMA_DISR_PTAI;

pub const AD_DMA_CHSS: u32 = 0xc4; /* dma channel stop status */
pub const AD_DMA_CHSS_RESS: u32 = 0x000001; /* resampler channel stopped */
pub const AD_DMA_CHSS_ADCS: u32 = 0x000002; /* ADC channel stopped */
pub const AD_DMA_CHSS_SYNS: u32 = 0x000004; /* synthesis channel stopped */
pub const AD_DMA_CHSS_WAVS: u32 = 0x000008; /* wave channel stopped */

pub const AD_GPIO_IPC: u32 = 0xc8; /* gpio port control */
pub const AD_GPIO_OP: u32 = 0xca; /* gpio output port status */
pub const AD_GPIO_IP: u32 = 0xcc; /* gpio  input port status */

pub const AD_AC97_BASE: u32 = 0x100; /* ac97 base register */

pub const AD_AC97_RESET: u32 = 0x100; /* reset */

pub const AD_AC97_PWR_CTL: u32 = 0x126; /* == AC97_POWERDOWN */
pub const AD_AC97_PWR_ADC: u32 = 0x0001; /* ADC ready status */
pub const AD_AC97_PWR_DAC: u32 = 0x0002; /* DAC ready status */
pub const AD_AC97_PWR_PR0: u32 = 0x0100; /* PR0 (ADC) powerdown */
pub const AD_AC97_PWR_PR1: u32 = 0x0200; /* PR1 (DAC) powerdown */

pub const AD_MISC_CTL: u32 = 0x176; /* misc control */
pub const AD_MISC_CTL_DACZ: u32 = 0x8000; /* set for zero fill, unset for repeat */
pub const AD_MISC_CTL_ARSR: u32 = 0x0001; /* set for SR1, unset for SR0 */
pub const AD_MISC_CTL_ALSR: u32 = 0x0100;
pub const AD_MISC_CTL_DLSR: u32 = 0x0400;
pub const AD_MISC_CTL_DRSR: u32 = 0x0004;

pub const AD_AC97_SR0: u32 = 0x178; /* sample rate 0, 0xbb80 == 48K */
pub const AD_AC97_SR0_48K: u32 = 0xbb80; /* 48KHz */
pub const AD_AC97_SR1: u32 = 0x17a; /* sample rate 1 */

pub const AD_AC97_ACIC: u32 = 0x180; /* ac97 codec interface control */
pub const AD_AC97_ACIC_ACIE: u32 = 0x0001; /* analog codec interface enable */
pub const AD_AC97_ACIC_ACRD: u32 = 0x0002; /* analog codec reset disable */
pub const AD_AC97_ACIC_ASOE: u32 = 0x0004; /* audio stream output enable */
pub const AD_AC97_ACIC_VSRM: u32 = 0x0008; /* variable sample rate mode */
pub const AD_AC97_ACIC_FSDH: u32 = 0x0100; /* force SDATA_OUT high */
pub const AD_AC97_ACIC_FSYH: u32 = 0x0200; /* force sync high */
pub const AD_AC97_ACIC_ACRDY: u32 = 0x8000; /* analog codec ready status */
/* bits 10 -> 14 reserved */

pub const AD_DS_MEMSIZE: u32 = 512;
pub const AD_OPL_MEMSIZE: u32 = 16;
pub const AD_MIDI_MEMSIZE: u32 = 16;

pub const AD_WAV_STATE: u32 = 0;
pub const AD_ADC_STATE: u32 = 1;
pub const AD_MAX_STATES: u32 = 2;

pub const AD_CHAN_WAV: u32 = 0x0001;
pub const AD_CHAN_ADC: u32 = 0x0002;
pub const AD_CHAN_RES: u32 = 0x0004;
pub const AD_CHAN_SYN: u32 = 0x0008;

/* The chip would support 4 GB buffers and 16 MB periods,
 * but let's not overdo it ... */
pub const BUFFER_BYTES_MAX: u32 = 256 * 1024;
pub const PERIOD_BYTES_MIN: u32 = 32;
pub const PERIOD_BYTES_MAX: u32 = BUFFER_BYTES_MAX / 2;
pub const PERIODS_MIN: u32 = 2;
pub const PERIODS_MAX: u32 = BUFFER_BYTES_MAX / PERIOD_BYTES_MIN;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
