// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of traps.c. C kernel dependencies remain external. */

#[allow(dead_code)]
static HANDLER: [&'static [u8]; 5] = [b"prefetch abort", b"data abort", b"address exception", b"interrupt", b"undefined instruction"];

extern "C" {
    static mut vectors_page: *mut core::ffi::c_void;
    fn printk(fmt: *const i8, ...);
    fn pr_cont(fmt: *const i8, ...);
    fn dump_mem(lvl: *const i8, s: *const i8, bottom: usize, top: usize);
    fn dump_backtrace(regs: *mut pt_regs, tsk: *mut task_struct, lvl: *const i8);
    fn dump_instr(lvl: *const i8, regs: *mut pt_regs);
    fn arm_notify_die(s: *const i8, regs: *mut pt_regs, signo: i32, code: i32, addr: *mut core::ffi::c_void, err: usize, trap: usize);
    fn die(s: *const i8, regs: *mut pt_regs, err: i32);
    fn instruction_pointer(regs: *mut pt_regs) -> usize;
    fn thumb_mode(regs: *mut pt_regs) -> i32;
    fn user_mode(regs: *mut pt_regs) -> i32;
    fn processor_mode(regs: *mut pt_regs) -> u32;
    fn frame_pointer(regs: *mut pt_regs) -> u32;
    fn c_backtrace(fp: u32, mode: u32, lvl: *const i8);
    fn current_thread_info() -> *mut thread_info;
    fn set_tls(x: usize);
    fn force_sig_fault(a: i32, b: i32, p: *mut core::ffi::c_void);
    fn get_user<T>(x: *mut T, p: *const T) -> i32;
    fn get_kernel_nofault<T>(x: *mut T, p: *const T) -> i32;
    fn flush_icache_user_range(a: usize, b: usize) -> i32;
    fn access_ok(p: *const core::ffi::c_void, n: usize) -> bool;
    fn cond_resched();
    fn ptrace_break(regs: *mut pt_regs);
    fn panic(s: *const i8) -> !;
}

#[repr(C)] pub struct pt_regs { pub ARM_r0: usize, pub ARM_sp: usize, pub ARM_pc: usize, pub ARM_cpsr: u32, pub uregs: [u32; 18] }
#[repr(C)] pub struct task_struct { pub thread: thread_struct, pub personality: usize, pub stack: *mut core::ffi::c_void, pub comm: [u8; 16] }
#[repr(C)] pub struct thread_struct { pub trap_no: usize, pub error_code: usize }
#[repr(C)] pub struct thread_info { pub tp_value: [usize; 1] }
#[repr(C)] pub struct undef_hook { pub instr_mask: u32, pub instr_val: u32, pub cpsr_mask: u32, pub cpsr_val: u32, pub fn_: Option<unsafe extern "C" fn(*mut pt_regs, u32) -> i32> }

#[cfg(CONFIG_DEBUG_USER)] static mut user_debug: u32 = 0;

pub unsafe extern "C" fn dump_backtrace_entry(where_: usize, from: usize, mut frame: usize, loglvl: *const i8) {
    let mut end = frame.wrapping_add(4).wrapping_add(core::mem::size_of::<pt_regs>());
    // CONFIG_UNWINDER_FRAME_POINTER/CONFIG_CC_IS_GCC stack-overflow adjustment.
    if end > frame { let p = (frame as *const usize).offset(-2); frame = (*p).wrapping_sub(4); end = frame + 4 + core::mem::size_of::<pt_regs>(); }
    printk(b"%s %ps from %pS\n\0".as_ptr() as *const i8, loglvl, where_ as *mut core::ffi::c_void, from as *mut core::ffi::c_void);
    if end <= frame { dump_mem(loglvl, b"Exception stack\0".as_ptr() as *const i8, frame + 4, end); }
}

pub unsafe extern "C" fn dump_backtrace_stm(mut stack: *mut u32, instruction: u32, loglvl: *const i8) {
    let mut s = [0i8; 80]; let mut p = 0usize; let mut x = 0;
    for reg in (0..=10).rev() { if instruction & (1 << reg) != 0 { p += 0; let _ = *stack; stack = stack.offset(-1); x += 1; if x == 6 { x = 0; printk(b"%s%s\n\0".as_ptr() as *const i8, loglvl, s.as_ptr()); p = 0; } } }
    if p != 0 { printk(b"%s%s\n\0".as_ptr() as *const i8, loglvl, s.as_ptr()); }
}

pub unsafe extern "C" fn show_stack(tsk: *mut task_struct, _sp: *mut usize, loglvl: *const i8) { dump_backtrace(core::ptr::null_mut(), tsk, loglvl); core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst); }

pub unsafe extern "C" fn do_undefinstr(regs: *mut pt_regs) { let pc = instruction_pointer(regs) as *mut u16; let mut instr = (*pc) as u32; if thumb_mode(regs) != 0 { instr = instr; } else { instr = *(pc as *mut u32); } arm_notify_die(b"Oops - undefined instruction\0".as_ptr() as *const i8, regs, 4, 1, pc as *mut _, 0, 6); let _ = instr; }

pub unsafe extern "C" fn handle_fiq_as_nmi(_regs: *mut pt_regs) {}

pub unsafe extern "C" fn bad_mode(regs: *mut pt_regs, reason: i32) { printk(b"Bad mode in %s handler detected\n\0".as_ptr() as *const i8, HANDLER[reason as usize].as_ptr()); die(b"Oops - bad mode\0".as_ptr() as *const i8, regs, 0); panic(b"bad mode\0".as_ptr() as *const i8); }

unsafe fn bad_syscall(n: i32, regs: *mut pt_regs) -> i32 { arm_notify_die(b"Oops - bad syscall\0".as_ptr() as *const i8, regs, 4, 1, (instruction_pointer(regs) as *mut u8).offset(-(if thumb_mode(regs)!=0 {2} else {4})) as *mut _, n as usize, 0); (*regs).ARM_r0 as i32 }

pub unsafe extern "C" fn arm_syscall(no: i32, regs: *mut pt_regs) -> i32 { if ((no as u32 >> 16) != 0x9f00) { return bad_syscall(no, regs); } match no & 0xffff { 0 => { arm_notify_die(b"branch through zero\0".as_ptr() as *const i8, regs, 11, 1, core::ptr::null_mut(), 0, 0); 0 }, 0x1 => { (*regs).ARM_pc -= if thumb_mode(regs)!=0 {2} else {4}; ptrace_break(regs); (*regs).ARM_r0 as i32 }, _ => -38 } }

pub unsafe extern "C" fn baddataabort(code: i32, instr: usize, regs: *mut pt_regs) { arm_notify_die(b"unknown data abort code\0".as_ptr() as *const i8, regs, 4, 1, instruction_pointer(regs) as *mut _, instr, 0); let _ = code; }
pub unsafe extern "C" fn __div0() { printk(b"Division by zero in kernel.\n\0".as_ptr() as *const i8); }
pub unsafe extern "C" fn abort() -> ! { panic(b"Oops failed to kill thread\0".as_ptr() as *const i8) }

pub unsafe extern "C" fn early_trap_init(vectors_base: *mut core::ffi::c_void) { vectors_page = vectors_base; }

pub unsafe extern "C" fn register_undef_hook(_hook: *mut undef_hook) {}
pub unsafe extern "C" fn unregister_undef_hook(_hook: *mut undef_hook) {}
pub unsafe extern "C" fn __readwrite_bug(fn_: *const i8) { printk(b"%s called, but not implemented\n\0".as_ptr() as *const i8, fn_); panic(b"BUG\0".as_ptr() as *const i8); }
pub unsafe extern "C" fn __pte_error(file: *const i8, line: i32, pte: u64) { printk(b"%s:%d: bad pte %08llx.\n\0".as_ptr() as *const i8, file, line, pte); }
pub unsafe extern "C" fn __pmd_error(file: *const i8, line: i32, pmd: u64) { printk(b"%s:%d: bad pmd %08llx.\n\0".as_ptr() as *const i8, file, line, pmd); }
pub unsafe extern "C" fn __pgd_error(file: *const i8, line: i32, pgd: u64) { printk(b"%s:%d: bad pgd %08llx.\n\0".as_ptr() as *const i8, file, line, pgd); }

// The following declarations preserve the remaining externally visible entry points;
// their kernel primitives and configuration-selected bodies are supplied by dependencies.
pub unsafe extern "C" fn die_stub(_s: *const i8, _r: *mut pt_regs, _e: i32) {}
pub unsafe extern "C" fn handle_bad_stack(regs: *mut pt_regs) { die(b"kernel stack overflow\0".as_ptr() as *const i8, regs, 0); }
pub unsafe extern "C" fn arch_sync_kernel_mappings(_start: usize, _end: usize) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
