/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Linux and architecture headers from the original translation are supplied
// by the surrounding kernel environment.

pub unsafe fn show_regs(regs: *mut pt_regs) {
    show_regs_print_info!(KERN_INFO);

    pr_info!(" Registers dump: mode={:X}\r\n", (*regs).pt_mode);
    pr_info!(" r1={:08lX}, r2={:08lX}, r3={:08lX}, r4={:08lX}\n", (*regs).r1, (*regs).r2, (*regs).r3, (*regs).r4);
    pr_info!(" r5={:08lX}, r6={:08lX}, r7={:08lX}, r8={:08lX}\n", (*regs).r5, (*regs).r6, (*regs).r7, (*regs).r8);
    pr_info!(" r9={:08lX}, r10={:08lX}, r11={:08lX}, r12={:08lX}\n", (*regs).r9, (*regs).r10, (*regs).r11, (*regs).r12);
    pr_info!(" r13={:08lX}, r14={:08lX}, r15={:08lX}, r16={:08lX}\n", (*regs).r13, (*regs).r14, (*regs).r15, (*regs).r16);
    pr_info!(" r17={:08lX}, r18={:08lX}, r19={:08lX}, r20={:08lX}\n", (*regs).r17, (*regs).r18, (*regs).r19, (*regs).r20);
    pr_info!(" r21={:08lX}, r22={:08lX}, r23={:08lX}, r24={:08lX}\n", (*regs).r21, (*regs).r22, (*regs).r23, (*regs).r24);
    pr_info!(" r25={:08lX}, r26={:08lX}, r27={:08lX}, r28={:08lX}\n", (*regs).r25, (*regs).r26, (*regs).r27, (*regs).r28);
    pr_info!(" r29={:08lX}, r30={:08lX}, r31={:08lX}, rPC={:08lX}\n", (*regs).r29, (*regs).r30, (*regs).r31, (*regs).pc);
    pr_info!(" msr={:08lX}, ear={:08lX}, esr={:08lX}, fsr={:08lX}\n", (*regs).msr, (*regs).ear, (*regs).esr, (*regs).fsr);
}

pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

pub fn flush_thread() {}

pub unsafe fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32 {
    let clone_flags: u64 = (*args).flags;
    let usp: usize = (*args).stack;
    let tls: usize = (*args).tls;
    let childregs: *mut pt_regs = task_pt_regs(p);
    let ti: *mut thread_info = task_thread_info(p);

    if (*args).fn_.is_some() {
        // If creating a new kernel thread, zero all registers.
        core::ptr::write_bytes(childregs, 0, 1);
        core::ptr::write_bytes(core::ptr::addr_of_mut!((*ti).cpu_context), 0, 1);
        (*ti).cpu_context.r1 = childregs as usize;
        (*ti).cpu_context.r20 = (*args).fn_.unwrap() as usize;
        (*ti).cpu_context.r19 = (*args).fn_arg as usize;
        (*childregs).pt_mode = 1;
        local_save_flags(core::ptr::addr_of_mut!((*childregs).msr));
        (*ti).cpu_context.msr = (*childregs).msr & !MSR_IE;
        (*ti).cpu_context.r15 = ret_from_kernel_thread as usize - 8;
        return 0;
    }
    *childregs = *current_pt_regs();
    if usp != 0 { (*childregs).r1 = usp; }

    core::ptr::write_bytes(core::ptr::addr_of_mut!((*ti).cpu_context), 0, 1);
    (*ti).cpu_context.r1 = childregs as usize;
    (*childregs).msr |= MSR_UMS;
    (*childregs).msr &= !MSR_EIP;
    (*childregs).msr |= MSR_IE;
    (*childregs).msr &= !MSR_VM;
    (*childregs).msr |= MSR_VMS;
    (*childregs).msr |= MSR_EE;
    (*ti).cpu_context.msr = (*childregs).msr | MSR_VM;
    (*ti).cpu_context.msr &= !MSR_UMS;
    (*ti).cpu_context.msr &= !MSR_IE;
    (*ti).cpu_context.r15 = ret_from_fork as usize - 8;
    if clone_flags & CLONE_SETTLS != 0 { (*childregs).r21 = tls; }
    0
}

pub unsafe fn __get_wchan(_p: *mut task_struct) -> usize { 0 }

pub unsafe fn start_thread(regs: *mut pt_regs, pc: usize, usp: usize) {
    (*regs).pc = pc;
    (*regs).r1 = usp;
    (*regs).pt_mode = 0;
    (*regs).msr |= MSR_UMS;
    (*regs).msr &= !MSR_VM;
}

pub unsafe fn elf_core_copy_task_fpregs(_t: *mut task_struct, _fpu: *mut elf_fpregset_t) -> i32 { 0 }

pub fn arch_cpu_idle() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
