/* SPDX-License-Identifier: GPL-2.0 */

// Original dependency: <linux/types.h>

pub const TS_FSM_SPECIFIC: i32 = 0; // specific character
pub const TS_FSM_WILDCARD: i32 = 1; // any character
pub const TS_FSM_DIGIT: i32 = 2; // isdigit()
pub const TS_FSM_XDIGIT: i32 = 3; // isxdigit()
pub const TS_FSM_PRINT: i32 = 4; // isprint()
pub const TS_FSM_ALPHA: i32 = 5; // isalpha()
pub const TS_FSM_ALNUM: i32 = 6; // isalnum()
pub const TS_FSM_ASCII: i32 = 7; // isascii()
pub const TS_FSM_CNTRL: i32 = 8; // iscntrl()
pub const TS_FSM_GRAPH: i32 = 9; // isgraph()
pub const TS_FSM_LOWER: i32 = 10; // islower()
pub const TS_FSM_UPPER: i32 = 11; // isupper()
pub const TS_FSM_PUNCT: i32 = 12; // ispunct()
pub const TS_FSM_SPACE: i32 = 13; // isspace()
pub const __TS_FSM_TYPE_MAX: i32 = 14;
pub const TS_FSM_TYPE_MAX: i32 = __TS_FSM_TYPE_MAX - 1;

pub const TS_FSM_SINGLE: i32 = 0; // 1 occurrence
pub const TS_FSM_PERHAPS: i32 = 1; // 1 or 0 occurrence
pub const TS_FSM_ANY: i32 = 2; // 0..n occurrences
pub const TS_FSM_MULTI: i32 = 3; // 1..n occurrences
pub const TS_FSM_HEAD_IGNORE: i32 = 4; // 0..n ignored occurrences at head
pub const __TS_FSM_RECUR_MAX: i32 = 5;
pub const TS_FSM_RECUR_MAX: i32 = __TS_FSM_RECUR_MAX - 1;

/**
 * struct ts_fsm_token - state machine token (state)
 * @type: type of token
 * @recur: number of recurrences
 * @value: character value for TS_FSM_SPECIFIC
 */
#[repr(C)]
pub struct ts_fsm_token {
    pub r#type: u16,
    pub recur: u8,
    pub value: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
