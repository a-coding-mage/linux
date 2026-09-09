/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 * Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
 *         Keyon Jie <yang.jie@linux.intel.com>
 */

/*
 * Topology IDs and tokens.
 *
 * ** MUST BE ALIGNED WITH TOPOLOGY CONFIGURATION TOKEN VALUES **
 */

/* Kcontrol IDs */
pub const SOF_TPLG_KCTL_VOL_ID: i32 = 256;
pub const SOF_TPLG_KCTL_ENUM_ID: i32 = 257;
pub const SOF_TPLG_KCTL_BYTES_ID: i32 = 258;
pub const SOF_TPLG_KCTL_SWITCH_ID: i32 = 259;
pub const SOF_TPLG_KCTL_BYTES_VOLATILE_RO: i32 = 260;
pub const SOF_TPLG_KCTL_BYTES_VOLATILE_RW: i32 = 261;
pub const SOF_TPLG_KCTL_BYTES_WO_ID: i32 = 262;

/* Tokens - must match values in topology configurations */
/* buffers */
pub const SOF_TKN_BUF_SIZE: i32 = 100;
pub const SOF_TKN_BUF_CAPS: i32 = 101;
pub const SOF_TKN_BUF_FLAGS: i32 = 102;

/* DAI */
/* Token retired with ABI 3.2, do not use for new capabilities */
pub const SOF_TKN_DAI_TYPE: i32 = 154;
pub const SOF_TKN_DAI_INDEX: i32 = 155;
pub const SOF_TKN_DAI_DIRECTION: i32 = 156;

/* scheduling */
pub const SOF_TKN_SCHED_PERIOD: i32 = 200;
pub const SOF_TKN_SCHED_PRIORITY: i32 = 201;
pub const SOF_TKN_SCHED_MIPS: i32 = 202;
pub const SOF_TKN_SCHED_CORE: i32 = 203;
pub const SOF_TKN_SCHED_FRAMES: i32 = 204;
pub const SOF_TKN_SCHED_TIME_DOMAIN: i32 = 205;
pub const SOF_TKN_SCHED_DYNAMIC_PIPELINE: i32 = 206;
pub const SOF_TKN_SCHED_LP_MODE: i32 = 207;
pub const SOF_TKN_SCHED_MEM_USAGE: i32 = 208;
pub const SOF_TKN_SCHED_USE_CHAIN_DMA: i32 = 209;
pub const SOF_TKN_SCHED_KCPS: i32 = 210;
pub const SOF_TKN_SCHED_DIRECTION: i32 = 211;
pub const SOF_TKN_SCHED_DIRECTION_VALID: i32 = 212;

/* volume */
pub const SOF_TKN_VOLUME_RAMP_STEP_TYPE: i32 = 250;
pub const SOF_TKN_VOLUME_RAMP_STEP_MS: i32 = 251;
pub const SOF_TKN_GAIN_RAMP_TYPE: i32 = 260;
pub const SOF_TKN_GAIN_RAMP_DURATION: i32 = 261;
pub const SOF_TKN_GAIN_VAL: i32 = 262;

/* SRC */
pub const SOF_TKN_SRC_RATE_IN: i32 = 300;
pub const SOF_TKN_SRC_RATE_OUT: i32 = 301;
/* ASRC */
pub const SOF_TKN_ASRC_RATE_IN: i32 = 320;
pub const SOF_TKN_ASRC_RATE_OUT: i32 = 321;
pub const SOF_TKN_ASRC_ASYNCHRONOUS_MODE: i32 = 322;
pub const SOF_TKN_ASRC_OPERATION_MODE: i32 = 323;
/* PCM */
pub const SOF_TKN_PCM_DMAC_CONFIG: i32 = 353;

/* Generic components */
pub const SOF_TKN_COMP_PERIOD_SINK_COUNT: i32 = 400;
pub const SOF_TKN_COMP_PERIOD_SOURCE_COUNT: i32 = 401;
pub const SOF_TKN_COMP_FORMAT: i32 = 402;
/* Token retired with ABI 3.2, do not use for new capabilities */
pub const SOF_TKN_COMP_CORE_ID: i32 = 404;
pub const SOF_TKN_COMP_UUID: i32 = 405;
pub const SOF_TKN_COMP_CPC: i32 = 406;
pub const SOF_TKN_COMP_IS_PAGES: i32 = 409;
pub const SOF_TKN_COMP_NUM_AUDIO_FORMATS: i32 = 410;
pub const SOF_TKN_COMP_NUM_INPUT_PINS: i32 = 411;
pub const SOF_TKN_COMP_NUM_OUTPUT_PINS: i32 = 412;
/* The token for input/output pin binding, specifying the connected widget name. */
pub const SOF_TKN_COMP_INPUT_PIN_BINDING_WNAME: i32 = 413;
pub const SOF_TKN_COMP_OUTPUT_PIN_BINDING_WNAME: i32 = 414;
pub const SOF_TKN_COMP_NUM_INPUT_AUDIO_FORMATS: i32 = 415;
pub const SOF_TKN_COMP_NUM_OUTPUT_AUDIO_FORMATS: i32 = 416;
/* The token value is copied to the dapm_widget's no_wname_in_kcontrol_name. */
pub const SOF_TKN_COMP_NO_WNAME_IN_KCONTROL_NAME: i32 = 417;
pub const SOF_TKN_COMP_SCHED_DOMAIN: i32 = 418;
pub const SOF_TKN_COMP_DOMAIN_ID: i32 = 419;
pub const SOF_TKN_COMP_STACK_BYTES_REQUIREMENT: i32 = 420;
pub const SOF_TKN_COMP_HEAP_BYTES_REQUIREMENT: i32 = 421;

/* SSP */
pub const SOF_TKN_INTEL_SSP_CLKS_CONTROL: i32 = 500;
pub const SOF_TKN_INTEL_SSP_MCLK_ID: i32 = 501;
pub const SOF_TKN_INTEL_SSP_SAMPLE_BITS: i32 = 502;
pub const SOF_TKN_INTEL_SSP_FRAME_PULSE_WIDTH: i32 = 503;
pub const SOF_TKN_INTEL_SSP_QUIRKS: i32 = 504;
pub const SOF_TKN_INTEL_SSP_TDM_PADDING_PER_SLOT: i32 = 505;
pub const SOF_TKN_INTEL_SSP_BCLK_DELAY: i32 = 506;

/* DMIC */
pub const SOF_TKN_INTEL_DMIC_DRIVER_VERSION: i32 = 600;
pub const SOF_TKN_INTEL_DMIC_CLK_MIN: i32 = 601;
pub const SOF_TKN_INTEL_DMIC_CLK_MAX: i32 = 602;
pub const SOF_TKN_INTEL_DMIC_DUTY_MIN: i32 = 603;
pub const SOF_TKN_INTEL_DMIC_DUTY_MAX: i32 = 604;
pub const SOF_TKN_INTEL_DMIC_NUM_PDM_ACTIVE: i32 = 605;
pub const SOF_TKN_INTEL_DMIC_SAMPLE_RATE: i32 = 608;
pub const SOF_TKN_INTEL_DMIC_FIFO_WORD_LENGTH: i32 = 609;
pub const SOF_TKN_INTEL_DMIC_UNMUTE_RAMP_TIME_MS: i32 = 610;
/* DMIC PDM */
pub const SOF_TKN_INTEL_DMIC_PDM_CTRL_ID: i32 = 700;
pub const SOF_TKN_INTEL_DMIC_PDM_MIC_A_Enable: i32 = 701;
pub const SOF_TKN_INTEL_DMIC_PDM_MIC_B_Enable: i32 = 702;
pub const SOF_TKN_INTEL_DMIC_PDM_POLARITY_A: i32 = 703;
pub const SOF_TKN_INTEL_DMIC_PDM_POLARITY_B: i32 = 704;
pub const SOF_TKN_INTEL_DMIC_PDM_CLK_EDGE: i32 = 705;
pub const SOF_TKN_INTEL_DMIC_PDM_SKEW: i32 = 706;

/* Tone */
pub const SOF_TKN_TONE_SAMPLE_RATE: i32 = 800;
/* Processing Components */
pub const SOF_TKN_PROCESS_TYPE: i32 = 900;
/* for backward compatibility */
pub const SOF_TKN_EFFECT_TYPE: i32 = SOF_TKN_PROCESS_TYPE;
/* SAI */
pub const SOF_TKN_IMX_SAI_MCLK_ID: i32 = 1000;
/* ESAI */
pub const SOF_TKN_IMX_ESAI_MCLK_ID: i32 = 1100;
/* Stream */
pub const SOF_TKN_STREAM_PLAYBACK_COMPATIBLE_D0I3: i32 = 1200;
pub const SOF_TKN_STREAM_CAPTURE_COMPATIBLE_D0I3: i32 = 1201;
pub const SOF_TKN_STREAM_PLAYBACK_PAUSE_SUPPORTED: i32 = 1202;
pub const SOF_TKN_STREAM_CAPTURE_PAUSE_SUPPORTED: i32 = 1203;
/* Led control for mute switches */
pub const SOF_TKN_MUTE_LED_USE: i32 = 1300;
pub const SOF_TKN_MUTE_LED_DIRECTION: i32 = 1301;
/* ALH */
pub const SOF_TKN_INTEL_ALH_RATE: i32 = 1400;
pub const SOF_TKN_INTEL_ALH_CH: i32 = 1401;
/* HDA */
pub const SOF_TKN_INTEL_HDA_RATE: i32 = 1500;
pub const SOF_TKN_INTEL_HDA_CH: i32 = 1501;
/* AFE */
pub const SOF_TKN_MEDIATEK_AFE_RATE: i32 = 1600;
pub const SOF_TKN_MEDIATEK_AFE_CH: i32 = 1601;
pub const SOF_TKN_MEDIATEK_AFE_FORMAT: i32 = 1602;
/* MIXER */
pub const SOF_TKN_MIXER_TYPE: i32 = 1700;
/* ACPDMIC */
pub const SOF_TKN_AMD_ACPDMIC_RATE: i32 = 1800;
pub const SOF_TKN_AMD_ACPDMIC_CH: i32 = 1801;

/* CAVS AUDIO FORMAT */
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_RATE: i32 = 1900;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_BIT_DEPTH: i32 = 1901;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_VALID_BIT_DEPTH: i32 = 1902;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_CHANNELS: i32 = 1903;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_CH_MAP: i32 = 1904;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_CH_CFG: i32 = 1905;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_INTERLEAVING_STYLE: i32 = 1906;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_FMT_CFG: i32 = 1907;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_SAMPLE_TYPE: i32 = 1908;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_INPUT_PIN_INDEX: i32 = 1909;
/* intentional token numbering discontinuity, reserved for future use */
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_RATE: i32 = 1930;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_BIT_DEPTH: i32 = 1931;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_VALID_BIT_DEPTH: i32 = 1932;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_CHANNELS: i32 = 1933;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_CH_MAP: i32 = 1934;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_CH_CFG: i32 = 1935;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_INTERLEAVING_STYLE: i32 = 1936;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_FMT_CFG: i32 = 1937;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_SAMPLE_TYPE: i32 = 1938;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUTPUT_PIN_INDEX: i32 = 1939;
/* intentional token numbering discontinuity, reserved for future use */
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IBS: i32 = 1970;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OBS: i32 = 1971;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_DMA_BUFFER_SIZE: i32 = 1972;
/* COPIER */
pub const SOF_TKN_INTEL_COPIER_NODE_TYPE: i32 = 1980;
pub const SOF_TKN_INTEL_COPIER_DEEP_BUFFER_DMA_MS: i32 = 1981;
/* ACP I2S */
pub const SOF_TKN_AMD_ACPI2S_RATE: i32 = 1700;
pub const SOF_TKN_AMD_ACPI2S_CH: i32 = 1701;
pub const SOF_TKN_AMD_ACPI2S_TDM_MODE: i32 = 1702;
pub const SOF_TKN_AMD_ACPI2S_FORMAT: i32 = 1703;
/* MICFIL PDM */
pub const SOF_TKN_IMX_MICFIL_RATE: i32 = 2000;
pub const SOF_TKN_IMX_MICFIL_CH: i32 = 2001;
/* ACP SDW */
pub const SOF_TKN_AMD_ACP_SDW_RATE: i32 = 2100;
pub const SOF_TKN_AMD_ACP_SDW_CH: i32 = 2101;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
