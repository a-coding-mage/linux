/* SPDX-License-Identifier: 0BSD */

/*
 * LZMA2 definitions
 *
 * Authors: Lasse Collin <lasse.collin@tukaani.org>
 *          Igor Pavlov <https://7-zip.org/>
 */

/* Range coder constants */
pub const RC_SHIFT_BITS: i32 = 8;
pub const RC_TOP_BITS: i32 = 24;
pub const RC_TOP_VALUE: i32 = 1 << RC_TOP_BITS;
pub const RC_BIT_MODEL_TOTAL_BITS: i32 = 11;
pub const RC_BIT_MODEL_TOTAL: i32 = 1 << RC_BIT_MODEL_TOTAL_BITS;
pub const RC_MOVE_BITS: i32 = 5;

/*
 * Maximum number of position states. A position state is the lowest pb
 * number of bits of the current uncompressed offset. In some places there
 * are different sets of probabilities for different position states.
 */
pub const POS_STATES_MAX: i32 = 1 << 4;

/*
 * This enum is used to track which LZMA symbols have occurred most recently
 * and in which order. This information is used to predict the next symbol.
 *
 * Symbols:
 *  - Literal: One 8-bit byte
 *  - Match: Repeat a chunk of data at some distance
 *  - Long repeat: Multi-byte match at a recently seen distance
 *  - Short repeat: One-byte repeat at a recently seen distance
 *
 * The symbol names are in from STATE_oldest_older_previous. REP means
 * either short or long repeated match, and NONLIT means any non-literal.
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lzma_state {
    STATE_LIT_LIT,
    STATE_MATCH_LIT_LIT,
    STATE_REP_LIT_LIT,
    STATE_SHORTREP_LIT_LIT,
    STATE_MATCH_LIT,
    STATE_REP_LIT,
    STATE_SHORTREP_LIT,
    STATE_LIT_MATCH,
    STATE_LIT_LONGREP,
    STATE_LIT_SHORTREP,
    STATE_NONLIT_MATCH,
    STATE_NONLIT_REP,
}

/* Total number of states */
pub const STATES: i32 = 12;

/* The lowest 7 states indicate that the previous state was a literal. */
pub const LIT_STATES: i32 = 7;

/* Indicate that the latest symbol was a literal. */
#[inline]
pub unsafe fn lzma_state_literal(state: *mut lzma_state) {
    if (*state as i32) <= lzma_state::STATE_SHORTREP_LIT_LIT as i32 {
        *state = lzma_state::STATE_LIT_LIT;
    } else if (*state as i32) <= lzma_state::STATE_LIT_SHORTREP as i32 {
        *state = core::mem::transmute::<i32, lzma_state>((*state as i32) - 3);
    } else {
        *state = core::mem::transmute::<i32, lzma_state>((*state as i32) - 6);
    }
}

/* Indicate that the latest symbol was a match. */
#[inline]
pub unsafe fn lzma_state_match(state: *mut lzma_state) {
    *state = if (*state as i32) < LIT_STATES {
        lzma_state::STATE_LIT_MATCH
    } else {
        lzma_state::STATE_NONLIT_MATCH
    };
}

/* Indicate that the latest state was a long repeated match. */
#[inline]
pub unsafe fn lzma_state_long_rep(state: *mut lzma_state) {
    *state = if (*state as i32) < LIT_STATES {
        lzma_state::STATE_LIT_LONGREP
    } else {
        lzma_state::STATE_NONLIT_REP
    };
}

/* Indicate that the latest symbol was a short match. */
#[inline]
pub unsafe fn lzma_state_short_rep(state: *mut lzma_state) {
    *state = if (*state as i32) < LIT_STATES {
        lzma_state::STATE_LIT_SHORTREP
    } else {
        lzma_state::STATE_NONLIT_REP
    };
}

/* Test if the previous symbol was a literal. */
#[inline]
pub fn lzma_state_is_literal(state: lzma_state) -> bool {
    (state as i32) < LIT_STATES
}

/* Each literal coder is divided in three sections. */
pub const LITERAL_CODER_SIZE: i32 = 0x300;

/* Maximum number of literal coders */
pub const LITERAL_CODERS_MAX: i32 = 1 << 4;

/* Minimum length of a match is two bytes. */
pub const MATCH_LEN_MIN: i32 = 2;

pub const LEN_LOW_BITS: i32 = 3;
pub const LEN_LOW_SYMBOLS: i32 = 1 << LEN_LOW_BITS;
pub const LEN_MID_BITS: i32 = 3;
pub const LEN_MID_SYMBOLS: i32 = 1 << LEN_MID_BITS;
pub const LEN_HIGH_BITS: i32 = 8;
pub const LEN_HIGH_SYMBOLS: i32 = 1 << LEN_HIGH_BITS;
pub const LEN_SYMBOLS: i32 = LEN_LOW_SYMBOLS + LEN_MID_SYMBOLS + LEN_HIGH_SYMBOLS;

pub const MATCH_LEN_MAX: i32 = MATCH_LEN_MIN + LEN_SYMBOLS - 1;
pub const DIST_STATES: i32 = 4;

#[inline]
pub fn lzma_get_dist_state(len: usize) -> usize {
    if len < (DIST_STATES + MATCH_LEN_MIN) as usize {
        len - MATCH_LEN_MIN as usize
    } else {
        (DIST_STATES - 1) as usize
    }
}

pub const DIST_SLOT_BITS: i32 = 6;
pub const DIST_SLOTS: i32 = 1 << DIST_SLOT_BITS;
pub const DIST_MODEL_START: i32 = 4;
pub const DIST_MODEL_END: i32 = 14;
pub const FULL_DISTANCES_BITS: i32 = DIST_MODEL_END / 2;
pub const FULL_DISTANCES: i32 = 1 << FULL_DISTANCES_BITS;
pub const ALIGN_BITS: i32 = 4;
pub const ALIGN_SIZE: i32 = 1 << ALIGN_BITS;
pub const ALIGN_MASK: i32 = ALIGN_SIZE - 1;
pub const PROBS_TOTAL: i32 = 1846 + LITERAL_CODERS_MAX * LITERAL_CODER_SIZE;
pub const REPS: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
