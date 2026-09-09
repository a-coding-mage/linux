/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    NetWinder Floating Point Emulator
    (c) Rebel.com, 1998-1999

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>

*/

/*
The FPSR is a 32 bit register consisting of 4 parts, each exactly
one byte.

    SYSTEM ID
    EXCEPTION TRAP ENABLE BYTE
    SYSTEM CONTROL BYTE
    CUMULATIVE EXCEPTION FLAGS BYTE

The FPCR is a 32 bit register consisting of bit flags.
*/

/* SYSTEM ID
------------
Note: the system id byte is read only  */

pub type FPSR = u32; /* type for floating point status register */
pub type FPCR = u32; /* type for floating point control register */

pub const MASK_SYSID: u32 = 0xff000000;
pub const BIT_HARDWARE: u32 = 0x80000000;
pub const FP_EMULATOR: u32 = 0x01000000; /* System ID for emulator */
pub const FP_ACCELERATOR: u32 = 0x81000000; /* System ID for FPA11 */

/* EXCEPTION TRAP ENABLE BYTE
----------------------------- */

pub const MASK_TRAP_ENABLE: u32 = 0x00ff0000;
pub const MASK_TRAP_ENABLE_STRICT: u32 = 0x001f0000;
pub const BIT_IXE: u32 = 0x00100000; /* inexact exception enable */
pub const BIT_UFE: u32 = 0x00080000; /* underflow exception enable */
pub const BIT_OFE: u32 = 0x00040000; /* overflow exception enable */
pub const BIT_DZE: u32 = 0x00020000; /* divide by zero exception enable */
pub const BIT_IOE: u32 = 0x00010000; /* invalid operation exception enable */

/* SYSTEM CONTROL BYTE
---------------------- */

pub const MASK_SYSTEM_CONTROL: u32 = 0x0000ff00;
pub const MASK_TRAP_STRICT: u32 = 0x00001f00;

pub const BIT_AC: u32 = 0x00001000; /* use alternative C-flag definition
                                       for compares */
pub const BIT_EP: u32 = 0x00000800; /* use expanded packed decimal format */
pub const BIT_SO: u32 = 0x00000400; /* select synchronous operation of FPA */
pub const BIT_NE: u32 = 0x00000200; /* NaN exception bit */
pub const BIT_ND: u32 = 0x00000100; /* no denormalized numbers bit */

/* CUMULATIVE EXCEPTION FLAGS BYTE
---------------------------------- */

pub const MASK_EXCEPTION_FLAGS: u32 = 0x000000ff;
pub const MASK_EXCEPTION_FLAGS_STRICT: u32 = 0x0000001f;

pub const BIT_IXC: u32 = 0x00000010; /* inexact exception flag */
pub const BIT_UFC: u32 = 0x00000008; /* underflow exception flag */
pub const BIT_OFC: u32 = 0x00000004; /* overfloat exception flag */
pub const BIT_DZC: u32 = 0x00000002; /* divide by zero exception flag */
pub const BIT_IOC: u32 = 0x00000001; /* invalid operation exception flag */

/* Floating Point Control Register
----------------------------------*/

pub const BIT_RU: u32 = 0x80000000; /* rounded up bit */
pub const BIT_IE: u32 = 0x10000000; /* inexact bit */
pub const BIT_MO: u32 = 0x08000000; /* mantissa overflow bit */
pub const BIT_EO: u32 = 0x04000000; /* exponent overflow bit */
pub const BIT_SB: u32 = 0x00000800; /* store bounce */
pub const BIT_AB: u32 = 0x00000400; /* arithmetic bounce */
pub const BIT_RE: u32 = 0x00000200; /* rounding exception */
pub const BIT_DA: u32 = 0x00000100; /* disable FPA */

pub const MASK_OP: u32 = 0x00f08010; /* AU operation code */
pub const MASK_PR: u32 = 0x00080080; /* AU precision */
pub const MASK_S1: u32 = 0x00070000; /* AU source register 1 */
pub const MASK_S2: u32 = 0x00000007; /* AU source register 2 */
pub const MASK_DS: u32 = 0x00007000; /* AU destination register */
pub const MASK_RM: u32 = 0x00000060; /* AU rounding mode */
pub const MASK_ALU: u32 = 0x9cfff2ff; /* only ALU can write these bits */
pub const MASK_RESET: u32 = 0x00000d00; /* bits set on reset, all others cleared */
pub const MASK_WFC: u32 = MASK_RESET;
pub const MASK_RFC: u32 = !MASK_RESET;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
