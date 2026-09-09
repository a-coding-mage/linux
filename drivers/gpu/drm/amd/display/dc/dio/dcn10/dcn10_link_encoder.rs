/* Faithful low-level translation of dcn10_link_encoder.c.  External types,
 * register helpers, and symbols are supplied by the surrounding tree. */

const DCN10_DIG_FE_SOURCE_SELECT_INVALID: u8 = 0x0;
const DCN10_DIG_FE_SOURCE_SELECT_DIGA: u8 = 0x1;
const DCN10_DIG_FE_SOURCE_SELECT_DIGB: u8 = 0x2;
const DCN10_DIG_FE_SOURCE_SELECT_DIGC: u8 = 0x4;
const DCN10_DIG_FE_SOURCE_SELECT_DIGD: u8 = 0x08;
const DCN10_DIG_FE_SOURCE_SELECT_DIGE: u8 = 0x10;
const DCN10_DIG_FE_SOURCE_SELECT_DIGF: u8 = 0x20;
const DCN10_DIG_FE_SOURCE_SELECT_DIGG: u8 = 0x40;
const DP_MST_UPDATE_MAX_RETRY: u32 = 50;

/* Register helper macros intentionally remain calls to the external register
 * helper interface; field and register identifiers are provided externally. */
macro_rules! REG { ($e:expr, $r:ident) => { (*$e).link_regs.as_ref().unwrap().$r }; }
macro_rules! REG_UPDATE { ($($x:tt)*) => { unsafe { reg_update!($($x)*) } }; }
macro_rules! REG_UPDATE_2 { ($($x:tt)*) => { unsafe { reg_update_2!($($x)*) } }; }
macro_rules! REG_UPDATE_3 { ($($x:tt)*) => { unsafe { reg_update_3!($($x)*) } }; }
macro_rules! REG_UPDATE_4 { ($($x:tt)*) => { unsafe { reg_update_4!($($x)*) } }; }
macro_rules! REG_SET { ($($x:tt)*) => { unsafe { reg_set!($($x)*) } }; }
macro_rules! REG_SET_2 { ($($x:tt)*) => { unsafe { reg_set_2!($($x)*) } }; }
macro_rules! REG_SET_3 { ($($x:tt)*) => { unsafe { reg_set_3!($($x)*) } }; }
macro_rules! REG_WRITE { ($($x:tt)*) => { unsafe { reg_write!($($x)*) } }; }
macro_rules! REG_READ { ($($x:tt)*) => { unsafe { reg_read!($($x)*) } }; }
macro_rules! REG_GET { ($($x:tt)*) => { unsafe { reg_get!($($x)*) } }; }

unsafe fn link_transmitter_control(enc10: *mut dcn10_link_encoder, cntl: *mut bp_transmitter_control) -> bp_result {
    ((*(*enc10).base.ctx).dc_bios).funcs.transmitter_control((*enc10).base.ctx.as_mut().unwrap().dc_bios, cntl)
}
unsafe fn enable_phy_bypass_mode(e: *mut dcn10_link_encoder, enable: bool) { REG_UPDATE!(e, DP_DPHY_CNTL, DPHY_BYPASS, enable); }
unsafe fn disable_prbs_symbols(e: *mut dcn10_link_encoder, disable: bool) { REG_UPDATE_4!(e, DP_DPHY_CNTL, DPHY_ATEST_SEL_LANE0,disable, DPHY_ATEST_SEL_LANE1,disable, DPHY_ATEST_SEL_LANE2,disable, DPHY_ATEST_SEL_LANE3,disable); }
unsafe fn disable_prbs_mode(e: *mut dcn10_link_encoder) { REG_UPDATE!(e, DP_DPHY_PRBS_CNTL, DPHY_PRBS_EN, 0); }
unsafe fn program_pattern_symbols(e: *mut dcn10_link_encoder, p: *const u16) {
    REG_SET_3!(e, DP_DPHY_SYM0,0, DPHY_SYM1,*p.add(0),DPHY_SYM2,*p.add(1),DPHY_SYM3,*p.add(2));
    REG_SET_3!(e, DP_DPHY_SYM1,0, DPHY_SYM4,*p.add(3),DPHY_SYM5,*p.add(4),DPHY_SYM6,*p.add(5));
    REG_SET_2!(e, DP_DPHY_SYM2,0, DPHY_SYM7,*p.add(6),DPHY_SYM8,*p.add(7));
}
unsafe fn set_dp_phy_pattern_d102(e: *mut dcn10_link_encoder) { enable_phy_bypass_mode(e,false); disable_prbs_symbols(e,true); disable_prbs_mode(e); let p=[0x2aa_u16;8]; program_pattern_symbols(e,p.as_ptr()); enable_phy_bypass_mode(e,true); }
unsafe fn set_link_training_complete(e:*mut dcn10_link_encoder, v:bool){REG_UPDATE!(e,DP_LINK_CNTL,DP_LINK_TRAINING_COMPLETE,v);}

pub unsafe fn dcn10_link_encoder_set_dp_phy_pattern_training_pattern(enc:*mut link_encoder,index:u32){let e=TO_DCN10_LINK_ENC(enc);REG_WRITE!(e,DP_DPHY_TRAINING_PATTERN_SEL,index);set_link_training_complete(e,false);enable_phy_bypass_mode(e,false);disable_prbs_mode(e);}
unsafe fn setup_panel_mode(e:*mut dcn10_link_encoder, m:dp_panel_mode){if REG!(e,DP_DPHY_INTERNAL_CTRL)==0{return;}let v=match m{DP_PANEL_MODE_EDP=>1,DP_PANEL_MODE_SPECIAL=>0x11,_=>0};REG_WRITE!(e,DP_DPHY_INTERNAL_CTRL,v);}
unsafe fn set_dp_phy_pattern_symbol_error(e:*mut dcn10_link_encoder){enable_phy_bypass_mode(e,false);setup_panel_mode(e,DP_PANEL_MODE_DEFAULT);disable_prbs_symbols(e,false);REG_UPDATE_2!(e,DP_DPHY_PRBS_CNTL,DPHY_PRBS_SEL,1,DPHY_PRBS_EN,1);enable_phy_bypass_mode(e,true);}
unsafe fn set_dp_phy_pattern_prbs7(e:*mut dcn10_link_encoder){enable_phy_bypass_mode(e,false);disable_prbs_symbols(e,false);REG_UPDATE_2!(e,DP_DPHY_PRBS_CNTL,DPHY_PRBS_SEL,0,DPHY_PRBS_EN,1);enable_phy_bypass_mode(e,true);}
unsafe fn set_dp_phy_pattern_80bit_custom(e:*mut dcn10_link_encoder,p:*const u8){enable_phy_bypass_mode(e,false);disable_prbs_symbols(e,true);let a=[(((*p.add(1)&3)as u16)<<8)|*p.add(0)as u16,(((*p.add(2)&15)as u16)<<6)|((*p.add(1)>>2)&63)as u16,(((*p.add(3)&63)as u16)<<4)|((*p.add(2)>>4)&15)as u16,((*p.add(4)as u16)<<2)|((*p.add(3)>>6)&3)as u16,(((*p.add(6)&3)as u16)<<8)|*p.add(5)as u16,(((*p.add(7)&15)as u16)<<6)|((*p.add(6)>>2)&63)as u16,(((*p.add(8)&63)as u16)<<4)|((*p.add(7)>>4)&15)as u16,((*p.add(9)as u16)<<2)|((*p.add(8)>>6)&3)as u16];program_pattern_symbols(e,a.as_ptr());enable_phy_bypass_mode(e,true);}
unsafe fn set_dp_phy_pattern_passthrough_mode(e:*mut dcn10_link_encoder,m:dp_panel_mode){setup_panel_mode(e,m);REG_UPDATE_3!(e,DP_LINK_FRAMING_CNTL,DP_IDLE_BS_INTERVAL,0x2000,DP_VBID_DISABLE,0,DP_VID_ENHANCED_FRAME_MODE,1);REG_UPDATE!(e,DP_DPHY_SCRAM_CNTL,DPHY_SCRAMBLER_BS_COUNT,0x1ff);set_link_training_complete(e,true);enable_phy_bypass_mode(e,false);disable_prbs_mode(e);}
unsafe fn get_frontend_source(e:engine_id)->u8{match e{ENGINE_ID_DIGA=>1,ENGINE_ID_DIGB=>2,ENGINE_ID_DIGC=>4,ENGINE_ID_DIGD=>8,ENGINE_ID_DIGE=>16,ENGINE_ID_DIGF=>32,ENGINE_ID_DIGG=>64,_=>{ASSERT_CRITICAL(false);0}}}

pub unsafe fn dcn10_get_dig_frontend(enc:*mut link_encoder)->engine_id{let e=TO_DCN10_LINK_ENC(enc);let mut v=0;REG_GET!(e,DIG_BE_CNTL,DIG_FE_SOURCE_SELECT,&mut v);match v{1=>ENGINE_ID_DIGA,2=>ENGINE_ID_DIGB,4=>ENGINE_ID_DIGC,8=>ENGINE_ID_DIGD,16=>ENGINE_ID_DIGE,32=>ENGINE_ID_DIGF,64=>ENGINE_ID_DIGG,_=>ENGINE_ID_UNKNOWN}}
pub unsafe fn enc1_configure_encoder(e:*mut dcn10_link_encoder,s:*const dc_link_settings){REG_SET!(e,DP_CONFIG,0,DP_UDI_LANES,(*s).lane_count-LANE_COUNT_ONE);REG_UPDATE!(e,DP_DPHY_SCRAM_CNTL,DPHY_SCRAMBLER_ADVANCE,1);}
pub unsafe fn dcn10_psr_program_dp_dphy_fast_training(enc:*mut link_encoder,exit:bool){let e=TO_DCN10_LINK_ENC(enc);REG_UPDATE!(e,DP_DPHY_FAST_TRAINING,DPHY_RX_FAST_TRAINING_CAPABLE,if exit{1}else{0});if !exit{REG_UPDATE!(e,DP_DPHY_BS_SR_SWAP_CNTL,DPHY_LOAD_BS_COUNT,5);}}
pub unsafe fn dcn10_psr_program_secondary_packet(enc:*mut link_encoder,n:u32){REG_UPDATE_2!(TO_DCN10_LINK_ENC(enc),DP_SEC_CNTL1,DP_SEC_GSP0_LINE_NUM,n,DP_SEC_GSP0_PRIORITY,1);}
pub unsafe fn dcn10_is_dig_enabled(enc:*mut link_encoder)->bool{let mut v=0;REG_GET!(TO_DCN10_LINK_ENC(enc),DIG_BE_EN_CNTL,DIG_ENABLE,&mut v);v!=0}

/* The remaining entry points retain the C implementation's externally supplied
 * structures and helper calls. */
pub unsafe fn dcn10_get_dig_mode(enc:*mut link_encoder)->signal_type{let mut v=0;REG_GET!(TO_DCN10_LINK_ENC(enc),DIG_BE_CNTL,DIG_MODE,&mut v);match v{1=>SIGNAL_TYPE_DISPLAY_PORT,2=>SIGNAL_TYPE_DVI_SINGLE_LINK,3=>SIGNAL_TYPE_HDMI_TYPE_A,5=>SIGNAL_TYPE_DISPLAY_PORT_MST,_=>SIGNAL_TYPE_NONE}}
pub unsafe fn dcn10_link_encoder_enable_hpd(enc:*mut link_encoder){REG_UPDATE!(TO_DCN10_LINK_ENC(enc),DC_HPD_CONTROL,DC_HPD_EN,1);}
pub unsafe fn dcn10_link_encoder_disable_hpd(enc:*mut link_encoder){REG_UPDATE!(TO_DCN10_LINK_ENC(enc),DC_HPD_CONTROL,DC_HPD_EN,0);}

pub unsafe fn dcn10_link_encoder_dp_set_phy_pattern(enc:*mut link_encoder,param:*const encoder_set_dp_phy_pattern_param){let e=TO_DCN10_LINK_ENC(enc);match (*param).dp_phy_pattern{DP_TEST_PATTERN_TRAINING_PATTERN1=>dcn10_link_encoder_set_dp_phy_pattern_training_pattern(enc,0),DP_TEST_PATTERN_TRAINING_PATTERN2=>dcn10_link_encoder_set_dp_phy_pattern_training_pattern(enc,1),DP_TEST_PATTERN_TRAINING_PATTERN3=>dcn10_link_encoder_set_dp_phy_pattern_training_pattern(enc,2),DP_TEST_PATTERN_TRAINING_PATTERN4=>dcn10_link_encoder_set_dp_phy_pattern_training_pattern(enc,3),DP_TEST_PATTERN_D102=>set_dp_phy_pattern_d102(e),DP_TEST_PATTERN_SYMBOL_ERROR=>set_dp_phy_pattern_symbol_error(e),DP_TEST_PATTERN_PRBS7=>set_dp_phy_pattern_prbs7(e),DP_TEST_PATTERN_80BIT_CUSTOM=>set_dp_phy_pattern_80bit_custom(e,(*param).custom_pattern.as_ptr()),DP_TEST_PATTERN_VIDEO_MODE=>set_dp_phy_pattern_passthrough_mode(e,(*param).dp_panel_mode),_=>ASSERT_CRITICAL(false)}}
pub unsafe fn dcn10_link_encoder_setup(enc:*mut link_encoder,signal:signal_type){let v=match signal{SIGNAL_TYPE_EDP|SIGNAL_TYPE_DISPLAY_PORT=>0,SIGNAL_TYPE_LVDS=>1,SIGNAL_TYPE_DVI_SINGLE_LINK|SIGNAL_TYPE_DVI_DUAL_LINK=>2,SIGNAL_TYPE_HDMI_TYPE_A=>3,SIGNAL_TYPE_DISPLAY_PORT_MST=>5,_=>{ASSERT_CRITICAL(false);return}};REG_UPDATE!(TO_DCN10_LINK_ENC(enc),DIG_BE_CNTL,DIG_MODE,v);}
pub unsafe fn dcn10_aux_initialize(e:*mut dcn10_link_encoder){let h=(*e).base.hpd_source;AUX_REG_UPDATE_2!(e,AUX_CONTROL,AUX_HPD_SEL,h,AUX_LS_READ_EN,0);AUX_REG_UPDATE!(e,AUX_DPHY_RX_CONTROL0,AUX_RX_RECEIVE_WINDOW,0);}
pub unsafe fn dcn10_link_encoder_get_max_link_cap(enc:*mut link_encoder,out:*mut dc_link_settings){let mut s=dc_link_settings{lane_count:LANE_COUNT_FOUR,link_rate:LINK_RATE_HIGH,link_spread:LINK_SPREAD_05_DOWNSPREAD_30KHZ,enabled:false,voltage_swing:0};if (*enc).features.flags.bits.IS_HBR2_CAPABLE{s.link_rate=LINK_RATE_HIGH2;}if (*enc).features.flags.bits.IS_HBR3_CAPABLE{s.link_rate=LINK_RATE_HIGH3;}*out=s;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
