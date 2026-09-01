// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for Ensoniq ES1370/ES1371 AudioPCI soundcard
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
 *              Thomas Sailer <sailer@ife.ee.ethz.ch>
 */

/* Power-Management-Code ( CONFIG_PM )
 * for ens1371 only ( FIXME )
 * derived from cs4281.c, atiixp.c and via82xx.c
 * using https://www.kernel.org/doc/html/latest/sound/kernel-api/writing-an-alsa-driver.html
 * by Kurt J. Bosch
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

/* Includes in the C source name Linux/ALSA dependencies supplied by other files. */
/* CHIP1370/CHIP1371 and SUPPORT_JOYSTICK are build-time configuration conditions. */

pub const DRIVER_NAME_1370: &[u8] = b"ENS1370\0";
pub const CHIP_NAME_1370: &[u8] = b"ES1370\0";
pub const DRIVER_NAME_1371: &[u8] = b"ENS1371\0";
pub const CHIP_NAME_1371: &[u8] = b"ES1371\0";

pub const ES1371REV_ES1373_A: c_uint = 0x04;
pub const ES1371REV_ES1373_B: c_uint = 0x06;
pub const ES1371REV_CT5880_A: c_uint = 0x07;
pub const CT5880REV_CT5880_C: c_uint = 0x02;
pub const CT5880REV_CT5880_D: c_uint = 0x03; /* ??? -jk */
pub const CT5880REV_CT5880_E: c_uint = 0x04; /* mw */
pub const ES1371REV_ES1371_B: c_uint = 0x09;
pub const EV1938REV_EV1938_A: c_uint = 0x00;
pub const ES1371REV_ES1373_8: c_uint = 0x08;

#[inline]
pub unsafe fn ES_REG(ensoniq: *mut ensoniq, x: c_ulong) -> c_ulong {
    unsafe { (*ensoniq).port.wrapping_add(x) }
}

pub const ES_REG_CONTROL: c_ulong = 0x00;
pub const ES_1370_ADC_STOP: c_uint = 1 << 31;
pub const ES_1370_XCTL1: c_uint = 1 << 30;
pub const ES_1373_BYPASS_P1: c_uint = 1 << 31;
pub const ES_1373_BYPASS_P2: c_uint = 1 << 30;
pub const ES_1373_BYPASS_R: c_uint = 1 << 29;
pub const ES_1373_TEST_BIT: c_uint = 1 << 28;
pub const ES_1373_RECEN_B: c_uint = 1 << 27;
pub const ES_1373_SPDIF_THRU: c_uint = 1 << 26;
#[inline] pub const fn ES_1371_JOY_ASEL(o: c_uint) -> c_uint { ((o) & 0x03) << 24 }
pub const ES_1371_JOY_ASELM: c_uint = 0x03 << 24;
#[inline] pub const fn ES_1371_JOY_ASELI(i: c_uint) -> c_uint { ((i) >> 24) & 0x03 }
#[inline] pub const fn ES_1371_GPIO_IN(i: c_uint) -> c_uint { ((i) >> 20) & 0x0f }
#[inline] pub const fn ES_1370_PCLKDIVO(o: c_uint) -> c_uint { ((o) & 0x1fff) << 16 }
pub const ES_1370_PCLKDIVM: c_uint = 0x1fff << 16;
#[inline] pub const fn ES_1370_PCLKDIVI(i: c_uint) -> c_uint { ((i) >> 16) & 0x1fff }
#[inline] pub const fn ES_1371_GPIO_OUT(o: c_uint) -> c_uint { ((o) & 0x0f) << 16 }
pub const ES_1371_GPIO_OUTM: c_uint = 0x0f << 16;
pub const ES_MSFMTSEL: c_uint = 1 << 15;
pub const ES_1370_M_SBB: c_uint = 1 << 14;
pub const ES_1371_SYNC_RES: c_uint = 1 << 14;
#[inline] pub const fn ES_1370_WTSRSEL(o: c_uint) -> c_uint { ((o) & 0x03) << 12 }
pub const ES_1370_WTSRSELM: c_uint = 0x03 << 12;
pub const ES_1371_ADC_STOP: c_uint = 1 << 13;
pub const ES_1371_PWR_INTRM: c_uint = 1 << 12;
pub const ES_1370_DAC_SYNC: c_uint = 1 << 11;
pub const ES_1371_M_CB: c_uint = 1 << 11;
pub const ES_CCB_INTRM: c_uint = 1 << 10;
pub const ES_1370_M_CB: c_uint = 1 << 9;
pub const ES_1370_XCTL0: c_uint = 1 << 8;
#[inline] pub const fn ES_1371_PDLEV(o: c_uint) -> c_uint { ((o) & 0x03) << 8 }
pub const ES_1371_PDLEVM: c_uint = 0x03 << 8;
pub const ES_BREQ: c_uint = 1 << 7;
pub const ES_DAC1_EN: c_uint = 1 << 6;
pub const ES_DAC2_EN: c_uint = 1 << 5;
pub const ES_ADC_EN: c_uint = 1 << 4;
pub const ES_UART_EN: c_uint = 1 << 3;
pub const ES_JYSTK_EN: c_uint = 1 << 2;
pub const ES_1370_CDC_EN: c_uint = 1 << 1;
pub const ES_1371_XTALCKDIS: c_uint = 1 << 1;
pub const ES_1370_SERR_DISABLE: c_uint = 1 << 0;
pub const ES_1371_PCICLKDIS: c_uint = 1 << 0;

pub const ES_REG_STATUS: c_ulong = 0x04;
pub const ES_INTR: c_uint = 1 << 31;
pub const ES_1371_ST_AC97_RST: c_uint = 1 << 29;
pub const ES_1373_REAR_BIT27: c_uint = 1 << 27;
pub const ES_1373_REAR_BIT26: c_uint = 1 << 26;
pub const ES_1373_REAR_BIT24: c_uint = 1 << 24;
#[inline] pub const fn ES_1373_GPIO_INT_EN(o: c_uint) -> c_uint { ((o) & 0x0f) << 20 }
pub const ES_1373_SPDIF_EN: c_uint = 1 << 18;
pub const ES_1373_SPDIF_TEST: c_uint = 1 << 17;
pub const ES_1371_TEST: c_uint = 1 << 16;
#[inline] pub const fn ES_1373_GPIO_INT(i: c_uint) -> c_uint { ((i) & 0x0f) >> 12 }
pub const ES_1370_CSTAT: c_uint = 1 << 10;
pub const ES_1370_CBUSY: c_uint = 1 << 9;
pub const ES_1370_CWRIP: c_uint = 1 << 8;
pub const ES_1371_SYNC_ERR: c_uint = 1 << 8;
#[inline] pub const fn ES_1371_VC(i: c_uint) -> c_uint { ((i) >> 6) & 0x03 }
#[inline] pub const fn ES_1370_VC(i: c_uint) -> c_uint { ((i) >> 5) & 0x03 }
pub const ES_1371_MPWR: c_uint = 1 << 5;
pub const ES_MCCB: c_uint = 1 << 4;
pub const ES_UART: c_uint = 1 << 3;
pub const ES_DAC1: c_uint = 1 << 2;
pub const ES_DAC2: c_uint = 1 << 1;
pub const ES_ADC: c_uint = 1 << 0;

pub const ES_REG_UART_DATA: c_ulong = 0x08;
pub const ES_REG_UART_STATUS: c_ulong = 0x09;
pub const ES_RXINT: c_uint = 1 << 7;
pub const ES_TXINT: c_uint = 1 << 2;
pub const ES_TXRDY: c_uint = 1 << 1;
pub const ES_RXRDY: c_uint = 1 << 0;
pub const ES_REG_UART_CONTROL: c_ulong = 0x09;
pub const ES_RXINTEN: c_uint = 1 << 7;
#[inline] pub const fn ES_TXINTENO(o: c_uint) -> c_uint { ((o) & 0x03) << 5 }
pub const ES_TXINTENM: c_uint = 0x03 << 5;
#[inline] pub const fn ES_TXINTENI(i: c_uint) -> c_uint { ((i) >> 5) & 0x03 }
#[inline] pub const fn ES_CNTRL(o: c_uint) -> c_uint { ((o) & 0x03) << 0 }
pub const ES_CNTRLM: c_uint = 0x03 << 0;
pub const ES_REG_UART_RES: c_ulong = 0x0a;
pub const ES_TEST_MODE: c_uint = 1 << 0;
pub const ES_REG_MEM_PAGE: c_ulong = 0x0c;
#[inline] pub const fn ES_MEM_PAGEO(o: c_uint) -> c_uint { ((o) & 0x0f) << 0 }
pub const ES_MEM_PAGEM: c_uint = 0x0f << 0;
#[inline] pub const fn ES_MEM_PAGEI(i: c_uint) -> c_uint { ((i) >> 0) & 0x0f }
pub const ES_REG_1370_CODEC: c_ulong = 0x10;
#[inline] pub const fn ES_1370_CODEC_WRITE(a: c_uint, d: c_uint) -> c_uint { (((a) & 0xff) << 8) | (((d) & 0xff) << 0) }
pub const ES_REG_1371_CODEC: c_ulong = 0x14;
pub const ES_1371_CODEC_RDY: c_uint = 1 << 31;
pub const ES_1371_CODEC_WIP: c_uint = 1 << 30;
pub const EV_1938_CODEC_MAGIC: c_uint = 1 << 26;
pub const ES_1371_CODEC_PIRD: c_uint = 1 << 23;
#[inline] pub const fn ES_1371_CODEC_WRITE(a: c_uint, d: c_uint) -> c_uint { (((a) & 0x7f) << 16) | (((d) & 0xffff) << 0) }
#[inline] pub const fn ES_1371_CODEC_READS(a: c_uint) -> c_uint { (((a) & 0x7f) << 16) | ES_1371_CODEC_PIRD }
#[inline] pub const fn ES_1371_CODEC_READ(i: c_uint) -> c_ushort { (((i) >> 0) & 0xffff) as c_ushort }

pub const ES_REG_1371_SMPRATE: c_ulong = 0x10;
#[inline] pub const fn ES_1371_SRC_RAM_ADDRO(o: c_uint) -> c_uint { ((o) & 0x7f) << 25 }
pub const ES_1371_SRC_RAM_ADDRM: c_uint = 0x7f << 25;
#[inline] pub const fn ES_1371_SRC_RAM_ADDRI(i: c_uint) -> c_uint { ((i) >> 25) & 0x7f }
pub const ES_1371_SRC_RAM_WE: c_uint = 1 << 24;
pub const ES_1371_SRC_RAM_BUSY: c_uint = 1 << 23;
pub const ES_1371_SRC_DISABLE: c_uint = 1 << 22;
pub const ES_1371_DIS_P1: c_uint = 1 << 21;
pub const ES_1371_DIS_P2: c_uint = 1 << 20;
pub const ES_1371_DIS_R1: c_uint = 1 << 19;
#[inline] pub const fn ES_1371_SRC_RAM_DATAO(o: c_uint) -> c_uint { ((o) & 0xffff) << 0 }
pub const ES_1371_SRC_RAM_DATAM: c_uint = 0xffff << 0;
#[inline] pub const fn ES_1371_SRC_RAM_DATAI(i: c_uint) -> c_uint { ((i) >> 0) & 0xffff }

pub const ES_REG_1371_LEGACY: c_ulong = 0x18;
pub const ES_1371_JFAST: c_uint = 1 << 31;
pub const ES_1371_HIB: c_uint = 1 << 30;
pub const ES_1371_VSB: c_uint = 1 << 29;
#[inline] pub const fn ES_1371_VMPUO(o: c_uint) -> c_uint { ((o) & 0x03) << 27 }
pub const ES_1371_VMPUM: c_uint = 0x03 << 27;
#[inline] pub const fn ES_1371_VMPUI(i: c_uint) -> c_uint { ((i) >> 27) & 0x03 }
#[inline] pub const fn ES_1371_VCDCO(o: c_uint) -> c_uint { ((o) & 0x03) << 25 }
pub const ES_1371_VCDCM: c_uint = 0x03 << 25;
#[inline] pub const fn ES_1371_VCDCI(i: c_uint) -> c_uint { ((i) >> 25) & 0x03 }
pub const ES_1371_FIRQ: c_uint = 1 << 24;
pub const ES_1371_SDMACAP: c_uint = 1 << 23;
pub const ES_1371_SPICAP: c_uint = 1 << 22;
pub const ES_1371_MDMACAP: c_uint = 1 << 21;
pub const ES_1371_MPICAP: c_uint = 1 << 20;
pub const ES_1371_ADCAP: c_uint = 1 << 19;
pub const ES_1371_SVCAP: c_uint = 1 << 18;
pub const ES_1371_CDCCAP: c_uint = 1 << 17;
pub const ES_1371_BACAP: c_uint = 1 << 16;
#[inline] pub const fn ES_1371_EXI(i: c_uint) -> c_uint { ((i) >> 8) & 0x07 }
#[inline] pub const fn ES_1371_AI(i: c_uint) -> c_uint { ((i) >> 3) & 0x1f }
pub const ES_1371_WR: c_uint = 1 << 2;
pub const ES_1371_LEGINT: c_uint = 1 << 0;

pub const ES_REG_CHANNEL_STATUS: c_ulong = 0x1c;
pub const ES_REG_SERIAL: c_ulong = 0x20;
pub const ES_1371_DAC_TEST: c_uint = 1 << 22;
#[inline] pub const fn ES_P2_END_INCO(o: c_uint) -> c_uint { ((o) & 0x07) << 19 }
pub const ES_P2_END_INCM: c_uint = 0x07 << 19;
#[inline] pub const fn ES_P2_END_INCI(i: c_uint) -> c_uint { ((i) >> 16) & 0x07 }
#[inline] pub const fn ES_P2_ST_INCO(o: c_uint) -> c_uint { ((o) & 0x07) << 16 }
pub const ES_P2_ST_INCM: c_uint = 0x07 << 16;
#[inline] pub const fn ES_P2_ST_INCI(i: c_uint) -> c_uint { ((i) << 16) & 0x07 }
pub const ES_R1_LOOP_SEL: c_uint = 1 << 15;
pub const ES_P2_LOOP_SEL: c_uint = 1 << 14;
pub const ES_P1_LOOP_SEL: c_uint = 1 << 13;
pub const ES_P2_PAUSE: c_uint = 1 << 12;
pub const ES_P1_PAUSE: c_uint = 1 << 11;
pub const ES_R1_INT_EN: c_uint = 1 << 10;
pub const ES_P2_INT_EN: c_uint = 1 << 9;
pub const ES_P1_INT_EN: c_uint = 1 << 8;
pub const ES_P1_SCT_RLD: c_uint = 1 << 7;
pub const ES_P2_DAC_SEN: c_uint = 1 << 6;
#[inline] pub const fn ES_R1_MODEO(o: c_uint) -> c_uint { ((o) & 0x03) << 4 }
pub const ES_R1_MODEM: c_uint = 0x03 << 4;
#[inline] pub const fn ES_R1_MODEI(i: c_uint) -> c_uint { ((i) >> 4) & 0x03 }
#[inline] pub const fn ES_P2_MODEO(o: c_uint) -> c_uint { ((o) & 0x03) << 2 }
pub const ES_P2_MODEM: c_uint = 0x03 << 2;
#[inline] pub const fn ES_P2_MODEI(i: c_uint) -> c_uint { ((i) >> 2) & 0x03 }
#[inline] pub const fn ES_P1_MODEO(o: c_uint) -> c_uint { ((o) & 0x03) << 0 }
pub const ES_P1_MODEM: c_uint = 0x03 << 0;
#[inline] pub const fn ES_P1_MODEI(i: c_uint) -> c_uint { ((i) >> 0) & 0x03 }

pub const ES_REG_DAC1_COUNT: c_ulong = 0x24;
pub const ES_REG_DAC2_COUNT: c_ulong = 0x28;
pub const ES_REG_ADC_COUNT: c_ulong = 0x2c;
#[inline] pub const fn ES_REG_CURR_COUNT(i: c_uint) -> c_uint { ((i) >> 16) & 0xffff }
#[inline] pub const fn ES_REG_COUNTO(o: c_uint) -> c_uint { ((o) & 0xffff) << 0 }
pub const ES_REG_COUNTM: c_uint = 0xffff << 0;
#[inline] pub const fn ES_REG_COUNTI(i: c_uint) -> c_uint { ((i) >> 0) & 0xffff }
pub const ES_REG_DAC1_FRAME: c_ulong = 0x30;
pub const ES_REG_DAC1_SIZE: c_ulong = 0x34;
pub const ES_REG_DAC2_FRAME: c_ulong = 0x38;
pub const ES_REG_DAC2_SIZE: c_ulong = 0x3c;
pub const ES_REG_ADC_FRAME: c_ulong = 0x30;
pub const ES_REG_ADC_SIZE: c_ulong = 0x34;
#[inline] pub const fn ES_REG_FCURR_COUNTO(o: c_uint) -> c_uint { ((o) & 0xffff) << 16 }
pub const ES_REG_FCURR_COUNTM: c_uint = 0xffff << 16;
#[inline] pub const fn ES_REG_FCURR_COUNTI(i: c_uint) -> c_uint { ((i) >> 14) & 0x3fffc }
#[inline] pub const fn ES_REG_FSIZEO(o: c_uint) -> c_uint { ((o) & 0xffff) << 0 }
pub const ES_REG_FSIZEM: c_uint = 0xffff << 0;
#[inline] pub const fn ES_REG_FSIZEI(i: c_uint) -> c_uint { ((i) >> 0) & 0xffff }
pub const ES_REG_PHANTOM_FRAME: c_ulong = 0x38;
pub const ES_REG_PHANTOM_COUNT: c_ulong = 0x3c;
pub const ES_REG_UART_FIFO: c_ulong = 0x30;
pub const ES_REG_UF_VALID: c_uint = 1 << 8;
#[inline] pub const fn ES_REG_UF_BYTEO(o: c_uint) -> c_uint { ((o) & 0xff) << 0 }
pub const ES_REG_UF_BYTEM: c_uint = 0xff << 0;
#[inline] pub const fn ES_REG_UF_BYTEI(i: c_uint) -> c_uint { ((i) >> 0) & 0xff }

pub const ES_PAGE_DAC: c_uint = 0x0c;
pub const ES_PAGE_ADC: c_uint = 0x0d;
pub const ES_PAGE_UART: c_uint = 0x0e;
pub const ES_PAGE_UART1: c_uint = 0x0f;

pub const ES_SMPREG_DAC1: c_uint = 0x70;
pub const ES_SMPREG_DAC2: c_uint = 0x74;
pub const ES_SMPREG_ADC: c_uint = 0x78;
pub const ES_SMPREG_VOL_ADC: c_uint = 0x6c;
pub const ES_SMPREG_VOL_DAC1: c_uint = 0x7c;
pub const ES_SMPREG_VOL_DAC2: c_uint = 0x7e;
pub const ES_SMPREG_TRUNC_N: c_uint = 0x00;
pub const ES_SMPREG_INT_REGS: c_uint = 0x01;
pub const ES_SMPREG_ACCUM_FRAC: c_uint = 0x02;
pub const ES_SMPREG_VFREQ_FRAC: c_uint = 0x03;

pub const ES_1370_SRCLOCK: c_uint = 1411200;
#[inline] pub const fn ES_1370_SRTODIV(x: c_uint) -> c_uint { ES_1370_SRCLOCK / x - 2 }

pub const ES_MODE_PLAY1: c_uint = 0x0001;
pub const ES_MODE_PLAY2: c_uint = 0x0002;
pub const ES_MODE_CAPTURE: c_uint = 0x0004;
pub const ES_MODE_OUTPUT: c_uint = 0x0001;
pub const ES_MODE_INPUT: c_uint = 0x0002;
pub const POLL_COUNT: c_uint = 0xa000;

#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub vendor: c_ushort, pub device: c_ushort, pub revision: c_uchar, pub irq: c_int, pub dev: device }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct snd_card { pub dev: *mut device, pub private_data: *mut c_void, pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>, pub sync_irq: c_int, pub driver: [c_char; 16], pub shortname: [c_char; 32], pub longname: [c_char; 80] }
#[repr(C)] pub struct snd_pcm { pub private_data: *mut c_void, pub info_flags: c_uint, pub name: [c_char; 80] }
#[repr(C)] pub struct snd_pcm_runtime { pub hw: snd_pcm_hardware, pub format: c_uint, pub channels: c_uint, pub rate: c_uint, pub dma_addr: c_ulong }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime }
#[repr(C)] pub struct snd_rawmidi { pub private_data: *mut c_void, pub info_flags: c_uint, pub name: [c_char; 80] }
#[repr(C)] pub struct snd_rawmidi_substream { pub rmidi: *mut snd_rawmidi }
#[repr(C)] pub struct snd_ac97 { pub private_data: *mut c_void, pub ext_id: c_uint }
#[repr(C)] pub struct snd_ak4531 { pub private_data: *mut c_void, pub write: Option<unsafe extern "C" fn(*mut snd_ak4531, c_ushort, c_ushort)>, pub private_free: Option<unsafe extern "C" fn(*mut snd_ak4531)> }
#[repr(C)] pub struct snd_dma_buffer { pub addr: c_ulong }
#[repr(C)] pub struct gameport { pub io: c_int }
#[repr(C)] pub struct snd_info_entry { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_info_buffer { _private: [u8; 0] }
#[repr(C)] pub struct snd_ac97_bus { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_long, pub id: snd_ctl_elem_id }
#[repr(C)] pub struct snd_ctl_elem_id { pub index: c_uint }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint }

pub type c_uchar = u8;
pub type snd_pcm_uframes_t = c_ulong;
pub type irqreturn_t = c_uint;

#[repr(C)] pub struct snd_ratnum { pub num: c_uint, pub den_min: c_uint, pub den_max: c_uint, pub den_step: c_uint }
#[repr(C)] pub struct snd_ratden { pub num_min: c_uint, pub num_max: c_uint, pub num_step: c_uint, pub den: c_uint }
#[repr(C)] pub struct snd_pcm_hw_constraint_list { pub count: c_uint, pub list: *const c_uint, pub mask: c_uint }
#[repr(C)] pub struct snd_pcm_hw_constraint_ratnums { pub nrats: c_uint, pub rats: *const snd_ratnum }
#[repr(C)] pub struct snd_pcm_hw_constraint_ratdens { pub nrats: c_uint, pub rats: *const snd_ratden }
#[repr(C)] pub struct snd_pcm_hardware { pub info: c_uint, pub formats: c_uint, pub rates: c_uint, pub rate_min: c_uint, pub rate_max: c_uint, pub channels_min: c_uint, pub channels_max: c_uint, pub buffer_bytes_max: c_uint, pub period_bytes_min: c_uint, pub period_bytes_max: c_uint, pub periods_min: c_uint, pub periods_max: c_uint, pub fifo_size: c_uint }
#[repr(C)] pub struct snd_pcm_ops { pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>, pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t> }
#[repr(C)] pub struct snd_rawmidi_ops { pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>, pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)> }
#[repr(C)] pub struct snd_pcm_chmap_elem { pub channels: c_uint, pub map: [c_uint; 4] }
#[repr(C)] pub struct snd_kcontrol_new { pub iface: c_uint, pub name: *const c_char, pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub access: c_uint, pub private_value: c_long }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_union }
#[repr(C)] pub union snd_ctl_elem_value_union { pub integer: snd_ctl_elem_value_integer, pub iec958: snd_ctl_elem_value_iec958 }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_iec958 { pub status: [c_uchar; 24] }
#[repr(C)] pub struct snd_ac97_bus_ops { pub write: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort, c_ushort)>, pub read: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort) -> c_ushort>, pub wait: Option<unsafe extern "C" fn(*mut snd_ac97)> }
#[repr(C)] pub struct snd_ac97_template { pub private_data: *mut c_void, pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97)>, pub pci: *mut pci_dev, pub scaps: c_uint }
#[repr(C)] pub struct pci_device_id { pub vendor: c_uint, pub device: c_uint, pub subvendor: c_uint, pub subdevice: c_uint, pub class: c_uint, pub class_mask: c_uint, pub driver_data: c_ulong }
#[repr(C)] pub struct snd_pci_quirk { pub subvendor: c_ushort, pub subdevice: c_ushort }
#[repr(C)] pub struct pci_driver { pub name: *const c_char, pub id_table: *const pci_device_id, pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>, pub driver: driver_inner }
#[repr(C)] pub struct driver_inner { pub pm: *const c_void }

#[repr(C)]
pub union ensoniq_u {
    pub es1371: ensoniq_es1371,
    pub es1370: ensoniq_es1370,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct ensoniq_es1371 { pub ac97: *mut snd_ac97 }
#[repr(C)] #[derive(Copy, Clone)] pub struct ensoniq_es1370 { pub pclkdiv_lock: c_int, pub ak4531: *mut snd_ak4531 }

#[repr(C)]
pub struct ensoniq {
    pub reg_lock: spinlock_t,
    pub src_mutex: mutex,
    pub irq: c_int,
    pub playback1size: c_ulong,
    pub playback2size: c_ulong,
    pub capture3size: c_ulong,
    pub port: c_ulong,
    pub mode: c_uint,
    pub uartm: c_uint,
    pub ctrl: c_uint,
    pub sctrl: c_uint,
    pub cssr: c_uint,
    pub uartc: c_uint,
    pub rev: c_uint,
    pub u: ensoniq_u,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm1: *mut snd_pcm,
    pub pcm2: *mut snd_pcm,
    pub playback1_substream: *mut snd_pcm_substream,
    pub playback2_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub p1_dma_size: c_uint,
    pub p2_dma_size: c_uint,
    pub c_dma_size: c_uint,
    pub p1_period_size: c_uint,
    pub p2_period_size: c_uint,
    pub c_period_size: c_uint,
    pub rmidi: *mut snd_rawmidi,
    pub midi_input: *mut snd_rawmidi_substream,
    pub midi_output: *mut snd_rawmidi_substream,
    pub spdif: c_uint,
    pub spdif_default: c_uint,
    pub spdif_stream: c_uint,
    pub dma_bug: *mut snd_dma_buffer,
    pub gameport: *mut gameport,
}

unsafe extern "C" {
    static mut index: [c_int; SNDRV_CARDS];
    static mut id: [*mut c_char; SNDRV_CARDS];
    static mut enable: [bool; SNDRV_CARDS];
    static mut joystick_port: [c_int; SNDRV_CARDS];
    static mut joystick: [bool; SNDRV_CARDS];
    static mut spdif: [c_int; SNDRV_CARDS];
    static mut lineio: [c_int; SNDRV_CARDS];
    static mut jiffies: c_ulong;
    static snd_pcm_std_chmaps: *const snd_pcm_chmap_elem;

    fn inl(port: c_ulong) -> c_uint;
    fn outl(value: c_uint, port: c_ulong);
    fn inw(port: c_ulong) -> c_ushort;
    fn outw(value: c_uint, port: c_ulong);
    fn inb(port: c_ulong) -> c_uchar;
    fn outb(value: c_uint, port: c_ulong);
    fn cond_resched();
    fn schedule_timeout_uninterruptible(timeout: c_long);
    fn time_after(a: c_ulong, b: c_ulong) -> bool;
    fn msleep(ms: c_uint);
    fn udelay(us: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut ensoniq;
    fn snd_pcm_group_next(s: *mut snd_pcm_substream, substream: *mut snd_pcm_substream) -> *mut snd_pcm_substream;
    fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, substream: *mut snd_pcm_substream);
    fn snd_pcm_format_width(format: c_uint) -> c_int;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: usize) -> snd_pcm_uframes_t;
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, list: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_hw_constraint_ratnums(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, rats: *const snd_pcm_hw_constraint_ratnums) -> c_int;
    fn snd_pcm_hw_constraint_ratdens(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, rats: *const snd_pcm_hw_constraint_ratdens) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: usize, max: usize);
    fn snd_pcm_add_chmap_ctls(pcm: *mut snd_pcm, stream: c_int, map: *const snd_pcm_chmap_elem, max_channels: c_int, device: c_int, private_value: *mut c_void) -> c_int;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut ensoniq;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn snd_pci_quirk_lookup(pci: *mut pci_dev, list: *const snd_pci_quirk) -> *const snd_pci_quirk;
    fn snd_ak4531_mixer(card: *mut snd_card, ak4531: *mut snd_ak4531, rak4531: *mut *mut snd_ak4531) -> c_int;
    fn gameport_allocate_port() -> *mut gameport;
    fn gameport_set_name(gp: *mut gameport, name: *const c_char);
    fn gameport_set_phys(gp: *mut gameport, fmt: *const c_char, ...);
    fn gameport_set_dev_parent(gp: *mut gameport, dev: *mut device);
    fn gameport_register_port(gp: *mut gameport);
    fn gameport_unregister_port(gp: *mut gameport);
    fn request_region(start: c_int, n: c_int, name: *const c_char) -> *mut c_void;
    fn release_region(start: c_int, n: c_int);
    fn pci_name(pci: *mut pci_dev) -> *const c_char;
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, private_data: *mut c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn str_on_off(v: c_uint) -> *const c_char;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn snd_devm_alloc_pages(dev: *mut device, ty: c_int, size: usize) -> *mut snd_dma_buffer;
    fn pci_set_master(pci: *mut pci_dev);
    fn snd_power_change_state(card: *mut snd_card, state: c_uint);
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn snd_ak4531_suspend(ak4531: *mut snd_ak4531);
    fn snd_ak4531_resume(ak4531: *mut snd_ak4531);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_rawmidi_new(card: *mut snd_card, id: *const c_char, device: c_int, output_count: c_int, input_count: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buffer: *mut c_uchar, count: usize);
    fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buffer: *mut c_uchar, count: usize) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, probe: c_int) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char);
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
}

pub const SNDRV_CARDS: usize = 8;
pub const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
pub const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
pub const SNDRV_PCM_TRIGGER_START: c_int = 0;
pub const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const ENOSYS: c_int = 38;
pub const EBUSY: c_int = 16;
pub const ENODEV: c_int = 19;
pub const ENOENT: c_int = 2;
pub const IRQ_NONE: irqreturn_t = 0;
pub const IRQ_HANDLED: irqreturn_t = 1;
pub const HZ: c_ulong = 100;
pub const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
pub const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
pub const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
pub const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
pub const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 4;
pub const SNDRV_PCM_INFO_SYNC_START: c_uint = 1 << 5;
pub const SNDRV_PCM_FMTBIT_U8: c_uint = 1 << 0;
pub const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 1;
pub const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 0;
pub const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 1;
pub const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 2;
pub const SNDRV_PCM_RATE_11025: c_uint = 1 << 3;
pub const SNDRV_PCM_RATE_22050: c_uint = 1 << 4;
pub const SNDRV_PCM_RATE_44100: c_uint = 1 << 5;
pub const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
pub const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
pub const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
pub const SNDRV_DMA_TYPE_DEV: c_int = 0;
pub const SNDRV_CTL_ELEM_TYPE_IEC958: c_uint = 4;
pub const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
pub const SNDRV_CTL_ELEM_IFACE_PCM: c_uint = 3;
pub const SNDRV_CTL_ELEM_IFACE_CARD: c_uint = 1;
pub const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
pub const SNDRV_CHMAP_MONO: c_uint = 1;
pub const SNDRV_CHMAP_RL: c_uint = 5;
pub const SNDRV_CHMAP_RR: c_uint = 6;
pub const AC97_RESET: c_ushort = 0;
pub const AC97_VENDOR_ID1: c_ushort = 0x7c;
pub const AC97_VENDOR_ID2: c_ushort = 0x7e;
pub const AC97_SCAP_AUDIO: c_uint = 1;
pub const AC97_EI_SPDIF: c_uint = 1 << 2;
pub const AC97_EI_SDAC: c_uint = 1 << 6;
pub const AK4531_RESET: c_uint = 0;
pub const PCI_ANY_ID: c_uint = 0xffff;
pub const PCI_VENDOR_ID_ENSONIQ: c_ushort = 0x1274;
pub const PCI_DEVICE_ID_ENSONIQ_CT5880: c_ushort = 0x5880;
pub const PCI_DEVICE_ID_ENSONIQ_ES1371: c_ushort = 0x1371;
pub const SNDRV_PCM_DEFAULT_CON_SPDIF: c_uint = 0x0200;
pub const IRQF_SHARED: c_ulong = 0x80;
pub const SNDRV_CTL_POWER_D3hot: c_uint = 3;
pub const SNDRV_CTL_POWER_D0: c_uint = 0;
pub const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 0;
pub const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 1;
pub const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 1;
pub const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 2;
pub const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 4;

static snd_es1370_fixed_rates: [c_uint; 4] = [5512, 11025, 22050, 44100];
static snd_es1370_hw_constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { count: 4, list: snd_es1370_fixed_rates.as_ptr(), mask: 0 };
static es1370_clock: snd_ratnum = snd_ratnum { num: ES_1370_SRCLOCK, den_min: 29, den_max: 353, den_step: 1 };
static snd_es1370_hw_constraints_clock: snd_pcm_hw_constraint_ratnums = snd_pcm_hw_constraint_ratnums { nrats: 1, rats: &es1370_clock };
static es1371_dac_clock: snd_ratden = snd_ratden { num_min: 3000 * (1 << 15), num_max: 48000 * (1 << 15), num_step: 3000, den: 1 << 15 };
static snd_es1371_hw_constraints_dac_clock: snd_pcm_hw_constraint_ratdens = snd_pcm_hw_constraint_ratdens { nrats: 1, rats: &es1371_dac_clock };
static es1371_adc_clock: snd_ratnum = snd_ratnum { num: 48000 << 15, den_min: 32768, den_max: 393216, den_step: 1 };
static snd_es1371_hw_constraints_adc_clock: snd_pcm_hw_constraint_ratnums = snd_pcm_hw_constraint_ratnums { nrats: 1, rats: &es1371_adc_clock };
static snd_ensoniq_sample_shift: [c_uint; 4] = [0, 1, 1, 2];

#[inline] const fn DIV_ROUND_CLOSEST(x: c_uint, divisor: c_uint) -> c_uint { (x + divisor / 2) / divisor }

unsafe extern "C" fn snd_es1371_wait_src_ready(ensoniq: *mut ensoniq) -> c_uint {
    let mut r: c_uint = 0;
    let mut t = 0;
    while t < POLL_COUNT {
        r = unsafe { inl(ES_REG(ensoniq, ES_REG_1371_SMPRATE)) };
        if (r & ES_1371_SRC_RAM_BUSY) == 0 { return r; }
        unsafe { cond_resched(); }
        t += 1;
    }
    unsafe { dev_err((*(*ensoniq).card).dev, c"wait src ready timeout 0x%lx [0x%x]\n".as_ptr(), ES_REG(ensoniq, ES_REG_1371_SMPRATE), r); }
    0
}

unsafe extern "C" fn snd_es1371_src_read(ensoniq: *mut ensoniq, reg: c_ushort) -> c_uint {
    let mut temp: c_uint;
    let orig: c_uint;
    let mut r: c_uint;
    orig = unsafe { snd_es1371_wait_src_ready(ensoniq) };
    temp = orig;
    r = temp & (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 | ES_1371_DIS_P2 | ES_1371_DIS_R1);
    r |= ES_1371_SRC_RAM_ADDRO(reg as c_uint) | 0x10000;
    unsafe { outl(r, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); }
    temp = unsafe { snd_es1371_wait_src_ready(ensoniq) };
    if (temp & 0x00870000) != 0x00010000 {
        let mut i = 0;
        while i < POLL_COUNT {
            temp = unsafe { inl(ES_REG(ensoniq, ES_REG_1371_SMPRATE)) };
            if (temp & 0x00870000) == 0x00010000 { break; }
            i += 1;
        }
    }
    r = orig & (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 | ES_1371_DIS_P2 | ES_1371_DIS_R1);
    r |= ES_1371_SRC_RAM_ADDRO(reg as c_uint);
    unsafe { outl(r, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); }
    temp
}

unsafe extern "C" fn snd_es1371_src_write(ensoniq: *mut ensoniq, reg: c_ushort, data: c_ushort) {
    let mut r = unsafe { snd_es1371_wait_src_ready(ensoniq) } & (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 | ES_1371_DIS_P2 | ES_1371_DIS_R1);
    r |= ES_1371_SRC_RAM_ADDRO(reg as c_uint) | ES_1371_SRC_RAM_DATAO(data as c_uint);
    unsafe { outl(r | ES_1371_SRC_RAM_WE, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); }
}

unsafe extern "C" fn snd_es1370_codec_write(ak4531: *mut snd_ak4531, reg: c_ushort, val: c_ushort) {
    let ensoniq = unsafe { (*ak4531).private_data as *mut ensoniq };
    let end_time = unsafe { jiffies }.wrapping_add(HZ / 10);
    loop {
        if unsafe { inl(ES_REG(ensoniq, ES_REG_STATUS)) & ES_1370_CSTAT } == 0 {
            unsafe { outw(ES_1370_CODEC_WRITE(reg as c_uint, val as c_uint), ES_REG(ensoniq, ES_REG_1370_CODEC)); }
            return;
        }
        unsafe { schedule_timeout_uninterruptible(1); }
        if !unsafe { time_after(end_time, jiffies) } { break; }
    }
    unsafe { dev_err((*(*ensoniq).card).dev, c"codec write timeout, status = 0x%x\n".as_ptr(), inl(ES_REG(ensoniq, ES_REG_STATUS))); }
}

#[inline]
unsafe fn is_ev1938(ensoniq: *mut ensoniq) -> bool { unsafe { (*(*ensoniq).pci).device == 0x8938 } }

unsafe extern "C" fn snd_es1371_codec_write(ac97: *mut snd_ac97, reg: c_ushort, val: c_ushort) {
    let ensoniq = unsafe { (*ac97).private_data as *mut ensoniq };
    let flag = if unsafe { is_ev1938(ensoniq) } { EV_1938_CODEC_MAGIC } else { 0 };
    unsafe { mutex_lock(&mut (*ensoniq).src_mutex); }
    let mut t = 0;
    while t < POLL_COUNT {
        if unsafe { inl(ES_REG(ensoniq, ES_REG_1371_CODEC)) & ES_1371_CODEC_WIP } == 0 {
            let x = unsafe { snd_es1371_wait_src_ready(ensoniq) };
            unsafe { outl((x & (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 | ES_1371_DIS_P2 | ES_1371_DIS_R1)) | 0x00010000, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); }
            let mut u = 0; while u < POLL_COUNT { if unsafe { inl(ES_REG(ensoniq, ES_REG_1371_SMPRATE)) & 0x00870000 } == 0 { break; } u += 1; }
            u = 0; while u < POLL_COUNT { if unsafe { inl(ES_REG(ensoniq, ES_REG_1371_SMPRATE)) & 0x00870000 } == 0x00010000 { break; } u += 1; }
            unsafe { outl(ES_1371_CODEC_WRITE(reg as c_uint, val as c_uint) | flag, ES_REG(ensoniq, ES_REG_1371_CODEC)); }
            unsafe { snd_es1371_wait_src_ready(ensoniq); outl(x, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); mutex_unlock(&mut (*ensoniq).src_mutex); }
            return;
        }
        t += 1;
    }
    unsafe { mutex_unlock(&mut (*ensoniq).src_mutex); dev_err((*(*ensoniq).card).dev, c"codec write timeout at 0x%lx [0x%x]\n".as_ptr(), ES_REG(ensoniq, ES_REG_1371_CODEC), inl(ES_REG(ensoniq, ES_REG_1371_CODEC))); }
}

unsafe extern "C" fn snd_es1371_codec_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort {
    let ensoniq = unsafe { (*ac97).private_data as *mut ensoniq };
    let flag = if unsafe { is_ev1938(ensoniq) } { EV_1938_CODEC_MAGIC } else { 0 };
    let mut fail: c_uint = 0;
    loop {
        unsafe { mutex_lock(&mut (*ensoniq).src_mutex); }
        let mut t = 0;
        while t < POLL_COUNT {
            if unsafe { inl(ES_REG(ensoniq, ES_REG_1371_CODEC)) & ES_1371_CODEC_WIP } == 0 {
                let mut x = unsafe { snd_es1371_wait_src_ready(ensoniq) };
                unsafe { outl((x & (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 | ES_1371_DIS_P2 | ES_1371_DIS_R1)) | 0x00010000, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); }
                let mut u = 0; while u < POLL_COUNT { if unsafe { inl(ES_REG(ensoniq, ES_REG_1371_SMPRATE)) & 0x00870000 } == 0 { break; } u += 1; }
                u = 0; while u < POLL_COUNT { if unsafe { inl(ES_REG(ensoniq, ES_REG_1371_SMPRATE)) & 0x00870000 } == 0x00010000 { break; } u += 1; }
                unsafe { outl(ES_1371_CODEC_READS(reg as c_uint) | flag, ES_REG(ensoniq, ES_REG_1371_CODEC)); snd_es1371_wait_src_ready(ensoniq); outl(x, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); }
                u = 0; while u < POLL_COUNT { if unsafe { inl(ES_REG(ensoniq, ES_REG_1371_CODEC)) & ES_1371_CODEC_WIP } == 0 { break; } u += 1; }
                u = 0; while u < POLL_COUNT {
                    x = unsafe { inl(ES_REG(ensoniq, ES_REG_1371_CODEC)) };
                    if (x & ES_1371_CODEC_RDY) != 0 {
                        if unsafe { is_ev1938(ensoniq) } {
                            let mut v = 0; while v < 100 { unsafe { inl(ES_REG(ensoniq, ES_REG_CONTROL)); } v += 1; }
                            x = unsafe { inl(ES_REG(ensoniq, ES_REG_1371_CODEC)) };
                        }
                        unsafe { mutex_unlock(&mut (*ensoniq).src_mutex); }
                        return ES_1371_CODEC_READ(x);
                    }
                    u += 1;
                }
                unsafe { mutex_unlock(&mut (*ensoniq).src_mutex); }
                fail += 1;
                if fail > 10 {
                    unsafe { dev_err((*(*ensoniq).card).dev, c"codec read timeout (final) at 0x%lx, reg = 0x%x [0x%x]\n".as_ptr(), ES_REG(ensoniq, ES_REG_1371_CODEC), reg as c_uint, inl(ES_REG(ensoniq, ES_REG_1371_CODEC))); }
                    return 0;
                }
                continue;
            }
            t += 1;
        }
        unsafe { mutex_unlock(&mut (*ensoniq).src_mutex); dev_err((*(*ensoniq).card).dev, c"codec read timeout at 0x%lx [0x%x]\n".as_ptr(), ES_REG(ensoniq, ES_REG_1371_CODEC), inl(ES_REG(ensoniq, ES_REG_1371_CODEC))); }
        return 0;
    }
}

unsafe extern "C" fn snd_es1371_codec_wait(ac97: *mut snd_ac97) {
    unsafe { msleep(750); snd_es1371_codec_read(ac97, AC97_RESET); snd_es1371_codec_read(ac97, AC97_VENDOR_ID1); snd_es1371_codec_read(ac97, AC97_VENDOR_ID2); msleep(50); }
}

unsafe extern "C" fn snd_es1371_adc_rate(ensoniq: *mut ensoniq, rate: c_uint) {
    unsafe { mutex_lock(&mut (*ensoniq).src_mutex); }
    let mut n = rate / 3000;
    if ((1 << n) & ((1 << 15) | (1 << 13) | (1 << 11) | (1 << 9))) != 0 { n -= 1; }
    let mut truncm = (21 * n - 1) | 1;
    let freq = ((48000u64 << 15) / rate as u64) as c_uint * n;
    if rate >= 24000 {
        if truncm > 239 { truncm = 239; }
        unsafe { snd_es1371_src_write(ensoniq, (ES_SMPREG_ADC + ES_SMPREG_TRUNC_N) as c_ushort, ((((239 - truncm) >> 1) << 9) | (n << 4)) as c_ushort); }
    } else {
        if truncm > 119 { truncm = 119; }
        unsafe { snd_es1371_src_write(ensoniq, (ES_SMPREG_ADC + ES_SMPREG_TRUNC_N) as c_ushort, (0x8000 | (((119 - truncm) >> 1) << 9) | (n << 4)) as c_ushort); }
    }
    unsafe {
        snd_es1371_src_write(ensoniq, (ES_SMPREG_ADC + ES_SMPREG_INT_REGS) as c_ushort, ((snd_es1371_src_read(ensoniq, (ES_SMPREG_ADC + ES_SMPREG_INT_REGS) as c_ushort) & 0x00ff) | ((freq >> 5) & 0xfc00)) as c_ushort);
        snd_es1371_src_write(ensoniq, (ES_SMPREG_ADC + ES_SMPREG_VFREQ_FRAC) as c_ushort, (freq & 0x7fff) as c_ushort);
        snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_ADC as c_ushort, (n << 8) as c_ushort);
        snd_es1371_src_write(ensoniq, (ES_SMPREG_VOL_ADC + 1) as c_ushort, (n << 8) as c_ushort);
        mutex_unlock(&mut (*ensoniq).src_mutex);
    }
}

unsafe extern "C" fn snd_es1371_dac1_rate(ensoniq: *mut ensoniq, rate: c_uint) {
    unsafe { mutex_lock(&mut (*ensoniq).src_mutex); }
    let freq = DIV_ROUND_CLOSEST(rate << 15, 3000);
    let mut r = (unsafe { snd_es1371_wait_src_ready(ensoniq) } & (ES_1371_SRC_DISABLE | ES_1371_DIS_P2 | ES_1371_DIS_R1)) | ES_1371_DIS_P1;
    unsafe { outl(r, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); snd_es1371_src_write(ensoniq, (ES_SMPREG_DAC1 + ES_SMPREG_INT_REGS) as c_ushort, ((snd_es1371_src_read(ensoniq, (ES_SMPREG_DAC1 + ES_SMPREG_INT_REGS) as c_ushort) & 0x00ff) | ((freq >> 5) & 0xfc00)) as c_ushort); snd_es1371_src_write(ensoniq, (ES_SMPREG_DAC1 + ES_SMPREG_VFREQ_FRAC) as c_ushort, (freq & 0x7fff) as c_ushort); }
    r = unsafe { snd_es1371_wait_src_ready(ensoniq) } & (ES_1371_SRC_DISABLE | ES_1371_DIS_P2 | ES_1371_DIS_R1);
    unsafe { outl(r, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); mutex_unlock(&mut (*ensoniq).src_mutex); }
}

unsafe extern "C" fn snd_es1371_dac2_rate(ensoniq: *mut ensoniq, rate: c_uint) {
    unsafe { mutex_lock(&mut (*ensoniq).src_mutex); }
    let freq = DIV_ROUND_CLOSEST(rate << 15, 3000);
    let mut r = (unsafe { snd_es1371_wait_src_ready(ensoniq) } & (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 | ES_1371_DIS_R1)) | ES_1371_DIS_P2;
    unsafe { outl(r, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); snd_es1371_src_write(ensoniq, (ES_SMPREG_DAC2 + ES_SMPREG_INT_REGS) as c_ushort, ((snd_es1371_src_read(ensoniq, (ES_SMPREG_DAC2 + ES_SMPREG_INT_REGS) as c_ushort) & 0x00ff) | ((freq >> 5) & 0xfc00)) as c_ushort); snd_es1371_src_write(ensoniq, (ES_SMPREG_DAC2 + ES_SMPREG_VFREQ_FRAC) as c_ushort, (freq & 0x7fff) as c_ushort); }
    r = unsafe { snd_es1371_wait_src_ready(ensoniq) } & (ES_1371_SRC_DISABLE | ES_1371_DIS_P1 | ES_1371_DIS_R1);
    unsafe { outl(r, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); mutex_unlock(&mut (*ensoniq).src_mutex); }
}

unsafe extern "C" fn snd_ensoniq_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let ensoniq = unsafe { snd_pcm_substream_chip(substream) };
    match cmd {
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            let mut what = 0;
            let mut s = substream;
            loop {
                if s == unsafe { (*ensoniq).playback1_substream } { what |= ES_P1_PAUSE; unsafe { snd_pcm_trigger_done(s, substream); } }
                else if s == unsafe { (*ensoniq).playback2_substream } { what |= ES_P2_PAUSE; unsafe { snd_pcm_trigger_done(s, substream); } }
                else if s == unsafe { (*ensoniq).capture_substream } { return -EINVAL; }
                s = unsafe { snd_pcm_group_next(s, substream) };
                if s.is_null() || s == substream { break; }
            }
            unsafe {
                if cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH { (*ensoniq).sctrl |= what; } else { (*ensoniq).sctrl &= !what; }
                outl((*ensoniq).sctrl, ES_REG(ensoniq, ES_REG_SERIAL));
            }
        }
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_STOP => {
            let mut what = 0;
            let mut s = substream;
            loop {
                if s == unsafe { (*ensoniq).playback1_substream } { what |= ES_DAC1_EN; unsafe { snd_pcm_trigger_done(s, substream); } }
                else if s == unsafe { (*ensoniq).playback2_substream } { what |= ES_DAC2_EN; unsafe { snd_pcm_trigger_done(s, substream); } }
                else if s == unsafe { (*ensoniq).capture_substream } { what |= ES_ADC_EN; unsafe { snd_pcm_trigger_done(s, substream); } }
                s = unsafe { snd_pcm_group_next(s, substream) };
                if s.is_null() || s == substream { break; }
            }
            unsafe {
                if cmd == SNDRV_PCM_TRIGGER_START { (*ensoniq).ctrl |= what; } else { (*ensoniq).ctrl &= !what; }
                outl((*ensoniq).ctrl, ES_REG(ensoniq, ES_REG_CONTROL));
            }
        }
        _ => return -EINVAL,
    }
    0
}

unsafe fn prepare_common(substream: *mut snd_pcm_substream, which: c_int) -> c_int {
    let ensoniq = unsafe { snd_pcm_substream_chip(substream) };
    let runtime = unsafe { (*substream).runtime };
    let mut mode = 0;
    if which == 1 { unsafe { (*ensoniq).p1_dma_size = snd_pcm_lib_buffer_bytes(substream); (*ensoniq).p1_period_size = snd_pcm_lib_period_bytes(substream); } }
    if which == 2 { unsafe { (*ensoniq).p2_dma_size = snd_pcm_lib_buffer_bytes(substream); (*ensoniq).p2_period_size = snd_pcm_lib_period_bytes(substream); } }
    if which == 3 { unsafe { (*ensoniq).c_dma_size = snd_pcm_lib_buffer_bytes(substream); (*ensoniq).c_period_size = snd_pcm_lib_period_bytes(substream); } }
    if unsafe { snd_pcm_format_width((*runtime).format) } == 16 { mode |= 0x02; }
    if unsafe { (*runtime).channels } > 1 { mode |= 0x01; }
    unsafe {
        match which {
            1 => {
                (*ensoniq).ctrl &= !ES_DAC1_EN;
                if (*runtime).rate == 48000 { (*ensoniq).ctrl |= ES_1373_BYPASS_P1; } else { (*ensoniq).ctrl &= !ES_1373_BYPASS_P1; }
                outl((*ensoniq).ctrl, ES_REG(ensoniq, ES_REG_CONTROL));
                outl(ES_MEM_PAGEO(ES_PAGE_DAC), ES_REG(ensoniq, ES_REG_MEM_PAGE));
                outl((*runtime).dma_addr as c_uint, ES_REG(ensoniq, ES_REG_DAC1_FRAME));
                outl(((*ensoniq).p1_dma_size >> 2) - 1, ES_REG(ensoniq, ES_REG_DAC1_SIZE));
                (*ensoniq).sctrl &= !(ES_P1_LOOP_SEL | ES_P1_PAUSE | ES_P1_SCT_RLD | ES_P1_MODEM);
                (*ensoniq).sctrl |= ES_P1_INT_EN | ES_P1_MODEO(mode);
                outl((*ensoniq).sctrl, ES_REG(ensoniq, ES_REG_SERIAL));
                outl(((*ensoniq).p1_period_size >> snd_ensoniq_sample_shift[mode as usize]) - 1, ES_REG(ensoniq, ES_REG_DAC1_COUNT));
                (*ensoniq).ctrl &= !ES_1370_WTSRSELM;
                (*ensoniq).ctrl |= match (*runtime).rate { 5512 => ES_1370_WTSRSEL(0), 11025 => ES_1370_WTSRSEL(1), 22050 => ES_1370_WTSRSEL(2), 44100 => ES_1370_WTSRSEL(3), _ => 0 };
                outl((*ensoniq).ctrl, ES_REG(ensoniq, ES_REG_CONTROL));
                snd_es1371_dac1_rate(ensoniq, (*runtime).rate);
            }
            2 => {
                (*ensoniq).ctrl &= !ES_DAC2_EN;
                outl((*ensoniq).ctrl, ES_REG(ensoniq, ES_REG_CONTROL));
                outl(ES_MEM_PAGEO(ES_PAGE_DAC), ES_REG(ensoniq, ES_REG_MEM_PAGE));
                outl((*runtime).dma_addr as c_uint, ES_REG(ensoniq, ES_REG_DAC2_FRAME));
                outl(((*ensoniq).p2_dma_size >> 2) - 1, ES_REG(ensoniq, ES_REG_DAC2_SIZE));
                (*ensoniq).sctrl &= !(ES_P2_LOOP_SEL | ES_P2_PAUSE | ES_P2_DAC_SEN | ES_P2_END_INCM | ES_P2_ST_INCM | ES_P2_MODEM);
                (*ensoniq).sctrl |= ES_P2_INT_EN | ES_P2_MODEO(mode) | ES_P2_END_INCO(if (mode & 2) != 0 { 2 } else { 1 }) | ES_P2_ST_INCO(0);
                outl((*ensoniq).sctrl, ES_REG(ensoniq, ES_REG_SERIAL));
                outl(((*ensoniq).p2_period_size >> snd_ensoniq_sample_shift[mode as usize]) - 1, ES_REG(ensoniq, ES_REG_DAC2_COUNT));
                if ((*ensoniq).u.es1370.pclkdiv_lock & ES_MODE_CAPTURE as c_int) == 0 {
                    (*ensoniq).ctrl &= !ES_1370_PCLKDIVM;
                    (*ensoniq).ctrl |= ES_1370_PCLKDIVO(ES_1370_SRTODIV((*runtime).rate));
                    (*ensoniq).u.es1370.pclkdiv_lock |= ES_MODE_PLAY2 as c_int;
                }
                outl((*ensoniq).ctrl, ES_REG(ensoniq, ES_REG_CONTROL));
                snd_es1371_dac2_rate(ensoniq, (*runtime).rate);
            }
            _ => {
                (*ensoniq).ctrl &= !ES_ADC_EN;
                outl((*ensoniq).ctrl, ES_REG(ensoniq, ES_REG_CONTROL));
                outl(ES_MEM_PAGEO(ES_PAGE_ADC), ES_REG(ensoniq, ES_REG_MEM_PAGE));
                outl((*runtime).dma_addr as c_uint, ES_REG(ensoniq, ES_REG_ADC_FRAME));
                outl(((*ensoniq).c_dma_size >> 2) - 1, ES_REG(ensoniq, ES_REG_ADC_SIZE));
                (*ensoniq).sctrl &= !(ES_R1_LOOP_SEL | ES_R1_MODEM);
                (*ensoniq).sctrl |= ES_R1_INT_EN | ES_R1_MODEO(mode);
                outl((*ensoniq).sctrl, ES_REG(ensoniq, ES_REG_SERIAL));
                outl(((*ensoniq).c_period_size >> snd_ensoniq_sample_shift[mode as usize]) - 1, ES_REG(ensoniq, ES_REG_ADC_COUNT));
                if ((*ensoniq).u.es1370.pclkdiv_lock & ES_MODE_PLAY2 as c_int) == 0 {
                    (*ensoniq).ctrl &= !ES_1370_PCLKDIVM;
                    (*ensoniq).ctrl |= ES_1370_PCLKDIVO(ES_1370_SRTODIV((*runtime).rate));
                    (*ensoniq).u.es1370.pclkdiv_lock |= ES_MODE_CAPTURE as c_int;
                }
                outl((*ensoniq).ctrl, ES_REG(ensoniq, ES_REG_CONTROL));
                snd_es1371_adc_rate(ensoniq, (*runtime).rate);
            }
        }
    }
    0
}
unsafe extern "C" fn snd_ensoniq_playback1_prepare(s: *mut snd_pcm_substream) -> c_int { unsafe { prepare_common(s, 1) } }
unsafe extern "C" fn snd_ensoniq_playback2_prepare(s: *mut snd_pcm_substream) -> c_int { unsafe { prepare_common(s, 2) } }
unsafe extern "C" fn snd_ensoniq_capture_prepare(s: *mut snd_pcm_substream) -> c_int { unsafe { prepare_common(s, 3) } }

unsafe fn pointer_common(substream: *mut snd_pcm_substream, enable: c_uint, page: c_uint, size_reg: c_ulong) -> snd_pcm_uframes_t {
    let ensoniq = unsafe { snd_pcm_substream_chip(substream) };
    if unsafe { inl(ES_REG(ensoniq, ES_REG_CONTROL)) & enable } != 0 {
        unsafe { outl(ES_MEM_PAGEO(page), ES_REG(ensoniq, ES_REG_MEM_PAGE)); }
        let ptr = unsafe { ES_REG_FCURR_COUNTI(inl(ES_REG(ensoniq, size_reg))) } as usize;
        unsafe { bytes_to_frames((*substream).runtime, ptr) }
    } else { 0 }
}
unsafe extern "C" fn snd_ensoniq_playback1_pointer(s: *mut snd_pcm_substream) -> snd_pcm_uframes_t { unsafe { pointer_common(s, ES_DAC1_EN, ES_PAGE_DAC, ES_REG_DAC1_SIZE) } }
unsafe extern "C" fn snd_ensoniq_playback2_pointer(s: *mut snd_pcm_substream) -> snd_pcm_uframes_t { unsafe { pointer_common(s, ES_DAC2_EN, ES_PAGE_DAC, ES_REG_DAC2_SIZE) } }
unsafe extern "C" fn snd_ensoniq_capture_pointer(s: *mut snd_pcm_substream) -> snd_pcm_uframes_t { unsafe { pointer_common(s, ES_ADC_EN, ES_PAGE_ADC, ES_REG_ADC_SIZE) } }

static snd_ensoniq_playback1: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_SYNC_START, formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE, rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_KNOT | SNDRV_PCM_RATE_11025 | SNDRV_PCM_RATE_22050 | SNDRV_PCM_RATE_44100, rate_min: 4000, rate_max: 48000, channels_min: 1, channels_max: 2, buffer_bytes_max: 128 * 1024, period_bytes_min: 64, period_bytes_max: 128 * 1024, periods_min: 1, periods_max: 1024, fifo_size: 0 };
static snd_ensoniq_playback2: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_SYNC_START, formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE, rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000, rate_min: 4000, rate_max: 48000, channels_min: 1, channels_max: 2, buffer_bytes_max: 128 * 1024, period_bytes_min: 64, period_bytes_max: 128 * 1024, periods_min: 1, periods_max: 1024, fifo_size: 0 };
static snd_ensoniq_capture: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_SYNC_START, formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE, rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000, rate_min: 4000, rate_max: 48000, channels_min: 1, channels_max: 2, buffer_bytes_max: 128 * 1024, period_bytes_min: 64, period_bytes_max: 128 * 1024, periods_min: 1, periods_max: 1024, fifo_size: 0 };

unsafe extern "C" fn snd_ensoniq_playback1_open(substream: *mut snd_pcm_substream) -> c_int {
    let ensoniq = unsafe { snd_pcm_substream_chip(substream) }; let runtime = unsafe { (*substream).runtime };
    unsafe { (*ensoniq).mode |= ES_MODE_PLAY1; (*ensoniq).playback1_substream = substream; (*runtime).hw = snd_ensoniq_playback1; snd_pcm_set_sync(substream); if (*ensoniq).spdif != 0 && (*ensoniq).playback2_substream.is_null() { (*ensoniq).spdif_stream = (*ensoniq).spdif_default; } snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &snd_es1370_hw_constraints_rates); snd_pcm_hw_constraint_ratdens(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &snd_es1371_hw_constraints_dac_clock); }
    0
}
unsafe extern "C" fn snd_ensoniq_playback2_open(substream: *mut snd_pcm_substream) -> c_int {
    let ensoniq = unsafe { snd_pcm_substream_chip(substream) }; let runtime = unsafe { (*substream).runtime };
    unsafe { (*ensoniq).mode |= ES_MODE_PLAY2; (*ensoniq).playback2_substream = substream; (*runtime).hw = snd_ensoniq_playback2; snd_pcm_set_sync(substream); if (*ensoniq).spdif != 0 && (*ensoniq).playback1_substream.is_null() { (*ensoniq).spdif_stream = (*ensoniq).spdif_default; } snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &snd_es1370_hw_constraints_clock); snd_pcm_hw_constraint_ratdens(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &snd_es1371_hw_constraints_dac_clock); }
    0
}
unsafe extern "C" fn snd_ensoniq_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let ensoniq = unsafe { snd_pcm_substream_chip(substream) }; let runtime = unsafe { (*substream).runtime };
    unsafe { (*ensoniq).mode |= ES_MODE_CAPTURE; (*ensoniq).capture_substream = substream; (*runtime).hw = snd_ensoniq_capture; snd_pcm_set_sync(substream); snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &snd_es1370_hw_constraints_clock); snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &snd_es1371_hw_constraints_adc_clock); }
    0
}
unsafe extern "C" fn snd_ensoniq_playback1_close(substream: *mut snd_pcm_substream) -> c_int { let e = unsafe { snd_pcm_substream_chip(substream) }; unsafe { (*e).playback1_substream = ptr::null_mut(); (*e).mode &= !ES_MODE_PLAY1; } 0 }
unsafe extern "C" fn snd_ensoniq_playback2_close(substream: *mut snd_pcm_substream) -> c_int { let e = unsafe { snd_pcm_substream_chip(substream) }; unsafe { (*e).playback2_substream = ptr::null_mut(); (*e).u.es1370.pclkdiv_lock &= !(ES_MODE_PLAY2 as c_int); (*e).mode &= !ES_MODE_PLAY2; } 0 }
unsafe extern "C" fn snd_ensoniq_capture_close(substream: *mut snd_pcm_substream) -> c_int { let e = unsafe { snd_pcm_substream_chip(substream) }; unsafe { (*e).capture_substream = ptr::null_mut(); (*e).u.es1370.pclkdiv_lock &= !(ES_MODE_CAPTURE as c_int); (*e).mode &= !ES_MODE_CAPTURE; } 0 }

static snd_ensoniq_playback1_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ensoniq_playback1_open), close: Some(snd_ensoniq_playback1_close), prepare: Some(snd_ensoniq_playback1_prepare), trigger: Some(snd_ensoniq_trigger), pointer: Some(snd_ensoniq_playback1_pointer) };
static snd_ensoniq_playback2_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ensoniq_playback2_open), close: Some(snd_ensoniq_playback2_close), prepare: Some(snd_ensoniq_playback2_prepare), trigger: Some(snd_ensoniq_trigger), pointer: Some(snd_ensoniq_playback2_pointer) };
static snd_ensoniq_capture_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ensoniq_capture_open), close: Some(snd_ensoniq_capture_close), prepare: Some(snd_ensoniq_capture_prepare), trigger: Some(snd_ensoniq_trigger), pointer: Some(snd_ensoniq_capture_pointer) };
static surround_map: [snd_pcm_chmap_elem; 3] = [snd_pcm_chmap_elem { channels: 1, map: [SNDRV_CHMAP_MONO, 0, 0, 0] }, snd_pcm_chmap_elem { channels: 2, map: [SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, 0, 0] }, snd_pcm_chmap_elem { channels: 0, map: [0; 4] }];

unsafe extern "C" fn snd_ensoniq_pcm(ensoniq: *mut ensoniq, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut(); let mut err = unsafe { snd_pcm_new((*ensoniq).card, c"ES137x/1".as_ptr(), device, 1, 1, &mut pcm) };
    if err < 0 { return err; }
    unsafe { snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ensoniq_playback2_ops); snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ensoniq_playback1_ops); snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_ensoniq_capture_ops); (*pcm).private_data = ensoniq as *mut c_void; (*pcm).info_flags = 0; strscpy((*pcm).name.as_mut_ptr(), c"ES137x DAC2/ADC".as_ptr()); (*ensoniq).pcm1 = pcm; snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*ensoniq).pci).dev, 64 * 1024, 128 * 1024); err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, surround_map.as_ptr(), 2, 0, ptr::null_mut()); }
    err
}
unsafe extern "C" fn snd_ensoniq_pcm2(ensoniq: *mut ensoniq, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut(); let mut err = unsafe { snd_pcm_new((*ensoniq).card, c"ES137x/2".as_ptr(), device, 1, 0, &mut pcm) };
    if err < 0 { return err; }
    unsafe { snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ensoniq_playback1_ops); snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ensoniq_playback2_ops); (*pcm).private_data = ensoniq as *mut c_void; (*pcm).info_flags = 0; strscpy((*pcm).name.as_mut_ptr(), c"ES137x DAC1".as_ptr()); (*ensoniq).pcm2 = pcm; snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*ensoniq).pci).dev, 64 * 1024, 128 * 1024); err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, surround_map.as_ptr(), 2, 0, ptr::null_mut()); }
    err
}

unsafe extern "C" fn snd_ens1373_spdif_info(_k: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int { unsafe { (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958; (*uinfo).count = 1; } 0 }
unsafe extern "C" fn snd_ens1373_spdif_default_get(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; unsafe { for i in 0..4 { (*u).value.iec958.status[i] = (((*e).spdif_default >> (8 * i)) & 0xff) as c_uchar; } } 0 }
unsafe extern "C" fn snd_ens1373_spdif_default_put(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; let val = unsafe { ((*u).value.iec958.status[0] as c_uint) | ((*u).value.iec958.status[1] as c_uint) << 8 | ((*u).value.iec958.status[2] as c_uint) << 16 | ((*u).value.iec958.status[3] as c_uint) << 24 }; unsafe { let change = ((*e).spdif_default != val) as c_int; (*e).spdif_default = val; if change != 0 && (*e).playback1_substream.is_null() && (*e).playback2_substream.is_null() { outl(val, ES_REG(e, ES_REG_CHANNEL_STATUS)); } change } }
unsafe extern "C" fn snd_ens1373_spdif_mask_get(_k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { unsafe { for i in 0..4 { (*u).value.iec958.status[i] = 0xff; } } 0 }
unsafe extern "C" fn snd_ens1373_spdif_stream_get(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; unsafe { for i in 0..4 { (*u).value.iec958.status[i] = (((*e).spdif_stream >> (8 * i)) & 0xff) as c_uchar; } } 0 }
unsafe extern "C" fn snd_ens1373_spdif_stream_put(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; let val = unsafe { ((*u).value.iec958.status[0] as c_uint) | ((*u).value.iec958.status[1] as c_uint) << 8 | ((*u).value.iec958.status[2] as c_uint) << 16 | ((*u).value.iec958.status[3] as c_uint) << 24 }; unsafe { let change = ((*e).spdif_stream != val) as c_int; (*e).spdif_stream = val; if change != 0 && (!(*e).playback1_substream.is_null() || !(*e).playback2_substream.is_null()) { outl(val, ES_REG(e, ES_REG_CHANNEL_STATUS)); } change } }
unsafe extern "C" fn snd_es1371_spdif_get(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; unsafe { (*u).value.integer.value[0] = if ((*e).ctrl & ES_1373_SPDIF_THRU) != 0 { 1 } else { 0 }; } 0 }
unsafe extern "C" fn snd_es1371_spdif_put(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; let nval1 = unsafe { if (*u).value.integer.value[0] != 0 { ES_1373_SPDIF_THRU } else { 0 } }; let nval2 = unsafe { if (*u).value.integer.value[0] != 0 { ES_1373_SPDIF_EN } else { 0 } }; unsafe { let change = (((*e).ctrl & ES_1373_SPDIF_THRU) != nval1) as c_int; (*e).ctrl = ((*e).ctrl & !ES_1373_SPDIF_THRU) | nval1; (*e).cssr = ((*e).cssr & !ES_1373_SPDIF_EN) | nval2; outl((*e).ctrl, ES_REG(e, ES_REG_CONTROL)); outl((*e).cssr, ES_REG(e, ES_REG_STATUS)); change } }
unsafe extern "C" fn snd_es1373_rear_get(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; let mut val = 0; unsafe { if ((*e).cssr & (ES_1373_REAR_BIT27 | ES_1373_REAR_BIT26 | ES_1373_REAR_BIT24)) == ES_1373_REAR_BIT26 { val = 1; } (*u).value.integer.value[0] = val; } 0 }
unsafe extern "C" fn snd_es1373_rear_put(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; let nval1 = unsafe { if (*u).value.integer.value[0] != 0 { ES_1373_REAR_BIT26 } else { ES_1373_REAR_BIT27 | ES_1373_REAR_BIT24 } }; unsafe { let mask = ES_1373_REAR_BIT27 | ES_1373_REAR_BIT26 | ES_1373_REAR_BIT24; let change = (((*e).cssr & mask) != nval1) as c_int; (*e).cssr = ((*e).cssr & !mask) | nval1; outl((*e).cssr, ES_REG(e, ES_REG_STATUS)); change } }
unsafe extern "C" fn snd_es1373_line_get(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; unsafe { (*u).value.integer.value[0] = if ((*e).ctrl & ES_1371_GPIO_OUT(4)) != 0 { 1 } else { 0 }; } 0 }
unsafe extern "C" fn snd_es1373_line_put(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; unsafe { let ctrl = (*e).ctrl; if (*u).value.integer.value[0] != 0 { (*e).ctrl |= ES_1371_GPIO_OUT(4); } else { (*e).ctrl &= !ES_1371_GPIO_OUT(4); } let changed = (ctrl != (*e).ctrl) as c_int; if changed != 0 { outl((*e).ctrl, ES_REG(e, ES_REG_CONTROL)); } changed } }
unsafe extern "C" fn snd_ensoniq_mixer_free_ac97(ac97: *mut snd_ac97) { let e = unsafe { (*ac97).private_data as *mut ensoniq }; unsafe { (*e).u.es1371.ac97 = ptr::null_mut(); } }

#[repr(C)] pub struct es1371_quirk { pub vid: c_ushort, pub did: c_ushort, pub rev: c_uchar }
unsafe extern "C" fn es1371_quirk_lookup(ensoniq: *mut ensoniq, mut list: *const es1371_quirk) -> c_int {
    while unsafe { (*list).vid } != PCI_ANY_ID as c_ushort {
        if unsafe { (*(*ensoniq).pci).vendor == (*list).vid && (*(*ensoniq).pci).device == (*list).did && (*ensoniq).rev == (*list).rev as c_uint } { return 1; }
        list = unsafe { list.add(1) };
    }
    0
}

static es1371_spdif_present: [es1371_quirk; 6] = [
    es1371_quirk { vid: PCI_VENDOR_ID_ENSONIQ, did: PCI_DEVICE_ID_ENSONIQ_CT5880, rev: CT5880REV_CT5880_C as c_uchar },
    es1371_quirk { vid: PCI_VENDOR_ID_ENSONIQ, did: PCI_DEVICE_ID_ENSONIQ_CT5880, rev: CT5880REV_CT5880_D as c_uchar },
    es1371_quirk { vid: PCI_VENDOR_ID_ENSONIQ, did: PCI_DEVICE_ID_ENSONIQ_CT5880, rev: CT5880REV_CT5880_E as c_uchar },
    es1371_quirk { vid: PCI_VENDOR_ID_ENSONIQ, did: PCI_DEVICE_ID_ENSONIQ_ES1371, rev: ES1371REV_CT5880_A as c_uchar },
    es1371_quirk { vid: PCI_VENDOR_ID_ENSONIQ, did: PCI_DEVICE_ID_ENSONIQ_ES1371, rev: ES1371REV_ES1373_8 as c_uchar },
    es1371_quirk { vid: PCI_ANY_ID as c_ushort, did: PCI_ANY_ID as c_ushort, rev: 0 },
];

unsafe extern "C" fn snd_ensoniq_1371_mixer(ensoniq: *mut ensoniq, has_spdif: c_int, has_line: c_int) -> c_int {
    let card = unsafe { (*ensoniq).card }; let mut pbus: *mut snd_ac97_bus = ptr::null_mut(); let ops = snd_ac97_bus_ops { write: Some(snd_es1371_codec_write), read: Some(snd_es1371_codec_read), wait: Some(snd_es1371_codec_wait) };
    let mut err = unsafe { snd_ac97_bus(card, 0, &ops, ptr::null_mut(), &mut pbus) }; if err < 0 { return err; }
    let mut ac97: snd_ac97_template = unsafe { zeroed() }; ac97.private_data = ensoniq as *mut c_void; ac97.private_free = Some(snd_ensoniq_mixer_free_ac97); ac97.pci = unsafe { (*ensoniq).pci }; ac97.scaps = AC97_SCAP_AUDIO;
    err = unsafe { snd_ac97_mixer(pbus, &mut ac97, &mut (*ensoniq).u.es1371.ac97) }; if err < 0 { return err; }
    if has_spdif > 0 || (has_spdif == 0 && unsafe { es1371_quirk_lookup(ensoniq, es1371_spdif_present.as_ptr()) } != 0) {
        unsafe { (*ensoniq).spdif_default = SNDRV_PCM_DEFAULT_CON_SPDIF; (*ensoniq).spdif_stream = SNDRV_PCM_DEFAULT_CON_SPDIF; outl((*ensoniq).spdif_default, ES_REG(ensoniq, ES_REG_CHANNEL_STATUS)); }
    }
    if unsafe { (*(*ensoniq).u.es1371.ac97).ext_id & AC97_EI_SDAC } != 0 { unsafe { (*ensoniq).cssr &= !(ES_1373_REAR_BIT27 | ES_1373_REAR_BIT24); (*ensoniq).cssr |= ES_1373_REAR_BIT26; } }
    if has_line > 0 { err = 0; }
    err
}

unsafe extern "C" fn snd_ensoniq_control_get(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; let mask = unsafe { (*k).private_value as c_uint }; unsafe { (*u).value.integer.value[0] = if ((*e).ctrl & mask) != 0 { 1 } else { 0 }; } 0 }
unsafe extern "C" fn snd_ensoniq_control_put(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { let e = unsafe { snd_kcontrol_chip(k) }; let mask = unsafe { (*k).private_value as c_uint }; let nval = unsafe { if (*u).value.integer.value[0] != 0 { mask } else { 0 } }; unsafe { let change = (((*e).ctrl & mask) != nval) as c_int; (*e).ctrl = ((*e).ctrl & !mask) | nval; outl((*e).ctrl, ES_REG(e, ES_REG_CONTROL)); change } }
unsafe extern "C" fn snd_ensoniq_mixer_free_ak4531(ak4531: *mut snd_ak4531) { let e = unsafe { (*ak4531).private_data as *mut ensoniq }; unsafe { (*e).u.es1370.ak4531 = ptr::null_mut(); } }
unsafe extern "C" fn snd_ensoniq_1370_mixer(ensoniq: *mut ensoniq) -> c_int {
    unsafe { outw(ES_1370_CODEC_WRITE(AK4531_RESET, 0x02), ES_REG(ensoniq, ES_REG_1370_CODEC)); inw(ES_REG(ensoniq, ES_REG_1370_CODEC)); udelay(100); outw(ES_1370_CODEC_WRITE(AK4531_RESET, 0x03), ES_REG(ensoniq, ES_REG_1370_CODEC)); inw(ES_REG(ensoniq, ES_REG_1370_CODEC)); udelay(100); }
    let mut ak4531: snd_ak4531 = unsafe { zeroed() }; ak4531.write = Some(snd_es1370_codec_write); ak4531.private_data = ensoniq as *mut c_void; ak4531.private_free = Some(snd_ensoniq_mixer_free_ak4531);
    unsafe { snd_ak4531_mixer((*ensoniq).card, &mut ak4531, &mut (*ensoniq).u.es1370.ak4531) }
}

unsafe extern "C" fn snd_ensoniq_get_joystick_port(ensoniq: *mut ensoniq, dev: c_int) -> c_int {
    let port = unsafe { joystick_port[dev as usize] };
    match port { 0 | 1 | 0x200 | 0x208 | 0x210 | 0x218 => port, _ => { unsafe { dev_err((*(*ensoniq).card).dev, c"invalid joystick port %#x".as_ptr(), port); } 0 } }
}
unsafe extern "C" fn snd_ensoniq_create_gameport(ensoniq: *mut ensoniq, dev: c_int) -> c_int {
    let mut io_port = unsafe { snd_ensoniq_get_joystick_port(ensoniq, dev) };
    match io_port {
        0 => return -ENOSYS,
        1 => { io_port = 0x200; while io_port <= 0x218 { if unsafe { !request_region(io_port, 8, c"ens137x: gameport".as_ptr()).is_null() } { break; } io_port += 8; } if io_port > 0x218 { unsafe { dev_warn((*(*ensoniq).card).dev, c"no gameport ports available\n".as_ptr()); } return -EBUSY; } }
        _ => if unsafe { request_region(io_port, 8, c"ens137x: gameport".as_ptr()).is_null() } { unsafe { dev_warn((*(*ensoniq).card).dev, c"gameport io port %#x in use\n".as_ptr(), io_port); } return -EBUSY; },
    }
    let gp = unsafe { gameport_allocate_port() }; unsafe { (*ensoniq).gameport = gp; }
    if gp.is_null() { unsafe { dev_err((*(*ensoniq).card).dev, c"cannot allocate memory for gameport\n".as_ptr()); release_region(io_port, 8); } return -ENOMEM; }
    unsafe { gameport_set_name(gp, c"ES137x".as_ptr()); gameport_set_phys(gp, c"pci%s/gameport0".as_ptr(), pci_name((*ensoniq).pci)); gameport_set_dev_parent(gp, &mut (*(*ensoniq).pci).dev); (*gp).io = io_port; (*ensoniq).ctrl |= ES_JYSTK_EN; (*ensoniq).ctrl &= !ES_1371_JOY_ASELM; (*ensoniq).ctrl |= ES_1371_JOY_ASEL(((io_port - 0x200) / 8) as c_uint); outl((*ensoniq).ctrl, ES_REG(ensoniq, ES_REG_CONTROL)); gameport_register_port((*ensoniq).gameport); }
    0
}
unsafe extern "C" fn snd_ensoniq_free_gameport(ensoniq: *mut ensoniq) { unsafe { if !(*ensoniq).gameport.is_null() { let port = (*(*ensoniq).gameport).io; gameport_unregister_port((*ensoniq).gameport); (*ensoniq).gameport = ptr::null_mut(); (*ensoniq).ctrl &= !ES_JYSTK_EN; outl((*ensoniq).ctrl, ES_REG(ensoniq, ES_REG_CONTROL)); release_region(port, 8); } } }

unsafe extern "C" fn snd_ensoniq_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let e = unsafe { (*entry).private_data as *mut ensoniq };
    unsafe { snd_iprintf(buffer, c"Ensoniq AudioPCI ES137x\n\n".as_ptr()); snd_iprintf(buffer, c"Joystick enable  : %s\n".as_ptr(), str_on_off((*e).ctrl & ES_JYSTK_EN)); snd_iprintf(buffer, c"MIC +5V bias     : %s\n".as_ptr(), str_on_off((*e).ctrl & ES_1370_XCTL1)); snd_iprintf(buffer, c"Line In to AOUT  : %s\n".as_ptr(), str_on_off((*e).ctrl & ES_1370_XCTL0)); snd_iprintf(buffer, c"Joystick port    : 0x%x\n".as_ptr(), ES_1371_JOY_ASELI((*e).ctrl) * 8 + 0x200); }
}
unsafe extern "C" fn snd_ensoniq_proc_init(ensoniq: *mut ensoniq) { unsafe { snd_card_ro_proc_new((*ensoniq).card, c"audiopci".as_ptr(), ensoniq as *mut c_void, Some(snd_ensoniq_proc_read)); } }
unsafe extern "C" fn snd_ensoniq_free(card: *mut snd_card) { let e = unsafe { (*card).private_data as *mut ensoniq }; unsafe { snd_ensoniq_free_gameport(e); outl(ES_1370_SERR_DISABLE, ES_REG(e, ES_REG_CONTROL)); outl(0, ES_REG(e, ES_REG_CONTROL)); outl(0, ES_REG(e, ES_REG_SERIAL)); } }

static es1371_ac97_reset_hack: [es1371_quirk; 6] = es1371_spdif_present;

unsafe extern "C" fn snd_ensoniq_chip_init(ensoniq: *mut ensoniq) {
    unsafe {
        outl((*ensoniq).ctrl, ES_REG(ensoniq, ES_REG_CONTROL)); outl((*ensoniq).sctrl, ES_REG(ensoniq, ES_REG_SERIAL));
        if !(*ensoniq).dma_bug.is_null() { outl(ES_MEM_PAGEO(ES_PAGE_ADC), ES_REG(ensoniq, ES_REG_MEM_PAGE)); outl((*(*ensoniq).dma_bug).addr as c_uint, ES_REG(ensoniq, ES_REG_PHANTOM_FRAME)); outl(0, ES_REG(ensoniq, ES_REG_PHANTOM_COUNT)); }
        outl(0, ES_REG(ensoniq, ES_REG_1371_LEGACY));
        if es1371_quirk_lookup(ensoniq, es1371_ac97_reset_hack.as_ptr()) != 0 { outl((*ensoniq).cssr, ES_REG(ensoniq, ES_REG_STATUS)); msleep(20); }
        outl((*ensoniq).ctrl | ES_1371_SYNC_RES, ES_REG(ensoniq, ES_REG_CONTROL)); inl(ES_REG(ensoniq, ES_REG_CONTROL)); udelay(20); outl((*ensoniq).ctrl, ES_REG(ensoniq, ES_REG_CONTROL));
        snd_es1371_wait_src_ready(ensoniq); outl(ES_1371_SRC_DISABLE, ES_REG(ensoniq, ES_REG_1371_SMPRATE));
        let mut idx = 0; while idx < 0x80 { snd_es1371_src_write(ensoniq, idx, 0); idx += 1; }
        snd_es1371_src_write(ensoniq, (ES_SMPREG_DAC1 + ES_SMPREG_TRUNC_N) as c_ushort, 16 << 4); snd_es1371_src_write(ensoniq, (ES_SMPREG_DAC1 + ES_SMPREG_INT_REGS) as c_ushort, 16 << 10);
        snd_es1371_src_write(ensoniq, (ES_SMPREG_DAC2 + ES_SMPREG_TRUNC_N) as c_ushort, 16 << 4); snd_es1371_src_write(ensoniq, (ES_SMPREG_DAC2 + ES_SMPREG_INT_REGS) as c_ushort, 16 << 10);
        snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_ADC as c_ushort, 1 << 12); snd_es1371_src_write(ensoniq, (ES_SMPREG_VOL_ADC + 1) as c_ushort, 1 << 12);
        snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_DAC1 as c_ushort, 1 << 12); snd_es1371_src_write(ensoniq, (ES_SMPREG_VOL_DAC1 + 1) as c_ushort, 1 << 12);
        snd_es1371_src_write(ensoniq, ES_SMPREG_VOL_DAC2 as c_ushort, 1 << 12); snd_es1371_src_write(ensoniq, (ES_SMPREG_VOL_DAC2 + 1) as c_ushort, 1 << 12);
        snd_es1371_adc_rate(ensoniq, 22050); snd_es1371_dac1_rate(ensoniq, 22050); snd_es1371_dac2_rate(ensoniq, 22050);
        snd_es1371_wait_src_ready(ensoniq); outl(0, ES_REG(ensoniq, ES_REG_1371_SMPRATE)); outl(ES_1371_CODEC_WRITE(0, 0), ES_REG(ensoniq, ES_REG_1371_CODEC));
        (*ensoniq).uartc = 0; outb((*ensoniq).uartc, ES_REG(ensoniq, ES_REG_UART_CONTROL)); outb(0, ES_REG(ensoniq, ES_REG_UART_RES)); outl((*ensoniq).cssr, ES_REG(ensoniq, ES_REG_STATUS));
    }
}

unsafe extern "C" fn snd_ensoniq_suspend(dev: *mut device) -> c_int { let card = unsafe { dev_get_drvdata(dev) as *mut snd_card }; let e = unsafe { (*card).private_data as *mut ensoniq }; unsafe { snd_power_change_state(card, SNDRV_CTL_POWER_D3hot); snd_ac97_suspend((*e).u.es1371.ac97); outw(ES_1370_CODEC_WRITE(AK4531_RESET, 0x02), ES_REG(e, ES_REG_1370_CODEC)); inw(ES_REG(e, ES_REG_1370_CODEC)); udelay(100); outw(ES_1370_CODEC_WRITE(AK4531_RESET, 0x03), ES_REG(e, ES_REG_1370_CODEC)); inw(ES_REG(e, ES_REG_1370_CODEC)); udelay(100); snd_ak4531_suspend((*e).u.es1370.ak4531); } 0 }
unsafe extern "C" fn snd_ensoniq_resume(dev: *mut device) -> c_int { let card = unsafe { dev_get_drvdata(dev) as *mut snd_card }; let e = unsafe { (*card).private_data as *mut ensoniq }; unsafe { snd_ensoniq_chip_init(e); snd_ac97_resume((*e).u.es1371.ac97); snd_ak4531_resume((*e).u.es1370.ak4531); snd_power_change_state(card, SNDRV_CTL_POWER_D0); } 0 }

unsafe extern "C" fn snd_ensoniq_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let e = unsafe { (*card).private_data as *mut ensoniq }; let mut err = unsafe { pcim_enable_device(pci) }; if err < 0 { return err; }
    unsafe { (*e).card = card; (*e).pci = pci; (*e).irq = -1; }
    err = unsafe { pcim_request_all_regions(pci, c"Ensoniq AudioPCI".as_ptr()) }; if err < 0 { return err; }
    unsafe { (*e).port = pci_resource_start(pci, 0); if devm_request_irq(&mut (*pci).dev, (*pci).irq, Some(snd_audiopci_interrupt), IRQF_SHARED, c"ens137x".as_ptr(), e as *mut c_void) != 0 { dev_err((*card).dev, c"unable to grab IRQ %d\n".as_ptr(), (*pci).irq); return -EBUSY; } (*e).irq = (*pci).irq; (*card).sync_irq = (*e).irq; (*e).dma_bug = snd_devm_alloc_pages(&mut (*pci).dev, SNDRV_DMA_TYPE_DEV, 16); pci_set_master(pci); (*e).rev = (*pci).revision as c_uint; (*e).ctrl = ES_1370_CDC_EN | ES_1370_PCLKDIVO(ES_1370_SRTODIV(8000)); (*e).sctrl = 0; (*e).cssr = 0; if es1371_quirk_lookup(e, es1371_ac97_reset_hack.as_ptr()) != 0 { (*e).cssr |= ES_1371_ST_AC97_RST; } (*card).private_free = Some(snd_ensoniq_free); snd_ensoniq_chip_init(e); snd_ensoniq_proc_init(e); }
    0
}

unsafe extern "C" fn snd_ensoniq_midi_interrupt(ensoniq: *mut ensoniq) {
    let rmidi = unsafe { (*ensoniq).rmidi }; if rmidi.is_null() { return; }
    let mut byte: c_uchar = 0; let mut mask = unsafe { if ((*ensoniq).uartm & ES_MODE_INPUT) != 0 { ES_RXRDY } else { 0 } };
    while mask != 0 { let status = unsafe { inb(ES_REG(ensoniq, ES_REG_UART_STATUS)) as c_uint }; if (status & mask) == 0 { break; } byte = unsafe { inb(ES_REG(ensoniq, ES_REG_UART_DATA)) }; unsafe { snd_rawmidi_receive((*ensoniq).midi_input, &mut byte, 1); } }
    mask = unsafe { if ((*ensoniq).uartm & ES_MODE_OUTPUT) != 0 { ES_TXRDY } else { 0 } };
    while mask != 0 { let status = unsafe { inb(ES_REG(ensoniq, ES_REG_UART_STATUS)) as c_uint }; if (status & mask) == 0 { break; } if unsafe { snd_rawmidi_transmit((*ensoniq).midi_output, &mut byte, 1) } != 1 { unsafe { (*ensoniq).uartc &= !ES_TXINTENM; outb((*ensoniq).uartc, ES_REG(ensoniq, ES_REG_UART_CONTROL)); } mask &= !ES_TXRDY; } else { unsafe { outb(byte as c_uint, ES_REG(ensoniq, ES_REG_UART_DATA)); } } }
}

unsafe extern "C" fn snd_ensoniq_midi_input_open(s: *mut snd_rawmidi_substream) -> c_int { let e = unsafe { (*(*s).rmidi).private_data as *mut ensoniq }; unsafe { (*e).uartm |= ES_MODE_INPUT; (*e).midi_input = s; if ((*e).uartm & ES_MODE_OUTPUT) == 0 { outb(ES_CNTRL(3), ES_REG(e, ES_REG_UART_CONTROL)); (*e).uartc = 0; outb((*e).uartc, ES_REG(e, ES_REG_UART_CONTROL)); (*e).ctrl |= ES_UART_EN; outl((*e).ctrl, ES_REG(e, ES_REG_CONTROL)); } } 0 }
unsafe extern "C" fn snd_ensoniq_midi_input_close(s: *mut snd_rawmidi_substream) -> c_int { let e = unsafe { (*(*s).rmidi).private_data as *mut ensoniq }; unsafe { if ((*e).uartm & ES_MODE_OUTPUT) == 0 { (*e).uartc = 0; outb((*e).uartc, ES_REG(e, ES_REG_UART_CONTROL)); (*e).ctrl &= !ES_UART_EN; outl((*e).ctrl, ES_REG(e, ES_REG_CONTROL)); } else { (*e).uartc &= !ES_RXINTEN; outb((*e).uartc, ES_REG(e, ES_REG_UART_CONTROL)); } (*e).midi_input = ptr::null_mut(); (*e).uartm &= !ES_MODE_INPUT; } 0 }
unsafe extern "C" fn snd_ensoniq_midi_output_open(s: *mut snd_rawmidi_substream) -> c_int { let e = unsafe { (*(*s).rmidi).private_data as *mut ensoniq }; unsafe { (*e).uartm |= ES_MODE_OUTPUT; (*e).midi_output = s; if ((*e).uartm & ES_MODE_INPUT) == 0 { outb(ES_CNTRL(3), ES_REG(e, ES_REG_UART_CONTROL)); (*e).uartc = 0; outb((*e).uartc, ES_REG(e, ES_REG_UART_CONTROL)); (*e).ctrl |= ES_UART_EN; outl((*e).ctrl, ES_REG(e, ES_REG_CONTROL)); } } 0 }
unsafe extern "C" fn snd_ensoniq_midi_output_close(s: *mut snd_rawmidi_substream) -> c_int { let e = unsafe { (*(*s).rmidi).private_data as *mut ensoniq }; unsafe { if ((*e).uartm & ES_MODE_INPUT) == 0 { (*e).uartc = 0; outb((*e).uartc, ES_REG(e, ES_REG_UART_CONTROL)); (*e).ctrl &= !ES_UART_EN; outl((*e).ctrl, ES_REG(e, ES_REG_CONTROL)); } else { (*e).uartc &= !ES_TXINTENM; outb((*e).uartc, ES_REG(e, ES_REG_UART_CONTROL)); } (*e).midi_output = ptr::null_mut(); (*e).uartm &= !ES_MODE_OUTPUT; } 0 }
unsafe extern "C" fn snd_ensoniq_midi_input_trigger(s: *mut snd_rawmidi_substream, up: c_int) { let e = unsafe { (*(*s).rmidi).private_data as *mut ensoniq }; unsafe { if up != 0 { if ((*e).uartc & ES_RXINTEN) == 0 { let mut idx = 0; while idx < 32 { inb(ES_REG(e, ES_REG_UART_DATA)); idx += 1; } (*e).uartc |= ES_RXINTEN; outb((*e).uartc, ES_REG(e, ES_REG_UART_CONTROL)); } } else if ((*e).uartc & ES_RXINTEN) != 0 { (*e).uartc &= !ES_RXINTEN; outb((*e).uartc, ES_REG(e, ES_REG_UART_CONTROL)); } } }
unsafe extern "C" fn snd_ensoniq_midi_output_trigger(s: *mut snd_rawmidi_substream, up: c_int) { let e = unsafe { (*(*s).rmidi).private_data as *mut ensoniq }; let mut byte: c_uchar = 0; unsafe { if up != 0 { if ES_TXINTENI((*e).uartc) == 0 { (*e).uartc |= ES_TXINTENO(1); while ES_TXINTENI((*e).uartc) == 1 && (inb(ES_REG(e, ES_REG_UART_STATUS)) as c_uint & ES_TXRDY) != 0 { if snd_rawmidi_transmit(s, &mut byte, 1) != 1 { (*e).uartc &= !ES_TXINTENM; } else { outb(byte as c_uint, ES_REG(e, ES_REG_UART_DATA)); } } outb((*e).uartc, ES_REG(e, ES_REG_UART_CONTROL)); } } else if ES_TXINTENI((*e).uartc) == 1 { (*e).uartc &= !ES_TXINTENM; outb((*e).uartc, ES_REG(e, ES_REG_UART_CONTROL)); } } }

static snd_ensoniq_midi_output: snd_rawmidi_ops = snd_rawmidi_ops { open: Some(snd_ensoniq_midi_output_open), close: Some(snd_ensoniq_midi_output_close), trigger: Some(snd_ensoniq_midi_output_trigger) };
static snd_ensoniq_midi_input: snd_rawmidi_ops = snd_rawmidi_ops { open: Some(snd_ensoniq_midi_input_open), close: Some(snd_ensoniq_midi_input_close), trigger: Some(snd_ensoniq_midi_input_trigger) };
unsafe extern "C" fn snd_ensoniq_midi(ensoniq: *mut ensoniq, device: c_int) -> c_int { let mut rmidi: *mut snd_rawmidi = ptr::null_mut(); let err = unsafe { snd_rawmidi_new((*ensoniq).card, c"ES1370/1".as_ptr(), device, 1, 1, &mut rmidi) }; if err < 0 { return err; } unsafe { strscpy((*rmidi).name.as_mut_ptr(), c"ES137x".as_ptr()); snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_ensoniq_midi_output); snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_ensoniq_midi_input); (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX; (*rmidi).private_data = ensoniq as *mut c_void; (*ensoniq).rmidi = rmidi; } 0 }

unsafe extern "C" fn snd_audiopci_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let ensoniq = dev_id as *mut ensoniq; if ensoniq.is_null() { return IRQ_NONE; }
    let status = unsafe { inl(ES_REG(ensoniq, ES_REG_STATUS)) }; if (status & ES_INTR) == 0 { return IRQ_NONE; }
    unsafe { let mut sctrl = (*ensoniq).sctrl; if (status & ES_DAC1) != 0 { sctrl &= !ES_P1_INT_EN; } if (status & ES_DAC2) != 0 { sctrl &= !ES_P2_INT_EN; } if (status & ES_ADC) != 0 { sctrl &= !ES_R1_INT_EN; } outl(sctrl, ES_REG(ensoniq, ES_REG_SERIAL)); outl((*ensoniq).sctrl, ES_REG(ensoniq, ES_REG_SERIAL)); }
    unsafe { if (status & ES_UART) != 0 { snd_ensoniq_midi_interrupt(ensoniq); } if (status & ES_DAC2) != 0 && !(*ensoniq).playback2_substream.is_null() { snd_pcm_period_elapsed((*ensoniq).playback2_substream); } if (status & ES_ADC) != 0 && !(*ensoniq).capture_substream.is_null() { snd_pcm_period_elapsed((*ensoniq).capture_substream); } if (status & ES_DAC1) != 0 && !(*ensoniq).playback1_substream.is_null() { snd_pcm_period_elapsed((*ensoniq).playback1_substream); } }
    IRQ_HANDLED
}

unsafe extern "C" fn __snd_audiopci_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    static mut DEV: c_int = 0;
    unsafe {
        if DEV >= SNDRV_CARDS as c_int { return -ENODEV; }
        if !enable[DEV as usize] { DEV += 1; return -ENOENT; }
        let mut card: *mut snd_card = ptr::null_mut();
        let mut err = snd_devm_card_new(&mut (*pci).dev, index[DEV as usize], id[DEV as usize], ptr::null_mut(), size_of::<ensoniq>(), &mut card); if err < 0 { return err; }
        let ensoniq = (*card).private_data as *mut ensoniq;
        err = snd_ensoniq_create(card, pci); if err < 0 { return err; }
        err = snd_ensoniq_1370_mixer(ensoniq); if err < 0 { return err; }
        err = snd_ensoniq_1371_mixer(ensoniq, spdif[DEV as usize], lineio[DEV as usize]); if err < 0 { return err; }
        err = snd_ensoniq_pcm(ensoniq, 0); if err < 0 { return err; }
        err = snd_ensoniq_pcm2(ensoniq, 1); if err < 0 { return err; }
        err = snd_ensoniq_midi(ensoniq, 0); if err < 0 { return err; }
        snd_ensoniq_create_gameport(ensoniq, DEV);
        strscpy((*card).driver.as_mut_ptr(), c"ENS137x".as_ptr()); strscpy((*card).shortname.as_mut_ptr(), c"Ensoniq AudioPCI".as_ptr());
        sprintf((*card).longname.as_mut_ptr(), c"%s %s at 0x%lx, irq %i".as_ptr(), (*card).shortname.as_ptr(), (*card).driver.as_ptr(), (*ensoniq).port, (*ensoniq).irq);
        err = snd_card_register(card); if err < 0 { return err; }
        pci_set_drvdata(pci, card as *mut c_void); DEV += 1; 0
    }
}
unsafe extern "C" fn snd_audiopci_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int { unsafe { snd_card_free_on_error(&mut (*pci).dev, __snd_audiopci_probe(pci, pci_id)) } }

static snd_audiopci_ids: [pci_device_id; 4] = [
    pci_device_id { vendor: 0x1274, device: 0x5000, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
    pci_device_id { vendor: 0x1274, device: 0x1371, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
    pci_device_id { vendor: 0x1274, device: 0x5880, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];
static ens137x_driver: pci_driver = pci_driver { name: c"ens137x".as_ptr(), id_table: snd_audiopci_ids.as_ptr(), probe: Some(snd_audiopci_probe), driver: driver_inner { pm: ptr::null() } };

/* module_param_array, MODULE_* metadata, DEFINE_SIMPLE_DEV_PM_OPS, and module_pci_driver
 * are kernel build macros in the C source; their dependency intent is preserved here.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
