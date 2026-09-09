// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/kernel/asm-offsets.c
 *
 * Copyright (C) 1995-2003 Russell King
 *               2001-2002 Keith Owens
 * Copyright (C) 2012 ARM Ltd.
 */
// COMPILE_OFFSETS
// External kernel and architecture declarations supplied by dependencies.

fn main() {
    DEFINE!(TSK_TI_CPU, offset_of!(task_struct, thread_info.cpu));
    DEFINE!(TSK_TI_FLAGS, offset_of!(task_struct, thread_info.flags));
    DEFINE!(TSK_TI_PREEMPT, offset_of!(task_struct, thread_info.preempt_count));
#[cfg(CONFIG_ARM64_SW_TTBR0_PAN)]
    DEFINE!(TSK_TI_TTBR0, offset_of!(task_struct, thread_info.ttbr0));
#[cfg(CONFIG_SHADOW_CALL_STACK)]
    DEFINE!(TSK_TI_SCS_BASE, offset_of!(task_struct, thread_info.scs_base));
#[cfg(CONFIG_SHADOW_CALL_STACK)]
    DEFINE!(TSK_TI_SCS_SP, offset_of!(task_struct, thread_info.scs_sp));
    DEFINE!(TSK_STACK, offset_of!(task_struct, stack));
#[cfg(CONFIG_STACKPROTECTOR)]
    DEFINE!(TSK_STACK_CANARY, offset_of!(task_struct, stack_canary));
    BLANK!();
    DEFINE!(THREAD_CPU_CONTEXT, offset_of!(task_struct, thread.cpu_context));
    DEFINE!(THREAD_SCTLR_USER, offset_of!(task_struct, thread.sctlr_user));
#[cfg(CONFIG_ARM64_PTR_AUTH)]
    DEFINE!(THREAD_KEYS_USER, offset_of!(task_struct, thread.keys_user));
#[cfg(CONFIG_ARM64_PTR_AUTH_KERNEL)]
    DEFINE!(THREAD_KEYS_KERNEL, offset_of!(task_struct, thread.keys_kernel));
#[cfg(CONFIG_ARM64_MTE)]
    DEFINE!(THREAD_MTE_CTRL, offset_of!(task_struct, thread.mte_ctrl));
    BLANK!();
    DEFINE!(S_X0, offset_of!(pt_regs, regs[0]));
    DEFINE!(S_X2, offset_of!(pt_regs, regs[2]));
    DEFINE!(S_X4, offset_of!(pt_regs, regs[4]));
    DEFINE!(S_X6, offset_of!(pt_regs, regs[6]));
    DEFINE!(S_X8, offset_of!(pt_regs, regs[8]));
    DEFINE!(S_X10, offset_of!(pt_regs, regs[10]));
    DEFINE!(S_X12, offset_of!(pt_regs, regs[12]));
    DEFINE!(S_X14, offset_of!(pt_regs, regs[14]));
    DEFINE!(S_X16, offset_of!(pt_regs, regs[16]));
    DEFINE!(S_X18, offset_of!(pt_regs, regs[18]));
    DEFINE!(S_X20, offset_of!(pt_regs, regs[20]));
    DEFINE!(S_X22, offset_of!(pt_regs, regs[22]));
    DEFINE!(S_X24, offset_of!(pt_regs, regs[24]));
    DEFINE!(S_X26, offset_of!(pt_regs, regs[26]));
    DEFINE!(S_X28, offset_of!(pt_regs, regs[28]));
    DEFINE!(S_FP, offset_of!(pt_regs, regs[29]));
    DEFINE!(S_LR, offset_of!(pt_regs, regs[30]));
    DEFINE!(S_SP, offset_of!(pt_regs, sp));
    DEFINE!(S_PC, offset_of!(pt_regs, pc));
    DEFINE!(S_PSTATE, offset_of!(pt_regs, pstate));
    DEFINE!(S_SYSCALLNO, offset_of!(pt_regs, syscallno));
    DEFINE!(S_SDEI_TTBR1, offset_of!(pt_regs, sdei_ttbr1));
    DEFINE!(S_PMR, offset_of!(pt_regs, pmr));
    DEFINE!(S_STACKFRAME, offset_of!(pt_regs, stackframe));
    DEFINE!(S_STACKFRAME_TYPE, offset_of!(pt_regs, stackframe.type));
    DEFINE!(PT_REGS_SIZE, size_of!(pt_regs));
    BLANK!();
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
    DEFINE!(FREGS_X0, offset_of!(__arch_ftrace_regs, regs[0]));
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
    DEFINE!(FREGS_X2, offset_of!(__arch_ftrace_regs, regs[2]));
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
    DEFINE!(FREGS_X4, offset_of!(__arch_ftrace_regs, regs[4]));
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
    DEFINE!(FREGS_X6, offset_of!(__arch_ftrace_regs, regs[6]));
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
    DEFINE!(FREGS_X8, offset_of!(__arch_ftrace_regs, regs[8]));
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
    DEFINE!(FREGS_FP, offset_of!(__arch_ftrace_regs, fp));
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
    DEFINE!(FREGS_LR, offset_of!(__arch_ftrace_regs, lr));
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
    DEFINE!(FREGS_SP, offset_of!(__arch_ftrace_regs, sp));
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
    DEFINE!(FREGS_PC, offset_of!(__arch_ftrace_regs, pc));
#[cfg(all(CONFIG_DYNAMIC_FTRACE_WITH_ARGS, CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS))]
    DEFINE!(FREGS_DIRECT_TRAMP, offset_of!(__arch_ftrace_regs, direct_tramp));
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
    DEFINE!(FREGS_SIZE, size_of!(__arch_ftrace_regs));
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
    BLANK!();
    DEFINE!(CPU_BOOT_TASK, offset_of!(secondary_data, task));
    BLANK!();
    DEFINE!(FTR_OVR_VAL_OFFSET, offset_of!(arm64_ftr_override, val));
    DEFINE!(FTR_OVR_MASK_OFFSET, offset_of!(arm64_ftr_override, mask));
    BLANK!();
#[cfg(CONFIG_KVM)]
    DEFINE!(VCPU_CONTEXT, offset_of!(kvm_vcpu, arch.ctxt));
#[cfg(CONFIG_KVM)]
    DEFINE!(VCPU_FAULT_DISR, offset_of!(kvm_vcpu, arch.fault.disr_el1));
#[cfg(CONFIG_KVM)]
    DEFINE!(VCPU_HCR_EL2, offset_of!(kvm_vcpu, arch.hcr_el2));
#[cfg(CONFIG_KVM)]
    DEFINE!(CPU_USER_PT_REGS, offset_of!(kvm_cpu_context, regs));
#[cfg(CONFIG_KVM)]
    DEFINE!(CPU_ELR_EL2, offset_of!(kvm_cpu_context, sys_regs[ELR_EL2]));
#[cfg(CONFIG_KVM)]
    DEFINE!(CPU_RGSR_EL1, offset_of!(kvm_cpu_context, sys_regs[RGSR_EL1]));
#[cfg(CONFIG_KVM)]
    DEFINE!(CPU_GCR_EL1, offset_of!(kvm_cpu_context, sys_regs[GCR_EL1]));
#[cfg(CONFIG_KVM)]
    DEFINE!(CPU_APIAKEYLO_EL1, offset_of!(kvm_cpu_context, sys_regs[APIAKEYLO_EL1]));
#[cfg(CONFIG_KVM)]
    DEFINE!(CPU_APIBKEYLO_EL1, offset_of!(kvm_cpu_context, sys_regs[APIBKEYLO_EL1]));
#[cfg(CONFIG_KVM)]
    DEFINE!(CPU_APDAKEYLO_EL1, offset_of!(kvm_cpu_context, sys_regs[APDAKEYLO_EL1]));
#[cfg(CONFIG_KVM)]
    DEFINE!(CPU_APDBKEYLO_EL1, offset_of!(kvm_cpu_context, sys_regs[APDBKEYLO_EL1]));
#[cfg(CONFIG_KVM)]
    DEFINE!(CPU_APGAKEYLO_EL1, offset_of!(kvm_cpu_context, sys_regs[APGAKEYLO_EL1]));
#[cfg(CONFIG_KVM)]
    DEFINE!(HOST_CONTEXT_VCPU, offset_of!(kvm_cpu_context, __hyp_running_vcpu));
#[cfg(CONFIG_KVM)]
    DEFINE!(HOST_DATA_CONTEXT, offset_of!(kvm_host_data, host_ctxt));
#[cfg(CONFIG_KVM)]
    DEFINE!(NVHE_INIT_MAIR_EL2, offset_of!(kvm_nvhe_init_params, mair_el2));
#[cfg(CONFIG_KVM)]
    DEFINE!(NVHE_INIT_TCR_EL2, offset_of!(kvm_nvhe_init_params, tcr_el2));
#[cfg(CONFIG_KVM)]
    DEFINE!(NVHE_INIT_TPIDR_EL2, offset_of!(kvm_nvhe_init_params, tpidr_el2));
#[cfg(CONFIG_KVM)]
    DEFINE!(NVHE_INIT_STACK_HYP_VA, offset_of!(kvm_nvhe_init_params, stack_hyp_va));
#[cfg(CONFIG_KVM)]
    DEFINE!(NVHE_INIT_PGD_PA, offset_of!(kvm_nvhe_init_params, pgd_pa));
#[cfg(CONFIG_KVM)]
    DEFINE!(NVHE_INIT_HCR_EL2, offset_of!(kvm_nvhe_init_params, hcr_el2));
#[cfg(CONFIG_KVM)]
    DEFINE!(NVHE_INIT_VTTBR, offset_of!(kvm_nvhe_init_params, vttbr));
#[cfg(CONFIG_KVM)]
    DEFINE!(NVHE_INIT_VTCR, offset_of!(kvm_nvhe_init_params, vtcr));
#[cfg(CONFIG_CPU_PM)]
    DEFINE!(CPU_CTX_SP, offset_of!(cpu_suspend_ctx, sp));
#[cfg(CONFIG_CPU_PM)]
    DEFINE!(MPIDR_HASH_MASK, offset_of!(mpidr_hash, mask));
#[cfg(CONFIG_CPU_PM)]
    DEFINE!(MPIDR_HASH_SHIFTS, offset_of!(mpidr_hash, shift_aff));
#[cfg(CONFIG_CPU_PM)]
    DEFINE!(SLEEP_STACK_DATA_SYSTEM_REGS, offset_of!(sleep_stack_data, system_regs));
#[cfg(CONFIG_CPU_PM)]
    DEFINE!(SLEEP_STACK_DATA_CALLEE_REGS, offset_of!(sleep_stack_data, callee_saved_regs));
    DEFINE!(ARM_SMCCC_RES_X0_OFFS, offset_of!(arm_smccc_res, a0));
    DEFINE!(ARM_SMCCC_RES_X2_OFFS, offset_of!(arm_smccc_res, a2));
    DEFINE!(ARM_SMCCC_QUIRK_ID_OFFS, offset_of!(arm_smccc_quirk, id));
    DEFINE!(ARM_SMCCC_QUIRK_STATE_OFFS, offset_of!(arm_smccc_quirk, state));
    DEFINE!(ARM_SMCCC_1_2_REGS_X0_OFFS, offset_of!(arm_smccc_1_2_regs, a0));
    DEFINE!(ARM_SMCCC_1_2_REGS_X2_OFFS, offset_of!(arm_smccc_1_2_regs, a2));
    DEFINE!(ARM_SMCCC_1_2_REGS_X4_OFFS, offset_of!(arm_smccc_1_2_regs, a4));
    DEFINE!(ARM_SMCCC_1_2_REGS_X6_OFFS, offset_of!(arm_smccc_1_2_regs, a6));
    DEFINE!(ARM_SMCCC_1_2_REGS_X8_OFFS, offset_of!(arm_smccc_1_2_regs, a8));
    DEFINE!(ARM_SMCCC_1_2_REGS_X10_OFFS, offset_of!(arm_smccc_1_2_regs, a10));
    DEFINE!(ARM_SMCCC_1_2_REGS_X12_OFFS, offset_of!(arm_smccc_1_2_regs, a12));
    DEFINE!(ARM_SMCCC_1_2_REGS_X14_OFFS, offset_of!(arm_smccc_1_2_regs, a14));
    DEFINE!(ARM_SMCCC_1_2_REGS_X16_OFFS, offset_of!(arm_smccc_1_2_regs, a16));
    BLANK!();
    DEFINE!(HIBERN_PBE_ORIG, offset_of!(pbe, orig_address));
    DEFINE!(HIBERN_PBE_ADDR, offset_of!(pbe, address));
    DEFINE!(HIBERN_PBE_NEXT, offset_of!(pbe, next));
    DEFINE!(ARM64_FTR_SYSVAL, offset_of!(arm64_ftr_reg, sys_val));
    BLANK!();
#[cfg(CONFIG_UNMAP_KERNEL_AT_EL0)]
    DEFINE!(TRAMP_VALIAS, TRAMP_VALIAS);
#[cfg(CONFIG_ARM_SDE_INTERFACE)]
    DEFINE!(SDEI_EVENT_INTREGS, offset_of!(sdei_registered_event, interrupted_regs));
#[cfg(CONFIG_ARM_SDE_INTERFACE)]
    DEFINE!(SDEI_EVENT_PRIORITY, offset_of!(sdei_registered_event, priority));
#[cfg(CONFIG_ARM64_PTR_AUTH)]
    DEFINE!(PTRAUTH_USER_KEY_APIA, offset_of!(ptrauth_keys_user, apia));
#[cfg(all(CONFIG_ARM64_PTR_AUTH, CONFIG_ARM64_PTR_AUTH_KERNEL))]
    DEFINE!(PTRAUTH_KERNEL_KEY_APIA, offset_of!(ptrauth_keys_kernel, apia));
#[cfg(CONFIG_ARM64_PTR_AUTH)]
    BLANK!();
#[cfg(CONFIG_KEXEC_CORE)]
    DEFINE!(KIMAGE_ARCH_DTB_MEM, offset_of!(kimage, arch.dtb_mem));
#[cfg(CONFIG_KEXEC_CORE)]
    DEFINE!(KIMAGE_ARCH_EL2_VECTORS, offset_of!(kimage, arch.el2_vectors));
#[cfg(CONFIG_KEXEC_CORE)]
    DEFINE!(KIMAGE_ARCH_ZERO_PAGE, offset_of!(kimage, arch.zero_page));
#[cfg(CONFIG_KEXEC_CORE)]
    DEFINE!(KIMAGE_ARCH_PHYS_OFFSET, offset_of!(kimage, arch.phys_offset));
#[cfg(CONFIG_KEXEC_CORE)]
    DEFINE!(KIMAGE_ARCH_TTBR1, offset_of!(kimage, arch.ttbr1));
#[cfg(CONFIG_KEXEC_CORE)]
    DEFINE!(KIMAGE_HEAD, offset_of!(kimage, head));
#[cfg(CONFIG_KEXEC_CORE)]
    DEFINE!(KIMAGE_START, offset_of!(kimage, start));
#[cfg(CONFIG_KEXEC_CORE)]
    BLANK!();
#[cfg(CONFIG_FUNCTION_TRACER)]
    DEFINE!(FTRACE_OPS_FUNC, offset_of!(ftrace_ops, func));
    BLANK!();
#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS)]
    DEFINE!(FTRACE_OPS_DIRECT_CALL, offset_of!(ftrace_ops, direct_call));
    DEFINE!(PIE_E0_ASM, PIE_E0);
    DEFINE!(PIE_E1_ASM, PIE_E1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
