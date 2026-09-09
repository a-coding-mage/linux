/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2004 Olof Johansson <olof@lixom.net>, IBM Corporation
 */

/* Offset from base to control register */
pub const DART_CNTL: u32 = 0;

/* Offset from base to exception register */
pub const DART_EXCP_U3: u32 = 0x10;
/* Offset from base to TLB tag registers */
pub const DART_TAGS_U3: u32 = 0x1000;

/* U4 registers */
pub const DART_BASE_U4: u32 = 0x10;
pub const DART_SIZE_U4: u32 = 0x20;
pub const DART_EXCP_U4: u32 = 0x30;
pub const DART_TAGS_U4: u32 = 0x1000;

/* Control Register fields */

/* U3 registers */
pub const DART_CNTL_U3_BASE_MASK: u32 = 0xfffff;
pub const DART_CNTL_U3_BASE_SHIFT: u32 = 12;
pub const DART_CNTL_U3_FLUSHTLB: u32 = 0x400;
pub const DART_CNTL_U3_ENABLE: u32 = 0x200;
pub const DART_CNTL_U3_SIZE_MASK: u32 = 0x1ff;
pub const DART_CNTL_U3_SIZE_SHIFT: u32 = 0;

/* U4 registers */
pub const DART_BASE_U4_BASE_MASK: u32 = 0xffffff;
pub const DART_BASE_U4_BASE_SHIFT: u32 = 0;
pub const DART_CNTL_U4_ENABLE: u32 = 0x80000000;
pub const DART_CNTL_U4_IONE: u32 = 0x40000000;
pub const DART_CNTL_U4_FLUSHTLB: u32 = 0x20000000;
pub const DART_CNTL_U4_IDLE: u32 = 0x10000000;
pub const DART_CNTL_U4_PAR_EN: u32 = 0x08000000;
pub const DART_CNTL_U4_IONE_MASK: u32 = 0x07ffffff;
pub const DART_SIZE_U4_SIZE_MASK: u32 = 0x1fff;
pub const DART_SIZE_U4_SIZE_SHIFT: u32 = 0;

#[macro_export]
macro_rules! DART_REG {
    ($dart:expr, $r:expr) => {
        unsafe { $dart.add(($r as usize) >> 2) }
    };
}

#[macro_export]
macro_rules! DART_IN {
    ($dart:expr, $r:expr) => {
        in_be32(DART_REG!($dart, $r))
    };
}

#[macro_export]
macro_rules! DART_OUT {
    ($dart:expr, $r:expr, $v:expr) => {
        out_be32(DART_REG!($dart, $r), $v)
    };
}

/* size of table in pages */

/* DART table fields */

pub const DARTMAP_VALID: u32 = 0x80000000;
pub const DARTMAP_RPNMASK: u32 = 0x00ffffff;

pub const DART_PAGE_SHIFT: u32 = 12;
pub const DART_PAGE_SIZE: u32 = 1 << DART_PAGE_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
