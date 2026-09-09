// SPDX-License-Identifier: GPL-2.0
/*
 * arch/arm/kernel/kgdb.c
 *
 * ARM KGDB support
 *
 * Copyright (c) 2002-2004 MontaVista Software, Inc
 * Copyright (c) 2008 Wind River Systems, Inc.
 *
 * Authors:  George Davis <davis_g@mvista.com>
 *           Deepak Saxena <dsaxena@plexity.net>
 */

use core::ffi::{c_char, c_int, c_void};

// Declarations supplied by the Linux ARM/kernel environment.
extern "C" {
    static mut dbg_reg_def: [dbg_reg_def_t; DBG_MAX_REG_NUM];
    static mut arch_kgdb_ops: kgdb_arch;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn kgdb_hex2long(ptr: *mut *mut c_char, value: *mut usize) -> c_int;
    fn kgdb_handle_exception(vector: c_int, signo: c_int, err_code: usize, regs: *mut pt_regs) -> c_int;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn register_die_notifier(nb: *mut notifier_block) -> c_int;
    fn unregister_die_notifier(nb: *mut notifier_block);
    fn register_undef_hook(hook: *mut undef_hook);
    fn unregister_undef_hook(hook: *mut undef_hook);
    fn copy_from_kernel_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> c_int;
    fn __patch_text(addr: *mut c_void, insn: u32);
    fn task_thread_info(task: *mut task_struct) -> *mut thread_info;
}

#[repr(C)]
pub struct dbg_reg_def_t {
    pub name: *const c_char,
    pub size: usize,
    pub offset: isize,
}

#[repr(C)]
pub struct pt_regs {
    pub ARM_r0: usize, pub ARM_r1: usize, pub ARM_r2: usize, pub ARM_r3: usize,
    pub ARM_r4: usize, pub ARM_r5: usize, pub ARM_r6: usize, pub ARM_r7: usize,
    pub ARM_r8: usize, pub ARM_r9: usize, pub ARM_r10: usize, pub ARM_fp: usize,
    pub ARM_ip: usize, pub ARM_sp: usize, pub ARM_lr: usize, pub ARM_pc: usize,
    pub ARM_cpsr: usize,
}

#[repr(C)]
pub struct cpu_context { pub r4: usize, pub r5: usize, pub r6: usize, pub r7: usize, pub r8: usize, pub r9: usize, pub sl: usize, pub fp: usize, pub sp: usize, pub pc: usize }
#[repr(C)] pub struct thread_info { pub cpu_context: cpu_context }
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct die_args { pub regs: *mut pt_regs, pub signr: c_int }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> c_int>, pub priority: c_int }
#[repr(C)] pub struct undef_hook { pub instr_mask: u32, pub instr_val: u32, pub cpsr_mask: u32, pub cpsr_val: u32, pub fn_: Option<unsafe extern "C" fn(*mut pt_regs, u32) -> c_int> }
#[repr(C)] pub struct kgdb_bkpt { pub saved_instr: *mut c_void, pub bpt_addr: usize }
#[repr(C)] pub struct kgdb_arch { pub gdb_bpt_instr: [u8; 4] }

pub const DBG_MAX_REG_NUM: usize = 26;
pub const GDB_MAX_REGS: usize = 26;
pub const _R4: usize = 4; pub const _R5: usize = 5; pub const _R6: usize = 6; pub const _R7: usize = 7;
pub const _R8: usize = 8; pub const _R9: usize = 9; pub const _R10: usize = 10; pub const _FP: usize = 11;
pub const _SPT: usize = 13; pub const _PC: usize = 15;
pub const EINVAL: c_int = 22;
pub const NOTIFY_DONE: c_int = 0; pub const NOTIFY_STOP: c_int = 0x8000;
pub const SIGTRAP: c_int = 5; pub const INT_MAX: c_int = 0x7fffffff;
pub const BREAK_INSTR_SIZE: usize = 4;
pub const KGDB_BREAKINST: u32 = 0xe7ffdefe;
pub const KGDB_COMPILED_BREAK: u32 = 0xe7ffdeff;
pub const PSR_T_BIT: u32 = 1 << 5; pub const MODE_MASK: u32 = 0x1f; pub const SVC_MODE: u32 = 0x13;

pub static mut dbg_reg_def_local: [dbg_reg_def_t; DBG_MAX_REG_NUM] = [dbg_reg_def_t { name: core::ptr::null(), size: 0, offset: 0 }; DBG_MAX_REG_NUM];

pub unsafe fn dbg_get_reg(regno: c_int, mem: *mut c_void, regs: *mut pt_regs) -> *mut c_char {
    if regno >= DBG_MAX_REG_NUM as c_int || regno < 0 { return core::ptr::null_mut(); }
    let reg = &dbg_reg_def[regno as usize];
    if reg.offset != -1 { memcpy(mem, (regs as *mut u8).offset(reg.offset) as *const c_void, reg.size); }
    else { memset(mem, 0, reg.size); }
    reg.name as *mut c_char
}

pub unsafe fn dbg_set_reg(regno: c_int, mem: *mut c_void, regs: *mut pt_regs) -> c_int {
    if regno >= DBG_MAX_REG_NUM as c_int || regno < 0 { return -EINVAL; }
    let reg = &dbg_reg_def[regno as usize];
    if reg.offset != -1 { memcpy((regs as *mut u8).offset(reg.offset) as *mut c_void, mem, reg.size); }
    0
}

pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut usize, task: *mut task_struct) {
    if task.is_null() { return; }
    for regno in 0..GDB_MAX_REGS { *gdb_regs.add(regno) = 0; }
    let ti = &*task_thread_info(task);
    *gdb_regs.add(_R4) = ti.cpu_context.r4; *gdb_regs.add(_R5) = ti.cpu_context.r5;
    *gdb_regs.add(_R6) = ti.cpu_context.r6; *gdb_regs.add(_R7) = ti.cpu_context.r7;
    *gdb_regs.add(_R8) = ti.cpu_context.r8; *gdb_regs.add(_R9) = ti.cpu_context.r9;
    *gdb_regs.add(_R10) = ti.cpu_context.sl; *gdb_regs.add(_FP) = ti.cpu_context.fp;
    *gdb_regs.add(_SPT) = ti.cpu_context.sp; *gdb_regs.add(_PC) = ti.cpu_context.pc;
}

pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, pc: usize) { (*regs).ARM_pc = pc; }
static mut compiled_break: c_int = 0;

pub unsafe fn kgdb_arch_handle_exception(_exception_vector: c_int, _signo: c_int, _err_code: c_int, remcom_in_buffer: *mut c_char, _remcom_out_buffer: *mut c_char, linux_regs: *mut pt_regs) -> c_int {
    let mut addr = 0usize;
    match *remcom_in_buffer as u8 {
        b'D' | b'k' | b'c' => {
            let mut ptr = remcom_in_buffer.add(1);
            if kgdb_hex2long(&mut ptr, &mut addr) != 0 { (*linux_regs).ARM_pc = addr; }
            else if compiled_break == 1 { (*linux_regs).ARM_pc = (*linux_regs).ARM_pc.wrapping_add(4); }
            compiled_break = 0; 0
        }, _ => -1,
    }
}

unsafe extern "C" fn kgdb_brk_fn(regs: *mut pt_regs, _instr: u32) -> c_int { kgdb_handle_exception(1, SIGTRAP, 0, regs); 0 }
unsafe extern "C" fn kgdb_compiled_brk_fn(regs: *mut pt_regs, _instr: u32) -> c_int { compiled_break = 1; kgdb_handle_exception(1, SIGTRAP, 0, regs); 0 }

static mut kgdb_brkpt_arm_hook: undef_hook = undef_hook { instr_mask: 0xffffffff, instr_val: KGDB_BREAKINST, cpsr_mask: PSR_T_BIT | MODE_MASK, cpsr_val: SVC_MODE, fn_: Some(kgdb_brk_fn) };
static mut kgdb_brkpt_thumb_hook: undef_hook = undef_hook { instr_mask: 0xffff, instr_val: KGDB_BREAKINST & 0xffff, cpsr_mask: PSR_T_BIT | MODE_MASK, cpsr_val: PSR_T_BIT | SVC_MODE, fn_: Some(kgdb_brk_fn) };
static mut kgdb_compiled_brkpt_arm_hook: undef_hook = undef_hook { instr_mask: 0xffffffff, instr_val: KGDB_COMPILED_BREAK, cpsr_mask: PSR_T_BIT | MODE_MASK, cpsr_val: SVC_MODE, fn_: Some(kgdb_compiled_brk_fn) };
static mut kgdb_compiled_brkpt_thumb_hook: undef_hook = undef_hook { instr_mask: 0xffff, instr_val: KGDB_COMPILED_BREAK & 0xffff, cpsr_mask: PSR_T_BIT | MODE_MASK, cpsr_val: PSR_T_BIT | SVC_MODE, fn_: Some(kgdb_compiled_brk_fn) };

unsafe fn __kgdb_notify(args: *mut die_args, cmd: usize) -> c_int { if kgdb_handle_exception(1, (*args).signr, cmd, (*args).regs) != 0 { NOTIFY_DONE } else { NOTIFY_STOP } }
unsafe extern "C" fn kgdb_notify(_self: *mut notifier_block, cmd: usize, ptr: *mut c_void) -> c_int { let mut flags = 0usize; local_irq_save(&mut flags); let ret = __kgdb_notify(ptr as *mut die_args, cmd); local_irq_restore(flags); ret }
static mut kgdb_notifier: notifier_block = notifier_block { notifier_call: Some(kgdb_notify), priority: -INT_MAX };

pub unsafe fn kgdb_arch_init() -> c_int { let ret = register_die_notifier(&mut kgdb_notifier); if ret != 0 { return ret; } register_undef_hook(&mut kgdb_brkpt_arm_hook); register_undef_hook(&mut kgdb_brkpt_thumb_hook); register_undef_hook(&mut kgdb_compiled_brkpt_arm_hook); register_undef_hook(&mut kgdb_compiled_brkpt_thumb_hook); 0 }
pub unsafe fn kgdb_arch_exit() { unregister_undef_hook(&mut kgdb_brkpt_arm_hook); unregister_undef_hook(&mut kgdb_brkpt_thumb_hook); unregister_undef_hook(&mut kgdb_compiled_brkpt_arm_hook); unregister_undef_hook(&mut kgdb_compiled_brkpt_thumb_hook); unregister_die_notifier(&mut kgdb_notifier); }
pub unsafe fn kgdb_arch_set_breakpoint(bpt: *mut kgdb_bkpt) -> c_int { let err = copy_from_kernel_nofault((*bpt).saved_instr, (*bpt).bpt_addr as *const c_void, BREAK_INSTR_SIZE); if err != 0 { return err; } __patch_text((*bpt).bpt_addr as *mut c_void, (*(arch_kgdb_ops.gdb_bpt_instr.as_ptr() as *const u32))); err }
pub unsafe fn kgdb_arch_remove_breakpoint(bpt: *mut kgdb_bkpt) -> c_int { __patch_text((*bpt).bpt_addr as *mut c_void, *((*bpt).saved_instr as *const u32)); 0 }

// Register our undef instruction hooks with ARM undef core.
pub static arch_kgdb_ops_local: kgdb_arch = kgdb_arch { gdb_bpt_instr: [0xfe, 0xde, 0xff, 0xe7] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
