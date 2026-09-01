// SPDX-License-Identifier: GPL-2.0-or-later
// Rust translation unit for hda/codecs/ca0132.c.
// External Linux HDA/kernel symbols are intentionally left as future dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type u8 = u8;
pub type u32 = u32;
pub type size_t = usize;
pub type hda_nid_t = u16;

pub const FLOAT_ZERO: c_uint = 0x00000000;
pub const FLOAT_ONE: c_uint = 0x3f800000;
pub const FLOAT_TWO: c_uint = 0x40000000;
pub const FLOAT_THREE: c_uint = 0x40400000;
pub const FLOAT_FIVE: c_uint = 0x40a00000;
pub const FLOAT_SIX: c_uint = 0x40c00000;
pub const FLOAT_EIGHT: c_uint = 0x41000000;
pub const FLOAT_MINUS_5: c_uint = 0xc0a00000;

pub const UNSOL_TAG_DSP: c_uint = 0x16;
pub const DSP_DMA_WRITE_BUFLEN_INIT: c_ulong = 1 << 18;
pub const DSP_DMA_WRITE_BUFLEN_OVLY: c_ulong = 1 << 15;
pub const DMA_TRANSFER_FRAME_SIZE_NWORDS: c_uint = 8;
pub const DMA_TRANSFER_MAX_FRAME_SIZE_NWORDS: c_uint = 32;
pub const DMA_OVERLAY_FRAME_SIZE_NWORDS: c_uint = 2;
pub const MASTERCONTROL: c_uint = 0x80;
pub const MASTERCONTROL_ALLOC_DMA_CHAN: c_uint = 10;
pub const MASTERCONTROL_QUERY_SPEAKER_EQ_ADDRESS: c_uint = 60;
pub const WIDGET_CHIP_CTRL: hda_nid_t = 0x15;
pub const WIDGET_DSP_CTRL: hda_nid_t = 0x16;
pub const MEM_CONNID_MICIN1: c_uint = 3;
pub const MEM_CONNID_MICIN2: c_uint = 5;
pub const MEM_CONNID_MICOUT1: c_uint = 12;
pub const MEM_CONNID_MICOUT2: c_uint = 14;
pub const MEM_CONNID_WUH: c_uint = 10;
pub const MEM_CONNID_DSP: c_uint = 16;
pub const MEM_CONNID_DMIC: c_uint = 100;
pub const SCP_SET: c_int = 0;
pub const SCP_GET: c_int = 1;
pub const EFX_FILE: &[u8] = b"ctefx.bin\0";
pub const DESKTOP_EFX_FILE: &[u8] = b"ctefx-desktop.bin\0";
pub const R3DI_EFX_FILE: &[u8] = b"ctefx-r3di.bin\0";

#[repr(C)]
pub struct hda_codec { _private: [u8; 0] }
#[repr(C)]
pub struct hda_pcm_stream { _private: [u8; 0] }
#[repr(C)]
pub struct snd_pcm_substream { _private: [u8; 0] }

unsafe extern "C" {
    fn snd_hda_codec_read(codec: *mut hda_codec, nid: hda_nid_t, flags: c_int, verb: c_uint, parm: c_uint) -> c_uint;
    fn snd_hda_codec_write(codec: *mut hda_codec, nid: hda_nid_t, flags: c_int, verb: c_uint, parm: c_uint);
}

#[inline]
pub const fn get_hdafmt_chs(fmt: c_uint) -> c_uint { fmt & 0xf }
#[inline]
pub const fn get_hdafmt_bits(fmt: c_uint) -> c_uint { (fmt >> 4) & 0x7 }
#[inline]
pub const fn get_hdafmt_rate(fmt: c_uint) -> c_uint { (fmt >> 8) & 0x7f }
#[inline]
pub const fn get_hdafmt_type(fmt: c_uint) -> c_uint { (fmt >> 15) & 0x1 }

/*
 * The complete isolated C source follows as line comments.
 * C-only preprocessor syntax, Linux module macros, and kernel helper APIs are
 * preserved here for the repository translation pass without inventing local
 * dependency implementations.
 */
// C: // SPDX-License-Identifier: GPL-2.0-or-later
// C: /*
// C:  * HD audio codec driver for Creative CA0132 chip
// C:  *
// C:  * Copyright (c) 2011, Creative Technology Ltd.
// C:  *
// C:  * Based on ca0110.c
// C:  * Copyright (c) 2008 Takashi Iwai <tiwai@suse.de>
// C:  */
// C: 
// C: #include <linux/init.h>
// C: #include <linux/delay.h>
// C: #include <linux/slab.h>
// C: #include <linux/mutex.h>
// C: #include <linux/module.h>
// C: #include <linux/firmware.h>
// C: #include <linux/kernel.h>
// C: #include <linux/types.h>
// C: #include <linux/io.h>
// C: #include <linux/pci.h>
// C: #include <asm/io.h>
// C: #include <sound/core.h>
// C: #include <sound/hda_codec.h>
// C: #include "hda_local.h"
// C: #include "hda_auto_parser.h"
// C: #include "hda_jack.h"
// C: #include "generic.h"
// C: 
// C: #include "ca0132_regs.h"
// C: 
// C: /* Enable this to see controls for tuning purpose. */
// C: #define ENABLE_TUNING_CONTROLS
// C: 
// C: #ifdef ENABLE_TUNING_CONTROLS
// C: #include <sound/tlv.h>
// C: #endif
// C: 
// C: #define FLOAT_ZERO	0x00000000
// C: #define FLOAT_ONE	0x3f800000
// C: #define FLOAT_TWO	0x40000000
// C: #define FLOAT_THREE     0x40400000
// C: #define FLOAT_FIVE	0x40a00000
// C: #define FLOAT_SIX       0x40c00000
// C: #define FLOAT_EIGHT     0x41000000
// C: #define FLOAT_MINUS_5	0xc0a00000
// C: 
// C: #define UNSOL_TAG_DSP	0x16
// C: 
// C: #define DSP_DMA_WRITE_BUFLEN_INIT (1UL<<18)
// C: #define DSP_DMA_WRITE_BUFLEN_OVLY (1UL<<15)
// C: 
// C: #define DMA_TRANSFER_FRAME_SIZE_NWORDS		8
// C: #define DMA_TRANSFER_MAX_FRAME_SIZE_NWORDS	32
// C: #define DMA_OVERLAY_FRAME_SIZE_NWORDS		2
// C: 
// C: #define MASTERCONTROL				0x80
// C: #define MASTERCONTROL_ALLOC_DMA_CHAN		10
// C: #define MASTERCONTROL_QUERY_SPEAKER_EQ_ADDRESS	60
// C: 
// C: #define WIDGET_CHIP_CTRL      0x15
// C: #define WIDGET_DSP_CTRL       0x16
// C: 
// C: #define MEM_CONNID_MICIN1     3
// C: #define MEM_CONNID_MICIN2     5
// C: #define MEM_CONNID_MICOUT1    12
// C: #define MEM_CONNID_MICOUT2    14
// C: #define MEM_CONNID_WUH        10
// C: #define MEM_CONNID_DSP        16
// C: #define MEM_CONNID_DMIC       100
// C: 
// C: #define SCP_SET    0
// C: #define SCP_GET    1
// C: 
// C: #define EFX_FILE   "ctefx.bin"
// C: #define DESKTOP_EFX_FILE   "ctefx-desktop.bin"
// C: #define R3DI_EFX_FILE  "ctefx-r3di.bin"
// C: 
// C: #ifdef CONFIG_SND_HDA_CODEC_CA0132_DSP
// C: MODULE_FIRMWARE(EFX_FILE);
// C: MODULE_FIRMWARE(DESKTOP_EFX_FILE);
// C: MODULE_FIRMWARE(R3DI_EFX_FILE);
// C: #endif
// C: 
// C: static const char *const dirstr[2] = { "Playback", "Capture" };
// C: 
// C: #define NUM_OF_OUTPUTS 2
// C: static const char *const out_type_str[2] = { "Speakers", "Headphone" };
// C: enum {
// C: 	SPEAKER_OUT,
// C: 	HEADPHONE_OUT,
// C: };
// C: 
// C: enum {
// C: 	DIGITAL_MIC,
// C: 	LINE_MIC_IN
// C: };
// C: 
// C: /* Strings for Input Source Enum Control */
// C: static const char *const in_src_str[3] = { "Microphone", "Line In", "Front Microphone" };
// C: #define IN_SRC_NUM_OF_INPUTS 3
// C: enum {
// C: 	REAR_MIC,
// C: 	REAR_LINE_IN,
// C: 	FRONT_MIC,
// C: };
// C: 
// C: enum {
// C: #define VNODE_START_NID    0x80
// C: 	VNID_SPK = VNODE_START_NID,			/* Speaker vnid */
// C: 	VNID_MIC,
// C: 	VNID_HP_SEL,
// C: 	VNID_AMIC1_SEL,
// C: 	VNID_HP_ASEL,
// C: 	VNID_AMIC1_ASEL,
// C: 	VNODE_END_NID,
// C: #define VNODES_COUNT  (VNODE_END_NID - VNODE_START_NID)
// C: 
// C: #define EFFECT_START_NID    0x90
// C: #define OUT_EFFECT_START_NID    EFFECT_START_NID
// C: 	SURROUND = OUT_EFFECT_START_NID,
// C: 	CRYSTALIZER,
// C: 	DIALOG_PLUS,
// C: 	SMART_VOLUME,
// C: 	X_BASS,
// C: 	EQUALIZER,
// C: 	OUT_EFFECT_END_NID,
// C: #define OUT_EFFECTS_COUNT  (OUT_EFFECT_END_NID - OUT_EFFECT_START_NID)
// C: 
// C: #define IN_EFFECT_START_NID  OUT_EFFECT_END_NID
// C: 	ECHO_CANCELLATION = IN_EFFECT_START_NID,
// C: 	VOICE_FOCUS,
// C: 	MIC_SVM,
// C: 	NOISE_REDUCTION,
// C: 	IN_EFFECT_END_NID,
// C: #define IN_EFFECTS_COUNT  (IN_EFFECT_END_NID - IN_EFFECT_START_NID)
// C: 
// C: 	VOICEFX = IN_EFFECT_END_NID,
// C: 	PLAY_ENHANCEMENT,
// C: 	CRYSTAL_VOICE,
// C: 	EFFECT_END_NID,
// C: 	OUTPUT_SOURCE_ENUM,
// C: 	INPUT_SOURCE_ENUM,
// C: 	XBASS_XOVER,
// C: 	EQ_PRESET_ENUM,
// C: 	SMART_VOLUME_ENUM,
// C: 	MIC_BOOST_ENUM,
// C: 	AE5_HEADPHONE_GAIN_ENUM,
// C: 	AE5_SOUND_FILTER_ENUM,
// C: 	ZXR_HEADPHONE_GAIN,
// C: 	SPEAKER_CHANNEL_CFG_ENUM,
// C: 	SPEAKER_FULL_RANGE_FRONT,
// C: 	SPEAKER_FULL_RANGE_REAR,
// C: 	BASS_REDIRECTION,
// C: 	BASS_REDIRECTION_XOVER,
// C: #define EFFECTS_COUNT  (EFFECT_END_NID - EFFECT_START_NID)
// C: };
// C: 
// C: /* Effects values size*/
// C: #define EFFECT_VALS_MAX_COUNT 12
// C: 
// C: /*
// C:  * Default values for the effect slider controls, they are in order of their
// C:  * effect NID's. Surround, Crystalizer, Dialog Plus, Smart Volume, and then
// C:  * X-bass.
// C:  */
// C: static const unsigned int effect_slider_defaults[] = {67, 65, 50, 74, 50};
// C: /* Amount of effect level sliders for ca0132_alt controls. */
// C: #define EFFECT_LEVEL_SLIDERS 5
// C: 
// C: /* Latency introduced by DSP blocks in milliseconds. */
// C: #define DSP_CAPTURE_INIT_LATENCY        0
// C: #define DSP_CRYSTAL_VOICE_LATENCY       124
// C: #define DSP_PLAYBACK_INIT_LATENCY       13
// C: #define DSP_PLAY_ENHANCEMENT_LATENCY    30
// C: #define DSP_SPEAKER_OUT_LATENCY         7
// C: 
// C: struct ct_effect {
// C: 	const char *name;
// C: 	hda_nid_t nid;
// C: 	int mid; /*effect module ID*/
// C: 	int reqs[EFFECT_VALS_MAX_COUNT]; /*effect module request*/
// C: 	int direct; /* 0:output; 1:input*/
// C: 	int params; /* number of default non-on/off params */
// C: 	/*effect default values, 1st is on/off. */
// C: 	unsigned int def_vals[EFFECT_VALS_MAX_COUNT];
// C: };
// C: 
// C: #define EFX_DIR_OUT 0
// C: #define EFX_DIR_IN  1
// C: 
// C: static const struct ct_effect ca0132_effects[EFFECTS_COUNT] = {
// C: 	{ .name = "Surround",
// C: 	  .nid = SURROUND,
// C: 	  .mid = 0x96,
// C: 	  .reqs = {0, 1},
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .params = 1,
// C: 	  .def_vals = {0x3F800000, 0x3F2B851F}
// C: 	},
// C: 	{ .name = "Crystalizer",
// C: 	  .nid = CRYSTALIZER,
// C: 	  .mid = 0x96,
// C: 	  .reqs = {7, 8},
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .params = 1,
// C: 	  .def_vals = {0x3F800000, 0x3F266666}
// C: 	},
// C: 	{ .name = "Dialog Plus",
// C: 	  .nid = DIALOG_PLUS,
// C: 	  .mid = 0x96,
// C: 	  .reqs = {2, 3},
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .params = 1,
// C: 	  .def_vals = {0x00000000, 0x3F000000}
// C: 	},
// C: 	{ .name = "Smart Volume",
// C: 	  .nid = SMART_VOLUME,
// C: 	  .mid = 0x96,
// C: 	  .reqs = {4, 5, 6},
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .params = 2,
// C: 	  .def_vals = {0x3F800000, 0x3F3D70A4, 0x00000000}
// C: 	},
// C: 	{ .name = "X-Bass",
// C: 	  .nid = X_BASS,
// C: 	  .mid = 0x96,
// C: 	  .reqs = {24, 23, 25},
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .params = 2,
// C: 	  .def_vals = {0x3F800000, 0x42A00000, 0x3F000000}
// C: 	},
// C: 	{ .name = "Equalizer",
// C: 	  .nid = EQUALIZER,
// C: 	  .mid = 0x96,
// C: 	  .reqs = {9, 10, 11, 12, 13, 14,
// C: 			15, 16, 17, 18, 19, 20},
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .params = 11,
// C: 	  .def_vals = {0x00000000, 0x00000000, 0x00000000, 0x00000000,
// C: 		       0x00000000, 0x00000000, 0x00000000, 0x00000000,
// C: 		       0x00000000, 0x00000000, 0x00000000, 0x00000000}
// C: 	},
// C: 	{ .name = "Echo Cancellation",
// C: 	  .nid = ECHO_CANCELLATION,
// C: 	  .mid = 0x95,
// C: 	  .reqs = {0, 1, 2, 3},
// C: 	  .direct = EFX_DIR_IN,
// C: 	  .params = 3,
// C: 	  .def_vals = {0x00000000, 0x3F3A9692, 0x00000000, 0x00000000}
// C: 	},
// C: 	{ .name = "Voice Focus",
// C: 	  .nid = VOICE_FOCUS,
// C: 	  .mid = 0x95,
// C: 	  .reqs = {6, 7, 8, 9},
// C: 	  .direct = EFX_DIR_IN,
// C: 	  .params = 3,
// C: 	  .def_vals = {0x3F800000, 0x3D7DF3B6, 0x41F00000, 0x41F00000}
// C: 	},
// C: 	{ .name = "Mic SVM",
// C: 	  .nid = MIC_SVM,
// C: 	  .mid = 0x95,
// C: 	  .reqs = {44, 45},
// C: 	  .direct = EFX_DIR_IN,
// C: 	  .params = 1,
// C: 	  .def_vals = {0x00000000, 0x3F3D70A4}
// C: 	},
// C: 	{ .name = "Noise Reduction",
// C: 	  .nid = NOISE_REDUCTION,
// C: 	  .mid = 0x95,
// C: 	  .reqs = {4, 5},
// C: 	  .direct = EFX_DIR_IN,
// C: 	  .params = 1,
// C: 	  .def_vals = {0x3F800000, 0x3F000000}
// C: 	},
// C: 	{ .name = "VoiceFX",
// C: 	  .nid = VOICEFX,
// C: 	  .mid = 0x95,
// C: 	  .reqs = {10, 11, 12, 13, 14, 15, 16, 17, 18},
// C: 	  .direct = EFX_DIR_IN,
// C: 	  .params = 8,
// C: 	  .def_vals = {0x00000000, 0x43C80000, 0x44AF0000, 0x44FA0000,
// C: 		       0x3F800000, 0x3F800000, 0x3F800000, 0x00000000,
// C: 		       0x00000000}
// C: 	}
// C: };
// C: 
// C: /* Tuning controls */
// C: #ifdef ENABLE_TUNING_CONTROLS
// C: 
// C: enum {
// C: #define TUNING_CTL_START_NID  0xC0
// C: 	WEDGE_ANGLE = TUNING_CTL_START_NID,
// C: 	SVM_LEVEL,
// C: 	EQUALIZER_BAND_0,
// C: 	EQUALIZER_BAND_1,
// C: 	EQUALIZER_BAND_2,
// C: 	EQUALIZER_BAND_3,
// C: 	EQUALIZER_BAND_4,
// C: 	EQUALIZER_BAND_5,
// C: 	EQUALIZER_BAND_6,
// C: 	EQUALIZER_BAND_7,
// C: 	EQUALIZER_BAND_8,
// C: 	EQUALIZER_BAND_9,
// C: 	TUNING_CTL_END_NID
// C: #define TUNING_CTLS_COUNT  (TUNING_CTL_END_NID - TUNING_CTL_START_NID)
// C: };
// C: 
// C: struct ct_tuning_ctl {
// C: 	const char *name;
// C: 	hda_nid_t parent_nid;
// C: 	hda_nid_t nid;
// C: 	int mid; /*effect module ID*/
// C: 	int req; /*effect module request*/
// C: 	int direct; /* 0:output; 1:input*/
// C: 	unsigned int def_val;/*effect default values*/
// C: };
// C: 
// C: static const struct ct_tuning_ctl ca0132_tuning_ctls[] = {
// C: 	{ .name = "Wedge Angle",
// C: 	  .parent_nid = VOICE_FOCUS,
// C: 	  .nid = WEDGE_ANGLE,
// C: 	  .mid = 0x95,
// C: 	  .req = 8,
// C: 	  .direct = EFX_DIR_IN,
// C: 	  .def_val = 0x41F00000
// C: 	},
// C: 	{ .name = "SVM Level",
// C: 	  .parent_nid = MIC_SVM,
// C: 	  .nid = SVM_LEVEL,
// C: 	  .mid = 0x95,
// C: 	  .req = 45,
// C: 	  .direct = EFX_DIR_IN,
// C: 	  .def_val = 0x3F3D70A4
// C: 	},
// C: 	{ .name = "EQ Band0",
// C: 	  .parent_nid = EQUALIZER,
// C: 	  .nid = EQUALIZER_BAND_0,
// C: 	  .mid = 0x96,
// C: 	  .req = 11,
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .def_val = 0x00000000
// C: 	},
// C: 	{ .name = "EQ Band1",
// C: 	  .parent_nid = EQUALIZER,
// C: 	  .nid = EQUALIZER_BAND_1,
// C: 	  .mid = 0x96,
// C: 	  .req = 12,
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .def_val = 0x00000000
// C: 	},
// C: 	{ .name = "EQ Band2",
// C: 	  .parent_nid = EQUALIZER,
// C: 	  .nid = EQUALIZER_BAND_2,
// C: 	  .mid = 0x96,
// C: 	  .req = 13,
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .def_val = 0x00000000
// C: 	},
// C: 	{ .name = "EQ Band3",
// C: 	  .parent_nid = EQUALIZER,
// C: 	  .nid = EQUALIZER_BAND_3,
// C: 	  .mid = 0x96,
// C: 	  .req = 14,
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .def_val = 0x00000000
// C: 	},
// C: 	{ .name = "EQ Band4",
// C: 	  .parent_nid = EQUALIZER,
// C: 	  .nid = EQUALIZER_BAND_4,
// C: 	  .mid = 0x96,
// C: 	  .req = 15,
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .def_val = 0x00000000
// C: 	},
// C: 	{ .name = "EQ Band5",
// C: 	  .parent_nid = EQUALIZER,
// C: 	  .nid = EQUALIZER_BAND_5,
// C: 	  .mid = 0x96,
// C: 	  .req = 16,
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .def_val = 0x00000000
// C: 	},
// C: 	{ .name = "EQ Band6",
// C: 	  .parent_nid = EQUALIZER,
// C: 	  .nid = EQUALIZER_BAND_6,
// C: 	  .mid = 0x96,
// C: 	  .req = 17,
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .def_val = 0x00000000
// C: 	},
// C: 	{ .name = "EQ Band7",
// C: 	  .parent_nid = EQUALIZER,
// C: 	  .nid = EQUALIZER_BAND_7,
// C: 	  .mid = 0x96,
// C: 	  .req = 18,
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .def_val = 0x00000000
// C: 	},
// C: 	{ .name = "EQ Band8",
// C: 	  .parent_nid = EQUALIZER,
// C: 	  .nid = EQUALIZER_BAND_8,
// C: 	  .mid = 0x96,
// C: 	  .req = 19,
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .def_val = 0x00000000
// C: 	},
// C: 	{ .name = "EQ Band9",
// C: 	  .parent_nid = EQUALIZER,
// C: 	  .nid = EQUALIZER_BAND_9,
// C: 	  .mid = 0x96,
// C: 	  .req = 20,
// C: 	  .direct = EFX_DIR_OUT,
// C: 	  .def_val = 0x00000000
// C: 	}
// C: };
// C: #endif
// C: 
// C: /* Voice FX Presets */
// C: #define VOICEFX_MAX_PARAM_COUNT 9
// C: 
// C: struct ct_voicefx {
// C: 	const char *name;
// C: 	hda_nid_t nid;
// C: 	int mid;
// C: 	int reqs[VOICEFX_MAX_PARAM_COUNT]; /*effect module request*/
// C: };
// C: 
// C: struct ct_voicefx_preset {
// C: 	const char *name; /*preset name*/
// C: 	unsigned int vals[VOICEFX_MAX_PARAM_COUNT];
// C: };
// C: 
// C: static const struct ct_voicefx ca0132_voicefx = {
// C: 	.name = "VoiceFX Capture Switch",
// C: 	.nid = VOICEFX,
// C: 	.mid = 0x95,
// C: 	.reqs = {10, 11, 12, 13, 14, 15, 16, 17, 18}
// C: };
// C: 
// C: static const struct ct_voicefx_preset ca0132_voicefx_presets[] = {
// C: 	{ .name = "Neutral",
// C: 	  .vals = { 0x00000000, 0x43C80000, 0x44AF0000,
// C: 		    0x44FA0000, 0x3F800000, 0x3F800000,
// C: 		    0x3F800000, 0x00000000, 0x00000000 }
// C: 	},
// C: 	{ .name = "Female2Male",
// C: 	  .vals = { 0x3F800000, 0x43C80000, 0x44AF0000,
// C: 		    0x44FA0000, 0x3F19999A, 0x3F866666,
// C: 		    0x3F800000, 0x00000000, 0x00000000 }
// C: 	},
// C: 	{ .name = "Male2Female",
// C: 	  .vals = { 0x3F800000, 0x43C80000, 0x44AF0000,
// C: 		    0x450AC000, 0x4017AE14, 0x3F6B851F,
// C: 		    0x3F800000, 0x00000000, 0x00000000 }
// C: 	},
// C: 	{ .name = "ScrappyKid",
// C: 	  .vals = { 0x3F800000, 0x43C80000, 0x44AF0000,
// C: 		    0x44FA0000, 0x40400000, 0x3F28F5C3,
// C: 		    0x3F800000, 0x00000000, 0x00000000 }
// C: 	},
// C: 	{ .name = "Elderly",
// C: 	  .vals = { 0x3F800000, 0x44324000, 0x44BB8000,
// C: 		    0x44E10000, 0x3FB33333, 0x3FB9999A,
// C: 		    0x3F800000, 0x3E3A2E43, 0x00000000 }
// C: 	},
// C: 	{ .name = "Orc",
// C: 	  .vals = { 0x3F800000, 0x43EA0000, 0x44A52000,
// C: 		    0x45098000, 0x3F266666, 0x3FC00000,
// C: 		    0x3F800000, 0x00000000, 0x00000000 }
// C: 	},
// C: 	{ .name = "Elf",
// C: 	  .vals = { 0x3F800000, 0x43C70000, 0x44AE6000,
// C: 		    0x45193000, 0x3F8E147B, 0x3F75C28F,
// C: 		    0x3F800000, 0x00000000, 0x00000000 }
// C: 	},
// C: 	{ .name = "Dwarf",
// C: 	  .vals = { 0x3F800000, 0x43930000, 0x44BEE000,
// C: 		    0x45007000, 0x3F451EB8, 0x3F7851EC,
// C: 		    0x3F800000, 0x00000000, 0x00000000 }
// C: 	},
// C: 	{ .name = "AlienBrute",
// C: 	  .vals = { 0x3F800000, 0x43BFC5AC, 0x44B28FDF,
// C: 		    0x451F6000, 0x3F266666, 0x3FA7D945,
// C: 		    0x3F800000, 0x3CF5C28F, 0x00000000 }
// C: 	},
// C: 	{ .name = "Robot",
// C: 	  .vals = { 0x3F800000, 0x43C80000, 0x44AF0000,
// C: 		    0x44FA0000, 0x3FB2718B, 0x3F800000,
// C: 		    0xBC07010E, 0x00000000, 0x00000000 }
// C: 	},
// C: 	{ .name = "Marine",
// C: 	  .vals = { 0x3F800000, 0x43C20000, 0x44906000,
// C: 		    0x44E70000, 0x3F4CCCCD, 0x3F8A3D71,
// C: 		    0x3F0A3D71, 0x00000000, 0x00000000 }
// C: 	},
// C: 	{ .name = "Emo",
// C: 	  .vals = { 0x3F800000, 0x43C80000, 0x44AF0000,
// C: 		    0x44FA0000, 0x3F800000, 0x3F800000,
// C: 		    0x3E4CCCCD, 0x00000000, 0x00000000 }
// C: 	},
// C: 	{ .name = "DeepVoice",
// C: 	  .vals = { 0x3F800000, 0x43A9C5AC, 0x44AA4FDF,
// C: 		    0x44FFC000, 0x3EDBB56F, 0x3F99C4CA,
// C: 		    0x3F800000, 0x00000000, 0x00000000 }
// C: 	},
// C: 	{ .name = "Munchkin",
// C: 	  .vals = { 0x3F800000, 0x43C80000, 0x44AF0000,
// C: 		    0x44FA0000, 0x3F800000, 0x3F1A043C,
// C: 		    0x3F800000, 0x00000000, 0x00000000 }
// C: 	}
// C: };
// C: 
// C: /* ca0132 EQ presets, taken from Windows Sound Blaster Z Driver */
// C: 
// C: #define EQ_PRESET_MAX_PARAM_COUNT 11
// C: 
// C: struct ct_eq {
// C: 	const char *name;
// C: 	hda_nid_t nid;
// C: 	int mid;
// C: 	int reqs[EQ_PRESET_MAX_PARAM_COUNT]; /*effect module request*/
// C: };
// C: 
// C: struct ct_eq_preset {
// C: 	const char *name; /*preset name*/
// C: 	unsigned int vals[EQ_PRESET_MAX_PARAM_COUNT];
// C: };
// C: 
// C: static const struct ct_eq ca0132_alt_eq_enum = {
// C: 	.name = "FX: Equalizer Preset Switch",
// C: 	.nid = EQ_PRESET_ENUM,
// C: 	.mid = 0x96,
// C: 	.reqs = {10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20}
// C: };
// C: 
// C: 
// C: static const struct ct_eq_preset ca0132_alt_eq_presets[] = {
// C: 	{ .name = "Flat",
// C: 	 .vals = { 0x00000000, 0x00000000, 0x00000000,
// C: 		   0x00000000, 0x00000000, 0x00000000,
// C: 		   0x00000000, 0x00000000, 0x00000000,
// C: 		   0x00000000, 0x00000000	     }
// C: 	},
// C: 	{ .name = "Acoustic",
// C: 	 .vals = { 0x00000000, 0x00000000, 0x3F8CCCCD,
// C: 		   0x40000000, 0x00000000, 0x00000000,
// C: 		   0x00000000, 0x00000000, 0x40000000,
// C: 		   0x40000000, 0x40000000	     }
// C: 	},
// C: 	{ .name = "Classical",
// C: 	 .vals = { 0x00000000, 0x00000000, 0x40C00000,
// C: 		   0x40C00000, 0x40466666, 0x00000000,
// C: 		   0x00000000, 0x00000000, 0x00000000,
// C: 		   0x40466666, 0x40466666	     }
// C: 	},
// C: 	{ .name = "Country",
// C: 	 .vals = { 0x00000000, 0xBF99999A, 0x00000000,
// C: 		   0x3FA66666, 0x3FA66666, 0x3F8CCCCD,
// C: 		   0x00000000, 0x00000000, 0x40000000,
// C: 		   0x40466666, 0x40800000	     }
// C: 	},
// C: 	{ .name = "Dance",
// C: 	 .vals = { 0x00000000, 0xBF99999A, 0x40000000,
// C: 		   0x40466666, 0x40866666, 0xBF99999A,
// C: 		   0xBF99999A, 0x00000000, 0x00000000,
// C: 		   0x40800000, 0x40800000	     }
// C: 	},
// C: 	{ .name = "Jazz",
// C: 	 .vals = { 0x00000000, 0x00000000, 0x00000000,
// C: 		   0x3F8CCCCD, 0x40800000, 0x40800000,
// C: 		   0x40800000, 0x00000000, 0x3F8CCCCD,
// C: 		   0x40466666, 0x40466666	     }
// C: 	},
// C: 	{ .name = "New Age",
// C: 	 .vals = { 0x00000000, 0x00000000, 0x40000000,
// C: 		   0x40000000, 0x00000000, 0x00000000,
// C: 		   0x00000000, 0x3F8CCCCD, 0x40000000,
// C: 		   0x40000000, 0x40000000	     }
// C: 	},
// C: 	{ .name = "Pop",
// C: 	 .vals = { 0x00000000, 0xBFCCCCCD, 0x00000000,
// C: 		   0x40000000, 0x40000000, 0x00000000,
// C: 		   0xBF99999A, 0xBF99999A, 0x00000000,
// C: 		   0x40466666, 0x40C00000	     }
// C: 	},
// C: 	{ .name = "Rock",
// C: 	 .vals = { 0x00000000, 0xBF99999A, 0xBF99999A,
// C: 		   0x3F8CCCCD, 0x40000000, 0xBF99999A,
// C: 		   0xBF99999A, 0x00000000, 0x00000000,
// C: 		   0x40800000, 0x40800000	     }
// C: 	},
// C: 	{ .name = "Vocal",
// C: 	 .vals = { 0x00000000, 0xC0000000, 0xBF99999A,
// C: 		   0xBF99999A, 0x00000000, 0x40466666,
// C: 		   0x40800000, 0x40466666, 0x00000000,
// C: 		   0x00000000, 0x3F8CCCCD	     }
// C: 	}
// C: };
// C: 
// C: /*
// C:  * DSP reqs for handling full-range speakers/bass redirection. If a speaker is
// C:  * set as not being full range, and bass redirection is enabled, all
// C:  * frequencies below the crossover frequency are redirected to the LFE
// C:  * channel. If the surround configuration has no LFE channel, this can't be
// C:  * enabled. X-Bass must be disabled when using these.
// C:  */
// C: enum speaker_range_reqs {
// C: 	SPEAKER_BASS_REDIRECT            = 0x15,
// C: 	SPEAKER_BASS_REDIRECT_XOVER_FREQ = 0x16,
// C: 	/* Between 0x16-0x1a are the X-Bass reqs. */
// C: 	SPEAKER_FULL_RANGE_FRONT_L_R     = 0x1a,
// C: 	SPEAKER_FULL_RANGE_CENTER_LFE    = 0x1b,
// C: 	SPEAKER_FULL_RANGE_REAR_L_R      = 0x1c,
// C: 	SPEAKER_FULL_RANGE_SURROUND_L_R  = 0x1d,
// C: 	SPEAKER_BASS_REDIRECT_SUB_GAIN   = 0x1e,
// C: };
// C: 
// C: /*
// C:  * Definitions for the DSP req's to handle speaker tuning. These all belong to
// C:  * module ID 0x96, the output effects module.
// C:  */
// C: enum speaker_tuning_reqs {
// C: 	/*
// C: 	 * Currently, this value is always set to 0.0f. However, on Windows,
// C: 	 * when selecting certain headphone profiles on the new Sound Blaster
// C: 	 * connect software, the QUERY_SPEAKER_EQ_ADDRESS req on mid 0x80 is
// C: 	 * sent. This gets the speaker EQ address area, which is then used to
// C: 	 * send over (presumably) an equalizer profile for the specific
// C: 	 * headphone setup. It is sent using the same method the DSP
// C: 	 * firmware is uploaded with, which I believe is why the 'ctspeq.bin'
// C: 	 * file exists in linux firmware tree but goes unused. It would also
// C: 	 * explain why the QUERY_SPEAKER_EQ_ADDRESS req is defined but unused.
// C: 	 * Once this profile is sent over, SPEAKER_TUNING_USE_SPEAKER_EQ is
// C: 	 * set to 1.0f.
// C: 	 */
// C: 	SPEAKER_TUNING_USE_SPEAKER_EQ           = 0x1f,
// C: 	SPEAKER_TUNING_ENABLE_CENTER_EQ         = 0x20,
// C: 	SPEAKER_TUNING_FRONT_LEFT_VOL_LEVEL     = 0x21,
// C: 	SPEAKER_TUNING_FRONT_RIGHT_VOL_LEVEL    = 0x22,
// C: 	SPEAKER_TUNING_CENTER_VOL_LEVEL         = 0x23,
// C: 	SPEAKER_TUNING_LFE_VOL_LEVEL            = 0x24,
// C: 	SPEAKER_TUNING_REAR_LEFT_VOL_LEVEL      = 0x25,
// C: 	SPEAKER_TUNING_REAR_RIGHT_VOL_LEVEL     = 0x26,
// C: 	SPEAKER_TUNING_SURROUND_LEFT_VOL_LEVEL  = 0x27,
// C: 	SPEAKER_TUNING_SURROUND_RIGHT_VOL_LEVEL = 0x28,
// C: 	/*
// C: 	 * Inversion is used when setting headphone virtualization to line
// C: 	 * out. Not sure why this is, but it's the only place it's ever used.
// C: 	 */
// C: 	SPEAKER_TUNING_FRONT_LEFT_INVERT        = 0x29,
// C: 	SPEAKER_TUNING_FRONT_RIGHT_INVERT       = 0x2a,
// C: 	SPEAKER_TUNING_CENTER_INVERT            = 0x2b,
// C: 	SPEAKER_TUNING_LFE_INVERT               = 0x2c,
// C: 	SPEAKER_TUNING_REAR_LEFT_INVERT         = 0x2d,
// C: 	SPEAKER_TUNING_REAR_RIGHT_INVERT        = 0x2e,
// C: 	SPEAKER_TUNING_SURROUND_LEFT_INVERT     = 0x2f,
// C: 	SPEAKER_TUNING_SURROUND_RIGHT_INVERT    = 0x30,
// C: 	/* Delay is used when setting surround speaker distance in Windows. */
// C: 	SPEAKER_TUNING_FRONT_LEFT_DELAY         = 0x31,
// C: 	SPEAKER_TUNING_FRONT_RIGHT_DELAY        = 0x32,
// C: 	SPEAKER_TUNING_CENTER_DELAY             = 0x33,
// C: 	SPEAKER_TUNING_LFE_DELAY                = 0x34,
// C: 	SPEAKER_TUNING_REAR_LEFT_DELAY          = 0x35,
// C: 	SPEAKER_TUNING_REAR_RIGHT_DELAY         = 0x36,
// C: 	SPEAKER_TUNING_SURROUND_LEFT_DELAY      = 0x37,
// C: 	SPEAKER_TUNING_SURROUND_RIGHT_DELAY     = 0x38,
// C: 	/* Of these two, only mute seems to ever be used. */
// C: 	SPEAKER_TUNING_MAIN_VOLUME              = 0x39,
// C: 	SPEAKER_TUNING_MUTE                     = 0x3a,
// C: };
// C: 
// C: /* Surround output channel count configuration structures. */
// C: #define SPEAKER_CHANNEL_CFG_COUNT 5
// C: enum {
// C: 	SPEAKER_CHANNELS_2_0,
// C: 	SPEAKER_CHANNELS_2_1,
// C: 	SPEAKER_CHANNELS_4_0,
// C: 	SPEAKER_CHANNELS_4_1,
// C: 	SPEAKER_CHANNELS_5_1,
// C: };
// C: 
// C: struct ca0132_alt_speaker_channel_cfg {
// C: 	const char *name;
// C: 	unsigned int val;
// C: };
// C: 
// C: static const struct ca0132_alt_speaker_channel_cfg speaker_channel_cfgs[] = {
// C: 	{ .name = "2.0",
// C: 	  .val = FLOAT_ONE
// C: 	},
// C: 	{ .name = "2.1",
// C: 	  .val = FLOAT_TWO
// C: 	},
// C: 	{ .name = "4.0",
// C: 	  .val = FLOAT_FIVE
// C: 	},
// C: 	{ .name = "4.1",
// C: 	  .val = FLOAT_SIX
// C: 	},
// C: 	{ .name = "5.1",
// C: 	  .val = FLOAT_EIGHT
// C: 	}
// C: };
// C: 
// C: /*
// C:  * DSP volume setting structs. Req 1 is left volume, req 2 is right volume,
// C:  * and I don't know what the third req is, but it's always zero. I assume it's
// C:  * some sort of update or set command to tell the DSP there's new volume info.
// C:  */
// C: #define DSP_VOL_OUT 0
// C: #define DSP_VOL_IN  1
// C: 
// C: struct ct_dsp_volume_ctl {
// C: 	hda_nid_t vnid;
// C: 	int mid; /* module ID*/
// C: 	unsigned int reqs[3]; /* scp req ID */
// C: };
// C: 
// C: static const struct ct_dsp_volume_ctl ca0132_alt_vol_ctls[] = {
// C: 	{ .vnid = VNID_SPK,
// C: 	  .mid = 0x32,
// C: 	  .reqs = {3, 4, 2}
// C: 	},
// C: 	{ .vnid = VNID_MIC,
// C: 	  .mid = 0x37,
// C: 	  .reqs = {2, 3, 1}
// C: 	}
// C: };
// C: 
// C: /* Values for ca0113_mmio_command_set for selecting output. */
// C: #define AE_CA0113_OUT_SET_COMMANDS 6
// C: struct ae_ca0113_output_set {
// C: 	unsigned int group[AE_CA0113_OUT_SET_COMMANDS];
// C: 	unsigned int target[AE_CA0113_OUT_SET_COMMANDS];
// C: 	unsigned int vals[NUM_OF_OUTPUTS][AE_CA0113_OUT_SET_COMMANDS];
// C: };
// C: 
// C: static const struct ae_ca0113_output_set ae5_ca0113_output_presets = {
// C: 	.group =  { 0x30, 0x30, 0x48, 0x48, 0x48, 0x30 },
// C: 	.target = { 0x2e, 0x30, 0x0d, 0x17, 0x19, 0x32 },
// C: 		    /* Speakers. */
// C: 	.vals =   { { 0x00, 0x00, 0x40, 0x00, 0x00, 0x3f },
// C: 		    /* Headphones. */
// C: 		    { 0x3f, 0x3f, 0x00, 0x00, 0x00, 0x00 } },
// C: };
// C: 
// C: static const struct ae_ca0113_output_set ae7_ca0113_output_presets = {
// C: 	.group  = { 0x30, 0x30, 0x48, 0x48, 0x48, 0x30 },
// C: 	.target = { 0x2e, 0x30, 0x0d, 0x17, 0x19, 0x32 },
// C: 		    /* Speakers. */
// C: 	.vals   = { { 0x00, 0x00, 0x40, 0x00, 0x00, 0x3f },
// C: 		    /* Headphones. */
// C: 		    { 0x3f, 0x3f, 0x00, 0x00, 0x02, 0x00 } },
// C: };
// C: 
// C: /* ae5 ca0113 command sequences to set headphone gain levels. */
// C: #define AE5_HEADPHONE_GAIN_PRESET_MAX_COMMANDS 4
// C: struct ae5_headphone_gain_set {
// C: 	const char *name;
// C: 	unsigned int vals[AE5_HEADPHONE_GAIN_PRESET_MAX_COMMANDS];
// C: };
// C: 
// C: static const struct ae5_headphone_gain_set ae5_headphone_gain_presets[] = {
// C: 	{ .name = "Low (16-31",
// C: 	  .vals = { 0xff, 0x2c, 0xf5, 0x32 }
// C: 	},
// C: 	{ .name = "Medium (32-149",
// C: 	  .vals = { 0x38, 0xa8, 0x3e, 0x4c }
// C: 	},
// C: 	{ .name = "High (150-600",
// C: 	  .vals = { 0xff, 0xff, 0xff, 0x7f }
// C: 	}
// C: };
// C: 
// C: struct ae5_filter_set {
// C: 	const char *name;
// C: 	unsigned int val;
// C: };
// C: 
// C: static const struct ae5_filter_set ae5_filter_presets[] = {
// C: 	{ .name = "Slow Roll Off",
// C: 	  .val = 0xa0
// C: 	},
// C: 	{ .name = "Minimum Phase",
// C: 	  .val = 0xc0
// C: 	},
// C: 	{ .name = "Fast Roll Off",
// C: 	  .val = 0x80
// C: 	}
// C: };
// C: 
// C: /*
// C:  * Data structures for storing audio router remapping data. These are used to
// C:  * remap a currently active streams ports.
// C:  */
// C: struct chipio_stream_remap_data {
// C: 	unsigned int stream_id;
// C: 	unsigned int count;
// C: 
// C: 	unsigned int offset[16];
// C: 	unsigned int value[16];
// C: };
// C: 
// C: static const struct chipio_stream_remap_data stream_remap_data[] = {
// C: 	{ .stream_id = 0x14,
// C: 	  .count     = 0x04,
// C: 	  .offset    = { 0x00, 0x04, 0x08, 0x0c },
// C: 	  .value     = { 0x0001f8c0, 0x0001f9c1, 0x0001fac6, 0x0001fbc7 },
// C: 	},
// C: 	{ .stream_id = 0x0c,
// C: 	  .count     = 0x0c,
// C: 	  .offset    = { 0x00, 0x04, 0x08, 0x0c, 0x10, 0x14, 0x18, 0x1c,
// C: 			 0x20, 0x24, 0x28, 0x2c },
// C: 	  .value     = { 0x0001e0c0, 0x0001e1c1, 0x0001e4c2, 0x0001e5c3,
// C: 			 0x0001e2c4, 0x0001e3c5, 0x0001e8c6, 0x0001e9c7,
// C: 			 0x0001ecc8, 0x0001edc9, 0x0001eaca, 0x0001ebcb },
// C: 	},
// C: 	{ .stream_id = 0x0c,
// C: 	  .count     = 0x08,
// C: 	  .offset    = { 0x08, 0x0c, 0x10, 0x14, 0x20, 0x24, 0x28, 0x2c },
// C: 	  .value     = { 0x000140c2, 0x000141c3, 0x000150c4, 0x000151c5,
// C: 			 0x000142c8, 0x000143c9, 0x000152ca, 0x000153cb },
// C: 	}
// C: };
// C: 
// C: enum hda_cmd_vendor_io {
// C: 	/* for DspIO node */
// C: 	VENDOR_DSPIO_SCP_WRITE_DATA_LOW      = 0x000,
// C: 	VENDOR_DSPIO_SCP_WRITE_DATA_HIGH     = 0x100,
// C: 
// C: 	VENDOR_DSPIO_STATUS                  = 0xF01,
// C: 	VENDOR_DSPIO_SCP_POST_READ_DATA      = 0x702,
// C: 	VENDOR_DSPIO_SCP_READ_DATA           = 0xF02,
// C: 	VENDOR_DSPIO_DSP_INIT                = 0x703,
// C: 	VENDOR_DSPIO_SCP_POST_COUNT_QUERY    = 0x704,
// C: 	VENDOR_DSPIO_SCP_READ_COUNT          = 0xF04,
// C: 
// C: 	/* for ChipIO node */
// C: 	VENDOR_CHIPIO_ADDRESS_LOW            = 0x000,
// C: 	VENDOR_CHIPIO_ADDRESS_HIGH           = 0x100,
// C: 	VENDOR_CHIPIO_STREAM_FORMAT          = 0x200,
// C: 	VENDOR_CHIPIO_DATA_LOW               = 0x300,
// C: 	VENDOR_CHIPIO_DATA_HIGH              = 0x400,
// C: 
// C: 	VENDOR_CHIPIO_8051_WRITE_DIRECT      = 0x500,
// C: 	VENDOR_CHIPIO_8051_READ_DIRECT       = 0xD00,
// C: 
// C: 	VENDOR_CHIPIO_GET_PARAMETER          = 0xF00,
// C: 	VENDOR_CHIPIO_STATUS                 = 0xF01,
// C: 	VENDOR_CHIPIO_HIC_POST_READ          = 0x702,
// C: 	VENDOR_CHIPIO_HIC_READ_DATA          = 0xF03,
// C: 
// C: 	VENDOR_CHIPIO_8051_DATA_WRITE        = 0x707,
// C: 	VENDOR_CHIPIO_8051_DATA_READ         = 0xF07,
// C: 	VENDOR_CHIPIO_8051_PMEM_READ         = 0xF08,
// C: 	VENDOR_CHIPIO_8051_IRAM_WRITE        = 0x709,
// C: 	VENDOR_CHIPIO_8051_IRAM_READ         = 0xF09,
// C: 
// C: 	VENDOR_CHIPIO_CT_EXTENSIONS_ENABLE   = 0x70A,
// C: 	VENDOR_CHIPIO_CT_EXTENSIONS_GET      = 0xF0A,
// C: 
// C: 	VENDOR_CHIPIO_PLL_PMU_WRITE          = 0x70C,
// C: 	VENDOR_CHIPIO_PLL_PMU_READ           = 0xF0C,
// C: 	VENDOR_CHIPIO_8051_ADDRESS_LOW       = 0x70D,
// C: 	VENDOR_CHIPIO_8051_ADDRESS_HIGH      = 0x70E,
// C: 	VENDOR_CHIPIO_FLAG_SET               = 0x70F,
// C: 	VENDOR_CHIPIO_FLAGS_GET              = 0xF0F,
// C: 	VENDOR_CHIPIO_PARAM_SET              = 0x710,
// C: 	VENDOR_CHIPIO_PARAM_GET              = 0xF10,
// C: 
// C: 	VENDOR_CHIPIO_PORT_ALLOC_CONFIG_SET  = 0x711,
// C: 	VENDOR_CHIPIO_PORT_ALLOC_SET         = 0x712,
// C: 	VENDOR_CHIPIO_PORT_ALLOC_GET         = 0xF12,
// C: 	VENDOR_CHIPIO_PORT_FREE_SET          = 0x713,
// C: 
// C: 	VENDOR_CHIPIO_PARAM_EX_ID_GET        = 0xF17,
// C: 	VENDOR_CHIPIO_PARAM_EX_ID_SET        = 0x717,
// C: 	VENDOR_CHIPIO_PARAM_EX_VALUE_GET     = 0xF18,
// C: 	VENDOR_CHIPIO_PARAM_EX_VALUE_SET     = 0x718,
// C: 
// C: 	VENDOR_CHIPIO_DMIC_CTL_SET           = 0x788,
// C: 	VENDOR_CHIPIO_DMIC_CTL_GET           = 0xF88,
// C: 	VENDOR_CHIPIO_DMIC_PIN_SET           = 0x789,
// C: 	VENDOR_CHIPIO_DMIC_PIN_GET           = 0xF89,
// C: 	VENDOR_CHIPIO_DMIC_MCLK_SET          = 0x78A,
// C: 	VENDOR_CHIPIO_DMIC_MCLK_GET          = 0xF8A,
// C: 
// C: 	VENDOR_CHIPIO_EAPD_SEL_SET           = 0x78D
// C: };
// C: 
// C: /*
// C:  *  Control flag IDs
// C:  */
// C: enum control_flag_id {
// C: 	/* Connection manager stream setup is bypassed/enabled */
// C: 	CONTROL_FLAG_C_MGR                  = 0,
// C: 	/* DSP DMA is bypassed/enabled */
// C: 	CONTROL_FLAG_DMA                    = 1,
// C: 	/* 8051 'idle' mode is disabled/enabled */
// C: 	CONTROL_FLAG_IDLE_ENABLE            = 2,
// C: 	/* Tracker for the SPDIF-in path is bypassed/enabled */
// C: 	CONTROL_FLAG_TRACKER                = 3,
// C: 	/* DigitalOut to Spdif2Out connection is disabled/enabled */
// C: 	CONTROL_FLAG_SPDIF2OUT              = 4,
// C: 	/* Digital Microphone is disabled/enabled */
// C: 	CONTROL_FLAG_DMIC                   = 5,
// C: 	/* ADC_B rate is 48 kHz/96 kHz */
// C: 	CONTROL_FLAG_ADC_B_96KHZ            = 6,
// C: 	/* ADC_C rate is 48 kHz/96 kHz */
// C: 	CONTROL_FLAG_ADC_C_96KHZ            = 7,
// C: 	/* DAC rate is 48 kHz/96 kHz (affects all DACs) */
// C: 	CONTROL_FLAG_DAC_96KHZ              = 8,
// C: 	/* DSP rate is 48 kHz/96 kHz */
// C: 	CONTROL_FLAG_DSP_96KHZ              = 9,
// C: 	/* SRC clock is 98 MHz/196 MHz (196 MHz forces rate to 96 KHz) */
// C: 	CONTROL_FLAG_SRC_CLOCK_196MHZ       = 10,
// C: 	/* SRC rate is 48 kHz/96 kHz (48 kHz disabled when clock is 196 MHz) */
// C: 	CONTROL_FLAG_SRC_RATE_96KHZ         = 11,
// C: 	/* Decode Loop (DSP->SRC->DSP) is disabled/enabled */
// C: 	CONTROL_FLAG_DECODE_LOOP            = 12,
// C: 	/* De-emphasis filter on DAC-1 disabled/enabled */
// C: 	CONTROL_FLAG_DAC1_DEEMPHASIS        = 13,
// C: 	/* De-emphasis filter on DAC-2 disabled/enabled */
// C: 	CONTROL_FLAG_DAC2_DEEMPHASIS        = 14,
// C: 	/* De-emphasis filter on DAC-3 disabled/enabled */
// C: 	CONTROL_FLAG_DAC3_DEEMPHASIS        = 15,
// C: 	/* High-pass filter on ADC_B disabled/enabled */
// C: 	CONTROL_FLAG_ADC_B_HIGH_PASS        = 16,
// C: 	/* High-pass filter on ADC_C disabled/enabled */
// C: 	CONTROL_FLAG_ADC_C_HIGH_PASS        = 17,
// C: 	/* Common mode on Port_A disabled/enabled */
// C: 	CONTROL_FLAG_PORT_A_COMMON_MODE     = 18,
// C: 	/* Common mode on Port_D disabled/enabled */
// C: 	CONTROL_FLAG_PORT_D_COMMON_MODE     = 19,
// C: 	/* Impedance for ramp generator on Port_A 16 Ohm/10K Ohm */
// C: 	CONTROL_FLAG_PORT_A_10KOHM_LOAD     = 20,
// C: 	/* Impedance for ramp generator on Port_D, 16 Ohm/10K Ohm */
// C: 	CONTROL_FLAG_PORT_D_10KOHM_LOAD     = 21,
// C: 	/* ASI rate is 48kHz/96kHz */
// C: 	CONTROL_FLAG_ASI_96KHZ              = 22,
// C: 	/* DAC power settings able to control attached ports no/yes */
// C: 	CONTROL_FLAG_DACS_CONTROL_PORTS     = 23,
// C: 	/* Clock Stop OK reporting is disabled/enabled */
// C: 	CONTROL_FLAG_CONTROL_STOP_OK_ENABLE = 24,
// C: 	/* Number of control flags */
// C: 	CONTROL_FLAGS_MAX = (CONTROL_FLAG_CONTROL_STOP_OK_ENABLE+1)
// C: };
// C: 
// C: /*
// C:  * Control parameter IDs
// C:  */
// C: enum control_param_id {
// C: 	/* 0: None, 1: Mic1In*/
// C: 	CONTROL_PARAM_VIP_SOURCE               = 1,
// C: 	/* 0: force HDA, 1: allow DSP if HDA Spdif1Out stream is idle */
// C: 	CONTROL_PARAM_SPDIF1_SOURCE            = 2,
// C: 	/* Port A output stage gain setting to use when 16 Ohm output
// C: 	 * impedance is selected*/
// C: 	CONTROL_PARAM_PORTA_160OHM_GAIN        = 8,
// C: 	/* Port D output stage gain setting to use when 16 Ohm output
// C: 	 * impedance is selected*/
// C: 	CONTROL_PARAM_PORTD_160OHM_GAIN        = 10,
// C: 
// C: 	/*
// C: 	 * This control param name was found in the 8051 memory, and makes
// C: 	 * sense given the fact the AE-5 uses it and has the ASI flag set.
// C: 	 */
// C: 	CONTROL_PARAM_ASI                      = 23,
// C: 
// C: 	/* Stream Control */
// C: 
// C: 	/* Select stream with the given ID */
// C: 	CONTROL_PARAM_STREAM_ID                = 24,
// C: 	/* Source connection point for the selected stream */
// C: 	CONTROL_PARAM_STREAM_SOURCE_CONN_POINT = 25,
// C: 	/* Destination connection point for the selected stream */
// C: 	CONTROL_PARAM_STREAM_DEST_CONN_POINT   = 26,
// C: 	/* Number of audio channels in the selected stream */
// C: 	CONTROL_PARAM_STREAMS_CHANNELS         = 27,
// C: 	/*Enable control for the selected stream */
// C: 	CONTROL_PARAM_STREAM_CONTROL           = 28,
// C: 
// C: 	/* Connection Point Control */
// C: 
// C: 	/* Select connection point with the given ID */
// C: 	CONTROL_PARAM_CONN_POINT_ID            = 29,
// C: 	/* Connection point sample rate */
// C: 	CONTROL_PARAM_CONN_POINT_SAMPLE_RATE   = 30,
// C: 
// C: 	/* Node Control */
// C: 
// C: 	/* Select HDA node with the given ID */
// C: 	CONTROL_PARAM_NODE_ID                  = 31
// C: };
// C: 
// C: /*
// C:  *  Dsp Io Status codes
// C:  */
// C: enum hda_vendor_status_dspio {
// C: 	/* Success */
// C: 	VENDOR_STATUS_DSPIO_OK                       = 0x00,
// C: 	/* Busy, unable to accept new command, the host must retry */
// C: 	VENDOR_STATUS_DSPIO_BUSY                     = 0x01,
// C: 	/* SCP command queue is full */
// C: 	VENDOR_STATUS_DSPIO_SCP_COMMAND_QUEUE_FULL   = 0x02,
// C: 	/* SCP response queue is empty */
// C: 	VENDOR_STATUS_DSPIO_SCP_RESPONSE_QUEUE_EMPTY = 0x03
// C: };
// C: 
// C: /*
// C:  *  Chip Io Status codes
// C:  */
// C: enum hda_vendor_status_chipio {
// C: 	/* Success */
// C: 	VENDOR_STATUS_CHIPIO_OK   = 0x00,
// C: 	/* Busy, unable to accept new command, the host must retry */
// C: 	VENDOR_STATUS_CHIPIO_BUSY = 0x01
// C: };
// C: 
// C: /*
// C:  *  CA0132 sample rate
// C:  */
// C: enum ca0132_sample_rate {
// C: 	SR_6_000        = 0x00,
// C: 	SR_8_000        = 0x01,
// C: 	SR_9_600        = 0x02,
// C: 	SR_11_025       = 0x03,
// C: 	SR_16_000       = 0x04,
// C: 	SR_22_050       = 0x05,
// C: 	SR_24_000       = 0x06,
// C: 	SR_32_000       = 0x07,
// C: 	SR_44_100       = 0x08,
// C: 	SR_48_000       = 0x09,
// C: 	SR_88_200       = 0x0A,
// C: 	SR_96_000       = 0x0B,
// C: 	SR_144_000      = 0x0C,
// C: 	SR_176_400      = 0x0D,
// C: 	SR_192_000      = 0x0E,
// C: 	SR_384_000      = 0x0F,
// C: 
// C: 	SR_COUNT        = 0x10,
// C: 
// C: 	SR_RATE_UNKNOWN = 0x1F
// C: };
// C: 
// C: enum dsp_download_state {
// C: 	DSP_DOWNLOAD_FAILED = -1,
// C: 	DSP_DOWNLOAD_INIT   = 0,
// C: 	DSP_DOWNLOADING     = 1,
// C: 	DSP_DOWNLOADED      = 2
// C: };
// C: 
// C: /* retrieve parameters from hda format */
// C: #define get_hdafmt_chs(fmt)	(fmt & 0xf)
// C: #define get_hdafmt_bits(fmt)	((fmt >> 4) & 0x7)
// C: #define get_hdafmt_rate(fmt)	((fmt >> 8) & 0x7f)
// C: #define get_hdafmt_type(fmt)	((fmt >> 15) & 0x1)
// C: 
// C: /*
// C:  * CA0132 specific
// C:  */
// C: 
// C: struct ca0132_spec {
// C: 	struct hda_gen_spec gen;
// C: 
// C: 	const struct snd_kcontrol_new *mixers[5];
// C: 	unsigned int num_mixers;
// C: 	const struct hda_verb *base_init_verbs;
// C: 	const struct hda_verb *base_exit_verbs;
// C: 	const struct hda_verb *chip_init_verbs;
// C: 	const struct hda_verb *desktop_init_verbs;
// C: 	struct hda_verb *spec_init_verbs;
// C: 	struct auto_pin_cfg autocfg;
// C: 
// C: 	/* Nodes configurations */
// C: 	struct hda_multi_out multiout;
// C: 	hda_nid_t out_pins[AUTO_CFG_MAX_OUTS];
// C: 	hda_nid_t dacs[AUTO_CFG_MAX_OUTS];
// C: 	unsigned int num_outputs;
// C: 	hda_nid_t input_pins[AUTO_PIN_LAST];
// C: 	hda_nid_t adcs[AUTO_PIN_LAST];
// C: 	hda_nid_t dig_out;
// C: 	hda_nid_t dig_in;
// C: 	unsigned int num_inputs;
// C: 	hda_nid_t shared_mic_nid;
// C: 	hda_nid_t shared_out_nid;
// C: 	hda_nid_t unsol_tag_hp;
// C: 	hda_nid_t unsol_tag_front_hp; /* for desktop ca0132 codecs */
// C: 	hda_nid_t unsol_tag_amic1;
// C: 
// C: 	/* chip access */
// C: 	struct mutex chipio_mutex; /* chip access mutex */
// C: 	u32 curr_chip_addx;
// C: 
// C: 	/* DSP download related */
// C: 	enum dsp_download_state dsp_state;
// C: 	unsigned int dsp_stream_id;
// C: 	unsigned int wait_scp;
// C: 	unsigned int wait_scp_header;
// C: 	unsigned int wait_num_data;
// C: 	unsigned int scp_resp_header;
// C: 	unsigned int scp_resp_data[4];
// C: 	unsigned int scp_resp_count;
// C: 	bool startup_check_entered;
// C: 	bool dsp_reload;
// C: 
// C: 	/* mixer and effects related */
// C: 	unsigned char dmic_ctl;
// C: 	int cur_out_type;
// C: 	int cur_mic_type;
// C: 	long vnode_lvol[VNODES_COUNT];
// C: 	long vnode_rvol[VNODES_COUNT];
// C: 	long vnode_lswitch[VNODES_COUNT];
// C: 	long vnode_rswitch[VNODES_COUNT];
// C: 	long effects_switch[EFFECTS_COUNT];
// C: 	long voicefx_val;
// C: 	long cur_mic_boost;
// C: 	/* ca0132_alt control related values */
// C: 	unsigned char in_enum_val;
// C: 	unsigned char out_enum_val;
// C: 	unsigned char channel_cfg_val;
// C: 	unsigned char speaker_range_val[2];
// C: 	unsigned char mic_boost_enum_val;
// C: 	unsigned char smart_volume_setting;
// C: 	unsigned char bass_redirection_val;
// C: 	long bass_redirect_xover_freq;
// C: 	long fx_ctl_val[EFFECT_LEVEL_SLIDERS];
// C: 	long xbass_xover_freq;
// C: 	long eq_preset_val;
// C: 	unsigned int tlv[4];
// C: 	struct hda_vmaster_mute_hook vmaster_mute;
// C: 	/* AE-5 Control values */
// C: 	unsigned char ae5_headphone_gain_val;
// C: 	unsigned char ae5_filter_val;
// C: 	/* ZxR Control Values */
// C: 	unsigned char zxr_gain_set;
// C: 
// C: 	struct hda_codec *codec;
// C: 	struct delayed_work unsol_hp_work;
// C: 
// C: #ifdef ENABLE_TUNING_CONTROLS
// C: 	long cur_ctl_vals[TUNING_CTLS_COUNT];
// C: #endif
// C: 	/*
// C: 	 * The Recon3D, Sound Blaster Z, Sound Blaster ZxR, and Sound Blaster
// C: 	 * AE-5 all use PCI region 2 to toggle GPIO and other currently unknown
// C: 	 * things.
// C: 	 */
// C: 	bool use_pci_mmio;
// C: 	void __iomem *mem_base;
// C: 
// C: 	/*
// C: 	 * Whether or not to use the alt functions like alt_select_out,
// C: 	 * alt_select_in, etc. Only used on desktop codecs for now, because of
// C: 	 * surround sound support.
// C: 	 */
// C: 	bool use_alt_functions;
// C: 
// C: 	/*
// C: 	 * Whether or not to use alt controls:	volume effect sliders, EQ
// C: 	 * presets, smart volume presets, and new control names with FX prefix.
// C: 	 * Renames PlayEnhancement and CrystalVoice too.
// C: 	 */
// C: 	bool use_alt_controls;
// C: };
// C: 
// C: /*
// C:  * CA0132 quirks table
// C:  */
// C: enum {
// C: 	QUIRK_ALIENWARE,
// C: 	QUIRK_ALIENWARE_M17XR4,
// C: 	QUIRK_SBZ,
// C: 	QUIRK_ZXR,
// C: 	QUIRK_ZXR_DBPRO,
// C: 	QUIRK_R3DI,
// C: 	QUIRK_R3D,
// C: 	QUIRK_AE5,
// C: 	QUIRK_AE7,
// C: 	QUIRK_GENERIC,
// C: 	QUIRK_NONE = HDA_FIXUP_ID_NOT_SET,
// C: };
// C: 
// C: #ifdef CONFIG_PCI
// C: #define ca0132_quirk(spec)		((spec)->codec->fixup_id)
// C: #define ca0132_use_pci_mmio(spec)	((spec)->use_pci_mmio)
// C: #define ca0132_use_alt_functions(spec)	((spec)->use_alt_functions)
// C: #define ca0132_use_alt_controls(spec)	((spec)->use_alt_controls)
// C: #else
// C: #define ca0132_quirk(spec)		({ (void)(spec); QUIRK_NONE; })
// C: #define ca0132_use_alt_functions(spec)	({ (void)(spec); false; })
// C: #define ca0132_use_pci_mmio(spec)	({ (void)(spec); false; })
// C: #define ca0132_use_alt_controls(spec)	({ (void)(spec); false; })
// C: #endif
// C: 
// C: static const struct hda_pintbl alienware_pincfgs[] = {
// C: 	{ 0x0b, 0x90170110 }, /* Builtin Speaker */
// C: 	{ 0x0c, 0x411111f0 }, /* N/A */
// C: 	{ 0x0d, 0x411111f0 }, /* N/A */
// C: 	{ 0x0e, 0x411111f0 }, /* N/A */
// C: 	{ 0x0f, 0x0321101f }, /* HP */
// C: 	{ 0x10, 0x411111f0 }, /* Headset?  disabled for now */
// C: 	{ 0x11, 0x03a11021 }, /* Mic */
// C: 	{ 0x12, 0xd5a30140 }, /* Builtin Mic */
// C: 	{ 0x13, 0x411111f0 }, /* N/A */
// C: 	{ 0x18, 0x411111f0 }, /* N/A */
// C: 	{}
// C: };
// C: 
// C: /* Sound Blaster Z pin configs taken from Windows Driver */
// C: static const struct hda_pintbl sbz_pincfgs[] = {
// C: 	{ 0x0b, 0x01017010 }, /* Port G -- Lineout FRONT L/R */
// C: 	{ 0x0c, 0x014510f0 }, /* SPDIF Out 1 */
// C: 	{ 0x0d, 0x014510f0 }, /* Digital Out */
// C: 	{ 0x0e, 0x01c510f0 }, /* SPDIF In */
// C: 	{ 0x0f, 0x0221701f }, /* Port A -- BackPanel HP */
// C: 	{ 0x10, 0x01017012 }, /* Port D -- Center/LFE or FP Hp */
// C: 	{ 0x11, 0x01017014 }, /* Port B -- LineMicIn2 / Rear L/R */
// C: 	{ 0x12, 0x01a170f0 }, /* Port C -- LineIn1 */
// C: 	{ 0x13, 0x908700f0 }, /* What U Hear In*/
// C: 	{ 0x18, 0x50d000f0 }, /* N/A */
// C: 	{}
// C: };
// C: 
// C: /* Sound Blaster ZxR pin configs taken from Windows Driver */
// C: static const struct hda_pintbl zxr_pincfgs[] = {
// C: 	{ 0x0b, 0x01047110 }, /* Port G -- Lineout FRONT L/R */
// C: 	{ 0x0c, 0x414510f0 }, /* SPDIF Out 1 - Disabled*/
// C: 	{ 0x0d, 0x014510f0 }, /* Digital Out */
// C: 	{ 0x0e, 0x41c520f0 }, /* SPDIF In - Disabled*/
// C: 	{ 0x0f, 0x0122711f }, /* Port A -- BackPanel HP */
// C: 	{ 0x10, 0x01017111 }, /* Port D -- Center/LFE */
// C: 	{ 0x11, 0x01017114 }, /* Port B -- LineMicIn2 / Rear L/R */
// C: 	{ 0x12, 0x01a271f0 }, /* Port C -- LineIn1 */
// C: 	{ 0x13, 0x908700f0 }, /* What U Hear In*/
// C: 	{ 0x18, 0x50d000f0 }, /* N/A */
// C: 	{}
// C: };
// C: 
// C: /* Recon3D pin configs taken from Windows Driver */
// C: static const struct hda_pintbl r3d_pincfgs[] = {
// C: 	{ 0x0b, 0x01014110 }, /* Port G -- Lineout FRONT L/R */
// C: 	{ 0x0c, 0x014510f0 }, /* SPDIF Out 1 */
// C: 	{ 0x0d, 0x014510f0 }, /* Digital Out */
// C: 	{ 0x0e, 0x01c520f0 }, /* SPDIF In */
// C: 	{ 0x0f, 0x0221401f }, /* Port A -- BackPanel HP */
// C: 	{ 0x10, 0x01016011 }, /* Port D -- Center/LFE or FP Hp */
// C: 	{ 0x11, 0x01011014 }, /* Port B -- LineMicIn2 / Rear L/R */
// C: 	{ 0x12, 0x02a090f0 }, /* Port C -- LineIn1 */
// C: 	{ 0x13, 0x908700f0 }, /* What U Hear In*/
// C: 	{ 0x18, 0x50d000f0 }, /* N/A */
// C: 	{}
// C: };
// C: 
// C: /* Sound Blaster AE-5 pin configs taken from Windows Driver */
// C: static const struct hda_pintbl ae5_pincfgs[] = {
// C: 	{ 0x0b, 0x01017010 }, /* Port G -- Lineout FRONT L/R */
// C: 	{ 0x0c, 0x014510f0 }, /* SPDIF Out 1 */
// C: 	{ 0x0d, 0x014510f0 }, /* Digital Out */
// C: 	{ 0x0e, 0x01c510f0 }, /* SPDIF In */
// C: 	{ 0x0f, 0x01017114 }, /* Port A -- Rear L/R. */
// C: 	{ 0x10, 0x01017012 }, /* Port D -- Center/LFE or FP Hp */
// C: 	{ 0x11, 0x012170ff }, /* Port B -- LineMicIn2 / Rear Headphone */
// C: 	{ 0x12, 0x01a170f0 }, /* Port C -- LineIn1 */
// C: 	{ 0x13, 0x908700f0 }, /* What U Hear In*/
// C: 	{ 0x18, 0x50d000f0 }, /* N/A */
// C: 	{}
// C: };
// C: 
// C: /* Recon3D integrated pin configs taken from Windows Driver */
// C: static const struct hda_pintbl r3di_pincfgs[] = {
// C: 	{ 0x0b, 0x01014110 }, /* Port G -- Lineout FRONT L/R */
// C: 	{ 0x0c, 0x014510f0 }, /* SPDIF Out 1 */
// C: 	{ 0x0d, 0x014510f0 }, /* Digital Out */
// C: 	{ 0x0e, 0x41c520f0 }, /* SPDIF In */
// C: 	{ 0x0f, 0x0221401f }, /* Port A -- BackPanel HP */
// C: 	{ 0x10, 0x01016011 }, /* Port D -- Center/LFE or FP Hp */
// C: 	{ 0x11, 0x01011014 }, /* Port B -- LineMicIn2 / Rear L/R */
// C: 	{ 0x12, 0x02a090f0 }, /* Port C -- LineIn1 */
// C: 	{ 0x13, 0x908700f0 }, /* What U Hear In*/
// C: 	{ 0x18, 0x500000f0 }, /* N/A */
// C: 	{}
// C: };
// C: 
// C: static const struct hda_pintbl ae7_pincfgs[] = {
// C: 	{ 0x0b, 0x01017010 },
// C: 	{ 0x0c, 0x014510f0 },
// C: 	{ 0x0d, 0x414510f0 },
// C: 	{ 0x0e, 0x01c520f0 },
// C: 	{ 0x0f, 0x01017114 },
// C: 	{ 0x10, 0x01017011 },
// C: 	{ 0x11, 0x018170ff },
// C: 	{ 0x12, 0x01a170f0 },
// C: 	{ 0x13, 0x908700f0 },
// C: 	{ 0x18, 0x500000f0 },
// C: 	{}
// C: };
// C: 
// C: static const struct hda_pintbl ca0132_generic_pincfgs[] = {
// C: 	{ 0x0b, 0x41014111 },
// C: 	{ 0x0c, 0x414520f0 }, /* SPDIF out */
// C: 	{ 0x0d, 0x01014010 }, /* lineout */
// C: 	{ 0x0e, 0x41c501f0 },
// C: 	{ 0x0f, 0x411111f0 }, /* disabled */
// C: 	{ 0x10, 0x411111f0 }, /* disabled */
// C: 	{ 0x11, 0x41012014 },
// C: 	{ 0x12, 0x37a790f0 }, /* mic */
// C: 	{ 0x13, 0x77a701f0 },
// C: 	{ 0x18, 0x500000f0 },
// C: 	{}
// C: };
// C: 
// C: static const struct hda_quirk ca0132_quirks[] = {
// C: 	SND_PCI_QUIRK(0x1028, 0x057b, "Alienware M17x R4", QUIRK_ALIENWARE_M17XR4),
// C: 	SND_PCI_QUIRK(0x1028, 0x0685, "Alienware 15 2015", QUIRK_ALIENWARE),
// C: 	SND_PCI_QUIRK(0x1028, 0x0688, "Alienware 17 2015", QUIRK_ALIENWARE),
// C: 	SND_PCI_QUIRK(0x1028, 0x0708, "Alienware 15 R2 2016", QUIRK_ALIENWARE),
// C: 	SND_PCI_QUIRK(0x1102, 0x0010, "Sound Blaster Z", QUIRK_SBZ),
// C: 	SND_PCI_QUIRK(0x1102, 0x0023, "Sound Blaster Z", QUIRK_SBZ),
// C: 	SND_PCI_QUIRK(0x1102, 0x0027, "Sound Blaster Z", QUIRK_SBZ),
// C: 	SND_PCI_QUIRK(0x1102, 0x0033, "Sound Blaster ZxR", QUIRK_SBZ),
// C: 	SND_PCI_QUIRK(0x1458, 0xA016, "Recon3Di", QUIRK_R3DI),
// C: 	SND_PCI_QUIRK(0x1458, 0xA026, "Gigabyte G1.Sniper Z97", QUIRK_R3DI),
// C: 	SND_PCI_QUIRK(0x1458, 0xA036, "Gigabyte GA-Z170X-Gaming 7", QUIRK_R3DI),
// C: 	SND_PCI_QUIRK(0x1458, 0xA046, "Gigabyte GA-Z170X-Gaming G1", QUIRK_GENERIC),
// C: 	SND_PCI_QUIRK(0x3842, 0x1038, "EVGA X99 Classified", QUIRK_R3DI),
// C: 	SND_PCI_QUIRK(0x3842, 0x104b, "EVGA X299 Dark", QUIRK_R3DI),
// C: 	SND_PCI_QUIRK(0x3842, 0x1055, "EVGA Z390 DARK", QUIRK_R3DI),
// C: 	SND_PCI_QUIRK(0x1102, 0x0013, "Recon3D", QUIRK_R3D),
// C: 	SND_PCI_QUIRK(0x1102, 0x0018, "Recon3D", QUIRK_R3D),
// C: 	SND_PCI_QUIRK(0x1102, 0x0051, "Sound Blaster AE-5", QUIRK_AE5),
// C: 	SND_PCI_QUIRK(0x1102, 0x0191, "Sound Blaster AE-5 Plus", QUIRK_AE5),
// C: 	SND_PCI_QUIRK(0x1102, 0x0081, "Sound Blaster AE-7", QUIRK_AE7),
// C: 	{}
// C: };
// C: 
// C: static const struct hda_model_fixup ca0132_quirk_models[] = {
// C: 	{ .id = QUIRK_ALIENWARE, .name = "alienware" },
// C: 	{ .id = QUIRK_ALIENWARE_M17XR4, .name = "alienware-m17xr4" },
// C: 	{ .id = QUIRK_SBZ, .name = "sbz" },
// C: 	{ .id = QUIRK_ZXR, .name = "zxr" },
// C: 	{ .id = QUIRK_ZXR_DBPRO, .name = "zxr-dbpro" },
// C: 	{ .id = QUIRK_R3DI, .name = "r3di" },
// C: 	{ .id = QUIRK_R3D, .name = "r3d" },
// C: 	{ .id = QUIRK_AE5, .name = "ae5" },
// C: 	{ .id = QUIRK_AE7, .name = "ae7" },
// C: 	{ .id = QUIRK_GENERIC, .name = "generic" },
// C: 	{}
// C: };
// C: 
// C: /* Output selection quirk info structures. */
// C: #define MAX_QUIRK_MMIO_GPIO_SET_VALS 3
// C: #define MAX_QUIRK_SCP_SET_VALS 2
// C: struct ca0132_alt_out_set_info {
// C: 	unsigned int dac2port; /* ParamID 0x0d value. */
// C: 
// C: 	bool has_hda_gpio;
// C: 	char hda_gpio_pin;
// C: 	char hda_gpio_set;
// C: 
// C: 	unsigned int mmio_gpio_count;
// C: 	char mmio_gpio_pin[MAX_QUIRK_MMIO_GPIO_SET_VALS];
// C: 	char mmio_gpio_set[MAX_QUIRK_MMIO_GPIO_SET_VALS];
// C: 
// C: 	unsigned int scp_cmds_count;
// C: 	unsigned int scp_cmd_mid[MAX_QUIRK_SCP_SET_VALS];
// C: 	unsigned int scp_cmd_req[MAX_QUIRK_SCP_SET_VALS];
// C: 	unsigned int scp_cmd_val[MAX_QUIRK_SCP_SET_VALS];
// C: 
// C: 	bool has_chipio_write;
// C: 	unsigned int chipio_write_addr;
// C: 	unsigned int chipio_write_data;
// C: };
// C: 
// C: struct ca0132_alt_out_set_quirk_data {
// C: 	int quirk_id;
// C: 
// C: 	bool has_headphone_gain;
// C: 	bool is_ae_series;
// C: 
// C: 	struct ca0132_alt_out_set_info out_set_info[NUM_OF_OUTPUTS];
// C: };
// C: 
// C: static const struct ca0132_alt_out_set_quirk_data quirk_out_set_data[] = {
// C: 	{ .quirk_id = QUIRK_R3DI,
// C: 	  .has_headphone_gain = false,
// C: 	  .is_ae_series       = false,
// C: 	  .out_set_info = {
// C: 		/* Speakers. */
// C: 		{ .dac2port         = 0x24,
// C: 		  .has_hda_gpio     = true,
// C: 		  .hda_gpio_pin     = 2,
// C: 		  .hda_gpio_set     = 1,
// C: 		  .mmio_gpio_count  = 0,
// C: 		  .scp_cmds_count   = 0,
// C: 		  .has_chipio_write = false,
// C: 		},
// C: 		/* Headphones. */
// C: 		{ .dac2port         = 0x21,
// C: 		  .has_hda_gpio     = true,
// C: 		  .hda_gpio_pin     = 2,
// C: 		  .hda_gpio_set     = 0,
// C: 		  .mmio_gpio_count  = 0,
// C: 		  .scp_cmds_count   = 0,
// C: 		  .has_chipio_write = false,
// C: 		} },
// C: 	},
// C: 	{ .quirk_id = QUIRK_R3D,
// C: 	  .has_headphone_gain = false,
// C: 	  .is_ae_series       = false,
// C: 	  .out_set_info = {
// C: 		/* Speakers. */
// C: 		{ .dac2port         = 0x24,
// C: 		  .has_hda_gpio     = false,
// C: 		  .mmio_gpio_count  = 1,
// C: 		  .mmio_gpio_pin    = { 1 },
// C: 		  .mmio_gpio_set    = { 1 },
// C: 		  .scp_cmds_count   = 0,
// C: 		  .has_chipio_write = false,
// C: 		},
// C: 		/* Headphones. */
// C: 		{ .dac2port         = 0x21,
// C: 		  .has_hda_gpio     = false,
// C: 		  .mmio_gpio_count  = 1,
// C: 		  .mmio_gpio_pin    = { 1 },
// C: 		  .mmio_gpio_set    = { 0 },
// C: 		  .scp_cmds_count   = 0,
// C: 		  .has_chipio_write = false,
// C: 		} },
// C: 	},
// C: 	{ .quirk_id = QUIRK_SBZ,
// C: 	  .has_headphone_gain = false,
// C: 	  .is_ae_series       = false,
// C: 	  .out_set_info = {
// C: 		/* Speakers. */
// C: 		{ .dac2port         = 0x18,
// C: 		  .has_hda_gpio     = false,
// C: 		  .mmio_gpio_count  = 3,
// C: 		  .mmio_gpio_pin    = { 7, 4, 1 },
// C: 		  .mmio_gpio_set    = { 0, 1, 1 },
// C: 		  .scp_cmds_count   = 0,
// C: 		  .has_chipio_write = false, },
// C: 		/* Headphones. */
// C: 		{ .dac2port         = 0x12,
// C: 		  .has_hda_gpio     = false,
// C: 		  .mmio_gpio_count  = 3,
// C: 		  .mmio_gpio_pin    = { 7, 4, 1 },
// C: 		  .mmio_gpio_set    = { 1, 1, 0 },
// C: 		  .scp_cmds_count   = 0,
// C: 		  .has_chipio_write = false,
// C: 		} },
// C: 	},
// C: 	{ .quirk_id = QUIRK_ZXR,
// C: 	  .has_headphone_gain = true,
// C: 	  .is_ae_series       = false,
// C: 	  .out_set_info = {
// C: 		/* Speakers. */
// C: 		{ .dac2port         = 0x24,
// C: 		  .has_hda_gpio     = false,
// C: 		  .mmio_gpio_count  = 3,
// C: 		  .mmio_gpio_pin    = { 2, 3, 5 },
// C: 		  .mmio_gpio_set    = { 1, 1, 0 },
// C: 		  .scp_cmds_count   = 0,
// C: 		  .has_chipio_write = false,
// C: 		},
// C: 		/* Headphones. */
// C: 		{ .dac2port         = 0x21,
// C: 		  .has_hda_gpio     = false,
// C: 		  .mmio_gpio_count  = 3,
// C: 		  .mmio_gpio_pin    = { 2, 3, 5 },
// C: 		  .mmio_gpio_set    = { 0, 1, 1 },
// C: 		  .scp_cmds_count   = 0,
// C: 		  .has_chipio_write = false,
// C: 		} },
// C: 	},
// C: 	{ .quirk_id = QUIRK_AE5,
// C: 	  .has_headphone_gain = true,
// C: 	  .is_ae_series       = true,
// C: 	  .out_set_info = {
// C: 		/* Speakers. */
// C: 		{ .dac2port          = 0xa4,
// C: 		  .has_hda_gpio      = false,
// C: 		  .mmio_gpio_count   = 0,
// C: 		  .scp_cmds_count    = 2,
// C: 		  .scp_cmd_mid       = { 0x96, 0x96 },
// C: 		  .scp_cmd_req       = { SPEAKER_TUNING_FRONT_LEFT_INVERT,
// C: 					 SPEAKER_TUNING_FRONT_RIGHT_INVERT },
// C: 		  .scp_cmd_val       = { FLOAT_ZERO, FLOAT_ZERO },
// C: 		  .has_chipio_write  = true,
// C: 		  .chipio_write_addr = 0x0018b03c,
// C: 		  .chipio_write_data = 0x00000012
// C: 		},
// C: 		/* Headphones. */
// C: 		{ .dac2port          = 0xa1,
// C: 		  .has_hda_gpio      = false,
// C: 		  .mmio_gpio_count   = 0,
// C: 		  .scp_cmds_count    = 2,
// C: 		  .scp_cmd_mid       = { 0x96, 0x96 },
// C: 		  .scp_cmd_req       = { SPEAKER_TUNING_FRONT_LEFT_INVERT,
// C: 					 SPEAKER_TUNING_FRONT_RIGHT_INVERT },
// C: 		  .scp_cmd_val       = { FLOAT_ONE, FLOAT_ONE },
// C: 		  .has_chipio_write  = true,
// C: 		  .chipio_write_addr = 0x0018b03c,
// C: 		  .chipio_write_data = 0x00000012
// C: 		} },
// C: 	},
// C: 	{ .quirk_id = QUIRK_AE7,
// C: 	  .has_headphone_gain = true,
// C: 	  .is_ae_series       = true,
// C: 	  .out_set_info = {
// C: 		/* Speakers. */
// C: 		{ .dac2port          = 0x58,
// C: 		  .has_hda_gpio      = false,
// C: 		  .mmio_gpio_count   = 1,
// C: 		  .mmio_gpio_pin     = { 0 },
// C: 		  .mmio_gpio_set     = { 1 },
// C: 		  .scp_cmds_count    = 2,
// C: 		  .scp_cmd_mid       = { 0x96, 0x96 },
// C: 		  .scp_cmd_req       = { SPEAKER_TUNING_FRONT_LEFT_INVERT,
// C: 					 SPEAKER_TUNING_FRONT_RIGHT_INVERT },
// C: 		  .scp_cmd_val       = { FLOAT_ZERO, FLOAT_ZERO },
// C: 		  .has_chipio_write  = true,
// C: 		  .chipio_write_addr = 0x0018b03c,
// C: 		  .chipio_write_data = 0x00000000
// C: 		},
// C: 		/* Headphones. */
// C: 		{ .dac2port          = 0x58,
// C: 		  .has_hda_gpio      = false,
// C: 		  .mmio_gpio_count   = 1,
// C: 		  .mmio_gpio_pin     = { 0 },
// C: 		  .mmio_gpio_set     = { 1 },
// C: 		  .scp_cmds_count    = 2,
// C: 		  .scp_cmd_mid       = { 0x96, 0x96 },
// C: 		  .scp_cmd_req       = { SPEAKER_TUNING_FRONT_LEFT_INVERT,
// C: 					 SPEAKER_TUNING_FRONT_RIGHT_INVERT },
// C: 		  .scp_cmd_val       = { FLOAT_ONE, FLOAT_ONE },
// C: 		  .has_chipio_write  = true,
// C: 		  .chipio_write_addr = 0x0018b03c,
// C: 		  .chipio_write_data = 0x00000010
// C: 		} },
// C: 	}
// C: };
// C: 
// C: /*
// C:  * CA0132 codec access
// C:  */
// C: static unsigned int codec_send_command(struct hda_codec *codec, hda_nid_t nid,
// C: 		unsigned int verb, unsigned int parm, unsigned int *res)
// C: {
// C: 	unsigned int response;
// C: 	response = snd_hda_codec_read(codec, nid, 0, verb, parm);
// C: 	*res = response;
// C: 
// C: 	return ((response == -1) ? -1 : 0);
// C: }
// C: 
// C: static int codec_set_converter_format(struct hda_codec *codec, hda_nid_t nid,
// C: 		unsigned short converter_format, unsigned int *res)
// C: {
// C: 	return codec_send_command(codec, nid, VENDOR_CHIPIO_STREAM_FORMAT,
// C: 				converter_format & 0xffff, res);
// C: }
// C: 
// C: static int codec_set_converter_stream_channel(struct hda_codec *codec,
// C: 				hda_nid_t nid, unsigned char stream,
// C: 				unsigned char channel, unsigned int *res)
// C: {
// C: 	unsigned char converter_stream_channel = 0;
// C: 
// C: 	converter_stream_channel = (stream << 4) | (channel & 0x0f);
// C: 	return codec_send_command(codec, nid, AC_VERB_SET_CHANNEL_STREAMID,
// C: 				converter_stream_channel, res);
// C: }
// C: 
// C: /* Chip access helper function */
// C: static int chipio_send(struct hda_codec *codec,
// C: 		       unsigned int reg,
// C: 		       unsigned int data)
// C: {
// C: 	unsigned int res;
// C: 	unsigned long timeout = jiffies + msecs_to_jiffies(1000);
// C: 
// C: 	/* send bits of data specified by reg */
// C: 	do {
// C: 		res = snd_hda_codec_read(codec, WIDGET_CHIP_CTRL, 0,
// C: 					 reg, data);
// C: 		if (res == VENDOR_STATUS_CHIPIO_OK)
// C: 			return 0;
// C: 		msleep(20);
// C: 	} while (time_before(jiffies, timeout));
// C: 
// C: 	return -EIO;
// C: }
// C: 
// C: /*
// C:  * Write chip address through the vendor widget -- NOT protected by the Mutex!
// C:  */
// C: static int chipio_write_address(struct hda_codec *codec,
// C: 				unsigned int chip_addx)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int res;
// C: 
// C: 	if (spec->curr_chip_addx == chip_addx)
// C: 			return 0;
// C: 
// C: 	/* send low 16 bits of the address */
// C: 	res = chipio_send(codec, VENDOR_CHIPIO_ADDRESS_LOW,
// C: 			  chip_addx & 0xffff);
// C: 
// C: 	if (res != -EIO) {
// C: 		/* send high 16 bits of the address */
// C: 		res = chipio_send(codec, VENDOR_CHIPIO_ADDRESS_HIGH,
// C: 				  chip_addx >> 16);
// C: 	}
// C: 
// C: 	spec->curr_chip_addx = (res < 0) ? ~0U : chip_addx;
// C: 
// C: 	return res;
// C: }
// C: 
// C: /*
// C:  * Write data through the vendor widget -- NOT protected by the Mutex!
// C:  */
// C: static int chipio_write_data(struct hda_codec *codec, unsigned int data)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int res;
// C: 
// C: 	/* send low 16 bits of the data */
// C: 	res = chipio_send(codec, VENDOR_CHIPIO_DATA_LOW, data & 0xffff);
// C: 
// C: 	if (res != -EIO) {
// C: 		/* send high 16 bits of the data */
// C: 		res = chipio_send(codec, VENDOR_CHIPIO_DATA_HIGH,
// C: 				  data >> 16);
// C: 	}
// C: 
// C: 	/*If no error encountered, automatically increment the address
// C: 	as per chip behaviour*/
// C: 	spec->curr_chip_addx = (res != -EIO) ?
// C: 					(spec->curr_chip_addx + 4) : ~0U;
// C: 	return res;
// C: }
// C: 
// C: /*
// C:  * Write multiple data through the vendor widget -- NOT protected by the Mutex!
// C:  */
// C: static int chipio_write_data_multiple(struct hda_codec *codec,
// C: 				      const u32 *data,
// C: 				      unsigned int count)
// C: {
// C: 	int status = 0;
// C: 
// C: 	if (data == NULL) {
// C: 		codec_dbg(codec, "chipio_write_data null ptr\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	while ((count-- != 0) && (status == 0))
// C: 		status = chipio_write_data(codec, *data++);
// C: 
// C: 	return status;
// C: }
// C: 
// C: 
// C: /*
// C:  * Read data through the vendor widget -- NOT protected by the Mutex!
// C:  */
// C: static int chipio_read_data(struct hda_codec *codec, unsigned int *data)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int res;
// C: 
// C: 	/* post read */
// C: 	res = chipio_send(codec, VENDOR_CHIPIO_HIC_POST_READ, 0);
// C: 
// C: 	if (res != -EIO) {
// C: 		/* read status */
// C: 		res = chipio_send(codec, VENDOR_CHIPIO_STATUS, 0);
// C: 	}
// C: 
// C: 	if (res != -EIO) {
// C: 		/* read data */
// C: 		*data = snd_hda_codec_read(codec, WIDGET_CHIP_CTRL, 0,
// C: 					   VENDOR_CHIPIO_HIC_READ_DATA,
// C: 					   0);
// C: 	}
// C: 
// C: 	/*If no error encountered, automatically increment the address
// C: 	as per chip behaviour*/
// C: 	spec->curr_chip_addx = (res != -EIO) ?
// C: 					(spec->curr_chip_addx + 4) : ~0U;
// C: 	return res;
// C: }
// C: 
// C: /*
// C:  * Write given value to the given address through the chip I/O widget.
// C:  * protected by the Mutex
// C:  */
// C: static int chipio_write(struct hda_codec *codec,
// C: 		unsigned int chip_addx, const unsigned int data)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int err;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	/* write the address, and if successful proceed to write data */
// C: 	err = chipio_write_address(codec, chip_addx);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	return chipio_write_data(codec, data);
// C: }
// C: 
// C: /*
// C:  * Write given value to the given address through the chip I/O widget.
// C:  * not protected by the Mutex
// C:  */
// C: static int chipio_write_no_mutex(struct hda_codec *codec,
// C: 		unsigned int chip_addx, const unsigned int data)
// C: {
// C: 	int err;
// C: 
// C: 
// C: 	/* write the address, and if successful proceed to write data */
// C: 	err = chipio_write_address(codec, chip_addx);
// C: 	if (err < 0)
// C: 		goto exit;
// C: 
// C: 	err = chipio_write_data(codec, data);
// C: 	if (err < 0)
// C: 		goto exit;
// C: 
// C: exit:
// C: 	return err;
// C: }
// C: 
// C: /*
// C:  * Write multiple values to the given address through the chip I/O widget.
// C:  * protected by the Mutex
// C:  */
// C: static int chipio_write_multiple(struct hda_codec *codec,
// C: 				 u32 chip_addx,
// C: 				 const u32 *data,
// C: 				 unsigned int count)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int status;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 	status = chipio_write_address(codec, chip_addx);
// C: 	if (status < 0)
// C: 		return status;
// C: 
// C: 	return chipio_write_data_multiple(codec, data, count);
// C: }
// C: 
// C: /*
// C:  * Read the given address through the chip I/O widget
// C:  * protected by the Mutex
// C:  */
// C: static int chipio_read(struct hda_codec *codec,
// C: 		unsigned int chip_addx, unsigned int *data)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int err;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	/* write the address, and if successful proceed to write data */
// C: 	err = chipio_write_address(codec, chip_addx);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	return chipio_read_data(codec, data);
// C: }
// C: 
// C: /*
// C:  * Set chip control flags through the chip I/O widget.
// C:  */
// C: static void chipio_set_control_flag(struct hda_codec *codec,
// C: 				    enum control_flag_id flag_id,
// C: 				    bool flag_state)
// C: {
// C: 	unsigned int val;
// C: 	unsigned int flag_bit;
// C: 
// C: 	flag_bit = (flag_state ? 1 : 0);
// C: 	val = (flag_bit << 7) | (flag_id);
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 			    VENDOR_CHIPIO_FLAG_SET, val);
// C: }
// C: 
// C: /*
// C:  * Set chip parameters through the chip I/O widget.
// C:  */
// C: static void chipio_set_control_param(struct hda_codec *codec,
// C: 		enum control_param_id param_id, int param_val)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int val;
// C: 
// C: 	if ((param_id < 32) && (param_val < 8)) {
// C: 		val = (param_val << 5) | (param_id);
// C: 		snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 				    VENDOR_CHIPIO_PARAM_SET, val);
// C: 	} else {
// C: 		guard(mutex)(&spec->chipio_mutex);
// C: 		if (chipio_send(codec, VENDOR_CHIPIO_STATUS, 0) == 0) {
// C: 			snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 					    VENDOR_CHIPIO_PARAM_EX_ID_SET,
// C: 					    param_id);
// C: 			snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 					    VENDOR_CHIPIO_PARAM_EX_VALUE_SET,
// C: 					    param_val);
// C: 		}
// C: 	}
// C: }
// C: 
// C: /*
// C:  * Set chip parameters through the chip I/O widget. NO MUTEX.
// C:  */
// C: static void chipio_set_control_param_no_mutex(struct hda_codec *codec,
// C: 		enum control_param_id param_id, int param_val)
// C: {
// C: 	int val;
// C: 
// C: 	if ((param_id < 32) && (param_val < 8)) {
// C: 		val = (param_val << 5) | (param_id);
// C: 		snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 				    VENDOR_CHIPIO_PARAM_SET, val);
// C: 	} else {
// C: 		if (chipio_send(codec, VENDOR_CHIPIO_STATUS, 0) == 0) {
// C: 			snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 					    VENDOR_CHIPIO_PARAM_EX_ID_SET,
// C: 					    param_id);
// C: 			snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 					    VENDOR_CHIPIO_PARAM_EX_VALUE_SET,
// C: 					    param_val);
// C: 		}
// C: 	}
// C: }
// C: /*
// C:  * Connect stream to a source point, and then connect
// C:  * that source point to a destination point.
// C:  */
// C: static void chipio_set_stream_source_dest(struct hda_codec *codec,
// C: 				int streamid, int source_point, int dest_point)
// C: {
// C: 	chipio_set_control_param_no_mutex(codec,
// C: 			CONTROL_PARAM_STREAM_ID, streamid);
// C: 	chipio_set_control_param_no_mutex(codec,
// C: 			CONTROL_PARAM_STREAM_SOURCE_CONN_POINT, source_point);
// C: 	chipio_set_control_param_no_mutex(codec,
// C: 			CONTROL_PARAM_STREAM_DEST_CONN_POINT, dest_point);
// C: }
// C: 
// C: /*
// C:  * Set number of channels in the selected stream.
// C:  */
// C: static void chipio_set_stream_channels(struct hda_codec *codec,
// C: 				int streamid, unsigned int channels)
// C: {
// C: 	chipio_set_control_param_no_mutex(codec,
// C: 			CONTROL_PARAM_STREAM_ID, streamid);
// C: 	chipio_set_control_param_no_mutex(codec,
// C: 			CONTROL_PARAM_STREAMS_CHANNELS, channels);
// C: }
// C: 
// C: /*
// C:  * Enable/Disable audio stream.
// C:  */
// C: static void chipio_set_stream_control(struct hda_codec *codec,
// C: 				int streamid, int enable)
// C: {
// C: 	chipio_set_control_param_no_mutex(codec,
// C: 			CONTROL_PARAM_STREAM_ID, streamid);
// C: 	chipio_set_control_param_no_mutex(codec,
// C: 			CONTROL_PARAM_STREAM_CONTROL, enable);
// C: }
// C: 
// C: /*
// C:  * Get ChipIO audio stream's status.
// C:  */
// C: static void chipio_get_stream_control(struct hda_codec *codec,
// C: 				int streamid, unsigned int *enable)
// C: {
// C: 	chipio_set_control_param_no_mutex(codec,
// C: 			CONTROL_PARAM_STREAM_ID, streamid);
// C: 	*enable = snd_hda_codec_read(codec, WIDGET_CHIP_CTRL, 0,
// C: 			   VENDOR_CHIPIO_PARAM_GET,
// C: 			   CONTROL_PARAM_STREAM_CONTROL);
// C: }
// C: 
// C: /*
// C:  * Set sampling rate of the connection point. NO MUTEX.
// C:  */
// C: static void chipio_set_conn_rate_no_mutex(struct hda_codec *codec,
// C: 				int connid, enum ca0132_sample_rate rate)
// C: {
// C: 	chipio_set_control_param_no_mutex(codec,
// C: 			CONTROL_PARAM_CONN_POINT_ID, connid);
// C: 	chipio_set_control_param_no_mutex(codec,
// C: 			CONTROL_PARAM_CONN_POINT_SAMPLE_RATE, rate);
// C: }
// C: 
// C: /*
// C:  * Set sampling rate of the connection point.
// C:  */
// C: static void chipio_set_conn_rate(struct hda_codec *codec,
// C: 				int connid, enum ca0132_sample_rate rate)
// C: {
// C: 	chipio_set_control_param(codec, CONTROL_PARAM_CONN_POINT_ID, connid);
// C: 	chipio_set_control_param(codec, CONTROL_PARAM_CONN_POINT_SAMPLE_RATE,
// C: 				 rate);
// C: }
// C: 
// C: /*
// C:  * Writes to the 8051's internal address space directly instead of indirectly,
// C:  * giving access to the special function registers located at addresses
// C:  * 0x80-0xFF.
// C:  */
// C: static void chipio_8051_write_direct(struct hda_codec *codec,
// C: 		unsigned int addr, unsigned int data)
// C: {
// C: 	unsigned int verb;
// C: 
// C: 	verb = VENDOR_CHIPIO_8051_WRITE_DIRECT | data;
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0, verb, addr);
// C: }
// C: 
// C: /*
// C:  * Writes to the 8051's exram, which has 16-bits of address space.
// C:  * Data at addresses 0x2000-0x7fff is mirrored to 0x8000-0xdfff.
// C:  * Data at 0x8000-0xdfff can also be used as program memory for the 8051 by
// C:  * setting the pmem bank selection SFR.
// C:  * 0xe000-0xffff is always mapped as program memory, with only 0xf000-0xffff
// C:  * being writable.
// C:  */
// C: static void chipio_8051_set_address(struct hda_codec *codec, unsigned int addr)
// C: {
// C: 	unsigned int tmp;
// C: 
// C: 	/* Lower 8-bits. */
// C: 	tmp = addr & 0xff;
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 			    VENDOR_CHIPIO_8051_ADDRESS_LOW, tmp);
// C: 
// C: 	/* Upper 8-bits. */
// C: 	tmp = (addr >> 8) & 0xff;
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 			    VENDOR_CHIPIO_8051_ADDRESS_HIGH, tmp);
// C: }
// C: 
// C: static void chipio_8051_set_data(struct hda_codec *codec, unsigned int data)
// C: {
// C: 	/* 8-bits of data. */
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 			    VENDOR_CHIPIO_8051_DATA_WRITE, data & 0xff);
// C: }
// C: 
// C: static unsigned int chipio_8051_get_data(struct hda_codec *codec)
// C: {
// C: 	return snd_hda_codec_read(codec, WIDGET_CHIP_CTRL, 0,
// C: 				   VENDOR_CHIPIO_8051_DATA_READ, 0);
// C: }
// C: 
// C: /* PLL_PMU writes share the lower address register of the 8051 exram writes. */
// C: static void chipio_8051_set_data_pll(struct hda_codec *codec, unsigned int data)
// C: {
// C: 	/* 8-bits of data. */
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 			    VENDOR_CHIPIO_PLL_PMU_WRITE, data & 0xff);
// C: }
// C: 
// C: static void chipio_8051_write_exram(struct hda_codec *codec,
// C: 		unsigned int addr, unsigned int data)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	chipio_8051_set_address(codec, addr);
// C: 	chipio_8051_set_data(codec, data);
// C: }
// C: 
// C: static void chipio_8051_write_exram_no_mutex(struct hda_codec *codec,
// C: 		unsigned int addr, unsigned int data)
// C: {
// C: 	chipio_8051_set_address(codec, addr);
// C: 	chipio_8051_set_data(codec, data);
// C: }
// C: 
// C: /* Readback data from the 8051's exram. No mutex. */
// C: static void chipio_8051_read_exram(struct hda_codec *codec,
// C: 		unsigned int addr, unsigned int *data)
// C: {
// C: 	chipio_8051_set_address(codec, addr);
// C: 	*data = chipio_8051_get_data(codec);
// C: }
// C: 
// C: static void chipio_8051_write_pll_pmu(struct hda_codec *codec,
// C: 		unsigned int addr, unsigned int data)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	chipio_8051_set_address(codec, addr & 0xff);
// C: 	chipio_8051_set_data_pll(codec, data);
// C: }
// C: 
// C: static void chipio_8051_write_pll_pmu_no_mutex(struct hda_codec *codec,
// C: 		unsigned int addr, unsigned int data)
// C: {
// C: 	chipio_8051_set_address(codec, addr & 0xff);
// C: 	chipio_8051_set_data_pll(codec, data);
// C: }
// C: 
// C: /*
// C:  * Enable clocks.
// C:  */
// C: static void chipio_enable_clocks(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	chipio_8051_write_pll_pmu_no_mutex(codec, 0x00, 0xff);
// C: 	chipio_8051_write_pll_pmu_no_mutex(codec, 0x05, 0x0b);
// C: 	chipio_8051_write_pll_pmu_no_mutex(codec, 0x06, 0xff);
// C: }
// C: 
// C: /*
// C:  * CA0132 DSP IO stuffs
// C:  */
// C: static int dspio_send(struct hda_codec *codec, unsigned int reg,
// C: 		      unsigned int data)
// C: {
// C: 	int res;
// C: 	unsigned long timeout = jiffies + msecs_to_jiffies(1000);
// C: 
// C: 	/* send bits of data specified by reg to dsp */
// C: 	do {
// C: 		res = snd_hda_codec_read(codec, WIDGET_DSP_CTRL, 0, reg, data);
// C: 		if ((res >= 0) && (res != VENDOR_STATUS_DSPIO_BUSY))
// C: 			return res;
// C: 		msleep(20);
// C: 	} while (time_before(jiffies, timeout));
// C: 
// C: 	return -EIO;
// C: }
// C: 
// C: /*
// C:  * Wait for DSP to be ready for commands
// C:  */
// C: static void dspio_write_wait(struct hda_codec *codec)
// C: {
// C: 	int status;
// C: 	unsigned long timeout = jiffies + msecs_to_jiffies(1000);
// C: 
// C: 	do {
// C: 		status = snd_hda_codec_read(codec, WIDGET_DSP_CTRL, 0,
// C: 						VENDOR_DSPIO_STATUS, 0);
// C: 		if ((status == VENDOR_STATUS_DSPIO_OK) ||
// C: 		    (status == VENDOR_STATUS_DSPIO_SCP_RESPONSE_QUEUE_EMPTY))
// C: 			break;
// C: 		msleep(1);
// C: 	} while (time_before(jiffies, timeout));
// C: }
// C: 
// C: /*
// C:  * Write SCP data to DSP
// C:  */
// C: static int dspio_write(struct hda_codec *codec, unsigned int scp_data)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int status;
// C: 
// C: 	dspio_write_wait(codec);
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 	status = dspio_send(codec, VENDOR_DSPIO_SCP_WRITE_DATA_LOW,
// C: 			    scp_data & 0xffff);
// C: 	if (status < 0)
// C: 		return status;
// C: 
// C: 	status = dspio_send(codec, VENDOR_DSPIO_SCP_WRITE_DATA_HIGH,
// C: 				    scp_data >> 16);
// C: 	if (status < 0)
// C: 		return status;
// C: 
// C: 	/* OK, now check if the write itself has executed*/
// C: 	status = snd_hda_codec_read(codec, WIDGET_DSP_CTRL, 0,
// C: 				    VENDOR_DSPIO_STATUS, 0);
// C: 
// C: 	return (status == VENDOR_STATUS_DSPIO_SCP_COMMAND_QUEUE_FULL) ?
// C: 			-EIO : 0;
// C: }
// C: 
// C: /*
// C:  * Write multiple SCP data to DSP
// C:  */
// C: static int dspio_write_multiple(struct hda_codec *codec,
// C: 				unsigned int *buffer, unsigned int size)
// C: {
// C: 	int status = 0;
// C: 	unsigned int count;
// C: 
// C: 	if (buffer == NULL)
// C: 		return -EINVAL;
// C: 
// C: 	count = 0;
// C: 	while (count < size) {
// C: 		status = dspio_write(codec, *buffer++);
// C: 		if (status != 0)
// C: 			break;
// C: 		count++;
// C: 	}
// C: 
// C: 	return status;
// C: }
// C: 
// C: static int dspio_read(struct hda_codec *codec, unsigned int *data)
// C: {
// C: 	int status;
// C: 
// C: 	status = dspio_send(codec, VENDOR_DSPIO_SCP_POST_READ_DATA, 0);
// C: 	if (status == -EIO)
// C: 		return status;
// C: 
// C: 	status = dspio_send(codec, VENDOR_DSPIO_STATUS, 0);
// C: 	if (status == -EIO ||
// C: 	    status == VENDOR_STATUS_DSPIO_SCP_RESPONSE_QUEUE_EMPTY)
// C: 		return -EIO;
// C: 
// C: 	*data = snd_hda_codec_read(codec, WIDGET_DSP_CTRL, 0,
// C: 				   VENDOR_DSPIO_SCP_READ_DATA, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int dspio_read_multiple(struct hda_codec *codec, unsigned int *buffer,
// C: 			       unsigned int *buf_size, unsigned int size_count)
// C: {
// C: 	int status = 0;
// C: 	unsigned int size = *buf_size;
// C: 	unsigned int count;
// C: 	unsigned int skip_count;
// C: 	unsigned int dummy;
// C: 
// C: 	if (buffer == NULL)
// C: 		return -1;
// C: 
// C: 	count = 0;
// C: 	while (count < size && count < size_count) {
// C: 		status = dspio_read(codec, buffer++);
// C: 		if (status != 0)
// C: 			break;
// C: 		count++;
// C: 	}
// C: 
// C: 	skip_count = count;
// C: 	if (status == 0) {
// C: 		while (skip_count < size) {
// C: 			status = dspio_read(codec, &dummy);
// C: 			if (status != 0)
// C: 				break;
// C: 			skip_count++;
// C: 		}
// C: 	}
// C: 	*buf_size = count;
// C: 
// C: 	return status;
// C: }
// C: 
// C: /*
// C:  * Construct the SCP header using corresponding fields
// C:  */
// C: static inline unsigned int
// C: make_scp_header(unsigned int target_id, unsigned int source_id,
// C: 		unsigned int get_flag, unsigned int req,
// C: 		unsigned int device_flag, unsigned int resp_flag,
// C: 		unsigned int error_flag, unsigned int data_size)
// C: {
// C: 	unsigned int header = 0;
// C: 
// C: 	header = (data_size & 0x1f) << 27;
// C: 	header |= (error_flag & 0x01) << 26;
// C: 	header |= (resp_flag & 0x01) << 25;
// C: 	header |= (device_flag & 0x01) << 24;
// C: 	header |= (req & 0x7f) << 17;
// C: 	header |= (get_flag & 0x01) << 16;
// C: 	header |= (source_id & 0xff) << 8;
// C: 	header |= target_id & 0xff;
// C: 
// C: 	return header;
// C: }
// C: 
// C: /*
// C:  * Extract corresponding fields from SCP header
// C:  */
// C: static inline void
// C: extract_scp_header(unsigned int header,
// C: 		   unsigned int *target_id, unsigned int *source_id,
// C: 		   unsigned int *get_flag, unsigned int *req,
// C: 		   unsigned int *device_flag, unsigned int *resp_flag,
// C: 		   unsigned int *error_flag, unsigned int *data_size)
// C: {
// C: 	if (data_size)
// C: 		*data_size = (header >> 27) & 0x1f;
// C: 	if (error_flag)
// C: 		*error_flag = (header >> 26) & 0x01;
// C: 	if (resp_flag)
// C: 		*resp_flag = (header >> 25) & 0x01;
// C: 	if (device_flag)
// C: 		*device_flag = (header >> 24) & 0x01;
// C: 	if (req)
// C: 		*req = (header >> 17) & 0x7f;
// C: 	if (get_flag)
// C: 		*get_flag = (header >> 16) & 0x01;
// C: 	if (source_id)
// C: 		*source_id = (header >> 8) & 0xff;
// C: 	if (target_id)
// C: 		*target_id = header & 0xff;
// C: }
// C: 
// C: #define SCP_MAX_DATA_WORDS  (16)
// C: 
// C: /* Structure to contain any SCP message */
// C: struct scp_msg {
// C: 	unsigned int hdr;
// C: 	unsigned int data[SCP_MAX_DATA_WORDS];
// C: };
// C: 
// C: static void dspio_clear_response_queue(struct hda_codec *codec)
// C: {
// C: 	unsigned long timeout = jiffies + msecs_to_jiffies(1000);
// C: 	unsigned int dummy = 0;
// C: 	int status;
// C: 
// C: 	/* clear all from the response queue */
// C: 	do {
// C: 		status = dspio_read(codec, &dummy);
// C: 	} while (status == 0 && time_before(jiffies, timeout));
// C: }
// C: 
// C: static int dspio_get_response_data(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int data = 0;
// C: 	unsigned int count;
// C: 
// C: 	if (dspio_read(codec, &data) < 0)
// C: 		return -EIO;
// C: 
// C: 	if ((data & 0x00ffffff) == spec->wait_scp_header) {
// C: 		spec->scp_resp_header = data;
// C: 		spec->scp_resp_count = data >> 27;
// C: 		count = spec->wait_num_data;
// C: 		dspio_read_multiple(codec, spec->scp_resp_data,
// C: 				    &spec->scp_resp_count, count);
// C: 		return 0;
// C: 	}
// C: 
// C: 	return -EIO;
// C: }
// C: 
// C: /*
// C:  * Send SCP message to DSP
// C:  */
// C: static int dspio_send_scp_message(struct hda_codec *codec,
// C: 				  unsigned char *send_buf,
// C: 				  unsigned int send_buf_size,
// C: 				  unsigned char *return_buf,
// C: 				  unsigned int return_buf_size,
// C: 				  unsigned int *bytes_returned)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int status;
// C: 	unsigned int scp_send_size = 0;
// C: 	unsigned int total_size;
// C: 	bool waiting_for_resp = false;
// C: 	unsigned int header;
// C: 	struct scp_msg *ret_msg;
// C: 	unsigned int resp_src_id, resp_target_id;
// C: 	unsigned int data_size, src_id, target_id, get_flag, device_flag;
// C: 
// C: 	if (bytes_returned)
// C: 		*bytes_returned = 0;
// C: 
// C: 	/* get scp header from buffer */
// C: 	header = *((unsigned int *)send_buf);
// C: 	extract_scp_header(header, &target_id, &src_id, &get_flag, NULL,
// C: 			   &device_flag, NULL, NULL, &data_size);
// C: 	scp_send_size = data_size + 1;
// C: 	total_size = (scp_send_size * 4);
// C: 
// C: 	if (send_buf_size < total_size)
// C: 		return -EINVAL;
// C: 
// C: 	if (get_flag || device_flag) {
// C: 		if (!return_buf || return_buf_size < 4 || !bytes_returned)
// C: 			return -EINVAL;
// C: 
// C: 		spec->wait_scp_header = *((unsigned int *)send_buf);
// C: 
// C: 		/* swap source id with target id */
// C: 		resp_target_id = src_id;
// C: 		resp_src_id = target_id;
// C: 		spec->wait_scp_header &= 0xffff0000;
// C: 		spec->wait_scp_header |= (resp_src_id << 8) | (resp_target_id);
// C: 		spec->wait_num_data = return_buf_size/sizeof(unsigned int) - 1;
// C: 		spec->wait_scp = 1;
// C: 		waiting_for_resp = true;
// C: 	}
// C: 
// C: 	status = dspio_write_multiple(codec, (unsigned int *)send_buf,
// C: 				      scp_send_size);
// C: 	if (status < 0) {
// C: 		spec->wait_scp = 0;
// C: 		return status;
// C: 	}
// C: 
// C: 	if (waiting_for_resp) {
// C: 		unsigned long timeout = jiffies + msecs_to_jiffies(1000);
// C: 		memset(return_buf, 0, return_buf_size);
// C: 		do {
// C: 			msleep(20);
// C: 		} while (spec->wait_scp && time_before(jiffies, timeout));
// C: 		waiting_for_resp = false;
// C: 		if (!spec->wait_scp) {
// C: 			ret_msg = (struct scp_msg *)return_buf;
// C: 			memcpy(&ret_msg->hdr, &spec->scp_resp_header, 4);
// C: 			memcpy(&ret_msg->data, spec->scp_resp_data,
// C: 			       spec->wait_num_data);
// C: 			*bytes_returned = (spec->scp_resp_count + 1) * 4;
// C: 			status = 0;
// C: 		} else {
// C: 			status = -EIO;
// C: 		}
// C: 		spec->wait_scp = 0;
// C: 	}
// C: 
// C: 	return status;
// C: }
// C: 
// C: /**
// C:  * dspio_scp - Prepare and send the SCP message to DSP
// C:  * @codec: the HDA codec
// C:  * @mod_id: ID of the DSP module to send the command
// C:  * @src_id: ID of the source
// C:  * @req: ID of request to send to the DSP module
// C:  * @dir: SET or GET
// C:  * @data: pointer to the data to send with the request, request specific
// C:  * @len: length of the data, in bytes
// C:  * @reply: point to the buffer to hold data returned for a reply
// C:  * @reply_len: length of the reply buffer returned from GET
// C:  *
// C:  * Returns zero or a negative error code.
// C:  */
// C: static int dspio_scp(struct hda_codec *codec,
// C: 		int mod_id, int src_id, int req, int dir, const void *data,
// C: 		unsigned int len, void *reply, unsigned int *reply_len)
// C: {
// C: 	int status = 0;
// C: 	struct scp_msg scp_send, scp_reply;
// C: 	unsigned int ret_bytes, send_size, ret_size;
// C: 	unsigned int send_get_flag, reply_resp_flag, reply_error_flag;
// C: 	unsigned int reply_data_size;
// C: 
// C: 	memset(&scp_send, 0, sizeof(scp_send));
// C: 	memset(&scp_reply, 0, sizeof(scp_reply));
// C: 
// C: 	if ((len != 0 && data == NULL) || (len > SCP_MAX_DATA_WORDS))
// C: 		return -EINVAL;
// C: 
// C: 	if (dir == SCP_GET && reply == NULL) {
// C: 		codec_dbg(codec, "dspio_scp get but has no buffer\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	if (reply != NULL && (reply_len == NULL || (*reply_len == 0))) {
// C: 		codec_dbg(codec, "dspio_scp bad resp buf len parms\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	scp_send.hdr = make_scp_header(mod_id, src_id, (dir == SCP_GET), req,
// C: 				       0, 0, 0, len/sizeof(unsigned int));
// C: 	if (data != NULL && len > 0) {
// C: 		len = min((unsigned int)(sizeof(scp_send.data)), len);
// C: 		memcpy(scp_send.data, data, len);
// C: 	}
// C: 
// C: 	ret_bytes = 0;
// C: 	send_size = sizeof(unsigned int) + len;
// C: 	status = dspio_send_scp_message(codec, (unsigned char *)&scp_send,
// C: 					send_size, (unsigned char *)&scp_reply,
// C: 					sizeof(scp_reply), &ret_bytes);
// C: 
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "dspio_scp: send scp msg failed\\n");
// C: 		return status;
// C: 	}
// C: 
// C: 	/* extract send and reply headers members */
// C: 	extract_scp_header(scp_send.hdr, NULL, NULL, &send_get_flag,
// C: 			   NULL, NULL, NULL, NULL, NULL);
// C: 	extract_scp_header(scp_reply.hdr, NULL, NULL, NULL, NULL, NULL,
// C: 			   &reply_resp_flag, &reply_error_flag,
// C: 			   &reply_data_size);
// C: 
// C: 	if (!send_get_flag)
// C: 		return 0;
// C: 
// C: 	if (reply_resp_flag && !reply_error_flag) {
// C: 		ret_size = (ret_bytes - sizeof(scp_reply.hdr))
// C: 					/ sizeof(unsigned int);
// C: 
// C: 		if (*reply_len < ret_size*sizeof(unsigned int)) {
// C: 			codec_dbg(codec, "reply too long for buf\\n");
// C: 			return -EINVAL;
// C: 		} else if (ret_size != reply_data_size) {
// C: 			codec_dbg(codec, "RetLen and HdrLen .NE.\\n");
// C: 			return -EINVAL;
// C: 		} else if (!reply) {
// C: 			codec_dbg(codec, "NULL reply\\n");
// C: 			return -EINVAL;
// C: 		} else {
// C: 			*reply_len = ret_size*sizeof(unsigned int);
// C: 			memcpy(reply, scp_reply.data, *reply_len);
// C: 		}
// C: 	} else {
// C: 		codec_dbg(codec, "reply ill-formed or errflag set\\n");
// C: 		return -EIO;
// C: 	}
// C: 
// C: 	return status;
// C: }
// C: 
// C: /*
// C:  * Set DSP parameters
// C:  */
// C: static int dspio_set_param(struct hda_codec *codec, int mod_id,
// C: 			int src_id, int req, const void *data, unsigned int len)
// C: {
// C: 	return dspio_scp(codec, mod_id, src_id, req, SCP_SET, data, len, NULL,
// C: 			NULL);
// C: }
// C: 
// C: static int dspio_set_uint_param(struct hda_codec *codec, int mod_id,
// C: 			int req, const unsigned int data)
// C: {
// C: 	return dspio_set_param(codec, mod_id, 0x20, req, &data,
// C: 			sizeof(unsigned int));
// C: }
// C: 
// C: /*
// C:  * Allocate a DSP DMA channel via an SCP message
// C:  */
// C: static int dspio_alloc_dma_chan(struct hda_codec *codec, unsigned int *dma_chan)
// C: {
// C: 	int status = 0;
// C: 	unsigned int size = sizeof(*dma_chan);
// C: 
// C: 	codec_dbg(codec, "     dspio_alloc_dma_chan() -- begin\\n");
// C: 	status = dspio_scp(codec, MASTERCONTROL, 0x20,
// C: 			MASTERCONTROL_ALLOC_DMA_CHAN, SCP_GET, NULL, 0,
// C: 			dma_chan, &size);
// C: 
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "dspio_alloc_dma_chan: SCP Failed\\n");
// C: 		return status;
// C: 	}
// C: 
// C: 	if ((*dma_chan + 1) == 0) {
// C: 		codec_dbg(codec, "no free dma channels to allocate\\n");
// C: 		return -EBUSY;
// C: 	}
// C: 
// C: 	codec_dbg(codec, "dspio_alloc_dma_chan: chan=%d\\n", *dma_chan);
// C: 	codec_dbg(codec, "     dspio_alloc_dma_chan() -- complete\\n");
// C: 
// C: 	return status;
// C: }
// C: 
// C: /*
// C:  * Free a DSP DMA via an SCP message
// C:  */
// C: static int dspio_free_dma_chan(struct hda_codec *codec, unsigned int dma_chan)
// C: {
// C: 	int status = 0;
// C: 	unsigned int dummy = 0;
// C: 
// C: 	codec_dbg(codec, "     dspio_free_dma_chan() -- begin\\n");
// C: 	codec_dbg(codec, "dspio_free_dma_chan: chan=%d\\n", dma_chan);
// C: 
// C: 	status = dspio_scp(codec, MASTERCONTROL, 0x20,
// C: 			MASTERCONTROL_ALLOC_DMA_CHAN, SCP_SET, &dma_chan,
// C: 			sizeof(dma_chan), NULL, &dummy);
// C: 
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "dspio_free_dma_chan: SCP Failed\\n");
// C: 		return status;
// C: 	}
// C: 
// C: 	codec_dbg(codec, "     dspio_free_dma_chan() -- complete\\n");
// C: 
// C: 	return status;
// C: }
// C: 
// C: /*
// C:  * (Re)start the DSP
// C:  */
// C: static int dsp_set_run_state(struct hda_codec *codec)
// C: {
// C: 	unsigned int dbg_ctrl_reg;
// C: 	unsigned int halt_state;
// C: 	int err;
// C: 
// C: 	err = chipio_read(codec, DSP_DBGCNTL_INST_OFFSET, &dbg_ctrl_reg);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	halt_state = (dbg_ctrl_reg & DSP_DBGCNTL_STATE_MASK) >>
// C: 		      DSP_DBGCNTL_STATE_LOBIT;
// C: 
// C: 	if (halt_state != 0) {
// C: 		dbg_ctrl_reg &= ~((halt_state << DSP_DBGCNTL_SS_LOBIT) &
// C: 				  DSP_DBGCNTL_SS_MASK);
// C: 		err = chipio_write(codec, DSP_DBGCNTL_INST_OFFSET,
// C: 				   dbg_ctrl_reg);
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		dbg_ctrl_reg |= (halt_state << DSP_DBGCNTL_EXEC_LOBIT) &
// C: 				DSP_DBGCNTL_EXEC_MASK;
// C: 		err = chipio_write(codec, DSP_DBGCNTL_INST_OFFSET,
// C: 				   dbg_ctrl_reg);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * Reset the DSP
// C:  */
// C: static int dsp_reset(struct hda_codec *codec)
// C: {
// C: 	unsigned int res;
// C: 	int retry = 20;
// C: 
// C: 	codec_dbg(codec, "dsp_reset\\n");
// C: 	do {
// C: 		res = dspio_send(codec, VENDOR_DSPIO_DSP_INIT, 0);
// C: 		retry--;
// C: 	} while (res == -EIO && retry);
// C: 
// C: 	if (!retry) {
// C: 		codec_dbg(codec, "dsp_reset timeout\\n");
// C: 		return -EIO;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * Convert chip address to DSP address
// C:  */
// C: static unsigned int dsp_chip_to_dsp_addx(unsigned int chip_addx,
// C: 					bool *code, bool *yram)
// C: {
// C: 	*code = *yram = false;
// C: 
// C: 	if (UC_RANGE(chip_addx, 1)) {
// C: 		*code = true;
// C: 		return UC_OFF(chip_addx);
// C: 	} else if (X_RANGE_ALL(chip_addx, 1)) {
// C: 		return X_OFF(chip_addx);
// C: 	} else if (Y_RANGE_ALL(chip_addx, 1)) {
// C: 		*yram = true;
// C: 		return Y_OFF(chip_addx);
// C: 	}
// C: 
// C: 	return INVALID_CHIP_ADDRESS;
// C: }
// C: 
// C: /*
// C:  * Check if the DSP DMA is active
// C:  */
// C: static bool dsp_is_dma_active(struct hda_codec *codec, unsigned int dma_chan)
// C: {
// C: 	unsigned int dma_chnlstart_reg;
// C: 
// C: 	chipio_read(codec, DSPDMAC_CHNLSTART_INST_OFFSET, &dma_chnlstart_reg);
// C: 
// C: 	return ((dma_chnlstart_reg & (1 <<
// C: 			(DSPDMAC_CHNLSTART_EN_LOBIT + dma_chan))) != 0);
// C: }
// C: 
// C: static int dsp_dma_setup_common(struct hda_codec *codec,
// C: 				unsigned int chip_addx,
// C: 				unsigned int dma_chan,
// C: 				unsigned int port_map_mask,
// C: 				bool ovly)
// C: {
// C: 	int status = 0;
// C: 	unsigned int chnl_prop;
// C: 	unsigned int dsp_addx;
// C: 	unsigned int active;
// C: 	bool code, yram;
// C: 
// C: 	codec_dbg(codec, "-- dsp_dma_setup_common() -- Begin ---------\\n");
// C: 
// C: 	if (dma_chan >= DSPDMAC_DMA_CFG_CHANNEL_COUNT) {
// C: 		codec_dbg(codec, "dma chan num invalid\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	if (dsp_is_dma_active(codec, dma_chan)) {
// C: 		codec_dbg(codec, "dma already active\\n");
// C: 		return -EBUSY;
// C: 	}
// C: 
// C: 	dsp_addx = dsp_chip_to_dsp_addx(chip_addx, &code, &yram);
// C: 
// C: 	if (dsp_addx == INVALID_CHIP_ADDRESS) {
// C: 		codec_dbg(codec, "invalid chip addr\\n");
// C: 		return -ENXIO;
// C: 	}
// C: 
// C: 	chnl_prop = DSPDMAC_CHNLPROP_AC_MASK;
// C: 	active = 0;
// C: 
// C: 	codec_dbg(codec, "   dsp_dma_setup_common()    start reg pgm\\n");
// C: 
// C: 	if (ovly) {
// C: 		status = chipio_read(codec, DSPDMAC_CHNLPROP_INST_OFFSET,
// C: 				     &chnl_prop);
// C: 
// C: 		if (status < 0) {
// C: 			codec_dbg(codec, "read CHNLPROP Reg fail\\n");
// C: 			return status;
// C: 		}
// C: 		codec_dbg(codec, "dsp_dma_setup_common() Read CHNLPROP\\n");
// C: 	}
// C: 
// C: 	if (!code)
// C: 		chnl_prop &= ~(1 << (DSPDMAC_CHNLPROP_MSPCE_LOBIT + dma_chan));
// C: 	else
// C: 		chnl_prop |=  (1 << (DSPDMAC_CHNLPROP_MSPCE_LOBIT + dma_chan));
// C: 
// C: 	chnl_prop &= ~(1 << (DSPDMAC_CHNLPROP_DCON_LOBIT + dma_chan));
// C: 
// C: 	status = chipio_write(codec, DSPDMAC_CHNLPROP_INST_OFFSET, chnl_prop);
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "write CHNLPROP Reg fail\\n");
// C: 		return status;
// C: 	}
// C: 	codec_dbg(codec, "   dsp_dma_setup_common()    Write CHNLPROP\\n");
// C: 
// C: 	if (ovly) {
// C: 		status = chipio_read(codec, DSPDMAC_ACTIVE_INST_OFFSET,
// C: 				     &active);
// C: 
// C: 		if (status < 0) {
// C: 			codec_dbg(codec, "read ACTIVE Reg fail\\n");
// C: 			return status;
// C: 		}
// C: 		codec_dbg(codec, "dsp_dma_setup_common() Read ACTIVE\\n");
// C: 	}
// C: 
// C: 	active &= (~(1 << (DSPDMAC_ACTIVE_AAR_LOBIT + dma_chan))) &
// C: 		DSPDMAC_ACTIVE_AAR_MASK;
// C: 
// C: 	status = chipio_write(codec, DSPDMAC_ACTIVE_INST_OFFSET, active);
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "write ACTIVE Reg fail\\n");
// C: 		return status;
// C: 	}
// C: 
// C: 	codec_dbg(codec, "   dsp_dma_setup_common()    Write ACTIVE\\n");
// C: 
// C: 	status = chipio_write(codec, DSPDMAC_AUDCHSEL_INST_OFFSET(dma_chan),
// C: 			      port_map_mask);
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "write AUDCHSEL Reg fail\\n");
// C: 		return status;
// C: 	}
// C: 	codec_dbg(codec, "   dsp_dma_setup_common()    Write AUDCHSEL\\n");
// C: 
// C: 	status = chipio_write(codec, DSPDMAC_IRQCNT_INST_OFFSET(dma_chan),
// C: 			DSPDMAC_IRQCNT_BICNT_MASK | DSPDMAC_IRQCNT_CICNT_MASK);
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "write IRQCNT Reg fail\\n");
// C: 		return status;
// C: 	}
// C: 	codec_dbg(codec, "   dsp_dma_setup_common()    Write IRQCNT\\n");
// C: 
// C: 	codec_dbg(codec,
// C: 		   "ChipA=0x%x,DspA=0x%x,dmaCh=%u, "
// C: 		   "CHSEL=0x%x,CHPROP=0x%x,Active=0x%x\\n",
// C: 		   chip_addx, dsp_addx, dma_chan,
// C: 		   port_map_mask, chnl_prop, active);
// C: 
// C: 	codec_dbg(codec, "-- dsp_dma_setup_common() -- Complete ------\\n");
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * Setup the DSP DMA per-transfer-specific registers
// C:  */
// C: static int dsp_dma_setup(struct hda_codec *codec,
// C: 			unsigned int chip_addx,
// C: 			unsigned int count,
// C: 			unsigned int dma_chan)
// C: {
// C: 	int status = 0;
// C: 	bool code, yram;
// C: 	unsigned int dsp_addx;
// C: 	unsigned int addr_field;
// C: 	unsigned int incr_field;
// C: 	unsigned int base_cnt;
// C: 	unsigned int cur_cnt;
// C: 	unsigned int dma_cfg = 0;
// C: 	unsigned int adr_ofs = 0;
// C: 	unsigned int xfr_cnt = 0;
// C: 	const unsigned int max_dma_count = 1 << (DSPDMAC_XFRCNT_BCNT_HIBIT -
// C: 						DSPDMAC_XFRCNT_BCNT_LOBIT + 1);
// C: 
// C: 	codec_dbg(codec, "-- dsp_dma_setup() -- Begin ---------\\n");
// C: 
// C: 	if (count > max_dma_count) {
// C: 		codec_dbg(codec, "count too big\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	dsp_addx = dsp_chip_to_dsp_addx(chip_addx, &code, &yram);
// C: 	if (dsp_addx == INVALID_CHIP_ADDRESS) {
// C: 		codec_dbg(codec, "invalid chip addr\\n");
// C: 		return -ENXIO;
// C: 	}
// C: 
// C: 	codec_dbg(codec, "   dsp_dma_setup()    start reg pgm\\n");
// C: 
// C: 	addr_field = dsp_addx << DSPDMAC_DMACFG_DBADR_LOBIT;
// C: 	incr_field   = 0;
// C: 
// C: 	if (!code) {
// C: 		addr_field <<= 1;
// C: 		if (yram)
// C: 			addr_field |= (1 << DSPDMAC_DMACFG_DBADR_LOBIT);
// C: 
// C: 		incr_field  = (1 << DSPDMAC_DMACFG_AINCR_LOBIT);
// C: 	}
// C: 
// C: 	dma_cfg = addr_field + incr_field;
// C: 	status = chipio_write(codec, DSPDMAC_DMACFG_INST_OFFSET(dma_chan),
// C: 				dma_cfg);
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "write DMACFG Reg fail\\n");
// C: 		return status;
// C: 	}
// C: 	codec_dbg(codec, "   dsp_dma_setup()    Write DMACFG\\n");
// C: 
// C: 	adr_ofs = (count - 1) << (DSPDMAC_DSPADROFS_BOFS_LOBIT +
// C: 							(code ? 0 : 1));
// C: 
// C: 	status = chipio_write(codec, DSPDMAC_DSPADROFS_INST_OFFSET(dma_chan),
// C: 				adr_ofs);
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "write DSPADROFS Reg fail\\n");
// C: 		return status;
// C: 	}
// C: 	codec_dbg(codec, "   dsp_dma_setup()    Write DSPADROFS\\n");
// C: 
// C: 	base_cnt = (count - 1) << DSPDMAC_XFRCNT_BCNT_LOBIT;
// C: 
// C: 	cur_cnt  = (count - 1) << DSPDMAC_XFRCNT_CCNT_LOBIT;
// C: 
// C: 	xfr_cnt = base_cnt | cur_cnt;
// C: 
// C: 	status = chipio_write(codec,
// C: 				DSPDMAC_XFRCNT_INST_OFFSET(dma_chan), xfr_cnt);
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "write XFRCNT Reg fail\\n");
// C: 		return status;
// C: 	}
// C: 	codec_dbg(codec, "   dsp_dma_setup()    Write XFRCNT\\n");
// C: 
// C: 	codec_dbg(codec,
// C: 		   "ChipA=0x%x, cnt=0x%x, DMACFG=0x%x, "
// C: 		   "ADROFS=0x%x, XFRCNT=0x%x\\n",
// C: 		   chip_addx, count, dma_cfg, adr_ofs, xfr_cnt);
// C: 
// C: 	codec_dbg(codec, "-- dsp_dma_setup() -- Complete ---------\\n");
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * Start the DSP DMA
// C:  */
// C: static int dsp_dma_start(struct hda_codec *codec,
// C: 			 unsigned int dma_chan, bool ovly)
// C: {
// C: 	unsigned int reg = 0;
// C: 	int status = 0;
// C: 
// C: 	codec_dbg(codec, "-- dsp_dma_start() -- Begin ---------\\n");
// C: 
// C: 	if (ovly) {
// C: 		status = chipio_read(codec,
// C: 				     DSPDMAC_CHNLSTART_INST_OFFSET, &reg);
// C: 
// C: 		if (status < 0) {
// C: 			codec_dbg(codec, "read CHNLSTART reg fail\\n");
// C: 			return status;
// C: 		}
// C: 		codec_dbg(codec, "-- dsp_dma_start()    Read CHNLSTART\\n");
// C: 
// C: 		reg &= ~(DSPDMAC_CHNLSTART_EN_MASK |
// C: 				DSPDMAC_CHNLSTART_DIS_MASK);
// C: 	}
// C: 
// C: 	status = chipio_write(codec, DSPDMAC_CHNLSTART_INST_OFFSET,
// C: 			reg | (1 << (dma_chan + DSPDMAC_CHNLSTART_EN_LOBIT)));
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "write CHNLSTART reg fail\\n");
// C: 		return status;
// C: 	}
// C: 	codec_dbg(codec, "-- dsp_dma_start() -- Complete ---------\\n");
// C: 
// C: 	return status;
// C: }
// C: 
// C: /*
// C:  * Stop the DSP DMA
// C:  */
// C: static int dsp_dma_stop(struct hda_codec *codec,
// C: 			unsigned int dma_chan, bool ovly)
// C: {
// C: 	unsigned int reg = 0;
// C: 	int status = 0;
// C: 
// C: 	codec_dbg(codec, "-- dsp_dma_stop() -- Begin ---------\\n");
// C: 
// C: 	if (ovly) {
// C: 		status = chipio_read(codec,
// C: 				     DSPDMAC_CHNLSTART_INST_OFFSET, &reg);
// C: 
// C: 		if (status < 0) {
// C: 			codec_dbg(codec, "read CHNLSTART reg fail\\n");
// C: 			return status;
// C: 		}
// C: 		codec_dbg(codec, "-- dsp_dma_stop()    Read CHNLSTART\\n");
// C: 		reg &= ~(DSPDMAC_CHNLSTART_EN_MASK |
// C: 				DSPDMAC_CHNLSTART_DIS_MASK);
// C: 	}
// C: 
// C: 	status = chipio_write(codec, DSPDMAC_CHNLSTART_INST_OFFSET,
// C: 			reg | (1 << (dma_chan + DSPDMAC_CHNLSTART_DIS_LOBIT)));
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "write CHNLSTART reg fail\\n");
// C: 		return status;
// C: 	}
// C: 	codec_dbg(codec, "-- dsp_dma_stop() -- Complete ---------\\n");
// C: 
// C: 	return status;
// C: }
// C: 
// C: /**
// C:  * dsp_allocate_router_ports - Allocate router ports
// C:  *
// C:  * @codec: the HDA codec
// C:  * @num_chans: number of channels in the stream
// C:  * @ports_per_channel: number of ports per channel
// C:  * @start_device: start device
// C:  * @port_map: pointer to the port list to hold the allocated ports
// C:  *
// C:  * Returns zero or a negative error code.
// C:  */
// C: static int dsp_allocate_router_ports(struct hda_codec *codec,
// C: 				     unsigned int num_chans,
// C: 				     unsigned int ports_per_channel,
// C: 				     unsigned int start_device,
// C: 				     unsigned int *port_map)
// C: {
// C: 	int status = 0;
// C: 	int res;
// C: 	u8 val;
// C: 
// C: 	status = chipio_send(codec, VENDOR_CHIPIO_STATUS, 0);
// C: 	if (status < 0)
// C: 		return status;
// C: 
// C: 	val = start_device << 6;
// C: 	val |= (ports_per_channel - 1) << 4;
// C: 	val |= num_chans - 1;
// C: 
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 			    VENDOR_CHIPIO_PORT_ALLOC_CONFIG_SET,
// C: 			    val);
// C: 
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 			    VENDOR_CHIPIO_PORT_ALLOC_SET,
// C: 			    MEM_CONNID_DSP);
// C: 
// C: 	status = chipio_send(codec, VENDOR_CHIPIO_STATUS, 0);
// C: 	if (status < 0)
// C: 		return status;
// C: 
// C: 	res = snd_hda_codec_read(codec, WIDGET_CHIP_CTRL, 0,
// C: 				VENDOR_CHIPIO_PORT_ALLOC_GET, 0);
// C: 
// C: 	*port_map = res;
// C: 
// C: 	return (res < 0) ? res : 0;
// C: }
// C: 
// C: /*
// C:  * Free router ports
// C:  */
// C: static int dsp_free_router_ports(struct hda_codec *codec)
// C: {
// C: 	int status = 0;
// C: 
// C: 	status = chipio_send(codec, VENDOR_CHIPIO_STATUS, 0);
// C: 	if (status < 0)
// C: 		return status;
// C: 
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 			    VENDOR_CHIPIO_PORT_FREE_SET,
// C: 			    MEM_CONNID_DSP);
// C: 
// C: 	status = chipio_send(codec, VENDOR_CHIPIO_STATUS, 0);
// C: 
// C: 	return status;
// C: }
// C: 
// C: /*
// C:  * Allocate DSP ports for the download stream
// C:  */
// C: static int dsp_allocate_ports(struct hda_codec *codec,
// C: 			unsigned int num_chans,
// C: 			unsigned int rate_multi, unsigned int *port_map)
// C: {
// C: 	int status;
// C: 
// C: 	codec_dbg(codec, "     dsp_allocate_ports() -- begin\\n");
// C: 
// C: 	if ((rate_multi != 1) && (rate_multi != 2) && (rate_multi != 4)) {
// C: 		codec_dbg(codec, "bad rate multiple\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	status = dsp_allocate_router_ports(codec, num_chans,
// C: 					   rate_multi, 0, port_map);
// C: 
// C: 	codec_dbg(codec, "     dsp_allocate_ports() -- complete\\n");
// C: 
// C: 	return status;
// C: }
// C: 
// C: static int dsp_allocate_ports_format(struct hda_codec *codec,
// C: 			const unsigned short fmt,
// C: 			unsigned int *port_map)
// C: {
// C: 	unsigned int num_chans;
// C: 
// C: 	unsigned int sample_rate_div = ((get_hdafmt_rate(fmt) >> 0) & 3) + 1;
// C: 	unsigned int sample_rate_mul = ((get_hdafmt_rate(fmt) >> 3) & 3) + 1;
// C: 	unsigned int rate_multi = sample_rate_mul / sample_rate_div;
// C: 
// C: 	if ((rate_multi != 1) && (rate_multi != 2) && (rate_multi != 4)) {
// C: 		codec_dbg(codec, "bad rate multiple\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	num_chans = get_hdafmt_chs(fmt) + 1;
// C: 
// C: 	return dsp_allocate_ports(codec, num_chans, rate_multi, port_map);
// C: }
// C: 
// C: /*
// C:  * free DSP ports
// C:  */
// C: static int dsp_free_ports(struct hda_codec *codec)
// C: {
// C: 	int status;
// C: 
// C: 	codec_dbg(codec, "     dsp_free_ports() -- begin\\n");
// C: 
// C: 	status = dsp_free_router_ports(codec);
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "free router ports fail\\n");
// C: 		return status;
// C: 	}
// C: 	codec_dbg(codec, "     dsp_free_ports() -- complete\\n");
// C: 
// C: 	return status;
// C: }
// C: 
// C: /*
// C:  *  HDA DMA engine stuffs for DSP code download
// C:  */
// C: struct dma_engine {
// C: 	struct hda_codec *codec;
// C: 	unsigned short m_converter_format;
// C: 	struct snd_dma_buffer *dmab;
// C: 	unsigned int buf_size;
// C: };
// C: 
// C: 
// C: enum dma_state {
// C: 	DMA_STATE_STOP  = 0,
// C: 	DMA_STATE_RUN   = 1
// C: };
// C: 
// C: static int dma_convert_to_hda_format(struct hda_codec *codec,
// C: 		unsigned int sample_rate,
// C: 		unsigned short channels,
// C: 		unsigned short *hda_format)
// C: {
// C: 	unsigned int format_val;
// C: 
// C: 	format_val = snd_hdac_stream_format(channels, 32, sample_rate);
// C: 
// C: 	if (hda_format)
// C: 		*hda_format = (unsigned short)format_val;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  *  Reset DMA for DSP download
// C:  */
// C: static int dma_reset(struct dma_engine *dma)
// C: {
// C: 	struct hda_codec *codec = dma->codec;
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int status;
// C: 
// C: 	if (dma->dmab->area)
// C: 		snd_hda_codec_load_dsp_cleanup(codec, dma->dmab);
// C: 
// C: 	status = snd_hda_codec_load_dsp_prepare(codec,
// C: 			dma->m_converter_format,
// C: 			dma->buf_size,
// C: 			dma->dmab);
// C: 	if (status < 0)
// C: 		return status;
// C: 	spec->dsp_stream_id = status;
// C: 	return 0;
// C: }
// C: 
// C: static int dma_set_state(struct dma_engine *dma, enum dma_state state)
// C: {
// C: 	bool cmd;
// C: 
// C: 	switch (state) {
// C: 	case DMA_STATE_STOP:
// C: 		cmd = false;
// C: 		break;
// C: 	case DMA_STATE_RUN:
// C: 		cmd = true;
// C: 		break;
// C: 	default:
// C: 		return 0;
// C: 	}
// C: 
// C: 	snd_hda_codec_load_dsp_trigger(dma->codec, cmd);
// C: 	return 0;
// C: }
// C: 
// C: static unsigned int dma_get_buffer_size(struct dma_engine *dma)
// C: {
// C: 	return dma->dmab->bytes;
// C: }
// C: 
// C: static unsigned char *dma_get_buffer_addr(struct dma_engine *dma)
// C: {
// C: 	return dma->dmab->area;
// C: }
// C: 
// C: static int dma_xfer(struct dma_engine *dma,
// C: 		const unsigned int *data,
// C: 		unsigned int count)
// C: {
// C: 	memcpy(dma->dmab->area, data, count);
// C: 	return 0;
// C: }
// C: 
// C: static void dma_get_converter_format(
// C: 		struct dma_engine *dma,
// C: 		unsigned short *format)
// C: {
// C: 	if (format)
// C: 		*format = dma->m_converter_format;
// C: }
// C: 
// C: static unsigned int dma_get_stream_id(struct dma_engine *dma)
// C: {
// C: 	struct ca0132_spec *spec = dma->codec->spec;
// C: 
// C: 	return spec->dsp_stream_id;
// C: }
// C: 
// C: struct dsp_image_seg {
// C: 	u32 magic;
// C: 	u32 chip_addr;
// C: 	u32 count;
// C: 	u32 data[];
// C: };
// C: 
// C: static const u32 g_magic_value = 0x4c46584d;
// C: static const u32 g_chip_addr_magic_value = 0xFFFFFF01;
// C: 
// C: static bool is_valid(const struct dsp_image_seg *p)
// C: {
// C: 	return p->magic == g_magic_value;
// C: }
// C: 
// C: static bool is_hci_prog_list_seg(const struct dsp_image_seg *p)
// C: {
// C: 	return g_chip_addr_magic_value == p->chip_addr;
// C: }
// C: 
// C: static bool is_last(const struct dsp_image_seg *p)
// C: {
// C: 	return p->count == 0;
// C: }
// C: 
// C: static size_t dsp_sizeof(const struct dsp_image_seg *p)
// C: {
// C: 	return struct_size(p, data, p->count);
// C: }
// C: 
// C: static const struct dsp_image_seg *get_next_seg_ptr(
// C: 				const struct dsp_image_seg *p)
// C: {
// C: 	return (struct dsp_image_seg *)((unsigned char *)(p) + dsp_sizeof(p));
// C: }
// C: 
// C: /*
// C:  * CA0132 chip DSP transfer stuffs.  For DSP download.
// C:  */
// C: #define INVALID_DMA_CHANNEL (~0U)
// C: 
// C: /*
// C:  * Program a list of address/data pairs via the ChipIO widget.
// C:  * The segment data is in the format of successive pairs of words.
// C:  * These are repeated as indicated by the segment's count field.
// C:  */
// C: static int dspxfr_hci_write(struct hda_codec *codec,
// C: 			const struct dsp_image_seg *fls)
// C: {
// C: 	int status;
// C: 	const u32 *data;
// C: 	unsigned int count;
// C: 
// C: 	if (fls == NULL || fls->chip_addr != g_chip_addr_magic_value) {
// C: 		codec_dbg(codec, "hci_write invalid params\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	count = fls->count;
// C: 	data = (u32 *)(fls->data);
// C: 	while (count >= 2) {
// C: 		status = chipio_write(codec, data[0], data[1]);
// C: 		if (status < 0) {
// C: 			codec_dbg(codec, "hci_write chipio failed\\n");
// C: 			return status;
// C: 		}
// C: 		count -= 2;
// C: 		data  += 2;
// C: 	}
// C: 	return 0;
// C: }
// C: 
// C: /**
// C:  * dspxfr_one_seg - Write a block of data into DSP code or data RAM using pre-allocated DMA engine.
// C:  *
// C:  * @codec: the HDA codec
// C:  * @fls: pointer to a fast load image
// C:  * @reloc: Relocation address for loading single-segment overlays, or 0 for
// C:  *	   no relocation
// C:  * @dma_engine: pointer to DMA engine to be used for DSP download
// C:  * @dma_chan: The number of DMA channels used for DSP download
// C:  * @port_map_mask: port mapping
// C:  * @ovly: TRUE if overlay format is required
// C:  *
// C:  * Returns zero or a negative error code.
// C:  */
// C: static int dspxfr_one_seg(struct hda_codec *codec,
// C: 			const struct dsp_image_seg *fls,
// C: 			unsigned int reloc,
// C: 			struct dma_engine *dma_engine,
// C: 			unsigned int dma_chan,
// C: 			unsigned int port_map_mask,
// C: 			bool ovly)
// C: {
// C: 	int status = 0;
// C: 	bool comm_dma_setup_done = false;
// C: 	const unsigned int *data;
// C: 	unsigned int chip_addx;
// C: 	unsigned int words_to_write;
// C: 	unsigned int buffer_size_words;
// C: 	unsigned char *buffer_addx;
// C: 	unsigned short hda_format;
// C: 	unsigned int sample_rate_div;
// C: 	unsigned int sample_rate_mul;
// C: 	unsigned int num_chans;
// C: 	unsigned int hda_frame_size_words;
// C: 	unsigned int remainder_words;
// C: 	const u32 *data_remainder;
// C: 	u32 chip_addx_remainder;
// C: 	unsigned int run_size_words;
// C: 	const struct dsp_image_seg *hci_write = NULL;
// C: 	unsigned long timeout;
// C: 	bool dma_active;
// C: 
// C: 	if (fls == NULL)
// C: 		return -EINVAL;
// C: 	if (is_hci_prog_list_seg(fls)) {
// C: 		hci_write = fls;
// C: 		fls = get_next_seg_ptr(fls);
// C: 	}
// C: 
// C: 	if (hci_write && (!fls || is_last(fls))) {
// C: 		codec_dbg(codec, "hci_write\\n");
// C: 		return dspxfr_hci_write(codec, hci_write);
// C: 	}
// C: 
// C: 	if (fls == NULL || dma_engine == NULL || port_map_mask == 0) {
// C: 		codec_dbg(codec, "Invalid Params\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	data = fls->data;
// C: 	chip_addx = fls->chip_addr;
// C: 	words_to_write = fls->count;
// C: 
// C: 	if (!words_to_write)
// C: 		return hci_write ? dspxfr_hci_write(codec, hci_write) : 0;
// C: 	if (reloc)
// C: 		chip_addx = (chip_addx & (0xFFFF0000 << 2)) + (reloc << 2);
// C: 
// C: 	if (!UC_RANGE(chip_addx, words_to_write) &&
// C: 	    !X_RANGE_ALL(chip_addx, words_to_write) &&
// C: 	    !Y_RANGE_ALL(chip_addx, words_to_write)) {
// C: 		codec_dbg(codec, "Invalid chip_addx Params\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	buffer_size_words = (unsigned int)dma_get_buffer_size(dma_engine) /
// C: 					sizeof(u32);
// C: 
// C: 	buffer_addx = dma_get_buffer_addr(dma_engine);
// C: 
// C: 	if (buffer_addx == NULL) {
// C: 		codec_dbg(codec, "dma_engine buffer NULL\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	dma_get_converter_format(dma_engine, &hda_format);
// C: 	sample_rate_div = ((get_hdafmt_rate(hda_format) >> 0) & 3) + 1;
// C: 	sample_rate_mul = ((get_hdafmt_rate(hda_format) >> 3) & 3) + 1;
// C: 	num_chans = get_hdafmt_chs(hda_format) + 1;
// C: 
// C: 	hda_frame_size_words = ((sample_rate_div == 0) ? 0 :
// C: 			(num_chans * sample_rate_mul / sample_rate_div));
// C: 
// C: 	if (hda_frame_size_words == 0) {
// C: 		codec_dbg(codec, "frmsz zero\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	buffer_size_words = min(buffer_size_words,
// C: 				(unsigned int)(UC_RANGE(chip_addx, 1) ?
// C: 				65536 : 32768));
// C: 	buffer_size_words -= buffer_size_words % hda_frame_size_words;
// C: 	codec_dbg(codec,
// C: 		   "chpadr=0x%08x frmsz=%u nchan=%u "
// C: 		   "rate_mul=%u div=%u bufsz=%u\\n",
// C: 		   chip_addx, hda_frame_size_words, num_chans,
// C: 		   sample_rate_mul, sample_rate_div, buffer_size_words);
// C: 
// C: 	if (buffer_size_words < hda_frame_size_words) {
// C: 		codec_dbg(codec, "dspxfr_one_seg:failed\\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	remainder_words = words_to_write % hda_frame_size_words;
// C: 	data_remainder = data;
// C: 	chip_addx_remainder = chip_addx;
// C: 
// C: 	data += remainder_words;
// C: 	chip_addx += remainder_words*sizeof(u32);
// C: 	words_to_write -= remainder_words;
// C: 
// C: 	while (words_to_write != 0) {
// C: 		run_size_words = min(buffer_size_words, words_to_write);
// C: 		codec_dbg(codec, "dspxfr (seg loop)cnt=%u rs=%u remainder=%u\\n",
// C: 			    words_to_write, run_size_words, remainder_words);
// C: 		dma_xfer(dma_engine, data, run_size_words*sizeof(u32));
// C: 		if (!comm_dma_setup_done) {
// C: 			status = dsp_dma_stop(codec, dma_chan, ovly);
// C: 			if (status < 0)
// C: 				return status;
// C: 			status = dsp_dma_setup_common(codec, chip_addx,
// C: 						dma_chan, port_map_mask, ovly);
// C: 			if (status < 0)
// C: 				return status;
// C: 			comm_dma_setup_done = true;
// C: 		}
// C: 
// C: 		status = dsp_dma_setup(codec, chip_addx,
// C: 						run_size_words, dma_chan);
// C: 		if (status < 0)
// C: 			return status;
// C: 		status = dsp_dma_start(codec, dma_chan, ovly);
// C: 		if (status < 0)
// C: 			return status;
// C: 		if (!dsp_is_dma_active(codec, dma_chan)) {
// C: 			codec_dbg(codec, "dspxfr:DMA did not start\\n");
// C: 			return -EIO;
// C: 		}
// C: 		status = dma_set_state(dma_engine, DMA_STATE_RUN);
// C: 		if (status < 0)
// C: 			return status;
// C: 		if (remainder_words != 0) {
// C: 			status = chipio_write_multiple(codec,
// C: 						chip_addx_remainder,
// C: 						data_remainder,
// C: 						remainder_words);
// C: 			if (status < 0)
// C: 				return status;
// C: 			remainder_words = 0;
// C: 		}
// C: 		if (hci_write) {
// C: 			status = dspxfr_hci_write(codec, hci_write);
// C: 			if (status < 0)
// C: 				return status;
// C: 			hci_write = NULL;
// C: 		}
// C: 
// C: 		timeout = jiffies + msecs_to_jiffies(2000);
// C: 		do {
// C: 			dma_active = dsp_is_dma_active(codec, dma_chan);
// C: 			if (!dma_active)
// C: 				break;
// C: 			msleep(20);
// C: 		} while (time_before(jiffies, timeout));
// C: 		if (dma_active)
// C: 			break;
// C: 
// C: 		codec_dbg(codec, "+++++ DMA complete\\n");
// C: 		dma_set_state(dma_engine, DMA_STATE_STOP);
// C: 		status = dma_reset(dma_engine);
// C: 
// C: 		if (status < 0)
// C: 			return status;
// C: 
// C: 		data += run_size_words;
// C: 		chip_addx += run_size_words*sizeof(u32);
// C: 		words_to_write -= run_size_words;
// C: 	}
// C: 
// C: 	if (remainder_words != 0) {
// C: 		status = chipio_write_multiple(codec, chip_addx_remainder,
// C: 					data_remainder, remainder_words);
// C: 	}
// C: 
// C: 	return status;
// C: }
// C: 
// C: /**
// C:  * dspxfr_image - Write the entire DSP image of a DSP code/data overlay to DSP memories
// C:  *
// C:  * @codec: the HDA codec
// C:  * @fls_data: pointer to a fast load image
// C:  * @reloc: Relocation address for loading single-segment overlays, or 0 for
// C:  *	   no relocation
// C:  * @sample_rate: sampling rate of the stream used for DSP download
// C:  * @channels: channels of the stream used for DSP download
// C:  * @ovly: TRUE if overlay format is required
// C:  *
// C:  * Returns zero or a negative error code.
// C:  */
// C: static int dspxfr_image(struct hda_codec *codec,
// C: 			const struct dsp_image_seg *fls_data,
// C: 			unsigned int reloc,
// C: 			unsigned int sample_rate,
// C: 			unsigned short channels,
// C: 			bool ovly)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int status;
// C: 	unsigned short hda_format = 0;
// C: 	unsigned int response;
// C: 	unsigned char stream_id = 0;
// C: 	struct dma_engine *dma_engine;
// C: 	unsigned int dma_chan;
// C: 	unsigned int port_map_mask;
// C: 
// C: 	if (fls_data == NULL)
// C: 		return -EINVAL;
// C: 
// C: 	dma_engine = kzalloc_obj(*dma_engine);
// C: 	if (!dma_engine)
// C: 		return -ENOMEM;
// C: 
// C: 	dma_engine->dmab = kzalloc_obj(*dma_engine->dmab);
// C: 	if (!dma_engine->dmab) {
// C: 		kfree(dma_engine);
// C: 		return -ENOMEM;
// C: 	}
// C: 
// C: 	dma_engine->codec = codec;
// C: 	dma_convert_to_hda_format(codec, sample_rate, channels, &hda_format);
// C: 	dma_engine->m_converter_format = hda_format;
// C: 	dma_engine->buf_size = (ovly ? DSP_DMA_WRITE_BUFLEN_OVLY :
// C: 			DSP_DMA_WRITE_BUFLEN_INIT) * 2;
// C: 
// C: 	dma_chan = ovly ? INVALID_DMA_CHANNEL : 0;
// C: 
// C: 	status = codec_set_converter_format(codec, WIDGET_CHIP_CTRL,
// C: 					hda_format, &response);
// C: 
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "set converter format fail\\n");
// C: 		goto exit;
// C: 	}
// C: 
// C: 	status = snd_hda_codec_load_dsp_prepare(codec,
// C: 				dma_engine->m_converter_format,
// C: 				dma_engine->buf_size,
// C: 				dma_engine->dmab);
// C: 	if (status < 0)
// C: 		goto exit;
// C: 	spec->dsp_stream_id = status;
// C: 
// C: 	if (ovly) {
// C: 		status = dspio_alloc_dma_chan(codec, &dma_chan);
// C: 		if (status < 0) {
// C: 			codec_dbg(codec, "alloc dmachan fail\\n");
// C: 			dma_chan = INVALID_DMA_CHANNEL;
// C: 			goto exit;
// C: 		}
// C: 	}
// C: 
// C: 	port_map_mask = 0;
// C: 	status = dsp_allocate_ports_format(codec, hda_format,
// C: 					&port_map_mask);
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "alloc ports fail\\n");
// C: 		goto exit;
// C: 	}
// C: 
// C: 	stream_id = dma_get_stream_id(dma_engine);
// C: 	status = codec_set_converter_stream_channel(codec,
// C: 			WIDGET_CHIP_CTRL, stream_id, 0, &response);
// C: 	if (status < 0) {
// C: 		codec_dbg(codec, "set stream chan fail\\n");
// C: 		goto exit;
// C: 	}
// C: 
// C: 	while ((fls_data != NULL) && !is_last(fls_data)) {
// C: 		if (!is_valid(fls_data)) {
// C: 			codec_dbg(codec, "FLS check fail\\n");
// C: 			status = -EINVAL;
// C: 			goto exit;
// C: 		}
// C: 		status = dspxfr_one_seg(codec, fls_data, reloc,
// C: 					dma_engine, dma_chan,
// C: 					port_map_mask, ovly);
// C: 		if (status < 0)
// C: 			break;
// C: 
// C: 		if (is_hci_prog_list_seg(fls_data))
// C: 			fls_data = get_next_seg_ptr(fls_data);
// C: 
// C: 		if ((fls_data != NULL) && !is_last(fls_data))
// C: 			fls_data = get_next_seg_ptr(fls_data);
// C: 	}
// C: 
// C: 	if (port_map_mask != 0)
// C: 		status = dsp_free_ports(codec);
// C: 
// C: 	if (status < 0)
// C: 		goto exit;
// C: 
// C: 	status = codec_set_converter_stream_channel(codec,
// C: 				WIDGET_CHIP_CTRL, 0, 0, &response);
// C: 
// C: exit:
// C: 	if (ovly && (dma_chan != INVALID_DMA_CHANNEL))
// C: 		dspio_free_dma_chan(codec, dma_chan);
// C: 
// C: 	if (dma_engine->dmab->area)
// C: 		snd_hda_codec_load_dsp_cleanup(codec, dma_engine->dmab);
// C: 	kfree(dma_engine->dmab);
// C: 	kfree(dma_engine);
// C: 
// C: 	return status;
// C: }
// C: 
// C: /*
// C:  * CA0132 DSP download stuffs.
// C:  */
// C: static void dspload_post_setup(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	codec_dbg(codec, "---- dspload_post_setup ------\\n");
// C: 	if (!ca0132_use_alt_functions(spec)) {
// C: 		/*set DSP speaker to 2.0 configuration*/
// C: 		chipio_write(codec, XRAM_XRAM_INST_OFFSET(0x18), 0x08080080);
// C: 		chipio_write(codec, XRAM_XRAM_INST_OFFSET(0x19), 0x3f800000);
// C: 
// C: 		/*update write pointer*/
// C: 		chipio_write(codec, XRAM_XRAM_INST_OFFSET(0x29), 0x00000002);
// C: 	}
// C: }
// C: 
// C: /**
// C:  * dspload_image - Download DSP from a DSP Image Fast Load structure.
// C:  *
// C:  * @codec: the HDA codec
// C:  * @fls: pointer to a fast load image
// C:  * @ovly: TRUE if overlay format is required
// C:  * @reloc: Relocation address for loading single-segment overlays, or 0 for
// C:  *	   no relocation
// C:  * @autostart: TRUE if DSP starts after loading; ignored if ovly is TRUE
// C:  * @router_chans: number of audio router channels to be allocated (0 means use
// C:  *		  internal defaults; max is 32)
// C:  *
// C:  * Download DSP from a DSP Image Fast Load structure. This structure is a
// C:  * linear, non-constant sized element array of structures, each of which
// C:  * contain the count of the data to be loaded, the data itself, and the
// C:  * corresponding starting chip address of the starting data location.
// C:  * Returns zero or a negative error code.
// C:  */
// C: static int dspload_image(struct hda_codec *codec,
// C: 			const struct dsp_image_seg *fls,
// C: 			bool ovly,
// C: 			unsigned int reloc,
// C: 			bool autostart,
// C: 			int router_chans)
// C: {
// C: 	int status = 0;
// C: 	unsigned int sample_rate;
// C: 	unsigned short channels;
// C: 
// C: 	codec_dbg(codec, "---- dspload_image begin ------\\n");
// C: 	if (router_chans == 0) {
// C: 		if (!ovly)
// C: 			router_chans = DMA_TRANSFER_FRAME_SIZE_NWORDS;
// C: 		else
// C: 			router_chans = DMA_OVERLAY_FRAME_SIZE_NWORDS;
// C: 	}
// C: 
// C: 	sample_rate = 48000;
// C: 	channels = (unsigned short)router_chans;
// C: 
// C: 	while (channels > 16) {
// C: 		sample_rate *= 2;
// C: 		channels /= 2;
// C: 	}
// C: 
// C: 	do {
// C: 		codec_dbg(codec, "Ready to program DMA\\n");
// C: 		if (!ovly)
// C: 			status = dsp_reset(codec);
// C: 
// C: 		if (status < 0)
// C: 			break;
// C: 
// C: 		codec_dbg(codec, "dsp_reset() complete\\n");
// C: 		status = dspxfr_image(codec, fls, reloc, sample_rate, channels,
// C: 				      ovly);
// C: 
// C: 		if (status < 0)
// C: 			break;
// C: 
// C: 		codec_dbg(codec, "dspxfr_image() complete\\n");
// C: 		if (autostart && !ovly) {
// C: 			dspload_post_setup(codec);
// C: 			status = dsp_set_run_state(codec);
// C: 		}
// C: 
// C: 		codec_dbg(codec, "LOAD FINISHED\\n");
// C: 	} while (0);
// C: 
// C: 	return status;
// C: }
// C: 
// C: #ifdef CONFIG_SND_HDA_CODEC_CA0132_DSP
// C: static bool dspload_is_loaded(struct hda_codec *codec)
// C: {
// C: 	unsigned int data = 0;
// C: 	int status = 0;
// C: 
// C: 	status = chipio_read(codec, 0x40004, &data);
// C: 	if ((status < 0) || (data != 1))
// C: 		return false;
// C: 
// C: 	return true;
// C: }
// C: #else
// C: #define dspload_is_loaded(codec)	false
// C: #endif
// C: 
// C: static bool dspload_wait_loaded(struct hda_codec *codec)
// C: {
// C: 	unsigned long timeout = jiffies + msecs_to_jiffies(2000);
// C: 
// C: 	do {
// C: 		if (dspload_is_loaded(codec)) {
// C: 			codec_info(codec, "ca0132 DSP downloaded and running\\n");
// C: 			return true;
// C: 		}
// C: 		msleep(20);
// C: 	} while (time_before(jiffies, timeout));
// C: 
// C: 	codec_err(codec, "ca0132 failed to download DSP\\n");
// C: 	return false;
// C: }
// C: 
// C: /*
// C:  * ca0113 related functions. The ca0113 acts as the HDA bus for the pci-e
// C:  * based cards, and has a second mmio region, region2, that's used for special
// C:  * commands.
// C:  */
// C: 
// C: /*
// C:  * For cards with PCI-E region2 (Sound Blaster Z/ZxR, Recon3D, and AE-5)
// C:  * the mmio address 0x320 is used to set GPIO pins. The format for the data
// C:  * The first eight bits are just the number of the pin. So far, I've only seen
// C:  * this number go to 7.
// C:  * AE-5 note: The AE-5 seems to use pins 2 and 3 to somehow set the color value
// C:  * of the on-card LED. It seems to use pin 2 for data, then toggles 3 to on and
// C:  * then off to send that bit.
// C:  */
// C: static void ca0113_mmio_gpio_set(struct hda_codec *codec, unsigned int gpio_pin,
// C: 		bool enable)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned short gpio_data;
// C: 
// C: 	gpio_data = gpio_pin & 0xF;
// C: 	gpio_data |= ((enable << 8) & 0x100);
// C: 
// C: 	writew(gpio_data, spec->mem_base + 0x320);
// C: }
// C: 
// C: /*
// C:  * Special pci region2 commands that are only used by the AE-5. They follow
// C:  * a set format, and require reads at certain points to seemingly 'clear'
// C:  * the response data. My first tests didn't do these reads, and would cause
// C:  * the card to get locked up until the memory was read. These commands
// C:  * seem to work with three distinct values that I've taken to calling group,
// C:  * target-id, and value.
// C:  */
// C: static void ca0113_mmio_command_set(struct hda_codec *codec, unsigned int group,
// C: 		unsigned int target, unsigned int value)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int write_val;
// C: 
// C: 	writel(0x0000007e, spec->mem_base + 0x210);
// C: 	readl(spec->mem_base + 0x210);
// C: 	writel(0x0000005a, spec->mem_base + 0x210);
// C: 	readl(spec->mem_base + 0x210);
// C: 	readl(spec->mem_base + 0x210);
// C: 
// C: 	writel(0x00800005, spec->mem_base + 0x20c);
// C: 	writel(group, spec->mem_base + 0x804);
// C: 
// C: 	writel(0x00800005, spec->mem_base + 0x20c);
// C: 	write_val = (target & 0xff);
// C: 	write_val |= (value << 8);
// C: 
// C: 
// C: 	writel(write_val, spec->mem_base + 0x204);
// C: 	/*
// C: 	 * Need delay here or else it goes too fast and works inconsistently.
// C: 	 */
// C: 	msleep(20);
// C: 
// C: 	readl(spec->mem_base + 0x860);
// C: 	readl(spec->mem_base + 0x854);
// C: 	readl(spec->mem_base + 0x840);
// C: 
// C: 	writel(0x00800004, spec->mem_base + 0x20c);
// C: 	writel(0x00000000, spec->mem_base + 0x210);
// C: 	readl(spec->mem_base + 0x210);
// C: 	readl(spec->mem_base + 0x210);
// C: }
// C: 
// C: /*
// C:  * This second type of command is used for setting the sound filter type.
// C:  */
// C: static void ca0113_mmio_command_set_type2(struct hda_codec *codec,
// C: 		unsigned int group, unsigned int target, unsigned int value)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int write_val;
// C: 
// C: 	writel(0x0000007e, spec->mem_base + 0x210);
// C: 	readl(spec->mem_base + 0x210);
// C: 	writel(0x0000005a, spec->mem_base + 0x210);
// C: 	readl(spec->mem_base + 0x210);
// C: 	readl(spec->mem_base + 0x210);
// C: 
// C: 	writel(0x00800003, spec->mem_base + 0x20c);
// C: 	writel(group, spec->mem_base + 0x804);
// C: 
// C: 	writel(0x00800005, spec->mem_base + 0x20c);
// C: 	write_val = (target & 0xff);
// C: 	write_val |= (value << 8);
// C: 
// C: 
// C: 	writel(write_val, spec->mem_base + 0x204);
// C: 	msleep(20);
// C: 	readl(spec->mem_base + 0x860);
// C: 	readl(spec->mem_base + 0x854);
// C: 	readl(spec->mem_base + 0x840);
// C: 
// C: 	writel(0x00800004, spec->mem_base + 0x20c);
// C: 	writel(0x00000000, spec->mem_base + 0x210);
// C: 	readl(spec->mem_base + 0x210);
// C: 	readl(spec->mem_base + 0x210);
// C: }
// C: 
// C: /*
// C:  * Setup GPIO for the other variants of Core3D.
// C:  */
// C: 
// C: /*
// C:  * Sets up the GPIO pins so that they are discoverable. If this isn't done,
// C:  * the card shows as having no GPIO pins.
// C:  */
// C: static void ca0132_gpio_init(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_SBZ:
// C: 	case QUIRK_AE5:
// C: 	case QUIRK_AE7:
// C: 		snd_hda_codec_write(codec, 0x01, 0, 0x793, 0x00);
// C: 		snd_hda_codec_write(codec, 0x01, 0, 0x794, 0x53);
// C: 		snd_hda_codec_write(codec, 0x01, 0, 0x790, 0x23);
// C: 		break;
// C: 	case QUIRK_R3DI:
// C: 		snd_hda_codec_write(codec, 0x01, 0, 0x793, 0x00);
// C: 		snd_hda_codec_write(codec, 0x01, 0, 0x794, 0x5B);
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: 
// C: }
// C: 
// C: /* Sets the GPIO for audio output. */
// C: static void ca0132_gpio_setup(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_SBZ:
// C: 		snd_hda_codec_set_gpio(codec, 0x07, 0x07, 0x04, 0);
// C: 		snd_hda_codec_write(codec, 0x01, 0,
// C: 				AC_VERB_SET_GPIO_DATA, 0x06);
// C: 		break;
// C: 	case QUIRK_R3DI:
// C: 		snd_hda_codec_set_gpio(codec, 0x1F, 0x1E, 0x0C, 0);
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: }
// C: 
// C: /*
// C:  * GPIO control functions for the Recon3D integrated.
// C:  */
// C: 
// C: enum r3di_gpio_bit {
// C: 	/* Bit 1 - Switch between front/rear mic. 0 = rear, 1 = front */
// C: 	R3DI_MIC_SELECT_BIT = 1,
// C: 	/* Bit 2 - Switch between headphone/line out. 0 = Headphone, 1 = Line */
// C: 	R3DI_OUT_SELECT_BIT = 2,
// C: 	/*
// C: 	 * I dunno what this actually does, but it stays on until the dsp
// C: 	 * is downloaded.
// C: 	 */
// C: 	R3DI_GPIO_DSP_DOWNLOADING = 3,
// C: 	/*
// C: 	 * Same as above, no clue what it does, but it comes on after the dsp
// C: 	 * is downloaded.
// C: 	 */
// C: 	R3DI_GPIO_DSP_DOWNLOADED = 4
// C: };
// C: 
// C: enum r3di_mic_select {
// C: 	/* Set GPIO bit 1 to 0 for rear mic */
// C: 	R3DI_REAR_MIC = 0,
// C: 	/* Set GPIO bit 1 to 1 for front microphone*/
// C: 	R3DI_FRONT_MIC = 1
// C: };
// C: 
// C: enum r3di_out_select {
// C: 	/* Set GPIO bit 2 to 0 for headphone */
// C: 	R3DI_HEADPHONE_OUT = 0,
// C: 	/* Set GPIO bit 2 to 1 for speaker */
// C: 	R3DI_LINE_OUT = 1
// C: };
// C: enum r3di_dsp_status {
// C: 	/* Set GPIO bit 3 to 1 until DSP is downloaded */
// C: 	R3DI_DSP_DOWNLOADING = 0,
// C: 	/* Set GPIO bit 4 to 1 once DSP is downloaded */
// C: 	R3DI_DSP_DOWNLOADED = 1
// C: };
// C: 
// C: 
// C: static void r3di_gpio_mic_set(struct hda_codec *codec,
// C: 		enum r3di_mic_select cur_mic)
// C: {
// C: 	unsigned int cur_gpio;
// C: 
// C: 	/* Get the current GPIO Data setup */
// C: 	cur_gpio = snd_hda_codec_read(codec, 0x01, 0, AC_VERB_GET_GPIO_DATA, 0);
// C: 
// C: 	switch (cur_mic) {
// C: 	case R3DI_REAR_MIC:
// C: 		cur_gpio &= ~(1 << R3DI_MIC_SELECT_BIT);
// C: 		break;
// C: 	case R3DI_FRONT_MIC:
// C: 		cur_gpio |= (1 << R3DI_MIC_SELECT_BIT);
// C: 		break;
// C: 	}
// C: 	snd_hda_codec_write(codec, codec->core.afg, 0,
// C: 			    AC_VERB_SET_GPIO_DATA, cur_gpio);
// C: }
// C: 
// C: static void r3di_gpio_dsp_status_set(struct hda_codec *codec,
// C: 		enum r3di_dsp_status dsp_status)
// C: {
// C: 	unsigned int cur_gpio;
// C: 
// C: 	/* Get the current GPIO Data setup */
// C: 	cur_gpio = snd_hda_codec_read(codec, 0x01, 0, AC_VERB_GET_GPIO_DATA, 0);
// C: 
// C: 	switch (dsp_status) {
// C: 	case R3DI_DSP_DOWNLOADING:
// C: 		cur_gpio |= (1 << R3DI_GPIO_DSP_DOWNLOADING);
// C: 		snd_hda_codec_write(codec, codec->core.afg, 0,
// C: 				AC_VERB_SET_GPIO_DATA, cur_gpio);
// C: 		break;
// C: 	case R3DI_DSP_DOWNLOADED:
// C: 		/* Set DOWNLOADING bit to 0. */
// C: 		cur_gpio &= ~(1 << R3DI_GPIO_DSP_DOWNLOADING);
// C: 
// C: 		snd_hda_codec_write(codec, codec->core.afg, 0,
// C: 				AC_VERB_SET_GPIO_DATA, cur_gpio);
// C: 
// C: 		cur_gpio |= (1 << R3DI_GPIO_DSP_DOWNLOADED);
// C: 		break;
// C: 	}
// C: 
// C: 	snd_hda_codec_write(codec, codec->core.afg, 0,
// C: 			    AC_VERB_SET_GPIO_DATA, cur_gpio);
// C: }
// C: 
// C: /*
// C:  * PCM callbacks
// C:  */
// C: static int ca0132_playback_pcm_prepare(struct hda_pcm_stream *hinfo,
// C: 			struct hda_codec *codec,
// C: 			unsigned int stream_tag,
// C: 			unsigned int format,
// C: 			struct snd_pcm_substream *substream)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	snd_hda_codec_setup_stream(codec, spec->dacs[0], stream_tag, 0, format);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_playback_pcm_cleanup(struct hda_pcm_stream *hinfo,
// C: 			struct hda_codec *codec,
// C: 			struct snd_pcm_substream *substream)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	if (spec->dsp_state == DSP_DOWNLOADING)
// C: 		return 0;
// C: 
// C: 	/*If Playback effects are on, allow stream some time to flush
// C: 	 *effects tail*/
// C: 	if (spec->effects_switch[PLAY_ENHANCEMENT - EFFECT_START_NID])
// C: 		msleep(50);
// C: 
// C: 	snd_hda_codec_cleanup_stream(codec, spec->dacs[0]);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static unsigned int ca0132_playback_pcm_delay(struct hda_pcm_stream *info,
// C: 			struct hda_codec *codec,
// C: 			struct snd_pcm_substream *substream)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int latency = DSP_PLAYBACK_INIT_LATENCY;
// C: 	struct snd_pcm_runtime *runtime = substream->runtime;
// C: 
// C: 	if (spec->dsp_state != DSP_DOWNLOADED)
// C: 		return 0;
// C: 
// C: 	/* Add latency if playback enhancement and either effect is enabled. */
// C: 	if (spec->effects_switch[PLAY_ENHANCEMENT - EFFECT_START_NID]) {
// C: 		if ((spec->effects_switch[SURROUND - EFFECT_START_NID]) ||
// C: 		    (spec->effects_switch[DIALOG_PLUS - EFFECT_START_NID]))
// C: 			latency += DSP_PLAY_ENHANCEMENT_LATENCY;
// C: 	}
// C: 
// C: 	/* Applying Speaker EQ adds latency as well. */
// C: 	if (spec->cur_out_type == SPEAKER_OUT)
// C: 		latency += DSP_SPEAKER_OUT_LATENCY;
// C: 
// C: 	return (latency * runtime->rate) / 1000;
// C: }
// C: 
// C: /*
// C:  * Digital out
// C:  */
// C: static int ca0132_dig_playback_pcm_open(struct hda_pcm_stream *hinfo,
// C: 					struct hda_codec *codec,
// C: 					struct snd_pcm_substream *substream)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	return snd_hda_multi_out_dig_open(codec, &spec->multiout);
// C: }
// C: 
// C: static int ca0132_dig_playback_pcm_prepare(struct hda_pcm_stream *hinfo,
// C: 			struct hda_codec *codec,
// C: 			unsigned int stream_tag,
// C: 			unsigned int format,
// C: 			struct snd_pcm_substream *substream)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	return snd_hda_multi_out_dig_prepare(codec, &spec->multiout,
// C: 					     stream_tag, format, substream);
// C: }
// C: 
// C: static int ca0132_dig_playback_pcm_cleanup(struct hda_pcm_stream *hinfo,
// C: 			struct hda_codec *codec,
// C: 			struct snd_pcm_substream *substream)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	return snd_hda_multi_out_dig_cleanup(codec, &spec->multiout);
// C: }
// C: 
// C: static int ca0132_dig_playback_pcm_close(struct hda_pcm_stream *hinfo,
// C: 					 struct hda_codec *codec,
// C: 					 struct snd_pcm_substream *substream)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	return snd_hda_multi_out_dig_close(codec, &spec->multiout);
// C: }
// C: 
// C: /*
// C:  * Analog capture
// C:  */
// C: static int ca0132_capture_pcm_prepare(struct hda_pcm_stream *hinfo,
// C: 					struct hda_codec *codec,
// C: 					unsigned int stream_tag,
// C: 					unsigned int format,
// C: 					struct snd_pcm_substream *substream)
// C: {
// C: 	snd_hda_codec_setup_stream(codec, hinfo->nid,
// C: 				   stream_tag, 0, format);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_capture_pcm_cleanup(struct hda_pcm_stream *hinfo,
// C: 			struct hda_codec *codec,
// C: 			struct snd_pcm_substream *substream)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	if (spec->dsp_state == DSP_DOWNLOADING)
// C: 		return 0;
// C: 
// C: 	snd_hda_codec_cleanup_stream(codec, hinfo->nid);
// C: 	return 0;
// C: }
// C: 
// C: static unsigned int ca0132_capture_pcm_delay(struct hda_pcm_stream *info,
// C: 			struct hda_codec *codec,
// C: 			struct snd_pcm_substream *substream)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int latency = DSP_CAPTURE_INIT_LATENCY;
// C: 	struct snd_pcm_runtime *runtime = substream->runtime;
// C: 
// C: 	if (spec->dsp_state != DSP_DOWNLOADED)
// C: 		return 0;
// C: 
// C: 	if (spec->effects_switch[CRYSTAL_VOICE - EFFECT_START_NID])
// C: 		latency += DSP_CRYSTAL_VOICE_LATENCY;
// C: 
// C: 	return (latency * runtime->rate) / 1000;
// C: }
// C: 
// C: /*
// C:  * Controls stuffs.
// C:  */
// C: 
// C: /*
// C:  * Mixer controls helpers.
// C:  */
// C: #define CA0132_CODEC_VOL_MONO(xname, nid, channel, dir) \\
// C: 	{ .iface = SNDRV_CTL_ELEM_IFACE_MIXER, \\
// C: 	  .name = xname, \\
// C: 	  .subdevice = HDA_SUBDEV_AMP_FLAG, \\
// C: 	  .access = SNDRV_CTL_ELEM_ACCESS_READWRITE | \\
// C: 			SNDRV_CTL_ELEM_ACCESS_TLV_READ | \\
// C: 			SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK, \\
// C: 	  .info = ca0132_volume_info, \\
// C: 	  .get = ca0132_volume_get, \\
// C: 	  .put = ca0132_volume_put, \\
// C: 	  .tlv = { .c = ca0132_volume_tlv }, \\
// C: 	  .private_value = HDA_COMPOSE_AMP_VAL(nid, channel, 0, dir) }
// C: 
// C: /*
// C:  * Creates a mixer control that uses defaults of HDA_CODEC_VOL except for the
// C:  * volume put, which is used for setting the DSP volume. This was done because
// C:  * the ca0132 functions were taking too much time and causing lag.
// C:  */
// C: #define CA0132_ALT_CODEC_VOL_MONO(xname, nid, channel, dir) \\
// C: 	{ .iface = SNDRV_CTL_ELEM_IFACE_MIXER, \\
// C: 	  .name = xname, \\
// C: 	  .subdevice = HDA_SUBDEV_AMP_FLAG, \\
// C: 	  .access = SNDRV_CTL_ELEM_ACCESS_READWRITE | \\
// C: 			SNDRV_CTL_ELEM_ACCESS_TLV_READ | \\
// C: 			SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK, \\
// C: 	  .info = snd_hda_mixer_amp_volume_info, \\
// C: 	  .get = snd_hda_mixer_amp_volume_get, \\
// C: 	  .put = ca0132_alt_volume_put, \\
// C: 	  .tlv = { .c = snd_hda_mixer_amp_tlv }, \\
// C: 	  .private_value = HDA_COMPOSE_AMP_VAL(nid, channel, 0, dir) }
// C: 
// C: #define CA0132_CODEC_MUTE_MONO(xname, nid, channel, dir) \\
// C: 	{ .iface = SNDRV_CTL_ELEM_IFACE_MIXER, \\
// C: 	  .name = xname, \\
// C: 	  .subdevice = HDA_SUBDEV_AMP_FLAG, \\
// C: 	  .info = snd_hda_mixer_amp_switch_info, \\
// C: 	  .get = ca0132_switch_get, \\
// C: 	  .put = ca0132_switch_put, \\
// C: 	  .private_value = HDA_COMPOSE_AMP_VAL(nid, channel, 0, dir) }
// C: 
// C: /* stereo */
// C: #define CA0132_CODEC_VOL(xname, nid, dir) \\
// C: 	CA0132_CODEC_VOL_MONO(xname, nid, 3, dir)
// C: #define CA0132_ALT_CODEC_VOL(xname, nid, dir) \\
// C: 	CA0132_ALT_CODEC_VOL_MONO(xname, nid, 3, dir)
// C: #define CA0132_CODEC_MUTE(xname, nid, dir) \\
// C: 	CA0132_CODEC_MUTE_MONO(xname, nid, 3, dir)
// C: 
// C: /* lookup tables */
// C: /*
// C:  * Lookup table with decibel values for the DSP. When volume is changed in
// C:  * Windows, the DSP is also sent the dB value in floating point. In Windows,
// C:  * these values have decimal points, probably because the Windows driver
// C:  * actually uses floating point. We can't here, so I made a lookup table of
// C:  * values -90 to 9. -90 is the lowest decibel value for both the ADC's and the
// C:  * DAC's, and 9 is the maximum.
// C:  */
// C: static const unsigned int float_vol_db_lookup[] = {
// C: 0xC2B40000, 0xC2B20000, 0xC2B00000, 0xC2AE0000, 0xC2AC0000, 0xC2AA0000,
// C: 0xC2A80000, 0xC2A60000, 0xC2A40000, 0xC2A20000, 0xC2A00000, 0xC29E0000,
// C: 0xC29C0000, 0xC29A0000, 0xC2980000, 0xC2960000, 0xC2940000, 0xC2920000,
// C: 0xC2900000, 0xC28E0000, 0xC28C0000, 0xC28A0000, 0xC2880000, 0xC2860000,
// C: 0xC2840000, 0xC2820000, 0xC2800000, 0xC27C0000, 0xC2780000, 0xC2740000,
// C: 0xC2700000, 0xC26C0000, 0xC2680000, 0xC2640000, 0xC2600000, 0xC25C0000,
// C: 0xC2580000, 0xC2540000, 0xC2500000, 0xC24C0000, 0xC2480000, 0xC2440000,
// C: 0xC2400000, 0xC23C0000, 0xC2380000, 0xC2340000, 0xC2300000, 0xC22C0000,
// C: 0xC2280000, 0xC2240000, 0xC2200000, 0xC21C0000, 0xC2180000, 0xC2140000,
// C: 0xC2100000, 0xC20C0000, 0xC2080000, 0xC2040000, 0xC2000000, 0xC1F80000,
// C: 0xC1F00000, 0xC1E80000, 0xC1E00000, 0xC1D80000, 0xC1D00000, 0xC1C80000,
// C: 0xC1C00000, 0xC1B80000, 0xC1B00000, 0xC1A80000, 0xC1A00000, 0xC1980000,
// C: 0xC1900000, 0xC1880000, 0xC1800000, 0xC1700000, 0xC1600000, 0xC1500000,
// C: 0xC1400000, 0xC1300000, 0xC1200000, 0xC1100000, 0xC1000000, 0xC0E00000,
// C: 0xC0C00000, 0xC0A00000, 0xC0800000, 0xC0400000, 0xC0000000, 0xBF800000,
// C: 0x00000000, 0x3F800000, 0x40000000, 0x40400000, 0x40800000, 0x40A00000,
// C: 0x40C00000, 0x40E00000, 0x41000000, 0x41100000
// C: };
// C: 
// C: /*
// C:  * This table counts from float 0 to 1 in increments of .01, which is
// C:  * useful for a few different sliders.
// C:  */
// C: static const unsigned int float_zero_to_one_lookup[] = {
// C: 0x00000000, 0x3C23D70A, 0x3CA3D70A, 0x3CF5C28F, 0x3D23D70A, 0x3D4CCCCD,
// C: 0x3D75C28F, 0x3D8F5C29, 0x3DA3D70A, 0x3DB851EC, 0x3DCCCCCD, 0x3DE147AE,
// C: 0x3DF5C28F, 0x3E051EB8, 0x3E0F5C29, 0x3E19999A, 0x3E23D70A, 0x3E2E147B,
// C: 0x3E3851EC, 0x3E428F5C, 0x3E4CCCCD, 0x3E570A3D, 0x3E6147AE, 0x3E6B851F,
// C: 0x3E75C28F, 0x3E800000, 0x3E851EB8, 0x3E8A3D71, 0x3E8F5C29, 0x3E947AE1,
// C: 0x3E99999A, 0x3E9EB852, 0x3EA3D70A, 0x3EA8F5C3, 0x3EAE147B, 0x3EB33333,
// C: 0x3EB851EC, 0x3EBD70A4, 0x3EC28F5C, 0x3EC7AE14, 0x3ECCCCCD, 0x3ED1EB85,
// C: 0x3ED70A3D, 0x3EDC28F6, 0x3EE147AE, 0x3EE66666, 0x3EEB851F, 0x3EF0A3D7,
// C: 0x3EF5C28F, 0x3EFAE148, 0x3F000000, 0x3F028F5C, 0x3F051EB8, 0x3F07AE14,
// C: 0x3F0A3D71, 0x3F0CCCCD, 0x3F0F5C29, 0x3F11EB85, 0x3F147AE1, 0x3F170A3D,
// C: 0x3F19999A, 0x3F1C28F6, 0x3F1EB852, 0x3F2147AE, 0x3F23D70A, 0x3F266666,
// C: 0x3F28F5C3, 0x3F2B851F, 0x3F2E147B, 0x3F30A3D7, 0x3F333333, 0x3F35C28F,
// C: 0x3F3851EC, 0x3F3AE148, 0x3F3D70A4, 0x3F400000, 0x3F428F5C, 0x3F451EB8,
// C: 0x3F47AE14, 0x3F4A3D71, 0x3F4CCCCD, 0x3F4F5C29, 0x3F51EB85, 0x3F547AE1,
// C: 0x3F570A3D, 0x3F59999A, 0x3F5C28F6, 0x3F5EB852, 0x3F6147AE, 0x3F63D70A,
// C: 0x3F666666, 0x3F68F5C3, 0x3F6B851F, 0x3F6E147B, 0x3F70A3D7, 0x3F733333,
// C: 0x3F75C28F, 0x3F7851EC, 0x3F7AE148, 0x3F7D70A4, 0x3F800000
// C: };
// C: 
// C: /*
// C:  * This table counts from float 10 to 1000, which is the range of the x-bass
// C:  * crossover slider in Windows.
// C:  */
// C: static const unsigned int float_xbass_xover_lookup[] = {
// C: 0x41200000, 0x41A00000, 0x41F00000, 0x42200000, 0x42480000, 0x42700000,
// C: 0x428C0000, 0x42A00000, 0x42B40000, 0x42C80000, 0x42DC0000, 0x42F00000,
// C: 0x43020000, 0x430C0000, 0x43160000, 0x43200000, 0x432A0000, 0x43340000,
// C: 0x433E0000, 0x43480000, 0x43520000, 0x435C0000, 0x43660000, 0x43700000,
// C: 0x437A0000, 0x43820000, 0x43870000, 0x438C0000, 0x43910000, 0x43960000,
// C: 0x439B0000, 0x43A00000, 0x43A50000, 0x43AA0000, 0x43AF0000, 0x43B40000,
// C: 0x43B90000, 0x43BE0000, 0x43C30000, 0x43C80000, 0x43CD0000, 0x43D20000,
// C: 0x43D70000, 0x43DC0000, 0x43E10000, 0x43E60000, 0x43EB0000, 0x43F00000,
// C: 0x43F50000, 0x43FA0000, 0x43FF0000, 0x44020000, 0x44048000, 0x44070000,
// C: 0x44098000, 0x440C0000, 0x440E8000, 0x44110000, 0x44138000, 0x44160000,
// C: 0x44188000, 0x441B0000, 0x441D8000, 0x44200000, 0x44228000, 0x44250000,
// C: 0x44278000, 0x442A0000, 0x442C8000, 0x442F0000, 0x44318000, 0x44340000,
// C: 0x44368000, 0x44390000, 0x443B8000, 0x443E0000, 0x44408000, 0x44430000,
// C: 0x44458000, 0x44480000, 0x444A8000, 0x444D0000, 0x444F8000, 0x44520000,
// C: 0x44548000, 0x44570000, 0x44598000, 0x445C0000, 0x445E8000, 0x44610000,
// C: 0x44638000, 0x44660000, 0x44688000, 0x446B0000, 0x446D8000, 0x44700000,
// C: 0x44728000, 0x44750000, 0x44778000, 0x447A0000
// C: };
// C: 
// C: /* The following are for tuning of products */
// C: #ifdef ENABLE_TUNING_CONTROLS
// C: 
// C: static const unsigned int voice_focus_vals_lookup[] = {
// C: 0x41A00000, 0x41A80000, 0x41B00000, 0x41B80000, 0x41C00000, 0x41C80000,
// C: 0x41D00000, 0x41D80000, 0x41E00000, 0x41E80000, 0x41F00000, 0x41F80000,
// C: 0x42000000, 0x42040000, 0x42080000, 0x420C0000, 0x42100000, 0x42140000,
// C: 0x42180000, 0x421C0000, 0x42200000, 0x42240000, 0x42280000, 0x422C0000,
// C: 0x42300000, 0x42340000, 0x42380000, 0x423C0000, 0x42400000, 0x42440000,
// C: 0x42480000, 0x424C0000, 0x42500000, 0x42540000, 0x42580000, 0x425C0000,
// C: 0x42600000, 0x42640000, 0x42680000, 0x426C0000, 0x42700000, 0x42740000,
// C: 0x42780000, 0x427C0000, 0x42800000, 0x42820000, 0x42840000, 0x42860000,
// C: 0x42880000, 0x428A0000, 0x428C0000, 0x428E0000, 0x42900000, 0x42920000,
// C: 0x42940000, 0x42960000, 0x42980000, 0x429A0000, 0x429C0000, 0x429E0000,
// C: 0x42A00000, 0x42A20000, 0x42A40000, 0x42A60000, 0x42A80000, 0x42AA0000,
// C: 0x42AC0000, 0x42AE0000, 0x42B00000, 0x42B20000, 0x42B40000, 0x42B60000,
// C: 0x42B80000, 0x42BA0000, 0x42BC0000, 0x42BE0000, 0x42C00000, 0x42C20000,
// C: 0x42C40000, 0x42C60000, 0x42C80000, 0x42CA0000, 0x42CC0000, 0x42CE0000,
// C: 0x42D00000, 0x42D20000, 0x42D40000, 0x42D60000, 0x42D80000, 0x42DA0000,
// C: 0x42DC0000, 0x42DE0000, 0x42E00000, 0x42E20000, 0x42E40000, 0x42E60000,
// C: 0x42E80000, 0x42EA0000, 0x42EC0000, 0x42EE0000, 0x42F00000, 0x42F20000,
// C: 0x42F40000, 0x42F60000, 0x42F80000, 0x42FA0000, 0x42FC0000, 0x42FE0000,
// C: 0x43000000, 0x43010000, 0x43020000, 0x43030000, 0x43040000, 0x43050000,
// C: 0x43060000, 0x43070000, 0x43080000, 0x43090000, 0x430A0000, 0x430B0000,
// C: 0x430C0000, 0x430D0000, 0x430E0000, 0x430F0000, 0x43100000, 0x43110000,
// C: 0x43120000, 0x43130000, 0x43140000, 0x43150000, 0x43160000, 0x43170000,
// C: 0x43180000, 0x43190000, 0x431A0000, 0x431B0000, 0x431C0000, 0x431D0000,
// C: 0x431E0000, 0x431F0000, 0x43200000, 0x43210000, 0x43220000, 0x43230000,
// C: 0x43240000, 0x43250000, 0x43260000, 0x43270000, 0x43280000, 0x43290000,
// C: 0x432A0000, 0x432B0000, 0x432C0000, 0x432D0000, 0x432E0000, 0x432F0000,
// C: 0x43300000, 0x43310000, 0x43320000, 0x43330000, 0x43340000
// C: };
// C: 
// C: static const unsigned int mic_svm_vals_lookup[] = {
// C: 0x00000000, 0x3C23D70A, 0x3CA3D70A, 0x3CF5C28F, 0x3D23D70A, 0x3D4CCCCD,
// C: 0x3D75C28F, 0x3D8F5C29, 0x3DA3D70A, 0x3DB851EC, 0x3DCCCCCD, 0x3DE147AE,
// C: 0x3DF5C28F, 0x3E051EB8, 0x3E0F5C29, 0x3E19999A, 0x3E23D70A, 0x3E2E147B,
// C: 0x3E3851EC, 0x3E428F5C, 0x3E4CCCCD, 0x3E570A3D, 0x3E6147AE, 0x3E6B851F,
// C: 0x3E75C28F, 0x3E800000, 0x3E851EB8, 0x3E8A3D71, 0x3E8F5C29, 0x3E947AE1,
// C: 0x3E99999A, 0x3E9EB852, 0x3EA3D70A, 0x3EA8F5C3, 0x3EAE147B, 0x3EB33333,
// C: 0x3EB851EC, 0x3EBD70A4, 0x3EC28F5C, 0x3EC7AE14, 0x3ECCCCCD, 0x3ED1EB85,
// C: 0x3ED70A3D, 0x3EDC28F6, 0x3EE147AE, 0x3EE66666, 0x3EEB851F, 0x3EF0A3D7,
// C: 0x3EF5C28F, 0x3EFAE148, 0x3F000000, 0x3F028F5C, 0x3F051EB8, 0x3F07AE14,
// C: 0x3F0A3D71, 0x3F0CCCCD, 0x3F0F5C29, 0x3F11EB85, 0x3F147AE1, 0x3F170A3D,
// C: 0x3F19999A, 0x3F1C28F6, 0x3F1EB852, 0x3F2147AE, 0x3F23D70A, 0x3F266666,
// C: 0x3F28F5C3, 0x3F2B851F, 0x3F2E147B, 0x3F30A3D7, 0x3F333333, 0x3F35C28F,
// C: 0x3F3851EC, 0x3F3AE148, 0x3F3D70A4, 0x3F400000, 0x3F428F5C, 0x3F451EB8,
// C: 0x3F47AE14, 0x3F4A3D71, 0x3F4CCCCD, 0x3F4F5C29, 0x3F51EB85, 0x3F547AE1,
// C: 0x3F570A3D, 0x3F59999A, 0x3F5C28F6, 0x3F5EB852, 0x3F6147AE, 0x3F63D70A,
// C: 0x3F666666, 0x3F68F5C3, 0x3F6B851F, 0x3F6E147B, 0x3F70A3D7, 0x3F733333,
// C: 0x3F75C28F, 0x3F7851EC, 0x3F7AE148, 0x3F7D70A4, 0x3F800000
// C: };
// C: 
// C: static const unsigned int equalizer_vals_lookup[] = {
// C: 0xC1C00000, 0xC1B80000, 0xC1B00000, 0xC1A80000, 0xC1A00000, 0xC1980000,
// C: 0xC1900000, 0xC1880000, 0xC1800000, 0xC1700000, 0xC1600000, 0xC1500000,
// C: 0xC1400000, 0xC1300000, 0xC1200000, 0xC1100000, 0xC1000000, 0xC0E00000,
// C: 0xC0C00000, 0xC0A00000, 0xC0800000, 0xC0400000, 0xC0000000, 0xBF800000,
// C: 0x00000000, 0x3F800000, 0x40000000, 0x40400000, 0x40800000, 0x40A00000,
// C: 0x40C00000, 0x40E00000, 0x41000000, 0x41100000, 0x41200000, 0x41300000,
// C: 0x41400000, 0x41500000, 0x41600000, 0x41700000, 0x41800000, 0x41880000,
// C: 0x41900000, 0x41980000, 0x41A00000, 0x41A80000, 0x41B00000, 0x41B80000,
// C: 0x41C00000
// C: };
// C: 
// C: static int tuning_ctl_set(struct hda_codec *codec, hda_nid_t nid,
// C: 			  const unsigned int *lookup, int idx)
// C: {
// C: 	int i;
// C: 
// C: 	for (i = 0; i < TUNING_CTLS_COUNT; i++) {
// C: 		if (nid == ca0132_tuning_ctls[i].nid) {
// C: 			CLASS(snd_hda_power, pm)(codec);
// C: 			dspio_set_param(codec, ca0132_tuning_ctls[i].mid, 0x20,
// C: 					ca0132_tuning_ctls[i].req,
// C: 					&(lookup[idx]), sizeof(unsigned int));
// C: 			return 1;
// C: 		}
// C: 	}
// C: 
// C: 	return -EINVAL;
// C: }
// C: 
// C: static int tuning_ctl_get(struct snd_kcontrol *kcontrol,
// C: 			  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 	int idx = nid - TUNING_CTL_START_NID;
// C: 
// C: 	*valp = spec->cur_ctl_vals[idx];
// C: 	return 0;
// C: }
// C: 
// C: static int voice_focus_ctl_info(struct snd_kcontrol *kcontrol,
// C: 			      struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	int chs = get_amp_channels(kcontrol);
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = chs == 3 ? 2 : 1;
// C: 	uinfo->value.integer.min = 20;
// C: 	uinfo->value.integer.max = 180;
// C: 	uinfo->value.integer.step = 1;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int voice_focus_ctl_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 	int idx;
// C: 
// C: 	idx = nid - TUNING_CTL_START_NID;
// C: 	/* any change? */
// C: 	if (spec->cur_ctl_vals[idx] == *valp)
// C: 		return 0;
// C: 
// C: 	spec->cur_ctl_vals[idx] = *valp;
// C: 
// C: 	idx = *valp - 20;
// C: 	tuning_ctl_set(codec, nid, voice_focus_vals_lookup, idx);
// C: 
// C: 	return 1;
// C: }
// C: 
// C: static int mic_svm_ctl_info(struct snd_kcontrol *kcontrol,
// C: 			      struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	int chs = get_amp_channels(kcontrol);
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = chs == 3 ? 2 : 1;
// C: 	uinfo->value.integer.min = 0;
// C: 	uinfo->value.integer.max = 100;
// C: 	uinfo->value.integer.step = 1;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int mic_svm_ctl_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 	int idx;
// C: 
// C: 	idx = nid - TUNING_CTL_START_NID;
// C: 	/* any change? */
// C: 	if (spec->cur_ctl_vals[idx] == *valp)
// C: 		return 0;
// C: 
// C: 	spec->cur_ctl_vals[idx] = *valp;
// C: 
// C: 	idx = *valp;
// C: 	tuning_ctl_set(codec, nid, mic_svm_vals_lookup, idx);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int equalizer_ctl_info(struct snd_kcontrol *kcontrol,
// C: 			      struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	int chs = get_amp_channels(kcontrol);
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = chs == 3 ? 2 : 1;
// C: 	uinfo->value.integer.min = 0;
// C: 	uinfo->value.integer.max = 48;
// C: 	uinfo->value.integer.step = 1;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int equalizer_ctl_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 	int idx;
// C: 
// C: 	idx = nid - TUNING_CTL_START_NID;
// C: 	/* any change? */
// C: 	if (spec->cur_ctl_vals[idx] == *valp)
// C: 		return 0;
// C: 
// C: 	spec->cur_ctl_vals[idx] = *valp;
// C: 
// C: 	idx = *valp;
// C: 	tuning_ctl_set(codec, nid, equalizer_vals_lookup, idx);
// C: 
// C: 	return 1;
// C: }
// C: 
// C: static const SNDRV_CTL_TLVD_DECLARE_DB_SCALE(voice_focus_db_scale, 2000, 100, 0);
// C: static const SNDRV_CTL_TLVD_DECLARE_DB_SCALE(eq_db_scale, -2400, 100, 0);
// C: 
// C: static int add_tuning_control(struct hda_codec *codec,
// C: 				hda_nid_t pnid, hda_nid_t nid,
// C: 				const char *name, int dir)
// C: {
// C: 	char namestr[SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
// C: 	int type = dir ? HDA_INPUT : HDA_OUTPUT;
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_VOLUME_MONO(namestr, nid, 1, 0, type);
// C: 
// C: 	knew.access = SNDRV_CTL_ELEM_ACCESS_READWRITE |
// C: 			SNDRV_CTL_ELEM_ACCESS_TLV_READ;
// C: 	knew.tlv.c = NULL;
// C: 	knew.tlv.p = NULL;
// C: 	switch (pnid) {
// C: 	case VOICE_FOCUS:
// C: 		knew.info = voice_focus_ctl_info;
// C: 		knew.get = tuning_ctl_get;
// C: 		knew.put = voice_focus_ctl_put;
// C: 		knew.tlv.p = voice_focus_db_scale;
// C: 		break;
// C: 	case MIC_SVM:
// C: 		knew.info = mic_svm_ctl_info;
// C: 		knew.get = tuning_ctl_get;
// C: 		knew.put = mic_svm_ctl_put;
// C: 		break;
// C: 	case EQUALIZER:
// C: 		knew.info = equalizer_ctl_info;
// C: 		knew.get = tuning_ctl_get;
// C: 		knew.put = equalizer_ctl_put;
// C: 		knew.tlv.p = eq_db_scale;
// C: 		break;
// C: 	default:
// C: 		return 0;
// C: 	}
// C: 	knew.private_value =
// C: 		HDA_COMPOSE_AMP_VAL(nid, 1, 0, type);
// C: 	snprintf(namestr, sizeof(namestr), "%s %s Volume", name, dirstr[dir]);
// C: 	return snd_hda_ctl_add(codec, nid, snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: static int add_tuning_ctls(struct hda_codec *codec)
// C: {
// C: 	int i;
// C: 	int err;
// C: 
// C: 	for (i = 0; i < TUNING_CTLS_COUNT; i++) {
// C: 		err = add_tuning_control(codec,
// C: 					ca0132_tuning_ctls[i].parent_nid,
// C: 					ca0132_tuning_ctls[i].nid,
// C: 					ca0132_tuning_ctls[i].name,
// C: 					ca0132_tuning_ctls[i].direct);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static void ca0132_init_tuning_defaults(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int i;
// C: 
// C: 	/* Wedge Angle defaults to 30.  10 below is 30 - 20.  20 is min. */
// C: 	spec->cur_ctl_vals[WEDGE_ANGLE - TUNING_CTL_START_NID] = 10;
// C: 	/* SVM level defaults to 0.74. */
// C: 	spec->cur_ctl_vals[SVM_LEVEL - TUNING_CTL_START_NID] = 74;
// C: 
// C: 	/* EQ defaults to 0dB. */
// C: 	for (i = 2; i < TUNING_CTLS_COUNT; i++)
// C: 		spec->cur_ctl_vals[i] = 24;
// C: }
// C: #endif /*ENABLE_TUNING_CONTROLS*/
// C: 
// C: /*
// C:  * Select the active output.
// C:  * If autodetect is enabled, output will be selected based on jack detection.
// C:  * If jack inserted, headphone will be selected, else built-in speakers
// C:  * If autodetect is disabled, output will be selected based on selection.
// C:  */
// C: static int ca0132_select_out(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int pin_ctl;
// C: 	int jack_present;
// C: 	int auto_jack;
// C: 	unsigned int tmp;
// C: 	int err;
// C: 
// C: 	codec_dbg(codec, "ca0132_select_out\\n");
// C: 
// C: 	CLASS(snd_hda_power_pm, pm)(codec);
// C: 
// C: 	auto_jack = spec->vnode_lswitch[VNID_HP_ASEL - VNODE_START_NID];
// C: 
// C: 	if (auto_jack)
// C: 		jack_present = snd_hda_jack_detect(codec, spec->unsol_tag_hp);
// C: 	else
// C: 		jack_present =
// C: 			spec->vnode_lswitch[VNID_HP_SEL - VNODE_START_NID];
// C: 
// C: 	if (jack_present)
// C: 		spec->cur_out_type = HEADPHONE_OUT;
// C: 	else
// C: 		spec->cur_out_type = SPEAKER_OUT;
// C: 
// C: 	if (spec->cur_out_type == SPEAKER_OUT) {
// C: 		codec_dbg(codec, "ca0132_select_out speaker\\n");
// C: 		/*speaker out config*/
// C: 		tmp = FLOAT_ONE;
// C: 		err = dspio_set_uint_param(codec, 0x80, 0x04, tmp);
// C: 		if (err < 0)
// C: 			return err;
// C: 		/*enable speaker EQ*/
// C: 		tmp = FLOAT_ONE;
// C: 		err = dspio_set_uint_param(codec, 0x8f, 0x00, tmp);
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		/* Setup EAPD */
// C: 		snd_hda_codec_write(codec, spec->out_pins[1], 0,
// C: 				    VENDOR_CHIPIO_EAPD_SEL_SET, 0x02);
// C: 		snd_hda_codec_write(codec, spec->out_pins[0], 0,
// C: 				    AC_VERB_SET_EAPD_BTLENABLE, 0x00);
// C: 		snd_hda_codec_write(codec, spec->out_pins[0], 0,
// C: 				    VENDOR_CHIPIO_EAPD_SEL_SET, 0x00);
// C: 		snd_hda_codec_write(codec, spec->out_pins[0], 0,
// C: 				    AC_VERB_SET_EAPD_BTLENABLE, 0x02);
// C: 
// C: 		/* disable headphone node */
// C: 		pin_ctl = snd_hda_codec_read(codec, spec->out_pins[1], 0,
// C: 					AC_VERB_GET_PIN_WIDGET_CONTROL, 0);
// C: 		snd_hda_set_pin_ctl(codec, spec->out_pins[1],
// C: 				    pin_ctl & ~PIN_HP);
// C: 		/* enable speaker node */
// C: 		pin_ctl = snd_hda_codec_read(codec, spec->out_pins[0], 0,
// C: 				AC_VERB_GET_PIN_WIDGET_CONTROL, 0);
// C: 		snd_hda_set_pin_ctl(codec, spec->out_pins[0],
// C: 				    pin_ctl | PIN_OUT);
// C: 	} else {
// C: 		codec_dbg(codec, "ca0132_select_out hp\\n");
// C: 		/*headphone out config*/
// C: 		tmp = FLOAT_ZERO;
// C: 		err = dspio_set_uint_param(codec, 0x80, 0x04, tmp);
// C: 		if (err < 0)
// C: 			return err;
// C: 		/*disable speaker EQ*/
// C: 		tmp = FLOAT_ZERO;
// C: 		err = dspio_set_uint_param(codec, 0x8f, 0x00, tmp);
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		/* Setup EAPD */
// C: 		snd_hda_codec_write(codec, spec->out_pins[0], 0,
// C: 				    VENDOR_CHIPIO_EAPD_SEL_SET, 0x00);
// C: 		snd_hda_codec_write(codec, spec->out_pins[0], 0,
// C: 				    AC_VERB_SET_EAPD_BTLENABLE, 0x00);
// C: 		snd_hda_codec_write(codec, spec->out_pins[1], 0,
// C: 				    VENDOR_CHIPIO_EAPD_SEL_SET, 0x02);
// C: 		snd_hda_codec_write(codec, spec->out_pins[0], 0,
// C: 				    AC_VERB_SET_EAPD_BTLENABLE, 0x02);
// C: 
// C: 		/* disable speaker*/
// C: 		pin_ctl = snd_hda_codec_read(codec, spec->out_pins[0], 0,
// C: 					AC_VERB_GET_PIN_WIDGET_CONTROL, 0);
// C: 		snd_hda_set_pin_ctl(codec, spec->out_pins[0],
// C: 				    pin_ctl & ~PIN_HP);
// C: 		/* enable headphone*/
// C: 		pin_ctl = snd_hda_codec_read(codec, spec->out_pins[1], 0,
// C: 					AC_VERB_GET_PIN_WIDGET_CONTROL, 0);
// C: 		snd_hda_set_pin_ctl(codec, spec->out_pins[1],
// C: 				    pin_ctl | PIN_HP);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int ae5_headphone_gain_set(struct hda_codec *codec, long val);
// C: static int zxr_headphone_gain_set(struct hda_codec *codec, long val);
// C: static int ca0132_effects_set(struct hda_codec *codec, hda_nid_t nid, long val);
// C: 
// C: static void ae5_mmio_select_out(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	const struct ae_ca0113_output_set *out_cmds;
// C: 	unsigned int i;
// C: 
// C: 	if (ca0132_quirk(spec) == QUIRK_AE5)
// C: 		out_cmds = &ae5_ca0113_output_presets;
// C: 	else
// C: 		out_cmds = &ae7_ca0113_output_presets;
// C: 
// C: 	for (i = 0; i < AE_CA0113_OUT_SET_COMMANDS; i++)
// C: 		ca0113_mmio_command_set(codec, out_cmds->group[i],
// C: 				out_cmds->target[i],
// C: 				out_cmds->vals[spec->cur_out_type][i]);
// C: }
// C: 
// C: static int ca0132_alt_set_full_range_speaker(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int quirk = ca0132_quirk(spec);
// C: 	unsigned int tmp;
// C: 	int err;
// C: 
// C: 	/* 2.0/4.0 setup has no LFE channel, so setting full-range does nothing. */
// C: 	if (spec->channel_cfg_val == SPEAKER_CHANNELS_4_0
// C: 			|| spec->channel_cfg_val == SPEAKER_CHANNELS_2_0)
// C: 		return 0;
// C: 
// C: 	/* Set front L/R full range. Zero for full-range, one for redirection. */
// C: 	tmp = spec->speaker_range_val[0] ? FLOAT_ZERO : FLOAT_ONE;
// C: 	err = dspio_set_uint_param(codec, 0x96,
// C: 			SPEAKER_FULL_RANGE_FRONT_L_R, tmp);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	/* When setting full-range rear, both rear and center/lfe are set. */
// C: 	tmp = spec->speaker_range_val[1] ? FLOAT_ZERO : FLOAT_ONE;
// C: 	err = dspio_set_uint_param(codec, 0x96,
// C: 			SPEAKER_FULL_RANGE_CENTER_LFE, tmp);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	err = dspio_set_uint_param(codec, 0x96,
// C: 			SPEAKER_FULL_RANGE_REAR_L_R, tmp);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	/*
// C: 	 * Only the AE series cards set this value when setting full-range,
// C: 	 * and it's always 1.0f.
// C: 	 */
// C: 	if (quirk == QUIRK_AE5 || quirk == QUIRK_AE7) {
// C: 		err = dspio_set_uint_param(codec, 0x96,
// C: 				SPEAKER_FULL_RANGE_SURROUND_L_R, FLOAT_ONE);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_surround_set_bass_redirection(struct hda_codec *codec,
// C: 		bool val)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 	int err;
// C: 
// C: 	if (val && spec->channel_cfg_val != SPEAKER_CHANNELS_4_0 &&
// C: 			spec->channel_cfg_val != SPEAKER_CHANNELS_2_0)
// C: 		tmp = FLOAT_ONE;
// C: 	else
// C: 		tmp = FLOAT_ZERO;
// C: 
// C: 	err = dspio_set_uint_param(codec, 0x96, SPEAKER_BASS_REDIRECT, tmp);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	/* If it is enabled, make sure to set the crossover frequency. */
// C: 	if (tmp) {
// C: 		tmp = float_xbass_xover_lookup[spec->xbass_xover_freq];
// C: 		err = dspio_set_uint_param(codec, 0x96,
// C: 				SPEAKER_BASS_REDIRECT_XOVER_FREQ, tmp);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * These are the commands needed to setup output on each of the different card
// C:  * types.
// C:  */
// C: static void ca0132_alt_select_out_get_quirk_data(struct hda_codec *codec,
// C: 		const struct ca0132_alt_out_set_quirk_data **quirk_data)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int quirk = ca0132_quirk(spec);
// C: 	unsigned int i;
// C: 
// C: 	*quirk_data = NULL;
// C: 	for (i = 0; i < ARRAY_SIZE(quirk_out_set_data); i++) {
// C: 		if (quirk_out_set_data[i].quirk_id == quirk) {
// C: 			*quirk_data = &quirk_out_set_data[i];
// C: 			return;
// C: 		}
// C: 	}
// C: }
// C: 
// C: static int ca0132_alt_select_out_quirk_set(struct hda_codec *codec)
// C: {
// C: 	const struct ca0132_alt_out_set_quirk_data *quirk_data;
// C: 	const struct ca0132_alt_out_set_info *out_info;
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int i, gpio_data;
// C: 	int err;
// C: 
// C: 	ca0132_alt_select_out_get_quirk_data(codec, &quirk_data);
// C: 	if (!quirk_data)
// C: 		return 0;
// C: 
// C: 	out_info = &quirk_data->out_set_info[spec->cur_out_type];
// C: 	if (quirk_data->is_ae_series)
// C: 		ae5_mmio_select_out(codec);
// C: 
// C: 	if (out_info->has_hda_gpio) {
// C: 		gpio_data = snd_hda_codec_read(codec, codec->core.afg, 0,
// C: 				AC_VERB_GET_GPIO_DATA, 0);
// C: 
// C: 		if (out_info->hda_gpio_set)
// C: 			gpio_data |= (1 << out_info->hda_gpio_pin);
// C: 		else
// C: 			gpio_data &= ~(1 << out_info->hda_gpio_pin);
// C: 
// C: 		snd_hda_codec_write(codec, codec->core.afg, 0,
// C: 				    AC_VERB_SET_GPIO_DATA, gpio_data);
// C: 	}
// C: 
// C: 	if (out_info->mmio_gpio_count) {
// C: 		for (i = 0; i < out_info->mmio_gpio_count; i++) {
// C: 			ca0113_mmio_gpio_set(codec, out_info->mmio_gpio_pin[i],
// C: 					out_info->mmio_gpio_set[i]);
// C: 		}
// C: 	}
// C: 
// C: 	if (out_info->scp_cmds_count) {
// C: 		for (i = 0; i < out_info->scp_cmds_count; i++) {
// C: 			err = dspio_set_uint_param(codec,
// C: 					out_info->scp_cmd_mid[i],
// C: 					out_info->scp_cmd_req[i],
// C: 					out_info->scp_cmd_val[i]);
// C: 			if (err < 0)
// C: 				return err;
// C: 		}
// C: 	}
// C: 
// C: 	chipio_set_control_param(codec, 0x0d, out_info->dac2port);
// C: 
// C: 	if (out_info->has_chipio_write) {
// C: 		chipio_write(codec, out_info->chipio_write_addr,
// C: 				out_info->chipio_write_data);
// C: 	}
// C: 
// C: 	if (quirk_data->has_headphone_gain) {
// C: 		if (spec->cur_out_type != HEADPHONE_OUT) {
// C: 			if (quirk_data->is_ae_series)
// C: 				ae5_headphone_gain_set(codec, 2);
// C: 			else
// C: 				zxr_headphone_gain_set(codec, 0);
// C: 		} else {
// C: 			if (quirk_data->is_ae_series)
// C: 				ae5_headphone_gain_set(codec,
// C: 						spec->ae5_headphone_gain_val);
// C: 			else
// C: 				zxr_headphone_gain_set(codec,
// C: 						spec->zxr_gain_set);
// C: 		}
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static void ca0132_set_out_node_pincfg(struct hda_codec *codec, hda_nid_t nid,
// C: 		bool out_enable, bool hp_enable)
// C: {
// C: 	unsigned int pin_ctl;
// C: 
// C: 	pin_ctl = snd_hda_codec_read(codec, nid, 0,
// C: 			AC_VERB_GET_PIN_WIDGET_CONTROL, 0);
// C: 
// C: 	pin_ctl = hp_enable ? pin_ctl | PIN_HP_AMP : pin_ctl & ~PIN_HP_AMP;
// C: 	pin_ctl = out_enable ? pin_ctl | PIN_OUT : pin_ctl & ~PIN_OUT;
// C: 	snd_hda_set_pin_ctl(codec, nid, pin_ctl);
// C: }
// C: 
// C: /*
// C:  * This function behaves similarly to the ca0132_select_out funciton above,
// C:  * except with a few differences. It adds the ability to select the current
// C:  * output with an enumerated control "output source" if the auto detect
// C:  * mute switch is set to off. If the auto detect mute switch is enabled, it
// C:  * will detect either headphone or lineout(SPEAKER_OUT) from jack detection.
// C:  * It also adds the ability to auto-detect the front headphone port.
// C:  */
// C: static int ca0132_alt_select_out(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp, outfx_set;
// C: 	int jack_present;
// C: 	int auto_jack;
// C: 	int err;
// C: 	/* Default Headphone is rear headphone */
// C: 	hda_nid_t headphone_nid = spec->out_pins[1];
// C: 
// C: 	codec_dbg(codec, "%s\\n", __func__);
// C: 
// C: 	CLASS(snd_hda_power_pm, pm)(codec);
// C: 
// C: 	auto_jack = spec->vnode_lswitch[VNID_HP_ASEL - VNODE_START_NID];
// C: 
// C: 	/*
// C: 	 * If headphone rear or front is plugged in, set to headphone.
// C: 	 * If neither is plugged in, set to rear line out. Only if
// C: 	 * hp/speaker auto detect is enabled.
// C: 	 */
// C: 	if (auto_jack) {
// C: 		jack_present = snd_hda_jack_detect(codec, spec->unsol_tag_hp) ||
// C: 			   snd_hda_jack_detect(codec, spec->unsol_tag_front_hp);
// C: 
// C: 		if (jack_present)
// C: 			spec->cur_out_type = HEADPHONE_OUT;
// C: 		else
// C: 			spec->cur_out_type = SPEAKER_OUT;
// C: 	} else
// C: 		spec->cur_out_type = spec->out_enum_val;
// C: 
// C: 	outfx_set = spec->effects_switch[PLAY_ENHANCEMENT - EFFECT_START_NID];
// C: 
// C: 	/* Begin DSP output switch, mute DSP volume. */
// C: 	err = dspio_set_uint_param(codec, 0x96, SPEAKER_TUNING_MUTE, FLOAT_ONE);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	err = ca0132_alt_select_out_quirk_set(codec);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	switch (spec->cur_out_type) {
// C: 	case SPEAKER_OUT:
// C: 		codec_dbg(codec, "%s speaker\\n", __func__);
// C: 
// C: 		/* Enable EAPD */
// C: 		snd_hda_codec_write(codec, spec->out_pins[0], 0,
// C: 			AC_VERB_SET_EAPD_BTLENABLE, 0x01);
// C: 
// C: 		/* Disable headphone node. */
// C: 		ca0132_set_out_node_pincfg(codec, spec->out_pins[1], 0, 0);
// C: 		/* Set front L-R to output. */
// C: 		ca0132_set_out_node_pincfg(codec, spec->out_pins[0], 1, 0);
// C: 		/* Set Center/LFE to output. */
// C: 		ca0132_set_out_node_pincfg(codec, spec->out_pins[2], 1, 0);
// C: 		/* Set rear surround to output. */
// C: 		ca0132_set_out_node_pincfg(codec, spec->out_pins[3], 1, 0);
// C: 
// C: 		/*
// C: 		 * Without PlayEnhancement being enabled, if we've got a 2.0
// C: 		 * setup, set it to floating point eight to disable any DSP
// C: 		 * processing effects.
// C: 		 */
// C: 		if (!outfx_set && spec->channel_cfg_val == SPEAKER_CHANNELS_2_0)
// C: 			tmp = FLOAT_EIGHT;
// C: 		else
// C: 			tmp = speaker_channel_cfgs[spec->channel_cfg_val].val;
// C: 
// C: 		err = dspio_set_uint_param(codec, 0x80, 0x04, tmp);
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		break;
// C: 	case HEADPHONE_OUT:
// C: 		codec_dbg(codec, "%s hp\\n", __func__);
// C: 		snd_hda_codec_write(codec, spec->out_pins[0], 0,
// C: 			AC_VERB_SET_EAPD_BTLENABLE, 0x00);
// C: 
// C: 		/* Disable all speaker nodes. */
// C: 		ca0132_set_out_node_pincfg(codec, spec->out_pins[0], 0, 0);
// C: 		ca0132_set_out_node_pincfg(codec, spec->out_pins[2], 0, 0);
// C: 		ca0132_set_out_node_pincfg(codec, spec->out_pins[3], 0, 0);
// C: 
// C: 		/* enable headphone, either front or rear */
// C: 		if (snd_hda_jack_detect(codec, spec->unsol_tag_front_hp))
// C: 			headphone_nid = spec->out_pins[2];
// C: 		else if (snd_hda_jack_detect(codec, spec->unsol_tag_hp))
// C: 			headphone_nid = spec->out_pins[1];
// C: 
// C: 		ca0132_set_out_node_pincfg(codec, headphone_nid, 1, 1);
// C: 
// C: 		if (outfx_set)
// C: 			err = dspio_set_uint_param(codec, 0x80, 0x04, FLOAT_ONE);
// C: 		else
// C: 			err = dspio_set_uint_param(codec, 0x80, 0x04, FLOAT_ZERO);
// C: 
// C: 		if (err < 0)
// C: 			return err;
// C: 		break;
// C: 	}
// C: 	/*
// C: 	 * If output effects are enabled, set the X-Bass effect value again to
// C: 	 * make sure that it's properly enabled/disabled for speaker
// C: 	 * configurations with an LFE channel.
// C: 	 */
// C: 	if (outfx_set)
// C: 		ca0132_effects_set(codec, X_BASS,
// C: 			spec->effects_switch[X_BASS - EFFECT_START_NID]);
// C: 
// C: 	/* Set speaker EQ bypass attenuation to 0. */
// C: 	err = dspio_set_uint_param(codec, 0x8f, 0x01, FLOAT_ZERO);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	/*
// C: 	 * Although unused on all cards but the AE series, this is always set
// C: 	 * to zero when setting the output.
// C: 	 */
// C: 	err = dspio_set_uint_param(codec, 0x96,
// C: 			SPEAKER_TUNING_USE_SPEAKER_EQ, FLOAT_ZERO);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	if (spec->cur_out_type == SPEAKER_OUT)
// C: 		err = ca0132_alt_surround_set_bass_redirection(codec,
// C: 				spec->bass_redirection_val);
// C: 	else
// C: 		err = ca0132_alt_surround_set_bass_redirection(codec, 0);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	/* Unmute DSP now that we're done with output selection. */
// C: 	err = dspio_set_uint_param(codec, 0x96,
// C: 			SPEAKER_TUNING_MUTE, FLOAT_ZERO);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	if (spec->cur_out_type == SPEAKER_OUT) {
// C: 		err = ca0132_alt_set_full_range_speaker(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static void ca0132_unsol_hp_delayed(struct work_struct *work)
// C: {
// C: 	struct ca0132_spec *spec = container_of(
// C: 		to_delayed_work(work), struct ca0132_spec, unsol_hp_work);
// C: 	struct hda_jack_tbl *jack;
// C: 
// C: 	if (ca0132_use_alt_functions(spec))
// C: 		ca0132_alt_select_out(spec->codec);
// C: 	else
// C: 		ca0132_select_out(spec->codec);
// C: 
// C: 	jack = snd_hda_jack_tbl_get(spec->codec, spec->unsol_tag_hp);
// C: 	if (jack) {
// C: 		jack->block_report = 0;
// C: 		snd_hda_jack_report_sync(spec->codec);
// C: 	}
// C: }
// C: 
// C: static void ca0132_set_dmic(struct hda_codec *codec, int enable);
// C: static int ca0132_mic_boost_set(struct hda_codec *codec, long val);
// C: static void resume_mic1(struct hda_codec *codec, unsigned int oldval);
// C: static int stop_mic1(struct hda_codec *codec);
// C: static int ca0132_cvoice_switch_set(struct hda_codec *codec);
// C: static int ca0132_alt_mic_boost_set(struct hda_codec *codec, long val);
// C: 
// C: /*
// C:  * Select the active VIP source
// C:  */
// C: static int ca0132_set_vipsource(struct hda_codec *codec, int val)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 
// C: 	if (spec->dsp_state != DSP_DOWNLOADED)
// C: 		return 0;
// C: 
// C: 	/* if CrystalVoice if off, vipsource should be 0 */
// C: 	if (!spec->effects_switch[CRYSTAL_VOICE - EFFECT_START_NID] ||
// C: 	    (val == 0)) {
// C: 		chipio_set_control_param(codec, CONTROL_PARAM_VIP_SOURCE, 0);
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICIN1, SR_96_000);
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICOUT1, SR_96_000);
// C: 		if (spec->cur_mic_type == DIGITAL_MIC)
// C: 			tmp = FLOAT_TWO;
// C: 		else
// C: 			tmp = FLOAT_ONE;
// C: 		dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 		tmp = FLOAT_ZERO;
// C: 		dspio_set_uint_param(codec, 0x80, 0x05, tmp);
// C: 	} else {
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICIN1, SR_16_000);
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICOUT1, SR_16_000);
// C: 		if (spec->cur_mic_type == DIGITAL_MIC)
// C: 			tmp = FLOAT_TWO;
// C: 		else
// C: 			tmp = FLOAT_ONE;
// C: 		dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 		tmp = FLOAT_ONE;
// C: 		dspio_set_uint_param(codec, 0x80, 0x05, tmp);
// C: 		msleep(20);
// C: 		chipio_set_control_param(codec, CONTROL_PARAM_VIP_SOURCE, val);
// C: 	}
// C: 
// C: 	return 1;
// C: }
// C: 
// C: static int ca0132_alt_set_vipsource(struct hda_codec *codec, int val)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 
// C: 	if (spec->dsp_state != DSP_DOWNLOADED)
// C: 		return 0;
// C: 
// C: 	codec_dbg(codec, "%s\\n", __func__);
// C: 
// C: 	chipio_set_stream_control(codec, 0x03, 0);
// C: 	chipio_set_stream_control(codec, 0x04, 0);
// C: 
// C: 	/* if CrystalVoice is off, vipsource should be 0 */
// C: 	if (!spec->effects_switch[CRYSTAL_VOICE - EFFECT_START_NID] ||
// C: 	    (val == 0) || spec->in_enum_val == REAR_LINE_IN) {
// C: 		codec_dbg(codec, "%s: off.", __func__);
// C: 		chipio_set_control_param(codec, CONTROL_PARAM_VIP_SOURCE, 0);
// C: 
// C: 		tmp = FLOAT_ZERO;
// C: 		dspio_set_uint_param(codec, 0x80, 0x05, tmp);
// C: 
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICIN1, SR_96_000);
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICOUT1, SR_96_000);
// C: 		if (ca0132_quirk(spec) == QUIRK_R3DI)
// C: 			chipio_set_conn_rate(codec, 0x0F, SR_96_000);
// C: 
// C: 
// C: 		if (spec->in_enum_val == REAR_LINE_IN)
// C: 			tmp = FLOAT_ZERO;
// C: 		else {
// C: 			if (ca0132_quirk(spec) == QUIRK_SBZ)
// C: 				tmp = FLOAT_THREE;
// C: 			else
// C: 				tmp = FLOAT_ONE;
// C: 		}
// C: 
// C: 		dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 
// C: 	} else {
// C: 		codec_dbg(codec, "%s: on.", __func__);
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICIN1, SR_16_000);
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICOUT1, SR_16_000);
// C: 		if (ca0132_quirk(spec) == QUIRK_R3DI)
// C: 			chipio_set_conn_rate(codec, 0x0F, SR_16_000);
// C: 
// C: 		if (spec->effects_switch[VOICE_FOCUS - EFFECT_START_NID])
// C: 			tmp = FLOAT_TWO;
// C: 		else
// C: 			tmp = FLOAT_ONE;
// C: 		dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 
// C: 		tmp = FLOAT_ONE;
// C: 		dspio_set_uint_param(codec, 0x80, 0x05, tmp);
// C: 
// C: 		msleep(20);
// C: 		chipio_set_control_param(codec, CONTROL_PARAM_VIP_SOURCE, val);
// C: 	}
// C: 
// C: 	chipio_set_stream_control(codec, 0x03, 1);
// C: 	chipio_set_stream_control(codec, 0x04, 1);
// C: 
// C: 	return 1;
// C: }
// C: 
// C: /*
// C:  * Select the active microphone.
// C:  * If autodetect is enabled, mic will be selected based on jack detection.
// C:  * If jack inserted, ext.mic will be selected, else built-in mic
// C:  * If autodetect is disabled, mic will be selected based on selection.
// C:  */
// C: static int ca0132_select_mic(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int jack_present;
// C: 	int auto_jack;
// C: 
// C: 	codec_dbg(codec, "ca0132_select_mic\\n");
// C: 
// C: 	CLASS(snd_hda_power_pm, pm)(codec);
// C: 
// C: 	auto_jack = spec->vnode_lswitch[VNID_AMIC1_ASEL - VNODE_START_NID];
// C: 
// C: 	if (auto_jack)
// C: 		jack_present = snd_hda_jack_detect(codec, spec->unsol_tag_amic1);
// C: 	else
// C: 		jack_present =
// C: 			spec->vnode_lswitch[VNID_AMIC1_SEL - VNODE_START_NID];
// C: 
// C: 	if (jack_present)
// C: 		spec->cur_mic_type = LINE_MIC_IN;
// C: 	else
// C: 		spec->cur_mic_type = DIGITAL_MIC;
// C: 
// C: 	if (spec->cur_mic_type == DIGITAL_MIC) {
// C: 		/* enable digital Mic */
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_DMIC, SR_32_000);
// C: 		ca0132_set_dmic(codec, 1);
// C: 		ca0132_mic_boost_set(codec, 0);
// C: 		/* set voice focus */
// C: 		ca0132_effects_set(codec, VOICE_FOCUS,
// C: 				   spec->effects_switch
// C: 				   [VOICE_FOCUS - EFFECT_START_NID]);
// C: 	} else {
// C: 		/* disable digital Mic */
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_DMIC, SR_96_000);
// C: 		ca0132_set_dmic(codec, 0);
// C: 		ca0132_mic_boost_set(codec, spec->cur_mic_boost);
// C: 		/* disable voice focus */
// C: 		ca0132_effects_set(codec, VOICE_FOCUS, 0);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * Select the active input.
// C:  * Mic detection isn't used, because it's kind of pointless on the SBZ.
// C:  * The front mic has no jack-detection, so the only way to switch to it
// C:  * is to do it manually in alsamixer.
// C:  */
// C: static int ca0132_alt_select_in(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 
// C: 	codec_dbg(codec, "%s\\n", __func__);
// C: 
// C: 	CLASS(snd_hda_power_pm, pm)(codec);
// C: 
// C: 	chipio_set_stream_control(codec, 0x03, 0);
// C: 	chipio_set_stream_control(codec, 0x04, 0);
// C: 
// C: 	spec->cur_mic_type = spec->in_enum_val;
// C: 
// C: 	switch (spec->cur_mic_type) {
// C: 	case REAR_MIC:
// C: 		switch (ca0132_quirk(spec)) {
// C: 		case QUIRK_SBZ:
// C: 		case QUIRK_R3D:
// C: 			ca0113_mmio_gpio_set(codec, 0, false);
// C: 			tmp = FLOAT_THREE;
// C: 			break;
// C: 		case QUIRK_ZXR:
// C: 			tmp = FLOAT_THREE;
// C: 			break;
// C: 		case QUIRK_R3DI:
// C: 			r3di_gpio_mic_set(codec, R3DI_REAR_MIC);
// C: 			tmp = FLOAT_ONE;
// C: 			break;
// C: 		case QUIRK_AE5:
// C: 			ca0113_mmio_command_set(codec, 0x30, 0x28, 0x00);
// C: 			tmp = FLOAT_THREE;
// C: 			break;
// C: 		case QUIRK_AE7:
// C: 			ca0113_mmio_command_set(codec, 0x30, 0x28, 0x00);
// C: 			tmp = FLOAT_THREE;
// C: 			chipio_set_conn_rate(codec, MEM_CONNID_MICIN2,
// C: 					SR_96_000);
// C: 			chipio_set_conn_rate(codec, MEM_CONNID_MICOUT2,
// C: 					SR_96_000);
// C: 			dspio_set_uint_param(codec, 0x80, 0x01, FLOAT_ZERO);
// C: 			break;
// C: 		default:
// C: 			tmp = FLOAT_ONE;
// C: 			break;
// C: 		}
// C: 
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICIN1, SR_96_000);
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICOUT1, SR_96_000);
// C: 		if (ca0132_quirk(spec) == QUIRK_R3DI)
// C: 			chipio_set_conn_rate(codec, 0x0F, SR_96_000);
// C: 
// C: 		dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 
// C: 		chipio_set_stream_control(codec, 0x03, 1);
// C: 		chipio_set_stream_control(codec, 0x04, 1);
// C: 		switch (ca0132_quirk(spec)) {
// C: 		case QUIRK_SBZ:
// C: 			chipio_write(codec, 0x18B098, 0x0000000C);
// C: 			chipio_write(codec, 0x18B09C, 0x0000000C);
// C: 			break;
// C: 		case QUIRK_ZXR:
// C: 			chipio_write(codec, 0x18B098, 0x0000000C);
// C: 			chipio_write(codec, 0x18B09C, 0x000000CC);
// C: 			break;
// C: 		case QUIRK_AE5:
// C: 			chipio_write(codec, 0x18B098, 0x0000000C);
// C: 			chipio_write(codec, 0x18B09C, 0x0000004C);
// C: 			break;
// C: 		default:
// C: 			break;
// C: 		}
// C: 		ca0132_alt_mic_boost_set(codec, spec->mic_boost_enum_val);
// C: 		break;
// C: 	case REAR_LINE_IN:
// C: 		ca0132_mic_boost_set(codec, 0);
// C: 		switch (ca0132_quirk(spec)) {
// C: 		case QUIRK_SBZ:
// C: 		case QUIRK_R3D:
// C: 			ca0113_mmio_gpio_set(codec, 0, false);
// C: 			break;
// C: 		case QUIRK_R3DI:
// C: 			r3di_gpio_mic_set(codec, R3DI_REAR_MIC);
// C: 			break;
// C: 		case QUIRK_AE5:
// C: 			ca0113_mmio_command_set(codec, 0x30, 0x28, 0x00);
// C: 			break;
// C: 		case QUIRK_AE7:
// C: 			ca0113_mmio_command_set(codec, 0x30, 0x28, 0x3f);
// C: 			chipio_set_conn_rate(codec, MEM_CONNID_MICIN2,
// C: 					SR_96_000);
// C: 			chipio_set_conn_rate(codec, MEM_CONNID_MICOUT2,
// C: 					SR_96_000);
// C: 			dspio_set_uint_param(codec, 0x80, 0x01, FLOAT_ZERO);
// C: 			break;
// C: 		default:
// C: 			break;
// C: 		}
// C: 
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICIN1, SR_96_000);
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICOUT1, SR_96_000);
// C: 		if (ca0132_quirk(spec) == QUIRK_R3DI)
// C: 			chipio_set_conn_rate(codec, 0x0F, SR_96_000);
// C: 
// C: 		if (ca0132_quirk(spec) == QUIRK_AE7)
// C: 			tmp = FLOAT_THREE;
// C: 		else
// C: 			tmp = FLOAT_ZERO;
// C: 		dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 
// C: 		switch (ca0132_quirk(spec)) {
// C: 		case QUIRK_SBZ:
// C: 		case QUIRK_AE5:
// C: 			chipio_write(codec, 0x18B098, 0x00000000);
// C: 			chipio_write(codec, 0x18B09C, 0x00000000);
// C: 			break;
// C: 		default:
// C: 			break;
// C: 		}
// C: 		chipio_set_stream_control(codec, 0x03, 1);
// C: 		chipio_set_stream_control(codec, 0x04, 1);
// C: 		break;
// C: 	case FRONT_MIC:
// C: 		switch (ca0132_quirk(spec)) {
// C: 		case QUIRK_SBZ:
// C: 		case QUIRK_R3D:
// C: 			ca0113_mmio_gpio_set(codec, 0, true);
// C: 			ca0113_mmio_gpio_set(codec, 5, false);
// C: 			tmp = FLOAT_THREE;
// C: 			break;
// C: 		case QUIRK_R3DI:
// C: 			r3di_gpio_mic_set(codec, R3DI_FRONT_MIC);
// C: 			tmp = FLOAT_ONE;
// C: 			break;
// C: 		case QUIRK_AE5:
// C: 			ca0113_mmio_command_set(codec, 0x30, 0x28, 0x3f);
// C: 			tmp = FLOAT_THREE;
// C: 			break;
// C: 		default:
// C: 			tmp = FLOAT_ONE;
// C: 			break;
// C: 		}
// C: 
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICIN1, SR_96_000);
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_MICOUT1, SR_96_000);
// C: 		if (ca0132_quirk(spec) == QUIRK_R3DI)
// C: 			chipio_set_conn_rate(codec, 0x0F, SR_96_000);
// C: 
// C: 		dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 
// C: 		chipio_set_stream_control(codec, 0x03, 1);
// C: 		chipio_set_stream_control(codec, 0x04, 1);
// C: 
// C: 		switch (ca0132_quirk(spec)) {
// C: 		case QUIRK_SBZ:
// C: 			chipio_write(codec, 0x18B098, 0x0000000C);
// C: 			chipio_write(codec, 0x18B09C, 0x000000CC);
// C: 			break;
// C: 		case QUIRK_AE5:
// C: 			chipio_write(codec, 0x18B098, 0x0000000C);
// C: 			chipio_write(codec, 0x18B09C, 0x0000004C);
// C: 			break;
// C: 		default:
// C: 			break;
// C: 		}
// C: 		ca0132_alt_mic_boost_set(codec, spec->mic_boost_enum_val);
// C: 		break;
// C: 	}
// C: 	ca0132_cvoice_switch_set(codec);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * Check if VNODE settings take effect immediately.
// C:  */
// C: static bool ca0132_is_vnode_effective(struct hda_codec *codec,
// C: 				     hda_nid_t vnid,
// C: 				     hda_nid_t *shared_nid)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid;
// C: 
// C: 	switch (vnid) {
// C: 	case VNID_SPK:
// C: 		nid = spec->shared_out_nid;
// C: 		break;
// C: 	case VNID_MIC:
// C: 		nid = spec->shared_mic_nid;
// C: 		break;
// C: 	default:
// C: 		return false;
// C: 	}
// C: 
// C: 	if (shared_nid)
// C: 		*shared_nid = nid;
// C: 
// C: 	return true;
// C: }
// C: 
// C: /*
// C: * The following functions are control change helpers.
// C: * They return 0 if no changed.  Return 1 if changed.
// C: */
// C: static int ca0132_voicefx_set(struct hda_codec *codec, int enable)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 
// C: 	/* based on CrystalVoice state to enable VoiceFX. */
// C: 	if (enable) {
// C: 		tmp = spec->effects_switch[CRYSTAL_VOICE - EFFECT_START_NID] ?
// C: 			FLOAT_ONE : FLOAT_ZERO;
// C: 	} else {
// C: 		tmp = FLOAT_ZERO;
// C: 	}
// C: 
// C: 	dspio_set_uint_param(codec, ca0132_voicefx.mid,
// C: 			     ca0132_voicefx.reqs[0], tmp);
// C: 
// C: 	return 1;
// C: }
// C: 
// C: /*
// C:  * Set the effects parameters
// C:  */
// C: static int ca0132_effects_set(struct hda_codec *codec, hda_nid_t nid, long val)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int on, tmp, channel_cfg;
// C: 	int num_fx = OUT_EFFECTS_COUNT + IN_EFFECTS_COUNT;
// C: 	int err = 0;
// C: 	int idx = nid - EFFECT_START_NID;
// C: 
// C: 	if ((idx < 0) || (idx >= num_fx))
// C: 		return 0; /* no changed */
// C: 
// C: 	/* for out effect, qualify with PE */
// C: 	if ((nid >= OUT_EFFECT_START_NID) && (nid < OUT_EFFECT_END_NID)) {
// C: 		/* if PE if off, turn off out effects. */
// C: 		if (!spec->effects_switch[PLAY_ENHANCEMENT - EFFECT_START_NID])
// C: 			val = 0;
// C: 		if (spec->cur_out_type == SPEAKER_OUT && nid == X_BASS) {
// C: 			channel_cfg = spec->channel_cfg_val;
// C: 			if (channel_cfg != SPEAKER_CHANNELS_2_0 &&
// C: 					channel_cfg != SPEAKER_CHANNELS_4_0)
// C: 				val = 0;
// C: 		}
// C: 	}
// C: 
// C: 	/* for in effect, qualify with CrystalVoice */
// C: 	if ((nid >= IN_EFFECT_START_NID) && (nid < IN_EFFECT_END_NID)) {
// C: 		/* if CrystalVoice if off, turn off in effects. */
// C: 		if (!spec->effects_switch[CRYSTAL_VOICE - EFFECT_START_NID])
// C: 			val = 0;
// C: 
// C: 		/* Voice Focus applies to 2-ch Mic, Digital Mic */
// C: 		if ((nid == VOICE_FOCUS) && (spec->cur_mic_type != DIGITAL_MIC))
// C: 			val = 0;
// C: 
// C: 		/* If Voice Focus on SBZ, set to two channel. */
// C: 		if ((nid == VOICE_FOCUS) && ca0132_use_pci_mmio(spec)
// C: 				&& (spec->cur_mic_type != REAR_LINE_IN)) {
// C: 			if (spec->effects_switch[CRYSTAL_VOICE -
// C: 						 EFFECT_START_NID]) {
// C: 
// C: 				if (spec->effects_switch[VOICE_FOCUS -
// C: 							 EFFECT_START_NID]) {
// C: 					tmp = FLOAT_TWO;
// C: 					val = 1;
// C: 				} else
// C: 					tmp = FLOAT_ONE;
// C: 
// C: 				dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 			}
// C: 		}
// C: 		/*
// C: 		 * For SBZ noise reduction, there's an extra command
// C: 		 * to module ID 0x47. No clue why.
// C: 		 */
// C: 		if ((nid == NOISE_REDUCTION) && ca0132_use_pci_mmio(spec)
// C: 				&& (spec->cur_mic_type != REAR_LINE_IN)) {
// C: 			if (spec->effects_switch[CRYSTAL_VOICE -
// C: 						 EFFECT_START_NID]) {
// C: 				if (spec->effects_switch[NOISE_REDUCTION -
// C: 							 EFFECT_START_NID])
// C: 					tmp = FLOAT_ONE;
// C: 				else
// C: 					tmp = FLOAT_ZERO;
// C: 			} else
// C: 				tmp = FLOAT_ZERO;
// C: 
// C: 			dspio_set_uint_param(codec, 0x47, 0x00, tmp);
// C: 		}
// C: 
// C: 		/* If rear line in disable effects. */
// C: 		if (ca0132_use_alt_functions(spec) &&
// C: 				spec->in_enum_val == REAR_LINE_IN)
// C: 			val = 0;
// C: 	}
// C: 
// C: 	codec_dbg(codec, "ca0132_effect_set: nid=0x%x, val=%ld\\n",
// C: 		    nid, val);
// C: 
// C: 	on = (val == 0) ? FLOAT_ZERO : FLOAT_ONE;
// C: 	err = dspio_set_uint_param(codec, ca0132_effects[idx].mid,
// C: 				   ca0132_effects[idx].reqs[0], on);
// C: 
// C: 	if (err < 0)
// C: 		return 0; /* no changed */
// C: 
// C: 	return 1;
// C: }
// C: 
// C: /*
// C:  * Turn on/off Playback Enhancements
// C:  */
// C: static int ca0132_pe_switch_set(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid;
// C: 	int i, ret = 0;
// C: 
// C: 	codec_dbg(codec, "ca0132_pe_switch_set: val=%ld\\n",
// C: 		    spec->effects_switch[PLAY_ENHANCEMENT - EFFECT_START_NID]);
// C: 
// C: 	if (ca0132_use_alt_functions(spec))
// C: 		ca0132_alt_select_out(codec);
// C: 
// C: 	i = OUT_EFFECT_START_NID - EFFECT_START_NID;
// C: 	nid = OUT_EFFECT_START_NID;
// C: 	/* PE affects all out effects */
// C: 	for (; nid < OUT_EFFECT_END_NID; nid++, i++)
// C: 		ret |= ca0132_effects_set(codec, nid, spec->effects_switch[i]);
// C: 
// C: 	return ret;
// C: }
// C: 
// C: /* Check if Mic1 is streaming, if so, stop streaming */
// C: static int stop_mic1(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int oldval = snd_hda_codec_read(codec, spec->adcs[0], 0,
// C: 						 AC_VERB_GET_CONV, 0);
// C: 	if (oldval != 0)
// C: 		snd_hda_codec_write(codec, spec->adcs[0], 0,
// C: 				    AC_VERB_SET_CHANNEL_STREAMID,
// C: 				    0);
// C: 	return oldval;
// C: }
// C: 
// C: /* Resume Mic1 streaming if it was stopped. */
// C: static void resume_mic1(struct hda_codec *codec, unsigned int oldval)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	/* Restore the previous stream and channel */
// C: 	if (oldval != 0)
// C: 		snd_hda_codec_write(codec, spec->adcs[0], 0,
// C: 				    AC_VERB_SET_CHANNEL_STREAMID,
// C: 				    oldval);
// C: }
// C: 
// C: /*
// C:  * Turn on/off CrystalVoice
// C:  */
// C: static int ca0132_cvoice_switch_set(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid;
// C: 	int i, ret = 0;
// C: 	unsigned int oldval;
// C: 
// C: 	codec_dbg(codec, "ca0132_cvoice_switch_set: val=%ld\\n",
// C: 		    spec->effects_switch[CRYSTAL_VOICE - EFFECT_START_NID]);
// C: 
// C: 	i = IN_EFFECT_START_NID - EFFECT_START_NID;
// C: 	nid = IN_EFFECT_START_NID;
// C: 	/* CrystalVoice affects all in effects */
// C: 	for (; nid < IN_EFFECT_END_NID; nid++, i++)
// C: 		ret |= ca0132_effects_set(codec, nid, spec->effects_switch[i]);
// C: 
// C: 	/* including VoiceFX */
// C: 	ret |= ca0132_voicefx_set(codec, (spec->voicefx_val ? 1 : 0));
// C: 
// C: 	/* set correct vipsource */
// C: 	oldval = stop_mic1(codec);
// C: 	if (ca0132_use_alt_functions(spec))
// C: 		ret |= ca0132_alt_set_vipsource(codec, 1);
// C: 	else
// C: 		ret |= ca0132_set_vipsource(codec, 1);
// C: 	resume_mic1(codec, oldval);
// C: 	return ret;
// C: }
// C: 
// C: static int ca0132_mic_boost_set(struct hda_codec *codec, long val)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int ret = 0;
// C: 
// C: 	if (val) /* on */
// C: 		ret = snd_hda_codec_amp_update(codec, spec->input_pins[0], 0,
// C: 					HDA_INPUT, 0, HDA_AMP_VOLMASK, 3);
// C: 	else /* off */
// C: 		ret = snd_hda_codec_amp_update(codec, spec->input_pins[0], 0,
// C: 					HDA_INPUT, 0, HDA_AMP_VOLMASK, 0);
// C: 
// C: 	return ret;
// C: }
// C: 
// C: static int ca0132_alt_mic_boost_set(struct hda_codec *codec, long val)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int ret = 0;
// C: 
// C: 	ret = snd_hda_codec_amp_update(codec, spec->input_pins[0], 0,
// C: 				HDA_INPUT, 0, HDA_AMP_VOLMASK, val);
// C: 	return ret;
// C: }
// C: 
// C: static int ae5_headphone_gain_set(struct hda_codec *codec, long val)
// C: {
// C: 	unsigned int i;
// C: 
// C: 	for (i = 0; i < 4; i++)
// C: 		ca0113_mmio_command_set(codec, 0x48, 0x11 + i,
// C: 				ae5_headphone_gain_presets[val].vals[i]);
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * gpio pin 1 is a relay that switches on/off, apparently setting the headphone
// C:  * amplifier to handle a 600 ohm load.
// C:  */
// C: static int zxr_headphone_gain_set(struct hda_codec *codec, long val)
// C: {
// C: 	ca0113_mmio_gpio_set(codec, 1, val);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * Manual output selection (HP/Speaker Playback Switch or alt Output Select)
// C:  * is meaningful only when HP/Speaker auto-detect is disabled, since the
// C:  * select_out path always prefers jack presence when auto-detect is on. When
// C:  * the user explicitly chooses an output, turn auto-detect off so the manual
// C:  * choice actually takes effect, and notify userspace so the auto-detect
// C:  * control reflects the new state.
// C:  */
// C: static void ca0132_disable_hp_auto_detect(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	struct snd_kcontrol *kctl;
// C: 
// C: 	if (!spec->vnode_lswitch[VNID_HP_ASEL - VNODE_START_NID])
// C: 		return;
// C: 
// C: 	spec->vnode_lswitch[VNID_HP_ASEL - VNODE_START_NID] = 0;
// C: 	kctl = snd_hda_find_mixer_ctl(codec,
// C: 				      "HP/Speaker Auto Detect Playback Switch");
// C: 	if (kctl)
// C: 		snd_ctl_notify(codec->card, SNDRV_CTL_EVENT_MASK_VALUE,
// C: 			       &kctl->id);
// C: }
// C: 
// C: static int ca0132_vnode_switch_set(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	hda_nid_t shared_nid = 0;
// C: 	bool effective;
// C: 	int ret = 0;
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int auto_jack;
// C: 
// C: 	if (nid == VNID_HP_SEL) {
// C: 		ca0132_disable_hp_auto_detect(codec);
// C: 		if (ca0132_use_alt_functions(spec))
// C: 			ca0132_alt_select_out(codec);
// C: 		else
// C: 			ca0132_select_out(codec);
// C: 		return 1;
// C: 	}
// C: 
// C: 	if (nid == VNID_AMIC1_SEL) {
// C: 		auto_jack =
// C: 			spec->vnode_lswitch[VNID_AMIC1_ASEL - VNODE_START_NID];
// C: 		if (!auto_jack)
// C: 			ca0132_select_mic(codec);
// C: 		return 1;
// C: 	}
// C: 
// C: 	if (nid == VNID_HP_ASEL) {
// C: 		if (ca0132_use_alt_functions(spec))
// C: 			ca0132_alt_select_out(codec);
// C: 		else
// C: 			ca0132_select_out(codec);
// C: 		return 1;
// C: 	}
// C: 
// C: 	if (nid == VNID_AMIC1_ASEL) {
// C: 		ca0132_select_mic(codec);
// C: 		return 1;
// C: 	}
// C: 
// C: 	/* if effective conditions, then update hw immediately. */
// C: 	effective = ca0132_is_vnode_effective(codec, nid, &shared_nid);
// C: 	if (effective) {
// C: 		int dir = get_amp_direction(kcontrol);
// C: 		int ch = get_amp_channels(kcontrol);
// C: 		unsigned long pval;
// C: 
// C: 		guard(mutex)(&codec->control_mutex);
// C: 		pval = kcontrol->private_value;
// C: 		kcontrol->private_value = HDA_COMPOSE_AMP_VAL(shared_nid, ch,
// C: 								0, dir);
// C: 		ret = snd_hda_mixer_amp_switch_put(kcontrol, ucontrol);
// C: 		kcontrol->private_value = pval;
// C: 	}
// C: 
// C: 	return ret;
// C: }
// C: /* End of control change helpers. */
// C: 
// C: static void ca0132_alt_bass_redirection_xover_set(struct hda_codec *codec,
// C: 		long idx)
// C: {
// C: 	CLASS(snd_hda_power, pm)(codec);
// C: 
// C: 	dspio_set_param(codec, 0x96, 0x20, SPEAKER_BASS_REDIRECT_XOVER_FREQ,
// C: 			&(float_xbass_xover_lookup[idx]), sizeof(unsigned int));
// C: }
// C: 
// C: /*
// C:  * Below I've added controls to mess with the effect levels, I've only enabled
// C:  * them on the Sound Blaster Z, but they would probably also work on the
// C:  * Chromebook. I figured they were probably tuned specifically for it, and left
// C:  * out for a reason.
// C:  */
// C: 
// C: /* Sets DSP effect level from the sliders above the controls */
// C: 
// C: static int ca0132_alt_slider_ctl_set(struct hda_codec *codec, hda_nid_t nid,
// C: 			  const unsigned int *lookup, int idx)
// C: {
// C: 	int i = 0;
// C: 	unsigned int y;
// C: 	/*
// C: 	 * For X_BASS, req 2 is actually crossover freq instead of
// C: 	 * effect level
// C: 	 */
// C: 	if (nid == X_BASS)
// C: 		y = 2;
// C: 	else
// C: 		y = 1;
// C: 
// C: 	CLASS(snd_hda_power, pm)(codec);
// C: 	if (nid == XBASS_XOVER) {
// C: 		for (i = 0; i < OUT_EFFECTS_COUNT; i++)
// C: 			if (ca0132_effects[i].nid == X_BASS)
// C: 				break;
// C: 
// C: 		dspio_set_param(codec, ca0132_effects[i].mid, 0x20,
// C: 				ca0132_effects[i].reqs[1],
// C: 				&(lookup[idx - 1]), sizeof(unsigned int));
// C: 	} else {
// C: 		/* Find the actual effect structure */
// C: 		for (i = 0; i < OUT_EFFECTS_COUNT; i++)
// C: 			if (nid == ca0132_effects[i].nid)
// C: 				break;
// C: 
// C: 		dspio_set_param(codec, ca0132_effects[i].mid, 0x20,
// C: 				ca0132_effects[i].reqs[y],
// C: 				&(lookup[idx]), sizeof(unsigned int));
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_xbass_xover_slider_ctl_get(struct snd_kcontrol *kcontrol,
// C: 			  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	long *valp = ucontrol->value.integer.value;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 
// C: 	if (nid == BASS_REDIRECTION_XOVER)
// C: 		*valp = spec->bass_redirect_xover_freq;
// C: 	else
// C: 		*valp = spec->xbass_xover_freq;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_slider_ctl_get(struct snd_kcontrol *kcontrol,
// C: 			  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 	int idx = nid - OUT_EFFECT_START_NID;
// C: 
// C: 	*valp = spec->fx_ctl_val[idx];
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * The X-bass crossover starts at 10hz, so the min is 1. The
// C:  * frequency is set in multiples of 10.
// C:  */
// C: static int ca0132_alt_xbass_xover_slider_info(struct snd_kcontrol *kcontrol,
// C: 		struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = 1;
// C: 	uinfo->value.integer.min = 1;
// C: 	uinfo->value.integer.max = 100;
// C: 	uinfo->value.integer.step = 1;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_effect_slider_info(struct snd_kcontrol *kcontrol,
// C: 		struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	int chs = get_amp_channels(kcontrol);
// C: 
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = chs == 3 ? 2 : 1;
// C: 	uinfo->value.integer.min = 0;
// C: 	uinfo->value.integer.max = 100;
// C: 	uinfo->value.integer.step = 1;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_xbass_xover_slider_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 	long *cur_val;
// C: 	int idx;
// C: 
// C: 	if (nid == BASS_REDIRECTION_XOVER)
// C: 		cur_val = &spec->bass_redirect_xover_freq;
// C: 	else
// C: 		cur_val = &spec->xbass_xover_freq;
// C: 
// C: 	/* any change? */
// C: 	if (*cur_val == *valp)
// C: 		return 0;
// C: 
// C: 	*cur_val = *valp;
// C: 
// C: 	idx = *valp;
// C: 	if (nid == BASS_REDIRECTION_XOVER)
// C: 		ca0132_alt_bass_redirection_xover_set(codec, *cur_val);
// C: 	else
// C: 		ca0132_alt_slider_ctl_set(codec, nid, float_xbass_xover_lookup, idx);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_effect_slider_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 	int idx;
// C: 
// C: 	idx = nid - EFFECT_START_NID;
// C: 	/* any change? */
// C: 	if (spec->fx_ctl_val[idx] == *valp)
// C: 		return 0;
// C: 
// C: 	spec->fx_ctl_val[idx] = *valp;
// C: 
// C: 	idx = *valp;
// C: 	ca0132_alt_slider_ctl_set(codec, nid, float_zero_to_one_lookup, idx);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: 
// C: /*
// C:  * Mic Boost Enum for alternative ca0132 codecs. I didn't like that the original
// C:  * only has off or full 30 dB, and didn't like making a volume slider that has
// C:  * traditional 0-100 in alsamixer that goes in big steps. I like enum better.
// C:  */
// C: #define MIC_BOOST_NUM_OF_STEPS 4
// C: #define MIC_BOOST_ENUM_MAX_STRLEN 10
// C: 
// C: static int ca0132_alt_mic_boost_info(struct snd_kcontrol *kcontrol,
// C: 				 struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	const char *sfx = "dB";
// C: 	char namestr[SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
// C: 
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
// C: 	uinfo->count = 1;
// C: 	uinfo->value.enumerated.items = MIC_BOOST_NUM_OF_STEPS;
// C: 	if (uinfo->value.enumerated.item >= MIC_BOOST_NUM_OF_STEPS)
// C: 		uinfo->value.enumerated.item = MIC_BOOST_NUM_OF_STEPS - 1;
// C: 	snprintf(namestr, sizeof(namestr), "%d %s",
// C: 		 (uinfo->value.enumerated.item * 10), sfx);
// C: 	strscpy(uinfo->value.enumerated.name, namestr);
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_mic_boost_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	ucontrol->value.enumerated.item[0] = spec->mic_boost_enum_val;
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_mic_boost_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int sel = ucontrol->value.enumerated.item[0];
// C: 	unsigned int items = MIC_BOOST_NUM_OF_STEPS;
// C: 
// C: 	if (sel >= items)
// C: 		return 0;
// C: 
// C: 	codec_dbg(codec, "ca0132_alt_mic_boost: boost=%d\\n",
// C: 		    sel);
// C: 
// C: 	spec->mic_boost_enum_val = sel;
// C: 
// C: 	if (spec->in_enum_val != REAR_LINE_IN)
// C: 		ca0132_alt_mic_boost_set(codec, spec->mic_boost_enum_val);
// C: 
// C: 	return 1;
// C: }
// C: 
// C: /*
// C:  * Sound BlasterX AE-5 Headphone Gain Controls.
// C:  */
// C: #define AE5_HEADPHONE_GAIN_MAX 3
// C: static int ae5_headphone_gain_info(struct snd_kcontrol *kcontrol,
// C: 				 struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	const char *sfx = " Ohms)";
// C: 	char namestr[SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
// C: 
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
// C: 	uinfo->count = 1;
// C: 	uinfo->value.enumerated.items = AE5_HEADPHONE_GAIN_MAX;
// C: 	if (uinfo->value.enumerated.item >= AE5_HEADPHONE_GAIN_MAX)
// C: 		uinfo->value.enumerated.item = AE5_HEADPHONE_GAIN_MAX - 1;
// C: 	snprintf(namestr, sizeof(namestr), "%s %s",
// C: 		 ae5_headphone_gain_presets[uinfo->value.enumerated.item].name,
// C: 		 sfx);
// C: 	strscpy(uinfo->value.enumerated.name, namestr);
// C: 	return 0;
// C: }
// C: 
// C: static int ae5_headphone_gain_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	ucontrol->value.enumerated.item[0] = spec->ae5_headphone_gain_val;
// C: 	return 0;
// C: }
// C: 
// C: static int ae5_headphone_gain_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int sel = ucontrol->value.enumerated.item[0];
// C: 	unsigned int items = AE5_HEADPHONE_GAIN_MAX;
// C: 
// C: 	if (sel >= items)
// C: 		return 0;
// C: 
// C: 	codec_dbg(codec, "ae5_headphone_gain: boost=%d\\n",
// C: 		    sel);
// C: 
// C: 	spec->ae5_headphone_gain_val = sel;
// C: 
// C: 	if (spec->out_enum_val == HEADPHONE_OUT)
// C: 		ae5_headphone_gain_set(codec, spec->ae5_headphone_gain_val);
// C: 
// C: 	return 1;
// C: }
// C: 
// C: /*
// C:  * Sound BlasterX AE-5 sound filter enumerated control.
// C:  */
// C: #define AE5_SOUND_FILTER_MAX 3
// C: 
// C: static int ae5_sound_filter_info(struct snd_kcontrol *kcontrol,
// C: 				 struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	char namestr[SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
// C: 
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
// C: 	uinfo->count = 1;
// C: 	uinfo->value.enumerated.items = AE5_SOUND_FILTER_MAX;
// C: 	if (uinfo->value.enumerated.item >= AE5_SOUND_FILTER_MAX)
// C: 		uinfo->value.enumerated.item = AE5_SOUND_FILTER_MAX - 1;
// C: 	snprintf(namestr, sizeof(namestr), "%s",
// C: 		 ae5_filter_presets[uinfo->value.enumerated.item].name);
// C: 	strscpy(uinfo->value.enumerated.name, namestr);
// C: 	return 0;
// C: }
// C: 
// C: static int ae5_sound_filter_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	ucontrol->value.enumerated.item[0] = spec->ae5_filter_val;
// C: 	return 0;
// C: }
// C: 
// C: static int ae5_sound_filter_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int sel = ucontrol->value.enumerated.item[0];
// C: 	unsigned int items = AE5_SOUND_FILTER_MAX;
// C: 
// C: 	if (sel >= items)
// C: 		return 0;
// C: 
// C: 	codec_dbg(codec, "ae5_sound_filter: %s\\n",
// C: 			ae5_filter_presets[sel].name);
// C: 
// C: 	spec->ae5_filter_val = sel;
// C: 
// C: 	ca0113_mmio_command_set_type2(codec, 0x48, 0x07,
// C: 			ae5_filter_presets[sel].val);
// C: 
// C: 	return 1;
// C: }
// C: 
// C: /*
// C:  * Input Select Control for alternative ca0132 codecs. This exists because
// C:  * front microphone has no auto-detect, and we need a way to set the rear
// C:  * as line-in
// C:  */
// C: static int ca0132_alt_input_source_info(struct snd_kcontrol *kcontrol,
// C: 				 struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
// C: 	uinfo->count = 1;
// C: 	uinfo->value.enumerated.items = IN_SRC_NUM_OF_INPUTS;
// C: 	if (uinfo->value.enumerated.item >= IN_SRC_NUM_OF_INPUTS)
// C: 		uinfo->value.enumerated.item = IN_SRC_NUM_OF_INPUTS - 1;
// C: 	strscpy(uinfo->value.enumerated.name,
// C: 			in_src_str[uinfo->value.enumerated.item]);
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_input_source_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	ucontrol->value.enumerated.item[0] = spec->in_enum_val;
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_input_source_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int sel = ucontrol->value.enumerated.item[0];
// C: 	unsigned int items = IN_SRC_NUM_OF_INPUTS;
// C: 
// C: 	/*
// C: 	 * The AE-7 has no front microphone, so limit items to 2: rear mic and
// C: 	 * line-in.
// C: 	 */
// C: 	if (ca0132_quirk(spec) == QUIRK_AE7)
// C: 		items = 2;
// C: 
// C: 	if (sel >= items)
// C: 		return 0;
// C: 
// C: 	codec_dbg(codec, "ca0132_alt_input_select: sel=%d, preset=%s\\n",
// C: 		    sel, in_src_str[sel]);
// C: 
// C: 	spec->in_enum_val = sel;
// C: 
// C: 	ca0132_alt_select_in(codec);
// C: 
// C: 	return 1;
// C: }
// C: 
// C: /* Sound Blaster Z Output Select Control */
// C: static int ca0132_alt_output_select_get_info(struct snd_kcontrol *kcontrol,
// C: 				 struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
// C: 	uinfo->count = 1;
// C: 	uinfo->value.enumerated.items = NUM_OF_OUTPUTS;
// C: 	if (uinfo->value.enumerated.item >= NUM_OF_OUTPUTS)
// C: 		uinfo->value.enumerated.item = NUM_OF_OUTPUTS - 1;
// C: 	strscpy(uinfo->value.enumerated.name,
// C: 			out_type_str[uinfo->value.enumerated.item]);
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_output_select_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	ucontrol->value.enumerated.item[0] = spec->out_enum_val;
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_output_select_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int sel = ucontrol->value.enumerated.item[0];
// C: 	unsigned int items = NUM_OF_OUTPUTS;
// C: 
// C: 	if (sel >= items)
// C: 		return 0;
// C: 
// C: 	codec_dbg(codec, "ca0132_alt_output_select: sel=%d, preset=%s\\n",
// C: 		    sel, out_type_str[sel]);
// C: 
// C: 	spec->out_enum_val = sel;
// C: 
// C: 	ca0132_disable_hp_auto_detect(codec);
// C: 	ca0132_alt_select_out(codec);
// C: 
// C: 	return 1;
// C: }
// C: 
// C: /* Select surround output type: 2.1, 4.0, 4.1, or 5.1. */
// C: static int ca0132_alt_speaker_channel_cfg_get_info(struct snd_kcontrol *kcontrol,
// C: 				 struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	unsigned int items = SPEAKER_CHANNEL_CFG_COUNT;
// C: 
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
// C: 	uinfo->count = 1;
// C: 	uinfo->value.enumerated.items = items;
// C: 	if (uinfo->value.enumerated.item >= items)
// C: 		uinfo->value.enumerated.item = items - 1;
// C: 	strscpy(uinfo->value.enumerated.name,
// C: 			speaker_channel_cfgs[uinfo->value.enumerated.item].name);
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_speaker_channel_cfg_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	ucontrol->value.enumerated.item[0] = spec->channel_cfg_val;
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_speaker_channel_cfg_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int sel = ucontrol->value.enumerated.item[0];
// C: 	unsigned int items = SPEAKER_CHANNEL_CFG_COUNT;
// C: 
// C: 	if (sel >= items)
// C: 		return 0;
// C: 
// C: 	codec_dbg(codec, "ca0132_alt_speaker_channels: sel=%d, channels=%s\\n",
// C: 		    sel, speaker_channel_cfgs[sel].name);
// C: 
// C: 	spec->channel_cfg_val = sel;
// C: 
// C: 	if (spec->out_enum_val == SPEAKER_OUT)
// C: 		ca0132_alt_select_out(codec);
// C: 
// C: 	return 1;
// C: }
// C: 
// C: /*
// C:  * Smart Volume output setting control. Three different settings, Normal,
// C:  * which takes the value from the smart volume slider. The two others, loud
// C:  * and night, disregard the slider value and have uneditable values.
// C:  */
// C: #define NUM_OF_SVM_SETTINGS 3
// C: static const char *const out_svm_set_enum_str[3] = {"Normal", "Loud", "Night" };
// C: 
// C: static int ca0132_alt_svm_setting_info(struct snd_kcontrol *kcontrol,
// C: 				 struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
// C: 	uinfo->count = 1;
// C: 	uinfo->value.enumerated.items = NUM_OF_SVM_SETTINGS;
// C: 	if (uinfo->value.enumerated.item >= NUM_OF_SVM_SETTINGS)
// C: 		uinfo->value.enumerated.item = NUM_OF_SVM_SETTINGS - 1;
// C: 	strscpy(uinfo->value.enumerated.name,
// C: 			out_svm_set_enum_str[uinfo->value.enumerated.item]);
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_svm_setting_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	ucontrol->value.enumerated.item[0] = spec->smart_volume_setting;
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_svm_setting_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int sel = ucontrol->value.enumerated.item[0];
// C: 	unsigned int items = NUM_OF_SVM_SETTINGS;
// C: 	unsigned int idx = SMART_VOLUME - EFFECT_START_NID;
// C: 	unsigned int tmp;
// C: 
// C: 	if (sel >= items)
// C: 		return 0;
// C: 
// C: 	codec_dbg(codec, "ca0132_alt_svm_setting: sel=%d, preset=%s\\n",
// C: 		    sel, out_svm_set_enum_str[sel]);
// C: 
// C: 	spec->smart_volume_setting = sel;
// C: 
// C: 	switch (sel) {
// C: 	case 0:
// C: 		tmp = FLOAT_ZERO;
// C: 		break;
// C: 	case 1:
// C: 		tmp = FLOAT_ONE;
// C: 		break;
// C: 	case 2:
// C: 		tmp = FLOAT_TWO;
// C: 		break;
// C: 	default:
// C: 		tmp = FLOAT_ZERO;
// C: 		break;
// C: 	}
// C: 	/* Req 2 is the Smart Volume Setting req. */
// C: 	dspio_set_uint_param(codec, ca0132_effects[idx].mid,
// C: 			ca0132_effects[idx].reqs[2], tmp);
// C: 	return 1;
// C: }
// C: 
// C: /* Sound Blaster Z EQ preset controls */
// C: static int ca0132_alt_eq_preset_info(struct snd_kcontrol *kcontrol,
// C: 				 struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	unsigned int items = ARRAY_SIZE(ca0132_alt_eq_presets);
// C: 
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
// C: 	uinfo->count = 1;
// C: 	uinfo->value.enumerated.items = items;
// C: 	if (uinfo->value.enumerated.item >= items)
// C: 		uinfo->value.enumerated.item = items - 1;
// C: 	strscpy(uinfo->value.enumerated.name,
// C: 		ca0132_alt_eq_presets[uinfo->value.enumerated.item].name);
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_eq_preset_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	ucontrol->value.enumerated.item[0] = spec->eq_preset_val;
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_alt_eq_preset_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int i, err = 0;
// C: 	int sel = ucontrol->value.enumerated.item[0];
// C: 	unsigned int items = ARRAY_SIZE(ca0132_alt_eq_presets);
// C: 
// C: 	if (sel >= items)
// C: 		return 0;
// C: 
// C: 	codec_dbg(codec, "%s: sel=%d, preset=%s\\n", __func__, sel,
// C: 			ca0132_alt_eq_presets[sel].name);
// C: 	/*
// C: 	 * Idx 0 is default.
// C: 	 * Default needs to qualify with CrystalVoice state.
// C: 	 */
// C: 	for (i = 0; i < EQ_PRESET_MAX_PARAM_COUNT; i++) {
// C: 		err = dspio_set_uint_param(codec, ca0132_alt_eq_enum.mid,
// C: 				ca0132_alt_eq_enum.reqs[i],
// C: 				ca0132_alt_eq_presets[sel].vals[i]);
// C: 		if (err < 0)
// C: 			break;
// C: 	}
// C: 
// C: 	if (err >= 0)
// C: 		spec->eq_preset_val = sel;
// C: 
// C: 	return 1;
// C: }
// C: 
// C: static int ca0132_voicefx_info(struct snd_kcontrol *kcontrol,
// C: 				 struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	unsigned int items = ARRAY_SIZE(ca0132_voicefx_presets);
// C: 
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
// C: 	uinfo->count = 1;
// C: 	uinfo->value.enumerated.items = items;
// C: 	if (uinfo->value.enumerated.item >= items)
// C: 		uinfo->value.enumerated.item = items - 1;
// C: 	strscpy(uinfo->value.enumerated.name,
// C: 	       ca0132_voicefx_presets[uinfo->value.enumerated.item].name);
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_voicefx_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	ucontrol->value.enumerated.item[0] = spec->voicefx_val;
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_voicefx_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int i, err = 0;
// C: 	int sel = ucontrol->value.enumerated.item[0];
// C: 
// C: 	if (sel >= ARRAY_SIZE(ca0132_voicefx_presets))
// C: 		return 0;
// C: 
// C: 	codec_dbg(codec, "ca0132_voicefx_put: sel=%d, preset=%s\\n",
// C: 		    sel, ca0132_voicefx_presets[sel].name);
// C: 
// C: 	/*
// C: 	 * Idx 0 is default.
// C: 	 * Default needs to qualify with CrystalVoice state.
// C: 	 */
// C: 	for (i = 0; i < VOICEFX_MAX_PARAM_COUNT; i++) {
// C: 		err = dspio_set_uint_param(codec, ca0132_voicefx.mid,
// C: 				ca0132_voicefx.reqs[i],
// C: 				ca0132_voicefx_presets[sel].vals[i]);
// C: 		if (err < 0)
// C: 			break;
// C: 	}
// C: 
// C: 	if (err >= 0) {
// C: 		spec->voicefx_val = sel;
// C: 		/* enable voice fx */
// C: 		ca0132_voicefx_set(codec, (sel ? 1 : 0));
// C: 	}
// C: 
// C: 	return 1;
// C: }
// C: 
// C: static int ca0132_switch_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	int ch = get_amp_channels(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 
// C: 	/* vnode */
// C: 	if ((nid >= VNODE_START_NID) && (nid < VNODE_END_NID)) {
// C: 		if (ch & 1) {
// C: 			*valp = spec->vnode_lswitch[nid - VNODE_START_NID];
// C: 			valp++;
// C: 		}
// C: 		if (ch & 2) {
// C: 			*valp = spec->vnode_rswitch[nid - VNODE_START_NID];
// C: 			valp++;
// C: 		}
// C: 		return 0;
// C: 	}
// C: 
// C: 	/* effects, include PE and CrystalVoice */
// C: 	if ((nid >= EFFECT_START_NID) && (nid < EFFECT_END_NID)) {
// C: 		*valp = spec->effects_switch[nid - EFFECT_START_NID];
// C: 		return 0;
// C: 	}
// C: 
// C: 	/* mic boost */
// C: 	if (nid == spec->input_pins[0]) {
// C: 		*valp = spec->cur_mic_boost;
// C: 		return 0;
// C: 	}
// C: 
// C: 	if (nid == ZXR_HEADPHONE_GAIN) {
// C: 		*valp = spec->zxr_gain_set;
// C: 		return 0;
// C: 	}
// C: 
// C: 	if (nid == SPEAKER_FULL_RANGE_FRONT || nid == SPEAKER_FULL_RANGE_REAR) {
// C: 		*valp = spec->speaker_range_val[nid - SPEAKER_FULL_RANGE_FRONT];
// C: 		return 0;
// C: 	}
// C: 
// C: 	if (nid == BASS_REDIRECTION) {
// C: 		*valp = spec->bass_redirection_val;
// C: 		return 0;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_switch_put(struct snd_kcontrol *kcontrol,
// C: 			     struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	int ch = get_amp_channels(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 
// C: 	codec_dbg(codec, "ca0132_switch_put: nid=0x%x, val=%ld\\n",
// C: 		    nid, *valp);
// C: 
// C: 	CLASS(snd_hda_power, pm)(codec);
// C: 	/* vnode */
// C: 	if ((nid >= VNODE_START_NID) && (nid < VNODE_END_NID)) {
// C: 		if (ch & 1) {
// C: 			spec->vnode_lswitch[nid - VNODE_START_NID] = *valp;
// C: 			valp++;
// C: 		}
// C: 		if (ch & 2) {
// C: 			spec->vnode_rswitch[nid - VNODE_START_NID] = *valp;
// C: 			valp++;
// C: 		}
// C: 		return ca0132_vnode_switch_set(kcontrol, ucontrol);
// C: 	}
// C: 
// C: 	/* PE */
// C: 	if (nid == PLAY_ENHANCEMENT) {
// C: 		spec->effects_switch[nid - EFFECT_START_NID] = *valp;
// C: 		return ca0132_pe_switch_set(codec);
// C: 	}
// C: 
// C: 	/* CrystalVoice */
// C: 	if (nid == CRYSTAL_VOICE) {
// C: 		spec->effects_switch[nid - EFFECT_START_NID] = *valp;
// C: 		return ca0132_cvoice_switch_set(codec);
// C: 	}
// C: 
// C: 	/* out and in effects */
// C: 	if (((nid >= OUT_EFFECT_START_NID) && (nid < OUT_EFFECT_END_NID)) ||
// C: 	    ((nid >= IN_EFFECT_START_NID) && (nid < IN_EFFECT_END_NID))) {
// C: 		spec->effects_switch[nid - EFFECT_START_NID] = *valp;
// C: 		return ca0132_effects_set(codec, nid, *valp);
// C: 	}
// C: 
// C: 	/* mic boost */
// C: 	if (nid == spec->input_pins[0]) {
// C: 		spec->cur_mic_boost = *valp;
// C: 		if (ca0132_use_alt_functions(spec)) {
// C: 			if (spec->in_enum_val != REAR_LINE_IN)
// C: 				return ca0132_mic_boost_set(codec, *valp);
// C: 		} else {
// C: 			/* Mic boost does not apply to Digital Mic */
// C: 			if (spec->cur_mic_type != DIGITAL_MIC)
// C: 				return ca0132_mic_boost_set(codec, *valp);
// C: 		}
// C: 
// C: 		return 1;
// C: 	}
// C: 
// C: 	if (nid == ZXR_HEADPHONE_GAIN) {
// C: 		spec->zxr_gain_set = *valp;
// C: 		if (spec->cur_out_type == HEADPHONE_OUT)
// C: 			return zxr_headphone_gain_set(codec, *valp);
// C: 		else
// C: 			return 0;
// C: 	}
// C: 
// C: 	if (nid == SPEAKER_FULL_RANGE_FRONT || nid == SPEAKER_FULL_RANGE_REAR) {
// C: 		spec->speaker_range_val[nid - SPEAKER_FULL_RANGE_FRONT] = *valp;
// C: 		if (spec->cur_out_type == SPEAKER_OUT)
// C: 			ca0132_alt_set_full_range_speaker(codec);
// C: 
// C: 		return 0;
// C: 	}
// C: 
// C: 	if (nid == BASS_REDIRECTION) {
// C: 		spec->bass_redirection_val = *valp;
// C: 		if (spec->cur_out_type == SPEAKER_OUT)
// C: 			ca0132_alt_surround_set_bass_redirection(codec, *valp);
// C: 
// C: 		return 0;
// C: 	}
// C: 
// C: 	return 1;
// C: }
// C: 
// C: /*
// C:  * Volume related
// C:  */
// C: /*
// C:  * Sets the internal DSP decibel level to match the DAC for output, and the
// C:  * ADC for input. Currently only the SBZ sets dsp capture volume level, and
// C:  * all alternative codecs set DSP playback volume.
// C:  */
// C: static void ca0132_alt_dsp_volume_put(struct hda_codec *codec, hda_nid_t nid)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int dsp_dir;
// C: 	unsigned int lookup_val;
// C: 
// C: 	if (nid == VNID_SPK)
// C: 		dsp_dir = DSP_VOL_OUT;
// C: 	else
// C: 		dsp_dir = DSP_VOL_IN;
// C: 
// C: 	lookup_val = spec->vnode_lvol[nid - VNODE_START_NID];
// C: 
// C: 	dspio_set_uint_param(codec,
// C: 		ca0132_alt_vol_ctls[dsp_dir].mid,
// C: 		ca0132_alt_vol_ctls[dsp_dir].reqs[0],
// C: 		float_vol_db_lookup[lookup_val]);
// C: 
// C: 	lookup_val = spec->vnode_rvol[nid - VNODE_START_NID];
// C: 
// C: 	dspio_set_uint_param(codec,
// C: 		ca0132_alt_vol_ctls[dsp_dir].mid,
// C: 		ca0132_alt_vol_ctls[dsp_dir].reqs[1],
// C: 		float_vol_db_lookup[lookup_val]);
// C: 
// C: 	dspio_set_uint_param(codec,
// C: 		ca0132_alt_vol_ctls[dsp_dir].mid,
// C: 		ca0132_alt_vol_ctls[dsp_dir].reqs[2], FLOAT_ZERO);
// C: }
// C: 
// C: static int ca0132_volume_info(struct snd_kcontrol *kcontrol,
// C: 			      struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	int ch = get_amp_channels(kcontrol);
// C: 	int dir = get_amp_direction(kcontrol);
// C: 	unsigned long pval;
// C: 	int err;
// C: 
// C: 	switch (nid) {
// C: 	case VNID_SPK:
// C: 		/* follow shared_out info */
// C: 		nid = spec->shared_out_nid;
// C: 		scoped_guard(mutex, &codec->control_mutex) {
// C: 			pval = kcontrol->private_value;
// C: 			kcontrol->private_value = HDA_COMPOSE_AMP_VAL(nid, ch, 0, dir);
// C: 			err = snd_hda_mixer_amp_volume_info(kcontrol, uinfo);
// C: 			kcontrol->private_value = pval;
// C: 		}
// C: 		break;
// C: 	case VNID_MIC:
// C: 		/* follow shared_mic info */
// C: 		nid = spec->shared_mic_nid;
// C: 		scoped_guard(mutex, &codec->control_mutex) {
// C: 			pval = kcontrol->private_value;
// C: 			kcontrol->private_value = HDA_COMPOSE_AMP_VAL(nid, ch, 0, dir);
// C: 			err = snd_hda_mixer_amp_volume_info(kcontrol, uinfo);
// C: 			kcontrol->private_value = pval;
// C: 		}
// C: 		break;
// C: 	default:
// C: 		err = snd_hda_mixer_amp_volume_info(kcontrol, uinfo);
// C: 	}
// C: 	return err;
// C: }
// C: 
// C: static int ca0132_volume_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	int ch = get_amp_channels(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 
// C: 	/* store the left and right volume */
// C: 	if (ch & 1) {
// C: 		*valp = spec->vnode_lvol[nid - VNODE_START_NID];
// C: 		valp++;
// C: 	}
// C: 	if (ch & 2) {
// C: 		*valp = spec->vnode_rvol[nid - VNODE_START_NID];
// C: 		valp++;
// C: 	}
// C: 	return 0;
// C: }
// C: 
// C: static int ca0132_volume_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	int ch = get_amp_channels(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 	hda_nid_t shared_nid = 0;
// C: 	bool effective;
// C: 	int changed = 1;
// C: 
// C: 	/* store the left and right volume */
// C: 	if (ch & 1) {
// C: 		spec->vnode_lvol[nid - VNODE_START_NID] = *valp;
// C: 		valp++;
// C: 	}
// C: 	if (ch & 2) {
// C: 		spec->vnode_rvol[nid - VNODE_START_NID] = *valp;
// C: 		valp++;
// C: 	}
// C: 
// C: 	/* if effective conditions, then update hw immediately. */
// C: 	effective = ca0132_is_vnode_effective(codec, nid, &shared_nid);
// C: 	if (effective) {
// C: 		int dir = get_amp_direction(kcontrol);
// C: 		unsigned long pval;
// C: 
// C: 		CLASS(snd_hda_power, pm)(codec);
// C: 		guard(mutex)(&codec->control_mutex);
// C: 		pval = kcontrol->private_value;
// C: 		kcontrol->private_value = HDA_COMPOSE_AMP_VAL(shared_nid, ch,
// C: 								0, dir);
// C: 		changed = snd_hda_mixer_amp_volume_put(kcontrol, ucontrol);
// C: 		kcontrol->private_value = pval;
// C: 	}
// C: 
// C: 	return changed;
// C: }
// C: 
// C: /*
// C:  * This function is the same as the one above, because using an if statement
// C:  * inside of the above volume control for the DSP volume would cause too much
// C:  * lag. This is a lot more smooth.
// C:  */
// C: static int ca0132_alt_volume_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	int ch = get_amp_channels(kcontrol);
// C: 	long *valp = ucontrol->value.integer.value;
// C: 	hda_nid_t vnid = 0;
// C: 
// C: 	switch (nid) {
// C: 	case 0x02:
// C: 		vnid = VNID_SPK;
// C: 		break;
// C: 	case 0x07:
// C: 		vnid = VNID_MIC;
// C: 		break;
// C: 	}
// C: 
// C: 	/* store the left and right volume */
// C: 	if (ch & 1) {
// C: 		spec->vnode_lvol[vnid - VNODE_START_NID] = *valp;
// C: 		valp++;
// C: 	}
// C: 	if (ch & 2) {
// C: 		spec->vnode_rvol[vnid - VNODE_START_NID] = *valp;
// C: 		valp++;
// C: 	}
// C: 
// C: 	CLASS(snd_hda_power, pm)(codec);
// C: 	ca0132_alt_dsp_volume_put(codec, vnid);
// C: 	guard(mutex)(&codec->control_mutex);
// C: 	return snd_hda_mixer_amp_volume_put(kcontrol, ucontrol);
// C: }
// C: 
// C: static int ca0132_volume_tlv(struct snd_kcontrol *kcontrol, int op_flag,
// C: 			     unsigned int size, unsigned int __user *tlv)
// C: {
// C: 	struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	hda_nid_t nid = get_amp_nid(kcontrol);
// C: 	int ch = get_amp_channels(kcontrol);
// C: 	int dir = get_amp_direction(kcontrol);
// C: 	unsigned long pval;
// C: 	int err;
// C: 
// C: 	switch (nid) {
// C: 	case VNID_SPK:
// C: 		/* follow shared_out tlv */
// C: 		nid = spec->shared_out_nid;
// C: 		scoped_guard(mutex, &codec->control_mutex) {
// C: 			pval = kcontrol->private_value;
// C: 			kcontrol->private_value = HDA_COMPOSE_AMP_VAL(nid, ch, 0, dir);
// C: 			err = snd_hda_mixer_amp_tlv(kcontrol, op_flag, size, tlv);
// C: 			kcontrol->private_value = pval;
// C: 		}
// C: 		break;
// C: 	case VNID_MIC:
// C: 		/* follow shared_mic tlv */
// C: 		nid = spec->shared_mic_nid;
// C: 		scoped_guard(mutex, &codec->control_mutex) {
// C: 			pval = kcontrol->private_value;
// C: 			kcontrol->private_value = HDA_COMPOSE_AMP_VAL(nid, ch, 0, dir);
// C: 			err = snd_hda_mixer_amp_tlv(kcontrol, op_flag, size, tlv);
// C: 			kcontrol->private_value = pval;
// C: 		}
// C: 		break;
// C: 	default:
// C: 		err = snd_hda_mixer_amp_tlv(kcontrol, op_flag, size, tlv);
// C: 	}
// C: 	return err;
// C: }
// C: 
// C: /* Add volume slider control for effect level */
// C: static int ca0132_alt_add_effect_slider(struct hda_codec *codec, hda_nid_t nid,
// C: 					const char *pfx, int dir)
// C: {
// C: 	char namestr[SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
// C: 	int type = dir ? HDA_INPUT : HDA_OUTPUT;
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_VOLUME_MONO(namestr, nid, 1, 0, type);
// C: 
// C: 	snprintf(namestr, sizeof(namestr), "FX: %s %s Volume", pfx, dirstr[dir]);
// C: 
// C: 	knew.tlv.c = NULL;
// C: 
// C: 	switch (nid) {
// C: 	case XBASS_XOVER:
// C: 		knew.info = ca0132_alt_xbass_xover_slider_info;
// C: 		knew.get = ca0132_alt_xbass_xover_slider_ctl_get;
// C: 		knew.put = ca0132_alt_xbass_xover_slider_put;
// C: 		break;
// C: 	default:
// C: 		knew.info = ca0132_alt_effect_slider_info;
// C: 		knew.get = ca0132_alt_slider_ctl_get;
// C: 		knew.put = ca0132_alt_effect_slider_put;
// C: 		knew.private_value =
// C: 			HDA_COMPOSE_AMP_VAL(nid, 1, 0, type);
// C: 		break;
// C: 	}
// C: 
// C: 	return snd_hda_ctl_add(codec, nid, snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: /*
// C:  * Added FX: prefix for the alternative codecs, because otherwise the surround
// C:  * effect would conflict with the Surround sound volume control. Also seems more
// C:  * clear as to what the switches do. Left alone for others.
// C:  */
// C: static int add_fx_switch(struct hda_codec *codec, hda_nid_t nid,
// C: 			 const char *pfx, int dir)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	char namestr[SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
// C: 	int type = dir ? HDA_INPUT : HDA_OUTPUT;
// C: 	struct snd_kcontrol_new knew =
// C: 		CA0132_CODEC_MUTE_MONO(namestr, nid, 1, type);
// C: 	/* If using alt_controls, add FX: prefix. But, don't add FX:
// C: 	 * prefix to OutFX or InFX enable controls.
// C: 	 */
// C: 	if (ca0132_use_alt_controls(spec) && (nid <= IN_EFFECT_END_NID))
// C: 		snprintf(namestr, sizeof(namestr), "FX: %s %s Switch", pfx, dirstr[dir]);
// C: 	else
// C: 		snprintf(namestr, sizeof(namestr), "%s %s Switch", pfx, dirstr[dir]);
// C: 
// C: 	return snd_hda_ctl_add(codec, nid, snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: static int add_voicefx(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_MUTE_MONO(ca0132_voicefx.name,
// C: 				    VOICEFX, 1, 0, HDA_INPUT);
// C: 	knew.info = ca0132_voicefx_info;
// C: 	knew.get = ca0132_voicefx_get;
// C: 	knew.put = ca0132_voicefx_put;
// C: 	return snd_hda_ctl_add(codec, VOICEFX, snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: /* Create the EQ Preset control */
// C: static int add_ca0132_alt_eq_presets(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_MUTE_MONO(ca0132_alt_eq_enum.name,
// C: 				    EQ_PRESET_ENUM, 1, 0, HDA_OUTPUT);
// C: 	knew.info = ca0132_alt_eq_preset_info;
// C: 	knew.get = ca0132_alt_eq_preset_get;
// C: 	knew.put = ca0132_alt_eq_preset_put;
// C: 	return snd_hda_ctl_add(codec, EQ_PRESET_ENUM,
// C: 				snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: /*
// C:  * Add enumerated control for the three different settings of the smart volume
// C:  * output effect. Normal just uses the slider value, and loud and night are
// C:  * their own things that ignore that value.
// C:  */
// C: static int ca0132_alt_add_svm_enum(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_MUTE_MONO("FX: Smart Volume Setting",
// C: 				    SMART_VOLUME_ENUM, 1, 0, HDA_OUTPUT);
// C: 	knew.info = ca0132_alt_svm_setting_info;
// C: 	knew.get = ca0132_alt_svm_setting_get;
// C: 	knew.put = ca0132_alt_svm_setting_put;
// C: 	return snd_hda_ctl_add(codec, SMART_VOLUME_ENUM,
// C: 				snd_ctl_new1(&knew, codec));
// C: 
// C: }
// C: 
// C: /*
// C:  * Create an Output Select enumerated control for codecs with surround
// C:  * out capabilities.
// C:  */
// C: static int ca0132_alt_add_output_enum(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_MUTE_MONO("Output Select",
// C: 				    OUTPUT_SOURCE_ENUM, 1, 0, HDA_OUTPUT);
// C: 	knew.info = ca0132_alt_output_select_get_info;
// C: 	knew.get = ca0132_alt_output_select_get;
// C: 	knew.put = ca0132_alt_output_select_put;
// C: 	return snd_hda_ctl_add(codec, OUTPUT_SOURCE_ENUM,
// C: 				snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: /*
// C:  * Add a control for selecting channel count on speaker output. Setting this
// C:  * allows the DSP to do bass redirection and channel upmixing on surround
// C:  * configurations.
// C:  */
// C: static int ca0132_alt_add_speaker_channel_cfg_enum(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_MUTE_MONO("Surround Channel Config",
// C: 				    SPEAKER_CHANNEL_CFG_ENUM, 1, 0, HDA_OUTPUT);
// C: 	knew.info = ca0132_alt_speaker_channel_cfg_get_info;
// C: 	knew.get = ca0132_alt_speaker_channel_cfg_get;
// C: 	knew.put = ca0132_alt_speaker_channel_cfg_put;
// C: 	return snd_hda_ctl_add(codec, SPEAKER_CHANNEL_CFG_ENUM,
// C: 				snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: /*
// C:  * Full range front stereo and rear surround switches. When these are set to
// C:  * full range, the lower frequencies from these channels are no longer
// C:  * redirected to the LFE channel.
// C:  */
// C: static int ca0132_alt_add_front_full_range_switch(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		CA0132_CODEC_MUTE_MONO("Full-Range Front Speakers",
// C: 				    SPEAKER_FULL_RANGE_FRONT, 1, HDA_OUTPUT);
// C: 
// C: 	return snd_hda_ctl_add(codec, SPEAKER_FULL_RANGE_FRONT,
// C: 				snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: static int ca0132_alt_add_rear_full_range_switch(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		CA0132_CODEC_MUTE_MONO("Full-Range Rear Speakers",
// C: 				    SPEAKER_FULL_RANGE_REAR, 1, HDA_OUTPUT);
// C: 
// C: 	return snd_hda_ctl_add(codec, SPEAKER_FULL_RANGE_REAR,
// C: 				snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: /*
// C:  * Bass redirection redirects audio below the crossover frequency to the LFE
// C:  * channel on speakers that are set as not being full-range. On configurations
// C:  * without an LFE channel, it does nothing. Bass redirection seems to be the
// C:  * replacement for X-Bass on configurations with an LFE channel.
// C:  */
// C: static int ca0132_alt_add_bass_redirection_crossover(struct hda_codec *codec)
// C: {
// C: 	const char *namestr = "Bass Redirection Crossover";
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_VOLUME_MONO(namestr, BASS_REDIRECTION_XOVER, 1, 0,
// C: 				HDA_OUTPUT);
// C: 
// C: 	knew.tlv.c = NULL;
// C: 	knew.info = ca0132_alt_xbass_xover_slider_info;
// C: 	knew.get = ca0132_alt_xbass_xover_slider_ctl_get;
// C: 	knew.put = ca0132_alt_xbass_xover_slider_put;
// C: 
// C: 	return snd_hda_ctl_add(codec, BASS_REDIRECTION_XOVER,
// C: 			snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: static int ca0132_alt_add_bass_redirection_switch(struct hda_codec *codec)
// C: {
// C: 	const char *namestr = "Bass Redirection";
// C: 	struct snd_kcontrol_new knew =
// C: 		CA0132_CODEC_MUTE_MONO(namestr, BASS_REDIRECTION, 1,
// C: 				HDA_OUTPUT);
// C: 
// C: 	return snd_hda_ctl_add(codec, BASS_REDIRECTION,
// C: 			snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: /*
// C:  * Create an Input Source enumerated control for the alternate ca0132 codecs
// C:  * because the front microphone has no auto-detect, and Line-in has to be set
// C:  * somehow.
// C:  */
// C: static int ca0132_alt_add_input_enum(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_MUTE_MONO("Input Source",
// C: 				    INPUT_SOURCE_ENUM, 1, 0, HDA_INPUT);
// C: 	knew.info = ca0132_alt_input_source_info;
// C: 	knew.get = ca0132_alt_input_source_get;
// C: 	knew.put = ca0132_alt_input_source_put;
// C: 	return snd_hda_ctl_add(codec, INPUT_SOURCE_ENUM,
// C: 				snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: /*
// C:  * Add mic boost enumerated control. Switches through 0dB to 30dB. This adds
// C:  * more control than the original mic boost, which is either full 30dB or off.
// C:  */
// C: static int ca0132_alt_add_mic_boost_enum(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_MUTE_MONO("Mic Boost Capture Switch",
// C: 				    MIC_BOOST_ENUM, 1, 0, HDA_INPUT);
// C: 	knew.info = ca0132_alt_mic_boost_info;
// C: 	knew.get = ca0132_alt_mic_boost_get;
// C: 	knew.put = ca0132_alt_mic_boost_put;
// C: 	return snd_hda_ctl_add(codec, MIC_BOOST_ENUM,
// C: 				snd_ctl_new1(&knew, codec));
// C: 
// C: }
// C: 
// C: /*
// C:  * Add headphone gain enumerated control for the AE-5. This switches between
// C:  * three modes, low, medium, and high. When non-headphone outputs are selected,
// C:  * it is automatically set to high. This is the same behavior as Windows.
// C:  */
// C: static int ae5_add_headphone_gain_enum(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_MUTE_MONO("AE-5: Headphone Gain",
// C: 				    AE5_HEADPHONE_GAIN_ENUM, 1, 0, HDA_OUTPUT);
// C: 	knew.info = ae5_headphone_gain_info;
// C: 	knew.get = ae5_headphone_gain_get;
// C: 	knew.put = ae5_headphone_gain_put;
// C: 	return snd_hda_ctl_add(codec, AE5_HEADPHONE_GAIN_ENUM,
// C: 				snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: /*
// C:  * Add sound filter enumerated control for the AE-5. This adds three different
// C:  * settings: Slow Roll Off, Minimum Phase, and Fast Roll Off. From what I've
// C:  * read into it, it changes the DAC's interpolation filter.
// C:  */
// C: static int ae5_add_sound_filter_enum(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		HDA_CODEC_MUTE_MONO("AE-5: Sound Filter",
// C: 				    AE5_SOUND_FILTER_ENUM, 1, 0, HDA_OUTPUT);
// C: 	knew.info = ae5_sound_filter_info;
// C: 	knew.get = ae5_sound_filter_get;
// C: 	knew.put = ae5_sound_filter_put;
// C: 	return snd_hda_ctl_add(codec, AE5_SOUND_FILTER_ENUM,
// C: 				snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: static int zxr_add_headphone_gain_switch(struct hda_codec *codec)
// C: {
// C: 	struct snd_kcontrol_new knew =
// C: 		CA0132_CODEC_MUTE_MONO("ZxR: 600 Ohm Gain",
// C: 				    ZXR_HEADPHONE_GAIN, 1, HDA_OUTPUT);
// C: 
// C: 	return snd_hda_ctl_add(codec, ZXR_HEADPHONE_GAIN,
// C: 				snd_ctl_new1(&knew, codec));
// C: }
// C: 
// C: /*
// C:  * Need to create follower controls for the alternate codecs that have surround
// C:  * capabilities.
// C:  */
// C: static const char * const ca0132_alt_follower_pfxs[] = {
// C: 	"Front", "Surround", "Center", "LFE", NULL,
// C: };
// C: 
// C: /*
// C:  * Also need special channel map, because the default one is incorrect.
// C:  * I think this has to do with the pin for rear surround being 0x11,
// C:  * and the center/lfe being 0x10. Usually the pin order is the opposite.
// C:  */
// C: static const struct snd_pcm_chmap_elem ca0132_alt_chmaps[] = {
// C: 	{ .channels = 2,
// C: 	  .map = { SNDRV_CHMAP_FL, SNDRV_CHMAP_FR } },
// C: 	{ .channels = 4,
// C: 	  .map = { SNDRV_CHMAP_FL, SNDRV_CHMAP_FR,
// C: 		   SNDRV_CHMAP_RL, SNDRV_CHMAP_RR } },
// C: 	{ .channels = 6,
// C: 	  .map = { SNDRV_CHMAP_FL, SNDRV_CHMAP_FR,
// C: 		   SNDRV_CHMAP_FC, SNDRV_CHMAP_LFE,
// C: 		   SNDRV_CHMAP_RL, SNDRV_CHMAP_RR } },
// C: 	{ }
// C: };
// C: 
// C: /* Add the correct chmap for streams with 6 channels. */
// C: static void ca0132_alt_add_chmap_ctls(struct hda_codec *codec)
// C: {
// C: 	int err = 0;
// C: 	struct hda_pcm *pcm;
// C: 
// C: 	list_for_each_entry(pcm, &codec->pcm_list_head, list) {
// C: 		struct hda_pcm_stream *hinfo =
// C: 			&pcm->stream[SNDRV_PCM_STREAM_PLAYBACK];
// C: 		struct snd_pcm_chmap *chmap;
// C: 		const struct snd_pcm_chmap_elem *elem;
// C: 
// C: 		elem = ca0132_alt_chmaps;
// C: 		if (hinfo->channels_max == 6) {
// C: 			err = snd_pcm_add_chmap_ctls(pcm->pcm,
// C: 					SNDRV_PCM_STREAM_PLAYBACK,
// C: 					elem, hinfo->channels_max, 0, &chmap);
// C: 			if (err < 0)
// C: 				codec_dbg(codec, "snd_pcm_add_chmap_ctls failed!");
// C: 		}
// C: 	}
// C: }
// C: 
// C: /*
// C:  * When changing Node IDs for Mixer Controls below, make sure to update
// C:  * Node IDs in ca0132_config() as well.
// C:  */
// C: static const struct snd_kcontrol_new ca0132_mixer[] = {
// C: 	CA0132_CODEC_VOL("Master Playback Volume", VNID_SPK, HDA_OUTPUT),
// C: 	CA0132_CODEC_MUTE("Master Playback Switch", VNID_SPK, HDA_OUTPUT),
// C: 	CA0132_CODEC_VOL("Capture Volume", VNID_MIC, HDA_INPUT),
// C: 	CA0132_CODEC_MUTE("Capture Switch", VNID_MIC, HDA_INPUT),
// C: 	HDA_CODEC_VOLUME("Analog-Mic2 Capture Volume", 0x08, 0, HDA_INPUT),
// C: 	HDA_CODEC_MUTE("Analog-Mic2 Capture Switch", 0x08, 0, HDA_INPUT),
// C: 	HDA_CODEC_VOLUME("What U Hear Capture Volume", 0x0a, 0, HDA_INPUT),
// C: 	HDA_CODEC_MUTE("What U Hear Capture Switch", 0x0a, 0, HDA_INPUT),
// C: 	CA0132_CODEC_MUTE_MONO("Mic1-Boost (30dB) Capture Switch",
// C: 			       0x12, 1, HDA_INPUT),
// C: 	CA0132_CODEC_MUTE_MONO("HP/Speaker Playback Switch",
// C: 			       VNID_HP_SEL, 1, HDA_OUTPUT),
// C: 	CA0132_CODEC_MUTE_MONO("AMic1/DMic Capture Switch",
// C: 			       VNID_AMIC1_SEL, 1, HDA_INPUT),
// C: 	CA0132_CODEC_MUTE_MONO("HP/Speaker Auto Detect Playback Switch",
// C: 			       VNID_HP_ASEL, 1, HDA_OUTPUT),
// C: 	CA0132_CODEC_MUTE_MONO("AMic1/DMic Auto Detect Capture Switch",
// C: 			       VNID_AMIC1_ASEL, 1, HDA_INPUT),
// C: 	{ } /* end */
// C: };
// C: 
// C: /*
// C:  * Desktop specific control mixer. Removes auto-detect for mic, and adds
// C:  * surround controls. Also sets both the Front Playback and Capture Volume
// C:  * controls to alt so they set the DSP's decibel level.
// C:  */
// C: static const struct snd_kcontrol_new desktop_mixer[] = {
// C: 	CA0132_ALT_CODEC_VOL("Front Playback Volume", 0x02, HDA_OUTPUT),
// C: 	CA0132_CODEC_MUTE("Front Playback Switch", VNID_SPK, HDA_OUTPUT),
// C: 	HDA_CODEC_VOLUME("Surround Playback Volume", 0x04, 0, HDA_OUTPUT),
// C: 	HDA_CODEC_MUTE("Surround Playback Switch", 0x04, 0, HDA_OUTPUT),
// C: 	HDA_CODEC_VOLUME_MONO("Center Playback Volume", 0x03, 1, 0, HDA_OUTPUT),
// C: 	HDA_CODEC_MUTE_MONO("Center Playback Switch", 0x03, 1, 0, HDA_OUTPUT),
// C: 	HDA_CODEC_VOLUME_MONO("LFE Playback Volume", 0x03, 2, 0, HDA_OUTPUT),
// C: 	HDA_CODEC_MUTE_MONO("LFE Playback Switch", 0x03, 2, 0, HDA_OUTPUT),
// C: 	CA0132_ALT_CODEC_VOL("Capture Volume", 0x07, HDA_INPUT),
// C: 	CA0132_CODEC_MUTE("Capture Switch", VNID_MIC, HDA_INPUT),
// C: 	HDA_CODEC_VOLUME("What U Hear Capture Volume", 0x0a, 0, HDA_INPUT),
// C: 	HDA_CODEC_MUTE("What U Hear Capture Switch", 0x0a, 0, HDA_INPUT),
// C: 	CA0132_CODEC_MUTE_MONO("HP/Speaker Auto Detect Playback Switch",
// C: 				VNID_HP_ASEL, 1, HDA_OUTPUT),
// C: 	{ } /* end */
// C: };
// C: 
// C: /*
// C:  * Same as the Sound Blaster Z, except doesn't use the alt volume for capture
// C:  * because it doesn't set decibel levels for the DSP for capture.
// C:  */
// C: static const struct snd_kcontrol_new r3di_mixer[] = {
// C: 	CA0132_ALT_CODEC_VOL("Front Playback Volume", 0x02, HDA_OUTPUT),
// C: 	CA0132_CODEC_MUTE("Front Playback Switch", VNID_SPK, HDA_OUTPUT),
// C: 	HDA_CODEC_VOLUME("Surround Playback Volume", 0x04, 0, HDA_OUTPUT),
// C: 	HDA_CODEC_MUTE("Surround Playback Switch", 0x04, 0, HDA_OUTPUT),
// C: 	HDA_CODEC_VOLUME_MONO("Center Playback Volume", 0x03, 1, 0, HDA_OUTPUT),
// C: 	HDA_CODEC_MUTE_MONO("Center Playback Switch", 0x03, 1, 0, HDA_OUTPUT),
// C: 	HDA_CODEC_VOLUME_MONO("LFE Playback Volume", 0x03, 2, 0, HDA_OUTPUT),
// C: 	HDA_CODEC_MUTE_MONO("LFE Playback Switch", 0x03, 2, 0, HDA_OUTPUT),
// C: 	CA0132_CODEC_VOL("Capture Volume", VNID_MIC, HDA_INPUT),
// C: 	CA0132_CODEC_MUTE("Capture Switch", VNID_MIC, HDA_INPUT),
// C: 	HDA_CODEC_VOLUME("What U Hear Capture Volume", 0x0a, 0, HDA_INPUT),
// C: 	HDA_CODEC_MUTE("What U Hear Capture Switch", 0x0a, 0, HDA_INPUT),
// C: 	CA0132_CODEC_MUTE_MONO("HP/Speaker Auto Detect Playback Switch",
// C: 				VNID_HP_ASEL, 1, HDA_OUTPUT),
// C: 	{ } /* end */
// C: };
// C: 
// C: static int ca0132_build_controls(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int i, num_fx, num_sliders;
// C: 	int err = 0;
// C: 
// C: 	/* Add Mixer controls */
// C: 	for (i = 0; i < spec->num_mixers; i++) {
// C: 		err = snd_hda_add_new_ctls(codec, spec->mixers[i]);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 	/* Setup vmaster with surround followers for desktop ca0132 devices */
// C: 	if (ca0132_use_alt_functions(spec)) {
// C: 		snd_hda_set_vmaster_tlv(codec, spec->dacs[0], HDA_OUTPUT,
// C: 					spec->tlv);
// C: 		snd_hda_add_vmaster(codec, "Master Playback Volume",
// C: 					spec->tlv, ca0132_alt_follower_pfxs,
// C: 					"Playback Volume", 0);
// C: 		err = __snd_hda_add_vmaster(codec, "Master Playback Switch",
// C: 					    NULL, ca0132_alt_follower_pfxs,
// C: 					    "Playback Switch",
// C: 					    true, 0, &spec->vmaster_mute.sw_kctl);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 
// C: 	/* Add in and out effects controls.
// C: 	 * VoiceFX, PE and CrystalVoice are added separately.
// C: 	 */
// C: 	num_fx = OUT_EFFECTS_COUNT + IN_EFFECTS_COUNT;
// C: 	for (i = 0; i < num_fx; i++) {
// C: 		/* Desktop cards break if Echo Cancellation is used. */
// C: 		if (ca0132_use_pci_mmio(spec)) {
// C: 			if (i == (ECHO_CANCELLATION - IN_EFFECT_START_NID +
// C: 						OUT_EFFECTS_COUNT))
// C: 				continue;
// C: 		}
// C: 
// C: 		err = add_fx_switch(codec, ca0132_effects[i].nid,
// C: 				    ca0132_effects[i].name,
// C: 				    ca0132_effects[i].direct);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 	/*
// C: 	 * If codec has use_alt_controls set to true, add effect level sliders,
// C: 	 * EQ presets, and Smart Volume presets. Also, change names to add FX
// C: 	 * prefix, and change PlayEnhancement and CrystalVoice to match.
// C: 	 */
// C: 	if (ca0132_use_alt_controls(spec)) {
// C: 		err = ca0132_alt_add_svm_enum(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		err = add_ca0132_alt_eq_presets(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		err = add_fx_switch(codec, PLAY_ENHANCEMENT,
// C: 					"Enable OutFX", 0);
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		err = add_fx_switch(codec, CRYSTAL_VOICE,
// C: 					"Enable InFX", 1);
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		num_sliders = OUT_EFFECTS_COUNT - 1;
// C: 		for (i = 0; i < num_sliders; i++) {
// C: 			err = ca0132_alt_add_effect_slider(codec,
// C: 					    ca0132_effects[i].nid,
// C: 					    ca0132_effects[i].name,
// C: 					    ca0132_effects[i].direct);
// C: 			if (err < 0)
// C: 				return err;
// C: 		}
// C: 
// C: 		err = ca0132_alt_add_effect_slider(codec, XBASS_XOVER,
// C: 					"X-Bass Crossover", EFX_DIR_OUT);
// C: 
// C: 		if (err < 0)
// C: 			return err;
// C: 	} else {
// C: 		err = add_fx_switch(codec, PLAY_ENHANCEMENT,
// C: 					"PlayEnhancement", 0);
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		err = add_fx_switch(codec, CRYSTAL_VOICE,
// C: 					"CrystalVoice", 1);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 	err = add_voicefx(codec);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	/*
// C: 	 * If the codec uses alt_functions, you need the enumerated controls
// C: 	 * to select the new outputs and inputs, plus add the new mic boost
// C: 	 * setting control.
// C: 	 */
// C: 	if (ca0132_use_alt_functions(spec)) {
// C: 		err = ca0132_alt_add_output_enum(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 		err = ca0132_alt_add_speaker_channel_cfg_enum(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 		err = ca0132_alt_add_front_full_range_switch(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 		err = ca0132_alt_add_rear_full_range_switch(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 		err = ca0132_alt_add_bass_redirection_crossover(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 		err = ca0132_alt_add_bass_redirection_switch(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 		err = ca0132_alt_add_mic_boost_enum(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 		/*
// C: 		 * ZxR only has microphone input, there is no front panel
// C: 		 * header on the card, and aux-in is handled by the DBPro board.
// C: 		 */
// C: 		if (ca0132_quirk(spec) != QUIRK_ZXR) {
// C: 			err = ca0132_alt_add_input_enum(codec);
// C: 			if (err < 0)
// C: 				return err;
// C: 		}
// C: 	}
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_AE5:
// C: 	case QUIRK_AE7:
// C: 		err = ae5_add_headphone_gain_enum(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 		err = ae5_add_sound_filter_enum(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 		break;
// C: 	case QUIRK_ZXR:
// C: 		err = zxr_add_headphone_gain_switch(codec);
// C: 		if (err < 0)
// C: 			return err;
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: 
// C: #ifdef ENABLE_TUNING_CONTROLS
// C: 	add_tuning_ctls(codec);
// C: #endif
// C: 
// C: 	err = snd_hda_jack_add_kctls(codec, &spec->autocfg);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	if (spec->dig_out) {
// C: 		err = snd_hda_create_spdif_out_ctls(codec, spec->dig_out,
// C: 						    spec->dig_out);
// C: 		if (err < 0)
// C: 			return err;
// C: 		err = snd_hda_create_spdif_share_sw(codec, &spec->multiout);
// C: 		if (err < 0)
// C: 			return err;
// C: 		/* spec->multiout.share_spdif = 1; */
// C: 	}
// C: 
// C: 	if (spec->dig_in) {
// C: 		err = snd_hda_create_spdif_in_ctls(codec, spec->dig_in);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 
// C: 	if (ca0132_use_alt_functions(spec))
// C: 		ca0132_alt_add_chmap_ctls(codec);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int dbpro_build_controls(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int err = 0;
// C: 
// C: 	if (spec->dig_out) {
// C: 		err = snd_hda_create_spdif_out_ctls(codec, spec->dig_out,
// C: 				spec->dig_out);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 
// C: 	if (spec->dig_in) {
// C: 		err = snd_hda_create_spdif_in_ctls(codec, spec->dig_in);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * PCM
// C:  */
// C: static const struct hda_pcm_stream ca0132_pcm_analog_playback = {
// C: 	.substreams = 1,
// C: 	.channels_min = 2,
// C: 	.channels_max = 6,
// C: 	.ops = {
// C: 		.prepare = ca0132_playback_pcm_prepare,
// C: 		.cleanup = ca0132_playback_pcm_cleanup,
// C: 		.get_delay = ca0132_playback_pcm_delay,
// C: 	},
// C: };
// C: 
// C: static const struct hda_pcm_stream ca0132_pcm_analog_capture = {
// C: 	.substreams = 1,
// C: 	.channels_min = 2,
// C: 	.channels_max = 2,
// C: 	.ops = {
// C: 		.prepare = ca0132_capture_pcm_prepare,
// C: 		.cleanup = ca0132_capture_pcm_cleanup,
// C: 		.get_delay = ca0132_capture_pcm_delay,
// C: 	},
// C: };
// C: 
// C: static const struct hda_pcm_stream ca0132_pcm_digital_playback = {
// C: 	.substreams = 1,
// C: 	.channels_min = 2,
// C: 	.channels_max = 2,
// C: 	.ops = {
// C: 		.open = ca0132_dig_playback_pcm_open,
// C: 		.close = ca0132_dig_playback_pcm_close,
// C: 		.prepare = ca0132_dig_playback_pcm_prepare,
// C: 		.cleanup = ca0132_dig_playback_pcm_cleanup
// C: 	},
// C: };
// C: 
// C: static const struct hda_pcm_stream ca0132_pcm_digital_capture = {
// C: 	.substreams = 1,
// C: 	.channels_min = 2,
// C: 	.channels_max = 2,
// C: };
// C: 
// C: static int ca0132_build_pcms(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	struct hda_pcm *info;
// C: 
// C: 	info = snd_hda_codec_pcm_new(codec, "CA0132 Analog");
// C: 	if (!info)
// C: 		return -ENOMEM;
// C: 	if (ca0132_use_alt_functions(spec)) {
// C: 		info->own_chmap = true;
// C: 		info->stream[SNDRV_PCM_STREAM_PLAYBACK].chmap
// C: 			= ca0132_alt_chmaps;
// C: 	}
// C: 	info->stream[SNDRV_PCM_STREAM_PLAYBACK] = ca0132_pcm_analog_playback;
// C: 	info->stream[SNDRV_PCM_STREAM_PLAYBACK].nid = spec->dacs[0];
// C: 	info->stream[SNDRV_PCM_STREAM_PLAYBACK].channels_max =
// C: 		spec->multiout.max_channels;
// C: 	info->stream[SNDRV_PCM_STREAM_CAPTURE] = ca0132_pcm_analog_capture;
// C: 	info->stream[SNDRV_PCM_STREAM_CAPTURE].substreams = 1;
// C: 	info->stream[SNDRV_PCM_STREAM_CAPTURE].nid = spec->adcs[0];
// C: 
// C: 	/* With the DSP enabled, desktops don't use this ADC. */
// C: 	if (!ca0132_use_alt_functions(spec)) {
// C: 		info = snd_hda_codec_pcm_new(codec, "CA0132 Analog Mic-In2");
// C: 		if (!info)
// C: 			return -ENOMEM;
// C: 		info->stream[SNDRV_PCM_STREAM_CAPTURE] =
// C: 			ca0132_pcm_analog_capture;
// C: 		info->stream[SNDRV_PCM_STREAM_CAPTURE].substreams = 1;
// C: 		info->stream[SNDRV_PCM_STREAM_CAPTURE].nid = spec->adcs[1];
// C: 	}
// C: 
// C: 	info = snd_hda_codec_pcm_new(codec, "CA0132 What U Hear");
// C: 	if (!info)
// C: 		return -ENOMEM;
// C: 	info->stream[SNDRV_PCM_STREAM_CAPTURE] = ca0132_pcm_analog_capture;
// C: 	info->stream[SNDRV_PCM_STREAM_CAPTURE].substreams = 1;
// C: 	info->stream[SNDRV_PCM_STREAM_CAPTURE].nid = spec->adcs[2];
// C: 
// C: 	if (!spec->dig_out && !spec->dig_in)
// C: 		return 0;
// C: 
// C: 	info = snd_hda_codec_pcm_new(codec, "CA0132 Digital");
// C: 	if (!info)
// C: 		return -ENOMEM;
// C: 	info->pcm_type = HDA_PCM_TYPE_SPDIF;
// C: 	if (spec->dig_out) {
// C: 		info->stream[SNDRV_PCM_STREAM_PLAYBACK] =
// C: 			ca0132_pcm_digital_playback;
// C: 		info->stream[SNDRV_PCM_STREAM_PLAYBACK].nid = spec->dig_out;
// C: 	}
// C: 	if (spec->dig_in) {
// C: 		info->stream[SNDRV_PCM_STREAM_CAPTURE] =
// C: 			ca0132_pcm_digital_capture;
// C: 		info->stream[SNDRV_PCM_STREAM_CAPTURE].nid = spec->dig_in;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int dbpro_build_pcms(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	struct hda_pcm *info;
// C: 
// C: 	info = snd_hda_codec_pcm_new(codec, "CA0132 Alt Analog");
// C: 	if (!info)
// C: 		return -ENOMEM;
// C: 	info->stream[SNDRV_PCM_STREAM_CAPTURE] = ca0132_pcm_analog_capture;
// C: 	info->stream[SNDRV_PCM_STREAM_CAPTURE].substreams = 1;
// C: 	info->stream[SNDRV_PCM_STREAM_CAPTURE].nid = spec->adcs[0];
// C: 
// C: 
// C: 	if (!spec->dig_out && !spec->dig_in)
// C: 		return 0;
// C: 
// C: 	info = snd_hda_codec_pcm_new(codec, "CA0132 Digital");
// C: 	if (!info)
// C: 		return -ENOMEM;
// C: 	info->pcm_type = HDA_PCM_TYPE_SPDIF;
// C: 	if (spec->dig_out) {
// C: 		info->stream[SNDRV_PCM_STREAM_PLAYBACK] =
// C: 			ca0132_pcm_digital_playback;
// C: 		info->stream[SNDRV_PCM_STREAM_PLAYBACK].nid = spec->dig_out;
// C: 	}
// C: 	if (spec->dig_in) {
// C: 		info->stream[SNDRV_PCM_STREAM_CAPTURE] =
// C: 			ca0132_pcm_digital_capture;
// C: 		info->stream[SNDRV_PCM_STREAM_CAPTURE].nid = spec->dig_in;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static void init_output(struct hda_codec *codec, hda_nid_t pin, hda_nid_t dac)
// C: {
// C: 	if (pin) {
// C: 		snd_hda_set_pin_ctl(codec, pin, PIN_HP);
// C: 		if (get_wcaps(codec, pin) & AC_WCAP_OUT_AMP)
// C: 			snd_hda_codec_write(codec, pin, 0,
// C: 					    AC_VERB_SET_AMP_GAIN_MUTE,
// C: 					    AMP_OUT_UNMUTE);
// C: 	}
// C: 	if (dac && (get_wcaps(codec, dac) & AC_WCAP_OUT_AMP))
// C: 		snd_hda_codec_write(codec, dac, 0,
// C: 				    AC_VERB_SET_AMP_GAIN_MUTE, AMP_OUT_ZERO);
// C: }
// C: 
// C: static void init_input(struct hda_codec *codec, hda_nid_t pin, hda_nid_t adc)
// C: {
// C: 	if (pin) {
// C: 		snd_hda_set_pin_ctl(codec, pin, PIN_VREF80);
// C: 		if (get_wcaps(codec, pin) & AC_WCAP_IN_AMP)
// C: 			snd_hda_codec_write(codec, pin, 0,
// C: 					    AC_VERB_SET_AMP_GAIN_MUTE,
// C: 					    AMP_IN_UNMUTE(0));
// C: 	}
// C: 	if (adc && (get_wcaps(codec, adc) & AC_WCAP_IN_AMP)) {
// C: 		snd_hda_codec_write(codec, adc, 0, AC_VERB_SET_AMP_GAIN_MUTE,
// C: 				    AMP_IN_UNMUTE(0));
// C: 
// C: 		/* init to 0 dB and unmute. */
// C: 		snd_hda_codec_amp_stereo(codec, adc, HDA_INPUT, 0,
// C: 					 HDA_AMP_VOLMASK, 0x5a);
// C: 		snd_hda_codec_amp_stereo(codec, adc, HDA_INPUT, 0,
// C: 					 HDA_AMP_MUTE, 0);
// C: 	}
// C: }
// C: 
// C: static void refresh_amp_caps(struct hda_codec *codec, hda_nid_t nid, int dir)
// C: {
// C: 	unsigned int caps;
// C: 
// C: 	caps = snd_hda_param_read(codec, nid, dir == HDA_OUTPUT ?
// C: 				  AC_PAR_AMP_OUT_CAP : AC_PAR_AMP_IN_CAP);
// C: 	snd_hda_override_amp_caps(codec, nid, dir, caps);
// C: }
// C: 
// C: /*
// C:  * Switch between Digital built-in mic and analog mic.
// C:  */
// C: static void ca0132_set_dmic(struct hda_codec *codec, int enable)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 	u8 val;
// C: 	unsigned int oldval;
// C: 
// C: 	codec_dbg(codec, "ca0132_set_dmic: enable=%d\\n", enable);
// C: 
// C: 	oldval = stop_mic1(codec);
// C: 	ca0132_set_vipsource(codec, 0);
// C: 	if (enable) {
// C: 		/* set DMic input as 2-ch */
// C: 		tmp = FLOAT_TWO;
// C: 		dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 
// C: 		val = spec->dmic_ctl;
// C: 		val |= 0x80;
// C: 		snd_hda_codec_write(codec, spec->input_pins[0], 0,
// C: 				    VENDOR_CHIPIO_DMIC_CTL_SET, val);
// C: 
// C: 		if (!(spec->dmic_ctl & 0x20))
// C: 			chipio_set_control_flag(codec, CONTROL_FLAG_DMIC, 1);
// C: 	} else {
// C: 		/* set AMic input as mono */
// C: 		tmp = FLOAT_ONE;
// C: 		dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 
// C: 		val = spec->dmic_ctl;
// C: 		/* clear bit7 and bit5 to disable dmic */
// C: 		val &= 0x5f;
// C: 		snd_hda_codec_write(codec, spec->input_pins[0], 0,
// C: 				    VENDOR_CHIPIO_DMIC_CTL_SET, val);
// C: 
// C: 		if (!(spec->dmic_ctl & 0x20))
// C: 			chipio_set_control_flag(codec, CONTROL_FLAG_DMIC, 0);
// C: 	}
// C: 	ca0132_set_vipsource(codec, 1);
// C: 	resume_mic1(codec, oldval);
// C: }
// C: 
// C: /*
// C:  * Initialization for Digital Mic.
// C:  */
// C: static void ca0132_init_dmic(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	u8 val;
// C: 
// C: 	/* Setup Digital Mic here, but don't enable.
// C: 	 * Enable based on jack detect.
// C: 	 */
// C: 
// C: 	/* MCLK uses MPIO1, set to enable.
// C: 	 * Bit 2-0: MPIO select
// C: 	 * Bit   3: set to disable
// C: 	 * Bit 7-4: reserved
// C: 	 */
// C: 	val = 0x01;
// C: 	snd_hda_codec_write(codec, spec->input_pins[0], 0,
// C: 			    VENDOR_CHIPIO_DMIC_MCLK_SET, val);
// C: 
// C: 	/* Data1 uses MPIO3. Data2 not use
// C: 	 * Bit 2-0: Data1 MPIO select
// C: 	 * Bit   3: set disable Data1
// C: 	 * Bit 6-4: Data2 MPIO select
// C: 	 * Bit   7: set disable Data2
// C: 	 */
// C: 	val = 0x83;
// C: 	snd_hda_codec_write(codec, spec->input_pins[0], 0,
// C: 			    VENDOR_CHIPIO_DMIC_PIN_SET, val);
// C: 
// C: 	/* Use Ch-0 and Ch-1. Rate is 48K, mode 1. Disable DMic first.
// C: 	 * Bit 3-0: Channel mask
// C: 	 * Bit   4: set for 48KHz, clear for 32KHz
// C: 	 * Bit   5: mode
// C: 	 * Bit   6: set to select Data2, clear for Data1
// C: 	 * Bit   7: set to enable DMic, clear for AMic
// C: 	 */
// C: 	if (ca0132_quirk(spec) == QUIRK_ALIENWARE_M17XR4)
// C: 		val = 0x33;
// C: 	else
// C: 		val = 0x23;
// C: 	/* keep a copy of dmic ctl val for enable/disable dmic purpuse */
// C: 	spec->dmic_ctl = val;
// C: 	snd_hda_codec_write(codec, spec->input_pins[0], 0,
// C: 			    VENDOR_CHIPIO_DMIC_CTL_SET, val);
// C: }
// C: 
// C: /*
// C:  * Initialization for Analog Mic 2
// C:  */
// C: static void ca0132_init_analog_mic2(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	chipio_8051_write_exram_no_mutex(codec, 0x1920, 0x00);
// C: 	chipio_8051_write_exram_no_mutex(codec, 0x192d, 0x00);
// C: }
// C: 
// C: static void ca0132_refresh_widget_caps(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int i;
// C: 
// C: 	codec_dbg(codec, "ca0132_refresh_widget_caps.\\n");
// C: 	snd_hda_codec_update_widgets(codec);
// C: 
// C: 	for (i = 0; i < spec->multiout.num_dacs; i++)
// C: 		refresh_amp_caps(codec, spec->dacs[i], HDA_OUTPUT);
// C: 
// C: 	for (i = 0; i < spec->num_outputs; i++)
// C: 		refresh_amp_caps(codec, spec->out_pins[i], HDA_OUTPUT);
// C: 
// C: 	for (i = 0; i < spec->num_inputs; i++) {
// C: 		refresh_amp_caps(codec, spec->adcs[i], HDA_INPUT);
// C: 		refresh_amp_caps(codec, spec->input_pins[i], HDA_INPUT);
// C: 	}
// C: }
// C: 
// C: 
// C: /* If there is an active channel for some reason, find it and free it. */
// C: static void ca0132_alt_free_active_dma_channels(struct hda_codec *codec)
// C: {
// C: 	unsigned int i, tmp;
// C: 	int status;
// C: 
// C: 	/* Read active DSPDMAC channel register. */
// C: 	status = chipio_read(codec, DSPDMAC_CHNLSTART_MODULE_OFFSET, &tmp);
// C: 	if (status >= 0) {
// C: 		/* AND against 0xfff to get the active channel bits. */
// C: 		tmp = tmp & 0xfff;
// C: 
// C: 		/* If there are no active channels, nothing to free. */
// C: 		if (!tmp)
// C: 			return;
// C: 	} else {
// C: 		codec_dbg(codec, "%s: Failed to read active DSP DMA channel register.\\n",
// C: 				__func__);
// C: 		return;
// C: 	}
// C: 
// C: 	/*
// C: 	 * Check each DSP DMA channel for activity, and if the channel is
// C: 	 * active, free it.
// C: 	 */
// C: 	for (i = 0; i < DSPDMAC_DMA_CFG_CHANNEL_COUNT; i++) {
// C: 		if (dsp_is_dma_active(codec, i)) {
// C: 			status = dspio_free_dma_chan(codec, i);
// C: 			if (status < 0)
// C: 				codec_dbg(codec, "%s: Failed to free active DSP DMA channel %d.\\n",
// C: 						__func__, i);
// C: 		}
// C: 	}
// C: }
// C: 
// C: /*
// C:  * In the case of CT_EXTENSIONS_ENABLE being set to 1, and the DSP being in
// C:  * use, audio is no longer routed directly to the DAC/ADC from the HDA stream.
// C:  * Instead, audio is now routed through the DSP's DMA controllers, which
// C:  * the DSP is tasked with setting up itself. Through debugging, it seems the
// C:  * cause of most of the no-audio on startup issues were due to improperly
// C:  * configured DSP DMA channels.
// C:  *
// C:  * Normally, the DSP configures these the first time an HDA audio stream is
// C:  * started post DSP firmware download. That is why creating a 'dummy' stream
// C:  * worked in fixing the audio in some cases. This works most of the time, but
// C:  * sometimes if a stream is started/stopped before the DSP can setup the DMA
// C:  * configuration registers, it ends up in a broken state. Issues can also
// C:  * arise if streams are started in an unusual order, i.e the audio output dma
// C:  * channel being sandwiched between the mic1 and mic2 dma channels.
// C:  *
// C:  * The solution to this is to make sure that the DSP has no DMA channels
// C:  * in use post DSP firmware download, and then to manually start each default
// C:  * DSP stream that uses the DMA channels. These are 0x0c, the audio output
// C:  * stream, 0x03, analog mic 1, and 0x04, analog mic 2.
// C:  */
// C: static void ca0132_alt_start_dsp_audio_streams(struct hda_codec *codec)
// C: {
// C: 	static const unsigned int dsp_dma_stream_ids[] = { 0x0c, 0x03, 0x04 };
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int i, tmp;
// C: 
// C: 	/*
// C: 	 * Check if any of the default streams are active, and if they are,
// C: 	 * stop them.
// C: 	 */
// C: 	scoped_guard(mutex, &spec->chipio_mutex) {
// C: 		for (i = 0; i < ARRAY_SIZE(dsp_dma_stream_ids); i++) {
// C: 			chipio_get_stream_control(codec, dsp_dma_stream_ids[i], &tmp);
// C: 
// C: 			if (tmp) {
// C: 				chipio_set_stream_control(codec,
// C: 							  dsp_dma_stream_ids[i], 0);
// C: 			}
// C: 		}
// C: 	}
// C: 
// C: 	/*
// C: 	 * If all DSP streams are inactive, there should be no active DSP DMA
// C: 	 * channels. Check and make sure this is the case, and if it isn't,
// C: 	 * free any active channels.
// C: 	 */
// C: 	ca0132_alt_free_active_dma_channels(codec);
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	/* Make sure stream 0x0c is six channels. */
// C: 	chipio_set_stream_channels(codec, 0x0c, 6);
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(dsp_dma_stream_ids); i++) {
// C: 		chipio_set_stream_control(codec,
// C: 				dsp_dma_stream_ids[i], 1);
// C: 
// C: 		/* Give the DSP some time to setup the DMA channel. */
// C: 		msleep(75);
// C: 	}
// C: }
// C: 
// C: /*
// C:  * The region of ChipIO memory from 0x190000-0x1903fc is a sort of 'audio
// C:  * router', where each entry represents a 48khz audio channel, with a format
// C:  * of an 8-bit destination, an 8-bit source, and an unknown 2-bit number
// C:  * value. The 2-bit number value is seemingly 0 if inactive, 1 if active,
// C:  * and 3 if it's using Sample Rate Converter ports.
// C:  * An example is:
// C:  * 0x0001f8c0
// C:  * In this case, f8 is the destination, and c0 is the source. The number value
// C:  * is 1.
// C:  * This region of memory is normally managed internally by the 8051, where
// C:  * the region of exram memory from 0x1477-0x1575 has each byte represent an
// C:  * entry within the 0x190000 range, and when a range of entries is in use, the
// C:  * ending value is overwritten with 0xff.
// C:  * 0x1578 in exram is a table of 0x25 entries, corresponding to the ChipIO
// C:  * streamID's, where each entry is a starting 0x190000 port offset.
// C:  * 0x159d in exram is the same as 0x1578, except it contains the ending port
// C:  * offset for the corresponding streamID.
// C:  *
// C:  * On certain cards, such as the SBZ/ZxR/AE7, these are originally setup by
// C:  * the 8051, then manually overwritten to remap the ports to work with the
// C:  * new DACs.
// C:  *
// C:  * Currently known portID's:
// C:  * 0x00-0x1f: HDA audio stream input/output ports.
// C:  * 0x80-0xbf: Sample rate converter input/outputs. Only valid ports seem to
// C:  *            have the lower-nibble set to 0x1, 0x2, and 0x9.
// C:  * 0xc0-0xdf: DSP DMA input/output ports. Dynamically assigned.
// C:  * 0xe0-0xff: DAC/ADC audio input/output ports.
// C:  *
// C:  * Currently known streamID's:
// C:  * 0x03: Mic1 ADC to DSP.
// C:  * 0x04: Mic2 ADC to DSP.
// C:  * 0x05: HDA node 0x02 audio stream to DSP.
// C:  * 0x0f: DSP Mic exit to HDA node 0x07.
// C:  * 0x0c: DSP processed audio to DACs.
// C:  * 0x14: DAC0, front L/R.
// C:  *
// C:  * It is possible to route the HDA audio streams directly to the DAC and
// C:  * bypass the DSP entirely, with the only downside being that since the DSP
// C:  * does volume control, the only volume control you'll get is through PCM on
// C:  * the PC side, in the same way volume is handled for optical out. This may be
// C:  * useful for debugging.
// C:  */
// C: static void chipio_remap_stream(struct hda_codec *codec,
// C: 		const struct chipio_stream_remap_data *remap_data)
// C: {
// C: 	unsigned int i, stream_offset;
// C: 
// C: 	/* Get the starting port for the stream to be remapped. */
// C: 	chipio_8051_read_exram(codec, 0x1578 + remap_data->stream_id,
// C: 			&stream_offset);
// C: 
// C: 	/*
// C: 	 * Check if the stream's port value is 0xff, because the 8051 may not
// C: 	 * have gotten around to setting up the stream yet. Wait until it's
// C: 	 * setup to remap it's ports.
// C: 	 */
// C: 	if (stream_offset == 0xff) {
// C: 		for (i = 0; i < 5; i++) {
// C: 			msleep(25);
// C: 
// C: 			chipio_8051_read_exram(codec, 0x1578 + remap_data->stream_id,
// C: 					&stream_offset);
// C: 
// C: 			if (stream_offset != 0xff)
// C: 				break;
// C: 		}
// C: 	}
// C: 
// C: 	if (stream_offset == 0xff) {
// C: 		codec_info(codec, "%s: Stream 0x%02x ports aren't allocated, remap failed!\\n",
// C: 				__func__, remap_data->stream_id);
// C: 		return;
// C: 	}
// C: 
// C: 	/* Offset isn't in bytes, its in 32-bit words, so multiply it by 4. */
// C: 	stream_offset *= 0x04;
// C: 	stream_offset += 0x190000;
// C: 
// C: 	for (i = 0; i < remap_data->count; i++) {
// C: 		chipio_write_no_mutex(codec,
// C: 				stream_offset + remap_data->offset[i],
// C: 				remap_data->value[i]);
// C: 	}
// C: 
// C: 	/* Update stream map configuration. */
// C: 	chipio_write_no_mutex(codec, 0x19042c, 0x00000001);
// C: }
// C: 
// C: /*
// C:  * Default speaker tuning values setup for alternative codecs.
// C:  */
// C: static const unsigned int sbz_default_delay_values[] = {
// C: 	/* Non-zero values are floating point 0.000198. */
// C: 	0x394f9e38, 0x394f9e38, 0x00000000, 0x00000000, 0x00000000, 0x00000000
// C: };
// C: 
// C: static const unsigned int zxr_default_delay_values[] = {
// C: 	/* Non-zero values are floating point 0.000220. */
// C: 	0x00000000, 0x00000000, 0x3966afcd, 0x3966afcd, 0x3966afcd, 0x3966afcd
// C: };
// C: 
// C: static const unsigned int ae5_default_delay_values[] = {
// C: 	/* Non-zero values are floating point 0.000100. */
// C: 	0x00000000, 0x00000000, 0x38d1b717, 0x38d1b717, 0x38d1b717, 0x38d1b717
// C: };
// C: 
// C: /*
// C:  * If we never change these, probably only need them on initialization.
// C:  */
// C: static void ca0132_alt_init_speaker_tuning(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int i, tmp, start_req, end_req;
// C: 	const unsigned int *values;
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_SBZ:
// C: 		values = sbz_default_delay_values;
// C: 		break;
// C: 	case QUIRK_ZXR:
// C: 		values = zxr_default_delay_values;
// C: 		break;
// C: 	case QUIRK_AE5:
// C: 	case QUIRK_AE7:
// C: 		values = ae5_default_delay_values;
// C: 		break;
// C: 	default:
// C: 		values = sbz_default_delay_values;
// C: 		break;
// C: 	}
// C: 
// C: 	tmp = FLOAT_ZERO;
// C: 	dspio_set_uint_param(codec, 0x96, SPEAKER_TUNING_ENABLE_CENTER_EQ, tmp);
// C: 
// C: 	start_req = SPEAKER_TUNING_FRONT_LEFT_VOL_LEVEL;
// C: 	end_req = SPEAKER_TUNING_REAR_RIGHT_VOL_LEVEL;
// C: 	for (i = start_req; i < end_req + 1; i++)
// C: 		dspio_set_uint_param(codec, 0x96, i, tmp);
// C: 
// C: 	start_req = SPEAKER_TUNING_FRONT_LEFT_INVERT;
// C: 	end_req = SPEAKER_TUNING_REAR_RIGHT_INVERT;
// C: 	for (i = start_req; i < end_req + 1; i++)
// C: 		dspio_set_uint_param(codec, 0x96, i, tmp);
// C: 
// C: 
// C: 	for (i = 0; i < 6; i++)
// C: 		dspio_set_uint_param(codec, 0x96,
// C: 				SPEAKER_TUNING_FRONT_LEFT_DELAY + i, values[i]);
// C: }
// C: 
// C: /*
// C:  * Initialize mic for non-chromebook ca0132 implementations.
// C:  */
// C: static void ca0132_alt_init_analog_mics(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 
// C: 	/* Mic 1 Setup */
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_MICIN1, SR_96_000);
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_MICOUT1, SR_96_000);
// C: 	if (ca0132_quirk(spec) == QUIRK_R3DI) {
// C: 		chipio_set_conn_rate(codec, 0x0F, SR_96_000);
// C: 		tmp = FLOAT_ONE;
// C: 	} else
// C: 		tmp = FLOAT_THREE;
// C: 	dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 
// C: 	/* Mic 2 setup (not present on desktop cards) */
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_MICIN2, SR_96_000);
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_MICOUT2, SR_96_000);
// C: 	if (ca0132_quirk(spec) == QUIRK_R3DI)
// C: 		chipio_set_conn_rate(codec, 0x0F, SR_96_000);
// C: 	tmp = FLOAT_ZERO;
// C: 	dspio_set_uint_param(codec, 0x80, 0x01, tmp);
// C: }
// C: 
// C: /*
// C:  * Sets the source of stream 0x14 to connpointID 0x48, and the destination
// C:  * connpointID to 0x91. If this isn't done, the destination is 0x71, and
// C:  * you get no sound. I'm guessing this has to do with the Sound Blaster Z
// C:  * having an updated DAC, which changes the destination to that DAC.
// C:  */
// C: static void sbz_connect_streams(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	codec_dbg(codec, "Connect Streams entered, mutex locked and loaded.\\n");
// C: 
// C: 	/* This value is 0x43 for 96khz, and 0x83 for 192khz. */
// C: 	chipio_write_no_mutex(codec, 0x18a020, 0x00000043);
// C: 
// C: 	/* Setup stream 0x14 with it's source and destination points */
// C: 	chipio_set_stream_source_dest(codec, 0x14, 0x48, 0x91);
// C: 	chipio_set_conn_rate_no_mutex(codec, 0x48, SR_96_000);
// C: 	chipio_set_conn_rate_no_mutex(codec, 0x91, SR_96_000);
// C: 	chipio_set_stream_channels(codec, 0x14, 2);
// C: 	chipio_set_stream_control(codec, 0x14, 1);
// C: 
// C: 	codec_dbg(codec, "Connect Streams exited, mutex released.\\n");
// C: }
// C: 
// C: /*
// C:  * Write data through ChipIO to setup proper stream destinations.
// C:  * Not sure how it exactly works, but it seems to direct data
// C:  * to different destinations. Example is f8 to c0, e0 to c0.
// C:  * All I know is, if you don't set these, you get no sound.
// C:  */
// C: static void sbz_chipio_startup_data(struct hda_codec *codec)
// C: {
// C: 	const struct chipio_stream_remap_data *dsp_out_remap_data;
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 	codec_dbg(codec, "Startup Data entered, mutex locked and loaded.\\n");
// C: 
// C: 	/* Remap DAC0's output ports. */
// C: 	chipio_remap_stream(codec, &stream_remap_data[0]);
// C: 
// C: 	/* Remap DSP audio output stream ports. */
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_SBZ:
// C: 		dsp_out_remap_data = &stream_remap_data[1];
// C: 		break;
// C: 
// C: 	case QUIRK_ZXR:
// C: 		dsp_out_remap_data = &stream_remap_data[2];
// C: 		break;
// C: 
// C: 	default:
// C: 		dsp_out_remap_data = NULL;
// C: 		break;
// C: 	}
// C: 
// C: 	if (dsp_out_remap_data)
// C: 		chipio_remap_stream(codec, dsp_out_remap_data);
// C: 
// C: 	codec_dbg(codec, "Startup Data exited, mutex released.\\n");
// C: }
// C: 
// C: static void ca0132_alt_dsp_initial_mic_setup(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 
// C: 	chipio_set_stream_control(codec, 0x03, 0);
// C: 	chipio_set_stream_control(codec, 0x04, 0);
// C: 
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_MICIN1, SR_96_000);
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_MICOUT1, SR_96_000);
// C: 
// C: 	tmp = FLOAT_THREE;
// C: 	dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 
// C: 	chipio_set_stream_control(codec, 0x03, 1);
// C: 	chipio_set_stream_control(codec, 0x04, 1);
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_SBZ:
// C: 		chipio_write(codec, 0x18b098, 0x0000000c);
// C: 		chipio_write(codec, 0x18b09C, 0x0000000c);
// C: 		break;
// C: 	case QUIRK_AE5:
// C: 		chipio_write(codec, 0x18b098, 0x0000000c);
// C: 		chipio_write(codec, 0x18b09c, 0x0000004c);
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: }
// C: 
// C: static void ae5_post_dsp_register_set(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	chipio_8051_write_direct(codec, 0x93, 0x10);
// C: 	chipio_8051_write_pll_pmu(codec, 0x44, 0xc2);
// C: 
// C: 	writeb(0xff, spec->mem_base + 0x304);
// C: 	writeb(0xff, spec->mem_base + 0x304);
// C: 	writeb(0xff, spec->mem_base + 0x304);
// C: 	writeb(0xff, spec->mem_base + 0x304);
// C: 	writeb(0x00, spec->mem_base + 0x100);
// C: 	writeb(0xff, spec->mem_base + 0x304);
// C: 	writeb(0x00, spec->mem_base + 0x100);
// C: 	writeb(0xff, spec->mem_base + 0x304);
// C: 	writeb(0x00, spec->mem_base + 0x100);
// C: 	writeb(0xff, spec->mem_base + 0x304);
// C: 	writeb(0x00, spec->mem_base + 0x100);
// C: 	writeb(0xff, spec->mem_base + 0x304);
// C: 
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x2b, 0x3f);
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x2d, 0x3f);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x07, 0x83);
// C: }
// C: 
// C: static void ae5_post_dsp_param_setup(struct hda_codec *codec)
// C: {
// C: 	/*
// C: 	 * Param3 in the 8051's memory is represented by the ascii string 'mch'
// C: 	 * which seems to be 'multichannel'. This is also mentioned in the
// C: 	 * AE-5's registry values in Windows.
// C: 	 */
// C: 	chipio_set_control_param(codec, 3, 0);
// C: 	/*
// C: 	 * I believe ASI is 'audio serial interface' and that it's used to
// C: 	 * change colors on the external LED strip connected to the AE-5.
// C: 	 */
// C: 	chipio_set_control_flag(codec, CONTROL_FLAG_ASI_96KHZ, 1);
// C: 
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0, 0x724, 0x83);
// C: 	chipio_set_control_param(codec, CONTROL_PARAM_ASI, 0);
// C: 
// C: 	chipio_8051_write_exram(codec, 0xfa92, 0x22);
// C: }
// C: 
// C: static void ae5_post_dsp_pll_setup(struct hda_codec *codec)
// C: {
// C: 	chipio_8051_write_pll_pmu(codec, 0x41, 0xc8);
// C: 	chipio_8051_write_pll_pmu(codec, 0x45, 0xcc);
// C: 	chipio_8051_write_pll_pmu(codec, 0x40, 0xcb);
// C: 	chipio_8051_write_pll_pmu(codec, 0x43, 0xc7);
// C: 	chipio_8051_write_pll_pmu(codec, 0x51, 0x8d);
// C: }
// C: 
// C: static void ae5_post_dsp_stream_setup(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0, 0x725, 0x81);
// C: 
// C: 	chipio_set_conn_rate_no_mutex(codec, 0x70, SR_96_000);
// C: 
// C: 	chipio_set_stream_source_dest(codec, 0x5, 0x43, 0x0);
// C: 
// C: 	chipio_set_stream_source_dest(codec, 0x18, 0x9, 0xd0);
// C: 	chipio_set_conn_rate_no_mutex(codec, 0xd0, SR_96_000);
// C: 	chipio_set_stream_channels(codec, 0x18, 6);
// C: 	chipio_set_stream_control(codec, 0x18, 1);
// C: 
// C: 	chipio_set_control_param_no_mutex(codec, CONTROL_PARAM_ASI, 4);
// C: 
// C: 	chipio_8051_write_pll_pmu_no_mutex(codec, 0x43, 0xc7);
// C: 
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x01, 0x80);
// C: }
// C: 
// C: static void ae5_post_dsp_startup_data(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	chipio_write_no_mutex(codec, 0x189000, 0x0001f101);
// C: 	chipio_write_no_mutex(codec, 0x189004, 0x0001f101);
// C: 	chipio_write_no_mutex(codec, 0x189024, 0x00014004);
// C: 	chipio_write_no_mutex(codec, 0x189028, 0x0002000f);
// C: 
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x0a, 0x05);
// C: 	chipio_set_control_param_no_mutex(codec, CONTROL_PARAM_ASI, 7);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x0b, 0x12);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x04, 0x00);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x06, 0x48);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x0a, 0x05);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x07, 0x83);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x0f, 0x00);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x10, 0x00);
// C: 	ca0113_mmio_gpio_set(codec, 0, true);
// C: 	ca0113_mmio_gpio_set(codec, 1, true);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x07, 0x80);
// C: 
// C: 	chipio_write_no_mutex(codec, 0x18b03c, 0x00000012);
// C: 
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x0f, 0x00);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x10, 0x00);
// C: }
// C: 
// C: static void ae7_post_dsp_setup_ports(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	/* Seems to share the same port remapping as the SBZ. */
// C: 	chipio_remap_stream(codec, &stream_remap_data[1]);
// C: 
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x30, 0x00);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x0d, 0x40);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x17, 0x00);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x19, 0x00);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x11, 0xff);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x12, 0xff);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x13, 0xff);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x14, 0x7f);
// C: }
// C: 
// C: static void ae7_post_dsp_asi_stream_setup(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0, 0x725, 0x81);
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x2b, 0x00);
// C: 
// C: 	chipio_set_conn_rate_no_mutex(codec, 0x70, SR_96_000);
// C: 
// C: 	chipio_set_stream_source_dest(codec, 0x05, 0x43, 0x00);
// C: 	chipio_set_stream_source_dest(codec, 0x18, 0x09, 0xd0);
// C: 
// C: 	chipio_set_conn_rate_no_mutex(codec, 0xd0, SR_96_000);
// C: 	chipio_set_stream_channels(codec, 0x18, 6);
// C: 	chipio_set_stream_control(codec, 0x18, 1);
// C: 
// C: 	chipio_set_control_param_no_mutex(codec, CONTROL_PARAM_ASI, 4);
// C: }
// C: 
// C: static void ae7_post_dsp_pll_setup(struct hda_codec *codec)
// C: {
// C: 	static const unsigned int addr[] = {
// C: 		0x41, 0x45, 0x40, 0x43, 0x51
// C: 	};
// C: 	static const unsigned int data[] = {
// C: 		0xc8, 0xcc, 0xcb, 0xc7, 0x8d
// C: 	};
// C: 	unsigned int i;
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(addr); i++)
// C: 		chipio_8051_write_pll_pmu_no_mutex(codec, addr[i], data[i]);
// C: }
// C: 
// C: static void ae7_post_dsp_asi_setup_ports(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	static const unsigned int target[] = {
// C: 		0x0b, 0x04, 0x06, 0x0a, 0x0c, 0x11, 0x12, 0x13, 0x14
// C: 	};
// C: 	static const unsigned int data[] = {
// C: 		0x12, 0x00, 0x48, 0x05, 0x5f, 0xff, 0xff, 0xff, 0x7f
// C: 	};
// C: 	unsigned int i;
// C: 
// C: 	guard(mutex)(&spec->chipio_mutex);
// C: 
// C: 	chipio_8051_write_pll_pmu_no_mutex(codec, 0x43, 0xc7);
// C: 
// C: 	chipio_write_no_mutex(codec, 0x189000, 0x0001f101);
// C: 	chipio_write_no_mutex(codec, 0x189004, 0x0001f101);
// C: 	chipio_write_no_mutex(codec, 0x189024, 0x00014004);
// C: 	chipio_write_no_mutex(codec, 0x189028, 0x0002000f);
// C: 
// C: 	ae7_post_dsp_pll_setup(codec);
// C: 	chipio_set_control_param_no_mutex(codec, CONTROL_PARAM_ASI, 7);
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(target); i++)
// C: 		ca0113_mmio_command_set(codec, 0x48, target[i], data[i]);
// C: 
// C: 	ca0113_mmio_command_set_type2(codec, 0x48, 0x07, 0x83);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x0f, 0x00);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x10, 0x00);
// C: 
// C: 	chipio_set_stream_source_dest(codec, 0x21, 0x64, 0x56);
// C: 	chipio_set_stream_channels(codec, 0x21, 2);
// C: 	chipio_set_conn_rate_no_mutex(codec, 0x56, SR_8_000);
// C: 
// C: 	chipio_set_control_param_no_mutex(codec, CONTROL_PARAM_NODE_ID, 0x09);
// C: 	/*
// C: 	 * In the 8051's memory, this param is referred to as 'n2sid', which I
// C: 	 * believe is 'node to streamID'. It seems to be a way to assign a
// C: 	 * stream to a given HDA node.
// C: 	 */
// C: 	chipio_set_control_param_no_mutex(codec, 0x20, 0x21);
// C: 
// C: 	chipio_write_no_mutex(codec, 0x18b038, 0x00000088);
// C: 
// C: 	/*
// C: 	 * Now, at this point on Windows, an actual stream is setup and
// C: 	 * seemingly sends data to the HDA node 0x09, which is the digital
// C: 	 * audio input node. This is left out here, because obviously I don't
// C: 	 * know what data is being sent. Interestingly, the AE-5 seems to go
// C: 	 * through the motions of getting here and never actually takes this
// C: 	 * step, but the AE-7 does.
// C: 	 */
// C: 
// C: 	ca0113_mmio_gpio_set(codec, 0, 1);
// C: 	ca0113_mmio_gpio_set(codec, 1, 1);
// C: 
// C: 	ca0113_mmio_command_set_type2(codec, 0x48, 0x07, 0x83);
// C: 	chipio_write_no_mutex(codec, 0x18b03c, 0x00000000);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x0f, 0x00);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x10, 0x00);
// C: 
// C: 	chipio_set_stream_source_dest(codec, 0x05, 0x43, 0x00);
// C: 	chipio_set_stream_source_dest(codec, 0x18, 0x09, 0xd0);
// C: 
// C: 	chipio_set_conn_rate_no_mutex(codec, 0xd0, SR_96_000);
// C: 	chipio_set_stream_channels(codec, 0x18, 6);
// C: 
// C: 	/*
// C: 	 * Runs again, this has been repeated a few times, but I'm just
// C: 	 * following what the Windows driver does.
// C: 	 */
// C: 	ae7_post_dsp_pll_setup(codec);
// C: 	chipio_set_control_param_no_mutex(codec, CONTROL_PARAM_ASI, 7);
// C: }
// C: 
// C: /*
// C:  * The Windows driver has commands that seem to setup ASI, which I believe to
// C:  * be some sort of audio serial interface. My current speculation is that it's
// C:  * related to communicating with the new DAC.
// C:  */
// C: static void ae7_post_dsp_asi_setup(struct hda_codec *codec)
// C: {
// C: 	chipio_8051_write_direct(codec, 0x93, 0x10);
// C: 
// C: 	chipio_8051_write_pll_pmu(codec, 0x44, 0xc2);
// C: 
// C: 	ca0113_mmio_command_set_type2(codec, 0x48, 0x07, 0x83);
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x2e, 0x3f);
// C: 
// C: 	chipio_set_control_param(codec, 3, 3);
// C: 	chipio_set_control_flag(codec, CONTROL_FLAG_ASI_96KHZ, 1);
// C: 
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0, 0x724, 0x83);
// C: 	chipio_set_control_param(codec, CONTROL_PARAM_ASI, 0);
// C: 	snd_hda_codec_write(codec, 0x17, 0, 0x794, 0x00);
// C: 
// C: 	chipio_8051_write_exram(codec, 0xfa92, 0x22);
// C: 
// C: 	ae7_post_dsp_pll_setup(codec);
// C: 	ae7_post_dsp_asi_stream_setup(codec);
// C: 
// C: 	chipio_8051_write_pll_pmu(codec, 0x43, 0xc7);
// C: 
// C: 	ae7_post_dsp_asi_setup_ports(codec);
// C: }
// C: 
// C: /*
// C:  * Setup default parameters for DSP
// C:  */
// C: static void ca0132_setup_defaults(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 	int num_fx;
// C: 	int idx, i;
// C: 
// C: 	if (spec->dsp_state != DSP_DOWNLOADED)
// C: 		return;
// C: 
// C: 	/* out, in effects + voicefx */
// C: 	num_fx = OUT_EFFECTS_COUNT + IN_EFFECTS_COUNT + 1;
// C: 	for (idx = 0; idx < num_fx; idx++) {
// C: 		for (i = 0; i <= ca0132_effects[idx].params; i++) {
// C: 			dspio_set_uint_param(codec, ca0132_effects[idx].mid,
// C: 					     ca0132_effects[idx].reqs[i],
// C: 					     ca0132_effects[idx].def_vals[i]);
// C: 		}
// C: 	}
// C: 
// C: 	/*remove DSP headroom*/
// C: 	tmp = FLOAT_ZERO;
// C: 	dspio_set_uint_param(codec, 0x96, 0x3C, tmp);
// C: 
// C: 	/*set speaker EQ bypass attenuation*/
// C: 	dspio_set_uint_param(codec, 0x8f, 0x01, tmp);
// C: 
// C: 	/* set AMic1 and AMic2 as mono mic */
// C: 	tmp = FLOAT_ONE;
// C: 	dspio_set_uint_param(codec, 0x80, 0x00, tmp);
// C: 	dspio_set_uint_param(codec, 0x80, 0x01, tmp);
// C: 
// C: 	/* set AMic1 as CrystalVoice input */
// C: 	tmp = FLOAT_ONE;
// C: 	dspio_set_uint_param(codec, 0x80, 0x05, tmp);
// C: 
// C: 	/* set WUH source */
// C: 	tmp = FLOAT_TWO;
// C: 	dspio_set_uint_param(codec, 0x31, 0x00, tmp);
// C: }
// C: 
// C: /*
// C:  * Setup default parameters for Recon3D/Recon3Di DSP.
// C:  */
// C: 
// C: static void r3d_setup_defaults(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 	int num_fx;
// C: 	int idx, i;
// C: 
// C: 	if (spec->dsp_state != DSP_DOWNLOADED)
// C: 		return;
// C: 
// C: 	ca0132_alt_init_analog_mics(codec);
// C: 	ca0132_alt_start_dsp_audio_streams(codec);
// C: 
// C: 	/*remove DSP headroom*/
// C: 	tmp = FLOAT_ZERO;
// C: 	dspio_set_uint_param(codec, 0x96, 0x3C, tmp);
// C: 
// C: 	/* set WUH source */
// C: 	tmp = FLOAT_TWO;
// C: 	dspio_set_uint_param(codec, 0x31, 0x00, tmp);
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_WUH, SR_48_000);
// C: 
// C: 	/* Set speaker source? */
// C: 	dspio_set_uint_param(codec, 0x32, 0x00, tmp);
// C: 
// C: 	if (ca0132_quirk(spec) == QUIRK_R3DI)
// C: 		r3di_gpio_dsp_status_set(codec, R3DI_DSP_DOWNLOADED);
// C: 
// C: 	/* Disable mute on Center/LFE. */
// C: 	if (ca0132_quirk(spec) == QUIRK_R3D) {
// C: 		ca0113_mmio_gpio_set(codec, 2, false);
// C: 		ca0113_mmio_gpio_set(codec, 4, true);
// C: 	}
// C: 
// C: 	/* Setup effect defaults */
// C: 	num_fx = OUT_EFFECTS_COUNT + IN_EFFECTS_COUNT + 1;
// C: 	for (idx = 0; idx < num_fx; idx++) {
// C: 		for (i = 0; i <= ca0132_effects[idx].params; i++) {
// C: 			dspio_set_uint_param(codec,
// C: 					ca0132_effects[idx].mid,
// C: 					ca0132_effects[idx].reqs[i],
// C: 					ca0132_effects[idx].def_vals[i]);
// C: 		}
// C: 	}
// C: }
// C: 
// C: /*
// C:  * Setup default parameters for the Sound Blaster Z DSP. A lot more going on
// C:  * than the Chromebook setup.
// C:  */
// C: static void sbz_setup_defaults(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 	int num_fx;
// C: 	int idx, i;
// C: 
// C: 	if (spec->dsp_state != DSP_DOWNLOADED)
// C: 		return;
// C: 
// C: 	ca0132_alt_init_analog_mics(codec);
// C: 	ca0132_alt_start_dsp_audio_streams(codec);
// C: 	sbz_connect_streams(codec);
// C: 	sbz_chipio_startup_data(codec);
// C: 
// C: 	/*
// C: 	 * Sets internal input loopback to off, used to have a switch to
// C: 	 * enable input loopback, but turned out to be way too buggy.
// C: 	 */
// C: 	tmp = FLOAT_ONE;
// C: 	dspio_set_uint_param(codec, 0x37, 0x08, tmp);
// C: 	dspio_set_uint_param(codec, 0x37, 0x10, tmp);
// C: 
// C: 	/*remove DSP headroom*/
// C: 	tmp = FLOAT_ZERO;
// C: 	dspio_set_uint_param(codec, 0x96, 0x3C, tmp);
// C: 
// C: 	/* set WUH source */
// C: 	tmp = FLOAT_TWO;
// C: 	dspio_set_uint_param(codec, 0x31, 0x00, tmp);
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_WUH, SR_48_000);
// C: 
// C: 	/* Set speaker source? */
// C: 	dspio_set_uint_param(codec, 0x32, 0x00, tmp);
// C: 
// C: 	ca0132_alt_dsp_initial_mic_setup(codec);
// C: 
// C: 	/* out, in effects + voicefx */
// C: 	num_fx = OUT_EFFECTS_COUNT + IN_EFFECTS_COUNT + 1;
// C: 	for (idx = 0; idx < num_fx; idx++) {
// C: 		for (i = 0; i <= ca0132_effects[idx].params; i++) {
// C: 			dspio_set_uint_param(codec,
// C: 					ca0132_effects[idx].mid,
// C: 					ca0132_effects[idx].reqs[i],
// C: 					ca0132_effects[idx].def_vals[i]);
// C: 		}
// C: 	}
// C: 
// C: 	ca0132_alt_init_speaker_tuning(codec);
// C: }
// C: 
// C: /*
// C:  * Setup default parameters for the Sound BlasterX AE-5 DSP.
// C:  */
// C: static void ae5_setup_defaults(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 	int num_fx;
// C: 	int idx, i;
// C: 
// C: 	if (spec->dsp_state != DSP_DOWNLOADED)
// C: 		return;
// C: 
// C: 	ca0132_alt_init_analog_mics(codec);
// C: 	ca0132_alt_start_dsp_audio_streams(codec);
// C: 
// C: 	/* New, unknown SCP req's */
// C: 	tmp = FLOAT_ZERO;
// C: 	dspio_set_uint_param(codec, 0x96, 0x29, tmp);
// C: 	dspio_set_uint_param(codec, 0x96, 0x2a, tmp);
// C: 	dspio_set_uint_param(codec, 0x80, 0x0d, tmp);
// C: 	dspio_set_uint_param(codec, 0x80, 0x0e, tmp);
// C: 
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x2e, 0x3f);
// C: 	ca0113_mmio_gpio_set(codec, 0, false);
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x28, 0x00);
// C: 
// C: 	/* Internal loopback off */
// C: 	tmp = FLOAT_ONE;
// C: 	dspio_set_uint_param(codec, 0x37, 0x08, tmp);
// C: 	dspio_set_uint_param(codec, 0x37, 0x10, tmp);
// C: 
// C: 	/*remove DSP headroom*/
// C: 	tmp = FLOAT_ZERO;
// C: 	dspio_set_uint_param(codec, 0x96, 0x3C, tmp);
// C: 
// C: 	/* set WUH source */
// C: 	tmp = FLOAT_TWO;
// C: 	dspio_set_uint_param(codec, 0x31, 0x00, tmp);
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_WUH, SR_48_000);
// C: 
// C: 	/* Set speaker source? */
// C: 	dspio_set_uint_param(codec, 0x32, 0x00, tmp);
// C: 
// C: 	ca0132_alt_dsp_initial_mic_setup(codec);
// C: 	ae5_post_dsp_register_set(codec);
// C: 	ae5_post_dsp_param_setup(codec);
// C: 	ae5_post_dsp_pll_setup(codec);
// C: 	ae5_post_dsp_stream_setup(codec);
// C: 	ae5_post_dsp_startup_data(codec);
// C: 
// C: 	/* out, in effects + voicefx */
// C: 	num_fx = OUT_EFFECTS_COUNT + IN_EFFECTS_COUNT + 1;
// C: 	for (idx = 0; idx < num_fx; idx++) {
// C: 		for (i = 0; i <= ca0132_effects[idx].params; i++) {
// C: 			dspio_set_uint_param(codec,
// C: 					ca0132_effects[idx].mid,
// C: 					ca0132_effects[idx].reqs[i],
// C: 					ca0132_effects[idx].def_vals[i]);
// C: 		}
// C: 	}
// C: 
// C: 	ca0132_alt_init_speaker_tuning(codec);
// C: }
// C: 
// C: /*
// C:  * Setup default parameters for the Sound Blaster AE-7 DSP.
// C:  */
// C: static void ae7_setup_defaults(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp;
// C: 	int num_fx;
// C: 	int idx, i;
// C: 
// C: 	if (spec->dsp_state != DSP_DOWNLOADED)
// C: 		return;
// C: 
// C: 	ca0132_alt_init_analog_mics(codec);
// C: 	ca0132_alt_start_dsp_audio_streams(codec);
// C: 	ae7_post_dsp_setup_ports(codec);
// C: 
// C: 	tmp = FLOAT_ZERO;
// C: 	dspio_set_uint_param(codec, 0x96,
// C: 			SPEAKER_TUNING_FRONT_LEFT_INVERT, tmp);
// C: 	dspio_set_uint_param(codec, 0x96,
// C: 			SPEAKER_TUNING_FRONT_RIGHT_INVERT, tmp);
// C: 
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x2e, 0x3f);
// C: 
// C: 	/* New, unknown SCP req's */
// C: 	dspio_set_uint_param(codec, 0x80, 0x0d, tmp);
// C: 	dspio_set_uint_param(codec, 0x80, 0x0e, tmp);
// C: 
// C: 	ca0113_mmio_gpio_set(codec, 0, false);
// C: 
// C: 	/* Internal loopback off */
// C: 	tmp = FLOAT_ONE;
// C: 	dspio_set_uint_param(codec, 0x37, 0x08, tmp);
// C: 	dspio_set_uint_param(codec, 0x37, 0x10, tmp);
// C: 
// C: 	/*remove DSP headroom*/
// C: 	tmp = FLOAT_ZERO;
// C: 	dspio_set_uint_param(codec, 0x96, 0x3C, tmp);
// C: 
// C: 	/* set WUH source */
// C: 	tmp = FLOAT_TWO;
// C: 	dspio_set_uint_param(codec, 0x31, 0x00, tmp);
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_WUH, SR_48_000);
// C: 
// C: 	/* Set speaker source? */
// C: 	dspio_set_uint_param(codec, 0x32, 0x00, tmp);
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x28, 0x00);
// C: 
// C: 	/*
// C: 	 * This is the second time we've called this, but this is seemingly
// C: 	 * what Windows does.
// C: 	 */
// C: 	ca0132_alt_init_analog_mics(codec);
// C: 
// C: 	ae7_post_dsp_asi_setup(codec);
// C: 
// C: 	/*
// C: 	 * Not sure why, but these are both set to 1. They're only set to 0
// C: 	 * upon shutdown.
// C: 	 */
// C: 	ca0113_mmio_gpio_set(codec, 0, true);
// C: 	ca0113_mmio_gpio_set(codec, 1, true);
// C: 
// C: 	/* Volume control related. */
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x0f, 0x04);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x10, 0x04);
// C: 	ca0113_mmio_command_set_type2(codec, 0x48, 0x07, 0x80);
// C: 
// C: 	/* out, in effects + voicefx */
// C: 	num_fx = OUT_EFFECTS_COUNT + IN_EFFECTS_COUNT + 1;
// C: 	for (idx = 0; idx < num_fx; idx++) {
// C: 		for (i = 0; i <= ca0132_effects[idx].params; i++) {
// C: 			dspio_set_uint_param(codec,
// C: 					ca0132_effects[idx].mid,
// C: 					ca0132_effects[idx].reqs[i],
// C: 					ca0132_effects[idx].def_vals[i]);
// C: 		}
// C: 	}
// C: 
// C: 	ca0132_alt_init_speaker_tuning(codec);
// C: }
// C: 
// C: /*
// C:  * Initialization of flags in chip
// C:  */
// C: static void ca0132_init_flags(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	if (ca0132_use_alt_functions(spec)) {
// C: 		chipio_set_control_flag(codec, CONTROL_FLAG_DSP_96KHZ, 1);
// C: 		chipio_set_control_flag(codec, CONTROL_FLAG_DAC_96KHZ, 1);
// C: 		chipio_set_control_flag(codec, CONTROL_FLAG_ADC_B_96KHZ, 1);
// C: 		chipio_set_control_flag(codec, CONTROL_FLAG_ADC_C_96KHZ, 1);
// C: 		chipio_set_control_flag(codec, CONTROL_FLAG_SRC_RATE_96KHZ, 1);
// C: 		chipio_set_control_flag(codec, CONTROL_FLAG_IDLE_ENABLE, 0);
// C: 		chipio_set_control_flag(codec, CONTROL_FLAG_SPDIF2OUT, 0);
// C: 		chipio_set_control_flag(codec,
// C: 				CONTROL_FLAG_PORT_D_10KOHM_LOAD, 0);
// C: 		chipio_set_control_flag(codec,
// C: 				CONTROL_FLAG_PORT_A_10KOHM_LOAD, 1);
// C: 	} else {
// C: 		chipio_set_control_flag(codec, CONTROL_FLAG_IDLE_ENABLE, 0);
// C: 		chipio_set_control_flag(codec,
// C: 				CONTROL_FLAG_PORT_A_COMMON_MODE, 0);
// C: 		chipio_set_control_flag(codec,
// C: 				CONTROL_FLAG_PORT_D_COMMON_MODE, 0);
// C: 		chipio_set_control_flag(codec,
// C: 				CONTROL_FLAG_PORT_A_10KOHM_LOAD, 0);
// C: 		chipio_set_control_flag(codec,
// C: 				CONTROL_FLAG_PORT_D_10KOHM_LOAD, 0);
// C: 		chipio_set_control_flag(codec, CONTROL_FLAG_ADC_C_HIGH_PASS, 1);
// C: 	}
// C: }
// C: 
// C: /*
// C:  * Initialization of parameters in chip
// C:  */
// C: static void ca0132_init_params(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	if (ca0132_use_alt_functions(spec)) {
// C: 		chipio_set_conn_rate(codec, MEM_CONNID_WUH, SR_48_000);
// C: 		chipio_set_conn_rate(codec, 0x0B, SR_48_000);
// C: 		chipio_set_control_param(codec, CONTROL_PARAM_SPDIF1_SOURCE, 0);
// C: 		chipio_set_control_param(codec, 0, 0);
// C: 		chipio_set_control_param(codec, CONTROL_PARAM_VIP_SOURCE, 0);
// C: 	}
// C: 
// C: 	chipio_set_control_param(codec, CONTROL_PARAM_PORTA_160OHM_GAIN, 6);
// C: 	chipio_set_control_param(codec, CONTROL_PARAM_PORTD_160OHM_GAIN, 6);
// C: }
// C: 
// C: static void ca0132_set_dsp_msr(struct hda_codec *codec, bool is96k)
// C: {
// C: 	chipio_set_control_flag(codec, CONTROL_FLAG_DSP_96KHZ, is96k);
// C: 	chipio_set_control_flag(codec, CONTROL_FLAG_DAC_96KHZ, is96k);
// C: 	chipio_set_control_flag(codec, CONTROL_FLAG_SRC_RATE_96KHZ, is96k);
// C: 	chipio_set_control_flag(codec, CONTROL_FLAG_SRC_CLOCK_196MHZ, is96k);
// C: 	chipio_set_control_flag(codec, CONTROL_FLAG_ADC_B_96KHZ, is96k);
// C: 	chipio_set_control_flag(codec, CONTROL_FLAG_ADC_C_96KHZ, is96k);
// C: 
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_MICIN1, SR_96_000);
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_MICOUT1, SR_96_000);
// C: 	chipio_set_conn_rate(codec, MEM_CONNID_WUH, SR_48_000);
// C: }
// C: 
// C: static bool ca0132_download_dsp_images(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	const struct dsp_image_seg *dsp_os_image;
// C: 	const struct firmware *fw_entry __free(firmware) = NULL;
// C: 	/*
// C: 	 * Alternate firmwares for different variants. The Recon3Di apparently
// C: 	 * can use the default firmware, but I'll leave the option in case
// C: 	 * it needs it again.
// C: 	 */
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_SBZ:
// C: 	case QUIRK_R3D:
// C: 	case QUIRK_AE5:
// C: 		if (request_firmware(&fw_entry, DESKTOP_EFX_FILE,
// C: 					codec->card->dev) != 0)
// C: 			codec_dbg(codec, "Desktop firmware not found.");
// C: 		else
// C: 			codec_dbg(codec, "Desktop firmware selected.");
// C: 		break;
// C: 	case QUIRK_R3DI:
// C: 		if (request_firmware(&fw_entry, R3DI_EFX_FILE,
// C: 					codec->card->dev) != 0)
// C: 			codec_dbg(codec, "Recon3Di alt firmware not detected.");
// C: 		else
// C: 			codec_dbg(codec, "Recon3Di firmware selected.");
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: 	/*
// C: 	 * Use default ctefx.bin if no alt firmware is detected, or if none
// C: 	 * exists for your particular codec.
// C: 	 */
// C: 	if (!fw_entry) {
// C: 		codec_dbg(codec, "Default firmware selected.");
// C: 		if (request_firmware(&fw_entry, EFX_FILE,
// C: 					codec->card->dev) != 0)
// C: 			return false;
// C: 	}
// C: 
// C: 	dsp_os_image = (struct dsp_image_seg *)(fw_entry->data);
// C: 	if (dspload_image(codec, dsp_os_image, 0, 0, true, 0)) {
// C: 		codec_err(codec, "ca0132 DSP load image failed\\n");
// C: 		return false;
// C: 	}
// C: 
// C: 	return dspload_wait_loaded(codec);
// C: }
// C: 
// C: static void ca0132_download_dsp(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: #ifndef CONFIG_SND_HDA_CODEC_CA0132_DSP
// C: 	return; /* NOP */
// C: #endif
// C: 
// C: 	if (spec->dsp_state == DSP_DOWNLOAD_FAILED)
// C: 		return; /* don't retry failures */
// C: 
// C: 	chipio_enable_clocks(codec);
// C: 	if (spec->dsp_state != DSP_DOWNLOADED) {
// C: 		spec->dsp_state = DSP_DOWNLOADING;
// C: 
// C: 		if (!ca0132_download_dsp_images(codec))
// C: 			spec->dsp_state = DSP_DOWNLOAD_FAILED;
// C: 		else
// C: 			spec->dsp_state = DSP_DOWNLOADED;
// C: 	}
// C: 
// C: 	/* For codecs using alt functions, this is already done earlier */
// C: 	if (spec->dsp_state == DSP_DOWNLOADED && !ca0132_use_alt_functions(spec))
// C: 		ca0132_set_dsp_msr(codec, true);
// C: }
// C: 
// C: static void ca0132_process_dsp_response(struct hda_codec *codec,
// C: 					struct hda_jack_callback *callback)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	codec_dbg(codec, "ca0132_process_dsp_response\\n");
// C: 	CLASS(snd_hda_power_pm, pm)(codec);
// C: 	if (spec->wait_scp) {
// C: 		if (dspio_get_response_data(codec) >= 0)
// C: 			spec->wait_scp = 0;
// C: 	}
// C: 
// C: 	dspio_clear_response_queue(codec);
// C: }
// C: 
// C: static void hp_callback(struct hda_codec *codec, struct hda_jack_callback *cb)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	struct hda_jack_tbl *tbl;
// C: 
// C: 	/* Delay enabling the HP amp, to let the mic-detection
// C: 	 * state machine run.
// C: 	 */
// C: 	tbl = snd_hda_jack_tbl_get(codec, cb->nid);
// C: 	if (tbl)
// C: 		tbl->block_report = 1;
// C: 	schedule_delayed_work(&spec->unsol_hp_work, msecs_to_jiffies(500));
// C: }
// C: 
// C: static void amic_callback(struct hda_codec *codec, struct hda_jack_callback *cb)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	if (ca0132_use_alt_functions(spec))
// C: 		ca0132_alt_select_in(codec);
// C: 	else
// C: 		ca0132_select_mic(codec);
// C: }
// C: 
// C: static void ca0132_setup_unsol(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	snd_hda_jack_detect_enable_callback(codec, spec->unsol_tag_hp, hp_callback);
// C: 	snd_hda_jack_detect_enable_callback(codec, spec->unsol_tag_amic1,
// C: 					    amic_callback);
// C: 	snd_hda_jack_detect_enable_callback(codec, UNSOL_TAG_DSP,
// C: 					    ca0132_process_dsp_response);
// C: 	/* Front headphone jack detection */
// C: 	if (ca0132_use_alt_functions(spec))
// C: 		snd_hda_jack_detect_enable_callback(codec,
// C: 			spec->unsol_tag_front_hp, hp_callback);
// C: }
// C: 
// C: /*
// C:  * Verbs tables.
// C:  */
// C: 
// C: /* Sends before DSP download. */
// C: static const struct hda_verb ca0132_base_init_verbs[] = {
// C: 	/*enable ct extension*/
// C: 	{0x15, VENDOR_CHIPIO_CT_EXTENSIONS_ENABLE, 0x1},
// C: 	{}
// C: };
// C: 
// C: /* Send at exit. */
// C: static const struct hda_verb ca0132_base_exit_verbs[] = {
// C: 	/*set afg to D3*/
// C: 	{0x01, AC_VERB_SET_POWER_STATE, 0x03},
// C: 	/*disable ct extension*/
// C: 	{0x15, VENDOR_CHIPIO_CT_EXTENSIONS_ENABLE, 0},
// C: 	{}
// C: };
// C: 
// C: /* Other verbs tables. Sends after DSP download. */
// C: 
// C: static const struct hda_verb ca0132_init_verbs0[] = {
// C: 	/* chip init verbs */
// C: 	{0x15, 0x70D, 0xF0},
// C: 	{0x15, 0x70E, 0xFE},
// C: 	{0x15, 0x707, 0x75},
// C: 	{0x15, 0x707, 0xD3},
// C: 	{0x15, 0x707, 0x09},
// C: 	{0x15, 0x707, 0x53},
// C: 	{0x15, 0x707, 0xD4},
// C: 	{0x15, 0x707, 0xEF},
// C: 	{0x15, 0x707, 0x75},
// C: 	{0x15, 0x707, 0xD3},
// C: 	{0x15, 0x707, 0x09},
// C: 	{0x15, 0x707, 0x02},
// C: 	{0x15, 0x707, 0x37},
// C: 	{0x15, 0x707, 0x78},
// C: 	{0x15, 0x53C, 0xCE},
// C: 	{0x15, 0x575, 0xC9},
// C: 	{0x15, 0x53D, 0xCE},
// C: 	{0x15, 0x5B7, 0xC9},
// C: 	{0x15, 0x70D, 0xE8},
// C: 	{0x15, 0x70E, 0xFE},
// C: 	{0x15, 0x707, 0x02},
// C: 	{0x15, 0x707, 0x68},
// C: 	{0x15, 0x707, 0x62},
// C: 	{0x15, 0x53A, 0xCE},
// C: 	{0x15, 0x546, 0xC9},
// C: 	{0x15, 0x53B, 0xCE},
// C: 	{0x15, 0x5E8, 0xC9},
// C: 	{}
// C: };
// C: 
// C: /* Extra init verbs for desktop cards. */
// C: static const struct hda_verb ca0132_init_verbs1[] = {
// C: 	{0x15, 0x70D, 0x20},
// C: 	{0x15, 0x70E, 0x19},
// C: 	{0x15, 0x707, 0x00},
// C: 	{0x15, 0x539, 0xCE},
// C: 	{0x15, 0x546, 0xC9},
// C: 	{0x15, 0x70D, 0xB7},
// C: 	{0x15, 0x70E, 0x09},
// C: 	{0x15, 0x707, 0x10},
// C: 	{0x15, 0x70D, 0xAF},
// C: 	{0x15, 0x70E, 0x09},
// C: 	{0x15, 0x707, 0x01},
// C: 	{0x15, 0x707, 0x05},
// C: 	{0x15, 0x70D, 0x73},
// C: 	{0x15, 0x70E, 0x09},
// C: 	{0x15, 0x707, 0x14},
// C: 	{0x15, 0x6FF, 0xC4},
// C: 	{}
// C: };
// C: 
// C: static void ca0132_init_chip(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	int num_fx;
// C: 	int i;
// C: 	unsigned int on;
// C: 
// C: 	mutex_init(&spec->chipio_mutex);
// C: 
// C: 	/*
// C: 	 * The Windows driver always does this upon startup, which seems to
// C: 	 * clear out any previous configuration. This should help issues where
// C: 	 * a boot into Windows prior to a boot into Linux breaks things. Also,
// C: 	 * Windows always sends the reset twice.
// C: 	 */
// C: 	if (ca0132_use_alt_functions(spec)) {
// C: 		chipio_set_control_flag(codec, CONTROL_FLAG_IDLE_ENABLE, 0);
// C: 		chipio_write_no_mutex(codec, 0x18b0a4, 0x000000c2);
// C: 
// C: 		snd_hda_codec_write(codec, codec->core.afg, 0,
// C: 			    AC_VERB_SET_CODEC_RESET, 0);
// C: 		snd_hda_codec_write(codec, codec->core.afg, 0,
// C: 			    AC_VERB_SET_CODEC_RESET, 0);
// C: 	}
// C: 
// C: 	spec->cur_out_type = SPEAKER_OUT;
// C: 	if (!ca0132_use_alt_functions(spec))
// C: 		spec->cur_mic_type = DIGITAL_MIC;
// C: 	else
// C: 		spec->cur_mic_type = REAR_MIC;
// C: 
// C: 	spec->cur_mic_boost = 0;
// C: 
// C: 	for (i = 0; i < VNODES_COUNT; i++) {
// C: 		spec->vnode_lvol[i] = 0x5a;
// C: 		spec->vnode_rvol[i] = 0x5a;
// C: 		spec->vnode_lswitch[i] = 0;
// C: 		spec->vnode_rswitch[i] = 0;
// C: 	}
// C: 
// C: 	/*
// C: 	 * Default states for effects are in ca0132_effects[].
// C: 	 */
// C: 	num_fx = OUT_EFFECTS_COUNT + IN_EFFECTS_COUNT;
// C: 	for (i = 0; i < num_fx; i++) {
// C: 		on = (unsigned int)ca0132_effects[i].reqs[0];
// C: 		spec->effects_switch[i] = on ? 1 : 0;
// C: 	}
// C: 	/*
// C: 	 * Sets defaults for the effect slider controls, only for alternative
// C: 	 * ca0132 codecs. Also sets x-bass crossover frequency to 80hz.
// C: 	 */
// C: 	if (ca0132_use_alt_controls(spec)) {
// C: 		/* Set speakers to default to full range. */
// C: 		spec->speaker_range_val[0] = 1;
// C: 		spec->speaker_range_val[1] = 1;
// C: 
// C: 		spec->xbass_xover_freq = 8;
// C: 		for (i = 0; i < EFFECT_LEVEL_SLIDERS; i++)
// C: 			spec->fx_ctl_val[i] = effect_slider_defaults[i];
// C: 
// C: 		spec->bass_redirect_xover_freq = 8;
// C: 	}
// C: 
// C: 	spec->voicefx_val = 0;
// C: 	spec->effects_switch[PLAY_ENHANCEMENT - EFFECT_START_NID] = 1;
// C: 	spec->effects_switch[CRYSTAL_VOICE - EFFECT_START_NID] = 0;
// C: 
// C: 	/*
// C: 	 * The ZxR doesn't have a front panel header, and it's line-in is on
// C: 	 * the daughter board. So, there is no input enum control, and we need
// C: 	 * to make sure that spec->in_enum_val is set properly.
// C: 	 */
// C: 	if (ca0132_quirk(spec) == QUIRK_ZXR)
// C: 		spec->in_enum_val = REAR_MIC;
// C: 
// C: #ifdef ENABLE_TUNING_CONTROLS
// C: 	ca0132_init_tuning_defaults(codec);
// C: #endif
// C: }
// C: 
// C: /*
// C:  * Recon3Di exit specific commands.
// C:  */
// C: /* prevents popping noise on shutdown */
// C: static void r3di_gpio_shutdown(struct hda_codec *codec)
// C: {
// C: 	snd_hda_codec_write(codec, 0x01, 0, AC_VERB_SET_GPIO_DATA, 0x00);
// C: }
// C: 
// C: /*
// C:  * Sound Blaster Z exit specific commands.
// C:  */
// C: static void sbz_region2_exit(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int i;
// C: 
// C: 	for (i = 0; i < 4; i++)
// C: 		writeb(0x0, spec->mem_base + 0x100);
// C: 	for (i = 0; i < 8; i++)
// C: 		writeb(0xb3, spec->mem_base + 0x304);
// C: 
// C: 	ca0113_mmio_gpio_set(codec, 0, false);
// C: 	ca0113_mmio_gpio_set(codec, 1, false);
// C: 	ca0113_mmio_gpio_set(codec, 4, true);
// C: 	ca0113_mmio_gpio_set(codec, 5, false);
// C: 	ca0113_mmio_gpio_set(codec, 7, false);
// C: }
// C: 
// C: static void sbz_set_pin_ctl_default(struct hda_codec *codec)
// C: {
// C: 	static const hda_nid_t pins[] = {0x0B, 0x0C, 0x0E, 0x12, 0x13};
// C: 	unsigned int i;
// C: 
// C: 	snd_hda_codec_write(codec, 0x11, 0,
// C: 			AC_VERB_SET_PIN_WIDGET_CONTROL, 0x40);
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(pins); i++)
// C: 		snd_hda_codec_write(codec, pins[i], 0,
// C: 				AC_VERB_SET_PIN_WIDGET_CONTROL, 0x00);
// C: }
// C: 
// C: static void ca0132_clear_unsolicited(struct hda_codec *codec)
// C: {
// C: 	static const hda_nid_t pins[] = {0x0B, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13};
// C: 	unsigned int i;
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(pins); i++) {
// C: 		snd_hda_codec_write(codec, pins[i], 0,
// C: 				AC_VERB_SET_UNSOLICITED_ENABLE, 0x00);
// C: 	}
// C: }
// C: 
// C: /* On shutdown, sends commands in sets of three */
// C: static void sbz_gpio_shutdown_commands(struct hda_codec *codec, int dir,
// C: 							int mask, int data)
// C: {
// C: 	if (dir >= 0)
// C: 		snd_hda_codec_write(codec, 0x01, 0,
// C: 				AC_VERB_SET_GPIO_DIRECTION, dir);
// C: 	if (mask >= 0)
// C: 		snd_hda_codec_write(codec, 0x01, 0,
// C: 				AC_VERB_SET_GPIO_MASK, mask);
// C: 
// C: 	if (data >= 0)
// C: 		snd_hda_codec_write(codec, 0x01, 0,
// C: 				AC_VERB_SET_GPIO_DATA, data);
// C: }
// C: 
// C: static void zxr_dbpro_power_state_shutdown(struct hda_codec *codec)
// C: {
// C: 	static const hda_nid_t pins[] = {0x05, 0x0c, 0x09, 0x0e, 0x08, 0x11, 0x01};
// C: 	unsigned int i;
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(pins); i++)
// C: 		snd_hda_codec_write(codec, pins[i], 0,
// C: 				AC_VERB_SET_POWER_STATE, 0x03);
// C: }
// C: 
// C: static void sbz_exit_chip(struct hda_codec *codec)
// C: {
// C: 	chipio_set_stream_control(codec, 0x03, 0);
// C: 	chipio_set_stream_control(codec, 0x04, 0);
// C: 
// C: 	/* Mess with GPIO */
// C: 	sbz_gpio_shutdown_commands(codec, 0x07, 0x07, -1);
// C: 	sbz_gpio_shutdown_commands(codec, 0x07, 0x07, 0x05);
// C: 	sbz_gpio_shutdown_commands(codec, 0x07, 0x07, 0x01);
// C: 
// C: 	chipio_set_stream_control(codec, 0x14, 0);
// C: 	chipio_set_stream_control(codec, 0x0C, 0);
// C: 
// C: 	chipio_set_conn_rate(codec, 0x41, SR_192_000);
// C: 	chipio_set_conn_rate(codec, 0x91, SR_192_000);
// C: 
// C: 	chipio_write(codec, 0x18a020, 0x00000083);
// C: 
// C: 	sbz_gpio_shutdown_commands(codec, 0x07, 0x07, 0x03);
// C: 	sbz_gpio_shutdown_commands(codec, 0x07, 0x07, 0x07);
// C: 	sbz_gpio_shutdown_commands(codec, 0x07, 0x07, 0x06);
// C: 
// C: 	chipio_set_stream_control(codec, 0x0C, 0);
// C: 
// C: 	chipio_set_control_param(codec, 0x0D, 0x24);
// C: 
// C: 	ca0132_clear_unsolicited(codec);
// C: 	sbz_set_pin_ctl_default(codec);
// C: 
// C: 	snd_hda_codec_write(codec, 0x0B, 0,
// C: 		AC_VERB_SET_EAPD_BTLENABLE, 0x00);
// C: 
// C: 	sbz_region2_exit(codec);
// C: }
// C: 
// C: static void r3d_exit_chip(struct hda_codec *codec)
// C: {
// C: 	ca0132_clear_unsolicited(codec);
// C: 	snd_hda_codec_write(codec, 0x01, 0, 0x793, 0x00);
// C: 	snd_hda_codec_write(codec, 0x01, 0, 0x794, 0x5b);
// C: }
// C: 
// C: static void ae5_exit_chip(struct hda_codec *codec)
// C: {
// C: 	chipio_set_stream_control(codec, 0x03, 0);
// C: 	chipio_set_stream_control(codec, 0x04, 0);
// C: 
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x32, 0x3f);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x07, 0x83);
// C: 	ca0113_mmio_command_set(codec, 0x48, 0x07, 0x83);
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x30, 0x00);
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x2b, 0x00);
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x2d, 0x00);
// C: 	ca0113_mmio_gpio_set(codec, 0, false);
// C: 	ca0113_mmio_gpio_set(codec, 1, false);
// C: 
// C: 	snd_hda_codec_write(codec, 0x01, 0, 0x793, 0x00);
// C: 	snd_hda_codec_write(codec, 0x01, 0, 0x794, 0x53);
// C: 
// C: 	chipio_set_control_param(codec, CONTROL_PARAM_ASI, 0);
// C: 
// C: 	chipio_set_stream_control(codec, 0x18, 0);
// C: 	chipio_set_stream_control(codec, 0x0c, 0);
// C: 
// C: 	snd_hda_codec_write(codec, 0x01, 0, 0x724, 0x83);
// C: }
// C: 
// C: static void ae7_exit_chip(struct hda_codec *codec)
// C: {
// C: 	chipio_set_stream_control(codec, 0x18, 0);
// C: 	chipio_set_stream_source_dest(codec, 0x21, 0xc8, 0xc8);
// C: 	chipio_set_stream_channels(codec, 0x21, 0);
// C: 	chipio_set_control_param(codec, CONTROL_PARAM_NODE_ID, 0x09);
// C: 	chipio_set_control_param(codec, 0x20, 0x01);
// C: 
// C: 	chipio_set_control_param(codec, CONTROL_PARAM_ASI, 0);
// C: 
// C: 	chipio_set_stream_control(codec, 0x18, 0);
// C: 	chipio_set_stream_control(codec, 0x0c, 0);
// C: 
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x2b, 0x00);
// C: 	snd_hda_codec_write(codec, 0x15, 0, 0x724, 0x83);
// C: 	ca0113_mmio_command_set_type2(codec, 0x48, 0x07, 0x83);
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x30, 0x00);
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x2e, 0x00);
// C: 	ca0113_mmio_gpio_set(codec, 0, false);
// C: 	ca0113_mmio_gpio_set(codec, 1, false);
// C: 	ca0113_mmio_command_set(codec, 0x30, 0x32, 0x3f);
// C: 
// C: 	snd_hda_codec_write(codec, 0x01, 0, 0x793, 0x00);
// C: 	snd_hda_codec_write(codec, 0x01, 0, 0x794, 0x53);
// C: }
// C: 
// C: static void zxr_exit_chip(struct hda_codec *codec)
// C: {
// C: 	chipio_set_stream_control(codec, 0x03, 0);
// C: 	chipio_set_stream_control(codec, 0x04, 0);
// C: 	chipio_set_stream_control(codec, 0x14, 0);
// C: 	chipio_set_stream_control(codec, 0x0C, 0);
// C: 
// C: 	chipio_set_conn_rate(codec, 0x41, SR_192_000);
// C: 	chipio_set_conn_rate(codec, 0x91, SR_192_000);
// C: 
// C: 	chipio_write(codec, 0x18a020, 0x00000083);
// C: 
// C: 	snd_hda_codec_write(codec, 0x01, 0, 0x793, 0x00);
// C: 	snd_hda_codec_write(codec, 0x01, 0, 0x794, 0x53);
// C: 
// C: 	ca0132_clear_unsolicited(codec);
// C: 	sbz_set_pin_ctl_default(codec);
// C: 	snd_hda_codec_write(codec, 0x0B, 0, AC_VERB_SET_EAPD_BTLENABLE, 0x00);
// C: 
// C: 	ca0113_mmio_gpio_set(codec, 5, false);
// C: 	ca0113_mmio_gpio_set(codec, 2, false);
// C: 	ca0113_mmio_gpio_set(codec, 3, false);
// C: 	ca0113_mmio_gpio_set(codec, 0, false);
// C: 	ca0113_mmio_gpio_set(codec, 4, true);
// C: 	ca0113_mmio_gpio_set(codec, 0, true);
// C: 	ca0113_mmio_gpio_set(codec, 5, true);
// C: 	ca0113_mmio_gpio_set(codec, 2, false);
// C: 	ca0113_mmio_gpio_set(codec, 3, false);
// C: }
// C: 
// C: static void ca0132_exit_chip(struct hda_codec *codec)
// C: {
// C: 	/* put any chip cleanup stuffs here. */
// C: 
// C: 	if (dspload_is_loaded(codec))
// C: 		dsp_reset(codec);
// C: }
// C: 
// C: /*
// C:  * This fixes a problem that was hard to reproduce. Very rarely, I would
// C:  * boot up, and there would be no sound, but the DSP indicated it had loaded
// C:  * properly. I did a few memory dumps to see if anything was different, and
// C:  * there were a few areas of memory uninitialized with a1a2a3a4. This function
// C:  * checks if those areas are uninitialized, and if they are, it'll attempt to
// C:  * reload the card 3 times. Usually it fixes by the second.
// C:  */
// C: static void sbz_dsp_startup_check(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int dsp_data_check[4];
// C: 	unsigned int cur_address = 0x390;
// C: 	unsigned int i;
// C: 	unsigned int failure = 0;
// C: 	unsigned int reload = 3;
// C: 
// C: 	if (spec->startup_check_entered)
// C: 		return;
// C: 
// C: 	spec->startup_check_entered = true;
// C: 
// C: 	for (i = 0; i < 4; i++) {
// C: 		chipio_read(codec, cur_address, &dsp_data_check[i]);
// C: 		cur_address += 0x4;
// C: 	}
// C: 	for (i = 0; i < 4; i++) {
// C: 		if (dsp_data_check[i] == 0xa1a2a3a4)
// C: 			failure = 1;
// C: 	}
// C: 
// C: 	codec_dbg(codec, "Startup Check: %d ", failure);
// C: 	if (failure)
// C: 		codec_info(codec, "DSP not initialized properly. Attempting to fix.");
// C: 	/*
// C: 	 * While the failure condition is true, and we haven't reached our
// C: 	 * three reload limit, continue trying to reload the driver and
// C: 	 * fix the issue.
// C: 	 */
// C: 	while (failure && (reload != 0)) {
// C: 		codec_info(codec, "Reloading... Tries left: %d", reload);
// C: 		sbz_exit_chip(codec);
// C: 		spec->dsp_state = DSP_DOWNLOAD_INIT;
// C: 		snd_hda_codec_init(codec);
// C: 		failure = 0;
// C: 		for (i = 0; i < 4; i++) {
// C: 			chipio_read(codec, cur_address, &dsp_data_check[i]);
// C: 			cur_address += 0x4;
// C: 		}
// C: 		for (i = 0; i < 4; i++) {
// C: 			if (dsp_data_check[i] == 0xa1a2a3a4)
// C: 				failure = 1;
// C: 		}
// C: 		reload--;
// C: 	}
// C: 
// C: 	if (!failure && reload < 3)
// C: 		codec_info(codec, "DSP fixed.");
// C: 
// C: 	if (!failure)
// C: 		return;
// C: 
// C: 	codec_info(codec, "DSP failed to initialize properly. Either try a full shutdown or a suspend to clear the internal memory.");
// C: }
// C: 
// C: /*
// C:  * This is for the extra volume verbs 0x797 (left) and 0x798 (right). These add
// C:  * extra precision for decibel values. If you had the dB value in floating point
// C:  * you would take the value after the decimal point, multiply by 64, and divide
// C:  * by 2. So for 8.59, it's (59 * 64) / 100. Useful if someone wanted to
// C:  * implement fixed point or floating point dB volumes. For now, I'll set them
// C:  * to 0 just incase a value has lingered from a boot into Windows.
// C:  */
// C: static void ca0132_alt_vol_setup(struct hda_codec *codec)
// C: {
// C: 	snd_hda_codec_write(codec, 0x02, 0, 0x797, 0x00);
// C: 	snd_hda_codec_write(codec, 0x02, 0, 0x798, 0x00);
// C: 	snd_hda_codec_write(codec, 0x03, 0, 0x797, 0x00);
// C: 	snd_hda_codec_write(codec, 0x03, 0, 0x798, 0x00);
// C: 	snd_hda_codec_write(codec, 0x04, 0, 0x797, 0x00);
// C: 	snd_hda_codec_write(codec, 0x04, 0, 0x798, 0x00);
// C: 	snd_hda_codec_write(codec, 0x07, 0, 0x797, 0x00);
// C: 	snd_hda_codec_write(codec, 0x07, 0, 0x798, 0x00);
// C: }
// C: 
// C: /*
// C:  * Extra commands that don't really fit anywhere else.
// C:  */
// C: static void sbz_pre_dsp_setup(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	writel(0x00820680, spec->mem_base + 0x01C);
// C: 	writel(0x00820680, spec->mem_base + 0x01C);
// C: 
// C: 	chipio_write(codec, 0x18b0a4, 0x000000c2);
// C: 
// C: 	snd_hda_codec_write(codec, 0x11, 0,
// C: 			AC_VERB_SET_PIN_WIDGET_CONTROL, 0x44);
// C: }
// C: 
// C: static void r3d_pre_dsp_setup(struct hda_codec *codec)
// C: {
// C: 	chipio_write(codec, 0x18b0a4, 0x000000c2);
// C: 
// C: 	chipio_8051_write_exram(codec, 0x1c1e, 0x5b);
// C: 
// C: 	snd_hda_codec_write(codec, 0x11, 0,
// C: 			AC_VERB_SET_PIN_WIDGET_CONTROL, 0x44);
// C: }
// C: 
// C: static void r3di_pre_dsp_setup(struct hda_codec *codec)
// C: {
// C: 	chipio_write(codec, 0x18b0a4, 0x000000c2);
// C: 
// C: 	chipio_8051_write_exram(codec, 0x1c1e, 0x5b);
// C: 	chipio_8051_write_exram(codec, 0x1920, 0x00);
// C: 	chipio_8051_write_exram(codec, 0x1921, 0x40);
// C: 
// C: 	snd_hda_codec_write(codec, 0x11, 0,
// C: 			AC_VERB_SET_PIN_WIDGET_CONTROL, 0x04);
// C: }
// C: 
// C: /*
// C:  * The ZxR seems to use alternative DAC's for the surround channels, which
// C:  * require PLL PMU setup for the clock rate, I'm guessing. Without setting
// C:  * this up, we get no audio out of the surround jacks.
// C:  */
// C: static void zxr_pre_dsp_setup(struct hda_codec *codec)
// C: {
// C: 	static const unsigned int addr[] = { 0x43, 0x40, 0x41, 0x42, 0x45 };
// C: 	static const unsigned int data[] = { 0x08, 0x0c, 0x0b, 0x07, 0x0d };
// C: 	unsigned int i;
// C: 
// C: 	chipio_write(codec, 0x189000, 0x0001f100);
// C: 	msleep(50);
// C: 	chipio_write(codec, 0x18900c, 0x0001f100);
// C: 	msleep(50);
// C: 
// C: 	/*
// C: 	 * This writes a RET instruction at the entry point of the function at
// C: 	 * 0xfa92 in exram. This function seems to have something to do with
// C: 	 * ASI. Might be some way to prevent the card from reconfiguring the
// C: 	 * ASI stuff itself.
// C: 	 */
// C: 	chipio_8051_write_exram(codec, 0xfa92, 0x22);
// C: 
// C: 	chipio_8051_write_pll_pmu(codec, 0x51, 0x98);
// C: 
// C: 	snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0, 0x725, 0x82);
// C: 	chipio_set_control_param(codec, CONTROL_PARAM_ASI, 3);
// C: 
// C: 	chipio_write(codec, 0x18902c, 0x00000000);
// C: 	msleep(50);
// C: 	chipio_write(codec, 0x18902c, 0x00000003);
// C: 	msleep(50);
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(addr); i++)
// C: 		chipio_8051_write_pll_pmu(codec, addr[i], data[i]);
// C: }
// C: 
// C: /*
// C:  * These are sent before the DSP is downloaded. Not sure
// C:  * what they do, or if they're necessary. Could possibly
// C:  * be removed. Figure they're better to leave in.
// C:  */
// C: static const unsigned int ca0113_mmio_init_address_sbz[] = {
// C: 	0x400, 0x408, 0x40c, 0x01c, 0xc0c, 0xc00, 0xc04, 0xc0c, 0xc0c, 0xc0c,
// C: 	0xc0c, 0xc08, 0xc08, 0xc08, 0xc08, 0xc08, 0xc04
// C: };
// C: 
// C: static const unsigned int ca0113_mmio_init_data_sbz[] = {
// C: 	0x00000030, 0x00000000, 0x00000003, 0x00000003, 0x00000003,
// C: 	0x00000003, 0x000000c1, 0x000000f1, 0x00000001, 0x000000c7,
// C: 	0x000000c1, 0x00000080
// C: };
// C: 
// C: static const unsigned int ca0113_mmio_init_data_zxr[] = {
// C: 	0x00000030, 0x00000000, 0x00000000, 0x00000003, 0x00000003,
// C: 	0x00000003, 0x00000001, 0x000000f1, 0x00000001, 0x000000c7,
// C: 	0x000000c1, 0x00000080
// C: };
// C: 
// C: static const unsigned int ca0113_mmio_init_address_ae5[] = {
// C: 	0x400, 0x42c, 0x46c, 0x4ac, 0x4ec, 0x43c, 0x47c, 0x4bc, 0x4fc, 0x408,
// C: 	0x100, 0x410, 0x40c, 0x100, 0x100, 0x830, 0x86c, 0x800, 0x86c, 0x800,
// C: 	0x804, 0x20c, 0x01c, 0xc0c, 0xc00, 0xc04, 0xc0c, 0xc0c, 0xc0c, 0xc0c,
// C: 	0xc08, 0xc08, 0xc08, 0xc08, 0xc08, 0xc04, 0x01c
// C: };
// C: 
// C: static const unsigned int ca0113_mmio_init_data_ae5[] = {
// C: 	0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
// C: 	0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000001,
// C: 	0x00000600, 0x00000014, 0x00000001, 0x0000060f, 0x0000070f,
// C: 	0x00000aff, 0x00000000, 0x0000006b, 0x00000001, 0x0000006b,
// C: 	0x00000057, 0x00800000, 0x00880680, 0x00000080, 0x00000030,
// C: 	0x00000000, 0x00000000, 0x00000003, 0x00000003, 0x00000003,
// C: 	0x00000001, 0x000000f1, 0x00000001, 0x000000c7, 0x000000c1,
// C: 	0x00000080, 0x00880680
// C: };
// C: 
// C: static void ca0132_mmio_init_sbz(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int tmp[2], i, count, cur_addr;
// C: 	const unsigned int *addr, *data;
// C: 
// C: 	addr = ca0113_mmio_init_address_sbz;
// C: 	for (i = 0; i < 3; i++)
// C: 		writel(0x00000000, spec->mem_base + addr[i]);
// C: 
// C: 	cur_addr = i;
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_ZXR:
// C: 		tmp[0] = 0x00880480;
// C: 		tmp[1] = 0x00000080;
// C: 		break;
// C: 	case QUIRK_SBZ:
// C: 		tmp[0] = 0x00820680;
// C: 		tmp[1] = 0x00000083;
// C: 		break;
// C: 	case QUIRK_R3D:
// C: 		tmp[0] = 0x00880680;
// C: 		tmp[1] = 0x00000083;
// C: 		break;
// C: 	default:
// C: 		tmp[0] = 0x00000000;
// C: 		tmp[1] = 0x00000000;
// C: 		break;
// C: 	}
// C: 
// C: 	for (i = 0; i < 2; i++)
// C: 		writel(tmp[i], spec->mem_base + addr[cur_addr + i]);
// C: 
// C: 	cur_addr += i;
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_ZXR:
// C: 		count = ARRAY_SIZE(ca0113_mmio_init_data_zxr);
// C: 		data = ca0113_mmio_init_data_zxr;
// C: 		break;
// C: 	default:
// C: 		count = ARRAY_SIZE(ca0113_mmio_init_data_sbz);
// C: 		data = ca0113_mmio_init_data_sbz;
// C: 		break;
// C: 	}
// C: 
// C: 	for (i = 0; i < count; i++)
// C: 		writel(data[i], spec->mem_base + addr[cur_addr + i]);
// C: }
// C: 
// C: static void ca0132_mmio_init_ae5(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	const unsigned int *addr, *data;
// C: 	unsigned int i, count;
// C: 
// C: 	addr = ca0113_mmio_init_address_ae5;
// C: 	data = ca0113_mmio_init_data_ae5;
// C: 	count = ARRAY_SIZE(ca0113_mmio_init_data_ae5);
// C: 
// C: 	if (ca0132_quirk(spec) == QUIRK_AE7) {
// C: 		writel(0x00000680, spec->mem_base + 0x1c);
// C: 		writel(0x00880680, spec->mem_base + 0x1c);
// C: 	}
// C: 
// C: 	for (i = 0; i < count; i++) {
// C: 		/*
// C: 		 * AE-7 shares all writes with the AE-5, except that it writes
// C: 		 * a different value to 0x20c.
// C: 		 */
// C: 		if (i == 21 && ca0132_quirk(spec) == QUIRK_AE7) {
// C: 			writel(0x00800001, spec->mem_base + addr[i]);
// C: 			continue;
// C: 		}
// C: 
// C: 		writel(data[i], spec->mem_base + addr[i]);
// C: 	}
// C: 
// C: 	if (ca0132_quirk(spec) == QUIRK_AE5)
// C: 		writel(0x00880680, spec->mem_base + 0x1c);
// C: }
// C: 
// C: static void ca0132_mmio_init(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_R3D:
// C: 	case QUIRK_SBZ:
// C: 	case QUIRK_ZXR:
// C: 		ca0132_mmio_init_sbz(codec);
// C: 		break;
// C: 	case QUIRK_AE5:
// C: 		ca0132_mmio_init_ae5(codec);
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: }
// C: 
// C: static const unsigned int ca0132_ae5_register_set_addresses[] = {
// C: 	0x304, 0x304, 0x304, 0x304, 0x100, 0x304, 0x100, 0x304, 0x100, 0x304,
// C: 	0x100, 0x304, 0x86c, 0x800, 0x86c, 0x800, 0x804
// C: };
// C: 
// C: static const unsigned char ca0132_ae5_register_set_data[] = {
// C: 	0x0f, 0x0e, 0x1f, 0x0c, 0x3f, 0x08, 0x7f, 0x00, 0xff, 0x00, 0x6b,
// C: 	0x01, 0x6b, 0x57
// C: };
// C: 
// C: /*
// C:  * This function writes to some SFR's, does some region2 writes, and then
// C:  * eventually resets the codec with the 0x7ff verb. Not quite sure why it does
// C:  * what it does.
// C:  */
// C: static void ae5_register_set(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	unsigned int count = ARRAY_SIZE(ca0132_ae5_register_set_addresses);
// C: 	const unsigned int *addr = ca0132_ae5_register_set_addresses;
// C: 	const unsigned char *data = ca0132_ae5_register_set_data;
// C: 	unsigned int i, cur_addr;
// C: 	unsigned char tmp[3];
// C: 
// C: 	if (ca0132_quirk(spec) == QUIRK_AE7)
// C: 		chipio_8051_write_pll_pmu(codec, 0x41, 0xc8);
// C: 
// C: 	chipio_8051_write_direct(codec, 0x93, 0x10);
// C: 	chipio_8051_write_pll_pmu(codec, 0x44, 0xc2);
// C: 
// C: 	if (ca0132_quirk(spec) == QUIRK_AE7) {
// C: 		tmp[0] = 0x03;
// C: 		tmp[1] = 0x03;
// C: 		tmp[2] = 0x07;
// C: 	} else {
// C: 		tmp[0] = 0x0f;
// C: 		tmp[1] = 0x0f;
// C: 		tmp[2] = 0x0f;
// C: 	}
// C: 
// C: 	for (i = cur_addr = 0; i < 3; i++, cur_addr++)
// C: 		writeb(tmp[i], spec->mem_base + addr[cur_addr]);
// C: 
// C: 	/*
// C: 	 * First writes are in single bytes, final are in 4 bytes. So, we use
// C: 	 * writeb, then writel.
// C: 	 */
// C: 	for (i = 0; cur_addr < 12; i++, cur_addr++)
// C: 		writeb(data[i], spec->mem_base + addr[cur_addr]);
// C: 
// C: 	for (; cur_addr < count; i++, cur_addr++)
// C: 		writel(data[i], spec->mem_base + addr[cur_addr]);
// C: 
// C: 	writel(0x00800001, spec->mem_base + 0x20c);
// C: 
// C: 	if (ca0132_quirk(spec) == QUIRK_AE7) {
// C: 		ca0113_mmio_command_set_type2(codec, 0x48, 0x07, 0x83);
// C: 		ca0113_mmio_command_set(codec, 0x30, 0x2e, 0x3f);
// C: 	} else {
// C: 		ca0113_mmio_command_set(codec, 0x30, 0x2d, 0x3f);
// C: 	}
// C: 
// C: 	chipio_8051_write_direct(codec, 0x90, 0x00);
// C: 	chipio_8051_write_direct(codec, 0x90, 0x10);
// C: 
// C: 	if (ca0132_quirk(spec) == QUIRK_AE5)
// C: 		ca0113_mmio_command_set(codec, 0x48, 0x07, 0x83);
// C: }
// C: 
// C: /*
// C:  * Extra init functions for alternative ca0132 codecs. Done
// C:  * here so they don't clutter up the main ca0132_init function
// C:  * anymore than they have to.
// C:  */
// C: static void ca0132_alt_init(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	ca0132_alt_vol_setup(codec);
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_SBZ:
// C: 		codec_dbg(codec, "SBZ alt_init");
// C: 		ca0132_gpio_init(codec);
// C: 		sbz_pre_dsp_setup(codec);
// C: 		snd_hda_sequence_write(codec, spec->chip_init_verbs);
// C: 		snd_hda_sequence_write(codec, spec->desktop_init_verbs);
// C: 		break;
// C: 	case QUIRK_R3DI:
// C: 		codec_dbg(codec, "R3DI alt_init");
// C: 		ca0132_gpio_init(codec);
// C: 		ca0132_gpio_setup(codec);
// C: 		r3di_gpio_dsp_status_set(codec, R3DI_DSP_DOWNLOADING);
// C: 		r3di_pre_dsp_setup(codec);
// C: 		snd_hda_sequence_write(codec, spec->chip_init_verbs);
// C: 		snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0, 0x6FF, 0xC4);
// C: 		break;
// C: 	case QUIRK_R3D:
// C: 		r3d_pre_dsp_setup(codec);
// C: 		snd_hda_sequence_write(codec, spec->chip_init_verbs);
// C: 		snd_hda_sequence_write(codec, spec->desktop_init_verbs);
// C: 		break;
// C: 	case QUIRK_AE5:
// C: 		ca0132_gpio_init(codec);
// C: 		chipio_8051_write_pll_pmu(codec, 0x49, 0x88);
// C: 		chipio_write(codec, 0x18b030, 0x00000020);
// C: 		snd_hda_sequence_write(codec, spec->chip_init_verbs);
// C: 		snd_hda_sequence_write(codec, spec->desktop_init_verbs);
// C: 		ca0113_mmio_command_set(codec, 0x30, 0x32, 0x3f);
// C: 		break;
// C: 	case QUIRK_AE7:
// C: 		ca0132_gpio_init(codec);
// C: 		chipio_8051_write_pll_pmu(codec, 0x49, 0x88);
// C: 		snd_hda_sequence_write(codec, spec->chip_init_verbs);
// C: 		snd_hda_sequence_write(codec, spec->desktop_init_verbs);
// C: 		chipio_write(codec, 0x18b008, 0x000000f8);
// C: 		chipio_write(codec, 0x18b008, 0x000000f0);
// C: 		chipio_write(codec, 0x18b030, 0x00000020);
// C: 		ca0113_mmio_command_set(codec, 0x30, 0x32, 0x3f);
// C: 		break;
// C: 	case QUIRK_ZXR:
// C: 		chipio_8051_write_pll_pmu(codec, 0x49, 0x88);
// C: 		snd_hda_sequence_write(codec, spec->chip_init_verbs);
// C: 		snd_hda_sequence_write(codec, spec->desktop_init_verbs);
// C: 		zxr_pre_dsp_setup(codec);
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: }
// C: 
// C: static int ca0132_init(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	struct auto_pin_cfg *cfg = &spec->autocfg;
// C: 	int i;
// C: 	bool dsp_loaded;
// C: 
// C: 	/*
// C: 	 * If the DSP is already downloaded, and init has been entered again,
// C: 	 * there's only two reasons for it. One, the codec has awaken from a
// C: 	 * suspended state, and in that case dspload_is_loaded will return
// C: 	 * false, and the init will be ran again. The other reason it gets
// C: 	 * re entered is on startup for some reason it triggers a suspend and
// C: 	 * resume state. In this case, it will check if the DSP is downloaded,
// C: 	 * and not run the init function again. For codecs using alt_functions,
// C: 	 * it will check if the DSP is loaded properly.
// C: 	 */
// C: 	if (spec->dsp_state == DSP_DOWNLOADED) {
// C: 		dsp_loaded = dspload_is_loaded(codec);
// C: 		if (!dsp_loaded) {
// C: 			spec->dsp_reload = true;
// C: 			spec->dsp_state = DSP_DOWNLOAD_INIT;
// C: 		} else {
// C: 			if (ca0132_quirk(spec) == QUIRK_SBZ)
// C: 				sbz_dsp_startup_check(codec);
// C: 			return 0;
// C: 		}
// C: 	}
// C: 
// C: 	if (spec->dsp_state != DSP_DOWNLOAD_FAILED)
// C: 		spec->dsp_state = DSP_DOWNLOAD_INIT;
// C: 	spec->curr_chip_addx = INVALID_CHIP_ADDRESS;
// C: 
// C: 	if (ca0132_use_pci_mmio(spec))
// C: 		ca0132_mmio_init(codec);
// C: 
// C: 	CLASS(snd_hda_power_pm, pm)(codec);
// C: 
// C: 	if (ca0132_quirk(spec) == QUIRK_AE5 || ca0132_quirk(spec) == QUIRK_AE7)
// C: 		ae5_register_set(codec);
// C: 
// C: 	ca0132_init_params(codec);
// C: 	ca0132_init_flags(codec);
// C: 
// C: 	snd_hda_sequence_write(codec, spec->base_init_verbs);
// C: 
// C: 	if (ca0132_use_alt_functions(spec))
// C: 		ca0132_alt_init(codec);
// C: 
// C: 	ca0132_download_dsp(codec);
// C: 
// C: 	ca0132_refresh_widget_caps(codec);
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_R3DI:
// C: 	case QUIRK_R3D:
// C: 		r3d_setup_defaults(codec);
// C: 		break;
// C: 	case QUIRK_SBZ:
// C: 	case QUIRK_ZXR:
// C: 		sbz_setup_defaults(codec);
// C: 		break;
// C: 	case QUIRK_AE5:
// C: 		ae5_setup_defaults(codec);
// C: 		break;
// C: 	case QUIRK_AE7:
// C: 		ae7_setup_defaults(codec);
// C: 		break;
// C: 	default:
// C: 		ca0132_setup_defaults(codec);
// C: 		ca0132_init_analog_mic2(codec);
// C: 		ca0132_init_dmic(codec);
// C: 		break;
// C: 	}
// C: 
// C: 	for (i = 0; i < spec->num_outputs; i++)
// C: 		init_output(codec, spec->out_pins[i], spec->dacs[0]);
// C: 
// C: 	init_output(codec, cfg->dig_out_pins[0], spec->dig_out);
// C: 
// C: 	for (i = 0; i < spec->num_inputs; i++)
// C: 		init_input(codec, spec->input_pins[i], spec->adcs[i]);
// C: 
// C: 	init_input(codec, cfg->dig_in_pin, spec->dig_in);
// C: 
// C: 	if (!ca0132_use_alt_functions(spec)) {
// C: 		snd_hda_sequence_write(codec, spec->chip_init_verbs);
// C: 		snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 			    VENDOR_CHIPIO_PARAM_EX_ID_SET, 0x0D);
// C: 		snd_hda_codec_write(codec, WIDGET_CHIP_CTRL, 0,
// C: 			    VENDOR_CHIPIO_PARAM_EX_VALUE_SET, 0x20);
// C: 	}
// C: 
// C: 	if (ca0132_quirk(spec) == QUIRK_SBZ)
// C: 		ca0132_gpio_setup(codec);
// C: 
// C: 	snd_hda_sequence_write(codec, spec->spec_init_verbs);
// C: 	if (ca0132_use_alt_functions(spec)) {
// C: 		ca0132_alt_select_out(codec);
// C: 		ca0132_alt_select_in(codec);
// C: 	} else {
// C: 		ca0132_select_out(codec);
// C: 		ca0132_select_mic(codec);
// C: 	}
// C: 
// C: 	snd_hda_jack_report_sync(codec);
// C: 
// C: 	/*
// C: 	 * Re set the PlayEnhancement switch on a resume event, because the
// C: 	 * controls will not be reloaded.
// C: 	 */
// C: 	if (spec->dsp_reload) {
// C: 		spec->dsp_reload = false;
// C: 		ca0132_pe_switch_set(codec);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int dbpro_init(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	struct auto_pin_cfg *cfg = &spec->autocfg;
// C: 	unsigned int i;
// C: 
// C: 	init_output(codec, cfg->dig_out_pins[0], spec->dig_out);
// C: 	init_input(codec, cfg->dig_in_pin, spec->dig_in);
// C: 
// C: 	for (i = 0; i < spec->num_inputs; i++)
// C: 		init_input(codec, spec->input_pins[i], spec->adcs[i]);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static void ca0132_free(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	cancel_delayed_work_sync(&spec->unsol_hp_work);
// C: 	snd_hda_power_up(codec);
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_SBZ:
// C: 		sbz_exit_chip(codec);
// C: 		break;
// C: 	case QUIRK_ZXR:
// C: 		zxr_exit_chip(codec);
// C: 		break;
// C: 	case QUIRK_R3D:
// C: 		r3d_exit_chip(codec);
// C: 		break;
// C: 	case QUIRK_AE5:
// C: 		ae5_exit_chip(codec);
// C: 		break;
// C: 	case QUIRK_AE7:
// C: 		ae7_exit_chip(codec);
// C: 		break;
// C: 	case QUIRK_R3DI:
// C: 		r3di_gpio_shutdown(codec);
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: 
// C: 	snd_hda_sequence_write(codec, spec->base_exit_verbs);
// C: 	ca0132_exit_chip(codec);
// C: 
// C: 	snd_hda_power_down(codec);
// C: #ifdef CONFIG_PCI
// C: 	if (spec->mem_base)
// C: 		pci_iounmap(codec->bus->pci, spec->mem_base);
// C: #endif
// C: 	kfree(spec->spec_init_verbs);
// C: 	kfree(codec->spec);
// C: 	codec->spec = NULL;
// C: }
// C: 
// C: static void dbpro_free(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	zxr_dbpro_power_state_shutdown(codec);
// C: 
// C: 	kfree(spec->spec_init_verbs);
// C: 	kfree(codec->spec);
// C: 	codec->spec = NULL;
// C: }
// C: 
// C: static void ca0132_config(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	spec->dacs[0] = 0x2;
// C: 	spec->dacs[1] = 0x3;
// C: 	spec->dacs[2] = 0x4;
// C: 
// C: 	spec->multiout.dac_nids = spec->dacs;
// C: 	spec->multiout.num_dacs = 3;
// C: 
// C: 	if (!ca0132_use_alt_functions(spec))
// C: 		spec->multiout.max_channels = 2;
// C: 	else
// C: 		spec->multiout.max_channels = 6;
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_ALIENWARE:
// C: 		codec_dbg(codec, "%s: QUIRK_ALIENWARE applied.\\n", __func__);
// C: 		snd_hda_apply_pincfgs(codec, alienware_pincfgs);
// C: 		break;
// C: 	case QUIRK_SBZ:
// C: 		codec_dbg(codec, "%s: QUIRK_SBZ applied.\\n", __func__);
// C: 		snd_hda_apply_pincfgs(codec, sbz_pincfgs);
// C: 		break;
// C: 	case QUIRK_ZXR:
// C: 		codec_dbg(codec, "%s: QUIRK_ZXR applied.\\n", __func__);
// C: 		snd_hda_apply_pincfgs(codec, zxr_pincfgs);
// C: 		break;
// C: 	case QUIRK_R3D:
// C: 		codec_dbg(codec, "%s: QUIRK_R3D applied.\\n", __func__);
// C: 		snd_hda_apply_pincfgs(codec, r3d_pincfgs);
// C: 		break;
// C: 	case QUIRK_R3DI:
// C: 		codec_dbg(codec, "%s: QUIRK_R3DI applied.\\n", __func__);
// C: 		snd_hda_apply_pincfgs(codec, r3di_pincfgs);
// C: 		break;
// C: 	case QUIRK_AE5:
// C: 		codec_dbg(codec, "%s: QUIRK_AE5 applied.\\n", __func__);
// C: 		snd_hda_apply_pincfgs(codec, ae5_pincfgs);
// C: 		break;
// C: 	case QUIRK_AE7:
// C: 		codec_dbg(codec, "%s: QUIRK_AE7 applied.\\n", __func__);
// C: 		snd_hda_apply_pincfgs(codec, ae7_pincfgs);
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_ALIENWARE:
// C: 		spec->num_outputs = 2;
// C: 		spec->out_pins[0] = 0x0b; /* speaker out */
// C: 		spec->out_pins[1] = 0x0f;
// C: 		spec->shared_out_nid = 0x2;
// C: 		spec->unsol_tag_hp = 0x0f;
// C: 
// C: 		spec->adcs[0] = 0x7; /* digital mic / analog mic1 */
// C: 		spec->adcs[1] = 0x8; /* analog mic2 */
// C: 		spec->adcs[2] = 0xa; /* what u hear */
// C: 
// C: 		spec->num_inputs = 3;
// C: 		spec->input_pins[0] = 0x12;
// C: 		spec->input_pins[1] = 0x11;
// C: 		spec->input_pins[2] = 0x13;
// C: 		spec->shared_mic_nid = 0x7;
// C: 		spec->unsol_tag_amic1 = 0x11;
// C: 		break;
// C: 	case QUIRK_SBZ:
// C: 	case QUIRK_R3D:
// C: 		spec->num_outputs = 2;
// C: 		spec->out_pins[0] = 0x0B; /* Line out */
// C: 		spec->out_pins[1] = 0x0F; /* Rear headphone out */
// C: 		spec->out_pins[2] = 0x10; /* Front Headphone / Center/LFE*/
// C: 		spec->out_pins[3] = 0x11; /* Rear surround */
// C: 		spec->shared_out_nid = 0x2;
// C: 		spec->unsol_tag_hp = spec->out_pins[1];
// C: 		spec->unsol_tag_front_hp = spec->out_pins[2];
// C: 
// C: 		spec->adcs[0] = 0x7; /* Rear Mic / Line-in */
// C: 		spec->adcs[1] = 0x8; /* Front Mic, but only if no DSP */
// C: 		spec->adcs[2] = 0xa; /* what u hear */
// C: 
// C: 		spec->num_inputs = 2;
// C: 		spec->input_pins[0] = 0x12; /* Rear Mic / Line-in */
// C: 		spec->input_pins[1] = 0x13; /* What U Hear */
// C: 		spec->shared_mic_nid = 0x7;
// C: 		spec->unsol_tag_amic1 = spec->input_pins[0];
// C: 
// C: 		/* SPDIF I/O */
// C: 		spec->dig_out = 0x05;
// C: 		spec->multiout.dig_out_nid = spec->dig_out;
// C: 		spec->dig_in = 0x09;
// C: 		break;
// C: 	case QUIRK_ZXR:
// C: 		spec->num_outputs = 2;
// C: 		spec->out_pins[0] = 0x0B; /* Line out */
// C: 		spec->out_pins[1] = 0x0F; /* Rear headphone out */
// C: 		spec->out_pins[2] = 0x10; /* Center/LFE */
// C: 		spec->out_pins[3] = 0x11; /* Rear surround */
// C: 		spec->shared_out_nid = 0x2;
// C: 		spec->unsol_tag_hp = spec->out_pins[1];
// C: 		spec->unsol_tag_front_hp = spec->out_pins[2];
// C: 
// C: 		spec->adcs[0] = 0x7; /* Rear Mic / Line-in */
// C: 		spec->adcs[1] = 0x8; /* Not connected, no front mic */
// C: 		spec->adcs[2] = 0xa; /* what u hear */
// C: 
// C: 		spec->num_inputs = 2;
// C: 		spec->input_pins[0] = 0x12; /* Rear Mic / Line-in */
// C: 		spec->input_pins[1] = 0x13; /* What U Hear */
// C: 		spec->shared_mic_nid = 0x7;
// C: 		spec->unsol_tag_amic1 = spec->input_pins[0];
// C: 		break;
// C: 	case QUIRK_ZXR_DBPRO:
// C: 		spec->adcs[0] = 0x8; /* ZxR DBPro Aux In */
// C: 
// C: 		spec->num_inputs = 1;
// C: 		spec->input_pins[0] = 0x11; /* RCA Line-in */
// C: 
// C: 		spec->dig_out = 0x05;
// C: 		spec->multiout.dig_out_nid = spec->dig_out;
// C: 
// C: 		spec->dig_in = 0x09;
// C: 		break;
// C: 	case QUIRK_AE5:
// C: 	case QUIRK_AE7:
// C: 		spec->num_outputs = 2;
// C: 		spec->out_pins[0] = 0x0B; /* Line out */
// C: 		spec->out_pins[1] = 0x11; /* Rear headphone out */
// C: 		spec->out_pins[2] = 0x10; /* Front Headphone / Center/LFE*/
// C: 		spec->out_pins[3] = 0x0F; /* Rear surround */
// C: 		spec->shared_out_nid = 0x2;
// C: 		spec->unsol_tag_hp = spec->out_pins[1];
// C: 		spec->unsol_tag_front_hp = spec->out_pins[2];
// C: 
// C: 		spec->adcs[0] = 0x7; /* Rear Mic / Line-in */
// C: 		spec->adcs[1] = 0x8; /* Front Mic, but only if no DSP */
// C: 		spec->adcs[2] = 0xa; /* what u hear */
// C: 
// C: 		spec->num_inputs = 2;
// C: 		spec->input_pins[0] = 0x12; /* Rear Mic / Line-in */
// C: 		spec->input_pins[1] = 0x13; /* What U Hear */
// C: 		spec->shared_mic_nid = 0x7;
// C: 		spec->unsol_tag_amic1 = spec->input_pins[0];
// C: 
// C: 		/* SPDIF I/O */
// C: 		spec->dig_out = 0x05;
// C: 		spec->multiout.dig_out_nid = spec->dig_out;
// C: 		break;
// C: 	case QUIRK_R3DI:
// C: 		spec->num_outputs = 2;
// C: 		spec->out_pins[0] = 0x0B; /* Line out */
// C: 		spec->out_pins[1] = 0x0F; /* Rear headphone out */
// C: 		spec->out_pins[2] = 0x10; /* Front Headphone / Center/LFE*/
// C: 		spec->out_pins[3] = 0x11; /* Rear surround */
// C: 		spec->shared_out_nid = 0x2;
// C: 		spec->unsol_tag_hp = spec->out_pins[1];
// C: 		spec->unsol_tag_front_hp = spec->out_pins[2];
// C: 
// C: 		spec->adcs[0] = 0x07; /* Rear Mic / Line-in */
// C: 		spec->adcs[1] = 0x08; /* Front Mic, but only if no DSP */
// C: 		spec->adcs[2] = 0x0a; /* what u hear */
// C: 
// C: 		spec->num_inputs = 2;
// C: 		spec->input_pins[0] = 0x12; /* Rear Mic / Line-in */
// C: 		spec->input_pins[1] = 0x13; /* What U Hear */
// C: 		spec->shared_mic_nid = 0x7;
// C: 		spec->unsol_tag_amic1 = spec->input_pins[0];
// C: 
// C: 		/* SPDIF I/O */
// C: 		spec->dig_out = 0x05;
// C: 		spec->multiout.dig_out_nid = spec->dig_out;
// C: 		break;
// C: 	default:
// C: 		spec->num_outputs = 2;
// C: 		spec->out_pins[0] = 0x0b; /* speaker out */
// C: 		spec->out_pins[1] = 0x10; /* headphone out */
// C: 		spec->shared_out_nid = 0x2;
// C: 		spec->unsol_tag_hp = spec->out_pins[1];
// C: 
// C: 		spec->adcs[0] = 0x7; /* digital mic / analog mic1 */
// C: 		spec->adcs[1] = 0x8; /* analog mic2 */
// C: 		spec->adcs[2] = 0xa; /* what u hear */
// C: 
// C: 		spec->num_inputs = 3;
// C: 		spec->input_pins[0] = 0x12;
// C: 		spec->input_pins[1] = 0x11;
// C: 		spec->input_pins[2] = 0x13;
// C: 		spec->shared_mic_nid = 0x7;
// C: 		spec->unsol_tag_amic1 = spec->input_pins[0];
// C: 
// C: 		/* SPDIF I/O */
// C: 		spec->dig_out = 0x05;
// C: 		spec->multiout.dig_out_nid = spec->dig_out;
// C: 		spec->dig_in = 0x09;
// C: 		break;
// C: 	}
// C: 
// C: 	/* Default HP/Speaker auto-detect from headphone pin verb: enable if the
// C: 	 * pin config indicates presence detect (not AC_DEFCFG_MISC_NO_PRESENCE).
// C: 	 */
// C: 	if (spec->unsol_tag_hp &&
// C: 	    (snd_hda_query_pin_caps(codec, spec->unsol_tag_hp) & AC_PINCAP_PRES_DETECT) &&
// C: 	    !(get_defcfg_misc(snd_hda_codec_get_pincfg(codec, spec->unsol_tag_hp)) &
// C: 	      AC_DEFCFG_MISC_NO_PRESENCE))
// C: 		spec->vnode_lswitch[VNID_HP_ASEL - VNODE_START_NID] = 1;
// C: }
// C: 
// C: static int ca0132_prepare_verbs(struct hda_codec *codec)
// C: {
// C: /* Verbs + terminator (an empty element) */
// C: #define NUM_SPEC_VERBS 2
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	spec->chip_init_verbs = ca0132_init_verbs0;
// C: 	/*
// C: 	 * Since desktop cards use pci_mmio, this can be used to determine
// C: 	 * whether or not to use these verbs instead of a separate bool.
// C: 	 */
// C: 	if (ca0132_use_pci_mmio(spec))
// C: 		spec->desktop_init_verbs = ca0132_init_verbs1;
// C: 	spec->spec_init_verbs = kzalloc_objs(struct hda_verb, NUM_SPEC_VERBS);
// C: 	if (!spec->spec_init_verbs)
// C: 		return -ENOMEM;
// C: 
// C: 	/* config EAPD */
// C: 	spec->spec_init_verbs[0].nid = 0x0b;
// C: 	spec->spec_init_verbs[0].param = 0x78D;
// C: 	spec->spec_init_verbs[0].verb = 0x00;
// C: 
// C: 	/* Previously commented configuration */
// C: 	/*
// C: 	spec->spec_init_verbs[2].nid = 0x0b;
// C: 	spec->spec_init_verbs[2].param = AC_VERB_SET_EAPD_BTLENABLE;
// C: 	spec->spec_init_verbs[2].verb = 0x02;
// C: 
// C: 	spec->spec_init_verbs[3].nid = 0x10;
// C: 	spec->spec_init_verbs[3].param = 0x78D;
// C: 	spec->spec_init_verbs[3].verb = 0x02;
// C: 
// C: 	spec->spec_init_verbs[4].nid = 0x10;
// C: 	spec->spec_init_verbs[4].param = AC_VERB_SET_EAPD_BTLENABLE;
// C: 	spec->spec_init_verbs[4].verb = 0x02;
// C: 	*/
// C: 
// C: 	/* Terminator: spec->spec_init_verbs[NUM_SPEC_VERBS-1] */
// C: 	return 0;
// C: }
// C: 
// C: /*
// C:  * The Sound Blaster ZxR shares the same PCI subsystem ID as some regular
// C:  * Sound Blaster Z cards. However, they have different HDA codec subsystem
// C:  * ID's. So, we check for the ZxR's subsystem ID, as well as the DBPro
// C:  * daughter boards ID.
// C:  */
// C: static void sbz_detect_quirk(struct hda_codec *codec)
// C: {
// C: 	switch (codec->core.subsystem_id) {
// C: 	case 0x11020033:
// C: 		codec->fixup_id = QUIRK_ZXR;
// C: 		break;
// C: 	case 0x1102003f:
// C: 		codec->fixup_id = QUIRK_ZXR_DBPRO;
// C: 		break;
// C: 	default:
// C: 		codec->fixup_id = QUIRK_SBZ;
// C: 		break;
// C: 	}
// C: }
// C: 
// C: static void ca0132_generic_init_hook(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	snd_hda_sequence_write(codec, spec->spec_init_verbs);
// C: }
// C: 
// C: static int ca0132_generic_probe(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 	struct auto_pin_cfg *cfg = &spec->gen.autocfg;
// C: 	int err;
// C: 
// C: 	snd_hda_gen_spec_init(&spec->gen);
// C: 
// C: 	snd_hda_apply_pincfgs(codec, ca0132_generic_pincfgs);
// C: 
// C: 	ca0132_init_chip(codec);
// C: 
// C: 	err = ca0132_prepare_verbs(codec);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	err = snd_hda_parse_pin_def_config(codec, cfg, NULL);
// C: 	if (err < 0)
// C: 		return err;
// C: 	err = snd_hda_gen_parse_auto_config(codec, cfg);
// C: 	if (err < 0)
// C: 		return err;
// C: 
// C: 	spec->gen.init_hook = ca0132_generic_init_hook;
// C: 	spec->gen.automute_speaker = 0;
// C: 	spec->gen.automute_lo = 0;
// C: 
// C: 	snd_hda_sequence_write(codec, spec->spec_init_verbs);
// C: 	return 0;
// C: }
// C: 
// C: static void ca0132_codec_remove(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_GENERIC:
// C: 		snd_hda_gen_remove(codec);
// C: 		return;
// C: 	case QUIRK_ZXR_DBPRO:
// C: 		return dbpro_free(codec);
// C: 	default:
// C: 		return ca0132_free(codec);
// C: 	}
// C: }
// C: 
// C: static int ca0132_codec_probe(struct hda_codec *codec,
// C: 			      const struct hda_device_id *id)
// C: {
// C: 	struct ca0132_spec *spec;
// C: 	int err;
// C: 
// C: 	codec_dbg(codec, "%s\\n", __func__);
// C: 
// C: 	spec = kzalloc_obj(*spec);
// C: 	if (!spec)
// C: 		return -ENOMEM;
// C: 	codec->spec = spec;
// C: 	spec->codec = codec;
// C: 
// C: 	/* These must be set before any path is taken */
// C: 	codec->pcm_format_first = 1;
// C: 	codec->no_sticky_stream = 1;
// C: 
// C: 	/* Detect codec quirk */
// C: 	snd_hda_pick_fixup(codec, ca0132_quirk_models, ca0132_quirks, NULL);
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_SBZ:
// C: 		sbz_detect_quirk(codec);
// C: 		break;
// C: 	case QUIRK_GENERIC:
// C: 		return ca0132_generic_probe(codec);
// C: 	default:
// C: 		break;
// C: 	}
// C: 
// C: 	spec->dsp_state = DSP_DOWNLOAD_INIT;
// C: 	spec->num_mixers = 1;
// C: 
// C: 	/* Set which mixers each quirk uses. */
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_SBZ:
// C: 		spec->mixers[0] = desktop_mixer;
// C: 		snd_hda_codec_set_name(codec, "Sound Blaster Z");
// C: 		break;
// C: 	case QUIRK_ZXR:
// C: 		spec->mixers[0] = desktop_mixer;
// C: 		snd_hda_codec_set_name(codec, "Sound Blaster ZxR");
// C: 		break;
// C: 	case QUIRK_ZXR_DBPRO:
// C: 		break;
// C: 	case QUIRK_R3D:
// C: 		spec->mixers[0] = desktop_mixer;
// C: 		snd_hda_codec_set_name(codec, "Recon3D");
// C: 		break;
// C: 	case QUIRK_R3DI:
// C: 		spec->mixers[0] = r3di_mixer;
// C: 		snd_hda_codec_set_name(codec, "Recon3Di");
// C: 		break;
// C: 	case QUIRK_AE5:
// C: 		spec->mixers[0] = desktop_mixer;
// C: 		snd_hda_codec_set_name(codec, "Sound BlasterX AE-5");
// C: 		break;
// C: 	case QUIRK_AE7:
// C: 		spec->mixers[0] = desktop_mixer;
// C: 		snd_hda_codec_set_name(codec, "Sound Blaster AE-7");
// C: 		break;
// C: 	default:
// C: 		spec->mixers[0] = ca0132_mixer;
// C: 		break;
// C: 	}
// C: 
// C: 	/* Setup whether or not to use alt functions/controls/pci_mmio */
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_SBZ:
// C: 	case QUIRK_R3D:
// C: 	case QUIRK_AE5:
// C: 	case QUIRK_AE7:
// C: 	case QUIRK_ZXR:
// C: 		spec->use_alt_controls = true;
// C: 		spec->use_alt_functions = true;
// C: 		spec->use_pci_mmio = true;
// C: 		break;
// C: 	case QUIRK_R3DI:
// C: 		spec->use_alt_controls = true;
// C: 		spec->use_alt_functions = true;
// C: 		spec->use_pci_mmio = false;
// C: 		break;
// C: 	default:
// C: 		spec->use_alt_controls = false;
// C: 		spec->use_alt_functions = false;
// C: 		spec->use_pci_mmio = false;
// C: 		break;
// C: 	}
// C: 
// C: #ifdef CONFIG_PCI
// C: 	if (spec->use_pci_mmio) {
// C: 		spec->mem_base = pci_iomap(codec->bus->pci, 2, 0xC20);
// C: 		if (spec->mem_base == NULL) {
// C: 			codec_warn(codec, "pci_iomap failed! Setting quirk to QUIRK_NONE.");
// C: 			codec->fixup_id = QUIRK_NONE;
// C: 		}
// C: 	}
// C: #endif
// C: 
// C: 	spec->base_init_verbs = ca0132_base_init_verbs;
// C: 	spec->base_exit_verbs = ca0132_base_exit_verbs;
// C: 
// C: 	INIT_DELAYED_WORK(&spec->unsol_hp_work, ca0132_unsol_hp_delayed);
// C: 
// C: 	ca0132_init_chip(codec);
// C: 
// C: 	ca0132_config(codec);
// C: 
// C: 	err = ca0132_prepare_verbs(codec);
// C: 	if (err < 0)
// C: 		goto error;
// C: 
// C: 	err = snd_hda_parse_pin_def_config(codec, &spec->autocfg, NULL);
// C: 	if (err < 0)
// C: 		goto error;
// C: 
// C: 	ca0132_setup_unsol(codec);
// C: 
// C: 	return 0;
// C: 
// C:  error:
// C: 	ca0132_codec_remove(codec);
// C: 	return err;
// C: }
// C: 
// C: static int ca0132_codec_build_controls(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_GENERIC:
// C: 		return snd_hda_gen_build_controls(codec);
// C: 	case QUIRK_ZXR_DBPRO:
// C: 		return dbpro_build_controls(codec);
// C: 	default:
// C: 		return ca0132_build_controls(codec);
// C: 	}
// C: }
// C: 
// C: static int ca0132_codec_build_pcms(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_GENERIC:
// C: 		return snd_hda_gen_build_pcms(codec);
// C: 	case QUIRK_ZXR_DBPRO:
// C: 		return dbpro_build_pcms(codec);
// C: 	default:
// C: 		return ca0132_build_pcms(codec);
// C: 	}
// C: }
// C: 
// C: static int ca0132_codec_init(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	switch (ca0132_quirk(spec)) {
// C: 	case QUIRK_GENERIC:
// C: 		return snd_hda_gen_init(codec);
// C: 	case QUIRK_ZXR_DBPRO:
// C: 		return dbpro_init(codec);
// C: 	default:
// C: 		return ca0132_init(codec);
// C: 	}
// C: }
// C: 
// C: static int ca0132_codec_suspend(struct hda_codec *codec)
// C: {
// C: 	struct ca0132_spec *spec = codec->spec;
// C: 
// C: 	if (ca0132_quirk(spec) == QUIRK_GENERIC)
// C: 		return 0;
// C: 
// C: 	cancel_delayed_work_sync(&spec->unsol_hp_work);
// C: 	return 0;
// C: }
// C: 
// C: static const struct hda_codec_ops ca0132_codec_ops = {
// C: 	.probe = ca0132_codec_probe,
// C: 	.remove = ca0132_codec_remove,
// C: 	.build_controls = ca0132_codec_build_controls,
// C: 	.build_pcms = ca0132_codec_build_pcms,
// C: 	.init = ca0132_codec_init,
// C: 	.unsol_event = snd_hda_jack_unsol_event,
// C: 	.suspend = ca0132_codec_suspend,
// C: };
// C: 
// C: /*
// C:  * driver entries
// C:  */
// C: static const struct hda_device_id snd_hda_id_ca0132[] = {
// C: 	HDA_CODEC_ID(0x11020011, "CA0132"),
// C: 	{} /* terminator */
// C: };
// C: MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_ca0132);
// C: 
// C: MODULE_LICENSE("GPL");
// C: MODULE_DESCRIPTION("Creative Sound Core3D codec");
// C: 
// C: static struct hda_codec_driver ca0132_driver = {
// C: 	.id = snd_hda_id_ca0132,
// C: 	.ops = &ca0132_codec_ops,
// C: };
// C: 
// C: module_hda_codec_driver(ca0132_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
