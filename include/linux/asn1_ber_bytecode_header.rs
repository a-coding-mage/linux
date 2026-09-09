/* SPDX-License-Identifier: GPL-2.0-or-later */
/* ASN.1 BER/DER/CER parsing state machine internal definitions
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* The Linux kernel includes and header guard have no direct Rust equivalent. */

use core::ffi::c_void;

pub type Asn1ActionT = unsafe extern "C" fn(
    context: *mut c_void,
    hdrlen: usize, /* In case of ANY type */
    tag: u8,       /* In case of ANY type */
    value: *const c_void,
    vlen: usize,
) -> i32;

#[repr(C)]
pub struct Asn1Decoder {
    pub machine: *const u8,
    pub machlen: usize,
    pub actions: *const Asn1ActionT,
}

#[repr(i32)]
pub enum Asn1Opcode {
    /* The tag-matching ops come first and the odd-numbered slots
     * are for OR_SKIP ops.
     */
    Asn1OpMatch = 0x00,
    Asn1OpMatchOrSkip = 0x01,
    Asn1OpMatchAct = 0x02,
    Asn1OpMatchActOrSkip = 0x03,
    Asn1OpMatchJump = 0x04,
    Asn1OpMatchJumpOrSkip = 0x05,
    Asn1OpMatchAny = 0x08,
    Asn1OpMatchAnyOrSkip = 0x09,
    Asn1OpMatchAnyAct = 0x0a,
    Asn1OpMatchAnyActOrSkip = 0x0b,
    /* Everything before here matches unconditionally */

    Asn1OpCondMatchOrSkip = 0x11,
    Asn1OpCondMatchActOrSkip = 0x13,
    Asn1OpCondMatchJumpOrSkip = 0x15,
    Asn1OpCondMatchAny = 0x18,
    Asn1OpCondMatchAnyOrSkip = 0x19,
    Asn1OpCondMatchAnyAct = 0x1a,
    Asn1OpCondMatchAnyActOrSkip = 0x1b,

    /* Everything before here will want a tag from the data */

    /* These are here to help fill up space */
    Asn1OpCondFail = 0x1c,
    Asn1OpComplete = 0x1d,
    Asn1OpAct = 0x1e,
    Asn1OpMaybeAct = 0x1f,

    /* The following eight have bit 0 -> SET, 1 -> OF, 2 -> ACT */
    Asn1OpEndSeq = 0x20,
    Asn1OpEndSet = 0x21,
    Asn1OpEndSeqOf = 0x22,
    Asn1OpEndSetOf = 0x23,
    Asn1OpEndSeqAct = 0x24,
    Asn1OpEndSetAct = 0x25,
    Asn1OpEndSeqOfAct = 0x26,
    Asn1OpEndSetOfAct = 0x27,

    Asn1OpReturn = 0x28,

    Asn1OpNr,
}

pub const ASN1_OP_MATCH__SKIP: i32 = 0x01;
pub const ASN1_OP_MATCH__ACT: i32 = 0x02;
pub const ASN1_OP_MATCH__JUMP: i32 = 0x04;
pub const ASN1_OP_MATCH__ANY: i32 = 0x08;
pub const ASN1_OP_MATCH__COND: i32 = 0x10;

pub const ASN1_OP__MATCHES_TAG: i32 = 0x1b;

pub const ASN1_OP_END__SET: i32 = 0x01;
pub const ASN1_OP_END__OF: i32 = 0x02;
pub const ASN1_OP_END__ACT: i32 = 0x04;

/* Callers provide the corresponding ASN1_* class, constructed, and tag values. */
#[macro_export]
macro_rules! _tag {
    ($class:expr, $cp:expr, $tag:expr) => {
        (($class << 6) | ($cp << 5) | $tag)
    };
}

#[macro_export]
macro_rules! _tagn {
    ($class:expr, $cp:expr, $tag:expr) => {
        (($class << 6) | ($cp << 5) | $tag)
    };
}

#[macro_export]
macro_rules! _jump_target {
    ($n:expr) => {
        $n
    };
}

#[macro_export]
macro_rules! _action {
    ($n:expr) => {
        $n
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
