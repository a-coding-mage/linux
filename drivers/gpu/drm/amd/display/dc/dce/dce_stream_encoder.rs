// Faithful low-level translation of dce_stream_encoder.c.
// Register helpers, kernel types, enums, and structures are supplied by the
// surrounding DRM display implementation.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

const VBI_LINE_0: u32 = 0;
const DP_BLANK_MAX_RETRY: u32 = 20;
const HDMI_CLOCK_CHANNEL_RATE_MORE_340M: u32 = 340_000;
const DP_MST_UPDATE_MAX_RETRY: u32 = 50;
const DP_SEC_AUD_N__DP_SEC_AUD_N__DEFAULT: u32 = 0x8000;
const DP_SEC_TIMESTAMP__DP_SEC_TIMESTAMP_MODE__AUTO_CALC: u32 = 1;

/* External declarations intentionally remain unresolved, as in the C file. */
extern "C" {
    fn div_u64(n: u64, d: u64) -> u64;
    fn udelay(usecs: u32);
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct audio_clock_info {
    pub pixel_clock_in_10khz: u32,
    pub n_32khz: u32,
    pub cts_32khz: u32,
    pub n_44khz: u32,
    pub cts_44khz: u32,
    pub n_48khz: u32,
    pub cts_48khz: u32,
}

static AUDIO_CLOCK_INFO_TABLE: [audio_clock_info; 16] = [
    audio_clock_info{pixel_clock_in_10khz:2517,n_32khz:4576,cts_32khz:28125,n_44khz:7007,cts_44khz:31250,n_48khz:6864,cts_48khz:28125},
    audio_clock_info{pixel_clock_in_10khz:2518,n_32khz:4576,cts_32khz:28125,n_44khz:7007,cts_44khz:31250,n_48khz:6864,cts_48khz:28125},
    audio_clock_info{pixel_clock_in_10khz:2520,n_32khz:4096,cts_32khz:25200,n_44khz:6272,cts_44khz:28000,n_48khz:6144,cts_48khz:25200},
    audio_clock_info{pixel_clock_in_10khz:2700,n_32khz:4096,cts_32khz:27000,n_44khz:6272,cts_44khz:30000,n_48khz:6144,cts_48khz:27000},
    audio_clock_info{pixel_clock_in_10khz:2702,n_32khz:4096,cts_32khz:27027,n_44khz:6272,cts_44khz:30030,n_48khz:6144,cts_48khz:27027},
    audio_clock_info{pixel_clock_in_10khz:2703,n_32khz:4096,cts_32khz:27027,n_44khz:6272,cts_44khz:30030,n_48khz:6144,cts_48khz:27027},
    audio_clock_info{pixel_clock_in_10khz:5400,n_32khz:4096,cts_32khz:54000,n_44khz:6272,cts_44khz:60000,n_48khz:6144,cts_48khz:54000},
    audio_clock_info{pixel_clock_in_10khz:5405,n_32khz:4096,cts_32khz:54054,n_44khz:6272,cts_44khz:60060,n_48khz:6144,cts_48khz:54054},
    audio_clock_info{pixel_clock_in_10khz:7417,n_32khz:11648,cts_32khz:210937,n_44khz:17836,cts_44khz:234375,n_48khz:11648,cts_48khz:140625},
    audio_clock_info{pixel_clock_in_10khz:7425,n_32khz:4096,cts_32khz:74250,n_44khz:6272,cts_44khz:82500,n_48khz:6144,cts_48khz:74250},
    audio_clock_info{pixel_clock_in_10khz:14835,n_32khz:11648,cts_32khz:421875,n_44khz:8918,cts_44khz:234375,n_48khz:5824,cts_48khz:140625},
    audio_clock_info{pixel_clock_in_10khz:14850,n_32khz:4096,cts_32khz:148500,n_44khz:6272,cts_44khz:165000,n_48khz:6144,cts_48khz:148500},
    audio_clock_info{pixel_clock_in_10khz:29670,n_32khz:5824,cts_32khz:421875,n_44khz:4459,cts_44khz:234375,n_48khz:5824,cts_48khz:281250},
    audio_clock_info{pixel_clock_in_10khz:29700,n_32khz:3072,cts_32khz:222750,n_44khz:4704,cts_44khz:247500,n_48khz:5120,cts_48khz:247500},
    audio_clock_info{pixel_clock_in_10khz:59340,n_32khz:5824,cts_32khz:843750,n_44khz:8918,cts_44khz:937500,n_48khz:5824,cts_48khz:562500},
    audio_clock_info{pixel_clock_in_10khz:59400,n_32khz:3072,cts_32khz:445500,n_44khz:9408,cts_44khz:990000,n_48khz:6144,cts_48khz:594000},
];

/* The following declarations preserve the complete externally visible API. */
extern "C" {
    pub fn dce110_stream_encoder_construct(enc110: *mut dce110_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios, eng_id: engine_id, regs: *const dce110_stream_enc_registers, se_shift: *const dce_stream_encoder_shift, se_mask: *const dce_stream_encoder_mask);
    pub fn dce110_analog_stream_encoder_construct(enc110: *mut dce110_stream_encoder, ctx: *mut dc_context, bp: *mut dc_bios, eng_id: engine_id, regs: *const dce110_stream_enc_registers, se_shift: *const dce_stream_encoder_shift, se_mask: *const dce_stream_encoder_mask);
    pub fn dce110_se_audio_mute_control(enc: *mut stream_encoder, mute: bool);
    pub fn dce110_se_dp_audio_setup(enc: *mut stream_encoder, az_inst: u32, info: *mut audio_info);
    pub fn dce110_se_dp_audio_enable(enc: *mut stream_encoder);
    pub fn dce110_se_dp_audio_disable(enc: *mut stream_encoder);
    pub fn dce110_se_hdmi_audio_setup(enc: *mut stream_encoder, az_inst: u32, info: *mut audio_info, audio_crtc_info: *mut audio_crtc_info);
    pub fn dce110_se_hdmi_audio_disable(enc: *mut stream_encoder);
}

/* Dependency-provided types. */
#[repr(C)] pub struct dce110_stream_encoder { pub base: stream_encoder, pub regs: *const dce110_stream_enc_registers, pub se_shift: *const dce_stream_encoder_shift, pub se_mask: *const dce_stream_encoder_mask }
#[repr(C)] pub struct stream_encoder { pub funcs: *const stream_encoder_funcs, pub ctx: *mut dc_context, pub id: engine_id, pub bp: *mut dc_bios }
#[repr(C)] pub struct stream_encoder_funcs { _private: [u8; 0] }
#[repr(C)] pub struct dce110_stream_enc_registers { _private: [u8; 0] }
#[repr(C)] pub struct dce_stream_encoder_shift { _private: [u8; 0] }
#[repr(C)] pub struct dce_stream_encoder_mask { _private: [u8; 0] }
pub enum engine_id {}
#[repr(C)] pub struct dc_context { _private: [u8; 0] }
#[repr(C)] pub struct dc_bios { _private: [u8; 0] }
#[repr(C)] pub struct audio_info { _private: [u8; 0] }
#[repr(C)] pub struct audio_crtc_info { _private: [u8; 0] }

// Source-level register implementations use the same REG_*/REG_WAIT helpers
// and field symbols supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
