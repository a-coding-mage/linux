/* SPDX-License-Identifier: GPL-2.0-only */
/* v4l2-tpg.h - Test Pattern Generator */

// C dependencies: linux types, errno, random, slab, vmalloc, videodev2.

#[repr(C)]
pub struct tpg_rbg_color8 { pub r: u8, pub g: u8, pub b: u8 }
#[repr(C)]
pub struct tpg_rbg_color16 { pub r: u16, pub g: u16, pub b: u16 }

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpg_color {
    TPG_COLOR_CSC_WHITE, TPG_COLOR_CSC_YELLOW, TPG_COLOR_CSC_CYAN,
    TPG_COLOR_CSC_GREEN, TPG_COLOR_CSC_MAGENTA, TPG_COLOR_CSC_RED,
    TPG_COLOR_CSC_BLUE, TPG_COLOR_CSC_BLACK, TPG_COLOR_75_YELLOW,
    TPG_COLOR_75_CYAN, TPG_COLOR_75_GREEN, TPG_COLOR_75_MAGENTA,
    TPG_COLOR_75_RED, TPG_COLOR_75_BLUE, TPG_COLOR_100_WHITE,
    TPG_COLOR_100_YELLOW, TPG_COLOR_100_CYAN, TPG_COLOR_100_GREEN,
    TPG_COLOR_100_MAGENTA, TPG_COLOR_100_RED, TPG_COLOR_100_BLUE,
    TPG_COLOR_100_BLACK, TPG_COLOR_TEXTFG, TPG_COLOR_TEXTBG,
    TPG_COLOR_RANDOM, TPG_COLOR_RAMP,
}
pub const TPG_COLOR_MAX: usize = 25 + 256;

extern "C" {
    pub static tpg_colors: [tpg_rbg_color8; TPG_COLOR_MAX];
    pub static tpg_rec709_to_linear: [u16; 255 * 16 + 1];
    pub static tpg_linear_to_rec709: [u16; 255 * 16 + 1];
    pub static tpg_csc_colors: [[[tpg_rbg_color16; 8]; V4L2_XFER_FUNC_SMPTE2084 as usize + 1]; V4L2_COLORSPACE_DCI_P3 as usize + 1];
}

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpg_pattern { TPG_PAT_75_COLORBAR, TPG_PAT_100_COLORBAR, TPG_PAT_CSC_COLORBAR,
    TPG_PAT_100_HCOLORBAR, TPG_PAT_100_COLORSQUARES, TPG_PAT_BLACK, TPG_PAT_WHITE,
    TPG_PAT_RED, TPG_PAT_GREEN, TPG_PAT_BLUE, TPG_PAT_CHECKERS_16X16,
    TPG_PAT_CHECKERS_2X2, TPG_PAT_CHECKERS_1X1, TPG_PAT_COLOR_CHECKERS_2X2,
    TPG_PAT_COLOR_CHECKERS_1X1, TPG_PAT_ALTERNATING_HLINES, TPG_PAT_ALTERNATING_VLINES,
    TPG_PAT_CROSS_1_PIXEL, TPG_PAT_CROSS_2_PIXELS, TPG_PAT_CROSS_10_PIXELS,
    TPG_PAT_GRAY_RAMP, TPG_PAT_NOISE }
extern "C" { pub static tpg_pattern_strings: *const *const i8; }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum tpg_quality { TPG_QUAL_COLOR, TPG_QUAL_GRAY, TPG_QUAL_NOISE }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum tpg_video_aspect { TPG_VIDEO_ASPECT_IMAGE, TPG_VIDEO_ASPECT_4X3, TPG_VIDEO_ASPECT_14X9_CENTRE, TPG_VIDEO_ASPECT_16X9_CENTRE, TPG_VIDEO_ASPECT_16X9_ANAMORPHIC }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum tpg_pixel_aspect { TPG_PIXEL_ASPECT_SQUARE, TPG_PIXEL_ASPECT_NTSC, TPG_PIXEL_ASPECT_PAL }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum tpg_move_mode { TPG_MOVE_NEG_FAST, TPG_MOVE_NEG, TPG_MOVE_NEG_SLOW, TPG_MOVE_NONE, TPG_MOVE_POS_SLOW, TPG_MOVE_POS, TPG_MOVE_POS_FAST }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum tgp_color_enc { TGP_COLOR_ENC_RGB, TGP_COLOR_ENC_YCBCR, TGP_COLOR_ENC_HSV, TGP_COLOR_ENC_LUMA }
extern "C" { pub static tpg_aspect_strings: *const *const i8; }
pub const TPG_MAX_PLANES: usize = 3;
pub const TPG_MAX_PAT_LINES: usize = 8;

// v4l2_rect and v4l2_std_id are supplied by videodev2; u32/u8/s16 are kernel integer types.
#[repr(C)]
pub struct tpg_data {
    pub src_width: u32, pub src_height: u32, pub buf_height: u32, pub scaled_width: u32,
    pub field: u32, pub field_alternate: bool, pub crop: v4l2_rect, pub compose: v4l2_rect,
    pub border: v4l2_rect, pub square: v4l2_rect, pub qual: tpg_quality, pub qual_offset: u32,
    pub alpha_component: u8, pub alpha_red_only: bool, pub brightness: u8, pub contrast: u8,
    pub saturation: u8, pub hue: i16, pub fourcc: u32, pub color_enc: tgp_color_enc,
    pub colorspace: u32, pub xfer_func: u32, pub ycbcr_enc: u32, pub hsv_enc: u32,
    pub real_xfer_func: u32, pub real_hsv_enc: u32, pub real_ycbcr_enc: u32,
    pub quantization: u32, pub real_quantization: u32, pub vid_aspect: tpg_video_aspect,
    pub pix_aspect: tpg_pixel_aspect, pub rgb_range: u32, pub real_rgb_range: u32,
    pub buffers: u32, pub planes: u32, pub interleaved: bool,
    pub vdownsampling: [u8; TPG_MAX_PLANES], pub hdownsampling: [u8; TPG_MAX_PLANES],
    pub hmask: [u32; TPG_MAX_PLANES], pub colors: [[u8; 3]; TPG_COLOR_MAX],
    pub textfg: [[u8; 8]; TPG_MAX_PLANES], pub textbg: [[u8; 8]; TPG_MAX_PLANES],
    pub twopixelsize: [u32; TPG_MAX_PLANES], pub bytesperline: [u32; TPG_MAX_PLANES],
    pub pattern: tpg_pattern, pub hflip: bool, pub vflip: bool, pub perc_fill: u32,
    pub perc_fill_blank: bool, pub show_border: bool, pub show_square: bool,
    pub insert_sav: bool, pub insert_eav: bool, pub insert_hdmi_video_guard_band: bool,
    pub mv_hor_mode: tpg_move_mode, pub mv_hor_count: i32, pub mv_hor_step: i32,
    pub mv_vert_mode: tpg_move_mode, pub mv_vert_count: i32, pub mv_vert_step: i32,
    pub recalc_colors: bool, pub recalc_lines: bool, pub recalc_square_border: bool,
    pub max_line_width: u32, pub lines: [[*mut u8; TPG_MAX_PLANES]; TPG_MAX_PAT_LINES],
    pub downsampled_lines: [[*mut u8; TPG_MAX_PLANES]; TPG_MAX_PAT_LINES],
    pub random_line: [*mut u8; TPG_MAX_PLANES], pub contrast_line: [*mut u8; TPG_MAX_PLANES],
    pub black_line: [*mut u8; TPG_MAX_PLANES],
}

extern "C" {
    pub fn tpg_init(tpg: *mut tpg_data, w: u32, h: u32); pub fn tpg_alloc(tpg: *mut tpg_data, max_w: u32) -> i32;
    pub fn tpg_free(tpg: *mut tpg_data); pub fn tpg_reset_source(tpg: *mut tpg_data, width: u32, height: u32, field: u32);
    pub fn tpg_log_status(tpg: *mut tpg_data); pub fn tpg_set_font(f: *const u8);
    pub fn tpg_gen_text(tpg: *const tpg_data, basep: *mut [*mut u8; 2], y: i32, x: i32, text: *const i8);
    pub fn tpg_calc_text_basep(tpg: *mut tpg_data, basep: *mut [*mut u8; 2], p: u32, vbuf: *mut u8);
    pub fn tpg_g_interleaved_plane(tpg: *const tpg_data, buf_line: u32) -> u32;
    pub fn tpg_fill_plane_buffer(tpg: *mut tpg_data, std: v4l2_std_id, p: u32, vbuf: *mut u8);
    pub fn tpg_fillbuffer(tpg: *mut tpg_data, std: v4l2_std_id, p: u32, vbuf: *mut u8);
    pub fn tpg_s_fourcc(tpg: *mut tpg_data, fourcc: u32) -> bool;
    pub fn tpg_s_crop_compose(tpg: *mut tpg_data, crop: *const v4l2_rect, compose: *const v4l2_rect);
    pub fn tpg_g_color_order(tpg: *const tpg_data) -> *const i8;
}

// Inline accessors below retain the C header's semantics.
#[inline] pub unsafe fn tpg_s_pattern(t: *mut tpg_data, p: tpg_pattern) { if (*t).pattern != p { (*t).pattern=p; (*t).recalc_colors=true; } }
#[inline] pub unsafe fn tpg_s_quality(t:*mut tpg_data,q:tpg_quality,o:u32){if (*t).qual!=q||(*t).qual_offset!=o{(*t).qual=q;(*t).qual_offset=o;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_g_quality(t:*const tpg_data)->tpg_quality{(*t).qual}
#[inline] pub unsafe fn tpg_s_alpha_component(t:*mut tpg_data,v:u8){if (*t).alpha_component!=v{(*t).alpha_component=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_s_alpha_mode(t:*mut tpg_data,v:bool){if (*t).alpha_red_only!=v{(*t).alpha_red_only=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_s_brightness(t:*mut tpg_data,v:u8){if (*t).brightness!=v{(*t).brightness=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_s_contrast(t:*mut tpg_data,v:u8){if (*t).contrast!=v{(*t).contrast=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_s_saturation(t:*mut tpg_data,v:u8){if (*t).saturation!=v{(*t).saturation=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_s_hue(t:*mut tpg_data,mut v:i16){v=v.clamp(-128,128);if (*t).hue!=v{(*t).hue=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_s_rgb_range(t:*mut tpg_data,v:u32){if (*t).rgb_range!=v{(*t).rgb_range=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_s_real_rgb_range(t:*mut tpg_data,v:u32){if (*t).real_rgb_range!=v{(*t).real_rgb_range=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_s_colorspace(t:*mut tpg_data,v:u32){if (*t).colorspace!=v{(*t).colorspace=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_g_colorspace(t:*const tpg_data)->u32{(*t).colorspace}
#[inline] pub unsafe fn tpg_s_ycbcr_enc(t:*mut tpg_data,v:u32){if (*t).ycbcr_enc!=v{(*t).ycbcr_enc=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_g_ycbcr_enc(t:*const tpg_data)->u32{(*t).ycbcr_enc}
#[inline] pub unsafe fn tpg_s_hsv_enc(t:*mut tpg_data,v:u32){if (*t).hsv_enc!=v{(*t).hsv_enc=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_g_hsv_enc(t:*const tpg_data)->u32{(*t).hsv_enc}
#[inline] pub unsafe fn tpg_s_xfer_func(t:*mut tpg_data,v:u32){if (*t).xfer_func!=v{(*t).xfer_func=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_g_xfer_func(t:*const tpg_data)->u32{(*t).xfer_func}
#[inline] pub unsafe fn tpg_s_quantization(t:*mut tpg_data,v:u32){if (*t).quantization!=v{(*t).quantization=v;(*t).recalc_colors=true;}}
#[inline] pub unsafe fn tpg_g_quantization(t:*const tpg_data)->u32{(*t).quantization}
#[inline] pub unsafe fn tpg_g_buffers(t:*const tpg_data)->u32{(*t).buffers}
#[inline] pub unsafe fn tpg_g_planes(t:*const tpg_data)->u32{if (*t).interleaved{1}else{(*t).planes}}
#[inline] pub unsafe fn tpg_g_interleaved(t:*const tpg_data)->bool{(*t).interleaved}
#[inline] pub unsafe fn tpg_g_twopixelsize(t:*const tpg_data,p:u32)->u32{(*t).twopixelsize[p as usize]}
#[inline] pub unsafe fn tpg_hdiv(t:*const tpg_data,p:u32,x:u32)->u32{((x/(*t).hdownsampling[p as usize])&(*t).hmask[p as usize])*(*t).twopixelsize[p as usize]/2}
#[inline] pub unsafe fn tpg_hscale(t:*const tpg_data,x:u32)->u32{x*(*t).scaled_width/(*t).src_width}
#[inline] pub unsafe fn tpg_hscale_div(t:*const tpg_data,p:u32,x:u32)->u32{tpg_hdiv(t,p,tpg_hscale(t,x))}
#[inline] pub unsafe fn tpg_g_bytesperline(t:*const tpg_data,p:u32)->u32{(*t).bytesperline[p as usize]}
#[inline] pub unsafe fn tpg_s_bytesperline(t:*mut tpg_data,plane:u32,bpl:u32){if (*t).buffers>1{(*t).bytesperline[plane as usize]=bpl;return;}let mut p=0;while p<tpg_g_planes(t){let w=bpl*(*t).twopixelsize[p as usize]/(*t).twopixelsize[0];(*t).bytesperline[p as usize]=w/(*t).hdownsampling[p as usize];p+=1;}if (*t).interleaved{(*t).bytesperline[1]=(*t).bytesperline[0];}}
#[inline] pub unsafe fn tpg_g_line_width(t:*const tpg_data,plane:u32)->u32{if (*t).buffers>1{return tpg_g_bytesperline(t,plane)}let mut w=0;let mut p=0;while p<tpg_g_planes(t){w+=tpg_g_bytesperline(t,p)/(*t).vdownsampling[p as usize] ;p+=1;}w}
#[inline] pub unsafe fn tpg_calc_line_width(t:*const tpg_data,_plane:u32,bpl:u32)->u32{if (*t).buffers>1{return bpl}let mut w=0;let mut p=0;while p<tpg_g_planes(t){let pw=bpl*(*t).twopixelsize[p as usize]/(*t).twopixelsize[0]/(*t).hdownsampling[p as usize];w+=pw/(*t).vdownsampling[p as usize];p+=1;}w}
#[inline] pub unsafe fn tpg_calc_plane_size(t:*const tpg_data,p:u32)->u32{if p>=tpg_g_planes(t){0}else{tpg_g_bytesperline(t,p)*(*t).buf_height/(*t).vdownsampling[p as usize]}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
