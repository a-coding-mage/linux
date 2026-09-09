/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (C) 2017-2018 ARM Limited */

/*
 * For use by other UAPI headers only.
 * Do not make direct use of header or its definitions.
 */

// The C header includes <linux/types.h>; the __u32 type is represented by u32
// in this translation.

pub const __SVE_VQ_BYTES: u32 = 16; /* number of bytes per quadword */

/*
 * Yes, __SVE_VQ_MAX is 512 QUADWORDS.
 *
 * To help ensure forward portability, this is much larger than the
 * current maximum value defined by the SVE architecture.  While arrays
 * or static allocations can be sized based on this value, watch out!
 * It will waste a surprisingly large amount of memory.
 *
 * Dynamic sizing based on the actual runtime vector length is likely to
 * be preferable for most purposes.
 */
pub const __SVE_VQ_MIN: u32 = 1;
pub const __SVE_VQ_MAX: u32 = 512;

pub const __SVE_VL_MIN: u32 = __SVE_VQ_MIN * __SVE_VQ_BYTES;
pub const __SVE_VL_MAX: u32 = __SVE_VQ_MAX * __SVE_VQ_BYTES;

pub const __SVE_NUM_ZREGS: u32 = 32;
pub const __SVE_NUM_PREGS: u32 = 16;

macro_rules! __sve_vl_valid {
    ($vl:expr) => {
        (($vl) % __SVE_VQ_BYTES == 0 &&
            ($vl) >= __SVE_VL_MIN &&
            ($vl) <= __SVE_VL_MAX)
    };
}

macro_rules! __sve_vq_from_vl {
    ($vl:expr) => {
        ($vl) / __SVE_VQ_BYTES
    };
}

macro_rules! __sve_vl_from_vq {
    ($vq:expr) => {
        ($vq) * __SVE_VQ_BYTES
    };
}

macro_rules! __SVE_ZREG_SIZE {
    ($vq:expr) => {
        (($vq) as u32) * __SVE_VQ_BYTES
    };
}

macro_rules! __SVE_PREG_SIZE {
    ($vq:expr) => {
        (($vq) as u32) * (__SVE_VQ_BYTES / 8)
    };
}

macro_rules! __SVE_FFR_SIZE {
    ($vq:expr) => {
        __SVE_PREG_SIZE!($vq)
    };
}

pub const __SVE_ZREGS_OFFSET: u32 = 0;

macro_rules! __SVE_ZREG_OFFSET {
    ($vq:expr, $n:expr) => {
        __SVE_ZREGS_OFFSET + __SVE_ZREG_SIZE!($vq) * ($n)
    };
}

macro_rules! __SVE_ZREGS_SIZE {
    ($vq:expr) => {
        __SVE_ZREG_OFFSET!($vq, __SVE_NUM_ZREGS) - __SVE_ZREGS_OFFSET
    };
}

macro_rules! __SVE_PREGS_OFFSET {
    ($vq:expr) => {
        __SVE_ZREGS_OFFSET + __SVE_ZREGS_SIZE!($vq)
    };
}

macro_rules! __SVE_PREG_OFFSET {
    ($vq:expr, $n:expr) => {
        __SVE_PREGS_OFFSET!($vq) + __SVE_PREG_SIZE!($vq) * ($n)
    };
}

macro_rules! __SVE_PREGS_SIZE {
    ($vq:expr) => {
        __SVE_PREG_OFFSET!($vq, __SVE_NUM_PREGS) - __SVE_PREGS_OFFSET!($vq)
    };
}

macro_rules! __SVE_FFR_OFFSET {
    ($vq:expr) => {
        __SVE_PREGS_OFFSET!($vq) + __SVE_PREGS_SIZE!($vq)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
