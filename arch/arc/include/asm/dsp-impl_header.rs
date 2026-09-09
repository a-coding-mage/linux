/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Synopsys, Inc. (www.synopsys.com)
 *
 * Author: Eugeniy Paltsev <Eugeniy.Paltsev@synopsys.com>
 */

// Dependency supplied by the original <asm/dsp.h> header.

pub const DSP_CTRL_DISABLED_ALL: usize = 0;

/*
 * The following assembler macros are retained as source-level documentation.
 * They are emitted by the ARC assembly users of this header.
 */
/* clobbers r5 register
.macro DSP_EARLY_INIT
#ifdef CONFIG_ISA_ARCV2
    lr r5, [ARC_AUX_DSP_BUILD]
    bmsk r5, r5, 7
    breq r5, 0, 1f
    mov r5, DSP_CTRL_DISABLED_ALL
    sr r5, [ARC_AUX_DSP_CTRL]
1:
#endif
.endm
*/

/* clobbers r10, r11 registers pair */
/* DSP_SAVE_REGFILE_IRQ and DSP_RESTORE_REGFILE_IRQ are ARC assembler macros;
 * their conditional bodies are preserved in the comment above the Rust API. */

#[cfg(CONFIG_ARC_DSP_SAVE_RESTORE_REGS)]
#[inline(always)]
pub unsafe fn dsp_save_restore(prev: *mut task_struct, next: *mut task_struct) {
    // As in the C implementation, each operation swaps an auxiliary register
    // with the corresponding saved value using the ARC AEX instruction.
    let saveto: *mut usize = &mut (*prev).thread.dsp.ACC0_GLO;
    let readfrom: *mut usize = &mut (*next).thread.dsp.ACC0_GLO;

    dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, ACC0_GLO), ARC_AUX_ACC0_GLO);
    dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, ACC0_GHI), ARC_AUX_ACC0_GHI);
    dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, DSP_BFLY0), ARC_AUX_DSP_BFLY0);
    dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, DSP_FFT_CTRL), ARC_AUX_DSP_FFT_CTRL);

    #[cfg(CONFIG_ARC_DSP_AGU_USERSPACE)]
    {
        dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, AGU_AP0), ARC_AUX_AGU_AP0);
        dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, AGU_AP1), ARC_AUX_AGU_AP1);
        dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, AGU_AP2), ARC_AUX_AGU_AP2);
        dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, AGU_AP3), ARC_AUX_AGU_AP3);
        dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, AGU_OS0), ARC_AUX_AGU_OS0);
        dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, AGU_OS1), ARC_AUX_AGU_OS1);
        dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, AGU_MOD0), ARC_AUX_AGU_MOD0);
        dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, AGU_MOD1), ARC_AUX_AGU_MOD1);
        dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, AGU_MOD2), ARC_AUX_AGU_MOD2);
        dsp_aux_save_restore(saveto, readfrom, offset_of!(dsp_callee_regs, AGU_MOD3), ARC_AUX_AGU_MOD3);
    }
}

#[cfg(not(CONFIG_ARC_DSP_SAVE_RESTORE_REGS))]
#[inline(always)]
pub unsafe fn dsp_save_restore(_p: *mut task_struct, _n: *mut task_struct) {}

#[inline(always)]
unsafe fn dsp_aux_save_restore(
    saveto: *mut usize,
    readfrom: *mut usize,
    offset: usize,
    aux: usize,
) {
    let mut scratch: usize;
    core::arch::asm!(
        "ld {scratch}, [{readfrom}, {offset}]",
        "aex {scratch}, [{aux}]",
        "st {scratch}, [{saveto}, {offset}]",
        scratch = lateout(reg) scratch,
        saveto = in(reg) saveto,
        readfrom = in(reg) readfrom,
        aux = in(reg) aux,
        offset = in(reg) offset,
        options(nostack)
    );
}

#[inline(always)]
pub unsafe fn dsp_exist() -> bool {
    let mut bcr: bcr_generic = core::mem::zeroed();
    READ_BCR(ARC_AUX_DSP_BUILD, &mut bcr);
    bcr.ver != 0
}

#[inline(always)]
pub unsafe fn agu_exist() -> bool {
    let mut bcr: bcr_generic = core::mem::zeroed();
    READ_BCR(ARC_AUX_AGU_BUILD, &mut bcr);
    bcr.ver != 0
}

#[inline(always)]
pub unsafe fn dsp_config_check() {
    CHK_OPT_STRICT(CONFIG_ARC_DSP_HANDLED, dsp_exist());
    CHK_OPT_WEAK(CONFIG_ARC_DSP_AGU_USERSPACE, agu_exist());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
