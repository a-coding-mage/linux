// SPDX-License-Identifier: GPL-2.0-only
/*
 * AArch64 KGDB support
 *
 * Based on arch/arm/kernel/kgdb.c
 *
 * Copyright (C) 2013 Cavium Inc.
 * Author: Vijaya Kumar K <vijaya.kumar@caviumnetworks.com>
 */

// Linux and architecture headers from the C implementation provide the
// external types, constants, macros, and functions referenced below.

#[allow(non_upper_case_globals)]
pub static mut dbg_reg_def: [dbg_reg_def_t; DBG_MAX_REG_NUM as usize] = [
    dbg_reg_def_t { name: b"x0\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 0 * 8 },
    dbg_reg_def_t { name: b"x1\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 1 * 8 },
    dbg_reg_def_t { name: b"x2\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 2 * 8 },
    dbg_reg_def_t { name: b"x3\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 3 * 8 },
    dbg_reg_def_t { name: b"x4\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 4 * 8 },
    dbg_reg_def_t { name: b"x5\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 5 * 8 },
    dbg_reg_def_t { name: b"x6\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 6 * 8 },
    dbg_reg_def_t { name: b"x7\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 7 * 8 },
    dbg_reg_def_t { name: b"x8\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 8 * 8 },
    dbg_reg_def_t { name: b"x9\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 9 * 8 },
    dbg_reg_def_t { name: b"x10\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 10 * 8 },
    dbg_reg_def_t { name: b"x11\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 11 * 8 },
    dbg_reg_def_t { name: b"x12\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 12 * 8 },
    dbg_reg_def_t { name: b"x13\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 13 * 8 },
    dbg_reg_def_t { name: b"x14\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 14 * 8 },
    dbg_reg_def_t { name: b"x15\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 15 * 8 },
    dbg_reg_def_t { name: b"x16\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 16 * 8 },
    dbg_reg_def_t { name: b"x17\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 17 * 8 },
    dbg_reg_def_t { name: b"x18\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 18 * 8 },
    dbg_reg_def_t { name: b"x19\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 19 * 8 },
    dbg_reg_def_t { name: b"x20\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 20 * 8 },
    dbg_reg_def_t { name: b"x21\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 21 * 8 },
    dbg_reg_def_t { name: b"x22\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 22 * 8 },
    dbg_reg_def_t { name: b"x23\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 23 * 8 },
    dbg_reg_def_t { name: b"x24\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 24 * 8 },
    dbg_reg_def_t { name: b"x25\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 25 * 8 },
    dbg_reg_def_t { name: b"x26\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 26 * 8 },
    dbg_reg_def_t { name: b"x27\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 27 * 8 },
    dbg_reg_def_t { name: b"x28\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 28 * 8 },
    dbg_reg_def_t { name: b"x29\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 29 * 8 },
    dbg_reg_def_t { name: b"x30\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, regs) + 30 * 8 },
    dbg_reg_def_t { name: b"sp\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, sp) },
    dbg_reg_def_t { name: b"pc\0".as_ptr() as *const i8, size: 8, offset: core::mem::offset_of!(pt_regs, pc) },
    // PSTATE is 64 bits in pt_regs, but the GDB protocol uses its lower 32 bits.
    dbg_reg_def_t { name: b"pstate\0".as_ptr() as *const i8, size: 4, offset: core::mem::offset_of!(pt_regs, pstate)
        + if cfg!(target_endian = "big") { 4 } else { 0 } },
    dbg_reg_def_t { name: b"v0\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v1\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v2\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v3\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v4\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v5\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v6\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v7\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v8\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v9\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v10\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v11\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v12\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v13\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v14\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v15\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v16\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v17\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v18\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v19\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v20\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v21\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v22\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v23\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v24\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v25\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v26\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v27\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v28\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v29\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"v30\0".as_ptr() as *const i8, size: 16, offset: -1 }, dbg_reg_def_t { name: b"v31\0".as_ptr() as *const i8, size: 16, offset: -1 },
    dbg_reg_def_t { name: b"fpsr\0".as_ptr() as *const i8, size: 4, offset: -1 }, dbg_reg_def_t { name: b"fpcr\0".as_ptr() as *const i8, size: 4, offset: -1 },
];

pub unsafe fn dbg_get_reg(regno: i32, mem: *mut core::ffi::c_void, regs: *mut pt_regs) -> *const i8 {
    if regno >= DBG_MAX_REG_NUM || regno < 0 { return core::ptr::null(); }
    let d = &dbg_reg_def[regno as usize];
    if d.offset != -1 { core::ptr::copy_nonoverlapping((regs as *mut u8).offset(d.offset), mem as *mut u8, d.size as usize); }
    else { core::ptr::write_bytes(mem as *mut u8, 0, d.size as usize); }
    d.name
}

pub unsafe fn dbg_set_reg(regno: i32, mem: *mut core::ffi::c_void, regs: *mut pt_regs) -> i32 {
    if regno >= DBG_MAX_REG_NUM || regno < 0 { return -EINVAL; }
    let d = &dbg_reg_def[regno as usize];
    if d.offset != -1 { core::ptr::copy_nonoverlapping(mem as *const u8, (regs as *mut u8).offset(d.offset), d.size as usize); }
    0
}

pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut u64, task: *mut task_struct) {
    let cpu_context = &(*task).thread.cpu_context;
    core::ptr::write_bytes(gdb_regs as *mut u8, 0, NUMREGBYTES as usize);
    (*gdb_regs.add(19)) = cpu_context.x19; (*gdb_regs.add(20)) = cpu_context.x20; (*gdb_regs.add(21)) = cpu_context.x21;
    (*gdb_regs.add(22)) = cpu_context.x22; (*gdb_regs.add(23)) = cpu_context.x23; (*gdb_regs.add(24)) = cpu_context.x24;
    (*gdb_regs.add(25)) = cpu_context.x25; (*gdb_regs.add(26)) = cpu_context.x26; (*gdb_regs.add(27)) = cpu_context.x27;
    (*gdb_regs.add(28)) = cpu_context.x28; (*gdb_regs.add(29)) = cpu_context.fp;
    (*gdb_regs.add(31)) = cpu_context.sp; (*gdb_regs.add(32)) = cpu_context.pc;
}

pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, pc: u64) { (*regs).pc = pc; }
static mut compiled_break: i32 = 0;

unsafe fn kgdb_arch_update_addr(regs: *mut pt_regs, remcom_in_buffer: *mut i8) {
    let mut addr: u64 = 0; let mut ptr = remcom_in_buffer.add(1);
    if kgdb_hex2long(&mut ptr, &mut addr) != 0 { kgdb_arch_set_pc(regs, addr); }
    else if compiled_break == 1 { kgdb_arch_set_pc(regs, (*regs).pc.wrapping_add(4)); }
    compiled_break = 0;
}

pub unsafe fn kgdb_arch_handle_exception(_exception_vector: i32, _signo: i32, _err_code: i32, input: *mut i8, _output: *mut i8, regs: *mut pt_regs) -> i32 {
    let mut err;
    match *input as u8 {
        b'D' | b'k' | b'c' => { kgdb_arch_update_addr(regs, input); atomic_set(&mut kgdb_cpu_doing_single_step, -1); kgdb_single_step = 0; if kernel_active_single_step() != 0 { kernel_disable_single_step(); } err = 0; }
        b's' => { kgdb_arch_update_addr(regs, input); atomic_set(&mut kgdb_cpu_doing_single_step, raw_smp_processor_id()); kgdb_single_step = 1; if kernel_active_single_step() == 0 { kernel_enable_single_step(regs); } else { kernel_rewind_single_step(regs); } err = 0; }
        _ => { err = -1; }
    }
    err
}

pub unsafe fn kgdb_brk_handler(regs: *mut pt_regs, _esr: u64) -> i32 { kgdb_handle_exception(1, SIGTRAP, 0, regs); DBG_HOOK_HANDLED }
pub unsafe fn kgdb_compiled_brk_handler(regs: *mut pt_regs, _esr: u64) -> i32 { compiled_break = 1; kgdb_handle_exception(1, SIGTRAP, 0, regs); DBG_HOOK_HANDLED }
pub unsafe fn kgdb_single_step_handler(regs: *mut pt_regs, _esr: u64) -> i32 { if kgdb_single_step == 0 { return DBG_HOOK_ERROR; } kgdb_handle_exception(0, SIGTRAP, 0, regs); DBG_HOOK_HANDLED }

unsafe fn __kgdb_notify(args: *mut die_args, cmd: u64) -> i32 { if kgdb_handle_exception(1, (*args).signr, cmd, (*args).regs) != 0 { NOTIFY_DONE } else { NOTIFY_STOP } }
unsafe extern "C" fn kgdb_notify(_self: *mut notifier_block, cmd: u64, ptr: *mut core::ffi::c_void) -> i32 { let mut flags = 0; local_irq_save(&mut flags); let ret = __kgdb_notify(ptr as *mut die_args, cmd); local_irq_restore(flags); ret }
static mut kgdb_notifier: notifier_block = notifier_block { notifier_call: Some(kgdb_notify), priority: -INT_MAX };
pub unsafe fn kgdb_arch_init() -> i32 { register_die_notifier(&mut kgdb_notifier) }
pub unsafe fn kgdb_arch_exit() { unregister_die_notifier(&mut kgdb_notifier); }
pub static arch_kgdb_ops: kgdb_arch = kgdb_arch {};

pub unsafe fn kgdb_arch_set_breakpoint(bpt: *mut kgdb_bkpt) -> i32 {
    BUILD_BUG_ON(AARCH64_INSN_SIZE != BREAK_INSTR_SIZE);
    let err = aarch64_insn_read((*bpt).bpt_addr as *mut core::ffi::c_void, (*bpt).saved_instr as *mut u32);
    if err != 0 { return err; }
    aarch64_insn_write((*bpt).bpt_addr as *mut core::ffi::c_void, AARCH64_BREAK_KGDB_DYN_DBG as u32)
}
pub unsafe fn kgdb_arch_remove_breakpoint(bpt: *mut kgdb_bkpt) -> i32 { aarch64_insn_write((*bpt).bpt_addr as *mut core::ffi::c_void, *((*bpt).saved_instr as *mut u32)) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
