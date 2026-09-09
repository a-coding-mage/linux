// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014-2016 Pratyush Anand <panand@redhat.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub const UPROBE_TRAP_NR: ::core::ffi::c_uint = ::core::ffi::c_uint::MAX;

extern "C" {
    fn is_insn32(insn: probe_opcode_t) -> bool;
    fn csky_probe_decode_insn(insn: *mut probe_opcode_t, api: *mut arch_uprobe_api) -> insn_decode_result;
    fn instruction_pointer(regs: *mut pt_regs) -> ::core::ffi::c_ulong;
    fn instruction_pointer_set(regs: *mut pt_regs, value: ::core::ffi::c_ulong);
    fn user_enable_single_step(task: *mut task_struct);
    fn user_disable_single_step(task: *mut task_struct);
    fn uprobe_pre_sstep_notifier(regs: *mut pt_regs) -> bool;
    fn uprobe_post_sstep_notifier(regs: *mut pt_regs) -> bool;
    fn WARN_ON_ONCE(condition: bool) -> bool;
    static mut current: *mut task_struct;
}

// These types and constants are declared by the architecture and uprobe headers.
extern "C" {
    type probe_opcode_t;
    type uprobe_opcode_t;
    type pt_regs;
    type arch_uprobe;
    type mm_struct;
    type task_struct;
    type uprobe_task;
    type return_instance;
    type notifier_block;
    type arch_uprobe_api;
    type insn_decode_result;
    type rp_check;
    static INSN_REJECTED: insn_decode_result;
    static INSN_GOOD_NO_SLOT: insn_decode_result;
    static UPROBE_SWBP_INSN: uprobe_opcode_t;
    static NOTIFY_DONE: ::core::ffi::c_int;
}

#[inline]
pub unsafe fn is_swbp_insn(insn: *mut uprobe_opcode_t) -> bool {
    (*insn as ::core::ffi::c_ulong & 0xffff) == UPROBE_SWBP_INSN as ::core::ffi::c_ulong
}

pub unsafe fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> ::core::ffi::c_ulong {
    instruction_pointer(regs)
}

pub unsafe fn arch_uprobe_analyze_insn(
    auprobe: *mut arch_uprobe,
    _mm: *mut mm_struct,
    _addr: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let mut insn: probe_opcode_t = *(&(*auprobe).insn[0] as *const _ as *const probe_opcode_t);
    (*auprobe).insn_size = if is_insn32(insn) { 4 } else { 2 };

    match csky_probe_decode_insn(&mut insn, &mut (*auprobe).api) {
        x if x as *const _ == &INSN_REJECTED as *const _ => -22,
        x if x as *const _ == &INSN_GOOD_NO_SLOT as *const _ => {
            (*auprobe).simulate = true;
            0
        }
        _ => 0,
    }
}

pub unsafe fn arch_uprobe_pre_xol(
    _auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> ::core::ffi::c_int {
    let utask = (*current).utask;
    (*utask).autask.saved_trap_no = (*current).thread.trap_no;
    (*current).thread.trap_no = UPROBE_TRAP_NR as _;
    instruction_pointer_set(regs, (*utask).xol_vaddr);
    user_enable_single_step(current);
    0
}

pub unsafe fn arch_uprobe_post_xol(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> ::core::ffi::c_int {
    let utask = (*current).utask;
    WARN_ON_ONCE((*current).thread.trap_no != UPROBE_TRAP_NR as _);
    (*current).thread.trap_no = (*utask).autask.saved_trap_no;
    instruction_pointer_set(regs, (*utask).vaddr.wrapping_add((*auprobe).insn_size as _));
    user_disable_single_step(current);
    0
}

pub unsafe fn arch_uprobe_xol_was_trapped(t: *mut task_struct) -> bool {
    if (*t).thread.trap_no != UPROBE_TRAP_NR as _ { true } else { false }
}

pub unsafe fn arch_uprobe_skip_sstep(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> bool {
    if !(*auprobe).simulate { return false; }
    let insn: probe_opcode_t = *(&(*auprobe).insn[0] as *const _ as *const probe_opcode_t);
    let addr = instruction_pointer(regs);
    if let Some(handler) = (*auprobe).api.handler {
        handler(insn, addr, regs);
    }
    true
}

pub unsafe fn arch_uprobe_abort_xol(_auprobe: *mut arch_uprobe, regs: *mut pt_regs) {
    let utask = (*current).utask;
    (*current).thread.trap_no = (*utask).autask.saved_trap_no;
    // Task has received a fatal signal, so reset back to probed address.
    instruction_pointer_set(regs, (*utask).vaddr);
    user_disable_single_step(current);
}

pub unsafe fn arch_uretprobe_is_alive(
    ret: *mut return_instance,
    ctx: rp_check,
    regs: *mut pt_regs,
) -> bool {
    if ctx == RP_CHECK_CHAIN_CALL { (*regs).usp <= (*ret).stack }
    else { (*regs).usp < (*ret).stack }
}

pub unsafe fn arch_uretprobe_hijack_return_addr(
    trampoline_vaddr: ::core::ffi::c_ulong,
    regs: *mut pt_regs,
) -> ::core::ffi::c_ulong {
    let ra = (*regs).lr;
    (*regs).lr = trampoline_vaddr;
    ra
}

pub unsafe fn arch_uprobe_exception_notify(
    _self: *mut notifier_block,
    _val: ::core::ffi::c_ulong,
    _data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int { NOTIFY_DONE as _ }

pub unsafe fn uprobe_breakpoint_handler(regs: *mut pt_regs) -> ::core::ffi::c_int {
    if uprobe_pre_sstep_notifier(regs) { 1 } else { 0 }
}

pub unsafe fn uprobe_single_step_handler(regs: *mut pt_regs) -> ::core::ffi::c_int {
    if uprobe_post_sstep_notifier(regs) { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
