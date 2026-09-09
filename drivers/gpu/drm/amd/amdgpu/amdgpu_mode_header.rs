/* Direct Rust translation of amdgpu_mode.h. External kernel/driver types are dependencies. */

#[repr(C)]
pub struct amdgpu_bo;
#[repr(C)] pub struct amdgpu_device;
#[repr(C)] pub struct amdgpu_encoder;
#[repr(C)] pub struct amdgpu_router;
#[repr(C)] pub struct amdgpu_hpd;
#[repr(C)] pub struct edid;
#[repr(C)] pub struct drm_edid;

#[macro_export] macro_rules! to_amdgpu_crtc { ($x:expr) => { container_of!($x, amdgpu_crtc, base) }; }
#[macro_export] macro_rules! to_amdgpu_connector { ($x:expr) => { container_of!($x, amdgpu_connector, base) }; }
#[macro_export] macro_rules! to_amdgpu_encoder { ($x:expr) => { container_of!($x, amdgpu_encoder, base) }; }
#[macro_export] macro_rules! to_amdgpu_framebuffer { ($x:expr) => { container_of!($x, amdgpu_framebuffer, base) }; }
#[macro_export] macro_rules! to_dm_plane_state { ($x:expr) => { container_of!($x, dm_plane_state, base) }; }

pub const AMDGPU_MAX_HPD_PINS: usize = 6;
pub const AMDGPU_MAX_CRTCS: usize = 6;
pub const AMDGPU_MAX_PLANES: usize = 6;
pub const AMDGPU_MAX_AFMT_BLOCKS: usize = 9;
pub const AMDGPU_HPD_CONNECT_INT_DELAY_IN_MS: u32 = 50;
pub const AMDGPU_HPD_DISCONNECT_INT_DELAY_IN_MS: u32 = 10;

#[repr(C)] #[derive(Copy, Clone)] pub enum amdgpu_rmx_type { RMX_OFF, RMX_FULL, RMX_CENTER, RMX_ASPECT }
#[repr(C)] #[derive(Copy, Clone)] pub enum amdgpu_underscan_type { UNDERSCAN_OFF, UNDERSCAN_ON, UNDERSCAN_AUTO }
#[repr(C)] #[derive(Copy, Clone)] pub enum amdgpu_hpd_id { AMDGPU_HPD_1=0, AMDGPU_HPD_2, AMDGPU_HPD_3, AMDGPU_HPD_4, AMDGPU_HPD_5, AMDGPU_HPD_6, AMDGPU_HPD_NONE=0xff }
#[repr(C)] #[derive(Copy, Clone)] pub enum amdgpu_crtc_irq { AMDGPU_CRTC_IRQ_VBLANK1=0, AMDGPU_CRTC_IRQ_VBLANK2, AMDGPU_CRTC_IRQ_VBLANK3, AMDGPU_CRTC_IRQ_VBLANK4, AMDGPU_CRTC_IRQ_VBLANK5, AMDGPU_CRTC_IRQ_VBLANK6, AMDGPU_CRTC_IRQ_VLINE1, AMDGPU_CRTC_IRQ_VLINE2, AMDGPU_CRTC_IRQ_VLINE3, AMDGPU_CRTC_IRQ_VLINE4, AMDGPU_CRTC_IRQ_VLINE5, AMDGPU_CRTC_IRQ_VLINE6, AMDGPU_CRTC_IRQ_NONE=0xff }
#[repr(C)] #[derive(Copy, Clone)] pub enum amdgpu_pageflip_irq { AMDGPU_PAGEFLIP_IRQ_D1=0, AMDGPU_PAGEFLIP_IRQ_D2, AMDGPU_PAGEFLIP_IRQ_D3, AMDGPU_PAGEFLIP_IRQ_D4, AMDGPU_PAGEFLIP_IRQ_D5, AMDGPU_PAGEFLIP_IRQ_D6, AMDGPU_PAGEFLIP_IRQ_NONE=0xff }
#[repr(C)] #[derive(Copy, Clone)] pub enum amdgpu_flip_status { AMDGPU_FLIP_NONE, AMDGPU_FLIP_PENDING, AMDGPU_FLIP_SUBMITTED }

pub const AMDGPU_MAX_I2C_BUS: usize = 16;
#[repr(C)] pub struct amdgpu_i2c_bus_rec { pub valid: bool, pub i2c_id:u8, pub hpd:amdgpu_hpd_id, pub hw_capable:bool, pub mm_i2c:bool, pub mask_clk_reg:u32, pub mask_data_reg:u32, pub a_clk_reg:u32, pub a_data_reg:u32, pub en_clk_reg:u32, pub en_data_reg:u32, pub y_clk_reg:u32, pub y_data_reg:u32, pub mask_clk_mask:u32, pub mask_data_mask:u32, pub a_clk_mask:u32, pub a_data_mask:u32, pub en_clk_mask:u32, pub en_data_mask:u32, pub y_clk_mask:u32, pub y_data_mask:u32 }
pub const AMDGPU_MAX_BIOS_CONNECTOR: usize = 16;
pub const AMDGPU_PLL_USE_BIOS_DIVS:u32=1<<0; pub const AMDGPU_PLL_NO_ODD_POST_DIV:u32=1<<1; pub const AMDGPU_PLL_USE_REF_DIV:u32=1<<2; pub const AMDGPU_PLL_LEGACY:u32=1<<3; pub const AMDGPU_PLL_PREFER_LOW_REF_DIV:u32=1<<4; pub const AMDGPU_PLL_PREFER_HIGH_REF_DIV:u32=1<<5; pub const AMDGPU_PLL_PREFER_LOW_FB_DIV:u32=1<<6; pub const AMDGPU_PLL_PREFER_HIGH_FB_DIV:u32=1<<7; pub const AMDGPU_PLL_PREFER_LOW_POST_DIV:u32=1<<8; pub const AMDGPU_PLL_PREFER_HIGH_POST_DIV:u32=1<<9; pub const AMDGPU_PLL_USE_FRAC_FB_DIV:u32=1<<10; pub const AMDGPU_PLL_PREFER_CLOSEST_LOWER:u32=1<<11; pub const AMDGPU_PLL_USE_POST_DIV:u32=1<<12; pub const AMDGPU_PLL_IS_LCD:u32=1<<13; pub const AMDGPU_PLL_PREFER_MINM_OVER_MAXP:u32=1<<14;
#[repr(C)] pub struct amdgpu_pll { pub reference_freq:u32,pub reference_div:u32,pub post_div:u32,pub pll_in_min:u32,pub pll_in_max:u32,pub pll_out_min:u32,pub pll_out_max:u32,pub lcd_pll_out_min:u32,pub lcd_pll_out_max:u32,pub best_vco:u32,pub min_ref_div:u32,pub max_ref_div:u32,pub min_post_div:u32,pub max_post_div:u32,pub min_feedback_div:u32,pub max_feedback_div:u32,pub min_frac_feedback_div:u32,pub max_frac_feedback_div:u32,pub flags:u32,pub id:u32 }

#[repr(C)] pub struct amdgpu_i2c_chan { pub adapter:i2c_adapter, pub dev:*mut drm_device, pub bit:i2c_algo_bit_data, pub rec:amdgpu_i2c_bus_rec, pub aux:drm_dp_aux, pub has_aux:bool, pub mutex:mutex }
#[repr(C)] pub struct amdgpu_afmt { pub enabled:bool,pub offset:i32,pub last_buffer_filled_status:bool,pub id:i32,pub pin:*mut amdgpu_audio_pin }
#[repr(C)] pub struct amdgpu_audio_pin { pub channels:i32,pub rate:i32,pub bits_per_sample:i32,pub status_bits:u8,pub category_code:u8,pub offset:u32,pub connected:bool,pub id:u32 }
#[repr(C)] pub struct amdgpu_audio { pub enabled:bool,pub pin:[amdgpu_audio_pin; AMDGPU_MAX_AFMT_BLOCKS],pub num_pins:i32 }

#[repr(C)] pub struct amdgpu_display_funcs { pub bandwidth_update:Option<unsafe extern "C" fn(*mut amdgpu_device)>, pub vblank_get_counter:Option<unsafe extern "C" fn(*mut amdgpu_device,i32)->u32>, pub backlight_set_level:Option<unsafe extern "C" fn(*mut amdgpu_encoder,u8)>, pub backlight_get_level:Option<unsafe extern "C" fn(*mut amdgpu_encoder)->u8>, pub hpd_sense:Option<unsafe extern "C" fn(*mut amdgpu_device,amdgpu_hpd_id)->bool>, pub hpd_set_polarity:Option<unsafe extern "C" fn(*mut amdgpu_device,amdgpu_hpd_id)>, pub hpd_get_gpio_reg:Option<unsafe extern "C" fn(*mut amdgpu_device)->u32>, pub page_flip:Option<unsafe extern "C" fn(*mut amdgpu_device,i32,u64,bool)>, pub page_flip_get_scanoutpos:Option<unsafe extern "C" fn(*mut amdgpu_device,i32,*mut u32,*mut u32)->i32>, pub add_encoder:Option<unsafe extern "C" fn(*mut amdgpu_device,u32,u32,u16)>, pub add_connector:Option<unsafe extern "C" fn(*mut amdgpu_device,u32,u32,i32,*mut amdgpu_i2c_bus_rec,u16,*mut amdgpu_hpd,*mut amdgpu_router)> }

#[repr(C)] pub struct amdgpu_framebuffer { pub base:drm_framebuffer,pub tiling_flags:u64,pub tmz_surface:bool,pub gfx12_dcc:bool,pub address:u64 }
#[repr(C)] pub struct amdgpu_mode_info {
 pub atom_context:*mut atom_context,pub atom_card_info:*mut card_info,pub mode_config_initialized:bool,
 pub crtcs:[*mut amdgpu_crtc;AMDGPU_MAX_CRTCS],pub planes:[*mut drm_plane;AMDGPU_MAX_PLANES],pub afmt:[*mut amdgpu_afmt;AMDGPU_MAX_AFMT_BLOCKS],
 pub coherent_mode_property:*mut drm_property,pub load_detect_property:*mut drm_property,pub underscan_property:*mut drm_property,pub underscan_hborder_property:*mut drm_property,pub underscan_vborder_property:*mut drm_property,pub audio_property:*mut drm_property,pub dither_property:*mut drm_property,pub abm_level_property:*mut drm_property,pub bios_hardcoded_edid:*const drm_edid,pub firmware_flags:u32,pub bl_encoder:*mut amdgpu_encoder,pub bl_level:u8,pub audio:amdgpu_audio,pub num_crtc:i32,pub num_hpd:i32,pub num_dig:i32,pub gpu_vm_support:bool,pub disp_priority:i32,pub funcs:*const amdgpu_display_funcs,pub plane_type:*const drm_plane_type,
 pub plane_degamma_lut_property:*mut drm_property,pub plane_degamma_lut_size_property:*mut drm_property,pub plane_degamma_tf_property:*mut drm_property,pub plane_hdr_mult_property:*mut drm_property,pub plane_ctm_property:*mut drm_property,pub plane_shaper_lut_property:*mut drm_property,pub plane_shaper_lut_size_property:*mut drm_property,pub plane_shaper_tf_property:*mut drm_property,pub plane_lut3d_property:*mut drm_property,pub plane_lut3d_size_property:*mut drm_property,pub plane_blend_lut_property:*mut drm_property,pub plane_blend_lut_size_property:*mut drm_property,pub plane_blend_tf_property:*mut drm_property,pub regamma_tf_property:*mut drm_property
}
pub const AMDGPU_MAX_BL_LEVEL:u32=0xFF;
#[repr(C)] pub struct amdgpu_backlight_privdata { pub encoder:*mut amdgpu_encoder,pub negative:u8 }
#[repr(C)] pub struct amdgpu_atom_ss { pub percentage:u16,pub percentage_divider:u16,pub type_:u8,pub step:u16,pub delay:u8,pub range:u8,pub refdiv:u8,pub rate:u16,pub amount:u16 }

#[repr(C)] pub struct amdgpu_crtc { pub base:drm_crtc,pub crtc_id:i32,pub enabled:bool,pub can_tile:bool,pub crtc_offset:u32,pub cursor_bo:*mut drm_gem_object,pub cursor_addr:u64,pub cursor_x:i32,pub cursor_y:i32,pub cursor_hot_x:i32,pub cursor_hot_y:i32,pub cursor_width:i32,pub cursor_height:i32,pub max_cursor_width:i32,pub max_cursor_height:i32,pub rmx_type:amdgpu_rmx_type,pub h_border:u8,pub v_border:u8,pub vsc:fixed20_12,pub hsc:fixed20_12,pub native_mode:drm_display_mode,pub pll_id:u32,pub pflip_works:*mut amdgpu_flip_work,pub pflip_status:amdgpu_flip_status,pub deferred_flip_completion:i32,pub dm_irq_params:dm_irq_params,pub ism:amdgpu_dm_ism,pub ss:amdgpu_atom_ss,pub ss_enabled:bool,pub adjusted_clock:u32,pub bpc:i32,pub pll_reference_div:u32,pub pll_post_div:u32,pub pll_flags:u32,pub encoder:*mut drm_encoder,pub connector:*mut drm_connector,pub line_time:u32,pub lb_vblank_lead_lines:u32,pub hw_mode:drm_display_mode,pub otg_inst:i32,pub event:*mut drm_pending_vblank_event,pub wb_pending:bool,pub wb_frame_done:bool,pub wb_enabled:bool,pub wb_conn:*mut drm_writeback_connector }

#[repr(C)] pub struct amdgpu_encoder_atom_dig { pub linkb:bool,pub coherent_mode:bool,pub dig_encoder:i32,pub lcd_misc:u32,pub panel_pwr_delay:u16,pub lcd_ss_id:u32,pub native_mode:drm_display_mode,pub bl_dev:*mut backlight_device,pub dpms_mode:i32,pub backlight_level:u8,pub panel_mode:i32,pub afmt:*mut amdgpu_afmt }
#[repr(C)] pub struct amdgpu_encoder { pub base:drm_encoder,pub encoder_enum:u32,pub encoder_id:u32,pub devices:u32,pub active_device:u32,pub flags:u32,pub pixel_clock:u32,pub rmx_type:amdgpu_rmx_type,pub underscan_type:amdgpu_underscan_type,pub underscan_hborder:u32,pub underscan_vborder:u32,pub native_mode:drm_display_mode,pub enc_priv:*mut core::ffi::c_void,pub audio_polling_active:i32,pub is_ext_encoder:bool,pub caps:u16 }
#[repr(C)] pub struct amdgpu_connector_atom_dig { pub dpcd:[u8;DP_RECEIVER_CAP_SIZE],pub downstream_ports:[u8;DP_MAX_DOWNSTREAM_PORTS],pub dp_sink_type:u8,pub dp_clock:i32,pub dp_lane_count:i32,pub edp_on:bool }
#[repr(C)] pub struct amdgpu_gpio_rec { pub valid:bool,pub id:u8,pub reg:u32,pub mask:u32,pub shift:u32 }
#[repr(C)] pub struct amdgpu_hpd { pub hpd:amdgpu_hpd_id,pub plugged_state:u8,pub gpio:amdgpu_gpio_rec }
#[repr(C)] pub struct amdgpu_router { pub router_id:u32,pub i2c_info:amdgpu_i2c_bus_rec,pub i2c_addr:u8,pub ddc_valid:bool,pub ddc_mux_type:u8,pub ddc_mux_control_pin:u8,pub ddc_mux_state:u8,pub cd_valid:bool,pub cd_mux_type:u8,pub cd_mux_control_pin:u8,pub cd_mux_state:u8 }
#[repr(C)] pub enum amdgpu_connector_audio { AMDGPU_AUDIO_DISABLE=0,AMDGPU_AUDIO_ENABLE=1,AMDGPU_AUDIO_AUTO=2 }
#[repr(C)] pub enum amdgpu_connector_dither { AMDGPU_FMT_DITHER_DISABLE=0,AMDGPU_FMT_DITHER_ENABLE=1 }
#[repr(C)] pub struct amdgpu_dm_dp_aux { pub aux:drm_dp_aux,pub ddc_service:*mut ddc_service }
#[repr(C)] pub struct amdgpu_i2c_adapter { pub base:i2c_adapter,pub ddc_service:*mut ddc_service,pub oem:bool }
#[macro_export] macro_rules! TO_DM_AUX { ($x:expr) => { container_of!($x, amdgpu_dm_dp_aux, aux) }; }
#[repr(C)] pub struct amdgpu_connector { pub base:drm_connector,pub connector_id:u32,pub devices:u32,pub ddc_bus:*mut amdgpu_i2c_chan,pub shared_ddc:bool,pub use_digital:bool,pub edid:*const drm_edid,pub con_priv:*mut core::ffi::c_void,pub dac_load_detect:bool,pub detected_by_load:bool,pub detected_hpd_without_ddc:bool,pub connector_object_id:u16,pub hpd:amdgpu_hpd,pub router:amdgpu_router,pub router_bus:*mut amdgpu_i2c_chan,pub audio:amdgpu_connector_audio,pub dither:amdgpu_connector_dither,pub pixelclock_for_modeset:usize }
#[repr(C)] pub struct amdgpu_mst_connector { pub base:amdgpu_connector,pub mst_mgr:drm_dp_mst_topology_mgr,pub dm_dp_aux:amdgpu_dm_dp_aux,pub mst_output_port:*mut drm_dp_mst_port,pub mst_root:*mut amdgpu_connector,pub is_mst_connector:bool,pub mst_encoder:*mut amdgpu_encoder }

#[macro_export] macro_rules! ENCODER_MODE_IS_DP { ($em:expr) => { ($em == ATOM_ENCODER_MODE_DP) || ($em == ATOM_ENCODER_MODE_DP_MST) }; }
pub const DRM_SCANOUTPOS_VALID:u32=1<<0; pub const DRM_SCANOUTPOS_IN_VBLANK:u32=1<<1; pub const DRM_SCANOUTPOS_ACCURATE:u32=1<<2; pub const USE_REAL_VBLANKSTART:u32=1<<30; pub const GET_DISTANCE_TO_VBLANKSTART:u32=1<<31;

extern "C" { pub fn amdgpu_link_encoder_connector(dev:*mut drm_device); pub fn amdgpu_get_connector_for_encoder(encoder:*mut drm_encoder)->*mut drm_connector; pub fn amdgpu_get_connector_for_encoder_init(encoder:*mut drm_encoder)->*mut drm_connector; pub fn amdgpu_dig_monitor_is_duallink(encoder:*mut drm_encoder,pixel_clock:u32)->bool; pub fn amdgpu_encoder_get_dp_bridge_encoder_id(encoder:*mut drm_encoder)->u16; pub fn amdgpu_get_external_encoder(encoder:*mut drm_encoder)->*mut drm_encoder; pub fn amdgpu_display_ddc_probe(c:*mut amdgpu_connector,use_aux:bool)->bool; pub fn amdgpu_encoder_set_active_device(encoder:*mut drm_encoder); pub fn amdgpu_display_get_crtc_scanoutpos(dev:*mut drm_device,pipe:u32,flags:u32,vpos:*mut i32,hpos:*mut i32,stime:*mut ktime_t,etime:*mut ktime_t,mode:*const drm_display_mode)->i32; pub fn amdgpufb_remove(dev:*mut drm_device,fb:*mut drm_framebuffer)->i32; pub fn amdgpu_enc_destroy(encoder:*mut drm_encoder); pub fn amdgpu_copy_fb(dev:*mut drm_device,dst_obj:*mut drm_gem_object); pub fn amdgpu_display_crtc_scaling_mode_fixup(crtc:*mut drm_crtc,mode:*const drm_display_mode,adjusted_mode:*mut drm_display_mode)->bool; pub fn amdgpu_panel_mode_fixup(encoder:*mut drm_encoder,adjusted_mode:*mut drm_display_mode); pub fn amdgpu_display_crtc_idx_to_irq_type(adev:*mut amdgpu_device,crtc:i32)->i32; pub fn amdgpu_crtc_get_scanout_position(crtc:*mut drm_crtc,in_vblank_irq:bool,vpos:*mut i32,hpos:*mut i32,stime:*mut ktime_t,etime:*mut ktime_t,mode:*const drm_display_mode)->bool; pub fn amdgpu_display_print_display_setup(dev:*mut drm_device); pub fn amdgpu_display_modeset_create_props(adev:*mut amdgpu_device)->i32; pub fn amdgpu_display_crtc_set_config(set:*mut drm_mode_set,ctx:*mut drm_modeset_acquire_ctx)->i32; pub fn amdgpu_display_crtc_page_flip_target(crtc:*mut drm_crtc,fb:*mut drm_framebuffer,event:*mut drm_pending_vblank_event,page_flip_flags:u32,target:u32,ctx:*mut drm_modeset_acquire_ctx)->i32; pub static amdgpu_mode_funcs:drm_mode_config_funcs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
