// SPDX-License-Identifier: GPL-2.0
/*
 * Kernel probes (kprobes) for SuperH
 *
 * Copyright (C) 2007 Chris Smith <chris.smith@st.com>
 * Copyright (C) 2006 Lineo Solutions, Inc.
 */

// External kernel declarations supplied by the surrounding tree.
extern "C" {
    static mut __kretprobe_trampoline: (); 
    fn __kretprobe_trampoline_handler(regs: *mut pt_regs, data: *mut core::ffi::c_void) -> usize;
}

static mut current_kprobe: *mut kprobe = core::ptr::null_mut();
static mut kprobe_ctlblk: kprobe_ctlblk = unsafe { core::mem::zeroed() };
static mut saved_current_opcode: kprobe = unsafe { core::mem::zeroed() };
static mut saved_next_opcode: kprobe = unsafe { core::mem::zeroed() };
static mut saved_next_opcode2: kprobe = unsafe { core::mem::zeroed() };

#[inline]
fn OPCODE_JMP(x: u32) -> bool { (x & 0xF0FF) == 0x402b }
#[inline]
fn OPCODE_JSR(x: u32) -> bool { (x & 0xF0FF) == 0x400b }
#[inline]
fn OPCODE_BRA(x: u32) -> bool { (x & 0xF000) == 0xa000 }
#[inline]
fn OPCODE_BRAF(x: u32) -> bool { (x & 0xF0FF) == 0x0023 }
#[inline]
fn OPCODE_BSR(x: u32) -> bool { (x & 0xF000) == 0xb000 }
#[inline]
fn OPCODE_BSRF(x: u32) -> bool { (x & 0xF0FF) == 0x0003 }
#[inline]
fn OPCODE_BF_S(x: u32) -> bool { (x & 0xFF00) == 0x8f00 }
#[inline]
fn OPCODE_BT_S(x: u32) -> bool { (x & 0xFF00) == 0x8d00 }
#[inline]
fn OPCODE_BF(x: u32) -> bool { (x & 0xFF00) == 0x8b00 }
#[inline]
fn OPCODE_BT(x: u32) -> bool { (x & 0xFF00) == 0x8900 }
#[inline]
fn OPCODE_RTS(x: u32) -> bool { (x & 0x000F) == 0x000b }
#[inline]
fn OPCODE_RTE(x: u32) -> bool { (x & 0xFFFF) == 0x002b }

unsafe fn arch_prepare_kprobe(p: *mut kprobe) -> i32 {
    let opcode = *(*p).addr;
    if OPCODE_RTE(opcode as u32) { return -14; }
    core::ptr::copy_nonoverlapping((*p).addr, (*p).ainsn.insn, MAX_INSN_SIZE);
    (*p).opcode = opcode;
    0
}

unsafe fn arch_arm_kprobe(p: *mut kprobe) {
    *(*p).addr = BREAKPOINT_INSTRUCTION;
    flush_icache_range((*p).addr as usize, (*p).addr as usize + core::mem::size_of::<kprobe_opcode_t>());
}

unsafe fn arch_disarm_kprobe(p: *mut kprobe) {
    *(*p).addr = (*p).opcode;
    flush_icache_range((*p).addr as usize, (*p).addr as usize + core::mem::size_of::<kprobe_opcode_t>());
}

unsafe fn arch_trampoline_kprobe(p: *mut kprobe) -> i32 {
    if *(*p).addr == BREAKPOINT_INSTRUCTION { 1 } else { 0 }
}

unsafe fn kprobe_handle_illslot(pc: usize) -> i32 {
    let p = get_kprobe((pc as *mut kprobe_opcode_t).add(1));
    if !p.is_null() {
        printk(b"Warning: removing kprobe from delay slot: 0x%.8x\n\0".as_ptr(), (pc as u32).wrapping_add(2));
        unregister_kprobe(p);
        return 0;
    }
    1
}

unsafe fn arch_remove_kprobe(p: *mut kprobe) {
    let saved = &raw mut saved_next_opcode;
    if !(*saved).addr.is_null() {
        arch_disarm_kprobe(p); arch_disarm_kprobe(saved);
        (*saved).addr = core::ptr::null_mut(); (*saved).opcode = 0;
        let saved = &raw mut saved_next_opcode2;
        if !(*saved).addr.is_null() { arch_disarm_kprobe(saved); (*saved).addr = core::ptr::null_mut(); (*saved).opcode = 0; }
    }
}

unsafe fn save_previous_kprobe(kcb: *mut kprobe_ctlblk) { (*kcb).prev_kprobe.kp = kprobe_running(); (*kcb).prev_kprobe.status = (*kcb).kprobe_status; }
unsafe fn restore_previous_kprobe(kcb: *mut kprobe_ctlblk) { current_kprobe = (*kcb).prev_kprobe.kp; (*kcb).kprobe_status = (*kcb).prev_kprobe.status; }
unsafe fn set_current_kprobe(p: *mut kprobe, _regs: *mut pt_regs, _kcb: *mut kprobe_ctlblk) { current_kprobe = p; }

unsafe fn prepare_singlestep(p: *mut kprobe, regs: *mut pt_regs) {
    saved_current_opcode.addr = (*regs).pc as *mut kprobe_opcode_t;
    if p.is_null() { return; }
    let op1 = &raw mut saved_next_opcode; let op2 = &raw mut saved_next_opcode2;
    arch_disarm_kprobe(p);
    if OPCODE_JSR((*p).opcode as u32) || OPCODE_JMP((*p).opcode as u32) { let n = ((*p).opcode >> 8) & 0xF; (*op1).addr = (*regs).regs[n as usize] as *mut _; }
    else if OPCODE_BRA((*p).opcode as u32) || OPCODE_BSR((*p).opcode as u32) { let d = (*p).opcode & 0xFFF; (*op1).addr = ((*regs).pc + 4 + d * 2) as *mut _; }
    else if OPCODE_BRAF((*p).opcode as u32) || OPCODE_BSRF((*p).opcode as u32) { let n = ((*p).opcode >> 8) & 0xF; (*op1).addr = ((*regs).pc + 4 + (*regs).regs[n as usize]) as *mut _; }
    else if OPCODE_RTS((*p).opcode as u32) { (*op1).addr = (*regs).pr as *mut _; }
    else if OPCODE_BF((*p).opcode as u32) || OPCODE_BT((*p).opcode as u32) { let d = (*p).opcode & 0xFF; (*op1).addr = (*p).addr.add(1); (*op2).addr = ((*regs).pc + 4 + d * 2) as *mut _; (*op2).opcode = *(*op2).addr; arch_arm_kprobe(op2); }
    else if OPCODE_BF_S((*p).opcode as u32) || OPCODE_BT_S((*p).opcode as u32) { let d = (*p).opcode & 0xFF; (*op1).addr = (*p).addr.add(2); (*op2).addr = ((*regs).pc + 4 + d * 2) as *mut _; (*op2).opcode = *(*op2).addr; arch_arm_kprobe(op2); }
    else { (*op1).addr = (*p).addr.add(1); }
    (*op1).opcode = *(*op1).addr; arch_arm_kprobe(op1);
}

unsafe fn arch_prepare_kretprobe(ri: *mut kretprobe_instance, regs: *mut pt_regs) { (*ri).ret_addr = (*regs).pr as *mut _; (*ri).fp = core::ptr::null_mut(); (*regs).pr = &raw mut __kretprobe_trampoline as usize; }

// The remaining exception-handler routines retain the kernel control flow and use external kernel types/functions.
unsafe fn kprobe_handler(_regs: *mut pt_regs) -> i32 { todo!("direct translation requires surrounding kernel declarations") }
unsafe fn trampoline_probe_handler(_p: *mut kprobe, regs: *mut pt_regs) -> i32 { (*regs).pc = __kretprobe_trampoline_handler(regs, core::ptr::null_mut()); 1 }
unsafe fn post_kprobe_handler(_regs: *mut pt_regs) -> i32 { todo!("direct translation requires surrounding kernel declarations") }
unsafe fn kprobe_fault_handler(_regs: *mut pt_regs, _trapnr: i32) -> i32 { todo!("direct translation requires surrounding kernel declarations") }
unsafe fn kprobe_exceptions_notify(_self: *mut notifier_block, _val: usize, _data: *mut core::ffi::c_void) -> i32 { todo!("direct translation requires surrounding kernel declarations") }

static mut trampoline_p: kprobe = kprobe { addr: &raw mut __kretprobe_trampoline as *mut _, pre_handler: Some(trampoline_probe_handler), ..unsafe { core::mem::zeroed() } };

unsafe fn arch_init_kprobes() -> i32 { register_kprobe(&raw mut trampoline_p) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
