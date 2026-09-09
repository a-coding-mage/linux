// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/m68k/kernel/process.c
 *
 *  Copyright (C) 1995  Hamish Macdonald
 *
 *  68060 fixes by Jesper Skov
 */

// Architecture-dependent process handling.
// C headers and build-provided symbols are intentionally supplied externally.

extern "C" {
    fn ret_from_fork();
    fn ret_from_kernel_thread();
    static mut mach_reset: Option<unsafe extern "C" fn()>;
    static mut mach_halt: Option<unsafe extern "C" fn()>;
    fn do_kernel_power_off();
    fn print_tainted() -> *const core::ffi::c_char;
    fn rdusp() -> usize;
    fn kernel_clone(args: *const kernel_clone_args) -> i32;
    fn sys_clone3(args: *const clone_args, size: usize) -> i32;
    fn task_stack_page(p: *mut task_struct) -> *mut core::ffi::c_void;
    fn current_pt_regs() -> *mut pt_regs;
    fn task_thread_info(p: *mut task_struct) -> *mut thread_info;
    fn in_sched_functions(pc: usize) -> bool;
}

pub const PS_S: u16 = 0x2000;
pub const USER_DATA: u32 = 0;
pub const CSIGNAL: u64 = 0x000000ff;
pub const CLONE_SETTLS: u64 = 0x00080000;

#[repr(C)]
pub struct pt_regs { pub format: u16, pub vector: u16, pub pc: usize, pub sr: u16, pub orig_d0: usize, pub d0: usize, pub a0: usize, pub a1: usize, pub a2: usize, pub d1: usize, pub d2: usize, pub d3: usize, pub d4: usize, pub d5: usize }
#[repr(C)] pub struct switch_stack { pub a6: usize, pub a3: usize, pub d7: usize, pub retpc: usize }
#[repr(C)] pub struct thread_info { pub tp_value: usize }
#[repr(C)] pub struct thread_struct { pub fc: u32, pub ksp: usize, pub esp0: usize, pub usp: usize, pub fpstate: [u8; 216], pub fp: [u8; 96], pub fpcntl: [u8; 12] }
#[repr(C)] pub struct task_struct { pub thread: thread_struct }
#[repr(C)] pub struct kernel_clone_args { pub flags: u64, pub pidfd: *mut i32, pub child_tid: *mut i32, pub parent_tid: *mut i32, pub exit_signal: u64, pub stack: usize, pub tls: usize, pub fn_: usize, pub fn_arg: usize }
#[repr(C)] pub struct clone_args { _private: [u8; 0] }
#[repr(C)] pub struct elf_fpregset_t { pub fpcntl: [u8; 12], pub fpregs: [u32; 24] }
pub const THREAD_SIZE: usize = 8192;

pub unsafe extern "C" fn arch_cpu_idle() {
    // C preprocessor configuration: MACH_ATARI_ONLY selects 0x2200.
    core::arch::asm!("stop #0x2000", options(nostack, preserves_flags));
}

pub unsafe extern "C" fn machine_restart(_unused: *mut core::ffi::c_char) -> ! {
    if let Some(f) = mach_reset { f(); }
    loop {}
}

pub unsafe extern "C" fn machine_halt() -> ! {
    if let Some(f) = mach_halt { f(); }
    loop {}
}

pub unsafe extern "C" fn machine_power_off() -> ! {
    do_kernel_power_off();
    loop {}
}

#[no_mangle] pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

pub unsafe extern "C" fn show_regs(regs: *mut pt_regs) {
    // pr_info calls are provided by the kernel environment.
    pr_info!("Format {:02x}  Vector: {:04x}  PC: {:08x}  Status: {:04x}    {:?}\n", (*regs).format, (*regs).vector, (*regs).pc, (*regs).sr, print_tainted());
    pr_info!("ORIG_D0: {:08x}  D0: {:08x}  A2: {:08x}  A1: {:08x}\n", (*regs).orig_d0, (*regs).d0, (*regs).a2, (*regs).a1);
    pr_info!("A0: {:08x}  D5: {:08x}  D4: {:08x}\n", (*regs).a0, (*regs).d5, (*regs).d4);
    pr_info!("D3: {:08x}  D2: {:08x}  D1: {:08x}\n", (*regs).d3, (*regs).d2, (*regs).d1);
    if (*regs).sr & PS_S == 0 { pr_info!("USP: {:08x}\n", rdusp()); }
}

pub unsafe extern "C" fn flush_thread() {
    // CONFIG_FPU-dependent fsave/frestore operations are supplied by the target.
    let _ = USER_DATA;
}

pub unsafe extern "C" fn m68k_clone(regs: *mut pt_regs) -> i32 {
    let args = kernel_clone_args { flags: ((*regs).d1 as u32 as u64) & !CSIGNAL, pidfd: (*regs).d3 as *mut i32, child_tid: (*regs).d4 as *mut i32, parent_tid: (*regs).d3 as *mut i32, exit_signal: (*regs).d1 as u64 & CSIGNAL, stack: (*regs).d2, tls: (*regs).d5, fn_: 0, fn_arg: 0 };
    kernel_clone(&args)
}

pub unsafe extern "C" fn m68k_clone3(regs: *mut pt_regs) -> i32 {
    sys_clone3((*regs).d1 as *const clone_args, (*regs).d2)
}

pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32 {
    let clone_flags = (*args).flags;
    let usp = (*args).stack;
    let tls = (*args).tls;
    #[repr(C)] struct fork_frame { sw: switch_stack, regs: pt_regs }
    let frame = (task_stack_page(p) as usize + THREAD_SIZE - core::mem::size_of::<fork_frame>()) as *mut fork_frame;
    (*p).thread.ksp = frame as usize;
    (*p).thread.esp0 = &mut (*frame).regs as *mut pt_regs as usize;
    (*p).thread.fc = USER_DATA;
    if (*args).fn_ != 0 {
        core::ptr::write_bytes(frame, 0, 1);
        (*frame).regs.sr = PS_S;
        (*frame).sw.a3 = (*args).fn_;
        (*frame).sw.d7 = (*args).fn_arg;
        (*frame).sw.retpc = ret_from_kernel_thread as usize;
        (*p).thread.usp = 0;
        return 0;
    }
    core::ptr::copy_nonoverlapping(current_pt_regs() as *const u8, frame as *mut u8, core::mem::size_of::<fork_frame>());
    (*frame).regs.d0 = 0;
    (*frame).sw.retpc = ret_from_fork as usize;
    (*p).thread.usp = if usp != 0 { usp } else { rdusp() };
    if clone_flags & CLONE_SETTLS != 0 { (*task_thread_info(p)).tp_value = tls; }
    0
}

pub unsafe extern "C" fn elf_core_copy_task_fpregs(_t: *mut task_struct, _fpu: *mut elf_fpregset_t) -> i32 {
    // CONFIG_FPU and emulator-specific register conversions are target-provided.
    1
}

pub unsafe extern "C" fn __get_wchan(p: *mut task_struct) -> usize {
    let stack_page = task_stack_page(p) as usize;
    let mut fp = (*( (*p).thread.ksp as *mut switch_stack)).a6;
    let mut count = 0;
    loop {
        if fp < stack_page + core::mem::size_of::<thread_info>() || fp >= 8184 + stack_page { return 0; }
        let pc = *((fp as *const usize).add(1));
        if !in_sched_functions(pc) { return pc; }
        fp = *(fp as *const usize);
        count += 1;
        if count > 16 { return 0; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
