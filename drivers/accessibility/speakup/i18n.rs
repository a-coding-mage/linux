// SPDX-License-Identifier: GPL-2.0
/* Internationalization implementation. */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

// External symbols and types are supplied by the surrounding kernel translation.
extern "C" {
    static mut speakup_info: SpeakupInfo;
    fn kmemdup_nul(src: *const c_void, len: usize, flags: c_int) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: usize);
}

#[repr(C)]
pub struct SpeakupInfo { pub spinlock: c_void }
#[repr(C)]
pub struct MsgGroup { pub name: *const c_char, pub start: MsgIndex, pub end: MsgIndex }
pub type MsgIndex = usize;

// Message indices are declared by the translated speakup headers.
extern "C" {
    static mut MSG_FIRST_INDEX: MsgIndex;
    static mut MSG_LAST_INDEX: MsgIndex;
}

static mut SPEAKUP_MSGS: [*mut c_char; 1024] = [ptr::null_mut(); 1024];
static mut SPEAKUP_DEFAULT_MSGS: [*mut c_char; 1024] = [ptr::null_mut(); 1024];

const DEFAULT_MESSAGES: &[(&str, &str)] = &[
    (MSG_BLANK, "blank"),\n    (MSG_IAM_ALIVE, "I'm aLive!"),\n    (MSG_YOU_KILLED_SPEAKUP, "You killed speakup!"),\n    (MSG_HEY_THATS_BETTER, "hey. That's better!"),\n    (MSG_YOU_TURNED_ME_OFF, "You turned me off!"),\n    (MSG_PARKED, "parked!"),\n    (MSG_UNPARKED, "unparked!"),\n    (MSG_MARK, "mark"),\n    (MSG_CUT, "cut"),\n    (MSG_MARK_CLEARED, "mark, cleared"),\n    (MSG_PASTE, "paste"),\n    (MSG_BRIGHT, "bright"),\n    (MSG_ON_BLINKING, "on blinking"),\n    (MSG_OFF, "off"),\n    (MSG_ON, "on"),\n    (MSG_NO_WINDOW, "no window"),\n    (MSG_CURSORING_OFF, "cursoring off"),\n    (MSG_CURSORING_ON, "cursoring on"),\n    (MSG_HIGHLIGHT_TRACKING, "highlight tracking"),\n    (MSG_READ_WINDOW, "read window"),\n    (MSG_READ_ALL, "read all"),\n    (MSG_EDIT_DONE, "edit done"),\n    (MSG_WINDOW_ALREADY_SET, "window already set, clear then reset"),\n    (MSG_END_BEFORE_START, "error end before start"),\n    (MSG_WINDOW_CLEARED, "window cleared"),\n    (MSG_WINDOW_SILENCED, "window silenced"),\n    (MSG_WINDOW_SILENCE_DISABLED, "window silence disabled"),\n    (MSG_ERROR, "error"),\n    (MSG_GOTO_CANCELED, "goto canceled"),\n    (MSG_GOTO, "go to?"),\n    (MSG_LEAVING_HELP, "leaving help"),\n    (MSG_IS_UNASSIGNED, "is unassigned"),\n    (MSG_HELP_INFO, "press space to exit, up or down to scroll, or a letter to go to a command"),\n    (MSG_EDGE_TOP, "top,"),\n    (MSG_EDGE_BOTTOM, "bottom,"),\n    (MSG_EDGE_LEFT, "left,"),\n    (MSG_EDGE_RIGHT, "right,"),\n    (MSG_NUMBER, "number"),\n    (MSG_SPACE, "space"),\n    (MSG_START, "start"),\n    (MSG_END, "end"),\n    (MSG_CTRL, "control-"),\n    (MSG_DISJUNCTION, "or"),\n    (MSG_POS_INFO, "line %ld, col %ld, t t y %d"),\n    (MSG_CHAR_INFO, "hex %02x, decimal %d"),\n    (MSG_REPEAT_DESC, "times %d ."),\n    (MSG_REPEAT_DESC2, "repeated %d ."),\n    (MSG_WINDOW_LINE, "window is line %d"),\n    (MSG_WINDOW_BOUNDARY, "%s at line %d, column %d"),\n    (MSG_EDIT_PROMPT, "edit  %s, press space when done"),\n    (MSG_NO_COMMAND, "no commands for %c"),\n    (MSG_KEYDESC, "is %s"),\n    (MSG_CTL_SHIFT, "shift"),\n    (MSG_CTL_ALTGR, "altgr"),\n    (MSG_CTL_CONTROL, "control"),\n    (MSG_CTL_ALT, "alt"),\n    (MSG_CTL_LSHIFT, "l shift"),\n    (MSG_CTL_SPEAKUP, "speakup"),\n    (MSG_CTL_LCONTROL, "l control"),\n    (MSG_CTL_RCONTROL, "r control"),\n    (MSG_CTL_CAPSSHIFT, "caps shift"),\n    (MSG_COLOR_BLACK, "black"),\n    (MSG_COLOR_BLUE, "blue"),\n    (MSG_COLOR_GREEN, "green"),\n    (MSG_COLOR_CYAN, "cyan"),\n    (MSG_COLOR_RED, "red"),\n    (MSG_COLOR_MAGENTA, "magenta"),\n    (MSG_COLOR_YELLOW, "yellow"),\n    (MSG_COLOR_WHITE, "white"),\n    (MSG_COLOR_GREY, "grey"),\n    (MSG_COLOR_BRIGHTBLUE, "bright blue"),\n    (MSG_COLOR_BRIGHTGREEN, "bright green"),\n    (MSG_COLOR_BRIGHTCYAN, "bright cyan"),\n    (MSG_COLOR_BRIGHTRED, "bright red"),\n    (MSG_COLOR_BRIGHTMAGENTA, "bright magenta"),\n    (MSG_COLOR_BRIGHTYELLOW, "bright yellow"),\n    (MSG_COLOR_BRIGHTWHITE, "bright white"),\n    (MSG_STATE_DOUBLE, "double"),\n    (MSG_STATE_SPEAKUP, "speakup"),\n    (MSG_STATE_ALT, "alt"),\n    (MSG_STATE_CONTROL, "ctrl"),\n    (MSG_STATE_ALTGR, "altgr"),\n    (MSG_STATE_SHIFT, "shift"),\n    (MSG_KEYNAME_ESC, "escape"),\n    (MSG_KEYNAME_1, "1"),\n    (MSG_KEYNAME_2, "2"),\n    (MSG_KEYNAME_3, "3"),\n    (MSG_KEYNAME_4, "4"),\n    (MSG_KEYNAME_5, "5"),\n    (MSG_KEYNAME_6, "6"),\n    (MSG_KEYNAME_7, "7"),\n    (MSG_KEYNAME_8, "8"),\n    (MSG_KEYNAME_9, "9"),\n    (MSG_KEYNAME_0, "0"),\n    (MSG_KEYNAME_DASH, "minus"),\n    (MSG_KEYNAME_EQUAL, "equal"),\n    (MSG_KEYNAME_BS, "back space"),\n    (MSG_KEYNAME_TAB, "tab"),\n    (MSG_KEYNAME_Q, "q"),\n    (MSG_KEYNAME_W, "w"),\n    (MSG_KEYNAME_E, "e"),\n    (MSG_KEYNAME_R, "r"),\n    (MSG_KEYNAME_T, "t"),\n    (MSG_KEYNAME_Y, "y"),\n    (MSG_KEYNAME_U, "u"),\n    (MSG_KEYNAME_I, "i"),\n    (MSG_KEYNAME_O, "o"),\n    (MSG_KEYNAME_P, "p"),\n    (MSG_KEYNAME_LEFTBRACE, "left brace"),\n    (MSG_KEYNAME_RIGHTBRACE, "right brace"),\n    (MSG_KEYNAME_ENTER, "enter"),\n    (MSG_KEYNAME_LEFTCTRL, "left control"),\n    (MSG_KEYNAME_A, "a"),\n    (MSG_KEYNAME_S, "s"),\n    (MSG_KEYNAME_D, "d"),\n    (MSG_KEYNAME_F, "f"),\n    (MSG_KEYNAME_G, "g"),\n    (MSG_KEYNAME_H, "h"),\n    (MSG_KEYNAME_J, "j"),\n    (MSG_KEYNAME_K, "k"),\n    (MSG_KEYNAME_L, "l"),\n    (MSG_KEYNAME_SEMICOLON, "semicolon"),\n    (MSG_KEYNAME_SINGLEQUOTE, "apostrophe"),\n    (MSG_KEYNAME_GRAVE, "accent"),\n    (MSG_KEYNAME_LEFTSHFT, "left shift"),\n    (MSG_KEYNAME_BACKSLASH, "back slash"),\n    (MSG_KEYNAME_Z, "z"),\n    (MSG_KEYNAME_X, "x"),\n    (MSG_KEYNAME_C, "c"),\n    (MSG_KEYNAME_V, "v"),\n    (MSG_KEYNAME_B, "b"),\n    (MSG_KEYNAME_N, "n"),\n    (MSG_KEYNAME_M, "m"),\n    (MSG_KEYNAME_COMMA, "comma"),\n    (MSG_KEYNAME_DOT, "dot"),\n    (MSG_KEYNAME_SLASH, "slash"),\n    (MSG_KEYNAME_RIGHTSHFT, "right shift"),\n    (MSG_KEYNAME_KPSTAR, "keypad asterisk"),\n    (MSG_KEYNAME_LEFTALT, "left alt"),\n    (MSG_KEYNAME_SPACE, "space"),\n    (MSG_KEYNAME_CAPSLOCK, "caps lock"),\n    (MSG_KEYNAME_F1, "f1"),\n    (MSG_KEYNAME_F2, "f2"),\n    (MSG_KEYNAME_F3, "f3"),\n    (MSG_KEYNAME_F4, "f4"),\n    (MSG_KEYNAME_F5, "f5"),\n    (MSG_KEYNAME_F6, "f6"),\n    (MSG_KEYNAME_F7, "f7"),\n    (MSG_KEYNAME_F8, "f8"),\n    (MSG_KEYNAME_F9, "f9"),\n    (MSG_KEYNAME_F10, "f10"),\n    (MSG_KEYNAME_NUMLOCK, "num lock"),\n    (MSG_KEYNAME_SCROLLLOCK, "scroll lock"),\n    (MSG_KEYNAME_KP7, "keypad 7"),\n    (MSG_KEYNAME_KP8, "keypad 8"),\n    (MSG_KEYNAME_KP9, "keypad 9"),\n    (MSG_KEYNAME_KPMINUS, "keypad minus"),\n    (MSG_KEYNAME_KP4, "keypad 4"),\n    (MSG_KEYNAME_KP5, "keypad 5"),\n    (MSG_KEYNAME_KP6, "keypad 6"),\n    (MSG_KEYNAME_KPPLUS, "keypad plus"),\n    (MSG_KEYNAME_KP1, "keypad 1"),\n    (MSG_KEYNAME_KP2, "keypad 2"),\n    (MSG_KEYNAME_KP3, "keypad 3"),\n    (MSG_KEYNAME_KP0, "keypad 0"),\n    (MSG_KEYNAME_KPDOT, "keypad dot"),\n    (MSG_KEYNAME_103RD, "103rd"),\n    (MSG_KEYNAME_F13, "f13"),\n    (MSG_KEYNAME_102ND, "102nd"),\n    (MSG_KEYNAME_F11, "f11"),\n    (MSG_KEYNAME_F12, "f12"),\n    (MSG_KEYNAME_F14, "f14"),\n    (MSG_KEYNAME_F15, "f15"),\n    (MSG_KEYNAME_F16, "f16"),\n    (MSG_KEYNAME_F17, "f17"),\n    (MSG_KEYNAME_F18, "f18"),\n    (MSG_KEYNAME_F19, "f19"),\n    (MSG_KEYNAME_F20, "f20"),\n    (MSG_KEYNAME_KPENTER, "keypad enter"),\n    (MSG_KEYNAME_RIGHTCTRL, "right control"),\n    (MSG_KEYNAME_KPSLASH, "keypad slash"),\n    (MSG_KEYNAME_SYSRQ, "sysrq"),\n    (MSG_KEYNAME_RIGHTALT, "right alt"),\n    (MSG_KEYNAME_LF, "line feed"),\n    (MSG_KEYNAME_HOME, "home"),\n    (MSG_KEYNAME_UP, "up"),\n    (MSG_KEYNAME_PGUP, "page up"),\n    (MSG_KEYNAME_LEFT, "left"),\n    (MSG_KEYNAME_RIGHT, "right"),\n    (MSG_KEYNAME_END, "end"),\n    (MSG_KEYNAME_DOWN, "down"),\n    (MSG_KEYNAME_PGDN, "page down"),\n    (MSG_KEYNAME_INS, "insert"),\n    (MSG_KEYNAME_DEL, "delete"),\n    (MSG_KEYNAME_MACRO, "macro"),\n    (MSG_KEYNAME_MUTE, "mute"),\n    (MSG_KEYNAME_VOLDOWN, "volume down"),\n    (MSG_KEYNAME_VOLUP, "volume up"),\n    (MSG_KEYNAME_POWER, "power"),\n    (MSG_KEYNAME_KPEQUAL, "keypad equal"),\n    (MSG_KEYNAME_KPPLUSDASH, "keypad plusminus"),\n    (MSG_KEYNAME_PAUSE, "pause"),\n    (MSG_KEYNAME_F21, "f21"),\n    (MSG_KEYNAME_F22, "f22"),\n    (MSG_KEYNAME_F23, "f23"),\n    (MSG_KEYNAME_F24, "f24"),\n    (MSG_KEYNAME_KPCOMMA, "keypad comma"),\n    (MSG_KEYNAME_LEFTMETA, "left meta"),\n    (MSG_KEYNAME_RIGHTMETA, "right meta"),\n    (MSG_KEYNAME_COMPOSE, "compose"),\n    (MSG_KEYNAME_STOP, "stop"),\n    (MSG_KEYNAME_AGAIN, "again"),\n    (MSG_KEYNAME_PROPS, "props"),\n    (MSG_KEYNAME_UNDO, "undo"),\n    (MSG_KEYNAME_FRONT, "front"),\n    (MSG_KEYNAME_COPY, "copy"),\n    (MSG_KEYNAME_OPEN, "open"),\n    (MSG_KEYNAME_PASTE, "paste"),\n    (MSG_KEYNAME_FIND, "find"),\n    (MSG_KEYNAME_CUT, "cut"),\n    (MSG_KEYNAME_HELP, "help"),\n    (MSG_KEYNAME_MENU, "menu"),\n    (MSG_KEYNAME_CALC, "calc"),\n    (MSG_KEYNAME_SETUP, "setup"),\n    (MSG_KEYNAME_SLEEP, "sleep"),\n    (MSG_KEYNAME_WAKEUP, "wakeup"),\n    (MSG_KEYNAME_FILE, "file"),\n    (MSG_KEYNAME_SENDFILE, "send file"),\n    (MSG_KEYNAME_DELFILE, "delete file"),\n    (MSG_KEYNAME_XFER, "transfer"),\n    (MSG_KEYNAME_PROG1, "prog1"),\n    (MSG_KEYNAME_PROG2, "prog2"),\n    (MSG_KEYNAME_WWW, "www"),\n    (MSG_KEYNAME_MSDOS, "msdos"),\n    (MSG_KEYNAME_COFFEE, "coffee"),\n    (MSG_KEYNAME_DIRECTION, "direction"),\n    (MSG_KEYNAME_CYCLEWINDOWS, "cycle windows"),\n    (MSG_KEYNAME_MAIL, "mail"),\n    (MSG_KEYNAME_BOOKMARKS, "bookmarks"),\n    (MSG_KEYNAME_COMPUTER, "computer"),\n    (MSG_KEYNAME_BACK, "back"),\n    (MSG_KEYNAME_FORWARD, "forward"),\n    (MSG_KEYNAME_CLOSECD, "close cd"),\n    (MSG_KEYNAME_EJECTCD, "eject cd"),\n    (MSG_KEYNAME_EJECTCLOSE, "eject close cd"),\n    (MSG_KEYNAME_NEXTSONG, "next song"),\n    (MSG_KEYNAME_PLAYPAUSE, "play pause"),\n    (MSG_KEYNAME_PREVSONG, "previous song"),\n    (MSG_KEYNAME_STOPCD, "stop cd"),\n    (MSG_KEYNAME_RECORD, "record"),\n    (MSG_KEYNAME_REWIND, "rewind"),\n    (MSG_KEYNAME_PHONE, "phone"),\n    (MSG_KEYNAME_ISO, "iso"),\n    (MSG_KEYNAME_CONFIG, "config"),\n    (MSG_KEYNAME_HOMEPG, "home page"),\n    (MSG_KEYNAME_REFRESH, "refresh"),\n    (MSG_KEYNAME_EXIT, "exit"),\n    (MSG_KEYNAME_MOVE, "move"),\n    (MSG_KEYNAME_EDIT, "edit"),\n    (MSG_KEYNAME_SCROLLUP, "scroll up"),\n    (MSG_KEYNAME_SCROLLDN, "scroll down"),\n    (MSG_KEYNAME_KPLEFTPAR, "keypad left paren"),\n    (MSG_KEYNAME_KPRIGHTPAR, "keypad right paren"),\n    (MSG_FUNCNAME_ATTRIB_BLEEP_DEC, "attribute bleep decrement"),\n    (MSG_FUNCNAME_ATTRIB_BLEEP_INC, "attribute bleep increment"),\n    (MSG_FUNCNAME_BLEEPS_DEC, "bleeps decrement"),\n    (MSG_FUNCNAME_BLEEPS_INC, "bleeps increment"),\n    (MSG_FUNCNAME_CHAR_FIRST, "character, first"),\n    (MSG_FUNCNAME_CHAR_LAST, "character, last"),\n    (MSG_FUNCNAME_CHAR_CURRENT, "character, say current"),\n    (MSG_FUNCNAME_CHAR_HEX_AND_DEC, "character, say hex and decimal"),\n    (MSG_FUNCNAME_CHAR_NEXT, "character, say next"),\n    (MSG_FUNCNAME_CHAR_PHONETIC, "character, say phonetic"),\n    (MSG_FUNCNAME_CHAR_PREVIOUS, "character, say previous"),\n    (MSG_FUNCNAME_CURSOR_PARK, "cursor park"),\n    (MSG_FUNCNAME_CUT, "cut"),\n    (MSG_FUNCNAME_EDIT_DELIM, "edit delimiters"),\n    (MSG_FUNCNAME_EDIT_EXNUM, "edit exnum"),\n    (MSG_FUNCNAME_EDIT_MOST, "edit most"),\n    (MSG_FUNCNAME_EDIT_REPEATS, "edit repeats"),\n    (MSG_FUNCNAME_EDIT_SOME, "edit some"),\n    (MSG_FUNCNAME_GOTO, "go to"),\n    (MSG_FUNCNAME_GOTO_BOTTOM, "go to bottom edge"),\n    (MSG_FUNCNAME_GOTO_LEFT, "go to left edge"),\n    (MSG_FUNCNAME_GOTO_RIGHT, "go to right edge"),\n    (MSG_FUNCNAME_GOTO_TOP, "go to top edge"),\n    (MSG_FUNCNAME_HELP, "help"),\n    (MSG_FUNCNAME_LINE_SAY_CURRENT, "line, say current"),\n    (MSG_FUNCNAME_LINE_SAY_NEXT, "line, say next"),\n    (MSG_FUNCNAME_LINE_SAY_PREVIOUS, "line, say previous"),\n    (MSG_FUNCNAME_LINE_SAY_WITH_INDENT, "line, say with indent"),\n    (MSG_FUNCNAME_PASTE, "paste"),\n    (MSG_FUNCNAME_PITCH_DEC, "pitch decrement"),\n    (MSG_FUNCNAME_PITCH_INC, "pitch increment"),\n    (MSG_FUNCNAME_PUNC_DEC, "punctuation decrement"),\n    (MSG_FUNCNAME_PUNC_INC, "punctuation increment"),\n    (MSG_FUNCNAME_PUNC_LEVEL_DEC, "punc level decrement"),\n    (MSG_FUNCNAME_PUNC_LEVEL_INC, "punc level increment"),\n    (MSG_FUNCNAME_QUIET, "quiet"),\n    (MSG_FUNCNAME_RATE_DEC, "rate decrement"),\n    (MSG_FUNCNAME_RATE_INC, "rate increment"),\n    (MSG_FUNCNAME_READING_PUNC_DEC, "reading punctuation decrement"),\n    (MSG_FUNCNAME_READING_PUNC_INC, "reading punctuation increment"),\n    (MSG_FUNCNAME_SAY_ATTRIBUTES, "say attributes"),\n    (MSG_FUNCNAME_SAY_FROM_LEFT, "say from left"),\n    (MSG_FUNCNAME_SAY_FROM_TOP, "say from top"),\n    (MSG_FUNCNAME_SAY_POSITION, "say position"),\n    (MSG_FUNCNAME_SAY_SCREEN, "say screen"),\n    (MSG_FUNCNAME_SAY_TO_BOTTOM, "say to bottom"),\n    (MSG_FUNCNAME_SAY_TO_RIGHT, "say to right"),\n    (MSG_FUNCNAME_SPEAKUP, "speakup"),\n    (MSG_FUNCNAME_SPEAKUP_LOCK, "speakup lock"),\n    (MSG_FUNCNAME_SPEAKUP_OFF, "speakup off"),\n    (MSG_FUNCNAME_SPEECH_KILL, "speech kill"),\n    (MSG_FUNCNAME_SPELL_DELAY_DEC, "spell delay decrement"),\n    (MSG_FUNCNAME_SPELL_DELAY_INC, "spell delay increment"),\n    (MSG_FUNCNAME_SPELL_WORD, "spell word"),\n    (MSG_FUNCNAME_SPELL_WORD_PHONETICALLY, "spell word phonetically"),\n    (MSG_FUNCNAME_TONE_DEC, "tone decrement"),\n    (MSG_FUNCNAME_TONE_INC, "tone increment"),\n    (MSG_FUNCNAME_VOICE_DEC, "voice decrement"),\n    (MSG_FUNCNAME_VOICE_INC, "voice increment"),\n    (MSG_FUNCNAME_VOLUME_DEC, "volume decrement"),\n    (MSG_FUNCNAME_VOLUME_INC, "volume increment"),\n    (MSG_FUNCNAME_WINDOW_CLEAR, "window, clear"),\n    (MSG_FUNCNAME_WINDOW_SAY, "window, say"),\n    (MSG_FUNCNAME_WINDOW_SET, "window, set"),\n    (MSG_FUNCNAME_WINDOW_SILENCE, "window, silence"),\n    (MSG_FUNCNAME_WORD_SAY_CURRENT, "word, say current"),\n    (MSG_FUNCNAME_WORD_SAY_NEXT, "word, say next"),\n    (MSG_FUNCNAME_WORD_SAY_PREVIOUS, "word, say previous"),
];

static mut ALL_GROUPS: [MsgGroup; 7] = [
    MsgGroup { name: b"ctl_keys\\0".as_ptr() as *const c_char, start: MSG_CTL_START, end: MSG_CTL_END },
    MsgGroup { name: b"colors\\0".as_ptr() as *const c_char, start: MSG_COLORS_START, end: MSG_COLORS_END },
    MsgGroup { name: b"formatted\\0".as_ptr() as *const c_char, start: MSG_FORMATTED_START, end: MSG_FORMATTED_END },
    MsgGroup { name: b"function_names\\0".as_ptr() as *const c_char, start: MSG_FUNCNAMES_START, end: MSG_FUNCNAMES_END },
    MsgGroup { name: b"key_names\\0".as_ptr() as *const c_char, start: MSG_KEYNAMES_START, end: MSG_KEYNAMES_END },
    MsgGroup { name: b"announcements\\0".as_ptr() as *const c_char, start: MSG_ANNOUNCEMENTS_START, end: MSG_ANNOUNCEMENTS_END },
    MsgGroup { name: b"states\\0".as_ptr() as *const c_char, start: MSG_STATES_START, end: MSG_STATES_END },
];
static NUM_GROUPS: usize = 7;

pub unsafe fn spk_msg_get(index: MsgIndex) -> *mut c_char { SPEAKUP_MSGS[index] }

unsafe fn next_specifier(mut input: *mut c_char) -> *mut c_char {
    let mut found = false;
    let mut next_percent = input;
    while !next_percent.is_null() && !found {
        while !next_percent.is_null() && *next_percent != 0 && *next_percent != b'%' as c_char { next_percent = next_percent.add(1); }
        if !next_percent.is_null() && *next_percent != 0 {
            while *next_percent == b'%' as c_char && *next_percent.add(1) == b'%' as c_char { next_percent = next_percent.add(2); }
            if *next_percent == b'%' as c_char { found = true; }
            else if *next_percent == 0 { next_percent = ptr::null_mut(); }
        } else { next_percent = ptr::null_mut(); }
    }
    next_percent
}

unsafe fn skip_flags(mut input: *mut c_char) -> *mut c_char {
    while *input != 0 && b" 0+-#".contains(&(*input as u8)) { input = input.add(1); } input
}
unsafe fn skip_width(mut input: *mut c_char) -> *mut c_char {
    while (*input as u8).is_ascii_digit() { input = input.add(1); }
    if *input == b'.' as c_char { input = input.add(1); while (*input as u8).is_ascii_digit() { input = input.add(1); } } input
}
unsafe fn skip_conversion(mut input: *mut c_char) -> *mut c_char {
    if *input == b'l' as c_char && *input.add(1) == b'd' as c_char { input = input.add(2); }
    else if *input != 0 && b"cdsx".contains(&(*input as u8)) { input = input.add(1); } input
}
unsafe fn find_specifier_end(input: *mut c_char) -> *mut c_char {
    let mut p = input.add(1); p = skip_flags(p); p = skip_width(p); skip_conversion(p)
}
unsafe fn compare_specifiers(input1: &mut *mut c_char, input2: &mut *mut c_char) -> bool {
    let end1 = find_specifier_end(*input1); let end2 = find_specifier_end(*input2);
    let len1 = end1.offset_from(*input1) as usize; let len2 = end2.offset_from(*input2) as usize;
    let same = len1 == len2 && core::slice::from_raw_parts(*input1 as *const u8, len1) == core::slice::from_raw_parts(*input2 as *const u8, len2);
    *input1 = end1; *input2 = end2; same
}
unsafe fn fmt_validate(template: *mut c_char, user: *mut c_char) -> bool {
    let mut t = template; let mut u = user; let mut valid = true; let mut comparing = true;
    while comparing && valid {
        t = next_specifier(t); u = next_specifier(u);
        if !t.is_null() && !u.is_null() { valid = compare_specifiers(&mut t, &mut u); }
        else { comparing = false; if !t.is_null() || !u.is_null() { valid = false; } }
    } valid
}

pub unsafe fn spk_msg_set(index: MsgIndex, text: *mut c_char, length: usize) -> isize {
    if index < MSG_FIRST_INDEX || index >= MSG_LAST_INDEX { return -22; }
    let newstr = kmemdup_nul(text as *const c_void, length, 0);
    if newstr.is_null() { return -12; }
    // Formatted-message validation is retained by fmt_validate; header constants identify its range.
    if index >= MSG_FORMATTED_START && index <= MSG_FORMATTED_END && !fmt_validate(SPEAKUP_DEFAULT_MSGS[index], newstr) { kfree(newstr as *mut c_void); return -22; }
    let mut flags = 0usize; spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    if SPEAKUP_MSGS[index] != SPEAKUP_DEFAULT_MSGS[index] { kfree(SPEAKUP_MSGS[index] as *mut c_void); }
    SPEAKUP_MSGS[index] = newstr; spin_unlock_irqrestore(&mut speakup_info.spinlock, flags); 0
}

pub unsafe fn spk_find_msg_group(group_name: *const c_char) -> *mut MsgGroup {
    for i in 0..NUM_GROUPS {
        let mut a = ALL_GROUPS[i].name; let mut b = group_name;
        while *a != 0 && *a == *b { a = a.add(1); b = b.add(1); }
        if *a == 0 && *b == 0 { return &mut ALL_GROUPS[i]; }
    } ptr::null_mut()
}
pub unsafe fn spk_reset_msg_group(group: *mut MsgGroup) {
    let mut flags = 0usize; spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    let mut i = (*group).start; while i <= (*group).end {
        if SPEAKUP_MSGS[i] != SPEAKUP_DEFAULT_MSGS[i] { kfree(SPEAKUP_MSGS[i] as *mut c_void); }
        SPEAKUP_MSGS[i] = SPEAKUP_DEFAULT_MSGS[i]; i += 1;
    } spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
}
pub unsafe fn spk_initialize_msgs() { for i in 0..1024 { SPEAKUP_MSGS[i] = SPEAKUP_DEFAULT_MSGS[i]; } }
pub unsafe fn spk_free_user_msgs() {
    let mut flags = 0usize; spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    let mut i = MSG_FIRST_INDEX; while i < MSG_LAST_INDEX {
        if SPEAKUP_MSGS[i] != SPEAKUP_DEFAULT_MSGS[i] { kfree(SPEAKUP_MSGS[i] as *mut c_void); SPEAKUP_MSGS[i] = SPEAKUP_DEFAULT_MSGS[i]; } i += 1;
    } spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
