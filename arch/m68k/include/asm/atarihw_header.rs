/* Rust translation of linux/atarihw.h. */

extern "C" {
    pub static mut atari_mch_cookie: u_long;
    pub static mut atari_mch_type: u_long;
    pub static mut atari_switches: u_long;
    pub static mut atari_rtc_year_offset: c_int;
    pub static mut atari_dont_touch_floppy_select: c_int;
    pub static mut atari_SCC_reset_done: c_int;
    pub fn atari_nvram_read(p: *mut c_char, n: size_t, off: *mut loff_t) -> ssize_t;
    pub fn atari_nvram_write(p: *mut c_char, n: size_t, off: *mut loff_t) -> ssize_t;
    pub fn atari_nvram_get_size() -> ssize_t;
    pub fn atari_nvram_set_checksum() -> c_long;
    pub fn atari_nvram_initialize() -> c_long;
    pub static mut atari_hw_present: atari_hw_present;
}

pub const ATARI_SWITCH_IKBD: u_long = 0x01;
pub const ATARI_SWITCH_MIDI: u_long = 0x02;
pub const ATARI_SWITCH_SND6: u_long = 0x04;
pub const ATARI_SWITCH_SND7: u_long = 0x08;
pub const ATARI_SWITCH_OVSC_SHIFT: u32 = 16;
pub const ATARI_SWITCH_OVSC_IKBD: u_long = ATARI_SWITCH_IKBD << ATARI_SWITCH_OVSC_SHIFT;
pub const ATARI_SWITCH_OVSC_MIDI: u_long = ATARI_SWITCH_MIDI << ATARI_SWITCH_OVSC_SHIFT;
pub const ATARI_SWITCH_OVSC_SND6: u_long = ATARI_SWITCH_SND6 << ATARI_SWITCH_OVSC_SHIFT;
pub const ATARI_SWITCH_OVSC_SND7: u_long = ATARI_SWITCH_SND7 << ATARI_SWITCH_OVSC_SHIFT;
pub const ATARI_SWITCH_OVSC_MASK: u_long = 0xffff0000;

#[inline] pub unsafe fn mach_is_st() -> bool { (atari_mch_cookie >> 16) == ATARI_MCH_ST }
#[inline] pub unsafe fn mach_is_ste() -> bool { (atari_mch_cookie >> 16) == ATARI_MCH_STE && atari_mch_cookie & 0xffff == 0 }
#[inline] pub unsafe fn mach_is_mste() -> bool { (atari_mch_cookie >> 16) == ATARI_MCH_STE && atari_mch_cookie & 0xffff == 0x10 }
#[inline] pub unsafe fn mach_is_tt() -> bool { (atari_mch_cookie >> 16) == ATARI_MCH_TT }
#[inline] pub unsafe fn mach_is_falcon() -> bool { (atari_mch_cookie >> 16) == ATARI_MCH_FALCON }
#[inline] pub unsafe fn mach_is_medusa() -> bool { atari_mch_type == ATARI_MACH_MEDUSA }
#[inline] pub unsafe fn mach_is_ab40() -> bool { atari_mch_type == ATARI_MACH_AB40 }

#[repr(C)] pub struct atari_hw_present { pub bits: [u8; 32] }
#[inline] pub unsafe fn atarihw_set(_name: usize) { /* C bit-field assignment; field-specific use is dependency-defined. */ }

#[repr(C)] pub struct SHIFTER_ST { pub pad1:u8,pub bas_hi:u8,pub pad2:u8,pub bas_md:u8,pub pad3:u8,pub vcounthi:u8,pub pad4:u8,pub vcountmid:u8,pub pad5:u8,pub vcountlow:u8,pub syncmode:u8,pub pad6:u8,pub pad7:u8,pub bas_lo:u8 }
#[repr(C)] pub struct SHIFTER_F030 { pub off_next:u16,pub scn_width:u16 }
#[repr(C)] pub struct SHIFTER_TT { pub char_dummy0:u8,pub bas_hi:u8,pub char_dummy1:u8,pub bas_md:u8,pub char_dummy2:u8,pub vcount_hi:u8,pub char_dummy3:u8,pub vcount_md:u8,pub char_dummy4:u8,pub vcount_lo:u8,pub st_sync:u16,pub char_dummy5:u8,pub bas_lo:u8,pub char_dummy6:[u8;50],pub color_reg:[u16;16],pub st_shiftmode:u8,pub char_dummy7:u8,pub tt_shiftmode:u16 }
#[repr(C)] pub struct VIDEL { pub st_shift:u16,pub pad1:u16,pub xoffset_s:u8,pub xoffset:u8,pub f_shift:u16,pub pad2:[u8;0x1a],pub hht:u16,pub hbb:u16,pub hbe:u16,pub hdb:u16,pub hde:u16,pub hss:u16,pub pad3:[u8;0x14],pub vft:u16,pub vbb:u16,pub vbe:u16,pub vdb:u16,pub vde:u16,pub vss:u16,pub pad4:[u8;0x12],pub control:u16,pub mode:u16 }
#[repr(C)] pub struct DMA_WD { pub fdc_acces_seccount:u16,pub dma_mode_status:u16,pub dma_vhi:u8,pub dma_hi:u8,pub char_dummy2:u8,pub dma_md:u8,pub char_dummy3:u8,pub dma_lo:u8,pub fdc_speed:u16 }
#[repr(C)] pub struct SOUND_YM { pub rd_data_reg_sel:u8,pub char_dummy1:u8,pub wd_data:u8 }
#[repr(C)] pub struct TT_DMA { pub char_dummy0:u8,pub dma_addr_hi:u8,pub char_dummy1:u8,pub dma_addr_hmd:u8,pub char_dummy2:u8,pub dma_addr_lmd:u8,pub char_dummy3:u8,pub dma_addr_lo:u8,pub char_dummy4:u8,pub dma_cnt_hi:u8,pub char_dummy5:u8,pub dma_cnt_hmd:u8,pub char_dummy6:u8,pub dma_cnt_lmd:u8,pub char_dummy7:u8,pub dma_cnt_lo:u8,pub dma_restdata:u_long,pub dma_ctrl:u16 }
#[repr(C)] pub struct TT_5380 { pub scsi_data:u8,pub char_dummy1:u8,pub scsi_icr:u8,pub char_dummy2:u8,pub scsi_mode:u8,pub char_dummy3:u8,pub scsi_tcr:u8,pub char_dummy4:u8,pub scsi_idstat:u8,pub char_dummy5:u8,pub scsi_dmastat:u8,pub char_dummy6:u8,pub scsi_targrcv:u8,pub char_dummy7:u8,pub scsi_inircv:u8 }
#[repr(C)] pub struct MATRIX { pub source:u16,pub destination:u16,pub external_frequency_divider:u8,pub internal_frequency_divider:u8 }
#[repr(C)] pub struct CODEC { pub tracks:u8,pub input_source:u8,pub adc_source:u8,pub gain:u8,pub attenuation:u8,pub unused1:u8,pub status:u8,pub unused2:u8,pub unused3:u8,pub unused4:u8,pub unused5:u8,pub gpio_directions:u8,pub unused6:u8,pub gpio_data:u8 }
#[repr(C)] pub struct BLITTER { pub halftone:[u16;16],pub src_x_inc:u16,pub src_y_inc:u16,pub src_address:u_long,pub endmask1:u16,pub endmask2:u16,pub endmask3:u16,pub dst_x_inc:u16,pub dst_y_inc:u16,pub dst_address:u_long,pub wd_per_line:u16,pub ln_per_bb:u16,pub hlf_op_reg:u16,pub log_op_reg:u16,pub lin_nm_reg:u16,pub skew_reg:u16 }
#[repr(C)] pub struct SCC { pub cha_a_ctrl:u8,pub char_dummy1:u8,pub cha_a_data:u8,pub char_dummy2:u8,pub cha_b_ctrl:u8,pub char_dummy3:u8,pub cha_b_data:u8 }
#[repr(C)] pub struct VIDEL_PALETTE { pub reg:[u_long;256] }
#[repr(C)] pub union DSPData { pub b:[u8;4],pub w:[u16;2],pub l:u_long }
#[repr(C)] pub struct DSP56K_HOST_INTERFACE { pub icr:u8,pub cvr:u8,pub isr:u8,pub ivr:u8,pub data:DSPData }
#[repr(C)] pub struct MFP { pub regs:[u8;45] }
#[repr(C)] pub struct TT_SCU { pub sys_mask:u8,pub char_dummy1:u8,pub sys_stat:u8,pub char_dummy2:u8,pub softint:u8,pub char_dummy3:u8,pub vmeint:u8,pub char_dummy4:u8,pub gp_reg1:u8,pub char_dummy5:u8,pub gp_reg2:u8,pub char_dummy6:u8,pub vme_mask:u8,pub char_dummy7:u8,pub vme_stat:u8 }
#[repr(C)] pub struct TT_RTC { pub regsel:u8,pub dummy:u8,pub data:u8 }
#[repr(C)] pub struct ACIA { pub key_ctrl:u8,pub char_dummy1:u8,pub key_data:u8,pub char_dummy2:u8,pub mid_ctrl:u8,pub char_dummy3:u8,pub mid_data:u8 }
#[repr(C)] pub struct TT_DMASND { pub bytes:[u8;52],pub cbar_src:u16,pub cbar_dst:u16,pub ext_div:u8,pub int_div:u8,pub rec_track_select:u8,pub dac_src:u8,pub adc_src:u8,pub input_gain:u8,pub output_atten:u16 }
#[repr(C)] pub struct TT_MICROWIRE { pub data:u16,pub mask:u16 }
#[repr(C)] pub struct MSTE_RTC { pub bytes:[u8;31] }

pub const ST_LOW:u32=0; pub const ST_MID:u32=1; pub const ST_HIGH:u32=2; pub const TT_LOW:u32=7; pub const TT_MID:u32=4; pub const TT_HIGH:u32=6;
pub const SHF_BAS:usize=0xffff8200; pub const SHF_FBAS:usize=0xffff820e; pub const SHF_TBAS:usize=0xffff8200; pub const VIDEL_BAS:usize=0xffff8260; pub const FWD_BAS:usize=0xffff8604; pub const YM_BAS:usize=0xffff8800; pub const TT_SCSI_DMA_BAS:usize=0xffff8700; pub const TT_5380_BAS:usize=0xffff8781; pub const MATRIX_BASE:usize=0xffff8930; pub const CODEC_BASE:usize=0xffff8936; pub const BLT_BAS:usize=0xffff8a00; pub const SCC_BAS:usize=0xffff8c81; pub const FPL_BAS:usize=0xffff9800; pub const DSP56K_HOST_INTERFACE_BASE:usize=0xffffa200; pub const MFP_BAS:usize=0xfffffa01; pub const TT_MFP_BAS:usize=0xfffffa81; pub const TT_SCU_BAS:usize=0xffff8e01; pub const TT_RTC_BAS:usize=0xffff8961; pub const ACIA_BAS:usize=0xfffffc00; pub const TT_DMASND_BAS:usize=0xffff8900; pub const TT_MICROWIRE_BAS:usize=0xffff8922; pub const MSTE_RTC_BAS:usize=0xfffffc21;
pub const TT_SHIFTER_STLOW:u16=0; pub const TT_SHIFTER_STMID:u16=0x100; pub const TT_SHIFTER_STHIGH:u16=0x200; pub const TT_SHIFTER_TTLOW:u16=0x700; pub const TT_SHIFTER_TTMID:u16=0x400; pub const TT_SHIFTER_TTHIGH:u16=0x600; pub const TT_SHIFTER_MODEMASK:u16=0x700; pub const TT_SHIFTER_NUMMODE:u16=8; pub const TT_SHIFTER_PALETTE_MASK:u16=0xf; pub const TT_SHIFTER_GRAYMODE:u16=0x1000;
pub const CODEC_SOURCE_ADC:u8=1; pub const CODEC_SOURCE_MATRIX:u8=2; pub const ADC_SOURCE_RIGHT_PSG:u8=1; pub const ADC_SOURCE_LEFT_PSG:u8=2; pub const CODEC_GAIN_RIGHT:u8=0xf; pub const CODEC_GAIN_LEFT:u8=0xf0; pub const CODEC_ATTENUATION_RIGHT:u8=0xf; pub const CODEC_ATTENUATION_LEFT:u8=0xf0; pub const CODEC_OVERFLOW_RIGHT:u8=1; pub const CODEC_OVERFLOW_LEFT:u8=2; pub const CODEC_GPIO_IN:u8=0; pub const CODEC_GPIO_OUT:u8=1;
pub const TT_PALETTE_BASE:usize=0xffff8400; pub const FPL_BASE:usize=0xffff9800; pub const ATARI_ETHERNAT_PHYS_ADDR:usize=0x80000000;

#[inline] pub unsafe fn dma_cache_maintenance(paddr: c_ulong, len:c_ulong, writeflag:c_int) { if writeflag != 0 { if !mach_is_medusa() || CPU_IS_060 { cache_push(paddr,len); } } else if !mach_is_medusa() { cache_clear(paddr,len); } }
pub const DMASND_MFP_INT_REPLAY:u8=1; pub const DMASND_MFP_INT_RECORD:u8=2; pub const DMASND_TIMERA_INT_REPLAY:u8=4; pub const DMASND_TIMERA_INT_RECORD:u8=8;
pub const DMASND_CTRL_OFF:u8=0; pub const DMASND_CTRL_ON:u8=1; pub const DMASND_CTRL_REPEAT:u8=2; pub const DMASND_CTRL_RECORD_ON:u8=0x10; pub const DMASND_CTRL_RECORD_OFF:u8=0; pub const DMASND_CTRL_RECORD_REPEAT:u8=0x20; pub const DMASND_CTRL_SELECT_REPLAY:u8=0; pub const DMASND_CTRL_SELECT_RECORD:u8=0x80; pub const DMASND_MODE_MONO:u8=0x80; pub const DMASND_MODE_STEREO:u8=0; pub const DMASND_MODE_8BIT:u8=0; pub const DMASND_MODE_16BIT:u8=0x40; pub const DMASND_MODE_6KHZ:u8=0; pub const DMASND_MODE_12KHZ:u8=1; pub const DMASND_MODE_25KHZ:u8=2; pub const DMASND_MODE_50KHZ:u8=3;
pub const MW_LM1992_ADDR:u16=0x400; pub const MW_LM1992_PSG_LOW:u16=0; pub const MW_LM1992_PSG_HIGH:u16=1; pub const MW_LM1992_PSG_OFF:u16=2;
#[inline] pub const fn mw_lm1992_volume(db:i32)->u16 { 0x0c0 + if db < -80 {0} else if db > 0 {40} else {(db+80)/2} as u16 }
pub const ACIA_DIV1:u8=0; pub const ACIA_DIV16:u8=1; pub const ACIA_DIV64:u8=2; pub const ACIA_RESET:u8=3;
pub const ACIA_D7E2S:u8=0<<2; pub const ACIA_D7O2S:u8=1<<2; pub const ACIA_D7E1S:u8=2<<2; pub const ACIA_D7O1S:u8=3<<2; pub const ACIA_D8N2S:u8=4<<2; pub const ACIA_D8N1S:u8=5<<2; pub const ACIA_D8E1S:u8=6<<2; pub const ACIA_D8O1S:u8=7<<2;
pub const ACIA_RLTID:u8=0; pub const ACIA_RLTIE:u8=1<<5; pub const ACIA_RHTID:u8=2<<5; pub const ACIA_RLTIDSB:u8=3<<5; pub const ACIA_RID:u8=0; pub const ACIA_RIE:u8=1<<7;
pub const ACIA_RDRF:u8=1; pub const ACIA_TDRE:u8=1<<1; pub const ACIA_DCD:u8=1<<2; pub const ACIA_CTS:u8=1<<3; pub const ACIA_FE:u8=1<<4; pub const ACIA_OVRN:u8=1<<5; pub const ACIA_PE:u8=1<<6; pub const ACIA_IRQ:u8=1<<7;
#[inline] pub const fn mw_lm1992_balleft(db:i32)->u16 { 0x140 + if db < -40 {0} else if db > 0 {20} else {(db+40)/2} as u16 }
#[inline] pub const fn mw_lm1992_balright(db:i32)->u16 { 0x100 + if db < -40 {0} else if db > 0 {20} else {(db+40)/2} as u16 }
#[inline] pub const fn mw_lm1992_treble(db:i32)->u16 { 0x080 + if db < -12 {0} else if db > 12 {12} else {(db/2)+6} as u16 }
#[inline] pub const fn mw_lm1992_bass(db:i32)->u16 { 0x040 + if db < -12 {0} else if db > 12 {12} else {(db/2)+6} as u16 }
extern "C" { pub fn cache_push(paddr:c_ulong,len:c_ulong); pub fn cache_clear(paddr:c_ulong,len:c_ulong); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
