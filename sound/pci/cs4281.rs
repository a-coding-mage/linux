// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for Cirrus Logic CS4281 based PCI soundcard
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, improper_ctypes, static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type bool_ = bool;
type u32 = c_uint;
type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_uint;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const ENOSYS: c_int = 38;
const EIO: c_int = 5;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool_; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* Enable switches */
static mut dual_codec: [bool_; SNDRV_CARDS] = [false; SNDRV_CARDS]; /* dual codec */

/* module_param_array/MODULE_* metadata omitted from executable Rust translation. */

/*
 *  Direct registers
 */
const CS4281_BA0_SIZE: c_uint = 0x1000;
const CS4281_BA1_SIZE: c_uint = 0x10000;

/*
 *  BA0 registers
 */
const BA0_HISR: c_ulong = 0x0000; /* Host Interrupt Status Register */
const BA0_HISR_INTENA: c_uint = 1 << 31; /* Internal Interrupt Enable Bit */
const BA0_HISR_MIDI: c_uint = 1 << 22; /* MIDI port interrupt */
const BA0_HISR_FIFOI: c_uint = 1 << 20; /* FIFO polled interrupt */
const BA0_HISR_DMAI: c_uint = 1 << 18; /* DMA interrupt (half or end) */
fn BA0_HISR_FIFO(c: c_uint) -> c_uint { 1 << (12 + c) } /* FIFO channel interrupt */
fn BA0_HISR_DMA(c: c_uint) -> c_uint { 1 << (8 + c) } /* DMA channel interrupt */
const BA0_HISR_GPPI: c_uint = 1 << 5; /* General Purpose Input (Primary chip) */
const BA0_HISR_GPSI: c_uint = 1 << 4; /* General Purpose Input (Secondary chip) */
const BA0_HISR_GP3I: c_uint = 1 << 3; /* GPIO3 pin Interrupt */
const BA0_HISR_GP1I: c_uint = 1 << 2; /* GPIO1 pin Interrupt */
const BA0_HISR_VUPI: c_uint = 1 << 1; /* VOLUP pin Interrupt */
const BA0_HISR_VDNI: c_uint = 1 << 0; /* VOLDN pin Interrupt */

const BA0_HICR: c_ulong = 0x0008; /* Host Interrupt Control Register */
const BA0_HICR_CHGM: c_uint = 1 << 1; /* INTENA Change Mask */
const BA0_HICR_IEV: c_uint = 1 << 0; /* INTENA Value */
const BA0_HICR_EOI: c_uint = 3 << 0; /* End of Interrupt command */
const BA0_HIMR: c_ulong = 0x000c; /* Host Interrupt Mask Register */
const BA0_IIER: c_ulong = 0x0010; /* ISA Interrupt Enable Register */
const BA0_HDSR0: c_uint = 0x00f0;
const BA0_HDSR1: c_uint = 0x00f4;
const BA0_HDSR2: c_uint = 0x00f8;
const BA0_HDSR3: c_uint = 0x00fc;
const BA0_HDSR_CH1P: c_uint = 1 << 25;
const BA0_HDSR_CH2P: c_uint = 1 << 24;
const BA0_HDSR_DHTC: c_uint = 1 << 17;
const BA0_HDSR_DTC: c_uint = 1 << 16;
const BA0_HDSR_DRUN: c_uint = 1 << 15;
const BA0_HDSR_RQ: c_uint = 1 << 7;

const BA0_DCA0: c_uint = 0x0110;
const BA0_DCC0: c_uint = 0x0114;
const BA0_DBA0: c_uint = 0x0118;
const BA0_DBC0: c_uint = 0x011c;
const BA0_DCA1: c_uint = 0x0120;
const BA0_DCC1: c_uint = 0x0124;
const BA0_DBA1: c_uint = 0x0128;
const BA0_DBC1: c_uint = 0x012c;
const BA0_DCA2: c_uint = 0x0130;
const BA0_DCC2: c_uint = 0x0134;
const BA0_DBA2: c_uint = 0x0138;
const BA0_DBC2: c_uint = 0x013c;
const BA0_DCA3: c_uint = 0x0140;
const BA0_DCC3: c_uint = 0x0144;
const BA0_DBA3: c_uint = 0x0148;
const BA0_DBC3: c_uint = 0x014c;
const BA0_DMR0: c_uint = 0x0150;
const BA0_DCR0: c_uint = 0x0154;
const BA0_DMR1: c_uint = 0x0158;
const BA0_DCR1: c_uint = 0x015c;
const BA0_DMR2: c_uint = 0x0160;
const BA0_DCR2: c_uint = 0x0164;
const BA0_DMR3: c_uint = 0x0168;
const BA0_DCR3: c_uint = 0x016c;

const BA0_DMR_DMA: c_uint = 1 << 29;
const BA0_DMR_POLL: c_uint = 1 << 28;
const BA0_DMR_TBC: c_uint = 1 << 25;
const BA0_DMR_CBC: c_uint = 1 << 24;
const BA0_DMR_SWAPC: c_uint = 1 << 22;
const BA0_DMR_SIZE20: c_uint = 1 << 20;
const BA0_DMR_USIGN: c_uint = 1 << 19;
const BA0_DMR_BEND: c_uint = 1 << 18;
const BA0_DMR_MONO: c_uint = 1 << 17;
const BA0_DMR_SIZE8: c_uint = 1 << 16;
const BA0_DMR_TYPE_DEMAND: c_uint = 0 << 6;
const BA0_DMR_TYPE_SINGLE: c_uint = 1 << 6;
const BA0_DMR_TYPE_BLOCK: c_uint = 2 << 6;
const BA0_DMR_TYPE_CASCADE: c_uint = 3 << 6;
const BA0_DMR_DEC: c_uint = 1 << 5;
const BA0_DMR_AUTO: c_uint = 1 << 4;
const BA0_DMR_TR_VERIFY: c_uint = 0 << 2;
const BA0_DMR_TR_WRITE: c_uint = 1 << 2;
const BA0_DMR_TR_READ: c_uint = 2 << 2;
const BA0_DCR_HTCIE: c_uint = 1 << 17;
const BA0_DCR_TCIE: c_uint = 1 << 16;
const BA0_DCR_MSK: c_uint = 1 << 0;

const BA0_FCR0: c_uint = 0x0180;
const BA0_FCR1: c_uint = 0x0184;
const BA0_FCR2: c_uint = 0x0188;
const BA0_FCR3: c_uint = 0x018c;
const BA0_FCR_FEN: c_uint = 1 << 31;
const BA0_FCR_DACZ: c_uint = 1 << 30;
const BA0_FCR_PSH: c_uint = 1 << 29;
fn BA0_FCR_RS(x: c_uint) -> c_uint { ((x) & 0x1f) << 24 }
fn BA0_FCR_LS(x: c_uint) -> c_uint { ((x) & 0x1f) << 16 }
fn BA0_FCR_SZ(x: c_uint) -> c_uint { ((x) & 0x7f) << 8 }
fn BA0_FCR_OF(x: c_uint) -> c_uint { ((x) & 0x7f) << 0 }
const BA0_FPDR0: c_uint = 0x0190;
const BA0_FPDR1: c_uint = 0x0194;
const BA0_FPDR2: c_uint = 0x0198;
const BA0_FPDR3: c_uint = 0x019c;
const BA0_FCHS: c_uint = 0x020c;
fn BA0_FCHS_RCO(x: c_uint) -> c_uint { 1 << (7 + (((x) & 3) << 3)) }
fn BA0_FCHS_LCO(x: c_uint) -> c_uint { 1 << (6 + (((x) & 3) << 3)) }
fn BA0_FCHS_MRP(x: c_uint) -> c_uint { 1 << (5 + (((x) & 3) << 3)) }
fn BA0_FCHS_FE(x: c_uint) -> c_uint { 1 << (4 + (((x) & 3) << 3)) }
fn BA0_FCHS_FF(x: c_uint) -> c_uint { 1 << (3 + (((x) & 3) << 3)) }
fn BA0_FCHS_IOR(x: c_uint) -> c_uint { 1 << (2 + (((x) & 3) << 3)) }
fn BA0_FCHS_RCI(x: c_uint) -> c_uint { 1 << (1 + (((x) & 3) << 3)) }
fn BA0_FCHS_LCI(x: c_uint) -> c_uint { 1 << (0 + (((x) & 3) << 3)) }
const BA0_FSIC0: c_uint = 0x0210;
const BA0_FSIC1: c_uint = 0x0214;
const BA0_FSIC2: c_uint = 0x0218;
const BA0_FSIC3: c_uint = 0x021c;
fn BA0_FSIC_FIC(x: c_uint) -> c_uint { ((x) & 0x7f) << 24 }
const BA0_FSIC_FORIE: c_uint = 1 << 23;
const BA0_FSIC_FURIE: c_uint = 1 << 22;
const BA0_FSIC_FSCIE: c_uint = 1 << 16;
fn BA0_FSIC_FSC(x: c_uint) -> c_uint { ((x) & 0x7f) << 8 }
const BA0_FSIC_FOR: c_uint = 1 << 7;
const BA0_FSIC_FUR: c_uint = 1 << 6;
const BA0_FSIC_FSCR: c_uint = 1 << 0;

const BA0_PMCS: c_ulong = 0x0344;
const BA0_CWPR: c_ulong = 0x03e0;
const BA0_EPPMC: c_ulong = 0x03e4;
const BA0_EPPMC_FPDN: c_uint = 1 << 14;
const BA0_GPIOR: c_uint = 0x03e8;
const BA0_SPMC: c_ulong = 0x03ec;
const BA0_SPMC_GIPPEN: c_uint = 1 << 15;
const BA0_SPMC_GISPEN: c_uint = 1 << 14;
const BA0_SPMC_EESPD: c_uint = 1 << 9;
const BA0_SPMC_ASDI2E: c_uint = 1 << 8;
const BA0_SPMC_ASDO: c_uint = 1 << 7;
const BA0_SPMC_WUP2: c_uint = 1 << 3;
const BA0_SPMC_WUP1: c_uint = 1 << 2;
const BA0_SPMC_ASYNC: c_uint = 1 << 1;
const BA0_SPMC_RSTN: c_uint = 1 << 0;
const BA0_CFLR: c_ulong = 0x03f0;
const BA0_CFLR_DEFAULT: c_uint = 0x00000001;
const BA0_IISR: c_uint = 0x03f4;
const BA0_TMS: c_uint = 0x03f8;
const BA0_SSVID: c_uint = 0x03fc;
const BA0_CLKCR1: c_ulong = 0x0400;
const BA0_CLKCR1_CLKON: c_uint = 1 << 25;
const BA0_CLKCR1_DLLRDY: c_uint = 1 << 24;
const BA0_CLKCR1_DLLOS: c_uint = 1 << 6;
const BA0_CLKCR1_SWCE: c_uint = 1 << 5;
const BA0_CLKCR1_DLLP: c_uint = 1 << 4;
fn BA0_CLKCR1_DLLSS(x: c_uint) -> c_uint { ((x) & 3) << 3 }
const BA0_FRR: c_uint = 0x0410;
const BA0_SLT12O: c_uint = 0x041c;
const BA0_SERMC: c_ulong = 0x0420;
const BA0_SERMC_FCRN: c_uint = 1 << 27;
const BA0_SERMC_ODSEN2: c_uint = 1 << 25;
const BA0_SERMC_ODSEN1: c_uint = 1 << 24;
const BA0_SERMC_SXLB: c_uint = 1 << 21;
const BA0_SERMC_SLB: c_uint = 1 << 20;
const BA0_SERMC_LOVF: c_uint = 1 << 19;
fn BA0_SERMC_TCID(x: c_int) -> c_uint { (((x) as c_uint) & 3) << 16 }
const BA0_SERMC_PXLB: c_uint = 5 << 1;
const BA0_SERMC_PLB: c_uint = 4 << 1;
const BA0_SERMC_PTC: c_uint = 7 << 1;
const BA0_SERMC_PTC_AC97: c_uint = 1 << 1;
const BA0_SERMC_MSPE: c_uint = 1 << 0;
const BA0_SERC1: c_ulong = 0x0428;
fn BA0_SERC1_SO1F(x: c_uint) -> c_uint { ((x) & 7) >> 1 }
const BA0_SERC1_AC97: c_uint = 1 << 1;
const BA0_SERC1_SO1EN: c_uint = 1 << 0;
const BA0_SERC2: c_ulong = 0x042c;
fn BA0_SERC2_SI1F(x: c_uint) -> c_uint { ((x) & 7) >> 1 }
const BA0_SERC2_AC97: c_uint = 1 << 1;
const BA0_SERC2_SI1EN: c_uint = 1 << 0;
const BA0_SLT12M: c_uint = 0x045c;
const BA0_ACCTL: c_ulong = 0x0460;
const BA0_ACCTL_TC: c_uint = 1 << 6;
const BA0_ACCTL_CRW: c_uint = 1 << 4;
const BA0_ACCTL_DCV: c_uint = 1 << 3;
const BA0_ACCTL_VFRM: c_uint = 1 << 2;
const BA0_ACCTL_ESYN: c_uint = 1 << 1;
const BA0_ACSTS: c_ulong = 0x0464;
const BA0_ACSTS_VSTS: c_uint = 1 << 1;
const BA0_ACSTS_CRDY: c_uint = 1 << 0;
const BA0_ACOSV: c_ulong = 0x0468;
fn BA0_ACOSV_SLV(x: c_uint) -> c_uint { 1 << ((x) - 3) }
const BA0_ACCAD: c_ulong = 0x046c;
const BA0_ACCDA: c_ulong = 0x0470;
const BA0_ACISV: c_ulong = 0x0474;
fn BA0_ACISV_SLV(x: c_uint) -> c_uint { 1 << ((x) - 3) }
const BA0_ACSAD: c_uint = 0x0478;
const BA0_ACSDA: c_ulong = 0x047c;
const BA0_JSPT: c_ulong = 0x0480;
const BA0_JSCTL: c_uint = 0x0484;
const BA0_JSC1: c_ulong = 0x0488;
const BA0_JSC2: c_ulong = 0x048c;
const BA0_JSIO: c_ulong = 0x04a0;
const BA0_MIDCR: c_ulong = 0x0490;
const BA0_MIDCR_MRST: c_uint = 1 << 5;
const BA0_MIDCR_MLB: c_uint = 1 << 4;
const BA0_MIDCR_TIE: c_uint = 1 << 3;
const BA0_MIDCR_RIE: c_uint = 1 << 2;
const BA0_MIDCR_RXE: c_uint = 1 << 1;
const BA0_MIDCR_TXE: c_uint = 1 << 0;
const BA0_MIDCMD: c_uint = 0x0494;
const BA0_MIDSR: c_ulong = 0x0494;
const BA0_MIDSR_RDA: c_uint = 1 << 15;
const BA0_MIDSR_TBE: c_uint = 1 << 14;
const BA0_MIDSR_RBE: c_uint = 1 << 7;
const BA0_MIDSR_TBF: c_uint = 1 << 6;
const BA0_MIDWP: c_ulong = 0x0498;
const BA0_MIDRP: c_ulong = 0x049c;
const BA0_AODSD1: c_uint = 0x04a8;
fn BA0_AODSD1_NDS(x: c_uint) -> c_uint { 1 << ((x) - 3) }
const BA0_AODSD2: c_uint = 0x04ac;
fn BA0_AODSD2_NDS(x: c_uint) -> c_uint { 1 << ((x) - 3) }
const BA0_CFGI: c_uint = 0x04b0;
const BA0_SLT12M2: c_uint = 0x04dc;
const BA0_ACSTS2: c_ulong = 0x04e4;
const BA0_ACISV2: c_uint = 0x04f4;
const BA0_ACSAD2: c_uint = 0x04f8;
const BA0_ACSDA2: c_ulong = 0x04fc;
const BA0_FMSR: c_uint = 0x0730;
const BA0_B0AP: c_ulong = 0x0730;
const BA0_FMDP: c_uint = 0x0734;
const BA0_B1AP: c_ulong = 0x0738;
const BA0_B1DP: c_uint = 0x073c;
const BA0_SSPM: c_ulong = 0x0740;
const BA0_SSPM_MIXEN: c_uint = 1 << 6;
const BA0_SSPM_CSRCEN: c_uint = 1 << 5;
const BA0_SSPM_PSRCEN: c_uint = 1 << 4;
const BA0_SSPM_JSEN: c_uint = 1 << 3;
const BA0_SSPM_ACLEN: c_uint = 1 << 2;
const BA0_SSPM_FMEN: c_uint = 1 << 1;
const BA0_DACSR: c_ulong = 0x0744;
const BA0_ADCSR: c_ulong = 0x0748;
const BA0_SSCR: c_uint = 0x074c;
const BA0_SSCR_HVS1: c_uint = 1 << 23;
const BA0_SSCR_MVCS: c_uint = 1 << 19;
const BA0_SSCR_MVLD: c_uint = 1 << 18;
const BA0_SSCR_MVAD: c_uint = 1 << 17;
const BA0_SSCR_MVMD: c_uint = 1 << 16;
const BA0_SSCR_XLPSRC: c_uint = 1 << 8;
const BA0_SSCR_LPSRC: c_uint = 1 << 7;
const BA0_SSCR_CDTX: c_uint = 1 << 5;
const BA0_SSCR_HVC: c_uint = 1 << 3;
const BA0_FMLVC: c_uint = 0x0754;
const BA0_FMRVC: c_uint = 0x0758;
const BA0_SRCSA: c_ulong = 0x075c;
const BA0_PPLVC: c_uint = 0x0760;
const BA0_PPRVC: c_uint = 0x0764;
const BA0_PASR: c_uint = 0x0768;
const BA0_CASR: c_uint = 0x076C;

/* Source Slot Numbers - Playback */
const SRCSLOT_LEFT_PCM_PLAYBACK: c_uint = 0;
const SRCSLOT_RIGHT_PCM_PLAYBACK: c_uint = 1;
const SRCSLOT_PHONE_LINE_1_DAC: c_uint = 2;
const SRCSLOT_CENTER_PCM_PLAYBACK: c_uint = 3;
const SRCSLOT_LEFT_SURROUND_PCM_PLAYBACK: c_uint = 4;
const SRCSLOT_RIGHT_SURROUND_PCM_PLAYBACK: c_uint = 5;
const SRCSLOT_LFE_PCM_PLAYBACK: c_uint = 6;
const SRCSLOT_PHONE_LINE_2_DAC: c_uint = 7;
const SRCSLOT_HEADSET_DAC: c_uint = 8;
const SRCSLOT_LEFT_WT: c_uint = 29;
const SRCSLOT_RIGHT_WT: c_uint = 30;
/* Source Slot Numbers - Capture */
const SRCSLOT_LEFT_PCM_RECORD: c_uint = 10;
const SRCSLOT_RIGHT_PCM_RECORD: c_uint = 11;
const SRCSLOT_PHONE_LINE_1_ADC: c_uint = 12;
const SRCSLOT_MIC_ADC: c_uint = 13;
const SRCSLOT_PHONE_LINE_2_ADC: c_uint = 17;
const SRCSLOT_HEADSET_ADC: c_uint = 18;
const SRCSLOT_SECONDARY_LEFT_PCM_RECORD: c_uint = 20;
const SRCSLOT_SECONDARY_RIGHT_PCM_RECORD: c_uint = 21;
const SRCSLOT_SECONDARY_PHONE_LINE_1_ADC: c_uint = 22;
const SRCSLOT_SECONDARY_MIC_ADC: c_uint = 23;
const SRCSLOT_SECONDARY_PHONE_LINE_2_ADC: c_uint = 27;
const SRCSLOT_SECONDARY_HEADSET_ADC: c_uint = 28;
const SRCSLOT_POWER_DOWN: c_uint = 31;
const CS4281_MODE_OUTPUT: c_uint = 1 << 0;
const CS4281_MODE_INPUT: c_uint = 1 << 1;

const JSPT_CAX: c_uint = 0x00000001;
const JSPT_CAY: c_uint = 0x00000002;
const JSPT_CBX: c_uint = 0x00000004;
const JSPT_CBY: c_uint = 0x00000008;
const JSPT_BA1: c_uint = 0x00000010;
const JSPT_BA2: c_uint = 0x00000020;
const JSPT_BB1: c_uint = 0x00000040;
const JSPT_BB2: c_uint = 0x00000080;
const JSCTL_SP_MASK: c_uint = 0x00000003;
const JSCTL_SP_SLOW: c_uint = 0x00000000;
const JSCTL_SP_MEDIUM_SLOW: c_uint = 0x00000001;
const JSCTL_SP_MEDIUM_FAST: c_uint = 0x00000002;
const JSCTL_SP_FAST: c_uint = 0x00000003;
const JSCTL_ARE: c_uint = 0x00000004;
const JSC1_Y1V_MASK: c_uint = 0x0000FFFF;
const JSC1_X1V_MASK: c_uint = 0xFFFF0000;
const JSC1_Y1V_SHIFT: c_uint = 0;
const JSC1_X1V_SHIFT: c_uint = 16;
const JSC2_Y2V_MASK: c_uint = 0x0000FFFF;
const JSC2_X2V_MASK: c_uint = 0xFFFF0000;
const JSC2_Y2V_SHIFT: c_uint = 0;
const JSC2_X2V_SHIFT: c_uint = 16;
const JSIO_DAX: c_uint = 0x00000001;
const JSIO_DAY: c_uint = 0x00000002;
const JSIO_DBX: c_uint = 0x00000004;
const JSIO_DBY: c_uint = 0x00000008;
const JSIO_AXOE: c_uint = 0x00000010;
const JSIO_AYOE: c_uint = 0x00000020;
const JSIO_BXOE: c_uint = 0x00000040;
const JSIO_BYOE: c_uint = 0x00000080;

#[repr(C)] pub struct snd_pcm_substream { runtime: *mut snd_pcm_runtime, rmidi: *mut snd_rawmidi }
#[repr(C)] pub struct snd_pcm_runtime { private_data: *mut c_void, hw: snd_pcm_hardware, channels: c_uint, format: c_uint, buffer_size: c_uint, period_size: c_uint, dma_addr: c_uint, rate: c_uint }
#[repr(C)] pub struct snd_ac97 { private_data: *mut c_void, private_free: Option<unsafe extern "C" fn(*mut snd_ac97)>, num: c_int }
#[repr(C)] pub struct snd_ac97_bus { private_data: *mut c_void, private_free: Option<unsafe extern "C" fn(*mut snd_ac97_bus)> }
#[repr(C)] pub struct pci_dev { dev: device, irq: c_int }
#[repr(C)] pub struct device { _priv: [u8; 0] }
#[repr(C)] pub struct snd_card { private_data: *mut c_void, dev: *mut device, sync_irq: c_int, private_free: Option<unsafe extern "C" fn(*mut snd_card)>, driver: [c_char; 32], shortname: [c_char; 32], longname: [c_char; 80] }
#[repr(C)] pub struct snd_pcm { private_data: *mut c_void, info_flags: c_uint, name: [c_char; 80] }
#[repr(C)] pub struct snd_rawmidi { private_data: *mut c_void, info_flags: c_uint, name: [c_char; 80] }
#[repr(C)] pub struct snd_rawmidi_substream { rmidi: *mut snd_rawmidi }
#[repr(C)] pub struct gameport { open: Option<unsafe extern "C" fn(*mut gameport, c_int) -> c_int>, read: Option<unsafe extern "C" fn(*mut gameport) -> u8>, trigger: Option<unsafe extern "C" fn(*mut gameport)>, cooked_read: Option<unsafe extern "C" fn(*mut gameport, *mut c_int, *mut c_int) -> c_int> }
#[repr(C)] pub struct snd_opl3 { private_data: *mut c_void, command: Option<unsafe extern "C" fn(*mut snd_opl3, u16, u8)>, reg_lock: spinlock_t }
#[repr(C)] pub struct snd_info_entry { private_data: *mut c_void, content: c_int, c: snd_info_entry_c, size: c_uint }
#[repr(C)] pub struct snd_info_entry_c { ops: *const snd_info_entry_ops }
#[repr(C)] pub struct snd_info_buffer { _priv: [u8; 0] }
#[repr(C)] pub struct file { _priv: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { private_value: c_ulong }
#[repr(C)] pub struct snd_ctl_elem_info { type_: c_int, count: c_uint, value: snd_ctl_elem_info_value }
#[repr(C)] pub struct snd_ctl_elem_info_value { integer: snd_ctl_elem_info_integer }
#[repr(C)] pub struct snd_ctl_elem_info_integer { min: c_long, max: c_long }
#[repr(C)] pub struct snd_ctl_elem_value { value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_ctl_elem_value_value { integer: snd_ctl_elem_value_integer }
#[repr(C)] pub struct snd_ctl_elem_value_integer { value: [c_long; 2] }
#[repr(C)] pub struct snd_ac97_template { private_data: *mut c_void, private_free: Option<unsafe extern "C" fn(*mut snd_ac97)>, num: c_int }
#[repr(C)] pub struct pci_device_id { vendor: c_uint, device: c_uint, subvendor: c_uint, subdevice: c_uint, class: c_uint, class_mask: c_uint, driver_data: c_ulong }
#[repr(C)] pub struct spinlock_t { _priv: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hardware { info: c_uint, formats: c_uint, rates: c_uint, rate_min: c_uint, rate_max: c_uint, channels_min: c_uint, channels_max: c_uint, buffer_bytes_max: c_uint, period_bytes_min: c_uint, period_bytes_max: c_uint, periods_min: c_uint, periods_max: c_uint, fifo_size: c_uint }
#[repr(C)] pub struct snd_pcm_ops { open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>, pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t> }
#[repr(C)] pub struct snd_rawmidi_ops { open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>, close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>, trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)> }
#[repr(C)] pub struct snd_kcontrol_new { iface: c_int, name: *const c_char, info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, private_value: c_ulong, tlv: snd_kcontrol_new_tlv }
#[repr(C)] pub union snd_kcontrol_new_tlv { p: *const c_uint }
#[repr(C)] pub struct snd_ac97_bus_ops { write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>, read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16> }
#[repr(C)] pub struct snd_info_entry_ops { read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut c_void, *mut file, *mut c_char, size_t, loff_t) -> ssize_t> }
#[repr(C)] pub struct dev_pm_ops { _priv: [u8; 0] }
#[repr(C)] pub struct pci_driver { name: *const c_char, id_table: *const pci_device_id, probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>, driver: pci_driver_inner }
#[repr(C)] pub struct pci_driver_inner { pm: *const dev_pm_ops }

#[repr(C)]
struct cs4281_dma {
    substream: *mut snd_pcm_substream,
    regDBA: c_uint,
    regDCA: c_uint,
    regDBC: c_uint,
    regDCC: c_uint,
    regDMR: c_uint,
    regDCR: c_uint,
    regHDSR: c_uint,
    regFCR: c_uint,
    regFSIC: c_uint,
    valDMR: c_uint,
    valDCR: c_uint,
    valFCR: c_uint,
    fifo_offset: c_uint,
    left_slot: u8,
    right_slot: u8,
    frag: c_int,
}

const SUSPEND_REGISTERS: usize = 20;

#[repr(C)]
struct cs4281 {
    irq: c_int,
    ba0: *mut c_void,
    ba1: *mut c_void,
    ba0_addr: c_ulong,
    ba1_addr: c_ulong,
    dual_codec: c_int,
    ac97_bus: *mut snd_ac97_bus,
    ac97: *mut snd_ac97,
    ac97_secondary: *mut snd_ac97,
    pci: *mut pci_dev,
    card: *mut snd_card,
    pcm: *mut snd_pcm,
    rmidi: *mut snd_rawmidi,
    midi_input: *mut snd_rawmidi_substream,
    midi_output: *mut snd_rawmidi_substream,
    dma: [cs4281_dma; 4],
    src_left_play_slot: u8,
    src_right_play_slot: u8,
    src_left_rec_slot: u8,
    src_right_rec_slot: u8,
    spurious_dhtc_irq: c_uint,
    spurious_dtc_irq: c_uint,
    reg_lock: spinlock_t,
    midcr: c_uint,
    uartm: c_uint,
    gameport: *mut gameport,
    suspend_regs: [u32; SUSPEND_REGISTERS],
}

extern "C" {
    fn writel(val: c_uint, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> c_uint;
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);
    static mut jiffies: c_ulong;
    static HZ: c_ulong;
    fn schedule_timeout_uninterruptible(timeout: c_long);
    fn time_after_eq(a: c_ulong, b: c_ulong) -> bool_;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut cs4281;
    fn snd_pcm_format_unsigned(format: c_uint) -> c_int;
    fn snd_pcm_format_big_endian(format: c_uint) -> c_int;
    fn snd_pcm_format_width(format: c_uint) -> c_int;
    fn snd_BUG_ON(cond: bool_) -> bool_;
    fn snd_pcm_hw_constraint_msbits(runtime: *mut snd_pcm_runtime, a: c_uint, b: c_uint, c: c_uint) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, typ: c_int, dev: *mut device, min: size_t, max: size_t);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut cs4281;
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, bus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(knew: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn copy_to_user_fromio(dst: *mut c_char, src: *mut c_void, count: size_t) -> c_int;
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, data: *mut c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>);
    fn snd_card_proc_new(card: *mut snd_card, name: *const c_char, entry: *mut *mut snd_info_entry) -> c_int;
    fn gameport_get_port_data(gp: *mut gameport) -> *mut cs4281;
    fn gameport_allocate_port() -> *mut gameport;
    fn gameport_set_name(gp: *mut gameport, name: *const c_char);
    fn gameport_set_phys(gp: *mut gameport, fmt: *const c_char, ...);
    fn pci_name(pci: *mut pci_dev) -> *const c_char;
    fn gameport_set_dev_parent(gp: *mut gameport, dev: *mut device);
    fn gameport_set_port_data(gp: *mut gameport, data: *mut c_void);
    fn gameport_register_port(gp: *mut gameport);
    fn gameport_unregister_port(gp: *mut gameport);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn pci_set_master(pci: *mut pci_dev);
    fn pcim_iomap_region(pci: *mut pci_dev, bar: c_int, name: *const c_char) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buf: *mut u8, count: size_t) -> c_int;
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buf: *mut u8, count: size_t) -> c_int;
    fn snd_rawmidi_new(card: *mut snd_card, id: *const c_char, device: c_int, output: c_int, input: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: size_t, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_opl3_new(card: *mut snd_card, hw: c_int, ropl3: *mut *mut snd_opl3) -> c_int;
    fn snd_opl3_init(opl3: *mut snd_opl3);
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, rrawmidi: *mut c_void) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int) -> c_int;
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
}

const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 3;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 4;
const SNDRV_PCM_FMTBIT_U8: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S8: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_U16_LE: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_U16_BE: c_uint = 1 << 4;
const SNDRV_PCM_FMTBIT_S16_BE: c_uint = 1 << 5;
const SNDRV_PCM_FMTBIT_U32_LE: c_uint = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << 7;
const SNDRV_PCM_FMTBIT_U32_BE: c_uint = 1 << 8;
const SNDRV_PCM_FMTBIT_S32_BE: c_uint = 1 << 9;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 29;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_INFO_CONTENT_DATA: c_int = 1;
const GAMEPORT_MODE_COOKED: c_int = 1;
const GAMEPORT_MODE_RAW: c_int = 2;
const IRQF_SHARED: c_ulong = 0x80;
const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 1;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 1 << 0;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 1 << 1;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 1 << 2;
const OPL3_RIGHT: u16 = 1;
const OPL3_HW_OPL3_CS4281: c_int = 0;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
static KBUILD_MODNAME: &[u8] = b"cs4281\0";
static mut THIS_MODULE: *mut c_void = ptr::null_mut();

static snd_cs4281_ids: [pci_device_id; 2] = [
    pci_device_id { vendor: 0, device: 0x6005, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 }, /* CS4281 */
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

const CS4281_FIFO_SIZE: c_uint = 32;

unsafe fn snd_cs4281_pokeBA0(chip: *mut cs4281, offset: c_ulong, val: c_uint) {
    writel(val, ((*chip).ba0 as *mut u8).add(offset as usize) as *mut c_void);
}

unsafe fn snd_cs4281_peekBA0(chip: *mut cs4281, offset: c_ulong) -> c_uint {
    readl(((*chip).ba0 as *mut u8).add(offset as usize) as *mut c_void)
}

unsafe extern "C" fn snd_cs4281_ac97_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    let chip = (*ac97).private_data as *mut cs4281;
    for _count in 0..2000 {
        snd_cs4281_pokeBA0(chip, BA0_ACCAD, reg as c_uint);
        snd_cs4281_pokeBA0(chip, BA0_ACCDA, val as c_uint);
        snd_cs4281_pokeBA0(chip, BA0_ACCTL, BA0_ACCTL_DCV | BA0_ACCTL_VFRM | BA0_ACCTL_ESYN | if (*ac97).num != 0 { BA0_ACCTL_TC } else { 0 });
        udelay(10);
        if snd_cs4281_peekBA0(chip, BA0_ACCTL) & BA0_ACCTL_DCV == 0 { return; }
    }
    dev_err((*(*chip).card).dev, b"AC'97 write problem, reg = 0x%x, val = 0x%x\n\0".as_ptr() as *const c_char, reg as c_int, val as c_int);
}

unsafe extern "C" fn snd_cs4281_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    let chip = (*ac97).private_data as *mut cs4281;
    let ac97_num = ptr::read_volatile(&(*ac97).num);
    snd_cs4281_peekBA0(chip, if ac97_num != 0 { BA0_ACSDA2 } else { BA0_ACSDA });
    snd_cs4281_pokeBA0(chip, BA0_ACCAD, reg as c_uint);
    snd_cs4281_pokeBA0(chip, BA0_ACCDA, 0);
    snd_cs4281_pokeBA0(chip, BA0_ACCTL, BA0_ACCTL_DCV | BA0_ACCTL_CRW | BA0_ACCTL_VFRM | BA0_ACCTL_ESYN | if ac97_num != 0 { BA0_ACCTL_TC } else { 0 });
    for _ in 0..500 {
        udelay(10);
        if snd_cs4281_peekBA0(chip, BA0_ACCTL) & BA0_ACCTL_DCV == 0 {
            for _ in 0..100 {
                if snd_cs4281_peekBA0(chip, if ac97_num != 0 { BA0_ACSTS2 } else { BA0_ACSTS }) & BA0_ACSTS_VSTS != 0 {
                    return snd_cs4281_peekBA0(chip, if ac97_num != 0 { BA0_ACSDA2 } else { BA0_ACSDA }) as u16;
                }
                udelay(10);
            }
            dev_err((*(*chip).card).dev, b"AC'97 read problem (ACSTS_VSTS), reg = 0x%x\n\0".as_ptr() as *const c_char, reg as c_int);
            return 0xffff;
        }
    }
    dev_err((*(*chip).card).dev, b"AC'97 read problem (ACCTL_DCV), reg = 0x%x\n\0".as_ptr() as *const c_char, reg as c_int);
    0xffff
}

unsafe extern "C" fn snd_cs4281_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let dma = (*(*substream).runtime).private_data as *mut cs4281_dma;
    let chip = snd_pcm_substream_chip(substream);
    match cmd {
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => { (*dma).valDCR |= BA0_DCR_MSK; (*dma).valFCR |= BA0_FCR_FEN; }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => { (*dma).valDCR &= !BA0_DCR_MSK; (*dma).valFCR &= !BA0_FCR_FEN; }
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            snd_cs4281_pokeBA0(chip, (*dma).regDMR as c_ulong, (*dma).valDMR & !BA0_DMR_DMA);
            (*dma).valDMR |= BA0_DMR_DMA; (*dma).valDCR &= !BA0_DCR_MSK; (*dma).valFCR |= BA0_FCR_FEN;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            (*dma).valDMR &= !(BA0_DMR_DMA | BA0_DMR_POLL); (*dma).valDCR |= BA0_DCR_MSK; (*dma).valFCR &= !BA0_FCR_FEN;
            if (*dma).regFCR != BA0_FCR0 { (*dma).valFCR &= !BA0_FCR_FEN; }
        }
        _ => return -EINVAL,
    }
    snd_cs4281_pokeBA0(chip, (*dma).regDMR as c_ulong, (*dma).valDMR);
    snd_cs4281_pokeBA0(chip, (*dma).regFCR as c_ulong, (*dma).valFCR);
    snd_cs4281_pokeBA0(chip, (*dma).regDCR as c_ulong, (*dma).valDCR);
    0
}

fn snd_cs4281_rate(rate: c_uint, real_rate: *mut c_uint) -> c_uint {
    unsafe { if !real_rate.is_null() { *real_rate = rate; } }
    match rate {
        8000 => 5, 11025 => 4, 16000 => 3, 22050 => 2, 44100 => 1, 48000 => 0,
        _ => {
            let val = 1536000 / rate;
            unsafe { if !real_rate.is_null() { *real_rate = 1536000 / val; } }
            val
        }
    }
}

unsafe fn snd_cs4281_mode(chip: *mut cs4281, dma: *mut cs4281_dma, runtime: *mut snd_pcm_runtime, capture: c_int, src: c_int) {
    (*dma).valDMR = BA0_DMR_TYPE_SINGLE | BA0_DMR_AUTO | if capture != 0 { BA0_DMR_TR_WRITE } else { BA0_DMR_TR_READ };
    if (*runtime).channels == 1 { (*dma).valDMR |= BA0_DMR_MONO; }
    if snd_pcm_format_unsigned((*runtime).format) > 0 { (*dma).valDMR |= BA0_DMR_USIGN; }
    if snd_pcm_format_big_endian((*runtime).format) > 0 { (*dma).valDMR |= BA0_DMR_BEND; }
    match snd_pcm_format_width((*runtime).format) {
        8 => { (*dma).valDMR |= BA0_DMR_SIZE8; if (*runtime).channels == 1 { (*dma).valDMR |= BA0_DMR_SWAPC; } }
        32 => (*dma).valDMR |= BA0_DMR_SIZE20,
        _ => {}
    }
    (*dma).frag = 0;
    (*dma).valDCR = BA0_DCR_TCIE | BA0_DCR_MSK;
    if (*runtime).buffer_size != (*runtime).period_size { (*dma).valDCR |= BA0_DCR_HTCIE; }
    snd_cs4281_pokeBA0(chip, (*dma).regDBA as c_ulong, (*runtime).dma_addr);
    snd_cs4281_pokeBA0(chip, (*dma).regDBC as c_ulong, (*runtime).buffer_size - 1);
    let rec_mono = ((*chip).dma[1].valDMR & BA0_DMR_MONO) == BA0_DMR_MONO;
    snd_cs4281_pokeBA0(chip, BA0_SRCSA, ((*chip).src_left_play_slot as c_uint) | ((*chip).src_right_play_slot as c_uint) << 8 | ((*chip).src_left_rec_slot as c_uint) << 16 | (if rec_mono { 31 } else { (*chip).src_right_rec_slot as c_uint }) << 24);
    if src != 0 {
        if capture == 0 {
            if (*dma).left_slot == (*chip).src_left_play_slot {
                snd_BUG_ON((*dma).right_slot != (*chip).src_right_play_slot);
                snd_cs4281_pokeBA0(chip, BA0_DACSR, snd_cs4281_rate((*runtime).rate, ptr::null_mut()));
            }
        } else if (*dma).left_slot == (*chip).src_left_rec_slot {
            snd_BUG_ON((*dma).right_slot != (*chip).src_right_rec_slot);
            snd_cs4281_pokeBA0(chip, BA0_ADCSR, snd_cs4281_rate((*runtime).rate, ptr::null_mut()));
        }
    }
    if (*dma).regFCR == BA0_FCR0 {
        snd_cs4281_pokeBA0(chip, (*dma).regFCR as c_ulong, snd_cs4281_peekBA0(chip, (*dma).regFCR as c_ulong) & !BA0_FCR_FEN);
    }
    (*dma).valFCR = BA0_FCR_LS((*dma).left_slot as c_uint) | BA0_FCR_RS(if capture != 0 && ((*dma).valDMR & BA0_DMR_MONO) != 0 { 31 } else { (*dma).right_slot as c_uint }) | BA0_FCR_SZ(CS4281_FIFO_SIZE) | BA0_FCR_OF((*dma).fifo_offset);
    snd_cs4281_pokeBA0(chip, (*dma).regFCR as c_ulong, (*dma).valFCR | if capture != 0 { BA0_FCR_PSH } else { 0 });
    if (*dma).regFCR == BA0_FCR0 { snd_cs4281_pokeBA0(chip, (*dma).regFCR as c_ulong, (*dma).valFCR | BA0_FCR_FEN); }
    snd_cs4281_pokeBA0(chip, (*dma).regFSIC as c_ulong, 0);
}

unsafe extern "C" fn snd_cs4281_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime; let dma = (*runtime).private_data as *mut cs4281_dma; let chip = snd_pcm_substream_chip(substream);
    snd_cs4281_mode(chip, dma, runtime, 0, 1); 0
}
unsafe extern "C" fn snd_cs4281_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime; let dma = (*runtime).private_data as *mut cs4281_dma; let chip = snd_pcm_substream_chip(substream);
    snd_cs4281_mode(chip, dma, runtime, 1, 1); 0
}
unsafe extern "C" fn snd_cs4281_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime; let dma = (*runtime).private_data as *mut cs4281_dma; let chip = snd_pcm_substream_chip(substream);
    ((*runtime).buffer_size - snd_cs4281_peekBA0(chip, (*dma).regDCC as c_ulong) - 1) as snd_pcm_uframes_t
}

static snd_cs4281_playback: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME, formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U16_LE | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_BE | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_U32_LE | SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_U32_BE | SNDRV_PCM_FMTBIT_S32_BE, rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000, rate_min: 4000, rate_max: 48000, channels_min: 1, channels_max: 2, buffer_bytes_max: 512 * 1024, period_bytes_min: 64, period_bytes_max: 512 * 1024, periods_min: 1, periods_max: 2, fifo_size: CS4281_FIFO_SIZE };
static snd_cs4281_capture: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME, formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U16_LE | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_BE | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_U32_LE | SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_U32_BE | SNDRV_PCM_FMTBIT_S32_BE, rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000, rate_min: 4000, rate_max: 48000, channels_min: 1, channels_max: 2, buffer_bytes_max: 512 * 1024, period_bytes_min: 64, period_bytes_max: 512 * 1024, periods_min: 1, periods_max: 2, fifo_size: CS4281_FIFO_SIZE };

unsafe extern "C" fn snd_cs4281_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream); let runtime = (*substream).runtime; let dma = &mut (*chip).dma[0] as *mut cs4281_dma;
    (*dma).substream = substream; (*dma).left_slot = 0; (*dma).right_slot = 1; (*runtime).private_data = dma as *mut c_void; (*runtime).hw = snd_cs4281_playback;
    snd_pcm_hw_constraint_msbits(runtime, 0, 32, 20); 0
}
unsafe extern "C" fn snd_cs4281_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream); let runtime = (*substream).runtime; let dma = &mut (*chip).dma[1] as *mut cs4281_dma;
    (*dma).substream = substream; (*dma).left_slot = 10; (*dma).right_slot = 11; (*runtime).private_data = dma as *mut c_void; (*runtime).hw = snd_cs4281_capture;
    snd_pcm_hw_constraint_msbits(runtime, 0, 32, 20); 0
}
unsafe extern "C" fn snd_cs4281_playback_close(substream: *mut snd_pcm_substream) -> c_int { let dma = (*(*substream).runtime).private_data as *mut cs4281_dma; (*dma).substream = ptr::null_mut(); 0 }
unsafe extern "C" fn snd_cs4281_capture_close(substream: *mut snd_pcm_substream) -> c_int { let dma = (*(*substream).runtime).private_data as *mut cs4281_dma; (*dma).substream = ptr::null_mut(); 0 }

static snd_cs4281_playback_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_cs4281_playback_open), close: Some(snd_cs4281_playback_close), prepare: Some(snd_cs4281_playback_prepare), trigger: Some(snd_cs4281_trigger), pointer: Some(snd_cs4281_pointer) };
static snd_cs4281_capture_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_cs4281_capture_open), close: Some(snd_cs4281_capture_close), prepare: Some(snd_cs4281_capture_prepare), trigger: Some(snd_cs4281_trigger), pointer: Some(snd_cs4281_pointer) };

unsafe fn snd_cs4281_pcm(chip: *mut cs4281, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let err = snd_pcm_new((*chip).card, b"CS4281\0".as_ptr() as *const c_char, device, 1, 1, &mut pcm);
    if err < 0 { return err; }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_cs4281_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_cs4281_capture_ops);
    (*pcm).private_data = chip as *mut c_void; (*pcm).info_flags = 0; strscpy((*pcm).name.as_mut_ptr(), b"CS4281\0".as_ptr() as *const c_char); (*chip).pcm = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 64 * 1024, 512 * 1024); 0
}

const CS_VOL_MASK: c_int = 0x1f;
unsafe extern "C" fn snd_cs4281_info_volume(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int { (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER; (*uinfo).count = 2; (*uinfo).value.integer.min = 0; (*uinfo).value.integer.max = CS_VOL_MASK as c_long; 0 }
unsafe extern "C" fn snd_cs4281_get_volume(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol); let regL = (((*kcontrol).private_value >> 16) & 0xffff) as c_ulong; let regR = ((*kcontrol).private_value & 0xffff) as c_ulong;
    (*ucontrol).value.integer.value[0] = (CS_VOL_MASK as c_uint - (snd_cs4281_peekBA0(chip, regL) & CS_VOL_MASK as c_uint)) as c_long;
    (*ucontrol).value.integer.value[1] = (CS_VOL_MASK as c_uint - (snd_cs4281_peekBA0(chip, regR) & CS_VOL_MASK as c_uint)) as c_long; 0
}
unsafe extern "C" fn snd_cs4281_put_volume(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol); let mut change = 0; let regL = (((*kcontrol).private_value >> 16) & 0xffff) as c_ulong; let regR = ((*kcontrol).private_value & 0xffff) as c_ulong;
    let mut volL = CS_VOL_MASK - (snd_cs4281_peekBA0(chip, regL) & CS_VOL_MASK as c_uint) as c_int; let mut volR = CS_VOL_MASK - (snd_cs4281_peekBA0(chip, regR) & CS_VOL_MASK as c_uint) as c_int;
    if (*ucontrol).value.integer.value[0] as c_int != volL { volL = CS_VOL_MASK - ((*ucontrol).value.integer.value[0] as c_int & CS_VOL_MASK); snd_cs4281_pokeBA0(chip, regL, volL as c_uint); change = 1; }
    if (*ucontrol).value.integer.value[1] as c_int != volR { volR = CS_VOL_MASK - ((*ucontrol).value.integer.value[1] as c_int & CS_VOL_MASK); snd_cs4281_pokeBA0(chip, regR, volR as c_uint); change = 1; }
    change
}
static db_scale_dsp: [c_uint; 4] = [0, (-4650i32) as c_uint, 150, 0];
static snd_cs4281_fm_vol: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Synth Playback Volume\0".as_ptr() as *const c_char, info: Some(snd_cs4281_info_volume), get: Some(snd_cs4281_get_volume), put: Some(snd_cs4281_put_volume), private_value: ((BA0_FMLVC << 16) | BA0_FMRVC) as c_ulong, tlv: snd_kcontrol_new_tlv { p: db_scale_dsp.as_ptr() } };
static snd_cs4281_pcm_vol: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"PCM Stream Playback Volume\0".as_ptr() as *const c_char, info: Some(snd_cs4281_info_volume), get: Some(snd_cs4281_get_volume), put: Some(snd_cs4281_put_volume), private_value: ((BA0_PPLVC << 16) | BA0_PPRVC) as c_ulong, tlv: snd_kcontrol_new_tlv { p: db_scale_dsp.as_ptr() } };

unsafe extern "C" fn snd_cs4281_mixer_free_ac97_bus(bus: *mut snd_ac97_bus) { let chip = (*bus).private_data as *mut cs4281; (*chip).ac97_bus = ptr::null_mut(); }
unsafe extern "C" fn snd_cs4281_mixer_free_ac97(ac97: *mut snd_ac97) { let chip = (*ac97).private_data as *mut cs4281; if (*ac97).num != 0 { (*chip).ac97_secondary = ptr::null_mut(); } else { (*chip).ac97 = ptr::null_mut(); } }
unsafe fn snd_cs4281_mixer(chip: *mut cs4281) -> c_int {
    let card = (*chip).card; let mut ac97: snd_ac97_template = mem::zeroed(); let ops = snd_ac97_bus_ops { write: Some(snd_cs4281_ac97_write), read: Some(snd_cs4281_ac97_read) };
    let mut err = snd_ac97_bus(card, 0, &ops, chip as *mut c_void, &mut (*chip).ac97_bus); if err < 0 { return err; }
    (*(*chip).ac97_bus).private_free = Some(snd_cs4281_mixer_free_ac97_bus);
    ac97.private_data = chip as *mut c_void; ac97.private_free = Some(snd_cs4281_mixer_free_ac97);
    err = snd_ac97_mixer((*chip).ac97_bus, &mut ac97, &mut (*chip).ac97); if err < 0 { return err; }
    if (*chip).dual_codec != 0 { ac97.num = 1; err = snd_ac97_mixer((*chip).ac97_bus, &mut ac97, &mut (*chip).ac97_secondary); if err < 0 { return err; } }
    err = snd_ctl_add(card, snd_ctl_new1(&snd_cs4281_fm_vol, chip as *mut c_void)); if err < 0 { return err; }
    err = snd_ctl_add(card, snd_ctl_new1(&snd_cs4281_pcm_vol, chip as *mut c_void)); if err < 0 { return err; } 0
}

unsafe extern "C" fn snd_cs4281_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data as *mut cs4281;
    snd_iprintf(buffer, b"Cirrus Logic CS4281\n\n\0".as_ptr() as *const c_char);
    snd_iprintf(buffer, b"Spurious half IRQs   : %u\n\0".as_ptr() as *const c_char, (*chip).spurious_dhtc_irq);
    snd_iprintf(buffer, b"Spurious end IRQs    : %u\n\0".as_ptr() as *const c_char, (*chip).spurious_dtc_irq);
}
unsafe extern "C" fn snd_cs4281_BA0_read(entry: *mut snd_info_entry, _fp: *mut c_void, _file: *mut file, buf: *mut c_char, count: size_t, pos: loff_t) -> ssize_t { let chip = (*entry).private_data as *mut cs4281; if copy_to_user_fromio(buf, ((*chip).ba0 as *mut u8).add(pos as usize) as *mut c_void, count) != 0 { return -EFAULT as ssize_t; } count as ssize_t }
unsafe extern "C" fn snd_cs4281_BA1_read(entry: *mut snd_info_entry, _fp: *mut c_void, _file: *mut file, buf: *mut c_char, count: size_t, pos: loff_t) -> ssize_t { let chip = (*entry).private_data as *mut cs4281; if copy_to_user_fromio(buf, ((*chip).ba1 as *mut u8).add(pos as usize) as *mut c_void, count) != 0 { return -EFAULT as ssize_t; } count as ssize_t }
static snd_cs4281_proc_ops_BA0: snd_info_entry_ops = snd_info_entry_ops { read: Some(snd_cs4281_BA0_read) };
static snd_cs4281_proc_ops_BA1: snd_info_entry_ops = snd_info_entry_ops { read: Some(snd_cs4281_BA1_read) };
unsafe fn snd_cs4281_proc_init(chip: *mut cs4281) {
    let mut entry: *mut snd_info_entry = ptr::null_mut();
    snd_card_ro_proc_new((*chip).card, b"cs4281\0".as_ptr() as *const c_char, chip as *mut c_void, Some(snd_cs4281_proc_read));
    if snd_card_proc_new((*chip).card, b"cs4281_BA0\0".as_ptr() as *const c_char, &mut entry) == 0 { (*entry).content = SNDRV_INFO_CONTENT_DATA; (*entry).private_data = chip as *mut c_void; (*entry).c.ops = &snd_cs4281_proc_ops_BA0; (*entry).size = CS4281_BA0_SIZE; }
    if snd_card_proc_new((*chip).card, b"cs4281_BA1\0".as_ptr() as *const c_char, &mut entry) == 0 { (*entry).content = SNDRV_INFO_CONTENT_DATA; (*entry).private_data = chip as *mut c_void; (*entry).c.ops = &snd_cs4281_proc_ops_BA1; (*entry).size = CS4281_BA1_SIZE; }
}

/* joystick support: translated as if IS_REACHABLE(CONFIG_GAMEPORT) is true; fallback functions are represented by the same names returning -ENOSYS/no-op when unavailable in integration. */
unsafe extern "C" fn snd_cs4281_gameport_trigger(gameport: *mut gameport) { let chip = gameport_get_port_data(gameport); if snd_BUG_ON(chip.is_null()) { return; } snd_cs4281_pokeBA0(chip, BA0_JSPT, 0xff); }
unsafe extern "C" fn snd_cs4281_gameport_read(gameport: *mut gameport) -> u8 { let chip = gameport_get_port_data(gameport); if snd_BUG_ON(chip.is_null()) { return 0; } snd_cs4281_peekBA0(chip, BA0_JSPT) as u8 }
/* COOKED_MODE conditional code preserved as active translation. */
unsafe extern "C" fn snd_cs4281_gameport_cooked_read(gameport: *mut gameport, axes: *mut c_int, buttons: *mut c_int) -> c_int {
    let chip = gameport_get_port_data(gameport); if snd_BUG_ON(chip.is_null()) { return 0; }
    let js1 = snd_cs4281_peekBA0(chip, BA0_JSC1); let js2 = snd_cs4281_peekBA0(chip, BA0_JSC2); let mut jst = snd_cs4281_peekBA0(chip, BA0_JSPT);
    *buttons = ((!jst >> 4) & 0x0f) as c_int;
    *axes.add(0) = (((js1 & JSC1_Y1V_MASK) >> JSC1_Y1V_SHIFT) & 0xffff) as c_int;
    *axes.add(1) = (((js1 & JSC1_X1V_MASK) >> JSC1_X1V_SHIFT) & 0xffff) as c_int;
    *axes.add(2) = (((js2 & JSC2_Y2V_MASK) >> JSC2_Y2V_SHIFT) & 0xffff) as c_int;
    *axes.add(3) = (((js2 & JSC2_X2V_MASK) >> JSC2_X2V_SHIFT) & 0xffff) as c_int;
    jst = 0; while jst < 4 { if *axes.add(jst as usize) == 0xffff { *axes.add(jst as usize) = -1; } jst += 1; } 0
}
unsafe extern "C" fn snd_cs4281_gameport_open(_gameport: *mut gameport, mode: c_int) -> c_int { match mode { GAMEPORT_MODE_COOKED | GAMEPORT_MODE_RAW => 0, _ => -1 } }
unsafe fn snd_cs4281_create_gameport(chip: *mut cs4281) -> c_int {
    let gp = gameport_allocate_port(); (*chip).gameport = gp; if gp.is_null() { dev_err((*(*chip).card).dev, b"cannot allocate memory for gameport\n\0".as_ptr() as *const c_char); return -ENOMEM; }
    gameport_set_name(gp, b"CS4281 Gameport\0".as_ptr() as *const c_char); gameport_set_phys(gp, b"pci%s/gameport0\0".as_ptr() as *const c_char, pci_name((*chip).pci)); gameport_set_dev_parent(gp, &mut (*(*chip).pci).dev);
    (*gp).open = Some(snd_cs4281_gameport_open); (*gp).read = Some(snd_cs4281_gameport_read); (*gp).trigger = Some(snd_cs4281_gameport_trigger); (*gp).cooked_read = Some(snd_cs4281_gameport_cooked_read); gameport_set_port_data(gp, chip as *mut c_void);
    snd_cs4281_pokeBA0(chip, BA0_JSIO, 0xff); snd_cs4281_pokeBA0(chip, BA0_JSCTL as c_ulong, JSCTL_SP_MEDIUM_SLOW); gameport_register_port(gp); 0
}
unsafe fn snd_cs4281_free_gameport(chip: *mut cs4281) { if !(*chip).gameport.is_null() { gameport_unregister_port((*chip).gameport); (*chip).gameport = ptr::null_mut(); } }

unsafe extern "C" fn snd_cs4281_free(card: *mut snd_card) {
    let chip = (*card).private_data as *mut cs4281;
    snd_cs4281_free_gameport(chip);
    snd_cs4281_pokeBA0(chip, BA0_HIMR, 0x7fffffff);
    snd_cs4281_pokeBA0(chip, BA0_CLKCR1, 0);
    snd_cs4281_pokeBA0(chip, BA0_SSPM, 0);
}

unsafe fn snd_cs4281_create(card: *mut snd_card, pci: *mut pci_dev, mut dual_codec_: c_int) -> c_int {
    let chip = (*card).private_data as *mut cs4281; let mut err = pcim_enable_device(pci); if err < 0 { return err; }
    spin_lock_init(&mut (*chip).reg_lock); (*chip).card = card; (*chip).pci = pci; (*chip).irq = -1; pci_set_master(pci);
    if dual_codec_ < 0 || dual_codec_ > 3 { dev_err((*card).dev, b"invalid dual_codec option %d\n\0".as_ptr() as *const c_char, dual_codec_); dual_codec_ = 0; }
    (*chip).dual_codec = dual_codec_;
    (*chip).ba0 = pcim_iomap_region(pci, 0, b"CS4281\0".as_ptr() as *const c_char); if IS_ERR((*chip).ba0) { return PTR_ERR((*chip).ba0); } (*chip).ba0_addr = pci_resource_start(pci, 0);
    (*chip).ba1 = pcim_iomap_region(pci, 1, b"CS4281\0".as_ptr() as *const c_char); if IS_ERR((*chip).ba1) { return PTR_ERR((*chip).ba1); } (*chip).ba1_addr = pci_resource_start(pci, 1);
    if devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_cs4281_interrupt, IRQF_SHARED, KBUILD_MODNAME.as_ptr() as *const c_char, chip as *mut c_void) != 0 { dev_err((*card).dev, b"unable to grab IRQ %d\n\0".as_ptr() as *const c_char, (*pci).irq); return -ENOMEM; }
    (*chip).irq = (*pci).irq; (*card).sync_irq = (*chip).irq; (*card).private_free = Some(snd_cs4281_free);
    err = snd_cs4281_chip_init(chip); if err != 0 { return err; }
    snd_cs4281_proc_init(chip); 0
}

unsafe fn snd_cs4281_chip_init(chip: *mut cs4281) -> c_int {
    let mut tmp = snd_cs4281_peekBA0(chip, BA0_EPPMC); let mut retry_count = 2;
    if tmp & BA0_EPPMC_FPDN != 0 { snd_cs4281_pokeBA0(chip, BA0_EPPMC, tmp & !BA0_EPPMC_FPDN); }
    'retry: loop {
        tmp = snd_cs4281_peekBA0(chip, BA0_CFLR);
        if tmp != BA0_CFLR_DEFAULT { snd_cs4281_pokeBA0(chip, BA0_CFLR, BA0_CFLR_DEFAULT); tmp = snd_cs4281_peekBA0(chip, BA0_CFLR); if tmp != BA0_CFLR_DEFAULT { dev_err((*(*chip).card).dev, b"CFLR setup failed (0x%x)\n\0".as_ptr() as *const c_char, tmp); return -EIO; } }
        snd_cs4281_pokeBA0(chip, BA0_CWPR, 0x4281);
        tmp = snd_cs4281_peekBA0(chip, BA0_SERC1); if tmp != BA0_SERC1_SO1EN | BA0_SERC1_AC97 { dev_err((*(*chip).card).dev, b"SERC1 AC'97 check failed (0x%x)\n\0".as_ptr() as *const c_char, tmp); return -EIO; }
        tmp = snd_cs4281_peekBA0(chip, BA0_SERC2); if tmp != BA0_SERC2_SI1EN | BA0_SERC2_AC97 { dev_err((*(*chip).card).dev, b"SERC2 AC'97 check failed (0x%x)\n\0".as_ptr() as *const c_char, tmp); return -EIO; }
        snd_cs4281_pokeBA0(chip, BA0_SSPM, BA0_SSPM_MIXEN | BA0_SSPM_CSRCEN | BA0_SSPM_PSRCEN | BA0_SSPM_JSEN | BA0_SSPM_ACLEN | BA0_SSPM_FMEN);
        snd_cs4281_pokeBA0(chip, BA0_CLKCR1, 0); snd_cs4281_pokeBA0(chip, BA0_SERMC, 0); snd_cs4281_pokeBA0(chip, BA0_ACCTL, 0); udelay(50);
        snd_cs4281_pokeBA0(chip, BA0_SPMC, 0); udelay(50); snd_cs4281_pokeBA0(chip, BA0_SPMC, BA0_SPMC_RSTN); msleep(50);
        if (*chip).dual_codec != 0 { snd_cs4281_pokeBA0(chip, BA0_SPMC, BA0_SPMC_RSTN | BA0_SPMC_ASDI2E); }
        snd_cs4281_pokeBA0(chip, BA0_SERMC, (if (*chip).dual_codec != 0 { BA0_SERMC_TCID((*chip).dual_codec) } else { BA0_SERMC_TCID(1) }) | BA0_SERMC_PTC_AC97 | BA0_SERMC_MSPE);
        snd_cs4281_pokeBA0(chip, BA0_CLKCR1, BA0_CLKCR1_DLLP); msleep(50); snd_cs4281_pokeBA0(chip, BA0_CLKCR1, BA0_CLKCR1_SWCE | BA0_CLKCR1_DLLP);
        let mut end_time = jiffies.wrapping_add(HZ); loop { if snd_cs4281_peekBA0(chip, BA0_CLKCR1) & BA0_CLKCR1_DLLRDY != 0 { break; } schedule_timeout_uninterruptible(1); if !time_after_eq(end_time, jiffies) { dev_err((*(*chip).card).dev, b"DLLRDY not seen\n\0".as_ptr() as *const c_char); return -EIO; } }
        snd_cs4281_pokeBA0(chip, BA0_ACCTL, BA0_ACCTL_ESYN);
        end_time = jiffies.wrapping_add(HZ); loop { if snd_cs4281_peekBA0(chip, BA0_ACSTS) & BA0_ACSTS_CRDY != 0 { break; } schedule_timeout_uninterruptible(1); if !time_after_eq(end_time, jiffies) { dev_err((*(*chip).card).dev, b"never read codec ready from AC'97 (0x%x)\n\0".as_ptr() as *const c_char, snd_cs4281_peekBA0(chip, BA0_ACSTS)); return -EIO; } }
        if (*chip).dual_codec != 0 { end_time = jiffies.wrapping_add(HZ); loop { if snd_cs4281_peekBA0(chip, BA0_ACSTS2) & BA0_ACSTS_CRDY != 0 { break; } schedule_timeout_uninterruptible(1); if !time_after_eq(end_time, jiffies) { dev_info((*(*chip).card).dev, b"secondary codec doesn't respond. disable it...\n\0".as_ptr() as *const c_char); (*chip).dual_codec = 0; break; } } }
        snd_cs4281_pokeBA0(chip, BA0_ACCTL, BA0_ACCTL_VFRM | BA0_ACCTL_ESYN);
        end_time = jiffies.wrapping_add(HZ); loop {
            if snd_cs4281_peekBA0(chip, BA0_ACISV) & (BA0_ACISV_SLV(3) | BA0_ACISV_SLV(4)) == (BA0_ACISV_SLV(3) | BA0_ACISV_SLV(4)) { break; }
            schedule_timeout_uninterruptible(1);
            if !time_after_eq(end_time, jiffies) { retry_count -= 1; if retry_count > 0 { continue 'retry; } dev_err((*(*chip).card).dev, b"never read ISV3 and ISV4 from AC'97\n\0".as_ptr() as *const c_char); return -EIO; }
        }
        break;
    }
    snd_cs4281_pokeBA0(chip, BA0_ACOSV, BA0_ACOSV_SLV(3) | BA0_ACOSV_SLV(4));
    for tmp_i in 0..4u32 { let dma = &mut (*chip).dma[tmp_i as usize]; dma.regDBA = BA0_DBA0 + tmp_i * 0x10; dma.regDCA = BA0_DCA0 + tmp_i * 0x10; dma.regDBC = BA0_DBC0 + tmp_i * 0x10; dma.regDCC = BA0_DCC0 + tmp_i * 0x10; dma.regDMR = BA0_DMR0 + tmp_i * 8; dma.regDCR = BA0_DCR0 + tmp_i * 8; dma.regHDSR = BA0_HDSR0 + tmp_i * 4; dma.regFCR = BA0_FCR0 + tmp_i * 4; dma.regFSIC = BA0_FSIC0 + tmp_i * 4; dma.fifo_offset = tmp_i * CS4281_FIFO_SIZE; snd_cs4281_pokeBA0(chip, dma.regFCR as c_ulong, BA0_FCR_LS(31) | BA0_FCR_RS(31) | BA0_FCR_SZ(CS4281_FIFO_SIZE) | BA0_FCR_OF(dma.fifo_offset)); }
    (*chip).src_left_play_slot = 0; (*chip).src_right_play_slot = 1; (*chip).src_left_rec_slot = 10; (*chip).src_right_rec_slot = 11;
    (*chip).dma[0].valFCR = BA0_FCR_FEN | BA0_FCR_LS(0) | BA0_FCR_RS(1) | BA0_FCR_SZ(CS4281_FIFO_SIZE) | BA0_FCR_OF((*chip).dma[0].fifo_offset);
    snd_cs4281_pokeBA0(chip, (*chip).dma[0].regFCR as c_ulong, (*chip).dma[0].valFCR);
    snd_cs4281_pokeBA0(chip, BA0_SRCSA, ((*chip).src_left_play_slot as c_uint) | ((*chip).src_right_play_slot as c_uint) << 8 | ((*chip).src_left_rec_slot as c_uint) << 16 | ((*chip).src_right_rec_slot as c_uint) << 24);
    snd_cs4281_pokeBA0(chip, BA0_PPLVC as c_ulong, 0); snd_cs4281_pokeBA0(chip, BA0_PPRVC as c_ulong, 0);
    snd_cs4281_pokeBA0(chip, BA0_HICR, BA0_HICR_EOI);
    snd_cs4281_pokeBA0(chip, BA0_HIMR, 0x7fffffff & !(BA0_HISR_MIDI | BA0_HISR_DMAI | BA0_HISR_DMA(0) | BA0_HISR_DMA(1) | BA0_HISR_DMA(2) | BA0_HISR_DMA(3)));
    0
}

unsafe fn snd_cs4281_midi_reset(chip: *mut cs4281) { snd_cs4281_pokeBA0(chip, BA0_MIDCR, (*chip).midcr | BA0_MIDCR_MRST); udelay(100); snd_cs4281_pokeBA0(chip, BA0_MIDCR, (*chip).midcr); }
unsafe extern "C" fn snd_cs4281_midi_input_open(substream: *mut snd_rawmidi_substream) -> c_int { let chip = (*(*substream).rmidi).private_data as *mut cs4281; (*chip).midcr |= BA0_MIDCR_RXE; (*chip).midi_input = substream; if (*chip).uartm & CS4281_MODE_OUTPUT == 0 { snd_cs4281_midi_reset(chip); } else { snd_cs4281_pokeBA0(chip, BA0_MIDCR, (*chip).midcr); } 0 }
unsafe extern "C" fn snd_cs4281_midi_input_close(substream: *mut snd_rawmidi_substream) -> c_int { let chip = (*(*substream).rmidi).private_data as *mut cs4281; (*chip).midcr &= !(BA0_MIDCR_RXE | BA0_MIDCR_RIE); (*chip).midi_input = ptr::null_mut(); if (*chip).uartm & CS4281_MODE_OUTPUT == 0 { snd_cs4281_midi_reset(chip); } else { snd_cs4281_pokeBA0(chip, BA0_MIDCR, (*chip).midcr); } (*chip).uartm &= !CS4281_MODE_INPUT; 0 }
unsafe extern "C" fn snd_cs4281_midi_output_open(substream: *mut snd_rawmidi_substream) -> c_int { let chip = (*(*substream).rmidi).private_data as *mut cs4281; (*chip).uartm |= CS4281_MODE_OUTPUT; (*chip).midcr |= BA0_MIDCR_TXE; (*chip).midi_output = substream; if (*chip).uartm & CS4281_MODE_INPUT == 0 { snd_cs4281_midi_reset(chip); } else { snd_cs4281_pokeBA0(chip, BA0_MIDCR, (*chip).midcr); } 0 }
unsafe extern "C" fn snd_cs4281_midi_output_close(substream: *mut snd_rawmidi_substream) -> c_int { let chip = (*(*substream).rmidi).private_data as *mut cs4281; (*chip).midcr &= !(BA0_MIDCR_TXE | BA0_MIDCR_TIE); (*chip).midi_output = ptr::null_mut(); if (*chip).uartm & CS4281_MODE_INPUT == 0 { snd_cs4281_midi_reset(chip); } else { snd_cs4281_pokeBA0(chip, BA0_MIDCR, (*chip).midcr); } (*chip).uartm &= !CS4281_MODE_OUTPUT; 0 }
unsafe extern "C" fn snd_cs4281_midi_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) { let chip = (*(*substream).rmidi).private_data as *mut cs4281; if up != 0 { if (*chip).midcr & BA0_MIDCR_RIE == 0 { (*chip).midcr |= BA0_MIDCR_RIE; snd_cs4281_pokeBA0(chip, BA0_MIDCR, (*chip).midcr); } } else if (*chip).midcr & BA0_MIDCR_RIE != 0 { (*chip).midcr &= !BA0_MIDCR_RIE; snd_cs4281_pokeBA0(chip, BA0_MIDCR, (*chip).midcr); } }
unsafe extern "C" fn snd_cs4281_midi_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let chip = (*(*substream).rmidi).private_data as *mut cs4281; let mut byte: u8 = 0;
    if up != 0 { if (*chip).midcr & BA0_MIDCR_TIE == 0 { (*chip).midcr |= BA0_MIDCR_TIE; while (*chip).midcr & BA0_MIDCR_TIE != 0 && snd_cs4281_peekBA0(chip, BA0_MIDSR) & BA0_MIDSR_TBF == 0 { if snd_rawmidi_transmit(substream, &mut byte, 1) != 1 { (*chip).midcr &= !BA0_MIDCR_TIE; } else { snd_cs4281_pokeBA0(chip, BA0_MIDWP, byte as c_uint); } } snd_cs4281_pokeBA0(chip, BA0_MIDCR, (*chip).midcr); } } else if (*chip).midcr & BA0_MIDCR_TIE != 0 { (*chip).midcr &= !BA0_MIDCR_TIE; snd_cs4281_pokeBA0(chip, BA0_MIDCR, (*chip).midcr); }
}
static snd_cs4281_midi_output: snd_rawmidi_ops = snd_rawmidi_ops { open: Some(snd_cs4281_midi_output_open), close: Some(snd_cs4281_midi_output_close), trigger: Some(snd_cs4281_midi_output_trigger) };
static snd_cs4281_midi_input: snd_rawmidi_ops = snd_rawmidi_ops { open: Some(snd_cs4281_midi_input_open), close: Some(snd_cs4281_midi_input_close), trigger: Some(snd_cs4281_midi_input_trigger) };
unsafe fn snd_cs4281_midi(chip: *mut cs4281, device: c_int) -> c_int { let mut rmidi: *mut snd_rawmidi = ptr::null_mut(); let err = snd_rawmidi_new((*chip).card, b"CS4281\0".as_ptr() as *const c_char, device, 1, 1, &mut rmidi); if err < 0 { return err; } strscpy((*rmidi).name.as_mut_ptr(), b"CS4281\0".as_ptr() as *const c_char); snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_cs4281_midi_output); snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_cs4281_midi_input); (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX; (*rmidi).private_data = chip as *mut c_void; (*chip).rmidi = rmidi; 0 }

unsafe extern "C" fn snd_cs4281_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut cs4281; if chip.is_null() { return IRQ_NONE; }
    let status = snd_cs4281_peekBA0(chip, BA0_HISR); if status & 0x7fffffff == 0 { snd_cs4281_pokeBA0(chip, BA0_HICR, BA0_HICR_EOI); return IRQ_NONE; }
    if status & (BA0_HISR_DMA(0) | BA0_HISR_DMA(1) | BA0_HISR_DMA(2) | BA0_HISR_DMA(3)) != 0 { for dma in 0..4u32 { let cdma = &mut (*chip).dma[dma as usize] as *mut cs4281_dma; let mut period_elapsed = false; if status & BA0_HISR_DMA(dma) != 0 { let val = snd_cs4281_peekBA0(chip, (*cdma).regHDSR as c_ulong); (*cdma).frag += 1; if val & BA0_HDSR_DHTC != 0 && ((*cdma).frag & 1) == 0 { (*cdma).frag -= 1; (*chip).spurious_dhtc_irq += 1; continue; } if val & BA0_HDSR_DTC != 0 && ((*cdma).frag & 1) != 0 { (*cdma).frag -= 1; (*chip).spurious_dtc_irq += 1; continue; } period_elapsed = true; } if period_elapsed { snd_pcm_period_elapsed((*cdma).substream); } } }
    if status & BA0_HISR_MIDI != 0 && !(*chip).rmidi.is_null() { let mut c: u8 = 0; while snd_cs4281_peekBA0(chip, BA0_MIDSR) & BA0_MIDSR_RBE == 0 { c = snd_cs4281_peekBA0(chip, BA0_MIDRP) as u8; if (*chip).midcr & BA0_MIDCR_RIE == 0 { continue; } snd_rawmidi_receive((*chip).midi_input, &mut c, 1); } while snd_cs4281_peekBA0(chip, BA0_MIDSR) & BA0_MIDSR_TBF == 0 { if (*chip).midcr & BA0_MIDCR_TIE == 0 { break; } if snd_rawmidi_transmit((*chip).midi_output, &mut c, 1) != 1 { (*chip).midcr &= !BA0_MIDCR_TIE; snd_cs4281_pokeBA0(chip, BA0_MIDCR, (*chip).midcr); break; } snd_cs4281_pokeBA0(chip, BA0_MIDWP, c as c_uint); } }
    snd_cs4281_pokeBA0(chip, BA0_HICR, BA0_HICR_EOI); IRQ_HANDLED
}

unsafe extern "C" fn snd_cs4281_opl3_command(opl3: *mut snd_opl3, cmd: u16, val: u8) {
    let chip = (*opl3).private_data as *mut cs4281; let port = if cmd & OPL3_RIGHT != 0 { ((*chip).ba0 as *mut u8).add(BA0_B1AP as usize) } else { ((*chip).ba0 as *mut u8).add(BA0_B0AP as usize) };
    writel(cmd as c_uint, port as *mut c_void); udelay(10); writel(val as c_uint, port.add(4) as *mut c_void); udelay(30);
}

unsafe extern "C" fn __snd_cs4281_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0; let mut card: *mut snd_card = ptr::null_mut(); let mut opl3: *mut snd_opl3 = ptr::null_mut();
    if dev >= SNDRV_CARDS as c_int { return -ENODEV; } if !enable[dev as usize] { dev += 1; return -ENOENT; }
    let mut err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, mem::size_of::<cs4281>(), &mut card); if err < 0 { return err; }
    let chip = (*card).private_data as *mut cs4281;
    err = snd_cs4281_create(card, pci, dual_codec[dev as usize] as c_int); if err < 0 { return err; }
    err = snd_cs4281_mixer(chip); if err < 0 { return err; }
    err = snd_cs4281_pcm(chip, 0); if err < 0 { return err; }
    err = snd_cs4281_midi(chip, 0); if err < 0 { return err; }
    err = snd_opl3_new(card, OPL3_HW_OPL3_CS4281, &mut opl3); if err < 0 { return err; }
    (*opl3).private_data = chip as *mut c_void; (*opl3).command = Some(snd_cs4281_opl3_command); snd_opl3_init(opl3);
    err = snd_opl3_hwdep_new(opl3, 0, 1, ptr::null_mut()); if err < 0 { return err; }
    snd_cs4281_create_gameport(chip); strscpy((*card).driver.as_mut_ptr(), b"CS4281\0".as_ptr() as *const c_char); strscpy((*card).shortname.as_mut_ptr(), b"Cirrus Logic CS4281\0".as_ptr() as *const c_char);
    sprintf((*card).longname.as_mut_ptr(), b"%s at 0x%lx, irq %d\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), (*chip).ba0_addr, (*chip).irq);
    err = snd_card_register(card); if err < 0 { return err; } pci_set_drvdata(pci, card as *mut c_void); dev += 1; 0
}
unsafe extern "C" fn snd_cs4281_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int { snd_card_free_on_error(&mut (*pci).dev, __snd_cs4281_probe(pci, pci_id)) }

static saved_regs: [c_int; SUSPEND_REGISTERS] = [BA0_JSCTL as c_int, BA0_GPIOR as c_int, BA0_SSCR as c_int, BA0_MIDCR as c_int, BA0_SRCSA as c_int, BA0_PASR as c_int, BA0_CASR as c_int, BA0_DACSR as c_int, BA0_ADCSR as c_int, BA0_FMLVC as c_int, BA0_FMRVC as c_int, BA0_PPLVC as c_int, BA0_PPRVC as c_int, 0, 0, 0, 0, 0, 0, 0];
const CLKCR1_CKRA: u32 = 0x00010000;

unsafe extern "C" fn cs4281_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card; let chip = (*card).private_data as *mut cs4281; let mut ulCLK: u32;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot); snd_ac97_suspend((*chip).ac97); snd_ac97_suspend((*chip).ac97_secondary);
    ulCLK = snd_cs4281_peekBA0(chip, BA0_CLKCR1); ulCLK |= CLKCR1_CKRA; snd_cs4281_pokeBA0(chip, BA0_CLKCR1, ulCLK);
    snd_cs4281_pokeBA0(chip, BA0_HICR, BA0_HICR_CHGM);
    for i in 0..saved_regs.len() { if saved_regs[i] != 0 { (*chip).suspend_regs[i] = snd_cs4281_peekBA0(chip, saved_regs[i] as c_ulong); } }
    snd_cs4281_pokeBA0(chip, BA0_SERMC, 0); snd_cs4281_pokeBA0(chip, BA0_SSPM, 0); snd_cs4281_pokeBA0(chip, BA0_CLKCR1, 0); snd_cs4281_pokeBA0(chip, BA0_SPMC, 0);
    ulCLK = snd_cs4281_peekBA0(chip, BA0_CLKCR1); ulCLK &= !CLKCR1_CKRA; snd_cs4281_pokeBA0(chip, BA0_CLKCR1, ulCLK); 0
}
unsafe extern "C" fn cs4281_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card; let chip = (*card).private_data as *mut cs4281; let mut ulCLK = snd_cs4281_peekBA0(chip, BA0_CLKCR1);
    ulCLK |= CLKCR1_CKRA; snd_cs4281_pokeBA0(chip, BA0_CLKCR1, ulCLK); snd_cs4281_chip_init(chip);
    for i in 0..saved_regs.len() { if saved_regs[i] != 0 { snd_cs4281_pokeBA0(chip, saved_regs[i] as c_ulong, (*chip).suspend_regs[i]); } }
    snd_ac97_resume((*chip).ac97); snd_ac97_resume((*chip).ac97_secondary);
    ulCLK = snd_cs4281_peekBA0(chip, BA0_CLKCR1); ulCLK &= !CLKCR1_CKRA; snd_cs4281_pokeBA0(chip, BA0_CLKCR1, ulCLK);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0); 0
}

static cs4281_pm: dev_pm_ops = dev_pm_ops { _priv: [] };
static mut cs4281_driver: pci_driver = pci_driver { name: KBUILD_MODNAME.as_ptr() as *const c_char, id_table: snd_cs4281_ids.as_ptr(), probe: Some(snd_cs4281_probe), driver: pci_driver_inner { pm: &cs4281_pm } };
/* module_pci_driver(cs4281_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
