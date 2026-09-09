/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * rc-map.h - define RC map names used by RC drivers
 *
 * Copyright (c) 2010 by Mauro Carvalho Chehab
 */

// Dependency intent: linux/input.h and uapi/linux/lirc.h provide keycodes,
// rc_proto, list_head, spinlock_t, and the RC_PROTO_* constants.

pub const RC_PROTO_BIT_NONE: u64 = 0;
pub const RC_PROTO_BIT_UNKNOWN: u64 = 1u64 << RC_PROTO_UNKNOWN;
pub const RC_PROTO_BIT_OTHER: u64 = 1u64 << RC_PROTO_OTHER;
pub const RC_PROTO_BIT_RC5: u64 = 1u64 << RC_PROTO_RC5;
pub const RC_PROTO_BIT_RC5X_20: u64 = 1u64 << RC_PROTO_RC5X_20;
pub const RC_PROTO_BIT_RC5_SZ: u64 = 1u64 << RC_PROTO_RC5_SZ;
pub const RC_PROTO_BIT_JVC: u64 = 1u64 << RC_PROTO_JVC;
pub const RC_PROTO_BIT_SONY12: u64 = 1u64 << RC_PROTO_SONY12;
pub const RC_PROTO_BIT_SONY15: u64 = 1u64 << RC_PROTO_SONY15;
pub const RC_PROTO_BIT_SONY20: u64 = 1u64 << RC_PROTO_SONY20;
pub const RC_PROTO_BIT_NEC: u64 = 1u64 << RC_PROTO_NEC;
pub const RC_PROTO_BIT_NECX: u64 = 1u64 << RC_PROTO_NECX;
pub const RC_PROTO_BIT_NEC32: u64 = 1u64 << RC_PROTO_NEC32;
pub const RC_PROTO_BIT_SANYO: u64 = 1u64 << RC_PROTO_SANYO;
pub const RC_PROTO_BIT_MCIR2_KBD: u64 = 1u64 << RC_PROTO_MCIR2_KBD;
pub const RC_PROTO_BIT_MCIR2_MSE: u64 = 1u64 << RC_PROTO_MCIR2_MSE;
pub const RC_PROTO_BIT_RC6_0: u64 = 1u64 << RC_PROTO_RC6_0;
pub const RC_PROTO_BIT_RC6_6A_20: u64 = 1u64 << RC_PROTO_RC6_6A_20;
pub const RC_PROTO_BIT_RC6_6A_24: u64 = 1u64 << RC_PROTO_RC6_6A_24;
pub const RC_PROTO_BIT_RC6_6A_32: u64 = 1u64 << RC_PROTO_RC6_6A_32;
pub const RC_PROTO_BIT_RC6_MCE: u64 = 1u64 << RC_PROTO_RC6_MCE;
pub const RC_PROTO_BIT_SHARP: u64 = 1u64 << RC_PROTO_SHARP;
pub const RC_PROTO_BIT_XMP: u64 = 1u64 << RC_PROTO_XMP;
pub const RC_PROTO_BIT_CEC: u64 = 1u64 << RC_PROTO_CEC;
pub const RC_PROTO_BIT_IMON: u64 = 1u64 << RC_PROTO_IMON;
pub const RC_PROTO_BIT_RCMM12: u64 = 1u64 << RC_PROTO_RCMM12;
pub const RC_PROTO_BIT_RCMM24: u64 = 1u64 << RC_PROTO_RCMM24;
pub const RC_PROTO_BIT_RCMM32: u64 = 1u64 << RC_PROTO_RCMM32;
pub const RC_PROTO_BIT_XBOX_DVD: u64 = 1u64 << RC_PROTO_XBOX_DVD;

// Build-time decoder configuration is supplied by the surrounding kernel build.
#[cfg(CONFIG_IR_RC5_DECODER)] pub const __RC_PROTO_RC5_CODEC: u64 = RC_PROTO_BIT_RC5 | RC_PROTO_BIT_RC5X_20 | RC_PROTO_BIT_RC5_SZ;
#[cfg(not(CONFIG_IR_RC5_DECODER))] pub const __RC_PROTO_RC5_CODEC: u64 = 0;
#[cfg(CONFIG_IR_JVC_DECODER)] pub const __RC_PROTO_JVC_CODEC: u64 = RC_PROTO_BIT_JVC;
#[cfg(not(CONFIG_IR_JVC_DECODER))] pub const __RC_PROTO_JVC_CODEC: u64 = 0;
#[cfg(CONFIG_IR_SONY_DECODER)] pub const __RC_PROTO_SONY_CODEC: u64 = RC_PROTO_BIT_SONY12 | RC_PROTO_BIT_SONY15 | RC_PROTO_BIT_SONY20;
#[cfg(not(CONFIG_IR_SONY_DECODER))] pub const __RC_PROTO_SONY_CODEC: u64 = 0;
#[cfg(CONFIG_IR_NEC_DECODER)] pub const __RC_PROTO_NEC_CODEC: u64 = RC_PROTO_BIT_NEC | RC_PROTO_BIT_NECX | RC_PROTO_BIT_NEC32;
#[cfg(not(CONFIG_IR_NEC_DECODER))] pub const __RC_PROTO_NEC_CODEC: u64 = 0;
#[cfg(CONFIG_IR_SANYO_DECODER)] pub const __RC_PROTO_SANYO_CODEC: u64 = RC_PROTO_BIT_SANYO;
#[cfg(not(CONFIG_IR_SANYO_DECODER))] pub const __RC_PROTO_SANYO_CODEC: u64 = 0;
#[cfg(CONFIG_IR_MCE_KBD_DECODER)] pub const __RC_PROTO_MCE_KBD_CODEC: u64 = RC_PROTO_BIT_MCIR2_KBD | RC_PROTO_BIT_MCIR2_MSE;
#[cfg(not(CONFIG_IR_MCE_KBD_DECODER))] pub const __RC_PROTO_MCE_KBD_CODEC: u64 = 0;
#[cfg(CONFIG_IR_RC6_DECODER)] pub const __RC_PROTO_RC6_CODEC: u64 = RC_PROTO_BIT_RC6_0 | RC_PROTO_BIT_RC6_6A_20 | RC_PROTO_BIT_RC6_6A_24 | RC_PROTO_BIT_RC6_6A_32 | RC_PROTO_BIT_RC6_MCE;
#[cfg(not(CONFIG_IR_RC6_DECODER))] pub const __RC_PROTO_RC6_CODEC: u64 = 0;
#[cfg(CONFIG_IR_SHARP_DECODER)] pub const __RC_PROTO_SHARP_CODEC: u64 = RC_PROTO_BIT_SHARP;
#[cfg(not(CONFIG_IR_SHARP_DECODER))] pub const __RC_PROTO_SHARP_CODEC: u64 = 0;
#[cfg(CONFIG_IR_XMP_DECODER)] pub const __RC_PROTO_XMP_CODEC: u64 = RC_PROTO_BIT_XMP;
#[cfg(not(CONFIG_IR_XMP_DECODER))] pub const __RC_PROTO_XMP_CODEC: u64 = 0;
#[cfg(CONFIG_IR_IMON_DECODER)] pub const __RC_PROTO_IMON_CODEC: u64 = RC_PROTO_BIT_IMON;
#[cfg(not(CONFIG_IR_IMON_DECODER))] pub const __RC_PROTO_IMON_CODEC: u64 = 0;
#[cfg(CONFIG_IR_RCMM_DECODER)] pub const __RC_PROTO_RCMM_CODEC: u64 = RC_PROTO_BIT_RCMM12 | RC_PROTO_BIT_RCMM24 | RC_PROTO_BIT_RCMM32;
#[cfg(not(CONFIG_IR_RCMM_DECODER))] pub const __RC_PROTO_RCMM_CODEC: u64 = 0;

pub const RC_PROTO_BIT_ALL_IR_DECODER: u64 = __RC_PROTO_RC5_CODEC | __RC_PROTO_JVC_CODEC | __RC_PROTO_SONY_CODEC | __RC_PROTO_NEC_CODEC | __RC_PROTO_SANYO_CODEC | __RC_PROTO_MCE_KBD_CODEC | __RC_PROTO_RC6_CODEC | __RC_PROTO_SHARP_CODEC | __RC_PROTO_XMP_CODEC | __RC_PROTO_IMON_CODEC | __RC_PROTO_RCMM_CODEC;
pub const RC_PROTO_BIT_ALL_IR_ENCODER: u64 = RC_PROTO_BIT_ALL_IR_DECODER;

#[macro_export] macro_rules! RC_SCANCODE_UNKNOWN { ($x:expr) => { $x }; }
#[macro_export] macro_rules! RC_SCANCODE_OTHER { ($x:expr) => { $x }; }
#[macro_export] macro_rules! RC_SCANCODE_NEC { ($addr:expr, $cmd:expr) => { (($addr << 8) | $cmd) }; }
#[macro_export] macro_rules! RC_SCANCODE_NECX { ($addr:expr, $cmd:expr) => { (($addr << 8) | $cmd) }; }
#[macro_export] macro_rules! RC_SCANCODE_NEC32 { ($data:expr) => { ($data & 0xffff_ffff) }; }
#[macro_export] macro_rules! RC_SCANCODE_RC5 { ($sys:expr, $cmd:expr) => { (($sys << 8) | $cmd) }; }
#[macro_export] macro_rules! RC_SCANCODE_RC5_SZ { ($sys:expr, $cmd:expr) => { (($sys << 8) | $cmd) }; }
#[macro_export] macro_rules! RC_SCANCODE_RC6_0 { ($sys:expr, $cmd:expr) => { (($sys << 8) | $cmd) }; }
#[macro_export] macro_rules! RC_SCANCODE_RC6_6A { ($vendor:expr, $sys:expr, $cmd:expr) => { (($vendor << 16) | ($sys << 8) | $cmd) }; }

#[repr(C)]
pub struct rc_map_table { pub scancode: u64, pub keycode: u32 }

#[repr(C)]
pub struct rc_map {
    pub scan: *mut rc_map_table,
    pub size: core::ffi::c_uint,
    pub len: core::ffi::c_uint,
    pub alloc: core::ffi::c_uint,
    pub rc_proto: rc_proto,
    pub name: *const core::ffi::c_char,
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct rc_map_list { pub list: list_head, pub map: rc_map }

#[cfg(CONFIG_MEDIA_CEC_RC)]
extern "C" { pub static mut cec_map: rc_map_list; }

extern "C" {
    pub fn rc_map_register(map: *mut rc_map_list) -> core::ffi::c_int;
    pub fn rc_map_unregister(map: *mut rc_map_list);
    pub fn rc_map_get(name: *const core::ffi::c_char) -> *mut rc_map;
}

// Names of the several keytables defined in-kernel
pub const RC_MAP_ADSTECH_DVB_T_PCI: &str = "rc-adstech-dvb-t-pci";
pub const RC_MAP_ALINK_DTU_M: &str = "rc-alink-dtu-m";
pub const RC_MAP_ANYSEE: &str = "rc-anysee";
pub const RC_MAP_APAC_VIEWCOMP: &str = "rc-apac-viewcomp";
pub const RC_MAP_ASTROMETA_T2HYBRID: &str = "rc-astrometa-t2hybrid";
pub const RC_MAP_ASUS_PC39: &str = "rc-asus-pc39";
pub const RC_MAP_ASUS_PS3_100: &str = "rc-asus-ps3-100";
pub const RC_MAP_ATI_TV_WONDER_HD_600: &str = "rc-ati-tv-wonder-hd-600";
pub const RC_MAP_ATI_X10: &str = "rc-ati-x10";
pub const RC_MAP_AVERMEDIA: &str = "rc-avermedia";
pub const RC_MAP_AVERMEDIA_A16D: &str = "rc-avermedia-a16d";
pub const RC_MAP_AVERMEDIA_CARDBUS: &str = "rc-avermedia-cardbus";
pub const RC_MAP_AVERMEDIA_DVBT: &str = "rc-avermedia-dvbt";
pub const RC_MAP_AVERMEDIA_M135A: &str = "rc-avermedia-m135a";
pub const RC_MAP_AVERMEDIA_M733A_RM_K6: &str = "rc-avermedia-m733a-rm-k6";
pub const RC_MAP_AVERMEDIA_RM_KS: &str = "rc-avermedia-rm-ks";
pub const RC_MAP_AVERTV_303: &str = "rc-avertv-303";
pub const RC_MAP_AZUREWAVE_AD_TU700: &str = "rc-azurewave-ad-tu700";
pub const RC_MAP_BEELINK_GS1: &str = "rc-beelink-gs1";
pub const RC_MAP_BEELINK_MXIII: &str = "rc-beelink-mxiii";
pub const RC_MAP_BEHOLD: &str = "rc-behold";
pub const RC_MAP_BEHOLD_COLUMBUS: &str = "rc-behold-columbus";
pub const RC_MAP_BUDGET_CI_OLD: &str = "rc-budget-ci-old";
pub const RC_MAP_CEC: &str = "rc-cec";
pub const RC_MAP_CINERGY: &str = "rc-cinergy";
pub const RC_MAP_CINERGY_1400: &str = "rc-cinergy-1400";
pub const RC_MAP_CT_90405: &str = "rc-ct-90405";
pub const RC_MAP_D680_DMB: &str = "rc-d680-dmb";
pub const RC_MAP_DELOCK_61959: &str = "rc-delock-61959";
pub const RC_MAP_DIB0700_NEC_TABLE: &str = "rc-dib0700-nec";
pub const RC_MAP_DIB0700_RC5_TABLE: &str = "rc-dib0700-rc5";
pub const RC_MAP_DIGITALNOW_TINYTWIN: &str = "rc-digitalnow-tinytwin";
pub const RC_MAP_DIGITTRADE: &str = "rc-digittrade";
pub const RC_MAP_DM1105_NEC: &str = "rc-dm1105-nec";
pub const RC_MAP_DNTV_LIVE_DVB_T: &str = "rc-dntv-live-dvb-t";
pub const RC_MAP_DNTV_LIVE_DVBT_PRO: &str = "rc-dntv-live-dvbt-pro";
pub const RC_MAP_DREAMBOX: &str = "rc-dreambox";
pub const RC_MAP_DTT200U: &str = "rc-dtt200u";
pub const RC_MAP_DVBSKY: &str = "rc-dvbsky";
pub const RC_MAP_DVICO_MCE: &str = "rc-dvico-mce";
pub const RC_MAP_DVICO_PORTABLE: &str = "rc-dvico-portable";
pub const RC_MAP_EMPTY: &str = "rc-empty";
pub const RC_MAP_EM_TERRATEC: &str = "rc-em-terratec";
pub const RC_MAP_ENCORE_ENLTV: &str = "rc-encore-enltv";
pub const RC_MAP_ENCORE_ENLTV2: &str = "rc-encore-enltv2";
pub const RC_MAP_ENCORE_ENLTV_FM53: &str = "rc-encore-enltv-fm53";
pub const RC_MAP_EVGA_INDTUBE: &str = "rc-evga-indtube";
pub const RC_MAP_EZTV: &str = "rc-eztv";
pub const RC_MAP_FLYDVB: &str = "rc-flydvb";
pub const RC_MAP_FLYVIDEO: &str = "rc-flyvideo";
pub const RC_MAP_FUSIONHDTV_MCE: &str = "rc-fusionhdtv-mce";
pub const RC_MAP_GADMEI_RM008Z: &str = "rc-gadmei-rm008z";
pub const RC_MAP_GEEKBOX: &str = "rc-geekbox";
pub const RC_MAP_GENIUS_TVGO_A11MCE: &str = "rc-genius-tvgo-a11mce";
pub const RC_MAP_GOTVIEW7135: &str = "rc-gotview7135";
pub const RC_MAP_HAUPPAUGE: &str = "rc-hauppauge";
pub const RC_MAP_HISI_POPLAR: &str = "rc-hisi-poplar";
pub const RC_MAP_HISI_TV_DEMO: &str = "rc-hisi-tv-demo";
pub const RC_MAP_IMON_MCE: &str = "rc-imon-mce";
pub const RC_MAP_IMON_PAD: &str = "rc-imon-pad";
pub const RC_MAP_IMON_RSC: &str = "rc-imon-rsc";
pub const RC_MAP_IODATA_BCTV7E: &str = "rc-iodata-bctv7e";
pub const RC_MAP_IT913X_V1: &str = "rc-it913x-v1";
pub const RC_MAP_IT913X_V2: &str = "rc-it913x-v2";
pub const RC_MAP_KAIOMY: &str = "rc-kaiomy";
pub const RC_MAP_KHADAS: &str = "rc-khadas";
pub const RC_MAP_KHAMSIN: &str = "rc-khamsin";
pub const RC_MAP_KWORLD_315U: &str = "rc-kworld-315u";
pub const RC_MAP_KWORLD_PC150U: &str = "rc-kworld-pc150u";
pub const RC_MAP_KWORLD_PLUS_TV_ANALOG: &str = "rc-kworld-plus-tv-analog";
pub const RC_MAP_LEADTEK_Y04G0051: &str = "rc-leadtek-y04g0051";
pub const RC_MAP_LME2510: &str = "rc-lme2510";
pub const RC_MAP_MANLI: &str = "rc-manli";
pub const RC_MAP_MECOOL_KII_PRO: &str = "rc-mecool-kii-pro";
pub const RC_MAP_MECOOL_KIII_PRO: &str = "rc-mecool-kiii-pro";
pub const RC_MAP_MEDION_X10: &str = "rc-medion-x10";
pub const RC_MAP_MEDION_X10_DIGITAINER: &str = "rc-medion-x10-digitainer";
pub const RC_MAP_MEDION_X10_OR2X: &str = "rc-medion-x10-or2x";
pub const RC_MAP_MINIX_NEO: &str = "rc-minix-neo";
pub const RC_MAP_MSI_DIGIVOX_II: &str = "rc-msi-digivox-ii";
pub const RC_MAP_MSI_DIGIVOX_III: &str = "rc-msi-digivox-iii";
pub const RC_MAP_MSI_TVANYWHERE: &str = "rc-msi-tvanywhere";
pub const RC_MAP_MSI_TVANYWHERE_PLUS: &str = "rc-msi-tvanywhere-plus";
pub const RC_MAP_MYGICA_UTV3: &str = "rc-mygica-utv3";
pub const RC_MAP_NEBULA: &str = "rc-nebula";
pub const RC_MAP_NEC_TERRATEC_CINERGY_XS: &str = "rc-nec-terratec-cinergy-xs";
pub const RC_MAP_NORWOOD: &str = "rc-norwood";
pub const RC_MAP_NPGTECH: &str = "rc-npgtech";
pub const RC_MAP_ODROID: &str = "rc-odroid";
pub const RC_MAP_PCTV_SEDNA: &str = "rc-pctv-sedna";
pub const RC_MAP_PINE64: &str = "rc-pine64";
pub const RC_MAP_PINNACLE_COLOR: &str = "rc-pinnacle-color";
pub const RC_MAP_PINNACLE_GREY: &str = "rc-pinnacle-grey";
pub const RC_MAP_PINNACLE_PCTV_HD: &str = "rc-pinnacle-pctv-hd";
pub const RC_MAP_PIXELVIEW: &str = "rc-pixelview";
pub const RC_MAP_PIXELVIEW_002T: &str = "rc-pixelview-002t";
pub const RC_MAP_PIXELVIEW_MK12: &str = "rc-pixelview-mk12";
pub const RC_MAP_PIXELVIEW_NEW: &str = "rc-pixelview-new";
pub const RC_MAP_POWERCOLOR_REAL_ANGEL: &str = "rc-powercolor-real-angel";
pub const RC_MAP_PROTEUS_2309: &str = "rc-proteus-2309";
pub const RC_MAP_PURPLETV: &str = "rc-purpletv";
pub const RC_MAP_PV951: &str = "rc-pv951";
pub const RC_MAP_RC6_MCE: &str = "rc-rc6-mce";
pub const RC_MAP_REAL_AUDIO_220_32_KEYS: &str = "rc-real-audio-220-32-keys";
pub const RC_MAP_REDDO: &str = "rc-reddo";
pub const RC_MAP_SIEMENS_GIGASET_RC20: &str = "rc-siemens-gigaset-rc20";
pub const RC_MAP_SNAPSTREAM_FIREFLY: &str = "rc-snapstream-firefly";
pub const RC_MAP_STREAMZAP: &str = "rc-streamzap";
pub const RC_MAP_SU3000: &str = "rc-su3000";
pub const RC_MAP_TANIX_TX3MINI: &str = "rc-tanix-tx3mini";
pub const RC_MAP_TANIX_TX5MAX: &str = "rc-tanix-tx5max";
pub const RC_MAP_TBS_NEC: &str = "rc-tbs-nec";
pub const RC_MAP_TECHNISAT_TS35: &str = "rc-technisat-ts35";
pub const RC_MAP_TECHNISAT_USB2: &str = "rc-technisat-usb2";
pub const RC_MAP_TERRATEC_CINERGY_C_PCI: &str = "rc-terratec-cinergy-c-pci";
pub const RC_MAP_TERRATEC_CINERGY_S2_HD: &str = "rc-terratec-cinergy-s2-hd";
pub const RC_MAP_TERRATEC_CINERGY_XS: &str = "rc-terratec-cinergy-xs";
pub const RC_MAP_TERRATEC_SLIM: &str = "rc-terratec-slim";
pub const RC_MAP_TERRATEC_SLIM_2: &str = "rc-terratec-slim-2";
pub const RC_MAP_TEVII_NEC: &str = "rc-tevii-nec";
pub const RC_MAP_TIVO: &str = "rc-tivo";
pub const RC_MAP_TOTAL_MEDIA_IN_HAND: &str = "rc-total-media-in-hand";
pub const RC_MAP_TOTAL_MEDIA_IN_HAND_02: &str = "rc-total-media-in-hand-02";
pub const RC_MAP_TREKSTOR: &str = "rc-trekstor";
pub const RC_MAP_TT_1500: &str = "rc-tt-1500";
pub const RC_MAP_TWINHAN_DTV_CAB_CI: &str = "rc-twinhan-dtv-cab-ci";
pub const RC_MAP_TWINHAN_VP1027_DVBS: &str = "rc-twinhan1027";
pub const RC_MAP_VEGA_S9X: &str = "rc-vega-s9x";
pub const RC_MAP_VIDEOMATE_K100: &str = "rc-videomate-k100";
pub const RC_MAP_VIDEOMATE_S350: &str = "rc-videomate-s350";
pub const RC_MAP_VIDEOMATE_TV_PVR: &str = "rc-videomate-tv-pvr";
pub const RC_MAP_KII_PRO: &str = "rc-videostrong-kii-pro";
pub const RC_MAP_WETEK_HUB: &str = "rc-wetek-hub";
pub const RC_MAP_WETEK_PLAY2: &str = "rc-wetek-play2";
pub const RC_MAP_WINFAST: &str = "rc-winfast";
pub const RC_MAP_WINFAST_USBII_DELUXE: &str = "rc-winfast-usbii-deluxe";
pub const RC_MAP_X96MAX: &str = "rc-x96max";
pub const RC_MAP_XBOX_360: &str = "rc-xbox-360";
pub const RC_MAP_XBOX_DVD: &str = "rc-xbox-dvd";
pub const RC_MAP_ZX_IRDEC: &str = "rc-zx-irdec";

/* Please, do not just append newer Remote Controller names at the end.
 * The names should be ordered in alphabetical order. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
