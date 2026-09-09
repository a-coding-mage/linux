/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* uapi/sound/asoc.h -- ALSA SoC Firmware Controls and DAPM */

// C dependencies: linux/types.h and sound/asound.h.

pub const SND_SOC_TPLG_MAX_CHAN: usize = 8;
pub const SND_SOC_TPLG_MAX_FORMATS: usize = 16;
pub const SND_SOC_TPLG_STREAM_CONFIG_MAX: usize = 8;
pub const SND_SOC_TPLG_HW_CONFIG_MAX: usize = 8;

pub const SND_SOC_TPLG_CTL_VOLSW: u32 = 1;
pub const SND_SOC_TPLG_CTL_VOLSW_SX: u32 = 2;
pub const SND_SOC_TPLG_CTL_VOLSW_XR_SX: u32 = 3;
pub const SND_SOC_TPLG_CTL_ENUM: u32 = 4;
pub const SND_SOC_TPLG_CTL_BYTES: u32 = 5;
pub const SND_SOC_TPLG_CTL_ENUM_VALUE: u32 = 6;
pub const SND_SOC_TPLG_CTL_RANGE: u32 = 7;
pub const SND_SOC_TPLG_CTL_STROBE: u32 = 8;
pub const SND_SOC_TPLG_DAPM_CTL_VOLSW: u32 = 64;
pub const SND_SOC_TPLG_DAPM_CTL_ENUM_DOUBLE: u32 = 65;
pub const SND_SOC_TPLG_DAPM_CTL_ENUM_VIRT: u32 = 66;
pub const SND_SOC_TPLG_DAPM_CTL_ENUM_VALUE: u32 = 67;
pub const SND_SOC_TPLG_DAPM_CTL_PIN: u32 = 68;

pub const SND_SOC_TPLG_DAPM_INPUT: u32 = 0;
pub const SND_SOC_TPLG_DAPM_OUTPUT: u32 = 1;
pub const SND_SOC_TPLG_DAPM_MUX: u32 = 2;
pub const SND_SOC_TPLG_DAPM_MIXER: u32 = 3;
pub const SND_SOC_TPLG_DAPM_PGA: u32 = 4;
pub const SND_SOC_TPLG_DAPM_OUT_DRV: u32 = 5;
pub const SND_SOC_TPLG_DAPM_ADC: u32 = 6;
pub const SND_SOC_TPLG_DAPM_DAC: u32 = 7;
pub const SND_SOC_TPLG_DAPM_SWITCH: u32 = 8;
pub const SND_SOC_TPLG_DAPM_PRE: u32 = 9;
pub const SND_SOC_TPLG_DAPM_POST: u32 = 10;
pub const SND_SOC_TPLG_DAPM_AIF_IN: u32 = 11;
pub const SND_SOC_TPLG_DAPM_AIF_OUT: u32 = 12;
pub const SND_SOC_TPLG_DAPM_DAI_IN: u32 = 13;
pub const SND_SOC_TPLG_DAPM_DAI_OUT: u32 = 14;
pub const SND_SOC_TPLG_DAPM_DAI_LINK: u32 = 15;
pub const SND_SOC_TPLG_DAPM_BUFFER: u32 = 16;
pub const SND_SOC_TPLG_DAPM_SCHEDULER: u32 = 17;
pub const SND_SOC_TPLG_DAPM_EFFECT: u32 = 18;
pub const SND_SOC_TPLG_DAPM_SIGGEN: u32 = 19;
pub const SND_SOC_TPLG_DAPM_SRC: u32 = 20;
pub const SND_SOC_TPLG_DAPM_ASRC: u32 = 21;
pub const SND_SOC_TPLG_DAPM_ENCODER: u32 = 22;
pub const SND_SOC_TPLG_DAPM_DECODER: u32 = 23;
pub const SND_SOC_TPLG_DAPM_LAST: u32 = SND_SOC_TPLG_DAPM_DECODER;
pub const SND_SOC_TPLG_MAGIC: u32 = 0x41536F43;
pub const SND_SOC_TPLG_NUM_TEXTS: usize = 16;
pub const SND_SOC_TPLG_ABI_VERSION: u32 = 0x5;
pub const SND_SOC_TPLG_ABI_VERSION_MIN: u32 = 0x5;
pub const SND_SOC_TPLG_TLV_SIZE: usize = 32;

pub const SND_SOC_TPLG_TYPE_MIXER: u32 = 1;
pub const SND_SOC_TPLG_TYPE_BYTES: u32 = 2;
pub const SND_SOC_TPLG_TYPE_ENUM: u32 = 3;
pub const SND_SOC_TPLG_TYPE_DAPM_GRAPH: u32 = 4;
pub const SND_SOC_TPLG_TYPE_DAPM_WIDGET: u32 = 5;
pub const SND_SOC_TPLG_TYPE_DAI_LINK: u32 = 6;
pub const SND_SOC_TPLG_TYPE_PCM: u32 = 7;
pub const SND_SOC_TPLG_TYPE_MANIFEST: u32 = 8;
pub const SND_SOC_TPLG_TYPE_CODEC_LINK: u32 = 9;
pub const SND_SOC_TPLG_TYPE_BACKEND_LINK: u32 = 10;
pub const SND_SOC_TPLG_TYPE_PDATA: u32 = 11;
pub const SND_SOC_TPLG_TYPE_DAI: u32 = 12;
pub const SND_SOC_TPLG_TYPE_MAX: u32 = SND_SOC_TPLG_TYPE_DAI;
pub const SND_SOC_TPLG_TYPE_VENDOR_FW: u32 = 1000;
pub const SND_SOC_TPLG_TYPE_VENDOR_CONFIG: u32 = 1001;
pub const SND_SOC_TPLG_TYPE_VENDOR_COEFF: u32 = 1002;
pub const SND_SOC_TPLG_TYPEVENDOR_CODEC: u32 = 1003;
pub const SND_SOC_TPLG_STREAM_PLAYBACK: u32 = 0;
pub const SND_SOC_TPLG_STREAM_CAPTURE: u32 = 1;
pub const SND_SOC_TPLG_TUPLE_TYPE_UUID: u32 = 0;
pub const SND_SOC_TPLG_TUPLE_TYPE_STRING: u32 = 1;
pub const SND_SOC_TPLG_TUPLE_TYPE_BOOL: u32 = 2;
pub const SND_SOC_TPLG_TUPLE_TYPE_BYTE: u32 = 3;
pub const SND_SOC_TPLG_TUPLE_TYPE_WORD: u32 = 4;
pub const SND_SOC_TPLG_TUPLE_TYPE_SHORT: u32 = 5;
pub const SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_RATES: u32 = 1 << 0;
pub const SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_CHANNELS: u32 = 1 << 1;
pub const SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_SAMPLEBITS: u32 = 1 << 2;
pub const SND_SOC_TPLG_DAI_CLK_GATE_UNDEFINED: u8 = 0;
pub const SND_SOC_TPLG_DAI_CLK_GATE_GATED: u8 = 1;
pub const SND_SOC_TPLG_DAI_CLK_GATE_CONT: u8 = 2;
pub const SND_SOC_TPLG_MCLK_CO: u8 = 0;
pub const SND_SOC_TPLG_MCLK_CI: u8 = 1;
pub const SND_SOC_DAI_FORMAT_I2S: u32 = 1;
pub const SND_SOC_DAI_FORMAT_RIGHT_J: u32 = 2;
pub const SND_SOC_DAI_FORMAT_LEFT_J: u32 = 3;
pub const SND_SOC_DAI_FORMAT_DSP_A: u32 = 4;
pub const SND_SOC_DAI_FORMAT_DSP_B: u32 = 5;
pub const SND_SOC_DAI_FORMAT_AC97: u32 = 6;
pub const SND_SOC_DAI_FORMAT_PDM: u32 = 7;
pub const SND_SOC_DAI_FORMAT_MSB: u32 = SND_SOC_DAI_FORMAT_LEFT_J;
pub const SND_SOC_DAI_FORMAT_LSB: u32 = SND_SOC_DAI_FORMAT_RIGHT_J;
pub const SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_RATES: u32 = 1 << 0;
pub const SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_CHANNELS: u32 = 1 << 1;
pub const SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_SAMPLEBITS: u32 = 1 << 2;
pub const SND_SOC_TPLG_LNK_FLGBIT_VOICE_WAKEUP: u32 = 1 << 3;
pub const SND_SOC_TPLG_BCLK_CP: u32 = 0;
pub const SND_SOC_TPLG_BCLK_CC: u32 = 1;
pub const SND_SOC_TPLG_BCLK_CM: u32 = SND_SOC_TPLG_BCLK_CP;
pub const SND_SOC_TPLG_BCLK_CS: u32 = SND_SOC_TPLG_BCLK_CC;
pub const SND_SOC_TPLG_FSYNC_CP: u32 = 0;
pub const SND_SOC_TPLG_FSYNC_CC: u32 = 1;
pub const SND_SOC_TPLG_FSYNC_CM: u32 = SND_SOC_TPLG_FSYNC_CP;
pub const SND_SOC_TPLG_FSYNC_CS: u32 = SND_SOC_TPLG_FSYNC_CC;

// External aliases supplied by linux/types.h and sound/asound.h.
pub type __le16 = u16;
pub type __le32 = u32;
pub type __le64 = u64;
pub type __u8 = u8;
pub const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44; // external dependency value

#[repr(C, packed)]
pub struct snd_soc_tplg_hdr { pub magic: __le32, pub abi: __le32, pub version: __le32, pub type_: __le32, pub size: __le32, pub vendor_type: __le32, pub payload_size: __le32, pub index: __le32, pub count: __le32 }
#[repr(C, packed)] pub struct snd_soc_tplg_vendor_uuid_elem { pub token: __le32, pub uuid: [u8; 16] }
#[repr(C, packed)] pub struct snd_soc_tplg_vendor_value_elem { pub token: __le32, pub value: __le32 }
#[repr(C, packed)] pub struct snd_soc_tplg_vendor_string_elem { pub token: __le32, pub string: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN] }
#[repr(C)] pub union snd_soc_tplg_vendor_array_data { pub uuid: [snd_soc_tplg_vendor_uuid_elem; 0], pub value: [snd_soc_tplg_vendor_value_elem; 0], pub string: [snd_soc_tplg_vendor_string_elem; 0] }
#[repr(C, packed)] pub struct snd_soc_tplg_vendor_array { pub size: __le32, pub type_: __le32, pub num_elems: __le32, pub data: snd_soc_tplg_vendor_array_data }
#[repr(C)] pub union snd_soc_tplg_private_data { pub data: [u8; 0], pub array: [snd_soc_tplg_vendor_array; 0] }
#[repr(C, packed)] pub struct snd_soc_tplg_private { pub size: __le32, pub data: snd_soc_tplg_private_data }
#[repr(C, packed)] pub struct snd_soc_tplg_tlv_dbscale { pub min: __le32, pub step: __le32, pub mute: __le32 }
#[repr(C)] pub union snd_soc_tplg_ctl_tlv_data { pub data: [__le32; SND_SOC_TPLG_TLV_SIZE], pub scale: snd_soc_tplg_tlv_dbscale }
#[repr(C, packed)] pub struct snd_soc_tplg_ctl_tlv { pub size: __le32, pub type_: __le32, pub data: snd_soc_tplg_ctl_tlv_data }
#[repr(C, packed)] pub struct snd_soc_tplg_channel { pub size: __le32, pub reg: __le32, pub shift: __le32, pub id: __le32 }
#[repr(C, packed)] pub struct snd_soc_tplg_io_ops { pub get: __le32, pub put: __le32, pub info: __le32 }
#[repr(C, packed)] pub struct snd_soc_tplg_ctl_hdr { pub size: __le32, pub type_: __le32, pub name: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub access: __le32, pub ops: snd_soc_tplg_io_ops, pub tlv: snd_soc_tplg_ctl_tlv }
#[repr(C, packed)] pub struct snd_soc_tplg_stream_caps { pub size: __le32, pub name: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub formats: __le64, pub rates: __le32, pub rate_min: __le32, pub rate_max: __le32, pub channels_min: __le32, pub channels_max: __le32, pub periods_min: __le32, pub periods_max: __le32, pub period_size_min: __le32, pub period_size_max: __le32, pub buffer_size_min: __le32, pub buffer_size_max: __le32, pub sig_bits: __le32 }
#[repr(C, packed)] pub struct snd_soc_tplg_stream { pub size: __le32, pub name: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub format: __le64, pub rate: __le32, pub period_bytes: __le32, pub buffer_bytes: __le32, pub channels: __le32 }
#[repr(C, packed)] pub struct snd_soc_tplg_hw_config { pub size: __le32, pub id: __le32, pub fmt: __le32, pub clock_gated: __u8, pub invert_bclk: __u8, pub invert_fsync: __u8, pub bclk_provider: __u8, pub fsync_provider: __u8, pub mclk_direction: __u8, pub reserved: __le16, pub mclk_rate: __le32, pub bclk_rate: __le32, pub fsync_rate: __le32, pub tdm_slots: __le32, pub tdm_slot_width: __le32, pub tx_slots: __le32, pub rx_slots: __le32, pub tx_channels: __le32, pub tx_chanmap: [__le32; SND_SOC_TPLG_MAX_CHAN], pub rx_channels: __le32, pub rx_chanmap: [__le32; SND_SOC_TPLG_MAX_CHAN] }
#[repr(C, packed)] pub struct snd_soc_tplg_manifest { pub size: __le32, pub control_elems: __le32, pub widget_elems: __le32, pub graph_elems: __le32, pub pcm_elems: __le32, pub dai_link_elems: __le32, pub dai_elems: __le32, pub reserved: [__le32; 20], pub priv_: snd_soc_tplg_private }
#[repr(C, packed)] pub struct snd_soc_tplg_mixer_control { pub hdr: snd_soc_tplg_ctl_hdr, pub size: __le32, pub min: __le32, pub max: __le32, pub platform_max: __le32, pub invert: __le32, pub num_channels: __le32, pub channel: [snd_soc_tplg_channel; SND_SOC_TPLG_MAX_CHAN], pub priv_: snd_soc_tplg_private }
#[repr(C, packed)] pub struct snd_soc_tplg_enum_control { pub hdr: snd_soc_tplg_ctl_hdr, pub size: __le32, pub num_channels: __le32, pub channel: [snd_soc_tplg_channel; SND_SOC_TPLG_MAX_CHAN], pub items: __le32, pub mask: __le32, pub count: __le32, pub texts: [[u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN]; SND_SOC_TPLG_NUM_TEXTS], pub values: [__le32; SND_SOC_TPLG_NUM_TEXTS * SNDRV_CTL_ELEM_ID_NAME_MAXLEN / 4], pub priv_: snd_soc_tplg_private }
#[repr(C, packed)] pub struct snd_soc_tplg_bytes_control { pub hdr: snd_soc_tplg_ctl_hdr, pub size: __le32, pub max: __le32, pub mask: __le32, pub base: __le32, pub num_regs: __le32, pub ext_ops: snd_soc_tplg_io_ops, pub priv_: snd_soc_tplg_private }
#[repr(C, packed)] pub struct snd_soc_tplg_dapm_graph_elem { pub sink: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub control: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub source: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN] }
#[repr(C, packed)] pub struct snd_soc_tplg_dapm_widget { pub size: __le32, pub id: __le32, pub name: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub sname: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub reg: __le32, pub shift: __le32, pub mask: __le32, pub subseq: __le32, pub invert: __le32, pub ignore_suspend: __le32, pub event_flags: __le16, pub event_type: __le16, pub num_kcontrols: __le32, pub priv_: snd_soc_tplg_private }
#[repr(C, packed)] pub struct snd_soc_tplg_pcm { pub size: __le32, pub pcm_name: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub dai_name: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub pcm_id: __le32, pub dai_id: __le32, pub playback: __le32, pub capture: __le32, pub compress: __le32, pub stream: [snd_soc_tplg_stream; SND_SOC_TPLG_STREAM_CONFIG_MAX], pub num_streams: __le32, pub caps: [snd_soc_tplg_stream_caps; 2], pub flag_mask: __le32, pub flags: __le32, pub priv_: snd_soc_tplg_private }
#[repr(C, packed)] pub struct snd_soc_tplg_link_config { pub size: __le32, pub id: __le32, pub name: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub stream_name: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub stream: [snd_soc_tplg_stream; SND_SOC_TPLG_STREAM_CONFIG_MAX], pub num_streams: __le32, pub hw_config: [snd_soc_tplg_hw_config; SND_SOC_TPLG_HW_CONFIG_MAX], pub num_hw_configs: __le32, pub default_hw_config_id: __le32, pub flag_mask: __le32, pub flags: __le32, pub priv_: snd_soc_tplg_private }
#[repr(C, packed)] pub struct snd_soc_tplg_dai { pub size: __le32, pub dai_name: [u8; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub dai_id: __le32, pub playback: __le32, pub capture: __le32, pub caps: [snd_soc_tplg_stream_caps; 2], pub flag_mask: __le32, pub flags: __le32, pub priv_: snd_soc_tplg_private }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
