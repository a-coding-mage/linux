/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2014 Imagination Technologies Ltd
 *
 * PM helper macros for CPU power off (e.g. Suspend-to-RAM).
 */

/*
 * The following assembler-only declarations are retained as Rust comments.
 * They are consumed by the MIPS assembler rather than by the Rust compiler.
 *
 * .macro SUSPEND_SAVE_REGS
 *     PTR_SUBU sp, PT_SIZE
 *     LONG_S $16, PT_R16(sp); LONG_S $17, PT_R17(sp)
 *     LONG_S $18, PT_R18(sp); LONG_S $19, PT_R19(sp)
 *     LONG_S $20, PT_R20(sp); LONG_S $21, PT_R21(sp)
 *     LONG_S $22, PT_R22(sp); LONG_S $23, PT_R23(sp)
 *     LONG_S $28, PT_R28(sp); LONG_S $30, PT_R30(sp)
 *     LONG_S $31, PT_R31(sp)
 *     mfc0 k0, CP0_STATUS
 *     LONG_S k0, PT_STATUS(sp)
 * .endm
 *
 * .macro RESUME_RESTORE_REGS_RETURN
 *     .set push; .set noreorder
 *     LONG_L k0, PT_STATUS(sp); mtc0 k0, CP0_STATUS
 *     LONG_L $16, PT_R16(sp); LONG_L $17, PT_R17(sp)
 *     LONG_L $18, PT_R18(sp); LONG_L $19, PT_R19(sp)
 *     LONG_L $20, PT_R20(sp); LONG_L $21, PT_R21(sp)
 *     LONG_L $22, PT_R22(sp); LONG_L $23, PT_R23(sp)
 *     LONG_L $28, PT_R28(sp); LONG_L $30, PT_R30(sp)
 *     LONG_L $31, PT_R31(sp)
 *     jr ra; PTR_ADDIU sp, PT_SIZE
 *     .set pop
 * .endm
 *
 * .macro LA_STATIC_SUSPEND
 *     PTR_LA t1, mips_static_suspend_state
 * .endm
 *
 * .macro SUSPEND_SAVE_STATIC
 *     /* With CONFIG_EVA: save CP0_SEGCTL0..2 to SSS_SEGCTL0..2. */
 *     LONG_S sp, SSS_SP(t1)
 * .endm
 *
 * .macro RESUME_RESTORE_STATIC
 *     /* With CONFIG_EVA: restore CP0_SEGCTL0..2 and use tlbw_use_hazard. */
 *     LONG_L sp, SSS_SP(t1)
 * .endm
 *
 * .macro SUSPEND_CACHE_FLUSH
 *     .extern __flush_cache_all
 *     PTR_LA t1, __flush_cache_all
 *     LONG_L t0, 0(t1); jalr t0; nop
 * .endm
 *
 * .macro SUSPEND_SAVE
 *     SUSPEND_SAVE_REGS; LA_STATIC_SUSPEND; SUSPEND_SAVE_STATIC
 *     SUSPEND_CACHE_FLUSH
 * .endm
 *
 * .macro RESUME_RESTORE_RETURN
 *     LA_STATIC_SUSPEND; RESUME_RESTORE_STATIC
 *     RESUME_RESTORE_REGS_RETURN
 * .endm
 */

/// Core saved CPU state across S2R.
///
/// This contains the minimal CPU state saved in static kernel data so the
/// remainder of the state can be restored, including EVA segmentation state.
#[repr(C)]
pub struct mips_static_suspend_state {
    #[cfg(feature = "CONFIG_EVA")]
    pub segctl: [::core::ffi::c_ulong; 3],
    pub sp: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
