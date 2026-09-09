// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC process.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others. All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 *
 * This file handles the architecture-dependent parts of process handling...
 */

// C headers and build-time __KERNEL_SYSCALLS__ configuration are supplied by
// the surrounding kernel translation unit.

#[repr(C)]
pub struct thread_info;
#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct kernel_clone_args {
    pub flags: u64,
    pub stack: usize,
    pub tls: usize,
    pub fn_: *mut core::ffi::c_void,
    pub fn_arg: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct pt_regs {
    pub gpr: [usize; 32],
    pub pc: usize,
    pub sr: usize,
    pub sp: usize,
}

extern "C" {
    static mut current_thread_info_set: [*mut thread_info; NR_CPUS];
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
    static mut lwa_flag: i32;
    static init_thread_info: thread_info;

    fn do_kernel_restart(cmd: *mut i8);
    fn mdelay(ms: u32);
    fn printk(fmt: *const i8, ...);
    fn do_kernel_power_off();
    fn raw_local_irq_enable();
    fn raw_local_irq_disable();
    fn mfspr(reg: usize) -> usize;
    fn mtspr(reg: usize, value: usize);
    fn show_regs_print_info(fmt: *const i8);
    fn show_registers(regs: *mut pt_regs);
    fn current_pt_regs() -> *mut pt_regs;
    fn task_stack_page(p: *mut task_struct) -> *mut core::ffi::c_void;
    fn task_thread_info(p: *mut task_struct) -> *mut thread_info;
    fn memset(dst: *mut core::ffi::c_void, value: i32, n: usize) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn save_fpu(task: *mut task_struct);
    fn restore_fpu(task: *mut task_struct);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn smp_processor_id() -> usize;
    fn _switch(old_ti: *mut thread_info, new_ti: *mut thread_info) -> *mut thread_info;
    fn current() -> *mut task_struct;
}

extern "C" {
    fn ret_from_fork();
}

const NR_CPUS: usize = 1;
const THREAD_SIZE: usize = 0;
const STACK_FRAME_OVERHEAD: usize = 0;
const CLONE_SETTLS: u64 = 0;
const SPR_UPR: usize = 0;
const SPR_PMR: usize = 0;
const SPR_SR: usize = 0;
const SPR_UPR_PMP: usize = 0;
const SPR_PMR_DME: usize = 0;
const SPR_SR_SM: usize = 0;
const KERN_INFO: *const i8 = b"<6>\0".as_ptr() as *const i8;
const KERN_DEFAULT: *const i8 = b"\0".as_ptr() as *const i8;

#[no_mangle]
pub unsafe extern "C" fn machine_restart(cmd: *mut i8) {
    do_kernel_restart(cmd);
    core::arch::asm!("l.nop 13");
    mdelay(1000);
    printk(b"Reboot failed -- System halted\n\0".as_ptr() as *const i8);
    loop {}
}

unsafe fn default_power_off() {
    core::arch::asm!("l.nop 1");
}

#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    printk(b"*** MACHINE HALT ***\n\0".as_ptr() as *const i8);
    core::arch::asm!("l.nop 1");
}

#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    printk(b"*** MACHINE POWER OFF ***\n\0".as_ptr() as *const i8);
    do_kernel_power_off();
    default_power_off();
}

#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle() {
    raw_local_irq_enable();
    if mfspr(SPR_UPR) & SPR_UPR_PMP != 0 {
        mtspr(SPR_PMR, mfspr(SPR_PMR) | SPR_PMR_DME);
    }
    raw_local_irq_disable();
}

pub unsafe extern "C" fn flush_thread() {}

pub unsafe extern "C" fn show_regs(regs: *mut pt_regs) {
    show_regs_print_info(KERN_DEFAULT);
    show_registers(regs);
}

#[no_mangle]
pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32 {
    let clone_flags = (*args).flags;
    let usp = (*args).stack;
    let tls = (*args).tls;
    let mut sp = task_stack_page(p) as usize + THREAD_SIZE;
    let top_of_kernel_stack = sp;
    sp -= STACK_FRAME_OVERHEAD;
    sp -= core::mem::size_of::<pt_regs>();
    let userregs = sp as *mut pt_regs;
    sp -= STACK_FRAME_OVERHEAD;
    sp -= core::mem::size_of::<pt_regs>();
    let kregs = sp as *mut pt_regs;

    if !(*args).fn_.is_null() {
        memset(kregs as *mut core::ffi::c_void, 0, core::mem::size_of::<pt_regs>());
        (*kregs).gpr[20] = (*args).fn_ as usize;
        (*kregs).gpr[22] = (*args).fn_arg as usize;
    } else {
        *userregs = *current_pt_regs();
        if usp != 0 { (*userregs).sp = usp; }
        if clone_flags & CLONE_SETTLS != 0 { (*userregs).gpr[10] = tls; }
        (*userregs).gpr[11] = 0;
        (*kregs).gpr[20] = 0;
    }
    (*kregs).sp = top_of_kernel_stack;
    (*kregs).gpr[9] = ret_from_fork as usize;
    (*task_thread_info(p)).ksp = kregs as usize;
    0
}

pub unsafe extern "C" fn start_thread(regs: *mut pt_regs, pc: usize, sp: usize) {
    let sr = mfspr(SPR_SR) & !SPR_SR_SM;
    memset(regs as *mut core::ffi::c_void, 0, core::mem::size_of::<pt_regs>());
    (*regs).pc = pc;
    (*regs).sr = sr;
    (*regs).sp = sp;
}

pub unsafe extern "C" fn __switch_to(old: *mut task_struct, new: *mut task_struct) -> *mut task_struct {
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    save_fpu(current());
    let new_ti = *(new as *mut *mut thread_info);
    let old_ti = *(old as *mut *mut thread_info);
    lwa_flag = 0;
    current_thread_info_set[smp_processor_id()] = new_ti;
    let last = _switch(old_ti, new_ti);
    restore_fpu(current());
    local_irq_restore(flags);
    *(last as *mut *mut task_struct)
}

pub unsafe extern "C" fn dump_elf_thread(dest: *mut usize, regs: *mut pt_regs) {
    *dest.add(0) = 0;
    memcpy(dest.add(1) as *mut core::ffi::c_void, (*regs).gpr.as_ptr().add(1) as *const core::ffi::c_void, 31 * core::mem::size_of::<usize>());
    *dest.add(32) = (*regs).pc;
    *dest.add(33) = (*regs).sr;
    *dest.add(34) = 0;
    *dest.add(35) = 0;
}

pub unsafe extern "C" fn __get_wchan(_p: *mut task_struct) -> usize {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
