// SPDX-License-Identifier: GPL-2.0
/*
 * This program is used to generate definitions needed by
 * assembly language modules.
 *
 * We use the technique used in the OSF Mach kernel code:
 * generate asm statements containing #defines,
 * compile this file to assembler, and then extract the
 * #defines from the assembly-language output.
 */

// The C translation unit defines COMPILE_OFFSETS for the included kernel
// headers.  The corresponding configuration is supplied by the build.

pub fn main() -> i32 {
    /* offsets into the thread_info struct */
    DEFINE!(TI_TASK, core::mem::offset_of!(thread_info, task));
    DEFINE!(TI_FLAGS, core::mem::offset_of!(thread_info, flags));
    DEFINE!(TI_CPU, core::mem::offset_of!(thread_info, cpu));
    DEFINE!(TI_PRE_COUNT, core::mem::offset_of!(thread_info, preempt_count));
    DEFINE!(TI_SIZE, core::mem::size_of::<thread_info>());

    #[cfg(CONFIG_HIBERNATION)]
    {
        DEFINE!(PBE_ADDRESS, core::mem::offset_of!(pbe, address));
        DEFINE!(PBE_ORIG_ADDRESS, core::mem::offset_of!(pbe, orig_address));
        DEFINE!(PBE_NEXT, core::mem::offset_of!(pbe, next));
        DEFINE!(SWSUSP_ARCH_REGS_SIZE, core::mem::size_of::<swsusp_arch_regs>());
    }

    DEFINE!(SH_SLEEP_MODE, core::mem::offset_of!(sh_sleep_data, mode));
    DEFINE!(SH_SLEEP_SF_PRE, core::mem::offset_of!(sh_sleep_data, sf_pre));
    DEFINE!(SH_SLEEP_SF_POST, core::mem::offset_of!(sh_sleep_data, sf_post));
    DEFINE!(SH_SLEEP_RESUME, core::mem::offset_of!(sh_sleep_data, resume));
    DEFINE!(SH_SLEEP_VBR, core::mem::offset_of!(sh_sleep_data, vbr));
    DEFINE!(SH_SLEEP_SPC, core::mem::offset_of!(sh_sleep_data, spc));
    DEFINE!(SH_SLEEP_SR, core::mem::offset_of!(sh_sleep_data, sr));
    DEFINE!(SH_SLEEP_SP, core::mem::offset_of!(sh_sleep_data, sp));
    DEFINE!(SH_SLEEP_BASE_ADDR, core::mem::offset_of!(sh_sleep_data, addr));
    DEFINE!(SH_SLEEP_BASE_DATA, core::mem::offset_of!(sh_sleep_data, data));
    DEFINE!(SH_SLEEP_REG_STBCR, core::mem::offset_of!(sh_sleep_regs, stbcr));
    DEFINE!(SH_SLEEP_REG_BAR, core::mem::offset_of!(sh_sleep_regs, bar));
    DEFINE!(SH_SLEEP_REG_PTEH, core::mem::offset_of!(sh_sleep_regs, pteh));
    DEFINE!(SH_SLEEP_REG_PTEL, core::mem::offset_of!(sh_sleep_regs, ptel));
    DEFINE!(SH_SLEEP_REG_TTB, core::mem::offset_of!(sh_sleep_regs, ttb));
    DEFINE!(SH_SLEEP_REG_TEA, core::mem::offset_of!(sh_sleep_regs, tea));
    DEFINE!(SH_SLEEP_REG_MMUCR, core::mem::offset_of!(sh_sleep_regs, mmucr));
    DEFINE!(SH_SLEEP_REG_PTEA, core::mem::offset_of!(sh_sleep_regs, ptea));
    DEFINE!(SH_SLEEP_REG_PASCR, core::mem::offset_of!(sh_sleep_regs, pascr));
    DEFINE!(SH_SLEEP_REG_IRMCR, core::mem::offset_of!(sh_sleep_regs, irmcr));
    DEFINE!(SH_SLEEP_REG_CCR, core::mem::offset_of!(sh_sleep_regs, ccr));
    DEFINE!(SH_SLEEP_REG_RAMCR, core::mem::offset_of!(sh_sleep_regs, ramcr));
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
