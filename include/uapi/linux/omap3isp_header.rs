/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Direct Rust translation of omap3isp.h. */

// External Linux ABI types/macros are supplied by the including environment.
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __s16 = i16;
pub type __s32 = i32;
pub type __s64 = i64;

pub const VIDIOC_OMAP3ISP_CCDC_CFG: _ = _IOWR('V' as u8, BASE_VIDIOC_PRIVATE + 1, omap3isp_ccdc_update_config);
pub const VIDIOC_OMAP3ISP_PRV_CFG: _ = _IOWR('V' as u8, BASE_VIDIOC_PRIVATE + 2, omap3isp_prev_update_config);
pub const VIDIOC_OMAP3ISP_AEWB_CFG: _ = _IOWR('V' as u8, BASE_VIDIOC_PRIVATE + 3, omap3isp_h3a_aewb_config);
pub const VIDIOC_OMAP3ISP_HIST_CFG: _ = _IOWR('V' as u8, BASE_VIDIOC_PRIVATE + 4, omap3isp_hist_config);
pub const VIDIOC_OMAP3ISP_AF_CFG: _ = _IOWR('V' as u8, BASE_VIDIOC_PRIVATE + 5, omap3isp_h3a_af_config);
pub const VIDIOC_OMAP3ISP_STAT_REQ: _ = _IOWR('V' as u8, BASE_VIDIOC_PRIVATE + 6, omap3isp_stat_data);
pub const VIDIOC_OMAP3ISP_STAT_REQ_TIME32: _ = _IOWR('V' as u8, BASE_VIDIOC_PRIVATE + 6, omap3isp_stat_data_time32);
pub const VIDIOC_OMAP3ISP_STAT_EN: _ = _IOWR('V' as u8, BASE_VIDIOC_PRIVATE + 7, ::core::ffi::c_ulong);

pub const V4L2_EVENT_OMAP3ISP_CLASS: _ = V4L2_EVENT_PRIVATE_START | 0x100;
pub const V4L2_EVENT_OMAP3ISP_AEWB: _ = V4L2_EVENT_OMAP3ISP_CLASS | 0x1;
pub const V4L2_EVENT_OMAP3ISP_AF: _ = V4L2_EVENT_OMAP3ISP_CLASS | 0x2;
pub const V4L2_EVENT_OMAP3ISP_HIST: _ = V4L2_EVENT_OMAP3ISP_CLASS | 0x3;

#[repr(C)] pub struct omap3isp_stat_event_status { pub frame_number: __u32, pub config_counter: __u16, pub buf_err: __u8 }

pub const OMAP3ISP_AEWB_MAX_SATURATION_LIM: u32 = 1023;
pub const OMAP3ISP_AEWB_MIN_WIN_H: u32 = 2; pub const OMAP3ISP_AEWB_MAX_WIN_H: u32 = 256;
pub const OMAP3ISP_AEWB_MIN_WIN_W: u32 = 6; pub const OMAP3ISP_AEWB_MAX_WIN_W: u32 = 256;
pub const OMAP3ISP_AEWB_MIN_WINVC: u32 = 1; pub const OMAP3ISP_AEWB_MIN_WINHC: u32 = 1;
pub const OMAP3ISP_AEWB_MAX_WINVC: u32 = 128; pub const OMAP3ISP_AEWB_MAX_WINHC: u32 = 36;
pub const OMAP3ISP_AEWB_MAX_WINSTART: u32 = 4095; pub const OMAP3ISP_AEWB_MIN_SUB_INC: u32 = 2;
pub const OMAP3ISP_AEWB_MAX_SUB_INC: u32 = 32; pub const OMAP3ISP_AEWB_MAX_BUF_SIZE: u32 = 83600;
pub const OMAP3ISP_AF_IIRSH_MIN: u32 = 0; pub const OMAP3ISP_AF_IIRSH_MAX: u32 = 4095;
pub const OMAP3ISP_AF_PAXEL_HORIZONTAL_COUNT_MIN: u32 = 1; pub const OMAP3ISP_AF_PAXEL_HORIZONTAL_COUNT_MAX: u32 = 36;
pub const OMAP3ISP_AF_PAXEL_VERTICAL_COUNT_MIN: u32 = 1; pub const OMAP3ISP_AF_PAXEL_VERTICAL_COUNT_MAX: u32 = 128;
pub const OMAP3ISP_AF_PAXEL_INCREMENT_MIN: u32 = 2; pub const OMAP3ISP_AF_PAXEL_INCREMENT_MAX: u32 = 32;
pub const OMAP3ISP_AF_PAXEL_HEIGHT_MIN: u32 = 2; pub const OMAP3ISP_AF_PAXEL_HEIGHT_MAX: u32 = 256;
pub const OMAP3ISP_AF_PAXEL_WIDTH_MIN: u32 = 16; pub const OMAP3ISP_AF_PAXEL_WIDTH_MAX: u32 = 256;
pub const OMAP3ISP_AF_PAXEL_HZSTART_MIN: u32 = 1; pub const OMAP3ISP_AF_PAXEL_HZSTART_MAX: u32 = 4095;
pub const OMAP3ISP_AF_PAXEL_VTSTART_MIN: u32 = 0; pub const OMAP3ISP_AF_PAXEL_VTSTART_MAX: u32 = 4095;
pub const OMAP3ISP_AF_THRESHOLD_MAX: u32 = 255; pub const OMAP3ISP_AF_COEF_MAX: u32 = 4095;
pub const OMAP3ISP_AF_PAXEL_SIZE: u32 = 48; pub const OMAP3ISP_AF_MAX_BUF_SIZE: u32 = 221184;

#[repr(C)] pub struct omap3isp_h3a_aewb_config { pub buf_size: __u32, pub config_counter: __u16, pub saturation_limit: __u16, pub win_height: __u16, pub win_width: __u16, pub ver_win_count: __u16, pub hor_win_count: __u16, pub ver_win_start: __u16, pub hor_win_start: __u16, pub blk_ver_win_start: __u16, pub blk_win_height: __u16, pub subsample_ver_inc: __u16, pub subsample_hor_inc: __u16, pub alaw_enable: __u8 }
#[repr(C)] pub struct timeval { pub tv_sec: __s64, pub tv_usec: __s64 }
#[repr(C)] pub struct omap3isp_stat_data { pub ts: timeval, pub buf: *mut core::ffi::c_void, pub buf_size: __u32, pub frame_number: __u16, pub cur_frame: __u16, pub config_counter: __u16 }
#[repr(C)] pub struct omap3isp_stat_data_time32 { pub ts: timeval32, pub buf: __u32, pub buf_size: __u32, pub frame_number: __u16, pub cur_frame: __u16, pub config_counter: __u16 }
#[repr(C)] pub struct timeval32 { pub tv_sec: __s32, pub tv_usec: __s32 }

pub const OMAP3ISP_HIST_BINS_32: u32 = 0; pub const OMAP3ISP_HIST_BINS_64: u32 = 1; pub const OMAP3ISP_HIST_BINS_128: u32 = 2; pub const OMAP3ISP_HIST_BINS_256: u32 = 3;
#[inline] pub const fn OMAP3ISP_HIST_MEM_SIZE_BINS(n: u32) -> u32 { (1u32 << (n + 5)) * 4 * 4 }
pub const OMAP3ISP_HIST_MEM_SIZE: u32 = 1024; pub const OMAP3ISP_HIST_MIN_REGIONS: u32 = 1; pub const OMAP3ISP_HIST_MAX_REGIONS: u32 = 4; pub const OMAP3ISP_HIST_MAX_WB_GAIN: u32 = 255; pub const OMAP3ISP_HIST_MIN_WB_GAIN: u32 = 0; pub const OMAP3ISP_HIST_MAX_BIT_WIDTH: u32 = 14; pub const OMAP3ISP_HIST_MIN_BIT_WIDTH: u32 = 8; pub const OMAP3ISP_HIST_MAX_WG: usize = 4; pub const OMAP3ISP_HIST_MAX_BUF_SIZE: u32 = 4096;
pub const OMAP3ISP_HIST_SOURCE_CCDC: u32 = 0; pub const OMAP3ISP_HIST_SOURCE_MEM: u32 = 1; pub const OMAP3ISP_HIST_CFA_BAYER: u32 = 0; pub const OMAP3ISP_HIST_CFA_FOVEONX3: u32 = 1;
#[repr(C)] pub struct omap3isp_hist_region { pub h_start: __u16, pub h_end: __u16, pub v_start: __u16, pub v_end: __u16 }
#[repr(C)] pub struct omap3isp_hist_config { pub buf_size: __u32, pub config_counter: __u16, pub num_acc_frames: __u8, pub hist_bins: __u16, pub cfa: __u8, pub wg: [__u8; 4], pub num_regions: __u8, pub region: [omap3isp_hist_region; 4] }

pub const OMAP3ISP_AF_NUM_COEF: usize = 11;
#[repr(C)] #[derive(Copy, Clone)] pub enum omap3isp_h3a_af_fvmode { OMAP3ISP_AF_MODE_SUMMED = 0, OMAP3ISP_AF_MODE_PEAK = 1 }
#[repr(C)] #[derive(Copy, Clone)] pub enum omap3isp_h3a_af_rgbpos { OMAP3ISP_AF_GR_GB_BAYER=0, OMAP3ISP_AF_RG_GB_BAYER=1, OMAP3ISP_AF_GR_BG_BAYER=2, OMAP3ISP_AF_RG_BG_BAYER=3, OMAP3ISP_AF_GG_RB_CUSTOM=4, OMAP3ISP_AF_RB_GG_CUSTOM=5 }
#[repr(C)] pub struct omap3isp_h3a_af_hmf { pub enable: __u8, pub threshold: __u8 }
#[repr(C)] pub struct omap3isp_h3a_af_iir { pub h_start: __u16, pub coeff_set0: [__u16; 11], pub coeff_set1: [__u16; 11] }
#[repr(C)] pub struct omap3isp_h3a_af_paxel { pub h_start: __u16, pub v_start: __u16, pub width: __u8, pub height: __u8, pub h_cnt: __u8, pub v_cnt: __u8, pub line_inc: __u8 }
#[repr(C)] pub struct omap3isp_h3a_af_config { pub buf_size: __u32, pub config_counter: __u16, pub hmf: omap3isp_h3a_af_hmf, pub iir: omap3isp_h3a_af_iir, pub paxel: omap3isp_h3a_af_paxel, pub rgb_pos: omap3isp_h3a_af_rgbpos, pub fvmode: omap3isp_h3a_af_fvmode, pub alaw_enable: __u8 }

pub const OMAP3ISP_CCDC_ALAW: u32=1<<0; pub const OMAP3ISP_CCDC_LPF: u32=1<<1; pub const OMAP3ISP_CCDC_BLCLAMP: u32=1<<2; pub const OMAP3ISP_CCDC_BCOMP: u32=1<<3; pub const OMAP3ISP_CCDC_FPC: u32=1<<4; pub const OMAP3ISP_CCDC_CULL: u32=1<<5; pub const OMAP3ISP_CCDC_CONFIG_LSC: u32=1<<7; pub const OMAP3ISP_CCDC_TBL_LSC: u32=1<<8; pub const OMAP3ISP_RGB_MAX: usize=3;
#[repr(C)] #[derive(Copy, Clone)] pub enum omap3isp_alaw_ipwidth { OMAP3ISP_ALAW_BIT12_3=0x3, OMAP3ISP_ALAW_BIT11_2=0x4, OMAP3ISP_ALAW_BIT10_1=0x5, OMAP3ISP_ALAW_BIT9_0=0x6 }
#[repr(C)] pub struct omap3isp_ccdc_lsc_config { pub offset: __u16, pub gain_mode_n: __u8, pub gain_mode_m: __u8, pub gain_format: __u8, pub fmtsph: __u16, pub fmtlnh: __u16, pub fmtslv: __u16, pub fmtlnv: __u16, pub initial_x: __u8, pub initial_y: __u8, pub size: __u32 }
#[repr(C)] pub struct omap3isp_ccdc_bclamp { pub obgain: __u8, pub obstpixel: __u8, pub oblines: __u8, pub oblen: __u8, pub dcsubval: __u16 }
#[repr(C)] pub struct omap3isp_ccdc_fpc { pub fpnum: __u16, pub fpcaddr: __u32 }
#[repr(C)] pub struct omap3isp_ccdc_blcomp { pub b_mg: __u8, pub gb_g: __u8, pub gr_cy: __u8, pub r_ye: __u8 }
#[repr(C)] pub struct omap3isp_ccdc_culling { pub v_pattern: __u8, pub h_odd: __u16, pub h_even: __u16 }
#[repr(C)] pub struct omap3isp_ccdc_update_config { pub update: __u16, pub flag: __u16, pub alawip: omap3isp_alaw_ipwidth, pub bclamp: *mut omap3isp_ccdc_bclamp, pub blcomp: *mut omap3isp_ccdc_blcomp, pub fpc: *mut omap3isp_ccdc_fpc, pub lsc_cfg: *mut omap3isp_ccdc_lsc_config, pub cull: *mut omap3isp_ccdc_culling, pub lsc: *mut __u8 }

pub const OMAP3ISP_PREV_LUMAENH:u32=1<<0; pub const OMAP3ISP_PREV_INVALAW:u32=1<<1; pub const OMAP3ISP_PREV_HRZ_MED:u32=1<<2; pub const OMAP3ISP_PREV_CFA:u32=1<<3; pub const OMAP3ISP_PREV_CHROMA_SUPP:u32=1<<4; pub const OMAP3ISP_PREV_WB:u32=1<<5; pub const OMAP3ISP_PREV_BLKADJ:u32=1<<6; pub const OMAP3ISP_PREV_RGB2RGB:u32=1<<7; pub const OMAP3ISP_PREV_COLOR_CONV:u32=1<<8; pub const OMAP3ISP_PREV_YC_LIMIT:u32=1<<9; pub const OMAP3ISP_PREV_DEFECT_COR:u32=1<<10; pub const OMAP3ISP_PREV_DRK_FRM_CAPTURE:u32=1<<12; pub const OMAP3ISP_PREV_DRK_FRM_SUBTRACT:u32=1<<13; pub const OMAP3ISP_PREV_LENS_SHADING:u32=1<<14; pub const OMAP3ISP_PREV_NF:u32=1<<15; pub const OMAP3ISP_PREV_GAMMA:u32=1<<16;
pub const OMAP3ISP_PREV_NF_TBL_SIZE:usize=64; pub const OMAP3ISP_PREV_CFA_TBL_SIZE:usize=576; pub const OMAP3ISP_PREV_CFA_BLK_SIZE:usize=144; pub const OMAP3ISP_PREV_GAMMA_TBL_SIZE:usize=1024; pub const OMAP3ISP_PREV_YENH_TBL_SIZE:usize=128; pub const OMAP3ISP_PREV_DETECT_CORRECT_CHANNELS:usize=4;
#[repr(C)] pub struct omap3isp_prev_hmed { pub odddist:__u8, pub evendist:__u8, pub thres:__u8 }
#[repr(C)] #[derive(Copy,Clone)] pub enum omap3isp_cfa_fmt { OMAP3ISP_CFAFMT_BAYER, OMAP3ISP_CFAFMT_SONYVGA, OMAP3ISP_CFAFMT_RGBFOVEON, OMAP3ISP_CFAFMT_DNSPL, OMAP3ISP_CFAFMT_HONEYCOMB, OMAP3ISP_CFAFMT_RRGGBBFOVEON }
#[repr(C)] pub struct omap3isp_prev_cfa { pub format:omap3isp_cfa_fmt, pub gradthrs_vert:__u8, pub gradthrs_horz:__u8, pub table:[[__u32;144];4] }
#[repr(C)] pub struct omap3isp_prev_csup { pub gain:__u8, pub thres:__u8, pub hypf_en:__u8 }
#[repr(C)] pub struct omap3isp_prev_wbal { pub dgain:__u16, pub coef3:__u8, pub coef2:__u8, pub coef1:__u8, pub coef0:__u8 }
#[repr(C)] pub struct omap3isp_prev_blkadj { pub red:__u8, pub green:__u8, pub blue:__u8 }
#[repr(C)] pub struct omap3isp_prev_rgbtorgb { pub matrix:[[__u16;3];3], pub offset:[__u16;3] }
#[repr(C)] pub struct omap3isp_prev_csc { pub matrix:[[__u16;3];3], pub offset:[__s16;3] }
#[repr(C)] pub struct omap3isp_prev_yclimit { pub minC:__u8, pub maxC:__u8, pub minY:__u8, pub maxY:__u8 }
#[repr(C)] pub struct omap3isp_prev_dcor { pub couplet_mode_en:__u8, pub detect_correct:[__u32;4] }
#[repr(C)] pub struct omap3isp_prev_nf { pub spread:__u8, pub table:[__u32;64] }
#[repr(C)] pub struct omap3isp_prev_gtables { pub red:[__u32;1024], pub green:[__u32;1024], pub blue:[__u32;1024] }
#[repr(C)] pub struct omap3isp_prev_luma { pub table:[__u32;128] }
#[repr(C)] pub struct omap3isp_prev_update_config { pub update:__u32, pub flag:__u32, pub shading_shift:__u32, pub luma:*mut omap3isp_prev_luma, pub hmed:*mut omap3isp_prev_hmed, pub cfa:*mut omap3isp_prev_cfa, pub csup:*mut omap3isp_prev_csup, pub wbal:*mut omap3isp_prev_wbal, pub blkadj:*mut omap3isp_prev_blkadj, pub rgb2rgb:*mut omap3isp_prev_rgbtorgb, pub csc:*mut omap3isp_prev_csc, pub yclimit:*mut omap3isp_prev_yclimit, pub dcor:*mut omap3isp_prev_dcor, pub nf:*mut omap3isp_prev_nf, pub gamma:*mut omap3isp_prev_gtables }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
