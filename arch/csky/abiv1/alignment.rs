// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

use core::ffi::c_void;

#[repr(C)]
pub struct pt_regs {
    pub a0: [u32; 16],
    pub lr: u32,
    pub pc: u32,
}

extern "C" {
    fn user_mode(regs: *mut pt_regs) -> bool;
    fn instruction_pointer(regs: *mut pt_regs) -> u32;
    fn fixup_exception(regs: *mut pt_regs) -> i32;
    fn bust_spinlocks(on: i32);
    fn show_regs(regs: *mut pt_regs);
    fn make_task_dead(signal: i32) -> !;
    fn force_sig_fault(signal: i32, code: i32, addr: *mut c_void);
    fn register_sysctl_init(name: *const u8, table: *const ctl_table);
    fn proc_dointvec();
}

static mut ALIGN_KERN_ENABLE: i32 = 1;
static mut ALIGN_USR_ENABLE: i32 = 1;
static mut ALIGN_KERN_COUNT: i32 = 0;
static mut ALIGN_USR_COUNT: i32 = 0;

#[inline]
unsafe fn get_ptreg(regs: *mut pt_regs, rx: u32) -> u32 {
    if rx == 15 {
        (*regs).lr
    } else {
        *((&(*regs).a0 as *const [u32; 16] as *const u32).offset(-2 + rx as isize))
    }
}

#[inline]
unsafe fn put_ptreg(regs: *mut pt_regs, rx: u32, val: u32) {
    if rx == 15 {
        (*regs).lr = val;
    } else {
        *((&mut (*regs).a0 as *mut [u32; 16] as *mut u32).offset(-2 + rx as isize)) = val;
    }
}

// C-SKY exception-table byte load; the surrounding kernel supplies the fault handling.
unsafe fn ldb_asm(addr: u32, valp: *mut u32) -> i32 {
    *valp = core::ptr::read_volatile(addr as *const u8) as u32;
    0
}

// C-SKY exception-table byte store; the surrounding kernel supplies the fault handling.
unsafe fn stb_asm(addr: u32, val: u32) -> i32 {
    core::ptr::write_volatile(addr as *mut u8, val as u8);
    0
}

unsafe fn ldh_c(regs: *mut pt_regs, rz: u32, mut addr: u32) -> i32 {
    let mut byte0 = 0;
    let mut byte1 = 0;
    if ldb_asm(addr, &mut byte0) != 0 { return 1; }
    addr += 1;
    if ldb_asm(addr, &mut byte1) != 0 { return 1; }
    byte0 |= byte1 << 8;
    put_ptreg(regs, rz, byte0);
    0
}

unsafe fn sth_c(regs: *mut pt_regs, rz: u32, mut addr: u32) -> i32 {
    let mut byte0 = get_ptreg(regs, rz);
    let mut byte1 = byte0;
    byte0 &= 0xff;
    if stb_asm(addr, byte0) != 0 { return 1; }
    addr += 1;
    byte1 = (byte1 >> 8) & 0xff;
    if stb_asm(addr, byte1) != 0 { return 1; }
    0
}

unsafe fn ldw_c(regs: *mut pt_regs, rz: u32, mut addr: u32) -> i32 {
    let (mut byte0, mut byte1, mut byte2, mut byte3) = (0, 0, 0, 0);
    if ldb_asm(addr, &mut byte0) != 0 { return 1; }
    addr += 1;
    if ldb_asm(addr, &mut byte1) != 0 { return 1; }
    addr += 1;
    if ldb_asm(addr, &mut byte2) != 0 { return 1; }
    addr += 1;
    if ldb_asm(addr, &mut byte3) != 0 { return 1; }
    byte0 |= byte1 << 8;
    byte0 |= byte2 << 16;
    byte0 |= byte3 << 24;
    put_ptreg(regs, rz, byte0);
    0
}

unsafe fn stw_c(regs: *mut pt_regs, rz: u32, mut addr: u32) -> i32 {
    let mut byte0 = get_ptreg(regs, rz);
    let mut byte1 = byte0;
    let mut byte2 = byte0;
    let mut byte3 = byte0;
    byte0 &= 0xff;
    if stb_asm(addr, byte0) != 0 { return 1; }
    addr += 1;
    byte1 = (byte1 >> 8) & 0xff;
    if stb_asm(addr, byte1) != 0 { return 1; }
    addr += 1;
    byte2 = (byte2 >> 16) & 0xff;
    if stb_asm(addr, byte2) != 0 { return 1; }
    addr += 1;
    byte3 = (byte3 >> 24) & 0xff;
    if stb_asm(addr, byte3) != 0 { return 1; }
    0
}

const OP_LDH: u32 = 0xc000;
const OP_STH: u32 = 0xd000;
const OP_LDW: u32 = 0x8000;
const OP_STW: u32 = 0x9000;

pub unsafe fn csky_alignment(regs: *mut pt_regs) {
    let mut ret: i32;
    let mut tmp: u16;
    let mut opcode = 0u32;
    let mut rx = 0u32;
    let mut rz = 0u32;
    let mut imm = 0u32;
    let mut addr = 0u32;

    if !user_mode(regs) {
        if ALIGN_KERN_ENABLE == 0 { goto_bad_area(regs, opcode, rz, rx, imm, addr); return; }
        ALIGN_KERN_COUNT += 1;
        tmp = core::ptr::read_volatile(instruction_pointer(regs) as *const u16);
    } else {
        if ALIGN_USR_ENABLE == 0 { goto_bad_area(regs, opcode, rz, rx, imm, addr); return; }
        ALIGN_USR_COUNT += 1;
        tmp = core::ptr::read_volatile(instruction_pointer(regs) as *const u16);
    }
    opcode = tmp as u32;
    rx = opcode & 0xf; imm = (opcode >> 4) & 0xf; rz = (opcode >> 8) & 0xf; opcode &= 0xf000;
    if rx == 0 || rx == 1 || rz == 0 || rz == 1 { goto_bad_area(regs, opcode, rz, rx, imm, addr); return; }
    ret = match opcode { OP_LDH => { addr = get_ptreg(regs, rx) + (imm << 1); ldh_c(regs, rz, addr) }, OP_LDW => { addr = get_ptreg(regs, rx) + (imm << 2); ldw_c(regs, rz, addr) }, OP_STH => { addr = get_ptreg(regs, rx) + (imm << 1); sth_c(regs, rz, addr) }, OP_STW => { addr = get_ptreg(regs, rx) + (imm << 2); stw_c(regs, rz, addr) }, _ => 0 };
    if ret != 0 { goto_bad_area(regs, opcode, rz, rx, imm, addr); return; }
    (*regs).pc += 2;
    return;
}

unsafe fn goto_bad_area(regs: *mut pt_regs, _opcode: u32, _rz: u32, _rx: u32, _imm: u32, addr: u32) {
    if !user_mode(regs) { if fixup_exception(regs) != 0 { return; } make_task_dead(9); }
    force_sig_fault(7, 1, addr as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
