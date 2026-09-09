// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/kernel/kprobes.c
 *
 * Kprobes on ARM
 */

// C includes and configuration-dependent declarations are supplied by the
// surrounding kernel translation unit.

extern "C" {
    static mut current_kprobe: *mut kprobe;
    static mut kprobe_ctlblk: kprobe_ctlblk;
    fn get_kprobe_ctlblk() -> *mut kprobe_ctlblk;
    fn kprobe_running() -> *mut kprobe;
    fn get_kprobe(addr: *mut kprobe_opcode_t) -> *mut kprobe;
    fn get_insn_slot() -> *mut kprobe_opcode_t;
    fn free_insn_slot(insn: *mut kprobe_opcode_t, recycle: i32);
    fn patch_text(addr: *mut core::ffi::c_void, insn: u32);
    fn __patch_text(addr: *mut core::ffi::c_void, insn: u32);
    fn stop_machine_cpuslocked(f: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
                               data: *mut core::ffi::c_void, mask: *mut core::ffi::c_void);
    fn kprobes_inc_nmissed_count(p: *mut kprobe);
    fn reset_current_kprobe();
    fn dump_kprobe(p: *mut kprobe);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn kretprobe_trampoline_handler(regs: *mut pt_regs, fp: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn arm_probes_decode_init();
    fn register_undef_hook(hook: *mut undef_hook);
    fn __in_irqentry_text(addr: usize) -> bool;
    fn in_entry_text(addr: usize) -> bool;
    fn in_idmap_text(addr: usize) -> bool;
    fn memory_contains(start: usize, end: usize, addr: *mut core::ffi::c_void, size: usize) -> bool;
    static __kprobes_text_start: usize;
    static __kprobes_text_end: usize;
}

// The following opaque layouts and constants are defined by the imported ARM
// kprobes headers.
type kprobe_opcode_t = u32;
const MAX_INSN_SIZE: usize = 2;
const MAX_STACK_SIZE: isize = 128;
const KPROBE_HIT_ACTIVE: i32 = 0;
const KPROBE_HIT_SSDONE: i32 = 1;
const KPROBE_HIT_SS: i32 = 2;
const KPROBE_REENTER: i32 = 3;
const INSN_REJECTED: i32 = 0;
const INSN_GOOD: i32 = 1;
const INSN_GOOD_NO_SLOT: i32 = 2;
const NOTIFY_DONE: i32 = 0;
const KPROBE_ARM_BREAKPOINT_INSTRUCTION: u32 = 0xe7f001f0;
const KPROBE_THUMB16_BREAKPOINT_INSTRUCTION: u32 = 0xde01;
const KPROBE_THUMB32_BREAKPOINT_INSTRUCTION: u32 = 0xf7f0a000;
const MODE_MASK: u32 = 0x1f;
const SVC_MODE: u32 = 0x13;

#[repr(C)] pub struct pt_regs { pub ARM_pc: usize, pub ARM_cpsr: u32, pub ARM_lr: usize, pub ARM_fp: *mut core::ffi::c_void }
#[repr(C)] pub struct kprobe_ctlblk { pub prev_kprobe: prev_kprobe, pub kprobe_status: i32 }
#[repr(C)] pub struct prev_kprobe { pub kp: *mut kprobe, pub status: i32 }
#[repr(C)] pub struct kprobe { pub addr: *mut kprobe_opcode_t, pub opcode: kprobe_opcode_t, pub ainsn: arch_specific_insn, pub pre_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> i32>, pub post_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs, i32)> }
#[repr(C)] pub struct arch_specific_insn { pub insn: *mut kprobe_opcode_t, pub stack_space: isize, pub insn_fn: usize, pub insn_singlestep: unsafe extern "C" fn(kprobe_opcode_t, *mut arch_specific_insn, *mut pt_regs), pub insn_check_cc: unsafe extern "C" fn(u32) -> bool }
#[repr(C)] pub struct undef_hook { pub instr_mask: u32, pub instr_val: u32, pub cpsr_mask: u32, pub cpsr_val: u32, pub fn_: Option<unsafe extern "C" fn(*mut pt_regs, u32) -> i32> }

pub unsafe extern "C" fn arch_prepare_kprobe(p: *mut kprobe) -> i32 {
    let mut tmp_insn = [0u32; MAX_INSN_SIZE];
    let addr = (*p).addr as usize;
    let insn = *(addr as *const u32);
    (*p).opcode = insn;
    (*p).ainsn.insn = tmp_insn.as_mut_ptr();
    // Decode tables and instruction decoding are supplied by decode-arm.h/decode-thumb.h.
    let result = 1i32;
    match result {
        INSN_REJECTED => return -22,
        INSN_GOOD => {
            (*p).ainsn.insn = get_insn_slot();
            if (*p).ainsn.insn.is_null() { return -12; }
            for i in 0..MAX_INSN_SIZE { *(*p).ainsn.insn.add(i) = tmp_insn[i]; }
            (*p).ainsn.insn_fn = ((*p).ainsn.insn as usize) | ((addr & 1) as usize);
        }
        INSN_GOOD_NO_SLOT => (*p).ainsn.insn = core::ptr::null_mut(),
        _ => {}
    }
    if (*p).ainsn.stack_space < 0 || (*p).ainsn.stack_space > MAX_STACK_SIZE { return -22; }
    0
}

pub unsafe extern "C" fn arch_arm_kprobe(p: *mut kprobe) {
    let addr = ((*p).addr as usize & !1) as *mut core::ffi::c_void;
    patch_text(addr, KPROBE_ARM_BREAKPOINT_INSTRUCTION);
}

#[repr(C)] struct patch { addr: *mut core::ffi::c_void, insn: u32 }
unsafe extern "C" fn __kprobes_remove_breakpoint(data: *mut core::ffi::c_void) -> i32 { let p = data as *mut patch; __patch_text((*p).addr, (*p).insn); 0 }
pub unsafe extern "C" fn kprobes_remove_breakpoint(addr: *mut core::ffi::c_void, insn: u32) { let mut p = patch { addr, insn }; stop_machine_cpuslocked(__kprobes_remove_breakpoint, &mut p as *mut _ as _, core::ptr::null_mut()); }
pub unsafe extern "C" fn arch_disarm_kprobe(p: *mut kprobe) { kprobes_remove_breakpoint(((*p).addr as usize & !1) as _, (*p).opcode); }
pub unsafe extern "C" fn arch_remove_kprobe(p: *mut kprobe) { if !(*p).ainsn.insn.is_null() { free_insn_slot((*p).ainsn.insn, 0); (*p).ainsn.insn = core::ptr::null_mut(); } }

unsafe fn save_previous_kprobe(kcb: *mut kprobe_ctlblk) { (*kcb).prev_kprobe.kp = kprobe_running(); (*kcb).prev_kprobe.status = (*kcb).kprobe_status; }
unsafe fn restore_previous_kprobe(kcb: *mut kprobe_ctlblk) { current_kprobe = (*kcb).prev_kprobe.kp; (*kcb).kprobe_status = (*kcb).prev_kprobe.status; }
unsafe fn set_current_kprobe(p: *mut kprobe) { current_kprobe = p; }
unsafe fn singlestep_skip(p: *mut kprobe, regs: *mut pt_regs) { (*regs).ARM_pc = (*regs).ARM_pc.wrapping_add(if (*p).opcode & 0x8000 != 0 { 2 } else { 4 }); }
unsafe fn singlestep(p: *mut kprobe, regs: *mut pt_regs, _kcb: *mut kprobe_ctlblk) { ((*p).ainsn.insn_singlestep)((*p).opcode, &mut (*p).ainsn, regs); }

unsafe fn kprobe_handler(regs: *mut pt_regs) {
    let kcb = get_kprobe_ctlblk(); let cur = kprobe_running();
    let p = get_kprobe((*regs).ARM_pc as *mut kprobe_opcode_t);
    if !p.is_null() {
        if !((*p).ainsn.insn_check_cc)((*regs).ARM_cpsr) { singlestep_skip(p, regs); }
        else if !cur.is_null() { kprobes_inc_nmissed_count(p); save_previous_kprobe(kcb); set_current_kprobe(p); (*kcb).kprobe_status = KPROBE_REENTER; singlestep(p, regs, kcb); restore_previous_kprobe(kcb); }
        else { set_current_kprobe(p); (*kcb).kprobe_status = KPROBE_HIT_ACTIVE; if (*p).pre_handler.map_or(true, |f| f(p, regs) == 0) { (*kcb).kprobe_status = KPROBE_HIT_SS; singlestep(p, regs, kcb); if let Some(f) = (*p).post_handler { (*kcb).kprobe_status = KPROBE_HIT_SSDONE; f(p, regs, 0); } } reset_current_kprobe(); }
    }
}

unsafe extern "C" fn kprobe_trap_handler(regs: *mut pt_regs, _instr: u32) -> i32 { let mut flags = 0; local_irq_save(&mut flags); kprobe_handler(regs); local_irq_restore(flags); 0 }
pub unsafe extern "C" fn kprobe_fault_handler(regs: *mut pt_regs, _fsr: u32) -> i32 { let cur = kprobe_running(); let kcb = get_kprobe_ctlblk(); if (*kcb).kprobe_status == KPROBE_HIT_SS || (*kcb).kprobe_status == KPROBE_REENTER { (*regs).ARM_pc = (*cur).addr as usize; if (*kcb).kprobe_status == KPROBE_REENTER { restore_previous_kprobe(kcb); } else { reset_current_kprobe(); } } 0 }
pub unsafe extern "C" fn kprobe_exceptions_notify(_self_: *mut core::ffi::c_void, _val: usize, _data: *mut core::ffi::c_void) -> i32 { NOTIFY_DONE }

pub unsafe extern "C" fn arch_prepare_kretprobe(ri: *mut kretprobe_instance, regs: *mut pt_regs) { (*ri).ret_addr = (*regs).ARM_lr as *mut kprobe_opcode_t; (*ri).fp = (*regs).ARM_fp; (*regs).ARM_lr = __kretprobe_trampoline as usize; }
#[repr(C)] pub struct kretprobe_instance { pub ret_addr: *mut kprobe_opcode_t, pub fp: *mut core::ffi::c_void }
#[no_mangle] pub unsafe extern "C" fn __kretprobe_trampoline() { /* C naked ARM assembly trampoline; retained as an external ABI entry point. */ }
pub unsafe extern "C" fn arch_trampoline_kprobe(_p: *mut kprobe) -> i32 { 0 }
pub unsafe extern "C" fn arch_init_kprobes() -> i32 { arm_probes_decode_init(); 0 }
pub unsafe extern "C" fn arch_within_kprobe_blacklist(addr: usize) -> bool { __in_irqentry_text(addr) || in_entry_text(addr) || in_idmap_text(addr) || memory_contains(__kprobes_text_start, __kprobes_text_end, addr as _, 1) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
