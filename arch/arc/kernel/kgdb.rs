// SPDX-License-Identifier: GPL-2.0-only
/*
 * kgdb support for ARC
 *
 * Copyright (C) 2012 Synopsys, Inc. (www.synopsys.com)
 */

// The Linux kernel and ARC definitions referenced by this translation are
// supplied by the surrounding Rust kernel sources.

struct pt_regs {
    fp: usize,
    sp: usize,
    blink: usize,
    ret: usize,
    status32: usize,
    lp_count: usize,
    lp_end: usize,
    lp_start: usize,
    bta: usize,
    ecr: Ecr,
}

struct Ecr {
    param: usize,
}

struct callee_regs;
struct task_struct;

extern "C" {
    static mut current: *mut TaskCurrent;
    static mut kgdb_cpu_doing_single_step: AtomicInt;

    fn get_reg(regno: i32, kernel_regs: *mut pt_regs, cregs: *mut callee_regs) -> usize;
    fn set_reg(regno: i32, value: usize, kernel_regs: *mut pt_regs, cregs: *mut callee_regs);
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    fn flush_icache_range(start: usize, end: usize);
    fn disasm_next_pc(
        ret: usize,
        regs: *mut pt_regs,
        cregs: *mut callee_regs,
        address0: *mut usize,
        address1: *mut usize,
    ) -> i32;
    fn kgdb_hex2long(ptr: *mut *mut i8, addr: *mut usize) -> i32;
    fn smp_processor_id() -> i32;
    fn atomic_set(value: *mut AtomicInt, v: i32);
    fn instruction_pointer(regs: *mut pt_regs) -> *mut usize;
    fn kgdb_handle_exception(e_vector: i32, signo: i32, err_code: i32, regs: *mut pt_regs);
    fn raw_smp_processor_id() -> i32;
    fn kgdb_nmicallback(cpu: i32, regs: *mut core::ffi::c_void);
}

struct TaskCurrent {
    thread: Thread,
}

struct Thread {
    callee_reg: *mut core::ffi::c_void,
}

struct AtomicInt;

#[repr(C)]
struct kgdb_arch {
    gdb_bpt_instr: [u8; 2],
}

const BREAK_INSTR_SIZE: usize = 2;
const GDB_MAX_REGS: i32 = 40;
const _R0: i32 = 0;
const _FP: i32 = 27;
const __SP: i32 = 28;
const _BLINK: i32 = 29;
const _RET: i32 = 30;
const _STATUS32: i32 = 31;
const _LP_COUNT: i32 = 32;
const _LP_END: i32 = 33;
const _LP_START: i32 = 34;
const _BTA: i32 = 35;
const _STOP_PC: i32 = 36;
const SIGTRAP: i32 = 5;

unsafe fn to_gdb_regs(gdb_regs: *mut usize, kernel_regs: *mut pt_regs, cregs: *mut callee_regs) {
    let mut regno: i32;

    regno = 0;
    while regno <= 26 {
        *gdb_regs.add((_R0 + regno) as usize) = get_reg(regno, kernel_regs, cregs);
        regno += 1;
    }

    regno = 27;
    while regno < GDB_MAX_REGS {
        *gdb_regs.add(regno as usize) = 0;
        regno += 1;
    }

    *gdb_regs.add(_FP as usize) = (*kernel_regs).fp;
    *gdb_regs.add(__SP as usize) = (*kernel_regs).sp;
    *gdb_regs.add(_BLINK as usize) = (*kernel_regs).blink;
    *gdb_regs.add(_RET as usize) = (*kernel_regs).ret;
    *gdb_regs.add(_STATUS32 as usize) = (*kernel_regs).status32;
    *gdb_regs.add(_LP_COUNT as usize) = (*kernel_regs).lp_count;
    *gdb_regs.add(_LP_END as usize) = (*kernel_regs).lp_end;
    *gdb_regs.add(_LP_START as usize) = (*kernel_regs).lp_start;
    *gdb_regs.add(_BTA as usize) = (*kernel_regs).bta;
    *gdb_regs.add(_STOP_PC as usize) = (*kernel_regs).ret;
}

unsafe fn from_gdb_regs(gdb_regs: *mut usize, kernel_regs: *mut pt_regs, cregs: *mut callee_regs) {
    let mut regno: i32 = 0;
    while regno <= 26 {
        set_reg(regno, *gdb_regs.add((regno + _R0) as usize), kernel_regs, cregs);
        regno += 1;
    }

    (*kernel_regs).fp = *gdb_regs.add(_FP as usize);
    (*kernel_regs).sp = *gdb_regs.add(__SP as usize);
    (*kernel_regs).blink = *gdb_regs.add(_BLINK as usize);
    (*kernel_regs).ret = *gdb_regs.add(_RET as usize);
    (*kernel_regs).status32 = *gdb_regs.add(_STATUS32 as usize);
    (*kernel_regs).lp_count = *gdb_regs.add(_LP_COUNT as usize);
    (*kernel_regs).lp_end = *gdb_regs.add(_LP_END as usize);
    (*kernel_regs).lp_start = *gdb_regs.add(_LP_START as usize);
    (*kernel_regs).bta = *gdb_regs.add(_BTA as usize);
}

pub unsafe fn pt_regs_to_gdb_regs(gdb_regs: *mut usize, kernel_regs: *mut pt_regs) {
    to_gdb_regs(gdb_regs, kernel_regs, (*(*current)).thread.callee_reg as *mut callee_regs);
}

pub unsafe fn gdb_regs_to_pt_regs(gdb_regs: *mut usize, kernel_regs: *mut pt_regs) {
    from_gdb_regs(gdb_regs, kernel_regs, (*(*current)).thread.callee_reg as *mut callee_regs);
}

pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut usize, task: *mut task_struct) {
    if !task.is_null() {
        to_gdb_regs(
            gdb_regs,
            task_pt_regs(task),
            (*(task as *mut TaskCurrent)).thread.callee_reg as *mut callee_regs,
        );
    }
}

struct single_step_data_t {
    opcode: [u16; 2],
    address: [usize; 2],
    is_branch: i32,
    armed: i32,
}

static mut single_step_data: single_step_data_t = single_step_data_t {
    opcode: [0; 2],
    address: [0; 2],
    is_branch: 0,
    armed: 0,
};

unsafe fn undo_single_step(_regs: *mut pt_regs) {
    if single_step_data.armed != 0 {
        let mut i = 0;
        let count = if single_step_data.is_branch != 0 { 2 } else { 1 };
        while i < count {
            memcpy(
                single_step_data.address[i] as *mut core::ffi::c_void,
                &single_step_data.opcode[i] as *const u16 as *const core::ffi::c_void,
                BREAK_INSTR_SIZE,
            );
            flush_icache_range(
                single_step_data.address[i],
                single_step_data.address[i].wrapping_add(BREAK_INSTR_SIZE),
            );
            i += 1;
        }
        single_step_data.armed = 0;
    }
}

unsafe fn place_trap(address: usize, save: *mut core::ffi::c_void) {
    memcpy(save, address as *const core::ffi::c_void, BREAK_INSTR_SIZE);
    memcpy(address as *mut core::ffi::c_void, &arch_kgdb_ops.gdb_bpt_instr as *const u8 as *const core::ffi::c_void, BREAK_INSTR_SIZE);
    flush_icache_range(address, address.wrapping_add(BREAK_INSTR_SIZE));
}

unsafe fn do_single_step(regs: *mut pt_regs) {
    single_step_data.is_branch = disasm_next_pc(
        (*regs).ret,
        regs,
        (*(*current)).thread.callee_reg as *mut callee_regs,
        &mut single_step_data.address[0],
        &mut single_step_data.address[1],
    );
    place_trap(single_step_data.address[0], &mut single_step_data.opcode[0] as *mut u16 as *mut core::ffi::c_void);
    if single_step_data.is_branch != 0 {
        place_trap(single_step_data.address[1], &mut single_step_data.opcode[1] as *mut u16 as *mut core::ffi::c_void);
    }
    single_step_data.armed += 1;
}

pub unsafe fn kgdb_arch_handle_exception(
    _e_vector: i32, _signo: i32, _err_code: i32, remcomInBuffer: *mut i8,
    _remcomOutBuffer: *mut i8, regs: *mut pt_regs,
) -> i32 {
    let mut addr: usize = 0;
    let mut ptr: *mut i8;
    undo_single_step(regs);
    match *remcomInBuffer as u8 {
        b's' | b'c' => {
            ptr = remcomInBuffer.add(1);
            if kgdb_hex2long(&mut ptr, &mut addr) != 0 { (*regs).ret = addr; }
        }
        b'D' | b'k' => {}
        _ => return -1,
    }
    atomic_set(&mut kgdb_cpu_doing_single_step, -1);
    if *remcomInBuffer as u8 == b's' {
        do_single_step(regs);
        atomic_set(&mut kgdb_cpu_doing_single_step, smp_processor_id());
    }
    0
}

pub unsafe fn kgdb_arch_init() -> i32 { single_step_data.armed = 0; 0 }

pub unsafe fn kgdb_trap(regs: *mut pt_regs) {
    if (*regs).ecr.param == 3 { *instruction_pointer(regs) -= BREAK_INSTR_SIZE; }
    kgdb_handle_exception(1, SIGTRAP, 0, regs);
}

pub unsafe fn kgdb_arch_exit() {}

pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, ip: usize) { *instruction_pointer(regs) = ip; }

pub unsafe fn kgdb_call_nmi_hook(_ignored: *mut core::ffi::c_void) {
    // Default implementation passes get_irq_regs() but we don't.
    kgdb_nmicallback(raw_smp_processor_id(), core::ptr::null_mut());
}

#[cfg(target_endian = "big")]
pub static arch_kgdb_ops: kgdb_arch = kgdb_arch { gdb_bpt_instr: [0x78, 0x7e] };
#[cfg(target_endian = "little")]
pub static arch_kgdb_ops: kgdb_arch = kgdb_arch { gdb_bpt_instr: [0x7e, 0x78] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
