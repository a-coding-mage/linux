/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Generate .byte code for some instructions not supported by old
 * binutils.
 */

pub const REG_NUM_INVALID: i32 = 100;

pub const REG_TYPE_R32: i32 = 0;
pub const REG_TYPE_R64: i32 = 1;
pub const REG_TYPE_INVALID: i32 = 100;

/* CONFIG_X86_64 controls whether the extended and 64-bit registers exist. */

macro_rules! R32_NUM {
    ($opd:ident, %eax) => { $opd = 0; };
    ($opd:ident, %ecx) => { $opd = 1; };
    ($opd:ident, %edx) => { $opd = 2; };
    ($opd:ident, %ebx) => { $opd = 3; };
    ($opd:ident, %esp) => { $opd = 4; };
    ($opd:ident, %ebp) => { $opd = 5; };
    ($opd:ident, %esi) => { $opd = 6; };
    ($opd:ident, %edi) => { $opd = 7; };
    ($opd:ident, %r8d) => { $opd = 8; };
    ($opd:ident, %r9d) => { $opd = 9; };
    ($opd:ident, %r10d) => { $opd = 10; };
    ($opd:ident, %r11d) => { $opd = 11; };
    ($opd:ident, %r12d) => { $opd = 12; };
    ($opd:ident, %r13d) => { $opd = 13; };
    ($opd:ident, %r14d) => { $opd = 14; };
    ($opd:ident, %r15d) => { $opd = 15; };
    ($opd:ident, $reg:tt) => { $opd = REG_NUM_INVALID; };
}

macro_rules! R64_NUM {
    ($opd:ident, %rax) => { $opd = 0; };
    ($opd:ident, %rcx) => { $opd = 1; };
    ($opd:ident, %rdx) => { $opd = 2; };
    ($opd:ident, %rbx) => { $opd = 3; };
    ($opd:ident, %rsp) => { $opd = 4; };
    ($opd:ident, %rbp) => { $opd = 5; };
    ($opd:ident, %rsi) => { $opd = 6; };
    ($opd:ident, %rdi) => { $opd = 7; };
    ($opd:ident, %r8) => { $opd = 8; };
    ($opd:ident, %r9) => { $opd = 9; };
    ($opd:ident, %r10) => { $opd = 10; };
    ($opd:ident, %r11) => { $opd = 11; };
    ($opd:ident, %r12) => { $opd = 12; };
    ($opd:ident, %r13) => { $opd = 13; };
    ($opd:ident, %r14) => { $opd = 14; };
    ($opd:ident, %r15) => { $opd = 15; };
    ($opd:ident, $reg:tt) => { $opd = REG_NUM_INVALID; };
}

macro_rules! REG_TYPE {
    ($type:ident, $reg:tt) => {{
        let mut reg_type_r32 = REG_NUM_INVALID;
        let mut reg_type_r64 = REG_NUM_INVALID;
        R32_NUM!(reg_type_r32, $reg);
        R64_NUM!(reg_type_r64, $reg);
        if reg_type_r64 != REG_NUM_INVALID {
            $type = REG_TYPE_R64;
        } else if reg_type_r32 != REG_NUM_INVALID {
            $type = REG_TYPE_R32;
        } else {
            $type = REG_TYPE_INVALID;
        }
    }};
}

macro_rules! PFX_REX {
    ($opd1:expr, $opd2:expr $(, $W:expr)?) => {{
        let w = 0 $(+ $W)?;
        if ((($opd1 | $opd2) & 8) != 0) || w != 0 {
            0x40 | (($opd1 & 8) >> 3) | (($opd2 & 8) >> 1) | (w << 3)
        } else {
            0
        }
    }};
}

macro_rules! MODRM {
    ($mod:expr, $opd1:expr, $opd2:expr) => {
        $mod | ($opd1 & 7) | (($opd2 & 7) << 3)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
