/* SPDX-License-Identifier: GPL-2.0 */

// Translated from testing/selftests/powerpc/include/basic_asm.h.
// Original C/assembler dependencies: <ppc-asm.h> and <asm/unistd.h>.

#[cfg(target_pointer_width = "64")]
pub const PPC_LL: &str = "ld";
#[cfg(target_pointer_width = "64")]
pub const PPC_STL: &str = "std";
#[cfg(target_pointer_width = "64")]
pub const PPC_STLU: &str = "stdu";

#[cfg(not(target_pointer_width = "64"))]
pub const PPC_LL: &str = "lwz";
#[cfg(not(target_pointer_width = "64"))]
pub const PPC_STL: &str = "stw";
#[cfg(not(target_pointer_width = "64"))]
pub const PPC_STLU: &str = "stwu";

#[macro_export]
macro_rules! LOAD_REG_IMMEDIATE {
    ($reg:tt, $expr:tt) => {
        concat!(
            "lis ", stringify!($reg), ", ", stringify!($expr), "@highest;\n",
            "ori ", stringify!($reg), ", ", stringify!($reg), ", ", stringify!($expr), "@higher;\n",
            "rldicr ", stringify!($reg), ", ", stringify!($reg), ", 32, 31;\n",
            "oris ", stringify!($reg), ", ", stringify!($reg), ", ", stringify!($expr), "@high;\n",
            "ori ", stringify!($reg), ", ", stringify!($reg), ", ", stringify!($expr), "@l;\n",
        )
    };
}

/*
 * Note: These macros assume that variables being stored on the stack are
 * sizeof(long), while this is usually the case it may not always be the
 * case for each use case.
 */

// The original header selects ABI v1/v2 using _CALL_ELF when __powerpc64__ is
// defined. Rust has no direct file-local equivalent for that preprocessor
// symbol, so expose both ABI constant sets behind explicit cfg names.

// ABIv2
#[cfg(all(target_pointer_width = "64", powerpc64_elfv2))]
pub const STACK_FRAME_MIN_SIZE: usize = 32;
#[cfg(all(target_pointer_width = "64", powerpc64_elfv2))]
pub const STACK_FRAME_TOC_POS: usize = 24;
#[cfg(all(target_pointer_width = "64", powerpc64_elfv2))]
pub const fn __STACK_FRAME_PARAM(_param: usize) -> usize {
    32 + (_param * 8)
}
#[cfg(all(target_pointer_width = "64", powerpc64_elfv2))]
pub const fn __STACK_FRAME_LOCAL(_num_params: usize, _var_num: usize) -> usize {
    __STACK_FRAME_PARAM(_num_params) + (_var_num * 8)
}

// ABIv1 below
#[cfg(all(target_pointer_width = "64", not(powerpc64_elfv2)))]
pub const STACK_FRAME_MIN_SIZE: usize = 112;
#[cfg(all(target_pointer_width = "64", not(powerpc64_elfv2)))]
pub const STACK_FRAME_TOC_POS: usize = 40;
#[cfg(all(target_pointer_width = "64", not(powerpc64_elfv2)))]
pub const fn __STACK_FRAME_PARAM(i: usize) -> usize {
    48 + (i * 8)
}

/*
 * Caveat: if a function passed more than 8 doublewords, the caller will have
 * made more space... which would render the 112 incorrect.
 */
#[cfg(all(target_pointer_width = "64", not(powerpc64_elfv2)))]
pub const fn __STACK_FRAME_LOCAL(_num_params: usize, _var_num: usize) -> usize {
    112 + (_var_num * 8)
}

// Common 64-bit
#[cfg(target_pointer_width = "64")]
pub const STACK_FRAME_LR_POS: usize = 16;
#[cfg(target_pointer_width = "64")]
pub const STACK_FRAME_CR_POS: usize = 8;

// 32-bit below
#[cfg(not(target_pointer_width = "64"))]
pub const STACK_FRAME_MIN_SIZE: usize = 16;
#[cfg(not(target_pointer_width = "64"))]
pub const STACK_FRAME_LR_POS: usize = 4;

#[cfg(not(target_pointer_width = "64"))]
pub const fn __STACK_FRAME_PARAM(_param: usize) -> usize {
    STACK_FRAME_MIN_SIZE + (_param * 4)
}
#[cfg(not(target_pointer_width = "64"))]
pub const fn __STACK_FRAME_LOCAL(_num_params: usize, _var_num: usize) -> usize {
    __STACK_FRAME_PARAM(_num_params) + (_var_num * 4)
}

/* Parameter x saved to the stack */
pub const fn STACK_FRAME_PARAM(var: usize) -> usize {
    __STACK_FRAME_PARAM(var)
}

/* Local variable x saved to the stack after x parameters */
pub const fn STACK_FRAME_LOCAL(num_params: usize, var: usize) -> usize {
    __STACK_FRAME_LOCAL(num_params, var)
}

/*
 * It is very important to note here that _extra is the extra amount of
 * stack space needed. This space can be accessed using STACK_FRAME_PARAM()
 * or STACK_FRAME_LOCAL() macros.
 *
 * r1 and r2 are not defined in ppc-asm.h (instead they are defined as sp
 * and toc). Kernel programmers tend to prefer rX even for r1 and r2, hence
 * %1 and %r2. r0 is defined in ppc-asm.h and therefore %r0 gets
 * preprocessed incorrectly, hence r0.
 */
pub const fn PUSH_BASIC_STACK_SIZE(_extra: usize) -> usize {
    ((_extra + 15) & !15) + STACK_FRAME_MIN_SIZE
}

#[macro_export]
macro_rules! PUSH_BASIC_STACK {
    ($extra:expr) => {
        concat!(
            "mflr r0;\n",
            PPC_STL,
            " r0, STACK_FRAME_LR_POS(%r1);\n",
            PPC_STLU,
            " %r1, -(((",
            stringify!($extra),
            " + 15) & ~15) + STACK_FRAME_MIN_SIZE)(%r1);\n",
        )
    };
}

#[macro_export]
macro_rules! POP_BASIC_STACK {
    ($extra:expr) => {
        concat!(
            "addi %r1, %r1, (((",
            stringify!($extra),
            " + 15) & ~15) + STACK_FRAME_MIN_SIZE);\n",
            PPC_LL,
            " r0, STACK_FRAME_LR_POS(%r1);\n",
            "mtlr r0;\n",
        )
    };
}

#[macro_export]
macro_rules! OP_REGS {
    ($op:tt, $reg_width:tt, $start_reg:tt, $end_reg:tt, $base_reg:tt) => {
        concat!(
            ".set i, ", stringify!($start_reg), "\n",
            ".rept (", stringify!($end_reg), " - ", stringify!($start_reg), " + 1)\n",
            stringify!($op), " i, (", stringify!($reg_width), " * (i - 0) + 0)(", stringify!($base_reg), ")\n",
            ".set i, i + 1\n",
            ".endr\n",
        )
    };
    ($op:tt, $reg_width:tt, $start_reg:tt, $end_reg:tt, $base_reg:tt, $base_reg_offset:tt) => {
        concat!(
            ".set i, ", stringify!($start_reg), "\n",
            ".rept (", stringify!($end_reg), " - ", stringify!($start_reg), " + 1)\n",
            stringify!($op), " i, (", stringify!($reg_width), " * (i - 0) + ", stringify!($base_reg_offset), ")(", stringify!($base_reg), ")\n",
            ".set i, i + 1\n",
            ".endr\n",
        )
    };
    ($op:tt, $reg_width:tt, $start_reg:tt, $end_reg:tt, $base_reg:tt, $base_reg_offset:tt, $skip:tt) => {
        concat!(
            ".set i, ", stringify!($start_reg), "\n",
            ".rept (", stringify!($end_reg), " - ", stringify!($start_reg), " + 1)\n",
            stringify!($op), " i, (", stringify!($reg_width), " * (i - ", stringify!($skip), ") + ", stringify!($base_reg_offset), ")(", stringify!($base_reg), ")\n",
            ".set i, i + 1\n",
            ".endr\n",
        )
    };
}
