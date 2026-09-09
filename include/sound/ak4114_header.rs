/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Routines for Asahi Kasei AK4114. */

use core::ffi::c_void;

pub const AK4114_REG_PWRDN: u8 = 0x00;
pub const AK4114_REG_FORMAT: u8 = 0x01;
pub const AK4114_REG_IO0: u8 = 0x02;
pub const AK4114_REG_IO1: u8 = 0x03;
pub const AK4114_REG_INT0_MASK: u8 = 0x04;
pub const AK4114_REG_INT1_MASK: u8 = 0x05;
pub const AK4114_REG_RCS0: u8 = 0x06;
pub const AK4114_REG_RCS1: u8 = 0x07;
pub const AK4114_REG_RXCSB0: u8 = 0x08;
pub const AK4114_REG_RXCSB1: u8 = 0x09;
pub const AK4114_REG_RXCSB2: u8 = 0x0a;
pub const AK4114_REG_RXCSB3: u8 = 0x0b;
pub const AK4114_REG_RXCSB4: u8 = 0x0c;
pub const AK4114_REG_TXCSB0: u8 = 0x0d;
pub const AK4114_REG_TXCSB1: u8 = 0x0e;
pub const AK4114_REG_TXCSB2: u8 = 0x0f;
pub const AK4114_REG_TXCSB3: u8 = 0x10;
pub const AK4114_REG_TXCSB4: u8 = 0x11;
pub const AK4114_REG_Pc0: u8 = 0x12;
pub const AK4114_REG_Pc1: u8 = 0x13;
pub const AK4114_REG_Pd0: u8 = 0x14;
pub const AK4114_REG_Pd1: u8 = 0x15;
pub const AK4114_REG_QSUB_ADDR: u8 = 0x16;
pub const AK4114_REG_QSUB_TRACK: u8 = 0x17;
pub const AK4114_REG_QSUB_INDEX: u8 = 0x18;
pub const AK4114_REG_QSUB_MINUTE: u8 = 0x19;
pub const AK4114_REG_QSUB_SECOND: u8 = 0x1a;
pub const AK4114_REG_QSUB_FRAME: u8 = 0x1b;
pub const AK4114_REG_QSUB_ZERO: u8 = 0x1c;
pub const AK4114_REG_QSUB_ABSMIN: u8 = 0x1d;
pub const AK4114_REG_QSUB_ABSSEC: u8 = 0x1e;
pub const AK4114_REG_QSUB_ABSFRM: u8 = 0x1f;
pub const AK4114_REG_RXCSB_SIZE: usize = (AK4114_REG_RXCSB4 - AK4114_REG_RXCSB0 + 1) as usize;
pub const AK4114_REG_TXCSB_SIZE: usize = (AK4114_REG_TXCSB4 - AK4114_REG_TXCSB0 + 1) as usize;
pub const AK4114_REG_QSUB_SIZE: usize = (AK4114_REG_QSUB_ABSFRM - AK4114_REG_QSUB_ADDR + 1) as usize;

pub const AK4114_CS12: u8 = 1 << 7; pub const AK4114_BCU: u8 = 1 << 6;
pub const AK4114_CM1: u8 = 1 << 5; pub const AK4114_CM0: u8 = 1 << 4;
pub const AK4114_OCKS1: u8 = 1 << 3; pub const AK4114_OCKS0: u8 = 1 << 2;
pub const AK4114_PWN: u8 = 1 << 1; pub const AK4114_RST: u8 = 1 << 0;
pub const AK4114_MONO: u8 = 1 << 7; pub const AK4114_DIF2: u8 = 1 << 6;
pub const AK4114_DIF1: u8 = 1 << 5; pub const AK4114_DIF0: u8 = 1 << 4;
pub const AK4114_DIF_16R: u8 = 0; pub const AK4114_DIF_18R: u8 = AK4114_DIF0;
pub const AK4114_DIF_20R: u8 = AK4114_DIF1; pub const AK4114_DIF_24R: u8 = AK4114_DIF1 | AK4114_DIF0;
pub const AK4114_DIF_24L: u8 = AK4114_DIF2; pub const AK4114_DIF_24I2S: u8 = AK4114_DIF2 | AK4114_DIF0;
pub const AK4114_DIF_I24L: u8 = AK4114_DIF2 | AK4114_DIF1; pub const AK4114_DIF_I24I2S: u8 = AK4114_DIF2 | AK4114_DIF1 | AK4114_DIF0;
pub const AK4114_DEAU: u8 = 1 << 3; pub const AK4114_DEM1: u8 = 1 << 2; pub const AK4114_DEM0: u8 = 1 << 1;
pub const AK4114_DEM_44KHZ: u8 = 0; pub const AK4114_DEM_48KHZ: u8 = AK4114_DEM1;
pub const AK4114_DEM_32KHZ: u8 = AK4114_DEM0 | AK4114_DEM1; pub const AK4114_DEM_96KHZ: u8 = AK4114_DEM1; pub const AK4114_DFS: u8 = 1;

pub const AK4114_TX1E: u8 = 1 << 7; pub const AK4114_OPS12: u8 = 1 << 6; pub const AK4114_OPS11: u8 = 1 << 5; pub const AK4114_OPS10: u8 = 1 << 4;
pub const AK4114_TX0E: u8 = 1 << 3; pub const AK4114_OPS02: u8 = 1 << 2; pub const AK4114_OPS01: u8 = 1 << 1; pub const AK4114_OPS00: u8 = 1;
pub const AK4114_EFH1: u8 = 1 << 7; pub const AK4114_EFH0: u8 = 1 << 6; pub const AK4114_EFH_512: u8 = 0;
pub const AK4114_EFH_1024: u8 = AK4114_EFH0; pub const AK4114_EFH_2048: u8 = AK4114_EFH1; pub const AK4114_EFH_4096: u8 = AK4114_EFH1 | AK4114_EFH0;
pub const AK4114_UDIT: u8 = 1 << 5; pub const AK4114_TLR: u8 = 1 << 4; pub const AK4114_DIT: u8 = 1 << 3;
pub const AK4114_IPS2: u8 = 1 << 2; pub const AK4114_IPS1: u8 = 1 << 1; pub const AK4114_IPS0: u8 = 1;
#[inline] pub const fn AK4114_IPS(x: u8) -> u8 { x & 7 }

pub const AK4117_MQI: u8 = 1 << 7; pub const AK4117_MAT: u8 = 1 << 6; pub const AK4117_MCI: u8 = 1 << 5; pub const AK4117_MUL: u8 = 1 << 4;
pub const AK4117_MDTS: u8 = 1 << 3; pub const AK4117_MPE: u8 = 1 << 2; pub const AK4117_MAN: u8 = 1 << 1; pub const AK4117_MPR: u8 = 1;
pub const AK4114_QINT: u8 = 1 << 7; pub const AK4114_AUTO: u8 = 1 << 6; pub const AK4114_CINT: u8 = 1 << 5; pub const AK4114_UNLCK: u8 = 1 << 4;
pub const AK4114_DTSCD: u8 = 1 << 3; pub const AK4114_PEM: u8 = 1 << 2; pub const AK4114_AUDION: u8 = 1 << 1; pub const AK4114_PAR: u8 = 1;
pub const AK4114_FS3: u8 = 1 << 7; pub const AK4114_FS2: u8 = 1 << 6; pub const AK4114_FS1: u8 = 1 << 5; pub const AK4114_FS0: u8 = 1 << 4;
pub const AK4114_FS_44100HZ: u8 = 0; pub const AK4114_FS_48000HZ: u8 = AK4114_FS1; pub const AK4114_FS_32000HZ: u8 = AK4114_FS1 | AK4114_FS0;
pub const AK4114_FS_88200HZ: u8 = AK4114_FS3; pub const AK4114_FS_96000HZ: u8 = AK4114_FS3 | AK4114_FS1;
pub const AK4114_FS_176400HZ: u8 = AK4114_FS3 | AK4114_FS2; pub const AK4114_FS_192000HZ: u8 = AK4114_FS3 | AK4114_FS2 | AK4114_FS1;
pub const AK4114_V: u8 = 1 << 3; pub const AK4114_QCRC: u8 = 1 << 1; pub const AK4114_CCRC: u8 = 1;
pub const AK4114_CHECK_NO_STAT: u32 = 1; pub const AK4114_CHECK_NO_RATE: u32 = 1 << 1; pub const AK4114_CONTROLS: usize = 15;

pub type ak4114_write_t = unsafe extern "C" fn(*mut c_void, u8, u8);
pub type ak4114_read_t = unsafe extern "C" fn(*mut c_void, u8) -> u8;
pub const AK4114_PARITY_ERRORS: usize = 0; pub const AK4114_V_BIT_ERRORS: usize = 1;
pub const AK4114_QCRC_ERRORS: usize = 2; pub const AK4114_CCRC_ERRORS: usize = 3; pub const AK4114_NUM_ERRORS: usize = 4;

#[repr(C)] pub struct snd_card;
#[repr(C)] pub struct snd_kcontrol;
#[repr(C)] pub struct snd_pcm_substream;
#[repr(C)] pub struct atomic_t;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct delayed_work;

#[repr(C)] pub struct ak4114 {
    pub card: *mut snd_card, pub write: Option<ak4114_write_t>, pub read: Option<ak4114_read_t>, pub private_data: *mut c_void,
    pub wq_processing: atomic_t, pub reinit_mutex: mutex, pub lock: spinlock_t, pub regmap: [u8; 6], pub txcsb: [u8; 5],
    pub kctls: [*mut snd_kcontrol; AK4114_CONTROLS], pub playback_substream: *mut snd_pcm_substream, pub capture_substream: *mut snd_pcm_substream,
    pub errors: [core::ffi::c_ulong; AK4114_NUM_ERRORS], pub rcs0: u8, pub rcs1: u8, pub work: delayed_work, pub check_flags: u32,
    pub change_callback_private: *mut c_void,
    pub change_callback: Option<unsafe extern "C" fn(*mut ak4114, u8, u8)>,
}

extern "C" {
    pub fn snd_ak4114_create(card: *mut snd_card, read: Option<ak4114_read_t>, write: Option<ak4114_write_t>, pgm: *const u8, txcsb: *const u8, private_data: *mut c_void, r_ak4114: *mut *mut ak4114) -> i32;
    pub fn snd_ak4114_reg_write(ak4114: *mut ak4114, reg: u8, mask: u8, val: u8);
    pub fn snd_ak4114_reinit(ak4114: *mut ak4114);
    pub fn snd_ak4114_build(ak4114: *mut ak4114, playback_substream: *mut snd_pcm_substream, capture_substream: *mut snd_pcm_substream) -> i32;
    pub fn snd_ak4114_external_rate(ak4114: *mut ak4114) -> i32;
    pub fn snd_ak4114_check_rate_and_errors(ak4114: *mut ak4114, flags: u32) -> i32;
    pub fn snd_ak4114_suspend(chip: *mut ak4114);
    pub fn snd_ak4114_resume(chip: *mut ak4114);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
