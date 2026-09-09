/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Definitions for EMU10K1 (SB Live!) chips. */

pub const EMU10K1_FX8010_PCM_COUNT: usize = 8;

pub const iMAC0: u32 = 0x00; pub const iMAC1: u32 = 0x01; pub const iMAC2: u32 = 0x02; pub const iMAC3: u32 = 0x03;
pub const iMACINT0: u32 = 0x04; pub const iMACINT1: u32 = 0x05; pub const iACC3: u32 = 0x06; pub const iMACMV: u32 = 0x07;
pub const iANDXOR: u32 = 0x08; pub const iTSTNEG: u32 = 0x09; pub const iLIMITGE: u32 = 0x0a; pub const iLIMITLT: u32 = 0x0b;
pub const iLOG: u32 = 0x0c; pub const iEXP: u32 = 0x0d; pub const iINTERP: u32 = 0x0e; pub const iSKIP: u32 = 0x0f;
pub const LOWORD_OPX_MASK: u32=0x000ffc00; pub const LOWORD_OPY_MASK:u32=0x000003ff; pub const HIWORD_OPCODE_MASK:u32=0x00f00000; pub const HIWORD_RESULT_MASK:u32=0x000ffc00; pub const HIWORD_OPA_MASK:u32=0x000003ff;
pub const A_LOWORD_OPX_MASK:u32=0x007ff000; pub const A_LOWORD_OPY_MASK:u32=0x000007ff; pub const A_HIWORD_OPCODE_MASK:u32=0x0f000000; pub const A_HIWORD_RESULT_MASK:u32=0x007ff000; pub const A_HIWORD_OPA_MASK:u32=0x000007ff;

pub const fn FXBUS(x:u32)->u32{0x00+x} pub const fn EXTIN(x:u32)->u32{0x10+x} pub const fn EXTOUT(x:u32)->u32{0x20+x} pub const fn FXBUS2(x:u32)->u32{0x30+x}
pub const fn A_FXBUS(x:u32)->u32{x} pub const fn A_EXTIN(x:u32)->u32{0x40+x} pub const fn A_P16VIN(x:u32)->u32{0x50+x} pub const fn A_EXTOUT(x:u32)->u32{0x60+x} pub const fn A_FXBUS2(x:u32)->u32{0x80+x} pub const fn A_EMU32OUTH(x:u32)->u32{0xa0+x} pub const fn A_EMU32OUTL(x:u32)->u32{0xb0+x} pub const fn A3_EMU32IN(x:u32)->u32{0x160+x} pub const fn A3_EMU32OUT(x:u32)->u32{0x1e0+x}

macro_rules! cseq { ($($n:ident=$v:expr),* $(,)?) => { $(pub const $n:u32=$v;)* }; }
cseq! { C_00000000=0x40,C_00000001=0x41,C_00000002=0x42,C_00000003=0x43,C_00000004=0x44,C_00000008=0x45,C_00000010=0x46,C_00000020=0x47,C_00000100=0x48,C_00010000=0x49,C_00080000=0x4a,C_10000000=0x4b,C_20000000=0x4c,C_40000000=0x4d,C_80000000=0x4e,C_7fffffff=0x4f,C_ffffffff=0x50,C_fffffffe=0x51,C_c0000000=0x52,C_4f1bbcdc=0x53,C_5a7ef9db=0x54,C_00100000=0x55,GPR_ACCU=0x56,GPR_COND=0x57,GPR_NOISE0=0x58,GPR_NOISE1=0x59,GPR_IRQ=0x5a,GPR_DBAC=0x5b }
cseq! { A_C_00000000=0xc0,A_C_00000001=0xc1,A_C_00000002=0xc2,A_C_00000003=0xc3,A_C_00000004=0xc4,A_C_00000008=0xc5,A_C_00000010=0xc6,A_C_00000020=0xc7,A_C_00000100=0xc8,A_C_00010000=0xc9,A_C_00000800=0xca,A_C_10000000=0xcb,A_C_20000000=0xcc,A_C_40000000=0xcd,A_C_80000000=0xce,A_C_7fffffff=0xcf,A_C_ffffffff=0xd0,A_C_fffffffe=0xd1,A_C_c0000000=0xd2,A_C_4f1bbcdc=0xd3,A_C_5a7ef9db=0xd4,A_C_00100000=0xd5,A_GPR_ACCU=0xd6,A_GPR_COND=0xd7,A_GPR_NOISE0=0xd8,A_GPR_NOISE1=0xd9,A_GPR_IRQ=0xda,A_GPR_DBAC=0xdb,A_GPR_DBACE=0xde }
pub const FXGPREGBASE:u32=0x100; pub const A_FXGPREGBASE:u32=0x400; pub const A_TANKMEMCTLREGBASE:u32=0x100; pub const A_TANKMEMCTLREG_MASK:u32=0x1f; pub const TANKMEMDATAREGBASE:u32=0x200; pub const TANKMEMDATAREG_MASK:u32=0x000fffff; pub const TANKMEMADDRREGBASE:u32=0x300; pub const TANKMEMADDRREG_ADDR_MASK:u32=0x000fffff; pub const TANKMEMADDRREG_CLEAR:u32=0x00800000; pub const TANKMEMADDRREG_ALIGN:u32=0x00400000; pub const TANKMEMADDRREG_WRITE:u32=0x00200000; pub const TANKMEMADDRREG_READ:u32=0x00100000;
pub const fn GPR(x:u32)->u32{FXGPREGBASE+x} pub const fn ITRAM_DATA(x:u32)->u32{TANKMEMDATAREGBASE+x} pub const fn ETRAM_DATA(x:u32)->u32{TANKMEMDATAREGBASE+0x80+x} pub const fn ITRAM_ADDR(x:u32)->u32{TANKMEMADDRREGBASE+x} pub const fn ETRAM_ADDR(x:u32)->u32{TANKMEMADDRREGBASE+0x80+x} pub const fn A_GPR(x:u32)->u32{A_FXGPREGBASE+x} pub const fn A_ITRAM_DATA(x:u32)->u32{TANKMEMDATAREGBASE+x} pub const fn A_ETRAM_DATA(x:u32)->u32{TANKMEMDATAREGBASE+0xc0+x} pub const fn A_ITRAM_ADDR(x:u32)->u32{TANKMEMADDRREGBASE+x} pub const fn A_ETRAM_ADDR(x:u32)->u32{TANKMEMADDRREGBASE+0xc0+x} pub const fn A_ITRAM_CTL(x:u32)->u32{A_TANKMEMCTLREGBASE+x} pub const fn A_ETRAM_CTL(x:u32)->u32{A_TANKMEMCTLREGBASE+0xc0+x}

pub const CC_REG_NORMALIZED:u32=C_00000001; pub const CC_REG_BORROW:u32=C_00000002; pub const CC_REG_MINUS:u32=C_00000004; pub const CC_REG_ZERO:u32=C_00000008; pub const CC_REG_SATURATE:u32=C_00000010; pub const CC_REG_NONZERO:u32=C_00000100;
pub const A_CC_REG_NORMALIZED:u32=A_C_00000001; pub const A_CC_REG_BORROW:u32=A_C_00000002; pub const A_CC_REG_MINUS:u32=A_C_00000004; pub const A_CC_REG_ZERO:u32=A_C_00000008; pub const A_CC_REG_SATURATE:u32=A_C_00000010; pub const A_CC_REG_NONZERO:u32=A_C_00000100;

macro_rules! vals { ($($n:ident=$v:expr),* $(,)?) => { $(pub const $n:u32=$v;)* }; }
vals! { FXBUS_PCM_LEFT=0,FXBUS_PCM_RIGHT=1,FXBUS_PCM_LEFT_REAR=2,FXBUS_PCM_RIGHT_REAR=3,FXBUS_MIDI_LEFT=4,FXBUS_MIDI_RIGHT=5,FXBUS_PCM_CENTER=6,FXBUS_PCM_LFE=7,FXBUS_PCM_LEFT_FRONT=8,FXBUS_PCM_RIGHT_FRONT=9,FXBUS_MIDI_REVERB=0xc,FXBUS_MIDI_CHORUS=0xd,FXBUS_PCM_LEFT_SIDE=0xe,FXBUS_PCM_RIGHT_SIDE=0xf,FXBUS_PT_LEFT=0x14,FXBUS_PT_RIGHT=0x15,
EXTIN_AC97_L=0,EXTIN_AC97_R=1,EXTIN_SPDIF_CD_L=2,EXTIN_SPDIF_CD_R=3,EXTIN_ZOOM_L=4,EXTIN_ZOOM_R=5,EXTIN_TOSLINK_L=6,EXTIN_TOSLINK_R=7,EXTIN_LINE1_L=8,EXTIN_LINE1_R=9,EXTIN_COAX_SPDIF_L=0xa,EXTIN_COAX_SPDIF_R=0xb,EXTIN_LINE2_L=0xc,EXTIN_LINE2_R=0xd,
EXTOUT_AC97_L=0,EXTOUT_AC97_R=1,EXTOUT_TOSLINK_L=2,EXTOUT_TOSLINK_R=3,EXTOUT_AC97_CENTER=4,EXTOUT_AC97_LFE=5,EXTOUT_HEADPHONE_L=6,EXTOUT_HEADPHONE_R=7,EXTOUT_REAR_L=8,EXTOUT_REAR_R=9,EXTOUT_ADC_CAP_L=0xa,EXTOUT_ADC_CAP_R=0xb,EXTOUT_MIC_CAP=0xc,EXTOUT_AC97_REAR_L=0xd,EXTOUT_AC97_REAR_R=0xe,EXTOUT_ACENTER=0x11,EXTOUT_ALFE=0x12 }
vals! { A_EXTIN_AC97_L=0,A_EXTIN_AC97_R=1,A_EXTIN_SPDIF_CD_L=2,A_EXTIN_SPDIF_CD_R=3,A_EXTIN_OPT_SPDIF_L=4,A_EXTIN_OPT_SPDIF_R=5,A_EXTIN_LINE2_L=8,A_EXTIN_LINE2_R=9,A_EXTIN_ADC_L=0xa,A_EXTIN_ADC_R=0xb,A_EXTIN_AUX2_L=0xc,A_EXTIN_AUX2_R=0xd,A_EXTOUT_FRONT_L=0,A_EXTOUT_FRONT_R=1,A_EXTOUT_CENTER=2,A_EXTOUT_LFE=3,A_EXTOUT_HEADPHONE_L=4,A_EXTOUT_HEADPHONE_R=5,A_EXTOUT_REAR_L=6,A_EXTOUT_REAR_R=7,A_EXTOUT_AFRONT_L=8,A_EXTOUT_AFRONT_R=9,A_EXTOUT_ACENTER=0xa,A_EXTOUT_ALFE=0xb,A_EXTOUT_ASIDE_L=0xc,A_EXTOUT_ASIDE_R=0xd,A_EXTOUT_AREAR_L=0xe,A_EXTOUT_AREAR_R=0xf,A_EXTOUT_AC97_L=0x10,A_EXTOUT_AC97_R=0x11,A_EXTOUT_ADC_CAP_L=0x16,A_EXTOUT_ADC_CAP_R=0x17,A_EXTOUT_MIC_CAP=0x18 }

pub const EMU10K1_DBG_ZC:u32=0x80000000; pub const EMU10K1_DBG_SATURATION_OCCURED:u32=0x02000000; pub const EMU10K1_DBG_SATURATION_ADDR:u32=0x01ff0000; pub const EMU10K1_DBG_SINGLE_STEP:u32=0x00008000; pub const EMU10K1_DBG_STEP:u32=0x00004000; pub const EMU10K1_DBG_CONDITION_CODE:u32=0x00003e00; pub const EMU10K1_DBG_SINGLE_STEP_ADDR:u32=0x1ff;
pub const A_DBG_ZC:u32=0x40000000; pub const A_DBG_SATURATION_OCCURED:u32=0x20000000; pub const A_DBG_SATURATION_ADDR:u32=0x0ffc0000; pub const A_DBG_SINGLE_STEP:u32=0x00020000; pub const A_DBG_STEP:u32=0x00010000; pub const A_DBG_CONDITION_CODE:u32=0x0000f800; pub const A_DBG_STEP_ADDR:u32=0x3ff;

#[repr(C)] pub struct snd_emu10k1_fx8010_info { pub internal_tram_size:u32,pub external_tram_size:u32,pub fxbus_names:[[i8;32];16],pub extin_names:[[i8;32];16],pub extout_names:[[i8;32];32],pub gpr_controls:u32 }
pub const EMU10K1_GPR_TRANSLATION_NONE:u32=0; pub const EMU10K1_GPR_TRANSLATION_TABLE100:u32=1; pub const EMU10K1_GPR_TRANSLATION_BASS:u32=2; pub const EMU10K1_GPR_TRANSLATION_TREBLE:u32=3; pub const EMU10K1_GPR_TRANSLATION_ONOFF:u32=4; pub const EMU10K1_GPR_TRANSLATION_NEGATE:u32=5; pub const EMU10K1_GPR_TRANSLATION_NEG_TABLE100:u32=6;
#[repr(C)] pub struct emu10k1_ctl_elem_id { pub pad:u32,pub iface:i32,pub device:u32,pub subdevice:u32,pub name:[u8;44],pub index:u32 }
#[repr(C)] pub struct snd_emu10k1_fx8010_control_gpr { pub id:emu10k1_ctl_elem_id,pub vcount:u32,pub count:u32,pub gpr:[u16;32],pub value:[i32;32],pub min:i32,pub max:i32,pub translation:u32,pub tlv:*const u32 }
#[repr(C)] pub struct snd_emu10k1_fx8010_control_old_gpr { pub id:emu10k1_ctl_elem_id,pub vcount:u32,pub count:u32,pub gpr:[u16;32],pub value:[u32;32],pub min:u32,pub max:u32,pub translation:u32 }
#[repr(C)] pub struct snd_emu10k1_fx8010_code { pub name:[i8;128],pub gpr_valid:[usize;4],pub gpr_map:*mut u32,pub gpr_add_control_count:u32,pub gpr_add_controls:*mut snd_emu10k1_fx8010_control_gpr,pub gpr_del_control_count:u32,pub gpr_del_controls:*mut emu10k1_ctl_elem_id,pub gpr_list_control_count:u32,pub gpr_list_control_total:u32,pub gpr_list_controls:*mut snd_emu10k1_fx8010_control_gpr,pub tram_valid:[usize;2],pub tram_data_map:*mut u32,pub tram_addr_map:*mut u32,pub code_valid:[usize;16],pub code:*mut u32 }
#[repr(C)] pub struct snd_emu10k1_fx8010_tram { pub address:u32,pub size:u32,pub samples:*mut u32 }
#[repr(C)] pub struct snd_emu10k1_fx8010_pcm_rec { pub substream:u32,pub res1:u32,pub channels:u32,pub tram_start:u32,pub buffer_size:u32,pub gpr_size:u16,pub gpr_ptr:u16,pub gpr_count:u16,pub gpr_tmpcount:u16,pub gpr_trigger:u16,pub gpr_running:u16,pub pad:u8,pub etram:[u8;32],pub res2:u32 }

#[repr(i32)] pub enum emu10k1_ctl_elem_iface { EMU10K1_CTL_ELEM_IFACE_MIXER=2, EMU10K1_CTL_ELEM_IFACE_PCM=3 }
/* SNDRV_PROTOCOL_VERSION and Linux ioctl encodings are supplied by the surrounding UAPI bindings. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
