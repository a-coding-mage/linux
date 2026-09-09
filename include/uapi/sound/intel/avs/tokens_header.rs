/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright(c) 2021 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

// C enum translated as constants because the source contains intentionally
// repeated discriminant values for distinct token namespaces.

/* struct avs_tplg */
pub const AVS_TKN_MANIFEST_NAME_STRING: u32 = 1;
pub const AVS_TKN_MANIFEST_VERSION_U32: u32 = 2;
pub const AVS_TKN_MANIFEST_NUM_LIBRARIES_U32: u32 = 3;
pub const AVS_TKN_MANIFEST_NUM_AFMTS_U32: u32 = 4;
pub const AVS_TKN_MANIFEST_NUM_MODCFGS_BASE_U32: u32 = 5;
pub const AVS_TKN_MANIFEST_NUM_MODCFGS_EXT_U32: u32 = 6;
pub const AVS_TKN_MANIFEST_NUM_PPLCFGS_U32: u32 = 7;
pub const AVS_TKN_MANIFEST_NUM_BINDINGS_U32: u32 = 8;
pub const AVS_TKN_MANIFEST_NUM_CONDPATH_TMPLS_U32: u32 = 9;
pub const AVS_TKN_MANIFEST_NUM_INIT_CONFIGS_U32: u32 = 10;
pub const AVS_TKN_MANIFEST_NUM_NHLT_CONFIGS_U32: u32 = 11;

/* struct avs_tplg_library */
pub const AVS_TKN_LIBRARY_ID_U32: u32 = 101;
pub const AVS_TKN_LIBRARY_NAME_STRING: u32 = 102;

/* struct avs_audio_format */
pub const AVS_TKN_AFMT_ID_U32: u32 = 201;
pub const AVS_TKN_AFMT_SAMPLE_RATE_U32: u32 = 202;
pub const AVS_TKN_AFMT_BIT_DEPTH_U32: u32 = 203;
pub const AVS_TKN_AFMT_CHANNEL_MAP_U32: u32 = 204;
pub const AVS_TKN_AFMT_CHANNEL_CFG_U32: u32 = 205;
pub const AVS_TKN_AFMT_INTERLEAVING_U32: u32 = 206;
pub const AVS_TKN_AFMT_NUM_CHANNELS_U32: u32 = 207;
pub const AVS_TKN_AFMT_VALID_BIT_DEPTH_U32: u32 = 208;
pub const AVS_TKN_AFMT_SAMPLE_TYPE_U32: u32 = 209;

/* struct avs_tplg_modcfg_base */
pub const AVS_TKN_MODCFG_BASE_ID_U32: u32 = 301;
pub const AVS_TKN_MODCFG_BASE_CPC_U32: u32 = 302;
pub const AVS_TKN_MODCFG_BASE_IBS_U32: u32 = 303;
pub const AVS_TKN_MODCFG_BASE_OBS_U32: u32 = 304;
pub const AVS_TKN_MODCFG_BASE_PAGES_U32: u32 = 305;

/* struct avs_tplg_modcfg_ext */
pub const AVS_TKN_MODCFG_EXT_ID_U32: u32 = 401;
pub const AVS_TKN_MODCFG_EXT_TYPE_UUID: u32 = 402;
pub const AVS_TKN_MODCFG_CPR_OUT_AFMT_ID_U32: u32 = 403;
pub const AVS_TKN_MODCFG_CPR_FEATURE_MASK_U32: u32 = 404;
pub const AVS_TKN_MODCFG_CPR_DMA_TYPE_U32: u32 = 405;
pub const AVS_TKN_MODCFG_CPR_DMABUFF_SIZE_U32: u32 = 406;
pub const AVS_TKN_MODCFG_CPR_VINDEX_U8: u32 = 407;
pub const AVS_TKN_MODCFG_CPR_BLOB_FMT_ID_U32: u32 = 408;
pub const AVS_TKN_MODCFG_MICSEL_OUT_AFMT_ID_U32: u32 = 409;
pub const AVS_TKN_MODCFG_INTELWOV_CPC_LP_MODE_U32: u32 = 410;
pub const AVS_TKN_MODCFG_SRC_OUT_FREQ_U32: u32 = 411;
pub const AVS_TKN_MODCFG_MUX_REF_AFMT_ID_U32: u32 = 412;
pub const AVS_TKN_MODCFG_MUX_OUT_AFMT_ID_U32: u32 = 413;
pub const AVS_TKN_MODCFG_AEC_REF_AFMT_ID_U32: u32 = 414;
pub const AVS_TKN_MODCFG_AEC_OUT_AFMT_ID_U32: u32 = 415;
pub const AVS_TKN_MODCFG_AEC_CPC_LP_MODE_U32: u32 = 416;
pub const AVS_TKN_MODCFG_ASRC_OUT_FREQ_U32: u32 = 417;
pub const AVS_TKN_MODCFG_ASRC_MODE_U8: u32 = 418;
pub const AVS_TKN_MODCFG_ASRC_DISABLE_JITTER_U8: u32 = 419;
pub const AVS_TKN_MODCFG_UPDOWN_MIX_OUT_CHAN_CFG_U32: u32 = 420;
pub const AVS_TKN_MODCFG_UPDOWN_MIX_COEFF_SELECT_U32: u32 = 421;
pub const AVS_TKN_MODCFG_UPDOWN_MIX_COEFF_0_S32: u32 = 422;
pub const AVS_TKN_MODCFG_UPDOWN_MIX_COEFF_1_S32: u32 = 423;
pub const AVS_TKN_MODCFG_UPDOWN_MIX_COEFF_2_S32: u32 = 424;
pub const AVS_TKN_MODCFG_UPDOWN_MIX_COEFF_3_S32: u32 = 425;
pub const AVS_TKN_MODCFG_UPDOWN_MIX_COEFF_4_S32: u32 = 426;
pub const AVS_TKN_MODCFG_UPDOWN_MIX_COEFF_5_S32: u32 = 427;
pub const AVS_TKN_MODCFG_UPDOWN_MIX_COEFF_6_S32: u32 = 428;
pub const AVS_TKN_MODCFG_UPDOWN_MIX_COEFF_7_S32: u32 = 429;
pub const AVS_TKN_MODCFG_UPDOWN_MIX_CHAN_MAP_U32: u32 = 430;
pub const AVS_TKN_MODCFG_EXT_NUM_INPUT_PINS_U16: u32 = 431;
pub const AVS_TKN_MODCFG_EXT_NUM_OUTPUT_PINS_U16: u32 = 432;
pub const AVS_TKN_MODCFG_WHM_REF_AFMT_ID_U32: u32 = 433;
pub const AVS_TKN_MODCFG_WHM_OUT_AFMT_ID_U32: u32 = 434;
pub const AVS_TKN_MODCFG_WHM_WAKE_TICK_PERIOD_U32: u32 = 435;
pub const AVS_TKN_MODCFG_WHM_VINDEX_U8: u32 = 436;
pub const AVS_TKN_MODCFG_WHM_DMA_TYPE_U32: u32 = 437;
pub const AVS_TKN_MODCFG_WHM_DMABUFF_SIZE_U32: u32 = 438;
pub const AVS_TKN_MODCFG_WHM_BLOB_AFMT_ID_U32: u32 = 439;
pub const AVS_TKN_MODCFG_PEAKVOL_VOLUME_U32: u32 = 440;
pub const AVS_TKN_MODCFG_PEAKVOL_CHANNEL_ID_U32: u32 = 441; /* reserved */
pub const AVS_TKN_MODCFG_PEAKVOL_CURVE_TYPE_U32: u32 = 442;
pub const AVS_TKN_MODCFG_PEAKVOL_CURVE_DURATION_U32: u32 = 443;

/* struct avs_tplg_pplcfg */
pub const AVS_TKN_PPLCFG_ID_U32: u32 = 1401;
pub const AVS_TKN_PPLCFG_REQ_SIZE_U16: u32 = 1402;
pub const AVS_TKN_PPLCFG_PRIORITY_U8: u32 = 1403;
pub const AVS_TKN_PPLCFG_LOW_POWER_BOOL: u32 = 1404;
pub const AVS_TKN_PPLCFG_ATTRIBUTES_U16: u32 = 1405;
pub const AVS_TKN_PPLCFG_TRIGGER_U32: u32 = 1406;

/* struct avs_tplg_binding */
pub const AVS_TKN_BINDING_ID_U32: u32 = 1501;
pub const AVS_TKN_BINDING_TARGET_TPLG_NAME_STRING: u32 = 1502;
pub const AVS_TKN_BINDING_TARGET_PATH_TMPL_ID_U32: u32 = 1503;
pub const AVS_TKN_BINDING_TARGET_PPL_ID_U32: u32 = 1504;
pub const AVS_TKN_BINDING_TARGET_MOD_ID_U32: u32 = 1505;
pub const AVS_TKN_BINDING_TARGET_MOD_PIN_U8: u32 = 1506;
pub const AVS_TKN_BINDING_MOD_ID_U32: u32 = 1507;
pub const AVS_TKN_BINDING_MOD_PIN_U8: u32 = 1508;
pub const AVS_TKN_BINDING_IS_SINK_U8: u32 = 1509;

/* struct avs_tplg_pipeline */
pub const AVS_TKN_PPL_ID_U32: u32 = 1601;
pub const AVS_TKN_PPL_PPLCFG_ID_U32: u32 = 1602;
pub const AVS_TKN_PPL_NUM_BINDING_IDS_U32: u32 = 1603;
pub const AVS_TKN_PPL_BINDING_ID_U32: u32 = 1604;

/* struct avs_tplg_module */
pub const AVS_TKN_MOD_ID_U32: u32 = 1701;
pub const AVS_TKN_MOD_MODCFG_BASE_ID_U32: u32 = 1702;
pub const AVS_TKN_MOD_IN_AFMT_ID_U32: u32 = 1703;
pub const AVS_TKN_MOD_CORE_ID_U8: u32 = 1704;
pub const AVS_TKN_MOD_PROC_DOMAIN_U8: u32 = 1705;
pub const AVS_TKN_MOD_MODCFG_EXT_ID_U32: u32 = 1706;
pub const AVS_TKN_MOD_KCONTROL_ID_U32: u32 = 1707;
pub const AVS_TKN_MOD_INIT_CONFIG_NUM_IDS_U32: u32 = 1708;
pub const AVS_TKN_MOD_INIT_CONFIG_ID_U32: u32 = 1709;
pub const AVS_TKN_MOD_NHLT_CONFIG_ID_U32: u32 = 1710;

/* struct avs_tplg_path_template */
pub const AVS_TKN_PATH_TMPL_ID_U32: u32 = 1801;

/* struct avs_tplg_path */
pub const AVS_TKN_PATH_ID_U32: u32 = 1901;
pub const AVS_TKN_PATH_FE_FMT_ID_U32: u32 = 1902;
pub const AVS_TKN_PATH_BE_FMT_ID_U32: u32 = 1903;

/* struct avs_tplg_path_template (conditional) */
pub const AVS_TKN_CONDPATH_TMPL_ID_U32: u32 = 1801;
pub const AVS_TKN_CONDPATH_TMPL_SOURCE_TPLG_NAME_STRING: u32 = 2002;
pub const AVS_TKN_CONDPATH_TMPL_SOURCE_PATH_TMPL_ID_U32: u32 = 2003;
pub const AVS_TKN_CONDPATH_TMPL_SINK_TPLG_NAME_STRING: u32 = 2004;
pub const AVS_TKN_CONDPATH_TMPL_SINK_PATH_TMPL_ID_U32: u32 = 2005;
pub const AVS_TKN_CONDPATH_TMPL_COND_TYPE_U32: u32 = 2006;
pub const AVS_TKN_CONDPATH_TMPL_OVERRIDABLE_BOOL: u32 = 2007;
pub const AVS_TKN_CONDPATH_TMPL_PRIORITY_U8: u32 = 2008;

/* struct avs_tplg_path (conditional) */
pub const AVS_TKN_CONDPATH_ID_U32: u32 = 1901;
pub const AVS_TKN_CONDPATH_SOURCE_PATH_ID_U32: u32 = 2102;
pub const AVS_TKN_CONDPATH_SINK_PATH_ID_U32: u32 = 2103;

/* struct avs_tplg_pin_format */
pub const AVS_TKN_PIN_FMT_INDEX_U32: u32 = 2201;
pub const AVS_TKN_PIN_FMT_IOBS_U32: u32 = 2202;
pub const AVS_TKN_PIN_FMT_AFMT_ID_U32: u32 = 2203;

/* struct avs_tplg_kcontrol */
pub const AVS_TKN_KCONTROL_ID_U32: u32 = 2301;

/* struct avs_tplg_init_config */
pub const AVS_TKN_INIT_CONFIG_ID_U32: u32 = 2401;
pub const AVS_TKN_INIT_CONFIG_PARAM_U8: u32 = 2402;
pub const AVS_TKN_INIT_CONFIG_LENGTH_U32: u32 = 2403;

/* struct avs_tplg_nhlt_config */
pub const AVS_TKN_NHLT_CONFIG_ID_U32: u32 = 2501;
pub const AVS_TKN_NHLT_CONFIG_SIZE_U32: u32 = 2502;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
