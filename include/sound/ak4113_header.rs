/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of the AK4113 header. */

pub const AK4113_REG_PWRDN: u8 = 0x00;
pub const AK4113_REG_FORMAT: u8 = 0x01;
pub const AK4113_REG_IO0: u8 = 0x02;
pub const AK4113_REG_IO1: u8 = 0x03;
pub const AK4113_REG_INT0_MASK: u8 = 0x04;
pub const AK4113_REG_INT1_MASK: u8 = 0x05;
pub const AK4113_REG_DATDTS: u8 = 0x06;
pub const AK4113_REG_RCS0: u8 = 0x07;
pub const AK4113_REG_RCS1: u8 = 0x08;
pub const AK4113_REG_RCS2: u8 = 0x09;
pub const AK4113_REG_RXCSB0: u8 = 0x0a;
pub const AK4113_REG_RXCSB1: u8 = 0x0b;
pub const AK4113_REG_RXCSB2: u8 = 0x0c;
pub const AK4113_REG_RXCSB3: u8 = 0x0d;
pub const AK4113_REG_RXCSB4: u8 = 0x0e;
pub const AK4113_REG_Pc0: u8 = 0x0f;
pub const AK4113_REG_Pc1: u8 = 0x10;
pub const AK4113_REG_Pd0: u8 = 0x11;
pub const AK4113_REG_Pd1: u8 = 0x12;
pub const AK4113_REG_QSUB_ADDR: u8 = 0x13;
pub const AK4113_REG_QSUB_TRACK: u8 = 0x14;
pub const AK4113_REG_QSUB_INDEX: u8 = 0x15;
pub const AK4113_REG_QSUB_MINUTE: u8 = 0x16;
pub const AK4113_REG_QSUB_SECOND: u8 = 0x17;
pub const AK4113_REG_QSUB_FRAME: u8 = 0x18;
pub const AK4113_REG_QSUB_ZERO: u8 = 0x19;
pub const AK4113_REG_QSUB_ABSMIN: u8 = 0x1a;
pub const AK4113_REG_QSUB_ABSSEC: u8 = 0x1b;
pub const AK4113_REG_QSUB_ABSFRM: u8 = 0x1c;
pub const AK4113_REG_RXCSB_SIZE: u8 = AK4113_REG_RXCSB4 - AK4113_REG_RXCSB0 + 1;
pub const AK4113_REG_QSUB_SIZE: u8 = AK4113_REG_QSUB_ABSFRM - AK4113_REG_QSUB_ADDR + 1;
pub const AK4113_WRITABLE_REGS: u8 = AK4113_REG_DATDTS + 1;

pub const AK4113_CS12: u8 = 1 << 7; pub const AK4113_BCU: u8 = 1 << 6;
pub const AK4113_CM1: u8 = 1 << 5; pub const AK4113_CM0: u8 = 1 << 4;
pub const AK4113_OCKS1: u8 = 1 << 3; pub const AK4113_OCKS0: u8 = 1 << 2;
pub const AK4113_PWN: u8 = 1 << 1; pub const AK4113_RST: u8 = 1;
pub const AK4113_VTX: u8 = 1 << 7; pub const AK4113_DIF2: u8 = 1 << 6;
pub const AK4113_DIF1: u8 = 1 << 5; pub const AK4113_DIF0: u8 = 1 << 4;
pub const AK4113_DEAU: u8 = 1 << 3; pub const AK4113_DEM1: u8 = 1 << 2; pub const AK4113_DEM0: u8 = 1 << 1;
pub const AK4113_DEM_OFF: u8 = AK4113_DEM0; pub const AK4113_DEM_44KHZ: u8 = 0; pub const AK4113_DEM_48KHZ: u8 = AK4113_DEM1; pub const AK4113_DEM_32KHZ: u8 = AK4113_DEM0 | AK4113_DEM1;
pub const AK4113_DIF_16R: u8 = 0; pub const AK4113_DIF_18R: u8 = AK4113_DIF0; pub const AK4113_DIF_20R: u8 = AK4113_DIF1; pub const AK4113_DIF_24R: u8 = AK4113_DIF1 | AK4113_DIF0; pub const AK4113_DIF_24L: u8 = AK4113_DIF2; pub const AK4113_DIF_24I2S: u8 = AK4113_DIF2 | AK4113_DIF0; pub const AK4113_DIF_I24L: u8 = AK4113_DIF2 | AK4113_DIF1; pub const AK4113_DIF_I24I2S: u8 = AK4113_DIF2 | AK4113_DIF1 | AK4113_DIF0;

pub const AK4113_XTL1: u8 = 1 << 6; pub const AK4113_XTL0: u8 = 1 << 5; pub const AK4113_UCE: u8 = 1 << 4; pub const AK4113_TXE: u8 = 1 << 3; pub const AK4113_OPS2: u8 = 1 << 2; pub const AK4113_OPS1: u8 = 1 << 1; pub const AK4113_OPS0: u8 = 1;
pub const AK4113_XTL_11_2896M: u8 = 0; pub const AK4113_XTL_12_288M: u8 = AK4113_XTL0; pub const AK4113_XTL_24_576M: u8 = AK4113_XTL1;
pub const AK4113_EFH1: u8 = 1 << 7; pub const AK4113_EFH0: u8 = 1 << 6; pub const AK4113_EFH_512LRCLK: u8 = 0; pub const AK4113_EFH_1024LRCLK: u8 = AK4113_EFH0; pub const AK4113_EFH_2048LRCLK: u8 = AK4113_EFH1; pub const AK4113_EFH_4096LRCLK: u8 = AK4113_EFH1 | AK4113_EFH0;
pub const AK4113_FAST: u8 = 1 << 5; pub const AK4113_XMCK: u8 = 1 << 4; pub const AK4113_DIV: u8 = 1 << 3; pub const AK4113_IPS2: u8 = 1 << 2; pub const AK4113_IPS1: u8 = 1 << 1; pub const AK4113_IPS0: u8 = 1;
#[inline] pub const fn AK4113_IPS(x: u8) -> u8 { x & 7 }
pub const AK4113_MQI: u8 = 1 << 7; pub const AK4113_MAUT: u8 = 1 << 6; pub const AK4113_MCIT: u8 = 1 << 5; pub const AK4113_MULK: u8 = 1 << 4; pub const AK4113_V: u8 = 1 << 3; pub const AK4113_STC: u8 = 1 << 2; pub const AK4113_MAN: u8 = 1 << 1; pub const AK4113_MPR: u8 = 1;
pub const AK4113_DCNT: u8 = 1 << 4; pub const AK4113_DTS16: u8 = 1 << 3; pub const AK4113_DTS14: u8 = 1 << 2; pub const AK4113_MDAT1: u8 = 1 << 1; pub const AK4113_MDAT0: u8 = 1;
pub const AK4113_QINT: u8 = 1 << 7; pub const AK4113_AUTO: u8 = 1 << 6; pub const AK4113_CINT: u8 = 1 << 5; pub const AK4113_UNLCK: u8 = 1 << 4; pub const AK4113_AUDION: u8 = 1 << 1; pub const AK4113_PAR: u8 = 1;
pub const AK4113_FS3: u8 = 1 << 7; pub const AK4113_FS2: u8 = 1 << 6; pub const AK4113_FS1: u8 = 1 << 5; pub const AK4113_FS0: u8 = 1 << 4; pub const AK4113_PEM: u8 = 1 << 3; pub const AK4113_DAT: u8 = 1 << 2; pub const AK4113_DTSCD: u8 = 1 << 1; pub const AK4113_NPCM: u8 = 1;
pub const AK4113_FS_8000HZ:u8=AK4113_FS3|AK4113_FS0; pub const AK4113_FS_11025HZ:u8=AK4113_FS2|AK4113_FS0; pub const AK4113_FS_16000HZ:u8=AK4113_FS2|AK4113_FS1|AK4113_FS0; pub const AK4113_FS_22050HZ:u8=AK4113_FS2; pub const AK4113_FS_24000HZ:u8=AK4113_FS2|AK4113_FS1; pub const AK4113_FS_32000HZ:u8=AK4113_FS1|AK4113_FS0; pub const AK4113_FS_44100HZ:u8=0; pub const AK4113_FS_48000HZ:u8=AK4113_FS1; pub const AK4113_FS_64000HZ:u8=AK4113_FS3|AK4113_FS1|AK4113_FS0; pub const AK4113_FS_88200HZ:u8=AK4113_FS3; pub const AK4113_FS_96000HZ:u8=AK4113_FS3|AK4113_FS1; pub const AK4113_FS_176400HZ:u8=AK4113_FS3|AK4113_FS2; pub const AK4113_FS_192000HZ:u8=AK4113_FS3|AK4113_FS2|AK4113_FS1;
pub const AK4113_QCRC:u8=1<<1; pub const AK4113_CCRC:u8=1; pub const AK4113_CHECK_NO_STAT:u32=1; pub const AK4113_CHECK_NO_RATE:u32=1<<1; pub const AK4113_CONTROLS:usize=13;

pub type Ak4113Write = unsafe extern "C" fn(*mut core::ffi::c_void, u8, u8);
pub type Ak4113Read = unsafe extern "C" fn(*mut core::ffi::c_void, u8) -> u8;

#[repr(usize)] pub enum Ak4113Error { ParityErrors, VBitErrors, QcrcErrors, CcrcErrors, NumErrors }

#[repr(C)] pub struct ak4113 {
    pub card: *mut snd_card, pub write: Option<Ak4113Write>, pub read: Option<Ak4113Read>, pub private_data: *mut core::ffi::c_void,
    pub wq_processing: atomic_t, pub reinit_mutex: mutex, pub lock: spinlock_t,
    pub regmap: [u8; AK4113_WRITABLE_REGS as usize], pub kctls: [*mut snd_kcontrol; AK4113_CONTROLS], pub substream: *mut snd_pcm_substream,
    pub errors: [c_ulong; Ak4113Error::NumErrors as usize], pub rcs0:u8, pub rcs1:u8, pub rcs2:u8, pub work: delayed_work, pub check_flags:u32,
    pub change_callback_private:*mut core::ffi::c_void, pub change_callback: Option<unsafe extern "C" fn(*mut ak4113,u8,u8)>,
}

extern "C" { pub fn snd_ak4113_create(card:*mut snd_card, read:Option<Ak4113Read>, write:Option<Ak4113Write>, pgm:*const u8, private_data:*mut core::ffi::c_void, r_ak4113:*mut *mut ak4113)->i32; pub fn snd_ak4113_reg_write(ak4113:*mut ak4113, reg:u8, mask:u8, val:u8); pub fn snd_ak4113_reinit(ak4113:*mut ak4113); pub fn snd_ak4113_build(ak4113:*mut ak4113, capture_substream:*mut snd_pcm_substream)->i32; pub fn snd_ak4113_external_rate(ak4113:*mut ak4113)->i32; pub fn snd_ak4113_check_rate_and_errors(ak4113:*mut ak4113, flags:u32)->i32; }
// These declarations are conditional on CONFIG_PM in the C header. When PM
// is disabled, the corresponding inline functions are no-ops.
extern "C" { pub fn snd_ak4113_suspend(chip:*mut ak4113); pub fn snd_ak4113_resume(chip:*mut ak4113); }

// External kernel types supplied by the including environment.
pub enum snd_card {} pub enum snd_kcontrol {} pub enum snd_pcm_substream {} pub enum atomic_t {} pub enum mutex {} pub enum spinlock_t {} pub enum delayed_work {}
pub type c_ulong = usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
