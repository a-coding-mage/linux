// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARMv8 single-step debug support and mdscr context switching.
 *
 * Copyright (C) 2012 ARM Limited
 *
 * Author: Will Deacon <will.deacon@arm.com>
 */

/* Linux and ARM dependencies are supplied by the surrounding kernel translation. */

extern "C" {
    fn cpuid_feature_extract_unsigned_field(reg: u64, shift: u32) -> u8;
    fn read_sanitised_ftr_reg(reg: u32) -> u64;
    fn write_sysreg(value: u64, reg: u32);
    fn read_sysreg(reg: u32) -> u64;
    fn debugfs_create_bool(name: *const u8, mode: u32, parent: *mut core::ffi::c_void, value: *mut bool);
    fn cpuhp_setup_state(state: i32, name: *const u8, startup: unsafe extern "C" fn(u32) -> i32, teardown: *const core::ffi::c_void) -> i32;
    fn preemptible() -> bool;
    fn this_cpu_inc_return(counter: *mut i32) -> i32;
    fn this_cpu_dec_return(counter: *mut i32) -> i32;
    fn isb();
    fn user_mode(regs: *mut pt_regs) -> bool;
    fn regs_irqs_disabled(regs: *mut pt_regs) -> bool;
    fn local_irq_enable();
    fn instruction_pointer(regs: *mut pt_regs) -> u64;
    fn arm64_force_sig_fault(sig: i32, code: i32, addr: u64, desc: *const u8);
    fn current_pt_regs() -> *mut pt_regs;
    fn uprobe_single_step_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn kgdb_single_step_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn pr_warn(format: *const u8);
    fn esr_brk_comment(esr: u64) -> u32;
    fn bug_brk_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn esr_is_cfi_brk(esr: u64) -> bool;
    fn cfi_brk_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn reserved_fault_brk_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn kasan_brk_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn esr_is_ubsan_brk(esr: u64) -> bool;
    fn ubsan_brk_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn kgdb_brk_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn kgdb_compiled_brk_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn kprobe_brk_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn kprobe_ss_brk_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn kretprobe_brk_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn uprobe_brk_handler(regs: *mut pt_regs, esr: u64) -> i32;
    fn die(msg: *const u8, regs: *mut pt_regs, esr: u64) -> !;
    fn compat_user_mode(regs: *mut pt_regs) -> bool;
    fn compat_thumb_mode(regs: *mut pt_regs) -> bool;
    fn get_user<T>(dst: *mut T, src: *const T) -> i32;
    fn send_user_sigtrap(code: i32);
    fn test_tsk_thread_flag(task: *mut task_struct, flag: i32) -> bool;
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn test_and_set_ti_thread_flag(ti: *mut thread_info, flag: i32) -> bool;
    fn task_thread_info(task: *mut task_struct) -> *mut thread_info;
    fn clear_ti_thread_flag(ti: *mut thread_info, flag: i32);
}

#[repr(C)] pub struct user_pt_regs { pub pstate: u64, pub _opaque: [u8; 0] }
#[repr(C)] pub struct pt_regs { pub user_regs: user_pt_regs, pub pc: u64, pub _opaque: [u8; 0] }
#[repr(C)] pub struct task_struct { _opaque: [u8; 0] }
#[repr(C)] pub struct thread_info { _opaque: [u8; 0] }

pub const DBG_ACTIVE_EL1: i32 = 1;
pub const DBG_HOOK_HANDLED: i32 = 0;
pub const DBG_HOOK_ERROR: i32 = -1;
pub const DBG_SPSR_SS: u64 = 1 << 21;
pub const MDSCR_EL1_MDE: u64 = 1 << 15;
pub const MDSCR_EL1_KDE: u64 = 1 << 13;
pub const MDSCR_EL1_SS: u64 = 1 << 0;
pub const TRAP_TRACE: i32 = 2;
pub const TRAP_BRKPT: i32 = 1;

pub const SYS_ID_AA64DFR0_EL1: u32 = 0;
pub const ID_AA64DFR0_EL1_DebugVer_SHIFT: u32 = 0;
pub const TIF_SINGLESTEP: i32 = 0;
pub const BUG_BRK_IMM: u32 = 0;
pub const FAULT_BRK_IMM: u32 = 0;
pub const KASAN_BRK_MASK: u32 = 0;
pub const KASAN_BRK_IMM: u32 = 0;
pub const KGDB_DYN_DBG_BRK_IMM: u32 = 0;
pub const KGDB_COMPILED_DBG_BRK_IMM: u32 = 0;
pub const KPROBES_BRK_IMM: u32 = 0;
pub const KPROBES_BRK_SS_IMM: u32 = 0;
pub const KRETPROBES_BRK_IMM: u32 = 0;
pub const UPROBES_BRK_IMM: u32 = 0;
pub const AARCH32_BREAK_THUMB2_LO: u16 = 0;
pub const AARCH32_BREAK_THUMB2_HI: u16 = 0;
pub const AARCH32_BREAK_THUMB: u16 = 0;
pub const AARCH32_BREAK_ARM: u32 = 0;

static mut DEBUG_ENABLED: bool = true;
static mut MDE_REF_COUNT: i32 = 0;
static mut KDE_REF_COUNT: i32 = 0;

pub unsafe fn debug_monitors_arch() -> u8 {
    cpuid_feature_extract_unsigned_field(read_sanitised_ftr_reg(SYS_ID_AA64DFR0_EL1), ID_AA64DFR0_EL1_DebugVer_SHIFT)
}

unsafe fn mdscr_write(mdscr: u64) { write_sysreg(mdscr, 0); }
unsafe fn mdscr_read() -> u64 { read_sysreg(0) }

unsafe fn create_debug_debugfs_entry() -> i32 {
    debugfs_create_bool(b"debug_enabled\0".as_ptr(), 0o644, core::ptr::null_mut(), &raw mut DEBUG_ENABLED);
    0
}

unsafe extern "C" fn early_debug_disable(_buf: *mut u8) -> i32 { DEBUG_ENABLED = false; 0 }

pub unsafe fn enable_debug_monitors(el: i32) {
    let mut enable = 0u64;
    if preemptible() { }
    if this_cpu_inc_return(&raw mut MDE_REF_COUNT) == 1 { enable = MDSCR_EL1_MDE; }
    if el == DBG_ACTIVE_EL1 && this_cpu_inc_return(&raw mut KDE_REF_COUNT) == 1 { enable |= MDSCR_EL1_KDE; }
    if enable != 0 && DEBUG_ENABLED { mdscr_write(mdscr_read() | enable); }
}

pub unsafe fn disable_debug_monitors(el: i32) {
    let mut disable = 0u64;
    if preemptible() { }
    if this_cpu_dec_return(&raw mut MDE_REF_COUNT) == 0 { disable = !MDSCR_EL1_MDE; }
    if el == DBG_ACTIVE_EL1 && this_cpu_dec_return(&raw mut KDE_REF_COUNT) == 0 { disable &= !MDSCR_EL1_KDE; }
    if disable != 0 { mdscr_write(mdscr_read() & disable); }
}

unsafe extern "C" fn clear_os_lock(_cpu: u32) -> i32 { write_sysreg(0, 0); write_sysreg(0, 0); isb(); 0 }
unsafe fn debug_monitors_init() -> i32 { cpuhp_setup_state(0, b"arm64/debug_monitors:starting\0".as_ptr(), clear_os_lock, core::ptr::null()) }

unsafe fn set_user_regs_spsr_ss(regs: *mut user_pt_regs) { (*regs).pstate |= DBG_SPSR_SS; }
unsafe fn clear_user_regs_spsr_ss(regs: *mut user_pt_regs) { (*regs).pstate &= !DBG_SPSR_SS; }
unsafe fn set_regs_spsr_ss(regs: *mut pt_regs) { set_user_regs_spsr_ss(&mut (*regs).user_regs); }
unsafe fn clear_regs_spsr_ss(regs: *mut pt_regs) { clear_user_regs_spsr_ss(&mut (*regs).user_regs); }

unsafe fn send_user_sigtrap_local(si_code: i32) {
    let regs = current_pt_regs();
    if !user_mode(regs) { return; }
    if !regs_irqs_disabled(regs) { local_irq_enable(); }
    arm64_force_sig_fault(5, si_code, instruction_pointer(regs), b"User debug trap\0".as_ptr());
}

pub unsafe fn do_el0_softstep(esr: u64, regs: *mut pt_regs) {
    if uprobe_single_step_handler(regs, esr) == DBG_HOOK_HANDLED { return; }
    send_user_sigtrap_local(TRAP_TRACE);
    user_rewind_single_step(core::ptr::null_mut());
}

pub unsafe fn do_el1_softstep(esr: u64, regs: *mut pt_regs) {
    if kgdb_single_step_handler(regs, esr) == DBG_HOOK_HANDLED { return; }
    pr_warn(b"Unexpected kernel single-step exception at EL1\n\0".as_ptr());
    set_regs_spsr_ss(regs);
}

unsafe fn call_el1_break_hook(regs: *mut pt_regs, esr: u64) -> i32 {
    let comment = esr_brk_comment(esr);
    if comment == BUG_BRK_IMM { return bug_brk_handler(regs, esr); }
    if comment == FAULT_BRK_IMM { return reserved_fault_brk_handler(regs, esr); }
    if comment == KASAN_BRK_IMM { return kasan_brk_handler(regs, esr); }
    if comment == KGDB_DYN_DBG_BRK_IMM { return kgdb_brk_handler(regs, esr); }
    if comment == KGDB_COMPILED_DBG_BRK_IMM { return kgdb_compiled_brk_handler(regs, esr); }
    if comment == KPROBES_BRK_IMM { return kprobe_brk_handler(regs, esr); }
    if comment == KPROBES_BRK_SS_IMM { return kprobe_ss_brk_handler(regs, esr); }
    if comment == KRETPROBES_BRK_IMM { return kretprobe_brk_handler(regs, esr); }
    if esr_is_cfi_brk(esr) { return cfi_brk_handler(regs, esr); }
    if esr_is_ubsan_brk(esr) { return ubsan_brk_handler(regs, esr); }
    DBG_HOOK_ERROR
}

pub unsafe fn do_el0_brk64(esr: u64, regs: *mut pt_regs) {
    if esr_brk_comment(esr) == UPROBES_BRK_IMM && uprobe_brk_handler(regs, esr) == DBG_HOOK_HANDLED { return; }
    send_user_sigtrap_local(TRAP_BRKPT);
}
pub unsafe fn do_el1_brk64(esr: u64, regs: *mut pt_regs) { if call_el1_break_hook(regs, esr) != DBG_HOOK_HANDLED { die(b"Oops - BRK\0".as_ptr(), regs, esr); } }

pub unsafe fn try_handle_aarch32_break(regs: *mut pt_regs) -> bool {
    if !compat_user_mode(regs) { return false; }
    let pc = instruction_pointer(regs) as *const u8;
    let bp = if compat_thumb_mode(regs) {
        let first = u16::from_le(*(pc as *const u16));
        if first == AARCH32_BREAK_THUMB2_LO { u16::from_le(*((pc.add(2)) as *const u16)) == AARCH32_BREAK_THUMB2_HI } else { first == AARCH32_BREAK_THUMB }
    } else { (u32::from_le(*(pc as *const u32)) & !0xf0000000) == AARCH32_BREAK_ARM };
    if !bp { return false; }
    send_user_sigtrap_local(TRAP_BRKPT); true
}

pub unsafe fn user_rewind_single_step(task: *mut task_struct) { if test_tsk_thread_flag(task, TIF_SINGLESTEP) { set_regs_spsr_ss(task_pt_regs(task)); } }
pub unsafe fn user_fastforward_single_step(task: *mut task_struct) { if test_tsk_thread_flag(task, TIF_SINGLESTEP) { clear_regs_spsr_ss(task_pt_regs(task)); } }
pub unsafe fn user_regs_reset_single_step(regs: *mut user_pt_regs, task: *mut task_struct) { if test_tsk_thread_flag(task, TIF_SINGLESTEP) { set_user_regs_spsr_ss(regs); } else { clear_user_regs_spsr_ss(regs); } }
pub unsafe fn kernel_enable_single_step(regs: *mut pt_regs) { set_regs_spsr_ss(regs); mdscr_write(mdscr_read() | MDSCR_EL1_SS); enable_debug_monitors(DBG_ACTIVE_EL1); }
pub unsafe fn kernel_disable_single_step() { mdscr_write(mdscr_read() & !MDSCR_EL1_SS); disable_debug_monitors(DBG_ACTIVE_EL1); }
pub unsafe fn kernel_active_single_step() -> u64 { mdscr_read() & MDSCR_EL1_SS }
pub unsafe fn kernel_rewind_single_step(regs: *mut pt_regs) { set_regs_spsr_ss(regs); }
pub unsafe fn kernel_fastforward_single_step(regs: *mut pt_regs) { clear_regs_spsr_ss(regs); }
pub unsafe fn user_enable_single_step(task: *mut task_struct) { let ti = task_thread_info(task); if !test_and_set_ti_thread_flag(ti, TIF_SINGLESTEP) { set_regs_spsr_ss(task_pt_regs(task)); } }
pub unsafe fn user_disable_single_step(task: *mut task_struct) { clear_ti_thread_flag(task_thread_info(task), TIF_SINGLESTEP); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
