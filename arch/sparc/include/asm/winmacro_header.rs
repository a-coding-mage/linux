/* SPDX-License-Identifier: GPL-2.0 */
/*
 * winmacro.h: Window loading-unloading macros.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

//! Rust translation of the SPARC register-window assembly macros.
//!
//! The constants and symbols used by these macros are supplied by the
//! corresponding ptrace/thread-info definitions and the kernel linker.

#[macro_export]
macro_rules! STORE_WINDOW {
    ($reg:ident) => {{
        unsafe { core::arch::asm!(
            concat!("std %l0, [%", stringify!($reg), " + RW_L0];\n",
                    "std %l2, [%", stringify!($reg), " + RW_L2];\n",
                    "std %l4, [%", stringify!($reg), " + RW_L4];\n",
                    "std %l6, [%", stringify!($reg), " + RW_L6];\n",
                    "std %i0, [%", stringify!($reg), " + RW_I0];\n",
                    "std %i2, [%", stringify!($reg), " + RW_I2];\n",
                    "std %i4, [%", stringify!($reg), " + RW_I4];\n",
                    "std %i6, [%", stringify!($reg), " + RW_I6];")
        ) }
    }};
}

#[macro_export]
macro_rules! LOAD_WINDOW {
    ($reg:ident) => {{
        unsafe { core::arch::asm!(
            concat!("ldd [%", stringify!($reg), " + RW_L0], %l0;\n",
                    "ldd [%", stringify!($reg), " + RW_L2], %l2;\n",
                    "ldd [%", stringify!($reg), " + RW_L4], %l4;\n",
                    "ldd [%", stringify!($reg), " + RW_L6], %l6;\n",
                    "ldd [%", stringify!($reg), " + RW_I0], %i0;\n",
                    "ldd [%", stringify!($reg), " + RW_I2], %i2;\n",
                    "ldd [%", stringify!($reg), " + RW_I4], %i4;\n",
                    "ldd [%", stringify!($reg), " + RW_I6], %i6;")
        ) }
    }};
}

/* Loading and storing struct pt_reg trap frames. */
#[macro_export]
macro_rules! LOAD_PT_INS { ($base_reg:ident) => {{ unsafe { core::arch::asm!(concat!(
    "ldd [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_I0], %i0;\n",
    "ldd [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_I2], %i2;\n",
    "ldd [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_I4], %i4;\n",
    "ldd [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_I6], %i6;")) } }}; }
#[macro_export]
macro_rules! LOAD_PT_GLOBALS { ($base_reg:ident) => {{ unsafe { core::arch::asm!(concat!(
    "ld [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_G1], %g1;\n",
    "ldd [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_G2], %g2;\n",
    "ldd [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_G4], %g4;\n",
    "ldd [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_G6], %g6;")) } }}; }
#[macro_export]
macro_rules! LOAD_PT_YREG { ($base_reg:ident, $scratch:ident) => {{ unsafe { core::arch::asm!(concat!(
    "ld [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_Y], %", stringify!($scratch), ";\n",
    "wr %", stringify!($scratch), ", 0x0, %y;")) } }}; }
#[macro_export]
macro_rules! LOAD_PT_PRIV { ($base_reg:ident, $pt_psr:ident, $pt_pc:ident, $pt_npc:ident) => {{ unsafe { core::arch::asm!(concat!(
    "ld [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_PSR], %", stringify!($pt_psr), ";\n",
    "ld [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_PC], %", stringify!($pt_pc), ";\n",
    "ld [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_NPC], %", stringify!($pt_npc), ";")) } }}; }
#[macro_export]
macro_rules! LOAD_PT_ALL { ($base_reg:ident, $pt_psr:ident, $pt_pc:ident, $pt_npc:ident, $scratch:ident) => {{
    $crate::LOAD_PT_YREG!($base_reg, $scratch); $crate::LOAD_PT_INS!($base_reg);
    $crate::LOAD_PT_GLOBALS!($base_reg); $crate::LOAD_PT_PRIV!($base_reg, $pt_psr, $pt_pc, $pt_npc);
}}; }

/* The remaining store/save macros retain the original instruction sequences. */
#[macro_export]
macro_rules! STORE_PT_INS { ($base_reg:ident) => {{ unsafe { core::arch::asm!(concat!(
    "std %i0, [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_I0];\n",
    "std %i2, [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_I2];\n",
    "std %i4, [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_I4];\n",
    "std %i6, [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_I6];")) } }}; }
#[macro_export]
macro_rules! STORE_PT_GLOBALS { ($base_reg:ident) => {{ unsafe { core::arch::asm!(concat!(
    "st %g1, [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_G1];\n",
    "std %g2, [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_G2];\n",
    "std %g4, [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_G4];\n",
    "std %g6, [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_G6];")) } }}; }
#[macro_export]
macro_rules! STORE_PT_YREG { ($base_reg:ident, $scratch:ident) => {{ unsafe { core::arch::asm!(concat!(
    "rd %y, %", stringify!($scratch), ";\n",
    "st %", stringify!($scratch), ", [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_Y];")) } }}; }
#[macro_export]
macro_rules! STORE_PT_PRIV { ($base_reg:ident, $pt_psr:ident, $pt_pc:ident, $pt_npc:ident) => {{ unsafe { core::arch::asm!(concat!(
    "st %", stringify!($pt_psr), ", [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_PSR];\n",
    "st %", stringify!($pt_pc), ", [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_PC];\n",
    "st %", stringify!($pt_npc), ", [%", stringify!($base_reg), " + STACKFRAME_SZ + PT_NPC];")) } }}; }
#[macro_export]
macro_rules! STORE_PT_ALL { ($base_reg:ident, $reg_psr:ident, $reg_pc:ident, $reg_npc:ident, $g_scratch:ident) => {{
    $crate::STORE_PT_PRIV!($base_reg, $reg_psr, $reg_pc, $reg_npc); $crate::STORE_PT_GLOBALS!($base_reg);
    $crate::STORE_PT_YREG!($base_reg, $g_scratch); $crate::STORE_PT_INS!($base_reg);
}}; }

#[macro_export]
macro_rules! SAVE_BOLIXED_USER_STACK { ($cur_reg:ident, $scratch:ident) => {{ unsafe { core::arch::asm!(concat!(
    "ld [%", stringify!($cur_reg), " + TI_W_SAVED], %", stringify!($scratch), ";\n",
    "sll %", stringify!($scratch), ", 2, %", stringify!($scratch), ";\n",
    "add %", stringify!($scratch), ", %", stringify!($cur_reg), ", %", stringify!($scratch), ";\n",
    "st %sp, [%", stringify!($scratch), " + TI_RWIN_SPTRS];\n",
    "sub %", stringify!($scratch), ", %", stringify!($cur_reg), ", %", stringify!($scratch), ";\n",
    "sll %", stringify!($scratch), ", 4, %", stringify!($scratch), ";\n",
    "add %", stringify!($scratch), ", %", stringify!($cur_reg), ", %", stringify!($scratch), ";\n",
    "std %l0, [%", stringify!($scratch), " + TI_REG_WINDOW + RW_L0];\n",
    "std %l2, [%", stringify!($scratch), " + TI_REG_WINDOW + RW_L2];\n",
    "std %l4, [%", stringify!($scratch), " + TI_REG_WINDOW + RW_L4];\n",
    "std %l6, [%", stringify!($scratch), " + TI_REG_WINDOW + RW_L6];\n",
    "std %i0, [%", stringify!($scratch), " + TI_REG_WINDOW + RW_I0];\n",
    "std %i2, [%", stringify!($scratch), " + TI_REG_WINDOW + RW_I2];\n",
    "std %i4, [%", stringify!($scratch), " + TI_REG_WINDOW + RW_I4];\n",
    "std %i6, [%", stringify!($scratch), " + TI_REG_WINDOW + RW_I6];\n",
    "sub %", stringify!($scratch), ", %", stringify!($cur_reg), ", %", stringify!($scratch), ";\n",
    "srl %", stringify!($scratch), ", 6, %", stringify!($scratch), ";\n",
    "add %", stringify!($scratch), ", 1, %", stringify!($scratch), ";\n",
    "st %", stringify!($scratch), ", [%", stringify!($cur_reg), " + TI_W_SAVED];")) } }}; }
}

/* LOAD_CURRENT has architecture-specific implementations.  The CONFIG_SMP
 * branch is preserved as conditional intent; the non-SMP form is available
 * through the same macro interface. */
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! LOAD_CURRENT { ($dest_reg:ident, $idreg:ident) => {{ unsafe { core::arch::asm!(concat!(
    "sethi %hi(current_set), %", stringify!($idreg), ";\n",
    "ld [%", stringify!($idreg), " + %lo(current_set)], %", stringify!($dest_reg), ";")) } }}; }

#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! LOAD_CURRENT {
    ($dest_reg:ident, $idreg:ident) => {{ unsafe { core::arch::asm!(concat!(
        "rd %tbr, %", stringify!($idreg), ";\n",
        "srl %", stringify!($idreg), ", 10, %", stringify!($idreg), ";\n",
        "and %", stringify!($idreg), ", 0xc, %", stringify!($idreg), ";\n",
        "lda [%g0] ASI_M_VIKING_TMP1, %", stringify!($idreg), ";\n",
        "sll %", stringify!($idreg), ", 2, %", stringify!($idreg), ";\n",
        "rd %asr17, %", stringify!($idreg), ";\n",
        "srl %", stringify!($idreg), ", 0x1c, %", stringify!($idreg), ";\n",
        "sll %", stringify!($idreg), ", 0x02, %", stringify!($idreg), ";\n",
        "sethi %hi(current_set), %", stringify!($dest_reg), ";\n",
        "or %", stringify!($dest_reg), ", %lo(current_set), %", stringify!($dest_reg), ";\n",
        "ld [%", stringify!($idreg), " + %", stringify!($dest_reg), "], %", stringify!($dest_reg), ";"
    )) } }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
