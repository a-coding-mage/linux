/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Routines for Asahi Kasei AK4117
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
 */

pub const AK4117_REG_PWRDN: u32 = 0x00; /* power down */
pub const AK4117_REG_CLOCK: u32 = 0x01; /* clock control */
pub const AK4117_REG_IO: u32 = 0x02; /* input/output control */
pub const AK4117_REG_INT0_MASK: u32 = 0x03; /* interrupt0 mask */
pub const AK4117_REG_INT1_MASK: u32 = 0x04; /* interrupt1 mask */
pub const AK4117_REG_RCS0: u32 = 0x05; /* receiver status 0 */
pub const AK4117_REG_RCS1: u32 = 0x06; /* receiver status 1 */
pub const AK4117_REG_RCS2: u32 = 0x07; /* receiver status 2 */
pub const AK4117_REG_RXCSB0: u32 = 0x08; /* RX channel status byte 0 */
pub const AK4117_REG_RXCSB1: u32 = 0x09; /* RX channel status byte 1 */
pub const AK4117_REG_RXCSB2: u32 = 0x0a; /* RX channel status byte 2 */
pub const AK4117_REG_RXCSB3: u32 = 0x0b; /* RX channel status byte 3 */
pub const AK4117_REG_RXCSB4: u32 = 0x0c; /* RX channel status byte 4 */
pub const AK4117_REG_Pc0: u32 = 0x0d; /* burst preamble Pc byte 0 */
pub const AK4117_REG_Pc1: u32 = 0x0e; /* burst preamble Pc byte 1 */
pub const AK4117_REG_Pd0: u32 = 0x0f; /* burst preamble Pd byte 0 */
pub const AK4117_REG_Pd1: u32 = 0x10; /* burst preamble Pd byte 1 */
pub const AK4117_REG_QSUB_ADDR: u32 = 0x11; /* Q-subcode address + control */
pub const AK4117_REG_QSUB_TRACK: u32 = 0x12; /* Q-subcode track */
pub const AK4117_REG_QSUB_INDEX: u32 = 0x13; /* Q-subcode index */
pub const AK4117_REG_QSUB_MINUTE: u32 = 0x14; /* Q-subcode minute */
pub const AK4117_REG_QSUB_SECOND: u32 = 0x15; /* Q-subcode second */
pub const AK4117_REG_QSUB_FRAME: u32 = 0x16; /* Q-subcode frame */
pub const AK4117_REG_QSUB_ZERO: u32 = 0x17; /* Q-subcode zero */
pub const AK4117_REG_QSUB_ABSMIN: u32 = 0x18; /* Q-subcode absolute minute */
pub const AK4117_REG_QSUB_ABSSEC: u32 = 0x19; /* Q-subcode absolute second */
pub const AK4117_REG_QSUB_ABSFRM: u32 = 0x1a; /* Q-subcode absolute frame */

pub const AK4117_REG_RXCSB_SIZE: u32 = (AK4117_REG_RXCSB4 - AK4117_REG_RXCSB0) + 1;
pub const AK4117_REG_QSUB_SIZE: u32 = (AK4117_REG_QSUB_ABSFRM - AK4117_REG_QSUB_ADDR) + 1;

pub const AK4117_EXCT: u32 = 1 << 4;
pub const AK4117_XTL1: u32 = 1 << 3;
pub const AK4117_XTL0: u32 = 1 << 2;
pub const AK4117_XTL_11_2896M: u32 = 0;
pub const AK4117_XTL_12_288M: u32 = AK4117_XTL0;
pub const AK4117_XTL_24_576M: u32 = AK4117_XTL1;
pub const AK4117_XTL_EXT: u32 = AK4117_XTL1 | AK4117_XTL0;
pub const AK4117_PWN: u32 = 1 << 1;
pub const AK4117_RST: u32 = 1 << 0;

pub const AK4117_LP: u32 = 1 << 7;
pub const AK4117_PKCS1: u32 = 1 << 6;
pub const AK4117_PKCS0: u32 = 1 << 5;
pub const AK4117_PKCS_512fs: u32 = 0;
pub const AK4117_PKCS_256fs: u32 = AK4117_PKCS0;
pub const AK4117_PKCS_128fs: u32 = AK4117_PKCS1;
pub const AK4117_DIV: u32 = 1 << 4;
pub const AK4117_XCKS1: u32 = 1 << 3;
pub const AK4117_XCKS0: u32 = 1 << 2;
pub const AK4117_XCKS_128fs: u32 = 0;
pub const AK4117_XCKS_256fs: u32 = AK4117_XCKS0;
pub const AK4117_XCKS_512fs: u32 = AK4117_XCKS1;
pub const AK4117_XCKS_1024fs: u32 = AK4117_XCKS1 | AK4117_XCKS0;
pub const AK4117_CM1: u32 = 1 << 1;
pub const AK4117_CM0: u32 = 1 << 0;
pub const AK4117_CM_PLL: u32 = 0;
pub const AK4117_CM_XTAL: u32 = AK4117_CM0;
pub const AK4117_CM_PLL_XTAL: u32 = AK4117_CM1;
pub const AK4117_CM_MONITOR: u32 = AK4117_CM0 | AK4117_CM1;

pub const AK4117_IPS: u32 = 1 << 7;
pub const AK4117_UOUTE: u32 = 1 << 6;
pub const AK4117_CS12: u32 = 1 << 5;
pub const AK4117_EFH2: u32 = 1 << 4;
pub const AK4117_EFH1: u32 = 1 << 3;
pub const AK4117_EFH_512LRCLK: u32 = 0;
pub const AK4117_EFH_1024LRCLK: u32 = AK4117_EFH1;
pub const AK4117_EFH_2048LRCLK: u32 = AK4117_EFH2;
pub const AK4117_EFH_4096LRCLK: u32 = AK4117_EFH1 | AK4117_EFH2;
pub const AK4117_DIF2: u32 = 1 << 2;
pub const AK4117_DIF1: u32 = 1 << 1;
pub const AK4117_DIF0: u32 = 1 << 0;
pub const AK4117_DIF_16R: u32 = 0;
pub const AK4117_DIF_18R: u32 = AK4117_DIF0;
pub const AK4117_DIF_20R: u32 = AK4117_DIF1;
pub const AK4117_DIF_24R: u32 = AK4117_DIF1 | AK4117_DIF0;
pub const AK4117_DIF_24L: u32 = AK4117_DIF2;
pub const AK4117_DIF_24I2S: u32 = AK4117_DIF2 | AK4117_DIF0;

pub const AK4117_MULK: u32 = 1 << 7;
pub const AK4117_MPAR: u32 = 1 << 6;
pub const AK4117_MAUTO: u32 = 1 << 5;
pub const AK4117_MV: u32 = 1 << 4;
pub const AK4117_MAUD: u32 = 1 << 3;
pub const AK4117_MSTC: u32 = 1 << 2;
pub const AK4117_MCIT: u32 = 1 << 1;
pub const AK4117_MQIT: u32 = 1 << 0;

pub const AK4117_UNLCK: u32 = 1 << 7;
pub const AK4117_PAR: u32 = 1 << 6;
pub const AK4117_AUTO: u32 = 1 << 5;
pub const AK4117_V: u32 = 1 << 4;
pub const AK4117_AUDION: u32 = 1 << 3;
pub const AK4117_STC: u32 = 1 << 2;
pub const AK4117_CINT: u32 = 1 << 1;
pub const AK4117_QINT: u32 = 1 << 0;

pub const AK4117_DTSCD: u32 = 1 << 6;
pub const AK4117_NPCM: u32 = 1 << 5;
pub const AK4117_PEM: u32 = 1 << 4;
pub const AK4117_FS3: u32 = 1 << 3;
pub const AK4117_FS2: u32 = 1 << 2;
pub const AK4117_FS1: u32 = 1 << 1;
pub const AK4117_FS0: u32 = 1 << 0;
pub const AK4117_FS_44100HZ: u32 = 0;
pub const AK4117_FS_48000HZ: u32 = AK4117_FS1;
pub const AK4117_FS_32000HZ: u32 = AK4117_FS1 | AK4117_FS0;
pub const AK4117_FS_88200HZ: u32 = AK4117_FS3;
pub const AK4117_FS_96000HZ: u32 = AK4117_FS3 | AK4117_FS1;
pub const AK4117_FS_176400HZ: u32 = AK4117_FS3 | AK4117_FS2;
pub const AK4117_FS_192000HZ: u32 = AK4117_FS3 | AK4117_FS2 | AK4117_FS1;

pub const AK4117_CCRC: u32 = 1 << 1;
pub const AK4117_QCRC: u32 = 1 << 0;
pub const AK4117_CHECK_NO_STAT: u32 = 1 << 0;
pub const AK4117_CHECK_NO_RATE: u32 = 1 << 1;
pub const AK4117_CONTROLS: usize = 13;

pub type Ak4117Write = unsafe extern "C" fn(*mut core::ffi::c_void, u8, u8);
pub type Ak4117Read = unsafe extern "C" fn(*mut core::ffi::c_void, u8) -> u8;

#[repr(i32)]
pub enum Ak4117Error {
    AK4117_PARITY_ERRORS,
    AK4117_V_BIT_ERRORS,
    AK4117_QCRC_ERRORS,
    AK4117_CCRC_ERRORS,
    AK4117_NUM_ERRORS,
}

#[repr(C)]
pub struct ak4117 {
    pub card: *mut snd_card,
    pub write: Option<Ak4117Write>,
    pub read: Option<Ak4117Read>,
    pub private_data: *mut core::ffi::c_void,
    /* C bitfield: unsigned int init: 1; */
    pub init: u32,
    pub lock: spinlock_t,
    pub regmap: [u8; 5],
    pub kctls: [*mut snd_kcontrol; AK4117_CONTROLS],
    pub substream: *mut snd_pcm_substream,
    pub errors: [c_ulong; AK4117_NUM_ERRORS as usize],
    pub rcs0: u8,
    pub rcs1: u8,
    pub rcs2: u8,
    pub timer: timer_list, /* statistic timer */
    pub change_callback_private: *mut core::ffi::c_void,
    pub change_callback: Option<unsafe extern "C" fn(*mut ak4117, u8, u8)>,
}

extern "C" {
    pub fn snd_ak4117_create(card: *mut snd_card, read: Option<Ak4117Read>, write: Option<Ak4117Write>, pgm: *const u8, private_data: *mut core::ffi::c_void, r_ak4117: *mut *mut ak4117) -> i32;
    pub fn snd_ak4117_reg_write(ak4117: *mut ak4117, reg: u8, mask: u8, val: u8);
    pub fn snd_ak4117_reinit(ak4117: *mut ak4117);
    pub fn snd_ak4117_build(ak4117: *mut ak4117, capture_substream: *mut snd_pcm_substream) -> i32;
    pub fn snd_ak4117_external_rate(ak4117: *mut ak4117) -> i32;
    pub fn snd_ak4117_check_rate_and_errors(ak4117: *mut ak4117, flags: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
