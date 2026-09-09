/* SPDX-License-Identifier: GPL-2.0 */
/*
 * See Documentation/core-api/errseq.rst and lib/errseq.c
 */

// Translated from the C header; the original header guard is omitted.

pub type errseq_t = u32;

extern "C" {
    pub fn errseq_set(eseq: *mut errseq_t, err: i32) -> errseq_t;
    pub fn errseq_sample(eseq: *mut errseq_t) -> errseq_t;
    pub fn errseq_check(eseq: *mut errseq_t, since: errseq_t) -> i32;
    pub fn errseq_check_and_advance(eseq: *mut errseq_t, since: *mut errseq_t) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
