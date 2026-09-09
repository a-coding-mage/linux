// SPDX-License-Identifier: GPL-2.0-only
/*
 * fpu.rs - save/restore of Floating Point Unit Registers on task switch
 *
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_ISA_ARCOMPACT)]
/*
 * To save/restore FPU regs, simplest scheme would use LR/SR insns.
 * However since SR serializes the pipeline, an alternate "hack" can be used
 * which uses the FPU Exchange insn (DEXCL) to r/w FPU regs.
 *
 * Store to 64bit dpfp1 reg from a pair of core regs:
 *   dexcl1 0, r1, r0  ; where r1:r0 is the 64 bit val
 *
 * Read from dpfp1 into pair of core regs (w/o clobbering dpfp1)
 *   mov_s    r3, 0
 *   daddh11  r1, r3, r3   ; get "hi" into r1 (dpfp1 unchanged)
 *   dexcl1   r0, r1, r3   ; get "low" into r0 (dpfp1 low clobbered)
 *   dexcl1    0, r1, r0   ; restore dpfp1 to orig value
 *
 * However we can tweak the read, so that read-out of outgoing task's FPU regs
 * and write of incoming task's regs happen in one shot. So all the work is
 * done before context switch
 */
pub unsafe fn fpu_save_restore(prev: *mut task_struct, next: *mut task_struct) {
    let saveto: *mut u32 = &mut (*prev).thread.fpu.aux_dpfp[0].l;
    let readfrom: *const u32 = &(*next).thread.fpu.aux_dpfp[0].l;

    let zero: u32 = 0;

    core::arch::asm!(
        "daddh11  {0}, {2}, {2}\n",
        "dexcl1   {1}, {3}, {4}\n",
        out(reg) *saveto.add(1),
        out(reg) *saveto,
        in(reg) zero,
        in(reg) *readfrom.add(1),
        in(reg) *readfrom,
    );

    core::arch::asm!(
        "daddh22  {0}, {2}, {2}\n",
        "dexcl2   {1}, {3}, {4}\n",
        out(reg) *saveto.add(3),
        out(reg) *saveto.add(2),
        in(reg) zero,
        in(reg) *readfrom.add(3),
        in(reg) *readfrom.add(2),
    );
}

#[cfg(not(CONFIG_ISA_ARCOMPACT))]
pub unsafe fn fpu_init_task(regs: *mut pt_regs) {
    let fwe: u32 = 0x80000000;

    /* default rounding mode */
    write_aux_reg(ARC_REG_FPU_CTRL, 0x100);

    /* Initialize to zero: setting requires FWE be set */
    write_aux_reg(ARC_REG_FPU_STATUS, fwe);
}

#[cfg(not(CONFIG_ISA_ARCOMPACT))]
pub unsafe fn fpu_save_restore(prev: *mut task_struct, next: *mut task_struct) {
    let save: *mut arc_fpu = &mut (*prev).thread.fpu;
    let restore: *mut arc_fpu = &mut (*next).thread.fpu;
    let fwe: u32 = 0x80000000;

    (*save).ctrl = read_aux_reg(ARC_REG_FPU_CTRL);
    (*save).status = read_aux_reg(ARC_REG_FPU_STATUS);

    write_aux_reg(ARC_REG_FPU_CTRL, (*restore).ctrl);
    write_aux_reg(ARC_REG_FPU_STATUS, fwe | (*restore).status);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
