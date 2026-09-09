// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/alpha/kernel/process.c
 *
 *  Copyright (C) 1995  Linus Torvalds
 */

/* This file handles the architecture-dependent parts of process handling. */

// C header dependencies are supplied by the surrounding kernel translation.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe extern "C" {
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
    static machine_power_off: unsafe extern "C" fn();
    fn wtint(value: i32);
    fn BUG() -> !;
    fn smp_processor_id() -> i32;
    fn local_irq_disable();
    fn set_cpu_present(cpu: i32, present: bool);
    fn set_cpu_possible(cpu: i32, possible: bool);
    fn halt() -> !;
    fn cpumask_empty(mask: *const core::ffi::c_void) -> bool;
    fn barrier();
    fn irq_exit();
    fn in_hardirq() -> bool;
    fn console_lock();
    fn console_unlock();
    fn do_take_over_console(con: *mut core::ffi::c_void, first: i32, last: i32, deflt: i32);
    fn pci_restore_srm_config();
    fn set_hae(value: u64);
    fn srm_paging_stop();
    fn show_regs_print_info(s: *const u8);
    fn dik_show_regs(regs: *mut pt_regs, arg: *mut core::ffi::c_void);
    fn wrusp(value: usize);
    fn wrfpcr(value: u64);
    fn ieee_swcr_to_fpcr(value: i32) -> u64;
    fn current_thread_info() -> *mut thread_info;
    fn rdusp() -> usize;
    fn task_thread_info(task: *mut task_struct) -> *mut thread_info;
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn current_pt_regs() -> *mut pt_regs;
    fn task_stack_page(task: *mut task_struct) -> *mut core::ffi::c_void;
    fn in_sched_functions(pc: usize) -> bool;
    fn on_each_cpu(func: unsafe extern "C" fn(*mut core::ffi::c_void), data: *mut core::ffi::c_void, wait: i32);
    fn memset(dst: *mut core::ffi::c_void, value: i32, n: usize) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    static mut hwrpb: *mut hwrpb_struct;
    static mut boot_cpuid: i32;
    static mut cpu_present_mask: *const core::ffi::c_void;
    static mut alpha_using_srm: bool;
    static mut srm_hae: u64;
    static mut alpha_mv: alpha_machine_vector;
    static mut dummy_con: core::ffi::c_void;
}

pub const LINUX_REBOOT_CMD_RESTART: i32 = 0x01234567;
pub const LINUX_REBOOT_CMD_HALT: i32 = 0xCDEF0123;
pub const LINUX_REBOOT_CMD_POWER_OFF: i32 = 0x4321FEDC;
pub const CLONE_SETTLS: u64 = 0x00080000;
pub const FPCR_DYN_NORMAL: u64 = 0;
pub const TS_SAVED_FP: u64 = 1;
pub const TS_RESTORE_FP: u64 = 2;

#[repr(C)] pub struct halt_info { pub mode: i32, pub restart_cmd: *mut u8 }

unsafe extern "C" fn common_shutdown_1(generic_ptr: *mut core::ffi::c_void) {
    let how = generic_ptr as *mut halt_info;
    let cpuid = smp_processor_id();
    local_irq_disable();
    let cpup = ((hwrpb as usize) + (*hwrpb).processor_offset as usize
        + (*hwrpb).processor_size as usize * cpuid as usize) as *mut percpu_struct;
    let pflags = &mut (*cpup).flags;
    let mut flags = *pflags;
    flags &= !0x00ff0001usize;
    if cpuid != boot_cpuid {
        flags |= 0x00040000;
        *pflags = flags;
        set_cpu_present(cpuid, false);
        set_cpu_possible(cpuid, false);
        halt();
    }
    if (*how).mode == LINUX_REBOOT_CMD_RESTART {
        if (*how).restart_cmd.is_null() { flags |= 0x00020000; } else { flags |= 0x00030000; }
    } else { flags |= 0x00040000; }
    *pflags = flags;
    set_cpu_present(boot_cpuid, false);
    set_cpu_possible(boot_cpuid, false);
    while !cpumask_empty(cpu_present_mask) { barrier(); }
    if alpha_using_srm {
        if in_hardirq() { irq_exit(); }
        console_lock();
        do_take_over_console(&mut dummy_con, 0, 63, 1);
        console_unlock();
        pci_restore_srm_config();
        set_hae(srm_hae);
    }
    if let Some(kill) = alpha_mv.kill_arch { kill((*how).mode); }
    if !alpha_using_srm && (*how).mode != LINUX_REBOOT_CMD_RESTART { return; }
    if alpha_using_srm { srm_paging_stop(); }
    halt();
}

unsafe extern "C" fn common_shutdown(mode: i32, restart_cmd: *mut u8) {
    let mut args = halt_info { mode, restart_cmd };
    on_each_cpu(common_shutdown_1, &mut args as *mut _ as *mut _, 0);
}

pub unsafe extern "C" fn machine_restart(restart_cmd: *mut u8) { common_shutdown(LINUX_REBOOT_CMD_RESTART, restart_cmd); }
pub unsafe extern "C" fn machine_halt() { common_shutdown(LINUX_REBOOT_CMD_HALT, core::ptr::null_mut()); }
pub unsafe extern "C" fn machine_power_off() { common_shutdown(LINUX_REBOOT_CMD_POWER_OFF, core::ptr::null_mut()); }

pub unsafe extern "C" fn show_regs(regs: *mut pt_regs) {
    show_regs_print_info(b"\0".as_ptr());
    dik_show_regs(regs, core::ptr::null_mut());
}

pub unsafe extern "C" fn start_thread(regs: *mut pt_regs, pc: usize, sp: usize) {
    (*regs).pc = pc; (*regs).ps = 8; wrusp(sp);
}

pub unsafe extern "C" fn flush_thread() {
    (*current_thread_info()).ieee_state = 0;
    wrfpcr(FPCR_DYN_NORMAL | ieee_swcr_to_fpcr(0));
    (*current_thread_info()).pcb.unique = 0;
}

pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32 {
    let clone_flags = (*args).flags; let usp = (*args).stack; let tls = (*args).tls;
    unsafe extern "C" { fn ret_from_fork(); fn ret_from_kernel_thread(); }
    let childti = task_thread_info(p); let childregs = task_pt_regs(p); let regs = current_pt_regs();
    let childstack = (childregs as *mut switch_stack).offset(-1);
    (*childti).pcb.ksp = childstack as usize; (*childti).pcb.flags = 1;
    (*childti).status |= TS_SAVED_FP | TS_RESTORE_FP;
    if let Some(f) = (*args).fn_ { memset(childstack as *mut _, 0, core::mem::size_of::<switch_stack>() + core::mem::size_of::<pt_regs>()); (*childstack).r26 = ret_from_kernel_thread as usize; (*childstack).r9 = f as usize; (*childstack).r10 = (*args).fn_arg; (*childregs).hae = alpha_mv.hae_cache; memset((*childti).fp.as_mut_ptr() as *mut _, 0, core::mem::size_of_val(&(*childti).fp)); (*childti).pcb.usp = 0; return 0; }
    if clone_flags & CLONE_SETTLS != 0 { (*childti).pcb.unique = tls; } else { (*regs).r20 = 0; }
    (*childti).pcb.usp = if usp != 0 { usp } else { rdusp() }; *childregs = *regs; (*childregs).r0 = 0; (*childregs).r19 = 0; (*childregs).r20 = 1;
    let stack = (regs as *mut switch_stack).offset(-1); *childstack = *stack; (*childstack).r26 = ret_from_fork as usize; 0
}

pub unsafe extern "C" fn dump_elf_thread(dest: *mut usize, pt: *mut pt_regs, ti: *mut thread_info) {
    let sw = (pt as *mut switch_stack).offset(-1);
    (*dest.add(0)) = (*pt).r0; (*dest.add(1)) = (*pt).r1; (*dest.add(2)) = (*pt).r2; (*dest.add(3)) = (*pt).r3; (*dest.add(4)) = (*pt).r4; (*dest.add(5)) = (*pt).r5; (*dest.add(6)) = (*pt).r6; (*dest.add(7)) = (*pt).r7; (*dest.add(8)) = (*pt).r8; (*dest.add(9)) = (*sw).r9; (*dest.add(10)) = (*sw).r10; (*dest.add(11)) = (*sw).r11; (*dest.add(12)) = (*sw).r12; (*dest.add(13)) = (*sw).r13; (*dest.add(14)) = (*sw).r14; (*dest.add(15)) = (*sw).r15; (*dest.add(16)) = (*pt).r16; (*dest.add(17)) = (*pt).r17; (*dest.add(18)) = (*pt).r18; (*dest.add(19)) = (*pt).r19; (*dest.add(20)) = (*pt).r20; (*dest.add(21)) = (*pt).r21; (*dest.add(22)) = (*pt).r22; (*dest.add(23)) = (*pt).r23; (*dest.add(24)) = (*pt).r24; (*dest.add(25)) = (*pt).r25; (*dest.add(26)) = (*pt).r26; (*dest.add(27)) = (*pt).r27; (*dest.add(28)) = (*pt).r28; (*dest.add(29)) = (*pt).gp; (*dest.add(30)) = if ti == current_thread_info() { rdusp() } else { (*ti).pcb.usp }; (*dest.add(31)) = (*pt).pc; (*dest.add(32)) = (*ti).pcb.unique;
}
pub unsafe extern "C" fn dump_elf_task(dest: *mut usize, task: *mut task_struct) -> i32 { dump_elf_thread(dest, task_pt_regs(task), task_thread_info(task)); 1 }
pub unsafe extern "C" fn elf_core_copy_task_fpregs(t: *mut task_struct, fpu: *mut u64) -> i32 { memcpy(fpu as *mut _, (*task_thread_info(t)).fp.as_ptr() as *const _, 32 * 8); 1 }

unsafe fn thread_saved_pc(t: *mut task_struct) -> usize {
    let base = task_stack_page(t) as usize; let sp = (*task_thread_info(t)).pcb.ksp;
    if sp > base && sp + 6 * 8 < base + 16 * 1024 { let fp = *((sp as *const usize).add(6)); if fp > sp && fp < base + 16 * 1024 { return *(fp as *const usize); } }
    0
}
pub unsafe extern "C" fn __get_wchan(p: *mut task_struct) -> usize {
    let pc = thread_saved_pc(p);
    if in_sched_functions(pc) { let frame = *(((*task_thread_info(p)).pcb.ksp as *const usize).add(6)); return *((frame as *const usize).add(12)); }
    pc
}

// External kernel structures and declarations are provided by the translated dependencies.
#[allow(dead_code)]
extern "C" { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
