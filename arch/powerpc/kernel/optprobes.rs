// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Code for Kernel probes Jump optimization.
 *
 * Copyright 2017, Anju T, IBM Corp.
 */

// Dependencies supplied by the kernel and architecture-specific headers.

const TMPL_CALL_HDLR_IDX: usize = (optprobe_template_call_handler as usize - optprobe_template_entry as usize);
const TMPL_EMULATE_IDX: usize = (optprobe_template_call_emulate as usize - optprobe_template_entry as usize);
const TMPL_RET_IDX: usize = (optprobe_template_ret as usize - optprobe_template_entry as usize);
const TMPL_OP_IDX: usize = (optprobe_template_op_address as usize - optprobe_template_entry as usize);
const TMPL_INSN_IDX: usize = (optprobe_template_insn as usize - optprobe_template_entry as usize);
const TMPL_END_IDX: usize = (optprobe_template_end as usize - optprobe_template_entry as usize);

static mut insn_page_in_use: bool = false;

pub unsafe fn alloc_optinsn_page() -> *mut core::ffi::c_void {
    if insn_page_in_use {
        return core::ptr::null_mut();
    }
    insn_page_in_use = true;
    &raw mut optinsn_slot as *mut _ as *mut core::ffi::c_void
}

pub unsafe fn free_optinsn_page(_page: *mut core::ffi::c_void) {
    insn_page_in_use = false;
}

/*
 * Check if we can optimize this probe. Returns NIP post-emulation if this can
 * be optimized and 0 otherwise.
 */
unsafe fn can_optimize(p: *mut kprobe) -> c_ulong {
    let mut regs: pt_regs = core::mem::zeroed();
    let mut op: instruction_op = core::mem::zeroed();
    let mut nip: c_ulong = 0;
    let addr: c_ulong = (*p).addr as c_ulong;

    /* kprobe placed for kretprobe during boot time has a 'nop' instruction. */
    if (*p).addr == (&raw const arch_rethook_trampoline as *const _ as *mut kprobe_opcode_t) {
        return addr.wrapping_add(core::mem::size_of::<kprobe_opcode_t>() as c_ulong);
    }
    if !is_kernel_addr(addr) {
        return 0;
    }

    regs.nip = addr;
    regs.trap = 0x0;
    regs.msr = MSR_KERNEL;

    if !is_conditional_branch(ppc_inst_read((*p).ainsn.insn))
        && analyse_instr(&mut op, &mut regs, ppc_inst_read((*p).ainsn.insn)) == 1
    {
        emulate_update_regs(&mut regs, &op);
        nip = regs.nip;
    }
    nip
}

unsafe fn optimized_callback(op: *mut optimized_kprobe, regs: *mut pt_regs) {
    if kprobe_disabled(&(*op).kp) {
        return;
    }
    preempt_disable();
    if kprobe_running() {
        kprobes_inc_nmissed_count(&mut (*op).kp);
    } else {
        __this_cpu_write(current_kprobe, &mut (*op).kp);
        regs_set_return_ip(regs, (*op).kp.addr as c_ulong);
        (*get_kprobe_ctlblk()).kprobe_status = KPROBE_HIT_ACTIVE;
        opt_pre_handler(&mut (*op).kp, regs);
        __this_cpu_write(current_kprobe, core::ptr::null_mut());
    }
    preempt_enable();
}

pub unsafe fn arch_remove_optimized_kprobe(op: *mut optimized_kprobe) {
    if !(*op).optinsn.insn.is_null() {
        free_optinsn_slot((*op).optinsn.insn, 1);
        (*op).optinsn.insn = core::ptr::null_mut();
    }
}

unsafe fn patch_imm32_load_insns(val: c_ulong, reg: c_int, mut addr: *mut kprobe_opcode_t) {
    patch_instruction(addr, ppc_inst(PPC_RAW_LIS(reg, PPC_HI(val))));
    addr = addr.add(1);
    patch_instruction(addr, ppc_inst(PPC_RAW_ORI(reg, reg, PPC_LO(val))));
}

/* Generate instructions to load provided immediate 64-bit value. */
unsafe fn patch_imm64_load_insns(val: c_ulonglong, reg: c_int, mut addr: *mut kprobe_opcode_t) {
    patch_instruction(addr, ppc_inst(PPC_RAW_LIS(reg, PPC_HIGHEST(val)))); addr = addr.add(1);
    patch_instruction(addr, ppc_inst(PPC_RAW_ORI(reg, reg, PPC_HIGHER(val)))); addr = addr.add(1);
    patch_instruction(addr, ppc_inst(PPC_RAW_SLDI(reg, reg, 32))); addr = addr.add(1);
    patch_instruction(addr, ppc_inst(PPC_RAW_ORIS(reg, reg, PPC_HI(val)))); addr = addr.add(1);
    patch_instruction(addr, ppc_inst(PPC_RAW_ORI(reg, reg, PPC_LO(val))));
}

unsafe fn patch_imm_load_insns(val: c_ulong, reg: c_int, addr: *mut kprobe_opcode_t) {
    if IS_ENABLED(CONFIG_PPC64) { patch_imm64_load_insns(val, reg, addr); }
    else { patch_imm32_load_insns(val, reg, addr); }
}

pub unsafe fn arch_prepare_optimized_kprobe(op: *mut optimized_kprobe, p: *mut kprobe) -> c_int {
    let mut branch_op_callback: ppc_inst_t = core::mem::zeroed();
    let mut branch_emulate_step: ppc_inst_t = core::mem::zeroed();
    let mut temp: ppc_inst_t = core::mem::zeroed();
    let mut buff: *mut kprobe_opcode_t;
    let mut b_offset: c_long;
    let nip = can_optimize(p);
    if nip == 0 { return -EILSEQ; }
    buff = get_optinsn_slot();
    if buff.is_null() { return -ENOMEM; }
    b_offset = buff as c_ulong as c_long - (*p).addr as c_ulong as c_long;
    if !is_offset_in_branch_range(b_offset) { free_optinsn_slot(buff, 0); return -ERANGE; }
    b_offset = buff.add(TMPL_RET_IDX) as c_ulong as c_long - nip as c_long;
    if !is_offset_in_branch_range(b_offset) { free_optinsn_slot(buff, 0); return -ERANGE; }
    let size = (TMPL_END_IDX * core::mem::size_of::<kprobe_opcode_t>()) / core::mem::size_of::<c_int>();
    for i in 0..size {
        if patch_instruction(buff.add(i), ppc_inst(*optprobe_template_entry.add(i))) < 0 {
            free_optinsn_slot(buff, 0); return -ERANGE;
        }
    }
    patch_imm_load_insns(op as c_ulong, 3, buff.add(TMPL_OP_IDX));
    let op_callback_addr = ppc_kallsyms_lookup_name("optimized_callback\0".as_ptr() as *const c_char);
    let emulate_step_addr = ppc_kallsyms_lookup_name("emulate_step\0".as_ptr() as *const c_char);
    if op_callback_addr == 0 || emulate_step_addr == 0 { WARN(1, "Unable to lookup optimized_callback()/emulate_step()\n"); free_optinsn_slot(buff, 0); return -ERANGE; }
    let mut rc = create_branch(&mut branch_op_callback, buff.add(TMPL_CALL_HDLR_IDX), op_callback_addr, BRANCH_SET_LINK);
    rc |= create_branch(&mut branch_emulate_step, buff.add(TMPL_EMULATE_IDX), emulate_step_addr, BRANCH_SET_LINK);
    if rc != 0 { free_optinsn_slot(buff, 0); return -ERANGE; }
    patch_instruction(buff.add(TMPL_CALL_HDLR_IDX), branch_op_callback);
    patch_instruction(buff.add(TMPL_EMULATE_IDX), branch_emulate_step);
    temp = ppc_inst_read((*p).ainsn.insn);
    patch_imm_load_insns(ppc_inst_as_ulong(temp), 4, buff.add(TMPL_INSN_IDX));
    patch_branch(buff.add(TMPL_RET_IDX), nip, 0);
    flush_icache_range(buff as c_ulong, buff.add(TMPL_END_IDX) as c_ulong);
    (*op).optinsn.insn = buff;
    0
}

pub unsafe fn arch_prepared_optinsn(optinsn: *mut arch_optimized_insn) -> c_int { (!(*optinsn).insn.is_null()) as c_int }

/* On powerpc, Optprobes always replaces one instruction. */
pub unsafe fn arch_check_optimized_kprobe(_op: *mut optimized_kprobe) -> c_int { 0 }

pub unsafe fn arch_optimize_kprobes(oplist: *mut list_head) {
    let mut instr: ppc_inst_t = core::mem::zeroed();
    let mut op: *mut optimized_kprobe;
    let mut tmp: *mut optimized_kprobe;
    list_for_each_entry_safe!(op, tmp, oplist, list);
    memcpy((*op).optinsn.copied_insn.as_mut_ptr(), (*op).kp.addr, RELATIVEJUMP_SIZE);
    create_branch(&mut instr, (*op).kp.addr, (*op).optinsn.insn as c_ulong, 0);
    patch_instruction((*op).kp.addr, instr);
    list_del_init(&mut (*op).list);
}

pub unsafe fn arch_unoptimize_kprobe(op: *mut optimized_kprobe) { arch_arm_kprobe(&mut (*op).kp); }

pub unsafe fn arch_unoptimize_kprobes(oplist: *mut list_head, done_list: *mut list_head) {
    let mut op: *mut optimized_kprobe;
    let mut tmp: *mut optimized_kprobe;
    list_for_each_entry_safe!(op, tmp, oplist, list);
    arch_unoptimize_kprobe(op);
    list_move(&mut (*op).list, done_list);
}

pub unsafe fn arch_within_optimized_kprobe(op: *mut optimized_kprobe, addr: *mut kprobe_opcode_t) -> c_int {
    ((*op).kp.addr <= addr && (*op).kp.addr.add(RELATIVEJUMP_SIZE / core::mem::size_of::<kprobe_opcode_t>()) > addr) as c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
