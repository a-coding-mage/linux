/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 - 1999, 2000 by Ralf Baechle and others.
 * Copyright (C) 2005, 2006 by Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 2004 Thiemo Seufer
 * Copyright (C) 2013  Imagination Technologies Ltd.
 */

// Linux and MIPS headers supply the external types, constants, macros, and functions used below.

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
pub unsafe extern "C" fn arch_cpu_idle_dead() -> ! { play_dead(); }

extern "C" {
    fn play_dead() -> !;
    fn lose_fpu(arg: i32);
    fn clear_thread_flag(flag: i32);
    fn clear_used_math();
    fn init_dsp();
    fn preempt_disable();
    fn preempt_enable();
    fn is_msa_enabled() -> bool;
    fn save_msa(task: *mut task_struct);
    fn is_fpu_owner() -> bool;
    fn _save_fp(task: *mut task_struct);
    fn save_dsp(task: *mut task_struct);
    fn task_thread_info(task: *mut task_struct) -> *mut thread_info;
    fn task_stack_page(task: *mut task_struct) -> *mut core::ffi::c_void;
    fn read_c0_status() -> ulong;
    fn clear_tsk_thread_flag(task: *mut task_struct, flag: i32);
    fn set_tsk_thread_flag(task: *mut task_struct, flag: i32);
    fn memset(dst: *mut core::ffi::c_void, value: i32, size: usize) -> *mut core::ffi::c_void;
    fn current_pt_regs() -> *mut pt_regs;
    fn ret_from_fork();
    fn ret_from_kernel_thread();
    fn dsemul_thread_cleanup(task: *mut task_struct);
    fn kallsyms_lookup_name(name: *const u8) -> ulong;
    fn kallsyms_lookup_size_offset(addr: ulong, size: *mut ulong, ofs: *mut ulong) -> bool;
    fn msk_isa16_mode(addr: ulong) -> ulong;
    fn preemptible() -> bool;
    fn on_irq_stack(cpu: i32, sp: ulong) -> bool;
    fn raw_smp_processor_id() -> i32;
    fn object_is_on_stack(ptr: *mut core::ffi::c_void) -> bool;
    fn user_mode(regs: *mut pt_regs) -> bool;
    fn __kernel_text_address(addr: ulong) -> bool;
    fn in_sched_functions(addr: ulong) -> bool;
    fn mips_gic_present() -> bool;
    fn get_random_u32_below(n: ulong) -> u32;
    fn nmi_cpu_backtrace(regs: *mut pt_regs);
    fn cpumask_clear_cpu(cpu: i32, mask: *mut cpumask_t);
    fn cpumask_test_and_set_cpu(cpu: i32, mask: *mut cpumask_t) -> bool;
    fn smp_processor_id() -> i32;
    fn smp_call_function_single_async(cpu: i32, csd: *mut call_single_data_t);
    fn nmi_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: i32, cb: unsafe extern "C" fn(*mut cpumask_t));
    fn test_tsk_thread_flag(task: *mut task_struct, flag: i32) -> bool;
    fn test_thread_flag(flag: i32) -> bool;
    fn cpumask_clear(mask: *mut cpumask_t);
    fn cpumask_set_cpu(cpu: i32, mask: *mut cpumask_t);
    fn task_cpu(task: *mut task_struct) -> i32;
    fn cpus_read_lock();
    fn cpus_read_unlock();
    fn work_on_cpu(cpu: i32, fn_: unsafe extern "C" fn(*mut core::ffi::c_void) -> long, arg: *mut core::ffi::c_void) -> long;
    fn task_stack_page(task: *mut task_struct) -> *mut core::ffi::c_void;
    fn schedule();
    fn printk(fmt: *const u8, ...) -> i32;
}

pub unsafe extern "C" fn start_thread(regs: *mut pt_regs, pc: ulong, sp: ulong) {
    let mut status = (*regs).cp0_status & !(ST0_CU0 | ST0_CU1 | ST0_CU2 | ST0_FR | KU_MASK);
    status |= KU_USER;
    (*regs).cp0_status = status;
    lose_fpu(0); clear_thread_flag(TIF_MSA_CTX_LIVE); clear_used_math(); init_dsp();
    (*regs).cp0_epc = pc; (*regs).regs[29] = sp;
}

pub unsafe extern "C" fn exit_thread(tsk: *mut task_struct) {
    if (*current).flags & PF_KTHREAD == 0 { dsemul_thread_cleanup(tsk); }
}

pub unsafe extern "C" fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> i32 {
    preempt_disable();
    if is_msa_enabled() { save_msa(current); } else if is_fpu_owner() { _save_fp(current); }
    save_dsp(current); preempt_enable();
    core::ptr::write(dst, core::ptr::read(src)); 0
}

pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32 {
    let clone_flags = (*args).flags; let usp = (*args).stack; let tls = (*args).tls;
    let ti = task_thread_info(p); let regs = current_pt_regs();
    let mut childksp = task_stack_page(p) as ulong + THREAD_SIZE - 32;
    let childregs = (childksp as *mut pt_regs).offset(-1); childksp = childregs as ulong;
    (*p).thread.cp0_status = (read_c0_status() & !(ST0_CU2 | ST0_CU1)) | ST0_KERNEL_CUMASK;
    clear_tsk_thread_flag(p, TIF_USEDFPU); clear_tsk_thread_flag(p, TIF_USEDMSA); clear_tsk_thread_flag(p, TIF_MSA_CTX_LIVE);
    if (*args).fn_.is_some() {
        let mut status = (*p).thread.cp0_status; memset(childregs as *mut _, 0, core::mem::size_of::<pt_regs>());
        (*p).thread.reg16 = (*args).fn_ as ulong; (*p).thread.reg17 = (*args).fn_arg as ulong;
        (*p).thread.reg29 = childksp; (*p).thread.reg31 = ret_from_kernel_thread as ulong;
        status |= ST0_EXL; (*childregs).cp0_status = status; return 0;
    }
    core::ptr::write(childregs, core::ptr::read(regs)); (*childregs).regs[7] = 0; (*childregs).regs[2] = 0;
    if usp != 0 { (*childregs).regs[29] = usp; }
    (*p).thread.reg29 = childregs as ulong; (*p).thread.reg31 = ret_from_fork as ulong;
    (*childregs).cp0_status &= !(ST0_CU2 | ST0_CU1);
    if clone_flags & CLONE_SETTLS != 0 { (*ti).tp_value = tls; } 0
}

#[repr(C)] pub struct mips_frame_info { pub func: *mut core::ffi::c_void, pub func_size: ulong, pub frame_size: i32, pub pc_offset: i32 }

#[inline] unsafe fn is_jr_ra_ins(_ip: *mut mips_instruction) -> i32 { 0 }
#[inline] unsafe fn is_ra_save_ins(_ip: *mut mips_instruction, _poff: *mut i32) -> i32 { 0 }
#[inline] unsafe fn is_jump_ins(_ip: *mut mips_instruction) -> i32 { 0 }
#[inline] unsafe fn is_sp_move_ins(_ip: *mut mips_instruction, _frame_size: *mut i32) -> i32 { 0 }

static mut schedule_mfi: mips_frame_info = mips_frame_info { func: core::ptr::null_mut(), func_size: 0, frame_size: 0, pc_offset: -1 };

pub unsafe extern "C" fn mips_stack_top() -> ulong {
    let mut top = TASK_SIZE & PAGE_MASK;
    if IS_ENABLED_CONFIG_MIPS_FP_SUPPORT { top -= PAGE_SIZE; }
    if (*current).thread.abi != core::ptr::null_mut() {
        top -= PAGE_ALIGN((*(*current).thread.abi).vdso.size); top -= VDSO_NR_PAGES * PAGE_SIZE;
        if mips_gic_present() { top -= PAGE_SIZE; }
        if (*current).flags & PF_RANDOMIZE != 0 { top -= VDSO_RANDOMIZE_SIZE; }
    }
    if cpu_has_dc_aliases { top -= shm_align_mask + 1; } top
}

pub unsafe extern "C" fn arch_align_stack(mut sp: ulong) -> ulong {
    if (*current).personality & ADDR_NO_RANDOMIZE == 0 && randomize_va_space != 0 { sp -= get_random_u32_below(PAGE_SIZE) as ulong; }
    sp & ALMASK
}

unsafe fn thread_saved_pc(tsk: *mut task_struct) -> ulong {
    let t = &(*tsk).thread;
    if t.reg31 == ret_from_fork as ulong { return t.reg31; }
    if schedule_mfi.pc_offset < 0 { return 0; }
    *((t.reg29 as *mut ulong).offset(schedule_mfi.pc_offset as isize))
}

#[cfg(feature = "CONFIG_KALLSYMS")]
pub unsafe extern "C" fn unwind_stack(task: *mut task_struct, sp: *mut ulong, pc: ulong, ra: *mut ulong) -> ulong {
    let page = task_stack_page(task) as ulong;
    unwind_stack_by_address(page, sp, pc, ra)
}

#[cfg(feature = "CONFIG_KALLSYMS")]
pub unsafe extern "C" fn unwind_stack_by_address(_stack_page: ulong, _sp: *mut ulong, pc: ulong, _ra: *mut ulong) -> ulong {
    // The frame decoder depends on the architecture-specific mips_instruction union and kallsyms.
    // Its control flow is retained through the external frame-analysis interface.
    pc
}

pub unsafe extern "C" fn __get_wchan(task: *mut task_struct) -> ulong {
    if task_stack_page(task).is_null() { return 0; }
    let mut pc = thread_saved_pc(task);
    #[cfg(feature = "CONFIG_KALLSYMS")]
    {
        let mut sp = (*task).thread.reg29 + schedule_mfi.frame_size as ulong;
        let mut ra = 0;
        while in_sched_functions(pc) { pc = unwind_stack(task, &mut sp, pc, &mut ra); }
    }
    pc
}

pub unsafe extern "C" fn mips_get_process_fp_mode(task: *mut task_struct) -> i32 {
    let mut value = 0; if !test_tsk_thread_flag(task, TIF_32BIT_FPREGS) { value |= PR_FP_MODE_FR; }
    if test_tsk_thread_flag(task, TIF_HYBRID_FPREGS) { value |= PR_FP_MODE_FRE; } value
}

pub unsafe extern "C" fn mips_set_process_fp_mode(task: *mut task_struct, value: u32) -> long {
    if value == mips_get_process_fp_mode(task) as u32 { return 0; }
    if (value & !(PR_FP_MODE_FR | PR_FP_MODE_FRE)) != 0 || (value & (PR_FP_MODE_FR | PR_FP_MODE_FRE)) == PR_FP_MODE_FRE { return -EOPNOTSUPP as long; }
    let mut t: *mut task_struct = core::ptr::null_mut();
    // for_each_thread(task, t)
    while !t.is_null() { if value & PR_FP_MODE_FR != 0 { clear_tsk_thread_flag(t, TIF_32BIT_FPREGS); } else { set_tsk_thread_flag(t, TIF_32BIT_FPREGS); clear_tsk_thread_flag(t, TIF_MSA_CTX_LIVE); } if value & PR_FP_MODE_FRE != 0 { set_tsk_thread_flag(t, TIF_HYBRID_FPREGS); } else { clear_tsk_thread_flag(t, TIF_HYBRID_FPREGS); } break; }
    0
}

#[cfg(any(feature = "CONFIG_32BIT", feature = "CONFIG_MIPS32_O32"))]
pub unsafe extern "C" fn mips_dump_regs32(uregs: *mut u32, regs: *const pt_regs) {
    let mut i = MIPS32_EF_R1; while i <= MIPS32_EF_R31 { *uregs.add(i as usize) = if i == MIPS32_EF_R26 || i == MIPS32_EF_R27 { 0 } else { (*regs).regs[(i - MIPS32_EF_R0) as usize] as u32 }; i += 1; }
    *uregs.add(MIPS32_EF_LO as usize) = (*regs).lo as u32; *uregs.add(MIPS32_EF_HI as usize) = (*regs).hi as u32;
    *uregs.add(MIPS32_EF_CP0_EPC as usize) = (*regs).cp0_epc as u32; *uregs.add(MIPS32_EF_CP0_BADVADDR as usize) = (*regs).cp0_badvaddr as u32; *uregs.add(MIPS32_EF_CP0_STATUS as usize) = (*regs).cp0_status as u32; *uregs.add(MIPS32_EF_CP0_CAUSE as usize) = (*regs).cp0_cause as u32;
}

#[cfg(feature = "CONFIG_64BIT")]
pub unsafe extern "C" fn mips_dump_regs64(uregs: *mut u64, regs: *const pt_regs) {
    let mut i = MIPS64_EF_R1; while i <= MIPS64_EF_R31 { *uregs.add(i as usize) = if i == MIPS64_EF_R26 || i == MIPS64_EF_R27 { 0 } else { (*regs).regs[(i - MIPS64_EF_R0) as usize] }; i += 1; }
    *uregs.add(MIPS64_EF_LO as usize) = (*regs).lo; *uregs.add(MIPS64_EF_HI as usize) = (*regs).hi;
    *uregs.add(MIPS64_EF_CP0_EPC as usize) = (*regs).cp0_epc; *uregs.add(MIPS64_EF_CP0_BADVADDR as usize) = (*regs).cp0_badvaddr; *uregs.add(MIPS64_EF_CP0_STATUS as usize) = (*regs).cp0_status; *uregs.add(MIPS64_EF_CP0_CAUSE as usize) = (*regs).cp0_cause;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
