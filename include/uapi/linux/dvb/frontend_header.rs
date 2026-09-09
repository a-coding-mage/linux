/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/* Rust translation of frontend.h. */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fe_caps {
    FE_IS_STUPID = 0, FE_CAN_INVERSION_AUTO = 0x1, FE_CAN_FEC_1_2 = 0x2,
    FE_CAN_FEC_2_3 = 0x4, FE_CAN_FEC_3_4 = 0x8, FE_CAN_FEC_4_5 = 0x10,
    FE_CAN_FEC_5_6 = 0x20, FE_CAN_FEC_6_7 = 0x40, FE_CAN_FEC_7_8 = 0x80,
    FE_CAN_FEC_8_9 = 0x100, FE_CAN_FEC_AUTO = 0x200, FE_CAN_QPSK = 0x400,
    FE_CAN_QAM_16 = 0x800, FE_CAN_QAM_32 = 0x1000, FE_CAN_QAM_64 = 0x2000,
    FE_CAN_QAM_128 = 0x4000, FE_CAN_QAM_256 = 0x8000, FE_CAN_QAM_AUTO = 0x10000,
    FE_CAN_TRANSMISSION_MODE_AUTO = 0x20000, FE_CAN_BANDWIDTH_AUTO = 0x40000,
    FE_CAN_GUARD_INTERVAL_AUTO = 0x80000, FE_CAN_HIERARCHY_AUTO = 0x100000,
    FE_CAN_8VSB = 0x200000, FE_CAN_16VSB = 0x400000, FE_HAS_EXTENDED_CAPS = 0x800000,
    FE_CAN_MULTISTREAM = 0x4000000, FE_CAN_TURBO_FEC = 0x8000000,
    FE_CAN_2G_MODULATION = 0x10000000, FE_NEEDS_BENDING = 0x20000000,
    FE_CAN_RECOVER = 0x40000000, FE_CAN_MUTE_TS = 0x80000000,
}
#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum fe_type { FE_QPSK, FE_QAM, FE_OFDM, FE_ATSC }
#[repr(C)] pub struct dvb_frontend_info { pub name: [i8;128], pub type_: fe_type, pub frequency_min:u32, pub frequency_max:u32, pub frequency_stepsize:u32, pub frequency_tolerance:u32, pub symbol_rate_min:u32, pub symbol_rate_max:u32, pub symbol_rate_tolerance:u32, pub notifier_delay:u32, pub caps:fe_caps }
#[repr(C)] pub struct dvb_diseqc_master_cmd { pub msg:[u8;6], pub msg_len:u8 }
#[repr(C)] pub struct dvb_diseqc_slave_reply { pub msg:[u8;4], pub msg_len:u8, pub timeout:i32 }

macro_rules! c_enum { ($name:ident { $($v:ident $(= $n:expr)?),* $(,)? }) => { #[repr(C)] #[derive(Copy,Clone,Debug,PartialEq,Eq)] pub enum $name { $($v $(= $n)?),* } }; }
c_enum!(fe_sec_voltage { SEC_VOLTAGE_13, SEC_VOLTAGE_18, SEC_VOLTAGE_OFF });
c_enum!(fe_sec_tone_mode { SEC_TONE_ON, SEC_TONE_OFF });
c_enum!(fe_sec_mini_cmd { SEC_MINI_A, SEC_MINI_B });
c_enum!(fe_status { FE_NONE=0x00, FE_HAS_SIGNAL=0x01, FE_HAS_CARRIER=0x02, FE_HAS_VITERBI=0x04, FE_HAS_SYNC=0x08, FE_HAS_LOCK=0x10, FE_TIMEDOUT=0x20, FE_REINIT=0x40 });
c_enum!(fe_spectral_inversion { INVERSION_OFF, INVERSION_ON, INVERSION_AUTO });
c_enum!(fe_code_rate { FEC_NONE=0, FEC_1_2, FEC_2_3, FEC_3_4, FEC_4_5, FEC_5_6, FEC_6_7, FEC_7_8, FEC_8_9, FEC_AUTO, FEC_3_5, FEC_9_10, FEC_2_5, FEC_1_3, FEC_1_4, FEC_5_9, FEC_7_9, FEC_8_15, FEC_11_15, FEC_13_18, FEC_9_20, FEC_11_20, FEC_23_36, FEC_25_36, FEC_13_45, FEC_26_45, FEC_28_45, FEC_32_45, FEC_77_90, FEC_11_45, FEC_4_15, FEC_14_45, FEC_7_15 });
c_enum!(fe_modulation { QPSK,QAM_16,QAM_32,QAM_64,QAM_128,QAM_256,QAM_AUTO,VSB_8,VSB_16,PSK_8,APSK_16,APSK_32,DQPSK,QAM_4_NR,QAM_1024,QAM_4096,APSK_8_L,APSK_16_L,APSK_32_L,APSK_64,APSK_64_L });
c_enum!(fe_transmit_mode { TRANSMISSION_MODE_2K,TRANSMISSION_MODE_8K,TRANSMISSION_MODE_AUTO,TRANSMISSION_MODE_4K,TRANSMISSION_MODE_1K,TRANSMISSION_MODE_16K,TRANSMISSION_MODE_32K,TRANSMISSION_MODE_C1,TRANSMISSION_MODE_C3780 });
c_enum!(fe_guard_interval { GUARD_INTERVAL_1_32,GUARD_INTERVAL_1_16,GUARD_INTERVAL_1_8,GUARD_INTERVAL_1_4,GUARD_INTERVAL_AUTO,GUARD_INTERVAL_1_128,GUARD_INTERVAL_19_128,GUARD_INTERVAL_19_256,GUARD_INTERVAL_PN420,GUARD_INTERVAL_PN595,GUARD_INTERVAL_PN945,GUARD_INTERVAL_1_64 });
c_enum!(fe_hierarchy { HIERARCHY_NONE,HIERARCHY_1,HIERARCHY_2,HIERARCHY_4,HIERARCHY_AUTO });
c_enum!(fe_interleaving { INTERLEAVING_NONE,INTERLEAVING_AUTO,INTERLEAVING_240,INTERLEAVING_720 });
c_enum!(fe_pilot { PILOT_ON,PILOT_OFF,PILOT_AUTO });
c_enum!(fe_rolloff { ROLLOFF_35,ROLLOFF_20,ROLLOFF_25,ROLLOFF_AUTO,ROLLOFF_15,ROLLOFF_10,ROLLOFF_5 });
c_enum!(fe_delivery_system { SYS_UNDEFINED,SYS_DVBC_ANNEX_A,SYS_DVBC_ANNEX_B,SYS_DVBT,SYS_DSS,SYS_DVBS,SYS_DVBS2,SYS_DVBH,SYS_ISDBT,SYS_ISDBS,SYS_ISDBC,SYS_ATSC,SYS_ATSCMH,SYS_DTMB,SYS_CMMB,SYS_DAB,SYS_DVBT2,SYS_TURBO,SYS_DVBC_ANNEX_C,SYS_DVBC2 });
pub const SYS_DVBC_ANNEX_AC: fe_delivery_system = fe_delivery_system::SYS_DVBC_ANNEX_A;
pub const SYS_DMBTH: fe_delivery_system = fe_delivery_system::SYS_DTMB;
c_enum!(atscmh_sccc_block_mode { ATSCMH_SCCC_BLK_SEP=0, ATSCMH_SCCC_BLK_COMB=1, ATSCMH_SCCC_BLK_RES=2 });
c_enum!(atscmh_sccc_code_mode { ATSCMH_SCCC_CODE_HLF=0, ATSCMH_SCCC_CODE_QTR=1, ATSCMH_SCCC_CODE_RES=2 });
c_enum!(atscmh_rs_frame_ensemble { ATSCMH_RSFRAME_ENS_PRI=0, ATSCMH_RSFRAME_ENS_SEC=1 });
c_enum!(atscmh_rs_frame_mode { ATSCMH_RSFRAME_PRI_ONLY=0, ATSCMH_RSFRAME_PRI_SEC=1, ATSCMH_RSFRAME_RES=2 });
c_enum!(atscmh_rs_code_mode { ATSCMH_RSCODE_211_187=0, ATSCMH_RSCODE_223_187=1, ATSCMH_RSCODE_235_187=2, ATSCMH_RSCODE_RES=3 });
c_enum!(fecap_scale_params { FE_SCALE_NOT_AVAILABLE=0, FE_SCALE_DECIBEL, FE_SCALE_RELATIVE, FE_SCALE_COUNTER });
#[repr(C, packed)] pub union dtv_stats_value { pub uvalue:u64, pub svalue:i64 }
#[repr(C, packed)] pub struct dtv_stats { pub scale:u8, pub value:dtv_stats_value }
pub const MAX_DTV_STATS:usize=4;
#[repr(C, packed)] pub struct dtv_fe_stats { pub len:u8, pub stat:[dtv_stats;MAX_DTV_STATS] }
#[repr(C, packed)] pub struct dtv_property_buffer { pub data:[u8;32], pub len:u32, pub reserved1:[u32;3], pub reserved2:*mut core::ffi::c_void }
#[repr(C)] pub union dtv_property_union { pub data:u32, pub st:dtv_fe_stats, pub buffer:dtv_property_buffer }
#[repr(C, packed)] pub struct dtv_property { pub cmd:u32, pub reserved:[u32;3], pub u:dtv_property_union, pub result:i32 }
#[repr(C)] pub struct dtv_properties { pub num:u32, pub props:*mut dtv_property }
pub const DTV_IOCTL_MAX_MSGS:u32=64; pub const FE_TUNE_MODE_ONESHOT:u32=0x01; pub const NO_STREAM_ID_FILTER:u32=!0u32; pub const LNA_AUTO:u32=!0u32;
pub const DTV_UNDEFINED:u32=0; pub const DTV_TUNE:u32=1; pub const DTV_CLEAR:u32=2; pub const DTV_FREQUENCY:u32=3; pub const DTV_MODULATION:u32=4; pub const DTV_BANDWIDTH_HZ:u32=5; pub const DTV_INVERSION:u32=6; pub const DTV_DISEQC_MASTER:u32=7; pub const DTV_SYMBOL_RATE:u32=8; pub const DTV_INNER_FEC:u32=9; pub const DTV_VOLTAGE:u32=10; pub const DTV_TONE:u32=11; pub const DTV_PILOT:u32=12; pub const DTV_ROLLOFF:u32=13; pub const DTV_DISEQC_SLAVE_REPLY:u32=14;
pub const DTV_FE_CAPABILITY_COUNT:u32=15; pub const DTV_FE_CAPABILITY:u32=16; pub const DTV_DELIVERY_SYSTEM:u32=17; pub const DTV_ISDBT_PARTIAL_RECEPTION:u32=18; pub const DTV_ISDBT_SOUND_BROADCASTING:u32=19; pub const DTV_ISDBT_SB_SUBCHANNEL_ID:u32=20; pub const DTV_ISDBT_SB_SEGMENT_IDX:u32=21; pub const DTV_ISDBT_SB_SEGMENT_COUNT:u32=22;
pub const DTV_ISDBT_LAYERA_FEC:u32=23; pub const DTV_ISDBT_LAYERA_MODULATION:u32=24; pub const DTV_ISDBT_LAYERA_SEGMENT_COUNT:u32=25; pub const DTV_ISDBT_LAYERA_TIME_INTERLEAVING:u32=26; pub const DTV_ISDBT_LAYERB_FEC:u32=27; pub const DTV_ISDBT_LAYERB_MODULATION:u32=28; pub const DTV_ISDBT_LAYERB_SEGMENT_COUNT:u32=29; pub const DTV_ISDBT_LAYERB_TIME_INTERLEAVING:u32=30; pub const DTV_ISDBT_LAYERC_FEC:u32=31; pub const DTV_ISDBT_LAYERC_MODULATION:u32=32; pub const DTV_ISDBT_LAYERC_SEGMENT_COUNT:u32=33; pub const DTV_ISDBT_LAYERC_TIME_INTERLEAVING:u32=34; pub const DTV_API_VERSION:u32=35; pub const DTV_CODE_RATE_HP:u32=36; pub const DTV_CODE_RATE_LP:u32=37; pub const DTV_GUARD_INTERVAL:u32=38; pub const DTV_TRANSMISSION_MODE:u32=39; pub const DTV_HIERARCHY:u32=40; pub const DTV_ISDBT_LAYER_ENABLED:u32=41; pub const DTV_STREAM_ID:u32=42; pub const DTV_ISDBS_TS_ID_LEGACY:u32=DTV_STREAM_ID; pub const DTV_DVBT2_PLP_ID_LEGACY:u32=43; pub const DTV_ENUM_DELSYS:u32=44;
pub const DTV_ATSCMH_FIC_VER:u32=45; pub const DTV_ATSCMH_PARADE_ID:u32=46; pub const DTV_ATSCMH_NOG:u32=47; pub const DTV_ATSCMH_TNOG:u32=48; pub const DTV_ATSCMH_SGN:u32=49; pub const DTV_ATSCMH_PRC:u32=50; pub const DTV_ATSCMH_RS_FRAME_MODE:u32=51; pub const DTV_ATSCMH_RS_FRAME_ENSEMBLE:u32=52; pub const DTV_ATSCMH_RS_CODE_MODE_PRI:u32=53; pub const DTV_ATSCMH_RS_CODE_MODE_SEC:u32=54; pub const DTV_ATSCMH_SCCC_BLOCK_MODE:u32=55; pub const DTV_ATSCMH_SCCC_CODE_MODE_A:u32=56; pub const DTV_ATSCMH_SCCC_CODE_MODE_B:u32=57; pub const DTV_ATSCMH_SCCC_CODE_MODE_C:u32=58; pub const DTV_ATSCMH_SCCC_CODE_MODE_D:u32=59; pub const DTV_INTERLEAVING:u32=60; pub const DTV_LNA:u32=61; pub const DTV_STAT_SIGNAL_STRENGTH:u32=62; pub const DTV_STAT_CNR:u32=63; pub const DTV_STAT_PRE_ERROR_BIT_COUNT:u32=64; pub const DTV_STAT_PRE_TOTAL_BIT_COUNT:u32=65; pub const DTV_STAT_POST_ERROR_BIT_COUNT:u32=66; pub const DTV_STAT_POST_TOTAL_BIT_COUNT:u32=67; pub const DTV_STAT_ERROR_BLOCK_COUNT:u32=68; pub const DTV_STAT_TOTAL_BLOCK_COUNT:u32=69; pub const DTV_SCRAMBLING_SEQUENCE_INDEX:u32=70; pub const DTV_MAX_COMMAND:u32=DTV_SCRAMBLING_SEQUENCE_INDEX;
// ioctl request macros depend on the Linux _IO/_IOR/_IOW definitions supplied by other headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
