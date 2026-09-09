/* SPDX-License-Identifier: MIT */
/* Rust translation of drm_mode.h. External integer aliases are supplied by dependencies. */

pub const DRM_CONNECTOR_NAME_LEN: usize = 32;
pub const DRM_DISPLAY_MODE_LEN: usize = 32;
pub const DRM_PROP_NAME_LEN: usize = 32;

pub const DRM_MODE_TYPE_BUILTIN: u32 = 1 << 0;
pub const DRM_MODE_TYPE_CLOCK_C: u32 = (1 << 1) | DRM_MODE_TYPE_BUILTIN;
pub const DRM_MODE_TYPE_CRTC_C: u32 = (1 << 2) | DRM_MODE_TYPE_BUILTIN;
pub const DRM_MODE_TYPE_PREFERRED: u32 = 1 << 3;
pub const DRM_MODE_TYPE_DEFAULT: u32 = 1 << 4;
pub const DRM_MODE_TYPE_USERDEF: u32 = 1 << 5;
pub const DRM_MODE_TYPE_DRIVER: u32 = 1 << 6;
pub const DRM_MODE_TYPE_ALL: u32 = DRM_MODE_TYPE_PREFERRED | DRM_MODE_TYPE_USERDEF | DRM_MODE_TYPE_DRIVER;

pub const DRM_MODE_FLAG_PHSYNC: u32 = 1<<0; pub const DRM_MODE_FLAG_NHSYNC: u32 = 1<<1;
pub const DRM_MODE_FLAG_PVSYNC: u32 = 1<<2; pub const DRM_MODE_FLAG_NVSYNC: u32 = 1<<3;
pub const DRM_MODE_FLAG_INTERLACE: u32 = 1<<4; pub const DRM_MODE_FLAG_DBLSCAN: u32 = 1<<5;
pub const DRM_MODE_FLAG_CSYNC: u32 = 1<<6; pub const DRM_MODE_FLAG_PCSYNC: u32 = 1<<7;
pub const DRM_MODE_FLAG_NCSYNC: u32 = 1<<8; pub const DRM_MODE_FLAG_HSKEW: u32 = 1<<9;
pub const DRM_MODE_FLAG_BCAST: u32 = 1<<10; pub const DRM_MODE_FLAG_PIXMUX: u32 = 1<<11;
pub const DRM_MODE_FLAG_DBLCLK: u32 = 1<<12; pub const DRM_MODE_FLAG_CLKDIV2: u32 = 1<<13;
pub const DRM_MODE_FLAG_3D_MASK: u32 = 0x1f<<14;
pub const DRM_MODE_FLAG_3D_NONE: u32 = 0<<14; pub const DRM_MODE_FLAG_3D_FRAME_PACKING: u32 = 1<<14;
pub const DRM_MODE_FLAG_3D_FIELD_ALTERNATIVE: u32 = 2<<14; pub const DRM_MODE_FLAG_3D_LINE_ALTERNATIVE: u32 = 3<<14;
pub const DRM_MODE_FLAG_3D_SIDE_BY_SIDE_FULL: u32 = 4<<14; pub const DRM_MODE_FLAG_3D_L_DEPTH: u32 = 5<<14;
pub const DRM_MODE_FLAG_3D_L_DEPTH_GFX_GFX_DEPTH: u32 = 6<<14; pub const DRM_MODE_FLAG_3D_TOP_AND_BOTTOM: u32 = 7<<14;
pub const DRM_MODE_FLAG_3D_SIDE_BY_SIDE_HALF: u32 = 8<<14;
pub const DRM_MODE_PICTURE_ASPECT_NONE: u32=0; pub const DRM_MODE_PICTURE_ASPECT_4_3:u32=1; pub const DRM_MODE_PICTURE_ASPECT_16_9:u32=2; pub const DRM_MODE_PICTURE_ASPECT_64_27:u32=3; pub const DRM_MODE_PICTURE_ASPECT_256_135:u32=4;
pub const DRM_MODE_FLAG_PIC_AR_MASK:u32=0x0f<<19;
pub const DRM_MODE_DPMS_ON:u32=0; pub const DRM_MODE_DPMS_STANDBY:u32=1; pub const DRM_MODE_DPMS_SUSPEND:u32=2; pub const DRM_MODE_DPMS_OFF:u32=3;
pub const DRM_MODE_SCALE_NONE:u32=0; pub const DRM_MODE_SCALE_FULLSCREEN:u32=1; pub const DRM_MODE_SCALE_CENTER:u32=2; pub const DRM_MODE_SCALE_ASPECT:u32=3;
pub const DRM_MODE_DITHERING_OFF:u32=0; pub const DRM_MODE_DITHERING_ON:u32=1; pub const DRM_MODE_DITHERING_AUTO:u32=2;
pub const DRM_MODE_DIRTY_OFF:u32=0; pub const DRM_MODE_DIRTY_ON:u32=1; pub const DRM_MODE_DIRTY_ANNOTATE:u32=2;
pub const DRM_MODE_LINK_STATUS_GOOD:u32=0; pub const DRM_MODE_LINK_STATUS_BAD:u32=1;
pub const DRM_MODE_ROTATE_0:u32=1<<0; pub const DRM_MODE_ROTATE_90:u32=1<<1; pub const DRM_MODE_ROTATE_180:u32=1<<2; pub const DRM_MODE_ROTATE_270:u32=1<<3;
pub const DRM_MODE_ROTATE_MASK:u32=DRM_MODE_ROTATE_0|DRM_MODE_ROTATE_90|DRM_MODE_ROTATE_180|DRM_MODE_ROTATE_270;
pub const DRM_MODE_REFLECT_X:u32=1<<4; pub const DRM_MODE_REFLECT_Y:u32=1<<5; pub const DRM_MODE_REFLECT_MASK:u32=DRM_MODE_REFLECT_X|DRM_MODE_REFLECT_Y;
pub const DRM_MODE_CONTENT_PROTECTION_UNDESIRED:u32=0; pub const DRM_MODE_CONTENT_PROTECTION_DESIRED:u32=1; pub const DRM_MODE_CONTENT_PROTECTION_ENABLED:u32=2;

#[repr(C)] #[derive(Copy,Clone)] pub struct drm_mode_modeinfo { pub clock:u32,pub hdisplay:u16,pub hsync_start:u16,pub hsync_end:u16,pub htotal:u16,pub hskew:u16,pub vdisplay:u16,pub vsync_start:u16,pub vsync_end:u16,pub vtotal:u16,pub vscan:u16,pub vrefresh:u32,pub flags:u32,pub type_:u32,pub name:[u8;DRM_DISPLAY_MODE_LEN] }
#[repr(C)] pub struct drm_mode_card_res { pub fb_id_ptr:u64,pub crtc_id_ptr:u64,pub connector_id_ptr:u64,pub encoder_id_ptr:u64,pub count_fbs:u32,pub count_crtcs:u32,pub count_connectors:u32,pub count_encoders:u32,pub min_width:u32,pub max_width:u32,pub min_height:u32,pub max_height:u32 }
#[repr(C)] pub struct drm_mode_crtc { pub set_connectors_ptr:u64,pub count_connectors:u32,pub crtc_id:u32,pub fb_id:u32,pub x:u32,pub y:u32,pub gamma_size:u32,pub mode_valid:u32,pub mode:drm_mode_modeinfo }
#[repr(C)] pub struct drm_mode_set_plane { pub plane_id:u32,pub crtc_id:u32,pub fb_id:u32,pub flags:u32,pub crtc_x:i32,pub crtc_y:i32,pub crtc_w:u32,pub crtc_h:u32,pub src_x:u32,pub src_y:u32,pub src_h:u32,pub src_w:u32 }
#[repr(C)] pub struct drm_mode_get_plane { pub plane_id:u32,pub crtc_id:u32,pub fb_id:u32,pub possible_crtcs:u32,pub gamma_size:u32,pub count_format_types:u32,pub format_type_ptr:u64 }
#[repr(C)] pub struct drm_mode_get_plane_res { pub plane_id_ptr:u64,pub count_planes:u32 }
#[repr(C)] pub struct drm_mode_get_encoder { pub encoder_id:u32,pub encoder_type:u32,pub crtc_id:u32,pub possible_crtcs:u32,pub possible_clones:u32 }
#[repr(C)] pub struct drm_mode_get_connector { pub encoders_ptr:u64,pub modes_ptr:u64,pub props_ptr:u64,pub prop_values_ptr:u64,pub count_modes:u32,pub count_props:u32,pub count_encoders:u32,pub encoder_id:u32,pub connector_id:u32,pub connector_type:u32,pub connector_type_id:u32,pub connection:u32,pub mm_width:u32,pub mm_height:u32,pub subpixel:u32,pub pad:u32 }
#[repr(C)] pub struct drm_mode_property_enum { pub value:u64,pub name:[u8;DRM_PROP_NAME_LEN] }
#[repr(C)] pub struct drm_mode_get_property { pub values_ptr:u64,pub enum_blob_ptr:u64,pub prop_id:u32,pub flags:u32,pub name:[u8;DRM_PROP_NAME_LEN],pub count_values:u32,pub count_enum_blobs:u32 }
#[repr(C)] pub struct drm_mode_connector_set_property { pub value:u64,pub prop_id:u32,pub connector_id:u32 }
#[repr(C)] pub struct drm_mode_obj_get_properties { pub props_ptr:u64,pub prop_values_ptr:u64,pub count_props:u32,pub obj_id:u32,pub obj_type:u32 }
#[repr(C)] pub struct drm_mode_obj_set_property { pub value:u64,pub prop_id:u32,pub obj_id:u32,pub obj_type:u32 }
#[repr(C)] pub struct drm_mode_get_blob { pub blob_id:u32,pub length:u32,pub data:u64 }
#[repr(C)] pub struct drm_mode_fb_cmd { pub fb_id:u32,pub width:u32,pub height:u32,pub pitch:u32,pub bpp:u32,pub depth:u32,pub handle:u32 }
#[repr(C)] pub struct drm_mode_fb_cmd2 { pub fb_id:u32,pub width:u32,pub height:u32,pub pixel_format:u32,pub flags:u32,pub handles:[u32;4],pub pitches:[u32;4],pub offsets:[u32;4],pub modifier:[u64;4] }
#[repr(C)] pub struct drm_mode_fb_dirty_cmd { pub fb_id:u32,pub flags:u32,pub color:u32,pub num_clips:u32,pub clips_ptr:u64 }
#[repr(C)] pub struct drm_mode_mode_cmd { pub connector_id:u32,pub mode:drm_mode_modeinfo }
#[repr(C)] pub struct drm_mode_cursor { pub flags:u32,pub crtc_id:u32,pub x:i32,pub y:i32,pub width:u32,pub height:u32,pub handle:u32 }
#[repr(C)] pub struct drm_mode_cursor2 { pub flags:u32,pub crtc_id:u32,pub x:i32,pub y:i32,pub width:u32,pub height:u32,pub handle:u32,pub hot_x:i32,pub hot_y:i32 }
#[repr(C)] pub struct drm_mode_crtc_lut { pub crtc_id:u32,pub gamma_size:u32,pub red:u64,pub green:u64,pub blue:u64 }
#[repr(C)] pub struct drm_color_ctm { pub matrix:[u64;9] }
#[repr(C)] pub struct drm_color_ctm_3x4 { pub matrix:[u64;12] }
#[repr(C)] pub struct drm_color_lut { pub red:u16,pub green:u16,pub blue:u16,pub reserved:u16 }
#[repr(C)] pub struct drm_color_lut32 { pub red:u32,pub green:u32,pub blue:u32,pub reserved:u32 }
#[repr(C)] pub struct drm_plane_size_hint { pub width:u16,pub height:u16 }
#[repr(C)] pub struct drm_mode_crtc_page_flip { pub crtc_id:u32,pub fb_id:u32,pub flags:u32,pub reserved:u32,pub user_data:u64 }
#[repr(C)] pub struct drm_mode_crtc_page_flip_target { pub crtc_id:u32,pub fb_id:u32,pub flags:u32,pub sequence:u32,pub user_data:u64 }
#[repr(C)] pub struct drm_mode_create_dumb { pub height:u32,pub width:u32,pub bpp:u32,pub flags:u32,pub handle:u32,pub pitch:u32,pub size:u64 }
#[repr(C)] pub struct drm_mode_map_dumb { pub handle:u32,pub pad:u32,pub offset:u64 }
#[repr(C)] pub struct drm_mode_destroy_dumb { pub handle:u32 }
#[repr(C)] pub struct drm_mode_atomic { pub flags:u32,pub count_objs:u32,pub objs_ptr:u64,pub count_props_ptr:u64,pub props_ptr:u64,pub prop_values_ptr:u64,pub reserved:u64,pub user_data:u64 }
#[repr(C)] pub struct drm_format_modifier_blob { pub version:u32,pub flags:u32,pub count_formats:u32,pub formats_offset:u32,pub count_modifiers:u32,pub modifiers_offset:u32 }
#[repr(C)] pub struct drm_format_modifier { pub formats:u64,pub offset:u32,pub pad:u32,pub modifier:u64 }
#[repr(C)] pub struct drm_mode_create_blob { pub data:u64,pub length:u32,pub blob_id:u32 }
#[repr(C)] pub struct drm_mode_destroy_blob { pub blob_id:u32 }
#[repr(C)] pub struct drm_mode_create_lease { pub object_ids:u64,pub object_count:u32,pub flags:u32,pub lessee_id:u32,pub fd:u32 }
#[repr(C)] pub struct drm_mode_list_lessees { pub count_lessees:u32,pub pad:u32,pub lessees_ptr:u64 }
#[repr(C)] pub struct drm_mode_get_lease { pub count_objects:u32,pub pad:u32,pub objects_ptr:u64 }
#[repr(C)] pub struct drm_mode_revoke_lease { pub lessee_id:u32 }
#[repr(C)] pub struct drm_mode_rect { pub x1:i32,pub y1:i32,pub x2:i32,pub y2:i32 }
#[repr(C)] pub struct drm_mode_closefb { pub fb_id:u32,pub pad:u32 }

#[repr(C)] #[derive(Copy,Clone)] pub struct drm_hdr_xy { pub x:u16,pub y:u16 }
#[repr(C)] pub struct hdr_metadata_infoframe { pub eotf:u8,pub metadata_type:u8,pub display_primaries:[drm_hdr_xy;3],pub white_point:drm_hdr_xy,pub max_display_mastering_luminance:u16,pub min_display_mastering_luminance:u16,pub max_cll:u16,pub max_fall:u16 }
#[repr(C)] pub union hdr_output_metadata_data { pub hdmi_metadata_type1:hdr_metadata_infoframe }
#[repr(C)] pub struct hdr_output_metadata { pub metadata_type:u32,pub hdmi_metadata_type1:hdr_output_metadata_data }

pub const DRM_MODE_PAGE_FLIP_EVENT:u32=0x01; pub const DRM_MODE_PAGE_FLIP_ASYNC:u32=0x02; pub const DRM_MODE_PAGE_FLIP_TARGET_ABSOLUTE:u32=0x4; pub const DRM_MODE_PAGE_FLIP_TARGET_RELATIVE:u32=0x8;
pub const DRM_MODE_PAGE_FLIP_TARGET:u32=DRM_MODE_PAGE_FLIP_TARGET_ABSOLUTE|DRM_MODE_PAGE_FLIP_TARGET_RELATIVE;
pub const DRM_MODE_ATOMIC_TEST_ONLY:u32=0x0100; pub const DRM_MODE_ATOMIC_NONBLOCK:u32=0x0200; pub const DRM_MODE_ATOMIC_ALLOW_MODESET:u32=0x0400;
pub const DRM_MODE_PRESENT_TOP_FIELD:u32=1<<0; pub const DRM_MODE_PRESENT_BOTTOM_FIELD:u32=1<<1;
pub const DRM_MODE_PROP_PENDING:u32=1<<0; pub const DRM_MODE_PROP_RANGE:u32=1<<1; pub const DRM_MODE_PROP_IMMUTABLE:u32=1<<2; pub const DRM_MODE_PROP_ENUM:u32=1<<3; pub const DRM_MODE_PROP_BLOB:u32=1<<4; pub const DRM_MODE_PROP_BITMASK:u32=1<<5;
pub const DRM_MODE_PROP_LEGACY_TYPE:u32=DRM_MODE_PROP_RANGE|DRM_MODE_PROP_ENUM|DRM_MODE_PROP_BLOB|DRM_MODE_PROP_BITMASK; pub const DRM_MODE_PROP_EXTENDED_TYPE:u32=0x0000ffc0;
pub const DRM_MODE_PROP_OBJECT:u32=1<<6; pub const DRM_MODE_PROP_SIGNED_RANGE:u32=2<<6; pub const DRM_MODE_PROP_ATOMIC:u32=0x80000000;
pub const DRM_MODE_OBJECT_CRTC:u32=0xcccccccc; pub const DRM_MODE_OBJECT_CONNECTOR:u32=0xc0c0c0c0; pub const DRM_MODE_OBJECT_ENCODER:u32=0xe0e0e0e0; pub const DRM_MODE_OBJECT_MODE:u32=0xdededede; pub const DRM_MODE_OBJECT_PROPERTY:u32=0xb0b0b0b0; pub const DRM_MODE_OBJECT_FB:u32=0xfbfbfbfb; pub const DRM_MODE_OBJECT_BLOB:u32=0xbbbbbbbb; pub const DRM_MODE_OBJECT_PLANE:u32=0xeeeeeeee; pub const DRM_MODE_OBJECT_COLOROP:u32=0xfafafafa; pub const DRM_MODE_OBJECT_ANY:u32=0;
pub const DRM_MODE_FB_INTERLACED:u32=1<<0; pub const DRM_MODE_FB_MODIFIERS:u32=1<<1; pub const DRM_MODE_FB_DIRTY_ANNOTATE_COPY:u32=1; pub const DRM_MODE_FB_DIRTY_ANNOTATE_FILL:u32=2; pub const DRM_MODE_FB_DIRTY_FLAGS:u32=3; pub const DRM_MODE_FB_DIRTY_MAX_CLIPS:u32=256;
pub const DRM_MODE_CURSOR_BO:u32=1; pub const DRM_MODE_CURSOR_MOVE:u32=2; pub const DRM_MODE_CURSOR_FLAGS:u32=3;
pub const FORMAT_BLOB_CURRENT:u32=1;

#[repr(i32)] pub enum drm_colorop_type { DRM_COLOROP_1D_CURVE, DRM_COLOROP_1D_LUT, DRM_COLOROP_CTM_3X4, DRM_COLOROP_MULTIPLIER, DRM_COLOROP_3D_LUT }
#[repr(i32)] pub enum drm_colorop_lut3d_interpolation_type { DRM_COLOROP_LUT3D_INTERPOLATION_TETRAHEDRAL }
#[repr(i32)] pub enum drm_colorop_lut1d_interpolation_type { DRM_COLOROP_LUT1D_INTERPOLATION_LINEAR }
#[repr(i32)] pub enum drm_mode_subconnector { DRM_MODE_SUBCONNECTOR_Automatic=0, DRM_MODE_SUBCONNECTOR_Unknown=0, DRM_MODE_SUBCONNECTOR_VGA=1, DRM_MODE_SUBCONNECTOR_DVID=3, DRM_MODE_SUBCONNECTOR_DVIA=4, DRM_MODE_SUBCONNECTOR_Composite=5, DRM_MODE_SUBCONNECTOR_SVIDEO=6, DRM_MODE_SUBCONNECTOR_Component=8, DRM_MODE_SUBCONNECTOR_SCART=9, DRM_MODE_SUBCONNECTOR_DisplayPort=10, DRM_MODE_SUBCONNECTOR_HDMIA=11, DRM_MODE_SUBCONNECTOR_Native=15, DRM_MODE_SUBCONNECTOR_Wireless=18 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
