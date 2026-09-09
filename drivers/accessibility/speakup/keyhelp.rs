// SPDX-License-Identifier: GPL-2.0+
/* speakup_keyhelp.c
 * help module for speakup
 *
 *written by David Borowski.
 *
 *  Copyright (C) 2003  David Borowski.
 */

// Dependencies supplied by the surrounding kernel/Speakup translation unit.

const MAXFUNCS: usize = 130;
const MAXKEYS: usize = 256;
static NUM_KEY_NAMES: i32 = MSG_KEYNAMES_END - MSG_KEYNAMES_START + 1;
static mut KEY_OFFSETS: [u16; MAXFUNCS] = [0; MAXFUNCS];
static mut KEY_DATA: [u16; MAXKEYS] = [0; MAXKEYS];
static mut MASKS: [u16; 6] = [32, 16, 8, 4, 2, 1];

static mut LETTER_OFFSETS: [i16; 26] = [-1; 26];

static FUNCVALS: [u8; 69] = [
    ATTRIB_BLEEP_DEC, ATTRIB_BLEEP_INC, BLEEPS_DEC, BLEEPS_INC,
    SAY_FIRST_CHAR, SAY_LAST_CHAR, SAY_CHAR, SAY_CHAR_NUM,
    SAY_NEXT_CHAR, SAY_PHONETIC_CHAR, SAY_PREV_CHAR, SPEAKUP_PARKED,
    SPEAKUP_CUT, EDIT_DELIM, EDIT_EXNUM, EDIT_MOST,
    EDIT_REPEAT, EDIT_SOME, SPEAKUP_GOTO, BOTTOM_EDGE,
    LEFT_EDGE, RIGHT_EDGE, TOP_EDGE, SPEAKUP_HELP,
    SAY_LINE, SAY_NEXT_LINE, SAY_PREV_LINE, SAY_LINE_INDENT,
    SPEAKUP_PASTE, PITCH_DEC, PITCH_INC, PUNCT_DEC,
    PUNCT_INC, PUNC_LEVEL_DEC, PUNC_LEVEL_INC, SPEAKUP_QUIET,
    RATE_DEC, RATE_INC, READING_PUNC_DEC, READING_PUNC_INC,
    SAY_ATTRIBUTES, SAY_FROM_LEFT, SAY_FROM_TOP, SAY_POSITION,
    SAY_SCREEN, SAY_TO_BOTTOM, SAY_TO_RIGHT, SPK_KEY,
    SPK_LOCK, SPEAKUP_OFF, SPEECH_KILL, SPELL_DELAY_DEC,
    SPELL_DELAY_INC, SPELL_WORD, SPELL_PHONETIC, TONE_DEC,
    TONE_INC, VOICE_DEC, VOICE_INC, VOL_DEC,
    VOL_INC, CLEAR_WIN, SAY_WIN, SET_WIN,
    ENABLE_WIN, SAY_WORD, SAY_NEXT_WORD, SAY_PREV_WORD, 0,
];

static mut STATE_TBL: *mut u8 = core::ptr::null_mut();
static mut CUR_ITEM: i32 = 0;
static mut NSTATES: i32 = 0;

unsafe fn build_key_data() {
    let mut counters = [0u8; MAXFUNCS];
    let mut ch: u8;
    let mut ch1: u8;
    let mut offset: i32 = 1;

    NSTATES = *STATE_TBL.offset(-1) as i32;
    KEY_OFFSETS = [0; MAXFUNCS];
    let mut kp = STATE_TBL.add(NSTATES as usize + 1);
    while { ch = *kp; kp = kp.add(1); ch != 0 } {
        for i in 0..NSTATES as isize {
            ch1 = *kp; kp = kp.add(1);
            if ch1 == 0 { continue; }
            if (*STATE_TBL.offset(i) & 16) != 0 && ch1 == SPK_KEY { continue; }
            counters[ch1 as usize] = counters[ch1 as usize].wrapping_add(1);
        }
    }
    for i in 0..MAXFUNCS {
        if counters[i] == 0 { continue; }
        KEY_OFFSETS[i] = offset as u16;
        offset += counters[i] as i32 + 1;
        if offset >= MAXKEYS as i32 { break; }
    }
    kp = STATE_TBL.add(NSTATES as usize + 1);
    while { ch = *kp; kp = kp.add(1); ch != 0 } {
        for i in 0..NSTATES as isize {
            ch1 = *kp; kp = kp.add(1);
            if ch1 == 0 { continue; }
            if (*STATE_TBL.offset(i) & 16) != 0 && ch1 == SPK_KEY { continue; }
            let key = ((*STATE_TBL.offset(i) as u16) << 8).wrapping_add(ch as u16);
            counters[ch1 as usize] = counters[ch1 as usize].wrapping_sub(1);
            offset = KEY_OFFSETS[ch1 as usize] as i32;
            if offset == 0 { continue; }
            let p_key = KEY_DATA.as_mut_ptr().add(offset as usize + counters[ch1 as usize] as usize);
            *p_key = key;
        }
    }
}

unsafe fn say_key(mut key: i32) {
    let state = key >> 8;
    key &= 0xff;
    for i in 0..6 {
        if (state & MASKS[i] as i32) != 0 { synth_printf(" %s", spk_msg_get(MSG_STATES_START + i as i32)); }
    }
    if key > 0 && key <= NUM_KEY_NAMES { synth_printf(" %s\n", spk_msg_get(MSG_KEYNAMES_START + key - 1)); }
}

unsafe fn help_init() {
    let mut start = SPACE as u8;
    let num_funcs = MSG_FUNCNAMES_END - MSG_FUNCNAMES_START + 1;
    STATE_TBL = spk_our_keys[0].add(SHIFT_TBL_SIZE + 2);
    for i in 0..num_funcs {
        let cur_funcname = spk_msg_get(MSG_FUNCNAMES_START + i);
        let first_letter = (*cur_funcname as char).to_ascii_lowercase() as u8;
        if first_letter < b'a' || first_letter > b'z' || start == first_letter { continue; }
        start = first_letter;
        LETTER_OFFSETS[(start & 31) as usize - 1] = i as i16;
    }
}

pub unsafe fn spk_handle_help(vc: *mut vc_data, mut typ: u8, mut ch: u8, mut key: u16) -> i32 {
    let mut name: *mut i8;
    if LETTER_OFFSETS[0] == -1 { help_init(); }
    if typ == KT_LATIN {
        if ch == SPACE { spk_special_handler = None; synth_printf("%s\n", spk_msg_get(MSG_LEAVING_HELP)); return 1; }
        ch = (ch as char).to_ascii_lowercase() as u8;
        if ch < b'a' || ch > b'z' { return -1; }
        if LETTER_OFFSETS[(ch - b'a') as usize] == -1 { synth_printf(spk_msg_get(MSG_NO_COMMAND), ch); synth_printf("\n"); return 1; }
        CUR_ITEM = LETTER_OFFSETS[(ch - b'a') as usize] as i32;
    } else if typ == KT_CUR {
        if ch == 0 && MSG_FUNCNAMES_START + CUR_ITEM + 1 <= MSG_FUNCNAMES_END { CUR_ITEM += 1; }
        else if ch == 3 && CUR_ITEM > 0 { CUR_ITEM -= 1; } else { return -1; }
    } else if typ == KT_SPKUP && ch == SPEAKUP_HELP && spk_special_handler.is_none() {
        spk_special_handler = Some(spk_handle_help); synth_printf("%s\n", spk_msg_get(MSG_HELP_INFO)); build_key_data(); return 1;
    } else {
        name = core::ptr::null_mut();
        if typ != KT_SPKUP && key > 0 && key as i32 <= NUM_KEY_NAMES { synth_printf("%s\n", spk_msg_get(MSG_KEYNAMES_START + key as i32 - 1)); return 1; }
        for i in 0..FUNCVALS.len() { if FUNCVALS[i] == 0 { break; } if ch == FUNCVALS[i] { name = spk_msg_get(MSG_FUNCNAMES_START + i as i32); break; } }
        if name.is_null() { return -1; }
        let kp = spk_our_keys[key as usize].add(1);
        let mut i = 0; while i < NSTATES as usize && ch != *kp.add(i) { i += 1; }
        key = key.wrapping_add((*STATE_TBL.add(i) as u16) << 8); say_key(key as i32); synth_printf(spk_msg_get(MSG_KEYDESC), name); synth_printf("\n"); return 1;
    }
    name = spk_msg_get(MSG_FUNCNAMES_START + CUR_ITEM); let func = FUNCVALS[CUR_ITEM as usize]; synth_printf("%s", name);
    if KEY_OFFSETS[func as usize] == 0 { synth_printf(" %s\n", spk_msg_get(MSG_IS_UNASSIGNED)); return 1; }
    let p_keys = KEY_DATA.as_ptr().add(KEY_OFFSETS[func as usize] as usize); let mut n = 0;
    while *p_keys.add(n) != 0 { if n > 0 { synth_printf("%s ", spk_msg_get(MSG_DISJUNCTION)); } say_key(*p_keys.add(n) as i32); n += 1; }
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
