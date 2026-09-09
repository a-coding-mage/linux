/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) AND MIT) */
/*
 * Rust translation of compress_params.h.
 * The Linux integer types below are supplied by the surrounding bindings.
 */

pub const MAX_NUM_CODECS: usize = 32;
pub const MAX_NUM_CODEC_DESCRIPTORS: usize = 32;
pub const MAX_NUM_BITRATES: usize = 32;
pub const MAX_NUM_SAMPLE_RATES: usize = 32;

pub const SND_AUDIOCODEC_PCM: __u32 = 0x00000001;
pub const SND_AUDIOCODEC_MP3: __u32 = 0x00000002;
pub const SND_AUDIOCODEC_AMR: __u32 = 0x00000003;
pub const SND_AUDIOCODEC_AMRWB: __u32 = 0x00000004;
pub const SND_AUDIOCODEC_AMRWBPLUS: __u32 = 0x00000005;
pub const SND_AUDIOCODEC_AAC: __u32 = 0x00000006;
pub const SND_AUDIOCODEC_WMA: __u32 = 0x00000007;
pub const SND_AUDIOCODEC_REAL: __u32 = 0x00000008;
pub const SND_AUDIOCODEC_VORBIS: __u32 = 0x00000009;
pub const SND_AUDIOCODEC_FLAC: __u32 = 0x0000000A;
pub const SND_AUDIOCODEC_IEC61937: __u32 = 0x0000000B;
pub const SND_AUDIOCODEC_G723_1: __u32 = 0x0000000C;
pub const SND_AUDIOCODEC_G729: __u32 = 0x0000000D;
pub const SND_AUDIOCODEC_BESPOKE: __u32 = 0x0000000E;
pub const SND_AUDIOCODEC_ALAC: __u32 = 0x0000000F;
pub const SND_AUDIOCODEC_APE: __u32 = 0x00000010;
pub const SND_AUDIOCODEC_OPUS_RAW: __u32 = 0x00000011;
pub const SND_AUDIOCODEC_MAX: __u32 = SND_AUDIOCODEC_OPUS_RAW;

macro_rules! u32_consts { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: __u32 = $v;)* }; }
u32_consts! {
 SND_AUDIOPROFILE_PCM=1, SND_AUDIOCHANMODE_MP3_MONO=1, SND_AUDIOCHANMODE_MP3_STEREO=2,
 SND_AUDIOCHANMODE_MP3_JOINTSTEREO=4, SND_AUDIOCHANMODE_MP3_DUAL=8,
 SND_AUDIOPROFILE_AMR=1, SND_AUDIOMODE_AMR_DTX_OFF=1, SND_AUDIOMODE_AMR_VAD1=2, SND_AUDIOMODE_AMR_VAD2=4,
 SND_AUDIOSTREAMFORMAT_UNDEFINED=0, SND_AUDIOSTREAMFORMAT_CONFORMANCE=1, SND_AUDIOSTREAMFORMAT_IF1=2,
 SND_AUDIOSTREAMFORMAT_IF2=4, SND_AUDIOSTREAMFORMAT_FSF=8, SND_AUDIOSTREAMFORMAT_RTPPAYLOAD=16, SND_AUDIOSTREAMFORMAT_ITU=32,
 SND_AUDIOPROFILE_AMRWB=1, SND_AUDIOMODE_AMRWB_DTX_OFF=1, SND_AUDIOMODE_AMRWB_VAD1=2, SND_AUDIOMODE_AMRWB_VAD2=4,
 SND_AUDIOPROFILE_AMRWBPLUS=1, SND_AUDIOPROFILE_AAC=1,
 SND_AUDIOMODE_AAC_MAIN=1, SND_AUDIOMODE_AAC_LC=2, SND_AUDIOMODE_AAC_SSR=4, SND_AUDIOMODE_AAC_LTP=8,
 SND_AUDIOMODE_AAC_HE=16, SND_AUDIOMODE_AAC_SCALABLE=32, SND_AUDIOMODE_AAC_ERLC=64, SND_AUDIOMODE_AAC_LD=128,
 SND_AUDIOMODE_AAC_HE_PS=256, SND_AUDIOMODE_AAC_HE_MPS=512,
 SND_AUDIOSTREAMFORMAT_MP2ADTS=1, SND_AUDIOSTREAMFORMAT_MP4ADTS=2, SND_AUDIOSTREAMFORMAT_MP4LOAS=4,
 SND_AUDIOSTREAMFORMAT_MP4LATM=8, SND_AUDIOSTREAMFORMAT_ADIF=16, SND_AUDIOSTREAMFORMAT_MP4FF=32, SND_AUDIOSTREAMFORMAT_RAW=64,
 SND_AUDIOPROFILE_WMA7=1, SND_AUDIOPROFILE_WMA8=2, SND_AUDIOPROFILE_WMA9=4, SND_AUDIOPROFILE_WMA10=8,
 SND_AUDIOPROFILE_WMA9_PRO=16, SND_AUDIOPROFILE_WMA9_LOSSLESS=32, SND_AUDIOPROFILE_WMA10_LOSSLESS=64,
 SND_AUDIOMODE_WMA_LEVEL1=1, SND_AUDIOMODE_WMA_LEVEL2=2, SND_AUDIOMODE_WMA_LEVEL3=4, SND_AUDIOMODE_WMA_LEVEL4=8,
 SND_AUDIOMODE_WMAPRO_LEVELM0=16, SND_AUDIOMODE_WMAPRO_LEVELM1=32, SND_AUDIOMODE_WMAPRO_LEVELM2=64, SND_AUDIOMODE_WMAPRO_LEVELM3=128,
 SND_AUDIOSTREAMFORMAT_WMA_ASF=1, SND_AUDIOSTREAMFORMAT_WMA_NOASF_HDR=2, SND_AUDIOPROFILE_REALAUDIO=1,
 SND_AUDIOMODE_REALAUDIO_G2=1, SND_AUDIOMODE_REALAUDIO_8=2, SND_AUDIOMODE_REALAUDIO_10=4, SND_AUDIOMODE_REALAUDIO_SURROUND=8,
 SND_AUDIOPROFILE_VORBIS=1, SND_AUDIOMODE_VORBIS=1, SND_AUDIOPROFILE_FLAC=1,
 SND_AUDIOMODE_FLAC_LEVEL0=1, SND_AUDIOMODE_FLAC_LEVEL1=2, SND_AUDIOMODE_FLAC_LEVEL2=4, SND_AUDIOMODE_FLAC_LEVEL3=8,
 SND_AUDIOMODE_FLAC_LEVEL4=16, SND_AUDIOMODE_FLAC_LEVEL5=32, SND_AUDIOMODE_FLAC_LEVEL6=64, SND_AUDIOMODE_FLAC_LEVEL7=128, SND_AUDIOMODE_FLAC_LEVEL8=256,
 SND_AUDIOSTREAMFORMAT_FLAC=1, SND_AUDIOSTREAMFORMAT_FLAC_OGG=2, SND_AUDIOPROFILE_IEC61937=1, SND_AUDIOPROFILE_IEC61937_SPDIF=2,
 SND_AUDIOMODE_IEC_REF_STREAM_HEADER=0, SND_AUDIOMODE_IEC_LPCM=1, SND_AUDIOMODE_IEC_AC3=2, SND_AUDIOMODE_IEC_MPEG1=4,
 SND_AUDIOMODE_IEC_MP3=8, SND_AUDIOMODE_IEC_MPEG2=16, SND_AUDIOMODE_IEC_AACLC=32, SND_AUDIOMODE_IEC_DTS=64,
 SND_AUDIOMODE_IEC_ATRAC=128, SND_AUDIOMODE_IEC_SACD=256, SND_AUDIOMODE_IEC_EAC3=512, SND_AUDIOMODE_IEC_DTS_HD=1024,
 SND_AUDIOMODE_IEC_MLP=2048, SND_AUDIOMODE_IEC_DST=4096, SND_AUDIOMODE_IEC_WMAPRO=8192, SND_AUDIOMODE_IEC_REF_CXT=16384,
 SND_AUDIOMODE_IEC_HE_AAC=32768, SND_AUDIOMODE_IEC_HE_AAC2=65536, SND_AUDIOMODE_IEC_MPEG_SURROUND=131072,
 SND_AUDIOPROFILE_G723_1=1, SND_AUDIOMODE_G723_1_ANNEX_A=1, SND_AUDIOMODE_G723_1_ANNEX_B=2, SND_AUDIOMODE_G723_1_ANNEX_C=4,
 SND_AUDIOPROFILE_G729=1, SND_AUDIOMODE_G729_ANNEX_A=1, SND_AUDIOMODE_G729_ANNEX_B=2,
 SND_RATECONTROLMODE_CONSTANTBITRATE=1, SND_RATECONTROLMODE_VARIABLEBITRATE=2
}

#[repr(C, packed(4))]
pub struct snd_enc_wma { pub super_block_align: __u32 }
#[repr(C, packed(4))]
pub struct snd_enc_vorbis { pub quality: __s32, pub managed: __u32, pub max_bit_rate: __u32, pub min_bit_rate: __u32, pub downmix: __u32 }
#[repr(C, packed(4))]
pub struct snd_enc_real { pub quant_bits: __u32, pub start_region: __u32, pub num_regions: __u32 }
#[repr(C, packed(4))]
pub struct snd_enc_flac { pub num: __u32, pub gain: __u32 }
#[repr(C, packed(4))]
pub struct snd_enc_generic { pub bw: __u32, pub reserved: [__s32; 15] }
#[repr(C, packed(4))]
pub struct snd_dec_flac { pub sample_size: __u16, pub min_blk_size: __u16, pub max_blk_size: __u16, pub min_frame_size: __u16, pub max_frame_size: __u16, pub reserved: __u16 }
#[repr(C, packed(4))]
pub struct snd_dec_wma { pub encoder_option: __u32, pub adv_encoder_option: __u32, pub adv_encoder_option2: __u32, pub reserved: __u32 }
#[repr(C, packed(4))]
pub struct snd_dec_alac { pub frame_length: __u32, pub compatible_version: __u8, pub pb: __u8, pub mb: __u8, pub kb: __u8, pub max_run: __u32, pub max_frame_bytes: __u32 }
#[repr(C, packed(4))]
pub struct snd_dec_ape { pub compatible_version: __u16, pub compression_level: __u16, pub format_flags: __u32, pub blocks_per_frame: __u32, pub final_frame_blocks: __u32, pub total_frames: __u32, pub seek_table_present: __u32 }
#[repr(C, packed(4))]
pub struct snd_dec_opus_ch_map { pub stream_count: __u8, pub coupled_count: __u8, pub channel_map: [__u8; 8] }
#[repr(C, packed(4))]
pub struct snd_dec_opus { pub version: __u8, pub num_channels: __u8, pub pre_skip: __u16, pub sample_rate: __u32, pub output_gain: __u16, pub mapping_family: __u8, pub chan_map: snd_dec_opus_ch_map }

#[repr(C)]
pub union snd_codec_options {
    pub wma: snd_enc_wma, pub vorbis: snd_enc_vorbis, pub real: snd_enc_real, pub flac: snd_enc_flac,
    pub generic: snd_enc_generic, pub flac_d: snd_dec_flac, pub wma_d: snd_dec_wma, pub alac_d: snd_dec_alac,
    pub ape_d: snd_dec_ape, pub opus_d: snd_dec_opus, pub src_d: snd_codec_options_src_d,
}
#[repr(C, packed(4))]
pub struct snd_codec_options_src_d { pub out_sample_rate: __u32 }
#[repr(C, packed(4))]
pub struct snd_codec_desc_src { pub out_sample_rate_min: __u32, pub out_sample_rate_max: __u32 }
#[repr(C)]
pub union snd_codec_desc_u { pub u_space: [__u32; 6], pub src: snd_codec_desc_src }
#[repr(C, packed(4))]
pub struct snd_codec_desc {
    pub max_ch: __u32, pub sample_rates: [__u32; MAX_NUM_SAMPLE_RATES], pub num_sample_rates: __u32,
    pub bit_rate: [__u32; MAX_NUM_BITRATES], pub num_bitrates: __u32, pub rate_control: __u32,
    pub profiles: __u32, pub modes: __u32, pub formats: __u32, pub min_buffer: __u32, pub pcm_formats: __u32,
    pub u: snd_codec_desc_u, pub reserved: [__u32; 8]
}
#[repr(C, packed(4))]
pub struct snd_codec {
    pub id: __u32, pub ch_in: __u32, pub ch_out: __u32, pub sample_rate: __u32, pub bit_rate: __u32,
    pub rate_control: __u32, pub profile: __u32, pub level: __u32, pub ch_mode: __u32, pub format: __u32,
    pub align: __u32, pub options: snd_codec_options, pub pcm_format: __u32, pub reserved: [__u32; 2]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
