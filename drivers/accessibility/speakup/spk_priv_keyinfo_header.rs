/* SPDX-License-Identifier: GPL-2.0+ */
/* spk_priv.h
 * review functions for the speakup screen review package.
 * originally written by: Kirk Reiser and Andy Berdan.
 *
 * extensively modified by David Borowski.
 *
 * Copyright (C) 1998  Kirk Reiser.
 * Copyright (C) 2003  David Borowski.
 */

pub const FIRST_SYNTH_VAR: i32 = RATE;
/* 0 is reserved for no remap */
pub const SPEAKUP_GOTO: i32 = 0x01;
pub const SPEECH_KILL: i32 = 0x02;
pub const SPEAKUP_QUIET: i32 = 0x03;
pub const SPEAKUP_CUT: i32 = 0x04;
pub const SPEAKUP_PASTE: i32 = 0x05;
pub const SAY_FIRST_CHAR: i32 = 0x06;
pub const SAY_LAST_CHAR: i32 = 0x07;
pub const SAY_CHAR: i32 = 0x08;
pub const SAY_PREV_CHAR: i32 = 0x09;
pub const SAY_NEXT_CHAR: i32 = 0x0a;
pub const SAY_WORD: i32 = 0x0b;
pub const SAY_PREV_WORD: i32 = 0x0c;
pub const SAY_NEXT_WORD: i32 = 0x0d;
pub const SAY_LINE: i32 = 0x0e;
pub const SAY_PREV_LINE: i32 = 0x0f;
pub const SAY_NEXT_LINE: i32 = 0x10;
pub const TOP_EDGE: i32 = 0x11;
pub const BOTTOM_EDGE: i32 = 0x12;
pub const LEFT_EDGE: i32 = 0x13;
pub const RIGHT_EDGE: i32 = 0x14;
pub const SPELL_PHONETIC: i32 = 0x15;
pub const SPELL_WORD: i32 = 0x16;
pub const SAY_SCREEN: i32 = 0x17;
pub const SAY_POSITION: i32 = 0x18;
pub const SAY_ATTRIBUTES: i32 = 0x19;
pub const SPEAKUP_OFF: i32 = 0x1a;
pub const SPEAKUP_PARKED: i32 = 0x1b;
pub const SAY_LINE_INDENT: i32 = 0x1c;
pub const SAY_FROM_TOP: i32 = 0x1d;
pub const SAY_TO_BOTTOM: i32 = 0x1e;
pub const SAY_FROM_LEFT: i32 = 0x1f;
pub const SAY_TO_RIGHT: i32 = 0x20;
pub const SAY_CHAR_NUM: i32 = 0x21;
pub const EDIT_SOME: i32 = 0x22;
pub const EDIT_MOST: i32 = 0x23;
pub const SAY_PHONETIC_CHAR: i32 = 0x24;
pub const EDIT_DELIM: i32 = 0x25;
pub const EDIT_REPEAT: i32 = 0x26;
pub const EDIT_EXNUM: i32 = 0x27;
pub const SET_WIN: i32 = 0x28;
pub const CLEAR_WIN: i32 = 0x29;
pub const ENABLE_WIN: i32 = 0x2a;
pub const SAY_WIN: i32 = 0x2b;
pub const SPK_LOCK: i32 = 0x2c;
pub const SPEAKUP_HELP: i32 = 0x2d;
pub const TOGGLE_CURSORING: i32 = 0x2e;
pub const READ_ALL_DOC: i32 = 0x2f;

/* one greater than the last func handler */
pub const SPKUP_MAX_FUNC: i32 = 0x30;

pub const SPK_KEY: i32 = 0x80;
pub const FIRST_EDIT_BITS: i32 = 0x22;
pub const FIRST_SET_VAR: i32 = SPELL_DELAY;

/* increase if adding more than 0x3f functions */
pub const VAR_START: i32 = 0x40;

/* keys for setting variables, must be ordered same as the enum for var_ids */
/* with dec being even and inc being 1 greater */
pub const SPELL_DELAY_DEC: i32 = VAR_START + 0;
pub const SPELL_DELAY_INC: i32 = SPELL_DELAY_DEC + 1;
pub const PUNC_LEVEL_DEC: i32 = SPELL_DELAY_DEC + 2;
pub const PUNC_LEVEL_INC: i32 = PUNC_LEVEL_DEC + 1;
pub const READING_PUNC_DEC: i32 = PUNC_LEVEL_DEC + 2;
pub const READING_PUNC_INC: i32 = READING_PUNC_DEC + 1;
pub const ATTRIB_BLEEP_DEC: i32 = READING_PUNC_DEC + 2;
pub const ATTRIB_BLEEP_INC: i32 = ATTRIB_BLEEP_DEC + 1;
pub const BLEEPS_DEC: i32 = ATTRIB_BLEEP_DEC + 2;
pub const BLEEPS_INC: i32 = BLEEPS_DEC + 1;
pub const RATE_DEC: i32 = BLEEPS_DEC + 2;
pub const RATE_INC: i32 = RATE_DEC + 1;
pub const PITCH_DEC: i32 = RATE_DEC + 2;
pub const PITCH_INC: i32 = PITCH_DEC + 1;
pub const VOL_DEC: i32 = PITCH_DEC + 2;
pub const VOL_INC: i32 = VOL_DEC + 1;
pub const TONE_DEC: i32 = VOL_DEC + 2;
pub const TONE_INC: i32 = TONE_DEC + 1;
pub const PUNCT_DEC: i32 = TONE_DEC + 2;
pub const PUNCT_INC: i32 = PUNCT_DEC + 1;
pub const VOICE_DEC: i32 = PUNCT_DEC + 2;
pub const VOICE_INC: i32 = VOICE_DEC + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
