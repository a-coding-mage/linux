// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 1995-2003 Russell King
 *               2001-2002 Keith Owens
 *
 * Generate definitions needed by assembly language modules.
 * This code generates raw asm output which is post-processed to extract
 * and format the required data.
 */

// The C source defines COMPILE_OFFSETS and includes kernel headers.  Those
// dependencies are supplied by the surrounding kernel translation unit.

/*
 * Make sure that the compiler and target are compatible.
 * The APCS-26 build-time error has no direct Rust equivalent here.
 */

pub fn main() -> i32 {
    DEFINE!("TSK_ACTIVE_MM", offset_of!(task_struct, active_mm));
    #[cfg(feature = "CONFIG_STACKPROTECTOR")]
    DEFINE!("TSK_STACK_CANARY", offset_of!(task_struct, stack_canary));
    BLANK!();
    DEFINE!("TI_FLAGS", offset_of!(thread_info, flags));
    DEFINE!("TI_PREEMPT", offset_of!(thread_info, preempt_count));
    DEFINE!("TI_CPU", offset_of!(thread_info, cpu));
    DEFINE!("TI_CPU_DOMAIN", offset_of!(thread_info, cpu_domain));
    DEFINE!("TI_CPU_SAVE", offset_of!(thread_info, cpu_context));
    DEFINE!("TI_ABI_SYSCALL", offset_of!(thread_info, abi_syscall));
    DEFINE!("TI_TP_VALUE", offset_of!(thread_info, tp_value));
    DEFINE!("TI_FPSTATE", offset_of!(thread_info, fpstate));
    #[cfg(feature = "CONFIG_VFP")]
    {
        DEFINE!("TI_VFPSTATE", offset_of!(thread_info, vfpstate));
        #[cfg(feature = "CONFIG_SMP")]
        DEFINE!("VFP_CPU", offset_of!(vfp_state, hard.cpu));
    }
    DEFINE!("SOFTIRQ_DISABLE_OFFSET", SOFTIRQ_DISABLE_OFFSET);
    #[cfg(feature = "CONFIG_ARM_THUMBEE")]
    DEFINE!("TI_THUMBEE_STATE", offset_of!(thread_info, thumbee_state));
    #[cfg(feature = "CONFIG_IWMMXT")]
    DEFINE!("TI_IWMMXT_STATE", offset_of!(thread_info, fpstate.iwmmxt));
    BLANK!();
    DEFINE!("S_R0", offset_of!(pt_regs, ARM_r0));
    DEFINE!("S_R1", offset_of!(pt_regs, ARM_r1));
    DEFINE!("S_R2", offset_of!(pt_regs, ARM_r2));
    DEFINE!("S_R3", offset_of!(pt_regs, ARM_r3));
    DEFINE!("S_R4", offset_of!(pt_regs, ARM_r4));
    DEFINE!("S_R5", offset_of!(pt_regs, ARM_r5));
    DEFINE!("S_R6", offset_of!(pt_regs, ARM_r6));
    DEFINE!("S_R7", offset_of!(pt_regs, ARM_r7));
    DEFINE!("S_R8", offset_of!(pt_regs, ARM_r8));
    DEFINE!("S_R9", offset_of!(pt_regs, ARM_r9));
    DEFINE!("S_R10", offset_of!(pt_regs, ARM_r10));
    DEFINE!("S_FP", offset_of!(pt_regs, ARM_fp));
    DEFINE!("S_IP", offset_of!(pt_regs, ARM_ip));
    DEFINE!("S_SP", offset_of!(pt_regs, ARM_sp));
    DEFINE!("S_LR", offset_of!(pt_regs, ARM_lr));
    DEFINE!("S_PC", offset_of!(pt_regs, ARM_pc));
    DEFINE!("S_PSR", offset_of!(pt_regs, ARM_cpsr));
    DEFINE!("S_OLD_R0", offset_of!(pt_regs, ARM_ORIG_r0));
    DEFINE!("PT_REGS_SIZE", size_of::<pt_regs>());
    DEFINE!("SVC_DACR", offset_of!(svc_pt_regs, dacr));
    DEFINE!("SVC_TTBCR", offset_of!(svc_pt_regs, ttbcr));
    DEFINE!("SVC_REGS_SIZE", size_of::<svc_pt_regs>());
    BLANK!();
    DEFINE!("SIGFRAME_RC3_OFFSET", offset_of!(sigframe, retcode[3]));
    DEFINE!("RT_SIGFRAME_RC3_OFFSET", offset_of!(rt_sigframe, sig.retcode[3]));
    BLANK!();
    #[cfg(feature = "CONFIG_CACHE_L2X0")]
    {
        DEFINE!("L2X0_R_PHY_BASE", offset_of!(l2x0_regs, phy_base));
        DEFINE!("L2X0_R_AUX_CTRL", offset_of!(l2x0_regs, aux_ctrl));
        DEFINE!("L2X0_R_TAG_LATENCY", offset_of!(l2x0_regs, tag_latency));
        DEFINE!("L2X0_R_DATA_LATENCY", offset_of!(l2x0_regs, data_latency));
        DEFINE!("L2X0_R_FILTER_START", offset_of!(l2x0_regs, filter_start));
        DEFINE!("L2X0_R_FILTER_END", offset_of!(l2x0_regs, filter_end));
        DEFINE!("L2X0_R_PREFETCH_CTRL", offset_of!(l2x0_regs, prefetch_ctrl));
        DEFINE!("L2X0_R_PWR_CTRL", offset_of!(l2x0_regs, pwr_ctrl));
        BLANK!();
    }
    #[cfg(feature = "CONFIG_CPU_HAS_ASID")]
    {
        DEFINE!("MM_CONTEXT_ID", offset_of!(mm_struct, context.id.counter));
        BLANK!();
    }
    DEFINE!("VMA_VM_MM", offset_of!(vm_area_struct, vm_mm));
    DEFINE!("VMA_VM_FLAGS", offset_of!(vm_area_struct, vm_flags));
    BLANK!();
    DEFINE!("VM_EXEC", VM_EXEC);
    BLANK!();
    DEFINE!("PAGE_SZ", PAGE_SIZE);
    BLANK!();
    DEFINE!("SYS_ERROR0", 0x9f0000);
    BLANK!();
    DEFINE!("SIZEOF_MACHINE_DESC", size_of::<machine_desc>());
    DEFINE!("MACHINFO_TYPE", offset_of!(machine_desc, nr));
    DEFINE!("MACHINFO_NAME", offset_of!(machine_desc, name));
    BLANK!();
    DEFINE!("PROC_INFO_SZ", size_of::<proc_info_list>());
    DEFINE!("PROCINFO_INITFUNC", offset_of!(proc_info_list, __cpu_flush));
    DEFINE!("PROCINFO_MM_MMUFLAGS", offset_of!(proc_info_list, __cpu_mm_mmu_flags));
    DEFINE!("PROCINFO_IO_MMUFLAGS", offset_of!(proc_info_list, __cpu_io_mmu_flags));
    BLANK!();
    #[cfg(feature = "MULTI_DABORT")]
    DEFINE!("PROCESSOR_DABT_FUNC", offset_of!(processor, _data_abort));
    #[cfg(feature = "MULTI_PABORT")]
    DEFINE!("PROCESSOR_PABT_FUNC", offset_of!(processor, _prefetch_abort));
    #[cfg(feature = "MULTI_CPU")]
    {
        DEFINE!("CPU_SLEEP_SIZE", offset_of!(processor, suspend_size));
        DEFINE!("CPU_DO_SUSPEND", offset_of!(processor, do_suspend));
        DEFINE!("CPU_DO_RESUME", offset_of!(processor, do_resume));
    }
    #[cfg(feature = "MULTI_CACHE")]
    DEFINE!("CACHE_FLUSH_KERN_ALL", offset_of!(cpu_cache_fns, flush_kern_all));
    #[cfg(feature = "CONFIG_ARM_CPU_SUSPEND")]
    {
        DEFINE!("SLEEP_SAVE_SP_SZ", size_of::<sleep_save_sp>());
        DEFINE!("SLEEP_SAVE_SP_PHYS", offset_of!(sleep_save_sp, save_ptr_stash_phys));
        DEFINE!("SLEEP_SAVE_SP_VIRT", offset_of!(sleep_save_sp, save_ptr_stash));
    }
    DEFINE!("ARM_SMCCC_QUIRK_ID_OFFS", offset_of!(arm_smccc_quirk, id));
    DEFINE!("ARM_SMCCC_QUIRK_STATE_OFFS", offset_of!(arm_smccc_quirk, state));
    BLANK!();
    DEFINE!("DMA_BIDIRECTIONAL", DMA_BIDIRECTIONAL);
    DEFINE!("DMA_TO_DEVICE", DMA_TO_DEVICE);
    DEFINE!("DMA_FROM_DEVICE", DMA_FROM_DEVICE);
    BLANK!();
    DEFINE!("CACHE_WRITEBACK_ORDER", __CACHE_WRITEBACK_ORDER);
    DEFINE!("CACHE_WRITEBACK_GRANULE", __CACHE_WRITEBACK_GRANULE);
    BLANK!();
    #[cfg(feature = "CONFIG_ARM_MPU")]
    {
        DEFINE!("MPU_RNG_INFO_RNGS", offset_of!(mpu_rgn_info, rgns));
        DEFINE!("MPU_RNG_INFO_USED", offset_of!(mpu_rgn_info, used));
        DEFINE!("MPU_RNG_SIZE", size_of::<mpu_rgn>());
        DEFINE!("MPU_RGN_DRBAR", offset_of!(mpu_rgn, drbar));
        DEFINE!("MPU_RGN_DRSR", offset_of!(mpu_rgn, drsr));
        DEFINE!("MPU_RGN_DRACR", offset_of!(mpu_rgn, dracr));
        DEFINE!("MPU_RGN_PRBAR", offset_of!(mpu_rgn, prbar));
        DEFINE!("MPU_RGN_PRLAR", offset_of!(mpu_rgn, prlar));
    }
    DEFINE!("KEXEC_START_ADDR", offset_of!(kexec_relocate_data, kexec_start_address));
    DEFINE!("KEXEC_INDIR_PAGE", offset_of!(kexec_relocate_data, kexec_indirection_page));
    DEFINE!("KEXEC_MACH_TYPE", offset_of!(kexec_relocate_data, kexec_mach_type));
    DEFINE!("KEXEC_R2", offset_of!(kexec_relocate_data, kexec_r2));
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
