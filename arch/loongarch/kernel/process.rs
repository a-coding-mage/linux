// SPDX-License-Identifier: GPL-2.0
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 *
 * Derived from MIPS:
 * Copyright (C) 1994 - 1999, 2000 by Ralf Baechle and others.
 * Copyright (C) 2005, 2006 by Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 2004 Thiemo Seufer
 * Copyright (C) 2013  Imagination Technologies Ltd.
 */

// Dependencies are supplied by the surrounding kernel translation.

#[cfg(CONFIG_STACKPROTECTOR)]
#[no_mangle]
pub static mut __stack_chk_guard: c_ulong = 0;

#[cfg(CONFIG_STACKPROTECTOR)]
extern "C" {
    fn EXPORT_SYMBOL(symbol: *const c_void);
}

// DEFINE_PER_CPU(struct task_struct *, cpu_tasks);

#[no_mangle]
pub static mut boot_option_idle_override: c_ulong = IDLE_NO_OVERRIDE;

extern "C" {
    fn restore_and_ret();
    fn ret_from_fork_asm();
    fn ret_from_kernel_thread_asm();
}

#[no_mangle]
pub unsafe extern "C" fn start_thread(regs: *mut pt_regs, pc: c_ulong, sp: c_ulong) {
    let mut crmd: c_ulong;
    let mut prmd: c_ulong;
    let mut euen: c_ulong;

    crmd = (*regs).csr_crmd & !(PLV_MASK);
    crmd |= PLV_USER;
    (*regs).csr_crmd = crmd;

    prmd = (*regs).csr_prmd & !(PLV_MASK);
    prmd |= PLV_USER;
    (*regs).csr_prmd = prmd;

    euen = (*regs).csr_euen & !(CSR_EUEN_FPEN);
    (*regs).csr_euen = euen;
    lose_fpu(0);
    lose_lbt(0);
    (*current).thread.fpu.fcsr = boot_cpu_data.fpu_csr0;

    clear_thread_flag(TIF_LSX_CTX_LIVE);
    clear_thread_flag(TIF_LASX_CTX_LIVE);
    clear_thread_flag(TIF_LBT_CTX_LIVE);
    clear_used_math();
    (*regs).csr_era = pc;
    (*regs).regs[3] = sp;
}

#[no_mangle]
pub unsafe extern "C" fn flush_thread() {
    flush_ptrace_hw_breakpoint(current);
}

#[no_mangle]
pub unsafe extern "C" fn exit_thread(_tsk: *mut task_struct) {}

#[no_mangle]
pub unsafe extern "C" fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> c_int {
    preempt_disable();

    if is_fpu_owner() {
        if is_lasx_enabled() {
            save_lasx(current);
        } else if is_lsx_enabled() {
            save_lsx(current);
        } else {
            save_fp(current);
        }
    }
    preempt_enable();

    if IS_ENABLED(CONFIG_RANDSTRUCT) {
        memcpy(dst as *mut c_void, src as *const c_void, size_of::<task_struct>());
        return 0;
    }

    (*dst).thread.fpu.fcsr = (*src).thread.fpu.fcsr;
    if !used_math() {
        memcpy(dst as *mut c_void, src as *const c_void, offset_of!(task_struct, thread.fpu.fpr));
    } else {
        memcpy(dst as *mut c_void, src as *const c_void, offset_of!(task_struct, thread.lbt.scr0));
    }

    #[cfg(CONFIG_CPU_HAS_LBT)]
    memcpy(&mut (*dst).thread.lbt as *mut _ as *mut c_void, &(*src).thread.lbt as *const _ as *const c_void, size_of::<loongarch_lbt>());

    0
}

#[no_mangle]
pub unsafe extern "C" fn ret_from_fork(prev: *mut task_struct, regs: *mut pt_regs) {
    schedule_tail(prev);
    syscall_exit_to_user_mode(regs);
}

#[no_mangle]
pub unsafe extern "C" fn ret_from_kernel_thread(prev: *mut task_struct, regs: *mut pt_regs, fn_: Option<unsafe extern "C" fn(*mut c_void) -> c_int>, fn_arg: *mut c_void) {
    schedule_tail(prev);
    if let Some(f) = fn_ { f(fn_arg); }
    syscall_exit_to_user_mode(regs);
}

#[no_mangle]
pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> c_int {
    let mut childksp: c_ulong;
    let tls: c_ulong = (*args).tls;
    let usp: c_ulong = (*args).stack;
    let clone_flags: u64 = (*args).flags;
    let childregs: *mut pt_regs;
    let regs: *mut pt_regs = current_pt_regs();

    childksp = task_stack_page(p) as c_ulong + THREAD_SIZE;
    childregs = (childksp as *mut pt_regs).sub(1);
    childksp = childregs as c_ulong;
    (*p).thread.sched_cfa = 0;
    (*p).thread.csr_euen = 0;
    (*p).thread.csr_crmd = csr_read32(LOONGARCH_CSR_CRMD);
    (*p).thread.csr_prmd = csr_read32(LOONGARCH_CSR_PRMD);
    (*p).thread.csr_ecfg = csr_read32(LOONGARCH_CSR_ECFG);
    if !(*args).fn_.is_null() {
        (*p).thread.reg03 = childksp;
        (*p).thread.reg23 = (*args).fn_ as c_ulong;
        (*p).thread.reg24 = (*args).fn_arg as c_ulong;
        (*p).thread.reg01 = ret_from_kernel_thread_asm as c_ulong;
        (*p).thread.sched_ra = ret_from_kernel_thread_asm as c_ulong;
        core::ptr::write_bytes(childregs, 0, 1);
        (*childregs).csr_euen = (*p).thread.csr_euen;
        (*childregs).csr_crmd = (*p).thread.csr_crmd;
        (*childregs).csr_prmd = (*p).thread.csr_prmd;
        (*childregs).csr_ecfg = (*p).thread.csr_ecfg;
    } else {
        *childregs = *regs;
        (*childregs).regs[4] = 0;
        if usp != 0 { (*childregs).regs[3] = usp; }
        (*p).thread.reg03 = childregs as c_ulong;
        (*p).thread.reg01 = ret_from_fork_asm as c_ulong;
        (*p).thread.sched_ra = ret_from_fork_asm as c_ulong;
        (*childregs).csr_euen = 0;
        if clone_flags & CLONE_SETTLS != 0 { (*childregs).regs[2] = tls; }
    }

    ptrace_hw_copy_thread(p);
    clear_tsk_thread_flag(p, TIF_USEDFPU);
    clear_tsk_thread_flag(p, TIF_USEDSIMD);
    clear_tsk_thread_flag(p, TIF_USEDLBT);
    clear_tsk_thread_flag(p, TIF_LSX_CTX_LIVE);
    clear_tsk_thread_flag(p, TIF_LASX_CTX_LIVE);
    clear_tsk_thread_flag(p, TIF_LBT_CTX_LIVE);
    0
}

#[no_mangle]
pub unsafe extern "C" fn __get_wchan(task: *mut task_struct) -> c_ulong {
    let mut pc: c_ulong = 0;
    let mut state: unwind_state = core::mem::zeroed();
    if !try_get_task_stack(task) { return 0; }
    unwind_start(&mut state, task, core::ptr::null_mut());
    while !unwind_done(&mut state) {
        pc = unwind_get_return_address(&mut state);
        if pc == 0 { break; }
        if in_sched_functions(pc) { unwind_next_frame(&mut state); continue; }
        break;
    }
    put_task_stack(task);
    pc
}

#[no_mangle]
pub unsafe extern "C" fn in_irq_stack(stack: c_ulong, info: *mut stack_info) -> bool {
    let begin = this_cpu_read(irq_stack) as c_ulong;
    let end = begin + IRQ_STACK_START;
    if stack < begin || stack >= end { return false; }
    let nextsp = *(end as *const c_ulong);
    if nextsp & (SZREG - 1) != 0 { return false; }
    (*info).begin = begin; (*info).end = end; (*info).next_sp = nextsp; (*info).type_ = STACK_TYPE_IRQ;
    true
}

#[no_mangle]
pub unsafe extern "C" fn in_task_stack(stack: c_ulong, task: *mut task_struct, info: *mut stack_info) -> bool {
    let begin = task_stack_page(task) as c_ulong;
    let end = begin + THREAD_SIZE;
    if stack < begin || stack >= end { return false; }
    (*info).begin = begin; (*info).end = end; (*info).next_sp = 0; (*info).type_ = STACK_TYPE_TASK;
    true
}

#[no_mangle]
pub unsafe extern "C" fn get_stack_info(mut stack: c_ulong, mut task: *mut task_struct, info: *mut stack_info) -> c_int {
    if task.is_null() { task = current; }
    if stack == 0 || stack & (SZREG - 1) != 0 { (*info).type_ = STACK_TYPE_UNKNOWN; return -EINVAL; }
    if in_task_stack(stack, task, info) { return 0; }
    if task != current { (*info).type_ = STACK_TYPE_UNKNOWN; return -EINVAL; }
    if in_irq_stack(stack, info) { return 0; }
    (*info).type_ = STACK_TYPE_UNKNOWN;
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn stack_top() -> c_ulong {
    let mut top = TASK_SIZE & PAGE_MASK;
    if !(*current).thread.vdso.is_null() {
        top -= PAGE_ALIGN((*(*current).thread.vdso).size);
        top -= VVAR_SIZE;
        if (*current).flags & PF_RANDOMIZE != 0 { top -= VDSO_RANDOMIZE_SIZE; }
    }
    top
}

#[no_mangle]
pub unsafe extern "C" fn arch_align_stack(mut sp: c_ulong) -> c_ulong {
    if (*current).personality & ADDR_NO_RANDOMIZE == 0 && randomize_va_space != 0 {
        sp -= get_random_u32_below(PAGE_SIZE);
    }
    sp & STACK_ALIGN
}

// static DEFINE_PER_CPU(call_single_data_t, backtrace_csd);
static mut backtrace_csd_busy: cpumask = unsafe { core::mem::zeroed() };

unsafe extern "C" fn handle_backtrace(_info: *mut c_void) {
    nmi_cpu_backtrace(get_irq_regs());
    cpumask_clear_cpu(smp_processor_id(), &mut backtrace_csd_busy);
}

unsafe extern "C" fn raise_backtrace(mask: *mut cpumask) {
    let mut cpu: c_int = 0;
    for_each_cpu!(cpu, mask) {
        if cpumask_test_and_set_cpu(cpu, &mut backtrace_csd_busy) {
            pr_warn!("Unable to send backtrace IPI to CPU%u - perhaps it hung?\n", cpu);
            continue;
        }
        let csd = &mut per_cpu!(backtrace_csd, cpu);
        csd.func = Some(handle_backtrace);
        smp_call_function_single_async(cpu, csd);
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_trigger_cpumask_backtrace(mask: *const cpumask, exclude_cpu: c_int) {
    nmi_trigger_cpumask_backtrace(mask, exclude_cpu, Some(raise_backtrace));
}

#[cfg(CONFIG_32BIT)]
pub unsafe extern "C" fn loongarch_dump_regs32(uregs: *mut u32, regs: *const pt_regs) {
    for i in LOONGARCH_EF_R1..=LOONGARCH_EF_R31 { *uregs.add(i as usize) = (*regs).regs[(i - LOONGARCH_EF_R0) as usize] as u32; }
    *uregs.add(LOONGARCH_EF_ORIG_A0 as usize) = (*regs).orig_a0 as u32;
    *uregs.add(LOONGARCH_EF_CSR_ERA as usize) = (*regs).csr_era as u32;
    *uregs.add(LOONGARCH_EF_CSR_BADV as usize) = (*regs).csr_badvaddr as u32;
    *uregs.add(LOONGARCH_EF_CSR_CRMD as usize) = (*regs).csr_crmd as u32;
    *uregs.add(LOONGARCH_EF_CSR_PRMD as usize) = (*regs).csr_prmd as u32;
    *uregs.add(LOONGARCH_EF_CSR_EUEN as usize) = (*regs).csr_euen as u32;
    *uregs.add(LOONGARCH_EF_CSR_ECFG as usize) = (*regs).csr_ecfg as u32;
    *uregs.add(LOONGARCH_EF_CSR_ESTAT as usize) = (*regs).csr_estat as u32;
}

#[cfg(not(CONFIG_32BIT))]
pub unsafe extern "C" fn loongarch_dump_regs64(uregs: *mut u64, regs: *const pt_regs) {
    for i in LOONGARCH_EF_R1..=LOONGARCH_EF_R31 { *uregs.add(i as usize) = (*regs).regs[(i - LOONGARCH_EF_R0) as usize]; }
    *uregs.add(LOONGARCH_EF_ORIG_A0 as usize) = (*regs).orig_a0;
    *uregs.add(LOONGARCH_EF_CSR_ERA as usize) = (*regs).csr_era;
    *uregs.add(LOONGARCH_EF_CSR_BADV as usize) = (*regs).csr_badvaddr;
    *uregs.add(LOONGARCH_EF_CSR_CRMD as usize) = (*regs).csr_crmd;
    *uregs.add(LOONGARCH_EF_CSR_PRMD as usize) = (*regs).csr_prmd;
    *uregs.add(LOONGARCH_EF_CSR_EUEN as usize) = (*regs).csr_euen;
    *uregs.add(LOONGARCH_EF_CSR_ECFG as usize) = (*regs).csr_ecfg;
    *uregs.add(LOONGARCH_EF_CSR_ESTAT as usize) = (*regs).csr_estat;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
