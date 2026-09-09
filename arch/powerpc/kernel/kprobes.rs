// SPDX-License-Identifier: GPL-2.0-or-later
/* Kernel Probes (KProbes), translated from the PowerPC implementation. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// Kernel headers and configuration supplied by the surrounding repository.
extern "C" {
    static __kprobes_text_start: c_void;
    static __kprobes_text_end: c_void;
    static _stext: c_void;
    static __head_end: c_void;
    static arch_rethook_trampoline: c_void;
    fn kallsyms_lookup_name(name: *const c_char) -> usize;
    fn ftrace_location_range(a: usize, b: usize) -> usize;
    fn ftrace_location(a: usize) -> usize;
    fn ppc_function_entry(a: *mut kprobe_opcode_t) -> *mut kprobe_opcode_t;
    fn strnchr(s: *const c_char, n: usize, c: c_int) -> *const c_char;
    fn strscpy(d: *mut c_char, s: *const c_char, n: usize) -> isize;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn ppc_inst_read(p: *const kprobe_opcode_t) -> ppc_inst_t;
    fn can_single_step(v: u64) -> bool;
    fn ppc_inst_val(i: ppc_inst_t) -> u32;
    fn ppc_inst_as_ulong(i: ppc_inst_t) -> usize;
    fn ppc_inst_prefixed(i: ppc_inst_t) -> bool;
    fn ppc_inst_len(i: ppc_inst_t) -> usize;
    fn patch_instruction(p: *mut kprobe_opcode_t, i: ppc_inst_t) -> c_int;
    fn ppc_inst(v: u32) -> ppc_inst_t;
    fn get_insn_slot() -> *mut kprobe_opcode_t;
    fn free_insn_slot(p: *mut kprobe_opcode_t, v: c_int);
    fn get_kprobe(p: *mut kprobe_opcode_t) -> *mut kprobe;
    fn kprobe_ftrace(p: *mut kprobe) -> bool;
    fn enable_single_step(r: *mut pt_regs);
    fn regs_set_return_ip(r: *mut pt_regs, v: usize);
    fn regs_set_return_msr(r: *mut pt_regs, v: usize);
    fn kprobe_running() -> *mut kprobe;
    fn get_kprobe_ctlblk() -> *mut kprobe_ctlblk;
    fn reset_current_kprobe();
    fn emulate_step(r: *mut pt_regs, i: ppc_inst_t) -> c_int;
    fn user_mode(r: *mut pt_regs) -> bool;
    fn preempt_disable();
    fn preempt_enable();
    fn get_kernel_nofault(v: *mut u32, p: *mut u32) -> c_int;
    fn is_trap(v: u32) -> bool;
    fn kprobes_inc_nmissed_count(p: *mut kprobe);
    fn search_exception_tables(ip: usize) -> *const exception_table_entry;
    fn extable_fixup(e: *const exception_table_entry) -> usize;
    fn printk(s: *const c_char, ...);
    fn BUG() -> !;
    fn WARN_ON_ONCE(v: c_int) -> c_int;
}

type kprobe_opcode_t = u32;
#[repr(C)] pub struct ppc_inst_t { pub val: u64 }
#[repr(C)] pub struct pt_regs { pub nip: usize, pub msr: usize }
#[repr(C)] pub struct exception_table_entry { _private: [u8; 0] }
#[repr(C)] pub struct kprobe_ctlblk {
    pub prev_kprobe: prev_kprobe, pub kprobe_status: c_int, pub kprobe_saved_msr: usize,
}
#[repr(C)] pub struct prev_kprobe { pub kp: *mut kprobe, pub status: c_int, pub saved_msr: usize }
#[repr(C)] pub struct kprobe_insn { pub insn: *mut kprobe_opcode_t, pub boostable: c_int }
#[repr(C)] pub struct kprobe {
    pub addr: *mut kprobe_opcode_t, pub ainsn: kprobe_insn, pub opcode: u32,
    pub pre_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> c_int>,
    pub post_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs, c_int)>,
}
#[repr(C)] pub struct kretprobe_blackpoint { pub addr: *mut c_void, pub name: *mut c_char }
#[no_mangle] pub static mut kretprobe_blacklist: [kretprobe_blackpoint; 1] = [kretprobe_blackpoint { addr: core::ptr::null_mut(), name: core::ptr::null_mut() }];

const EINVAL: c_int = 22; const ENOMEM: c_int = 12;
const BREAKPOINT_INSTRUCTION: u32 = 0x7fe00008;
const PAGE_MASK: usize = !4095; const MSR_IR: usize = 1 << 5; const MSR_DR: usize = 1 << 4;
const MSR_SINGLESTEP: usize = 1 << 21;
const KPROBE_HIT_SS: c_int = 1; const KPROBE_REENTER: c_int = 2;
const KPROBE_HIT_ACTIVE: c_int = 4; const KPROBE_HIT_SSDONE: c_int = 5;

pub unsafe fn arch_within_kprobe_blacklist(addr: usize) -> bool {
    (addr >= &__kprobes_text_start as *const _ as usize && addr < &__kprobes_text_end as *const _ as usize) ||
    (addr >= &_stext as *const _ as usize && addr < &__head_end as *const _ as usize)
}

pub unsafe fn kprobe_lookup_name(name: *const c_char, offset: u32) -> *mut kprobe_opcode_t {
    let mut addr = kallsyms_lookup_name(name) as *mut kprobe_opcode_t;
    // PPC64 ELF ABI v2 uses local entry points; ABI v1 requires dot-symbol lookup.
    #[cfg(CONFIG_PPC64_ELF_ABI_V2)] if !addr.is_null() && offset == 0 {
        #[cfg(CONFIG_KPROBES_ON_FTRACE)] { let f = ftrace_location_range(addr as usize, addr as usize + 16); if f != 0 { addr = f as *mut _; } else { addr = ppc_function_entry(addr); } }
        #[cfg(not(CONFIG_KPROBES_ON_FTRACE))] { addr = ppc_function_entry(addr); }
    }
    addr
}

unsafe fn arch_kprobe_on_func_entry(addr: usize, offset: usize) -> bool {
    let ip = ftrace_location(addr); if ip != 0 { return offset <= ip.wrapping_sub(addr); }
    #[cfg(all(CONFIG_PPC64_ELF_ABI_V2, not(CONFIG_PPC_KERNEL_PCREL)))] { return offset <= 8; }
    offset == 0
}
pub unsafe fn arch_adjust_kprobe_addr(addr: usize, offset: usize, on: *mut bool) -> *mut kprobe_opcode_t { *on = arch_kprobe_on_func_entry(addr, offset); (addr.wrapping_add(offset)) as *mut _ }

pub unsafe fn arch_prepare_kprobe(p: *mut kprobe) -> c_int {
    let mut ret = 0; let insn = ppc_inst_read((*p).addr); let prev = get_kprobe((*p).addr.sub(1));
    if (*p).addr as usize & 3 != 0 { ret = -EINVAL; }
    else if !can_single_step(ppc_inst_val(insn) as u64) { ret = -EINVAL; }
    else if (*p).addr as usize & !PAGE_MASK != 0 && ppc_inst_prefixed(ppc_inst_read((*p).addr.sub(1))) { ret = -EINVAL; }
    if !prev.is_null() && !kprobe_ftrace(prev) && ppc_inst_prefixed(ppc_inst_read((*prev).ainsn.insn)) { ret = -EINVAL; }
    if ret == 0 { (*p).ainsn.insn = get_insn_slot(); if (*p).ainsn.insn.is_null() { ret = -ENOMEM; } }
    if ret == 0 { patch_instruction((*p).ainsn.insn, insn); (*p).opcode = ppc_inst_val(insn); }
    (*p).ainsn.boostable = 0; ret
}
pub unsafe fn arch_arm_kprobe(p: *mut kprobe) { WARN_ON_ONCE(patch_instruction((*p).addr, ppc_inst(BREAKPOINT_INSTRUCTION))); }
pub unsafe fn arch_disarm_kprobe(p: *mut kprobe) { WARN_ON_ONCE(patch_instruction((*p).addr, ppc_inst((*p).opcode))); }
pub unsafe fn arch_remove_kprobe(p: *mut kprobe) { if !(*p).ainsn.insn.is_null() { free_insn_slot((*p).ainsn.insn, 0); (*p).ainsn.insn = core::ptr::null_mut(); } }

unsafe fn prepare_singlestep(p: *mut kprobe, r: *mut pt_regs) { enable_single_step(r); regs_set_return_ip(r, (*p).ainsn.insn as usize); }
unsafe fn save_previous_kprobe(k: *mut kprobe_ctlblk) { (*k).prev_kprobe.kp = kprobe_running(); (*k).prev_kprobe.status = (*k).kprobe_status; (*k).prev_kprobe.saved_msr = (*k).kprobe_saved_msr; }
unsafe fn restore_previous_kprobe(k: *mut kprobe_ctlblk) { (*k).kprobe_status = (*k).prev_kprobe.status; (*k).kprobe_saved_msr = (*k).prev_kprobe.saved_msr; }
unsafe fn set_current_kprobe(p: *mut kprobe, r: *mut pt_regs, k: *mut kprobe_ctlblk) { (*k).kprobe_saved_msr = (*r).msr; }
unsafe fn try_to_emulate(p: *mut kprobe, r: *mut pt_regs) -> c_int { let ret = emulate_step(r, ppc_inst_read((*p).ainsn.insn)); if ret < 0 { BUG(); } else if ret == 0 && (*p).ainsn.boostable != 1 { (*p).ainsn.boostable = -1; } else if ret > 0 { (*p).ainsn.boostable = 1; } ret }

pub unsafe fn kprobe_handler(r: *mut pt_regs) -> c_int {
    if user_mode(r) || ((*r).msr & MSR_IR == 0 || (*r).msr & MSR_DR == 0) { return 0; }
    preempt_disable(); let k = get_kprobe_ctlblk(); let p = get_kprobe((*r).nip as *mut _); if p.is_null() { preempt_enable(); return 0; }
    (*k).kprobe_status = KPROBE_HIT_ACTIVE; set_current_kprobe(p, r, k);
    if let Some(f) = (*p).pre_handler { if f(p, r) != 0 { reset_current_kprobe(); preempt_enable(); return 1; } }
    if (*p).ainsn.boostable >= 0 && try_to_emulate(p, r) > 0 { if let Some(f) = (*p).post_handler { f(p, r, 0); } (*k).kprobe_status = KPROBE_HIT_SSDONE; reset_current_kprobe(); preempt_enable(); return 1; }
    prepare_singlestep(p, r); (*k).kprobe_status = KPROBE_HIT_SS; 1
}

pub unsafe fn kprobe_post_handler(r: *mut pt_regs) -> c_int { let p = kprobe_running(); if p.is_null() || user_mode(r) { return 0; } let k = get_kprobe_ctlblk(); let len = ppc_inst_len(ppc_inst_read((*p).ainsn.insn)); if (*p).ainsn.insn as usize + len != (*r).nip { return 0; } if (*k).kprobe_status != KPROBE_REENTER { if let Some(f) = (*p).post_handler { f(p, r, 0); } } regs_set_return_ip(r, (*p).addr as usize + len); regs_set_return_msr(r, (*r).msr | (*k).kprobe_saved_msr); reset_current_kprobe(); preempt_enable(); if (*r).msr & MSR_SINGLESTEP != 0 { 0 } else { 1 } }
pub unsafe fn kprobe_fault_handler(r: *mut pt_regs, _trapnr: c_int) -> c_int { let k = get_kprobe_ctlblk(); match (*k).kprobe_status { KPROBE_HIT_SS | KPROBE_REENTER => { regs_set_return_msr(r, ((*r).msr & !MSR_SINGLESTEP) | (*k).kprobe_saved_msr); reset_current_kprobe(); preempt_enable(); }, _ => {} } 0 }
pub unsafe fn arch_trampoline_kprobe(p: *mut kprobe) -> c_int { if (*p).addr == &arch_rethook_trampoline as *const _ as *mut _ { 1 } else { 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
