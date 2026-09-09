/* Translated from drm_edid.h. */

pub const EDID_LENGTH: u32 = 128;
pub const DDC_ADDR: u32 = 0x50;
pub const DDC_ADDR2: u32 = 0x52;
pub const CEA_EXT: u32 = 0x02;
pub const VTB_EXT: u32 = 0x10;
pub const DI_EXT: u32 = 0x40;
pub const LS_EXT: u32 = 0x50;
pub const MI_EXT: u32 = 0x60;
pub const DISPLAYID_EXT: u32 = 0x70;

#[repr(C, packed)]
pub struct est_timings { pub t1: u8, pub t2: u8, pub mfg_rsvd: u8 }
pub const EDID_TIMING_ASPECT_SHIFT: u32 = 6;
pub const EDID_TIMING_ASPECT_MASK: u32 = 0x3 << EDID_TIMING_ASPECT_SHIFT;
pub const EDID_TIMING_VFREQ_SHIFT: u32 = 0;
pub const EDID_TIMING_VFREQ_MASK: u32 = 0x3f << EDID_TIMING_VFREQ_SHIFT;

#[repr(C, packed)]
pub struct std_timing { pub hsize: u8, pub vfreq_aspect: u8 }
pub const DRM_EDID_PT_HSYNC_POSITIVE: u32 = 1 << 1;
pub const DRM_EDID_PT_VSYNC_POSITIVE: u32 = 1 << 2;
pub const DRM_EDID_PT_SEPARATE_SYNC: u32 = 3 << 3;
pub const DRM_EDID_PT_STEREO: u32 = 1 << 5;
pub const DRM_EDID_PT_INTERLACED: u32 = 1 << 7;

#[repr(C, packed)]
pub struct detailed_pixel_timing { pub hactive_lo:u8, pub hblank_lo:u8, pub hactive_hblank_hi:u8, pub vactive_lo:u8, pub vblank_lo:u8, pub vactive_vblank_hi:u8, pub hsync_offset_lo:u8, pub hsync_pulse_width_lo:u8, pub vsync_offset_pulse_width_lo:u8, pub hsync_vsync_offset_pulse_width_hi:u8, pub width_mm_lo:u8, pub height_mm_lo:u8, pub width_height_mm_hi:u8, pub hborder:u8, pub vborder:u8, pub misc:u8 }
#[repr(C, packed)] pub struct detailed_data_string { pub str_: [u8; 13] }

pub const DRM_EDID_RANGE_OFFSET_MIN_VFREQ:u32=1<<0; pub const DRM_EDID_RANGE_OFFSET_MAX_VFREQ:u32=1<<1; pub const DRM_EDID_RANGE_OFFSET_MIN_HFREQ:u32=1<<2; pub const DRM_EDID_RANGE_OFFSET_MAX_HFREQ:u32=1<<3;
pub const DRM_EDID_DEFAULT_GTF_SUPPORT_FLAG:u32=0; pub const DRM_EDID_RANGE_LIMITS_ONLY_FLAG:u32=1; pub const DRM_EDID_SECONDARY_GTF_SUPPORT_FLAG:u32=2; pub const DRM_EDID_CVT_SUPPORT_FLAG:u32=4;
pub const DRM_EDID_CVT_FLAGS_STANDARD_BLANKING:u32=1<<3; pub const DRM_EDID_CVT_FLAGS_REDUCED_BLANKING:u32=1<<4;

#[repr(C)] pub enum drm_edid_quirk { DRM_EDID_QUIRK_DP_DPCD_PROBE, DRM_EDID_QUIRK_NUM }
#[repr(C, packed)] pub struct detailed_data_monitor_range { pub min_vfreq:u8, pub max_vfreq:u8, pub min_hfreq_khz:u8, pub max_hfreq_khz:u8, pub pixel_clock_mhz:u8, pub flags:u8, pub formula: detailed_data_monitor_range_formula }
#[repr(C)] pub union detailed_data_monitor_range_formula { pub gtf2: detailed_data_monitor_range_gtf2, pub cvt: detailed_data_monitor_range_cvt }
#[repr(C, packed)] pub struct detailed_data_monitor_range_gtf2 { pub reserved:u8, pub hfreq_start_khz:u8, pub c:u8, pub m:u16, pub k:u8, pub j:u8 }
#[repr(C, packed)] pub struct detailed_data_monitor_range_cvt { pub version:u8, pub data1:u8, pub data2:u8, pub supported_aspects:u8, pub flags:u8, pub supported_scalings:u8, pub preferred_refresh:u8 }
#[repr(C, packed)] pub struct detailed_data_wpindex { pub white_yx_lo:u8, pub white_x_hi:u8, pub white_y_hi:u8, pub gamma:u8 }
#[repr(C, packed)] pub struct detailed_data_color_point { pub windex1:u8, pub wpindex1:[u8;3], pub windex2:u8, pub wpindex2:[u8;3] }
#[repr(C, packed)] pub struct cvt_timing { pub code:[u8;3] }
#[repr(C)] pub union detailed_non_pixel_data { pub str_: detailed_data_string, pub range:detailed_data_monitor_range, pub color:detailed_data_wpindex, pub timings:[std_timing;6], pub cvt:[cvt_timing;4] }
#[repr(C, packed)] pub struct detailed_non_pixel { pub pad1:u8, pub type_:u8, pub pad2:u8, pub data:detailed_non_pixel_data }
pub const EDID_DETAIL_EST_TIMINGS:u32=0xf7; pub const EDID_DETAIL_CVT_3BYTE:u32=0xf8; pub const EDID_DETAIL_COLOR_MGMT_DATA:u32=0xf9; pub const EDID_DETAIL_STD_MODES:u32=0xfa; pub const EDID_DETAIL_MONITOR_CPDATA:u32=0xfb; pub const EDID_DETAIL_MONITOR_NAME:u32=0xfc; pub const EDID_DETAIL_MONITOR_RANGE:u32=0xfd; pub const EDID_DETAIL_MONITOR_STRING:u32=0xfe; pub const EDID_DETAIL_MONITOR_SERIAL:u32=0xff;
#[repr(C)] pub union detailed_timing_data { pub pixel_data:detailed_pixel_timing, pub other_data:detailed_non_pixel }
#[repr(C, packed)] pub struct detailed_timing { pub pixel_clock:u16, pub data:detailed_timing_data }

pub const DRM_EDID_INPUT_SERRATION_VSYNC:u32=1<<0; pub const DRM_EDID_INPUT_SYNC_ON_GREEN:u32=1<<1; pub const DRM_EDID_INPUT_COMPOSITE_SYNC:u32=1<<2; pub const DRM_EDID_INPUT_SEPARATE_SYNCS:u32=1<<3; pub const DRM_EDID_INPUT_BLANK_TO_BLACK:u32=1<<4; pub const DRM_EDID_INPUT_VIDEO_LEVEL:u32=3<<5; pub const DRM_EDID_INPUT_DIGITAL:u32=1<<7;
pub const DRM_EDID_DIGITAL_DEPTH_MASK:u32=7<<4; pub const DRM_EDID_DIGITAL_DEPTH_UNDEF:u32=0<<4; pub const DRM_EDID_DIGITAL_DEPTH_6:u32=1<<4; pub const DRM_EDID_DIGITAL_DEPTH_8:u32=2<<4; pub const DRM_EDID_DIGITAL_DEPTH_10:u32=3<<4; pub const DRM_EDID_DIGITAL_DEPTH_12:u32=4<<4; pub const DRM_EDID_DIGITAL_DEPTH_14:u32=5<<4; pub const DRM_EDID_DIGITAL_DEPTH_16:u32=6<<4; pub const DRM_EDID_DIGITAL_DEPTH_RSVD:u32=7<<4;
pub const DRM_EDID_DIGITAL_TYPE_MASK:u32=7; pub const DRM_EDID_DIGITAL_TYPE_UNDEF:u32=0; pub const DRM_EDID_DIGITAL_TYPE_DVI:u32=1; pub const DRM_EDID_DIGITAL_TYPE_HDMI_A:u32=2; pub const DRM_EDID_DIGITAL_TYPE_HDMI_B:u32=3; pub const DRM_EDID_DIGITAL_TYPE_MDDI:u32=4; pub const DRM_EDID_DIGITAL_TYPE_DP:u32=5; pub const DRM_EDID_DIGITAL_DFP_1_X:u32=1;
pub const DRM_EDID_FEATURE_DEFAULT_GTF:u32=1; pub const DRM_EDID_FEATURE_CONTINUOUS_FREQ:u32=1; pub const DRM_EDID_FEATURE_PREFERRED_TIMING:u32=1<<1; pub const DRM_EDID_FEATURE_STANDARD_COLOR:u32=1<<2; pub const DRM_EDID_FEATURE_DISPLAY_TYPE:u32=3<<3; pub const DRM_EDID_FEATURE_COLOR_MASK:u32=3<<3; pub const DRM_EDID_FEATURE_RGB:u32=0; pub const DRM_EDID_FEATURE_RGB_YCRCB444:u32=1<<3; pub const DRM_EDID_FEATURE_RGB_YCRCB422:u32=2<<3; pub const DRM_EDID_FEATURE_RGB_YCRCB:u32=3<<3; pub const DRM_EDID_FEATURE_PM_ACTIVE_OFF:u32=1<<5; pub const DRM_EDID_FEATURE_PM_SUSPEND:u32=1<<6; pub const DRM_EDID_FEATURE_PM_STANDBY:u32=1<<7;
pub const DRM_EDID_HDMI_DC_48:u32=1<<6; pub const DRM_EDID_HDMI_DC_36:u32=1<<5; pub const DRM_EDID_HDMI_DC_30:u32=1<<4; pub const DRM_EDID_HDMI_DC_Y444:u32=1<<3; pub const DRM_EDID_YCBCR420_DC_48:u32=1<<2; pub const DRM_EDID_YCBCR420_DC_36:u32=1<<1; pub const DRM_EDID_YCBCR420_DC_30:u32=1; pub const DRM_EDID_YCBCR420_DC_MASK:u32=7;
pub const DRM_EDID_MAX_FRL_RATE_MASK:u32=0xf0; pub const DRM_EDID_FAPA_START_LOCATION:u32=1; pub const DRM_EDID_ALLM:u32=1<<1; pub const DRM_EDID_FVA:u32=1<<2; pub const DRM_EDID_DC_30BIT_420:u32=1; pub const DRM_EDID_DC_36BIT_420:u32=1<<1; pub const DRM_EDID_DC_48BIT_420:u32=1<<2; pub const DRM_EDID_CNMVRR:u32=1<<3; pub const DRM_EDID_CINEMA_VRR:u32=1<<4; pub const DRM_EDID_MDELTA:u32=1<<5; pub const DRM_EDID_VRR_MAX_UPPER_MASK:u32=0xc0; pub const DRM_EDID_VRR_MAX_LOWER_MASK:u32=0xff; pub const DRM_EDID_VRR_MIN_MASK:u32=0x3f; pub const DRM_EDID_DSC_10BPC:u32=1; pub const DRM_EDID_DSC_12BPC:u32=1<<1; pub const DRM_EDID_DSC_16BPC:u32=1<<2; pub const DRM_EDID_DSC_ALL_BPP:u32=1<<3; pub const DRM_EDID_DSC_NATIVE_420:u32=1<<6; pub const DRM_EDID_DSC_1P2:u32=1<<7; pub const DRM_EDID_DSC_MAX_FRL_RATE_MASK:u32=0xf0; pub const DRM_EDID_DSC_MAX_SLICES:u32=0xf; pub const DRM_EDID_DSC_TOTAL_CHUNK_KBYTES:u32=0x3f;

#[repr(C, packed)] pub struct drm_edid_product_id { pub manufacturer_name:u16, pub product_code:u16, pub serial_number:u32, pub week_of_manufacture:u8, pub year_of_manufacture:u8 }
#[repr(C, packed)] pub struct edid { pub header:[u8;8], pub product_id:drm_edid_product_id, pub version:u8, pub revision:u8, pub input:u8, pub width_cm:u8, pub height_cm:u8, pub gamma:u8, pub features:u8, pub red_green_lo:u8, pub blue_white_lo:u8, pub red_x:u8, pub red_y:u8, pub green_x:u8, pub green_y:u8, pub blue_x:u8, pub blue_y:u8, pub white_x:u8, pub white_y:u8, pub established_timings:est_timings, pub standard_timings:[std_timing;8], pub detailed_timings:[detailed_timing;4], pub extensions:u8, pub checksum:u8 }
#[repr(C)] pub struct drm_edid_ident { pub panel_id:u32, pub name:*const i8 }
#[repr(C, packed)] pub struct cea_sad { pub format:u8, pub channels:u8, pub freq:u8, pub byte2:u8 }

/* External declarations supplied by the surrounding DRM translation. */
extern "C" {
    pub fn drm_edid_to_sad(edid:*const edid, sads:*mut *mut cea_sad) -> i32;
    pub fn drm_edid_header_is_valid(edid:*const core::ffi::c_void) -> i32;
    pub fn drm_edid_to_speaker_allocation(edid:*const edid,sadb:*mut *mut u8)->i32;
    pub fn drm_probe_ddc(adapter:*mut core::ffi::c_void)->bool;
    pub fn drm_get_edid(connector:*mut core::ffi::c_void,adapter:*mut core::ffi::c_void)->*mut edid;
    pub fn drm_edid_duplicate(e:*const edid)->*mut edid;
    pub fn drm_add_edid_modes(connector:*mut core::ffi::c_void,e:*mut edid)->i32;
    pub fn drm_edid_is_valid(e:*mut edid)->bool;
    pub fn drm_edid_get_monitor_name(e:*const edid,name:*mut i8,buflen:i32);
}

#[inline] pub unsafe fn drm_edid_decode_mfg_id(mfg_id:u16, vend:*mut i8) -> *const i8 { *vend = (b'@' + ((mfg_id>>10)&0x1f) as u8) as i8; *vend.add(1)=(b'@'+((mfg_id>>5)&0x1f) as u8) as i8; *vend.add(2)=(b'@'+(mfg_id&0x1f) as u8) as i8; *vend.add(3)=0; vend }
#[inline] pub const fn drm_edid_encode_panel_id(a:u32,b:u32,c:u32,product_id:u32)->u32 { (((a-('@' as u32))&0x1f)<<26)|(((b-('@' as u32))&0x1f)<<21)|(((c-('@' as u32))&0x1f)<<16)|(product_id&0xffff) }
#[inline] pub unsafe fn drm_edid_decode_panel_id(panel_id:u32,vend:*mut i8,product_id:*mut u16) { *product_id=(panel_id&0xffff) as u16; drm_edid_decode_mfg_id((panel_id>>16) as u16,vend); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
