/*
 * tie.h -- compile-time HAL definitions dependent on CORE & TIE configuration
 *
 * NOTE: This header file is not meant to be included directly.
 *
 * This header file describes this specific Xtensa processor's TIE extensions
 * that extend basic Xtensa core functionality. It is customized to this
 * Xtensa processor configuration.
 *
 * Copyright (c) 1999-2015 Cadence Design Systems Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to permit
 * persons to whom the Software is furnished to do so, subject to the
 * following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

pub const XCHAL_CP_NUM: u32 = 0;
pub const XCHAL_CP_MAX: u32 = 0;
pub const XCHAL_CP_MASK: u32 = 0x00;
pub const XCHAL_CP_PORT_MASK: u32 = 0x00;

pub const XCHAL_NCP_SA_SIZE: u32 = 28;
pub const XCHAL_NCP_SA_ALIGN: u32 = 4;
pub const XCHAL_TOTAL_SA_SIZE: u32 = 32;
pub const XCHAL_TOTAL_SA_ALIGN: u32 = 4;

/*
 * Detailed contents of save areas. The caller must define the XCHAL_SA_REG
 * macro before expanding the XCHAL_xxx_SA_LIST() macros. This conditional
 * macro-expansion interface is preserved below as a Rust macro.
 */

pub const XCHAL_NCP_SA_NUM: u32 = 7;

#[macro_export]
macro_rules! XCHAL_NCP_SA_LIST {
    ($s:expr, $XCHAL_SA_REG:ident) => {
        $XCHAL_SA_REG!($s, 1, 0, 0, 1, acclo, 4, 4, 4, 0x0210, sr, 16, 32, 0, 0, 0);
        $XCHAL_SA_REG!($s, 1, 0, 0, 1, acchi, 4, 4, 4, 0x0211, sr, 17, 8, 0, 0, 0);
        $XCHAL_SA_REG!($s, 0, 0, 0, 1, scompare1, 4, 4, 4, 0x020C, sr, 12, 32, 0, 0, 0);
        $XCHAL_SA_REG!($s, 0, 0, 0, 1, m0, 4, 4, 4, 0x0220, sr, 32, 32, 0, 0, 0);
        $XCHAL_SA_REG!($s, 0, 0, 0, 1, m1, 4, 4, 4, 0x0221, sr, 33, 32, 0, 0, 0);
        $XCHAL_SA_REG!($s, 0, 0, 0, 1, m2, 4, 4, 4, 0x0222, sr, 34, 32, 0, 0, 0);
        $XCHAL_SA_REG!($s, 0, 0, 0, 1, m3, 4, 4, 4, 0x0223, sr, 35, 32, 0, 0, 0);
    };
}

macro_rules! define_empty_sa_list {
    ($name:ident) => {
        #[macro_export]
        macro_rules! $name { ($s:expr) => {}; }
    };
}

pub const XCHAL_CP0_SA_NUM: u32 = 0;
pub const XCHAL_CP1_SA_NUM: u32 = 0;
pub const XCHAL_CP2_SA_NUM: u32 = 0;
pub const XCHAL_CP3_SA_NUM: u32 = 0;
pub const XCHAL_CP4_SA_NUM: u32 = 0;
pub const XCHAL_CP5_SA_NUM: u32 = 0;
pub const XCHAL_CP6_SA_NUM: u32 = 0;
pub const XCHAL_CP7_SA_NUM: u32 = 0;

/* XCHAL_CP0_SA_LIST through XCHAL_CP7_SA_LIST are empty in the source. */
define_empty_sa_list!(XCHAL_CP0_SA_LIST);
define_empty_sa_list!(XCHAL_CP1_SA_LIST);
define_empty_sa_list!(XCHAL_CP2_SA_LIST);
define_empty_sa_list!(XCHAL_CP3_SA_LIST);
define_empty_sa_list!(XCHAL_CP4_SA_LIST);
define_empty_sa_list!(XCHAL_CP5_SA_LIST);
define_empty_sa_list!(XCHAL_CP6_SA_LIST);
define_empty_sa_list!(XCHAL_CP7_SA_LIST);

/* Byte length of instruction from its first nibble (op0 field), per FLIX. */
pub const XCHAL_OP0_FORMAT_LENGTHS: [u8; 16] = [3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 3, 3];

/* Byte length of instruction from its first byte, per FLIX. */
pub const XCHAL_BYTE0_FORMAT_LENGTHS: [u8; 256] = [
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3, 3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3, 3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3, 3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3, 3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3, 3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3, 3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3, 3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
    3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3, 3,3,3,3,3,3,3,3,2,2,2,2,2,2,3,3,
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
