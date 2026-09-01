/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  sst-atom-controls.h - Intel MID Platform driver header file
 *
 *  Copyright (C) 2013-14 Intel Corp
 *  Author: Ramesh Babu <ramesh.babu.koul@intel.com>
 *  	Omair M Abdullah <omair.m.abdullah@intel.com>
 *  	Samreen Nilofer <samreen.nilofer@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

/* Dependencies from C header:
 * #include <sound/soc.h>
 * #include <sound/tlv.h>
 */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};

pub const MERR_DPCM_AUDIO: c_int = 0;
pub const MERR_DPCM_DEEP_BUFFER: c_int = 1;
pub const MERR_DPCM_COMPR: c_int = 2;

/* define a bit for each mixer input */
pub const fn SST_MIX_IP(x: c_int) -> c_int {
    x
}

pub const SST_IP_MODEM: c_int = SST_MIX_IP(0);
pub const SST_IP_BT: c_int = SST_MIX_IP(1);
pub const SST_IP_CODEC0: c_int = SST_MIX_IP(2);
pub const SST_IP_CODEC1: c_int = SST_MIX_IP(3);
pub const SST_IP_LOOP0: c_int = SST_MIX_IP(4);
pub const SST_IP_LOOP1: c_int = SST_MIX_IP(5);
pub const SST_IP_LOOP2: c_int = SST_MIX_IP(6);
pub const SST_IP_PROBE: c_int = SST_MIX_IP(7);
pub const SST_IP_VOIP: c_int = SST_MIX_IP(12);
pub const SST_IP_PCM0: c_int = SST_MIX_IP(13);
pub const SST_IP_PCM1: c_int = SST_MIX_IP(14);
pub const SST_IP_MEDIA0: c_int = SST_MIX_IP(17);
pub const SST_IP_MEDIA1: c_int = SST_MIX_IP(18);
pub const SST_IP_MEDIA2: c_int = SST_MIX_IP(19);
pub const SST_IP_MEDIA3: c_int = SST_MIX_IP(20);

pub const SST_IP_LAST: c_int = SST_IP_MEDIA3;

pub const SST_SWM_INPUT_COUNT: c_int = SST_IP_LAST + 1;
pub const SST_CMD_SWM_MAX_INPUTS: usize = 6;

pub const SST_PATH_ID_SHIFT: c_int = 8;
pub const SST_DEFAULT_LOCATION_ID: u16 = 0xFFFF;
pub const SST_DEFAULT_CELL_NBR: u16 = 0xFF;
pub const SST_DEFAULT_MODULE_ID: u16 = 0xFFFF;

/*
 * Audio DSP Path Ids. Specified by the audio DSP FW
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_path_index {
    SST_PATH_INDEX_MODEM_OUT = 0x00 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_CODEC_OUT0 = 0x02 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_CODEC_OUT1 = 0x03 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_SPROT_LOOP_OUT = 0x04 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_MEDIA_LOOP1_OUT = 0x05 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_MEDIA_LOOP2_OUT = 0x06 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_VOIP_OUT = 0x0C << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_PCM0_OUT = 0x0D << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_PCM1_OUT = 0x0E << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_PCM2_OUT = 0x0F << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_MEDIA0_OUT = 0x12 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_MEDIA1_OUT = 0x13 << SST_PATH_ID_SHIFT,

    /* Start of input paths */
    SST_PATH_INDEX_MODEM_IN = 0x80 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_CODEC_IN0 = 0x82 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_CODEC_IN1 = 0x83 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_SPROT_LOOP_IN = 0x84 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_MEDIA_LOOP1_IN = 0x85 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_MEDIA_LOOP2_IN = 0x86 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_VOIP_IN = 0x8C << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_PCM0_IN = 0x8D << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_PCM1_IN = 0x8E << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_MEDIA0_IN = 0x8F << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_MEDIA1_IN = 0x90 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_MEDIA2_IN = 0x91 << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_MEDIA3_IN = 0x9C << SST_PATH_ID_SHIFT,
    SST_PATH_INDEX_RESERVED = 0xFF << SST_PATH_ID_SHIFT,
}

/*
 * path IDs
 */
pub const SST_SWM_IN_MODEM: c_int = (sst_path_index::SST_PATH_INDEX_MODEM_IN as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_IN_CODEC0: c_int = (sst_path_index::SST_PATH_INDEX_CODEC_IN0 as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_IN_CODEC1: c_int = (sst_path_index::SST_PATH_INDEX_CODEC_IN1 as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_IN_SPROT_LOOP: c_int = (sst_path_index::SST_PATH_INDEX_SPROT_LOOP_IN as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_IN_MEDIA_LOOP1: c_int = (sst_path_index::SST_PATH_INDEX_MEDIA_LOOP1_IN as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_IN_MEDIA_LOOP2: c_int = (sst_path_index::SST_PATH_INDEX_MEDIA_LOOP2_IN as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_IN_VOIP: c_int = (sst_path_index::SST_PATH_INDEX_VOIP_IN as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_IN_PCM0: c_int = (sst_path_index::SST_PATH_INDEX_PCM0_IN as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_IN_PCM1: c_int = (sst_path_index::SST_PATH_INDEX_PCM1_IN as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_IN_MEDIA0: c_int = (sst_path_index::SST_PATH_INDEX_MEDIA0_IN as c_int) | SST_DEFAULT_CELL_NBR as c_int; /* Part of Media Mixer */
pub const SST_SWM_IN_MEDIA1: c_int = (sst_path_index::SST_PATH_INDEX_MEDIA1_IN as c_int) | SST_DEFAULT_CELL_NBR as c_int; /* Part of Media Mixer */
pub const SST_SWM_IN_MEDIA2: c_int = (sst_path_index::SST_PATH_INDEX_MEDIA2_IN as c_int) | SST_DEFAULT_CELL_NBR as c_int; /* Part of Media Mixer */
pub const SST_SWM_IN_MEDIA3: c_int = (sst_path_index::SST_PATH_INDEX_MEDIA3_IN as c_int) | SST_DEFAULT_CELL_NBR as c_int; /* Part of Media Mixer */
pub const SST_SWM_IN_END: c_int = (sst_path_index::SST_PATH_INDEX_RESERVED as c_int) | SST_DEFAULT_CELL_NBR as c_int;

/*
 * path IDs
 */
pub const SST_SWM_OUT_MODEM: c_int = (sst_path_index::SST_PATH_INDEX_MODEM_OUT as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_OUT_CODEC0: c_int = (sst_path_index::SST_PATH_INDEX_CODEC_OUT0 as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_OUT_CODEC1: c_int = (sst_path_index::SST_PATH_INDEX_CODEC_OUT1 as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_OUT_SPROT_LOOP: c_int = (sst_path_index::SST_PATH_INDEX_SPROT_LOOP_OUT as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_OUT_MEDIA_LOOP1: c_int = (sst_path_index::SST_PATH_INDEX_MEDIA_LOOP1_OUT as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_OUT_MEDIA_LOOP2: c_int = (sst_path_index::SST_PATH_INDEX_MEDIA_LOOP2_OUT as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_OUT_VOIP: c_int = (sst_path_index::SST_PATH_INDEX_VOIP_OUT as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_OUT_PCM0: c_int = (sst_path_index::SST_PATH_INDEX_PCM0_OUT as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_OUT_PCM1: c_int = (sst_path_index::SST_PATH_INDEX_PCM1_OUT as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_OUT_PCM2: c_int = (sst_path_index::SST_PATH_INDEX_PCM2_OUT as c_int) | SST_DEFAULT_CELL_NBR as c_int;
pub const SST_SWM_OUT_MEDIA0: c_int = (sst_path_index::SST_PATH_INDEX_MEDIA0_OUT as c_int) | SST_DEFAULT_CELL_NBR as c_int; /* Part of Media Mixer */
pub const SST_SWM_OUT_MEDIA1: c_int = (sst_path_index::SST_PATH_INDEX_MEDIA1_OUT as c_int) | SST_DEFAULT_CELL_NBR as c_int; /* Part of Media Mixer */
pub const SST_SWM_OUT_END: c_int = (sst_path_index::SST_PATH_INDEX_RESERVED as c_int) | SST_DEFAULT_CELL_NBR as c_int;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_ipc_msg {
    SST_IPC_IA_CMD = 1,
    SST_IPC_IA_SET_PARAMS,
    SST_IPC_IA_GET_PARAMS,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_cmd_type {
    SST_CMD_BYTES_SET = 1,
    SST_CMD_BYTES_GET = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_task {
    SST_TASK_SBA = 1,
    SST_TASK_MMX = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_type {
    SST_TYPE_CMD = 1,
    SST_TYPE_PARAMS,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_flag {
    SST_FLAG_BLOCKED = 1,
    SST_FLAG_NONBLOCK,
}

/*
 * Enumeration for indexing the gain cells in VB_SET_GAIN DSP command
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_gain_index {
    /* GAIN IDs for SB task start here */
    SST_GAIN_INDEX_CODEC_OUT0,
    SST_GAIN_INDEX_CODEC_OUT1,
    SST_GAIN_INDEX_CODEC_IN0,
    SST_GAIN_INDEX_CODEC_IN1,
    SST_GAIN_INDEX_SPROT_LOOP_OUT,
    SST_GAIN_INDEX_MEDIA_LOOP1_OUT,
    SST_GAIN_INDEX_MEDIA_LOOP2_OUT,
    SST_GAIN_INDEX_PCM0_IN_LEFT,
    SST_GAIN_INDEX_PCM0_IN_RIGHT,
    SST_GAIN_INDEX_PCM1_OUT_LEFT,
    SST_GAIN_INDEX_PCM1_OUT_RIGHT,
    SST_GAIN_INDEX_PCM1_IN_LEFT,
    SST_GAIN_INDEX_PCM1_IN_RIGHT,
    SST_GAIN_INDEX_PCM2_OUT_LEFT,
    SST_GAIN_INDEX_PCM2_OUT_RIGHT,
    SST_GAIN_INDEX_VOIP_OUT,
    SST_GAIN_INDEX_VOIP_IN,

    /* Gain IDs for MMX task start here */
    SST_GAIN_INDEX_MEDIA0_IN_LEFT,
    SST_GAIN_INDEX_MEDIA0_IN_RIGHT,
    SST_GAIN_INDEX_MEDIA1_IN_LEFT,
    SST_GAIN_INDEX_MEDIA1_IN_RIGHT,
    SST_GAIN_INDEX_MEDIA2_IN_LEFT,
    SST_GAIN_INDEX_MEDIA2_IN_RIGHT,
    SST_GAIN_INDEX_GAIN_END,
}

/*
 * Audio DSP module IDs specified by FW spec
 * TODO: Update with all modules
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_module_id {
    SST_MODULE_ID_PCM = 0x0001,
    SST_MODULE_ID_MP3 = 0x0002,
    SST_MODULE_ID_MP24 = 0x0003,
    SST_MODULE_ID_AAC = 0x0004,
    SST_MODULE_ID_AACP = 0x0005,
    SST_MODULE_ID_EAACP = 0x0006,
    SST_MODULE_ID_WMA9 = 0x0007,
    SST_MODULE_ID_WMA10 = 0x0008,
    SST_MODULE_ID_WMA10P = 0x0009,
    SST_MODULE_ID_RA = 0x000A,
    SST_MODULE_ID_DDAC3 = 0x000B,
    SST_MODULE_ID_TRUE_HD = 0x000C,
    SST_MODULE_ID_HD_PLUS = 0x000D,
    SST_MODULE_ID_SRC = 0x0064,
    SST_MODULE_ID_DOWNMIX = 0x0066,
    SST_MODULE_ID_GAIN_CELL = 0x0067,
    SST_MODULE_ID_SPROT = 0x006D,
    SST_MODULE_ID_BASS_BOOST = 0x006E,
    SST_MODULE_ID_STEREO_WDNG = 0x006F,
    SST_MODULE_ID_AV_REMOVAL = 0x0070,
    SST_MODULE_ID_MIC_EQ = 0x0071,
    SST_MODULE_ID_SPL = 0x0072,
    SST_MODULE_ID_ALGO_VTSV = 0x0073,
    SST_MODULE_ID_NR = 0x0076,
    SST_MODULE_ID_BWX = 0x0077,
    SST_MODULE_ID_DRP = 0x0078,
    SST_MODULE_ID_MDRP = 0x0079,
    SST_MODULE_ID_ANA = 0x007A,
    SST_MODULE_ID_AEC = 0x007B,
    SST_MODULE_ID_NR_SNS = 0x007C,
    SST_MODULE_ID_SER = 0x007D,
    SST_MODULE_ID_AGC = 0x007E,
    SST_MODULE_ID_CNI = 0x007F,
    SST_MODULE_ID_CONTEXT_ALGO_AWARE = 0x0080,
    SST_MODULE_ID_FIR_24 = 0x0081,
    SST_MODULE_ID_IIR_24 = 0x0082,
    SST_MODULE_ID_ASRC = 0x0083,
    SST_MODULE_ID_TONE_GEN = 0x0084,
    SST_MODULE_ID_BMF = 0x0086,
    SST_MODULE_ID_EDL = 0x0087,
    SST_MODULE_ID_GLC = 0x0088,
    SST_MODULE_ID_FIR_16 = 0x0089,
    SST_MODULE_ID_IIR_16 = 0x008A,
    SST_MODULE_ID_DNR = 0x008B,
    SST_MODULE_ID_VIRTUALIZER = 0x008C,
    SST_MODULE_ID_VISUALIZATION = 0x008D,
    SST_MODULE_ID_LOUDNESS_OPTIMIZER = 0x008E,
    SST_MODULE_ID_REVERBERATION = 0x008F,
    SST_MODULE_ID_CNI_TX = 0x0090,
    SST_MODULE_ID_REF_LINE = 0x0091,
    SST_MODULE_ID_VOLUME = 0x0092,
    SST_MODULE_ID_FILT_DCR = 0x0094,
    SST_MODULE_ID_SLV = 0x009A,
    SST_MODULE_ID_NLF = 0x009B,
    SST_MODULE_ID_TNR = 0x009C,
    SST_MODULE_ID_WNR = 0x009D,
    SST_MODULE_ID_LOG = 0xFF00,
    SST_MODULE_ID_TASK = 0xFFFF,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_cmd {
    SBA_IDLE = 14,
    SBA_VB_SET_SPEECH_PATH = 26,
    MMX_SET_GAIN = 33,
    SBA_VB_SET_GAIN = 33,
    FBA_VB_RX_CNI = 35,
    MMX_SET_GAIN_TIMECONST = 36,
    SBA_VB_SET_TIMECONST = 36,
    SBA_VB_START = 85,
    SBA_SET_SWM = 114,
    SBA_SET_MDRP = 116,
    SBA_HW_SET_SSP = 117,
    SBA_SET_MEDIA_LOOP_MAP = 118,
    SBA_SET_MEDIA_PATH = 119,
    MMX_SET_MEDIA_PATH = 119,
    SBA_VB_LPRO = 126,
    SBA_VB_SET_FIR = 128,
    SBA_VB_SET_IIR = 129,
    SBA_SET_SSP_SLOT_MAP = 130,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_dsp_switch {
    SST_SWITCH_OFF = 0,
    SST_SWITCH_ON = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_path_switch {
    SST_PATH_OFF = 0,
    SST_PATH_ON = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_swm_state {
    SST_SWM_OFF = 0,
    SST_SWM_ON = 3,
}

#[inline]
pub unsafe fn SST_FILL_LOCATION_IDS(dst: *mut sst_destination_id, cell_idx: u8, pipe_id: u8) {
    unsafe {
        (*dst).location_id.p.cell_nbr_idx = cell_idx;
        (*dst).location_id.p.path_id = pipe_id;
    }
}

#[inline]
pub unsafe fn SST_FILL_LOCATION_ID(dst: *mut sst_destination_id, loc_id: u16) {
    unsafe {
        (*dst).location_id.f = loc_id;
    }
}

#[inline]
pub unsafe fn SST_FILL_MODULE_ID(dst: *mut sst_destination_id, mod_id: u16) {
    unsafe {
        (*dst).module_id = mod_id;
    }
}

#[inline]
pub unsafe fn SST_FILL_DESTINATION1(dst: *mut sst_destination_id, id: u32) {
    unsafe {
        SST_FILL_LOCATION_ID(dst, (id & 0xFFFF) as u16);
        SST_FILL_MODULE_ID(dst, ((id & 0xFFFF0000) >> 16) as u16);
    }
}

#[inline]
pub unsafe fn SST_FILL_DESTINATION2(dst: *mut sst_destination_id, loc_id: u16, mod_id: u16) {
    unsafe {
        SST_FILL_LOCATION_ID(dst, loc_id);
        SST_FILL_MODULE_ID(dst, mod_id);
    }
}

#[inline]
pub unsafe fn SST_FILL_DESTINATION3(dst: *mut sst_destination_id, cell_idx: u8, path_id: u8, mod_id: u16) {
    unsafe {
        SST_FILL_LOCATION_IDS(dst, cell_idx, path_id);
        SST_FILL_MODULE_ID(dst, mod_id);
    }
}

#[inline]
pub unsafe fn SST_FILL_DEFAULT_DESTINATION(dst: *mut sst_destination_id) {
    unsafe {
        SST_FILL_DESTINATION2(dst, SST_DEFAULT_LOCATION_ID, SST_DEFAULT_MODULE_ID);
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_location_id_p {
    pub cell_nbr_idx: u8, /* module index */
    pub path_id: u8,     /* pipe_id */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sst_location_id {
    pub p: sst_location_id_p, /* part */
    pub f: u16,               /* full */
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_destination_id {
    pub location_id: sst_location_id,
    pub module_id: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_dsp_header {
    pub dst: sst_destination_id,
    pub command_id: u16,
    pub length: u16,
}

/*
 *
 * Common Commands
 *
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_cmd_generic {
    pub header: sst_dsp_header,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct swm_input_ids {
    pub input_id: sst_destination_id,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_cmd_set_swm {
    pub header: sst_dsp_header,
    pub output_id: sst_destination_id,
    pub switch_state: u16,
    pub nb_inputs: u16,
    pub input: [swm_input_ids; SST_CMD_SWM_MAX_INPUTS],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_cmd_set_media_path {
    pub header: sst_dsp_header,
    pub switch_state: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct pcm_cfg {
    /* C bitfields: u8 s_length:2; u8 rate:3; u8 format:3; */
    pub bitfield_1: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_cmd_set_speech_path_config {
    /* C bitfield: u16 rsvd:8; followed by struct pcm_cfg cfg. */
    pub rsvd: u8,
    pub cfg: pcm_cfg,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_cmd_set_speech_path {
    pub header: sst_dsp_header,
    pub switch_state: u16,
    pub config: sst_cmd_set_speech_path_config,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct gain_cell {
    pub dest: sst_destination_id,
    pub cell_gain_left: i16,
    pub cell_gain_right: i16,
    pub gain_time_constant: u16,
}

pub const NUM_GAIN_CELLS: usize = 1;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_cmd_set_gain_dual {
    pub header: sst_dsp_header,
    pub gain_cell_num: u16,
    pub cell_gains: [gain_cell; NUM_GAIN_CELLS],
}

#[repr(C, packed)]
pub struct sst_cmd_set_params {
    pub dst: sst_destination_id,
    pub command_id: u16,
    pub params: [c_char; 0],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_cmd_sba_vb_start {
    pub header: sst_dsp_header,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sba_media_loop_params_part {
    /* C bitfield: u16 rsvd:8; followed by struct pcm_cfg cfg. */
    pub rsvd: u8,
    pub cfg: pcm_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sba_media_loop_params {
    pub part: sba_media_loop_params_part,
    pub full: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_cmd_sba_set_media_loop_map {
    pub header: sst_dsp_header,
    pub switch_state: u16,
    pub param: sba_media_loop_params,
    pub map: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_cmd_tone_stop {
    pub header: sst_dsp_header,
    pub switch_state: u16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_ssp_mode {
    SSP_MODE_PROVIDER = 0,
    SSP_MODE_CONSUMER = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_ssp_pcm_mode {
    SSP_PCM_MODE_NORMAL = 0,
    SSP_PCM_MODE_NETWORK = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_ssp_duplex {
    SSP_DUPLEX = 0,
    SSP_RX = 1,
    SSP_TX = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_ssp_fs_frequency {
    SSP_FS_8_KHZ = 0,
    SSP_FS_16_KHZ = 1,
    SSP_FS_44_1_KHZ = 2,
    SSP_FS_48_KHZ = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_ssp_fs_polarity {
    SSP_FS_ACTIVE_LOW = 0,
    SSP_FS_ACTIVE_HIGH = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_ssp_protocol {
    SSP_MODE_PCM = 0,
    SSP_MODE_I2S = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_ssp_port_id {
    SSP_MODEM = 0,
    SSP_BT = 1,
    SSP_FM = 2,
    SSP_CODEC = 3,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_cmd_sba_hw_set_ssp {
    pub header: sst_dsp_header,
    pub selection: u16, /* 0:SSP0(def), 1:SSP1, 2:SSP2 */
    pub switch_state: u16,
    /* C bitfields: nb_bits_per_slots:6, nb_slots:4, mode:3, duplex:3 */
    pub bits_slots_mode_duplex: u16,
    /* C bitfields: active_tx_slot_map:8, reserved1:8 */
    pub active_tx_slot_map_reserved1: u16,
    /* C bitfields: active_rx_slot_map:8, reserved2:8 */
    pub active_rx_slot_map_reserved2: u16,
    pub frame_sync_frequency: u16,
    /* C bitfields: frame_sync_polarity:8, data_polarity:8 */
    pub frame_sync_polarity_data_polarity: u16,
    pub frame_sync_width: u16, /* 1 to N clocks */
    /* C bitfields: ssp_protocol:8, start_delay:8 */
    pub ssp_protocol_start_delay: u16,
}

pub const SST_MAX_TDM_SLOTS: usize = 8;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_param_sba_ssp_slot_map {
    pub header: sst_dsp_header,
    pub param_id: u16,
    pub param_len: u16,
    pub ssp_index: u16,
    pub rx_slot_map: [u8; SST_MAX_TDM_SLOTS],
    pub tx_slot_map: [u8; SST_MAX_TDM_SLOTS],
}

pub const SST_PROBE_EXTRACTOR: c_int = 0;
pub const SST_PROBE_INJECTOR: c_int = 1;

/**** widget defines *****/

pub const SST_MODULE_GAIN: c_int = 1;
pub const SST_MODULE_ALGO: c_int = 2;

pub const SST_FMT_MONO: c_int = 0;
pub const SST_FMT_STEREO: c_int = 3;

/* physical SSP numbers */
pub const SST_SSP0: c_int = 0;
pub const SST_SSP1: c_int = 1;
pub const SST_SSP2: c_int = 2;
pub const SST_SSP_LAST: c_int = SST_SSP2;

pub const SST_NUM_SSPS: c_int = SST_SSP_LAST + 1; /* physical SSPs */
pub const SST_MAX_SSP_MUX: usize = 2; /* single SSP muxed between pipes */
pub const SST_MAX_SSP_DOMAINS: usize = 2; /* domains present in each pipe */

#[repr(C)]
pub struct snd_kcontrol {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sst_pcm_format {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sst_module {
    pub kctl: *mut snd_kcontrol,
    pub node: list_head,
}

#[repr(C)]
pub struct sst_ssp_config {
    pub ssp_id: u8,
    pub bits_per_slot: u8,
    pub slots: u8,
    pub ssp_mode: u8,
    pub pcm_mode: u8,
    pub duplex: u8,
    pub ssp_protocol: u8,
    pub fs_frequency: u8,
    pub active_slot_map: u8,
    pub start_delay: u8,
    pub fs_width: u16,
    pub frame_sync_polarity: u8,
    pub data_polarity: u8,
}

#[repr(C)]
pub struct sst_ssp_cfg {
    pub ssp_number: u8,
    pub mux_shift: *const c_int,
    pub domain_shift: *const [c_int; SST_MAX_SSP_MUX],
    pub ssp_config: *const [[sst_ssp_config; SST_MAX_SSP_DOMAINS]; SST_MAX_SSP_MUX],
}

#[repr(C)]
pub struct sst_ids {
    pub location_id: u16,
    pub module_id: u16,
    pub task_id: u8,
    pub format: u8,
    pub reg: u8,
    pub parent_wname: *const c_char,
    pub parent_w: *mut snd_soc_dapm_widget,
    pub algo_list: list_head,
    pub gain_list: list_head,
    pub pcm_fmt: *const sst_pcm_format,
}

/* The SST_AIF_IN, SST_AIF_OUT, SST_INPUT, SST_OUTPUT, SST_DAPM_OUTPUT,
 * SST_PATH, SST_LINKED_PATH, SST_PATH_MEDIA_LOOP, SST_PATH_INPUT,
 * SST_PATH_LINKED_INPUT, SST_PATH_OUTPUT, SST_PATH_LINKED_OUTPUT,
 * SST_PATH_MEDIA_LOOP_OUTPUT, and SST_SWM_MIXER C macros expand to
 * initializers for externally defined snd_soc_dapm_widget structures.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_gain_kcontrol_type {
    SST_GAIN_TLV,
    SST_GAIN_MUTE,
    SST_GAIN_RAMP_DURATION,
}

#[repr(C)]
pub struct sst_gain_mixer_control {
    pub stereo: bool,
    pub type_: sst_gain_kcontrol_type,
    pub gain_val: *mut sst_gain_value,
    pub max: c_int,
    pub min: c_int,
    pub instance_id: u16,
    pub module_id: u16,
    pub pipe_id: u16,
    pub task_id: u16,
    pub pname: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub w: *mut snd_soc_dapm_widget,
}

#[repr(C)]
pub struct sst_gain_value {
    pub ramp_duration: u16,
    pub l_gain: i16,
    pub r_gain: i16,
    pub mute: bool,
}

pub const SST_GAIN_VOLUME_DEFAULT: c_int = -1440;
pub const SST_GAIN_RAMP_DURATION_DEFAULT: c_int = 5; /* timeconstant */
pub const SST_GAIN_MUTE_DEFAULT: bool = true;

/* SST_GAIN_KCONTROL_TLV, SST_GAIN_KCONTROL_INT, SST_GAIN_KCONTROL_BOOL,
 * SST_CONTROL_NAME, SST_COMBO_CONTROL_NAME, and SST_GAIN_KCONTROLS expand to
 * external ALSA control initializers and string-pasting expressions in C.
 */

pub const SST_GAIN_TC_MIN: c_int = 5;
pub const SST_GAIN_TC_MAX: c_int = 5000;
pub const SST_GAIN_MIN_VALUE: c_int = -1440; /* in 0.1 DB units */
pub const SST_GAIN_MAX_VALUE: c_int = 360;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_algo_kcontrol_type {
    SST_ALGO_PARAMS,
    SST_ALGO_BYPASS,
}

#[repr(C)]
pub struct sst_algo_control {
    pub type_: sst_algo_kcontrol_type,
    pub max: c_int,
    pub module_id: u16,
    pub pipe_id: u16,
    pub task_id: u16,
    pub cmd_id: u16,
    pub bypass: bool,
    pub params: *mut c_uchar,
    pub w: *mut snd_soc_dapm_widget,
}

/* size of the control = size of params + size of length field */
pub const fn SST_ALGO_CTL_VALUE(
    xcount: c_int,
    xtype: sst_algo_kcontrol_type,
    xpipe: u16,
    xmod: u16,
    xtask: u16,
    xcmd: u16,
) -> sst_algo_control {
    sst_algo_control {
        max: xcount + core::mem::size_of::<u16>() as c_int,
        type_: xtype,
        module_id: xmod,
        pipe_id: xpipe,
        task_id: xtask,
        cmd_id: xcmd,
        bypass: false,
        params: core::ptr::null_mut(),
        w: core::ptr::null_mut(),
    }
}

/* SST_ALGO_KCONTROL, SST_ALGO_KCONTROL_BYTES, SST_ALGO_KCONTROL_BOOL,
 * SST_ALGO_BYPASS_PARAMS, and SST_COMBO_ALGO_KCONTROL_BYTES expand to
 * externally defined ALSA mixer control initializers.
 */

#[repr(C)]
pub struct sst_enum {
    pub tx: bool,
    pub reg: c_uint,
    pub max: c_uint,
    pub texts: *const *const c_char,
    pub w: *mut snd_soc_dapm_widget,
}

/* only 4 slots/channels supported atm */
pub const fn SST_SSP_SLOT_ENUM(s_ch_no: c_uint, is_tx: bool, xtexts: *const *const c_char) -> sst_enum {
    sst_enum {
        reg: s_ch_no,
        tx: is_tx,
        max: 4 + 1,
        texts: xtexts,
        w: core::ptr::null_mut(),
    }
}

/* SST_SLOT_CTL_NAME, SST_SSP_SLOT_CTL, SST_MUX_CTL_NAME, SST_SSP_MUX_ENUM,
 * and SST_SSP_MUX_CTL expand to external ALSA control names/initializers.
 */

pub const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44;

extern "C" {
    pub fn sst_fill_ssp_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    pub fn sst_fill_ssp_config(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    pub fn sst_fill_ssp_defaults(dai: *mut snd_soc_dai);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
